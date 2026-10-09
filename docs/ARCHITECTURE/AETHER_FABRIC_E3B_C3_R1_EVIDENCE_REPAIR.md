# AETHER/FABRIC E3B-C3-R1 — HTTP evidence integrity repair

Status: PROTECTED PARTIAL REPAIR. Issue #127 remains OPEN. Parent #110; original C3 issue #124.
Exact R1 reviewed head: 9a055c676207cdbd8208dbafb1b01842eb4aa22a
Protected R1 merge/readback: 6185368fdcfa1375a9623de5b776b4b65f684f13
Exact-head CI, GCL conformance and Supply Chain: PASS.
Residual positive-case evidence is documented in R2. The original
unqualified 23-positive certificate remains WITHHELD.

Protected predecessor: `abd43a849dd3fe7acb0abf881ebc3e737f05a15a`.

## Why this correction exists

The original C3 completion receipt classified 23 controller/profile exercises as
first-lane differential closure. A later review found that the adjudicator
accepted pre-labelled verdicts and the 23-profile test did not execute 23
authenticated HTTP requests or pair outcomes with the authoritative responses.

The original C3 mechanical-placement/no-widening tests remain valid inside
their stated one-resource domain, but the broader real-HTTP equivalence
certificate was premature. This tranche preserves the prior work and
**withdraws the unqualified all-23 HTTP-equivalence interpretation**.

## Repair

1. Eligible authenticated reference-path operations now attach their actual
   completion status and SHA-256 of their canonical serialized successful
   result to the corresponding C2 request identity. This is computed on the
   authoritative reference result and cannot affect that result.
   `execute`, `execute_partitioned` and trace-resolution completion paths
   participate. Failed reference requests carry the status but do not
   acquire a fabricated success/result digest.
2. The controller's existing bounded evidence queue now counts evictions.
   `c3_replay_bundle` discloses that count. An overflow causes replay
   adjudication to fail, not to silently certify incomplete coverage.
   `export_c3_replay_bundle` writes JSON using create-new semantics,
   synchronizes it to storage, refuses overwrites and supports replay.
   Export is *explicit*, not an automatically configured production sink.
3. C3 records retain operation-profile, issuer-build, selector-build,
   effective-policy and canonical-request-payload digests alongside the
   existing exact request/admission/envelope/placement/attempt/control and
   resource identities. The adjudicator rejects absent or malformed
   provenance. The final 23-operation checker independently revalidates
   operation/method/path/profile bindings, non-empty request/subject,
   identity/digest presence, distinct request IDs, resource-selection
   projection, reference-only authority, HTTP success status and
   reference-result digest, even if a caller supplied the label `Equivalent`.
4. An authenticated in-process HTTP-router test dispatches all 23 first-lane
   handlers through the actual Axum router and compares their statuses
   with a separately initialized reference-only router. All 23 statuses
   agree; representative stable successful JSON bodies agree. This is real
   routed-handler coverage, **not 23 network socket sessions** and not a
   claim that all 23 requests successfully executed their domain semantics.
5. New hostile tests reject fake pre-labelled 23-profile coverage without
   paired reference outcomes, broken build/profile digests, duplicate
   request identities, wrong routes, dropped records, changed revisions,
   and overwrite of existing replay evidence.

## Current exact acceptance evidence, before protected reviews

In the new 23-route in-process HTTP test, 11 returned HTTP 200:
history, history_page, append_dry_run, append_receipts, schema_catalog,
current_state, coordination_pilot_report, coordination_delta_report,
partition_status, federated_history, search_vectors.

The other 12 reached the authenticated HTTP handlers, produced paired
reference statuses, and matched reference-only statuses, but did not
produce successful domain results in the empty-fixture test:
- HTTP 400: as_of, parse_document, run_document, run_document_page,
  partition_history, partition_state, federated_run_document,
  federated_report, get_artifact_reference.
- HTTP 409: explain_tuple.
- HTTP 404: resolve_trace_handle, resolve_trace_handle_page.

Consequently `adjudicate_c3_replay_bundle` correctly withholds the
all-23 positive coverage certificate. The 23 direct C2 controller
exercise is expressly a structural test; it now **expects an error**
if used as completed HTTP evidence.

## Remaining evidence boundary

The original broad C3 requirement is NOT yet completely discharged.
A further positive-case fixture tranche must seed the required AETHER
journal, valid DSL, partition topology, execution/trace handles and
artifact references so all 23 handlers return successful reference
results with trustworthy paired digests. Reference-result digests are
not cross-process signatures or independently reproducible raw-result
archives; the replay bundle must be captured from the trusted
AETHER-owned controller and externally corroborated by exact-head tests.
For continuous operation, an optional durable capture sink and
tamper/loss controls would require further reviewed implementation;
create-new snapshot export is deliberate/manual here.

This repair does not establish multi-resource selection equivalence
(the protected F1E domain is one local blocking resource pool).

No C4 or E3B-A activation gate is waived. `ReferenceOnly` remains the
only production-authoritative path; no FABRIC permit, reservation, queue,
dispatch, operative realization, cross-process authority, semantic
state write, response mutation or activation is authorized.

## Post-review C3/R1 verdict hardening

The repaired C3 observation constructor emits disagreement, never
equivalent, unless the paired reference HTTP result completed with
HTTP 200 and a canonical result digest. The aggregate checker then
reconstructs a C2 source projection from every stored observation,
recomputes its individual verdict, and refuses mismatched labels or
missing/duplicate request evidence. This also makes a direct controller
test, a failed handler, or a partially captured response incapable
of proving full 23-operation successful-reference closure.

The resulting record remains a trusted-controller observation, not
cryptographic proof that arbitrary externally supplied JSON came
from AETHER. Full positive-case fixture coverage and continuous
durable capture are still unproven and must not be inferred.
