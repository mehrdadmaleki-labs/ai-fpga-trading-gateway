use gateway_rs::{
    Decision, Order, RiskEngine, RiskPolicy, RiskState, Side,
};
use gateway_rs::risk::RejectReason;

fn engine() -> RiskEngine {
    let policy = RiskPolicy::default();
    let mut state = RiskState::default();
    state.reference_prices_cents.insert("AAPL".into(), 20_000);
    RiskEngine::new(policy, state)
}

#[test]
fn accepts_order_inside_limits() {
    let e = engine();
    let order = Order {
        id: 1,
        symbol: "AAPL".into(),
        side: Side::Buy,
        quantity: 100,
        price_cents: 20_000,
    };

    assert_eq!(e.evaluate(&order), Decision::Approved);
}

#[test]
fn rejects_order_over_notional_limit() {
    let e = engine();
    let order = Order {
        id: 2,
        symbol: "AAPL".into(),
        side: Side::Buy,
        quantity: 10_000,
        price_cents: 20_000,
    };

    assert_eq!(
        e.evaluate(&order),
        Decision::Rejected(RejectReason::OrderValueExceeded)
    );
}

#[test]
fn rejects_price_outside_collar() {
    let e = engine();
    let order = Order {
        id: 3,
        symbol: "AAPL".into(),
        side: Side::Buy,
        quantity: 100,
        price_cents: 21_000, // 5% above reference
    };

    assert_eq!(
        e.evaluate(&order),
        Decision::Rejected(RejectReason::PriceOutsideCollar)
    );
}

// TODO(HINT):
// Add tests for:
// - kill switch
// - selling into a short position
// - exact boundary values
// - multiplication overflow
// - missing reference price
// - position exactly equal to max
// - a sequence of fills that eventually reaches the limit
