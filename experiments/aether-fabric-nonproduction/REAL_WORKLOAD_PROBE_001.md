# E3B non-production real-workload probe — first measured evidence

Disposition: **PARTIAL LABORATORY PASS / FULL PRACTICAL ACCEPTANCE WITHHELD**. This record is a non-operative engineering observation, not C3 closure, C4 readiness, or E3B-A activation authority.

## Exact evidence

- Exact tested code/workflow commit: `ac20a0a604d04e20d97a2bf50074f6ec1dd2179e`.
- Replayed GitHub-hosted Ubuntu workflow: [run 37905154967](https://github.com/grandchallenge/AETHER/actions/runs/37905154967).
- Create-new test execution artifact: [artifact 11603809452](https://github.com/grandchallenge/AETHER/actions/runs/37905154967/artifacts/11603809452). The artifact contains the exact job log and JSON receipt; successful execution was inspected in the run logs.
- Test source: `crates/aether_http/tests/fabric_real_workload_probe.rs`. CI calls the test explicitly; no production switch or semantic/authority code was changed.
- Source input SHA-256: `ed2936d4e7ff955ff5b353343e35e30c27eadeb04338abb4c36bdfd01a9c3c4c`; AETHER state/derived/query projection SHA-256: `b6187a10b4377872f872a880d2ff5ca198476d3060368e017d4246d8bbdecfb6`.

## What was actually run

Two freshly instantiated in-memory AETHER services were seeded with genuine coordination-pilot history. Both received authenticated HTTP `POST /v1/documents/run` requests for the real `current` `execution_authorized` rule. Each response was checked for successful execution and substantive, matching query/derived-state results. Sixteen paired baseline and candidate probes ran while four background workers continuously submitted genuine AETHER computations to the original test route.

The test harness owns a single-slot semaphore on each local router. It measured *real queueing of genuine requests at that harness boundary*, not queue depth from a production AETHER executor. There was **no artificial wait inserted**. FABRIC's protected F1B logic selected the less-congested alternative, and its fresh realization validator supplied non-authoritative evidence. The test harness, not live FABRIC dispatch, sent the HTTP request to the selected router.

## Observations on the exact run

| Metric | Original congested local route | Alternative local route |
| --- | ---: | ---: |
| Median HTTP request time, 16 trials | 34.100953 ms | 11.469116 ms |
| 95th percentile by recorded sample-rank method | 42.177661 ms | 13.075056 ms |

Median selector/realization processing overhead was 1.722103 ms, recorded separately from candidate HTTP time. The harness reference queue had depth 1 at sampling and observed a peak depth of 5. The benefit is **local under deliberately imposed contention by actual computation**; these figures cannot establish production throughput, energy benefit, cost saving, multi-host placement, or general speedup.

## Faults and fallback

- Revocation after placement and before fresh realization: **candidate blocked**.
- A selected resource marked unavailable by explicit **fault injection** after placement: **candidate blocked**. This does not simulate a full physical-machine outage.
- During a harness-only routing-flag switch, **7 candidate HTTP operations were in flight**. All 24 launched candidate requests completed once and matched the semantic projection; a new request completed via the original AETHER router.
- These observations do **not** establish genuine AETHER control-path cutover or production in-flight migration/recovery. AETHER's original live execution mechanism was never replaced.

## Integrity, replay, and exceptions

All observations are tied to this exact source candidate and hosted run. The complete test run reported one success and no test failures. An earlier diagnostic replay exposed a failure caused by the hosted log masking the bearer-token source string; it was corrected and followed by this fresh exact-head replay. Never reuse log-rendered source as authoritative source bytes.

The C1 `source_operation_admitted` input remains a **harness assumption**. The real HTTP requests themselves do go through AETHER's normal authentication. The candidate has no live AETHER execution authority, acquires no production permit, and does not route through production AETHER/FABRIC. Neither multi-machine selection nor actual production-pool capacity was tested. p99/throughput/resource-cost and representative heterogeneous machines remain unmeasured.

## Required before the actual acceptance question can be answered

The first-lane `explain_tuple` classification in [#130](https://github.com/grandchallenge/AETHER/issues/130) is still unsettled. Full [#133](https://github.com/grandchallenge/AETHER/issues/133) acceptance additionally needs an AETHER source-bound, governed, default-off non-production dispatcher; real internal executor queue/capacity metrics; safe, fault-injected active-work cutover semantics; reproducible independent review; and repeatable multi-resource measurements. It cannot be inferred from this harness. E3B-A production activation is a distinct reserved approval and remains **not authorized**.
