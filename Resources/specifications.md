## 🛠️ Mechanical Design Considerations

The mechanical design plays a critical role in ensuring the device is **comfortable, durable, functional, and repairable**. As a wearable system, it must be lightweight, non-intrusive, and allow reliable contact between the SpO₂ sensor and the user's skin.

---

## 🧊 1. Enclosure Design

### 📦 Materials

- **Primary:** 3D-printed PLA or PETG (recycled PETG recommended)
- **Alternative:** Biodegradable resin or injection-molded ABS (for larger batches)
- **Reasoning:** PLA is easy to prototype and safe on skin, PETG for better strength and temperature resistance

### 🛠️ Assembly

- **Modular case** with screw or snap-fit design
- **Sensor area:** Must be **open to skin** or **covered with optical window** (e.g., thin transparent silicone or acrylic)
- **Battery and board access:** Removable back panel for repairs

---

## 🖐️ 2. Ergonomics

- **Target Use:** Worn on wrist or ankle.
- **Strap Compatibility:** Standard 22mm watch strap lugs or built-in flexible loop
- **Weight Goal:** < 50g total
- **Curved Base (Optional):** Follows wrist contour for comfort

---

## 💨 3. Sensor Positioning & Skin Contact

- **SpO₂ Sensor Window:**
  - Mounted flush against skin
  - Light-isolated from ambient light (important for signal accuracy)
  - Optional foam/silicone ring around sensor to block light leakage

- **Design Considerations:**
  - Place sensor module in a **cutout** with light baffle walls
  - Use **semi-flexible base** or slight spring mount for better skin contact

---

## 🔋 4. Battery & Heat Considerations

- **Battery Compartment:** Isolated chamber with thermal clearance from MCU
- **Thermal Management:**
  - ESP32 can get warm during Wi-Fi bursts — heat should not transfer to skin
  - Use thermal pads or copper trace plane near ESP32 to spread heat away from contact zone

---

## 🧰 5. Repairability & Serviceability

- **Access Points:**
  - Separate battery and board sections for easier troubleshooting
  - Use **standard screws** (e.g., M2 or M2.5) instead of glue
  - **Connectors, not soldered joints**, for battery, display, and sensor

- **Labeling:** Consider silk-screened or embossed pinouts on inner shell

---

## 🧲 6. Mounting & Modularity

- **Optional Rails / Slots** for additional boards (e.g., microSD, accelerometer)
- **Hidden Debug Port:** UART access hole for reprogramming without teardown
- **Magnetic Charging (Optional):** Embedded magnets for easy dock recharge

---

## 📐 7. Draft Dimensions (Target)

| Component      | Target Dimensions          |
| -------------- | -------------------------- |
| PCB            | ~25 × 50 mm                |
| Total Device   | ~30 × 55 × 15 mm           |
| OLED Cutout    | ~10 × 30 mm                |
| Sensor Window  | ~10 × 10 mm (skin contact) |
| Battery Cavity | Fits 501235 Li-Po (~1 Wh)  |

---

## 🎨 8. Visual & Aesthetic

- **Minimalist enclosure** with rounded corners
- **Textured or matte surface** to reduce scratches
- **Color choices:** Neutral tones, optionally color-coded for sensors

---

> 🧩 3D design will be done in **SolidWorks**, **Fusion360**, or **FreeCAD**, exported as STL/STEP files for printing or CNC.

---

## 📎 Summary: Mechanical Design Goals

| Feature              | Implementation Target                  |
| -------------------- | -------------------------------------- |
| Comfort & Ergonomics | ✅ Wrist-mount, low weight              |
| Repairability        | ✅ Screw-open + modular                 |
| Sensor Accuracy      | ✅ Light-blocked & skin contact ensured |
| Heat Isolation       | ✅ MCU thermally decoupled              |
| Case Modularity      | ✅ Battery + board separated            |
| Aesthetic & Clean UI | ✅ Cutouts for OLED & ports             |
|                      |                                        |
#### Reliability

**Reliability** is defined as the ability of the system to consistently provide accurate SpO₂ readings, maintain stable operation, and ensure robust communication over its intended operational lifespan.

---

## ✅ Key Reliability Dimensions

### 1. 📊 Data Accuracy & Precision (Medical Reliability)

- **Definition:** Closeness of measured SpO₂ values to a clinical-grade reference.
- **Metrics:**
  - Mean Absolute Error (MAE)
  - Root Mean Squared Error (RMSE)
  - Correlation Coefficient (r)
- **Targets:**
  - RMSE < 2%
  - MAE < 1.5% for 90–100% SpO₂ range
  - MAE < 3% for <90% SpO₂ range

---

### 2. ⏱️ Uptime / Availability

- **Definition:** Percentage of total time the device operates without failure.
- **Metrics:**
  - Uptime % = (Total Time − Downtime) / Total Time × 100
  - Mean Time Between Failures (MTBF)
- **Target:** > 99% uptime

---

### 3. 📥 Sensor Read Success Rate

- **Definition:** Frequency of successful SpO₂ readings without error.
- **Metric:**  
  - Valid Read Rate = Valid Reads / Total Read Attempts
- **Target:** > 95%

---

### 4. 🌐 Communication Reliability

- **Definition:** Consistency and integrity of data sent to server/local dashboard.
- **Metrics:**
  - Packet Loss Rate (%)
  - Retry Count
  - Transmission Latency (ms)
- **Targets:**
  - Packet Loss < 1%
  - Latency < 1s (real-time) or < 10s (periodic)

---

### 5. 🔋 Power Reliability

- **Definition:** Duration of operation per battery cycle and stability of power.
- **Metrics:**
  - Battery Life (hours/days)
  - Power Consumption (mW/hour)
- **Target:** > 24–48 hours per charge. Realistically 8-10 hours
- Power: 0.1 to 0.05W

---

### 6. ⚠️ Fault Tolerance

- **Definition:** System’s ability to detect, handle, and recover from errors.
- **Metrics:**
  - Recovery Time after Failure (RTO)
  - Error Detection Coverage (%)
- **Targets:**
  - RTO < 30 seconds
  - Automatic recovery from sensor/Wi-Fi faults

---

## 🖥️ Optional: Reliability Dashboard Elements

- Battery percentage & estimated time left
- SpO₂ read success/failure logs
- Communication latency & error counts
- Packet loss history
- Device uptime stats
- Fault event log (e.g., sensor disconnects, Wi-Fi dropouts)

---

> ℹ️ **Note:** These reliability metrics should be monitored continuously or logged at regular intervals for quality assurance and debugging purposes.

#### Sustainability

---

## 🧭 Scope of Sustainability

This section outlines how sustainability is built into the **design, usage, and maintenance** of the system across several dimensions:

---

### 🔋 Power Efficiency

- **Target Draw:** Average power consumption of **50–100 mW**
- **Strategies:**
  - Deep sleep modes between reads
  - Short bursts for Wi-Fi transmission
  - Minimal OLED usage (on-demand UI)
- **Impact:** Reduces overall energy usage and charging frequency

---

### 🔋 Battery Sustainability

- **Battery:** Rechargeable 1 Wh lithium-polymer cell
- **Runtime Goal:** > 3 days per full charge under typical usage
- **Cycle Optimization:** Encourage shallow charge/discharge (20–80%) for extended life
- **Charging Interface:** USB-C for convenience and universal compatibility

---

### ♻️ Recharge Cycle & Longevity

- **Expected Lifespan:** > 500 full charge cycles
- **Practices:**
  - Smart sleep modes to limit energy drain
  - Battery status indicators to promote optimal charging habits

---

### 💸 Cost & Affordability

- **Design Philosophy:** Use common, inexpensive components that are easy to source and replace
- **Board & Component Reusability:** Modular design wherever possible (sensors, display, battery)

---

### 🛠️ Easy Repair & Replace

- **Modular Parts:** Sensor, screen, and battery can be swapped
- **No Solder-Only Dependencies:** Use of headers, JST connectors, or sockets where possible
- **Documentation:** Clear assembly and troubleshooting guides provided

---

### 📦 Packaging (If Applicable)

- **Eco-Friendly Recommendation:**
  - Recycled/recyclable paper-based packaging
  - Avoid plastic foam and excess wrapping
- **Optional Case:** 3D-printable casing in PLA or recycled PETG

---

### 🛰️ OTA Updates & Maintenance

- **Firmware Updatability:** Optional OTA or simple USB reflash supported
- **Open-Source Firmware:** Maintained in Git-based version control
- **Fail-Safe Bootloader:** Prevents bricking during update failures

---

### 💾 Local Processing & Dashboard

- **Local Data Handling:** Processes and stores readings locally to minimize dependency on cloud services
- **Dashboard:** Self-hosted on device or LAN — no external servers required
- **Interface:** Accessible via local web interface (no external apps needed)

---

### 🚫 No Proprietary Apps

- **No App Store Dependency:** The device is usable via browser on any phone/computer
- **No Forced Expiry:** Doesn’t rely on third-party software that may become deprecated

---

## ✅ Summary of Sustainable Practices

| Feature              | Implementation Status              |
| -------------------- | ---------------------------------- |
| Power Efficiency     | ✅ Deep sleep + smart wake-up       |
| Battery Longevity    | ✅ Rechargeable, >500 cycles        |
| OTA / Local Updates  | ✅ USB / OTA via web tools          |
| Replaceable Parts    | ✅ Modular design for sensor & more |
| Local Dashboard      | ✅ Web UI, no app needed            |
| Cloud-Free Operation | ✅ Local processing & storage       |
| Packaging (optional) | ♻️ Eco-friendly suggestion made     |

> ♻️ This project aims to minimize e-waste, energy use, and software obsolescence — creating a low-maintenance, user-friendly health device that respects both users and the environment.
