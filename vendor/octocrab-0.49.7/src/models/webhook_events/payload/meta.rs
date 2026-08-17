use serde::{Deserialize, Serialize};

use crate::models::{hooks::Hook, HookId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MetaWebhookEventPayload {
    pub action: MetaWebhookEventAction,
    pub hook: Hook,
    pub enterprise: Option<serde_json::Value>,
    pub hook_id: HookId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MetaWebhookEventAction {
    Deleted,
}
