#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_backtrace as _;


#[embassy_executor::task]
async fn run(mut green_led: Output<'static>) {
    let mut next = Instant::now();

    loop {
        green_led.set_high();
        println!("Green LED on, at time {:?}", next);
        Timer::after(Duration::from_millis(500)).await;
        green_led.set_low();

        next += Duration::from_millis(1000);
        Timer::at(next).await;
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    esp_hal::system::software_reset()
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut green_led = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let mut red_led = Output::new(peripherals.GPIO7, Level::High, OutputConfig::default());

    esp_println::println!("Init!");

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    spawner.spawn(run(green_led)).ok();

    let mut next = Instant::now();

    loop {
        red_led.set_high();
        println!("Red LED on, at time {:?}", next);
        Timer::after(Duration::from_millis(1000)).await;
        red_led.set_low();

        next += Duration::from_millis(2000);
        Timer::at(next).await;
        
    }
}


// We see that 4 green cycle = 1 red, but in real life, RED LED always seem to drift a bit earlier???????