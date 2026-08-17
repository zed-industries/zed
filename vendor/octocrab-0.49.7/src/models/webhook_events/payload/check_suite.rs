use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckSuiteWebhookEventPayload {
    pub action: CheckSuiteWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub check_suite: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CheckSuiteWebhookEventAction {
    Completed,
    Requested,
    Rerequested,
}
