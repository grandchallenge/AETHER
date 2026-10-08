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

## 6. Current validation

Focused local validation:

- `cargo check -p aether_control_bridge -p aether_http` — PASS;
- C2 focused HTTP/control tests: 4/4 PASS:
  - admitted real-handler history request emits candidate evidence while
    remaining reference-authoritative;
  - actual excluded append handler never enters C2;
  - forced excluded operation is shadow-rejected;
  - exact replay is idempotent.

Full workspace, clippy and E2/F1/E3B-R regression replay remain required on the
final candidate head before protection.

## 7. Hard exclusions

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

## 8. Governance boundary

C2 is not complete until its exact final candidate head has:

- focused and full regression evidence;
- Formalist, Adversary and Referee exact-head passes;
- protected CI/supply-chain/policy/security/routing checks;
- no unresolved review threads;
- protected merge/readback; and
- issue #120 completion receipt.

C2 completion may authorize only the separately governed differential/readiness
successor. It does not authorize live production routing.
