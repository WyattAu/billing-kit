# Pattern adoption: billing-kit

billing-kit is a stateless billing-math and webhook-verification crate.
It holds no balances, no persistence, and no I/O; ledger state lives in
the sibling `ledger-kit` crate. Declared against the WyattAu pattern
canon per `pattern-standards/references/adoption-format.md`.

| Pattern | Status | Evidence |
|---|---|---|
| T1 | adopted | src/price.rs:48 (tax_rate range validated once at construction; downstream methods never re-check) |
| T2 | partial | src/status.rs:70 (runtime FSM transition table over an enum, the canon-endorsed alternative to phantom typestate for this graph shape) |
| T13 | adopted | src/webhooks.rs:74 (now_unix injected as a parameter; the crate performs no I/O and holds no state, so tests need no mocks) |
| T27 | n/a | CHANGELOG.md:25 (stateless crate; double-entry ownership sits in sibling ledger-kit, same decimal-money type posts without conversion) |
| T33 | n/a | src/webhooks.rs:1 (pure signature verifiers; no delivery persistence exists to dedupe, P54 defers dedupe to the persisting receiver) |
| P11 | planned | REQUIREMENTS.md:13 (proration REQ-BL-002 must split with sum-preserving rounding; target 0.3.0) |
| P12 | partial | src/price.rs:60 (currency carried as a unit through every operation; mismatch is a runtime CurrencyMismatch, compile-time phantom units live in decimal-money) |
| P16 | partial | src/fx.rs:18 (fixed 200 bps drift tolerance plus a 30 min lock window guards stale quotes; no decay-widened quote type) |
| P54 | adopted | src/webhooks.rs:80 (timestamped HMAC-SHA256 payload with a 300 s replay window; constant-time compare at :29; missing headers fail closed at :76) |
| P55 | partial | src/status.rs:62 (documents the consumer-side CAS contract `WHERE status = $current` that guards transitions; the crate itself keeps no persistence) |
