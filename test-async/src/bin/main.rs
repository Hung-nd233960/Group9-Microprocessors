#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_backtrace as _;
use embassy_time::Instant;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    esp_hal::system::software_reset()
}

#[embassy_executor::task]
async fn run(mut green_led: Output<'static>) {
    loop {
        green_led.set_high();
        esp_println::println!(
            "[{:?}] Green LED ON",
            Instant::now().as_millis(),

        );
        Timer::after(Duration::from_millis(500)).await;
        green_led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut green_led = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let mut red_led = Output::new(peripherals.GPIO7, Level::High, OutputConfig::default());

    println!("Init!");

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    spawner.spawn(run(green_led)).ok();

    loop {

        red_led.set_high();
        esp_println::println!(
        "[{:?}] Red LED ON", 
        Instant::now().as_millis()
        );
        Timer::after(Duration::from_millis(2_000)).await;
        red_led.set_low();
        Timer::after(Duration::from_millis(2_000)).await;
    }
}

// We see that 4 green cycle = 1 red, but in real life, RED LED always seem to drift a bit earlier???????