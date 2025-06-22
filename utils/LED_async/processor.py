import re
import numpy as np
import matplotlib.pyplot as plt
from sklearn.linear_model import LinearRegression

def parse_microseconds(file_path):
    times = []
    with open(file_path, 'r', encoding="utf-8") as f:
        for line in f:
            # Handle CSV or raw log format
            line = line.strip()
            if line == "":
                continue
            if re.match(r'^\d+$', line):  # Plain CSV number
                times.append(int(line))
            else:  # Text line like "LED toggled at 1234567 µs since epoch"
                match = re.search(r'(\d+)\s*µs', line)
                if match:
                    times.append(int(match.group(1)))
    return np.array(times)

def analyze_timer(times_us):
    n_cycles = np.arange(len(times_us)).reshape(-1, 1)
    times_us = times_us.reshape(-1, 1)

    # Fit linear model: t = a*n + b
    model = LinearRegression()
    model.fit(n_cycles, times_us)

    predicted = model.predict(n_cycles)
    residuals = times_us - predicted

    print(f"Estimated period (a): {model.coef_[0][0]:.2f} µs")
    print(f"Estimated start time (b): {model.intercept_[0]:.2f} µs")
    print(f"Mean absolute error: {np.mean(np.abs(residuals)):.2f} µs")
    print(f"Standard deviation of residuals: {np.std(residuals):.2f} µs")

    # Plot actual vs predicted
    plt.figure(figsize=(10, 6))
    plt.plot(n_cycles, times_us, label="Actual", marker='o', linestyle='--')
    plt.plot(n_cycles, predicted, label="Linear Fit", color='red')
    plt.title("Timer Ticks vs Epoch Time")
    plt.xlabel("Cycle Index (n)")
    plt.ylabel("Time since Epoch (µs)")
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.show()

    # Plot residuals
    plt.figure(figsize=(10, 4))
    plt.plot(n_cycles, residuals, marker='x', color='purple')
    plt.title("Residuals (Actual - Fitted)")
    plt.xlabel("Cycle Index (n)")
    plt.ylabel("Residual (µs)")
    plt.grid(True)
    plt.tight_layout()
    plt.show()

# === USAGE ===
# Replace with your file path:
file_path = "output_soft.csv"
times = parse_microseconds(file_path)
analyze_timer(times)
