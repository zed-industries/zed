use serde::{Deserialize, Serialize};

use super::{PusherType, RefType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeleteWebhookEventPayload {
    pub enterprise: Option<serde_json::Value>,
    pub pusher_type: PusherType,
    pub r#ref: String,
    pub ref_type: RefType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeleteWebhookEventAction {
    Created,
    Deleted,
    Edited,
}
