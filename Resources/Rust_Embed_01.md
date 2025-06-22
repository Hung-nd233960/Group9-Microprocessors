# Rust Embedded
## Day 31/3/2025: Reasons of using Rust
Rust offers a **new way of thinking** about embedded software — safer, more expressive, and maintainable — without sacrificing low-level control.

### **Memory Safety Without a Garbage Collector**
* **C/C++** rely on manual memory management — bugs like buffer overflows, use-after-free, null dereferencing are common and dangerous in health-critical systems like OxyRem.
* **Rust enforces safety at compile time** via its *ownership model*, preventing those bugs without runtime overhead.
* No **garbage collector** means Rust runs deterministically — crucial for real-time data like SpO₂ or heart rate.

> For instance, we *cannot* access freed memory in Rust — the compiler will stop you.

### 2. **Zero-Cost Abstractions**

* Rust allows writing **high-level code (like iterators, traits, closures)** that compiles down to **efficient machine code** with **no performance cost**.
* This means we can make your embedded code **clean, testable, and safe** — without sacrificing control.

> With Rust’s abstractions, embedded code becomes **more readable and maintainable** while remaining **as fast as C**.

### 3. **Concurrency Made Safe**

> Rust’s type system prevents **data races at compile time**, unlike C which can easily corrupt shared data.

* Even if our system doesn't use threads, **async/await** (like with [Embassy](https://embassy.dev/)) gives us **cooperative multitasking** in a `no_std` environment.

> However, **If our project is under time pressure**, async may be overkill unless we need multitasking.

### 4. **`no_std` Ecosystem for Bare Metal**

* Rust can run **without the standard library**, which is essential for **bare-metal embedded targets** like ESP32-C3.
* We get **direct hardware control**, deterministic behavior, and total control over the binary size and layout.

Our project benefits from `no_std` because:

* We want low power, no heap allocation (unless managed).
* You want offline-first operation → no OS, no panic-heavy dependencies.
* We don't need networking

> `no_std` Rust is growing bigger, with proper support for interrupts, peripherals, and even logging frameworks.

### 5. **Growing Ecosystem & Hardware Abstraction Layer (HAL) Support**

* Rust's `embedded-hal` traits define **unified interfaces** (for I2C, SPI, ADC, GPIO, etc.) that are **implemented by platform-specific crates** like `esp-hal`, `nrf-hal`, etc.
* These HALs **abstract away chip details** and provide **safe APIs to control peripherals**.

> Example: `esp-hal` lets you configure ADC channels or I2C peripherals safely without worrying about raw registers.

### 6. **Tooling & Developer Experience**

* `cargo` for package/dependency management
* `rust-analyzer` for IDE support
* `probe-rs` and `esp-flash` for **debugging and flashing**
* **Integration testing** at HAL level becomes possible

For our project, these tools make development **more predictable and traceable** compared to debugging in C.


### Key Embedded Programming Concepts

Here are important definitions we need 

| Term                                 | Definition                                                                                                                                               |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Binding**                          | A low-level **link between Rust and C code**, usually using FFI (`extern "C"`). Used when leveraging vendor C SDKs (like ESP-IDF).                       |
| **Wrapper**                          | A **safe Rust abstraction** over unsafe bindings. Lets you use C code safely from Rust.                                                                  |
| **Hardware Abstraction Layer (HAL)** | A crate that abstracts over microcontroller peripherals (like GPIO, I2C, SPI) via common Rust **traits**. Makes embedded code **portable** and safer.    |
| **`no_std`**                         | A Rust programming mode without the standard library. Essential for bare-metal environments like ESP32. Gives **full control** over memory and hardware. |

###  Practical Fit 
| Component          | Recommendation                                                                                           |
| ------------------ | -------------------------------------------------------------------------------------------------------- |
| **Platform**       | ✅ ESP32-C3 – good for low power + wireless                                                            |
| **Language**       | ✅ Rust with `no_std`                                                                                     |
| **HAL**            | ✅ Use [`esp-hal`](https://github.com/esp-rs/esp-hal) — stable, safe                                      |
| **Sensor Drivers** | ✅ Use crates for `MAX30102` or write wrappers if C drivers exist                                         |
| **Async**          | ⚠️ Use only if needed (`embassy`) – test simple tasks first                                               |
| **Logging**        | ✅ Log to Flash/EEPROM, or transmit via UART/BLE                                                          |
| **RTOS?**          | ❌ Not needed unless doing complex multitasking                                                           |
| **Bindings**       | ⚠️ Only use ESP-IDF bindings if necessary (e.g. for advanced BLE). Use Rust-native crates where possible |

### Summary

| Advantages                  | Why It Matters for                     |
| --------------------------- | -------------------------------------------- |
| **Memory safety**         | Prevents fatal bugs in health monitoring     |
| **Zero-cost abstraction** | Clean code with high performance             |
| **Safe concurrency**      | Async multitasking for I/O or BLE (optional) |
| **`no_std` support**      | Bare-metal, low-power, offline-friendly      |
| **Growing HAL ecosystem** | Easier peripheral control on ESP32           |
| **Modern tooling**        | Faster development, easier debugging         |

## Day 1/4/2025: Setup esp-hal
### Overview of esp-hal
#### 1. 🧩 What is `esp-hal`?

`esp-hal` (short for **ESP Hardware Abstraction Layer**) is a Rust crate that enables safe, low-level access to ESP32-series hardware **without needing the Espressif C SDK (`esp-idf`)**. It is part of the [esp-rs](https://github.com/esp-rs) ecosystem and is designed for:

* **`no_std` embedded programming**: no OS, no dynamic memory, full control over the hardware.
* **Bare-metal development** using Rust's safety guarantees.
* **Interfacing with microcontroller peripherals** like GPIO, I2C, SPI, UART, ADC, PWM, and Timers.

For our project, which involves **reading health signals** (like SpO₂ and heart rate), `esp-hal` is ideal due to:

* **Determinism**
* **Power efficiency**
* **Low-latency response**
* **Offline-first design**

#### 2. Features of `esp-hal`

| Feature             | Description                          | Example Use in OxyRem                                   |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| **GPIO**            | Digital pins, input/output modes     | Interface with LEDs or buttons                          |
| **ADC**             | Analog-to-Digital Converter          | Read analog values from pulse sensor or SpO₂            |
| **I2C**             | Inter-IC Communication               | Communicate with SpO₂ sensors like MAX30102             |
| **UART**            | Serial communication                 | Debug logs or interface with Bluetooth/UART peripherals |
| **Timers / Delays** | Software timing and scheduling       | Sampling at fixed intervals                             |
| **PWM**             | Pulse-width modulation               | Drive a buzzer or status LED                            |
| **Interrupts**      | External and internal event handling | Respond to sensor events without polling                |

#### 3. Architecture and System Requirements

**Supported Chips**

* ESP32
* ESP32-C2, C3 (your case: RISC-V!)
* ESP32-S2, S3
* ESP32-H2

**System Traits**

* No operating system required
* Based on `embedded-hal` 1.0 traits
* Written in pure safe/unsafe Rust (`no_std`)
* Support for board-level abstraction (`esp-devices`, `esp-boards`)


#### 4. Crates & Components Used

| Crate                        | Purpose                                            |
| ---------------------------- | -------------------------------------------------- |
| `esp32c3-hal`                | HAL implementation for ESP32-C3                    |
| `embedded-hal`               | Common traits like `DigitalOutputPin`, `I2c`, etc. |
| `panic-halt` / `panic-probe` | Panic behavior in `no_std`                         |
| `critical-section`           | Manages safe critical sections (for interrupts)    |
| `espflash`                   | USB flashing and serial monitor                    |
| `esp-generate`               | Templating new projects with correct setup         |

## Day 2-5/4/2025
### Setting Up the Development Environment

**1. Install the Toolchain**

```bash
rustup install nightly
rustup default nightly

# Install espup to configure toolchains
cargo install espup
espup install
```

This installs:

* Rust target: `riscv32imc-unknown-none-elf`
* Required tools: `espflash`, `cargo-espflash`

### Create a New Project

```bash
cargo install esp-generate
esp-generate --chip esp32c3 blinky
```
This will open a screen asking you to select options:
* Select the option "Enable unstable HAL features"

Just save it by pressing "s" in the keyboard.
This generates a ready-to-use Rust project for `esp32c3` with `hal`

### Understand Project Structure

| File/Folder          | Purpose                                        |
| -------------------- | ---------------------------------------------- |
| `.cargo/config.toml` | Tells cargo to use the RISC-V target           |
| `main.rs`            | Your program entry point (`#[entry]`)          |
| `Cargo.toml`         | Dependencies like `esp32c3-hal`, panic handler |
| `build.rs`           | Setup for linking and symbol generation        |


### Building, Flashing, and Monitoring

**Flash our board:**

```bash
cargo run --release
```

* Uses USB to flash the firmware.
* `--monitor` opens a serial interface to print logs (useful for debugging).
* Use `--port` if you have multiple devices connected.

### Running Code Example: Blinking an LED

```rust
#![no_std]
#![no_main]

use esp32c3_hal::{pac, prelude::*, clock::ClockControl, delay::Delay, gpio::IO};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut led = io.pins.gpio2.into_push_pull_output();
    let mut delay = Delay::new(&clocks);

    loop {
        led.set_high().unwrap();
        delay.delay_ms(500);
        led.set_low().unwrap();
        delay.delay_ms(500);
    }
}
```
<video src="Videos/blinky.mp4" width="480" height="320" controls></video>

### Why Use `esp-hal`?

| Advantage           | Benefit for OxyRem                              |
| ------------------- | ----------------------------------------------- |
| `no_std`            | Runs independently of OS; perfect for wearables |
| Safe Rust           | Prevents memory bugs common in C/C++            |
| Peripheral Control  | Access to ADC, I2C, GPIO, UART for sensors      |
| Compile-time Checks | Ensures correctness early                       |
| Community Support   | `esp-rs` is an active and growing community     |
| Offline First       | No reliance on OS or internet to operate        |
| Flexible Flashing   | `espflash` makes deploying fast and simple      |

### Concepts in Embedded Rust

| Term                                 | Meaning                                                                          |
| ------------------------------------ | -------------------------------------------------------------------------------- |
| **Wrapper**                          | A Rust type that safely wraps a low-level register or FFI call (e.g., `GpioPin`) |
| **Binding**                          | Link between Rust code and external (C) APIs or hardware                         |
| **HAL (Hardware Abstraction Layer)** | Provides a safe interface for controlling peripherals                            |
| **PAC (Peripheral Access Crate)**    | Low-level access to registers, generated from SVD files                          |
| **RT (Runtime)**                     | Sets up startup code, interrupt vectors, etc.                                    |
| **`no_std`**                         | Rust mode without the standard library (critical for MCUs)                       |
| **critical-section**                 | Mechanism to safely handle shared resources or interrupts                        |




