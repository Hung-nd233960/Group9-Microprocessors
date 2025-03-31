# **Secure Client-Server Connection System for SpO₂ Monitoring Device**

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

## **Conclusion**

This architecture ensures a robust, secure, and user-friendly connection between SpO₂ monitoring clients and the central server. The combination of **proximity-based pairing, cryptographic authentication, and encrypted communication** mitigates security risks while maintaining ease of use for end-users.
