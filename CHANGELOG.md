# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

## [0.1.1] - 2026-09-12

### Added

- config-knob behavior matrix: tests/config_matrix.rs pins every Price knob — tax_rate (0/20/100 change tax+gross; 101/-1 rejected; boundaries accepted), net amount scales outputs, currency is carried through tax_amount/gross, discount percent changes discounted gross linearly and clamps above 100.


## [0.1.0] - 2026-09-03

### Added
- Billing primitives — Price with tax/discount, re-exporting decimal-money.
- Published to crates.io (2026-09-03).
