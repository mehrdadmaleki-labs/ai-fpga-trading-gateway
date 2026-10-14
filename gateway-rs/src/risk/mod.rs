pub mod checks;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectReason {
    KillSwitchActive,
    OrderValueExceeded,
    PositionLimitExceeded,
    PriceOutsideCollar,
    MissingReferencePrice,
    ArithmeticOverflow,

    // TODO(HINT):
    // Add explicit rejection reasons for every future risk rule.
    // Avoid one generic "InvalidOrder": good observability matters.
}
