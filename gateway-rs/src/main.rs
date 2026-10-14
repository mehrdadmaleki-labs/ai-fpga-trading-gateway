use gateway_rs::{Order, RiskEngine, RiskPolicy, RiskState, Side};

fn main() {
    let policy = RiskPolicy::default();

    let mut state = RiskState::default();
    state
        .reference_prices_cents
        .insert("AAPL".to_string(), 20_000); // 200.00

    let engine = RiskEngine::new(policy, state);

    let order = Order {
        id: 1,
        symbol: "AAPL".to_string(),
        side: Side::Buy,
        quantity: 1_000,
        price_cents: 20_100, // 201.00
    };

    let decision = engine.evaluate(&order);

    println!("Order: {:?}", order);
    println!("Decision: {:?}", decision);

    // TODO(HINT):
    // Next step:
    // 1. read newline-delimited JSON from stdin or UDP,
    // 2. deserialize into Order,
    // 3. run evaluate(),
    // 4. emit a compact decision message.
}
