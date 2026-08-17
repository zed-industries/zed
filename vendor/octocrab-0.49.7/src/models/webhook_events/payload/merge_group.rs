use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergeGroupWebhookEventPayload {
    pub action: MergeGroupWebhookEventAction,
    pub merge_group: serde_json::Value,
    pub reason: Option<MergeGroupDestructionReason>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MergeGroupWebhookEventAction {
    ChecksRequested,
    Destroyed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MergeGroupDestructionReason {
    Merged,
    Invalidated,
    Dequeued,
}
