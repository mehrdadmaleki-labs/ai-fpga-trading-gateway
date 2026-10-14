use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub side: Side,
    pub quantity: u64,
    /// Price in integer cents.
    ///
    /// TODO(HINT):
    /// Do not use f64 in the latency-critical path for money.
    /// Integer fixed-point representation is easier to reason about.
    pub price_cents: u64,
}

impl Order {
    pub fn notional_cents(&self) -> Option<u64> {
        // TODO(HINT):
        // Multiplication can overflow. In a real risk gateway,
        // an overflow must reject the order rather than wrap silently.
        self.quantity.checked_mul(self.price_cents)
    }
}
