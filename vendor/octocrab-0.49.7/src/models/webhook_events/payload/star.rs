use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StarWebhookEventPayload {
    pub action: StarWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub starred_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StarWebhookEventAction {
    Created,
    Deleted,
}
