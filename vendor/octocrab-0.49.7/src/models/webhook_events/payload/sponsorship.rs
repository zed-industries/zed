use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SponsorshipWebhookEventPayload {
    pub action: SponsorshipWebhookEventAction,
    pub sponsorship: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
    pub changes: Option<serde_json::Value>,
    pub effective_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SponsorshipWebhookEventAction {
    Cancelled,
    Created,
    Edited,
    PendingCancellation,
    PendingTierChange,
    TierChanged,
}
