# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

## [0.2.0] - 2026-10-03

### Added

- `PaymentStatus`: 6-state payment lifecycle (pending / confirming /
  completed / failed / expired / refunded) with a validated transition
  table, `Display`, and serde. `is_settled` / `is_open` predicates; illegal
  transitions are a typed `TransitionError`.
- FX drift guard (`exceeds_fx_tolerance`, `FX_DRIFT_TOLERANCE_BPS`,
  `FX_LOCK_WINDOW_SECONDS`): a locked-in rate is compared against the live
  rate and the drift is bounded rather than silently re-converted.
- Provider webhook verifiers: `verify_fena`, `verify_wallid`,
  `verify_bacs_dd` (HMAC-SHA256 over the raw body).

### Changed

- **decimal-money moved from 0.2 to 1.** billing-kit 0.1.1 declared
  `decimal-money = "^0.2"` while ledger-kit 0.1.0 declares `^1.1`, so the
  two money types could not meet in one graph: a `Price` could not be
  posted to a `Ledger` without a lossy hand-conversion. This release
  aligns on `decimal-money = "1"`, which is the same type ledger-kit
  re-exports as `MonetaryAmount` — so `Price::gross()` posts with no
  conversion at all. Found by estate-integration's `money_stack` suite.

## [0.1.1] - 2026-09-12

### Added

- config-knob behavior matrix: tests/config_matrix.rs pins every Price knob — tax_rate (0/20/100 change tax+gross; 101/-1 rejected; boundaries accepted), net amount scales outputs, currency is carried through tax_amount/gross, discount percent changes discounted gross linearly and clamps above 100.


## [0.1.0] - 2026-09-03

### Added
- Billing primitives — Price with tax/discount, re-exporting decimal-money.
- Published to crates.io (2026-09-03).
