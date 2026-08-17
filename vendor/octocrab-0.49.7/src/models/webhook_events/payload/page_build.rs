use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PageBuildWebhookEventPayload {
    pub build: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
    pub id: u64,
}
