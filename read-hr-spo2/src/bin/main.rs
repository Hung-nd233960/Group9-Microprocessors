#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_backtrace as _;
use esp_hal::{clock::CpuClock, time::Rate};
use max3010x::{Max3010x, Led, SampleAveraging};

const SAMPLE_RATE: f32 = 100.0;
const BUFFER_SIZE: usize = 100;
const THRESHOLD: f32 = 100000.0; // Tùy chỉnh phù hợp với tín hiệu thực tế
const REFRACTORY_SEC: f32 = 0.7;

fn detect_peaks(signal: &[f32; BUFFER_SIZE], threshold: f32) -> ([usize; BUFFER_SIZE], usize) {
    let mut peaks = [0; BUFFER_SIZE];
    let mut count = 0;
    let mut last_peak_index: Option<usize> = None;

    for i in 1..(BUFFER_SIZE - 1) {
        let prev = signal[i - 1];
        let curr = signal[i];
        let next = signal[i + 1];

        if curr > prev && curr > next && curr > threshold {
            let t_now = i as f32 / SAMPLE_RATE;
            if let Some(last) = last_peak_index {
                let t_last = last as f32 / SAMPLE_RATE;
                if t_now - t_last < REFRACTORY_SEC {
                    continue;
                }
            }
            peaks[count] = i;
            count += 1;
            last_peak_index = Some(i);
        }
    }

    (peaks, count)
}

fn calculate_bpm(peaks: &[usize; BUFFER_SIZE], count: usize) -> f32 {
    if count < 2 {
        return 0.0;
    }

    let mut total_interval = 0.0;
    let mut valid = 0;
    let mut last_time = peaks[0] as f32 / SAMPLE_RATE;

    for i in 1..count {
        let now = peaks[i] as f32 / SAMPLE_RATE;
        let dt = now - last_time;

        if dt >= REFRACTORY_SEC {
            total_interval += dt;
            valid += 1;
            last_time = now;
        }
    }

    if valid == 0 {
        0.0
    } else {
        60.0 / (total_interval / valid as f32)
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
    println!("Init!");

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);
    println!("Embassy init!");

    let i2c = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO7)
    .with_sda(peripherals.GPIO6)
    .into_async();

    let max3010x = Max3010x::new_max30102(i2c);
    let mut sensor = max3010x.into_heart_rate().unwrap();

    sensor.set_sample_averaging(SampleAveraging::Sa4).unwrap();
    sensor.set_pulse_amplitude(Led::Led1, 0x24).unwrap();
    sensor.set_pulse_amplitude(Led::Led2, 0x00).unwrap();
    sensor.set_pulse_width(max3010x::LedPulseWidth::Pw69).unwrap();
    sensor.set_sampling_rate(max3010x::SamplingRate::Sps400).unwrap();
    sensor.enable_fifo_rollover().unwrap();

    let mut data = [0; 3];
    let mut ir_buffer = [0.0; BUFFER_SIZE];
    let mut index = 0;

    loop {
        let samples = sensor.read_fifo(&mut data).unwrap();
        
        for i in 0..samples {
            let ir = ((data[i as usize]) as f32 )/2.0;
            ir_buffer[index] = ir;
            println!("Sample {}: IR={:.2}", index, ir);
            index += 1;

            if index >= BUFFER_SIZE {
                let (peaks, peak_count) = detect_peaks(&ir_buffer, THRESHOLD);
                println!("Detected {} peaks", peak_count);
                for i in 0..peak_count {
                    println!("Peak {} at index {}", i + 1, peaks[i]);
                }

                let bpm = calculate_bpm(&peaks, peak_count);
                println!("Heart Rate: {:.2} BPM", bpm);

                index = 0;
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}
