#[derive(Debug, Clone)]
pub struct RiskPolicy {
    /// Maximum value of one order, in cents.
    pub max_order_notional_cents: u64,

    /// Maximum absolute position per symbol.
    pub max_abs_position: i64,

    /// Maximum allowed deviation from the reference price, in basis points.
    /// 100 bps = 1%.
    pub max_price_deviation_bps: u64,

    /// Global emergency stop.
    pub kill_switch: bool,

    // TODO(HINT):
    // Later add:
    // - max_orders_per_second
    // - max_total_exposure
    // - symbol allow/deny lists
    // - strategy-specific limits
    // - per-symbol position limits
}

impl Default for RiskPolicy {
    fn default() -> Self {
        Self {
            max_order_notional_cents: 50_000_000, // €/$500,000 equivalent in cents
            max_abs_position: 10_000,
            max_price_deviation_bps: 200, // 2%
            kill_switch: false,
        }
    }
}
