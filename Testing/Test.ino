#include <Wire.h>

#define I2C_ADDR 0x57

// FIFO: 32 samples, 18-bit per sample, packed as 3 bytes
uint8_t fifo[32][3];
uint8_t fifo_wr_ptr = 0;
uint8_t fifo_rd_ptr = 0;
uint8_t overflow_counter = 0;

uint8_t reg[256];
uint8_t last_reg = 0;
uint8_t fifo_byte_index = 0; // Track which byte of a sample is being read
bool expecting_register = true; // Track if the next onReceive byte is a register address

void setup() {
  Wire.begin(I2C_ADDR);
  Wire.onReceive(onReceive);
  Wire.onRequest(onRequest);

  Serial.begin(115200);
  reg[0xFF] = 0x15; // PART_ID

  // Fill FIFO with 18-bit sample encoded as per datasheet
  for (int i = 0; i < 32; i++) {
    uint32_t sample = random(0, 0x3FFFF); // 18-bit
    fifo[i][0] = ((sample >> 16) & 0x03) << 6; // top 2 bits in bits 7–6
    fifo[i][1] = (sample >> 8) & 0xFF;
    fifo[i][2] = sample & 0xFF;
  }

  Serial.println("MAX30102 HR Mode Emulator Ready");
}

void loop() {
  static unsigned long last = 0;
  if (millis() - last > 1000) {
    last = millis();

    uint32_t sample = random(0, 0x3FFFF);
    fifo[fifo_wr_ptr][0] = ((sample >> 16) & 0x03) << 6;
    fifo[fifo_wr_ptr][1] = (sample >> 8) & 0xFF;
    fifo[fifo_wr_ptr][2] = sample & 0xFF;

    fifo_wr_ptr = (fifo_wr_ptr + 1) % 32;

    if (fifo_wr_ptr == fifo_rd_ptr) {
      overflow_counter = min(overflow_counter + 1, 31);
    }
  }
}

void onReceive(int len) {
  if (len < 1) return;

  // First byte should be the register address
  last_reg = Wire.read();
  expecting_register = false;
  fifo_byte_index = 0;
  len--;

  while (len-- > 0) {
    uint8_t value = Wire.read();
    reg[last_reg] = value;

    if (last_reg == 0x04) fifo_wr_ptr = value;
    if (last_reg == 0x05) overflow_counter = value;
    if (last_reg == 0x06) fifo_rd_ptr = value;

    Serial.print("Wrote 0x"); Serial.print(value, HEX);
    Serial.print(" to reg 0x"); Serial.println(last_reg, HEX);

    last_reg++;
  }
}

void onRequest() {
  uint8_t out = 0x00;

  if (last_reg == 0x07) {
    // Serve FIFO one byte at a time, cycle through 3-byte samples
    out = fifo[fifo_rd_ptr][fifo_byte_index];
    fifo_byte_index++;
    if (fifo_byte_index >= 3) {
      fifo_byte_index = 0;
      fifo_rd_ptr = (fifo_rd_ptr + 1) % 32;
    }
    // FIFO register does not auto-increment last_reg
  } else {
    out = reg[last_reg];
    last_reg++; // auto-increment for non-FIFO registers
  }

  Wire.write(out);

  Serial.print("[onRequest] reg 0x");
  Serial.print(last_reg, HEX);
  Serial.print(" → 0x");
  Serial.println(out, HEX);
} 
