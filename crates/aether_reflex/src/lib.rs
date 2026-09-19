//! Bounded probabilistic reflex decisions for AETHER.
//!
//! This crate is deliberately non-authoritative. It validates a closed-world
//! probabilistic judgment, binds it to an exact semantic cut, and applies a
//! deterministic gate. It does not append datoms, grant capabilities, or
//! execute actions.

use aether_ast::FederatedCut;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const NORMALIZATION_TOLERANCE: f64 = 1.0e-6;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecisionSchema {
    pub question_id: String,
    pub version: String,
    pub digest: String,
    pub choices: Vec<String>,
}

impl DecisionSchema {
    pub fn validate(&self) -> Result<(), ReflexError> {
        if self.question_id.trim().is_empty() {
            return Err(ReflexError::EmptyQuestionId);
        }
        if self.version.trim().is_empty() {
            return Err(ReflexError::EmptySchemaVersion);
        }
        if !is_sha256_ref(&self.digest) {
            return Err(ReflexError::InvalidDigest("decision_schema_digest"));
        }
        if self.choices.len() < 2 {
            return Err(ReflexError::InsufficientChoices);
        }

        let mut seen = BTreeSet::new();
        for choice in &self.choices {
            if choice.trim().is_empty() {
                return Err(ReflexError::EmptyChoice);
            }
            if !seen.insert(choice.clone()) {
                return Err(ReflexError::DuplicateChoice(choice.clone()));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateProjectionRef {
    pub projection_id: String,
    pub cut: FederatedCut,
    pub content_digest: String,
    pub policy_digest: String,
}

impl StateProjectionRef {
    pub fn validate(&self) -> Result<(), ReflexError> {
        if self.projection_id.trim().is_empty() {
            return Err(ReflexError::EmptyProjectionId);
        }
        if !is_sha256_ref(&self.content_digest) {
            return Err(ReflexError::InvalidDigest("content_digest"));
        }
        if !is_sha256_ref(&self.policy_digest) {
            return Err(ReflexError::InvalidDigest("policy_digest"));
        }
        validate_exact_cut(&self.cut)

    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderModelRef {
    pub provider: String,
    pub model: String,
    pub revision: String,
}

impl ProviderModelRef {
    pub fn validate(&self) -> Result<(), ReflexError> {
        if self.provider.trim().is_empty() {
            return Err(ReflexError::EmptyProviderField("provider"));
        }
        if self.model.trim().is_empty() {
            return Err(ReflexError::EmptyProviderField("model"));
        }
        if self.revision.trim().is_empty() {
            return Err(ReflexError::EmptyProviderField("revision"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecisionRequest {
    pub decision_id: String,
    pub schema: DecisionSchema,
    pub projection: StateProjectionRef,
}

impl DecisionRequest {
    pub fn validate(&self) -> Result<(), ReflexError> {
        if self.decision_id.trim().is_empty() {
            return Err(ReflexError::EmptyDecisionId);
        }
        self.schema.validate()?;
        self.projection.validate()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChoiceProbability {
    pub choice: String,
    pub probability: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProbabilisticDecision {
    pub decision_id: String,
    pub model: ProviderModelRef,
    pub distribution: Vec<ChoiceProbability>,
}

impl ProbabilisticDecision {
    pub fn validate_against(&self, request: &DecisionRequest) -> Result<(), ReflexError> {
        request.validate()?;
        self.model.validate()?;

        if self.decision_id != request.decision_id {
            return Err(ReflexError::DecisionIdMismatch);
        }
        if self.distribution.len() != request.schema.choices.len() {
            return Err(ReflexError::DistributionCardinality {
                expected: request.schema.choices.len(),
                actual: self.distribution.len(),
            });
        }

        let allowed = request
            .schema
            .choices
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let mut seen = BTreeSet::new();
        let mut sum = 0.0;

        for item in &self.distribution {
            if !allowed.contains(item.choice.as_str()) {
                return Err(ReflexError::UnknownChoice(item.choice.clone()));
            }
            if !seen.insert(item.choice.clone()) {
                return Err(ReflexError::DuplicateDistributionChoice(item.choice.clone()));
            }
            if !item.probability.is_finite()
                || item.probability < 0.0
                || item.probability > 1.0
            {
                return Err(ReflexError::InvalidProbability {
                    choice: item.choice.clone(),
                    value: item.probability,
                });
            }
            sum += item.probability;
        }

        for choice in &request.schema.choices {
            if !seen.contains(choice) {
                return Err(ReflexError::MissingChoice(choice.clone()));
            }
        }
        if (sum - 1.0).abs() > NORMALIZATION_TOLERANCE {
            return Err(ReflexError::ProbabilityMass(sum));
        }
        Ok(())
    }
}

pub trait ReflexProvider {
    fn evaluate(&self, request: &DecisionRequest) -> Result<ProbabilisticDecision, ReflexError>;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReflexPolicy {
    pub policy_ref: String,
    pub min_act_probability: f64,
    pub min_top_two_margin: f64,
}

impl ReflexPolicy {
    pub fn validate(&self) -> Result<(), ReflexError> {
        if self.policy_ref.trim().is_empty() {
            return Err(ReflexError::InvalidPolicy("policy_ref"));
        }
        validate_unit_interval(self.min_act_probability, "min_act_probability")?;
        validate_unit_interval(self.min_top_two_margin, "min_top_two_margin")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorityGrant {
    pub grant_ref: String,
    pub authority_digest: String,
    pub authority_cut: FederatedCut,
    pub principal_ref: String,
    pub capability_ref: String,
    pub authorized_choices: Vec<String>,
}

impl AuthorityGrant {
    pub fn validate(&self) -> Result<(), ReflexError> {
        if self.grant_ref.trim().is_empty() {
            return Err(ReflexError::InvalidAuthority("grant_ref"));
        }
        if !is_sha256_ref(&self.authority_digest) {
            return Err(ReflexError::InvalidDigest("authority_digest"));
        }
        validate_exact_cut(&self.authority_cut)?;
        if self.principal_ref.trim().is_empty() {
            return Err(ReflexError::InvalidAuthority("principal_ref"));
        }
        if self.capability_ref.trim().is_empty() {
            return Err(ReflexError::InvalidAuthority("capability_ref"));
        }
        let mut seen = BTreeSet::new();
        for choice in &self.authorized_choices {
            if choice.trim().is_empty() {
                return Err(ReflexError::InvalidAuthority("authorized_choices"));
            }
            if !seen.insert(choice) {
                return Err(ReflexError::InvalidAuthority("duplicate authorized choice"));
            }
        }
        Ok(())
    }

    pub fn allows(&self, choice: &str) -> bool {
        self.authorized_choices.iter().any(|allowed| allowed == choice)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateOutcome {
    Act,
    Deliberate,
    Escalate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateReason {
    AuthorizedConfidence,
    ProbabilityBelowThreshold,
    MarginBelowThreshold,
    AuthorityMissing,
    ChoiceNotAuthorized,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GateResult {
    pub outcome: GateOutcome,
    pub reason: GateReason,
    pub selected_choice: String,
    pub selected_probability: f64,
    pub runner_up_probability: f64,
}

pub fn gate_decision(
    request: &DecisionRequest,
    decision: &ProbabilisticDecision,
    policy: &ReflexPolicy,
    authority: Option<&AuthorityGrant>,
) -> Result<GateResult, ReflexError> {
    policy.validate()?;
    decision.validate_against(request)?;
    if let Some(authority) = authority {
        authority.validate()?;
        if authority.authority_cut.clone().normalized()
            != request.projection.cut.clone().normalized()
        {
            return Err(ReflexError::AuthorityCutMismatch);
        }
        if authority
            .authorized_choices
            .iter()
            .any(|choice| !request.schema.choices.contains(choice))
        {
            return Err(ReflexError::AuthorityChoiceOutsideSchema);
        }
    }

    let (choice, top, runner_up) = top_two(request, decision)?;

    if top < policy.min_act_probability {
        return Ok(GateResult {
            outcome: GateOutcome::Deliberate,
            reason: GateReason::ProbabilityBelowThreshold,
            selected_choice: choice,
            selected_probability: top,
            runner_up_probability: runner_up,
        });
    }
    if top - runner_up < policy.min_top_two_margin {
        return Ok(GateResult {
            outcome: GateOutcome::Deliberate,
            reason: GateReason::MarginBelowThreshold,
            selected_choice: choice,
            selected_probability: top,
            runner_up_probability: runner_up,
        });
    }

    match authority {
        None => Ok(GateResult {
            outcome: GateOutcome::Escalate,
            reason: GateReason::AuthorityMissing,
            selected_choice: choice,
            selected_probability: top,
            runner_up_probability: runner_up,
        }),
        Some(authority) if !authority.allows(&choice) => Ok(GateResult {
            outcome: GateOutcome::Escalate,
            reason: GateReason::ChoiceNotAuthorized,
            selected_choice: choice,
            selected_probability: top,
            runner_up_probability: runner_up,
        }),
        Some(_) => Ok(GateResult {
            outcome: GateOutcome::Act,
            reason: GateReason::AuthorizedConfidence,
            selected_choice: choice,
            selected_probability: top,
            runner_up_probability: runner_up,
        }),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityEffect {
    None,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorityReceipt {
    pub grant_ref: String,
    pub authority_digest: String,
    pub authority_cut: FederatedCut,
    pub principal_ref: String,
    pub capability_ref: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecisionReceipt {
    pub schema_version: String,
    pub decision_id: String,
    pub question_id: String,
    pub question_schema_version: String,
    pub question_schema_digest: String,
    pub projection: StateProjectionRef,
    pub model: ProviderModelRef,
    pub distribution: Vec<ChoiceProbability>,
    pub policy_ref: String,
    pub gate: GateResult,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<AuthorityReceipt>,
    pub authority_effect: AuthorityEffect,
}

impl DecisionReceipt {
    pub fn build(
        request: &DecisionRequest,
        decision: &ProbabilisticDecision,
        policy: &ReflexPolicy,
        authority: Option<&AuthorityGrant>,
    ) -> Result<Self, ReflexError> {
        let gate = gate_decision(request, decision, policy, authority)?;
        Ok(Self {
            schema_version: "aether/reflex-decision-receipt/1".to_string(),
            decision_id: request.decision_id.clone(),
            question_id: request.schema.question_id.clone(),
            question_schema_version: request.schema.version.clone(),
            question_schema_digest: request.schema.digest.clone(),
            projection: request.projection.clone(),
            model: decision.model.clone(),
            distribution: decision.distribution.clone(),
            policy_ref: policy.policy_ref.clone(),
            gate,
            authority: authority.map(|grant| AuthorityReceipt {
                grant_ref: grant.grant_ref.clone(),
                authority_digest: grant.authority_digest.clone(),
                authority_cut: grant.authority_cut.clone(),
                principal_ref: grant.principal_ref.clone(),
                capability_ref: grant.capability_ref.clone(),
            }),
            authority_effect: AuthorityEffect::None,
        })
    }
}

fn top_two(
    request: &DecisionRequest,
    decision: &ProbabilisticDecision,
) -> Result<(String, f64, f64), ReflexError> {
    let scores = decision
        .distribution
        .iter()
        .map(|item| (item.choice.as_str(), item.probability))
        .collect::<BTreeMap<_, _>>();

    let mut top_choice = None;
    let mut top = -1.0;
    let mut runner_up = -1.0;

    for choice in &request.schema.choices {
        let probability = *scores
            .get(choice.as_str())
            .ok_or_else(|| ReflexError::MissingChoice(choice.clone()))?;
        if probability > top {
            runner_up = top;
            top = probability;
            top_choice = Some(choice.clone());
        } else if probability > runner_up {
            runner_up = probability;
        }
    }

    Ok((
        top_choice.ok_or(ReflexError::InsufficientChoices)?,
        top,
        runner_up.max(0.0),
    ))
}

fn validate_exact_cut(cutset: &FederatedCut) -> Result<(), ReflexError> {
    if cutset.cuts.is_empty() {
        return Err(ReflexError::EmptyCut);
    }
    let mut partitions = BTreeSet::new();
    for cut in &cutset.cuts {
        if cut.partition.as_str().trim().is_empty() {
            return Err(ReflexError::EmptyPartition);
        }
        if cut.as_of.is_none() {
            return Err(ReflexError::NonExactCut(cut.partition.to_string()));
        }
        if !partitions.insert(cut.partition.to_string()) {
            return Err(ReflexError::DuplicatePartition(cut.partition.to_string()));
        }
    }
    Ok(())
}

fn validate_unit_interval(value: f64, field: &'static str) -> Result<(), ReflexError> {
    if value.is_finite() && (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(ReflexError::InvalidPolicy(field))
    }
}

fn is_sha256_ref(value: &str) -> bool {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return false;
    };
    hex.len() == 64 && hex.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[derive(Debug, Error, PartialEq)]
pub enum ReflexError {
    #[error("decision id must not be empty")]
    EmptyDecisionId,
    #[error("question id must not be empty")]
    EmptyQuestionId,
    #[error("decision schema version must not be empty")]
    EmptySchemaVersion,
    #[error("a closed decision schema requires at least two choices")]
    InsufficientChoices,
    #[error("decision choice must not be empty")]
    EmptyChoice,
    #[error("duplicate decision choice: {0}")]
    DuplicateChoice(String),
    #[error("projection id must not be empty")]
    EmptyProjectionId,
    #[error("projection cut must contain at least one partition")]
    EmptyCut,
    #[error("partition id must not be empty")]
    EmptyPartition,
    #[error("projection cut for partition {0} is not exact")]
    NonExactCut(String),
    #[error("duplicate partition in projection cut: {0}")]
    DuplicatePartition(String),
    #[error("invalid SHA-256 reference in {0}")]
    InvalidDigest(&'static str),
    #[error("provider field {0} must not be empty")]
    EmptyProviderField(&'static str),
    #[error("provider response decision id does not match request")]
    DecisionIdMismatch,
    #[error("distribution cardinality mismatch: expected {expected}, got {actual}")]
    DistributionCardinality { expected: usize, actual: usize },
    #[error("distribution contains unknown choice: {0}")]
    UnknownChoice(String),
    #[error("distribution contains duplicate choice: {0}")]
    DuplicateDistributionChoice(String),
    #[error("distribution is missing choice: {0}")]
    MissingChoice(String),
    #[error("invalid probability for {choice}: {value}")]
    InvalidProbability { choice: String, value: f64 },
    #[error("probability mass must sum to one; got {0}")]
    ProbabilityMass(f64),
    #[error("invalid reflex policy field: {0}")]
    InvalidPolicy(&'static str),
    #[error("invalid authority grant field: {0}")]
    InvalidAuthority(&'static str),
    #[error("authority grant is bound to a different semantic cut")]
    AuthorityCutMismatch,
    #[error("authority grant contains a choice outside the decision schema")]
    AuthorityChoiceOutsideSchema,
    #[error("provider failure: {0}")]
    Provider(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use aether_ast::{ElementId, PartitionCut};

    fn digest(ch: char) -> String {
        format!("sha256:{}", ch.to_string().repeat(64))
    }

    fn schema() -> DecisionSchema {
        DecisionSchema {
            question_id: "route.safe".to_string(),
            version: "1".to_string(),
            digest: digest('c'),
            choices: vec!["yes".to_string(), "no".to_string()],
        }
    }

    fn request() -> DecisionRequest {
        DecisionRequest {
            decision_id: "d-1".to_string(),
            schema: schema(),
            projection: StateProjectionRef {
                projection_id: "p-1".to_string(),
                cut: FederatedCut {
                    cuts: vec![PartitionCut::as_of("local", ElementId::new(42))],
                },
                content_digest: digest('a'),
                policy_digest: digest('b'),
            },
        }
    }

    fn decision(yes: f64, no: f64) -> ProbabilisticDecision {
        ProbabilisticDecision {
            decision_id: "d-1".to_string(),
            model: ProviderModelRef {
                provider: "jev".to_string(),
                model: "system-one".to_string(),
                revision: "example".to_string(),
            },
            distribution: vec![
                ChoiceProbability {
                    choice: "yes".to_string(),
                    probability: yes,
                },
                ChoiceProbability {
                    choice: "no".to_string(),
                    probability: no,
                },
            ],
        }
    }

    fn policy() -> ReflexPolicy {
        ReflexPolicy {
            policy_ref: "policy/reflex/test".to_string(),
            min_act_probability: 0.90,
            min_top_two_margin: 0.50,
        }
    }

    fn authority() -> AuthorityGrant {
        AuthorityGrant {
            grant_ref: "grant/1".to_string(),
            authority_digest: digest('d'),
            authority_cut: request().projection.cut,
            principal_ref: "principal/operator".to_string(),
            capability_ref: "capability/route".to_string(),
            authorized_choices: vec!["yes".to_string()],
        }
    }

    #[test]
    fn exact_cut_is_required() {
        let mut request = request();
        request.projection.cut.cuts[0] = PartitionCut::current("local");
        assert_eq!(
            request.validate(),
            Err(ReflexError::NonExactCut("local".to_string()))
        );
    }

    #[test]
    fn malformed_probability_mass_fails_closed() {
        let result = decision(0.80, 0.10).validate_against(&request());
        assert!(matches!(result, Err(ReflexError::ProbabilityMass(_))));
    }

    #[test]
    fn unknown_choice_fails_closed() {
        let mut decision = decision(0.95, 0.05);
        decision.distribution[1].choice = "maybe".to_string();
        assert_eq!(
            decision.validate_against(&request()),
            Err(ReflexError::UnknownChoice("maybe".to_string()))
        );
    }

    #[test]
    fn high_confidence_without_authority_escalates() {
        let result = gate_decision(&request(), &decision(0.95, 0.05), &policy(), None).unwrap();
        assert_eq!(result.outcome, GateOutcome::Escalate);
        assert_eq!(result.reason, GateReason::AuthorityMissing);
    }

    #[test]
    fn high_confidence_with_choice_authority_may_act() {
        let result = gate_decision(
            &request(),
            &decision(0.95, 0.05),
            &policy(),
            Some(&authority()),
        )
        .unwrap();
        assert_eq!(result.outcome, GateOutcome::Act);
        assert_eq!(result.reason, GateReason::AuthorizedConfidence);
    }

    #[test]
    fn authority_from_a_different_cut_fails_closed() {
        let mut grant = authority();
        grant.authority_cut.cuts[0] =
            PartitionCut::as_of("local", ElementId::new(41));
        assert_eq!(
            gate_decision(
                &request(),
                &decision(0.95, 0.05),
                &policy(),
                Some(&grant),
            ),
            Err(ReflexError::AuthorityCutMismatch)
        );
    }

    #[test]
    fn authority_choice_outside_schema_fails_closed() {
        let mut grant = authority();
        grant.authorized_choices.push("maybe".to_string());
        assert_eq!(
            gate_decision(
                &request(),
                &decision(0.95, 0.05),
                &policy(),
                Some(&grant),
            ),
            Err(ReflexError::AuthorityChoiceOutsideSchema)
        );
    }

    #[test]
    fn high_confidence_cannot_launder_wrong_choice_authority() {
        let mut grant = authority();
        grant.authorized_choices = vec!["no".to_string()];
        let result = gate_decision(
            &request(),
            &decision(0.95, 0.05),
            &policy(),
            Some(&grant),
        )
        .unwrap();
        assert_eq!(result.outcome, GateOutcome::Escalate);
        assert_eq!(result.reason, GateReason::ChoiceNotAuthorized);
    }

    #[test]
    fn low_margin_deliberates() {
        let mut policy = policy();
        policy.min_act_probability = 0.50;
        policy.min_top_two_margin = 0.10;
        let result = gate_decision(
            &request(),
            &decision(0.51, 0.49),
            &policy,
            Some(&authority()),
        )
        .unwrap();
        assert_eq!(result.outcome, GateOutcome::Deliberate);
        assert_eq!(result.reason, GateReason::MarginBelowThreshold);
    }

    #[test]
    fn receipt_has_no_authority_effect_even_when_gate_allows_act() {
        let receipt = DecisionReceipt::build(
            &request(),
            &decision(0.95, 0.05),
            &policy(),
            Some(&authority()),
        )
        .unwrap();
        assert_eq!(receipt.authority_effect, AuthorityEffect::None);
        let encoded = serde_json::to_value(receipt).unwrap();
        assert_eq!(encoded["authority_effect"], "none");
        assert_eq!(encoded["gate"]["outcome"], "act");
    }
}
