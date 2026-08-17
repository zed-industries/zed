use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{BranchProtectionRuleId, RepositoryId};

use super::OldValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BranchProtectionRuleWebhookEventPayload {
    pub action: BranchProtectionRuleWebhookEventAction,
    pub changes: Option<BranchProtectionRuleWebhookEventChanges>,
    pub enterprise: Option<serde_json::Value>,
    pub rule: BranchProtectionRule,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BranchProtectionRuleWebhookEventAction {
    Created,
    Deleted,
    Edited,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BranchProtectionRuleWebhookEventChanges {
    pub admin_enforced: Option<OldValue<Option<bool>>>,
    pub authorized_actor_names: Option<OldValue<Vec<String>>>,
    pub authorized_actors_only: Option<OldValue<Option<bool>>>,
    pub authorized_dismissal_actors_only: Option<OldValue<Option<bool>>>,
    pub linear_history_requirement_enforcement_level: Option<OldValue<BranchProtectionRuleLevel>>,
    pub required_status_checks: Option<OldValue<Vec<String>>>,
    pub required_status_checks_enforcement_level: Option<OldValue<BranchProtectionRuleLevel>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BranchProtectionRule {
    pub admin_enfored: bool,
    pub allow_deletions_enforcement_level: BranchProtectionRuleLevel,
    pub allow_force_pushes_enforcement_level: BranchProtectionRuleLevel,
    pub authorized_actor_names: Vec<String>,
    pub authorized_actors_only: bool,
    pub authorized_dismissal_actors_only: bool,
    pub create_protected: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub dismiss_stale_reviews_on_push: bool,
    pub id: BranchProtectionRuleId,
    pub ignore_approvals_from_contributors: bool,
    pub linear_history_requirement_enforcement_level: BranchProtectionRuleLevel,
    pub merge_queue_enforcement_level: BranchProtectionRuleLevel,
    pub name: String,
    pub pull_request_reviews_enforcement_level: BranchProtectionRuleLevel,
    pub repository_id: RepositoryId,
    pub require_code_owner_review: bool,
    pub require_last_push_approval: Option<bool>,
    pub required_approving_review_count: i64,
    pub required_conversation_resolution_level: BranchProtectionRuleLevel,
    pub required_deployments_enforcement_level: BranchProtectionRuleLevel,
    pub required_status_checks: Vec<String>,
    pub required_status_checks_enforcement_level: BranchProtectionRuleLevel,
    pub signature_requirement_enforcement_level: BranchProtectionRuleLevel,
    pub strict_required_status_checks_policy: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BranchProtectionRuleLevel {
    Off,
    NonAdmins,
    Everyone,
}
