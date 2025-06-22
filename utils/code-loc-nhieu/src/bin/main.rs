//! MAX30102 async IR reader using Embassy for ESP32-C6
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    i2c::master::I2c,
    peripherals::Peripherals,
    prelude::*,
    timer::timg::TimerGroup,
    time::Rate,
};
use esp_println::println;
use embedded_hal_async::i2c::I2c as AsyncI2c;
use embedded_hal::i2c::SevenBitAddress;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::MHz(160));
    let peripherals = esp_hal::init(config);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let i2c = I2c::new(
        peripherals.I2C0,
        peripherals.GPIO6,
        peripherals.GPIO7,
        Rate::from_khz(400),
    )
    .unwrap()
    .into_async();

    let mut sensor = Max30102::new(i2c);

    match sensor.init().await {
        Ok(_) => println!("MAX30102 init OK (IR-only + BPM)"),
        Err(_) => {
            println!("I2C: Không tìm thấy MAX30102 (ACK failed)");
            loop {}
        }
    }

    loop {
        match sensor.read_ir().await {
            Ok(ir) => println!("IR = {}", ir),
            Err(_) => println!("Lỗi khi đọc IR"),
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

pub struct Max30102<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> Max30102<I2C>
where
    I2C: AsyncI2c<SevenBitAddress>,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c, address: 0x57 }
    }

    pub async fn init(&mut self) -> Result<(), ()> {
        self.write_reg(0x09, 0x40).await?;
        Timer::after(Duration::from_millis(10)).await;

        self.write_reg(0x04, 0x00).await?;
        self.write_reg(0x05, 0x00).await?;
        self.write_reg(0x06, 0x00).await?;

        self.write_reg(0x08, 0b0101_1111).await?;
        self.write_reg(0x09, 0x02).await?;
        self.write_reg(0x0A, 0b0100_0011).await?;
        self.write_reg(0x0D, 0x24).await?;

        Ok(())
    }

    pub async fn read_ir(&mut self) -> Result<u32, ()> {
        let mut buf = [0u8; 3];
        self.i2c.write_read(self.address, &[0x07], &mut buf).await.map_err(|_| ())?;
        let value = ((buf[0] as u32) << 16 | (buf[1] as u32) << 8 | buf[2] as u32) & 0x3FFFF;
        Ok(value)
    }

    async fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), ()> {
        self.i2c.write(self.address, &[reg, val]).await.map_err(|_| ())
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {:?}", info);
    loop {}
}
