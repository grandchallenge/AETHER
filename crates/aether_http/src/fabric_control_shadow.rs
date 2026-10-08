use crate::{fabric_equivalence::adjudicate_f1e_live_equivalence, http::HttpKernelState};
use aether_ast::PolicyContext;
use aether_control_bridge::{
    unix_ms_to_rfc3339, AetherMechanicalAuthorityIssuer, IssuerBuildIdentity,
    OperationAdmissionInput, OperationClass, PolicyBindingEvidence,
};
use aether_fabric::{
    canonicalize_serializable, realize_selected_placement, sha256_hex, PlacementDecision,
    RealizationRequest, SelectorImplementationIdentity,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};

pub const C2_SHADOW_DECISION_REVISION: &str = "aether-http-c2-shadow/1";
pub const C2_SHADOW_SOURCE_AUTHORITY: &str = "aether-http-semantic-edge";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct C2ShadowConfig {
    pub issuer_ref: String,
    pub issuer_revision: String,
    pub issuer_build: IssuerBuildIdentity,
    pub selector: SelectorImplementationIdentity,
    pub registry_capacity: usize,
    pub evidence_capacity: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct C2ShadowRequestMaterial {
    pub typed_request_payload: Option<Value>,
    pub requested_policy_context: Option<PolicyContext>,
    pub payload_error: Option<String>,
}

impl C2ShadowRequestMaterial {
    pub(crate) fn empty(requested_policy_context: Option<PolicyContext>) -> Self {
        Self {
            typed_request_payload: Some(serde_json::json!({})),
            requested_policy_context,
            payload_error: None,
        }
    }

    pub(crate) fn from_serializable<T: Serialize>(
        value: &T,
        requested_policy_context: Option<PolicyContext>,
    ) -> Self {
        match serde_json::to_value(value) {
            Ok(value) => Self {
                typed_request_payload: Some(value),
                requested_policy_context,
                payload_error: None,
            },
            Err(error) => Self {
                typed_request_payload: None,
                requested_policy_context,
                payload_error: Some(error.to_string()),
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum C2ShadowDisposition {
    CandidateValidated,
    CandidateUnavailable,
    ShadowRejected,
    ShadowFailed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct C2ShadowEvidence {
    pub request_id: String,
    pub method: String,
    pub path: String,
    pub namespace_ref: String,
    pub principal_ref: String,
    pub operation_class: Option<OperationClass>,
    pub operation_admission_id: Option<String>,
    pub envelope_id: Option<String>,
    pub placement_decision_id: Option<String>,
    pub candidate_mechanical_attempt_id: Option<String>,
    pub permitted_set_equal: Option<bool>,
    pub decision_resource_snapshot_digest: Option<String>,
    pub fresh_control_witness_revision: Option<String>,
    pub fresh_resource_snapshot_digest: Option<String>,
    pub reference_permitted_resource_ids: Vec<String>,
    pub fabric_selected_resource_id: Option<String>,
    pub differential_equivalent: Option<bool>,
    pub disposition: C2ShadowDisposition,
    pub detail: Option<String>,
    pub authority_effect: String,
    pub reference_path_authoritative: bool,
    #[serde(default)]
    pub operation_profile_digest: Option<String>,
    #[serde(default)]
    pub issuer_build_digest: Option<String>,
    #[serde(default)]
    pub selector_build_digest: Option<String>,
    #[serde(default)]
    pub effective_policy_digest: Option<String>,
    #[serde(default)]
    pub request_payload_digest: Option<String>,
    #[serde(default)]
    pub reference_result_status: Option<u16>,
    #[serde(default)]
    pub reference_result_digest: Option<String>,
}

pub(crate) struct C2ShadowController {
    issuer: Mutex<Option<AetherMechanicalAuthorityIssuer>>,
    initialization_error: Option<String>,
    selector: SelectorImplementationIdentity,
    evidence: Mutex<VecDeque<C2ShadowEvidence>>,
    evidence_capacity: usize,
    evicted_observations: AtomicU64,
    issuer_build_digest: String,
    selector_build_digest: String,
}

impl C2ShadowController {
    pub(crate) fn new(config: C2ShadowConfig, operation_timeout_ms: u64) -> Self {
        let issuer_build_digest = config.issuer_build.digest().unwrap_or_default();
        let selector_build_digest = aether_fabric::canonicalize_serializable(&config.selector)
            .map(|bytes| sha256_hex(&bytes))
            .unwrap_or_default();
        let issuer = AetherMechanicalAuthorityIssuer::for_c2_shadow(
            config.issuer_ref,
            config.issuer_revision,
            config.issuer_build,
            operation_timeout_ms,
            config.registry_capacity,
        );
        let (issuer, initialization_error) = match issuer {
            Ok(issuer) => (Some(issuer), None),
            Err(error) => (None, Some(error.to_string())),
        };
        Self {
            issuer: Mutex::new(issuer),
            initialization_error,
            selector: config.selector,
            evidence: Mutex::new(VecDeque::with_capacity(config.evidence_capacity.max(1))),
            evidence_capacity: config.evidence_capacity.max(1),
            evicted_observations: AtomicU64::new(0),
            issuer_build_digest,
            selector_build_digest,
        }
    }

    fn retain(&self, evidence: C2ShadowEvidence) {
        let Ok(mut retained) = self.evidence.lock() else {
            return;
        };
        while retained.len() >= self.evidence_capacity {
            retained.pop_front();
            self.evicted_observations.fetch_add(1, Ordering::Relaxed);
        }
        retained.push_back(evidence);
    }

    pub(crate) fn evicted_observations(&self) -> u64 {
        self.evicted_observations.load(Ordering::Relaxed)
    }

    /// Pair the authoritative completion with the exact shadow request identity.
    /// A missing/evicted shadow entry must never be silently treated as coverage.
    pub(crate) fn record_reference_result(
        &self,
        request_id: &str,
        status: u16,
        result_digest: Option<String>,
    ) {
        if let Ok(mut retained) = self.evidence.lock() {
            if let Some(entry) = retained
                .iter_mut()
                .rev()
                .find(|item| item.request_id == request_id)
            {
                entry.reference_result_status = Some(status);
                entry.reference_result_digest = result_digest;
            }
        }
    }

    pub(crate) fn snapshot(&self) -> Vec<C2ShadowEvidence> {
        self.evidence
            .lock()
            .map(|items| items.iter().cloned().collect())
            .unwrap_or_default()
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn observe_request(
        &self,
        state: &HttpKernelState,
        request_id: &str,
        method: &str,
        path: &str,
        namespace_ref: &str,
        principal_ref: &str,
        token_ref: Option<&str>,
        required_scope: &str,
        effective_policy: Option<&PolicyContext>,
        material: C2ShadowRequestMaterial,
        decided_at_unix_ms: u64,
    ) {
        let mut base = C2ShadowEvidence {
            request_id: request_id.to_owned(),
            method: method.to_owned(),
            path: path.to_owned(),
            namespace_ref: namespace_ref.to_owned(),
            principal_ref: principal_ref.to_owned(),
            operation_class: None,
            operation_admission_id: None,
            envelope_id: None,
            placement_decision_id: None,
            candidate_mechanical_attempt_id: None,
            permitted_set_equal: None,
            decision_resource_snapshot_digest: None,
            fresh_control_witness_revision: None,
            fresh_resource_snapshot_digest: None,
            reference_permitted_resource_ids: Vec::new(),
            fabric_selected_resource_id: None,
            differential_equivalent: None,
            disposition: C2ShadowDisposition::ShadowFailed,
            detail: None,
            authority_effect: "none".into(),
            reference_path_authoritative: true,
            operation_profile_digest: None,
            issuer_build_digest: Some(self.issuer_build_digest.clone()),
            selector_build_digest: Some(self.selector_build_digest.clone()),
            effective_policy_digest: None,
            request_payload_digest: None,
            reference_result_status: None,
            reference_result_digest: None,
        };

        if let Some(error) = material.payload_error {
            base.detail = Some(format!(
                "typed request payload serialization failed: {error}"
            ));
            self.retain(base);
            return;
        }
        let Some(typed_request_payload) = material.typed_request_payload else {
            base.detail = Some("typed request payload is unavailable".into());
            self.retain(base);
            return;
        };

        let operation_class = match OperationClass::from_http(method, path) {
            Ok(operation_class) => operation_class,
            Err(error) => {
                base.disposition = C2ShadowDisposition::ShadowRejected;
                base.detail = Some(error.to_string());
                self.retain(base);
                return;
            }
        };
        base.operation_class = Some(operation_class);
        base.operation_profile_digest = operation_class.profile().digest().ok();

        let profile = operation_class.profile();
        let policy_binding = if profile.policy_binding_required {
            match effective_policy {
                Some(policy) => match canonicalize_serializable(policy) {
                    Ok(bytes) => PolicyBindingEvidence::Bound {
                        effective_policy_digest: sha256_hex(&bytes),
                    },
                    Err(error) => {
                        base.detail =
                            Some(format!("effective policy canonicalization failed: {error}"));
                        self.retain(base);
                        return;
                    }
                },
                None => PolicyBindingEvidence::Public,
            }
        } else {
            PolicyBindingEvidence::Public
        };

        base.effective_policy_digest = match &policy_binding {
            PolicyBindingEvidence::Bound {
                effective_policy_digest,
            } => Some(effective_policy_digest.clone()),
            PolicyBindingEvidence::Public => Some(sha256_hex(b"aether-public-policy/1")),
            PolicyBindingEvidence::DeniedEscalation => None,
        };
        base.request_payload_digest = canonicalize_serializable(&typed_request_payload)
            .map(|bytes| sha256_hex(&bytes))
            .ok();

        let input = OperationAdmissionInput {
            request_id: request_id.to_owned(),
            correlation_id: request_id.to_owned(),
            operation_class,
            http_method: method.to_owned(),
            http_path: path.to_owned(),
            namespace_ref: namespace_ref.to_owned(),
            principal_ref: principal_ref.to_owned(),
            token_ref: token_ref.map(str::to_owned),
            required_scope: required_scope.to_owned(),
            policy_binding,
            typed_request_payload,
            source_operation_admitted: true,
            decision_revision: C2_SHADOW_DECISION_REVISION.into(),
            decided_at_unix_ms,
            source_authority_ref: C2_SHADOW_SOURCE_AUTHORITY.into(),
        };

        if let Some(error) = &self.initialization_error {
            base.detail = Some(format!("C2 shadow issuer initialization failed: {error}"));
            self.retain(base);
            return;
        }

        let Ok(mut issuer_guard) = self.issuer.lock() else {
            base.detail = Some("C2 shadow issuer lock is poisoned".into());
            self.retain(base);
            return;
        };
        let Some(issuer) = issuer_guard.as_mut() else {
            base.detail = Some("C2 shadow issuer is unavailable".into());
            self.retain(base);
            return;
        };

        let bundle = match issuer.issue_control_bundle(&input) {
            Ok(bundle) => bundle,
            Err(error) => {
                base.disposition = C2ShadowDisposition::ShadowRejected;
                base.detail = Some(error.to_string());
                self.retain(base);
                return;
            }
        };
        if let Err(error) = issuer.verify_bundle(&bundle) {
            base.detail = Some(format!("issued bundle verification failed: {error}"));
            self.retain(base);
            return;
        }
        base.operation_admission_id = Some(bundle.admission.operation_admission_id.clone());
        base.envelope_id = Some(bundle.control_state_witness.envelope_id.clone());

        let comparison = match state.fabric_shadow_compare_placement(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &bundle.control_state_witness,
            decided_at_unix_ms,
            &self.selector,
        ) {
            Ok(comparison) => comparison,
            Err(error) => {
                base.detail = Some(format!("selector shadow validation failed: {error}"));
                self.retain(base);
                return;
            }
        };
        base.permitted_set_equal = Some(comparison.permitted_set_equal);
        base.decision_resource_snapshot_digest =
            Some(comparison.resource_snapshot.snapshot_digest.clone());
        base.reference_permitted_resource_ids = comparison.reference_permitted_resource_ids.clone();
        base.fabric_selected_resource_id = comparison.fabric_selected_resource_id.clone();
        match adjudicate_f1e_live_equivalence(&comparison) {
            Ok(_) => base.differential_equivalent = Some(true),
            Err(error) => {
                base.differential_equivalent = Some(false);
                base.detail = Some(format!("C3 differential adjudication failed: {error}"));
                self.retain(base);
                return;
            }
        }

        let selected = match comparison.fabric_decision {
            PlacementDecision::Selected(selected) => selected,
            PlacementDecision::Unavailable(_) => {
                base.disposition = C2ShadowDisposition::CandidateUnavailable;
                self.retain(base);
                return;
            }
        };
        base.placement_decision_id = Some(selected.placement_decision_id.clone());

        let realization_time_unix_ms = decided_at_unix_ms.saturating_add(1);
        let current_witness = match issuer.registry().observe(
            &bundle.control_state_witness.envelope_id,
            realization_time_unix_ms,
        ) {
            Ok(witness) => witness,
            Err(error) => {
                base.detail = Some(format!("fresh control observation failed: {error}"));
                self.retain(base);
                return;
            }
        };
        base.fresh_control_witness_revision = Some(current_witness.observed_revision.clone());
        let current_snapshot = match state.fabric_reference_pool_snapshot(realization_time_unix_ms)
        {
            Ok(snapshot) => snapshot,
            Err(error) => {
                base.detail = Some(format!("fresh resource snapshot failed: {error}"));
                self.retain(base);
                return;
            }
        };
        base.fresh_resource_snapshot_digest = Some(current_snapshot.snapshot_digest.clone());
        let occurred_at = match unix_ms_to_rfc3339(realization_time_unix_ms) {
            Ok(value) => value,
            Err(error) => {
                base.detail = Some(format!("realization timestamp failed: {error}"));
                self.retain(base);
                return;
            }
        };
        let realization_request = RealizationRequest {
            realization_request_id: format!("c2-shadow-realize:{request_id}"),
            realization_time_unix_ms,
            occurred_at,
        };
        let route = match realize_selected_placement(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &selected,
            &current_snapshot,
            &current_witness,
            &realization_request,
        ) {
            Ok(route) => route,
            Err(error) => {
                base.detail = Some(format!("pure realization validation failed: {error}"));
                self.retain(base);
                return;
            }
        };
        if route.authority_effect != "none" {
            base.detail = Some("candidate RouteRealized evidence acquired authority effect".into());
            self.retain(base);
            return;
        }
        base.candidate_mechanical_attempt_id = Some(route.mechanical_attempt_id);
        base.disposition = C2ShadowDisposition::CandidateValidated;
        self.retain(base);
    }
}
