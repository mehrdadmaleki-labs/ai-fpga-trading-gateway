use crate::{
    order::{Order, Side},
    policy::RiskPolicy,
    risk::RejectReason,
    state::RiskState,
};

pub fn check_kill_switch(policy: &RiskPolicy) -> Result<(), RejectReason> {
    if policy.kill_switch {
        return Err(RejectReason::KillSwitchActive);
    }
    Ok(())
}

pub fn check_order_value(order: &Order, policy: &RiskPolicy) -> Result<(), RejectReason> {
    let notional = order
        .notional_cents()
        .ok_or(RejectReason::ArithmeticOverflow)?;

    if notional > policy.max_order_notional_cents {
        return Err(RejectReason::OrderValueExceeded);
    }

    Ok(())
}

pub fn projected_position(order: &Order, state: &RiskState) -> Result<i64, RejectReason> {
    let qty = i64::try_from(order.quantity).map_err(|_| RejectReason::ArithmeticOverflow)?;
    let current = state.position(&order.symbol);

    match order.side {
        Side::Buy => current.checked_add(qty),
        Side::Sell => current.checked_sub(qty),
    }
    .ok_or(RejectReason::ArithmeticOverflow)
}

pub fn check_position(
    order: &Order,
    policy: &RiskPolicy,
    state: &RiskState,
) -> Result<(), RejectReason> {
    let projected = projected_position(order, state)?;

    if projected.unsigned_abs() > policy.max_abs_position.unsigned_abs() {
        return Err(RejectReason::PositionLimitExceeded);
    }

    Ok(())
}

pub fn check_price_collar(
    order: &Order,
    policy: &RiskPolicy,
    state: &RiskState,
) -> Result<(), RejectReason> {
    let reference = state
        .reference_price(&order.symbol)
        .ok_or(RejectReason::MissingReferencePrice)?;

    // allowed_delta = reference * bps / 10_000
    //
    // TODO(HINT):
    // Think carefully about integer rounding.
    // Decide whether the safest behaviour is rounding up or down.
    let allowed_delta = reference
        .checked_mul(policy.max_price_deviation_bps)
        .ok_or(RejectReason::ArithmeticOverflow)?
        / 10_000;

    let lower = reference.saturating_sub(allowed_delta);
    let upper = reference.saturating_add(allowed_delta);

    if order.price_cents < lower || order.price_cents > upper {
        return Err(RejectReason::PriceOutsideCollar);
    }

    Ok(())
}
