use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ScheduleWebhookEventPayload {
    pub schedule: String,
    pub workflow: String,
}
