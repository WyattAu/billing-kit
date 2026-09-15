//! Config-knob behavior matrix for billing-kit's `Price`.
//!
//! Knobs: `net` amount, `tax_rate`, currency, and the discount percent
//! argument. Each must observably change the computed output.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use billing_kit::{Currency, Price};
use rust_decimal_macros::dec;

// --- tax_rate knob --------------------------------------------------------

#[test]
fn knob_tax_rate_changes_tax_and_gross() {
    let net = dec!(100);

    let zero = Price::new(net, Currency::GBP, dec!(0)).unwrap();
    let vat20 = Price::new(net, Currency::GBP, dec!(20)).unwrap();
    let vat100 = Price::new(net, Currency::GBP, dec!(100)).unwrap();

    assert_eq!(zero.tax_amount().amount, dec!(0));
    assert_eq!(zero.gross().amount, dec!(100));

    assert_eq!(vat20.tax_amount().amount, dec!(20));
    assert_eq!(vat20.gross().amount, dec!(120));

    assert_eq!(vat100.tax_amount().amount, dec!(100));
    assert_eq!(vat100.gross().amount, dec!(200));
}

#[test]
fn knob_tax_rate_rejects_out_of_range() {
    assert!(Price::new(dec!(100), Currency::GBP, dec!(101)).is_err());
    assert!(Price::new(dec!(100), Currency::GBP, dec!(-1)).is_err());
    // Boundary values are the documented contract.
    assert!(Price::new(dec!(100), Currency::GBP, dec!(0)).is_ok());
    assert!(Price::new(dec!(100), Currency::GBP, dec!(100)).is_ok());
}

// --- net amount knob -------------------------------------------------------

#[test]
fn knob_net_amount_scales_tax_and_gross() {
    let small = Price::new(dec!(50), Currency::GBP, dec!(20)).unwrap();
    let large = Price::new(dec!(500), Currency::GBP, dec!(20)).unwrap();

    assert_eq!(small.gross().amount, dec!(60));
    assert_eq!(large.gross().amount, dec!(600));
    assert_ne!(small.gross().amount, large.gross().amount);
}

// --- currency knob ---------------------------------------------------------

#[test]
fn knob_currency_is_carried_through_all_outputs() {
    let gbp = Price::new(dec!(100), Currency::GBP, dec!(20)).unwrap();
    let usd = Price::new(dec!(100), Currency::USD, dec!(20)).unwrap();

    assert_eq!(gbp.tax_amount().currency, Currency::GBP);
    assert_eq!(gbp.gross().currency, Currency::GBP);
    assert_eq!(usd.tax_amount().currency, Currency::USD);
    assert_eq!(usd.gross().currency, Currency::USD);
    assert_ne!(gbp.gross().currency, usd.gross().currency);
}

// --- discount percent argument ---------------------------------------------

#[test]
fn knob_discount_percent_changes_discounted_gross() {
    let p = Price::new(dec!(100), Currency::GBP, dec!(20)).unwrap(); // gross 120

    assert_eq!(p.gross_minus_discount(dec!(0)).amount, dec!(120));
    assert_eq!(p.gross_minus_discount(dec!(50)).amount, dec!(60));
    assert_eq!(p.gross_minus_discount(dec!(100)).amount, dec!(0));
    // Clamped above 100: never negative.
    assert_eq!(p.gross_minus_discount(dec!(150)).amount, dec!(0));
}

#[test]
fn knob_discount_of_gross_is_linear_in_percent() {
    let p = Price::new(dec!(200), Currency::GBP, dec!(0)).unwrap();

    let d10 = p.discount_of_gross(dec!(10)).amount;
    let d20 = p.discount_of_gross(dec!(20)).amount;
    assert_eq!(d10, dec!(20));
    assert_eq!(d20, dec!(40));
    assert_eq!(d20, d10 + d10, "doubling the knob must double the discount");
}
