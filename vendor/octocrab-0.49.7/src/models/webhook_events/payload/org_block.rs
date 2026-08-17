use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgBlockWebhookEventPayload {
    pub action: OrgBlockWebhookEventAction,
    pub blocked_user: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OrgBlockWebhookEventAction {
    Blocked,
    Unblocked,
}
