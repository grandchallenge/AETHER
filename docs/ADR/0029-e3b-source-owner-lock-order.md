# ADR 0029 — E3B source ownership and lock order

**Status:** Candidate for #146; NON-OPERATIVE TEST-ONLY, not authority to dispatch.  
**Exact predecessor:** protected main 0fb224a10b4a77b753495eafb8df4f8c55c2b493.  
**Authority:** C0, ADR 0025–0028, #136/#140/#143, #130 and #133.

## Current source ordering and missing linearization

The reference HttpKernelState::execute takes the HttpAuth mutex for bearer-token/scope/namespace authorization and releases it before the rate limiter and per-namespace semaphore. C2 shadow observation can run before rate/namespace admission. BoundedBlockingExecutor::run takes admission and worker semaphore permits across an async wait, then invokes spawn_blocking. The run_document effective policy is bound INSIDE the AETHER semantic closure, not at HTTP authentication or queue admission. Cancellation before worker start and completion after genuine start are distinct.

Auth reload holds status then auth (status -> auth), replacing HttpAuth and incrementing only an in-process generation. RateLimiter::admit takes the bucket mutex. admit_namespace takes namespace_admission while retrieving a semaphore and then releases it. C2ShadowController has its own issuer mutex and evidence mutex; the real EnvelopeControlRegistry is accessible for mutation only through its AETHER issuer capability. Its observe method reports active/revoked/superseded/expired at a supplied time, but no C1 guard is held with token/policy/namespace state or the actual worker start. FABRIC candidate realization is read-only.

**No existing atomic current-source/control-state transaction spans these objects.** A captured C1 witness, a matching HTTP auth generation or a worker permit is therefore subject to revocation, source reconfiguration and replay before start. The current policy and namespace sources lack independently demonstrated revisioned, revocable interfaces. The HTTP generation is not restart-safe.

## Prospective source-owner transaction (separately governed)

Prefer a single AETHER-owned synchronous source-owner actor, rather than unrelated new locks. It must own the current token/scope and authenticated principal, effective policy and namespace revisions, C1 envelope lifecycle, exact RouteRealized attempt identity, one-shot consumption ledger and trustworthy expiry clock. Authenticate and obtain ordinary queue/worker permits outside that owner. Immediately before any future mechanical start, enter the owner ONCE, validate fresh source/envelope/resource/profile/binding/attempt, atomically consume at most one start, and exit BEFORE semantic work. Do not hold any owner guard over await, selector calls, audit emission or semantic execution.

Revocation, supersession, auth reload, policy and namespace changes MUST linearize against that same owner start transaction. The existing status -> auth reload edge needs a specified deadlock-free migration; inserting an owner guard inside the existing lock graph without global order can deadlock. A worker acquired before revocation must not start after revocation wins linearization. An already-started synchronous result cannot be undone by cancelling the HTTP future. A failed/poisoned owner or missing source revision denies only new mechanical starts; the reference route is unchanged.

On restart, either persist an authenticated, process-incarnation-bound replay/consumption ledger, or fail closed on all pre-restart attempts. Reinitializing epochs to zero and reconstructing an attempt from serialized records is forbidden. Clock rollback/skew and expiry require a future trusted source design. Real source state APIs, attempt identity lifetime, and Article IX/XI classification must precede any live integration.

## Bounded test-only prototype

The module source_owner_prototype.rs is compiled only with cfg(test). It takes the real C1 in-process issuer and EnvelopeControlRegistry inside one harness mutex together with SIMULATED auth/policy/namespace revisions and a one-attempt model lifecycle. Its C1 issuance fixture explicitly fabricates source_operation_admitted for TESTS; this is NOT genuine HTTP admission and cannot become a dispatch authority. It tests real C1 revocation, current C1 expiry, duplicate start rejection, finish-after-revocation, simulated reload and absent revisions, wrong resource, cancellation, poisoned owner and fail-closed restart. The real HttpAuth, semantic policy, namespace and worker objects are NOT connected. PR #144 already covers actual executor cancellation separately.

This is a useful C1 concurrency/ownership prototype, not proof that the production locks are unified or that any actual FABRIC work starts. No live_fabric, second resource, new issuer capability, article IX semantic ownership transfer, C3 #130 reinterpretation, #133 practical acceptance or E3B-A activation is approved.

## Constitutional classification and next gate

This exact test-only/documentation delta is NO_ARTICLE_IX_AUTHORITY_CHANGE and not a production Article X permission expansion. No Article XI amendment is implied merely by this non-operative prototype. A separately proposed operative source-owner, policy/reload reordering, or FABRIC-directed start must receive fresh exact-source Article IX/X/XI classification and reserved dispositions wherever applicable, followed by exact-head protected tests. Production activation remains expressly unset.
