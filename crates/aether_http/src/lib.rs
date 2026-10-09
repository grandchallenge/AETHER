use aether_partition::*;
use aether_pilot::*;

pub mod admission {
    pub use aether_service_core::admission::*;
}
pub mod execution {
    pub use aether_service_core::execution::*;
}
pub mod sidecar {
    pub use aether_sidecar::*;
}

pub mod deployment;
mod fabric_c3_differential;
mod fabric_control_shadow;
mod fabric_cutover;
mod fabric_equivalence;
mod fabric_shadow;
pub mod http;
mod source_binding;
pub mod status;

pub use deployment::{
    default_audit_log_path, serve_pilot_http_service, DeploymentError, PilotAuthConfig,
    PilotConcurrencyConfig, PilotHttpTransportConfig, PilotServiceConfig, PilotStorageConfig,
    PilotTokenConfig, ResolvedPilotHttpTransport, ResolvedPilotServiceConfig, ResolvedPilotStorage,
    ResolvedPilotTokenSummary,
};
pub use fabric_c3_differential::{
    adjudicate_c3_first_lane_coverage, adjudicate_c3_observation, adjudicate_c3_replay_bundle,
    C3CoverageError, C3CoverageSummary, C3DifferentialEvidence, C3DifferentialVerdict,
    C3DisagreementClass, C3ReplayBundle, C3_DIFFERENTIAL_REVISION,
};
pub use fabric_control_shadow::{C2ShadowConfig, C2ShadowDisposition, C2ShadowEvidence};
pub use fabric_cutover::FabricCutoverError;
pub use fabric_equivalence::{
    adjudicate_f1e_live_equivalence, FabricDifferentialEquivalenceError,
    FabricDifferentialEquivalenceEvidence,
};
pub use fabric_shadow::FabricShadowPlacementComparison;
pub use http::{
    http_router, http_router_with_options, http_router_with_partitioned_options,
    http_router_with_postgres_namespaces, http_router_with_postgres_namespaces_and_tls,
    http_router_with_sqlite_namespaces, AuditContext, AuditEntry, AuditLogResponse, AuthScope,
    FabricRoutingMode, HealthResponse, HttpAccessToken, HttpAuthConfig, HttpKernelOptions,
    HttpKernelState, HttpResourceLimits, PageInfo, PageRequest, PagedHistoryResponse,
    PagedRunDocumentResponse, PagedTraceResponse, StructuredErrorResponse, AETHER_NAMESPACE_HEADER,
    AETHER_REQUEST_ID_HEADER,
};
pub use source_binding::{SourceBoundObservation, SourceBoundReadback};
pub use status::{
    AuthReloadResponse, NamespaceStatusSummary, PrincipalStatusSummary, ReplicaStatusSummary,
    ServiceMode, ServiceResourceControlStatus, ServiceStatusResponse, ServiceStatusStorage,
    ServiceTransportStatus,
};
