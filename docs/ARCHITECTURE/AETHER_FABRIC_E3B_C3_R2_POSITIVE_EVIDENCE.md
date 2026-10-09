# AETHER/FABRIC E3B-C3-R2 — Positive-case HTTP evidence and irreducible legacy exception

Status: CANDIDATE — exact-head protected review/merge/readback pending
Parent governance: #110
Open evidence repair: #127
Protected R1 predecessor: 6185368fdcfa1375a9623de5b776b4b65f684f13

## Purpose

R1 corrected the false claim that 23 direct C2 controller operations
amount to 23 successful authenticated HTTP requests. Its empty-fixture
matrix had 11 successful requests and 12 matching domain errors.

R2 supplies genuine, valid fixtures for all twelve domain-error routes:
- source-locked coordination-pilot journal history and parsed DSL;
- valid current/as-of cuts;
- two separate in-memory AETHER kernel services, each seeded with the
  same history, document execution and corresponding trace handle;
- independent SQLite leader partition instances with identical topology;
- registered artifact references bound to the latest current journal element.

Both reference-only and shadow-enabled Axum routers execute the entire
23-operation first lane using an authenticated principal with the same
scopes. HTTP statuses are compared for every route. The shadow path
remains non-operative and routing remains ReferenceOnly.

## Actual 23-route result

22 authenticated HTTP operations completed successfully with HTTP 200:

history, history_page, append_dry_run, append_receipts,
schema_catalog, current_state, as_of, parse_document,
run_document, run_document_page, coordination_pilot_report,
coordination_delta_report, partition_status, partition_history,
partition_state, federated_history, federated_run_document,
federated_report, resolve_trace_handle, resolve_trace_handle_page,
get_artifact_reference, search_vectors.

One operation, explain_tuple, returned HTTP 409 on *both* paths, with
the exact structured error code ambiguous_tuple_reference and
matching error/details. This is not a fixture failure: the primary
service implementation of KernelServiceCore::explain_tuple in
crates/aether_service_core/src/lib.rs unconditionally returns
ApiError::AmbiguousTupleReference. The 23/23-success condition is
therefore not satisfiable under current protected AETHER semantic API.

The R2 test requires:
- the exact 22/1 status and error-code split, not 23 artificial green;
- all 23 requests admitted and paired in the real C2 shadow stream;
- 22 records individually classified equivalent with HTTP 200 and
  digest bound to authoritative reference completion;
- explain_tuple to remain 409, classified non-equivalent/unpaired;
- all-23 aggregate success certification to FAIL CLOSED with
  DifferentialDisagreement;
- FabricRoutingMode::ReferenceOnly unchanged.

## What is proved and what is not

This validates positive reference completion and non-operative FABRIC
mechanical comparison for the 22 functioning HTTP first-lane operations
under the protected F1E one-local-resource domain. It validates the
remaining endpoint as an intentionally denied legacy API, without
altering its semantic contract.

It does not prove 23 successful HTTP results, and it does not make
an HTTP error into a FABRIC production authorization. The original
23/23-positive acceptance predicate is inconsistent with current
AETHER explain_tuple semantics.

The correct next step is a separately reviewed source-of-truth
classification decision: either freeze the first lane as
22 positive + 1 declared negative-control legacy endpoint, or specify
a semantic API change/replacement that genuinely permits 23 positive
operations. The latter may trigger semantic/constitutional authority
governance; it cannot be silently performed as a C3 evidence fix.

Other limitations persist: the replay export is explicit, not an
automatically configured continuous durable/signed capture sink;
the inner protected F1E comparison covers only one local blocking
resource pool. E3B-A, live FABRIC routing, and C4 activation-readiness
remain unapproved/blocked until governing requirements are satisfied.
