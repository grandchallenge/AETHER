# AETHER/FABRIC E3B-C2 — Real-HTTP Shadow Control-Bridge Integration

Status: implementation candidate; exact-head governance pending  
Issue: #120  
Parent: #110  
Protected C1 basis: `2e959418f8a27dfb72beb8502e2b3ecdf82c6f65`  
Protected documentary basis: `c0ef165dbbef735d502acc08fe2a4ee1021c5887`

## 1. Scope

C2 connects the protected C1 AETHER-owned control bridge to ordinary admitted
HTTP requests in a **shadow-only** lane.

For eligible first-lane operations, the HTTP boundary now:

1. completes the existing authentication, namespace and scope checks;
2. applies the existing effective-policy binding where that operation requires it;
3. constructs the exact C1 `OperationAdmissionInput`;
4. issues and verifies the C1 operation-admission, mechanical-authorization,
   E1 envelope, E3 placement constraint and current control witness;
5. runs the protected F1 selector against the same local-pool reference surface;
6. retains the resulting non-authoritative placement/unavailable evidence without
   route realization or mechanical-attempt creation; and
7. continues the actual request through the pre-existing AETHER reference path.

The shadow lane cannot change the HTTP response or acquire production routing
authority.

## 2. Runtime boundary

C2 adds `C2ShadowConfig` as an optional `HttpKernelOptions` setting. The
default remains disabled. Enabling the shadow lane does not change
`FabricRoutingMode`, whose default and operative production value remain
`ReferenceOnly`.

The C2 issuer factory is explicit and same-process-only. Its build identity,
selector identity, registry capacity and evidence capacity are supplied
explicitly by the caller. Invalid issuer initialization becomes retained shadow
failure evidence rather than a fallback authority path.

No `live_fabric` mode is introduced.

## 3. Bounded evidence

`C2ShadowEvidence` records:

- exact request ID, method, path, namespace and principal;
- operation class;
- operation-admission ID;
- E1 envelope ID;
- placement-decision ID when selection succeeds;
- reference/FABRIC permitted-set equality;
- disposition and failure detail;
- `authority_effect = none`;
- `reference_path_authoritative = true`.

The evidence queue is bounded and process-local.

C2 does not construct `RouteRealized` and does not mint a
`mechanical_attempt_id`. Shadow execution stops at non-authoritative placement
comparison/evidence. It acquires no permit, reserves/enqueues/dispatches no work,
does not start a semantic attempt, and cannot activate FABRIC routing.

## 4. Closed first lane

The same protected 23-operation C1 registry is used.

The seven mutation classes remain excluded:

- append;
- schema register;
- schema activate;
- partition promote;
- partition append;
- artifact-reference register;
- vector-record register.

The actual excluded handlers do not enter the C2 shadow lane. If an excluded
operation is forcibly presented to the controller, C1 rejects it fail-closed.

## 5. Replay and failure semantics

Exact C2 replay reuses the C1 deterministic identities and idempotent registry
behavior. Equal request identity/material at equal decision time produces equal
shadow evidence.

Shadow serialization, issuer, selector, control-witness or resource-snapshot
failures are retained as `ShadowFailed`/`ShadowRejected` evidence and are not
substituted for the reference HTTP result.

Policy escalation remains an ordinary AETHER denial, not a shadow-only warning.

## 6. Exact implementation identity

Implementation code commit:

`7fc72fb9f2e73189a36786f79c2bf48f7b7d1e9d`

Implementation tree:

`4be87c99e842b5b347900a8931ce5adeddd89111`

Exact source identities:

- `crates/aether_http/src/fabric_control_shadow.rs`
  - Git blob: `43482b4e7834e6ab9ce5bf9ded7740132f8c443d`
  - SHA-256: `5a3a3cc2bb76854e6a30818454bcc4517c79594d92d120f019e90d90a8aa93a4`
- `crates/aether_http/src/http.rs`
  - Git blob: `c20cb554d0ca14bfa9ff9fa66e6035262d254290`
  - SHA-256: `e2233cbf95bb6f3643d821bc45882f8e41e4d3099c5209ea1b12494b319fd172`
- `crates/aether_control_bridge/src/lib.rs`
  - Git blob: `7f87c91151d28357a7342194b1d1cc719e81d4e1`
  - SHA-256: `adf7978acc41816f8ecbf68b72d2e88a063872eb4e731fa3591561e12c56847a`

## 7. Current validation

Exact-code local validation on `7fc72fb9f2e73189a36786f79c2bf48f7b7d1e9d`:

- C2 focused HTTP/control suite: 6/6 PASS:
  - admitted real-handler history request emits candidate evidence while
    remaining reference-authoritative;
  - exact replay is idempotent;
  - forced excluded mutation is shadow-rejected;
  - actual excluded append handler never enters C2;
  - shadow issuer-initialization failure cannot alter the successful reference
    HTTP response;
  - ordinary HTTP authorization denial never enters the C2 lane;
- C1 control-bridge suite: 29/29 PASS plus compile-fail authority-capability
  doctest PASS;
- `aether_fabric`: 38/38 PASS;
- `aether_http`: 45/45 PASS;
- `cargo test --workspace -j 2` after clean target rebuild: PASS;
- `cargo clippy --workspace --all-targets -j 2 -- -D warnings`: PASS;
- `python -m pytest python/tests/test_aether_fabric_e2.py python/tests/test_aether_fabric_f1a.py`:
  16/16 PASS.

The first full-workspace replay encountered stale Windows target-cache/compiler
artifacts and a compiler-memory failure. The target tree was removed and the
same exact code was replayed from a clean build with bounded parallelism; that
clean replay passed. Existing stress/soak tests remain ignored under their
pre-existing annotations.

Protected CI on the final governance candidate must independently rebuild and
validate C2.

## 8. Hard exclusions

C2 does not authorize or implement:

- production FABRIC queue acquisition, reservation or dispatch;
- operative `RouteRealized` execution;
- production mechanical attempts;
- production semantic start/result admission changes;
- HTTP response mutation from shadow evidence;
- cross-process/network authority transport;
- production credential/signing-key selection;
- any of the seven excluded mutation operations;
- `live_fabric`;
- E3B-A.

## 9. Governance boundary

C2 is not complete until its exact final candidate head has:

- focused and full regression evidence;
- Formalist, Adversary and Referee exact-head passes;
- protected CI/supply-chain/policy/security/routing checks;
- no unresolved review threads;
- protected merge/readback; and
- issue #120 completion receipt.

C2 completion may authorize only the separately governed differential/readiness
successor. It does not authorize live production routing.
