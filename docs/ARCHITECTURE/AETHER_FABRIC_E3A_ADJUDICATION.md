# AETHER/FABRIC E3A — Exact-Head Adjudication

Status: candidate — substantive PASS; governance gates pending
Issue: #94
Protected basis: `41f4bfc6873e522fa1320bd456c04d3d7f754959`
Protected E2 basis: `fce5095cfd0db8766ae0c7d5e80c091b41284862`
Protected E3/F1 specification: `1ccce8756f7e79fe16753f2c8b3da182addc94c5`

## Evidence chain

- F1A contract/reference adapter: `24589252ca5584c1ac4385f8fd57b9c680eebbf3`
- F1B deterministic selector: `04393c83eebe00ef7b24b5efcd767ffbdbc3b3d3`
- F1C hostile/replay evidence: `c21aebe269a83fe729c226189f6643de7f6fc1da`
- F1D shadow integration: `ebe535885ae73b7950a2f540a137f2836bdb2941`
- F1E differential equivalence: `47bec0af7823c0295f55a6b5f6ae3dac832bfeae`
- protected F1E readback: `41f4bfc6873e522fa1320bd456c04d3d7f754959`

E3A adjudicates this chain only. It adds no selector, route, queue, retry,
transport, attempt, semantic transition, policy interpretation, or authority.

## Independent exact-state checks

### E1 remains unchanged

The complete `schemas/aether_fabric/e1` surface has identical blob/tree SHAs
at protected E2 and the E3A basis:

- `README.md` — `801d8efb78937cd878e644f7296e0538e4920916`
- `envelope_control.schema.json` — `8cae53d8493b7c039a3017a5f76ff0950a10487c`
- `identity_binding.schema.json` — `8130c31b666fd1c92563d8f8ef91ed21722d0247`
- `mechanical_envelope.schema.json` — `1fc666846897b1e45d18b2f27ad76596079e322a`
- `mechanical_event.schema.json` — `fcfffc853b5af6b01e46df8547b411c566214473`
- `semantic_lifecycle_event.schema.json` — `3bbac1ec8807755ccc192e44eee294d620972eb3`
- `telemetry_evidence.schema.json` — `ee315370135a91a22f23fadf70afd09140a0bae9`
- `examples/` tree — `d7d520db7678d8695f65d85825bda54d87d0752d`

Thus E1 `aether-fabric/1.0` was not silently rewritten.

### E3/F1 is explicit and separate

The protected tree contains separate `aether-fabric/1.1` placement constraint,
control witness, resource snapshot and placement decision schemas. The Rust
contract requires exact 1.1 headers and fails closed on protocol, digest,
narrowing, snapshot, control-state or selector-identity errors.

The live identity is exactly `aether-local-blocking-pool`; anonymous semaphore
permits are not fabricated into stable worker identities.

F1D/F1E remain non-operative evidence. The AETHER reference path remains
authoritative.

## Acceptance adjudication

1. Non-operative placement-selection responsibility only — **PASS**.
2. E1 `1.0` byte/schema stability — **PASS** by exact blob/tree comparison.
3. Explicit negotiated E3/F1 records with fail-closed validation — **PASS**.
4. Actual local blocking-pool live baseline, no synthetic worker identities — **PASS**.
5. Exact decision/evidence digests bind selector implementation identity — **PASS**.
6. Canonicalization and upstream freshness rules explicit — **PASS**.
7. Control-state, revocation and idempotency evidence — **PASS within E3A scope**. F1C rejects exact-time revoked/superseded witnesses and proves idempotency; F1D rejects revoked state before comparison. A later placement-to-`RouteRealized` recheck is not claimed because E3A performs no realization.
8. Shadow path mints no mechanical attempt and has no queue/permit/dispatch side effect — **PASS**.
9. Live-domain no-authorization-widening differential evidence — **PASS**.
10. Synthetic multi-resource evidence not reported as live equivalence — **PASS**.
11. Applicable E2 invariants remain green — **PASS subject to this E3A head replay**.
12. Hostile F1 vectors pass — **PASS subject to this E3A head replay**.
13. Rollback requires no semantic-state migration — **PASS**.
14. E3A Formalist/Adversary/Referee exact-head passes — **PENDING-GATE**.
15. E3A protected checks green and no unresolved threads — **PENDING-GATE**.
16. E3A protected-main readback after merge — **PENDING-GATE**.
17. E3A completion receipt records exact source head and protected merge — **PENDING-GATE**.

## Candidate disposition

All substantive implementation/evidence criteria available before E3A's own
governance transaction are satisfied.

**SUBSTANTIVE PASS / GOVERNANCE PENDING.**

Any candidate-head change invalidates later exact-head review/check authority.

Before review, the final E3A head must replay the applicable E2/F1A harness,
`aether_fabric` hostile/replay tests, `aether_http` F1D/F1E tests, full Rust
workspace, clippy with warnings denied, formatting and diff integrity.

## Non-authority

Completion of E3A does not authorize E3B live FABRIC routing/cutover, queue or
dispatch authority, semantic extraction, general FABRIC activation, autonomous
policy adaptation, replica authority, distributed FABRIC, or generalized GCL
migration.
