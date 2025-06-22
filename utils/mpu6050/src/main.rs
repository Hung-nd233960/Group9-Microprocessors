#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*, Blocking};
use esp_hal::i2c::I2c;
use esp_hal::gpio::Io;
use esp_hal::peripherals::I2C0;
use core::mem::MaybeUninit;
use libm::sqrtf;
use log::info;
extern crate alloc;

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

#[derive(Debug, PartialEq)]
enum State {
    Normal,
    DetectingFallOrRun,
    FallDetected,
    Impact,
    PostFall,
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
        io.pins.gpio8,
        io.pins.gpio9,
        1000_u32.kHz(),
    );

    let mut sensor = mpu6050::Mpu6050::new(i2c);
    sensor.init(&mut delay).unwrap();

    let mut state = State::Normal;
    let mut stable_counter = 0u8;
    let mut detecting_counter = 0u8;

    loop {
        let acc = sensor.get_acc();
        let gyro = sensor.get_gyro();

        if let (Ok(acc), Ok(gyro)) = (acc, gyro) {
            let (ax, ay, az) = (acc[0], acc[1], acc[2]);
            let (gx, gy, gz) = (gyro[0], gyro[1], gyro[2]);

            let a_total = sqrtf(ax * ax + ay * ay + az * az);
            let omega = sqrtf(gx * gx + gy * gy + gz * gz);

            match state {
                State::Normal => {
                    if a_total > 1.7 && omega > 3.5 {
                        state = State::DetectingFallOrRun;
                        detecting_counter = 0;
                        info!("High motion detected. Determining if it's a fall or running...");
                    }
                }

                State::DetectingFallOrRun => {
                    detecting_counter += 1;

                    if a_total < 1.4 && omega < 0.2 {
                        // Sau vài chu kỳ thấy bất động ⇒ fall
                        if detecting_counter > 2 {
                            state = State::FallDetected;
                            detecting_counter = 0;
                            info!("Fall likely. Transitioning to FALL_DETECTED.");
                        }
                    } else if a_total > 1.2 && omega > 0.8 {
                        // Sau chuyển động mạnh mà vẫn còn hoạt động mạnh ⇒ running
                        if detecting_counter > 2 {
                            state = State::Normal;
                            detecting_counter = 0;
                            info!("Likely running. Returning to NORMAL.");
                        }
                    } else if detecting_counter > 6 {
                        // Không rõ ⇒ quay lại bình thường
                        state = State::Normal;
                        detecting_counter = 0;
                        info!("Uncertain event. Returning to NORMAL.");
                    }
                }

                State::FallDetected => {
                    if a_total < 0.5 && omega < 8.0 {
                        stable_counter += 1;
                        if stable_counter > 4 {
                            state = State::Impact;
                            info!("Impact detected. Monitoring post-fall state...");
                            stable_counter = 0;
                        }
                    } else if a_total < 1.3 && omega < 2.0 {
                        stable_counter += 1;
                        if stable_counter > 6 {
                            state = State::Normal;
                            info!("False alarm. Returning to NORMAL.");
                            stable_counter = 0;
                        }
                    } else {
                        stable_counter = 0;
                    }
                }

                State::Impact => {
                    if a_total < 0.3 && omega < 10.0 {
                        stable_counter += 1;
                        if stable_counter > 10 {
                            state = State::PostFall;
                            info!("Post-fall state confirmed.");
                            stable_counter = 0;
                        }
                    } else {
                        state = State::Normal;
                        stable_counter = 0;
                    }
                }

                State::PostFall => {
                    info!("User is immobile. Possible fall confirmed.");
                    if a_total > 1.0 || omega > 20.0 {
                        state = State::Normal;
                        info!("Movement detected. Resetting to NORMAL.");
                    }
                }
            }

            info!(
                "Accel Total: {:.2}, Gyro Total: {:.2}, State: {:?}",
                a_total, omega, state
            );
        } else {
            log::error!("MPU6050 read error");
        }

        delay.delay(500.millis());
    }
}