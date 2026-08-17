use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrganizationWebhookEventPayload {
    pub action: OrganizationWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub membership: Option<serde_json::Value>,
    pub changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OrganizationWebhookEventAction {
    Deleted,
    MemberAdded,
    MemberInvited,
    MemberRemoved,
    Renamed,
}
