use serde::{Deserialize, Serialize};

use crate::models::{hooks::Hook, HookId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PingWebhookEventPayload {
    pub hook: Option<Hook>,
    pub hook_id: Option<HookId>,
    pub zen: Option<String>,
}
