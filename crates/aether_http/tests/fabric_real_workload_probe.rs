//! Non-production AETHER computation probe. The harness routes test traffic;
//! no production FABRIC route, permit, queue admission, or semantic authority is enabled.
use aether_control_bridge::{
    AetherMechanicalAuthorityIssuer, IssuerBuildIdentity, OperationAdmissionInput, OperationClass,
    PolicyBindingEvidence,
};
use aether_fabric::{
    f1b_scheduler_policy_identity, f1b_select_resource, local_blocking_pool_observation,
    realize_selected_placement, PlacementDecision, RealizationRequest, ResourceHealth,
    ResourceObservation, ResourceSnapshot, SelectorImplementationIdentity,
};
use aether_http::{http_router_with_options, AuthScope, HttpAuthConfig, HttpKernelOptions};
use aether_pilot::{coordination_pilot_dsl, coordination_pilot_seed_history};
use aether_service_core::{AppendRequest, InMemoryKernelService, KernelService};
use axum::{
    body::{to_bytes, Body},
    http::{header::AUTHORIZATION, header::CONTENT_TYPE, Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use std::{
    env,
    fs::OpenOptions,
    io::Write,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::Semaphore;
use tower::ServiceExt;

const TOKEN: &str = "isolated-real-workload-probe";
const TRIALS: usize = 16;
const INFLIGHT: usize = 24;

#[derive(Clone)]
struct LocalLabResource {
    id: &'static str,
    router: Router,
    slot: Arc<Semaphore>,
    pending: Arc<AtomicUsize>,
    active: Arc<AtomicUsize>,
    completed: Arc<AtomicUsize>,
    high_queue: Arc<AtomicUsize>,
}
impl LocalLabResource {
    fn new(id: &'static str) -> Self {
        let mut service = InMemoryKernelService::new();
        service
            .append(AppendRequest {
                datoms: coordination_pilot_seed_history(),
            })
            .expect("source-pinned, non-empty coordination history");
        let auth = HttpAuthConfig::new().with_token(TOKEN, "nonprod-analyst", [AuthScope::Query]);
        Self {
            id,
            router: http_router_with_options(
                service,
                HttpKernelOptions::default()
                    .with_auth(auth)
                    .with_namespace_work_limits(2, 32),
            ),
            slot: Arc::new(Semaphore::new(1)),
            pending: Arc::new(AtomicUsize::new(0)),
            active: Arc::new(AtomicUsize::new(0)),
            completed: Arc::new(AtomicUsize::new(0)),
            high_queue: Arc::new(AtomicUsize::new(0)),
        }
    }
    async fn run(&self, payload: Vec<u8>) -> (StatusCode, Vec<u8>) {
        let depth = self.pending.fetch_add(1, Ordering::SeqCst) + 1;
        self.high_queue.fetch_max(depth, Ordering::SeqCst);
        let permit = self.slot.clone().acquire_owned().await.unwrap();
        self.pending.fetch_sub(1, Ordering::SeqCst);
        self.active.fetch_add(1, Ordering::SeqCst);
        let request = Request::builder()
            .method("POST")
            .uri("/v1/documents/run")
            .header(AUTHORIZATION, format!("Bearer {TOKEN}"))
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(payload))
            .unwrap();
        let response = self.router.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), 8 * 1024 * 1024)
            .await
            .unwrap()
            .to_vec();
        self.active.fetch_sub(1, Ordering::SeqCst);
        self.completed.fetch_add(1, Ordering::SeqCst);
        drop(permit);
        (status, bytes)
    }
    fn observation(&self) -> ResourceObservation {
        let available = self.slot.available_permits() as u64;
        let mut observation = local_blocking_pool_observation(
            1,
            available,
            self.pending.load(Ordering::SeqCst) as u64,
        );
        observation.resource_id = self.id.to_owned();
        observation
    }
}

fn work_payload() -> Vec<u8> {
    let dsl = coordination_pilot_dsl(
        "current",
        "goal execution_authorized(t, worker, epoch)\n  keep t, worker, epoch",
    );
    serde_json::to_vec(&json!({"dsl": dsl})).unwrap()
}

fn semantic_projection(response: &(StatusCode, Vec<u8>)) -> Value {
    assert_eq!(
        response.0,
        StatusCode::OK,
        "real source-authorized HTTP execution failed: {}",
        String::from_utf8_lossy(&response.1)
    );
    let parsed: Value = serde_json::from_slice(&response.1).unwrap();
    let rows = parsed
        .pointer("/query/rows")
        .and_then(Value::as_array)
        .expect("query rows must be present");
    assert!(
        !rows.is_empty(),
        "workload must derive meaningful query results"
    );
    json!({
        "state":parsed.get("state"),
        "derived":parsed.get("derived"),
        "query":parsed.get("query"),
    })
}
fn millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
fn issuer() -> AetherMechanicalAuthorityIssuer {
    AetherMechanicalAuthorityIssuer::for_c2_shadow(
        "nonprod-lab-issuer",
        "probe/1",
        IssuerBuildIdentity {
            source_commit: "nonprod-probe".into(),
            source_tree: "nonprod".into(),
            artifact_sha256: "a".repeat(64),
        },
        30_000,
        80,
    )
    .unwrap()
}
fn selector() -> SelectorImplementationIdentity {
    SelectorImplementationIdentity {
        selector_id: "protected-f1b-probe".into(),
        source_commit: "nonprod-probe".into(),
        source_tree: "nonprod".into(),
        artifact_sha256: "b".repeat(64),
    }
}
fn admission(t: u64, tag: &str, payload: &Value) -> OperationAdmissionInput {
    let p = OperationClass::RunDocument.profile();
    OperationAdmissionInput {
        request_id: format!("probe-{tag}"),
        correlation_id: format!("probe-corr-{tag}"),
        operation_class: OperationClass::RunDocument,
        http_method: p.http_method,
        http_path: p.http_path,
        namespace_ref: "default".into(),
        principal_ref: "nonprod-analyst".into(),
        token_ref: Some("nonprod-analyst-token".into()),
        required_scope: p.required_scope,
        policy_binding: PolicyBindingEvidence::Bound {
            effective_policy_digest: "c".repeat(64),
        },
        typed_request_payload: payload.clone(),
        source_operation_admitted: true, // Harness assumption, NOT production authority.
        decision_revision: "harness/1".into(),
        decided_at_unix_ms: t,
        source_authority_ref: "laboratory-assumption-only".into(),
    }
}
fn snapshot(t: u64, reference: &LocalLabResource, spare: &LocalLabResource) -> ResourceSnapshot {
    ResourceSnapshot::new(
        format!(
            "observed-{t}-{}-{}",
            reference.pending.load(Ordering::SeqCst),
            reference.active.load(Ordering::SeqCst)
        ),
        t,
        "observed-harness-semaphore-queue-not-aether-production",
        vec![reference.observation(), spare.observation()],
    )
    .unwrap()
}
fn pctl(values: &mut [f64], pct: usize) -> f64 {
    values.sort_by(|a, b| a.total_cmp(b));
    let idx = (values.len() - 1) * pct / 100;
    values[idx]
}
async fn wait_for_backlog(reference: &LocalLabResource) {
    let wait = async {
        loop {
            if reference.active.load(Ordering::SeqCst) > 0
                && reference.pending.load(Ordering::SeqCst) > 0
            {
                break;
            }
            tokio::task::yield_now().await;
        }
    };
    tokio::time::timeout(std::time::Duration::from_secs(10), wait)
        .await
        .expect("real background AETHER workload must create observed backlog");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn nonproduction_real_workload_observed_contention_and_inflight_fallback() {
    let reference = LocalLabResource::new("probe-reference");
    let spare = LocalLabResource::new("probe-spare");
    let payload = work_payload();
    let payload_value: Value = serde_json::from_slice(&payload).unwrap();

    // Both actual HTTP handlers evaluate the same seeded, nontrivial rule programme.
    let original = reference.run(payload.clone()).await;
    let same = spare.run(payload.clone()).await;
    let expected = semantic_projection(&original);
    assert_eq!(expected, semantic_projection(&same));
    let semantic_digest =
        aether_fabric::sha256_hex(&aether_fabric::canonicalize_serializable(&expected).unwrap());
    let input_digest = aether_fabric::sha256_hex(&payload);

    // Background saturation is *real AETHER computation*, not an injected sleep.
    // Queue counters belong to this isolated, bounded harness; they are not
    // presented as metrics from AETHER's hidden production blocking executor.
    let stop = Arc::new(AtomicBool::new(false));
    let mut background = Vec::new();
    for _ in 0..4 {
        let lane = reference.clone();
        let p = payload.clone();
        let flag = stop.clone();
        background.push(tokio::spawn(async move {
            let mut completed = 0usize;
            while !flag.load(Ordering::SeqCst) && completed < 256 {
                let out = lane.run(p.clone()).await;
                assert_eq!(out.0, StatusCode::OK);
                completed += 1;
            }
            completed
        }));
    }
    wait_for_backlog(&reference).await;
    let observed_queue_depth = reference.pending.load(Ordering::SeqCst);
    assert!(observed_queue_depth > 0);
    let mut controller = issuer();
    let mut baseline_ms = Vec::new();
    let mut candidate_ms = Vec::new();
    let mut selector_overhead_ms = Vec::new();
    let mut selected = Vec::new();
    let mut failed_candidates = 0usize;

    for i in 0..TRIALS {
        let first = Instant::now();
        let baseline = reference.run(payload.clone()).await;
        baseline_ms.push(first.elapsed().as_secs_f64() * 1000.0);
        assert_eq!(semantic_projection(&baseline), expected);

        // AETHER HTTP still performs actual token/policy admission on each
        // outgoing request. C1 issuance is merely a harness permission model.
        let tag = format!("trial-{i}");
        let time = millis();
        let input = admission(time, &tag, &payload_value);
        let bundle = controller.issue_control_bundle(&input).unwrap();
        controller.verify_bundle(&bundle).unwrap();
        let selected_at = Instant::now();
        let choice = f1b_select_resource(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &snapshot(time, &reference, &spare),
            &bundle.control_state_witness,
            &f1b_scheduler_policy_identity(),
            &selector(),
        )
        .unwrap();
        let p = match choice {
            PlacementDecision::Selected(p) => p,
            PlacementDecision::Unavailable(_) => {
                failed_candidates += 1;
                continue;
            }
        };
        let fresh_t = time + 1;
        let current_witness = controller
            .registry()
            .observe(&bundle.placement_constraint.envelope_id, fresh_t)
            .unwrap();
        let route = realize_selected_placement(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &p,
            &snapshot(fresh_t, &reference, &spare),
            &current_witness,
            &RealizationRequest {
                realization_request_id: format!("realize-{tag}"),
                realization_time_unix_ms: fresh_t,
                occurred_at: "2026-10-09T00:00:00Z".into(),
            },
        )
        .unwrap();
        selector_overhead_ms.push(selected_at.elapsed().as_secs_f64() * 1000.0);
        assert_eq!(route.authority_effect, "none");
        selected.push(route.endpoint_id.clone());
        // Only the test harness, never production AETHER, uses the selection.
        assert_eq!(route.endpoint_id, spare.id);
        let start = Instant::now();
        let candidate = spare.run(payload.clone()).await;
        candidate_ms.push(start.elapsed().as_secs_f64() * 1000.0);
        assert_eq!(semantic_projection(&candidate), expected);
    }
    assert!(
        !candidate_ms.is_empty(),
        "must realize at least one test placement"
    );

    // Make two more placement decisions, then withdraw permission / capacity.
    let control_t = millis();
    let revoked_bundle = controller
        .issue_control_bundle(&admission(control_t, "revoke", &payload_value))
        .unwrap();
    let revoke_snapshot = snapshot(control_t, &reference, &spare);
    let revoke_placement = match f1b_select_resource(
        &revoked_bundle.envelope_bytes,
        &revoked_bundle.placement_constraint,
        &revoke_snapshot,
        &revoked_bundle.control_state_witness,
        &f1b_scheduler_policy_identity(),
        &selector(),
    )
    .unwrap()
    {
        PlacementDecision::Selected(p) => p,
        PlacementDecision::Unavailable(_) => panic!("spare must be eligible"),
    };
    controller
        .revoke_envelope(
            &revoked_bundle.placement_constraint.envelope_id,
            control_t + 1,
            "nonproduction test revocation",
        )
        .unwrap();
    let after_revoke = controller
        .registry()
        .observe(
            &revoked_bundle.placement_constraint.envelope_id,
            control_t + 2,
        )
        .unwrap();
    let denied = realize_selected_placement(
        &revoked_bundle.envelope_bytes,
        &revoked_bundle.placement_constraint,
        &revoke_placement,
        &snapshot(control_t + 2, &reference, &spare),
        &after_revoke,
        &RealizationRequest {
            realization_request_id: "revoke-attempt".into(),
            realization_time_unix_ms: control_t + 2,
            occurred_at: "2026-10-09T00:00:00Z".into(),
        },
    );
    assert!(denied.is_err());

    // Separate injected resource-loss case: candidate placement exists, but the
    // resource becomes unavailable before fresh realization. This is still an
    // isolated *fault injection*, not a physical host failure observation.
    let lost_t = millis();
    let lost_bundle = controller
        .issue_control_bundle(&admission(lost_t, "resource-loss", &payload_value))
        .unwrap();
    let lost_placement = match f1b_select_resource(
        &lost_bundle.envelope_bytes,
        &lost_bundle.placement_constraint,
        &snapshot(lost_t, &reference, &spare),
        &lost_bundle.control_state_witness,
        &f1b_scheduler_policy_identity(),
        &selector(),
    )
    .unwrap()
    {
        PlacementDecision::Selected(p) => p,
        PlacementDecision::Unavailable(_) => {
            panic!("resource-loss setup needs a selected candidate")
        }
    };
    let mut current_resources = vec![reference.observation(), spare.observation()];
    for resource in &mut current_resources {
        if resource.resource_id == lost_placement.resource_id {
            resource.health = ResourceHealth::Unavailable;
            resource.available_capacity.available = 0;
        }
    }
    let lost_snapshot = ResourceSnapshot::new(
        "fault-injected-unavailable",
        lost_t + 2,
        "nonproduction-fault-injection",
        current_resources,
    )
    .unwrap();
    let fresh_witness = controller
        .registry()
        .observe(&lost_bundle.placement_constraint.envelope_id, lost_t + 2)
        .unwrap();
    let loss_blocked = realize_selected_placement(
        &lost_bundle.envelope_bytes,
        &lost_bundle.placement_constraint,
        &lost_placement,
        &lost_snapshot,
        &fresh_witness,
        &RealizationRequest {
            realization_request_id: "lost-selected-resource".into(),
            realization_time_unix_ms: lost_t + 2,
            occurred_at: "2026-10-09T00:00:00Z".into(),
        },
    )
    .is_err();
    assert!(loss_blocked);

    // An in-flight fallback test switches only the *harness* routing flag.
    // Candidate requests already launched remain on their originally chosen
    // isolated HTTP router, and complete exactly once; the new request goes
    // to the original AETHER router, still authoritative throughout.
    stop.store(true, Ordering::SeqCst);
    for task in background {
        task.await.unwrap();
    }
    let mut inflight_tasks = Vec::new();
    let spare_completions_before = spare.completed.load(Ordering::SeqCst);
    for _ in 0..INFLIGHT {
        let lane = spare.clone();
        let data = payload.clone();
        inflight_tasks.push(tokio::spawn(async move { lane.run(data).await }));
    }
    wait_for_backlog(&spare).await;
    let candidate_inflight_at_fallback =
        spare.active.load(Ordering::SeqCst) + spare.pending.load(Ordering::SeqCst);
    assert!(candidate_inflight_at_fallback > 0);
    let lab_use_fabric = AtomicBool::new(true);
    lab_use_fabric.store(false, Ordering::SeqCst);
    let post_fallback = if lab_use_fabric.load(Ordering::SeqCst) {
        spare.run(payload.clone()).await
    } else {
        reference.run(payload.clone()).await
    };
    assert_eq!(semantic_projection(&post_fallback), expected);
    for task in inflight_tasks {
        assert_eq!(semantic_projection(&task.await.unwrap()), expected);
    }
    assert_eq!(
        spare.completed.load(Ordering::SeqCst) - spare_completions_before,
        INFLIGHT
    );

    // Measure without a claim that candidate always wins; the result may be negative.
    let baseline_p50 = pctl(&mut baseline_ms, 50);
    let baseline_p95 = pctl(&mut baseline_ms, 95);
    let candidate_p50 = pctl(&mut candidate_ms, 50);
    let candidate_p95 = pctl(&mut candidate_ms, 95);
    let selector_p50 = pctl(&mut selector_overhead_ms, 50);
    let measured_benefit = candidate_p50 + selector_p50 < baseline_p50;

    let receipt = json!({
        "kind":"AETHER_FABRIC_E3B_NONPRODUCTION_REAL_WORKLOAD_PROBE/1",
        "production_routing_unchanged":true,
        "c3_lane_classification_complete":false,
        "c4_readiness_complete":false,
        "source_admission_for_control_bundle":"harness_assumption_only",
        "observed_queue_source":"harness_owned_semaphore_reflecting_real_AETHER_HTTP_computation",
        "artificial_queue_delay_ms":0,
        "real_workload":"coordination_pilot_seed_history plus current execution_authorized rule evaluation",
        "actual_http_scope":"query",
        "input_sha256":input_digest,"semantic_projection_sha256":semantic_digest,
        "trial_pairs":TRIALS,"realized_candidate_trials":candidate_ms.len(),
        "unavailable_candidate_trials":failed_candidates,
        "observed_reference_queue_depth_at_start":observed_queue_depth,
        "reference_high_water_queue_depth":reference.high_queue.load(Ordering::SeqCst),
        "reference_p50_ms":baseline_p50,"reference_p95_ms":baseline_p95,
        "candidate_p50_ms":candidate_p50,"candidate_p95_ms":candidate_p95,
        "selector_p50_ms":selector_p50,
        "observed_lab_p50_benefit":measured_benefit,
        "selected_resources":selected,
        "revocation_prevented_realization":true,
        "fault_injected_selected_resource_loss_prevented_realization":loss_blocked,
        "candidate_work_in_flight_at_harness_fallback":candidate_inflight_at_fallback,
        "inflight_candidate_requests_completed_exactly_once":true,
        "fresh_reference_request_during_harness_fallback":true,
        "real_AETHER_dispatcher_fallback_proven":false,
        "real_production_workload_cutover_proven":false,
        "multi_machine_placement_proven":false,
        "production_activation_authorized":false
    });
    let receipt_text = serde_json::to_string_pretty(&receipt).unwrap();
    println!("E3B_NONPROD_REAL_WORKLOAD_RECEIPT={receipt_text}");
    if let Ok(path) = env::var("AETHER_FABRIC_REAL_PROBE_RECEIPT") {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .unwrap();
        file.write_all(receipt_text.as_bytes()).unwrap();
        file.sync_all().unwrap();
    }
}
