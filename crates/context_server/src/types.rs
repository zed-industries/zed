use collections::HashMap;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::client::RequestId;

pub const LATEST_PROTOCOL_VERSION: &str = "2025-03-26";
pub const VERSION_2024_11_05: &str = "2024-11-05";

pub mod requests {
    use super::*;

    macro_rules! request {
        ($method:expr, $name:ident, $params:ty, $response:ty) => {
            pub struct $name;

            impl Request for $name {
                type Params = $params;
                type Response = $response;
                const METHOD: &'static str = $method;
            }
        };
    }

    request!(
        "initialize",
        Initialize,
        InitializeParams,
        InitializeResponse
    );
    request!("tools/call", CallTool, CallToolParams, CallToolResponse);
    request!(
        "resources/unsubscribe",
        ResourcesUnsubscribe,
        ResourcesUnsubscribeParams,
        ()
    );
    request!(
        "resources/subscribe",
        ResourcesSubscribe,
        ResourcesSubscribeParams,
        ()
    );
    request!(
        "resources/read",
        ResourcesRead,
        ResourcesReadParams,
        ResourcesReadResponse
    );
    request!("resources/list", ResourcesList, (), ResourcesListResponse);
    request!(
        "logging/setLevel",
        LoggingSetLevel,
        LoggingSetLevelParams,
        ()
    );
    request!(
        "prompts/get",
        PromptsGet,
        PromptsGetParams,
        PromptsGetResponse
    );
    request!("prompts/list", PromptsList, (), PromptsListResponse);
    request!(
        "completion/complete",
        CompletionComplete,
        CompletionCompleteParams,
        CompletionCompleteResponse
    );
    request!("ping", Ping, (), ());
    request!("tools/list", ListTools, (), ListToolsResponse);
    request!(
        "resources/templates/list",
        ListResourceTemplates,
        (),
        ListResourceTemplatesResponse
    );
    request!("roots/list", ListRoots, (), ListRootsResponse);
}

pub trait Request {
    type Params: DeserializeOwned + Serialize + Send + Sync + 'static;
    type Response: DeserializeOwned + Serialize + Send + Sync + 'static;
    const METHOD: &'static str;
}

pub mod notifications {
    use super::*;

    macro_rules! notification {
        ($method:expr, $name:ident, $params:ty) => {
            pub struct $name;

            impl Notification for $name {
                type Params = $params;
                const METHOD: &'static str = $method;
            }
        };
    }

    notification!("notifications/initialized", Initialized, ());
    notification!("notifications/progress", Progress, ProgressParams);
    notification!("notifications/message", Message, MessageParams);
    notification!("notifications/cancelled", Cancelled, CancelledParams);
    notification!(
        "notifications/resources/updated",
        ResourcesUpdated,
        ResourcesUpdatedParams
    );
    notification!(
        "notifications/resources/list_changed",
        ResourcesListChanged,
        ()
    );
    notification!("notifications/tools/list_changed", ToolsListChanged, ());
    notification!("notifications/prompts/list_changed", PromptsListChanged, ());
    notification!("notifications/roots/list_changed", RootsListChanged, ());
}

pub trait Notification {
    type Params: DeserializeOwned + Serialize + Send + Sync + 'static;
    const METHOD: &'static str;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageParams {
    pub level: LoggingLevel,
    pub logger: Option<String>,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesUpdatedParams {
    pub uri: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProtocolVersion(pub String);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub protocol_version: ProtocolVersion,
    pub capabilities: ClientCapabilities,
    pub client_info: Implementation,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesUnsubscribeParams {
    pub uri: Url,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesSubscribeParams {
    pub uri: Url,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesReadParams {
    pub uri: Url,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingSetLevelParams {
    pub level: LoggingLevel,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptsGetParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, String>>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionCompleteParams {
    #[serde(rename = "ref")]
    pub reference: CompletionReference,
    pub argument: CompletionArgument,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionReference {
    Prompt(PromptReference),
    Resource(ResourceReference),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptReference {
    #[serde(rename = "type")]
    pub ty: PromptReferenceType,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceReference {
    #[serde(rename = "type")]
    pub ty: PromptReferenceType,
    pub uri: Url,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptReferenceType {
    #[serde(rename = "ref/prompt")]
    Prompt,
    #[serde(rename = "ref/resource")]
    Resource,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionArgument {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResponse {
    pub protocol_version: ProtocolVersion,
    pub capabilities: ServerCapabilities,
    pub server_info: Implementation,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesReadResponse {
    pub contents: Vec<ResourceContentsType>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceContentsType {
    Text(TextResourceContents),
    Blob(BlobResourceContents),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesListResponse {
    pub resources: Vec<Resource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SamplingMessage {
    pub role: Role,
    pub content: MessageContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageRequest {
    pub messages: Vec<SamplingMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_preferences: Option<ModelPreferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageResult {
    pub role: Role,
    pub content: MessageContent,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptMessage {
    pub role: Role,
    pub content: MessageContent,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<MessageAnnotations>,
    },
    #[serde(rename = "image", rename_all = "camelCase")]
    Image {
        data: String,
        mime_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<MessageAnnotations>,
    },
    #[serde(rename = "audio", rename_all = "camelCase")]
    Audio {
        data: String,
        mime_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<MessageAnnotations>,
    },
    #[serde(rename = "resource")]
    Resource {
        resource: ResourceContents,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<MessageAnnotations>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAnnotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<Role>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptsGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub messages: Vec<PromptMessage>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptsListResponse {
    pub prompts: Vec<Prompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionCompleteResponse {
    pub completion: CompletionResult,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionResult {
    pub values: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prompt {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptArgument {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<RootsCapabilities>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptsCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapabilities>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub input_schema: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<ToolAnnotations>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolAnnotations {
    /// A human-readable title for the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// If true, the tool does not modify its environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_hint: Option<bool>,
    /// If true, the tool may perform destructive updates to its environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destructive_hint: Option<bool>,
    /// If true, calling the tool repeatedly with the same arguments will have no additional effect on its environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotent_hint: Option<bool>,
    /// If true, this tool may interact with an "open world" of external entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_world_hint: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Implementation {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub uri: Url,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceContents {
    pub uri: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextResourceContents {
    pub uri: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobResourceContents {
    pub uri: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    pub blob: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTemplate {
    pub uri_template: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoggingLevel {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelPreferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<ModelHint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_priority: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_priority: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligence_priority: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelHint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ClientNotification {
    Initialized,
    Progress(ProgressParams),
    RootsListChanged,
    Cancelled(CancelledParams),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelledParams {
    pub request_id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProgressToken {
    String(String),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressParams {
    pub progress_token: ProgressToken,
    pub progress: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolResponse {
    pub content: Vec<ToolResponseContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structured_content: Option<serde_json::Value>,
}

impl CallToolResponse {
    pub fn text_contents(&self) -> String {
        let mut text = String::new();
        for chunk in &self.content {
            if let ToolResponseContent::Text { text: chunk, .. } = chunk {
                text.push_str(chunk)
            };
        }
        text
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolResponseContent {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<MessageAnnotations>,
    },
    #[serde(rename = "image", rename_all = "camelCase")]
    Image {
        data: String,
        mime_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<MessageAnnotations>,
    },
    #[serde(rename = "audio", rename_all = "camelCase")]
    Audio {
        data: String,
        mime_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<MessageAnnotations>,
    },
    #[serde(rename = "resource")]
    Resource {
        resource: ResourceContents,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<MessageAnnotations>,
    },
}

impl ToolResponseContent {
    pub fn text(&self) -> Option<&str> {
        if let ToolResponseContent::Text { text, .. } = self {
            Some(text)
        } else {
            None
        }
    }

    /// Returns the audience annotation for this content block, if present.
    ///
    /// MCP spec (2025-03-26) `Annotations.audience`: an array of `Role`
    /// values indicating the intended recipients of this content block.
    /// <https://modelcontextprotocol.io/specification/2025-03-26/server/tools>
    pub fn audience(&self) -> Option<&Vec<Role>> {
        let annotations = match self {
            Self::Text { annotations, .. }
            | Self::Image { annotations, .. }
            | Self::Audio { annotations, .. }
            | Self::Resource { annotations, .. } => annotations,
        };
        annotations.as_ref().and_then(|a| a.audience.as_ref())
    }

    /// Returns `true` if this content block is intended only for the user —
    /// i.e. the audience contains `User` but not `Assistant`.
    ///
    /// Per the MCP spec, absent or empty `audience` means the content is
    /// for both user and model (returns `false`).  `["user"]` means
    /// display-only (returns `true`).  `["user", "assistant"]` means both
    /// (returns `false`).
    ///
    /// Used by `ContextServerTool::run()` to partition tool response blocks.
    /// See `crates/agent/src/tests/test_mcp_audience.rs` for the full
    /// routing table and integration tests.
    pub fn is_user_only(&self) -> bool {
        match self.audience() {
            None => false,
            Some(roles) => {
                let has_user = roles.iter().any(|r| matches!(r, Role::User));
                let has_assistant = roles.iter().any(|r| matches!(r, Role::Assistant));
                has_user && !has_assistant
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListToolsResponse {
    pub tools: Vec<Tool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourceTemplatesResponse {
    pub resource_templates: Vec<ResourceTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRootsResponse {
    pub roots: Vec<Root>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub uri: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_block(text: &str) -> ToolResponseContent {
        ToolResponseContent::Text {
            text: text.to_string(),
            annotations: None,
        }
    }

    fn text_block_with_audience(text: &str, audience: Vec<Role>) -> ToolResponseContent {
        ToolResponseContent::Text {
            text: text.to_string(),
            annotations: Some(MessageAnnotations {
                audience: Some(audience),
                priority: None,
            }),
        }
    }

    fn image_block_with_audience(audience: Vec<Role>) -> ToolResponseContent {
        ToolResponseContent::Image {
            data: "fake".to_string(),
            mime_type: "image/png".to_string(),
            annotations: Some(MessageAnnotations {
                audience: Some(audience),
                priority: None,
            }),
        }
    }

    #[test]
    fn test_no_annotations_is_not_user_only() {
        let block = text_block("hello");
        assert!(!block.is_user_only());
        assert!(block.audience().is_none());
    }

    #[test]
    fn test_empty_audience_is_not_user_only() {
        let block = ToolResponseContent::Text {
            text: "hello".into(),
            annotations: Some(MessageAnnotations {
                audience: Some(vec![]),
                priority: None,
            }),
        };
        assert!(!block.is_user_only());
    }

    #[test]
    fn test_user_only_audience() {
        let block = text_block_with_audience("secret", vec![Role::User]);
        assert!(block.is_user_only());
        assert_eq!(block.audience().unwrap(), &vec![Role::User]);
    }

    #[test]
    fn test_assistant_only_is_not_user_only() {
        let block = text_block_with_audience("model-only", vec![Role::Assistant]);
        assert!(!block.is_user_only());
    }

    #[test]
    fn test_both_roles_is_not_user_only() {
        let block = text_block_with_audience("shared", vec![Role::User, Role::Assistant]);
        assert!(!block.is_user_only());
    }

    #[test]
    fn test_both_roles_reversed_is_not_user_only() {
        let block = text_block_with_audience("shared", vec![Role::Assistant, Role::User]);
        assert!(!block.is_user_only());
    }

    #[test]
    fn test_image_block_user_only() {
        let block = image_block_with_audience(vec![Role::User]);
        assert!(block.is_user_only());
    }

    #[test]
    fn test_image_block_both_is_not_user_only() {
        let block = image_block_with_audience(vec![Role::User, Role::Assistant]);
        assert!(!block.is_user_only());
    }

    #[test]
    fn test_resource_block_audience() {
        let block = ToolResponseContent::Resource {
            resource: ResourceContents {
                uri: Url::parse("file:///test").unwrap(),
                mime_type: None,
            },
            annotations: Some(MessageAnnotations {
                audience: Some(vec![Role::User]),
                priority: None,
            }),
        };
        assert!(block.is_user_only());
    }

    #[test]
    fn test_audio_block_no_annotations() {
        let block = ToolResponseContent::Audio {
            data: "audio-data".to_string(),
            mime_type: "audio/wav".to_string(),
            annotations: None,
        };
        assert!(!block.is_user_only());
        assert!(block.audience().is_none());
    }

    #[test]
    fn test_annotations_with_only_priority_is_not_user_only() {
        let block = ToolResponseContent::Text {
            text: "hello".into(),
            annotations: Some(MessageAnnotations {
                audience: None,
                priority: Some(0.5),
            }),
        };
        assert!(!block.is_user_only());
        assert!(block.audience().is_none());
    }

    #[test]
    fn test_tool_response_content_text_accessor() {
        let block = text_block_with_audience("hello", vec![Role::User]);
        assert_eq!(block.text(), Some("hello"));

        let image = image_block_with_audience(vec![Role::User]);
        assert_eq!(image.text(), None);
    }

    #[test]
    fn test_serde_round_trip_with_annotations() {
        let block = text_block_with_audience("test", vec![Role::User]);
        let json = serde_json::to_string(&block).unwrap();
        let deserialized: ToolResponseContent = serde_json::from_str(&json).unwrap();
        assert!(deserialized.is_user_only());
        assert_eq!(deserialized.text(), Some("test"));
    }

    #[test]
    fn test_serde_round_trip_without_annotations() {
        let block = text_block("plain");
        let json = serde_json::to_string(&block).unwrap();
        assert!(!json.contains("annotations"));
        let deserialized: ToolResponseContent = serde_json::from_str(&json).unwrap();
        assert!(!deserialized.is_user_only());
    }

    #[test]
    fn test_deserialize_from_mcp_wire_format() {
        let json = r#"{"type":"text","text":"preview data","annotations":{"audience":["user"]}}"#;
        let block: ToolResponseContent = serde_json::from_str(json).unwrap();
        assert!(block.is_user_only());
        assert_eq!(block.text(), Some("preview data"));
    }

    #[test]
    fn test_deserialize_without_annotations_field() {
        let json = r#"{"type":"text","text":"plain"}"#;
        let block: ToolResponseContent = serde_json::from_str(json).unwrap();
        assert!(!block.is_user_only());
        assert!(block.audience().is_none());
    }
}
