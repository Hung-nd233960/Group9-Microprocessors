#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_backtrace as _;
use esp_hal::{clock::CpuClock, time::Rate};
use max3010x::{Max3010x, Led, SampleAveraging, Mode, SampleRate, AdcRange};

const SAMPLE_RATE: f32 = 200.0; // Tần số lấy mẫu 200 Hz
const BUFFER_SIZE: usize = 100;
const THRESHOLD: f32 = 100.0; // Ngưỡng phát hiện đỉnh cho tín hiệu thô

// Hàm phát hiện đỉnh (dùng tín hiệu thô)
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
        return 0.0;
    }
    let mut intervals = 0.0;
    let mut valid_intervals = 0;
    for i in 1..peak_count {
        let interval = (peaks[i] - peaks[i - 1]) as f32 / SAMPLE_RATE; // Giây
        if interval > 0.2 && interval < 2.0 { // 30-300 BPM
            intervals += interval;
            valid_intervals += 1;
        }
    }
    if valid_intervals == 0 {
        return 0.0;
    }
    60.0 / (intervals / valid_intervals as f32)
}

// Hàm tính SpO2 (dùng tín hiệu thô)
fn calculate_spo2(ir_signal: &[f32; BUFFER_SIZE], red_signal: &[f32; BUFFER_SIZE]) -> f32 {
    // Tính DC (trung bình tín hiệu thô)
    let mut dc_ir = 0.0;
    let mut dc_red = 0.0;
    for i in 0..BUFFER_SIZE {
        dc_ir += ir_signal[i];
        dc_red += red_signal[i];
    }
    dc_ir /= BUFFER_SIZE as f32;
    dc_red /= BUFFER_SIZE as f32;

    // Tính AC (max - min của tín hiệu thô)
    let mut max_ir = ir_signal[0];
    let mut min_ir = ir_signal[0];
    let mut max_red = red_signal[0];
    let mut min_red = red_signal[0];
    for i in 0..BUFFER_SIZE {
        if ir_signal[i] > max_ir { max_ir = ir_signal[i]; }
        if ir_signal[i] < min_ir { min_ir = ir_signal[i]; }
        if red_signal[i] > max_red { max_red = red_signal[i]; }
        if red_signal[i] < min_red { min_red = red_signal[i]; }
    }
    let ac_ir = max_ir - min_ir;
    let ac_red = max_red - min_red;

    println!("DC_IR={:.2}, DC_Red={:.2}, AC_IR={:.2}, AC_Red={:.2}", dc_ir, dc_red, ac_ir, ac_red);
    if dc_ir < 1000.0 || dc_red < 1000.0 || ac_ir < 10.0 || ac_red < 10.0 {
        println!("Error: Weak signal or no pulse");
        return 0.0;
    }
    let r = (ac_red / dc_red) / (ac_ir / dc_ir);
    println!("R={:.2}", r);

    let spo2 = 104.0 - 17.0 * r; // Công thức điều chỉnh
    if spo2 < 0.0 || spo2 > 100.0 {
        println!("Invalid SpO2: {:.2}", spo2);
        0.0
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

    sensor.set_mode(Mode::Spo2).unwrap();
    sensor.set_sample_averaging(SampleAveraging::Sa16).unwrap();
    sensor.set_sample_rate(SampleRate::Sps200).unwrap();
    sensor.set_adc_range(AdcRange::Lsb4096).unwrap();
    sensor.set_pulse_amplitude(Led::Led1, 0xFF).unwrap(); // IR tối đa
    sensor.set_pulse_amplitude(Led::Led2, 0x1F).unwrap(); // Red giảm
    sensor.enable_fifo_rollover().unwrap();
    let mut data = [0; 32]; // Chứa nhiều mẫu
    let part_id = sensor.get_part_id().unwrap();

    println!("Part ID: {:#X}", part_id);
    println!("Mode: {:?}", sensor.get_mode().unwrap());
    println!("IR amplitude: {:?}", sensor.get_pulse_amplitude(Led::Led1).unwrap());

    let mut ir_buffer = [0.0; BUFFER_SIZE];
    let mut red_buffer = [0.0; BUFFER_SIZE];
    let mut buffer_index = 0;

    loop {
        let samples_read: u8 = sensor.read_fifo(&mut data).unwrap();
        println!("Samples read: {}", samples_read);
        for i in 0..(samples_read / 2) {
            // Đọc 24 bit (3 byte) cho IR và Red
            let ir_raw = (data[i as usize * 2] & 0xFFFFFF) as f32; // 24 bit IR
            let red_raw = (data[i as usize * 2 + 1] & 0xFFFFFF) as f32; // 24 bit Red
            println!("Raw data[{}]: IR={:032b}, Red={:032b}", i, data[i as usize * 2], data[i as usize * 2 + 1]);
            println!("Sample {}: IR={:.2}, Red={:.2}", i, ir_raw, red_raw);

            ir_buffer[buffer_index % BUFFER_SIZE] = ir_raw;
            red_buffer[buffer_index % BUFFER_SIZE] = red_raw;
            buffer_index = (buffer_index + 1) % BUFFER_SIZE;

            if buffer_index == 0 {
                let (peaks, peak_count) = detect_peaks(&ir_buffer, THRESHOLD);
                let bpm = calculate_bpm(&peaks, peak_count);
                println!("Heart Rate: {:.2} BPM", bpm);
                let spo2 = calculate_spo2(&ir_buffer, &red_buffer);
                println!("SpO2: {:.2}%", spo2);
            }
        }
        Timer::after(Duration::from_millis(1000)).await;
    }
}