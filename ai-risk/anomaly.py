"""
Minimal AI risk prototype.

Goal:
Learn "normal" order-flow behaviour and assign an anomaly score to new windows.

This is intentionally offline first.
Do NOT put this Python model in the nanosecond/microsecond critical order path.
"""

from dataclasses import dataclass
import numpy as np
from sklearn.ensemble import IsolationForest


@dataclass
class RiskWindow:
    orders_per_second: float
    mean_order_value: float
    cancellation_ratio: float
    rejection_ratio: float
    volatility: float

    def as_array(self) -> np.ndarray:
        return np.array([
            self.orders_per_second,
            self.mean_order_value,
            self.cancellation_ratio,
            self.rejection_ratio,
            self.volatility,
        ], dtype=float)


def build_demo_training_data(n: int = 1000) -> np.ndarray:
    rng = np.random.default_rng(42)

    # TODO(HINT):
    # Replace synthetic Gaussian data with windows produced by your simulator.
    # Be careful: features have very different scales.
    # Later add StandardScaler in a Pipeline.
    return np.column_stack([
        rng.normal(5_000, 500, n),
        rng.normal(30_000, 4_000, n),
        rng.normal(0.40, 0.05, n),
        rng.normal(0.02, 0.01, n),
        rng.normal(0.015, 0.004, n),
    ])


def train_model(x: np.ndarray) -> IsolationForest:
    model = IsolationForest(
        n_estimators=200,
        contamination=0.01,
        random_state=42,
    )
    model.fit(x)
    return model


def main() -> None:
    x = build_demo_training_data()
    model = train_model(x)

    current = RiskWindow(
        orders_per_second=27_000,
        mean_order_value=180_000,
        cancellation_ratio=0.93,
        rejection_ratio=0.20,
        volatility=0.08,
    )

    row = current.as_array().reshape(1, -1)
    prediction = model.predict(row)[0]  # -1 anomaly, +1 normal
    raw_score = model.decision_function(row)[0]

    print("Current window:", current)
    print("Prediction:", "ANOMALY" if prediction == -1 else "NORMAL")
    print("Decision-function score:", raw_score)

    # TODO(HINT):
    # Do NOT immediately convert "ANOMALY" into a kill-switch action.
    # First map anomaly signals to *recommendations*, then validate
    # recommendations against deterministic hard limits.


if __name__ == "__main__":
    main()
