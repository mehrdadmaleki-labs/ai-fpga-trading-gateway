use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct RiskState {
    /// Current signed position by symbol.
    /// Positive = long, negative = short.
    pub positions: HashMap<String, i64>,

    /// Reference price used by the price-collar check.
    pub reference_prices_cents: HashMap<String, u64>,

    // TODO(HINT):
    // Keep mutable trading state separate from static policy.
    // This separation will make a future FPGA register/configuration
    // interface much easier to design.
}

impl RiskState {
    pub fn position(&self, symbol: &str) -> i64 {
        *self.positions.get(symbol).unwrap_or(&0)
    }

    pub fn reference_price(&self, symbol: &str) -> Option<u64> {
        self.reference_prices_cents.get(symbol).copied()
    }
}
