#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_backtrace as _;
use esp_hal::{clock::CpuClock, time::Rate};
use max3010x::{Max3010x, Led, SampleAveraging};
use libm::tanf; // Thêm libm để hỗ trợ hàm tan trong no_std

// Định nghĩa hằng số cho bộ lọc
const SAMPLE_RATE: f32 = 100.0; // Tần số lấy mẫu (Hz), giả định 100 Hz
const BUFFER_SIZE: usize = 100; // Kích thước bộ đệm cho dữ liệu IR

// Hàm lọc tín hiệu: Chỉ sử dụng bandpass filter
fn filter_signal(signal: &[f32; BUFFER_SIZE], lowcut: f32, highcut: f32, fs: f32) -> [f32; BUFFER_SIZE] {
    let mut filtered = [0.0; BUFFER_SIZE];
    let alpha = tanf(2.0 * 3.14159 * lowcut / fs);
    let beta = tanf(2.0 * 3.14159 * highcut / fs);

    // Bandpass filter: Loại bỏ nhiễu tần số thấp (< 0.5 Hz) và cao (> 5 Hz)
    for i in 2..signal.len() {
        filtered[i] = signal[i]
            + alpha * (signal[i] - signal[i - 2])
            - beta * filtered[i - 1]
            + 0.1 * filtered[i - 2];
    }
    filtered
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
    sensor.set_pulse_amplitude(Led::Led1, 0x1F).unwrap(); // IR LED1
    sensor.enable_fifo_rollover().unwrap();
    let mut data = [0; 3];
    let part_id = sensor.get_part_id().unwrap();

    println!("Part ID: {:#X}", part_id);

    // Bộ đệm để lưu trữ dữ liệu IR
    let mut ir_buffer = [0.0; BUFFER_SIZE];
    let mut buffer_index = 0;

    loop {
        let samples_read: u8 = sensor.read_fifo(&mut data).unwrap();
        for i in 0..samples_read {
            // Giả định dữ liệu IR nằm ở data[i][0] (theo tài liệu MAX30102)
            let ir_value = data[i as usize] as f32;
            ir_buffer[buffer_index % BUFFER_SIZE] = ir_value;
            buffer_index = (buffer_index + 1) % BUFFER_SIZE;

            // In dữ liệu thô
            println!("Sample {}: {:?}", i, data[i as usize]);

            // Nếu bộ đệm đầy, áp dụng bộ lọc
            if buffer_index == 0 {
                let filtered_data = filter_signal(&ir_buffer, 0.5, 5.0, SAMPLE_RATE);
                for (j, &filtered_value) in filtered_data.iter().enumerate() {
                    println!("Filtered Sample {}: {:.2}", j, filtered_value);
                }
            }
        }

        Timer::after(Duration::from_millis(1000)).await;
    }
}