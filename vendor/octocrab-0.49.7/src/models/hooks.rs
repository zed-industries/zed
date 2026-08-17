use super::{webhook_events::WebhookEventType, *};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Hook {
    pub r#type: String,
    pub active: bool,
    /// Only included in webhook payload received by GitHub Apps. When you
    /// register a new GitHub App, GitHub sends a ping event to the webhook URL
    /// you specified during registration. The GitHub App ID sent in this field
    /// is required for authenticating an app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<AppId>,
    pub id: u64,
    /// The type of webhook. At the time of writing, the only valid value is
    /// 'web'
    pub name: String,
    pub events: Vec<WebhookEventType>,
    pub config: Config,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_response: Option<LastResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliveries_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LastResponse {
    pub code: Option<i64>,
    pub status: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Json,
    #[default]
    Form,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Delivery {
    pub id: HookDeliveryId,
    pub guid: String,
    pub delivered_at: DateTime<Utc>,
    pub duration: f64,
    pub status: String,
    pub status_code: usize,
    pub event: Option<WebhookEventType>,
    pub action: Option<String>,
    pub installation_id: Option<InstallationId>,
    pub repository_id: Option<InstallationId>,
    pub redelivery: bool,
}
