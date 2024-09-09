use collections::HashMap;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestType {
    Initialize,
    CallTool,
    ResourcesUnsubscribe,
    ResourcesSubscribe,
    ResourcesRead,
    ResourcesList,
    LoggingSetLevel,
    PromptsGet,
    PromptsList,
    CompletionComplete,
}

impl RequestType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestType::Initialize => "initialize",
            RequestType::CallTool => "tools/call",
            RequestType::ResourcesUnsubscribe => "resources/unsubscribe",
            RequestType::ResourcesSubscribe => "resources/subscribe",
            RequestType::ResourcesRead => "resources/read",
            RequestType::ResourcesList => "resources/list",
            RequestType::LoggingSetLevel => "logging/setLevel",
            RequestType::PromptsGet => "prompts/get",
            RequestType::PromptsList => "prompts/list",
            RequestType::CompletionComplete => "completion/complete",
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub protocol_version: u32,
    pub capabilities: ClientCapabilities,
    pub client_info: EntityInfo,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolParams {
    pub name: String,
    pub arguments: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesUnsubscribeParams {
    pub uri: Url,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesSubscribeParams {
    pub uri: Url,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesReadParams {
    pub uri: Url,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingSetLevelParams {
    pub level: LoggingLevel,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptsGetParams {
    pub name: String,
    pub arguments: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionCompleteParams {
    pub r#ref: CompletionReference,
    pub argument: CompletionArgument,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CompletionReference {
    Prompt(PromptReference),
    Resource(ResourceReference),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptReference {
    pub r#type: PromptReferenceType,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptReferenceType {
    #[serde(rename = "ref/prompt")]
    Prompt,
    #[serde(rename = "ref/resource")]
    Resource,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceReference {
    pub r#type: String,
    pub uri: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionArgument {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResponse {
    pub protocol_version: u32,
    pub capabilities: ServerCapabilities,
    pub server_info: EntityInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesReadResponse {
    pub contents: Vec<ResourceContent>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesListResponse {
    pub resource_templates: Option<Vec<ResourceTemplate>>,
    pub resources: Vec<Resource>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptsGetResponse {
    pub description: Option<String>,
    pub prompt: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptsListResponse {
    pub prompts: Vec<PromptInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionCompleteResponse {
    pub completion: CompletionResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionResult {
    pub values: Vec<String>,
    pub total: Option<u32>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PromptInfo {
    pub name: String,
    pub arguments: Option<Vec<PromptArgument>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PromptArgument {
    pub name: String,
    pub description: Option<String>,
    pub required: Option<bool>,
}

// Shared Types

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    pub experimental: Option<HashMap<String, serde_json::Value>>,
    pub sampling: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    pub experimental: Option<HashMap<String, serde_json::Value>>,
    pub logging: Option<HashMap<String, serde_json::Value>>,
    pub prompts: Option<HashMap<String, serde_json::Value>>,
    pub resources: Option<ResourcesCapabilities>,
    pub tools: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesCapabilities {
    pub subscribe: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub uri: Url,
    pub mime_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceContent {
    pub uri: Url,
    pub mime_type: Option<String>,
    pub content_type: String,
    pub text: Option<String>,
    pub data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTemplate {
    pub uri_template: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoggingLevel {
    Debug,
    Info,
    Warning,
    Error,
}

// Client Notifications

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationType {
    Initialized,
    Progress,
}

impl NotificationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationType::Initialized => "notifications/initialized",
            NotificationType::Progress => "notifications/progress",
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ClientNotification {
    Initialized,
    Progress(ProgressParams),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressParams {
    pub progress_token: String,
    pub progress: f64,
    pub total: Option<f64>,
}

// Helper Types that don't map directly to the protocol

pub enum CompletionTotal {
    Exact(u32),
    HasMore,
    Unknown,
}

impl CompletionTotal {
    pub fn from_options(has_more: Option<bool>, total: Option<u32>) -> Self {
        match (has_more, total) {
            (_, Some(count)) => CompletionTotal::Exact(count),
            (Some(true), _) => CompletionTotal::HasMore,
            _ => CompletionTotal::Unknown,
        }
    }
}

pub struct Completion {
    pub values: Vec<String>,
    pub total: CompletionTotal,
}
