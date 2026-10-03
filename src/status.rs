//! Payment lifecycle status (6-state) with a validated transition table.
//!
//! Mirrors the state machine shared by the ecom-engine and BlocMarket
//! payment services: `Pending -> Confirming -> Completed`, with `Failed`
//! and `Expired` terminal-from-pending, and `Refunded` from `Completed`.

use serde::{Deserialize, Serialize};

/// Payment lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    /// Created, awaiting provider confirmation.
    Pending,
    /// Provider has partially confirmed (e.g. crypto mempool).
    Confirming,
    /// Settled.
    Completed,
    /// Rejected or errored before settlement.
    Failed,
    /// Timed out without confirmation.
    Expired,
    /// Settled then returned to the payer.
    Refunded,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PaymentStatus {
    /// Wire/DB representation (lowercase).
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Confirming => "confirming",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Expired => "expired",
            Self::Refunded => "refunded",
        }
    }

    /// True once money has settled (refunds can only follow settlement).
    #[must_use]
    pub fn is_settled(&self) -> bool {
        matches!(self, Self::Completed | Self::Refunded)
    }

    /// True while the payment can still be confirmed by a provider.
    #[must_use]
    pub fn is_open(&self) -> bool {
        matches!(self, Self::Pending | Self::Confirming)
    }

    /// Validates a transition and returns the target status.
    ///
    /// CAS-guard semantics matching the SQL `WHERE status = $current`
    /// updates both consumers run before mutating persistence:
    /// - `Pending -> Confirming | Completed | Failed | Expired`
    /// - `Confirming -> Completed | Failed | Expired`
    /// - `Completed -> Refunded`
    ///
    /// # Errors
    ///
    /// [`TransitionError::Illegal`] when the pair is not in the table.
    pub fn transition(&self, target: Self) -> Result<Self, TransitionError> {
        let valid = matches!(
            (self, target),
            (Self::Pending, Self::Confirming)
                | (Self::Pending, Self::Completed)
                | (Self::Pending, Self::Failed)
                | (Self::Pending, Self::Expired)
                | (Self::Confirming, Self::Completed)
                | (Self::Confirming, Self::Failed)
                | (Self::Confirming, Self::Expired)
                | (Self::Completed, Self::Refunded)
        );
        if valid {
            Ok(target)
        } else {
            Err(TransitionError {
                from: *self,
                to: target,
            })
        }
    }
}

/// Illegal status transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("invalid payment status transition from {from} to {to}")]
pub struct TransitionError {
    /// Current status.
    pub from: PaymentStatus,
    /// Attempted target status.
    pub to: PaymentStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_pending_confirming_completed() {
        let s = PaymentStatus::Pending
            .transition(PaymentStatus::Confirming)
            .unwrap()
            .transition(PaymentStatus::Completed)
            .unwrap();
        assert_eq!(s, PaymentStatus::Completed);
    }

    #[test]
    fn direct_completion_from_pending_is_allowed() {
        assert!(PaymentStatus::Pending
            .transition(PaymentStatus::Completed)
            .is_ok());
    }

    #[test]
    fn refund_requires_settlement() {
        assert!(PaymentStatus::Pending.transition(PaymentStatus::Refunded).is_err());
        assert!(PaymentStatus::Completed
            .transition(PaymentStatus::Refunded)
            .is_ok());
    }

    #[test]
    fn terminal_states_are_closed() {
        for terminal in [
            PaymentStatus::Failed,
            PaymentStatus::Expired,
            PaymentStatus::Refunded,
        ] {
            assert!(terminal.transition(PaymentStatus::Pending).is_err());
            assert!(terminal.transition(PaymentStatus::Completed).is_err());
        }
    }

    #[test]
    fn as_str_roundtrip_is_lowercase() {
        for s in [
            PaymentStatus::Pending,
            PaymentStatus::Confirming,
            PaymentStatus::Completed,
            PaymentStatus::Failed,
            PaymentStatus::Expired,
            PaymentStatus::Refunded,
        ] {
            assert_eq!(s.as_str(), serde_json::to_string(&s).unwrap().trim_matches('"'));
        }
    }
}
