# AETHER/FABRIC E3B-C0 — Authority-Issuance Contract

Status: protected C0 contract — C1 off-path implementation authorized; E3B-A not authorized
Issue: #113
Parent: #110
Protected predecessor: `0a1552adf719dbe57667a9b8705b986f0f381483`

Normative C0 inputs:

- `C0_AUTHORITY_SOURCE_INVENTORY.md`
- `C0_ISSUANCE_STATE_MACHINE.md`
- `C0_RECORD_MAPPING.md`
- `C0_CONTROL_INTEGRITY.md`
- `C0_THREATS.md`
- `C0_CONSTITUTIONAL_DISPOSITION.md`

This document is the C0-F synthesis and authoritative C1 handoff.

## 1. C0 disposition

```text
authority_source_inventory: COMPLETE
issuance_state_machine: COMPLETE
record_and_identity_mapping: COMPLETE
control_integrity_revocation_replay: COMPLETE
hostile_threat_analysis: COMPLETE
constitutional_disposition: NO_ARTICLE_IX_AUTHORITY_CHANGE
C0 substantive disposition: PASS
C0 protected disposition: COMPLETE
C1 implementation: AUTHORIZED OFF-PATH ONLY
E3B-A activation: NOT AUTHORIZED
live_fabric_routing: NOT ACTIVE
```

The substantive PASS becomes C0 completion only after exact-head
Formalist/Adversary/Referee review, protected checks, protected merge/readback
and #113 completion receipt.

## 2. Normative authority law

1. AETHER owns operation admission and mechanical authorization.
2. FABRIC owns no upstream authorization/revocation capability.
3. Authentication, namespace resolution, policy visibility, capacity, queue
   state and liveness are source facts only.
4. Every first-lane envelope references an exact AETHER mechanical
   authorization decision.
5. Mechanical authorization does not imply semantic start or semantic success.
6. Placement evidence has `authority_effect = none`.
7. Realization requires a new current AETHER control-state witness after
   placement.
8. `mechanical_attempt_id` is born only at `RouteRealized`.
9. AETHER alone records `SemanticExecutionStarted`.
10. The local/no-FABRIC AETHER path remains valid.
11. Mutation endpoints excluded by C0 remain on the reference path.
12. Cross-process authority transport is not authorized by C0.

## 3. First-lane operation registry

C1 SHALL implement an exact closed-world registry containing only these 23
operation classes:

| Operation class | HTTP path | Scope | Policy binding | Evidence persistence |
| --- | --- | --- | --- | --- |
| `history` | `GET /v1/history` | Ops | required | no |
| `history_page` | `GET /v1/history/page` | Ops | required | no |
| `append_dry_run` | `POST /v1/append/dry-run` | Append | none | no |
| `append_receipts` | `GET /v1/append/receipts` | Ops | none | no |
| `schema_catalog` | `GET /v1/schema` | Query | none | no |
| `current_state` | `POST /v1/state/current` | Query | required | no |
| `as_of` | `POST /v1/state/as-of` | Query | required | no |
| `parse_document` | `POST /v1/documents/parse` | Query | none | no |
| `run_document` | `POST /v1/documents/run` | Query | required | AETHER execution evidence |
| `run_document_page` | `POST /v1/documents/run/page` | Query | required | AETHER execution evidence |
| `coordination_pilot_report` | `POST /v1/reports/pilot/coordination` | Query | required | AETHER execution evidence |
| `coordination_delta_report` | `POST /v1/reports/pilot/coordination-delta` | Query | required | AETHER execution evidence |
| `partition_status` | `GET /v1/partitions/status` | Ops | none | no |
| `partition_history` | `POST /v1/partitions/history` | Query | required | no |
| `partition_state` | `POST /v1/partitions/state` | Query | required | no |
| `federated_history` | `POST /v1/federated/history` | Query | required | no |
| `federated_run_document` | `POST /v1/federated/run` | Query | required | AETHER execution evidence |
| `federated_report` | `POST /v1/federated/report` | Explain | required | AETHER execution evidence |
| `explain_tuple` | `POST /v1/explain/tuple` | Explain | required | no |
| `resolve_trace_handle` | `POST /v1/explanations/resolve` | Explain | required | no |
| `resolve_trace_handle_page` | `POST /v1/explanations/resolve/page` | Explain | required | no |
| `get_artifact_reference` | `POST /v1/sidecars/artifacts/get` | Query | required | no |
| `search_vectors` | `POST /v1/sidecars/vectors/search` | Query | required | no |

Closed-world behavior:
- unknown operation class => reject;
- mutation operation class => reject;
- operation path/scope/profile mismatch => reject.

## 4. Explicitly excluded first-lane mutations

C1 SHALL NOT issue first-lane authorization for:

- `POST /v1/append`;
- `POST /v1/schema/register`;
- `POST /v1/schema/activate`;
- `POST /v1/partitions/promote`;
- `POST /v1/partitions/append`;
- `POST /v1/sidecars/artifacts/register`;
- `POST /v1/sidecars/vectors/register`.

A later tranche requires separate semantic-start/commit mapping for these
operations.

## 5. First-lane mechanical profile

Exact logical profile:
`aether-http-local-blocking/1`.

Required projection:

```text
permitted_actions            = ["queue_pre_start"]
eligible_resource_classes    = ["local-blocking-pool"]
trust_zones                  = ["aether-process"]
locality_constraints         = ["local-process"]
required_capabilities        = ["blocking_execution"]
capacity_unit                = "blocking_admission_slot"
minimum_capacity_units       = 1
priority_class               = "normal"
retry_policy.max_attempts    = 1
retry_policy.backoff_class   = "none"
max_parallel_copies          = 1
max_snapshot_age_ms          = 1000
```

Envelope expiry:
`expires_at <= issued_at + operation_timeout_ms`.

A protected later profile may narrow these values. It may not widen them
silently.

## 6. C1 crate/module boundary

C1 SHALL create an AETHER-owned Rust crate:

`crates/aether_control_bridge`

Ownership:
AETHER.

The crate may consume protected E1/E3 DTO/validation types from
`aether_fabric`, but FABRIC code SHALL NOT own or receive the authority-minting
capability.

C1 must not connect the crate to ordinary HTTP execution.

### Required public data types

- `OperationClass`
- `OperationProfile`
- `OperationAdmissionInput`
- `AetherOperationAdmissionDecision`
- `MechanicalAuthorizationDecision`
- `IssuedMechanicalControlBundle`
- `EnvelopeControlState`
- `EnvelopeControlRegistry`
- `AuthorityIssuanceError`

### Required issuer type

`AetherMechanicalAuthorityIssuer`

Responsibilities:
- validate closed-world operation profile;
- canonicalize operation manifest;
- create admitted/rejected operation decision;
- create mechanical authorization decision;
- emit exact E1 envelope bytes;
- register active envelope;
- project first placement constraint;
- obtain current control witness.

### Required authority capability

The in-process implementation SHALL use a non-serializable, module-controlled
authority capability/proof.

It must not provide a public constructor that lets FABRIC or arbitrary callers
mint upstream authority from deserialized records.

The capability is an implementation enforcement of
`aether-control-bridge-inproc/1`; it is not constitutional authority by
itself.

## 7. C1 canonicalization and identity

C1 SHALL reuse or exactly mirror the protected deterministic canonicalization
rules required by C0-C/C0-D.

Required stable identities:

- operation admission ID;
- mechanical authorization ID;
- envelope ID/digest;
- constraint ID/digest;
- witness ID/digest.

Tests SHALL prove:
- exact same inputs => same logical IDs/bytes;
- changed namespace/principal/policy/request/profile/issuer revision => changed
  authorization identity;
- changed bytes under same claimed ID => hard conflict.

## 8. C1 control registry

C1 SHALL implement a bounded in-memory registry only.

Required operations:

- authorize/register;
- observe current witness;
- revoke;
- supersede;
- exact idempotent duplicate readback.

Required state:
- active;
- revoked;
- superseded;
- expiry projection.

No database/network replication is authorized in C1.

## 9. C1 hostile/golden tests

Minimum required tests:

1. all 23 first-lane operation classes resolve to exact protected profiles;
2. all seven excluded mutation classes are rejected;
3. unknown operation class rejected;
4. token success alone cannot issue envelope;
5. namespace permit/capacity cannot issue envelope;
6. effective policy escalation rejected before authorization;
7. changed operation manifest rejected;
8. forged authorization reference rejected;
9. exact duplicate issuance idempotent;
10. duplicate envelope ID with different bytes rejected;
11. active witness valid;
12. revoked witness rejects placement/realization;
13. superseded witness rejects placement/realization;
14. expired witness rejects placement/realization;
15. cross-request replay rejected;
16. cross-namespace replay rejected;
17. retry cannot extend expiry;
18. E3 constraint cannot widen E1;
19. selector implementation identity remains part of later placement input;
20. serialized/reparsed record alone cannot recreate issuer capability;
21. cross-process use of in-process integrity profile rejected;
22. C1 exposes no live routing/configuration switch.

## 10. C1 repository surfaces

Expected C1 delta:

- root `Cargo.toml` / `Cargo.lock` workspace membership;
- `crates/aether_control_bridge/Cargo.toml`;
- `crates/aether_control_bridge/src/lib.rs`;
- focused unit/integration tests;
- optional non-runtime JSON fixtures under
  `schemas/aether_fabric/c0/` only if needed for conformance;
- STATUS/ROADMAP/KNOWN_LIMITATIONS/ADR update.

C1 SHALL NOT modify ordinary endpoint handlers to route requests through the
issuer.

## 11. C1 acceptance

C1 completes only when:

- all C0 contract tests exist and pass;
- E1/E2/F1/E3B-R regressions remain green;
- full Rust workspace and clippy are green;
- exact source/build/profile identities are recorded;
- Formalist/Adversary/Referee exact-head passes close;
- protected checks and thread closure close;
- protected merge/readback and receipt close.

C1 completion authorizes only C2 shadow integration.

## 12. C2 handoff

C2 may then add real-HTTP **shadow issuance**:

1. perform the existing AETHER auth/namespace/policy checks;
2. create C0 admission/authorization/control records;
3. feed them through protected selector + realization validation;
4. record candidate evidence;
5. continue actual request execution through the reference path.

C2 candidate evidence must not:
- acquire a FABRIC permit for production effect;
- replace reference routing;
- create production `SemanticExecutionStarted` bindings;
- enable `live_fabric`.

## 13. Constitutional disposition

From `C0_CONSTITUTIONAL_DISPOSITION.md`:

```text
NO_ARTICLE_IX_AUTHORITY_CHANGE
```

This exact C0 contract may proceed to C1/C2 under current effective
Constitution 1.2.0 without an Article XI amendment.

Any later authority-boundary change reopens that disposition.

## 14. Rollback

Before live activation:
- remove/disable C1/C2 shadow issuer path;
- retain reference execution;
- discard non-authoritative shadow registry/evidence as governed test evidence;
- no journal/semantic-state migration.

## 15. C0-F acceptance matrix

| Requirement | Disposition |
| --- | --- |
| explicit issuance state machine | PASS |
| every authority-bearing field has named source | PASS |
| no HTTP/auth/capacity authority promotion | PASS |
| identity strata distinct | PASS |
| revocation/supersession/expiry explicit | PASS |
| integrity/replay identity exact | PASS |
| endpoint classes enumerated | PASS |
| hostile authority laundering closed | PASS |
| constitutional disposition explicit | PASS — NO_ARTICLE_IX_AUTHORITY_CHANGE |
| migration/rollback no semantic rewrite | PASS |
| exact-head Formalist/Adversary/Referee | PASS — reviews 5442309710 / 5442310641 / 5442311146 |
| protected checks/thread closure | PASS — CI 112793424262; Supply Chain 112793420944; policy 112793354406; security 112793353961; routing 112793355338; no review threads |
| protected merge/readback | PASS — source head `3a7558311e92616c1200fbcc501e188ed495d192`; protected merge `4eb776748b3c70829bd8dca0c1bc48cb6210776f` |
| completion receipt | issued after protected documentary readback on issue #113 |

## 16. Current boundary

```text
C0 substantive work A-F: COMPLETE
C0 protected completion: COMPLETE
C1 implementation: AUTHORIZED OFF-PATH ONLY
E3B-A: BLOCKED
reserved_activation_disposition: UNSET
live_fabric_routing: NOT ACTIVE
```

`main` read back the protected C0 merge exactly at `4eb776748b3c70829bd8dca0c1bc48cb6210776f`.
C1 may now implement only the off-path AETHER control-bridge crate defined here.
Ordinary HTTP hookup remains deferred to C2 shadow integration.


## 17. Protected C0 governance evidence

Exact reviewed C0 source head:
`3a7558311e92616c1200fbcc501e188ed495d192`.

Protected C0 squash merge:
`4eb776748b3c70829bd8dca0c1bc48cb6210776f`.

Exact-head reviews:
- Formalist PASS — `5442309710`;
- Adversary PASS — `5442310641`;
- Referee COMPLETE — `5442311146`.

Exact-head protected gates:
- Required CI — `112793424262` success;
- Required Supply Chain — `112793420944` success;
- policy — `112793354406` success;
- security — `112793353961` success;
- routing-enforcement — `112793355338` success;
- unresolved review threads — none.

The protected merge changes specification/governance only. It does not add the
C1 crate, connect ordinary HTTP traffic to the issuer, create a live FABRIC
route, or populate the reserved E3B-A activation disposition.
