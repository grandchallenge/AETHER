# E3B-C0-B — Issuance State Machine and Ownership

Status: C0 synthesis input
Issue: #113
Protected source basis: `0a1552adf719dbe57667a9b8705b986f0f381483`

## 1. Purpose

Freeze the first-lane AETHER -> control-bridge -> FABRIC transition sequence
without changing AETHER semantic authority.

The state machine distinguishes permission to request mechanical realization
from semantic execution, semantic success, result admission and institutional
authority.

## 2. Domains and owners

| Domain | Owner | C0 meaning |
| --- | --- | --- |
| HTTP edge | AETHER HTTP | request decoding, auth input, namespace header |
| semantic/policy | AETHER | operation class, namespace, policy visibility, operation admission |
| control bridge | AETHER | mechanical authorization, envelope lifecycle, constraints, current-state witness |
| mechanical | FABRIC | resource selection, route realization, pre-start capacity evidence |
| semantic execution | AETHER | semantic start, execution/computation, authoritative mutation/result semantics |
| institutional | INTELLECT/POL where applicable | not changed by C0 |

FABRIC cannot issue or revoke AETHER control-bridge authority.

## 3. Frozen first-lane state machine

```text
Q0 REQUEST_RECEIVED
  -> Q1 REQUEST_AUTHENTICATED
  -> Q2 NAMESPACE_RESOLVED
  -> Q3 OPERATION_PROFILE_AND_POLICY_BOUND
  -> Q4 AETHER_OPERATION_ADMITTED
  -> Q5 MECHANICAL_AUTHORIZATION_DECIDED
  -> Q6 ENVELOPE_AUTHORIZED_ACTIVE
  -> Q7 PLACEMENT_CONSTRAINT_PROJECTED
  -> Q8 CURRENT_CONTROL_STATE_WITNESSED
  -> Q9 PLACEMENT_SELECTED | PLACEMENT_UNAVAILABLE
  -> Q10 REALIZATION_RECHECKED
  -> Q11 ROUTE_REALIZED | PRE_START_REJECTED
  -> Q12 QUEUE_ADMITTED | PRE_START_REJECTED
  -> Q13 SEMANTIC_EXECUTION_STARTED
  -> Q14 SEMANTIC_EXECUTION_COMPLETED
```

For FIRST_LANE_EVIDENCE_WRITE operations, AETHER execution/proof persistence
may occur only at or after Q13.

For EXCLUDED_MUTATION operations, C0 stops before Q4 and leaves the existing
reference path untouched.

## 4. Transition contract

### Q0 -> Q1: request authentication

Owner: AETHER HTTP.

Inputs:
- request headers;
- auth configuration.

Linearization:
- successful `HttpAuth::authorize` against the exact current auth
  configuration.

Output facts:
- principal;
- principal ID when configured;
- token ID when configured;
- required-scope satisfaction;
- namespace allowance;
- token revocation check.

Not authority for:
- mechanical envelope issuance by itself;
- semantic start;
- result admission.

Failure: HTTP unauthorized/forbidden; no control-bridge record exists.

### Q1 -> Q2: namespace resolution

Owner: AETHER HTTP/namespace layer.

Input:
- `X-Aether-Namespace` or default namespace.

Linearization:
- successful construction of exact `NamespaceId`.

Output:
- namespace identity.

Failure: validation error; no control-bridge record exists.

### Q2 -> Q3: operation profile and policy binding

Owner: AETHER.

Inputs:
- endpoint/operation class;
- required `AuthScope`;
- authenticated principal;
- requested policy context if present;
- protected operation-profile registry;
- pre-execution structural/resource checks.

Linearization:
- exact effective policy decision plus exact operation-profile revision.

Output:
- `OperationAdmissionInput`.

Policy binding must be performed before mechanical issuance for all first-lane
operations, even where the current reference code performs the pure calculation
inside the blocking closure.

Failure:
- policy escalation;
- unknown operation profile;
- unsupported protocol/profile;
- structural/resource precheck failure.

No weaker fallback to FABRIC is allowed.

### Q3 -> Q4: AETHER operation admission

Owner: AETHER semantic edge.

Record:
`AetherOperationAdmissionDecision`.

Meaning:
AETHER admits this exact request to *attempt* this exact operation class under
the effective policy and operation profile.

It does not mean the operation has semantically started or succeeded.

Linearization:
- creation of the immutable admission record from exact Q3 inputs.

Identity:
- deterministic digest of the canonical admission-input record and AETHER
  issuer revision.

Failure:
- disposition `rejected`; no mechanical authorization may follow.

### Q4 -> Q5: mechanical authorization decision

Owner: AETHER control bridge.

Record:
`MechanicalAuthorizationDecision`.

Inputs:
- admitted Q4 record;
- exact immutable operation manifest;
- exact protected mechanical profile.

Meaning:
AETHER permits bounded mechanical realization for the admitted operation.

Linearization:
- control-bridge issuer commits the decision to its in-process C1 control
  registry or equivalent protected issuer state.

Authority:
- mechanical-only;
- subordinate to Q4;
- no semantic-result authority.

### Q5 -> Q6: envelope authorization

Owner: AETHER control bridge.

Record:
`MechanicalEnvelopeAuthorized` (`aether-fabric/1.0`).

Linearization:
- envelope is inserted in the AETHER envelope-control registry with state
  `active` and revision 1.

The registry, not FABRIC, is the source of current envelope state.

Initial first-lane profile:
- action: `queue_pre_start`;
- resource: `local-blocking-pool`;
- trust: `aether-process`;
- locality: `local-process`;
- capability: `blocking_execution`;
- max attempts: 1;
- max parallel copies: 1.

Failure:
- malformed/unsupported profile;
- digest/integrity failure;
- missing operation admission;
- missing immutable manifest.

### Q6 -> Q7: placement constraint projection

Owner: AETHER control bridge.

Record:
`PlacementConstraintSet` (`aether-fabric/1.1`).

Inputs:
- exact E1 envelope bytes/digest;
- first-lane operation profile;
- resource requirement;
- decision time;
- current witness reference/digest.

Law:
- E3 may only narrow E1.

Failure:
- any widening;
- unsupported capacity unit;
- missing/current-state binding.

### Q7 -> Q8: current control-state witness

Owner: AETHER envelope-control registry/projection.

Record:
`ControlStateWitness`.

Linearization:
- atomic read of the registry's current state/revision for the exact envelope
  after all prior revocation/supersession writes visible at that point.

States:
`active | revoked | superseded | expired`.

The witness is historical evidence of the observation point. It is not a lease
that remains current forever.

Failure:
- unknown envelope;
- digest mismatch;
- unavailable registry;
- non-active state when placement is requested.

### Q8 -> Q9: FABRIC placement selection

Owner: FABRIC deterministic selector.

Inputs:
- exact E1 envelope;
- exact placement constraint;
- exact current witness;
- canonical resource snapshot;
- exact scheduler policy artifact;
- exact selector implementation identity.

Output:
`PlacementSelected | PlacementUnavailable`.

Authority effect:
`none`.

The selected resource must remain inside the AETHER-authorized set.

### Q9 -> Q10: realization-time recheck

Owner split:
- AETHER supplies a new current control-state witness;
- FABRIC/E3B bridge validates placement and current resource state.

Required:
- newly observed witness after placement;
- current resource snapshot;
- exact placement evidence;
- exact realization-request identity.

A Q8 witness cannot substitute for the Q10 fresh witness.

Failure:
- revoked/superseded/expired envelope;
- stale snapshot;
- selected resource no longer admissible;
- selector/build/digest mismatch;
- replay conflict.

### Q10 -> Q11: route realization

Owner: FABRIC.

Output:
`RouteRealized`.

This is the first point where `mechanical_attempt_id` exists.

Identity:
- deterministic/idempotent for the exact realization request;
- a distinct explicit retry request yields a new attempt ID.

Authority effect:
`none`.

### Q11 -> Q12: pre-start queue admission

Owner:
- FABRIC for mechanical resource/queue admission;
- AETHER retains same-namespace ordering and semantic start rules.

Output:
`QueueAdmitted | PreStartRejected`.

Current first lane has no automatic retry.

### Q12 -> Q13: semantic execution start

Owner: AETHER.

Record:
`SemanticExecutionStarted`.

If FABRIC participated, AETHER binds both:
- `mechanical_envelope_id`;
- `mechanical_attempt_id`.

Both or neither are present.

A `PreStartRejected` attempt cannot later become semantically started.

### Q13 -> Q14: semantic completion

Owner: AETHER.

FABRIC failure, liveness loss or later envelope revocation cannot reinterpret
an already-started semantic operation as pre-start rejected.

## 5. Envelope control transitions

Independent of request progression:

```text
ACTIVE -> REVOKED
ACTIVE -> SUPERSEDED
ACTIVE -> EXPIRED
```

Owner: AETHER control registry.

FABRIC cannot perform these transitions.

A child/narrowed envelope is not a transition of its parent.

Revocation/supersession/expiry:
- prevents new Q9/Q10/Q11 work;
- may terminate queued-but-not-started work as pre-start rejected;
- does not erase Q13 semantic work already started.

## 6. Local/no-FABRIC branch

AETHER retains a legal local path:

```text
Q0 -> Q1 -> Q2 -> Q3 -> Q4
   -> SemanticExecutionStarted(no mechanical binding)
   -> SemanticExecutionCompleted
```

C0 does not require every admitted AETHER operation to use FABRIC.

## 7. Replay and retry

### Exact duplicate observation

Re-observing the same immutable Q4/Q5/Q6 record identity is idempotent.

### Placement replay

Replaying Q9 evidence does not create Q11. Q10 must run again with a fresh
witness/current resource state.

### Mechanical retry

The first lane sets `max_attempts = 1`; no automatic mechanical retry exists.

A future explicit retry contract must:
- retain correlation identity;
- create a new realization-request identity;
- create a new `mechanical_attempt_id`;
- use the same or narrower still-valid envelope;
- not reset expiry/revocation/supersession.

### HTTP retry

A new HTTP request is not mechanically identical merely because its payload is
equal. It receives a new request/correlation identity unless an endpoint's
existing semantic idempotency contract explicitly binds it otherwise.

## 8. Failure rule

Unknown or missing authority evidence always fails closed before Q11.

Reference routing may remain available as an operational rollback mode only
when it independently satisfies the original AETHER authorization path.
Integrity failure in a FABRIC candidate path must be visible; it cannot be
silently rewritten as a successful FABRIC attempt.

## 9. C0-B disposition

The state machine is internally consistent with the protected E1 event algebra:

- mechanical authorization remains upstream-owned;
- `RouteRealized` creates mechanical attempt identity;
- queue admission does not imply semantic start;
- AETHER owns semantic lifecycle;
- local/no-FABRIC execution remains valid.
