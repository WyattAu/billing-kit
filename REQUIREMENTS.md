# Requirements — billing-kit

Numbered, testable requirements. Every requirement maps to at least one named
test or doc-comment contract; security-relevant items cite threat-model rows.

Scope: Billing primitives (`billing-kit`) — money-quantities, proration, and invoice line computation

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-BL-001 | Line totals compute in integer minor units; no floating-point arithmetic anywhere in billing math | MUST |
| REQ-BL-002 | Proration splits periods by exact integer arithmetic with documented rounding (toward zero) | MUST |
| REQ-BL-003 | Negative quantities/periods return typed errors | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-BL-100 | Unwrapping is denied in production paths; overflow uses checked arithmetic returning errors | MUST |

## Observability & API hygiene

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-BL-900 | All fallible public APIs return typed errors; production `unwrap`/`expect` is denied or explicitly justified with an invariant comment | MUST |
| REQ-BL-901 | Public items carry doc comments with runnable examples where practical | SHOULD |
