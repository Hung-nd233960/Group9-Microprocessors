#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay , prelude::*};
use esp_hal::i2c::I2c;
use esp_hal::gpio::Io;
use esp_hal::timer::systimer::SystemTimer;
use max3010x::{Max3010x, Led, SampleAveraging};
use embassy_time::{Instant, Duration, Timer};

extern crate alloc;
use core::mem::MaybeUninit;

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr() as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    init_heap();

    let mut delay = Delay::new();
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    esp_println::logger::init_logger_from_env();

    let i2c = I2c::new(
        peripherals.I2C0,
        io.pins.gpio6, // SDA 
        io.pins.gpio7, // SCL 
        400_u32.kHz(),
    );
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let max3010x = Max3010x::new_max30102(i2c);
    let mut sensor = max3010x.into_heart_rate().unwrap();

    sensor.set_sample_averaging(SampleAveraging::Sa4).unwrap();
    sensor.set_pulse_amplitude(Led::Led1, 0x1F).unwrap(); // IR LED1
    sensor.enable_fifo_rollover().unwrap();

    let mut data = [0u32; 4];

    loop {
        match sensor.read_fifo(&mut data) {
            Ok(n) => {
                for (i, sample) in data.iter().take(n.into()).enumerate() {
                    esp_println::println!("IR Sample {}: {}", i, sample);
                }
            }
            Err(e) => {
                esp_println::println!("Error: {:?}", e);
            }
        }

        delay.delay(500.millis());
    }
}