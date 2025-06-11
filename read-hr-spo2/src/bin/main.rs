#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_backtrace as _;
use esp_hal::{clock::CpuClock, time::Rate};
use max3010x::{Max3010x, Led, SampleAveraging};
use libm::tanf; 

const SAMPLE_RATE: f32 = 100.0; // Tần số lấy mẫu (Hz)
const BUFFER_SIZE: usize = 100; // Kích thước bộ đệm
const THRESHOLD: f32 = 0.5; // Ngưỡng phát hiện đỉnh 

// Hàm lọc tín hiệu: Bandpass filter
fn filter_signal(signal: &[f32; BUFFER_SIZE], lowcut: f32, highcut: f32, fs: f32) -> [f32; BUFFER_SIZE] {
    let mut filtered = [0.0; BUFFER_SIZE];
    let alpha = tanf(2.0 * 3.14159 * lowcut / fs);
    let beta = tanf(2.0 * 3.14159 * highcut / fs);

    // Bandpass filter
    for i in 2..signal.len() {
        filtered[i] = signal[i]
            + alpha * (signal[i] - signal[i - 2])
            - beta * filtered[i - 1]
            + 0.1 * filtered[i - 2];
    }

    // Kalman filter để làm mượt
    let mut kalman = KalmanFilter::new();
    let mut kalman_filtered = [0.0; BUFFER_SIZE];
    for i in 0..filtered.len() {
        kalman_filtered[i] = kalman.update(filtered[i]);
    }
    kalman_filtered
}

// Cấu trúc cho bộ lọc Kalman
struct KalmanFilter {
    x: f32, // Trạng thái
    p: f32, // Độ không chắc chắn
    q: f32, // Nhiễu quá trình
    r: f32, // Nhiễu đo lường
}

impl KalmanFilter {
    fn new() -> Self {
        KalmanFilter {
            x: 0.0,
            p: 1.0,
            q: 0.01,
            r: 0.1,
        }
    }

    fn update(&mut self, measurement: f32) -> f32 {
        self.p = self.p + self.q;
        let k = self.p / (self.p + self.r);
        self.x = self.x + k * (measurement - self.x);
        self.p = (1.0 - k) * self.p;
        self.x
    }
}

// Hàm phát hiện đỉnh để tính nhịp tim
fn detect_peaks(signal: &[f32; BUFFER_SIZE], threshold: f32) -> ([usize; BUFFER_SIZE], usize) {
    let mut peaks = [0; BUFFER_SIZE];
    let mut peak_count = 0;
    for i in 1..signal.len() - 1 {
        if signal[i] > signal[i - 1] && signal[i] > signal[i + 1] && signal[i] > threshold {
            peaks[peak_count] = i;
            peak_count += 1;
        }
    }
    (peaks, peak_count)
}

// Hàm tính nhịp tim (BPM)
fn calculate_bpm(peaks: &[usize; BUFFER_SIZE], peak_count: usize) -> f32 {
    if peak_count < 2 {
        return 0.0; // Không đủ đỉnh để tính
    }
    let mut intervals = 0.0;
    let mut valid_intervals = 0;
    for i in 1..peak_count {
        let interval = (peaks[i] - peaks[i - 1]) as f32 / SAMPLE_RATE; // Giây
        if interval > 0.2 && interval < 2.0 { // Lọc nhịp bất thường (30-300 BPM)
            intervals += interval;
            valid_intervals += 1;
        }
    }
    if valid_intervals == 0 {
        return 0.0;
    }
    60.0 / (intervals / valid_intervals as f32) // BPM = 60 / trung bình khoảng cách
}

// Hàm tính SpO2
fn calculate_spo2(ir_signal: &[f32; BUFFER_SIZE], red_signal: &[f32; BUFFER_SIZE],
                  ir_filtered: &[f32; BUFFER_SIZE], red_filtered: &[f32; BUFFER_SIZE]) -> f32 {
    // Tính DC (trung bình tín hiệu thô)
    let mut dc_ir = 0.0;
    let mut dc_red = 0.0;
    for i in 0..BUFFER_SIZE {
        dc_ir += ir_signal[i];
        dc_red += red_signal[i];
    }
    dc_ir /= BUFFER_SIZE as f32;
    dc_red /= BUFFER_SIZE as f32;

    // Tính AC (max - min của tín hiệu đã lọc)
    let mut ac_ir = 0.0;
    let mut ac_red = 0.0;
    for i in 0..BUFFER_SIZE {
        if ir_filtered[i] > ac_ir { ac_ir = ir_filtered[i]; }
        if ir_filtered[i] < -ac_ir { ac_ir = -ir_filtered[i]; }
        if red_filtered[i] > ac_red { ac_red = red_filtered[i]; }
        if red_filtered[i] < -ac_red { ac_red = -red_filtered[i]; }
    }

    // Tính R ratio
    if dc_ir == 0.0 || dc_red == 0.0 || ac_ir == 0.0 || ac_red == 0.0 {
        return 0.0; // Tránh chia cho 0
    }
    let r = (ac_red / dc_red) / (ac_ir / dc_ir);

    // Ước lượng SpO2
    let spo2 = 110.0 - 25.0 * r;
    if spo2 < 0.0 || spo2 > 100.0 {
        0.0 // Giá trị không hợp lệ
    } else {
        spo2
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    esp_hal::system::software_reset()
}

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_println::println!("Init!");

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    println!("Embassy init!");

    let i2c_bus = esp_hal::i2c::master::I2c::new(peripherals.I2C0, 
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)))
        .unwrap().with_scl(peripherals.GPIO7).with_sda(peripherals.GPIO6).into_async();

    let max3010x = Max3010x::new_max30102(i2c_bus);
    let mut sensor = max3010x.into_oximeter().unwrap();

    sensor.set_sample_averaging(SampleAveraging::Sa4).unwrap();
    sensor.set_pulse_amplitude(Led::Led1, 0x1F).unwrap(); // IR LED
    sensor.set_pulse_amplitude(Led::Led2, 0x1F).unwrap(); // Red LED
    sensor.enable_fifo_rollover().unwrap();
    let mut data = [0; 3];
    let part_id = sensor.get_part_id().unwrap();

    println!("Part ID: {:#X}", part_id);

    // Bộ đệm cho IR và Red
    let mut ir_buffer = [0.0; BUFFER_SIZE];
    let mut red_buffer = [0.0; BUFFER_SIZE];
    let mut buffer_index = 0;

    loop {
        let samples_read: u8 = sensor.read_fifo(&mut data).unwrap();
        for i in 0..samples_read {
            // Giả định: IR ở 24 bit cao, Red ở 24 bit thấp
            let ir_value = ((data[i as usize] >> 24) & 0xFFFF) as f32;
            let red_value = (data[i as usize] & 0xFFFF) as f32;
            ir_buffer[buffer_index % BUFFER_SIZE] = ir_value;
            red_buffer[buffer_index % BUFFER_SIZE] = red_value;
            buffer_index = (buffer_index + 1) % BUFFER_SIZE;

            // In dữ liệu thô
            println!("Sample {}: IR={:.2}, Red={:.2}", i, ir_value, red_value);

            // Nếu bộ đệm đầy, xử lý tín hiệu
            if buffer_index == 0 {
                // Lọc tín hiệu IR và Red
                let ir_filtered = filter_signal(&ir_buffer, 0.5, 5.0, SAMPLE_RATE);
                let red_filtered = filter_signal(&red_buffer, 0.5, 5.0, SAMPLE_RATE);

                // In tín hiệu đã lọc
                for (j, &value) in ir_filtered.iter().enumerate() {
                    println!("Filtered IR Sample {}: {:.2}", j, value);
                }

                // Tính nhịp tim
                let (peaks, peak_count) = detect_peaks(&ir_filtered, THRESHOLD);
                let bpm = calculate_bpm(&peaks, peak_count);
                println!("Heart Rate: {:.2} BPM", bpm);

                // Tính SpO2
                let spo2 = calculate_spo2(&ir_buffer, &red_buffer, &ir_filtered, &red_filtered);
                println!("SpO2: {:.2}%", spo2);
            }
        }

        Timer::after(Duration::from_millis(1000)).await;
    }
}