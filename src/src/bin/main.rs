//! Alternating sensor and display by destructing and reconstructing with single I2C ownership.

#![no_std]
#![no_main]

use core::fmt::Write;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    i2c::master::I2c,
    peripherals::I2C0,
    time::Rate,
    timer::timg::TimerGroup,
    Async,
};

use esp_println::println;
use max3010x::{marker::ic::Max30102 as Max3010xDevice, marker::mode::HeartRate as HeartRateMode, Led, Max3010x, SampleAveraging};
use ssd1306::{prelude::*, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306Async};
use heapless::String;

type I2cBusType = I2c<'static, Async>;

const SAMPLE_RATE: f32 = 100.0;
const BUFFER_SIZE: usize = 100;
const THRESHOLD: f32 = 100_000.0;
const REFRACTORY_SEC: f32 = 0.7;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let mut i2c = I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO6)
    .with_sda(peripherals.GPIO7)
    .into_async();

    let mut bpm = 0.0;
    let mut show_bpm = true;
    let mut text_timer = 0;

    println!("Booted!");


    loop {
        // === SENSOR ===
        let max3010x = Max3010x::new_max30102(i2c);
        let mut sensor = max3010x.into_heart_rate().unwrap();

        sensor.set_sample_averaging(SampleAveraging::Sa4).unwrap();
        sensor.set_pulse_amplitude(Led::Led1, 0x20).unwrap();
        sensor.set_pulse_amplitude(Led::Led2, 0x1F).unwrap();
        sensor.set_pulse_width(max3010x::LedPulseWidth::Pw69).unwrap();
        sensor.set_sampling_rate(max3010x::SamplingRate::Sps100).unwrap(); // Match SAMPLE_RATE
        sensor.enable_fifo_rollover().unwrap();

        let mut ir_buffer = [0.0; BUFFER_SIZE];
        let mut data = [0; 3];
        let mut index = 0;

        while index < BUFFER_SIZE {
            Timer::after(Duration::from_millis((1000.0 / SAMPLE_RATE) as u64)).await;

            let samples = sensor.read_fifo(&mut data).unwrap_or(0);
            for i in 0..samples {
                let ir = data[i as usize] as f32;
                println!("Sample {}: IR={}", index, ir);
                ir_buffer[index] = ir;
                index += 1;

                if index >= BUFFER_SIZE {
                    break;
                }
            }
        }

        let (peaks, peak_count) = detect_peaks(&ir_buffer, THRESHOLD);
        bpm = calculate_bpm(&peaks, peak_count);
        println!("Detected {} peaks, BPM: {:.2}", peak_count, bpm);

        i2c = sensor.destroy();

        // === DISPLAY ===
        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().await.unwrap();

        let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        display.clear(BinaryColor::Off).unwrap();

        let mut buf: String<64> = String::new();
        write!(buf, "Heart rate: {:.2} BPM", bpm).unwrap();

        Text::with_baseline(&buf, Point::new(0, 32), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().await.unwrap();

        i2c = display.release().release();
    }

}

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
