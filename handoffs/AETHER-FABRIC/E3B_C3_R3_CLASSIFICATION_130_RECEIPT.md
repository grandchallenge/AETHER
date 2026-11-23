# E3B-C3-R3: Protected classified HTTP acceptance receipt

**Disposition:** PASS / COMPLETE for the bounded #130 negative-control
classification; **NOT** live C4, source-owned start or E3B-A activation.

**Parent:** [#110](https://github.com/grandchallenge/AETHER/issues/110).
**Classification issue:** [#130](https://github.com/grandchallenge/AETHER/issues/130),
closed with
[RESULT/1](https://github.com/grandchallenge/AETHER/issues/130#issuecomment-7101111690).
**Follow-ons:** [#127](https://github.com/grandchallenge/AETHER/issues/127)
(durable/signed evidence residual),
[#148](https://github.com/grandchallenge/AETHER/issues/148)
(real source-owned atomic start),
[#133](https://github.com/grandchallenge/AETHER/issues/133)
(practical workload acceptance).

## Immutable source and admission

- Starting protected main:
  `511bb12fceddce7dd5a01b3a22051dedae8c54a3`.
- Reviewed implementation candidate:
  `968ec4becd5c80854a9843ced3abc55a43a54aa5`.
- Protected [implementation PR #165](https://github.com/grandchallenge/AETHER/pull/165).
- Verified GitHub protected merge and GCL_CONTROL main readback:
  `edd2acfd8533643afe5fe17d2392a72e5598cb50`.
- New [ADR 0032](../../docs/ADR/0032-c3-first-lane-negative-control.md)
  and [C3-R3 contract](../../docs/ARCHITECTURE/AETHER_FABRIC_E3B_C3_R3_CLASSIFIED_ACCEPTANCE.md)
  are present on protected main. Original C0 contract is explicitly clarified.

## Source-level and negative-control evidence

C0's 23 observable HTTP profiles remain unchanged. The C1 issuer now
rejects `ExplainTuple` before operation admission or mechanical bundle
creation even if C2 synthetically asserts source admission. Exactly 22
profiles remain mechanically eligible. Source `KernelServiceCore::explain_tuple`
continues to deny with HTTP 409/`ambiguous_tuple_reference`.

The new typed C3-classified check requires 22 positive source-adjudicated,
paired HTTP-200 outcomes and one authentic paired HTTP-409 negative
control, matching request/profile/subject identity and canonical error
code/digest and showing zero mechanical authorization, placement, attempt
or authority effect. Missing/duplicate observations, evidence loss,
foreign request ID, altered digest, tampered error code and forged
denied envelope fail the dedicated hosted/local real-handler fixture.

The historical strict 23-positive verifier is unmodified as a distinct
contract and continues to fail closed. This new certificate proves only
the bounded non-operative first-lane classification, **not** a valid
production dispatch decision or signed HTTP evidence feed.

## Exact-head tests and checks

| Gate | Outcome | Receipt |
| --- | --- | --- |
| Authenticated 23-route seeded Axum matrix; 22 successes, one 409; hostile classified negative tests | PASS | Local source test `c3_positive_case_authenticated_http_matrix_evidence` on candidate |
| `aether_control_bridge` unit tests | PASS (30/30) | Local exact candidate |
| `aether_http` unit tests | PASS (83/83) | Local serial replay |
| `cargo fmt --all --check` and `git diff --check` | PASS | Local exact candidate |
| CI: Rust PR fast and Required CI gate | PASS | [run 45173637558](https://github.com/grandchallenge/AETHER/actions/runs/45173637558) |
| GCL conformance | PASS | [run 45173637824](https://github.com/grandchallenge/AETHER/actions/runs/45173637824) |
| Supply Chain | PASS | [run 45173637528](https://github.com/grandchallenge/AETHER/actions/runs/45173637528) |
| GH-OS routing enforcement | PASS | [run 45173637225](https://github.com/grandchallenge/AETHER/actions/runs/45173637225) |
| PR inline review threads | NONE OPEN | PR #165 readback |

Non-authoring logical review passes under AETHER/AGENTS.md:
Formalist [6021668583](https://github.com/grandchallenge/AETHER/pull/165#pullrequestreview-6021668583),
Adversary [6021669008](https://github.com/grandchallenge/AETHER/pull/165#pullrequestreview-6021669008),
and Referee [6021669442](https://github.com/grandchallenge/AETHER/pull/165#pullrequestreview-6021669442).
All are COMMENTED role passes by the same authoring-system identity, **not**
independent human GitHub approvals.

An early parallel run had one namespace-concurrency timing failure that
passed on isolated and serial rerun. A separate local
`cargo test --workspace` attempt ran on side-by-side Rust 1.95 but was
interrupted by the CHAD remote-command timeout; **no full-local-workspace
PASS is claimed**. Current `Cargo.toml` declares Rust 1.86 although locked
`sysinfo 0.39.6` requires 1.95; this separate contract issue is tracked
in [#166](https://github.com/grandchallenge/AETHER/issues/166).
Exact-head protected CI is the asserted required-gate evidence.

## Withheld claims and next action

ReferenceOnly remains the default, no AETHER semantic, mutation,
policy/namespace authority, C1 production issuer or executor routing
was modified, no second resource was made eligible, and there is no
operative FABRIC attempt. An authenticated, fresh, atomic source-control
transaction binding current token, policy, namespace, C1 and actual
worker start remains unproved and belongs to #148. Genuine performance
and safe rollback acceptance belong to #133. E3B-A is a distinct
reserved gate and remains **UNSET / NOT ACTIVATED**.
