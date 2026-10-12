//! E3B #146: test-only AETHER source-owner prototype, NOT an execution issuer.
//!
//! A real C1 registry shares this harness lock with simulated source revisions
//! and a model one-attempt ledger. Production HTTP, policy and executor locks
//! do NOT share the test lock. No reference path, router or FABRIC calls here.
use aether_control_bridge::{
    AetherMechanicalAuthorityIssuer, IssuerBuildIdentity, OperationAdmissionInput, OperationClass,
    PolicyBindingEvidence,
};
use aether_fabric::{ControlState, AETHER_LOCAL_BLOCKING_POOL_ID};
use std::sync::{Arc, Mutex};

const T0: u64 = 1_797_000_000_000;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Binding {
    request: String,
    envelope: String,
    attempt: String,
    principal: String,
    token: String,
    namespace: String,
    profile: String,
    payload: String,
    policy: String,
    resource: String,
    incarnation: u64,
}

impl Binding {
    fn valid(&self) -> bool {
        self.incarnation > 0
            && [
                &self.request,
                &self.envelope,
                &self.attempt,
                &self.principal,
                &self.token,
                &self.namespace,
                &self.profile,
                &self.payload,
                &self.policy,
            ]
            .iter()
            .all(|field| !field.is_empty())
            && self.resource == AETHER_LOCAL_BLOCKING_POOL_ID
    }
}

#[derive(Clone, Debug)]
struct SimulatedSource {
    auth: u64,
    policy: Option<u64>,
    namespace: Option<u64>,
    incarnation: u64,
    available: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Queued,
    WorkerAcquired,
    Started,
    Rejected,
    Cancelled,
    Completed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Denial {
    MissingSource,
    MissingRevision,
    Identity,
    Phase,
    Changed,
    C1Invalid,
    Unavailable,
}

struct Owner {
    issuer: AetherMechanicalAuthorityIssuer,
    initial: SimulatedSource,
    current: SimulatedSource,
    binding: Binding,
    c1_revision: String,
    phase: Phase,
    starts: u64,
    closed: bool,
}

#[derive(Clone)]
struct Prototype(Arc<Mutex<Owner>>);

impl Prototype {
    fn with_owner<T>(&self, f: impl FnOnce(&mut Owner) -> T) -> Result<T, Denial> {
        let mut owner = self.0.lock().map_err(|_| Denial::Unavailable)?;
        if owner.closed {
            return Err(Denial::Unavailable);
        }
        Ok(f(&mut owner))
    }

    fn worker_acquired(&self) -> Result<(), Denial> {
        self.with_owner(|owner| {
            if owner.phase != Phase::Queued {
                return Err(Denial::Phase);
            }
            owner.phase = Phase::WorkerAcquired;
            Ok(())
        })?
    }

    /// This is ONLY a model transition. The fixture's synthetic C1 admission
    /// is not proof of real HTTP source authorization.
    fn model_start(&self, binding: &Binding, now_ms: u64) -> Result<(), Denial> {
        self.with_owner(|owner| {
            if binding != &owner.binding || !binding.valid() {
                return Err(Denial::Identity);
            }
            if owner.phase != Phase::WorkerAcquired {
                return Err(Denial::Phase);
            }
            let verdict = (|| {
                if !owner.current.available {
                    return Err(Denial::MissingSource);
                }
                if owner.current.policy.is_none() || owner.current.namespace.is_none() {
                    return Err(Denial::MissingRevision);
                }
                if owner.initial.auth != owner.current.auth
                    || owner.initial.policy != owner.current.policy
                    || owner.initial.namespace != owner.current.namespace
                    || owner.initial.incarnation != owner.current.incarnation
                    || binding.incarnation != owner.current.incarnation
                {
                    return Err(Denial::Changed);
                }
                // REAL C1 registry observation inside the model lock.
                // This is not synchronized with the live HTTP auth mutex.
                let witness = owner
                    .issuer
                    .registry()
                    .observe(&owner.binding.envelope, now_ms)
                    .map_err(|_| Denial::C1Invalid)?;
                if witness.observed_state != ControlState::Active
                    || witness.observed_revision != owner.c1_revision
                {
                    return Err(Denial::C1Invalid);
                }
                Ok(())
            })();
            if verdict.is_ok() {
                owner.phase = Phase::Started;
                owner.starts += 1;
            } else {
                owner.phase = Phase::Rejected;
            }
            verdict
        })?
    }

    fn revoke(&self) {
        self.with_owner(|owner| {
            owner
                .issuer
                .revoke_envelope(&owner.binding.envelope, T0 + 1, "test-only")
                .expect("valid C1 revocation");
        })
        .expect("model owner available");
    }

    fn cancel(&self) -> Result<(), Denial> {
        self.with_owner(|owner| {
            if !matches!(owner.phase, Phase::Queued | Phase::WorkerAcquired) {
                return Err(Denial::Phase);
            }
            owner.phase = Phase::Cancelled;
            Ok(())
        })?
    }

    fn complete(&self) -> Result<(), Denial> {
        self.with_owner(|owner| {
            if owner.phase != Phase::Started {
                return Err(Denial::Phase);
            }
            owner.phase = Phase::Completed;
            Ok(())
        })?
    }

    fn crash_without_replay_ledger(&self) {
        // No durable identity or ledger recovery exists in this prototype.
        // Deny all starts rather than reconstructing a consumed attempt.
        if let Ok(mut owner) = self.0.lock() {
            owner.closed = true;
        }
    }
}

fn fixture() -> (Prototype, Binding) {
    let profile = OperationClass::RunDocument.profile();
    let input = OperationAdmissionInput {
        request_id: "test-request".into(),
        correlation_id: "test-correlation".into(),
        operation_class: OperationClass::RunDocument,
        http_method: profile.http_method.clone(),
        http_path: profile.http_path.clone(),
        namespace_ref: "default".into(),
        principal_ref: "principal:test".into(),
        token_ref: Some("token:test".into()),
        required_scope: profile.required_scope.clone(),
        policy_binding: PolicyBindingEvidence::Bound {
            effective_policy_digest: "a".repeat(64),
        },
        typed_request_payload: serde_json::json!({"synthetic": true}),
        source_operation_admitted: true, // SYNTHETIC TEST INPUT, NEVER LIVE.
        decision_revision: "test-only".into(),
        decided_at_unix_ms: T0,
        source_authority_ref: "synthetic-test-only".into(),
    };
    let mut issuer = AetherMechanicalAuthorityIssuer::for_c2_shadow(
        "e3b-146-test-issuer",
        "test-revision",
        IssuerBuildIdentity {
            source_commit: "test".into(),
            source_tree: "test".into(),
            artifact_sha256: "7".repeat(64),
        },
        5_000,
        4,
    )
    .expect("real C1 issuer");
    let bundle = issuer.issue_control_bundle(&input).expect("test control");
    let envelope = bundle.control_state_witness.envelope_id;
    let c1_revision = issuer
        .registry()
        .observe(&envelope, T0)
        .expect("real registry witness")
        .observed_revision;
    let binding = Binding {
        request: input.request_id,
        envelope,
        attempt: "hypothetical-route-realized-id".into(),
        principal: input.principal_ref,
        token: input.token_ref.expect("test token"),
        namespace: input.namespace_ref,
        profile: profile.digest().expect("profile digest"),
        payload: "b".repeat(64),
        policy: "a".repeat(64),
        resource: AETHER_LOCAL_BLOCKING_POOL_ID.into(),
        incarnation: 19,
    };
    let simulated_source = SimulatedSource {
        auth: 2,
        policy: Some(3),
        namespace: Some(4),
        incarnation: 19,
        available: true,
    };
    let prototype = Prototype(Arc::new(Mutex::new(Owner {
        issuer,
        initial: simulated_source.clone(),
        current: simulated_source,
        binding: binding.clone(),
        c1_revision,
        phase: Phase::Queued,
        starts: 0,
        closed: false,
    })));
    (prototype, binding)
}

#[test]
fn actual_c1_revocation_at_worker_barrier_prevents_model_start() {
    let (owner, binding) = fixture();
    owner.worker_acquired().unwrap();
    let parallel = owner.clone();
    std::thread::spawn(move || parallel.revoke())
        .join()
        .unwrap();
    assert_eq!(owner.model_start(&binding, T0 + 2), Err(Denial::C1Invalid));
    assert_eq!(owner.with_owner(|s| s.starts), Ok(0));
    assert_eq!(owner.model_start(&binding, T0 + 2), Err(Denial::Phase));
}

#[test]
fn real_c1_expiration_is_rechecked_at_model_start() {
    let (owner, binding) = fixture();
    owner.worker_acquired().unwrap();
    assert_eq!(owner.model_start(&binding, T0 + 5_000), Err(Denial::C1Invalid));
    assert_eq!(owner.with_owner(|s| s.starts), Ok(0));
}

#[test]
fn model_single_use_and_post_start_revocation_preserve_completion() {
    let (owner, binding) = fixture();
    owner.worker_acquired().unwrap();
    owner.model_start(&binding, T0).unwrap();
    owner.revoke();
    assert_eq!(owner.model_start(&binding, T0 + 2), Err(Denial::Phase));
    assert_eq!(owner.cancel(), Err(Denial::Phase));
    owner.complete().unwrap();
    assert_eq!(owner.with_owner(|s| (s.phase, s.starts)), Ok((Phase::Completed, 1)));
}

#[test]
fn simulated_current_state_missing_or_changed_fails_closed() {
    for case in 0..6 {
        let (owner, binding) = fixture();
        owner.worker_acquired().unwrap();
        let mut wrong = binding.clone();
        let expected = match case {
            0 => {
                owner.with_owner(|s| s.current.policy = None).unwrap();
                Denial::MissingRevision
            }
            1 => {
                owner.with_owner(|s| s.current.namespace = None).unwrap();
                Denial::MissingRevision
            }
            2 => {
                owner.with_owner(|s| s.current.auth += 1).unwrap();
                Denial::Changed
            }
            3 => {
                owner.with_owner(|s| s.current.incarnation += 1).unwrap();
                Denial::Changed
            }
            4 => {
                owner.with_owner(|s| s.current.available = false).unwrap();
                Denial::MissingSource
            }
            _ => {
                wrong.resource = "second-resource".into();
                Denial::Identity
            }
        };
        assert_eq!(owner.model_start(&wrong, T0), Err(expected));
        assert_eq!(owner.with_owner(|s| s.starts), Ok(0));
    }
}

#[test]
fn cancellation_poison_and_restart_without_ledger_deny_all() {
    let (owner, binding) = fixture();
    owner.cancel().unwrap();
    assert_eq!(owner.model_start(&binding, T0), Err(Denial::Phase));

    let (owner, binding) = fixture();
    owner.worker_acquired().unwrap();
    owner.crash_without_replay_ledger();
    assert_eq!(owner.model_start(&binding, T0), Err(Denial::Unavailable));

    let (owner, binding) = fixture();
    owner.worker_acquired().unwrap();
    let poison = owner.clone();
    let _ = std::thread::spawn(move || {
        let _guard = poison.0.lock().unwrap();
        panic!("intentional test-only poison");
    })
    .join();
    assert_eq!(owner.model_start(&binding, T0), Err(Denial::Unavailable));
}
