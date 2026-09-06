# ADR 0021 — AETHER/FABRIC E1 Versioned Interface Law

Status: Proposed for E1 review, Revision 1 after Formalist pass
Date: 2026-09-05
Issue: #85
Protocol candidate: `aether-fabric/1.0`

## Context

E0 merged at `411411dfcc29757bbf68589b817b8bffeceb7bcb`
classified the live AETHER estate at responsibility rather than crate granularity.
The Council-approved AETHER/FABRIC direction therefore requires a narrower
versioned contract before any implementation can move.

E0 retained in AETHER:

- append/admission, cuts/replay, policy, provenance and proof identity;
- semantic coordination lease/fence meaning;
- partition/federation semantics, leader epoch and fencing;
- semantic namespace identity;
- same-namespace semantic serialization and started-operation lifecycle;
- semantic sidecar identity/provenance/policy;
- no-partial-authority guarantees.

E0 identified mechanical candidates:

- endpoint/resource liveness and location;
- route/worker realization;
- global capacity and pre-start queue/backpressure;
- replica byte/prefix movement beneath AETHER fencing;
- artifact/vector physical locality;
- audit-event transport;
- host/capacity/locality/cost/latency observations.

## Decision

Adopt a candidate transport-neutral E1 contract family:

```text
aether-fabric/1.0
```

E1 defines specification/schema law only. It does not create a FABRIC runtime or
move responsibility.

The contract comprises:

1. closed `MechanicalEnvelopeAuthorized` records;
2. explicit child-envelope derivation and replacement/supersession semantics;
3. upstream-owned `MechanicalEnvelopeRevoked` control records;
4. exact mechanical-attempt identity across route/queue/delivery;
5. AETHER semantic-start binding to the exact mechanical attempt when FABRIC
   participates;
6. non-collapsible endpoint/actor/institutional/controller identities;
7. telemetry evidence-return rather than direct policy mutation;
8. replica movement without authority promotion;
9. sidecar physical locality without semantic relevance/visibility;
10. fail-closed version/capability negotiation;
11. E2 conformance vectors and schema fixtures.

## Mechanical envelope law

`MechanicalEnvelopeAuthorized` binds an upstream governed authorization to a
closed mechanical scope.

Required identity includes:

- immutable `envelope_id`;
- `authorization_ref` and `issuer_ref`;
- `scope_ref` plus `scope_digest`;
- payload reference;
- permitted actions/resources/zones;
- mechanical/fairness policy refs;
- retry/redundancy/priority/deadline;
- required capabilities;
- optional child-derivation or supersession lineage.

`scope_digest` is SHA-256 over the **exact immutable byte sequence identified by
`scope_ref`**. E1 does not rely on conceptual-object canonicalization.

A derived child names `derived_from_envelope_id` and must be a verified narrowing
of that exact parent. `derived_from_envelope_id` is distinct from
`supersedes_envelope_id`; a child does not invalidate its parent merely by
existing.

Downstream envelopes may narrow but never widen actions, resources, trust zones,
retry, priority, deadline, redundancy or security constraints.

## Revocation law

Early revocation is explicit rather than implied by liveness or queue state.

`MechanicalEnvelopeRevoked`:

- is a `control_bridge` record owned by an upstream governed record;
- names the exact envelope, issuer and authorization reference;
- requires an `integrity_profile_ref`;
- prevents new/pre-start work after verified revocation;
- may cause queued-but-not-started work to terminate with
  `PreStartRejected(reason=envelope_revoked)`;
- cannot mechanically erase an AETHER `SemanticExecutionStarted` attempt;
- cannot be self-issued by FABRIC as authority.

Revocation does not itself create a replacement envelope.

## Attempt identity and semantic-start law

Every FABRIC realization attempt has a required `mechanical_attempt_id`.
Retries use a new mechanical attempt ID while retaining the higher-level
correlation ID.

Mechanical path:

```text
MechanicalEnvelopeAuthorized(E)
 -> RouteRealized(E,M)
 -> QueueAdmitted(E,M) | PreStartRejected(E,M)
 -> PayloadDispatched/Delivered/Receipt(E,M)?
```

If AETHER crosses semantic start through FABRIC:

```text
SemanticExecutionStarted(
  semantic_attempt_id=S,
  mechanical_envelope_id=E,
  mechanical_attempt_id=M)
```

The envelope and mechanical-attempt bindings are either both present or both
absent. Both absent is the local/no-FABRIC AETHER path.

Critical invariant:

```text
PreStartRejected(E,M) xor SemanticExecutionStarted(S,E,M)
```

Once semantic start exists, FABRIC timeout, disconnect, revocation or scheduler
state cannot reinterpret the attempt as safely not-started. AETHER owns semantic
completion/failure.

This preserves the existing
`cancel_before_start_complete_after_start` resource contract.

## Identity law

At least four strata remain distinct:

```text
EndpointResourceId
SemanticActorRef
InstitutionalPrincipalRef
ControllerRef
```

String equality, co-location, reachability, liveness, capability, correlation ID
or common credential material is insufficient to collapse them.

Cross-stratum mappings require explicit bounded observation/reference records.
Authorization scope references bind an immutable `scope_ref` and SHA-256 of its
exact bytes.

## Event law

Mechanical, semantic, institutional, execution and upstream control events are
separate domains.

Mechanical events carry `authority_effect: none` and exact
`mechanical_attempt_id`.
AETHER semantic lifecycle events are in a separate schema with
`authority_owner: AETHER`.
Upstream envelope revocation is in a separate control schema.

Delivery, completion, semantic admission and institutional judgment are never
collapsed into one status.

## Version and integrity law

- unsupported protocol major: reject before start;
- unsupported required capability: reject before start;
- unknown required record type: fail closed;
- no implicit downgrade;
- no dropping constraints to make work routable;
- schema validation is structural only;
- authority/integrity verification is separate;
- integrity failure rejects before start with no weaker fallback.

E1 does not choose the final cryptographic signature/credential/key profile.
It requires explicit profile references on authority-bearing control paths.

## Telemetry law

Operational observations are mechanical only:

```text
observation
 -> provenance-bearing semantic submission
 -> AETHER admission/rejection
 -> later governed allocator/decision may consume admitted evidence
```

Integer queue-depth/replication-lag values and floating latency/cost values are
both valid JSON numeric observations under the candidate schema.

No unadmitted telemetry-to-policy edge exists in E1.

## Replica law

FABRIC may later move bytes/prefixes. AETHER retains:

- source/partition cut identity;
- leader epoch;
- promotion authority;
- stale-epoch rejection;
- divergent-prefix fencing;
- semantic acceptance of replica state.

Byte equality and endpoint health do not confer authority.

## Sidecar law

FABRIC/storage may later coordinate physical locality. AETHER retains semantic
identity, provenance, visibility/policy and result admission.

Cache hit, object availability and vector rank are observations only.

## Formalist Revision-1 closure

The first Formalist pass returned five material defects. Revision 1 resolves
them as follows:

1. **attempt identity** — mechanical attempt ID is mandatory across mechanical
   path; semantic start binds exact envelope+attempt or neither;
2. **child-envelope lineage** — `derived_from_envelope_id` added and separated
   from supersession;
3. **revocation** — explicit upstream-owned `MechanicalEnvelopeRevoked` record
   and schema added;
4. **scope digest domain** — `scope_ref` plus SHA-256 over exact referenced bytes;
5. **telemetry JSON Schema overlap** — numeric telemetry uses non-overlapping
   type semantics; integer/floating fixtures added.

## Candidate schema set

- `mechanical_envelope.schema.json`
- `envelope_control.schema.json`
- `mechanical_event.schema.json`
- `semantic_lifecycle_event.schema.json`
- `telemetry_evidence.schema.json`
- `identity_binding.schema.json`
- valid/invalid examples under `schemas/aether_fabric/e1/examples/`

## E0 ambiguity disposition

Candidate boundaries now exist for:

- sidecar physical locality;
- replica movement;
- namespace-to-worker realization;
- pre-start queue/resource realization;
- telemetry evidence return.

Still unresolved pending E2 evidence:

- durable storage adapter extraction;
- generic transport extraction from `aether_http`;
- per-limit classification where semantic safety and operational capacity remain
  mixed.

The temporary `aether_api` facade is not a protocol owner.

## Consequences

Benefits:

- future FABRIC optimization is explicitly non-authoritative;
- attempt/retry/hidden-commit invariants are testable;
- child/supersession/revocation lifecycles are distinguishable;
- identity/event collapse is testable;
- existing resource and partition-fencing semantics remain AETHER-owned;
- telemetry and physical locality become evidence rather than policy.

Costs:

- more typed records and explicit handoffs;
- final integrity/security profile remains future work;
- E2 must build substantial conformance evidence before extraction;
- some current outer-crate responsibilities remain mixed.

## Rejected alternatives

- one generic cross-plane status object;
- endpoint capability as permission;
- FABRIC-owned semantic lease/leader epoch;
- exactly-once transport as truth semantics;
- wholesale move of `aether_http` or `aether_partition`;
- implicit revocation from endpoint failure;
- child-envelope existence as implicit parent supersession.

## Non-authority

This ADR does not:

- create/activate FABRIC;
- move implementation;
- modify Article IX/AETHER authority;
- activate POL/AETHER-Learn;
- change GHOS controller admission;
- authorize deployment or claim promotion.

## Verification and next stage

E1 requires:

- Formalist re-review of Revision 1;
- Adversary attack on widening, revocation forgery, attempt replay, downgrade,
  starvation, event/identity collapse and telemetry bypass;
- Referee completeness disposition;
- exact-head protected checks.

If E1 closes, only E2 conformance-harness work is authorized. Runtime extraction
remains prohibited until corresponding E2/D1-D5 evidence passes.
