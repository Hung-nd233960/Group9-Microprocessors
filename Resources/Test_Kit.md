## Testing

### Goal: Create a testing system that tests the client-side system that emulates the MAX30102

## Testing

### Goal: Create a testing system that tests the client-side system that emulates the MAX30102

This testing system aims to verify the correctness and robustness of an I²C slave device that emulates the behavior of the MAX30102 heart rate and SpO₂ sensor. The emulator replicates key register-based behaviors of the real sensor, and the tester acts as a master controller that interacts with it, just like production firmware would.

---

## Vision

The long-term goal is to build a **complete test toolchain** that:

- Allows for **hardware-in-the-loop** testing of embedded firmware without requiring actual sensor hardware.
- Supports **simulation of known input waveforms** and validates how the firmware interprets them.
- Enables a **computer-driven testing harness** (e.g., via UART, USB, or GUI) to script test cases, monitor results, and flag regressions.
- Mimics full-device behavior including:
  - Configuration registers
  - FIFO reads
  - Part ID checking
  - Mode transitions

By decoupling the firmware development from the physical sensor, developers can:
- Test firmware even before hardware arrives.
- Run reproducible, automated validation loops (e.g., CI for microcontrollers).
- Simulate edge cases like FIFO overflow, faulty configurations, or flat-line signals.

---

### Why Arduino, not Rust or MicroPython for testing?

Arduino was chosen as the development platform for the tester and emulator **because it offers the fastest and most reliable development path** — especially for implementing **I²C slave devices**, which is critical in this project.

#### ✅ Development Speed vs Effort Ratio
When building tools like a MAX30102 emulator or a testing controller, the **goal is rapid iteration**: test, break, tweak, verify. Arduino’s ecosystem provides:
- **Minimal setup** — plug in, flash, and go
- Immediate feedback via `Serial.println`
- A single programming model across AVR, ESP, and RP2040 boards
- Intuitive APIs like `Wire.begin(address)` and `onReceive()` that abstract I²C slave complexity

This allows you to go from concept to working prototype in **minutes**, rather than hours or days.

#### ✅ Only Platform with Stable I²C Slave Support

| Feature | Arduino | Rust | MicroPython |
|--------|---------|------|-------------|
| I²C Master | ✅ Stable | ✅ Stable | ✅ Stable |
| **I²C Slave** | ✅ Works across AVR, ESP32, RP2040 | ❌ Limited or unsupported | ❌ Not available or unreliable |
| ISR-style receive handlers | ✅ `onReceive()` | ⚠️ Not idiomatic or portable | ❌ Not available |
| Community examples | ✅ Tons | ⚠️ Sparse | ⚠️ Very few |

- In **Rust**, implementing an I²C slave generally requires:
  - Peripheral-level programming using PACs (Peripheral Access Crates)
  - Custom interrupt handlers
  - Unsupported or unfinished driver crates (especially for async slave)

- In **MicroPython**, I²C slave is **not implemented** or **not stable** on most platforms, especially ESP32 and RP2040.

In contrast, **Arduino just works** — with a single `Wire.onReceive()` callback and consistent behavior across architectures.

#### ✅ Maximal Value, Minimal Friction

For this project — where the emulator is a **test scaffold** and not the final product — Arduino delivers:
- The **shortest path to working hardware**
- **Reliable I²C protocol compliance**
- Enough flexibility to emulate registers, handle multi-byte reads, and simulate FIFO behavior

This lets the focus remain on **testing the firmware-under-test**, not debugging the emulator’s infrastructure.

---

### Summary

Arduino is not the most modern platform, but it is:
- ✅ The **fastest to build and debug** on
- ✅ The **only platform with mature I²C slave support**
- ✅ The most reliable choice for prototyping a hardware emulator or tester

Once the infrastructure is stable, **Rust can be used where its strengths matter** — such as firmware correctness, memory safety, and asynchronous control. But for testing and mocking? Arduino wins by simplicity.

## Theory of Operation

### MAX30102 Communication Overview

The MAX30102 communicates using the **I²C protocol**, where it acts as a **slave** with 7-bit address `0x57`.

#### I²C Transactions for MAX30102:
- **Write**:
  - `[START] → [0x57 << 1 | 0] → [register address] → [data] → [STOP]`
- **Read**:
  - `[START] → [0x57 << 1 | 0] → [register address] → [RESTART] → [0x57 << 1 | 1] → [data bytes] → [STOP]`

#### Important Registers Emulated:
| Register | Name                 | Behavior                                  |
| -------- | -------------------- | ----------------------------------------- |
| `0xFF`   | PART_ID              | Returns `0x15` to identify the device     |
| `0x04`   | FIFO_WR_PTR          | Write pointer for FIFO buffer             |
| `0x05`   | OVF_COUNTER          | Counter for FIFO overflow events          |
| `0x06`   | FIFO_RD_PTR          | Read pointer                              |
| `0x07`   | FIFO_DATA            | 3-byte FIFO samples, 18-bit packed values |
| `0x09`   | MODE_CONFIG          | Mode select (not fully implemented)       |
| `0x0A`   | SPO2_CONFIG          | Sampling config (placeholder)             |
| `0x0C`   | LED1_PA (IR current) | LED current control (placeholder)         |

---

## Code Structure

### Emulator (UNO as I²C Slave)

- Emulates the MAX30102 over I²C address `0x57`
- Handles reads/writes for relevant registers
- Populates FIFO with random (or injected) 18-bit sample data
- Supports protocol-compliant FIFO read behavior

```cpp
Wire.begin(0x57);         // Start as I2C slave
Wire.onReceive(...);      // Handle register writes
Wire.onRequest(...);      // Respond with register or FIFO content
```

### Tester (ESP32 or Arduino as I²C Master)

- Sends `write` commands to configure or request registers
- Reads back responses to verify emulator behavior
- Reconstructs and prints 18-bit FIFO samples
    

```cpp
Wire.beginTransmission(0x57); Wire.write(0xFF); // PART_ID Wire.endTransmission(false); Wire.requestFrom(0x57, 1);`
```


➡️ See: `Max30102 Tester.ino`

---

## Example: FIFO Read Trace

When the master requests a FIFO sample:

```cpp
`Wire.write(0x07);             // FIFO_DATA register Wire.requestFrom(0x57, 3);    // Request 3 bytes`
```
The emulator returns one sample:

|Byte|Content|Notes|
|---|---|---|
|Byte 1|`xx000000`|Top 2 bits of 18-bit sample|
|Byte 2|Full middle byte|Sample bits 15–8|
|Byte 3|Full lower byte|Sample bits 7–0|

This matches the official MAX30102 datasheet format.

---

## Future Improvements

- Inject real waveform data (CSV or serial input)
- Simulate interrupts (e.g., data ready)
- Add SpO₂ dual-channel simulation (RED/IR)
- Automate test reports (via Python or PC GUI)
- Allow waveform “modes” (e.g., flatline, sinusoidal)
    

---

## Summary

This testing framework provides a solid foundation for validating embedded firmware that talks to the MAX30102. By emulating I²C behavior faithfully and building a real-time tester, the system offers robustness, reproducibility, and deeper understanding — even without the real sensor.

