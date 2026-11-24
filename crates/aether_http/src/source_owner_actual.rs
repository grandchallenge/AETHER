//! #148: real source and worker barrier, diagnostic only. No dispatch permit.
use super::*;
use aether_control_bridge::{
    unix_ms_to_rfc3339, AetherMechanicalAuthorityIssuer, IssuerBuildIdentity,
    OperationAdmissionInput, OperationClass, PolicyBindingEvidence,
};
use aether_fabric::{
    canonicalize_serializable, realize_selected_placement, sha256_hex, ControlState,
    PlacementDecision, RealizationRequest, RouteRealizedEvidence, SelectorImplementationIdentity,
    AETHER_LOCAL_BLOCKING_POOL_ID,
};
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourceGrant {
    generation: u64,
    principal: String,
    token: String,
    namespace: String,
    policy: String,
}

impl SourceGrant {
    // Read the genuine auth owner and effective policy binding; no invented epochs.
    fn read(auth: &HttpAuth, headers: &HeaderMap) -> Option<Self> {
        let ns = namespace_from_headers(headers).ok()?;
        let principal = auth.authorize(headers, AuthScope::Query, &ns).ok()?;
        let policy = bound_policy_context(&principal, None).ok()?;
        Some(Self {
            generation: principal.auth_generation,
            principal: principal.principal_id?,
            token: principal.token_id?,
            namespace: ns.to_string(),
            policy: match policy {
                Some(p) => sha256_hex(&canonicalize_serializable(&p).ok()?),
                None => sha256_hex(b"aether-public-policy/1"),
            },
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Verdict {
    MatchNonOperative,
    SourceDenied,
    SourceChanged,
    ControlInvalid,
    ClockInvalid,
    Unavailable,
    IdentityMismatch,
    AlreadyInspected,
}

struct Probe {
    source: std::sync::Weak<Mutex<HttpAuth>>,
    workers: std::sync::Weak<Semaphore>,
    grant: SourceGrant,
    issuer: Mutex<AetherMechanicalAuthorityIssuer>,
    input: OperationAdmissionInput,
    envelope: String,
    revision: String,
    route: RouteRealizedEvidence,
    issued: Instant,
    ttl: u64,
    used: std::sync::atomic::AtomicBool,
}

impl Probe {
    fn prepare(
        state: &HttpKernelState,
        headers: &HeaderMap,
        ttl: u64,
    ) -> (Arc<Self>, tokio::sync::OwnedSemaphorePermit) {
        let grant = SourceGrant::read(&state.auth.lock().unwrap(), headers).unwrap();
        let ns = namespace_from_headers(headers).unwrap();
        state.rate_limiter.admit(&ns, &grant.principal).unwrap();
        let permit = state.admit_namespace(&ns).unwrap();
        let issued = Instant::now();
        let now = now_millis();
        let profile = OperationClass::RunDocument.profile();
        let input = OperationAdmissionInput {
            request_id: format!("probe-148-{:032x}", rand::random::<u128>()),
            correlation_id: format!("probe-148-correlation-{:032x}", rand::random::<u128>()),
            operation_class: OperationClass::RunDocument,
            http_method: profile.http_method,
            http_path: profile.http_path,
            namespace_ref: grant.namespace.clone(),
            principal_ref: grant.principal.clone(),
            token_ref: Some(grant.token.clone()),
            required_scope: profile.required_scope,
            policy_binding: PolicyBindingEvidence::Bound {
                effective_policy_digest: grant.policy.clone(),
            },
            typed_request_payload: serde_json::to_value(RunDocumentRequest {
                dsl: crate::coordination_pilot_dsl("current", "goal task_ready(t)\n  keep t"),
                policy_context: None,
            })
            .unwrap(),
            // Real admission checks above, but NOT a source semantic admission.
            source_operation_admitted: true,
            decision_revision: "test-only".into(),
            decided_at_unix_ms: now,
            source_authority_ref: "test-only-no-dispatch".into(),
        };
        let mut issuer = AetherMechanicalAuthorityIssuer::for_c2_shadow(
            "source-148-test",
            "test-only",
            IssuerBuildIdentity {
                source_commit: "test-only".into(),
                source_tree: "test-only".into(),
                artifact_sha256: "7".repeat(64),
            },
            ttl,
            8,
        )
        .unwrap();
        let bundle = issuer.issue_control_bundle(&input).unwrap();
        issuer.verify_bundle(&bundle).unwrap();
        assert_eq!(bundle.admission.effective_policy_digest, grant.policy);
        let selector = SelectorImplementationIdentity {
            selector_id: "source-148-test".into(),
            source_commit: "test-only".into(),
            source_tree: "test-only".into(),
            artifact_sha256: "8".repeat(64),
        };
        let comparison = state
            .fabric_shadow_compare_placement(
                &bundle.envelope_bytes,
                &bundle.placement_constraint,
                &bundle.control_state_witness,
                now,
                &selector,
            )
            .unwrap();
        assert!(comparison.permitted_set_equal);
        let PlacementDecision::Selected(selected) = comparison.fabric_decision else {
            panic!("fixture requires one admissible local pool");
        };
        let route = realize_selected_placement(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &selected,
            &comparison.resource_snapshot,
            &bundle.control_state_witness,
            &RealizationRequest {
                realization_request_id: format!("test-realize:{}", input.request_id),
                realization_time_unix_ms: now,
                occurred_at: unix_ms_to_rfc3339(now).unwrap(),
            },
        )
        .unwrap();
        assert_eq!(route.authority_effect, "none");
        assert_eq!(route.endpoint_id, AETHER_LOCAL_BLOCKING_POOL_ID);
        let witness = bundle.control_state_witness;
        (
            Arc::new(Self {
                source: Arc::downgrade(&state.auth),
                workers: Arc::downgrade(&state.blocking.workers),
                grant,
                issuer: Mutex::new(issuer),
                input,
                envelope: witness.envelope_id,
                revision: witness.observed_revision,
                route,
                issued,
                ttl,
                used: std::sync::atomic::AtomicBool::new(false),
            }),
            permit,
        )
    }

    // Private one-use inspection. No guard crosses an await; contention denies.
    // This is a fixture C1 lock, not the live HTTP/C1 ownership migration.
    fn inspect(
        &self,
        state: &HttpKernelState,
        headers: &HeaderMap,
        route: &RouteRealizedEvidence,
    ) -> Verdict {
        if self.used.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return Verdict::AlreadyInspected;
        }
        if !std::sync::Weak::ptr_eq(&self.source, &Arc::downgrade(&state.auth))
            || !std::sync::Weak::ptr_eq(&self.workers, &Arc::downgrade(&state.blocking.workers))
            || route != &self.route
            || route.envelope_id != self.envelope
            || route.correlation_id != self.input.correlation_id
            || route.endpoint_id != AETHER_LOCAL_BLOCKING_POOL_ID
            || route.authority_effect != "none"
        {
            return Verdict::IdentityMismatch;
        }
        let Ok(auth) = state.auth.try_lock() else {
            return Verdict::Unavailable;
        };
        let Ok(issuer) = self.issuer.try_lock() else {
            return Verdict::Unavailable;
        };
        let Some(current) = SourceGrant::read(&auth, headers) else {
            return Verdict::SourceDenied;
        };
        if current != self.grant {
            return Verdict::SourceChanged;
        }
        let now = now_millis();
        if now < self.input.decided_at_unix_ms {
            return Verdict::ClockInvalid;
        }
        let Ok(witness) = issuer.registry().observe(&self.envelope, now) else {
            return Verdict::ControlInvalid;
        };
        if witness.observed_state != ControlState::Active
            || witness.observed_revision != self.revision
            || self.issued.elapsed().as_millis() >= u128::from(self.ttl)
        {
            return Verdict::ControlInvalid;
        }
        Verdict::MatchNonOperative
    }
}

struct Fixture {
    state: HttpKernelState,
    config: PilotServiceConfig,
    root: PathBuf,
    path: PathBuf,
    headers: HeaderMap,
}

impl Fixture {
    fn new() -> Self {
        use crate::{
            PilotAuthConfig, PilotConcurrencyConfig, PilotHttpTransportConfig, PilotStorageConfig,
            PilotTokenConfig,
        };
        let root = std::env::temp_dir().join(format!("aether-148-{:032x}", rand::random::<u128>()));
        fs::create_dir_all(&root).unwrap();
        let path = root.join("pilot-service.json");
        let data_root = root.join("data");
        let config = PilotServiceConfig {
            config_version: "source-148".into(),
            schema_version: "v1".into(),
            service_mode: ServiceMode::SingleNode,
            bind_addr: "127.0.0.1:3000".into(),
            http_transport: PilotHttpTransportConfig::default(),
            concurrency: PilotConcurrencyConfig {
                namespace_workers: 1,
                namespace_queue: 1,
                audit_queue: 1024,
            },
            database_path: None,
            storage: Some(PilotStorageConfig::Sqlite {
                data_root: data_root.clone(),
            }),
            audit_log_path: Some(root.join("audit.jsonl")),
            auth: PilotAuthConfig {
                tokens: vec![PilotTokenConfig {
                    principal: "operator".into(),
                    principal_id: Some("principal:operator".into()),
                    token_id: Some("token:operator".into()),
                    scopes: vec![AuthScope::Query],
                    policy_context: Some(PolicyContext {
                        capabilities: vec!["read".into()],
                        visibilities: vec!["staff".into()],
                    }),
                    token: Some("fixture-token".into()),
                    token_env: None,
                    token_file: None,
                    token_command: None,
                    namespaces: vec![NamespaceId::default()],
                    revoked: false,
                }],
                revoked_token_ids: Vec::new(),
                revoked_principal_ids: Vec::new(),
            },
        };
        fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
        let resolved = config.clone().resolve(&path).unwrap();
        let state = HttpKernelState::with_sqlite_namespaces(
            data_root,
            HttpKernelOptions::new()
                .with_auth(resolved.auth.clone())
                .with_service_status(resolved.service_status())
                .with_auth_reload_config_path(path.clone())
                .with_namespace_work_limits(1, 1),
        );
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_static("Bearer fixture-token"),
        );
        Self {
            state,
            config,
            root,
            path,
            headers,
        }
    }
    fn reload(&mut self, change: impl FnOnce(&mut PilotServiceConfig)) {
        change(&mut self.config);
        fs::write(&self.path, serde_json::to_vec(&self.config).unwrap()).unwrap();
        self.state.reload_auth_from_config().unwrap();
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[derive(Clone, Copy, Debug)]
enum Change {
    None,
    Reload,
    RevokeToken,
    Scope,
    Namespace,
    Policy,
    C1Revoke,
    C1Supersede,
    Expire,
    TamperedRoute,
}

async fn at_barrier(change: Change, queued: bool) -> Verdict {
    let mut fixture = Fixture::new();
    let state = fixture.state.clone();
    let ttl = if matches!(change, Change::Expire) {
        200
    } else {
        30_000
    };
    let (probe, ns_permit) = Probe::prepare(&state, &fixture.headers, ttl);
    let held = Arc::clone(&state.blocking.workers)
        .acquire_owned()
        .await
        .unwrap();
    let (acquired_tx, acquired_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = mpsc::sync_channel(1);
    let worker_probe = Arc::clone(&probe);
    let worker_state = state.clone();
    let headers = fixture.headers.clone();
    let mut route = probe.route.clone();
    if matches!(change, Change::TamperedRoute) {
        route.mechanical_attempt_id = "foreign-attempt".into();
    }
    let task = tokio::spawn(async move {
        let executor = worker_state.blocking.clone();
        executor
            .run(move || {
                let _ns = ns_permit;
                acquired_tx.send(()).unwrap();
                release_rx.recv_timeout(Duration::from_secs(15)).unwrap();
                Ok(worker_probe.inspect(&worker_state, &headers, &route))
            })
            .await
    });
    tokio::time::timeout(Duration::from_secs(15), async {
        while state.blocking.admitted.available_permits() != 1 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("real worker admission while worker held");
    assert_eq!(state.blocking.workers.available_permits(), 0);
    let mutate = |f: &mut Fixture| match change {
        Change::None | Change::Expire | Change::TamperedRoute => {}
        Change::Reload => f.reload(|_| {}),
        Change::RevokeToken => f.reload(|c| c.auth.tokens[0].revoked = true),
        Change::Scope => f.reload(|c| c.auth.tokens[0].scopes = vec![AuthScope::Ops]),
        Change::Namespace => {
            f.reload(|c| c.auth.tokens[0].namespaces = vec![NamespaceId::new("other").unwrap()])
        }
        Change::Policy => f.reload(|c| {
            c.auth.tokens[0].policy_context = Some(PolicyContext {
                capabilities: vec!["read".into()],
                visibilities: vec!["public".into()],
            })
        }),
        Change::C1Revoke => probe
            .issuer
            .lock()
            .unwrap()
            .revoke_envelope(&probe.envelope, now_millis(), "test revocation")
            .unwrap(),
        Change::C1Supersede => {
            let mut input = probe.input.clone();
            input.request_id.push_str("-replacement");
            input.correlation_id.push_str("-replacement");
            input.decided_at_unix_ms = now_millis();
            probe
                .issuer
                .lock()
                .unwrap()
                .issue_superseding_control_bundle(&probe.envelope, &input)
                .unwrap();
        }
    };
    if queued {
        mutate(&mut fixture);
    }
    drop(held);
    tokio::time::timeout(Duration::from_secs(15), acquired_rx)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.blocking.workers.available_permits(), 0);
    if !queued {
        mutate(&mut fixture);
    }
    if matches!(change, Change::Expire) {
        tokio::time::sleep(Duration::from_millis(ttl + 25)).await;
        assert_eq!(
            probe
                .issuer
                .lock()
                .unwrap()
                .registry()
                .observe(&probe.envelope, now_millis())
                .unwrap()
                .observed_state,
            ControlState::Expired
        );
    }
    release_tx.send(()).unwrap();
    let verdict = tokio::time::timeout(Duration::from_secs(15), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(state.blocking.workers.available_permits(), 1);
    assert_eq!(state.blocking.admitted.available_permits(), 2);
    assert_eq!(
        probe.inspect(&state, &fixture.headers, &probe.route),
        Verdict::AlreadyInspected
    );
    assert_eq!(
        state.fabric_routing_mode(),
        FabricRoutingMode::ReferenceOnly
    );
    verdict
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unchanged_source_and_c1_match_inside_real_worker() {
    assert_eq!(
        at_barrier(Change::None, false).await,
        Verdict::MatchNonOperative
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn real_reload_and_c1_transitions_at_queue_and_acquired_worker() {
    for queued in [true, false] {
        for (change, expected) in [
            (Change::Reload, Verdict::SourceChanged),
            (Change::RevokeToken, Verdict::SourceDenied),
            (Change::Scope, Verdict::SourceDenied),
            (Change::Namespace, Verdict::SourceDenied),
            (Change::Policy, Verdict::SourceChanged),
            (Change::C1Revoke, Verdict::ControlInvalid),
            (Change::C1Supersede, Verdict::ControlInvalid),
        ] {
            assert_eq!(
                at_barrier(change, queued).await,
                expected,
                "{change:?}, queued={queued}"
            );
        }
    }
}

#[test]
fn fresh_source_owner_and_lock_contention_deny_replay() {
    let fixture = Fixture::new();
    let (probe, _permit) = Probe::prepare(&fixture.state, &fixture.headers, 30_000);
    let other = Fixture::new();
    assert_eq!(
        probe.inspect(&other.state, &other.headers, &probe.route),
        Verdict::IdentityMismatch
    );
    assert_eq!(
        probe.inspect(&fixture.state, &fixture.headers, &probe.route),
        Verdict::AlreadyInspected
    );
    drop(_permit);
    let (probe, _permit) = Probe::prepare(&fixture.state, &fixture.headers, 30_000);
    let guard = fixture.state.auth.lock().unwrap();
    assert_eq!(
        probe.inspect(&fixture.state, &fixture.headers, &probe.route),
        Verdict::Unavailable
    );
    drop(guard);
    assert_eq!(
        probe.inspect(&fixture.state, &fixture.headers, &probe.route),
        Verdict::AlreadyInspected
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn expired_control_and_foreign_attempt_deny_at_real_worker_barrier() {
    assert_eq!(
        at_barrier(Change::Expire, false).await,
        Verdict::ControlInvalid
    );
    assert_eq!(
        at_barrier(Change::TamperedRoute, false).await,
        Verdict::IdentityMismatch
    );
}

#[test]
fn poisoned_c1_owner_denies_and_consumes_probe() {
    let fixture = Fixture::new();
    let (probe, _permit) = Probe::prepare(&fixture.state, &fixture.headers, 30_000);
    let poison = Arc::clone(&probe);
    assert!(std::thread::spawn(move || {
        let _lock = poison.issuer.lock().unwrap();
        panic!("deliberate fixture owner poison");
    })
    .join()
    .is_err());
    assert_eq!(
        probe.inspect(&fixture.state, &fixture.headers, &probe.route),
        Verdict::Unavailable
    );
    assert_eq!(
        probe.inspect(&fixture.state, &fixture.headers, &probe.route),
        Verdict::AlreadyInspected
    );
}
