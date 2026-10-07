use serde::de::{Error as DeError, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest as ShaDigest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use thiserror::Error;

mod realization;
mod selector;
pub use realization::*;
pub use selector::*;

pub const E1_PROTOCOL_FAMILY: &str = "aether-fabric";
pub const E1_PROTOCOL_MAJOR: u32 = 1;
pub const E1_PROTOCOL_MINOR: u32 = 0;
pub const E3_PROTOCOL_MAJOR: u32 = 1;
pub const E3_PROTOCOL_MINOR: u32 = 1;
pub const CANONICALIZATION_VERSION: &str = "gcl-cjson-set-v1";

pub const AETHER_LOCAL_BLOCKING_POOL_ID: &str = "aether-local-blocking-pool";
pub const AETHER_LOCAL_BLOCKING_POOL_CLASS: &str = "local-blocking-pool";
pub const AETHER_LOCAL_TRUST_ZONE: &str = "aether-process";
pub const AETHER_BLOCKING_CAPABILITY: &str = "blocking_execution";
pub const CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT: &str = "blocking_admission_slot";

#[derive(Debug, Error)]
pub enum FabricContractError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("duplicate JSON key: {0}")]
    DuplicateKey(String),
    #[error("invalid protocol: {0}")]
    Protocol(String),
    #[error("digest mismatch for {0}")]
    DigestMismatch(&'static str),
    #[error("invalid E1 envelope: {0}")]
    Envelope(String),
    #[error("placement constraint widens E1 authority: {0}")]
    Widening(String),
    #[error("invalid resource snapshot: {0}")]
    Snapshot(String),
    #[error("invalid control-state witness: {0}")]
    ControlState(String),
    #[error("invalid selector identity: {0}")]
    SelectorIdentity(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub capacity_unit: String,
    pub minimum_capacity_units: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlacementConstraintSet {
    pub schema_version: String,
    pub protocol_family: String,
    pub protocol_major: u32,
    pub protocol_minor: u32,
    pub record_type: String,
    pub constraint_set_id: String,
    pub envelope_id: String,
    pub envelope_digest: String,
    pub correlation_id: String,
    pub resource_requirements: ResourceRequirements,
    pub max_snapshot_age_ms: u64,
    pub decision_time_unix_ms: u64,
    pub control_state_ref: String,
    pub control_state_digest: String,
    pub source_authority_ref: String,
    pub authority_effect: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eligible_resource_classes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_zones: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_capabilities: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality_constraints: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlState {
    Active,
    Revoked,
    Superseded,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ControlStateWitness {
    pub schema_version: String,
    pub protocol_family: String,
    pub protocol_major: u32,
    pub protocol_minor: u32,
    pub record_type: String,
    pub witness_id: String,
    pub envelope_id: String,
    pub envelope_digest: String,
    pub e1_expires_at: String,
    pub observed_state: ControlState,
    pub observed_revision: String,
    pub observed_at_unix_ms: u64,
    pub source_authority_ref: String,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceHealth {
    Eligible,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapacityObservation {
    pub unit: String,
    pub total: u64,
    pub available: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceObservation {
    pub resource_id: String,
    pub resource_class: String,
    pub capabilities: Vec<String>,
    pub trust_zone: String,
    pub health: ResourceHealth,
    pub available_capacity: CapacityObservation,
    pub queue_depth: u64,
    pub locality: Vec<String>,
    #[serde(default)]
    pub scheduler_metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    pub schema_version: String,
    pub protocol_family: String,
    pub protocol_major: u32,
    pub protocol_minor: u32,
    pub record_type: String,
    pub snapshot_id: String,
    pub snapshot_digest: String,
    pub observed_at_unix_ms: u64,
    pub observation_source: String,
    pub canonicalization_version: String,
    pub resources: Vec<ResourceObservation>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SchedulerPolicyIdentity {
    pub policy_id: String,
    pub artifact_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SelectorImplementationIdentity {
    pub selector_id: String,
    pub source_commit: String,
    pub source_tree: String,
    pub artifact_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlacementSelected {
    pub schema_version: String,
    pub protocol_family: String,
    pub protocol_major: u32,
    pub protocol_minor: u32,
    pub record_type: String,
    pub placement_decision_id: String,
    pub event_id: String,
    pub correlation_id: String,
    pub envelope_id: String,
    pub envelope_digest: String,
    pub placement_constraint_id: String,
    pub placement_constraint_digest: String,
    pub resource_snapshot_id: String,
    pub resource_snapshot_digest: String,
    pub scheduler_policy_id: String,
    pub scheduler_policy_digest: String,
    pub selector_implementation_digest: String,
    pub decision_time_unix_ms: u64,
    pub decision_input_digest: String,
    pub control_state_ref: String,
    pub control_state_digest: String,
    pub resource_id: String,
    pub eligibility_evidence: Vec<String>,
    pub selection_rank: u64,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlacementUnavailable {
    pub schema_version: String,
    pub protocol_family: String,
    pub protocol_major: u32,
    pub protocol_minor: u32,
    pub record_type: String,
    pub placement_decision_id: String,
    pub event_id: String,
    pub correlation_id: String,
    pub envelope_id: String,
    pub envelope_digest: String,
    pub placement_constraint_id: String,
    pub placement_constraint_digest: String,
    pub resource_snapshot_id: String,
    pub resource_snapshot_digest: String,
    pub scheduler_policy_id: String,
    pub scheduler_policy_digest: String,
    pub selector_implementation_digest: String,
    pub decision_time_unix_ms: u64,
    pub decision_input_digest: String,
    pub control_state_ref: String,
    pub control_state_digest: String,
    pub reason: String,
    pub authority_effect: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionInputDigests {
    pub envelope_digest: String,
    pub placement_constraint_digest: String,
    pub resource_snapshot_digest: String,
    pub scheduler_policy_digest: String,
    pub selector_implementation_digest: String,
    pub control_state_digest: String,
    pub decision_input_digest: String,
}

struct NoDuplicateValue(Value);

impl<'de> Deserialize<'de> for NoDuplicateValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NoDuplicateVisitor)
    }
}

struct NoDuplicateVisitor;

impl<'de> Visitor<'de> for NoDuplicateVisitor {
    type Value = NoDuplicateValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("valid JSON without duplicate object keys")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(NoDuplicateValue(Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(NoDuplicateValue(Value::Number(value.into())))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(NoDuplicateValue(Value::Number(value.into())))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let number = serde_json::Number::from_f64(value)
            .ok_or_else(|| E::custom("non-finite numbers are forbidden"))?;
        Ok(NoDuplicateValue(Value::Number(number)))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Ok(NoDuplicateValue(Value::String(value.to_owned())))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(NoDuplicateValue(Value::String(value)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(NoDuplicateValue(Value::Null))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(NoDuplicateValue(Value::Null))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        NoDuplicateValue::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(NoDuplicateValue(value)) = seq.next_element()? {
            values.push(value);
        }
        Ok(NoDuplicateValue(Value::Array(values)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut object = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if object.contains_key(&key) {
                return Err(A::Error::custom(format!("duplicate key: {key}")));
            }
            let NoDuplicateValue(value) = map.next_value()?;
            object.insert(key, value);
        }
        Ok(NoDuplicateValue(Value::Object(object)))
    }
}

pub fn parse_json_no_duplicates(bytes: &[u8]) -> Result<Value, FabricContractError> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = NoDuplicateValue::deserialize(&mut deserializer).map_err(|error| {
        let text = error.to_string();
        if let Some(key) = text.split("duplicate key: ").nth(1) {
            FabricContractError::DuplicateKey(
                key.split(" at line ").next().unwrap_or(key).to_owned(),
            )
        } else {
            FabricContractError::Json(error)
        }
    })?;
    deserializer.end()?;
    Ok(value.0)
}

pub fn canonicalize_json_bytes(bytes: &[u8]) -> Result<Vec<u8>, FabricContractError> {
    let mut value = parse_json_no_duplicates(bytes)?;
    canonicalize_value(&mut value, None)?;
    Ok(serde_json::to_vec(&value)?)
}

pub fn canonicalize_serializable<T: Serialize>(value: &T) -> Result<Vec<u8>, FabricContractError> {
    let mut value = serde_json::to_value(value)?;
    canonicalize_value(&mut value, None)?;
    Ok(serde_json::to_vec(&value)?)
}

fn canonicalize_value(
    value: &mut Value,
    parent_key: Option<&str>,
) -> Result<(), FabricContractError> {
    match value {
        Value::Object(object) => {
            for (key, child) in object.iter_mut() {
                canonicalize_value(child, Some(key))?;
            }
        }
        Value::Array(values) => {
            for child in values.iter_mut() {
                canonicalize_value(child, None)?;
            }
            match parent_key {
                Some("resources") => {
                    values.sort_by(|left, right| {
                        left.get("resource_id")
                            .and_then(Value::as_str)
                            .cmp(&right.get("resource_id").and_then(Value::as_str))
                    });
                }
                Some(
                    "capabilities"
                    | "eligible_resource_classes"
                    | "trust_zones"
                    | "required_capabilities"
                    | "locality"
                    | "locality_constraints"
                    | "eligibility_evidence",
                ) => {
                    values.sort_by_key(|value| serde_json::to_string(value).unwrap_or_default());
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

fn bytes_to_lower_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        write!(&mut out, "{byte:02x}").expect("writing to String cannot fail");
    }
    out
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    bytes_to_lower_hex(Sha256::digest(bytes).as_slice())
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

fn require_sha256(value: &str, field: &'static str) -> Result<(), FabricContractError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(FabricContractError::SelectorIdentity(format!(
            "{field} must be a lowercase SHA-256 hex digest"
        )));
    }
    Ok(())
}

fn string_set(value: &Value, field: &str) -> Result<BTreeSet<String>, FabricContractError> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| FabricContractError::Envelope(format!("missing or invalid {field}")))?
        .iter()
        .map(|item| {
            item.as_str().map(str::to_owned).ok_or_else(|| {
                FabricContractError::Envelope(format!("{field} contains non-string"))
            })
        })
        .collect()
}

fn optional_set(values: &Option<Vec<String>>) -> Option<BTreeSet<String>> {
    values
        .as_ref()
        .map(|values| values.iter().cloned().collect())
}

fn validate_e3_header(
    family: &str,
    major: u32,
    minor: u32,
    schema_version: &str,
    record_type: &str,
    expected_record_type: &str,
) -> Result<(), FabricContractError> {
    if family != E1_PROTOCOL_FAMILY
        || major != E3_PROTOCOL_MAJOR
        || minor != E3_PROTOCOL_MINOR
        || schema_version != "1.1"
        || record_type != expected_record_type
    {
        return Err(FabricContractError::Protocol(format!(
            "expected aether-fabric/1.1 {expected_record_type}"
        )));
    }
    Ok(())
}

pub fn validate_placement_constraint_narrowing(
    e1_envelope_bytes: &[u8],
    constraint: &PlacementConstraintSet,
) -> Result<(), FabricContractError> {
    validate_e3_header(
        &constraint.protocol_family,
        constraint.protocol_major,
        constraint.protocol_minor,
        &constraint.schema_version,
        &constraint.record_type,
        "PlacementConstraintSet",
    )?;

    if constraint.authority_effect != "mechanical_narrowing_only" {
        return Err(FabricContractError::Widening(
            "authority_effect must be mechanical_narrowing_only".into(),
        ));
    }
    if constraint.resource_requirements.capacity_unit != CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT
        || constraint.resource_requirements.minimum_capacity_units == 0
    {
        return Err(FabricContractError::Widening(
            "unsupported or zero resource requirement".into(),
        ));
    }

    let envelope = parse_json_no_duplicates(e1_envelope_bytes)?;
    if envelope.get("protocol_family").and_then(Value::as_str) != Some(E1_PROTOCOL_FAMILY)
        || envelope.get("protocol_major").and_then(Value::as_u64) != Some(E1_PROTOCOL_MAJOR as u64)
        || envelope.get("protocol_minor").and_then(Value::as_u64) != Some(E1_PROTOCOL_MINOR as u64)
        || envelope.get("schema_version").and_then(Value::as_str) != Some("1.0")
        || envelope.get("record_type").and_then(Value::as_str)
            != Some("MechanicalEnvelopeAuthorized")
    {
        return Err(FabricContractError::Envelope(
            "expected exact aether-fabric/1.0 MechanicalEnvelopeAuthorized".into(),
        ));
    }

    let envelope_id = envelope
        .get("envelope_id")
        .and_then(Value::as_str)
        .ok_or_else(|| FabricContractError::Envelope("missing envelope_id".into()))?;
    if envelope_id != constraint.envelope_id {
        return Err(FabricContractError::Envelope(
            "constraint references different envelope_id".into(),
        ));
    }
    let correlation_id = envelope
        .get("correlation_id")
        .and_then(Value::as_str)
        .ok_or_else(|| FabricContractError::Envelope("missing correlation_id".into()))?;
    if correlation_id != constraint.correlation_id {
        return Err(FabricContractError::Envelope(
            "constraint correlation_id mismatch".into(),
        ));
    }

    if sha256_hex(e1_envelope_bytes) != constraint.envelope_digest {
        return Err(FabricContractError::DigestMismatch("E1 envelope"));
    }

    let e1_resources = string_set(&envelope, "eligible_resource_classes")?;
    if let Some(e3_resources) = optional_set(&constraint.eligible_resource_classes) {
        if !e3_resources.is_subset(&e1_resources) {
            return Err(FabricContractError::Widening(
                "eligible_resource_classes not subset of E1".into(),
            ));
        }
    }

    let e1_zones = string_set(&envelope, "trust_zones")?;
    if let Some(e3_zones) = optional_set(&constraint.trust_zones) {
        if !e3_zones.is_subset(&e1_zones) {
            return Err(FabricContractError::Widening(
                "trust_zones not subset of E1".into(),
            ));
        }
    }

    let e1_capabilities = string_set(&envelope, "required_capabilities")?;
    if let Some(e3_capabilities) = optional_set(&constraint.required_capabilities) {
        if !e1_capabilities.is_subset(&e3_capabilities) {
            return Err(FabricContractError::Widening(
                "required_capabilities may only add requirements".into(),
            ));
        }
    }

    let e1_locality = string_set(&envelope, "locality_constraints")?;
    if let Some(e3_locality) = optional_set(&constraint.locality_constraints) {
        if !e1_locality.is_subset(&e3_locality) {
            return Err(FabricContractError::Widening(
                "locality_constraints may only add constraints".into(),
            ));
        }
    }

    Ok(())
}

impl ControlStateWitness {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, FabricContractError> {
        validate_e3_header(
            &self.protocol_family,
            self.protocol_major,
            self.protocol_minor,
            &self.schema_version,
            &self.record_type,
            "ControlStateWitness",
        )?;
        canonicalize_serializable(self)
    }

    pub fn digest(&self) -> Result<String, FabricContractError> {
        Ok(sha256_hex(&self.canonical_bytes()?))
    }
}

pub fn validate_control_state_witness(
    e1_envelope_bytes: &[u8],
    constraint: &PlacementConstraintSet,
    witness: &ControlStateWitness,
) -> Result<(), FabricContractError> {
    let envelope = parse_json_no_duplicates(e1_envelope_bytes)?;
    if witness.envelope_id != constraint.envelope_id
        || witness.envelope_id
            != envelope
                .get("envelope_id")
                .and_then(Value::as_str)
                .unwrap_or_default()
    {
        return Err(FabricContractError::ControlState(
            "witness envelope_id mismatch".into(),
        ));
    }
    if witness.envelope_digest != constraint.envelope_digest
        || witness.envelope_digest != sha256_hex(e1_envelope_bytes)
    {
        return Err(FabricContractError::DigestMismatch(
            "control witness envelope",
        ));
    }
    if witness.witness_id != constraint.control_state_ref {
        return Err(FabricContractError::ControlState(
            "control_state_ref does not name witness".into(),
        ));
    }
    if witness.digest()? != constraint.control_state_digest {
        return Err(FabricContractError::DigestMismatch("control state witness"));
    }
    if witness.e1_expires_at
        != envelope
            .get("expires_at")
            .and_then(Value::as_str)
            .unwrap_or_default()
    {
        return Err(FabricContractError::ControlState(
            "witness expiry projection differs from E1".into(),
        ));
    }
    if witness.authority_effect != "none" {
        return Err(FabricContractError::ControlState(
            "control witness must have authority_effect=none".into(),
        ));
    }
    if witness.source_authority_ref.is_empty() || witness.observed_revision.is_empty() {
        return Err(FabricContractError::ControlState(
            "control witness source/revision must be explicit".into(),
        ));
    }
    if witness.observed_state != ControlState::Active {
        return Err(FabricContractError::ControlState(format!(
            "envelope is not active: {:?}",
            witness.observed_state
        )));
    }
    if witness.observed_at_unix_ms != constraint.decision_time_unix_ms {
        return Err(FabricContractError::ControlState(
            "control witness must be observed at the exact decision time".into(),
        ));
    }
    Ok(())
}

impl ResourceSnapshot {
    pub fn new(
        snapshot_id: impl Into<String>,
        observed_at_unix_ms: u64,
        observation_source: impl Into<String>,
        resources: Vec<ResourceObservation>,
    ) -> Result<Self, FabricContractError> {
        let mut snapshot = Self {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: E3_PROTOCOL_MAJOR,
            protocol_minor: E3_PROTOCOL_MINOR,
            record_type: "ResourceSnapshot".into(),
            snapshot_id: snapshot_id.into(),
            snapshot_digest: String::new(),
            observed_at_unix_ms,
            observation_source: observation_source.into(),
            canonicalization_version: CANONICALIZATION_VERSION.into(),
            resources,
        };
        snapshot.validate_structure()?;
        snapshot.snapshot_digest = snapshot.compute_digest()?;
        Ok(snapshot)
    }

    fn digest_material(&self) -> Result<Vec<u8>, FabricContractError> {
        let mut copy = self.clone();
        copy.snapshot_digest.clear();
        canonicalize_serializable(&copy)
    }

    pub fn compute_digest(&self) -> Result<String, FabricContractError> {
        Ok(sha256_hex(&self.digest_material()?))
    }

    pub fn verify_digest(&self) -> Result<(), FabricContractError> {
        if self.snapshot_digest != self.compute_digest()? {
            return Err(FabricContractError::DigestMismatch("resource snapshot"));
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, FabricContractError> {
        self.validate_structure()?;
        self.verify_digest()?;
        canonicalize_serializable(self)
    }

    pub fn validate_structure(&self) -> Result<(), FabricContractError> {
        validate_e3_header(
            &self.protocol_family,
            self.protocol_major,
            self.protocol_minor,
            &self.schema_version,
            &self.record_type,
            "ResourceSnapshot",
        )?;
        if self.canonicalization_version != CANONICALIZATION_VERSION {
            return Err(FabricContractError::Snapshot(
                "unsupported canonicalization_version".into(),
            ));
        }
        let mut ids = BTreeSet::new();
        for resource in &self.resources {
            if !ids.insert(resource.resource_id.clone()) {
                return Err(FabricContractError::Snapshot(format!(
                    "duplicate resource_id {}",
                    resource.resource_id
                )));
            }
            if resource.available_capacity.unit != CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT {
                return Err(FabricContractError::Snapshot(
                    "unsupported capacity unit".into(),
                ));
            }
            if resource.available_capacity.available > resource.available_capacity.total {
                return Err(FabricContractError::Snapshot(
                    "available capacity exceeds total".into(),
                ));
            }
        }
        Ok(())
    }
}

pub fn validate_snapshot_freshness(
    snapshot: &ResourceSnapshot,
    constraint: &PlacementConstraintSet,
) -> Result<(), FabricContractError> {
    snapshot.validate_structure()?;
    snapshot.verify_digest()?;
    if snapshot.observed_at_unix_ms > constraint.decision_time_unix_ms {
        return Err(FabricContractError::Snapshot(
            "snapshot observed after decision time".into(),
        ));
    }
    let age = constraint
        .decision_time_unix_ms
        .saturating_sub(snapshot.observed_at_unix_ms);
    if age > constraint.max_snapshot_age_ms {
        return Err(FabricContractError::Snapshot(format!(
            "snapshot age {age} exceeds max {}",
            constraint.max_snapshot_age_ms
        )));
    }
    Ok(())
}

pub fn reference_resource_admissible(
    e1_envelope_bytes: &[u8],
    constraint: &PlacementConstraintSet,
    snapshot: &ResourceSnapshot,
    witness: &ControlStateWitness,
    resource_id: &str,
) -> Result<bool, FabricContractError> {
    validate_placement_constraint_narrowing(e1_envelope_bytes, constraint)?;
    validate_control_state_witness(e1_envelope_bytes, constraint, witness)?;
    validate_snapshot_freshness(snapshot, constraint)?;

    let envelope = parse_json_no_duplicates(e1_envelope_bytes)?;
    let resource = match snapshot
        .resources
        .iter()
        .find(|resource| resource.resource_id == resource_id)
    {
        Some(resource) => resource,
        None => return Ok(false),
    };

    if resource.health != ResourceHealth::Eligible {
        return Ok(false);
    }

    let e1_resource_classes = string_set(&envelope, "eligible_resource_classes")?;
    let effective_resource_classes =
        optional_set(&constraint.eligible_resource_classes).unwrap_or(e1_resource_classes);
    if !effective_resource_classes.contains(&resource.resource_class) {
        return Ok(false);
    }

    let e1_trust_zones = string_set(&envelope, "trust_zones")?;
    let effective_trust_zones = optional_set(&constraint.trust_zones).unwrap_or(e1_trust_zones);
    if !effective_trust_zones.contains(&resource.trust_zone) {
        return Ok(false);
    }

    let mut required_capabilities = string_set(&envelope, "required_capabilities")?;
    if let Some(extra) = optional_set(&constraint.required_capabilities) {
        required_capabilities.extend(extra);
    }
    let resource_capabilities: BTreeSet<_> = resource.capabilities.iter().cloned().collect();
    if !required_capabilities.is_subset(&resource_capabilities) {
        return Ok(false);
    }

    let mut required_locality = string_set(&envelope, "locality_constraints")?;
    if let Some(extra) = optional_set(&constraint.locality_constraints) {
        required_locality.extend(extra);
    }
    let resource_locality: BTreeSet<_> = resource.locality.iter().cloned().collect();
    if !required_locality.is_subset(&resource_locality) {
        return Ok(false);
    }

    if resource.available_capacity.unit != constraint.resource_requirements.capacity_unit
        || resource.available_capacity.available
            < constraint.resource_requirements.minimum_capacity_units
    {
        return Ok(false);
    }

    Ok(true)
}

impl SchedulerPolicyIdentity {
    pub fn validate(&self) -> Result<(), FabricContractError> {
        if self.policy_id.is_empty() {
            return Err(FabricContractError::SelectorIdentity(
                "policy_id is empty".into(),
            ));
        }
        require_sha256(&self.artifact_sha256, "policy artifact")
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, FabricContractError> {
        self.validate()?;
        canonicalize_serializable(self)
    }
}

impl SelectorImplementationIdentity {
    pub fn validate(&self) -> Result<(), FabricContractError> {
        if self.selector_id.is_empty()
            || self.source_commit.is_empty()
            || self.source_tree.is_empty()
        {
            return Err(FabricContractError::SelectorIdentity(
                "selector_id/source_commit/source_tree must be non-empty".into(),
            ));
        }
        require_sha256(&self.artifact_sha256, "selector artifact")
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, FabricContractError> {
        self.validate()?;
        canonicalize_serializable(self)
    }
}

impl PlacementConstraintSet {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, FabricContractError> {
        validate_e3_header(
            &self.protocol_family,
            self.protocol_major,
            self.protocol_minor,
            &self.schema_version,
            &self.record_type,
            "PlacementConstraintSet",
        )?;
        canonicalize_serializable(self)
    }

    pub fn digest(&self) -> Result<String, FabricContractError> {
        Ok(sha256_hex(&self.canonical_bytes()?))
    }
}

pub fn decision_input_digests(
    e1_envelope_bytes: &[u8],
    constraint: &PlacementConstraintSet,
    snapshot: &ResourceSnapshot,
    policy: &SchedulerPolicyIdentity,
    selector: &SelectorImplementationIdentity,
    witness: &ControlStateWitness,
) -> Result<DecisionInputDigests, FabricContractError> {
    validate_placement_constraint_narrowing(e1_envelope_bytes, constraint)?;
    validate_control_state_witness(e1_envelope_bytes, constraint, witness)?;
    validate_snapshot_freshness(snapshot, constraint)?;
    policy.validate()?;
    selector.validate()?;

    let constraint_bytes = constraint.canonical_bytes()?;
    let snapshot_bytes = snapshot.canonical_bytes()?;
    let policy_bytes = policy.canonical_bytes()?;
    let selector_bytes = selector.canonical_bytes()?;
    let witness_bytes = witness.canonical_bytes()?;

    let envelope_digest = sha256_hex(e1_envelope_bytes);
    let placement_constraint_digest = sha256_hex(&constraint_bytes);
    let resource_snapshot_digest = snapshot.snapshot_digest.clone();
    let scheduler_policy_digest = sha256_hex(&policy_bytes);
    let selector_implementation_digest = sha256_hex(&selector_bytes);
    let control_state_digest = sha256_hex(&witness_bytes);

    let decision_input_digest = framed_digest(&[
        ("e1-envelope", e1_envelope_bytes),
        ("placement-constraint", &constraint_bytes),
        ("resource-snapshot", &snapshot_bytes),
        ("scheduler-policy", &policy_bytes),
        ("selector-implementation", &selector_bytes),
        ("control-state-witness", &witness_bytes),
    ]);

    Ok(DecisionInputDigests {
        envelope_digest,
        placement_constraint_digest,
        resource_snapshot_digest,
        scheduler_policy_digest,
        selector_implementation_digest,
        control_state_digest,
        decision_input_digest,
    })
}

pub fn placement_decision_id(decision_input_digest: &str, selector_digest: &str) -> String {
    framed_digest(&[
        ("placement-decision", decision_input_digest.as_bytes()),
        ("selector", selector_digest.as_bytes()),
    ])
}

pub fn local_blocking_pool_observation(
    total_admission_slots: u64,
    admission_slots_available: u64,
    queue_depth: u64,
) -> ResourceObservation {
    ResourceObservation {
        resource_id: AETHER_LOCAL_BLOCKING_POOL_ID.into(),
        resource_class: AETHER_LOCAL_BLOCKING_POOL_CLASS.into(),
        capabilities: vec![AETHER_BLOCKING_CAPABILITY.into()],
        trust_zone: AETHER_LOCAL_TRUST_ZONE.into(),
        health: if admission_slots_available > 0 {
            ResourceHealth::Eligible
        } else {
            ResourceHealth::Unavailable
        },
        available_capacity: CapacityObservation {
            unit: CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT.into(),
            total: total_admission_slots,
            available: admission_slots_available,
        },
        queue_depth,
        locality: vec!["local-process".into()],
        scheduler_metadata: BTreeMap::from([
            (
                "surface".into(),
                "aether_http::BoundedBlockingExecutor".into(),
            ),
            ("identity_kind".into(), "resource_pool".into()),
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e1_envelope_bytes() -> Vec<u8> {
        br#"{
          "schema_version":"1.0",
          "protocol_family":"aether-fabric",
          "protocol_major":1,
          "protocol_minor":0,
          "record_type":"MechanicalEnvelopeAuthorized",
          "domain":"control_bridge",
          "authority_owner":"upstream_governed_record",
          "authority_effect":"mechanical_only",
          "envelope_id":"E1",
          "correlation_id":"C1",
          "authorization_ref":"A1",
          "issuer_ref":"I1",
          "scope_ref":"scope://one",
          "scope_digest":{"algorithm":"sha256","value":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"},
          "payload_ref":"payload://one",
          "permitted_actions":["dispatch_payload"],
          "eligible_resource_classes":["local-blocking-pool","gpu"],
          "trust_zones":["aether-process","managed"],
          "locality_constraints":["local-process"],
          "mechanical_policy_ref":"MP1",
          "priority_class":"normal",
          "fairness_policy_ref":"FP1",
          "retry_policy":{"max_attempts":1,"backoff_class":"none"},
          "redundancy_policy":{"max_parallel_copies":1},
          "expires_at":"2026-10-07T00:00:00Z",
          "required_capabilities":["blocking_execution"],
          "integrity_profile_ref":"IP1"
        }"#.to_vec()
    }

    fn witness(e1: &[u8]) -> ControlStateWitness {
        ControlStateWitness {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: 1,
            protocol_minor: 1,
            record_type: "ControlStateWitness".into(),
            witness_id: "W1".into(),
            envelope_id: "E1".into(),
            envelope_digest: sha256_hex(e1),
            e1_expires_at: "2026-10-07T00:00:00Z".into(),
            observed_state: ControlState::Active,
            observed_revision: "rev-1".into(),
            observed_at_unix_ms: 1_050,
            source_authority_ref: "AUTH".into(),
            authority_effect: "none".into(),
        }
    }

    fn constraint(e1: &[u8], witness: &ControlStateWitness) -> PlacementConstraintSet {
        PlacementConstraintSet {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: 1,
            protocol_minor: 1,
            record_type: "PlacementConstraintSet".into(),
            constraint_set_id: "PC1".into(),
            envelope_id: "E1".into(),
            envelope_digest: sha256_hex(e1),
            correlation_id: "C1".into(),
            resource_requirements: ResourceRequirements {
                capacity_unit: CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT.into(),
                minimum_capacity_units: 1,
            },
            max_snapshot_age_ms: 100,
            decision_time_unix_ms: 1_050,
            control_state_ref: witness.witness_id.clone(),
            control_state_digest: witness.digest().unwrap(),
            source_authority_ref: "AUTH".into(),
            authority_effect: "mechanical_narrowing_only".into(),
            eligible_resource_classes: Some(vec!["local-blocking-pool".into()]),
            trust_zones: Some(vec!["aether-process".into()]),
            required_capabilities: Some(vec!["blocking_execution".into()]),
            locality_constraints: Some(vec!["local-process".into(), "same-host".into()]),
        }
    }

    #[test]
    fn canonicalization_is_order_independent_for_resource_sets() {
        let left = ResourceSnapshot::new(
            "S1",
            1_000,
            "test",
            vec![
                ResourceObservation {
                    resource_id: "b".into(),
                    resource_class: "gpu".into(),
                    capabilities: vec!["z".into(), "a".into()],
                    trust_zone: "managed".into(),
                    health: ResourceHealth::Eligible,
                    available_capacity: CapacityObservation {
                        unit: CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT.into(),
                        total: 2,
                        available: 1,
                    },
                    queue_depth: 0,
                    locality: vec!["west".into(), "rack-1".into()],
                    scheduler_metadata: BTreeMap::new(),
                },
                local_blocking_pool_observation(8, 8, 0),
            ],
        )
        .unwrap();
        let right = ResourceSnapshot::new(
            "S1",
            1_000,
            "test",
            vec![
                local_blocking_pool_observation(8, 8, 0),
                ResourceObservation {
                    resource_id: "b".into(),
                    resource_class: "gpu".into(),
                    capabilities: vec!["a".into(), "z".into()],
                    trust_zone: "managed".into(),
                    health: ResourceHealth::Eligible,
                    available_capacity: CapacityObservation {
                        unit: CAPACITY_UNIT_BLOCKING_ADMISSION_SLOT.into(),
                        total: 2,
                        available: 1,
                    },
                    queue_depth: 0,
                    locality: vec!["rack-1".into(), "west".into()],
                    scheduler_metadata: BTreeMap::new(),
                },
            ],
        )
        .unwrap();
        assert_eq!(left.snapshot_digest, right.snapshot_digest);
        assert_eq!(
            left.canonical_bytes().unwrap(),
            right.canonical_bytes().unwrap()
        );
    }

    #[test]
    fn duplicate_json_keys_are_rejected() {
        let error = canonicalize_json_bytes(br#"{"a":1,"a":2}"#).unwrap_err();
        assert!(matches!(error, FabricContractError::DuplicateKey(_)));
    }

    #[test]
    fn widening_resource_class_is_rejected() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let mut constraint = constraint(&e1, &witness);
        constraint.eligible_resource_classes = Some(vec!["unknown".into()]);
        assert!(matches!(
            validate_placement_constraint_narrowing(&e1, &constraint),
            Err(FabricContractError::Widening(_))
        ));
    }

    #[test]
    fn removing_required_capability_is_widening() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let mut constraint = constraint(&e1, &witness);
        constraint.required_capabilities = Some(Vec::new());
        assert!(matches!(
            validate_placement_constraint_narrowing(&e1, &constraint),
            Err(FabricContractError::Widening(_))
        ));
    }

    #[test]
    fn stale_snapshot_fails_closed() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let snapshot = ResourceSnapshot::new(
            "S1",
            900,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();
        assert!(validate_snapshot_freshness(&snapshot, &constraint).is_err());
    }

    #[test]
    fn revoked_control_state_fails_closed() {
        let e1 = e1_envelope_bytes();
        let mut witness = witness(&e1);
        witness.observed_state = ControlState::Revoked;
        let constraint = constraint(&e1, &witness);
        assert!(validate_control_state_witness(&e1, &constraint, &witness).is_err());
    }

    #[test]
    fn stale_or_authoritative_control_witness_fails_closed() {
        let e1 = e1_envelope_bytes();

        let mut stale = witness(&e1);
        let stale_constraint = constraint(&e1, &stale);
        stale.observed_at_unix_ms = stale_constraint.decision_time_unix_ms - 1;
        assert!(validate_control_state_witness(&e1, &stale_constraint, &stale).is_err());

        let mut authoritative = witness(&e1);
        authoritative.authority_effect = "mechanical_only".into();
        let authoritative_constraint = constraint(&e1, &authoritative);
        assert!(
            validate_control_state_witness(&e1, &authoritative_constraint, &authoritative).is_err()
        );
    }

    #[test]
    fn selector_implementation_identity_changes_decision_input() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let constraint = constraint(&e1, &witness);
        let snapshot = ResourceSnapshot::new(
            "S1",
            1_000,
            "test",
            vec![local_blocking_pool_observation(8, 8, 0)],
        )
        .unwrap();
        let policy = SchedulerPolicyIdentity {
            policy_id: "lex-v1".into(),
            artifact_sha256: "1".repeat(64),
        };
        let selector_a = SelectorImplementationIdentity {
            selector_id: "selector".into(),
            source_commit: "commit-a".into(),
            source_tree: "tree-a".into(),
            artifact_sha256: "2".repeat(64),
        };
        let selector_b = SelectorImplementationIdentity {
            artifact_sha256: "3".repeat(64),
            ..selector_a.clone()
        };
        let a = decision_input_digests(&e1, &constraint, &snapshot, &policy, &selector_a, &witness)
            .unwrap();
        let b = decision_input_digests(&e1, &constraint, &snapshot, &policy, &selector_b, &witness)
            .unwrap();
        assert_ne!(a.decision_input_digest, b.decision_input_digest);
    }

    #[test]
    fn reference_admissibility_matches_local_blocking_pool_contract() {
        let e1 = e1_envelope_bytes();
        let witness = witness(&e1);
        let mut constraint = constraint(&e1, &witness);
        constraint.locality_constraints = Some(vec!["local-process".into()]);

        let available = ResourceSnapshot::new(
            "S-workers-busy-queue-open",
            1_000,
            "test",
            vec![local_blocking_pool_observation(72, 64, 0)],
        )
        .unwrap();
        assert!(reference_resource_admissible(
            &e1,
            &constraint,
            &available,
            &witness,
            AETHER_LOCAL_BLOCKING_POOL_ID,
        )
        .unwrap());

        let saturated = ResourceSnapshot::new(
            "S-saturated",
            1_000,
            "test",
            vec![local_blocking_pool_observation(72, 0, 64)],
        )
        .unwrap();
        assert!(!reference_resource_admissible(
            &e1,
            &constraint,
            &saturated,
            &witness,
            AETHER_LOCAL_BLOCKING_POOL_ID,
        )
        .unwrap());
    }

    #[test]
    fn placement_selected_is_pre_attempt_evidence() {
        let value = serde_json::to_value(PlacementSelected {
            schema_version: "1.1".into(),
            protocol_family: E1_PROTOCOL_FAMILY.into(),
            protocol_major: 1,
            protocol_minor: 1,
            record_type: "PlacementSelected".into(),
            placement_decision_id: "P1".into(),
            event_id: "EV1".into(),
            correlation_id: "C1".into(),
            envelope_id: "E1".into(),
            envelope_digest: "a".repeat(64),
            placement_constraint_id: "PC1".into(),
            placement_constraint_digest: "b".repeat(64),
            resource_snapshot_id: "S1".into(),
            resource_snapshot_digest: "c".repeat(64),
            scheduler_policy_id: "POL1".into(),
            scheduler_policy_digest: "d".repeat(64),
            selector_implementation_digest: "e".repeat(64),
            decision_time_unix_ms: 1_050,
            decision_input_digest: "f".repeat(64),
            control_state_ref: "W1".into(),
            control_state_digest: "0".repeat(64),
            resource_id: AETHER_LOCAL_BLOCKING_POOL_ID.into(),
            eligibility_evidence: vec!["eligible".into()],
            selection_rank: 1,
            authority_effect: "none".into(),
        })
        .unwrap();
        assert!(value.get("mechanical_attempt_id").is_none());
        assert_eq!(
            value.get("authority_effect").and_then(Value::as_str),
            Some("none")
        );
    }
}
