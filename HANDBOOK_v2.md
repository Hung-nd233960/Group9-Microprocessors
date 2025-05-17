# Engineering Handbook

## Overall Information

This handbook serves as a comprehensive guide for the engineering principles, methodologies, and technical details involved in this project. It is structured to facilitate understanding for both new and experienced engineers, providing insights into the development, implementation, and operation of the system.

The handbook is designed to be read sequentially, but each chapter can also be referenced independently as needed.

## How To Read This Handbook

This handbook is written in Markdown format, which is a lightweight markup language that makes it easy to structure and style text. For an optimal reading experience, it is recommended to use a dedicated Markdown reader such as Obsidian or Visual Studio Code with a Markdown plugin. These tools will provide features like easy navigation, links, and proper rendering of the content.

If you want to view images or any local content referenced in the handbook, make sure to clone the repository where this handbook is stored. This will allow you to access any embedded images or files linked within the document. If you're new to Markdown, most text editors and viewers can also display it correctly, but using a specialized tool will enhance your experience.


# Table Of Contents

### [Week 12: Assemble Together. Lab Work 2](#week-12-assemble-together-1-lab-work-2)


[[#Week 1: Team-Forming, Workflow and Topic]]
[[#Week 2 Ideas and Vision]]
[[#Week 3: Rust Basics 1]]
[[#Week 4 Rust Basics 2]]
[[#Week 12: Assemble Together 1. Lab Work 2]]





# Phase 1: Preparation
## Week 1: Team-Forming, Workflow and Topic

### Date: Mar 03, 2025

#### a. Group 9:
- Consists of 3 members
#### b. Workflow and Rules
- Summary:
	- All group work resides in the common public GitHub repository
	- Messages via Messenger
	- Meetings through Teams
	- Offline meetings in Lab when needed.
- More details in the file [WORKFLOW](Resources/WORKFLOW.md)
#### c. Topic

#### d. Conclusion

## Week 2: Ideas and Vision
#### Date: Mar 10, 2025

## Week 3: Rust Basics 1
#### Date: Mar 17, 2025

#### Task: Learn the basics of Rust. Part 1: The Basics

#### a. Rust the Programming Language

Rust is a general-purpose programming language emphasizing performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory.

In Basics 1, we will learn about basics of Rust: Cargo, Variables, Data Types, Functions, Control Flow, Enums, Structs

The task for the week is to:

- Understand the basics of Rust
- How to Install Rust environment on machine, how to work with Cargo, rustup, setup in VSCode, Extensions
- Write a small code that is "guessing game", guessing a number from 1 to 10 in command line
- Compare C, Rust and Python by adding 1 million times.

Please document everything you done, even the installation of Rust, environment

#### b. Conclusion:

More details in the file [Rust_01](Resources/Rust_01.md)
## Week 4: Rust Basics 2

#### Date:  Mar 24, 2025

#### a. Borrow Checker, Ownership

Ownership is a discipline for ensuring the **safety** of Rust programs. Ownership In Rust, **each value has a single owner**, and when that owner goes out of scope, the value is **automatically dropped** (freed).

In researching in Rust Ownership system, you should understand:
- How memory is managed in Rust (or any computer system): the Heap, the Stack, how variables is store and references
- Some other forms of memory manipulation: malloc(), pointers in C
- Key rules of Ownership, what Ownership is. Aliasing, Mutation, borrow checker enforce permissions
- Slice Type

Your task is to note down what you have learned, and include relevant diagrams showing reference conditions and connections when necessary in examples.
#### b. Concurrency and async/await

**Concurrency** is a discipline for ensuring the **correctness and performance** of programs that execute **multiple tasks in overlapping time**. In Rust, concurrency is designed to be **memory-safe and race-free** at compile time, thanks to the **ownership system and the borrow checker**.

In researching concurrency in Rust, you should understand:

- What **threads** are (OS-level or user-level), and how they share CPU cores.
- The difference between:
* Concurrency (task switching)
	* Parallelism (actual simultaneous execution on multiple cores)
	* Asynchronous programming (non-blocking I/O, task suspension)
- What **data races** are, and why languages like C++ and Java need locks/mutexes to avoid them.
- Why async/await is important for microprocessors than true core parallelism. How to do async/await in Tokio.

Your task is to note down what you have learned.
#### c. Conclusion

More details in the file [Rust_02](Resources/Rust_02.md)
## Week 5: Rust Embedded 1: esp-hal, no-std
#### Date: Mar 31, 2025

#### a. Overall Development Solutions for ESP

ESP microcontrollers by Espressif support multiple development environments for different needs. **ESP-IDF** is the official C/C++ SDK for low-level, full-featured development. **Arduino-ESP32** simplifies programming with the Arduino API in C++. **MicroPython** and **CircuitPython** offer easy scripting with Python, ideal for rapid prototyping. Each environment balances control, performance, and ease of use depending on the project.

Overall Embedded Rust development space have project embedded-hal and embedded-hal-async standard

For Rust, esp-rs provide two main paths for developing in Rust:
- Native: hardware abstraction layers for ESP directly written in Rust: there are esp-hal for no-std and std (esp-idf-hal)
- Wrappers that abstracts away C of ESP-IDF in Rust. For example, esp-idf-sys.

Your task is to:

- Evaluate why Rust is superior to other languages in embedded environment
- Give definitions for wrappers, bindings, hardware abstraction layers embedded programming.
- Consider what should be used in the scope of the project, given the constraints in project time, complexity and effort.
#### b. Set up esp-hal

esp-hal is a no-std bare metal hardware abstraction layer for ESP processors in Rust.

Your task is to:
- Overview on crate esp-hal: What functions does it serve, what settings, system can it uses
- Showing how to setup environment for esp-hal development, you should show how to include to Cargo, how to set up new projects with esp-generate and flash with espflash, cargo espflash workflow.
#### c. Conclusion

 Given project complexity and limitation of time, effort. Development no-std with esp-hal is chosen as the framework for the project.

More details is in the file [Rust_Embed_01](Resources/Rust_Embed_01.md)
## Week 6: Rust Embedded 2: Embassy Framework + Lab Work 1: Soldering Basics

### Date: Apr 7, 2025

Embassy is a project to make async/await a first-class option for embedded development.
Soldering is a needed skill for every electronics engineer.

Your task is to: 
- Understand Embassy: How it achieves async without RTOS. What are executors. How to config your peripherals to work async with Embassy.
- Learn soldering: How to solder, choosing the right tool, temperature and solder basic GPIO pins on the lab.

#### c. Conclusion

More details is in the file [Rust_Embed_02](Resources/Rust_Embed_02)

## Week 7: Serial Communication Protocols: I2C, SPI, UART. 

### Date: Apr 14, 2025

This week focuses on the fundamental ways embedded devices talk to each other. These protocols form the backbone of sensor interfacing, memory communication, and peripheral control in microcontroller systems.

Your task is to:

- Learn the theory behind each protocol: how I2C handles addressing, how SPI ensures speed through simplicity, and why UART remains ubiquitous.
- Analyze their timing, signaling, and wiring differences.
- Practice: use a microcontroller to interface with at least one I2C sensor and one SPI device. Observe signal traces with a logic analyzer or oscilloscope.

#### Conclusion

More details is in the file [CommProtocol.md](Resources/CommProtocol.md)

# Phase 2: Design and Create

## Week 8: System Design 1: What to Include?

### Date: Apr 21, 2025

This week shifts from isolated components to thinking like a system designer. You’ll begin architecting a complete embedded device—from sensing to decision-making to user interaction.

Your task is to:

* Define the **hardware stack**: Choose your MCU , power supply, input/output components, sensors (e.g., temperature, IMU, light), and user interface elements like buttons or a small display (e.g., OLED or TFT screen).
* Identify the **software stack**: What operating model are you using—bare metal, RTIC, or async with Embassy? What crates/libraries are needed for drivers, HALs, or communication layers?
* Map the system in terms of **data flow**: Where does the data originate, how is it processed, and where does it go (e.g., display, logs, actuators)?
* Create **flowcharts** for user interactions or sensor events.
* Draw **software diagrams** to clarify module responsibilities, message passing, and layers of abstraction.
* Design **finite state machines (FSMs)** to handle device modes (e.g., idle, collecting, error, transmitting).

By the end of the week, you should have a clear top-down plan of your system—hardware and software—that will guide implementation in future labs. 
#### Conclusion

More details is in the file [System_Design_01](Resources/System_Design_01)

## Week 9: Sensors: MAX30102, MPU6050

### Date: Apr 28, 2025

This week focuses on working with real-world sensors—capturing meaningful data from the physical environment and integrating it into your system.

Your task is to:

- Understand how the **MAX30102** works: its use in pulse oximetry and heart rate monitoring, I2C interface, and how to configure it for reliable readings. Learn how to read raw IR/Red data and filter it for usable heart rate or SpO2 estimation.
- Explore the **MPU6050**: a 6-axis IMU combining accelerometer and gyroscope. Learn about register configuration, scaling factors, and how to interpret motion data.
- Integrate both sensors into your system via I2C.
- Test and visualize sensor data on a screen or via serial logging.
- Research and note about the library [max3010x](https://docs.rs/max3010x/latest/max3010x/) library and [mpu6050](https://docs.rs/mpu6050/latest/mpu6050/) library, what functions do they support, how to do sensor reading.
- Consider the possibilities of algorithms and tools to calculate, save and process sensor data. Kalman filters are recommended. 

Extend your system diagrams to include sensor data flow and processing stages. Document your signal processing steps and update your FSMs to include sensor-driven state transitions.
#### Conclusion

More details is in the file [Sensors](Resources/Sensors)
## Week 10: Components: ESP32-C3, ESP32-C6, SSD1306

### Date: May 05, 2025

#### a. Microcontrollers: ESP32-C3, ESP32-C6

This week focuses on key components you’ll likely use in your final embedded system design: microcontrollers and displays. You'll get hands-on with the **ESP32-C3** and **ESP32-C6**, two modern, RISC-V-based Wi-Fi/Bluetooth MCUs from Espressif, and the **SSD1306**, a widely used I2C OLED display controller.

Your task is to:

- Explore the **ESP32-C3** and **C6**: understand their core differences (e.g., RISC-V vs Xtensa, BLE/Wi-Fi capabilities, peripherals), and how to flash Rust firmware using tools like `espflash`.
- Set up a minimal Rust application using the `esp-hal` and Embassy (or bare-metal if preferred). Blink an LED, log over serial, and initialize I2C.
- Update your system diagrams and FSMs to include display logic (e.g., idle screen, error messages, real-time sensor output).
- Compare power consumption, peripheral support, and async compatibility between the ESP32-C3 and C6—decide which fits your final project best. Evaluate which pins support I2C, SPI, UART.
#### b. OLED Screen: SSD1306

- Understand the workings of SSD1306 and its Rust driver: How does images get encoded, transferred? What is Buffered Graphics?
- Buffered Graphics of SSD1306 are compatible with another popular graphics library: embedded_graphics. Please explain the functionalities of the library and how to design from it.
- Integrate the **SSD1306** OLED display: initialize over I2C, draw text and graphics, and build a reusable screen abstraction.
#### c. Conclusion

More details is in the file [Controller](Resources/controller.md)
# Phase 3: Report and Testing

## Week 12: Assemble Together 1. Lab Work 2

#### a. In Software


#### b. In Hardware


#### c. How to make things work together

#### d. Conclusion

### Date: May 12, 2025
## Week 13: Assemble Together 2. Report 1. Testing 1

#### a. Report Standards, Preparation

#### b. Preparation

#### c. Conclusion
### Date: May 19, 2025
## Week 14: Assemble Together 3. Report 2. Testing 2
### Date: May 26, 2025

#### a. Packaging

#### b. Toolchain

#### c. Slides

#### d. Conclusion
## Week 15: Report 3 and Conclusion

### Date: Jun 02, 2025

#### a. Ratings, evaluation, compliance.
#### b. Key ideas found

#### c. Scrapped Ideas

#### d. Beyond Microprocessors

#### e. Conclusion


