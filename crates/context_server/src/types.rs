use collections::HashMap;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::client::RequestId;

pub const LATEST_PROTOCOL_VERSION: &str = "2025-11-25";
pub const VERSION_2025_03_26: &str = "2025-03-26";
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

    // Task-augmented tool call: same JSON-RPC method as CallTool, but the
    // response is a CreateTaskResult instead of CallToolResponse. The caller
    // chooses which request type to use based on task support negotiation.
    request!(
        "tools/call",
        CallToolAsTask,
        CallToolParams,
        CreateTaskResult
    );
    request!("tasks/get", TasksGet, TasksGetParams, Task);
    request!(
        "tasks/result",
        TasksResult,
        TasksResultParams,
        serde_json::Value
    );
    request!("tasks/list", TasksList, TasksListParams, TasksListResponse);
    request!("tasks/cancel", TasksCancel, TasksCancelParams, Task);
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
    notification!(
        "notifications/tasks/status",
        TaskStatus,
        TaskStatusNotificationParams
    );
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskParams>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<ClientTasksCapabilities>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<ServerTasksCapabilities>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<ToolExecution>,
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

impl PartialEq for ProgressToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ProgressToken::String(a), ProgressToken::String(b)) => a == b,
            (ProgressToken::Number(a), ProgressToken::Number(b)) => a.to_bits() == b.to_bits(),
            _ => false,
        }
    }
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

// ---------------------------------------------------------------------------
// MCP Task types (spec version 2025-11-25, experimental)
// ---------------------------------------------------------------------------

/// Parameters for task augmentation, included in the request params.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
}

/// A task represents the execution state of a request.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub task_id: String,
    pub status: TaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_interval: Option<u64>,
}

/// Task status values per the MCP spec state machine.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Working,
    InputRequired,
    Completed,
    Failed,
    Cancelled,
}

/// `_meta` key for model-immediate-response per MCP Tasks spec (2025-11-25).
/// When present in a `CreateTaskResult`, the value is a provisional
/// `CallToolResponse` that can be returned to the model immediately while
/// the task continues running in the background.
pub const MODEL_IMMEDIATE_RESPONSE_KEY: &str = "io.modelcontextprotocol/model-immediate-response";

/// Response to a task-augmented request. Contains task metadata and optional
/// `_meta` (which may include `io.modelcontextprotocol/model-immediate-response`).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskResult {
    pub task: Task,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for `tasks/get`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksGetParams {
    pub task_id: String,
}

/// Parameters for `tasks/result`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksResultParams {
    pub task_id: String,
}

/// Parameters for `tasks/list`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Response for `tasks/list`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksListResponse {
    pub tasks: Vec<Task>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Parameters for `tasks/cancel`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksCancelParams {
    pub task_id: String,
}

/// Parameters for `notifications/tasks/status`. The notification carries the
/// full Task state inline.
pub type TaskStatusNotificationParams = Task;

// ---------------------------------------------------------------------------
// MCP Task capability types
// ---------------------------------------------------------------------------

/// Server-side tasks capabilities declared during initialization.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTasksCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<ServerTaskRequestsCapabilities>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTaskRequestsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ServerTaskToolsCapabilities>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTaskToolsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call: Option<serde_json::Value>,
}

/// Client-side tasks capabilities declared during initialization.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientTasksCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<ClientTaskRequestsCapabilities>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientTaskRequestsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<ClientTaskSamplingCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elicitation: Option<ClientTaskElicitationCapabilities>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientTaskSamplingCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_message: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientTaskElicitationCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Tool execution metadata
// ---------------------------------------------------------------------------

/// Execution metadata on a Tool, controlling task support negotiation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolExecution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_support: Option<TaskSupport>,
}

/// Per-tool task support level.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskSupport {
    Required,
    Optional,
    Forbidden,
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
    fn test_create_task_result_with_model_immediate_response() {
        // Exact JSON the Python MCP server sends over the wire when
        // model-immediate-response is injected into _meta.
        let json = r#"{"_meta":{"io.modelcontextprotocol/model-immediate-response":{"content":[{"type":"text","text":"Task started — the full result will arrive in about 5 seconds. You can continue working."}],"isError":false}},"task":{"taskId":"t1","status":"working","createdAt":"2026-04-12T04:44:45.818973Z","lastUpdatedAt":"2026-04-12T04:44:45.819731Z","ttl":300000,"pollInterval":2000}}"#;
        let result: CreateTaskResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.task.task_id, "t1");
        assert_eq!(result.task.status, TaskStatus::Working);
        assert!(result.meta.is_some());
        let meta = result.meta.as_ref().unwrap();
        assert!(meta.contains_key(MODEL_IMMEDIATE_RESPONSE_KEY));

        // The provisional response should parse as CallToolResponse.
        let provisional_value = &meta[MODEL_IMMEDIATE_RESPONSE_KEY];
        let provisional: CallToolResponse =
            serde_json::from_value(provisional_value.clone()).unwrap();
        assert_eq!(provisional.content.len(), 1);
        assert_eq!(
            provisional.content[0].text(),
            Some("Task started — the full result will arrive in about 5 seconds. You can continue working.")
        );
    }

    #[test]
    fn test_create_task_result_with_model_immediate_response_string() {
        // Per the spec (2025-11-25), the value "should be a string intended
        // to be passed as an immediate tool result to the model."
        let json = r#"{"_meta":{"io.modelcontextprotocol/model-immediate-response":"Task accepted, running in background."},"task":{"taskId":"t2","status":"working","createdAt":"2026-01-01T00:00:00Z","lastUpdatedAt":"2026-01-01T00:00:01Z","ttl":300000,"pollInterval":2000}}"#;
        let result: CreateTaskResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.task.task_id, "t2");
        let meta = result.meta.as_ref().unwrap();
        let value = &meta[MODEL_IMMEDIATE_RESPONSE_KEY];

        // The value is a plain JSON string, not a CallToolResponse object.
        assert!(value.is_string());
        assert_eq!(
            value.as_str().unwrap(),
            "Task accepted, running in background."
        );

        // Our Zed-side code wraps this into a CallToolResponse with a single
        // text content block. Verify the string can't accidentally parse as
        // a CallToolResponse (it shouldn't — it's just a string).
        assert!(
            serde_json::from_value::<CallToolResponse>(value.clone()).is_err(),
            "a plain string should not parse as CallToolResponse"
        );
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

    // -----------------------------------------------------------------------
    // MCP Task type tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_task_status_serde_round_trip() {
        let statuses = vec![
            (TaskStatus::Working, "\"working\""),
            (TaskStatus::InputRequired, "\"input_required\""),
            (TaskStatus::Completed, "\"completed\""),
            (TaskStatus::Failed, "\"failed\""),
            (TaskStatus::Cancelled, "\"cancelled\""),
        ];
        for (status, expected_json) in statuses {
            let json = serde_json::to_string(&status).unwrap();
            assert_eq!(json, expected_json);
            let deserialized: TaskStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, status);
        }
    }

    #[test]
    fn test_task_serde_round_trip() {
        let task = Task {
            task_id: "abc-123".to_string(),
            status: TaskStatus::Working,
            status_message: Some("Processing...".to_string()),
            created_at: "2025-11-25T10:30:00Z".to_string(),
            last_updated_at: "2025-11-25T10:40:00Z".to_string(),
            ttl: Some(60000),
            poll_interval: Some(5000),
        };
        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.task_id, "abc-123");
        assert_eq!(deserialized.status, TaskStatus::Working);
        assert_eq!(deserialized.status_message.as_deref(), Some("Processing..."));
        assert_eq!(deserialized.ttl, Some(60000));
        assert_eq!(deserialized.poll_interval, Some(5000));
    }

    #[test]
    fn test_task_deserialize_from_wire_format() {
        let json = r#"{
            "taskId": "786512e2-9e0d-44bd-8f29-789f320fe840",
            "status": "completed",
            "createdAt": "2025-11-25T10:30:00Z",
            "lastUpdatedAt": "2025-11-25T10:50:00Z",
            "ttl": 60000,
            "pollInterval": 5000
        }"#;
        let task: Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.task_id, "786512e2-9e0d-44bd-8f29-789f320fe840");
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.status_message.is_none());
        assert_eq!(task.ttl, Some(60000));
    }

    #[test]
    fn test_create_task_result_serde() {
        let json = r#"{
            "task": {
                "taskId": "test-task-1",
                "status": "working",
                "statusMessage": "Starting...",
                "createdAt": "2025-01-01T00:00:00Z",
                "lastUpdatedAt": "2025-01-01T00:00:00Z",
                "ttl": 30000,
                "pollInterval": 1000
            },
            "_meta": {
                "io.modelcontextprotocol/model-immediate-response": "Task started, result will be available shortly."
            }
        }"#;
        let result: CreateTaskResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.task.task_id, "test-task-1");
        assert_eq!(result.task.status, TaskStatus::Working);
        assert!(result.meta.is_some());
        let meta = result.meta.unwrap();
        assert!(meta.contains_key("io.modelcontextprotocol/model-immediate-response"));
    }

    #[test]
    fn test_task_support_serde() {
        assert_eq!(
            serde_json::to_string(&TaskSupport::Required).unwrap(),
            r#""required""#
        );
        assert_eq!(
            serde_json::to_string(&TaskSupport::Optional).unwrap(),
            r#""optional""#
        );
        assert_eq!(
            serde_json::to_string(&TaskSupport::Forbidden).unwrap(),
            r#""forbidden""#
        );
        let deserialized: TaskSupport = serde_json::from_str(r#""optional""#).unwrap();
        assert_eq!(deserialized, TaskSupport::Optional);
    }

    #[test]
    fn test_tool_with_execution_field() {
        let json = r#"{
            "name": "long_running_tool",
            "description": "A tool that takes a while",
            "inputSchema": {"type": "object"},
            "execution": {
                "taskSupport": "required"
            }
        }"#;
        let tool: Tool = serde_json::from_str(json).unwrap();
        assert_eq!(tool.name, "long_running_tool");
        let execution = tool.execution.unwrap();
        assert_eq!(execution.task_support, Some(TaskSupport::Required));
    }

    #[test]
    fn test_tool_without_execution_field_backwards_compat() {
        let json = r#"{
            "name": "simple_tool",
            "inputSchema": {"type": "object"}
        }"#;
        let tool: Tool = serde_json::from_str(json).unwrap();
        assert!(tool.execution.is_none());
    }

    #[test]
    fn test_server_capabilities_with_tasks() {
        let json = r#"{
            "tools": {"listChanged": true},
            "tasks": {
                "list": {},
                "cancel": {},
                "requests": {
                    "tools": {
                        "call": {}
                    }
                }
            }
        }"#;
        let caps: ServerCapabilities = serde_json::from_str(json).unwrap();
        assert!(caps.tools.is_some());
        let tasks = caps.tasks.unwrap();
        assert!(tasks.list.is_some());
        assert!(tasks.cancel.is_some());
        let requests = tasks.requests.unwrap();
        assert!(requests.tools.unwrap().call.is_some());
    }

    #[test]
    fn test_server_capabilities_without_tasks_backwards_compat() {
        let json = r#"{"tools": {"listChanged": true}}"#;
        let caps: ServerCapabilities = serde_json::from_str(json).unwrap();
        assert!(caps.tasks.is_none());
    }

    #[test]
    fn test_call_tool_params_with_task_augmentation() {
        let json = r#"{
            "name": "get_weather",
            "arguments": {"city": "New York"},
            "task": {"ttl": 60000}
        }"#;
        let params: CallToolParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.name, "get_weather");
        let task = params.task.unwrap();
        assert_eq!(task.ttl, Some(60000));
    }

    #[test]
    fn test_call_tool_params_without_task_backwards_compat() {
        let json = r#"{"name": "echo", "arguments": {"text": "hi"}}"#;
        let params: CallToolParams = serde_json::from_str(json).unwrap();
        assert!(params.task.is_none());
    }

    #[test]
    fn test_progress_token_partial_eq() {
        assert_eq!(
            ProgressToken::String("abc".to_string()),
            ProgressToken::String("abc".to_string())
        );
        assert_ne!(
            ProgressToken::String("abc".to_string()),
            ProgressToken::String("def".to_string())
        );
        assert_eq!(ProgressToken::Number(42.0), ProgressToken::Number(42.0));
        assert_ne!(ProgressToken::Number(1.0), ProgressToken::Number(2.0));
        assert_ne!(
            ProgressToken::String("42".to_string()),
            ProgressToken::Number(42.0)
        );
    }

}
