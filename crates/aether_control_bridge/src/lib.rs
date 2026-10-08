//! AETHER-owned off-path mechanical-authority bridge.
//!
//! C1 implements the protected C0 authority-issuance contract without wiring the
//! issuer into ordinary HTTP execution.  The crate deliberately has no public
//! issuer constructor in C1: record serialization is not authority, and a later
//! C2 integration tranche must add its own reviewed AETHER-owned factory.
//!
//! ```compile_fail
//! use aether_control_bridge::AetherMechanicalAuthorityIssuer;
//! let _issuer: AetherMechanicalAuthorityIssuer = serde_json::from_str("{}").unwrap();
//! ```

use aether_fabric::{
    canonicalize_serializable, parse_json_no_duplicates, sha256_hex,
    validate_control_state_witness, validate_placement_constraint_narrowing, ControlState,
    ControlStateWitness, FabricContractError, PlacementConstraintSet, ResourceRequirements,
    AETHER_BLOCKING_CAPABILITY, AETHER_LOCAL_BLOCKING_POOL_CLASS, AETHER_LOCAL_TRUST_ZONE,
    CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT, E1_PROTOCOL_FAMILY, E1_PROTOCOL_MAJOR,
    E1_PROTOCOL_MINOR, E3_PROTOCOL_MAJOR, E3_PROTOCOL_MINOR,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest as ShaDigest, Sha256};
use std::collections::BTreeMap;
use thiserror::Error;

pub const C1_OPERATION_MANIFEST_VERSION: &str = "aether-operation-manifest/1";
pub const C1_OPERATION_ADMISSION_VERSION: &str = "aether-operation-admission/1";
pub const C1_MECHANICAL_AUTHORIZATION_VERSION: &str = "aether-mechanical-authorization/1";
pub const C1_LOCAL_MECHANICAL_PROFILE: &str = "aether-http-local-blocking/1";
pub const C1_FAIRNESS_PROFILE: &str = "aether-bounded-admission/1";
pub const C1_INTEGRITY_PROFILE: &str = "aether-control-bridge-inproc/1";
pub const C1_MAX_SNAPSHOT_AGE_MS: u64 = 1_000;
pub const C1_LIVE_ROUTING_AVAILABLE: bool = false;

pub fn c1_live_routing_available() -> bool {
    C1_LIVE_ROUTING_AVAILABLE
}

const LOCALITY_LOCAL_PROCESS: &str = "local-process";
const ACTION_QUEUE_PRE_START: &str = "queue_pre_start";
const PRIORITY_NORMAL: &str = "normal";
const RETRY_NONE: &str = "none";
const PUBLIC_POLICY_MARKER: &[u8] = b"aether-public-policy/1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationClass {
    History,
    HistoryPage,
    AppendDryRun,
    AppendReceipts,
    SchemaCatalog,
    CurrentState,
    AsOf,
    ParseDocument,
    RunDocument,
    RunDocumentPage,
    CoordinationPilotReport,
    CoordinationDeltaReport,
    PartitionStatus,
    PartitionHistory,
    PartitionState,
    FederatedHistory,
    FederatedRunDocument,
    FederatedReport,
    ExplainTuple,
    ResolveTraceHandle,
    ResolveTraceHandlePage,
    GetArtifactReference,
    SearchVectors,
}

impl OperationClass {
    pub const ALL_FIRST_LANE: [Self; 23] = [
        Self::History,
        Self::HistoryPage,
        Self::AppendDryRun,
        Self::AppendReceipts,
        Self::SchemaCatalog,
        Self::CurrentState,
        Self::AsOf,
        Self::ParseDocument,
        Self::RunDocument,
        Self::RunDocumentPage,
        Self::CoordinationPilotReport,
        Self::CoordinationDeltaReport,
        Self::PartitionStatus,
        Self::PartitionHistory,
        Self::PartitionState,
        Self::FederatedHistory,
        Self::FederatedRunDocument,
        Self::FederatedReport,
        Self::ExplainTuple,
        Self::ResolveTraceHandle,
        Self::ResolveTraceHandlePage,
        Self::GetArtifactReference,
        Self::SearchVectors,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::History => "history",
            Self::HistoryPage => "history_page",
            Self::AppendDryRun => "append_dry_run",
            Self::AppendReceipts => "append_receipts",
            Self::SchemaCatalog => "schema_catalog",
            Self::CurrentState => "current_state",
            Self::AsOf => "as_of",
            Self::ParseDocument => "parse_document",
            Self::RunDocument => "run_document",
            Self::RunDocumentPage => "run_document_page",
            Self::CoordinationPilotReport => "coordination_pilot_report",
            Self::CoordinationDeltaReport => "coordination_delta_report",
            Self::PartitionStatus => "partition_status",
            Self::PartitionHistory => "partition_history",
            Self::PartitionState => "partition_state",
            Self::FederatedHistory => "federated_history",
            Self::FederatedRunDocument => "federated_run_document",
            Self::FederatedReport => "federated_report",
            Self::ExplainTuple => "explain_tuple",
            Self::ResolveTraceHandle => "resolve_trace_handle",
            Self::ResolveTraceHandlePage => "resolve_trace_handle_page",
            Self::GetArtifactReference => "get_artifact_reference",
            Self::SearchVectors => "search_vectors",
        }
    }

    pub fn from_http(method: &str, path: &str) -> Result<Self, AuthorityIssuanceError> {
        let operation = match (method, path) {
            ("GET", "/v1/history") => Self::History,
            ("GET", "/v1/history/page") => Self::HistoryPage,
            ("POST", "/v1/append/dry-run") => Self::AppendDryRun,
            ("GET", "/v1/append/receipts") => Self::AppendReceipts,
            ("GET", "/v1/schema") => Self::SchemaCatalog,
            ("POST", "/v1/state/current") => Self::CurrentState,
            ("POST", "/v1/state/as-of") => Self::AsOf,
            ("POST", "/v1/documents/parse") => Self::ParseDocument,
            ("POST", "/v1/documents/run") => Self::RunDocument,
            ("POST", "/v1/documents/run/page") => Self::RunDocumentPage,
            ("POST", "/v1/reports/pilot/coordination") => Self::CoordinationPilotReport,
            ("POST", "/v1/reports/pilot/coordination-delta") => Self::CoordinationDeltaReport,
            ("GET", "/v1/partitions/status") => Self::PartitionStatus,
            ("POST", "/v1/partitions/history") => Self::PartitionHistory,
            ("POST", "/v1/partitions/state") => Self::PartitionState,
            ("POST", "/v1/federated/history") => Self::FederatedHistory,
            ("POST", "/v1/federated/run") => Self::FederatedRunDocument,
            ("POST", "/v1/federated/report") => Self::FederatedReport,
            ("POST", "/v1/explain/tuple") => Self::ExplainTuple,
            ("POST", "/v1/explanations/resolve") => Self::ResolveTraceHandle,
            ("POST", "/v1/explanations/resolve/page") => Self::ResolveTraceHandlePage,
            ("POST", "/v1/sidecars/artifacts/get") => Self::GetArtifactReference,
            ("POST", "/v1/sidecars/vectors/search") => Self::SearchVectors,
            ("POST", "/v1/append")
            | ("POST", "/v1/schema/register")
            | ("POST", "/v1/schema/activate")
            | ("POST", "/v1/partitions/promote")
            | ("POST", "/v1/partitions/append")
            | ("POST", "/v1/sidecars/artifacts/register")
            | ("POST", "/v1/sidecars/vectors/register") => {
                return Err(AuthorityIssuanceError::ExcludedMutation {
                    method: method.to_owned(),
                    path: path.to_owned(),
                });
            }
            _ => {
                return Err(AuthorityIssuanceError::UnknownOperation {
                    method: method.to_owned(),
                    path: path.to_owned(),
                });
            }
        };
        Ok(operation)
    }

    pub fn profile(self) -> OperationProfile {
        let (method, path, scope, policy, evidence) = match self {
            Self::History => ("GET", "/v1/history", "ops", true, false),
            Self::HistoryPage => ("GET", "/v1/history/page", "ops", true, false),
            Self::AppendDryRun => ("POST", "/v1/append/dry-run", "append", false, false),
            Self::AppendReceipts => ("GET", "/v1/append/receipts", "ops", false, false),
            Self::SchemaCatalog => ("GET", "/v1/schema", "query", false, false),
            Self::CurrentState => ("POST", "/v1/state/current", "query", true, false),
            Self::AsOf => ("POST", "/v1/state/as-of", "query", true, false),
            Self::ParseDocument => ("POST", "/v1/documents/parse", "query", false, false),
            Self::RunDocument => ("POST", "/v1/documents/run", "query", true, true),
            Self::RunDocumentPage => ("POST", "/v1/documents/run/page", "query", true, true),
            Self::CoordinationPilotReport => (
                "POST",
                "/v1/reports/pilot/coordination",
                "query",
                true,
                true,
            ),
            Self::CoordinationDeltaReport => (
                "POST",
                "/v1/reports/pilot/coordination-delta",
                "query",
                true,
                true,
            ),
            Self::PartitionStatus => ("GET", "/v1/partitions/status", "ops", false, false),
            Self::PartitionHistory => ("POST", "/v1/partitions/history", "query", true, false),
            Self::PartitionState => ("POST", "/v1/partitions/state", "query", true, false),
            Self::FederatedHistory => ("POST", "/v1/federated/history", "query", true, false),
            Self::FederatedRunDocument => ("POST", "/v1/federated/run", "query", true, true),
            Self::FederatedReport => ("POST", "/v1/federated/report", "explain", true, true),
            Self::ExplainTuple => ("POST", "/v1/explain/tuple", "explain", true, false),
            Self::ResolveTraceHandle => {
                ("POST", "/v1/explanations/resolve", "explain", true, false)
            }
            Self::ResolveTraceHandlePage => (
                "POST",
                "/v1/explanations/resolve/page",
                "explain",
                true,
                false,
            ),
            Self::GetArtifactReference => {
                ("POST", "/v1/sidecars/artifacts/get", "query", true, false)
            }
            Self::SearchVectors => ("POST", "/v1/sidecars/vectors/search", "query", true, false),
        };
        OperationProfile {
            profile_ref: format!("aether-http-op/{}/1", self.as_str()),
            operation_class: self,
            http_method: method.into(),
            http_path: path.into(),
            required_scope: scope.into(),
            policy_binding_required: policy,
            aether_execution_evidence_persistence: evidence,
            mechanical_profile_ref: C1_LOCAL_MECHANICAL_PROFILE.into(),
            permitted_actions: vec![ACTION_QUEUE_PRE_START.into()],
            eligible_resource_classes: vec![AETHER_LOCAL_BLOCKING_POOL_CLASS.into()],
            trust_zones: vec![AETHER_LOCAL_TRUST_ZONE.into()],
            locality_constraints: vec![LOCALITY_LOCAL_PROCESS.into()],
            required_capabilities: vec![AETHER_BLOCKING_CAPABILITY.into()],
            capacity_unit: CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT.into(),
            minimum_capacity_units: 1,
            priority_class: PRIORITY_NORMAL.into(),
            max_attempts: 1,
            backoff_class: RETRY_NONE.into(),
            max_parallel_copies: 1,
            max_snapshot_age_ms: C1_MAX_SNAPSHOT_AGE_MS,
            integrity_profile_ref: C1_INTEGRITY_PROFILE.into(),
            fairness_policy_ref: C1_FAIRNESS_PROFILE.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperationProfile {
    pub profile_ref: String,
    pub operation_class: OperationClass,
    pub http_method: String,
    pub http_path: String,
    pub required_scope: String,
    pub policy_binding_required: bool,
    pub aether_execution_evidence_persistence: bool,
    pub mechanical_profile_ref: String,
    pub permitted_actions: Vec<String>,
    pub eligible_resource_classes: Vec<String>,
    pub trust_zones: Vec<String>,
    pub locality_constraints: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub capacity_unit: String,
    pub minimum_capacity_units: u64,
    pub priority_class: String,
    pub max_attempts: u64,
    pub backoff_class: String,
    pub max_parallel_copies: u64,
    pub max_snapshot_age_ms: u64,
    pub integrity_profile_ref: String,
    pub fairness_policy_ref: String,
}

impl OperationProfile {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, AuthorityIssuanceError> {
        Ok(canonicalize_serializable(self)?)
    }

    pub fn digest(&self) -> Result<String, AuthorityIssuanceError> {
        Ok(sha256_hex(&self.canonical_bytes()?))
    }

    fn validate_binding(
        &self,
        input: &OperationAdmissionInput,
    ) -> Result<(), AuthorityIssuanceError> {
        if self.operation_class != input.operation_class
            || self.http_method != input.http_method
            || self.http_path != input.http_path
            || self.required_scope != input.required_scope
        {
            return Err(AuthorityIssuanceError::OperationProfileMismatch);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MechanicalProfile {
    pub profile_ref: String,
    pub permitted_actions: Vec<String>,
    pub eligible_resource_classes: Vec<String>,
    pub trust_zones: Vec<String>,
    pub locality_constraints: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub capacity_unit: String,
    pub minimum_capacity_units: u64,
    pub priority_class: String,
    pub max_attempts: u64,
    pub backoff_class: String,
    pub max_parallel_copies: u64,
    pub max_snapshot_age_ms: u64,
    pub integrity_profile_ref: String,
    pub fairness_policy_ref: String,
}

impl MechanicalProfile {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, AuthorityIssuanceError> {
        Ok(canonicalize_serializable(self)?)
    }

    pub fn digest(&self) -> Result<String, AuthorityIssuanceError> {
        Ok(sha256_hex(&self.canonical_bytes()?))
    }

    fn validate_operation_profile(
        &self,
        operation: &OperationProfile,
    ) -> Result<(), AuthorityIssuanceError> {
        if self.profile_ref != operation.mechanical_profile_ref
            || self.permitted_actions != operation.permitted_actions
            || self.eligible_resource_classes != operation.eligible_resource_classes
            || self.trust_zones != operation.trust_zones
            || self.locality_constraints != operation.locality_constraints
            || self.required_capabilities != operation.required_capabilities
            || self.capacity_unit != operation.capacity_unit
            || self.minimum_capacity_units != operation.minimum_capacity_units
            || self.priority_class != operation.priority_class
            || self.max_attempts != operation.max_attempts
            || self.backoff_class != operation.backoff_class
            || self.max_parallel_copies != operation.max_parallel_copies
            || self.max_snapshot_age_ms != operation.max_snapshot_age_ms
            || self.integrity_profile_ref != operation.integrity_profile_ref
            || self.fairness_policy_ref != operation.fairness_policy_ref
        {
            return Err(AuthorityIssuanceError::OperationProfileMismatch);
        }
        Ok(())
    }
}

pub fn protected_local_mechanical_profile() -> MechanicalProfile {
    MechanicalProfile {
        profile_ref: C1_LOCAL_MECHANICAL_PROFILE.into(),
        permitted_actions: vec![ACTION_QUEUE_PRE_START.into()],
        eligible_resource_classes: vec![AETHER_LOCAL_BLOCKING_POOL_CLASS.into()],
        trust_zones: vec![AETHER_LOCAL_TRUST_ZONE.into()],
        locality_constraints: vec![LOCALITY_LOCAL_PROCESS.into()],
        required_capabilities: vec![AETHER_BLOCKING_CAPABILITY.into()],
        capacity_unit: CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT.into(),
        minimum_capacity_units: 1,
        priority_class: PRIORITY_NORMAL.into(),
        max_attempts: 1,
        backoff_class: RETRY_NONE.into(),
        max_parallel_copies: 1,
        max_snapshot_age_ms: C1_MAX_SNAPSHOT_AGE_MS,
        integrity_profile_ref: C1_INTEGRITY_PROFILE.into(),
        fairness_policy_ref: C1_FAIRNESS_PROFILE.into(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum PolicyBindingEvidence {
    Public,
    Bound { effective_policy_digest: String },
    DeniedEscalation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationAdmissionInput {
    pub request_id: String,
    pub correlation_id: String,
    pub operation_class: OperationClass,
    pub http_method: String,
    pub http_path: String,
    pub namespace_ref: String,
    pub principal_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_ref: Option<String>,
    pub required_scope: String,
    pub policy_binding: PolicyBindingEvidence,
    pub typed_request_payload: Value,
    pub source_operation_admitted: bool,
    pub decision_revision: String,
    pub decided_at_unix_ms: u64,
    pub source_authority_ref: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionDisposition {
    Admitted,
    Rejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AetherOperationAdmissionDecision {
    pub schema_version: String,
    pub record_type: String,
    pub operation_admission_id: String,
    pub request_id: String,
    pub correlation_id: String,
    pub operation_class: OperationClass,
    pub namespace_ref: String,
    pub principal_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_ref: Option<String>,
    pub required_scope: String,
    pub effective_policy_digest: String,
    pub operation_manifest_ref: String,
    pub operation_manifest_digest: String,
    pub operation_profile_ref: String,
    pub operation_profile_digest: String,
    pub decision: AdmissionDisposition,
    pub decision_revision: String,
    pub decided_at_unix_ms: u64,
    pub source_authority_ref: String,
    pub authority_effect: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IssuerBuildIdentity {
    pub source_commit: String,
    pub source_tree: String,
    pub artifact_sha256: String,
}

impl IssuerBuildIdentity {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, AuthorityIssuanceError> {
        require_sha256(&self.artifact_sha256, "issuer artifact")?;
        if self.source_commit.is_empty() || self.source_tree.is_empty() {
            return Err(AuthorityIssuanceError::InvalidInput(
                "issuer source_commit/source_tree must be explicit".into(),
            ));
        }
        Ok(canonicalize_serializable(self)?)
    }

    pub fn digest(&self) -> Result<String, AuthorityIssuanceError> {
        Ok(sha256_hex(&self.canonical_bytes()?))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MechanicalAuthorizationDecision {
    pub schema_version: String,
    pub record_type: String,
    pub mechanical_authorization_id: String,
    pub operation_admission_ref: String,
    pub operation_admission_digest: String,
    pub correlation_id: String,
    pub scope_manifest_ref: String,
    pub scope_manifest_digest: String,
    pub payload_manifest_ref: String,
    pub payload_manifest_digest: String,
    pub mechanical_profile_ref: String,
    pub mechanical_profile_digest: String,
    pub expires_at: String,
    pub expires_at_unix_ms: u64,
    pub issuer_ref: String,
    pub issuer_revision: String,
    pub issuer_build_digest: String,
    pub decided_at_unix_ms: u64,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeControlState {
    Active,
    Revoked,
    Superseded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuedMechanicalControlBundle {
    pub admission: AetherOperationAdmissionDecision,
    pub authorization: MechanicalAuthorizationDecision,
    pub operation_manifest_bytes: Vec<u8>,
    pub payload_manifest_bytes: Vec<u8>,
    pub operation_profile_bytes: Vec<u8>,
    pub mechanical_profile_bytes: Vec<u8>,
    pub envelope_bytes: Vec<u8>,
    pub placement_constraint: PlacementConstraintSet,
    pub control_state_witness: ControlStateWitness,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorityTransport {
    SameProcess,
    CrossProcess,
}

#[derive(Debug, Error)]
pub enum AuthorityIssuanceError {
    #[error(transparent)]
    Fabric(#[from] FabricContractError),
    #[error("excluded authoritative mutation endpoint: {method} {path}")]
    ExcludedMutation { method: String, path: String },
    #[error("unknown first-lane operation: {method} {path}")]
    UnknownOperation { method: String, path: String },
    #[error("operation profile does not match the exact protected method/path/scope")]
    OperationProfileMismatch,
    #[error("operation admission rejected: {0}")]
    AdmissionRejected(String),
    #[error("invalid authority-issuance input: {0}")]
    InvalidInput(String),
    #[error("authority record integrity conflict: {0}")]
    IntegrityConflict(String),
    #[error("envelope control transition is invalid: {0}")]
    InvalidControlTransition(String),
    #[error("envelope registry is full")]
    RegistryFull,
    #[error("unknown envelope: {0}")]
    UnknownEnvelope(String),
    #[error("cross-process/network authority transport is not authorized by C1/C2")]
    CrossProcessUnauthorized,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct OperationManifest {
    manifest_version: String,
    operation_class: OperationClass,
    namespace_ref: String,
    request_input_ref: String,
    effective_policy_digest: String,
    typed_request_payload_digest: String,
    operation_profile_ref: String,
    operation_profile_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct AdmissionIdentityMaterial {
    schema_version: String,
    record_type: String,
    request_id: String,
    correlation_id: String,
    operation_class: OperationClass,
    namespace_ref: String,
    principal_ref: String,
    token_ref: Option<String>,
    required_scope: String,
    effective_policy_digest: String,
    operation_manifest_ref: String,
    operation_manifest_digest: String,
    operation_profile_ref: String,
    operation_profile_digest: String,
    decision: AdmissionDisposition,
    decision_revision: String,
    decided_at_unix_ms: u64,
    source_authority_ref: String,
    authority_effect: String,
    rejection_reason: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct MechanicalIdentityMaterial {
    schema_version: String,
    record_type: String,
    operation_admission_ref: String,
    operation_admission_digest: String,
    correlation_id: String,
    scope_manifest_ref: String,
    scope_manifest_digest: String,
    payload_manifest_ref: String,
    payload_manifest_digest: String,
    mechanical_profile_ref: String,
    mechanical_profile_digest: String,
    expires_at: String,
    expires_at_unix_ms: u64,
    issuer_ref: String,
    issuer_revision: String,
    issuer_build_digest: String,
    decided_at_unix_ms: u64,
    authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct Sha256Ref {
    algorithm: String,
    value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct RetryPolicy {
    max_attempts: u64,
    backoff_class: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct RedundancyPolicy {
    max_parallel_copies: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct MechanicalEnvelope {
    schema_version: String,
    protocol_family: String,
    protocol_major: u32,
    protocol_minor: u32,
    record_type: String,
    domain: String,
    authority_owner: String,
    authority_effect: String,
    envelope_id: String,
    correlation_id: String,
    authorization_ref: String,
    issuer_ref: String,
    scope_ref: String,
    scope_digest: Sha256Ref,
    payload_ref: String,
    permitted_actions: Vec<String>,
    eligible_resource_classes: Vec<String>,
    trust_zones: Vec<String>,
    locality_constraints: Vec<String>,
    mechanical_policy_ref: String,
    priority_class: String,
    fairness_policy_ref: String,
    retry_policy: RetryPolicy,
    redundancy_policy: RedundancyPolicy,
    expires_at: String,
    required_capabilities: Vec<String>,
    integrity_profile_ref: String,
}

#[derive(Clone, Debug)]
struct EnvelopeControlEntry {
    envelope_id: String,
    envelope_digest: String,
    envelope_bytes: Vec<u8>,
    authorization_ref: String,
    issuer_ref: String,
    state: EnvelopeControlState,
    revision: u64,
    authorized_at_unix_ms: u64,
    expires_at: String,
    expires_at_unix_ms: u64,
    superseded_by: Option<String>,
    revoked_at_unix_ms: Option<u64>,
    revocation_reason: Option<String>,
}

#[derive(Debug)]
struct AuthorityCapability {
    proof_digest: String,
}

#[derive(Debug)]
pub struct EnvelopeControlRegistry {
    capacity: usize,
    authority_proof_digest: String,
    source_authority_ref: String,
    entries: BTreeMap<String, EnvelopeControlEntry>,
}

impl EnvelopeControlRegistry {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn observe(
        &self,
        envelope_id: &str,
        observed_at_unix_ms: u64,
    ) -> Result<ControlStateWitness, AuthorityIssuanceError> {
        let entry = self
            .entries
            .get(envelope_id)
            .ok_or_else(|| AuthorityIssuanceError::UnknownEnvelope(envelope_id.into()))?;
        if observed_at_unix_ms < entry.authorized_at_unix_ms {
            return Err(AuthorityIssuanceError::InvalidControlTransition(format!(
                "cannot observe envelope {envelope_id} before its authorization time"
            )));
        }
        let observed_state = match entry.state {
            EnvelopeControlState::Revoked => ControlState::Revoked,
            EnvelopeControlState::Superseded => ControlState::Superseded,
            EnvelopeControlState::Active if observed_at_unix_ms >= entry.expires_at_unix_ms => {
                ControlState::Expired
            }
            EnvelopeControlState::Active => ControlState::Active,
        };
        let revision = format!("registry-rev:{}", entry.revision);
        let state_label = match observed_state {
            ControlState::Active => "active",
            ControlState::Revoked => "revoked",
            ControlState::Superseded => "superseded",
            ControlState::Expired => "expired",
        };
        let witness_id = framed_digest(&[
            ("envelope-id", entry.envelope_id.as_bytes()),
            ("envelope-digest", entry.envelope_digest.as_bytes()),
            ("state", state_label.as_bytes()),
            ("revision", revision.as_bytes()),
            ("observed-at-ms", &observed_at_unix_ms.to_be_bytes()),
        ]);
        Ok(ControlStateWitness {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: E3_PROTOCOL_MAJOR,
            protocol_minor: E3_PROTOCOL_MINOR,
            record_type: "ControlStateWitness".into(),
            witness_id,
            envelope_id: entry.envelope_id.clone(),
            envelope_digest: entry.envelope_digest.clone(),
            e1_expires_at: entry.expires_at.clone(),
            observed_state,
            observed_revision: revision,
            observed_at_unix_ms,
            source_authority_ref: self.source_authority_ref.clone(),
            authority_effect: "none".into(),
        })
    }

    fn check_capability(
        &self,
        capability: &AuthorityCapability,
    ) -> Result<(), AuthorityIssuanceError> {
        if capability.proof_digest != self.authority_proof_digest {
            return Err(AuthorityIssuanceError::IntegrityConflict(
                "issuer capability proof mismatch".into(),
            ));
        }
        Ok(())
    }

    fn register_entry(
        &mut self,
        capability: &AuthorityCapability,
        entry: EnvelopeControlEntry,
    ) -> Result<(), AuthorityIssuanceError> {
        self.check_capability(capability)?;
        if let Some(existing) = self.entries.get(&entry.envelope_id) {
            if existing.envelope_digest == entry.envelope_digest
                && existing.envelope_bytes == entry.envelope_bytes
                && existing.authorization_ref == entry.authorization_ref
                && existing.issuer_ref == entry.issuer_ref
                && existing.expires_at_unix_ms == entry.expires_at_unix_ms
            {
                return Ok(());
            }
            return Err(AuthorityIssuanceError::IntegrityConflict(format!(
                "envelope_id {} is already bound to different bytes or authority",
                entry.envelope_id
            )));
        }
        if self.entries.len() >= self.capacity {
            return Err(AuthorityIssuanceError::RegistryFull);
        }
        self.entries.insert(entry.envelope_id.clone(), entry);
        Ok(())
    }

    fn revoke(
        &mut self,
        capability: &AuthorityCapability,
        envelope_id: &str,
        revoked_at_unix_ms: u64,
        reason: &str,
    ) -> Result<(), AuthorityIssuanceError> {
        self.check_capability(capability)?;
        let entry = self
            .entries
            .get_mut(envelope_id)
            .ok_or_else(|| AuthorityIssuanceError::UnknownEnvelope(envelope_id.into()))?;
        if revoked_at_unix_ms < entry.authorized_at_unix_ms
            || revoked_at_unix_ms >= entry.expires_at_unix_ms
        {
            return Err(AuthorityIssuanceError::InvalidControlTransition(format!(
                "revocation time is outside the active interval for envelope {envelope_id}"
            )));
        }
        match entry.state {
            EnvelopeControlState::Active => {
                entry.state = EnvelopeControlState::Revoked;
                entry.revision = entry.revision.saturating_add(1);
                entry.revoked_at_unix_ms = Some(revoked_at_unix_ms);
                entry.revocation_reason = Some(reason.into());
                Ok(())
            }
            EnvelopeControlState::Revoked
                if entry.revoked_at_unix_ms == Some(revoked_at_unix_ms)
                    && entry.revocation_reason.as_deref() == Some(reason) =>
            {
                Ok(())
            }
            _ => Err(AuthorityIssuanceError::InvalidControlTransition(format!(
                "cannot revoke envelope {envelope_id} from {:?}",
                entry.state
            ))),
        }
    }

    fn supersede_with_entry(
        &mut self,
        capability: &AuthorityCapability,
        prior_envelope_id: &str,
        transition_at_unix_ms: u64,
        replacement: EnvelopeControlEntry,
    ) -> Result<(), AuthorityIssuanceError> {
        self.check_capability(capability)?;
        if prior_envelope_id == replacement.envelope_id {
            return Err(AuthorityIssuanceError::InvalidControlTransition(
                "an envelope cannot supersede itself".into(),
            ));
        }
        if self.entries.contains_key(&replacement.envelope_id) {
            return Err(AuthorityIssuanceError::IntegrityConflict(
                "replacement envelope already exists".into(),
            ));
        }
        if self.entries.len() >= self.capacity {
            return Err(AuthorityIssuanceError::RegistryFull);
        }
        let prior = self
            .entries
            .get(prior_envelope_id)
            .ok_or_else(|| AuthorityIssuanceError::UnknownEnvelope(prior_envelope_id.into()))?;
        if prior.state != EnvelopeControlState::Active {
            return Err(AuthorityIssuanceError::InvalidControlTransition(format!(
                "only an active envelope may be superseded; found {:?}",
                prior.state
            )));
        }
        if transition_at_unix_ms < prior.authorized_at_unix_ms
            || transition_at_unix_ms >= prior.expires_at_unix_ms
        {
            return Err(AuthorityIssuanceError::InvalidControlTransition(format!(
                "supersession time is outside the active interval for envelope {prior_envelope_id}"
            )));
        }

        let replacement_id = replacement.envelope_id.clone();
        self.entries.insert(replacement_id.clone(), replacement);
        let prior = self
            .entries
            .get_mut(prior_envelope_id)
            .expect("prior envelope was validated before replacement insertion");
        prior.state = EnvelopeControlState::Superseded;
        prior.revision = prior.revision.saturating_add(1);
        prior.superseded_by = Some(replacement_id);
        Ok(())
    }
}

#[derive(Debug)]
pub struct AetherMechanicalAuthorityIssuer {
    issuer_ref: String,
    issuer_revision: String,
    build: IssuerBuildIdentity,
    operation_timeout_ms: u64,
    capability: AuthorityCapability,
    registry: EnvelopeControlRegistry,
}

#[derive(Clone, Debug)]
struct PreparedControlBundle {
    admission: AetherOperationAdmissionDecision,
    authorization: MechanicalAuthorizationDecision,
    operation_manifest_bytes: Vec<u8>,
    payload_manifest_bytes: Vec<u8>,
    operation_profile_bytes: Vec<u8>,
    mechanical_profile_bytes: Vec<u8>,
    envelope_bytes: Vec<u8>,
    envelope_id: String,
    envelope_digest: String,
    expires_at: String,
    expires_at_unix_ms: u64,
    mechanical_profile: MechanicalProfile,
}

impl AetherMechanicalAuthorityIssuer {
    /// Construct the AETHER-owned issuer for the separately governed C2
    /// real-HTTP shadow lane.
    ///
    /// This creates only the same-process authority capability already defined
    /// by C1. It does not enable routing, dispatch, queue admission, or
    /// cross-process authority transport.
    pub fn for_c2_shadow(
        issuer_ref: impl Into<String>,
        issuer_revision: impl Into<String>,
        build: IssuerBuildIdentity,
        operation_timeout_ms: u64,
        registry_capacity: usize,
    ) -> Result<Self, AuthorityIssuanceError> {
        let issuer_ref = issuer_ref.into();
        let issuer_revision = issuer_revision.into();
        if issuer_ref.trim().is_empty() || issuer_revision.trim().is_empty() {
            return Err(AuthorityIssuanceError::InvalidInput(
                "C2 shadow issuer_ref/issuer_revision must be non-empty".into(),
            ));
        }
        if operation_timeout_ms == 0 {
            return Err(AuthorityIssuanceError::InvalidInput(
                "C2 shadow operation_timeout_ms must be greater than zero".into(),
            ));
        }
        if registry_capacity == 0 {
            return Err(AuthorityIssuanceError::InvalidInput(
                "C2 shadow registry_capacity must be greater than zero".into(),
            ));
        }
        let build_digest = build.digest()?;
        let proof_digest = framed_digest(&[
            ("issuer-ref", issuer_ref.as_bytes()),
            ("issuer-revision", issuer_revision.as_bytes()),
            ("issuer-build", build_digest.as_bytes()),
        ]);
        Ok(Self {
            issuer_ref: issuer_ref.clone(),
            issuer_revision,
            build,
            operation_timeout_ms,
            capability: AuthorityCapability {
                proof_digest: proof_digest.clone(),
            },
            registry: EnvelopeControlRegistry {
                capacity: registry_capacity,
                authority_proof_digest: proof_digest,
                source_authority_ref: issuer_ref,
                entries: BTreeMap::new(),
            },
        })
    }

    pub fn registry(&self) -> &EnvelopeControlRegistry {
        &self.registry
    }

    pub fn validate_transport(
        &self,
        transport: AuthorityTransport,
    ) -> Result<(), AuthorityIssuanceError> {
        match transport {
            AuthorityTransport::SameProcess => Ok(()),
            AuthorityTransport::CrossProcess => {
                Err(AuthorityIssuanceError::CrossProcessUnauthorized)
            }
        }
    }

    pub fn decide_operation(
        &self,
        input: &OperationAdmissionInput,
    ) -> Result<AetherOperationAdmissionDecision, AuthorityIssuanceError> {
        let (decision, _, _, _) = self.admission_material(input)?;
        Ok(decision)
    }

    pub fn issue_control_bundle(
        &mut self,
        input: &OperationAdmissionInput,
    ) -> Result<IssuedMechanicalControlBundle, AuthorityIssuanceError> {
        self.validate_transport(AuthorityTransport::SameProcess)?;
        let prepared = self.prepare_control_bundle(input)?;
        let entry = prepared.registry_entry(&self.issuer_ref);
        self.registry.register_entry(&self.capability, entry)?;
        self.finalize_prepared(prepared, input.decided_at_unix_ms)
    }

    pub fn issue_superseding_control_bundle(
        &mut self,
        prior_envelope_id: &str,
        input: &OperationAdmissionInput,
    ) -> Result<IssuedMechanicalControlBundle, AuthorityIssuanceError> {
        self.validate_transport(AuthorityTransport::SameProcess)?;
        let prepared = self.prepare_control_bundle(input)?;
        let entry = prepared.registry_entry(&self.issuer_ref);
        self.registry.supersede_with_entry(
            &self.capability,
            prior_envelope_id,
            input.decided_at_unix_ms,
            entry,
        )?;
        self.finalize_prepared(prepared, input.decided_at_unix_ms)
    }

    pub fn revoke_envelope(
        &mut self,
        envelope_id: &str,
        revoked_at_unix_ms: u64,
        reason: &str,
    ) -> Result<(), AuthorityIssuanceError> {
        if reason.trim().is_empty() {
            return Err(AuthorityIssuanceError::InvalidInput(
                "revocation reason must be non-empty".into(),
            ));
        }
        self.registry
            .revoke(&self.capability, envelope_id, revoked_at_unix_ms, reason)
    }

    pub fn verify_bundle(
        &self,
        bundle: &IssuedMechanicalControlBundle,
    ) -> Result<(), AuthorityIssuanceError> {
        verify_admission_identity(&bundle.admission)?;
        if bundle.admission.decision != AdmissionDisposition::Admitted {
            return Err(AuthorityIssuanceError::AdmissionRejected(
                bundle
                    .admission
                    .rejection_reason
                    .clone()
                    .unwrap_or_else(|| "record is rejected".into()),
            ));
        }
        if sha256_hex(&bundle.operation_manifest_bytes)
            != bundle.admission.operation_manifest_digest
        {
            return Err(AuthorityIssuanceError::IntegrityConflict(
                "operation manifest digest mismatch".into(),
            ));
        }
        if sha256_hex(&bundle.operation_profile_bytes) != bundle.admission.operation_profile_digest
        {
            return Err(AuthorityIssuanceError::IntegrityConflict(
                "operation profile digest mismatch".into(),
            ));
        }
        if sha256_hex(&bundle.payload_manifest_bytes)
            != bundle.authorization.payload_manifest_digest
        {
            return Err(AuthorityIssuanceError::IntegrityConflict(
                "payload manifest digest mismatch".into(),
            ));
        }
        let protected_mechanical_profile = protected_local_mechanical_profile();
        let protected_mechanical_profile_bytes = protected_mechanical_profile.canonical_bytes()?;
        if bundle.mechanical_profile_bytes != protected_mechanical_profile_bytes
            || sha256_hex(&bundle.mechanical_profile_bytes)
                != bundle.authorization.mechanical_profile_digest
        {
            return Err(AuthorityIssuanceError::IntegrityConflict(
                "mechanical profile digest/bytes mismatch".into(),
            ));
        }
        verify_mechanical_identity(&bundle.authorization)?;

        let admission_digest = sha256_hex(&canonicalize_serializable(&bundle.admission)?);
        if bundle.authorization.operation_admission_ref != bundle.admission.operation_admission_id
            || bundle.authorization.operation_admission_digest != admission_digest
            || bundle.authorization.scope_manifest_ref != bundle.admission.operation_manifest_ref
            || bundle.authorization.scope_manifest_digest
                != bundle.admission.operation_manifest_digest
            || bundle.authorization.mechanical_profile_ref != C1_LOCAL_MECHANICAL_PROFILE
            || bundle.authorization.mechanical_profile_digest
                != protected_mechanical_profile.digest()?
            || bundle.authorization.issuer_ref != self.issuer_ref
            || bundle.authorization.issuer_revision != self.issuer_revision
            || bundle.authorization.issuer_build_digest != self.build.digest()?
        {
            return Err(AuthorityIssuanceError::IntegrityConflict(
                "mechanical authorization does not bind the exact admitted operation/issuer".into(),
            ));
        }

        let envelope = parse_json_no_duplicates(&bundle.envelope_bytes)?;
        let expected_envelope_id = envelope_id(
            &bundle.authorization.mechanical_authorization_id,
            &bundle.operation_manifest_bytes,
            &bundle.mechanical_profile_bytes,
        );
        let envelope_field = string_field(&envelope, "envelope_id")?;
        let auth_field = string_field(&envelope, "authorization_ref")?;
        let issuer_field = string_field(&envelope, "issuer_ref")?;
        let scope_field = string_field(&envelope, "scope_ref")?;
        let payload_field = string_field(&envelope, "payload_ref")?;
        let integrity_field = string_field(&envelope, "integrity_profile_ref")?;
        let scope_digest = envelope
            .get("scope_digest")
            .and_then(Value::as_object)
            .and_then(|object| object.get("value"))
            .and_then(Value::as_str)
            .ok_or_else(|| {
                AuthorityIssuanceError::IntegrityConflict("missing E1 scope digest".into())
            })?;
        if envelope_field != expected_envelope_id
            || auth_field != bundle.authorization.mechanical_authorization_id
            || issuer_field != self.issuer_ref
            || scope_field != bundle.authorization.scope_manifest_ref
            || scope_digest != bundle.authorization.scope_manifest_digest
            || payload_field != bundle.authorization.payload_manifest_ref
            || integrity_field != C1_INTEGRITY_PROFILE
        {
            return Err(AuthorityIssuanceError::IntegrityConflict(
                "E1 envelope authority binding mismatch".into(),
            ));
        }

        validate_placement_constraint_narrowing(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
        )?;
        validate_control_state_witness(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &bundle.control_state_witness,
        )?;
        let expected_constraint_id = constraint_id(
            &sha256_hex(&bundle.envelope_bytes),
            &bundle.control_state_witness.digest()?,
            &bundle.authorization.mechanical_profile_digest,
            bundle.placement_constraint.decision_time_unix_ms,
        );
        if bundle.placement_constraint.constraint_set_id != expected_constraint_id {
            return Err(AuthorityIssuanceError::IntegrityConflict(
                "placement constraint identity mismatch".into(),
            ));
        }
        Ok(())
    }

    fn admission_material(
        &self,
        input: &OperationAdmissionInput,
    ) -> Result<
        (
            AetherOperationAdmissionDecision,
            Vec<u8>,
            Vec<u8>,
            OperationProfile,
        ),
        AuthorityIssuanceError,
    > {
        validate_non_empty_input(input)?;
        let resolved = OperationClass::from_http(&input.http_method, &input.http_path)?;
        if resolved != input.operation_class {
            return Err(AuthorityIssuanceError::OperationProfileMismatch);
        }
        let profile = input.operation_class.profile();
        profile.validate_binding(input)?;
        let profile_bytes = profile.canonical_bytes()?;
        let profile_digest = sha256_hex(&profile_bytes);

        let payload_bytes = canonicalize_serializable(&input.typed_request_payload)?;
        let payload_digest = sha256_hex(&payload_bytes);
        let (effective_policy_digest, policy_rejection) =
            effective_policy_digest(&profile, &input.policy_binding)?;

        let manifest = OperationManifest {
            manifest_version: C1_OPERATION_MANIFEST_VERSION.into(),
            operation_class: input.operation_class,
            namespace_ref: input.namespace_ref.clone(),
            request_input_ref: format!("aether-request:{}", input.request_id),
            effective_policy_digest: effective_policy_digest.clone(),
            typed_request_payload_digest: payload_digest,
            operation_profile_ref: profile.profile_ref.clone(),
            operation_profile_digest: profile_digest.clone(),
        };
        let manifest_bytes = canonicalize_serializable(&manifest)?;
        let manifest_digest = sha256_hex(&manifest_bytes);
        let manifest_ref = format!("aether-operation-manifest:sha256:{manifest_digest}");

        let rejection_reason = if !input.source_operation_admitted {
            Some("AETHER source operation admission was not granted".into())
        } else {
            policy_rejection
        };
        let decision = if rejection_reason.is_some() {
            AdmissionDisposition::Rejected
        } else {
            AdmissionDisposition::Admitted
        };
        let material = AdmissionIdentityMaterial {
            schema_version: "1.0".into(),
            record_type: "AetherOperationAdmissionDecision".into(),
            request_id: input.request_id.clone(),
            correlation_id: input.correlation_id.clone(),
            operation_class: input.operation_class,
            namespace_ref: input.namespace_ref.clone(),
            principal_ref: input.principal_ref.clone(),
            token_ref: input.token_ref.clone(),
            required_scope: input.required_scope.clone(),
            effective_policy_digest: effective_policy_digest.clone(),
            operation_manifest_ref: manifest_ref.clone(),
            operation_manifest_digest: manifest_digest.clone(),
            operation_profile_ref: profile.profile_ref.clone(),
            operation_profile_digest: profile_digest.clone(),
            decision: decision.clone(),
            decision_revision: input.decision_revision.clone(),
            decided_at_unix_ms: input.decided_at_unix_ms,
            source_authority_ref: input.source_authority_ref.clone(),
            authority_effect: "operation_attempt_only".into(),
            rejection_reason: rejection_reason.clone(),
        };
        let material_bytes = canonicalize_serializable(&material)?;
        let operation_admission_id = framed_digest(&[("operation-admission", &material_bytes)]);
        let record = AetherOperationAdmissionDecision {
            schema_version: material.schema_version,
            record_type: material.record_type,
            operation_admission_id,
            request_id: material.request_id,
            correlation_id: material.correlation_id,
            operation_class: material.operation_class,
            namespace_ref: material.namespace_ref,
            principal_ref: material.principal_ref,
            token_ref: material.token_ref,
            required_scope: material.required_scope,
            effective_policy_digest: material.effective_policy_digest,
            operation_manifest_ref: material.operation_manifest_ref,
            operation_manifest_digest: material.operation_manifest_digest,
            operation_profile_ref: material.operation_profile_ref,
            operation_profile_digest: material.operation_profile_digest,
            decision: material.decision,
            decision_revision: material.decision_revision,
            decided_at_unix_ms: material.decided_at_unix_ms,
            source_authority_ref: material.source_authority_ref,
            authority_effect: material.authority_effect,
            rejection_reason: material.rejection_reason,
        };
        Ok((record, manifest_bytes, profile_bytes, profile))
    }

    fn prepare_control_bundle(
        &self,
        input: &OperationAdmissionInput,
    ) -> Result<PreparedControlBundle, AuthorityIssuanceError> {
        let (admission, operation_manifest_bytes, operation_profile_bytes, profile) =
            self.admission_material(input)?;
        if admission.decision != AdmissionDisposition::Admitted {
            return Err(AuthorityIssuanceError::AdmissionRejected(
                admission
                    .rejection_reason
                    .clone()
                    .unwrap_or_else(|| "operation admission rejected".into()),
            ));
        }
        let payload_manifest_bytes = canonicalize_serializable(&input.typed_request_payload)?;
        let payload_manifest_digest = sha256_hex(&payload_manifest_bytes);
        let payload_manifest_ref =
            format!("aether-request-payload:sha256:{payload_manifest_digest}");
        let mechanical_profile = protected_local_mechanical_profile();
        mechanical_profile.validate_operation_profile(&profile)?;
        let mechanical_profile_bytes = mechanical_profile.canonical_bytes()?;
        let mechanical_profile_digest = sha256_hex(&mechanical_profile_bytes);
        let operation_admission_digest = sha256_hex(&canonicalize_serializable(&admission)?);
        let expires_at_unix_ms = input
            .decided_at_unix_ms
            .checked_add(self.operation_timeout_ms)
            .ok_or_else(|| AuthorityIssuanceError::InvalidInput("expiry overflow".into()))?;
        let expires_at = unix_ms_to_rfc3339(expires_at_unix_ms)?;
        let issuer_build_digest = self.build.digest()?;
        let mechanical_material = MechanicalIdentityMaterial {
            schema_version: "1.0".into(),
            record_type: "MechanicalAuthorizationDecision".into(),
            operation_admission_ref: admission.operation_admission_id.clone(),
            operation_admission_digest,
            correlation_id: admission.correlation_id.clone(),
            scope_manifest_ref: admission.operation_manifest_ref.clone(),
            scope_manifest_digest: admission.operation_manifest_digest.clone(),
            payload_manifest_ref,
            payload_manifest_digest,
            mechanical_profile_ref: mechanical_profile.profile_ref.clone(),
            mechanical_profile_digest: mechanical_profile_digest.clone(),
            expires_at: expires_at.clone(),
            expires_at_unix_ms,
            issuer_ref: self.issuer_ref.clone(),
            issuer_revision: self.issuer_revision.clone(),
            issuer_build_digest,
            decided_at_unix_ms: input.decided_at_unix_ms,
            authority_effect: "mechanical_only".into(),
        };
        let mechanical_bytes = canonicalize_serializable(&mechanical_material)?;
        let mechanical_authorization_id =
            framed_digest(&[("mechanical-authorization", &mechanical_bytes)]);
        let authorization = MechanicalAuthorizationDecision {
            schema_version: mechanical_material.schema_version,
            record_type: mechanical_material.record_type,
            mechanical_authorization_id: mechanical_authorization_id.clone(),
            operation_admission_ref: mechanical_material.operation_admission_ref,
            operation_admission_digest: mechanical_material.operation_admission_digest,
            correlation_id: mechanical_material.correlation_id,
            scope_manifest_ref: mechanical_material.scope_manifest_ref,
            scope_manifest_digest: mechanical_material.scope_manifest_digest,
            payload_manifest_ref: mechanical_material.payload_manifest_ref,
            payload_manifest_digest: mechanical_material.payload_manifest_digest,
            mechanical_profile_ref: mechanical_material.mechanical_profile_ref,
            mechanical_profile_digest: mechanical_material.mechanical_profile_digest,
            expires_at: mechanical_material.expires_at,
            expires_at_unix_ms: mechanical_material.expires_at_unix_ms,
            issuer_ref: mechanical_material.issuer_ref,
            issuer_revision: mechanical_material.issuer_revision,
            issuer_build_digest: mechanical_material.issuer_build_digest,
            decided_at_unix_ms: mechanical_material.decided_at_unix_ms,
            authority_effect: mechanical_material.authority_effect,
        };

        let id = envelope_id(
            &mechanical_authorization_id,
            &operation_manifest_bytes,
            &mechanical_profile_bytes,
        );
        let envelope = MechanicalEnvelope {
            schema_version: "1.0".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: E1_PROTOCOL_MAJOR,
            protocol_minor: E1_PROTOCOL_MINOR,
            record_type: "MechanicalEnvelopeAuthorized".into(),
            domain: "control_bridge".into(),
            authority_owner: "upstream_governed_record".into(),
            authority_effect: "mechanical_only".into(),
            envelope_id: id.clone(),
            correlation_id: admission.correlation_id.clone(),
            authorization_ref: mechanical_authorization_id,
            issuer_ref: self.issuer_ref.clone(),
            scope_ref: admission.operation_manifest_ref.clone(),
            scope_digest: Sha256Ref {
                algorithm: "sha256".into(),
                value: admission.operation_manifest_digest.clone(),
            },
            payload_ref: authorization.payload_manifest_ref.clone(),
            permitted_actions: mechanical_profile.permitted_actions.clone(),
            eligible_resource_classes: mechanical_profile.eligible_resource_classes.clone(),
            trust_zones: mechanical_profile.trust_zones.clone(),
            locality_constraints: mechanical_profile.locality_constraints.clone(),
            mechanical_policy_ref: mechanical_profile.profile_ref.clone(),
            priority_class: mechanical_profile.priority_class.clone(),
            fairness_policy_ref: mechanical_profile.fairness_policy_ref.clone(),
            retry_policy: RetryPolicy {
                max_attempts: mechanical_profile.max_attempts,
                backoff_class: mechanical_profile.backoff_class.clone(),
            },
            redundancy_policy: RedundancyPolicy {
                max_parallel_copies: mechanical_profile.max_parallel_copies,
            },
            expires_at: expires_at.clone(),
            required_capabilities: mechanical_profile.required_capabilities.clone(),
            integrity_profile_ref: mechanical_profile.integrity_profile_ref.clone(),
        };
        let envelope_bytes = canonicalize_serializable(&envelope)?;
        let envelope_digest = sha256_hex(&envelope_bytes);

        Ok(PreparedControlBundle {
            admission,
            authorization,
            operation_manifest_bytes,
            payload_manifest_bytes,
            operation_profile_bytes,
            mechanical_profile_bytes,
            envelope_bytes,
            envelope_id: id,
            envelope_digest,
            expires_at,
            expires_at_unix_ms,
            mechanical_profile,
        })
    }

    fn finalize_prepared(
        &self,
        prepared: PreparedControlBundle,
        decision_time_unix_ms: u64,
    ) -> Result<IssuedMechanicalControlBundle, AuthorityIssuanceError> {
        let witness = self
            .registry
            .observe(&prepared.envelope_id, decision_time_unix_ms)?;
        if witness.observed_state != ControlState::Active {
            return Err(AuthorityIssuanceError::InvalidControlTransition(
                "newly issued envelope is not active".into(),
            ));
        }
        let witness_digest = witness.digest()?;
        let constraint = PlacementConstraintSet {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: E3_PROTOCOL_MAJOR,
            protocol_minor: E3_PROTOCOL_MINOR,
            record_type: "PlacementConstraintSet".into(),
            constraint_set_id: constraint_id(
                &prepared.envelope_digest,
                &witness_digest,
                &prepared.authorization.mechanical_profile_digest,
                decision_time_unix_ms,
            ),
            envelope_id: prepared.envelope_id.clone(),
            envelope_digest: prepared.envelope_digest.clone(),
            correlation_id: prepared.admission.correlation_id.clone(),
            resource_requirements: ResourceRequirements {
                capacity_unit: prepared.mechanical_profile.capacity_unit.clone(),
                minimum_capacity_units: prepared.mechanical_profile.minimum_capacity_units,
            },
            max_snapshot_age_ms: prepared.mechanical_profile.max_snapshot_age_ms,
            decision_time_unix_ms,
            control_state_ref: witness.witness_id.clone(),
            control_state_digest: witness_digest,
            source_authority_ref: self.issuer_ref.clone(),
            authority_effect: "mechanical_narrowing_only".into(),
            eligible_resource_classes: Some(
                prepared
                    .mechanical_profile
                    .eligible_resource_classes
                    .clone(),
            ),
            trust_zones: Some(prepared.mechanical_profile.trust_zones.clone()),
            required_capabilities: Some(prepared.mechanical_profile.required_capabilities.clone()),
            locality_constraints: Some(prepared.mechanical_profile.locality_constraints.clone()),
        };
        validate_placement_constraint_narrowing(&prepared.envelope_bytes, &constraint)?;
        validate_control_state_witness(&prepared.envelope_bytes, &constraint, &witness)?;

        let bundle = IssuedMechanicalControlBundle {
            admission: prepared.admission,
            authorization: prepared.authorization,
            operation_manifest_bytes: prepared.operation_manifest_bytes,
            payload_manifest_bytes: prepared.payload_manifest_bytes,
            operation_profile_bytes: prepared.operation_profile_bytes,
            mechanical_profile_bytes: prepared.mechanical_profile_bytes,
            envelope_bytes: prepared.envelope_bytes,
            placement_constraint: constraint,
            control_state_witness: witness,
        };
        self.verify_bundle(&bundle)?;
        Ok(bundle)
    }
}

impl PreparedControlBundle {
    fn registry_entry(&self, issuer_ref: &str) -> EnvelopeControlEntry {
        EnvelopeControlEntry {
            envelope_id: self.envelope_id.clone(),
            envelope_digest: self.envelope_digest.clone(),
            envelope_bytes: self.envelope_bytes.clone(),
            authorization_ref: self.authorization.mechanical_authorization_id.clone(),
            issuer_ref: issuer_ref.into(),
            state: EnvelopeControlState::Active,
            revision: 1,
            authorized_at_unix_ms: self.authorization.decided_at_unix_ms,
            expires_at: self.expires_at.clone(),
            expires_at_unix_ms: self.expires_at_unix_ms,
            superseded_by: None,
            revoked_at_unix_ms: None,
            revocation_reason: None,
        }
    }
}

fn validate_non_empty_input(input: &OperationAdmissionInput) -> Result<(), AuthorityIssuanceError> {
    let required = [
        ("request_id", input.request_id.as_str()),
        ("correlation_id", input.correlation_id.as_str()),
        ("http_method", input.http_method.as_str()),
        ("http_path", input.http_path.as_str()),
        ("namespace_ref", input.namespace_ref.as_str()),
        ("principal_ref", input.principal_ref.as_str()),
        ("required_scope", input.required_scope.as_str()),
        ("decision_revision", input.decision_revision.as_str()),
        ("source_authority_ref", input.source_authority_ref.as_str()),
    ];
    if let Some((field, _)) = required.iter().find(|(_, value)| value.trim().is_empty()) {
        return Err(AuthorityIssuanceError::InvalidInput(format!(
            "{field} must be non-empty"
        )));
    }
    if input.http_method != input.http_method.to_ascii_uppercase() {
        return Err(AuthorityIssuanceError::InvalidInput(
            "http_method must be uppercase".into(),
        ));
    }
    Ok(())
}

fn effective_policy_digest(
    profile: &OperationProfile,
    evidence: &PolicyBindingEvidence,
) -> Result<(String, Option<String>), AuthorityIssuanceError> {
    match evidence {
        PolicyBindingEvidence::DeniedEscalation => Ok((
            sha256_hex(PUBLIC_POLICY_MARKER),
            Some("policy binding denied escalation".into()),
        )),
        PolicyBindingEvidence::Public => Ok((sha256_hex(PUBLIC_POLICY_MARKER), None)),
        PolicyBindingEvidence::Bound {
            effective_policy_digest,
        } => {
            require_sha256(effective_policy_digest, "effective policy")?;
            if !profile.policy_binding_required {
                return Ok((
                    effective_policy_digest.clone(),
                    Some("operation profile does not accept a bound policy context".into()),
                ));
            }
            Ok((effective_policy_digest.clone(), None))
        }
    }
}

fn verify_admission_identity(
    record: &AetherOperationAdmissionDecision,
) -> Result<(), AuthorityIssuanceError> {
    let material = AdmissionIdentityMaterial {
        schema_version: record.schema_version.clone(),
        record_type: record.record_type.clone(),
        request_id: record.request_id.clone(),
        correlation_id: record.correlation_id.clone(),
        operation_class: record.operation_class,
        namespace_ref: record.namespace_ref.clone(),
        principal_ref: record.principal_ref.clone(),
        token_ref: record.token_ref.clone(),
        required_scope: record.required_scope.clone(),
        effective_policy_digest: record.effective_policy_digest.clone(),
        operation_manifest_ref: record.operation_manifest_ref.clone(),
        operation_manifest_digest: record.operation_manifest_digest.clone(),
        operation_profile_ref: record.operation_profile_ref.clone(),
        operation_profile_digest: record.operation_profile_digest.clone(),
        decision: record.decision.clone(),
        decision_revision: record.decision_revision.clone(),
        decided_at_unix_ms: record.decided_at_unix_ms,
        source_authority_ref: record.source_authority_ref.clone(),
        authority_effect: record.authority_effect.clone(),
        rejection_reason: record.rejection_reason.clone(),
    };
    let expected = framed_digest(&[(
        "operation-admission",
        &canonicalize_serializable(&material)?,
    )]);
    if record.operation_admission_id != expected
        || record.schema_version != "1.0"
        || record.record_type != "AetherOperationAdmissionDecision"
        || record.authority_effect != "operation_attempt_only"
    {
        return Err(AuthorityIssuanceError::IntegrityConflict(
            "operation admission identity/header mismatch".into(),
        ));
    }
    Ok(())
}

fn verify_mechanical_identity(
    record: &MechanicalAuthorizationDecision,
) -> Result<(), AuthorityIssuanceError> {
    let material = MechanicalIdentityMaterial {
        schema_version: record.schema_version.clone(),
        record_type: record.record_type.clone(),
        operation_admission_ref: record.operation_admission_ref.clone(),
        operation_admission_digest: record.operation_admission_digest.clone(),
        correlation_id: record.correlation_id.clone(),
        scope_manifest_ref: record.scope_manifest_ref.clone(),
        scope_manifest_digest: record.scope_manifest_digest.clone(),
        payload_manifest_ref: record.payload_manifest_ref.clone(),
        payload_manifest_digest: record.payload_manifest_digest.clone(),
        mechanical_profile_ref: record.mechanical_profile_ref.clone(),
        mechanical_profile_digest: record.mechanical_profile_digest.clone(),
        expires_at: record.expires_at.clone(),
        expires_at_unix_ms: record.expires_at_unix_ms,
        issuer_ref: record.issuer_ref.clone(),
        issuer_revision: record.issuer_revision.clone(),
        issuer_build_digest: record.issuer_build_digest.clone(),
        decided_at_unix_ms: record.decided_at_unix_ms,
        authority_effect: record.authority_effect.clone(),
    };
    let expected = framed_digest(&[(
        "mechanical-authorization",
        &canonicalize_serializable(&material)?,
    )]);
    if record.mechanical_authorization_id != expected
        || record.schema_version != "1.0"
        || record.record_type != "MechanicalAuthorizationDecision"
        || record.authority_effect != "mechanical_only"
    {
        return Err(AuthorityIssuanceError::IntegrityConflict(
            "mechanical authorization identity/header mismatch".into(),
        ));
    }
    Ok(())
}

fn envelope_id(
    mechanical_authorization_id: &str,
    operation_manifest_bytes: &[u8],
    operation_profile_bytes: &[u8],
) -> String {
    framed_digest(&[
        (
            "mechanical-authorization-id",
            mechanical_authorization_id.as_bytes(),
        ),
        ("operation-manifest", operation_manifest_bytes),
        ("operation-profile", operation_profile_bytes),
    ])
}

fn constraint_id(
    envelope_digest: &str,
    witness_digest: &str,
    operation_profile_digest: &str,
    decision_time_unix_ms: u64,
) -> String {
    framed_digest(&[
        ("e1-envelope-digest", envelope_digest.as_bytes()),
        ("control-witness-digest", witness_digest.as_bytes()),
        (
            "operation-profile-digest",
            operation_profile_digest.as_bytes(),
        ),
        ("decision-time-ms", &decision_time_unix_ms.to_be_bytes()),
    ])
}

fn require_sha256(value: &str, field: &str) -> Result<(), AuthorityIssuanceError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(AuthorityIssuanceError::InvalidInput(format!(
            "{field} must be lowercase SHA-256 hex"
        )));
    }
    Ok(())
}

fn string_field<'a>(value: &'a Value, field: &str) -> Result<&'a str, AuthorityIssuanceError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            AuthorityIssuanceError::IntegrityConflict(format!("missing/invalid E1 field {field}"))
        })
}

fn bytes_to_lower_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        write!(&mut out, "{byte:02x}").expect("writing to String cannot fail");
    }
    out
}

fn framed_digest(parts: &[(&str, &[u8])]) -> String {
    let mut hasher = Sha256::new();
    for (label, bytes) in parts {
        hasher.update((label.len() as u64).to_be_bytes());
        hasher.update(label.as_bytes());
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(bytes);
    }
    bytes_to_lower_hex(hasher.finalize().as_slice())
}

pub fn unix_ms_to_rfc3339(unix_ms: u64) -> Result<String, AuthorityIssuanceError> {
    let seconds = unix_ms / 1_000;
    let millis = unix_ms % 1_000;
    let days = seconds / 86_400;
    let second_of_day = seconds % 86_400;
    let hour = second_of_day / 3_600;
    let minute = (second_of_day % 3_600) / 60;
    let second = second_of_day % 60;

    let z = i64::try_from(days)
        .map_err(|_| AuthorityIssuanceError::InvalidInput("timestamp out of range".into()))?
        + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    if month <= 2 {
        year += 1;
    }
    if !(0..=9_999).contains(&year) {
        return Err(AuthorityIssuanceError::InvalidInput(
            "timestamp year outside RFC3339 profile".into(),
        ));
    }

    if millis == 0 {
        Ok(format!(
            "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z"
        ))
    } else {
        Ok(format!(
            "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z"
        ))
    }
}

#[cfg(test)]
impl AetherMechanicalAuthorityIssuer {
    fn for_tests(operation_timeout_ms: u64, registry_capacity: usize) -> Self {
        assert!(operation_timeout_ms > 0);
        let build = IssuerBuildIdentity {
            source_commit: "c1-test-source".into(),
            source_tree: "c1-test-tree".into(),
            artifact_sha256: "7".repeat(64),
        };
        let issuer_ref = "aether-control-bridge:c1-test-issuer".to_string();
        let issuer_revision = "c1-test-revision-1".to_string();
        let proof_digest = framed_digest(&[
            ("issuer-ref", issuer_ref.as_bytes()),
            ("issuer-revision", issuer_revision.as_bytes()),
            (
                "issuer-build",
                build.digest().expect("test build identity").as_bytes(),
            ),
        ]);
        Self {
            issuer_ref: issuer_ref.clone(),
            issuer_revision,
            build,
            operation_timeout_ms,
            capability: AuthorityCapability {
                proof_digest: proof_digest.clone(),
            },
            registry: EnvelopeControlRegistry {
                capacity: registry_capacity,
                authority_proof_digest: proof_digest,
                source_authority_ref: issuer_ref,
                entries: BTreeMap::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aether_fabric::{
        decision_input_digests, f1b_scheduler_policy_identity, f1b_select_resource,
        local_blocking_pool_observation, realize_selected_placement, PlacementDecision,
        RealizationRequest, ResourceSnapshot, SelectorImplementationIdentity,
    };
    use serde_json::json;
    use std::collections::BTreeSet;

    const T0: u64 = 1_797_000_000_000;

    fn input(class: OperationClass) -> OperationAdmissionInput {
        let profile = class.profile();
        OperationAdmissionInput {
            request_id: "req-001".into(),
            correlation_id: "corr-001".into(),
            operation_class: class,
            http_method: profile.http_method.clone(),
            http_path: profile.http_path.clone(),
            namespace_ref: "default".into(),
            principal_ref: "principal:test".into(),
            token_ref: Some("token:test".into()),
            required_scope: profile.required_scope.clone(),
            policy_binding: if profile.policy_binding_required {
                PolicyBindingEvidence::Bound {
                    effective_policy_digest: "a".repeat(64),
                }
            } else {
                PolicyBindingEvidence::Public
            },
            typed_request_payload: json!({"query":"stable","n":1}),
            source_operation_admitted: true,
            decision_revision: "aether-admission-rev:1".into(),
            decided_at_unix_ms: T0,
            source_authority_ref: "aether-http-semantic-edge".into(),
        }
    }

    fn issue(
        class: OperationClass,
    ) -> (
        AetherMechanicalAuthorityIssuer,
        IssuedMechanicalControlBundle,
    ) {
        let mut issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 16);
        let bundle = issuer.issue_control_bundle(&input(class)).unwrap();
        (issuer, bundle)
    }

    fn current_constraint(
        bundle: &IssuedMechanicalControlBundle,
        witness: &ControlStateWitness,
        decision_time_unix_ms: u64,
    ) -> PlacementConstraintSet {
        let mut constraint = bundle.placement_constraint.clone();
        constraint.decision_time_unix_ms = decision_time_unix_ms;
        constraint.control_state_ref = witness.witness_id.clone();
        constraint.control_state_digest = witness.digest().unwrap();
        constraint.constraint_set_id = constraint_id(
            &constraint.envelope_digest,
            &constraint.control_state_digest,
            &bundle.authorization.mechanical_profile_digest,
            decision_time_unix_ms,
        );
        constraint
    }

    fn selector_identity(seed: char) -> SelectorImplementationIdentity {
        SelectorImplementationIdentity {
            selector_id: "f1b-test-selector".into(),
            source_commit: "source".into(),
            source_tree: "tree".into(),
            artifact_sha256: seed.to_string().repeat(64),
        }
    }

    fn snapshot(time: u64) -> ResourceSnapshot {
        ResourceSnapshot::new(
            "snapshot-c1",
            time,
            "c1-test",
            vec![local_blocking_pool_observation(1, 1, 0)],
        )
        .unwrap()
    }

    #[test]
    fn c1_registry_contains_exactly_23_first_lane_profiles() {
        let mut seen = BTreeSet::new();
        for class in OperationClass::ALL_FIRST_LANE {
            let profile = class.profile();
            assert_eq!(
                OperationClass::from_http(&profile.http_method, &profile.http_path).unwrap(),
                class
            );
            assert!(seen.insert((profile.http_method, profile.http_path)));
            assert_eq!(profile.mechanical_profile_ref, C1_LOCAL_MECHANICAL_PROFILE);
            assert_eq!(profile.permitted_actions, vec![ACTION_QUEUE_PRE_START]);
            assert_eq!(profile.max_attempts, 1);
            assert_eq!(profile.max_parallel_copies, 1);
        }
        assert_eq!(seen.len(), 23);
    }

    #[test]
    fn seven_authoritative_mutations_are_closed_world_rejections() {
        let excluded = [
            ("POST", "/v1/append"),
            ("POST", "/v1/schema/register"),
            ("POST", "/v1/schema/activate"),
            ("POST", "/v1/partitions/promote"),
            ("POST", "/v1/partitions/append"),
            ("POST", "/v1/sidecars/artifacts/register"),
            ("POST", "/v1/sidecars/vectors/register"),
        ];
        for (method, path) in excluded {
            assert!(matches!(
                OperationClass::from_http(method, path),
                Err(AuthorityIssuanceError::ExcludedMutation { .. })
            ));
        }
    }

    #[test]
    fn unknown_operation_is_rejected() {
        assert!(matches!(
            OperationClass::from_http("POST", "/v1/unknown"),
            Err(AuthorityIssuanceError::UnknownOperation { .. })
        ));
    }

    #[test]
    fn token_success_alone_cannot_issue_envelope() {
        let mut issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        let mut request = input(OperationClass::History);
        request.source_operation_admitted = false;
        let decision = issuer.decide_operation(&request).unwrap();
        assert_eq!(decision.decision, AdmissionDisposition::Rejected);
        assert!(matches!(
            issuer.issue_control_bundle(&request),
            Err(AuthorityIssuanceError::AdmissionRejected(_))
        ));
    }

    #[test]
    fn namespace_or_capacity_source_facts_cannot_substitute_for_admission() {
        let mut issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        let mut request = input(OperationClass::CurrentState);
        request.namespace_ref = "namespace-with-free-capacity".into();
        request.typed_request_payload = json!({"namespace_permit":true,"available_slots":99});
        request.source_operation_admitted = false;
        assert!(matches!(
            issuer.issue_control_bundle(&request),
            Err(AuthorityIssuanceError::AdmissionRejected(_))
        ));
    }

    #[test]
    fn denied_policy_escalation_rejects_before_mechanical_authorization() {
        let mut issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        let mut request = input(OperationClass::History);
        request.policy_binding = PolicyBindingEvidence::DeniedEscalation;
        let decision = issuer.decide_operation(&request).unwrap();
        assert_eq!(decision.decision, AdmissionDisposition::Rejected);
        assert!(decision.rejection_reason.unwrap().contains("policy"));
        assert!(issuer.registry().is_empty());
        assert!(issuer.issue_control_bundle(&request).is_err());
    }

    #[test]
    fn changed_operation_manifest_is_detected() {
        let (issuer, mut bundle) = issue(OperationClass::History);
        bundle.operation_manifest_bytes =
            canonicalize_serializable(&json!({"tampered":true})).unwrap();
        assert!(matches!(
            issuer.verify_bundle(&bundle),
            Err(AuthorityIssuanceError::IntegrityConflict(_))
        ));
    }

    #[test]
    fn forged_envelope_authorization_reference_is_detected() {
        let (issuer, mut bundle) = issue(OperationClass::History);
        let mut envelope = parse_json_no_duplicates(&bundle.envelope_bytes).unwrap();
        envelope["authorization_ref"] = Value::String("forged".into());
        bundle.envelope_bytes = canonicalize_serializable(&envelope).unwrap();
        assert!(matches!(
            issuer.verify_bundle(&bundle),
            Err(AuthorityIssuanceError::IntegrityConflict(_))
                | Err(AuthorityIssuanceError::Fabric(_))
        ));
    }

    #[test]
    fn exact_duplicate_issuance_is_idempotent() {
        let mut issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        let request = input(OperationClass::RunDocument);
        let first = issuer.issue_control_bundle(&request).unwrap();
        let second = issuer.issue_control_bundle(&request).unwrap();
        assert_eq!(first, second);
        assert_eq!(issuer.registry().len(), 1);
    }

    #[test]
    fn duplicate_envelope_id_with_different_bytes_is_hard_conflict() {
        let (mut issuer, bundle) = issue(OperationClass::History);
        let id = bundle.placement_constraint.envelope_id.clone();
        let mut conflicting = issuer.registry.entries.get(&id).unwrap().clone();
        conflicting.envelope_digest = "f".repeat(64);
        assert!(matches!(
            issuer
                .registry
                .register_entry(&issuer.capability, conflicting),
            Err(AuthorityIssuanceError::IntegrityConflict(_))
        ));
    }

    #[test]
    fn active_witness_validates_against_exact_constraint() {
        let (issuer, bundle) = issue(OperationClass::SearchVectors);
        issuer.verify_bundle(&bundle).unwrap();
        assert_eq!(
            bundle.control_state_witness.observed_state,
            ControlState::Active
        );
    }

    #[test]
    fn revoked_witness_blocks_selection_and_realization() {
        let (mut issuer, bundle) = issue(OperationClass::History);
        let snap = snapshot(T0);
        let policy = f1b_scheduler_policy_identity();
        let selector = selector_identity('3');
        let selected = match f1b_select_resource(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &snap,
            &bundle.control_state_witness,
            &policy,
            &selector,
        )
        .unwrap()
        {
            PlacementDecision::Selected(selected) => selected,
            PlacementDecision::Unavailable(_) => panic!("expected placement selection"),
        };

        issuer
            .revoke_envelope(
                &bundle.placement_constraint.envelope_id,
                T0 + 1,
                "operator revoke",
            )
            .unwrap();
        let revoked = issuer
            .registry()
            .observe(&bundle.placement_constraint.envelope_id, T0 + 2)
            .unwrap();
        let current = current_constraint(&bundle, &revoked, T0 + 2);
        assert!(f1b_select_resource(
            &bundle.envelope_bytes,
            &current,
            &snapshot(T0 + 2),
            &revoked,
            &policy,
            &selector
        )
        .is_err());

        let request = RealizationRequest {
            realization_request_id: "realize-revoked".into(),
            realization_time_unix_ms: T0 + 2,
            occurred_at: "2026-12-09T00:00:00Z".into(),
        };
        assert!(realize_selected_placement(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &selected,
            &snapshot(T0 + 2),
            &revoked,
            &request
        )
        .is_err());
    }

    #[test]
    fn superseded_witness_blocks_new_selection() {
        let mut issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        let first = issuer
            .issue_control_bundle(&input(OperationClass::History))
            .unwrap();
        let mut replacement_input = input(OperationClass::History);
        replacement_input.request_id = "req-002".into();
        replacement_input.correlation_id = "corr-002".into();
        replacement_input.decided_at_unix_ms = T0 + 10;
        let _replacement = issuer
            .issue_superseding_control_bundle(
                &first.placement_constraint.envelope_id,
                &replacement_input,
            )
            .unwrap();
        let witness = issuer
            .registry()
            .observe(&first.placement_constraint.envelope_id, T0 + 11)
            .unwrap();
        assert_eq!(witness.observed_state, ControlState::Superseded);
        let constraint = current_constraint(&first, &witness, T0 + 11);
        assert!(f1b_select_resource(
            &first.envelope_bytes,
            &constraint,
            &snapshot(T0 + 11),
            &witness,
            &f1b_scheduler_policy_identity(),
            &selector_identity('3')
        )
        .is_err());
    }

    #[test]
    fn expired_witness_blocks_new_selection() {
        let (issuer, bundle) = issue(OperationClass::History);
        let expiry = bundle.authorization.expires_at_unix_ms;
        let witness = issuer
            .registry()
            .observe(&bundle.placement_constraint.envelope_id, expiry)
            .unwrap();
        assert_eq!(witness.observed_state, ControlState::Expired);
        let constraint = current_constraint(&bundle, &witness, expiry);
        assert!(f1b_select_resource(
            &bundle.envelope_bytes,
            &constraint,
            &snapshot(expiry),
            &witness,
            &f1b_scheduler_policy_identity(),
            &selector_identity('3')
        )
        .is_err());
    }

    #[test]
    fn distinct_http_request_identity_does_not_collapse_replay() {
        let issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        let first = input(OperationClass::History);
        let mut second = first.clone();
        second.request_id = "req-002".into();
        let a = issuer.decide_operation(&first).unwrap();
        let b = issuer.decide_operation(&second).unwrap();
        assert_ne!(a.operation_admission_id, b.operation_admission_id);
    }

    #[test]
    fn cross_namespace_replay_changes_authorization_identity() {
        let issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        let first = input(OperationClass::History);
        let mut second = first.clone();
        second.namespace_ref = "other".into();
        let a = issuer.decide_operation(&first).unwrap();
        let b = issuer.decide_operation(&second).unwrap();
        assert_ne!(a.operation_admission_id, b.operation_admission_id);
    }

    #[test]
    fn duplicate_retry_cannot_extend_original_expiry() {
        let mut issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        let request = input(OperationClass::History);
        let first = issuer.issue_control_bundle(&request).unwrap();
        let second = issuer.issue_control_bundle(&request).unwrap();
        assert_eq!(
            first.authorization.expires_at_unix_ms,
            second.authorization.expires_at_unix_ms
        );
        let witness = issuer
            .registry()
            .observe(
                &first.placement_constraint.envelope_id,
                first.authorization.expires_at_unix_ms,
            )
            .unwrap();
        assert_eq!(witness.observed_state, ControlState::Expired);
    }

    #[test]
    fn e3_constraint_cannot_widen_e1_authority() {
        let (_issuer, bundle) = issue(OperationClass::History);
        let mut widened = bundle.placement_constraint.clone();
        widened.eligible_resource_classes =
            Some(vec![AETHER_LOCAL_BLOCKING_POOL_CLASS.into(), "gpu".into()]);
        assert!(validate_placement_constraint_narrowing(&bundle.envelope_bytes, &widened).is_err());
    }

    #[test]
    fn selector_implementation_identity_remains_in_decision_input() {
        let (_issuer, bundle) = issue(OperationClass::History);
        let snap = snapshot(T0);
        let policy = f1b_scheduler_policy_identity();
        let a = decision_input_digests(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &snap,
            &policy,
            &selector_identity('3'),
            &bundle.control_state_witness,
        )
        .unwrap();
        let b = decision_input_digests(
            &bundle.envelope_bytes,
            &bundle.placement_constraint,
            &snap,
            &policy,
            &selector_identity('4'),
            &bundle.control_state_witness,
        )
        .unwrap();
        assert_ne!(
            a.selector_implementation_digest,
            b.selector_implementation_digest
        );
        assert_ne!(a.decision_input_digest, b.decision_input_digest);
    }

    #[test]
    fn serialized_records_are_data_not_recreated_issuer_authority() {
        let (issuer, bundle) = issue(OperationClass::History);
        let bytes = serde_json::to_vec(&bundle.admission).unwrap();
        let reparsed: AetherOperationAdmissionDecision = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(reparsed, bundle.admission);
        assert_eq!(reparsed.authority_effect, "operation_attempt_only");
        assert_eq!(issuer.registry().len(), 1);
        // No constructor/deserializer exists for AetherMechanicalAuthorityIssuer or its
        // private AuthorityCapability; the compile-fail crate example enforces that boundary.
    }

    #[test]
    fn cross_process_integrity_profile_is_rejected() {
        let issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        assert!(issuer
            .validate_transport(AuthorityTransport::SameProcess)
            .is_ok());
        assert!(matches!(
            issuer.validate_transport(AuthorityTransport::CrossProcess),
            Err(AuthorityIssuanceError::CrossProcessUnauthorized)
        ));
    }

    #[test]
    fn c1_has_no_live_routing_or_configuration_switch() {
        assert!(!c1_live_routing_available());
    }

    #[test]
    fn changed_path_or_scope_cannot_reuse_operation_profile() {
        let issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        let mut request = input(OperationClass::History);
        request.http_path = "/v1/history/page".into();
        assert!(matches!(
            issuer.decide_operation(&request),
            Err(AuthorityIssuanceError::OperationProfileMismatch)
        ));
        let mut request = input(OperationClass::History);
        request.required_scope = "query".into();
        assert!(matches!(
            issuer.decide_operation(&request),
            Err(AuthorityIssuanceError::OperationProfileMismatch)
        ));
    }

    #[test]
    fn envelope_profile_is_exactly_local_queue_pre_start_one_attempt() {
        let (_issuer, bundle) = issue(OperationClass::History);
        let envelope = parse_json_no_duplicates(&bundle.envelope_bytes).unwrap();
        assert_eq!(envelope["permitted_actions"], json!(["queue_pre_start"]));
        assert_eq!(
            envelope["eligible_resource_classes"],
            json!(["local-blocking-pool"])
        );
        assert_eq!(envelope["trust_zones"], json!(["aether-process"]));
        assert_eq!(envelope["locality_constraints"], json!(["local-process"]));
        assert_eq!(
            envelope["required_capabilities"],
            json!(["blocking_execution"])
        );
        assert_eq!(envelope["retry_policy"]["max_attempts"], json!(1));
        assert_eq!(
            envelope["redundancy_policy"]["max_parallel_copies"],
            json!(1)
        );
        assert_eq!(
            envelope["integrity_profile_ref"],
            json!(C1_INTEGRITY_PROFILE)
        );
    }

    #[test]
    fn registry_is_bounded_and_exact_duplicates_do_not_consume_capacity() {
        let mut issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 1);
        let first_input = input(OperationClass::History);
        let first = issuer.issue_control_bundle(&first_input).unwrap();
        let duplicate = issuer.issue_control_bundle(&first_input).unwrap();
        assert_eq!(first, duplicate);
        assert_eq!(issuer.registry().len(), 1);

        let mut second = input(OperationClass::CurrentState);
        second.request_id = "req-second".into();
        second.correlation_id = "corr-second".into();
        assert!(matches!(
            issuer.issue_control_bundle(&second),
            Err(AuthorityIssuanceError::RegistryFull)
        ));
    }

    #[test]
    fn revocation_is_idempotent_only_for_identical_transition() {
        let (mut issuer, bundle) = issue(OperationClass::History);
        let id = bundle.placement_constraint.envelope_id;
        issuer.revoke_envelope(&id, T0 + 1, "same").unwrap();
        issuer.revoke_envelope(&id, T0 + 1, "same").unwrap();
        assert!(matches!(
            issuer.revoke_envelope(&id, T0 + 2, "different"),
            Err(AuthorityIssuanceError::InvalidControlTransition(_))
        ));
    }

    #[test]
    fn witness_cannot_predate_authorization() {
        let (issuer, bundle) = issue(OperationClass::History);
        assert!(matches!(
            issuer
                .registry()
                .observe(&bundle.placement_constraint.envelope_id, T0 - 1),
            Err(AuthorityIssuanceError::InvalidControlTransition(_))
        ));
    }

    #[test]
    fn expired_envelope_cannot_be_revoked_or_superseded() {
        let (mut issuer, bundle) = issue(OperationClass::History);
        let id = bundle.placement_constraint.envelope_id.clone();
        let expiry = bundle.authorization.expires_at_unix_ms;

        assert!(matches!(
            issuer.revoke_envelope(&id, expiry, "too late"),
            Err(AuthorityIssuanceError::InvalidControlTransition(_))
        ));

        let mut replacement = input(OperationClass::History);
        replacement.request_id = "req-after-expiry".into();
        replacement.correlation_id = "corr-after-expiry".into();
        replacement.decided_at_unix_ms = expiry;
        assert!(matches!(
            issuer.issue_superseding_control_bundle(&id, &replacement),
            Err(AuthorityIssuanceError::InvalidControlTransition(_))
        ));

        let witness = issuer.registry().observe(&id, expiry).unwrap();
        assert_eq!(witness.observed_state, ControlState::Expired);
    }

    #[test]
    fn rfc3339_expiry_is_bound_to_operation_timeout() {
        let mut issuer = AetherMechanicalAuthorityIssuer::for_tests(5_000, 8);
        let bundle = issuer
            .issue_control_bundle(&input(OperationClass::History))
            .unwrap();
        assert_eq!(bundle.authorization.expires_at_unix_ms, T0 + 5_000);
        let envelope = parse_json_no_duplicates(&bundle.envelope_bytes).unwrap();
        assert_eq!(
            envelope["expires_at"].as_str().unwrap(),
            bundle.authorization.expires_at
        );
        assert!(bundle.authorization.expires_at.ends_with('Z'));
    }
}
