# AETHER/FABRIC E3B-C0 — Upstream Authority-Record Issuance Plan

Status: protected complete — C0-A through C0-F
Issue: #113
Parent: #110
Protected basis: `e8b1a5cc8801c19eda81b37b847a82072c2e72d0`
Protected E3B-R basis: `04659f96b0010e7d88e13fb6c8c26c6e8654f0ac`

## 1. Objective

C0 specifies the AETHER-owned control-bridge producer required before ordinary
live HTTP work can safely enter the protected FABRIC resource-selection and
realization path.

C0 is specification/governance only.

It does not:
- enable live FABRIC routing;
- alter `FabricRoutingMode::ReferenceOnly`;
- add queue or dispatch authority;
- activate E3B-A;
- create production credentials;
- change AETHER semantic admission or result-admission authority; or
- grant FABRIC the right to issue upstream control records.

The end product is a protected contract that tells a later C1 implementation
exactly who may issue each required record, from which admitted AETHER facts,
at what linearization point, with what identity/integrity/revocation semantics,
and with what fail-closed behavior.

## 2. Current protected facts

At the protected basis:

- HTTP authentication yields an authenticated principal/token context with
  scopes, optional namespace restrictions, policy context, source identity and
  revocation state.
- namespace resolution is explicit and separate from authentication.
- namespace/resource admission is separate from semantic authority.
- E3B-R keeps the ordinary HTTP path on `reference_only`.
- E3B-R can safely consume exact upstream records and realize
  `PlacementSelected -> RouteRealized`, but it cannot create upstream
  authority.
- E1 requires `MechanicalEnvelopeAuthorized` and
  `MechanicalEnvelopeRevoked` to be produced by an upstream governed issuer.
- FABRIC may observe capacity/liveness and realize mechanics, but those facts do
  not create semantic, institutional, constitutional or controller authority.

Therefore these existing facts are inputs to a future issuance decision, not
the issuance decision itself:

```text
authenticated token/principal
namespace
scope
policy context
resource availability
local liveness
queue capacity
```

None is individually or collectively sufficient to imply
`MechanicalEnvelopeAuthorized`.

## 3. Proposed ownership architecture

C0 shall specify an AETHER-owned control-bridge component, provisionally named:

`AetherMechanicalAuthorityIssuer`

The final C0 specification may rename it, but ownership may not move into
FABRIC or the HTTP transport layer.

The intended separation is:

```text
HTTP authentication / namespace resolution
        |
        v
AETHER semantic-operation admission
        |
        v
AETHER MechanicalAuthorizationDecision
        |
        +--> MechanicalEnvelopeAuthorized
        +--> PlacementConstraintSet
        |
        v
AETHER envelope-control registry/projection
        |
        +--> current ControlStateWitness
        |
        v
FABRIC deterministic selector
        |
        +--> PlacementSelected
        |
        v
E3B realization bridge performs fresh current-state recheck
        |
        +--> RouteRealized | fail closed
```

The issuer must be upstream of FABRIC and subordinate to AETHER's existing
semantic/policy authority.

## 4. Required issuance state machine

C0 shall freeze one explicit state machine. The planning target is:

```text
Q0 REQUEST_RECEIVED
  -> Q1 AUTHENTICATED
  -> Q2 NAMESPACE_RESOLVED
  -> Q3 POLICY_BOUND
  -> Q4 SEMANTIC_OPERATION_ADMITTED
  -> Q5 MECHANICAL_AUTHORIZATION_DECIDED
  -> Q6 ENVELOPE_AUTHORIZED
  -> Q7 PLACEMENT_CONSTRAINT_PROJECTED
  -> Q8 CONTROL_STATE_WITNESSED
  -> Q9 PLACEMENT_SELECTED
  -> Q10 REALIZATION_RECHECKED
  -> Q11 ROUTE_REALIZED
  -> Q12 QUEUE_ADMITTED | PRE_START_REJECTED
  -> Q13 SEMANTIC_EXECUTION_STARTED
```

C0 must define, for every transition:

- authoritative owner;
- exact input record(s);
- whether the transition is semantic, control-bridge or mechanical;
- linearization point;
- stable identity;
- event/decision digest;
- replay/idempotency behavior;
- revocation/supersession interaction;
- expiry rule;
- fail-closed behavior.

The local/no-FABRIC AETHER path remains legal and must not acquire artificial
mechanical bindings.

## 5. Record-by-record specification obligations

### 5.1 MechanicalAuthorizationDecision

C0 should introduce an AETHER-owned decision record or equivalent typed
authority fact upstream of the E1 envelope.

It must answer one narrow question:

> Has AETHER admitted this exact operation to request bounded mechanical
> realization under the specified mechanical policy?

It must not mean:
- semantic execution started;
- result admitted;
- institutional approval;
- controller admission;
- transport success; or
- resource availability.

Minimum required bindings:

- semantic/request operation identity;
- namespace identity;
- authenticated principal reference;
- policy/admission decision reference;
- exact immutable scope/payload manifest reference;
- mechanical policy reference;
- decision revision;
- decision time;
- source authority identity.

C0 must determine whether this record already has an exact existing AETHER
equivalent. If not, it must specify the smallest new type rather than infer the
decision from HTTP success.

### 5.2 MechanicalEnvelopeAuthorized

Owner: AETHER/upstream governed issuer.

Required source facts:
- one valid mechanical-authorization decision;
- exact immutable scope/payload manifest;
- versioned mechanical policy;
- explicit permitted actions/resources/trust/locality;
- bounded retry/redundancy/priority/deadline;
- required capabilities;
- integrity profile.

The envelope's `authorization_ref` must reference the exact upstream decision,
not a token or successful HTTP response.

### 5.3 PlacementConstraintSet

Owner: AETHER control-bridge projector.

Inputs:
- exact authorized E1 envelope;
- explicit resource requirements for the admitted operation class;
- upstream freshness rule;
- decision time;
- current control-state reference.

It may only narrow E1.

### 5.4 ControlStateWitness

Owner: AETHER envelope-control registry/projection.

It must report the current governed state of one exact envelope revision:

```text
active | revoked | superseded | expired
```

It must bind:
- envelope ID and digest;
- current control revision;
- observation time;
- source authority identity;
- exact expiry projection.

FABRIC must not self-attest this witness.

C0 must define the authoritative source of revocation/supersession and how
current state is linearized under concurrent updates.

### 5.5 PlacementSelected

Owner: FABRIC deterministic selector.

Inputs remain the protected F1A/F1B exact decision surface.

C0 must specify where the selector is invoked relative to the upstream records
and how the exact selector implementation/build identity is attached.

`PlacementSelected` remains non-authoritative evidence.

## 6. Identity model

C0 must preserve at least these distinct identity strata:

- HTTP/token identity;
- institutional principal;
- semantic actor/request;
- namespace;
- semantic attempt;
- mechanical authorization decision;
- envelope;
- placement decision;
- endpoint/resource;
- mechanical attempt;
- controller.

String equality, shared credentials, common request IDs or co-location must not
collapse these identities.

A request/correlation ID may link records but never serve as authority.

## 7. Integrity profile work

C0 must choose and document an exact integrity/authentication profile for
control-bridge records.

The decision must state separately:

1. in-process C1 shadow integrity;
2. any future cross-process/network activation integrity.

The profile must bind exact canonical bytes and issuer identity for at least:

- protocol/version;
- authorization decision;
- envelope/control identity;
- scope/payload refs and digests;
- mechanical policy reference;
- actions/resources/trust/locality;
- retry/redundancy/priority/deadline;
- required capabilities;
- lineage;
- expiry/revocation/supersession identity.

No live activation packet is admissible with an integrity profile described
only by human-readable version labels.

## 8. Revocation and current-state model

C0 must define one authoritative control-state registry/projection.

Required behavior:

- only upstream AETHER authority can create authorization/revocation or a
  replacement envelope;
- FABRIC liveness/capacity cannot revoke or authorize;
- revocation/supersession/expiry prevents new/pre-start realization;
- already-started semantic execution remains under AETHER lifecycle law;
- placement evidence may remain historical after revocation but cannot
  authorize realization;
- realization performs a fresh current-state check;
- replaying an old witness cannot revive an invalid envelope.

## 9. Endpoint / operation-class inventory

Before freezing the mapping, C0 must inventory every current HTTP operation
that crosses the bounded blocking executor and classify it as one of:

- mechanically realizable under the first E3B lane;
- local-only / no-FABRIC;
- semantically unsafe for this extraction;
- out of scope.

For each admitted operation class, freeze:

- required AuthScope;
- semantic admission fact;
- immutable scope/payload manifest;
- permitted mechanical action;
- resource requirement class;
- required capabilities;
- default locality/trust constraints;
- retry ceiling;
- semantic-start boundary.

No blanket rule such as "all authenticated HTTP work receives an envelope" is
allowed.

## 10. Threat model

C0 must produce explicit adversarial analysis for at least:

- authentication -> mechanical-authority laundering;
- namespace admission -> mechanical-authority laundering;
- capacity/liveness -> authority laundering;
- FABRIC self-authorization;
- stale control witness;
- placement replay after revocation;
- cross-request replay;
- cross-namespace reuse;
- principal/actor/resource/controller identity collapse;
- implicit protocol downgrade;
- weakened child constraint;
- forged authorization_ref;
- forged scope/payload digest;
- expiry reset on retry;
- duplicate issuance;
- supersession race;
- TOCTOU between placement and realization;
- selector/build substitution;
- fallback to reference behavior after integrity failure in a way that hides an
  invalid FABRIC authorization attempt.

## 11. Constitutional determination

C0 must explicitly answer:

```text
Does this issuer merely project existing AETHER authority into a typed
mechanical control record, or does it create/change effective AETHER authority?
```

Required disposition:

- `NO_ARTICLE_IX_AUTHORITY_CHANGE`; or
- `ARTICLE_XI_REQUIRED`.

The working architectural hypothesis is that an issuer which only projects an
already-admitted AETHER decision into bounded mechanical authorization does not
change Article IX authority.

That hypothesis is not final until the exact C0 mapping is reviewed against the
effective Constitution 1.2.0 record.

If C0 gives the issuer discretion to create new semantic/policy authority,
Article XI is triggered before implementation/activation.

## 12. C0 work packages

### C0-A — protected authority-source inventory

Deliver:
- endpoint/operation inventory;
- current auth/namespace/policy/admission facts;
- candidate semantic-admission source for each operation;
- explicit "not authority" facts.

Output:
`C0_AUTHORITY_SOURCE_INVENTORY.md`.

### C0-B — issuance state machine and ownership

Deliver:
- final Q-state machine;
- owner/domain/linearization for every transition;
- retry/replay/failure transitions;
- local/no-FABRIC branch.

Output:
`C0_ISSUANCE_STATE_MACHINE.md`.

### C0-C — record mapping and identity ledger

Deliver:
- field-level source mapping for
  `MechanicalAuthorizationDecision`,
  `MechanicalEnvelopeAuthorized`,
  `PlacementConstraintSet`,
  `ControlStateWitness`,
  `PlacementSelected`;
- identity non-collapse ledger;
- digest/canonicalization inputs.

Output:
`C0_RECORD_MAPPING.md`.

### C0-D — integrity, revocation and replay contract

Deliver:
- integrity profile;
- current-state registry/projection;
- revocation/supersession/expiry law;
- replay/idempotency law;
- placement-to-realization TOCTOU rule.

Output:
`C0_CONTROL_INTEGRITY.md`.

### C0-E — adversarial and constitutional review

Deliver:
- hostile threat matrix;
- Article IX / Article XI determination;
- migration/rollback statement;
- explicit forbidden implications.

Outputs:
- `C0_THREATS.md`;
- `C0_CONSTITUTIONAL_DISPOSITION.md`.

### C0-F — synthesis / implementation handoff

Synthesize A-E into one authoritative contract and produce the exact C1
implementation specification.

Output:
`AETHER_FABRIC_E3B_C0_AUTHORITY_ISSUANCE_CONTRACT.md`.

No C1 implementation begins before C0-F is protected.

## 13. Parallel support / independent work

C0-A through C0-E are separable enough for independent zero/low-context review:

- WP-A: current code/authority-source inventory;
- WP-B: state machine and event-law consistency;
- WP-C: identity/integrity/replay analysis;
- WP-D: hostile authority-laundering attack;
- WP-E: constitutional/Article IX boundary review.

Synthesis must occur only after the source inventory and at least one adversarial
return are available.

External contributions may inform C0 but do not replace native exact-head
adjudication.

## 14. C0 acceptance gates

C0 is complete only when:

1. the full issuance state machine is explicit;
2. every authority-bearing field has a named source;
3. no HTTP/auth/capacity fact is silently promoted into authority;
4. identity strata remain distinct;
5. revocation/supersession/expiry current-state semantics are explicit;
6. integrity and replay identities are exact;
7. endpoint classes are enumerated rather than blanket-authorized;
8. hostile authority-laundering analysis is closed;
9. constitutional disposition is explicit;
10. migration/rollback requires no semantic-state rewrite;
11. Formalist PASS exists on the exact C0 synthesis head;
12. Adversary PASS exists on that exact head;
13. Referee COMPLETE exists on that exact head;
14. required protected checks are green;
15. no unresolved review threads remain;
16. protected merge/readback succeeds;
17. completion receipt records exact source head and protected merge.

## 15. Successor implementation plan

Only after protected C0:

### C1 — off-path issuer implementation

- implement the typed AETHER issuer/control registry;
- keep it disconnected from ordinary HTTP routing;
- add golden/hostile/replay vectors;
- prove deterministic byte/digest identity.

### C2 — real HTTP shadow issuance

- generate exact control records for admitted real requests;
- feed them through selector + realization validation;
- continue executing through reference routing;
- compare exact candidate/reference behavior;
- no dispatch effect from candidate evidence.

### C3 — differential authority/equivalence closure

- prove shadow-issued records match intended AETHER authority;
- prove no authorization widening;
- prove revocation/rollback behavior;
- re-run constitutional determination against implementation.

### C4 — activation-readiness adjudication

- bind exact issuer implementation;
- bind exact routing delta;
- bind exact rollback delta;
- bind canary/deployment scope;
- fresh review/check/readback.

### A1 — reserved activation

Only after the exact activation packet carries authentic reserved
production/control-plane approval.

## 16. Stop conditions

C0 stops and returns to governance if any of these occurs:

- no authoritative semantic-admission fact exists for the proposed endpoint;
- the issuer would have to infer authority from HTTP/auth/capacity facts;
- identity strata cannot be kept distinct;
- current revocation state cannot be sourced authoritatively;
- integrity provenance cannot be verified;
- the exact design changes effective Article IX authority;
- the requested endpoint class requires a semantic-contract change rather than
  a mechanical projection.

Otherwise C0 proceeds through exact-head protection without further activation
approval.

## 17. Current boundary

```text
E3B-R: PROTECTED COMPLETE
C0-A-F: SUBSTANTIVE COMPLETE
C0 PROTECTED COMPLETION: COMPLETE
C1: AUTHORIZED OFF-PATH ONLY
E3B-A: BLOCKED
reserved_activation_disposition: UNSET
live_fabric_routing: NOT ACTIVE
```


## Execution result

C0-A through C0-F are now instantiated on the C0 synthesis branch:

- `C0_AUTHORITY_SOURCE_INVENTORY.md`
- `C0_ISSUANCE_STATE_MACHINE.md`
- `C0_RECORD_MAPPING.md`
- `C0_CONTROL_INTEGRITY.md`
- `C0_THREATS.md`
- `C0_CONSTITUTIONAL_DISPOSITION.md`
- `AETHER_FABRIC_E3B_C0_AUTHORITY_ISSUANCE_CONTRACT.md`
- ADR 0027.

Substantive disposition: **PASS**. Exact-head governance closed on source head `3a7558311e92616c1200fbcc501e188ed495d192`, protected merge `4eb776748b3c70829bd8dca0c1bc48cb6210776f`.