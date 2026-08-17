use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketplacePurchaseWebhookEventPayload {
    pub action: MarketplacePurchaseWebhookEventAction,
    pub effective_date: Option<String>,
    pub marketplace_purchase: serde_json::Value,
    pub previous_marketplace_purchase: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MarketplacePurchaseWebhookEventAction {
    Cancelled,
    Changed,
    PendingChange,
    PendingChangeCancelled,
    Purchased,
}
