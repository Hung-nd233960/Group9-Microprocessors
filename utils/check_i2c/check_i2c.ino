#include <Wire.h>

void setup() {
  Wire.begin(8,9); // Khởi động I2C với SDA & SCL mặc định
  Serial.begin(115200);
  while (!Serial); // Chờ Serial kết nối (trên ESP32-C3)
  Serial.println("Đang quét các thiết bị I2C...");

  byte count = 0;
  for (byte address = 1; address < 127; ++address) {
    Wire.beginTransmission(address);
    if (Wire.endTransmission() == 0) {
      Serial.print("Thiết bị I2C tìm thấy tại địa chỉ 0x");
      Serial.println(address, HEX);
      count++;
    }
    delay(10);
  }

  if (count == 0)
    Serial.println("Không tìm thấy thiết bị I2C nào.");
  else
    Serial.println("Hoàn tất quét I2C.");
}

void loop() {
  // Không làm gì ở đây
}
