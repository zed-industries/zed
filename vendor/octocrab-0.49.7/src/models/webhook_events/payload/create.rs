use serde::{Deserialize, Serialize};

use super::{PusherType, RefType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateWebhookEventPayload {
    pub description: Option<String>,
    pub enterprise: Option<serde_json::Value>,
    pub master_branch: String,
    pub pusher_type: PusherType,
    pub r#ref: String,
    pub ref_type: RefType,
}
