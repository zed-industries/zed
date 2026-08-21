use collections::HashMap;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::client::RequestId;

pub const VERSION_2024_11_05: &str = "2024-11-05";
pub const VERSION_2025_03_26: &str = "2025-03-26";
pub const VERSION_2025_06_18: &str = "2025-06-18";
pub const VERSION_2025_11_25: &str = "2025-11-25";
pub const VERSION_2026_07_28: &str = "2026-07-28";
pub const LATEST_PROTOCOL_VERSION: &str = VERSION_2026_07_28;
/// The newest protocol revision that still uses the `initialize` handshake,
/// offered when falling back to servers that predate 2026-07-28.
pub const LATEST_LEGACY_PROTOCOL_VERSION: &str = VERSION_2025_11_25;

/// Protocol versions that include the streamable HTTP transport's
/// `MCP-Protocol-Version` header requirement on post-initialize requests.
pub fn requires_protocol_version_header(version: &str) -> bool {
    matches!(
        version,
        VERSION_2025_06_18 | VERSION_2025_11_25 | VERSION_2026_07_28
    )
}

/// Protocol revisions from 2026-07-28 onward are "modern": there is no
/// `initialize` handshake, and every request instead carries its protocol
/// version, client info, and client capabilities in `_meta`.
pub fn is_modern_protocol_version(version: &str) -> bool {
    version >= VERSION_2026_07_28
}

/// `_meta` keys defined by the MCP specification.
pub mod meta_keys {
    pub const PROTOCOL_VERSION: &str = "io.modelcontextprotocol/protocolVersion";
    pub const CLIENT_INFO: &str = "io.modelcontextprotocol/clientInfo";
    pub const CLIENT_CAPABILITIES: &str = "io.modelcontextprotocol/clientCapabilities";
    pub const SERVER_INFO: &str = "io.modelcontextprotocol/serverInfo";
    pub const LOG_LEVEL: &str = "io.modelcontextprotocol/logLevel";
    pub const SUBSCRIPTION_ID: &str = "io.modelcontextprotocol/subscriptionId";
}

/// Error codes reserved for the MCP specification (`-32020` to `-32099`),
/// introduced in the 2026-07-28 revision.
pub mod error_codes {
    pub const HEADER_MISMATCH: i32 = -32020;
    pub const MISSING_REQUIRED_CLIENT_CAPABILITY: i32 = -32021;
    pub const UNSUPPORTED_PROTOCOL_VERSION: i32 = -32022;
}

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
    request!("server/discover", ServerDiscover, (), DiscoverResponse);
    request!(
        "subscriptions/listen",
        SubscriptionsListen,
        SubscriptionsListenParams,
        SubscriptionsListenResponse
    );
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
        "notifications/subscriptions/acknowledged",
        SubscriptionsAcknowledged,
        SubscriptionsAcknowledgedParams
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Previously-resolved argument values so the server can provide
    /// context-sensitive completions (added in MCP 2025-06-18).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<CompletionContext>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, String>>,
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

/// Discriminates ordinary results from multi round-trip interim results.
/// Required on all results from 2026-07-28 onward; results from
/// earlier-protocol servers that omit it must be treated as `Complete`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultType {
    Complete,
    InputRequired,
}

/// Whether shared intermediaries may cache a response (added in MCP
/// 2026-07-28 as part of `CacheableResult`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheScope {
    Public,
    Private,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverResponse {
    pub supported_versions: Vec<ProtocolVersion>,
    pub capabilities: ServerCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_scope: Option<CacheScope>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

impl DiscoverResponse {
    /// The server identity self-reported in the result's `_meta`, if any.
    pub fn server_info(&self) -> Option<Implementation> {
        let value = self.meta.as_ref()?.get(meta_keys::SERVER_INFO)?;
        serde_json::from_value(value.clone()).ok()
    }
}

/// The notification types a client opts into with `subscriptions/listen`.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools_list_changed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompts_list_changed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources_list_changed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_subscriptions: Option<Vec<Url>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionsListenParams {
    pub notifications: NotificationFilter,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// The (empty) response to `subscriptions/listen`, sent by the server only
/// when it gracefully closes the subscription.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionsListenResponse {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionsAcknowledgedParams {
    pub notifications: NotificationFilter,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// An interim result indicating the server needs additional input before it
/// can complete the request (multi round-trip requests, MCP 2026-07-28).
/// The client retries the original request with `inputResponses` for each
/// entry in `input_requests`, echoing `request_state` verbatim.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputRequiredResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_requests: Option<HashMap<String, InputRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_state: Option<String>,
}

/// A server-to-client request embedded in an [`InputRequiredResult`]
/// (e.g. `elicitation/create`, `sampling/createMessage`, or `roots/list`).
#[derive(Debug, Serialize, Deserialize)]
pub struct InputRequest {
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

/// The `data` payload of an `UnsupportedProtocolVersionError` (code
/// [`error_codes::UNSUPPORTED_PROTOCOL_VERSION`]).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsupportedProtocolVersionData {
    pub supported: Vec<ProtocolVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested: Option<ProtocolVersion>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_scope: Option<CacheScope>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_scope: Option<CacheScope>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_scope: Option<CacheScope>,
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
    /// Human-readable display name (added in MCP 2025-06-18).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptArgument {
    pub name: String,
    /// Human-readable display name (added in MCP 2025-06-18).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<RootsCapabilities>,
    /// Extensions the client supports, keyed by reverse-DNS extension ID
    /// (added in MCP 2026-07-28).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
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
    /// Extensions the server supports, keyed by reverse-DNS extension ID
    /// (added in MCP 2026-07-28).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub name: String,
    /// Human-readable display name (added in MCP 2025-06-18).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Implementation {
    pub name: String,
    /// Human-readable display name (added in MCP 2025-06-18).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub version: String,
    /// Human-readable description of the implementation (added in MCP 2025-11-25).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub uri: Url,
    pub name: String,
    /// Human-readable display name (added in MCP 2025-06-18).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
    /// Human-readable display name (added in MCP 2025-06-18).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
            if let ToolResponseContent::Text { text: chunk } = chunk {
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
    Text { text: String },
    #[serde(rename = "image", rename_all = "camelCase")]
    Image { data: String, mime_type: String },
    #[serde(rename = "audio", rename_all = "camelCase")]
    Audio { data: String, mime_type: String },
    #[serde(rename = "resource")]
    Resource { resource: ResourceContents },
    /// Link to an MCP resource on the server, without inlining its contents.
    /// Added in MCP 2025-06-18.
    #[serde(rename = "resource_link", rename_all = "camelCase")]
    ResourceLink {
        uri: Url,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mime_type: Option<String>,
    },
}

impl ToolResponseContent {
    pub fn text(&self) -> Option<&str> {
        if let ToolResponseContent::Text { text } = self {
            Some(text)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListToolsResponse {
    pub tools: Vec<Tool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_scope: Option<CacheScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourceTemplatesResponse {
    pub resource_templates: Vec<ResourceTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_scope: Option<CacheScope>,
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

    #[test]
    fn test_discover_response_from_spec_example() {
        let response: DiscoverResponse = serde_json::from_value(serde_json::json!({
            "resultType": "complete",
            "supportedVersions": ["2026-07-28"],
            "capabilities": {
                "tools": {},
                "resources": {}
            },
            "_meta": {
                "io.modelcontextprotocol/serverInfo": {
                    "name": "ExampleServer",
                    "version": "1.0.0"
                }
            },
            "instructions": "This server provides weather and resource utilities.",
            "ttlMs": 3600000,
            "cacheScope": "public"
        }))
        .unwrap();

        assert_eq!(
            response.supported_versions,
            vec![ProtocolVersion(VERSION_2026_07_28.to_string())]
        );
        assert!(response.capabilities.tools.is_some());
        assert!(response.capabilities.resources.is_some());
        assert!(response.capabilities.prompts.is_none());
        assert_eq!(response.ttl_ms, Some(3_600_000));
        assert_eq!(response.cache_scope, Some(CacheScope::Public));
        let server_info = response.server_info().unwrap();
        assert_eq!(server_info.name, "ExampleServer");
        assert_eq!(server_info.version, "1.0.0");
    }

    #[test]
    fn test_input_required_result_from_spec_example() {
        let result: InputRequiredResult = serde_json::from_value(serde_json::json!({
            "resultType": "input_required",
            "inputRequests": {
                "github_login": {
                    "method": "elicitation/create",
                    "params": {
                        "mode": "form",
                        "message": "Please provide your GitHub username"
                    }
                }
            },
            "requestState": "AEAD-protected blob"
        }))
        .unwrap();

        let requests = result.input_requests.unwrap();
        assert_eq!(requests["github_login"].method, "elicitation/create");
        assert_eq!(result.request_state.as_deref(), Some("AEAD-protected blob"));

        // requestState-only interim results are valid too.
        let result: InputRequiredResult = serde_json::from_value(serde_json::json!({
            "resultType": "input_required",
            "requestState": "opaque"
        }))
        .unwrap();
        assert!(result.input_requests.is_none());
    }

    #[test]
    fn test_subscriptions_listen_params_serialization() {
        let params = SubscriptionsListenParams {
            notifications: NotificationFilter {
                tools_list_changed: Some(true),
                resource_subscriptions: Some(vec![
                    Url::parse("file:///project/config.json").unwrap(),
                ]),
                ..Default::default()
            },
            meta: None,
        };

        assert_eq!(
            serde_json::to_value(&params).unwrap(),
            serde_json::json!({
                "notifications": {
                    "toolsListChanged": true,
                    "resourceSubscriptions": ["file:///project/config.json"]
                }
            })
        );
    }

    #[test]
    fn test_unsupported_protocol_version_error_data() {
        let data: UnsupportedProtocolVersionData = serde_json::from_value(serde_json::json!({
            "supported": ["2026-07-28", "2025-11-25"],
            "requested": "1900-01-01"
        }))
        .unwrap();

        assert_eq!(
            data.supported,
            vec![
                ProtocolVersion(VERSION_2026_07_28.to_string()),
                ProtocolVersion(VERSION_2025_11_25.to_string())
            ]
        );
        assert_eq!(data.requested, Some(ProtocolVersion("1900-01-01".into())));
    }

    #[test]
    fn test_modern_protocol_version_predicate() {
        assert!(is_modern_protocol_version(VERSION_2026_07_28));
        assert!(is_modern_protocol_version("2027-01-01"));
        assert!(!is_modern_protocol_version(VERSION_2025_11_25));
        assert!(!is_modern_protocol_version(VERSION_2024_11_05));
    }

    #[test]
    fn test_cacheable_list_tools_response_tolerates_missing_fields() {
        // Legacy servers omit resultType/ttlMs/cacheScope entirely.
        let response: ListToolsResponse = serde_json::from_value(serde_json::json!({
            "tools": []
        }))
        .unwrap();
        assert!(response.ttl_ms.is_none());
        assert!(response.cache_scope.is_none());

        let response: ListToolsResponse = serde_json::from_value(serde_json::json!({
            "resultType": "complete",
            "tools": [],
            "ttlMs": 60000,
            "cacheScope": "private"
        }))
        .unwrap();
        assert_eq!(response.ttl_ms, Some(60_000));
        assert_eq!(response.cache_scope, Some(CacheScope::Private));
    }
}
