use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TeamAddWebhookEventPayload {
    pub enterprise: Option<serde_json::Value>,
    pub team: serde_json::Value,
}
