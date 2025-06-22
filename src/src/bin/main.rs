//! Alternating sensor and display by destructing and reconstructing with single I2C ownership.

#![no_std]
#![no_main]

use core::fmt::Write;
use micromath::F32Ext;

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
    clock::CpuClock, gpio::Io, gpio::InputConfig, gpio::Pull, gpio::Input,
    i2c::master::I2c, parl_io::TxCreatorFullDuplex, peripherals::I2C0, time::Rate, timer::timg::TimerGroup, Async
};

use esp_println::println;
use max3010x::{
    marker::ic::Max30102 as Max3010xDevice,
    marker::mode::HeartRate as HeartRateMode,
    Led, Max3010x, SampleAveraging,
};
use ssd1306::{
    prelude::*, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306Async,
};
use heapless::String;

type I2cBusType = I2c<'static, Async>;

const SAMPLE_RATE: f32 = 100.0;
const FAST_BUFFER_SIZE: usize = 100;
const SLOW_BUFFER_SIZE: usize = 500;
const THRESHOLD: f32 = 100_000.0;

use core::sync::atomic::{AtomicBool, Ordering};
static SLOW_MODE: AtomicBool = AtomicBool::new(true);

#[embassy_executor::task]
async fn watch_button(mut button: Input<'static, >) {
    loop {
        button.wait_for_falling_edge().await;
        let current = SLOW_MODE.load(Ordering::Relaxed);
        SLOW_MODE.store(!current, Ordering::Relaxed);
        println!("Mode toggled: {}", if !current { "Slow" } else { "Fast" });
        Timer::after_millis(100).await; // debounce
    }
}

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
    let mut io = Io::new(peripherals.IO_MUX);
    let mut REFRACTORY_SEC: f32 = 0.7;
    
    let button = peripherals.GPIO8;
    let input_config = InputConfig::default().with_pull(Pull::Up);
    let mut button = Input::new(button, input_config);
    _spawner.spawn(watch_button(button)).unwrap();
    let mut slow_mode = true;
    if slow_mode {
        REFRACTORY_SEC = 0.65;
    } else {
        REFRACTORY_SEC = 0.7;
    }

    println!("Booted!");

    loop {
        let slow_mode = SLOW_MODE.load(Ordering::Relaxed);
        let buffer_size = if slow_mode { SLOW_BUFFER_SIZE } else { FAST_BUFFER_SIZE };
        let max3010x = Max3010x::new_max30102(i2c);
        let mut sensor: Max3010x<I2c<'_, Async>, Max3010xDevice, HeartRateMode> = max3010x.into_heart_rate().unwrap();

        sensor.set_sample_averaging(SampleAveraging::Sa4).unwrap();
        sensor.set_pulse_amplitude(Led::Led1, 0x20).unwrap();
        sensor.set_pulse_amplitude(Led::Led2, 0x00).unwrap();
        sensor.set_pulse_width(max3010x::LedPulseWidth::Pw69).unwrap();
        sensor.set_sampling_rate(max3010x::SamplingRate::Sps100).unwrap();
        sensor.enable_fifo_rollover().unwrap();

        let mut ir_buffer = [0.0; SLOW_BUFFER_SIZE];
        let mut data: [u32; 3] = [0; 3];
        let mut index = 0;

        while index < buffer_size {
            Timer::after(Duration::from_millis((1000.0 / SAMPLE_RATE) as u64)).await;
            let samples: u8 = sensor.read_fifo(&mut data).unwrap_or(0);
            for i in 0..samples {
                ir_buffer[index] = data[i as usize] as f32;
                println!("Sample {}: IR={}", index, ir_buffer[index]);
                index += 1;
                if index >= buffer_size {
                    break;
                }
            }
        }
        // Make peaks and peak_count mutable and sized for the largest buffer
        static mut PEAKS: [usize; SLOW_BUFFER_SIZE] = [0; SLOW_BUFFER_SIZE];
        let mut peak_count: usize = 0;

        // SAFETY: Only this task accesses PEAKS, so this is safe in this context.
        let peaks: &mut [usize] = unsafe { &mut PEAKS };

        if slow_mode {
            let (found_peaks, count) = detect_peaks_slow(&ir_buffer, buffer_size, THRESHOLD, REFRACTORY_SEC);
            peaks[..count].copy_from_slice(&found_peaks[..count]);
            peak_count = count;
        } else {
            let (found_peaks, count) = detect_peaks_fast(&ir_buffer, buffer_size, THRESHOLD, REFRACTORY_SEC);
            peaks[..count].copy_from_slice(&found_peaks[..count]);
            peak_count = count;
        }

        i2c = sensor.destroy();
        let interface: I2CInterface<I2c<'_, Async>> = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().await.unwrap();
        let text_style: MonoTextStyle<'_, BinaryColor> = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        display.clear(BinaryColor::Off).unwrap();
        let mut buf: String<128> = String::new();

        if slow_mode {
            let (hr, sdpp, sdcd, rmscd) = calculate_hrv(&peaks, peak_count);
            println!("SLOW MODE: HR: {:.1}, SDPP: {:.1}, SDCD: {:.1}, RMSCD: {:.1}", hr, sdpp, sdcd, rmscd);
            write!(buf, "HR: {:.1}\nSDPP: {:.1}\nSDCD: {:.1}\nRMSCD: {:.1}", hr, sdpp, sdcd, rmscd).unwrap();
        } else {
            let bpm = calculate_bpm(&peaks, peak_count);
            println!("FAST MODE: BPM: {:.2}", bpm);
            write!(buf, "FAST MODE\nBPM: {:.2}", bpm).unwrap();
        }

        Text::with_baseline(&buf, Point::new(0, 0), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().await.unwrap();

        i2c = display.release().release();
    }
}

fn detect_peaks_slow(signal: &[f32], len: usize, threshold: f32, refrac: f32) -> ([usize; SLOW_BUFFER_SIZE], usize) {
    let REFRACTORY_SEC = refrac;
    let mut peaks = [0; SLOW_BUFFER_SIZE];
    let mut count = 0;
    let mut last_peak_index: Option<usize> = None;

    for i in 1..(len - 1) {
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
fn detect_peaks_fast(signal: &[f32], len: usize, threshold: f32, refrag: f32) -> ([usize; FAST_BUFFER_SIZE], usize) {
    let REFRACTORY_SEC = refrag;
    let mut peaks = [0; FAST_BUFFER_SIZE];
    let mut count = 0;
    let mut last_peak_index: Option<usize> = None;

    for i in 1..(len - 1) {
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

fn calculate_bpm(peaks: &[usize], count: usize) -> f32 {
    if count < 2 {
        return 0.0;
    }
    let mut total = 0.0;
    let mut valid = 0;
    for i in 1..count {
        let dt = (peaks[i] - peaks[i - 1]) as f32 / SAMPLE_RATE;
        if dt > 0.3 && dt < 2.0 {
            total += dt;
            valid += 1;
        }
    }
    if valid == 0 {
        return 0.0;
    }
    60.0 / (total / valid as f32)
}

fn calculate_hrv(peaks: &[usize], count: usize) -> (f32, f32, f32, f32) {
    if count < 3 {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let mut intervals = [0.0f32; SLOW_BUFFER_SIZE];
    let mut valid = 0;
    for i in 1..count {
        let dt = (peaks[i] - peaks[i - 1]) as f32 / SAMPLE_RATE * 1000.0;
        if dt > 300.0 && dt < 2000.0 {
            intervals[valid] = dt;
            valid += 1;
        }
    }
    if valid < 2 {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let mean_ppi = intervals[..valid].iter().sum::<f32>() / valid as f32;
    let hr = 60000.0 / mean_ppi;
    let sdpp = stddev(&intervals[..valid], mean_ppi);
    let mut diffs = [0.0f32; SLOW_BUFFER_SIZE];
    for i in 1..valid {
        diffs[i - 1] = (intervals[i] - intervals[i - 1]).abs();
    }
    let sdcd = stddev(&diffs[..valid - 1], mean(&diffs[..valid - 1]));
    let rmscd = (diffs[..valid - 1].iter().map(|x| x * x).sum::<f32>() / (valid - 1) as f32).sqrt();

    (hr, sdpp, sdcd, rmscd)
}

fn mean(data: &[f32]) -> f32 {
    data.iter().copied().sum::<f32>() / data.len() as f32
}

fn stddev(data: &[f32], mean: f32) -> f32 {
    let var = data.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / data.len() as f32;
    var.sqrt()
}
