# E3B-C0-A — Authority-Source and Endpoint Inventory

Status: C0 synthesis input
Issue: #113
Protected source basis: `0a1552adf719dbe57667a9b8705b986f0f381483`

## 1. Finding

The current HTTP service has exactly three production crossings into the bounded
blocking executor:

1. `HttpKernelState::execute`;
2. `HttpKernelState::execute_partitioned`;
3. `HttpKernelState::resolve_execution_trace`.

Thirty endpoint handlers reach one of those crossings.

There is no current runtime `MechanicalEnvelopeAuthorized` producer on the
ordinary HTTP path.

## 2. Current AETHER source facts

### Authentication

`HttpAuth::authorize` validates:

- bearer-token identity when auth is configured;
- required `AuthScope`;
- allowed namespace;
- token revocation state.

It returns an `AuthenticatedPrincipal` containing principal, principal ID,
token ID and optional policy context.

If no auth tokens are configured, the current service admits an anonymous
principal. That behavior is a transport/authentication configuration fact. It
is not mechanical authority.

### Namespace

`namespace_from_headers` resolves `X-Aether-Namespace` or the default
namespace. Namespace identity is an AETHER semantic partitioning fact.

`admit_namespace` is a bounded semaphore. It is an operational
sequencing/backpressure fact, not an authority grant.

### Policy

`apply_policy_binding` ensures a requested policy context does not exceed the
authenticated token's granted policy context and records the effective
capabilities/visibilities.

This is an AETHER policy-visibility fact. It may be an input to mechanical
authorization, but does not itself create a mechanical envelope.

### Capacity and liveness

The bounded blocking executor, rate limiter, queue depth, available permits and
resource snapshots are mechanical/operational observations only. They cannot
create or widen authority.

## 3. Required new AETHER fact

No existing record exactly means:

> AETHER admits this exact request/operation class to request bounded mechanical
> realization under this exact mechanical profile.

C0 therefore requires a new AETHER-owned typed fact:

`AetherOperationAdmissionDecision`

It is narrower than semantic success and earlier than
`SemanticExecutionStarted`.

It means only that AETHER has completed the pre-mechanical checks required to
allow this exact operation to request mechanical realization.

It SHALL bind:

- operation class;
- request/correlation identity;
- namespace;
- authenticated principal reference;
- token reference when present, for provenance only;
- effective policy binding or explicit no-policy profile;
- exact operation manifest digest;
- operation-profile digest;
- admitted/rejected disposition;
- decision revision and time;
- AETHER source-authority identity.

It SHALL NOT imply:

- the semantic operation has started;
- append/schema/leader/sidecar mutation is accepted;
- a result is admitted;
- institutional permission;
- GHOS controller admission; or
- a resource exists.

`MechanicalAuthorizationDecision` in later C0 documents is the mechanical
projection of an admitted `AetherOperationAdmissionDecision`; the two may be
represented by one implementation type only if their distinct meanings remain
explicit.

## 4. Blocking-crossing inventory

Classification:

- **FIRST_LANE** — eligible for C1/C2 off-path/shadow issuance because the
  operation does not directly mutate AETHER authoritative semantic state.
- **FIRST_LANE_EVIDENCE_WRITE** — read/evaluate operation is eligible, but its
  current implementation persists subordinate execution/proof evidence. That
  persistence remains AETHER-owned and must occur only after semantic start.
- **EXCLUDED_MUTATION** — authoritative or provenance-bearing mutation remains
  on the reference path in the first lane. A later tranche must separately type
  its semantic-start/commit boundary.

| Handler | HTTP path | Scope | Current policy binding | Classification |
| --- | --- | --- | --- | --- |
| `history` | `GET /v1/history` | Ops | yes | FIRST_LANE |
| `history_page` | `GET /v1/history/page` | Ops | yes | FIRST_LANE |
| `append` | `POST /v1/append` | Append | principal injected; admission inside service | EXCLUDED_MUTATION |
| `append_dry_run` | `POST /v1/append/dry-run` | Append | no policy context; validation only | FIRST_LANE |
| `append_receipts_endpoint` | `GET /v1/append/receipts` | Ops | no | FIRST_LANE |
| `schema_catalog_endpoint` | `GET /v1/schema` | Query | no | FIRST_LANE |
| `register_schema_endpoint` | `POST /v1/schema/register` | Ops | no | EXCLUDED_MUTATION |
| `activate_schema_endpoint` | `POST /v1/schema/activate` | Ops | no | EXCLUDED_MUTATION |
| `current_state` | `POST /v1/state/current` | Query | yes | FIRST_LANE |
| `as_of` | `POST /v1/state/as-of` | Query | yes | FIRST_LANE |
| `parse_document` | `POST /v1/documents/parse` | Query | no | FIRST_LANE |
| `run_document` | `POST /v1/documents/run` | Query | yes | FIRST_LANE_EVIDENCE_WRITE |
| `run_document_page` | `POST /v1/documents/run/page` | Query | yes | FIRST_LANE_EVIDENCE_WRITE |
| `coordination_pilot_report` | `POST /v1/reports/pilot/coordination` | Query | yes | FIRST_LANE_EVIDENCE_WRITE |
| `coordination_delta_report` | `POST /v1/reports/pilot/coordination-delta` | Query | yes | FIRST_LANE_EVIDENCE_WRITE |
| `partition_status` | `GET /v1/partitions/status` | Ops | no | FIRST_LANE |
| `promote_replica` | `POST /v1/partitions/promote` | Ops | no | EXCLUDED_MUTATION |
| `partition_append` | `POST /v1/partitions/append` | Append | principal injected; append admission inside service | EXCLUDED_MUTATION |
| `partition_history` | `POST /v1/partitions/history` | Query | yes | FIRST_LANE |
| `partition_state` | `POST /v1/partitions/state` | Query | yes | FIRST_LANE |
| `federated_history` | `POST /v1/federated/history` | Query | yes | FIRST_LANE |
| `federated_run_document` | `POST /v1/federated/run` | Query | yes | FIRST_LANE_EVIDENCE_WRITE |
| `federated_report` | `POST /v1/federated/report` | Explain | yes | FIRST_LANE_EVIDENCE_WRITE |
| `explain_tuple` | `POST /v1/explain/tuple` | Explain | yes | FIRST_LANE |
| `resolve_trace_handle` | `POST /v1/explanations/resolve` | Explain | yes, before blocking crossing | FIRST_LANE |
| `resolve_trace_handle_page` | `POST /v1/explanations/resolve/page` | Explain | yes, before blocking crossing | FIRST_LANE |
| `register_artifact_reference` | `POST /v1/sidecars/artifacts/register` | Append | no | EXCLUDED_MUTATION |
| `get_artifact_reference` | `POST /v1/sidecars/artifacts/get` | Query | yes | FIRST_LANE |
| `register_vector_record` | `POST /v1/sidecars/vectors/register` | Append | no | EXCLUDED_MUTATION |
| `search_vectors` | `POST /v1/sidecars/vectors/search` | Query | yes | FIRST_LANE |

## 5. Why the seven mutations are excluded

The first C1/C2 lane does not attempt to externalize the semantic commit
boundary for:

- append admission/commit;
- schema registration;
- schema activation;
- replica promotion/leader-epoch authority;
- partition append;
- sidecar artifact registration;
- sidecar vector registration.

Those mutations currently happen inside AETHER service calls. Mechanical
queueing may eventually precede them, but C0 does not claim that HTTP
authentication or operation-class admission is equivalent to their semantic
acceptance.

They stay on the existing reference path until a later exact contract binds
their `SemanticExecutionStarted` and commit/admission semantics.

## 6. FIRST_LANE admission inputs

For each FIRST_LANE operation, the future AETHER issuer may consume only:

1. resolved operation class/path;
2. successful `HttpAuth::authorize` result for the required scope and
   namespace;
3. successful policy binding when the endpoint has a policy context;
4. successful pre-execution structural/resource-limit checks that can be
   performed without executing the semantic operation;
5. exact immutable operation-manifest bytes and digest;
6. a protected operation profile.

For endpoints where `apply_policy_binding` currently occurs inside the
blocking closure, C1/C2 must move or duplicate only the pure binding calculation
to the pre-mechanical decision surface. It must not move the semantic
computation itself.

## 7. FIRST_LANE_EVIDENCE_WRITE rule

`run_document` and derivative report/federated operations persist execution
receipts/traces through AETHER's execution store.

Those receipts are subordinate AETHER execution/provenance evidence. Their
persistence is not a FABRIC effect.

The C1/C2 mechanical envelope may authorize only pre-start queue/resource
realization. AETHER still owns the point at which evaluation begins and
execution evidence may be persisted.

## 8. Local-only HTTP surfaces

Current service status, audit-log readback, auth reload and similar
non-blocking-control endpoints do not cross the bounded blocking executor. They
remain local AETHER control/diagnostic paths and are outside the first FABRIC
lane.

## 9. First-lane mechanical profile

For C1/C2 shadow work, all FIRST_LANE operations use the same narrow mechanical
class unless a later protected profile explicitly narrows further:

- resource class: `local-blocking-pool`;
- capacity unit: `blocking_admission_slot`;
- required capability: `blocking_execution`;
- trust zone: `aether-process`;
- locality: `local-process`;
- permitted mechanical action: `queue_pre_start`;
- automatic retry ceiling: one attempt;
- parallel redundancy ceiling: one;
- no semantic/result authority effect.

A cross-process `dispatch_payload` action is not part of C1/C2.

## 10. C0-A disposition

The protected source inventory is sufficient to continue C0.

No existing HTTP/auth/capacity record is promoted into mechanical authority.
The missing fact is explicitly the AETHER-owned operation/mechanical
authorization decision defined for C0-B/C0-C.

Mutation endpoints are not blockers for C0 because they are excluded from the
first lane rather than silently generalized.
