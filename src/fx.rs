//! FX drift guard — rejects provider-quoted rates that drift too far from
//! the rate locked at order time.
//!
//! Ported from `ecom-core::fx` (ecom-engine): `FX_DRIFT_TOLERANCE_BPS = 200`
//! (2%), with an identity/zero locked rate trivially passing (GBP orders).

use rust_decimal::Decimal;

/// Drift tolerance in basis points: 200 bps = 2%.
pub const FX_DRIFT_TOLERANCE_BPS: i64 = 200;

/// Seconds a locked FX rate is considered fresh (30 minutes).
pub const FX_LOCK_WINDOW_SECONDS: i64 = 30 * 60;

/// Returns true when the live rate has drifted beyond the tolerance from the
/// locked rate. A zero locked rate trivially passes (division guarded).
#[must_use]
pub fn exceeds_fx_tolerance(locked: Decimal, live: Decimal) -> bool {
    if locked.is_zero() {
        return false;
    }
    let tolerance = Decimal::new(FX_DRIFT_TOLERANCE_BPS, 4); // 0.02 = 2%
    let drift = (live - locked).abs() / locked;
    drift > tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(s: &str) -> Decimal {
        s.parse().expect("valid decimal literal")
    }

    #[test]
    fn within_tolerance_passes() {
        assert!(!exceeds_fx_tolerance(dec("1.25"), dec("1.26")));
    }

    #[test]
    fn beyond_tolerance_rejects() {
        assert!(exceeds_fx_tolerance(dec("1.00"), dec("1.05")));
    }

    #[test]
    fn exactly_at_tolerance_boundary_passes() {
        // 200 bps of 1.00 = 0.02; drift == tolerance is not *beyond* it.
        assert!(!exceeds_fx_tolerance(dec("1.00"), dec("1.02")));
    }

    #[test]
    fn zero_locked_rate_trivially_passes() {
        assert!(!exceeds_fx_tolerance(dec("0"), dec("99")));
    }
}
