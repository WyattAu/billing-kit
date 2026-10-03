#![forbid(unsafe_code)]
#![deny(missing_docs)]
// Test code asserts invariants directly; unwrap/expect keeps failures loud.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
//! Billing primitives built on `decimal-money`.
//!
//! Re-exports [`decimal_money::CurrencyAmount`] and [`decimal_money::Currency`]
//! and provides [`Price`] — a net/gross/tax helper that mirrors
//! `ecom-core::Money::percentage_of` and the promo/insurance math in
//! `ecom-http` (`crates/ecom-http/tests/integration_test.rs`).

pub use decimal_money::{Currency, CurrencyAmount, MoneyError};
pub use rust_decimal::Decimal;

mod fx;
mod price;
mod status;
mod webhooks;

pub use fx::{exceeds_fx_tolerance, FX_DRIFT_TOLERANCE_BPS, FX_LOCK_WINDOW_SECONDS};
pub use price::{Price, PriceError};
pub use status::{PaymentStatus, TransitionError};
pub use webhooks::{verify_bacs_dd, verify_fena, verify_wallid};
