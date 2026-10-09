# AETHER/FABRIC: isolated local workload canary

**Status: non-production experimental observation only.** This test is not E3B-A activation, C4 readiness certification, genuine multi-host resource scheduling, or authorization to change the AETHER execution path.

## Question
Can the protected FABRIC selector choose a less-congested local resource, exercise a genuine authenticated AETHER request in a disposable lab, reject revoked or unavailable placements, and send a *new* request via the original AETHER path?

## Mechanism
The ignored test in `crates/aether_http/tests/fabric_nonprod_canary.rs` builds two fresh in-memory AETHER HTTP routers, each with its own service. It invokes actual authenticated `GET /v1/history` requests, and independently exercises the real protected C1 shadow control issuer, F1B selector, and fresh E3B realization validator.

The **test harness**, not AETHER's production HTTP execution path, chooses the next router on the basis of the non-authoritative candidate. The C1 shadow issuer is given an explicitly *assumed* source admission for laboratory testing, not a production authority decision. The harness does not issue permits or send live work through FABRIC.

The baseline holds new requests on one local router and injects **65 ms of artificial queue wait**. The candidate compares the same HTTP operation on another local router without this injected delay. This demonstrates *avoidable modeled waiting*, not a measured improvement under uncontrolled real traffic. Do not extrapolate median latency to production throughput, performance, financial benefit, or multi-machine capacity.

Failure cases are distinct:
- revoke a selected envelope before realization; current witness rejects it, and the harness sends no request;
- make the selected resource unavailable before realization; current resource check rejects it;
- send a fresh authenticated request through the original AETHER router after the candidate trials. This is **not** proof of migrating or finishing in-flight requests after a production rollback.
- unauthenticated HTTP is rejected by the real AETHER router.

## Reproduction (explicit opt-in)
From the repository root, with Rust available and writable `experiments/aether-fabric-nonproduction`:

```sh
AETHER_FABRIC_CANARY_NONPROD=1 AETHER_FABRIC_CANARY_RECEIPT=experiments/aether-fabric-nonproduction/receipt-<unique>.json \
  cargo test -p aether_http --test fabric_nonprod_canary -- --ignored --nocapture
```

On Windows CMD, use `set AETHER_FABRIC_CANARY_NONPROD=1` and `set AETHER_FABRIC_CANARY_RECEIPT=experiments\\aether-fabric-nonproduction\\receipt-<unique>.json` before the same cargo test command.

The test refuses to run without opt-in. Its receipt is created exclusively with create-new semantics and fsynced. The record includes exact Git HEAD, test source SHA-256, the measurements, fail-closed dispositions, and explicit nonclaims. Each run must have a unique receipt filename; failed runs must not be silently promoted.

## Acceptance and next boundary
Successful execution of this canary supplies *laboratory evidence* of routing-choice advantage under an injected delay, live HTTP result equality, pre-realization revocation/resource-loss rejection, and a fresh-request harness rollback. **It does not satisfy the original production question.** A genuine cutover demonstration still requires settled C3 lane acceptance (issue #130), source-bound AETHER admission instead of a harness assumption, a distinct governed live-path readiness decision, instrumented realistic workload/queue observations, an actual default-OFF switching implementation, in-flight failure/recovery evidence, exact-head independent review, and reserved E3B-A approval before production activation. No scope in this document supersedes those gates.
