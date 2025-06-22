#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_backtrace as _;
use esp_hal::{clock::CpuClock, time::Rate};
use max3010x::{Max3010x, Led, SampleAveraging};



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

    loop {  
        
        let samples_read: u8 = sensor.read_fifo(&mut data).unwrap();
        for i in 0..samples_read {
            println!("Sample {}: {:?}", i, data[i as usize]);

        Timer::after(Duration::from_millis(1000)).await;
        }

    }
}

