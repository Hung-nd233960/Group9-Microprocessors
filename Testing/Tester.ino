#include <Wire.h>

#define MAX30102_ADDR 0x57
#define SDA_PIN 4
#define SCL_PIN 5

void setup() {
  Serial.begin(115200);

  Wire.begin(0x57); // Start as I2C master
  delay(500);
}

void loop() {
  // === PART ID Read ===
  Wire.beginTransmission(MAX30102_ADDR);
  Wire.write(0xFF); // PART_ID register
  if (Wire.endTransmission(false) == 0) {
    Wire.requestFrom(MAX30102_ADDR, 1);
    if (Wire.available()) {
      uint8_t part_id = Wire.read();
      Serial.print("PART ID: 0x"); Serial.println(part_id, HEX);
    } else {
      Serial.println("No response from PART_ID");
    }
  } else {
    Serial.println("I2C write to PART_ID failed");
  }

  // === FIFO Sample Read (3 bytes) ===
  Wire.beginTransmission(MAX30102_ADDR);
  Wire.write(0x07); // FIFO_DATA register
  if (Wire.endTransmission(false) == 0) {
    Wire.requestFrom(MAX30102_ADDR, 3);
    if (Wire.available() >= 3) {
      uint8_t b0 = Wire.read();
      uint8_t b1 = Wire.read();
      uint8_t b2 = Wire.read();

      Serial.print("FIFO: ");
      Serial.print(b0, HEX); Serial.print(" ");
      Serial.print(b1, HEX); Serial.print(" ");
      Serial.println(b2, HEX);

      uint32_t sample = ((b0 >> 6) << 16) | (b1 << 8) | b2;
      Serial.print("Sample (18-bit): 0x");
      Serial.println(sample, HEX);
    } else {
      Serial.println("FIFO read: insufficient bytes");
    }
  } else {
    Serial.println("I2C write to FIFO failed");
  }

  Serial.println("-----");
  delay(1000); // Poll every 1s
}
