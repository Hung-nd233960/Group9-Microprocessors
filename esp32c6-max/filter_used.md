# PPG Signal Filtering: Bandpass and Kalman Filters

This document explains the bandpass filter and Kalman filter used to process photoplethysmography (PPG) signals from the MAX30102 sensor in a Rust program running on an ESP32-C6 microcontroller. Both filters are implemented in a `no_std` environment to ensure compatibility with embedded systems. The document covers the purpose, mathematical foundation, and implementation details of each filter, along with an analysis of the corresponding code segments.

## 1. Bandpass Filter

### Purpose
The bandpass filter is used to isolate the frequency components of the PPG signal that correspond to heart rate (typically 0.5–5 Hz, equivalent to 30–300 beats per minute, BPM). It removes:
- **Low-frequency noise** (below 0.5 Hz), including the DC component (static baseline from skin, tissue, and non-pulsatile blood).
- **High-frequency noise** (above 5 Hz), such as motion artifacts, environmental light interference, or electronic noise.

By focusing on the 0.5–5 Hz range, the bandpass filter ensures the signal contains primarily the AC component (pulsatile blood flow) needed for heart rate detection and SpO2 calculation.

### Mathematical Foundation
The bandpass filter is implemented as a simple Infinite Impulse Response (IIR) filter, which approximates a Butterworth-like response. The filter combines:
- A **high-pass component** to remove low frequencies (including DC).
- A **low-pass component** to remove high frequencies.

The filter equation used is an IIR approximation:

y[i] = x[i] + a(X[i] - x[i-2]) - by[i-1] + 0.1y[i-2]

- a = tan(2pi x lowcut/fs) : coefficient for lowcut frequency
- b = tan(2pi x highcut/fs) : coefficient for highcut frequency
- 0.1 : A fixed coefficient to stabilize the filter response
- y[i] : Output (filtered signal) at sample [i]
- x[i] : Input (raw PPG signal) at sample [i]
- fs : Sampling frequency (100 Hz in this case)

### Code Analysis
The bandpass filter is implemented in the `filter_signal` function. Below is the relevant code snippet with detailed explanations:

```rust
// Hàm lọc tín hiệu: Chỉ sử dụng bandpass filter
fn filter_signal(signal: &[f32; BUFFER_SIZE], lowcut: f32, highcut: f32, fs: f32) -> [f32; BUFFER_SIZE] {
    let mut filtered = [0.0; BUFFER_SIZE];
    let alpha = tanf(2.0 * 3.14159 * lowcut / fs);
    let beta = tanf(2.0 * 3.14159 * highcut / fs);

    // Bandpass filter: Loại bỏ nhiễu tần số thấp (< 0.5 Hz) và cao (> 5 Hz)
    for i in 2..signal.len() {
        filtered[i] = signal[i]
            + alpha * (signal[i] - signal[i - 2])
            - beta * filtered[i - 1]
            + 0.1 * filtered[i - 2];
    }
    filtered
}
```

- **Line-by-line explanation**:
  - `fn filter_signal(signal: &[f32; BUFFER_SIZE], lowcut: f32, highcut: f32, fs: f32) -> [f32; BUFFER_SIZE]`:
    - Defines the function that takes a static array of raw IR samples (`signal`), low and high cutoff frequencies (`lowcut = 0.5 Hz`, `highcut = 5.0 Hz`), and sampling frequency (`fs = 100 Hz`).
    - Returns a static array of filtered samples.
    - Uses a fixed-size array (`[f32; BUFFER_SIZE]`) to ensure compatibility with `no_std`.
  - `let mut filtered = [0.0; BUFFER_SIZE]`:
    - Creates a static array initialized with zeros to store the filtered signal.
  - `let alpha = tanf(2.0 * 3.14159 * lowcut / fs)`:
    - Calculates the coefficient a = tan(2pi x lowcut/fs) for the low-frequency cutoff (0.5 Hz).
    - Uses `tanf` from the `libm` crate, as `no_std` does not provide the standard `tan` function.
  - `let beta = tanf(2.0 * 3.14159 * highcut / fs)`:
    - Calculates the coefficient b = tan(2pi x highcut/fs) for the high-frequency cutoff (5 Hz).
  - `for i in 2..signal.len()`:
    - Iterates over the input signal starting from index 2, as the filter uses samples x[i-2] and y[i-2], requiring at least two prior samples.
  - `filtered[i] = signal[i] + alpha * (signal[i] - signal[i - 2]) - beta * filtered[i - 1] + 0.1 * filtered[i - 2]`:
    - Implements the IIR bandpass filter equation.
    - `signal[i]`: Current raw sample.
    - `alpha * (signal[i] - signal[i - 2])`: Emphasizes signal changes to remove low frequencies (high-pass effect).
    - `- beta * filtered[i - 1] + 0.1 * filtered[i - 2]`: Feedback terms to smooth the signal and remove high frequencies (low-pass effect).
  - `filtered`:
    - Returns the filtered signal array, with frequencies outside 0.5–5 Hz attenuated.

### Characteristics
- **Advantages**:
  - Simple and computationally lightweight, suitable for resource-constrained devices like the ESP32-C6.
  - Effectively removes DC components and high-frequency noise, isolating the PPG signal for heart rate detection.
- **Limitations**:
  - Fixed coefficients (a, b, 0.1) may not adapt well to dynamic noise (e.g., motion artifacts).
  - Less effective against non-stationary noise compared to adaptive filters like Kalman.

## 2. Kalman Filter

### Purpose
The Kalman filter is used to further smooth the bandpass-filtered PPG signal, reducing residual noise, particularly from motion artifacts or random fluctuations. It is an adaptive filter that optimally estimates the true signal by combining predictions (based on a model of the signal) with measurements (the bandpass-filtered signal).

In this context, the Kalman filter:
- Enhances the signal quality for more accurate peak detection (for heart rate) or SpO2 calculation.
- Adapts to changing noise conditions, making it ideal for wearable devices where users may move during measurement.

### Mathematical Foundation
The Kalman filter assumes the PPG signal follows a simple dynamic model (e.g., a constant or slowly varying signal) with added noise. It uses two steps:
1. **Prediction**: Estimates the next state based on the current state and a process model.
2. **Update**: Corrects the prediction using the actual measurement, weighted by the Kalman gain.

The filter maintains:
- **State x: The estimated true signal value.
- **Covariance p: Uncertainty in the state estimate.
- **Process noise q: Expected noise in the signal dynamics (e.g., 0.01).
- **Measurement noise r: Expected noise in the measurements (e.g., 0.1).

The Kalman filter equations are:
1. **Prediction**:
   
   p = p + q
   
2. **Update**:
   
   k = p / (p+r) : Kalman gain 
   
   
   x = x + k(measurement - x) : Update state
   
   
   p = (1 - k)p : Update uncertainty
   

Where:
-  k : Kalman gain, balancing trust in the prediction vs. the measurement.
-  measurement : The bandpass-filtered signal value.

### Code Analysis
The Kalman filter is implemented as a struct (`KalmanFilter`) with an `update` method, applied after the bandpass filter in the `filter_signal` function. Below is the relevant code with explanations:

```rust
// Cấu trúc cho bộ lọc Kalman
struct KalmanFilter {
    x: f32, // Trạng thái (giá trị tín hiệu ước lượng)
    p: f32, // Độ không chắc chắn của trạng thái
    q: f32, // Nhiễu quá trình (process noise)
    r: f32, // Nhiễu đo lường (measurement noise)
}

impl KalmanFilter {
    // Khởi tạo bộ lọc Kalman
    fn new() -> Self {
        KalmanFilter {
            x: 0.0, // Giá trị ban đầu
            p: 1.0, // Độ không chắc chắn ban đầu
            q: 0.01, // Nhiễu quá trình (điều chỉnh theo thực tế)
            r: 0.1, // Nhiễu đo lường (điều chỉnh theo thực tế)
        }
    }

    // Cập nhật bộ lọc Kalman với giá trị đo mới
    fn update(&mut self, measurement: f32) -> f32 {
        // Dự đoán
        self.p = self.p + self.q;

        // Cập nhật
        let k = self.p / (self.p + self.r); // Kalman gain
        self.x = self.x + k * (measurement - self.x); // Cập nhật trạng thái
        self.p = (1.0 - k) * self.p; // Cập nhật độ không chắc chắn

        self.x
    }
}

// Trong hàm filter_signal
// Áp dụng bộ lọc Kalman để làm mượt tín hiệu
let mut kalman = KalmanFilter::new();
let mut kalman_filtered = [0.0; BUFFER_SIZE];
for i in 0..filtered.len() {
    kalman_filtered[i] = kalman.update(filtered[i]);
}
```

- **Line-by-line explanation**:
  - **Struct `KalmanFilter`**:
    - `x: f32`: The estimated signal value (initially 0.0).
    - `p: f32`: The covariance of the state estimate (initially 1.0, indicating high uncertainty).
    - `q: f32`: Process noise (0.01), representing expected variation in the signal dynamics.
    - `r: f32`: Measurement noise (0.1), representing expected noise in the bandpass-filtered signal.
  - `fn new() -> Self`:
    - Initializes the Kalman filter with default values.
    - `q = 0.01` and `r = 0.1` are empirical values that may need tuning based on actual signal noise.
  - `fn update(&mut self, measurement: f32) -> f32`:
    - Implements the Kalman filter equations.
    - `self.p = self.p + self.q`: Increases uncertainty by adding process noise.
    - `let k = self.p / (self.p + self.r)`: Calculates Kalman gain, determining how much to trust the measurement vs. the prediction.
    - `self.x = self.x + k * (measurement - self.x)`: Updates the state by blending the prediction with the measurement error.
    - `self.p = (1.0 - k) * self.p`: Updates the covariance, reducing uncertainty.
    - Returns the updated state (`self.x`).
  - `let mut kalman = KalmanFilter::new()`:
    - Creates a new Kalman filter instance for processing the bandpass-filtered signal.
  - `let mut kalman_filtered = [0.0; BUFFER_SIZE]`:
    - Creates a static array to store the Kalman-filtered signal, compatible with `no_std`.
  - `for i in 0..filtered.len()`:
    - Iterates over the bandpass-filtered signal.
  - `kalman_filtered[i] = kalman.update(filtered[i])`:
    - Applies the Kalman filter to each sample, producing a smoother signal.

### Characteristics
- **Advantages**:
  - Adapts to dynamic noise, such as motion artifacts, better than a static bandpass filter.
  - Produces a smoother signal, improving peak detection and SpO2 calculation accuracy.
  - Suitable for real-time processing due to low computational overhead (single-state model).
- **Limitations**:
  - Requires tuning of `q` and `r` parameters for optimal performance.
  - Assumes a simple signal model (constant or slowly varying), which may not capture complex PPG dynamics.

## 3. Integration in the Rust Code
The bandpass and Kalman filters are integrated into the `filter_signal` function, which processes a buffer of raw IR samples from the MAX30102 sensor. The function is called when the buffer is full (100 samples), and the filtered signal is printed via UART for debugging.

### Full filter_signal Code
```rust
fn filter_signal(signal: &[f32; BUFFER_SIZE], lowcut: f32, highcut: f32, fs: f32) -> [f32; BUFFER_SIZE] {
    let mut filtered = [0.0; BUFFER_SIZE];
    let alpha = tanf(2.0 * 3.14159 * lowcut / fs);
    let beta = tanf(2.0 * 3.14159 * highcut / fs);

    // Bandpass filter: Loại bỏ nhiễu tần số thấp (< 0.5 Hz) và cao (> 5 Hz)
    for i in 2..signal.len() {
        filtered[i] = signal[i]
            + alpha * (signal[i] - signal[i - 2])
            - beta * filtered[i - 1]
            + 0.1 * filtered[i - 2];
    }

    // Áp dụng bộ lọc Kalman để làm mượt tín hiệu
    let mut kalman = KalmanFilter::new();
    let mut kalman_filtered = [0.0; BUFFER_SIZE];
    for i in 0..filtered.len() {
        kalman_filtered[i] = kalman.update(filtered[i]);
    }

    kalman_filtered
}
```

- **Integration**:
  - The bandpass filter processes the raw IR signal first, isolating the 0.5–5 Hz range.
  - The Kalman filter then smooths the bandpass-filtered signal, reducing residual noise.
  - The final output (`kalman_filtered`) is used for printing and can be used for further processing (e.g., peak detection or SpO2 calculation).

### Usage in the Main Loop
The filters are applied in the main loop when the IR buffer is full:

```rust
if buffer_index == 0 {
    let filtered_data = filter_signal(&ir_buffer, 0.5, 5.0, SAMPLE_RATE);
    for (j, &filtered_value) in filtered_data.iter().enumerate() {
        println!("Filtered Sample {}: {:.2}", j, filtered_value);
    }
}
```

- The `ir_buffer` (100 samples) is filled with raw IR data from the MAX30102.
- When full (`buffer_index == 0`), `filter_signal` is called to apply both filters.
- The filtered samples are printed with two decimal places for debugging.

## 4. Recommendations
- **Tuning the Filters**:
  - **Bandpass Filter**: Adjust `lowcut` (e.g., 0.7 Hz) or `highcut` (e.g., 4 Hz) if the PPG signal is not clear or contains residual noise.
  - **Kalman Filter**: Tune `q` (process noise) and `r` (measurement noise):
    - Increase `q` (e.g., 0.1) if the signal changes rapidly.
    - Increase `r` (e.g., 0.5) if measurements are noisy.
- **Verification**: Check the UART output (`Filtered Sample`) to ensure the PPG signal shows clear, periodic peaks corresponding to heartbeats.
- **Next Steps**:
  - Implement peak detection on the filtered signal to calculate heart rate.
  - Process both Red and IR signals to calculate SpO2 using the ratio R = (AC_Red / DC_Red) / (AC_IR / DC_IR).

## 5. Conclusion
The bandpass filter isolates the PPG signal's relevant frequency range (0.5–5 Hz), while the Kalman filter smooths the signal to reduce residual noise, particularly from motion artifacts. Together, they produce a clean signal suitable for heart rate and SpO2 calculations. The implementation is optimized for the `no_std` environment, using static arrays and the `libm` crate to ensure compatibility with the ESP32-C6.