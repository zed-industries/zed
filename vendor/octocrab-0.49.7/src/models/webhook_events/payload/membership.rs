use serde::{Deserialize, Serialize};

use super::MembershipScope;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MembershipWebhookEventPayload {
    pub action: MembershipWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub member: serde_json::Value,
    pub scope: MembershipScope,
    pub team: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MembershipWebhookEventAction {
    Added,
    Removed,
}
