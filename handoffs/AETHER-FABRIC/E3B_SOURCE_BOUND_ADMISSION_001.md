# AETHER E3B — Source-Bound Admission at the Real Execution Boundary

**Handoff ID:** `AETHER-E3B-SOURCE-BOUND-ADMISSION-001`  
**Status:** IMPLEMENTATION_HANDOFF / NON-PRODUCTION / DEFAULT-OFF  
**Assigned through:** [AETHER issue #136](https://github.com/grandchallenge/AETHER/issues/136)  
**Parent authority:** [E3B #110](https://github.com/grandchallenge/AETHER/issues/110)  
**Practical acceptance dependency:** [#133](https://github.com/grandchallenge/AETHER/issues/133)  
**Open C3 interpretation:** [#130](https://github.com/grandchallenge/AETHER/issues/130)  
**Protected implementation starting point (2026-10-09):** `253b9fe0f11fb8d03ff14e38ca18178cff19b308`  
**Execution mode:** bounded, authorized, agent-driven; no production cutover.

## 1. Assignment for a zero-context coding agent

Authenticate with GitHub using your own authorized identity, open issue #136, and read repository `AGENTS.md` **before writing code**. Retrieve this document from the protected `main` branch and establish current HEAD. If `main` has advanced since the starting SHA above, inspect the delta and bind all claims, branches, CI and final readback to the **new exact source head**, never stale receipts. GitHub authentication is a prerequisite for posting a return or PR; anonymous browsing alone cannot complete this assignment.

**Objective:** Produce and test an AETHER-owned, **non-operative source-bound admission proof** at the real HTTP-to-`BoundedBlockingExecutor` boundary. The proof must distinguish authenticated requests from requests that actually passed the relevant AETHER rate, namespace, policy and operation constraints. It must remain incapable of independently authorizing FABRIC routing. Preserve the current reference execution path, audit and semantic ownership.

Implement the bounded work end-to-end: source analysis → authority-classification/ADR → default-off source-binding implementation → hostile tests → exact-head replay → non-reserved logical Formalist/Adversary/Referee passes → PR/required controls → protected merge/readback and issue update, stopping only at a legitimate reserved or substantive boundary.

**Important:** The target is *not* a permission minted by FABRIC, and not a new general-purpose permission grant. It is AETHER's evidence of its own **actual existing admission decision** and fresh state, suitable for a *later, separately governed* mechanical authorization and realization bridge. Any change that makes the evidence operative authorization crosses a separate explicit authority gate.

## 2. Protected observations (facts, not a new authorization)

The protected source as of the starting head shows:

1. `crates/aether_http/src/http.rs`: `HttpKernelState::execute` establishes request identity, namespace and token/scope authority; C2 shadow observation can run **before** rate-limit and namespace admission; then rate-limit and `admit_namespace` are applied; the actual work enters `BoundedBlockingExecutor::run`.
2. Endpoint-specific effective policy binding, notably for `POST /v1/documents/run`, is currently applied inside the AETHER semantic-operation closure. Pre-queue HTTP authentication therefore does **not** prove the effective policy has been accepted. Do not claim an authoritative policy binding earlier than the stage at which AETHER actually resolves it.
3. `crates/aether_http/src/fabric_control_shadow.rs`: `C2ShadowController::observe_request` constructs a `source_operation_admitted: true` input for **shadow issuance**. Such a record cannot prove downstream rate admission, namespace admission, executable policy or future authorization.
4. `crates/aether_http/src/fabric_cutover.rs`: `fabric_candidate_realize_placement` is an explicitly off-path, read-only readiness verifier; it never acquires a permit or calls the live executor.
5. `crates/aether_control_bridge/src/lib.rs`: existing C1 `OperationAdmissionInput`, closed-world operation profiles, internal issuer and control registry support non-live issuance/revocation semantics. The C0 contract limits live eligible resources to `aether-local-blocking-pool`.
6. PR #135, protected merge `253b9fe0f11fb8d03ff14e38ca18178cff19b308`, proves an actual authenticated `run_document` call passes through the real HTTP/executor boundary, that a denied HTTP request does not join that queue, and that the existing FABRIC snapshot reads live executor semaphore capacity without acquiring it. It does **not** show FABRIC-directed execution.
7. #130 remains an explicit C3 decision: the legacy `ExplainTuple` path returns an intentional HTTP 409. It must not be counted as a successful positive authorization, silently changed, or used to infer 23/23 coverage.

Governing documents: `docs/ARCHITECTURE/AETHER_FABRIC_E3B_C0_AUTHORITY_ISSUANCE_CONTRACT.md`, `docs/ARCHITECTURE/AETHER_FABRIC_E3B_LIVE_CONTROL_PATH.md`, `docs/ARCHITECTURE/AETHER_FABRIC_E3B_READINESS_ADJUDICATION.md`, `AGENTS.md`, and linked source issues.

## 3. Bounded implementation sequence

### P0 — Exact-source and authority-map audit

On a clean branch, read the precise implementations of `execute`, `execute_partitioned`, `resolve_execution_trace`, `run_document`, `apply_policy_binding`, `admit_namespace`, `BoundedBlockingExecutor::run`, `C2ShadowController`, `OperationAdmissionInput` and `fabric_candidate_realize_placement`. Diagram when **each** fact becomes authoritative: method/profile validation, token/scope, namespace, policy, rate, namespace permit, worker start, semantic start/result, revocation/supersession/expiry.

Deliver an ADR or contract note that identifies (a) facts known only at authentication, (b) facts known after rate/namespace admission, (c) policy and semantic facts learned only inside the operation closure, and (d) what is currently *not* exposed as an AETHER-issued fresh authoritative control-state witness. Identify whether the proposed change is truly observational and within the protected no-Article-IX-change scope. If operative control would change Article IX, record the required Article XI lane and **do not implement that operative step**.

### P1 — AETHER-owned, non-operative source binding

Start with **one** positive operation: authenticated `POST /v1/documents/run`, Query scope, seeded substantive `execution_authorized` rule query. Introduce the smallest typed, AETHER-owned source-binding observation at the actual reference execution seam, ideally in a separated, clearly internal module. It may record:

- source request identity, exact operation manifest/profile and payload digest;
- AETHER-validated principal/token identity, namespace and required scope;
- genuine rate and namespace-admission outcomes, with the permit lifecycle distinguished from actual worker start;
- **effective** policy context digest, but only after AETHER's own policy-binding step has succeeded;
- local observation time, relevant source auth/config revision or generation when genuinely available, and a non-serializable internal association to the current request/operation where needed;
- explicit disposition `OBSERVED_NON_OPERATIVE` or rejection, **never** a dispatch permit, successful semantic attempt, fresh governed control witness or `RouteRealized`.

Do not invent a trusted version/revision if no source exposes it. If a sound authorization handle requires extending AETHER's internal authority API, propose the narrow typed API and tests first; classify authority implications before adding operative minting. Serialized JSON, user-supplied flags, C2 shadow testimony, matching endpoint strings, or an available permit cannot independently recreate the issuer capability.

**Default-OFF rule:** Existing production/default `FabricRoutingMode::ReferenceOnly` calls the identical reference operation with identical semantics and no new failure mode. Any preview/observation instrumentation must be explicit, non-operative, bounded in memory, and incapable of blocking or rerouting reference work on emission failure. Do not add `live_fabric`, bypass source admission, or change the eligible resource domain.

**Crucial ordering rule:** Do not move current policy checks ahead of the semantic execution closure or turn an emitted receipt into operative authorization silently merely to obtain a cleaner hook. If reordering is necessary for a future operative mechanism, preserve it as a separate governed design/successor with equivalence tests and constitutional analysis.

### P2 — Freshness, hostile races, and no-authority laundering

Add focused tests for:

- valid authenticated `run_document` with actual source/admission/policy facts → read-only observation tied to the exact HTTP execution and nonempty matching semantic result;
- wrong token/scope, revoked token, namespace denial, rate limit, namespace saturation, malformed payload, unsupported/excluded mutation, and policy escalation → **no successful source-bound proof**;
- policy failure that occurs *after* worker start is distinguished from policy success and from successfully enqueued work; it cannot be laundered as `source_operation_admitted = true` for an operative candidate;
- changed payload, principal, namespace, policy, source revision, source-control state, or operation profile → changed identity or rejected replay; forged/cross-request/cross-namespace/cross-principal records rejected;
- late revocation, supersession, expiry, config reload, stale snapshot, changed source decision and queue wait → no *new mechanically authorized* work on a future governed route; do not falsify a real revocation result when no such source API is yet available;
- cancellation before worker start versus completion after genuine start; one logical operation cannot duplicate attempts or acquire authority by repeated observation;
- preview failure, poison, retention exhaustion or malformed records → fail closed on **claims** while ordinary reference execution remains correct, observable and not redirected;
- default-OFF route remains unchanged; service audit, namespace isolation, quota, timeout, ordering, idempotence, semantic result/provenance and existing F1/C1/C2/C3 tests remain green.

Use *real* `HttpKernelState` / `BoundedBlockingExecutor` and genuine authenticated HTTP operations, as in PR #135. Inject the minimum deterministic scheduling controls for race tests. Distinguish controlled faults from production outages.

### P3 — Evidence and protected admission

Produce an exact-head evidence record containing: protected predecessor SHA, implementation SHA, operation and payload identity, policy proof point, source revision availability, test commands and statuses, denied matrix, race results, default-off regression, unchanged reference route, any missing source authority, nonclaims and next legitimate steps.

Run at least `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and targeted `aether_http`, `aether_control_bridge`, `aether_fabric` suites in the same exact head/CI environment. Do not substitute earlier green runs. Resolve PR review threads and required GCL, security, supply-chain and execution-routing controls; apply non-reserved Formalist, Adversary, Referee audit passes following `AGENTS.md`. Protected merge only when controls permit it, followed by exact `main` readback. Record outcomes in issue #136 and parent #133.

If the source-issued current control witness cannot be made genuine without a reserved rule, finish the **non-operative** implementation and contract, record the precise evidence gap, and prepare the next bounded governance packet instead of fabricating issuer capability.

## 4. Hard limits / authorization boundaries

- **NO E3B-A activation.** The reserved production/control-plane activation disposition is unset; neither this handoff nor a passing PR can supply it.
- **NO MULTI-RESOURCE EXPANSION.** First admitted live domain remains one `aether-local-blocking-pool`. No second endpoint, GPU/remote machine or new executor class without its own governed work.
- **NO AUTHORITY TRANSFER.** AETHER owns admission, control-state revocation and semantic start/result; FABRIC owns only mechanical selection/readiness evidence.
- **NO SEMANTIC REWRITE.** Do not redefine DSL, authority, tracing, policy, journal replay, namespace ordering or effective Article IX by implementation convenience.
- **NO C3 PROMOTION FROM C2 SHADOW.** Issue #130 must be adjudicated on its own lane; deliberate `explain_tuple` 409 is a negative observation, not positive execution.
- **NO FALSE COMPLETION.** A finished source-bound non-operative proof is a successful *preparation unit*, not a fully authorized dispatch or #133 closure.

## 5. Acceptance / explicit return

**AETHER-E3B-SOURCE-BOUND-ADMISSION-001 is complete** only when: (1) the exact-source stage map and authority classification are protected; (2) real-AETHER off-path source-bound observation and denial/race tests pass; (3) reference/default-OFF semantics remain unchanged; (4) exact-head audit/CI/PR/merge/readback is recorded; and (5) successor requirements are enumerated with named authority boundaries.

Return to [issue #136](https://github.com/grandchallenge/AETHER/issues/136) with a concise `GCL-CONTRIBUTION-RESULT/1`-labelled summary: assignment ID; PASS / PARTIAL / BLOCKED / REFUTED; exact predecessor/candidate/merge heads; PR and test run URLs; proven properties; remaining unproven claims; next permitted step. If the standard parser requires different syntax, follow the repository's canonical grammar instead of inventing protocol fields.

**Failure/blocked path:** State exactly which governance, authority, authentication, safety, materially changed-state, or evidentiary boundary prevented the next action, and report any completed safe subset. Never report `READY`, `MERGED`, `AUTHORIZED` or `ACTIVATED` before verifying it.

## 6. Successor after this handoff

Only after protected admission of the source-binding proof may a distinct governed tranche consider converting authentic, fresh AETHER source records into *operative* E1/E3 mechanical authorization at the genuine execution boundary. That tranche must separately resolve issue #130, recheck constitutional authority, remain limited to the one local pool unless expanded by a protected decision, and prepare an exact default-OFF, reversible cutover. **E3B-A production activation always remains a later, reserved action.**
