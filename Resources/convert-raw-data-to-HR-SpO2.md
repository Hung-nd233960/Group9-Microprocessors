# Converting Raw PPG Data to Heart Rate and SpO2

This document outlines the process of converting raw photoplethysmography (PPG) data from the MAX30102 sensor into heart rate (beats per minute, BPM) and oxygen saturation (SpO2, %) in a Rust program running on an ESP32-C6 microcontroller in a `no_std` environment. It details each step, including data acquisition, filtering, peak detection, and calculations, with a focus on handling the bandpass filter’s impact on the DC component for accurate SpO2 computation. Address the question of whether the bandpass filter prevents calculating the R ratio for SpO2.

## 1. Overview of the Process
The conversion of raw PPG data to heart rate and SpO2 involves several steps:
1. **Data Acquisition**: Collect raw IR and Red PPG signals from the MAX30102 sensor via I2C.
2. **Preprocessing**: Apply filtering to remove noise and isolate the pulsatile (AC) component.
3. **DC Component Extraction**: Compute the DC component of both IR and Red signals before filtering.
4. **Peak Detection**: Identify peaks in the filtered IR signal to calculate heart rate.
5. **SpO2 Calculation**: Use AC and DC components of IR and Red signals to compute the  R  ratio and estimate SpO2.
6. **Output**: Display or store the heart rate and SpO2 values.

The Rust code uses a bandpass filter (0.5–5 Hz) and a Kalman filter to process the IR signal. This document explains how to extend the code to handle both IR and Red signals for SpO2 and ensure the DC component is preserved for the  R  ratio.

## 2. Data Acquisition
### Purpose
The MAX30102 sensor measures light absorption from IR and Red LEDs to produce PPG signals. The IR signal is primarily used for heart rate detection, while both IR and Red signals are needed for SpO2 calculation.

### Implementation in Rust Code
In the provided code, raw data is acquired in the main loop using the `max3010x` library:

```rust
let samples_read: u8 = sensor.read_fifo(&mut data).unwrap();
for i in 0..samples_read {
    let ir_value = data[i as usize] as f32;
    ir_buffer[buffer_index % BUFFER_SIZE] = ir_value;
    buffer_index = (buffer_index + 1) % BUFFER_SIZE;
    println!("Sample {}: {:?}", i, data[i as usize]);
}
```

- **Explanation**:
  - `sensor.read_fifo(&mut data)`: Reads data from the MAX30102’s FIFO, storing it in the `data` array (assumed to be `[u32; 3]`).
  - `let ir_value = data[i as usize] as f32`: Assumes `data[i]` contains the IR signal, converted to `f32` for processing.
  - `ir_buffer`: Stores 100 IR samples in a circular buffer for filtering.
- **Limitation**: The code assumes `data[i]` contains only IR values. According to the MAX30102 datasheet, the FIFO typically interleaves IR and Red samples (6 bytes per sample: 3 bytes IR, 3 bytes Red). The `max3010x` library’s `read_fifo` output needs verification to extract both IR and Red correctly.

### Modification for SpO2
To calculate SpO2, both IR and Red signals must be extracted. Assuming the `read_fifo` method returns interleaved IR and Red values, modify the code to store both:

```rust
let mut ir_buffer = [0.0; BUFFER_SIZE];
let mut red_buffer = [0.0; BUFFER_SIZE];
for i in 0..samples_read {
    let ir_value = (data[i as usize] >> 16) as f32; // Example: IR in upper 16 bits
    let red_value = (data[i as usize] & 0xFFFF) as f32; // Example: Red in lower 16 bits
    ir_buffer[buffer_index % BUFFER_SIZE] = ir_value;
    red_buffer[buffer_index % BUFFER_SIZE] = red_value;
}
```

- **Note**: The bit-shifting (`>> 16`, `& 0xFFFF`) is hypothetical and depends on the `max3010x` library’s data format. Consult the library documentation or MAX30102 datasheet to confirm the FIFO structure.

## 3. Preprocessing with Bandpass Filter
### Purpose
The bandpass filter isolates the AC component of the PPG signal (0.5–5 Hz) for heart rate detection, removing:
- **DC component** (below 0.5 Hz): Static baseline from skin, tissue, and non-pulsatile blood.
- **High-frequency noise** (above 5 Hz): Motion artifacts, light interference, or electronic noise.

### Mathematical Foundation
The bandpass filter uses an IIR approximation:

y[i] = x[i] + a(X[i] - x[i-2]) - by[i-1] + 0.1y[i-2]


Where:
- a = tan(2pi x lowcut/fs) : coefficient for lowcut frequency
- b = tan(2pi x highcut/fs) : coefficient for highcut frequency
- 0.1 : A fixed coefficient to stabilize the filter response
- y[i] : Output (filtered signal) at sample [i]
- x[i] : Input (raw PPG signal) at sample [i]
- fs : Sampling frequency (100 Hz in this case)

### Code Implementation
The bandpass filter is implemented in the `filter_signal` function:

```rust
fn filter_signal(signal: &[f32; BUFFER_SIZE], lowcut: f32, highcut: f32, fs: f32) -> [f32; BUFFER_SIZE] {
    let mut filtered = [0.0; BUFFER_SIZE];
    let alpha = tanf(2.0 * 3.14159 * lowcut / fs);
    let beta = tanf(2.0 * 3.14159 * highcut / fs);
    for i in 2..signal.len() {
        filtered[i] = signal[i]
            + alpha * (signal[i] - signal[i - 2])
            - beta * filtered[i - 1]
            + 0.1 * filtered[i - 2];
    }
    filtered
}
```

- **Impact on DC Component**: This filter removes frequencies below 0.5 Hz, including the DC component, producing a signal centered around zero (AC component only). This is ideal for heart rate detection but removes the DC component needed for SpO2.

### Handling DC for SpO2
To calculate SpO2, the DC component must be computed **before** bandpass filtering. The DC component is typically the mean of the raw signal over a window:


DC = 1/N * Series of i=0 to N-1 of x[i]


Where N is the window size (e.g., `BUFFER_SIZE = 100`).

## 4. Kalman Filter for Smoothing
### Purpose
The Kalman filter smooths the bandpass-filtered signal, reducing residual noise (e.g., motion artifacts) to improve peak detection accuracy for heart rate and signal quality for SpO2.

### Mathematical Foundation
The Kalman filter estimates the true signal using:
- **Prediction**:  p = p + q 
- **Update**:
   
   k = p / (p+r) : Kalman gain 
   
   
   x = x + k(measurement - x) : Update state
   
   
   p = (1 - k)p : Update uncertainty
  

Where:
-  x : Estimated signal value.
-  p : Covariance (uncertainty).
-  q = 0.01 : Process noise.
-  r = 0.1 : Measurement noise.
-  k : Kalman gain.

### Code Implementation
The Kalman filter is applied after the bandpass filter in `filter_signal`:

```rust
struct KalmanFilter {
    x: f32, p: f32, q: f32, r: f32,
}
impl KalmanFilter {
    fn new() -> Self {
        KalmanFilter { x: 0.0, p: 1.0, q: 0.01, r: 0.1 }
    }
    fn update(&mut self, measurement: f32) -> f32 {
        self.p = self.p + self.q;
        let k = self.p / (self.p + self.r);
        self.x = self.x + k * (measurement - self.x);
        self.p = (1.0 - k) * self.p;
        self.x
    }
}
let mut kalman = KalmanFilter::new();
let mut kalman_filtered = [0.0; BUFFER_SIZE];
for i in 0..filtered.len() {
    kalman_filtered[i] = kalman.update(filtered[i]);
}
```

- **Note**: The Kalman filter operates on the AC component (post-bandpass), so it doesn’t affect the DC component needed for SpO2.

## 5. Peak Detection for Heart Rate
### Purpose
Heart rate is calculated by detecting peaks in the filtered IR signal, which correspond to heartbeats. The time interval between peaks determines the heart rate in BPM.

### Mathematical Foundation
1. **Peak Detection**: A sample is a peak if it’s greater than its neighbors and exceeds a threshold (to avoid noise).
2. **Heart Rate Calculation**:
   
   BPM = 60 / Delta t
   
   Where Delta t  is the time interval between consecutive peaks (in seconds), and fs = 100Hz gives Delta t = number of samples / fs 

### Suggested Implementation
Add a peak detection function to the Rust code:

```rust
fn detect_peaks(signal: &[f32; BUFFER_SIZE], threshold: f32) -> [usize; BUFFER_SIZE] {
    let mut peaks = [0; BUFFER_SIZE];
    let mut peak_count = 0;
    for i in 1..signal.len() - 1 {
        if signal[i] > signal[i - 1] && signal[i] > signal[i + 1] && signal[i] > threshold {
            peaks[peak_count] = i;
            peak_count += 1;
        }
    }
    peaks
}
```

- **Usage**:
  - Apply to `kalman_filtered` from `filter_signal`.
  - Compute BPM: Average the intervals between peaks and convert to BPM.

## 6. SpO2 Calculation
### Purpose
SpO2 is estimated using the ratio of AC and DC components of Red and IR signals, which reflects the oxygen saturation level in the blood.

### Mathematical Foundation
1. **Compute AC and DC Components**:
   - **DC**: Mean of the raw signal over the buffer:
     
       DC = 1/N * Series of i=0 to N-1 of x[i]
     
   - **AC**: Peak-to-peak amplitude of the bandpass-filtered signal (difference between max and min values).
2. **Calculate  R  Ratio**:
   
   R = (AC_Red / DC_Red) / (AC_IR / DC_IR).
   
3. **Estimate SpO2**:
   
   SpO2 (%) = 110 - 25R
   
   This is an empirical formula; actual coefficients may vary based on calibration.

### Suggested Implementation
Modify the code to compute DC and AC components and calculate SpO2:

```rust
fn calculate_spo2(ir_signal: &[f32; BUFFER_SIZE], red_signal: &[f32; BUFFER_SIZE], 
                  ir_filtered: &[f32; BUFFER_SIZE], red_filtered: &[f32; BUFFER_SIZE]) -> f32 {
    // Compute DC components (mean of raw signals)
    let mut dc_ir = 0.0;
    let mut dc_red = 0.0;
    for i in 0..BUFFER_SIZE {
        dc_ir += ir_signal[i];
        dc_red += red_signal[i];
    }
    dc_ir /= BUFFER_SIZE as f32;
    dc_red /= BUFFER_SIZE as f32;

    // Compute AC components (peak-to-peak of filtered signals)
    let mut ac_ir = 0.0;
    let mut ac_red = 0.0;
    for i in 0..BUFFER_SIZE {
        if ir_filtered[i] > ac_ir { ac_ir = ir_filtered[i]; }
        if ir_filtered[i] < -ac_ir { ac_ir = -ir_filtered[i]; }
        if red_filtered[i] > ac_red { ac_red = red_filtered[i]; }
        if red_filtered[i] < -ac_red { ac_red = -red_filtered[i]; }
    }

    // Calculate R ratio
    let r = (ac_red / dc_red) / (ac_ir / dc_ir);

    // Estimate SpO2
    110.0 - 25.0 * r
}
```

- **Integration**: Call this function in the main loop after filtering both IR and Red signals.

## 7. Impact of Bandpass Filter on SpO2 Calculation
### Question: Does the bandpass filter remove the DC component, preventing  R  calculation?
- **Answer**: Yes, the bandpass filter (0.5–5 Hz) removes the DC component, as it attenuates frequencies below 0.5 Hz. However, this does not prevent calculating  R , as the DC component can be computed from the **raw signal** before bandpass filtering.
- **Solution**:
  - **DC Component**: Calculate the mean of the raw IR and Red signals (`ir_buffer` and `red_buffer`) before applying `filter_signal`.
  - **AC Component**: Use the bandpass-filtered signals (`ir_filtered` and `red_filtered`) to extract the peak-to-peak amplitude.
  - **Code Modification**: Store raw IR and Red signals, compute DC components, then apply bandpass filtering to get AC components, as shown in the `calculate_spo2` function above.

### Code Modification in Main Loop
Update the main loop to handle both IR and Red signals and compute SpO2:

```rust
let mut ir_buffer = [0.0; BUFFER_SIZE];
let mut red_buffer = [0.0; BUFFER_SIZE];
let mut buffer_index = 0;
loop {
    let samples_read: u8 = sensor.read_fifo(&mut data).unwrap();
    for i in 0..samples_read {
        let ir_value = (data[i as usize] >> 16) as f32; // Adjust based on FIFO format
        let red_value = (data[i as usize] & 0xFFFF) as f32;
        ir_buffer[buffer_index % BUFFER_SIZE] = ir_value;
        red_buffer[buffer_index % BUFFER_SIZE] = red_value;
        buffer_index = (buffer_index + 1) % BUFFER_SIZE;
        println!("Sample {}: IR={:.2}, Red={:.2}", i, ir_value, red_value);
        if buffer_index == 0 {
            let ir_filtered = filter_signal(&ir_buffer, 0.5, 5.0, SAMPLE_RATE);
            let red_filtered = filter_signal(&red_buffer, 0.5, 5.0, SAMPLE_RATE);
            let spo2 = calculate_spo2(&ir_buffer, &red_buffer, &ir_filtered, &red_filtered);
            println!("SpO2: {:.2}%", spo2);
            // Add peak detection for heart rate here
        }
    }
    Timer::after(Duration::from_millis(1000)).await;
}
```

## 8. Tuning and Verification
- **Bandpass Filter**:
  - Adjust `lowcut` (e.g., 0.7 Hz) or `highcut` (e.g., 4 Hz) if peaks are not clear.
  - Verify the filtered signal via UART output (`Filtered Sample`).
- **Kalman Filter**:
  - Tune `q` (e.g., 0.001–0.1) and `r` (e.g., 0.01–0.5) based on signal noise.
  - Ensure the signal is smooth but retains PPG peaks.
- **Heart Rate**:
  - Check peak detection accuracy by comparing BPM to a reference (e.g., pulse oximeter).
  - Adjust the threshold in `detect_peaks` to avoid false positives.
- **SpO2**:
  - Validate SpO2 against a medical-grade device (typical range: 95–100% for healthy individuals).
  - Calibrate the SpO2 formula coefficients (110, 25) if necessary.
- **FIFO Data**: Confirm the `read_fifo` data format to correctly extract IR and Red values.

## 9. Conclusion
Converting raw PPG data to heart rate and SpO2 involves acquiring IR and Red signals, preprocessing with bandpass and Kalman filters, extracting DC components before filtering, detecting peaks for heart rate, and calculating the R ratio for SpO2. The bandpass filter removes the DC component, but this can be addressed by computing the DC from raw signals before filtering. The provided Rust code can be extended with peak detection and SpO2 calculation, ensuring accurate results in a `no_std` environment on the ESP32-C6.

References : 

https://www.ti.com/lit/an/slaa655/slaa655.pdf?ts=1748254022937&ref_url=https%253A%252F%252Fwww.google.com%252F

https://www.quora.com/How-is-an-IR-sensor-able-to-detect-a-heart-beat

https://community.element14.com/products/manufacturers/analog-devices/f/forum/29859/how-to-compute-heart-rate-and-spo2-from-max30100

https://makezine.com/projects/ir-pulse-sensor/

https://www.researchgate.net/publication/342042713_HEART_RATE_MONITOR_USING_INFRARED

