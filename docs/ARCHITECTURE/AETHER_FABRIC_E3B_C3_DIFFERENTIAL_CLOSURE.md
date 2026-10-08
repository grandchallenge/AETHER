# AETHER/FABRIC E3B-C3 differential closure

Status: **protected complete - non-operative; C4 requires separate governance**

Parent governance: issue #110
C3 governance: issue #124
Protected C2 basis: `3f7105d91f924b7d2568589a98ed939be1e406f5`
Exact C3 source head: `adc96dc70fde82e2e108aa2cab9f29d68a516bc5`
Protected C3 merge: `3a1798baeba0e5bc05ab46fde8a2c9d12c48d8ee`

## Objective

C3 turns the C2 shadow record stream into an explicit fail-closed differential ledger. It answers whether the protected first-lane FABRIC shadow path is equivalent/no-wider than the existing authoritative AETHER reference path while remaining non-operative.

C3 does not activate FABRIC routing. `FabricRoutingMode::ReferenceOnly` remains authoritative.

## Contract

`aether-http-c3-differential/1` binds each adjudicated observation to exact request/method/path/namespace/principal identity; operation class/profile; admission and mechanical-envelope identities; placement and candidate-attempt identities where present; decision-time and fresh realization-time resource snapshots; fresh control-witness revision; reference permitted resources and FABRIC selection; the protected F1E equivalence verdict; and explicit authority-effect/reference-authority state.

Each observation is `equivalent`, `unavailable_equivalent`, or a classified disagreement. Disagreement classes fail C3 closed: shadow failure, shadow rejection, operation-binding mismatch, missing identity binding, permitted-set mismatch, non-none authority effect, or loss of reference authority.

## Structural coverage

The production C0/C1 first lane is a closed registry of 23 operation profiles. C3 requires the adjudicated operation-class set to equal that registry exactly; missing or out-of-domain coverage is a hard failure.

The exhaustive test drives the real C2 controller through every one of those 23 protected production HTTP operation profiles and then requires exact C3 coverage closure. This is structural/controller coverage, not a claim that 23 separate network sockets were opened. End-to-end real-handler evidence remains supplied by the admitted `history` request test; the actual excluded `append` handler proves a mutation endpoint cannot enter the lane.

## Reused protected theorem

C3 deliberately reuses `adjudicate_f1e_live_equivalence` as its inner permitted-set/no-widening predicate. It does not create a second definition of equivalence. C3 adds the HTTP/C1/C2 identity bindings, exact first-lane coverage requirement, and durable disagreement taxonomy around that protected predicate.

## Hostile and replay evidence

The combined C1/C2/FABRIC/C3 suites cover revocation, supersession, expiry, stale snapshots, selected-resource inadmissibility, path/scope/namespace and policy substitution, forged bindings, exact replay, changed-request retry, all seven excluded mutations, forced shadow failure, deliberate permitted-set disagreement, and selected-resource/snapshot/authority-effect tampering.

Failures remain evidence-only. They cannot mutate the HTTP result, acquire a production permit, queue or reserve work, dispatch an operative attempt, or alter semantic state.

## Local validation

Exact source head `adc96dc70fde82e2e108aa2cab9f29d68a516bc5` passed:

- C3 focused tests: 7/7;
- C2 shadow tests, including exhaustive 23-profile C3 controller closure: 7/7;
- `aether_control_bridge`: 29/29 plus compile-fail authority doctest;
- `aether_fabric`: 38/38;
- `aether_http` in full workspace replay: 52/52;
- `cargo test --workspace -j 2`: PASS, with only pre-existing explicitly ignored stress/soak workloads skipped;
- `cargo clippy --workspace --all-targets -j 2 -- -D warnings`: PASS;
- `cargo fmt --all -- --check`: PASS;
- `git diff --check`: PASS;
- E2/F1A Python replay: 16/16.

The local machine initially exhausted disk/PDB capacity while rebuilding a new worktree. Only disposable Rust `target` directories from the completed C2 worktree and failed C3 rebuild were removed. The exact source was then replayed successfully with bounded jobs and reduced local debug-symbol generation. No source, repository evidence, or test-selection semantics changed.

## Protected adjudication

PR #125 reviewed exact source head `adc96dc70fde82e2e108aa2cab9f29d68a516bc5`.

Exact-head reviews:
- Formalist PASS: `5463957115`;
- Adversary PASS: `5463957443`;
- Referee COMPLETE/PASS: `5463957700`.

Exact-head protected workflows:
- GCL conformance run `37858129492`: PASS;
- CI run `37858127437`: PASS;
- Supply Chain run `37858127536`: PASS;
- policy / policy `113587157978`: PASS;
- security / action-policy `113587158288`: PASS;
- Rust PR fast `113587187412`: PASS;
- Required CI gate `113587715578`: PASS;
- Supply-chain PR policy `113587152907`: PASS;
- Required Supply Chain gate `113587228968`: PASS.

There were no inline review threads. PR #125 was protected-merged as `3a1798baeba0e5bc05ab46fde8a2c9d12c48d8ee`, and protected-main readback was exact at that merge before this documentary closure delta.

## Hard boundary

C3 does not authorize production FABRIC permit/queue/reservation/enqueue/dispatch; operative production `RouteRealized`; response or semantic-state mutation; cross-process authority transport/signing infrastructure; mutation widening; `live_fabric`; or E3B-A.

C3 completion makes only a separately governed C4 activation-readiness tranche eligible. C4 itself must remain non-activating. The reserved activation disposition remains a later A0 authority gate.

Current boundary:

```text
E3B-R: PROTECTED COMPLETE
C0-A-F: PROTECTED COMPLETE
C1: PROTECTED COMPLETE
C2: PROTECTED COMPLETE
C3: PROTECTED COMPLETE
C4 activation-readiness: ELIGIBLE ONLY UNDER SEPARATE GOVERNANCE
E3B-A: BLOCKED
reserved_activation_disposition: UNSET
live_fabric_routing: NOT ACTIVE
```
