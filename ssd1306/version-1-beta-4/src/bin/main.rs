#![no_std]
#![no_main]

use core::fmt::Write;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    text::{Baseline, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Line,
};
use embedded_graphics::primitives::PrimitiveStyle;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Instant};
use esp_hal::{clock::CpuClock, time::Rate};
use esp_hal::timer::systimer::SystemTimer;
use ssd1306::mode::DisplayConfigAsync;
use ssd1306::{
    prelude::*, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306Async,
};
use heapless::{Vec, String};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // generator version: 0.3.1
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO9)
    .with_sda(peripherals.GPIO8)
    .into_async();

    let interface = I2CDisplayInterface::new(i2c_bus);

    // initialize the display
    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();

    //BPM Data
    let bpm = 72;
    //SpO2 data
    let spo2 = 100;

    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    //Text counter
    let mut text_timer: u64 = 0;
    let mut show_bpm = true;

    let mut buffer: Vec<i32, DISPLAY_WIDTH> = Vec::new();
    buffer.resize(DISPLAY_WIDTH, BASELINE_Y).unwrap();

    let mut counter = 0;
    let mut wave_index = 0;
    let mut current_wave = get_waveform(WaveType::Normal);
    let mut tick = 0;

    //Screen toggle
    let mut next_toggle = Instant::now() + Duration::from_secs(10);
    let mut display_on = true;

    display.flush().await.unwrap();

    loop {
 // Scroll buffer
        for i in 0..(DISPLAY_WIDTH - 1) {
            buffer[i] = buffer[i + 1];
        }

        // Add next waveform point
        buffer[DISPLAY_WIDTH - 1] = BASELINE_Y + current_wave[wave_index];

        wave_index += 1;
        if wave_index >= current_wave.len() {
            wave_index = 0;

            // After each waveform finishes, choose a new one every 3rd beat
            tick += 1;
            current_wave = get_waveform(match tick % 3 {
                0 => WaveType::Normal,
                1 => WaveType::Fast,
                _ => WaveType::Fast,
            });
        }

        // Draw waveform
        display.clear(BinaryColor::Off).unwrap();

        for x in 1..DISPLAY_WIDTH {
            Line::new(
                Point::new((x - 1) as i32, buffer[x - 1]),
                Point::new(x as i32, buffer[x]),
            )
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();
        }

        //Display text
        let mut buffer: String<64> = String::new();
        buffer.clear();

        if show_bpm {
            write!(buffer, "Heart rate: {:.2} BPM", bpm).unwrap();
        } else {
            write!(buffer, "SpO2: {:.2}%", spo2).unwrap();
        }

        Text::with_baseline(&buffer, Point::new(0, 44), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().await.unwrap();


        let now = Instant::now();
        
        if display_on && now >= next_toggle {
            display.set_display_on(false).await.unwrap();
            display_on = false;
            next_toggle = now + Duration::from_secs(5);
        } else if !display_on && now >= next_toggle {
            display.set_display_on(true).await.unwrap();
            display_on = true;
            next_toggle = now + Duration::from_secs(5);
        }
        
        Timer::after(Duration::from_millis(30)).await;

        text_timer += 30;
        if text_timer >= 2000 {
            show_bpm = !show_bpm;  // toggle between true/false
            text_timer = 0;
        }
        counter += 2;
    }
}

const BASELINE_Y: i32 = 20;
const DISPLAY_WIDTH: usize = 128;
const NORMAL_WAVE: [i32; 20] = [0, 5, 10, 20, 10, 5, 0, -5, -10, -20, -10, -5, 0, 5, 10, 5, 0, -5, -10, 0];
const FAST_WAVE:  [i32; 20] = [0, 1, 2, 3, 2, 1, 0, 0, -1, -2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

enum WaveType {
    Normal,
    Fast
}

fn get_waveform(wave_type: WaveType) -> &'static [i32] {
    match wave_type {
        WaveType::Normal => &NORMAL_WAVE,
        WaveType::Fast => &FAST_WAVE,
    }
}