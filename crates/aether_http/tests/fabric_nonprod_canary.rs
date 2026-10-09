//! Disposable, non-production lab. AETHER's actual HTTP routing is NEVER changed.
use aether_control_bridge::{
    AetherMechanicalAuthorityIssuer, IssuerBuildIdentity, OperationAdmissionInput, OperationClass,
    PolicyBindingEvidence,
};
use aether_fabric::{
    f1b_scheduler_policy_identity, f1b_select_resource, local_blocking_pool_observation,
    realize_selected_placement, PlacementDecision, PlacementSelected, RealizationRequest,
    ResourceHealth, ResourceSnapshot, SelectorImplementationIdentity,
};
use aether_http::{http_router_with_options, AuthScope, HttpAuthConfig, HttpKernelOptions};
use aether_service_core::InMemoryKernelService;
use axum::{
    body::{to_bytes, Body},
    http::{header::AUTHORIZATION, Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use std::{
    env,
    fs::OpenOptions,
    io::Write,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tower::ServiceExt;
const TOKEN: &str = "nonprod-aether-fabric-canary";
const BUSY_MS: u64 = 65;
fn router() -> Router {
    let auth = HttpAuthConfig::new().with_token(TOKEN, "isolated-canary", [AuthScope::Ops]);
    http_router_with_options(
        InMemoryKernelService::new(),
        HttpKernelOptions::default().with_auth(auth),
    )
}
async fn history(router: &Router, authorized: bool) -> (StatusCode, Vec<u8>) {
    let mut req = Request::builder().method("GET").uri("/v1/history");
    if authorized {
        req = req.header(AUTHORIZATION, format!("Bearer {TOKEN}"));
    }
    let resp = router
        .clone()
        .oneshot(req.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    (
        status,
        to_bytes(resp.into_body(), 1_048_576)
            .await
            .unwrap()
            .to_vec(),
    )
}
fn issuer() -> AetherMechanicalAuthorityIssuer {
    AetherMechanicalAuthorityIssuer::for_c2_shadow(
        "nonproduction-shadow-issuer",
        "lab/1",
        IssuerBuildIdentity {
            source_commit: "fixed-lab-source".into(),
            source_tree: "fixed-lab-tree".into(),
            artifact_sha256: "a".repeat(64),
        },
        30_000,
        48,
    )
    .unwrap()
}
fn input(t: u64, tag: &str) -> OperationAdmissionInput {
    let p = OperationClass::History.profile();
    OperationAdmissionInput {
        request_id: format!("canary-{tag}"),
        correlation_id: format!("corr-{tag}"),
        operation_class: OperationClass::History,
        http_method: p.http_method,
        http_path: p.http_path,
        namespace_ref: "default".into(),
        principal_ref: "isolated-canary".into(),
        token_ref: Some("lab-only".into()),
        required_scope: p.required_scope,
        policy_binding: PolicyBindingEvidence::Bound {
            effective_policy_digest: "b".repeat(64),
        },
        typed_request_payload: json!({}),
        source_operation_admitted: true, // LAB ASSUMPTION: NOT A PRODUCTION AUTHORIZATION.
        decision_revision: "nonprod-lab".into(),
        decided_at_unix_ms: t,
        source_authority_ref: "harness-assumption-not-production".into(),
    }
}
fn snapshot(t: u64, id: &str, lose_spare: bool) -> ResourceSnapshot {
    let mut busy = local_blocking_pool_observation(1, 1, 8);
    busy.resource_id = "lab-reference".into();
    let mut spare = local_blocking_pool_observation(1, if lose_spare { 0 } else { 1 }, 0);
    spare.resource_id = "lab-spare".into();
    if lose_spare {
        spare.health = ResourceHealth::Unavailable;
    }
    ResourceSnapshot::new(id, t, "isolated-local-lab", vec![busy, spare]).unwrap()
}
fn selector() -> SelectorImplementationIdentity {
    SelectorImplementationIdentity {
        selector_id: "f1b-lab".into(),
        source_commit: "lab".into(),
        source_tree: "lab".into(),
        artifact_sha256: "c".repeat(64),
    }
}
fn placement(
    issuer: &mut AetherMechanicalAuthorityIssuer,
    t: u64,
    tag: &str,
) -> (
    aether_control_bridge::IssuedMechanicalControlBundle,
    PlacementSelected,
) {
    let bundle = issuer.issue_control_bundle(&input(t, tag)).unwrap();
    issuer.verify_bundle(&bundle).unwrap();
    let p = f1b_select_resource(
        &bundle.envelope_bytes,
        &bundle.placement_constraint,
        &snapshot(t, &format!("decision-{tag}"), false),
        &bundle.control_state_witness,
        &f1b_scheduler_policy_identity(),
        &selector(),
    )
    .unwrap();
    let p = match p {
        PlacementDecision::Selected(p) => p,
        PlacementDecision::Unavailable(_) => panic!("no local candidate"),
    };
    assert_eq!(p.resource_id, "lab-spare");
    (bundle, p)
}
fn fresh_realization(
    issuer: &AetherMechanicalAuthorityIssuer,
    bundle: &aether_control_bridge::IssuedMechanicalControlBundle,
    p: &PlacementSelected,
    t: u64,
    tag: &str,
    lose_spare: bool,
) -> Result<aether_fabric::RouteRealizedEvidence, aether_fabric::RealizationError> {
    let w = issuer
        .registry()
        .observe(&bundle.placement_constraint.envelope_id, t)
        .unwrap();
    realize_selected_placement(
        &bundle.envelope_bytes,
        &bundle.placement_constraint,
        p,
        &snapshot(t, &format!("fresh-{tag}"), lose_spare),
        &w,
        &RealizationRequest {
            realization_request_id: format!("realize-{tag}"),
            realization_time_unix_ms: t,
            occurred_at: "2026-10-09T00:00:00Z".into(),
        },
    )
}
fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "opt-in disposable canary, create-new receipt required"]
async fn workload_placement_revocation_resource_loss_rollback() {
    assert_eq!(env::var("AETHER_FABRIC_CANARY_NONPROD").as_deref(), Ok("1"));
    let relative_path = env::var("AETHER_FABRIC_CANARY_RECEIPT").expect("new receipt path");
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative_path);
    let base = router();
    let spare = router();
    let expected = history(&base, true).await;
    assert_eq!(expected.0, StatusCode::OK);
    assert_eq!(history(&spare, true).await, expected);
    assert_eq!(history(&base, false).await.0, StatusCode::UNAUTHORIZED);
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let mut issuer = issuer();
    let mut baseline = vec![];
    let mut candidate = vec![];
    for _ in 0..6 {
        let start = Instant::now();
        tokio::time::sleep(Duration::from_millis(BUSY_MS)).await; // modeled queue wait, NOT measured production backlog.
        assert_eq!(history(&base, true).await, expected);
        baseline.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    for i in 0..6 {
        let tag = format!("trial-{i}");
        let start = Instant::now();
        let now = t + i * 3;
        let (bundle, p) = placement(&mut issuer, now, &tag);
        let proof = fresh_realization(&issuer, &bundle, &p, now + 1, &tag, false).unwrap();
        assert_eq!(proof.endpoint_id, "lab-spare");
        assert_eq!(proof.authority_effect, "none");
        // Test harness manually invokes alternative real AETHER HTTP router.
        // No production permit/queue/dispatch is created by shadow machinery.
        assert_eq!(history(&spare, true).await, expected);
        candidate.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    let (rb, rp) = placement(&mut issuer, t + 100, "revoked");
    issuer
        .revoke_envelope(
            &rb.placement_constraint.envelope_id,
            t + 101,
            "controlled test",
        )
        .unwrap();
    let revoked_blocked = fresh_realization(&issuer, &rb, &rp, t + 102, "revoked", false).is_err();
    assert!(revoked_blocked);
    let (lb, lp) = placement(&mut issuer, t + 110, "lost");
    let vanished_blocked = fresh_realization(&issuer, &lb, &lp, t + 111, "lost", true).is_err();
    assert!(vanished_blocked);
    // In-lab rollback only: a fresh request returns to the original router.
    let fallback = history(&base, true).await;
    assert_eq!(fallback, expected);
    let baseline_ms = median(&mut baseline);
    let candidate_ms = median(&mut candidate);
    assert!(
        baseline_ms > candidate_ms + 20.0,
        "modeled queue wait must be avoidable"
    );
    let head_output = std::process::Command::new("git")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(head_output.status.success());
    let source_head = String::from_utf8(head_output.stdout)
        .unwrap()
        .trim()
        .to_owned();
    let test_source_sha256 = aether_fabric::sha256_hex(
        &std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fabric_nonprod_canary.rs"),
        )
        .unwrap(),
    );
    let receipt: Value = json!({
     "source_head":source_head,"test_source_sha256":test_source_sha256,
     "kind":"AETHER_FABRIC_NONPRODUCTION_LOCAL_ROUTER_CANARY/1",
     "production_routing_unchanged":true,
     "source_operation_admitted":"test_harness_assumption_only",
     "operation":"GET /v1/history",
     "routers":2,"actual_authenticated_http_requests":15,"unauthenticated_denial_requests":1,
     "reference_queue_wait_injected_ms":BUSY_MS,
     "reference_median_ms":baseline_ms,"candidate_median_ms":candidate_ms,
     "response_bytes_equal":true,"fabric_chose_spare_local_router":true,
     "revocation_blocked_candidate":revoked_blocked,"resource_loss_blocked_candidate":vanished_blocked,
     "fresh_reference_request_after_lab_rollback":fallback==expected,
     "real_operational_benefit_demonstrated":false,
     "production_cutover_demonstrated":false,"inflight_recovery_demonstrated":false,
     "multi_host_demonstrated":false,"production_activation_authorized":false
    });
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap();
    f.write_all(serde_json::to_string_pretty(&receipt).unwrap().as_bytes())
        .unwrap();
    f.sync_all().unwrap();
    println!(
        "CANARY_NONPROD_RECEIPT={} REFERENCE_MEDIAN_MS={:.3} CANDIDATE_MEDIAN_MS={:.3}",
        path.display(),
        baseline_ms,
        candidate_ms
    );
}
