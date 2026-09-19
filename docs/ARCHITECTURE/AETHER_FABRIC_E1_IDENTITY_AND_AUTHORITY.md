# AETHER/FABRIC E1 Identity and Authority Strata

Status: E1 candidate
Issue: #85
Protocol: `aether-fabric/1.0`

## 1. Purpose

Prevent identity equality, credential possession, co-location, liveness, or
transport reachability from being mistaken for authority.

The separation requires at least four non-collapsible identity strata:

1. endpoint/resource identity;
2. AETHER semantic actor identity;
3. institutional office/guild identity;
4. GHOS controller identity.

A fifth identity class—constitutional principal/authority record—is referenced
where needed but remains owned by INTELLECT/governed authority rather than
FABRIC.

## 2. Endpoint/resource identity

### Definition

An endpoint/resource identity names a concrete mechanical target or resource,
for example:

- worker process;
- service endpoint;
- host;
- GPU/resource slot;
- storage target;
- replica endpoint;
- queue/worker-pool member.

Canonical E1 concept:

```text
EndpointResourceId
```

Properties:

- stable enough for one mechanical contract/audit window;
- names operational realization only;
- may carry resource class, trust zone and locality metadata;
- may be healthy/unhealthy, available/unavailable;
- creates **no semantic or institutional authority by itself**.

Forbidden inference:

```text
endpoint_id == semantic_actor_id  => actor_authorized      [false]
endpoint_healthy                  => actor_authorized      [false]
endpoint_has_gpu                  => work_permitted        [false]
```

## 3. Semantic actor identity

### Definition

A semantic actor identity is an identity represented in AETHER provenance or
coordination state.

Canonical concept:

```text
SemanticActorRef
```

It may identify the asserted source of:

- a claim;
- evidence;
- work outcome;
- coordination action;
- provenance-bearing submission.

A semantic actor reference proves only what the AETHER provenance/authority
contract says it proves. It is not automatically an institutional office or a
GHOS controller.

Forbidden inference:

```text
semantic_actor_exists => institutional_office_held [false]
semantic_actor_exists => endpoint_reachable         [false]
semantic_actor_exists => GHOS_controller_admitted   [false]
```

## 4. Institutional office/guild identity

### Definition

An institutional identity denotes a bounded role, office, guild, or governed
organizational locus under POL/INTELLECT semantics.

Canonical concept:

```text
InstitutionalPrincipalRef
```

Examples may include:

- a Council office;
- a Minder office;
- a POL guild;
- a governed allocator office;
- an authorized deployment operator role.

Its authority comes from the governing institutional record, not from process
identity, endpoint liveness or token possession alone.

An E1 mechanical envelope may carry an opaque `issuer_ref` or
`authorization_ref` pointing to such authority. FABRIC may require and audit the
reference, but it does not reinterpret the office's powers.

## 5. GHOS controller identity

### Definition

A GHOS controller identity denotes a controller admitted under the live GHOS
execution governance.

Canonical concept:

```text
ControllerRef
```

Current AETHER routing evidence binds governed final workflows to the admitted
`GITHUB_ACTIONS` persistent controller. E1 does not add another controller.

Forbidden inference:

```text
EndpointResourceId(x) reachable => ControllerRef(x) admitted   [false]
SemanticActorRef(x) exists       => ControllerRef(x) admitted   [false]
FABRIC delivered payload to x    => ControllerRef(x) admitted   [false]
```

## 6. Constitutional/governing authority reference

E1 may need to preserve a reference to the governing authorization from which a
mechanical envelope was derived.

Canonical concept:

```text
AuthorizationRef
```

This is an opaque, stable reference to a separately governed decision/record.

FABRIC may:

- require that a reference be present;
- bind it into integrity/audit material;
- reject a structurally invalid/expired envelope under a future security
  profile.

FABRIC may not:

- mint an `AuthorizationRef`;
- expand its scope;
- infer additional powers from the referenced principal's name;
- decide that a different office would have equivalent authority;
- treat an inaccessible authorization record as permission to proceed.

## 7. Identity bridge records

Cross-stratum mappings must be explicit and directional.

### 7.1 `ActorEndpointBindingObserved`

Purpose: record that a semantic actor is currently associated with an endpoint
for a bounded operational window.

Conceptual fields:

```text
record_type
binding_id
semantic_actor_ref
endpoint_id
observed_at
expires_at
source_ref
correlation_id?
```

This is an **observation**, not a delegation of authority.

### 7.2 `InstitutionalActorAuthorizationRef`

Purpose: reference a governed record stating that a semantic actor may exercise
some bounded institutional capability.

The governing semantics remain in POL/AETHER/INTELLECT. FABRIC sees only the
opaque authorization binding needed for its mechanical envelope.

### 7.3 `ControllerEndpointBindingRef`

Purpose: reference GHOS's own binding between an admitted controller and the
endpoint/process through which it executes.

E1 does not define or change GHOS controller admission. It only forbids FABRIC
from replacing this reference with endpoint reachability.

## 8. No identity-by-equality rule

The following values may be textually equal without becoming the same identity
kind:

```text
endpoint_id = "agent-17"
semantic_actor_ref = "agent-17"
institutional_principal_ref = "agent-17"
controller_ref = "agent-17"
```

Type/domain identity controls meaning. String equality, URL equality, shared
credential material, common host, common process or common certificate subject
is insufficient to collapse the strata.

## 9. Mechanical envelope issuer law

`MechanicalEnvelopeAuthorized` carries:

```text
issuer_ref
authorization_ref
scope_digest
```

The contract means:

- `issuer_ref` identifies who/what emitted the closed envelope;
- `authorization_ref` points to the upstream governed authority/decision;
- `scope_digest` binds the exact mechanical scope being realized.

The mechanical layer may validate internal envelope consistency. It must not
infer that the issuer has broader rights than the exact envelope expresses.

## 10. Capability representation

Mechanical capability and institutional permission are different types.

### Mechanical capability

Examples:

```text
supports_payload_transport
supports_locality_zone_ca-west
supports_replica_prefix_copy
supports_object_location
supports_queue_class_bounded
```

These describe what an endpoint/FABRIC implementation **can mechanically do**.

### Institutional permission

Examples:

```text
may_investigate_work_object
may_submit_evidence
may_approve_release
may_promote_partition_authority
may_amend_constitution
```

These describe what an actor/office **is authorized to do**.

E1 schemas may contain `required_capabilities` only from the mechanical
capability vocabulary. Institutional permission must be represented by the
opaque authorization reference and remains upstream.

## 11. Trust zone and security labels

A trust zone is a mechanical/security constraint, not semantic visibility.

For example:

```text
trust_zone = gcl-managed-runner
```

may constrain where bytes are sent. It does not replace AETHER policy scope.

Required law:

```text
trust_zone_allows_transport !=> policy_scope_allows_semantic_visibility
```

Both constraints must independently hold when both apply.

## 12. Identity lifecycle

### Endpoint/resource

May appear/disappear frequently. Expiry/liveness affects mechanical routing
only.

### Semantic actor

Lifecycle follows AETHER provenance/coordination contracts.

### Institutional office/guild

Lifecycle follows POL/INTELLECT governance.

### GHOS controller

Lifecycle follows GHOS admission/revocation.

A lifecycle transition in one stratum does not automatically transition
another.

## 13. Revocation/supersession behavior

If an upstream authority supersedes/revokes a mechanical envelope:

- new pre-start attempts under the envelope stop;
- queued-but-not-semantic-started attempts may be rejected;
- a semantic attempt already marked `SemanticExecutionStarted` continues under
  AETHER's own lifecycle unless a separately governed semantic cancellation
  mechanism exists;
- endpoint bindings may expire independently;
- GHOS controller revocation follows GHOS rules and cannot be overridden by a
  still-live FABRIC route.

## 14. Correlation is not identity

`correlation_id`, `event_id`, `attempt_id`, `envelope_id`, and `payload_ref`
are operational/trace identities. None are actor, office, endpoint or controller
authority identities.

Matching an authorized correlation ID must never be sufficient to authorize a
new route or action.

## 15. Required E2 hostile tests

E2 identity tests must include:

1. same string used in all four identity strata;
2. healthy endpoint with no semantic/institutional authorization;
3. authorized semantic actor bound to wrong endpoint;
4. expired actor-endpoint observation;
5. endpoint with valid mechanical capability but no institutional permission;
6. institutional office authorization whose mechanical envelope is expired;
7. delivered GHOS payload to an unadmitted controller endpoint;
8. reused correlation ID from a valid prior operation;
9. trust-zone-allowed endpoint lacking AETHER policy visibility;
10. superseded envelope whose endpoint remains healthy.

Required result: no case may acquire authority by equality, reachability,
liveness, capability or correlation reuse.

## 16. E1 conclusion

Identity is typed by domain. Cross-domain mappings are explicit references or
observations, never implicit equality. This is necessary for FABRIC to remain
mechanically useful while non-authoritative with respect to semantic admission,
institutional power, constitutional authority and GHOS controller admission.
