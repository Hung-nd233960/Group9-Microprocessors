# Introduction to the Kalman Filter and Its Application in Sensor Fusion

The **Kalman Filter** is a powerful mathematical tool used for estimating the true state of a system from noisy measurements. It is often implemented in systems where sensor data is noisy or incomplete, providing accurate and real-time estimates of quantities such as position, velocity, and orientation.

---

## How the Kalman Filter Works

At its core, the Kalman Filter performs recursive estimation through two main steps:

### 1. Prediction
- Projects the current state estimate forward in time based on the system's dynamics.
- Calculates the predicted estimate's uncertainty.

### 2. Update
- Incorporates new measurement data.
- Adjusts the predicted estimate using the measurement, weighted by their respective uncertainties.

This process leverages probabilistic models assuming Gaussian noise, optimizing the estimate's accuracy over time.

---

## Why Use Kalman Filter?

- **Noise reduction**: Combines multiple noisy sensors to produce a cleaner result.
- **Optimality**: Under Gaussian noise assumptions, it provides the best linear unbiased estimate.
- **Sensor fusion**: Integrates different measurements (e.g., accelerometer and gyroscope) efficiently.
- **Real-time processing**: Suitable for embedded systems and continuous data streams.

---

## Application: Estimating Tilt Angle with Sensor Data

In embedded systems, such as drones or robotics, you often measure tilt angles with imu sensors. Accelerometers measure gravity but are susceptible to linear acceleration disturbances, while gyroscopes measure angular velocity but can drift over time.

By combining both using a Kalman filter, you can obtain a more accurate and stable estimate of tilt angles.

---

## Understanding the Provided `Kalman` Class

Below is the class implementation that implements the Kalman filter for a single-variable estimation (e.g., an angle):

```cpp  
#ifndef _Kalman_h_  
#define _Kalman_h_  

class Kalman {  
public:  
    Kalman();  

    // This function estimates the current angle based on sensor inputs  
    // newAngle: angle measured from accelerometer (degrees)  
    // newRate: angular velocity from gyroscope (degrees/sec)  
    // dt: elapsed time in seconds  
    float getAngle(float newAngle, float newRate, float dt);  

    void setAngle(float angle);        // Initialize the angle  
    float getRate();                   // Get the current rate (gyroscope data without bias)  

    // Functions to set the noise covariance parameters  
    void setQangle(float Q_angle);     // Set process noise variance for the angle  
    void setQbias(float Q_bias);       // Set process noise variance for gyro bias  
    void setRmeasure(float R_measure); // Set measurement noise variance  

    // Functions to get current parameter values  
    float getQangle();  
    float getQbias();  
    float getRmeasure();  

private:  
    // Kalman filter variables  
    float Q_angle;   // Variance in acceleration measurement (process noise for angle)  
    float Q_bias;    // Variance in gyro bias (process noise for bias)  
    float R_measure; // Variance in measurement noise  

    float angle;     // The filtered estimate of the angle  
    float bias;      // Estimated gyro bias  
    float rate;      // Unbiased rate of change of angle  

    float P[2][2];   // Error covariance matrix  
};  

#endif  
```
## In-Depth Explanation of the Code
### Variables
- Q_angle (process noise for the angle): Represents the uncertainty in the process model for the angle; higher values mean trusting the model less.
- Q_bias (process noise for the gyro bias): Represents the rate at which the gyro bias changes; tuning affects how quickly the filter adapts to bias drift.
- R_measure (measurement noise variance): Reflects how noisy your sensor measurements are; smaller values give more weight to new measurements.
### State Variables
- angle: The current estimated tilt angle.
- bias: The estimated bias of gyroscopic measurements.
- rate: The corrected angular velocity, obtained by removing the estimated bias from the gyroscope measurement.
### Covariance Matrix P
- Tracks the uncertainty (error covariance) in the estimation.
- Initialized as a 2x2 matrix and updated every iteration.
## How the getAngle() Function Works
Here's a simplified explanation of this core method:
```
cpp
float Kalman::getAngle(float newAngle, float newRate, float dt) {  
    // 1. Predict phase  
    rate = newRate - bias; // Remove bias from gyroscope measurement  
    angle += dt * rate;    // Predict new angle based on rate  
    
    // Update error covariance matrix P  
    P[0][0] += dt * (dt*P[1][1] - P[0][1] - P[1][0] + Q_angle);  
    P[0][1] -= dt * P[1][1];  
    P[1][0] -= dt * P[1][1];  
    P[1][1] += Q_bias * dt;  

    // 2. Kalman Gain Calculation  
    float S = P[0][0] + R_measure; // Innovation covariance  
    float K[2]; // Kalman gain  
    K[0] = P[0][0] / S;  
    K[1] = P[1][0] / S;  

    // 3. Update estimate with measurement `newAngle`  
    float y = newAngle - angle; // Innovation or residual  
    angle += K[0] * y;  
    bias += K[1] * y;  

    // Update error covariance matrix after measurement update  
    float P00_temp = P[0][0];  
    float P01_temp = P[0][1];  

    P[0][0] -= K[0] * P00_temp;  
    P[0][1] -= K[0] * P01_temp;  
    P[1][0] -= K[1] * P00_temp;  
    P[1][1] -= K[1] * P[01_temp];  

    return angle; // Return the filtered angle estimate  
}  
```
This function performs both the prediction step (projecting previous state forward) and the correction step (refining estimate with the new measurement).
## Practical Implementation
In an embedded system observing tilt, you typically do the following continuously:

1. Read newAngle from an accelerometer sensor.
2. Read newRate (angular velocity) from a gyroscope.
3. Call getAngle(newAngle, newRate, dt) at each time step.

This method will yield a stable, filtered estimate of the angle, significantly improving over raw sensor values affected by noise.

## Tuning the Filter Parameters
Proper tuning of the parameters (Q_angle, Q_bias, R_measure) is crucial:

- Increase Q_angle or Q_bias if your system is highly dynamic and needs to trust the model less.
- Decrease R_measure if your sensor's noise is low to give more weight to measurements.
- The parameters depend on sensor specifications and application dynamics, often requiring empirical tuning.

## Summary
This Kalman class encapsulates a detailed implementation of the Kalman filter for a single variable, optimized for tilt estimation:

- It fuses accelerometer and gyroscope data to counteract noise and drift.
- It adaptively updates the estimates based on incoming data.
- It simplifies complex sensor fusion algorithms into a usable class suitable for real-time embedded applications.


