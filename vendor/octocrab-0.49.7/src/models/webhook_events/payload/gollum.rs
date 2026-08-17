use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GollumWebhookEventPayload {
    pub enterprise: Option<serde_json::Value>,
    /// The pages that were updated
    pub pages: Vec<serde_json::Value>,
}
