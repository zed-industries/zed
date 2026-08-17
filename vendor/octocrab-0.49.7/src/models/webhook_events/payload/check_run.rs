use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckRunWebhookEventPayload {
    pub action: CheckRunWebhookEventAction,
    pub check_run: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CheckRunWebhookEventAction {
    Completed,
    Created,
    RequestedAction,
    Rerequested,
}
