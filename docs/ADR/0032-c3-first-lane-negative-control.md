# ADR 0032: C3 first-lane HTTP negative control and source issuance narrowing

**Status:** Candidate for protected adjudication, issue #130. No production activation.
**Starting main:** `511bb12fceddce7dd5a01b3a22051dedae8c54a3`.
**Source:** C0 authority-issuance contract; R1/R2 actual seeded HTTP results;
`KernelServiceCore::explain_tuple` deliberately rejects as ambiguous.

## Decision

Retain the entire 23-class *observable* HTTP registry and existing source
semantics. No successful mechanical authorization may be issued for the legacy
`ExplainTuple` class. Eligibility is defined in AETHER C1, not delegated to a
caller-provided C2 shadow admission flag. This is a deny-only contract change.

C3's original strict 23-positive predicate remains untouched as a deliberately
unmet legacy certificate. A **separate** classified verifier accepts 22
independently adjudicated, paired positive HTTP-200 records plus one explicit
denied control, which must match authenticated reference and shadow HTTP-409
outcomes with canonical error code `ambiguous_tuple_reference`, equal
canonical error projections, reference authority intact, and no C1 envelope,
placement, selection or mechanical attempt. Retention loss, duplicates,
provenance inconsistency and corrupted/replayed identity fail closed.

The classified receipt's claim is *differential HTTP acceptance under a
single local reference-pool shadow harness only*. It does not prove production
source-owned consumptive authority, full runtime equivalence, signed continuous
capture, multi-resource routing, or cost/performance advantage.

## Alternatives rejected

- Return 200 from `explain_tuple` by inventing a global tuple identity:
  changes primary AETHER semantics and would require separate Article IX/XI
  classification and an exact-source semantic API proposal.
- Treat HTTP 409 as successful placement: creates an authorization-laundering
  defect.
- Omit the 23rd profile from the observable registry: loses regression and
  closed-world coverage of the legacy endpoint.
- Treat a caller-provided `source_operation_admitted` boolean as permission:
  does not attest semantic acceptance or current source state.

## Authority and integration

No Article IX authority transfer is proposed. The admitted delta strictly
narrows the off-path C1 issuer; C2 remains non-operative, and
`ReferenceOnly` retains all semantic execution. Real source-current control,
revocation linearization, and restart-safe single-use start are independently
open in #148. E3B-R and E3B-A remain separate gates.

Review requires exact-source unit and authenticated 23-route HTTP fixtures,
hostile denial/tamper/loss/replay coverage, full Rust workspace fmt/clippy/tests,
GCL conformance, Supply Chain, GH-OS routing enforcement and three logical
non-reserved Formalist/Adversary/Referee passes before protected merge/readback.
