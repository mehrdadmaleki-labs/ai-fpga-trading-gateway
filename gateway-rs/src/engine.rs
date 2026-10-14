use crate::{
    order::Order,
    policy::RiskPolicy,
    risk::{
        checks::{check_kill_switch, check_order_value, check_position, check_price_collar},
        RejectReason,
    },
    state::RiskState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Approved,
    Rejected(RejectReason),
}

pub struct RiskEngine {
    pub policy: RiskPolicy,
    pub state: RiskState,
}

impl RiskEngine {
    pub fn new(policy: RiskPolicy, state: RiskState) -> Self {
        Self { policy, state }
    }

    pub fn evaluate(&self, order: &Order) -> Decision {
        // IMPORTANT DESIGN IDEA:
        // Keep the hot-path checks explicit and ordered.
        //
        // TODO(HINT):
        // Later benchmark whether ordering cheap/high-rejection checks first
        // improves average latency without hurting determinism.

        let checks = [
            check_kill_switch(&self.policy),
            check_order_value(order, &self.policy),
            check_position(order, &self.policy, &self.state),
            check_price_collar(order, &self.policy, &self.state),
        ];

        for result in checks {
            if let Err(reason) = result {
                return Decision::Rejected(reason);
            }
        }

        Decision::Approved
    }

    pub fn apply_fill(&mut self, order: &Order) {
        // TODO(HINT):
        // For Version 1, assume the approved order fills completely.
        // Later distinguish:
        // approved order != filled order
        // and add partial fills / open orders / cancellations.

        let qty = match i64::try_from(order.quantity) {
            Ok(q) => q,
            Err(_) => return,
        };

        let entry = self
            .state
            .positions
            .entry(order.symbol.clone())
            .or_insert(0);

        match order.side {
            crate::order::Side::Buy => *entry += qty,
            crate::order::Side::Sell => *entry -= qty,
        }
    }
}
