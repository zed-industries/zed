use serde::{Deserialize, Serialize};

use crate::models::Repository;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForkWebhookEventPayload {
    // TODO: Make sure that it's a crate::models::Repository
    pub forkee: Repository,
    pub enterprise: Option<serde_json::Value>,
}
