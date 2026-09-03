#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Billing primitives built on `decimal-money`.
//!
//! Re-exports [`decimal_money::CurrencyAmount`] and [`decimal_money::Currency`]
//! and provides [`Price`] — a net/gross/tax helper that mirrors
//! `ecom-core::Money::percentage_of` and the promo/insurance math in
//! `ecom-http` (`crates/ecom-http/tests/integration_test.rs`).

pub use decimal_money::{Currency, CurrencyAmount, MoneyError};
pub use rust_decimal::Decimal;

mod price;
pub use price::{Price, PriceError};
