# Threat Model — billing-kit

Reference: STRIDE. Scope: the crate's public API surface. Trust boundary:
(1) bytes/inputs entering public constructors and parsers, (2) concurrent
callers sharing interior state. billing-kit is an in-process library — it opens
no sockets and inherits the embedding process's trust domain.

Purpose: Billing primitives (`billing-kit`) — money-quantities, proration, and invoice line computation

## Assets

| ID | Asset | Exposed via |
|----|-------|-------------|
| A1 | monetary correctness (no lost/created cents) | hostile input, concurrent callers |
| A2 | auditability of computed lines | hostile input, concurrent callers |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Residual risk |
|---|--------|----------|---------|------------|---------------|
| T1 | Rounding drift accumulates across lines | Tampering | `proration math` | integer math with single documented rounding step; property tests sum lines == total | documented |
| T2 | Overflow on extreme quantities | DoS | `arithmetic` | checked ops return typed errors (clippy `unwrap_used = deny` enforced) | documented |

## Repudiation

The crate keeps no audit trail; attribution of calls to callers is out of
scope for an in-process library.

## Out of Scope

- Network transport security (the crate never opens sockets).
- Storage-host compromise: an attacker who controls the host can bypass all
  in-process mitigations.
- Denial of service via resource exhaustion of the host process beyond the
  bounds enforced above.

Reviewed: 2026-09-11
