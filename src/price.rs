use decimal_money::{Currency, CurrencyAmount, MoneyError};
use rust_decimal::Decimal;

/// Errors for [`Price`] construction.
#[derive(Debug, thiserror::Error)]
pub enum PriceError {
    /// Tax rate outside 0..=100.
    #[error("tax rate must be between 0 and 100, got {0}")]
    InvalidTaxRate(Decimal),
    /// Currency mismatch between net and tax operations.
    #[error(transparent)]
    Money(#[from] MoneyError),
}

/// A net price plus a percentage tax rate (e.g. VAT 20%).
///
/// All amounts share a single [`Currency`]; operations return
/// [`MoneyError::CurrencyMismatch`] if currencies diverge.
///
/// Mirrors `ecom_core::Money::percentage_of` — `tax = net * rate / 100`
/// — but surfaces the rate explicitly for invoicing.
///
/// ```rust
/// use billing_kit::{Price, Currency};
/// use rust_decimal_macros::dec;
/// let p = Price::new(dec!(100), Currency::GBP, dec!(20)).unwrap();
/// assert_eq!(p.tax_amount().amount, dec!(20));
/// assert_eq!(p.gross().amount, dec!(120));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Price {
    /// Net (pre-tax) amount.
    pub net: CurrencyAmount,
    /// Tax rate as percentage (0-100).
    pub tax_rate: Decimal,
}

impl Price {
    /// Create a new `Price`.
    ///
    /// # Errors
    /// Returns [`PriceError::InvalidTaxRate`] if `tax_rate` is not in `0..=100`.
    pub fn new(net_amount: impl Into<Decimal>, currency: Currency, tax_rate: Decimal) -> Result<Self, PriceError> {
        if tax_rate < Decimal::ZERO || tax_rate > Decimal::from(100) {
            return Err(PriceError::InvalidTaxRate(tax_rate));
        }
        Ok(Self {
            net: CurrencyAmount::new(net_amount, currency),
            tax_rate,
        })
    }

    /// Tax amount = `net * tax_rate / 100`.
    #[must_use]
    pub fn tax_amount(&self) -> CurrencyAmount {
        let tax = self.net.amount * self.tax_rate / Decimal::from(100);
        CurrencyAmount::new(tax, self.net.currency)
    }

    /// Gross amount = `net + tax`.
    #[must_use]
    pub fn gross(&self) -> CurrencyAmount {
        // unwrap: same currency, decimal addition cannot mismatch.
        (self.net.clone() + self.tax_amount()).expect("same currency")
    }

    /// Discount amount at `percent`% of gross (0-100).
    #[must_use]
    pub fn discount_of_gross(&self, percent: Decimal) -> CurrencyAmount {
        let gross = self.gross();
        CurrencyAmount::new(gross.amount * percent / Decimal::from(100), gross.currency)
    }

    /// Apply a promo discount to gross, returning the discounted gross.
    ///
    /// Clamps `percent` to 0..=100 and never returns a negative amount.
    #[must_use]
    pub fn gross_minus_discount(&self, percent: Decimal) -> CurrencyAmount {
        let gross = self.gross();
        let pct = percent.clamp(Decimal::ZERO, Decimal::from(100));
        let discount = gross.amount * pct / Decimal::from(100);
        let discounted = (gross.amount - discount).max(Decimal::ZERO);
        CurrencyAmount::new(discounted, gross.currency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn vat_20() {
        let p = Price::new(dec!(100), Currency::GBP, dec!(20)).unwrap();
        assert_eq!(p.tax_amount().amount, dec!(20));
        assert_eq!(p.gross().amount, dec!(120));
    }

    #[test]
    fn zero_tax() {
        let p = Price::new(dec!(50), Currency::GBP, dec!(0)).unwrap();
        assert_eq!(p.gross().amount, dec!(50));
    }

    #[test]
    fn invalid_tax_rejected() {
        assert!(Price::new(dec!(10), Currency::GBP, dec!(101)).is_err());
        assert!(Price::new(dec!(10), Currency::GBP, dec!(-1)).is_err());
    }

    #[test]
    fn discount_of_gross() {
        let p = Price::new(dec!(100), Currency::GBP, dec!(0)).unwrap();
        assert_eq!(p.discount_of_gross(dec!(10)).amount, dec!(10));
    }

    #[test]
    fn gross_minus_discount_clamps() {
        let p = Price::new(dec!(100), Currency::GBP, dec!(0)).unwrap();
        assert_eq!(p.gross_minus_discount(dec!(100)).amount, dec!(0));
        assert_eq!(p.gross_minus_discount(dec!(200)).amount, dec!(0));
    }
}
