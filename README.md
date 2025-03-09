# OxyRem

## 🚀 Overview
**OxyRem** is a **wearable health-monitoring device** designed to track **blood oxygen levels (SpO₂) and heart rate**, ensuring real-time health insights for users. Built using **Rust for embedded systems**, OxyRem prioritizes efficiency, reliability, and an offline-first approach to ensure uninterrupted data logging. The project aims to address the increasing demand for **accessible, real-time health monitoring**, particularly for individuals with respiratory issues, athletes, and elderly users.

## 🩺 The Problem
Millions of people worldwide suffer from conditions that require **continuous monitoring of blood oxygen levels and heart rate**—such as **COPD, sleep apnea, and cardiovascular diseases**. Existing solutions often fall short due to:
- **High cost** of medical-grade devices.
- **Short battery life** in consumer wearables due to inefficient software.
- **Limited offline functionality**, requiring a constant internet connection.
- **Lack of customizable features** for different user needs.

OxyRem tackles these issues by providing a **power-efficient, offline-first wearable** that can store and transmit data **only when needed**—reducing power consumption while ensuring vital data is accessible when required.

## 📦 Features
- **Monitor Heart Rate & SpO₂** – Continuous tracking with real-time updates.
- **Data Logging & Backup** – Saves data offline and syncs when Wi-Fi is available.
- **Alerts & Warnings** – Detects abnormal vitals and notifies the user.
- **(In Evaluation) GPS Tracking & Fall Detection** – Potential future features for expanded health monitoring.

## 🛠 Installation & Setup
### Prerequisites
- Rust toolchain
- ESP32 development environment
- Python 3.x (for optional debugging tools)

### Installation Steps
1. Clone the repository:
   ```sh
   git clone https://github.com/your-username/OxyRem.git
   ```
2. Navigate to the project directory:
   ```sh
   cd OxyRem
   ```
3. Build and flash firmware:
   ```sh
   cargo build --release
   ```

## 📖 Usage
Once installed, the device automatically starts monitoring SpO₂ and heart rate. Data is logged every **30s to 5 minutes** (configurable) and stored locally. When connected to a **trusted Wi-Fi**, logs are uploaded to the server or sent via **Telegram notifications** (if enabled). The device UI displays live vitals on an **OLED screen**.

## 📜 Contribution Guidelines
We welcome contributions! Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to submit issues, feature requests, and pull requests.

## 🔥 Issue Tracking
- Report bugs or suggest features in the [Issues](https://github.com/your-username/OxyRem/issues) section.
- Issues are categorized by severity and urgency.

## 🌳 Branching Strategy
- `main` – Stable production-ready branch.
- `develop` – Latest development changes.
- Feature branches: `feature/your-feature-name`

## 🙌 Acknowledgments
- Thanks to contributors, Rust Embedded community, and ESP32 developers.
- Libraries and frameworks used: Rust Embedded HAL, LVGL for UI (if applicable), and Flask (server-side support).

## 📜 Documentation
- [Contribution Guidelines](CONTRIBUTING.md)
- [Project Rules](RULES.md)
- [Changelog](CHANGELOG.md)
- [Workflow](WORKFLOW.md)

---
### 🛠 Maintainers
- [Your Name](https://github.com/your-username)

