# AETHER/FABRIC E3B-C2 â€” Real-HTTP Shadow Control-Bridge Integration

Status: protected complete — differential/readiness successor eligible only under separate governance; E3B-A blocked
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
6. performs a fresh control-state/resource observation and pure candidate
   `RouteRealized` validation when a resource is selected;
7. retains that candidate evidence with `authority_effect = none`; and
8. continues the actual request through the pre-existing AETHER reference path.

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
- candidate mechanical-attempt identity from pure realization validation;
- reference/FABRIC permitted-set equality;
- disposition and failure detail;
- `authority_effect = none`;
- `reference_path_authoritative = true`.

The evidence queue is bounded and process-local.

A candidate `RouteRealized` record and its candidate mechanical-attempt identity
are validation evidence only. They acquire no permit, reserve/enqueue/dispatch no
work, create no operative/production attempt, do not start a semantic attempt,
and cannot activate FABRIC routing. The actual request remains on the AETHER
reference path.

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

Shadow serialization, issuer, selector, fresh-control, resource-snapshot or
pure-realization failures are retained as `ShadowFailed`/`ShadowRejected`
evidence and are not substituted for the reference HTTP result.

Policy escalation remains an ordinary AETHER denial, not a shadow-only warning.

## 6. Exact implementation identity

Protected partial implementation merge:

`212abc8774f9c9c7e0ce4bea26ebcd4c0dca8c2c` (PR #121)

Bounded realization-repair code commit:

`3f533024ac805568bd4018999fdee082f264cc4a`

Repair implementation tree:

`8301e5edda31872e9ee8765db7ca5028d634a929`

Exact repair source identities:

- `crates/aether_http/src/fabric_control_shadow.rs`
  - Git blob: `050fba18ce6dcc810260616e8cfbdb9d19a38f8f`
  - SHA-256: `4ebcd6144cb10f7152a43040566472009c1b3cde5cce3d23a67071497012fdd9`
- `crates/aether_http/src/http.rs`
  - Git blob: `235d0909d0d4729cf914cad9f789bcd1d9ce60a6`
  - SHA-256: `85ab990a5af75b52f6ceeada14c69ab69996aa5e94a9df3f447255e44bf9aa85`
- `crates/aether_control_bridge/src/lib.rs`
  - Git blob: `7f87c91151d28357a7342194b1d1cc719e81d4e1`
  - SHA-256: `adf7978acc41816f8ecbf68b72d2e88a063872eb4e731fa3591561e12c56847a`

The #121 merge remains a valid protected transition. The repair binds the
additional pure-realization evidence required by the pre-existing issue #120
scope clarification; it does not retroactively rewrite or invalidate #121.

## 7. Current validation

Exact-code local validation on repair commit `3f533024ac805568bd4018999fdee082f264cc4a`:

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

Protected exact-head validation completed on repair governance head
`a41043c9ee21de725cc841ba3122e931906da6b6`: CI run `37802860987`, Supply
Chain run `37802861103`, and GCL conformance run `37802862376` all succeeded.
Required CI gate `113400428507`, Required Supply Chain gate `113399455695`,
Rust PR fast `113399394008`, policy `113399328420`, security/action-policy
`113399327726`, and routing-enforcement `113399327787` all succeeded.

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

## 9. Protected governance completion

C2 is protected complete through two preserved transitions:

- PR #121 protected the ordinary-request C1 issuance and F1 selector-comparison
  subset at merge `212abc8774f9c9c7e0ce4bea26ebcd4c0dca8c2c`;
- post-merge readback against the pre-existing issue #120 clarification identified
  the remaining pure-realization evidence obligation;
- exact repair head `a41043c9ee21de725cc841ba3122e931906da6b6`
  received Formalist PASS review `5459229503`, Adversary PASS review
  `5459230065`, and Referee completion review `5459230978`;
- all required protected checks on that exact head passed, with no inline review
  threads;
- PR #122 protected the bounded realization repair at merge
  `6caa2d634bcc6c0592728895334268db77ae2c31`;
- protected `main` readback was exact at that merge before this documentary
  completion delta.

C2 completion may authorize only a separately governed differential/readiness
successor. It does not authorize live production routing.

Current boundary:

```text
E3B-R: PROTECTED COMPLETE
C0-A-F: PROTECTED COMPLETE
C1: PROTECTED COMPLETE
C2: PROTECTED COMPLETE
next differential/readiness tranche: ELIGIBLE ONLY UNDER SEPARATE GOVERNANCE
E3B-A: BLOCKED
reserved_activation_disposition: UNSET
live_fabric_routing: NOT ACTIVE
```
