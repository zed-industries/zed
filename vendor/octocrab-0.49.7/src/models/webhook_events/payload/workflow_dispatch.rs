use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowDispatchWebhookEventPayload {
    pub enterprise: Option<serde_json::Value>,
    pub inputs: serde_json::Value,
    pub r#ref: String,
    pub workflow: String,
}
