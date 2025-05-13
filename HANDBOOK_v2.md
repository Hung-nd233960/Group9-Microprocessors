# Engineering Handbook

## Overall Information

This handbook serves as a comprehensive guide for the engineering principles, methodologies, and technical details involved in this project. It is structured to facilitate understanding for both new and experienced engineers, providing insights into the development, implementation, and operation of the system.

The handbook is designed to be read sequentially, but each chapter can also be referenced independently as needed.

## How To Read This Handbook

This handbook is written in Markdown format, which is a lightweight markup language that makes it easy to structure and style text. For an optimal reading experience, it is recommended to use a dedicated Markdown reader such as Obsidian or Visual Studio Code with a Markdown plugin. These tools will provide features like easy navigation, links, and proper rendering of the content.

If you want to view images or any local content referenced in the handbook, make sure to clone the repository where this handbook is stored. This will allow you to access any embedded images or files linked within the document. If you're new to Markdown, most text editors and viewers can also display it correctly, but using a specialized tool will enhance your experience.

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

In Basics 1, we will learn about basics of Rust: Cargo, Variables, Data Types, Functions, Control Flow, Enum, Structs

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

Embassy is a project to make async/await a first-class option for embedded development.
Soldering is a needed skill for every electronics engineer.

Your task is to: 
- Understand Embassy: How it achieves async without RTOS. What are executors. How to config your peripherals to work async with Embassy.
- Learn soldering: How to solder, choosing the right tool, temperature and solder basic GPIO pins on the lab.

More details is in the file [Rust_Embed_02](Resources/Rust_Embed_02)

## Week 7: Serial Communication Protocols: I2C, SPI, UART. 


# Phase 2: Design and Create


## Week 8: System Design 1: What to Include?


## Week 9: Sensors: MAX30102, MPU6050


## Week 10: Components: ESP32-C3, ESP32-C6, SSD1306

# Phase 3: Report and Testing

## Week 12: Assemble Together. Testing 1. Lab Work 2


## Week 13: Report 1. Testing 2


## Week 14: Report 2


## Week 15: Report 3 and Conclusion