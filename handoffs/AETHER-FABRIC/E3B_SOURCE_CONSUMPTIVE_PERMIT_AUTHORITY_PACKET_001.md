# E3B Source-Issued Consumptive Start — Authority Packet 001

**State:** PREPARATION_ONLY / NOT APPROVED FOR OPERATIVE IMPLEMENTATION OR DEPLOYMENT.  
**Source basis:** AETHER protected main `566e7f4743a64d33e4a9ef378f4fa6f66466efc9` following test-only #143 PR #144; C0 authority-issuance contract, C1 registry, C2 shadow, E3B-R realization, ADRs 0025–0028.  
**Linked gates:** parent #110, C3 interpretation #130, practical #133. E3B-A activation explicitly **UNSET**.

## Proposed one-lane source transaction (not implemented)

1. Actual AETHER bearer-token authorization, namespace and scope admission yield source-owned request identity; the semantic operation remains `POST /v1/documents/run`, Query scope, and current **effective** policy is bound by AETHER. C2 shadow evidence and a historical observer never count as authority.
2. AETHER's governed C1 issuer must create a same-process E1 envelope whose request/policy/profile/issuer identity is linked to that *actual* source operation. C0's `aether-http-local-blocking/1` profile remains `queue_pre_start`, only local blocking-pool class, one attempt, no backoff, one parallel copy; seven mutation endpoints excluded.
3. Any FABRIC selection/realization remains non-authoritative. The existing `CandidateReadiness` method is pure and does not acquire resources. A `RouteRealized` fact is where C0 defines the `mechanical_attempt_id`, not HTTP request creation.
4. Worker acquisition is a *necessary* mechanical condition but not proof of current control. Immediately before the blocking semantic closure begins, one AETHER-owned **atomic current-source and control-state transaction** must validate the exact request/envelope/attempt, current authenticated principal/token/namespace/scope, *effective* policy provenance and version, active nonexpired/nonsuperseded C1 registry state and exact local resource/profile, then consume a one-start record or deny. This check and consumption must linearize against relevant revocations, reloads and retries. No lock may be held during semantic rule execution.
5. A start event may be recorded only if the consuming transition succeeds; no independent FABRIC permission to start semantics. A queue timeout or caller cancellation **before** start must consume/release the attempt without starting. After a real start, caller cancellation must not fabricate rollback of completed synchronous AETHER work; evidence of actual result and delivery must remain distinct.
6. Missing source revisions, loss/poisoned locks, inconsistent registry generation, replayed/tampered request or issuer evidence and an unavailable source transaction all fail closed on **new mechanical starts**, while the authorized reference path remains independently valid.

## Outstanding proof obligations before an operative candidate

- **Current source ownership:** identify the actual policy-store revision and versioned namespace source (not just an HTTP auth epoch); bind all to the true C1 envelope and request without deserialized `source_operation_admitted: true`. The present `HttpAuth::generation` is not restart-safe.
- **Atomicity architecture:** publish a lock-order graph or a single-owner source-start actor that safely serializes auth reload's `status -> auth` lock path, C1 registry revoke/supersede, policy/namespace mutations, executor worker acquisition and attempt consumption. No await or user semantic work while holding those locks. Prove revocation between authorization, placement, queue, worker acquisition and start cannot produce a new start.
- **Attempt identity/deduplication:** retain unforgeable same-attempt identities and one-shot consumption across cancellations/retries; specify restart/crash recovery/durability or explicitly disallow reuse and demonstrate fail-closed behavior on process reset.
- **Clock/expiry:** define monotone/trusted time source, expiry checks at start, older witnessed placement epochs, clock rollback/skew, and replay after envelope expiry.
- **Hostile actual-runtime evidence:** seeded substantive `run_document` semantics; deterministic barriers for before/after real C1 revoke/supersede/expire, auth-file reload, policy/namespace change, wrong principal/scope, exhausted worker queue, cancellation, crash/restart and exact duplicate attempt. Label every model-only fixture separately from real HTTP/worker/C1 integrated tests.
- **Reference equivalence:** exact result identity and provenance, unchanged default route, no append/journal semantics mutation; deployable rollback preserving local AETHER path. Nontrivial #133 workload benefit and C3 #130 remain separately governed.
- **Security/constitutional review:** role-scoped INTELLECT Article IX/X classification of actual production delta; reserve Human Steward disposition **only where effective authority gates require**, not for routine test-only work. Any Article XI amendment follows actual constitutional trigger. Obtain distinct E3B-A activation authority and protected exact-head checks before live routing.

## Stop boundary

This packet provides **no issuer capability**, no mechanical permit, no FABRIC dispatch hook, no candidate deployment, and no authorization to change `ReferenceOnly`. Implement the source-owner transaction only through a separately admitted, classified operative change; stop and record a blocking authority decision if it crosses Article X or XI.
