# billing-kit

[![docs.rs](https://docs.rs/billing-kit/badge.svg)](https://docs.rs/billing-kit)
[![crates.io](https://img.shields.io/crates/v/billing-kit.svg)](https://crates.io/crates/billing-kit)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Billing primitives on top of [`decimal-money`](https://github.com/WyattAu/money).

- Re-exports `Currency`, `CurrencyAmount`, `MoneyError`
- `Price` — net + tax_rate% (VAT), with `gross()`, `tax_amount()`, `discount_of_gross()`, `gross_minus_discount()`

## Quick start

```rust
use billing_kit::{Price, Currency};
use rust_decimal_macros::dec;

let p = Price::new(dec!(100), Currency::GBP, dec!(20)).unwrap();
assert_eq!(p.gross().amount, dec!(120));
```

## Assessment vs ecom-engine

`ecom-core::Money` now wraps `CurrencyAmount` (GBP) internally; `billing-kit` is **not required** for the GBP migration (which preserved `Money(Decimal)` storage + `sqlx(transparent)`). Scaffold this crate when extracting promo/insurance/VAT math from `ecom-http` into a shared billing crate.
