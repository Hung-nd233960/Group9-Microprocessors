# Engineering Handbook

## Overall Information

This handbook serves as a comprehensive guide for the engineering principles, methodologies, and technical details involved in this project. It is structured to facilitate understanding for both new and experienced engineers, providing insights into the development, implementation, and operation of the system.

The handbook is designed to be read sequentially, but each chapter can also be referenced independently as needed.

## How To Read This Handbook

This handbook is written in Markdown format, which is a lightweight markup language that makes it easy to structure and style text. For an optimal reading experience, it is recommended to use a dedicated Markdown reader such as Obsidian or Visual Studio Code with a Markdown plugin. These tools will provide features like easy navigation, links, and proper rendering of the content.

If you want to view images or any local content referenced in the handbook, make sure to clone the repository where this handbook is stored. This will allow you to access any embedded images or files linked within the document. If you're new to Markdown, most text editors and viewers can also display it correctly, but using a specialized tool will enhance your experience.

## Table of Contents

1. **[Introduction](#introduction)**  
   a. [Overview of the Project](#overview-of-the-project)  
   b. [Purpose of This Handbook](#purpose-of-this-handbook)  
   c. [How to Use This Handbook](#how-to-use-this-handbook)  

2. **[Project Background](#project-background)**  
   a. [Project Scope](#project-scope)  
   b. [Objectives and Goals](#objectives-and-goals)  
   c. [Technical Challenges](#technical-challenges)  
   - [Overview](#overview)  
   - [Soldering](#soldering)  
   - [Hardware](#hardware)  
   - [Software](#software)  

   d. [Design Considerations](#design-considerations)  
   - [Overall Considerations](#overall-considerations)  
   - [Initial Setup and Pairing](#initial-setup-and-pairing)  
   - [Hardware Design](#hardware-design)  
   - [Security and Safety](#security-and-safety)  
   - [Reliability](#reliability)  
   - [Sustainability](#sustainability)  

   e. [Development Methodology](#development-methodology)  
   - [Workflow](#workflow)  
   - [Tools](#tools)  
   - [Results](#results)  
   - [Timeline](#timeline)
3. [Design Process](#design-process)

---

## Introduction

### Overview of the Project

(Description here...)

### Purpose of This Handbook

(Description here...)

### How to Use This Handbook

(Description here...)

## Project Background

### Project Scope

#### Week 1, 2, 3

## **INVESTIGATION ABOUT THE NECESSITY OF HR, SpO2, LOCATION AND STEP TRACKING** 🩺

## **📢 THE IMPORTANCE OF MEASURING HEART RATE (HR) AND BLOOD OXYGEN SATURATION (SPO₂)**

Heart rate (HR) and blood oxygen saturation (SpO₂) are two vital indicators of overall health, particularly in monitoring **cardiovascular and respiratory functions**. Measuring these parameters provides critical insights into a person's well-being and can help detect potential health issues before they become severe.  

### **What are Heart Rate (HR) and Blood Oxygen Saturation (SpO₂)?**  

- **Heart Rate (HR)** refers to the number of times the heart beats per minute (**bpm**). A normal resting heart rate for adults typically ranges from **60 to 100 bpm**.  
- **Blood Oxygen Saturation (SpO₂)** is the percentage of oxygen in the blood that is carried by red blood cells. A healthy SpO₂ level usually falls between **95% and 100%**.

---

### **Roles of HR and SpO₂ in Health Monitoring**  

HR and SpO₂ are **critical indicators of cardiovascular and respiratory health**:  

- **HR Monitoring** helps assess heart function, detect arrhythmias, and monitor stress or fitness levels.  
- **SpO₂ Monitoring** ensures that the body is receiving enough oxygen, which is essential for brain and organ function. Low SpO₂ may indicate conditions like **lung disease, respiratory infections, or circulation problems**.  
- Regular monitoring of these parameters is especially useful for **patients with heart conditions, respiratory diseases (such as COPD or pneumonia), and individuals recovering from infections like COVID-19**.  

---

### **OxyRem vs. Commercial Devices**  

| **Feature**                      | **OxyRem**                                    | **Xiaomi Mi Band 9**           | **Apple Watch Series 9**                   | **Masimo MightySat (Medical-Grade)**        |
| -------------------------------- | --------------------------------------------- | ------------------------------ | ------------------------------------------ | ------------------------------------------- |
| **Heart Rate (HR) Monitoring**   | ✅ Yes (optimized for accuracy)                | ✅ Yes                          | ✅ Yes (ECG & advanced sensors)             | ✅ Yes (medical-grade)                       |
| **SpO₂ Monitoring**              | ✅ Yes (real-time tracking)                    | ✅ Yes                          | ✅ Yes (but depends on fit)                 | ✅ Yes (high precision)                      |
| **Alert Warnings**               | ✅ Yes (custom warning system for HR & SpO₂)   | ❌ No                           | ✅ Yes (via app notifications)              | ✅ Yes (medical alerts)                      |
| **Location & Footstep Tracking** | ✅ Yes (built-in tracking)                     | ✅ Yes (fitness-based tracking) | ✅ Yes (GPS & activity tracking)            | ❌ No                                        |
| **Emergency Alarm Speaker**      | ✅ Yes (built-in speaker for critical alerts)  | ❌ No                           | ⚠️ Limited (only app notifications)         | ❌ No                                        |
| **Battery Life**                 | ✅ Long-lasting (optimized power usage)        | ⚠️ 14-16 days                   | ❌ 18-24 hours                              | ✅ Long (but only for SpO₂ tracking)         |
| **No Bloatware**                 | ✅ Yes (efficient & focused on core functions) | ❌ No (fitness-focused apps)    | ❌ No (many extra features drain power)     | ✅ Yes (medical focus, but limited features) |
| **Security**                     | ✅ High (encrypted, privacy-focused)           | ⚠️ Basic (data shared with app) | ❌ Medium (integrated with Apple ecosystem) | ✅ High (hospital-grade)                     |
| **Cost**                         | ✅ <$30 (affordable for all)                   | ➖ ~$50-55                      | ❌ $399+ (premium pricing)                  | ❌ $250+ (for hospitals & professionals)     |

### **What Makes OxyRem Stand Out?**  

- **Affordable & focused on critical health tracking** – Unlike smartwatches that include extra (and sometimes unnecessary) features.  
- **Emergency alarm system** – A **life-saving function** that fitness trackers and even some medical-grade devices lack.  
- **Long battery life with no bloatware** – Unlike smartwatches that require **frequent charging**, OxyRem maximizes usability.  
- **Secure & privacy-focused** – Data is stored **safely** without unnecessary third-party access.  

### **Common Devices for Measuring HR and SpO₂**  

Several devices are used to track HR and SpO₂, including:  

- **Pulse Oximeters** – Small, non-invasive devices that clip onto a finger to measure SpO₂ and HR.  
- **Smartwatches and Fitness Trackers** – Wearable devices that continuously monitor HR and SpO₂, providing real-time data.  
- **Medical Monitors** – Used in hospitals and clinics for precise monitoring of vital signs, often connected to other diagnostic tools.  

Understanding these measurements and using the right devices can help individuals **detect early warning signs of health issues and take proactive steps to improve their well-being**.  

---

## **💥IMPACT OF CARDIOVASCULAR AND RESPIRATORY DISEASES IN VIETNAM INCORPORATING DETAILED DATA FROM VARIOUS SOURCES**

### **💔 Cardiovascular Diseases (CVDs) in Vietnam**

- **Prevalence and Mortality**:
  - In 2019, approximately **2.4 million individuals** in Vietnam were living with cardiovascular diseases, with **65%** of these cases attributed to atherosclerosis.
  - Cardiovascular diseases accounted for **31%** of all deaths in Vietnam in 2016.

- **Specific Conditions**:
  - **Stroke**: Leading cause of death in Vietnam, with around **200,000 new cases annually**; approximately **50%** of these cases are fatal.
  - **Ischemic Heart Disease**: Consistently among the top causes of death, with disability-adjusted life-years (DALYs) per 100,000 individuals rising steadily since 2009, reaching **1,569** in 2019.

- **Risk Factors**:
  - **Hypertension**: Prevalence among adults aged 18-69 years is **18.9%**, with only **13.6%** of these individuals receiving management at health facilities.
  - **High Cholesterol**: Approximately **13.1%** of the population has raised total cholesterol levels (≥5.0 mmol/L).
  - **Physical Inactivity**: About **28.7%** of adults are insufficiently active, engaging in less than 150 minutes of moderate-intensity physical activity per week.

- **Age and Gender Disparities**:
  - The prevalence of having at least two metabolic risk factors is **28%** in women and **32%** in men.
  - High overall CVD risk (≥20% over 10 years) is observed in **20%** of men and **5%** of women, particularly at older ages.

---

### **🤒 Respiratory Diseases in Vietnam**

- **Chronic Obstructive Pulmonary Disease (COPD)**:
  - Overall prevalence is approximately **6.9%**, with higher rates in men (**12.9%**) compared to women (**4.4%**). Notably, urban areas in Vietnam report a higher prevalence (**11.1%**) than rural areas.
  - A significant proportion (**94%**) of COPD cases were previously undiagnosed, highlighting the need for improved screening and awareness.

- **Asthma**:
  - The prevalence of asthma in adults has increased, with earlier estimates around **2%** and more recent studies suggesting higher rates.

- **Combined Respiratory Conditions**:
  - Among patients with chronic respiratory symptoms, the relative prevalence was **26%** for asthma, **42%** for COPD, and **32%** for asthma-COPD overlap (ACO).

- **Risk Factors**:
  - **Smoking**: High rates of smoking contribute significantly to the prevalence of lung diseases such as COPD and asthma.
  - **Air Pollution**: Environmental factors, including air pollution, exacerbate respiratory conditions.

- **E-Cigarette Use and Respiratory Health**:
  - In the first six months of 2024, approximately **100 cases** of e-cigarette poisoning were treated at the Poison Control Centre of Bạch Mai Hospital in Hanoi.

---

### **Correlation Between HR and SpO₂ Abnormalities and These Conditions**

- **Heart Rate (HR) and Blood Oxygen Saturation (SpO₂)**:
  - Monitoring HR and SpO₂ is crucial in managing cardiovascular and respiratory diseases.
  - Reduced heart rate variability (HRV) is associated with COPD severity, indicating autonomic dysfunction.
  - Low SpO₂ levels can signal hypoxemia, common in both acute heart failure and chronic respiratory conditions.
  - Regular monitoring can aid in early detection and management, potentially reducing morbidity and mortality associated with these diseases.

### 💡 The top 2 leading causes of death in Vietnam is **Stroke** and **Ischaemic heart disease** according to WHO

---

## **🩸 SYMPTOMPS OF LOW HEART RATE (BRADYCARDIA) AND LOW BLOOD OXYGEN (HYPOXEMIA), ALONG WITH THEIR POTENTIAL DANGERS**

| **Condition**                                             | **Symptoms**                                                                                                                                                                                              | **Why It’s Dangerous**                                                                                                                                             |
| --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Low Heart Rate (Bradycardia)** <br> (*HR below 60 BPM*) | - Dizziness or lightheadedness  <br> - Fatigue or weakness  <br> - Chest pain or discomfort  <br> - Confusion or memory issues  <br> - Fainting (syncope)                                                 | - Reduced blood flow to the brain and vital organs <br> - Increased risk of heart failure or cardiac arrest <br> - Can lead to permanent brain damage if untreated |
| **Low Blood Oxygen (Hypoxemia)** <br> (*SpO₂ below 90%*)  | - Shortness of breath (dyspnea)  <br> - Cyanosis (blue lips or fingertips)  <br> - Confusion or difficulty concentrating  <br> - Extreme fatigue or drowsiness  <br> - Increased heart rate (tachycardia) | - Oxygen deprivation can cause organ failure <br> - Increased risk of respiratory arrest and death <br> - Chronic hypoxemia is linked to lung and heart diseases   |

&rarr; Regular monitoring of **heart rate (HR)** and **blood oxygen saturation (SpO₂)** is essential for detecting early signs of cardiovascular and respiratory issues. With the right measuring devices, individuals can identify abnormalities before they become life-threatening. However, with **LOCATION** and **STEP TRACKING**, we are capable of better early disease detection, emergency response and personal fitness.

## **❗ THE NECESSITY OF INTEGRATING HR, SPO₂, LOCATION, AND STEP TRACKING**

### **Enhanced Early Detection and Health Monitoring**  

Tracking HR and SpO₂ alone is useful, but adding **location and movement data** provides deeper insights into health risks:  

- **Heart and lung disease management:** A sudden drop in SpO₂ combined with **low step count or prolonged inactivity** may indicate breathing difficulties, fatigue, or underlying conditions like COPD or heart failure.  
- **Activity-based health tracking:** Monitoring HR, SpO₂, and **location changes** together can reveal how different environments (e.g., high altitudes, polluted areas) affect oxygen levels and cardiovascular function.  
- **Stress and fatigue detection:** If **HR remains elevated** despite low physical activity, it may indicate **stress, dehydration, or cardiovascular strain**.  

---

### **Emergency Response and Safety**  

Real-time HR, SpO₂, and location data are **critical in emergencies**:  

- **Fall and health incident detection:** If a user’s HR drops abnormally and no steps are recorded for a long time, the device can automatically **send an SOS alert** with their location.  
- **Outdoor safety:** For athletes, hikers, or elderly individuals, **tracking movement, oxygen levels, and HR** can help detect **overexertion, dehydration, or potential heart issues**.  
- **Pandemic or respiratory disease response:** If SpO₂ drops in a person with suspected **COVID-19, pneumonia, or respiratory infections**, tracking movement helps **prevent spreading** the illness by identifying high-risk zones.

---

### **Lifestyle Optimization and Personalized Health Insights**  

By combining HR, SpO₂, steps, and location, people can **better understand their daily habits and optimize their health**:  

- **Step tracking helps detect inactivity** – Low step count combined with an increasing HR may indicate **poor cardiovascular fitness** or **early fatigue issues**.  
- **Sleep and recovery analysis** – Monitoring HR and SpO₂ at night, along with movement patterns, can help detect **sleep apnea or poor recovery from exercise**.  
- **Smart coaching and fitness improvement** – Devices can adjust workout intensity based on real-time HR and SpO₂ data, ensuring **safe and effective training**.  

### Objectives and Goals

(Description here...)

### Technical Challenges

#### Overview

![Image](sequential_diagram.png)

#### Week 4

#### Soldering

## Soldering Techniques

## Required Tools and Materials

- Soldering flux (preferably containing rosin, also used to clean the soldering tip by dipping it into the flux container)
- Soldering iron (for small components, use a fine-tip soldering iron; for larger components, use a pulse soldering iron with a larger, slightly curved tip)
- Solder wire
- The metal object to be soldered
- Practice soldering board (10x22 cm is sufficient)
- Face mask (to protect against toxic fumes; inhaling too much can cause infertility)
- Electrical wires
- LED (for testing connections)
- Resistor (to protect the LED)
- Solder sucker (optional but useful for removing excess solder; reheat the solder joint and press the button to suck up the solder)

## Wire Soldering Process

- Take two wires with exposed metal ends and scrape off any oxidation (this ensures the solder adheres properly)
- Clamp the two metal ends together
- Apply a small amount of flux from the solder wire onto the joint
- Position the solder wire close to the joint and directly apply the soldering iron in the middle; move gently while heating for about 3 seconds, then remove the iron and let it cool

## Soldering Process for Any Component Leads

- Prepare the component leads and insert them correctly into the holes, ensuring the longer lead is facing upward and the shorter lead downward
- Flip the component over and secure it so that the long leads remain aligned
- Apply flux to the short leads using the solder wire
- Bring the solder wire close to the short lead and start soldering, moving carefully to ensure an even flow of solder; use a fine-tip soldering iron to prevent solder from spreading to adjacent holes
- Heat each joint for about 3 seconds before moving to the next one
- After finishing all joints, gently brush off any excess flux

## Practicing with a Soldering Board

- Prepare small pieces of electrical wire and strip both ends (cut an old wire and strip it)
- Insert the wires into two holes on the practice board
- Solder one end of the wire to a resistor lead, then solder the other resistor lead to the positive lead of the LED; finally, solder the negative LED lead to the remaining wire end. **Apply flux at all soldering points**
- Use an adapter to test the connection by connecting the positive terminal to the resistor lead and the negative terminal to the wire attached to the LED’s negative lead. If the LED lights up, the soldering is successful

## Using a Solder Sucker

- Heat the solder joint with the soldering iron until the solder melts, then quickly use the solder sucker to remove the molten solder
- This method allows you to separate previously soldered connections; wipe off any residue for a clean finish

## Tips and Best Practices

- Hold the soldered joint in the air; placing it on a surface will cause heat dissipation, making the solder dry too quickly, which prevents proper solder flow
- Avoid working in environments with fans or air conditioning
- Hold both the soldering iron and solder wire firmly; otherwise, the solder might flow in the wrong direction or not adhere properly

#### Hardware

(Description here...)

#### Software

(Description here...)

### Design Considerations

#### Overall Considerations

(Description here...)

#### Initial Setup and Pairing

#### Week 5,6,7 : Communication Design

## **Secure Client-Server Connection System for SpO₂ Monitoring Device**

## **1. Server Initialization**

Each server unit is pre-configured with the following security measures:

- A unique **UUID** embedded in firmware.
- A cryptographic **key pair** (private and public key) for authentication.
- An internal **secret passphrase** for secure pairing.
- A **QR code** on the device packaging containing a secondary passphrase for ownership verification.

Upon initial activation:

1. The server remains inactive until the user **presses a physical button** to enable setup mode.
2. A temporary **Wi-Fi hotspot** is broadcast, requiring the QR-code-derived passphrase for access.
3. The user connects to this hotspot and enters **Wi-Fi credentials** through a configuration interface.
4. Once credentials are stored, the server **connects to the home network** and setup mode is disabled.

## **2. Client-Server Pairing Process**

Each wearable client device is equipped with:

- A unique **UUID**
- A cryptographic **key pair**
- An internal **secret passphrase**

To establish a secure pairing:

1. The user **places the client device in close proximity** to the server (within BLE/NFC range).
2. A **physical button press** on both devices initiates the pairing process.
3. The server and client **exchange public keys and UUIDs** securely.
4. A **challenge-response authentication** mechanism ensures the server’s authenticity:
   - The client generates a **random nonce** and sends it to the server.
   - The server signs the nonce using its **private key** and returns it.
   - The client verifies the signature using the **server’s public key**.
5. If authentication is successful, both devices **store each other’s UUIDs**, preventing future unauthorized connections.

## **3. Secure Data Transmission**

Following successful pairing:

- The client device connects to Wi-Fi and transmits **SpO₂ and heart rate data** to the server.
- Data exchange is **encrypted** using AES-256 to ensure confidentiality.
- The client only communicates with its **verified server UUID**, mitigating the risk of spoofing.
- If the server is replaced, **re-pairing is required** to establish a new trusted connection.

## **4. Remote Access via Telegram API**

For remote monitoring, the server integrates with the **Telegram API**:

- The server can send real-time **alerts and logs** via Telegram.
- This functionality is implemented using **MicroPython**, enabling lightweight but secure remote interaction.
- **API keys** ensure restricted access to authorized users only.

## **5. Data Storage and Web-Based Management**

- The server logs all data locally using an **SD card** for secure storage.
- A **web-based UI**, accessible within the local network, provides:
  - **Device management** (pairing, settings, and firmware updates)
  - **Real-time sensor data visualization**
  - **Historical logs and system diagnostics**
- The server runs a **lightweight HTTP server** to facilitate this interface.

## **6. Security and Reliability Measures**

- **Mutual authentication** between client and server prevents unauthorized devices from accessing or spoofing the system.
- **Encrypted communication** ensures that health data remains private.
- **Physical button presses and close-proximity pairing** protect against remote attacks.
- In the event of failure, hardware access is required for diagnostics, preventing remote exploitation.

## **7. Assessment and Review**

### **Why This Approach Was Chosen**

This approach was selected to balance **security, ease of use, and hardware constraints**:

- **Security**: Ensuring data integrity and preventing spoofing was a priority, leading to the use of cryptographic key pairs and challenge-response authentication.
- **Ease of Use**: Physical button pairing and QR code verification provide a user-friendly experience while maintaining security.
- **Hardware Constraints**: The ESP32 platform has limited computational resources, so encryption and web-hosting were optimized to ensure smooth performance.

### **Alternative Approaches Considered and Discarded**

1. **Fully Open Wi-Fi Setup Mode**
   - **Reason Discarded**: A completely open Wi-Fi network during setup would introduce a security risk, allowing unauthorized users to access the server.
2. **NFC-Only Pairing**
   - **Reason Discarded**: While NFC is secure and convenient, requiring additional NFC hardware on both server and client was deemed unnecessary given that BLE can fulfill the same function.
3. **Static IP Address Assignment**
   - **Reason Discarded**: Manually assigning static IPs is complex for end users, and dynamically discovering the server’s local IP via a pairing mechanism is a more flexible solution.
4. **Cloud-Based Server for Data Storage**
   - **Reason Discarded**: Storing data on a cloud server would introduce privacy concerns and external dependencies, whereas a local SD card keeps data contained and secure.
5. **Full TLS Encryption for Local Communication**
   - **Reason Discarded**: While TLS is highly secure, it introduces computational overhead on ESP32 hardware. AES-256 encryption with pre-shared keys provides a practical alternative.

## **Conclusion**

This architecture ensures a robust, secure, and user-friendly connection between SpO₂ monitoring clients and the central server. The combination of **proximity-based pairing, cryptographic authentication, and encrypted communication** mitigates security risks while maintaining ease of use for end-users. The design choices made prioritize practical security within the constraints of embedded systems, ensuring a reliable and efficient solution.

### COMMUNICATION IS CONSIDERED OUT OF SCOPE OF THIS PROJECT

#### Hardware Design

(Description here...)

#### Security and Safety

(Description here...)

### WEEK 9: System Requirements

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

### Development Methodology

#### Workflow

### WEEK 10

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

#### Tools

(Description here...)

#### Results

(Description here...)

#### Timeline

## OxyRem Project Plan

## 📌 Overview

This document outlines the overall work required to develop **OxyRem**, a wearable SpO₂ and heart rate monitor built using **Rust for embedded systems**. The project is structured into multiple phases, including learning, hardware development, software implementation, testing, and reporting.

## 🚀 Key Objectives

- **Learn Rust & Rust Embedded** – Mastering Rust and its embedded capabilities.
- **Understand Communication Protocols** – SPI, I2C, UART for sensor and OLED interaction.
- **Develop a Logging, Alert, and Warning System** – Efficient data storage, processing, and notifications.
- **Design the Wearable Frame & Electronics** – Soldering, integrating hardware, and mechanical design.
- **Prepare Reports & Final Presentation** – Document findings and present the project effectively.

## 📅 Detailed Timeline

### **Week 1: Learning Rust & Rust Embedded**

- Set up the **Rust toolchain** and development environment.
- Learn about **memory safety, concurrency, and embedded constraints**.
- Study **Rust Embedded HAL and frameworks like Embassy**.

### **Week 2: Learning Communication Protocols & Controlling Peripherals**

- Understand **SPI, I2C, UART** communication.
- Interface with **OLED screens, SpO₂ sensors, and microcontrollers**.
- Implement basic peripheral control in Rust.

### **Week 3: Testing Peripherals**

- Test individual components (OLED, sensors, ESP32 communication).
- Debug hardware interactions and optimize sensor readings.

### **Week 4: Mid-Term Report**

- Summarize progress so far.
- Identify potential risks and adjust the roadmap if needed.

### **Week 5: Designing the Frame & Continued Peripheral Testing**

- Design the **wearable casing using SolidWorks**.
- **3D print and assemble** initial prototypes.
- Further improve peripheral interactions.

### **Week 6: Server-Side Implementation & Logging System**

- Develop a **local/offline-first logging system**.
- Implement **Wi-Fi synchronization & Telegram alerts**.
- Test backend data handling.

### **Week 7: Full System Integration & Testing**

- Combine firmware, sensors, and UI.
- Perform **stress tests and battery consumption analysis**.
- Debug and optimize for real-world usage.

### **Week 8: Project Report**

- Compile final documentation.
- Include **technical findings, challenges, and optimizations**.

### **Week 9: Presentation & Refinement**

- Prepare **final slides and demo**.
- Polish both **hardware and software** for best results.

### **Weeks 10-12: Reserved for Extra Features & Improvements**

- If time allows, explore:
  - **GPS Tracking & Fall Detection**
  - **Battery efficiency optimizations**

---
