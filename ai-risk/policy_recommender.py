"""
Translate AI observations into conservative policy recommendations.

The key rule:
AI may recommend.
A deterministic policy-validation layer must approve the change.
"""

from dataclasses import dataclass


@dataclass
class PolicyRecommendation:
    max_order_notional_multiplier: float
    reason: str


def recommend(anomaly: bool, volatility: float) -> PolicyRecommendation:
    if anomaly and volatility > 0.05:
        return PolicyRecommendation(
            max_order_notional_multiplier=0.5,
            reason="High anomaly score combined with high volatility",
        )

    return PolicyRecommendation(
        max_order_notional_multiplier=1.0,
        reason="No policy reduction recommended",
    )


# TODO(HINT):
# Next version:
# 1. represent recommendations as structured JSON;
# 2. send them to the Rust control plane;
# 3. Rust validates min/max ranges;
# 4. a human-approval mode can be used before auto-activation;
# 5. only then update runtime policy / future FPGA registers.
