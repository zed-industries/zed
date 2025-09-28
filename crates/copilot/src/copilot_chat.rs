use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;

use anyhow::Context as _;
use anyhow::{Result, anyhow};
use chrono::DateTime;
use collections::HashSet;
use fs::Fs;
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use gpui::WeakEntity;
use gpui::{App, AsyncApp, Global, prelude::*};
use http_client::HttpRequestExt;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use itertools::Itertools;
use paths::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use settings::watch_config_dir;

// Inline Responses API implementation
use std::sync::atomic::{AtomicU64, Ordering};

static MESSAGE_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ResponseInputItem {
    SystemMessage {
        role: String, // "system"
        content: Vec<ResponseInputContent>,
    },
    UserMessage {
        role: String, // "user"
        content: Vec<ResponseInputContent>,
    },
    AssistantMessage {
        role: String, // "assistant"
        content: Vec<ResponseOutputContent>,
        #[serde(rename = "type")]
        type_field: String, // "message"
        status: String, // "completed"
        id: String,
    },
    FunctionCall {
        #[serde(rename = "type")]
        type_field: String, // "function_call"
        name: String,
        arguments: String,
        call_id: String,
    },
    FunctionCallOutput {
        #[serde(rename = "type")]
        type_field: String, // "function_call_output"
        call_id: String,
        output: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ResponseInputContent {
    Text {
        #[serde(rename = "type")]
        type_field: String, // "text"
        text: String,
    },
    ImageUrl {
        #[serde(rename = "type")]
        type_field: String, // "image_url"
        #[serde(rename = "image_url")]
        image_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ResponseOutputContent {
    Text {
        #[serde(rename = "type")]
        type_field: String, // "text"
        text: String,
    },
}

fn create_responses_request_body(request: &Request) -> Result<String, serde_json::Error> {
    // Convert messages to ResponseInputItem array following VSCode patterns
    let input_items = map_messages_to_responses_input(&request.messages);

    // Create Responses API request following the documented format
    let mut responses_request = json!({
        "model": request.model,
        "input": input_items,
        "stream": true,
        "include": ["reasoning.encrypted_content"],
        "store": false
    });

    if let Some(previous_response_id) = request.previous_response_id.as_ref() {
        if previous_response_id.starts_with("resp_") {
            responses_request["previous_response_id"] = json!(previous_response_id);
        } else {
            log::warn!(
                "Ignoring unexpected previous_response_id '{}'; expected it to start with 'resp_'",
                previous_response_id
            );
        }
    }

    // Add tools if present
    if !request.tools.is_empty() {
        let responses_tools: Vec<serde_json::Value> = request
            .tools
            .iter()
            .map(|tool| match tool {
                Tool::Function { function } => {
                    log::debug!(
                        "GPT-5-Codex tool definition: name={}, description={}, original_parameters={}",
                        function.name,
                        function.description,
                        serde_json::to_string_pretty(&function.parameters).unwrap_or_default()
                    );
                    let normalized_params = normalize_tool_parameters(&function.parameters);
                    log::debug!(
                        "GPT-5-Codex normalized parameters for {}: {}",
                        function.name,
                        serde_json::to_string_pretty(&normalized_params).unwrap_or_default()
                    );
                    json!({
                        "type": "function",
                        "name": function.name,
                        "description": function.description,
                        "parameters": normalized_params,
                        "strict": false
                    })
                }
            })
            .collect();
        responses_request["tools"] = json!(responses_tools);
        log::debug!(
            "GPT-5-Codex final tools sent to API: {}",
            serde_json::to_string_pretty(&responses_request["tools"]).unwrap_or_default()
        );
    }

    // Add tool_choice if present
    if let Some(tool_choice) = &request.tool_choice {
        responses_request["tool_choice"] = match tool_choice {
            ToolChoice::Auto => json!("auto"),
            ToolChoice::Any => json!("auto"), // Responses API uses "auto" for any
            ToolChoice::None => json!("none"),
        };
    }

    // Note: GPT-5 Codex doesn't support temperature parameter, so we omit it

    serde_json::to_string(&responses_request)
}

fn normalize_tool_parameters(parameters: &serde_json::Value) -> serde_json::Value {
    match parameters {
        serde_json::Value::Object(map) => {
            let mut normalized = map.clone();

            // Ensure the schema has a type field
            if !normalized.contains_key("type") {
                normalized.insert(
                    "type".to_string(),
                    serde_json::Value::String("object".to_string()),
                );
            }

            // Ensure the schema has a properties field
            if !normalized.contains_key("properties") {
                normalized.insert(
                    "properties".to_string(),
                    serde_json::Value::Object(Default::default()),
                );
            }

            // Ensure required fields are preserved - this is crucial for GPT-5-Codex
            // The original schema should already have the required array, but let's make sure it's not lost
            log::debug!(
                "Tool schema normalization: input has required field = {}, required = {:?}",
                map.contains_key("required"),
                map.get("required")
            );

            serde_json::Value::Object(normalized)
        }
        _ => {
            log::warn!("Tool parameters are not an object, creating default schema");
            json!({
                "type": "object",
                "properties": {}
            })
        }
    }
}

fn json_value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(s) => Some(s.clone()),
        _ => serde_json::to_string(value).ok(),
    }
}

fn tool_chunk_from_function_call(
    index: usize,
    item_id: String,
    call_id: Option<String>,
    name: Option<String>,
    function: Value,
    arguments: Option<String>,
    arguments_delta: Option<String>,
    arguments_json_patch: Option<Value>,
) -> Option<ToolCallChunk> {
    let fallback_id = call_id.clone().or_else(|| Some(item_id.clone()));

    let mut function_map = match function {
        Value::Object(map) => map,
        Value::Null => Map::new(),
        other => {
            let mut map = Map::new();
            map.insert("value".to_string(), other);
            map
        }
    };

    if let Some(name_value) = name.clone() {
        function_map
            .entry("name".to_string())
            .or_insert(Value::String(name_value));
    }

    if let Some(arguments) = arguments {
        function_map.insert("arguments".to_string(), Value::String(arguments));
    }

    if let Some(arguments_delta) = arguments_delta {
        function_map.insert(
            "arguments_delta".to_string(),
            Value::String(arguments_delta),
        );
    }

    if let Some(arguments_json_patch) = arguments_json_patch {
        function_map.insert("arguments_json_patch".to_string(), arguments_json_patch);
    }

    let mut container = Map::new();
    if let Some(call_id) = call_id.clone() {
        container.insert("call_id".to_string(), Value::String(call_id));
    }
    container.insert("id".to_string(), Value::String(item_id.clone()));
    if let Some(name_value) = name.clone() {
        container.insert("name".to_string(), Value::String(name_value));
    }
    container.insert("function".to_string(), Value::Object(function_map));

    let container_value = Value::Object(container);
    let chunk = tool_chunk_from_value(index, fallback_id, &container_value);
    if let Some(chunk) = &chunk {
        log::debug!(
            "tool_chunk_from_function_call -> id={:?}, name={:?}, args={:?}",
            chunk.id,
            chunk
                .function
                .as_ref()
                .and_then(|f| f.name.as_ref())
                .map(|s| s.as_str()),
            chunk
                .function
                .as_ref()
                .and_then(|f| f.arguments.as_ref())
                .map(|s| s.as_str())
        );
    } else {
        log::debug!(
            "tool_chunk_from_function_call produced None (item_id={}, call_id={:?}, name={:?})",
            item_id,
            call_id,
            name
        );
    }
    chunk
}

fn tool_chunk_from_value(
    index: usize,
    fallback_id: Option<String>,
    value: &Value,
) -> Option<ToolCallChunk> {
    let call_id = value
        .get("call_id")
        .and_then(Value::as_str)
        .or_else(|| value.get("id").and_then(Value::as_str))
        .or_else(|| fallback_id.as_deref())
        .map(|s| s.to_string());

    let function_value = value.get("function").unwrap_or(value);
    let mut name = function_value
        .get("name")
        .and_then(Value::as_str)
        .map(|s| s.to_string())
        .or_else(|| {
            value
                .get("name")
                .and_then(Value::as_str)
                .map(|s| s.to_string())
        });

    let arguments = function_value
        .get("arguments")
        .and_then(json_value_to_string)
        .or_else(|| {
            function_value
                .get("arguments_json_patch")
                .and_then(json_value_to_string)
        })
        .or_else(|| {
            function_value
                .get("arguments_delta")
                .and_then(json_value_to_string)
        });

    if name.as_ref().is_some_and(|n| n.trim().is_empty()) {
        name = None;
    }

    if name.is_none() {
        name = call_id.clone();
    }

    let function_chunk = if name.is_some() || arguments.is_some() {
        Some(FunctionChunk { name, arguments })
    } else {
        None
    };

    if call_id.is_none() && function_chunk.is_none() {
        log::debug!(
            "tool_chunk_from_value dropping entry: missing id and function (fallback_id={:?}, value={})",
            fallback_id,
            value
        );
        return None;
    }

    let chunk = ToolCallChunk {
        index,
        id: call_id,
        function: function_chunk,
    };

    log::debug!(
        "tool_chunk_from_value -> id={:?}, name={:?}, args={:?}",
        chunk.id,
        chunk
            .function
            .as_ref()
            .and_then(|f| f.name.as_ref())
            .map(|s| s.as_str()),
        chunk
            .function
            .as_ref()
            .and_then(|f| f.arguments.as_ref())
            .map(|s| s.as_str())
    );

    Some(chunk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_chunk_from_function_call_prefers_top_level_arguments() {
        let chunk = tool_chunk_from_function_call(
            0,
            "item".to_string(),
            Some("call".to_string()),
            Some("read_file".to_string()),
            serde_json::json!({}),
            Some("{\"path\":\"/Users/example/project/file.rs\"}".to_string()),
            None,
            None,
        )
        .expect("expected chunk");

        let function = chunk.function.expect("expected function payload");
        assert_eq!(
            function.arguments.as_deref(),
            Some("{\"path\":\"/Users/example/project/file.rs\"}")
        );
    }

    #[test]
    fn tool_chunk_from_function_call_preserves_arguments_delta() {
        let chunk = tool_chunk_from_function_call(
            1,
            "item".to_string(),
            Some("call".to_string()),
            Some("read_file".to_string()),
            serde_json::json!({}),
            None,
            Some("{\"path\":\"src/lib.rs\"}".to_string()),
            None,
        )
        .expect("expected chunk");

        let function = chunk.function.expect("expected function payload");
        assert_eq!(
            function.arguments.as_deref(),
            Some("{\"path\":\"src/lib.rs\"}")
        );
    }

    #[test]
    fn test_resilient_model_schema_deserialize() {
        let json = r#"{
              "data": [
                {
                  "billing": {
                    "is_premium": false,
                    "multiplier": 0
                  },
                  "capabilities": {
                    "family": "gpt-4",
                    "limits": {
                      "max_context_window_tokens": 8192,
                      "max_output_tokens": 8192
                    },
                    "object": "model_capabilities",
                    "supports": {
                      "reasoning": false,
                      "tool_calling": true,
                      "vision": false
                    },
                    "tokenizer": "cl100k_base",
                    "type": "text"
                  },
                  "endpoints": ["chat/completions"],
                  "id": "gpt-4",
                  "name": "GPT-4",
                  "object": "model",
                  "policy": {
                    "state": "allowed"
                  },
                  "vendor": "unknown"
                }
              ]
        }"#;

        let schema: ModelSchema = serde_json::from_str(json).unwrap();
        assert_eq!(schema.data.len(), 1);
        assert_eq!(schema.data[0].vendor, ModelVendor::Unknown);
    }

    #[test]
    fn test_unknown_vendor_resilience() {
        let json = r#"{
              "data": [
                {
                  "billing": {
                    "is_premium": false,
                    "multiplier": 1
                  },
                  "capabilities": {
                    "family": "future-model",
                    "limits": {
                      "max_context_window_tokens": 128000,
                      "max_output_tokens": 8192,
                      "max_prompt_tokens": 120000
                    },
                    "object": "model_capabilities",
                    "supports": { "streaming": true, "tool_calls": true },
                    "type": "chat"
                  },
                  "id": "future-model-v1",
                  "is_chat_default": false,
                  "is_chat_fallback": false,
                  "model_picker_enabled": true,
                  "name": "Future Model v1",
                  "object": "model",
                  "preview": false,
                  "vendor": "SomeNewVendor",
                  "version": "v1.0"
                }
              ],
              "object": "list"
            }"#;

        let schema: ModelSchema = serde_json::from_str(json).unwrap();

        assert_eq!(schema.data.len(), 1);
        assert_eq!(schema.data[0].id, "future-model-v1");
        assert_eq!(schema.data[0].vendor, ModelVendor::Unknown);
    }
}

fn map_messages_to_responses_input(messages: &[ChatMessage]) -> Vec<ResponseInputItem> {
    let mut input_items = Vec::new();

    for message in messages {
        match message {
            ChatMessage::System { content } => {
                input_items.push(ResponseInputItem::SystemMessage {
                    role: "system".to_string(),
                    content: vec![ResponseInputContent::Text {
                        type_field: "input_text".to_string(),
                        text: content.clone(),
                    }],
                });
            }
            ChatMessage::User { content } => {
                let content_parts = match content {
                    ChatMessageContent::Plain(text) => {
                        vec![ResponseInputContent::Text {
                            type_field: "input_text".to_string(),
                            text: text.clone(),
                        }]
                    }
                    ChatMessageContent::Multipart(parts) => parts
                        .iter()
                        .map(|part| match part {
                            ChatMessagePart::Text { text } => ResponseInputContent::Text {
                                type_field: "input_text".to_string(),
                                text: text.clone(),
                            },
                            ChatMessagePart::Image { image_url } => {
                                ResponseInputContent::ImageUrl {
                                    type_field: "input_image".to_string(),
                                    image_url: image_url.url.clone(),
                                    detail: None,
                                }
                            }
                        })
                        .collect(),
                };

                input_items.push(ResponseInputItem::UserMessage {
                    role: "user".to_string(),
                    content: content_parts,
                });
            }
            ChatMessage::Assistant {
                content,
                tool_calls,
            } => {
                // Add assistant message if there's text content
                let text_content = match content {
                    ChatMessageContent::Plain(text) => text.clone(),
                    ChatMessageContent::Multipart(parts) => parts
                        .iter()
                        .filter_map(|part| match part {
                            ChatMessagePart::Text { text } => Some(text.as_str()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join(" "),
                };

                if !text_content.is_empty() {
                    input_items.push(ResponseInputItem::AssistantMessage {
                        role: "assistant".to_string(),
                        content: vec![ResponseOutputContent::Text {
                            type_field: "output_text".to_string(),
                            text: text_content,
                        }],
                        type_field: "message".to_string(),
                        status: "completed".to_string(),
                        id: format!("msg_{}", MESSAGE_COUNTER.fetch_add(1, Ordering::Relaxed)),
                    });
                }

                // Add tool calls if present
                for tool_call in tool_calls {
                    match &tool_call.content {
                        ToolCallContent::Function { function } => {
                            if function.name.trim().is_empty() {
                                log::warn!(
                                    "Skipping assistant tool call with empty name (id={}, args={})",
                                    tool_call.id,
                                    function.arguments
                                );
                                continue;
                            }

                            log::debug!(
                                "Copilot Responses request tool call: id={}, name={}, args={}",
                                tool_call.id,
                                function.name,
                                function.arguments
                            );

                            input_items.push(ResponseInputItem::FunctionCall {
                                type_field: "function_call".to_string(),
                                name: function.name.clone(),
                                arguments: function.arguments.clone(),
                                call_id: tool_call.id.clone(),
                            });
                        }
                    }
                }
            }
            ChatMessage::Tool {
                content,
                tool_call_id,
            } => {
                let mut text_fragments = Vec::new();
                let mut image_contents: Vec<ResponseInputContent> = Vec::new();

                match content {
                    ChatMessageContent::Plain(text) => text_fragments.push(text.clone()),
                    ChatMessageContent::Multipart(parts) => {
                        for part in parts {
                            match part {
                                ChatMessagePart::Text { text } => text_fragments.push(text.clone()),
                                ChatMessagePart::Image { image_url } => {
                                    image_contents.push(ResponseInputContent::ImageUrl {
                                        type_field: "input_image".to_string(),
                                        image_url: image_url.url.clone(),
                                        detail: None,
                                    });
                                }
                            }
                        }
                    }
                }

                let output_text = text_fragments.join(" ");

                input_items.push(ResponseInputItem::FunctionCallOutput {
                    type_field: "function_call_output".to_string(),
                    call_id: tool_call_id.clone(),
                    output: output_text,
                });

                if !image_contents.is_empty() {
                    let mut content_parts = Vec::with_capacity(image_contents.len() + 1);
                    content_parts.push(ResponseInputContent::Text {
                        type_field: "input_text".to_string(),
                        text: "Image associated with the above tool call:".to_string(),
                    });
                    content_parts.extend(image_contents);

                    input_items.push(ResponseInputItem::UserMessage {
                        role: "user".to_string(),
                        content: content_parts,
                    });
                }
            }
        }
    }

    input_items
}

pub const COPILOT_OAUTH_ENV_VAR: &str = "GH_COPILOT_TOKEN";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct CopilotChatConfiguration {
    pub enterprise_uri: Option<String>,
}

impl CopilotChatConfiguration {
    pub fn token_url(&self) -> String {
        if let Some(enterprise_uri) = &self.enterprise_uri {
            let domain = Self::parse_domain(enterprise_uri);
            format!("https://api.{}/copilot_internal/v2/token", domain)
        } else {
            "https://api.github.com/copilot_internal/v2/token".to_string()
        }
    }

    pub fn oauth_domain(&self) -> String {
        if let Some(enterprise_uri) = &self.enterprise_uri {
            Self::parse_domain(enterprise_uri)
        } else {
            "github.com".to_string()
        }
    }

    pub fn api_url_from_endpoint(&self, endpoint: &str, use_responses_api: bool) -> String {
        if use_responses_api {
            format!("{}/responses", endpoint)
        } else {
            format!("{}/chat/completions", endpoint)
        }
    }

    pub fn models_url_from_endpoint(&self, endpoint: &str) -> String {
        format!("{}/models", endpoint)
    }

    fn parse_domain(enterprise_uri: &str) -> String {
        let uri = enterprise_uri.trim_end_matches('/');

        if let Some(domain) = uri.strip_prefix("https://") {
            domain.split('/').next().unwrap_or(domain).to_string()
        } else if let Some(domain) = uri.strip_prefix("http://") {
            domain.split('/').next().unwrap_or(domain).to_string()
        } else {
            uri.split('/').next().unwrap_or(uri).to_string()
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Deserialize)]
struct ModelSchema {
    #[serde(deserialize_with = "deserialize_models_skip_errors")]
    data: Vec<Model>,
}

fn deserialize_models_skip_errors<'de, D>(deserializer: D) -> Result<Vec<Model>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw_values = Vec::<serde_json::Value>::deserialize(deserializer)?;
    let models = raw_values
        .into_iter()
        .filter_map(|value| match serde_json::from_value::<Model>(value) {
            Ok(model) => Some(model),
            Err(err) => {
                log::warn!("GitHub Copilot Chat model failed to deserialize: {:?}", err);
                None
            }
        })
        .collect();

    Ok(models)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Model {
    billing: ModelBilling,
    capabilities: ModelCapabilities,
    id: String,
    name: String,
    policy: Option<ModelPolicy>,
    vendor: ModelVendor,
    is_chat_default: bool,
    // The model with this value true is selected by VSCode copilot if a premium request limit is
    // reached. Zed does not currently implement this behaviour
    is_chat_fallback: bool,
    model_picker_enabled: bool,
    #[serde(default)]
    supported_endpoints: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct ModelBilling {
    is_premium: bool,
    multiplier: f64,
    // List of plans a model is restricted to
    // Field is not present if a model is available for all plans
    #[serde(default)]
    restricted_to: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ModelCapabilities {
    family: String,
    #[serde(default)]
    limits: ModelLimits,
    supports: ModelSupportedFeatures,
    #[serde(rename = "type")]
    model_type: String,
    #[serde(default)]
    tokenizer: Option<String>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ModelLimits {
    #[serde(default)]
    max_context_window_tokens: usize,
    #[serde(default)]
    max_output_tokens: usize,
    #[serde(default)]
    max_prompt_tokens: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ModelPolicy {
    state: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ModelSupportedFeatures {
    #[serde(default)]
    streaming: bool,
    #[serde(default)]
    tool_calls: bool,
    #[serde(default)]
    parallel_tool_calls: bool,
    #[serde(default)]
    vision: bool,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ModelVendor {
    // Azure OpenAI should have no functional difference from OpenAI in Copilot Chat
    #[serde(alias = "Azure OpenAI")]
    OpenAI,
    Google,
    Anthropic,
    #[serde(rename = "xAI")]
    XAI,
    /// Unknown vendor that we don't explicitly support yet
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum ChatMessagePart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    Image { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ImageUrl {
    pub url: String,
}

impl Model {
    pub fn uses_streaming(&self) -> bool {
        self.capabilities.supports.streaming
    }

    pub fn uses_responses_api(&self) -> bool {
        // Prefer the metadata advertised by GitHub's models endpoint when available.
        if !self.supported_endpoints.is_empty() {
            let has_responses = self
                .supported_endpoints
                .iter()
                .any(|endpoint| endpoint == "/responses");
            let has_chat_completions = self
                .supported_endpoints
                .iter()
                .any(|endpoint| endpoint == "/chat/completions");

            if has_responses && !has_chat_completions {
                return true;
            }
        }

        // Fallback for older metadata payloads that omit supported_endpoints.
        self.capabilities.family.eq_ignore_ascii_case("gpt-5-codex")
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn display_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn max_token_count(&self) -> u64 {
        self.capabilities.limits.max_prompt_tokens
    }

    pub fn supports_tools(&self) -> bool {
        self.capabilities.supports.tool_calls
    }

    pub fn vendor(&self) -> ModelVendor {
        self.vendor
    }

    pub fn supports_vision(&self) -> bool {
        self.capabilities.supports.vision
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        self.capabilities.supports.parallel_tool_calls
    }

    pub fn tokenizer(&self) -> Option<&str> {
        self.capabilities.tokenizer.as_deref()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Request {
    pub intent: bool,
    pub n: usize,
    pub stream: bool,
    pub temperature: f32,
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip)]
    pub use_responses_api: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
    Function { function: Function },
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Any,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    Assistant {
        content: ChatMessageContent,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
    },
    User {
        content: ChatMessageContent,
    },
    System {
        content: String,
    },
    Tool {
        content: ChatMessageContent,
        tool_call_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ChatMessageContent {
    Plain(String),
    Multipart(Vec<ChatMessagePart>),
}

impl ChatMessageContent {
    pub fn empty() -> Self {
        ChatMessageContent::Multipart(vec![])
    }
}

impl From<Vec<ChatMessagePart>> for ChatMessageContent {
    fn from(mut parts: Vec<ChatMessagePart>) -> Self {
        if let [ChatMessagePart::Text { text }] = parts.as_mut_slice() {
            ChatMessageContent::Plain(std::mem::take(text))
        } else {
            ChatMessageContent::Multipart(parts)
        }
    }
}

impl From<String> for ChatMessageContent {
    fn from(text: String) -> Self {
        ChatMessageContent::Plain(text)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(flatten)]
    pub content: ToolCallContent,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolCallContent {
    Function { function: FunctionContent },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FunctionContent {
    pub name: String,
    pub arguments: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct ResponseEvent {
    pub choices: Vec<ResponseChoice>,
    pub id: String,
    pub usage: Option<Usage>,
}

// Response structure for Responses API (used by GPT-5 Codex)
#[derive(Deserialize, Debug)]
pub struct ResponsesApiStreamEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub response: Option<ResponsesApiResponse>,
    pub delta: Option<String>,
    pub text: Option<String>,
    pub item_id: Option<String>,
    pub output_index: Option<usize>,
    pub content_index: Option<usize>,
    // Tool call fields
    pub tool_call_id: Option<String>,
    pub tool: Option<serde_json::Value>,
    #[serde(default)]
    pub item: Option<serde_json::Value>,
    #[serde(default)]
    pub error: Option<ResponsesApiStreamError>,
}

#[derive(Deserialize, Debug)]
pub struct ResponsesApiWrapper {
    pub response: ResponsesApiResponse,
}

#[derive(Deserialize, Debug)]
pub struct ResponsesApiResponse {
    pub id: String,
    pub object: String,
    pub status: String,
    pub output: Option<Vec<ResponsesApiOutput>>,
    pub usage: Option<Usage>,
    // Optional fields that may be present
    #[serde(default)]
    pub background: bool,
    pub created_at: Option<i64>,
    pub error: Option<serde_json::Value>,
    pub incomplete_details: Option<serde_json::Value>,
    pub instructions: Option<String>,
    pub max_output_tokens: Option<i64>,
    pub max_tool_calls: Option<i64>,
    #[serde(default)]
    pub metadata: serde_json::Value,
    pub model: Option<String>,
    #[serde(default)]
    pub parallel_tool_calls: bool,
    pub previous_response_id: Option<String>,
    pub prompt_cache_key: Option<String>,
    pub reasoning: Option<serde_json::Value>,
    pub safety_identifier: Option<String>,
    pub service_tier: Option<String>,
    #[serde(default)]
    pub store: bool,
    pub temperature: Option<f64>,
    pub text: Option<serde_json::Value>,
    pub tool_choice: Option<String>,
    #[serde(default)]
    pub tools: Vec<serde_json::Value>,
    pub top_logprobs: Option<i64>,
    pub top_p: Option<f64>,
    pub truncation: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ResponsesApiOutput {
    #[serde(rename = "message")]
    Message {
        id: String,
        status: String,
        role: String,
        content: Vec<ResponsesApiContent>,
    },
    #[serde(rename = "function_call")]
    FunctionCall {
        id: String,
        #[serde(default)]
        call_id: Option<String>,
        #[serde(default)]
        name: Option<String>,
        #[serde(default)]
        function: Value,
        #[serde(default)]
        arguments: Option<String>,
        #[serde(default)]
        arguments_delta: Option<String>,
        #[serde(default)]
        arguments_json_patch: Option<Value>,
        #[serde(default)]
        status: Option<String>,
    },
    #[serde(rename = "reasoning")]
    Reasoning {
        id: String,
        summary: Vec<serde_json::Value>,
    },
}

#[derive(Deserialize, Debug)]
pub struct ResponsesApiContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
    #[serde(default)]
    pub annotations: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct ResponsesApiStreamError {
    pub message: String,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub param: Option<String>,
}

impl ResponsesApiStreamEvent {
    fn into_response_event(self) -> Option<Result<ResponseEvent>> {
        log::debug!(
            "Processing ResponsesApiStreamEvent: type={}, delta={:?}, text={:?}",
            self.event_type,
            self.delta,
            self.text
        );

        let index = self.output_index.unwrap_or(0);
        let item_id = self
            .item_id
            .clone()
            .unwrap_or_else(|| "unknown".to_string());

        match self.event_type.as_str() {
            "response.output_text.delta" => {
                let delta_text = self.delta?;
                let choice = ResponseChoice {
                    index,
                    finish_reason: None,
                    delta: Some(ResponseDelta {
                        content: Some(delta_text),
                        role: Some(Role::Assistant),
                        tool_calls: vec![],
                    }),
                    message: None,
                };

                Some(Ok(ResponseEvent {
                    choices: vec![choice],
                    id: item_id,
                    usage: None,
                }))
            }
            "response.content_part.added" => {
                // Kick off a new content part so downstream consumers know to display streaming text.
                let choice = ResponseChoice {
                    index,
                    finish_reason: None,
                    delta: Some(ResponseDelta {
                        content: Some(String::new()),
                        role: Some(Role::Assistant),
                        tool_calls: vec![],
                    }),
                    message: None,
                };

                Some(Ok(ResponseEvent {
                    choices: vec![choice],
                    id: item_id,
                    usage: None,
                }))
            }
            "response.output_text.done" => {
                let choice = ResponseChoice {
                    index,
                    finish_reason: None,
                    delta: Some(ResponseDelta {
                        content: None,
                        role: Some(Role::Assistant),
                        tool_calls: vec![],
                    }),
                    message: None,
                };

                Some(Ok(ResponseEvent {
                    choices: vec![choice],
                    id: item_id,
                    usage: None,
                }))
            }
            "response.output_item.added" => {
                let item_value = self.item?;
                match serde_json::from_value::<ResponsesApiOutput>(item_value) {
                    Ok(ResponsesApiOutput::FunctionCall {
                        id,
                        call_id,
                        name,
                        function,
                        arguments,
                        arguments_delta,
                        arguments_json_patch,
                        ..
                    }) => {
                        let chunk = tool_chunk_from_function_call(
                            index,
                            id,
                            call_id,
                            name,
                            function,
                            arguments,
                            arguments_delta,
                            arguments_json_patch,
                        )
                        .or_else(|| {
                            self.tool.as_ref().and_then(|value| {
                                tool_chunk_from_value(index, self.tool_call_id.clone(), value)
                            })
                        });

                        if let Some(mut chunk) = chunk {
                            if chunk.id.is_none() {
                                chunk.id = self.tool_call_id.clone();
                            }

                            log::debug!(
                                "Responses stream tool_call.added -> id={:?}, name={:?}, args={:?}",
                                chunk.id,
                                chunk
                                    .function
                                    .as_ref()
                                    .and_then(|f| f.name.as_ref())
                                    .map(|s| s.as_str()),
                                chunk
                                    .function
                                    .as_ref()
                                    .and_then(|f| f.arguments.as_ref())
                                    .map(|s| s.as_str())
                            );

                            let choice = ResponseChoice {
                                index,
                                finish_reason: None,
                                delta: Some(ResponseDelta {
                                    content: None,
                                    role: Some(Role::Assistant),
                                    tool_calls: vec![chunk],
                                }),
                                message: None,
                            };

                            Some(Ok(ResponseEvent {
                                choices: vec![choice],
                                id: item_id,
                                usage: None,
                            }))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            "response.output_item.done" => {
                let item_value = self.item?;
                match serde_json::from_value::<ResponsesApiOutput>(item_value) {
                    Ok(ResponsesApiOutput::FunctionCall {
                        id,
                        call_id,
                        name,
                        function,
                        arguments,
                        arguments_delta,
                        arguments_json_patch,
                        ..
                    }) => {
                        let chunk = tool_chunk_from_function_call(
                            index,
                            id,
                            call_id,
                            name,
                            function,
                            arguments,
                            arguments_delta,
                            arguments_json_patch,
                        )
                        .or_else(|| {
                            self.tool.as_ref().and_then(|value| {
                                tool_chunk_from_value(index, self.tool_call_id.clone(), value)
                            })
                        });

                        if let Some(mut chunk) = chunk {
                            if chunk.id.is_none() {
                                chunk.id = self.tool_call_id.clone();
                            }

                            log::debug!(
                                "Responses stream tool_call.done -> id={:?}, name={:?}, args={:?}",
                                chunk.id,
                                chunk
                                    .function
                                    .as_ref()
                                    .and_then(|f| f.name.as_ref())
                                    .map(|s| s.as_str()),
                                chunk
                                    .function
                                    .as_ref()
                                    .and_then(|f| f.arguments.as_ref())
                                    .map(|s| s.as_str())
                            );
                            let choice = ResponseChoice {
                                index,
                                finish_reason: None,
                                delta: Some(ResponseDelta {
                                    content: None,
                                    role: Some(Role::Assistant),
                                    tool_calls: vec![chunk],
                                }),
                                message: None,
                            };

                            Some(Ok(ResponseEvent {
                                choices: vec![choice],
                                id: item_id,
                                usage: None,
                            }))
                        } else {
                            None
                        }
                    }
                    Ok(ResponsesApiOutput::Message { .. })
                    | Ok(ResponsesApiOutput::Reasoning { .. }) => None,
                    Err(err) => Some(Err(anyhow!(
                        "Failed to decode Responses API output item: {err}"
                    ))),
                }
            }
            "response.completed" => {
                let response = self.response?;
                let choices = response
                    .output
                    .unwrap_or_default()
                    .into_iter()
                    .enumerate()
                    .filter_map(|(choice_index, output)| match output {
                        ResponsesApiOutput::Message { role, content, .. } => {
                            let text_content = content
                                .into_iter()
                                .filter_map(|c| c.text)
                                .collect::<Vec<_>>()
                                .join("");

                            let role = match role.as_str() {
                                "user" => Role::User,
                                "system" => Role::System,
                                _ => Role::Assistant,
                            };

                            let message_delta = if text_content.is_empty() {
                                None
                            } else {
                                Some(ResponseDelta {
                                    content: Some(text_content),
                                    role: Some(role),
                                    tool_calls: vec![],
                                })
                            };

                            Some(ResponseChoice {
                                index: choice_index,
                                finish_reason: Some("stop".to_string()),
                                delta: Some(ResponseDelta {
                                    content: None,
                                    role: Some(role),
                                    tool_calls: vec![],
                                }),
                                message: message_delta,
                            })
                        }
                        ResponsesApiOutput::FunctionCall {
                            id,
                            call_id,
                            name,
                            function,
                            arguments,
                            arguments_delta,
                            arguments_json_patch,
                            ..
                        } => {
                            let chunk = tool_chunk_from_function_call(
                                choice_index,
                                id,
                                call_id,
                                name,
                                function,
                                arguments,
                                arguments_delta,
                                arguments_json_patch,
                            )
                            .or_else(|| {
                                self.tool.as_ref().and_then(|value| {
                                    tool_chunk_from_value(
                                        choice_index,
                                        self.tool_call_id.clone(),
                                        value,
                                    )
                                })
                            });

                            if let Some(mut chunk) = chunk {
                                if chunk.id.is_none() {
                                    chunk.id = self.tool_call_id.clone();
                                }

                                log::debug!(
                                    "Responses stream completion tool_call -> id={:?}, name={:?}, args={:?}",
                                    chunk.id,
                                    chunk
                                        .function
                                        .as_ref()
                                        .and_then(|f| f.name.as_ref())
                                        .map(|s| s.as_str()),
                                    chunk
                                        .function
                                        .as_ref()
                                        .and_then(|f| f.arguments.as_ref())
                                        .map(|s| s.as_str())
                                );

                                Some(ResponseChoice {
                                    index: choice_index,
                                    finish_reason: Some("tool_calls".to_string()),
                                    delta: Some(ResponseDelta {
                                        content: None,
                                        role: Some(Role::Assistant),
                                        tool_calls: vec![chunk],
                                    }),
                                    message: None,
                                })
                            } else {
                                None
                            }
                        }
                        ResponsesApiOutput::Reasoning { .. } => None,
                    })
                    .collect::<Vec<_>>();

                if choices.is_empty() {
                    return None;
                }

                Some(Ok(ResponseEvent {
                    choices,
                    id: response.id,
                    usage: response.usage,
                }))
            }
            "error" => {
                let err = self
                    .error
                    .map(|error| {
                        if let Some(code) = error.code {
                            anyhow!("Responses API error ({code}): {}", error.message)
                        } else {
                            anyhow!("Responses API error: {}", error.message)
                        }
                    })
                    .unwrap_or_else(|| anyhow!("Responses API error"));

                Some(Err(err))
            }
            "response.tool_call.delta" => {
                // Partial tool call deltas are optional; capture them when tool payload is provided.
                let mut chunk = self.tool.as_ref().and_then(|value| {
                    tool_chunk_from_value(index, self.tool_call_id.clone(), value)
                });

                if chunk.is_none() {
                    if let Some(delta_text) = self.delta.clone() {
                        if self.tool_call_id.is_some() {
                            chunk = Some(ToolCallChunk {
                                index,
                                id: self.tool_call_id.clone(),
                                function: Some(FunctionChunk {
                                    name: None,
                                    arguments: Some(delta_text),
                                }),
                            });
                        }
                    }
                }

                if let Some(mut chunk) = chunk {
                    if chunk.id.is_none() {
                        chunk.id = self.tool_call_id.clone();
                    }

                    let choice = ResponseChoice {
                        index,
                        finish_reason: None,
                        delta: Some(ResponseDelta {
                            content: None,
                            role: Some(Role::Assistant),
                            tool_calls: vec![chunk],
                        }),
                        message: None,
                    };

                    return Some(Ok(ResponseEvent {
                        choices: vec![choice],
                        id: item_id,
                        usage: None,
                    }));
                }

                None
            }
            _ => None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    #[serde(alias = "output_tokens")]
    pub completion_tokens: u64,
    #[serde(alias = "input_tokens")]
    pub prompt_tokens: u64,
    #[serde(default)]
    pub total_tokens: u64,
    #[serde(alias = "output_tokens_details", default)]
    pub output_tokens_details: Option<ResponsesUsageTokenDetails>,
    #[serde(alias = "input_tokens_details", default)]
    pub input_tokens_details: Option<ResponsesUsageTokenDetails>,
}

#[derive(Deserialize, Debug)]
pub struct ResponsesUsageTokenDetails {
    #[serde(default)]
    pub reasoning_tokens: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseChoice {
    pub index: usize,
    pub finish_reason: Option<String>,
    pub delta: Option<ResponseDelta>,
    pub message: Option<ResponseDelta>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseDelta {
    pub content: Option<String>,
    pub role: Option<Role>,
    #[serde(default)]
    pub tool_calls: Vec<ToolCallChunk>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[derive(Deserialize)]
struct ApiTokenResponse {
    token: String,
    expires_at: i64,
    endpoints: ApiTokenResponseEndpoints,
}

#[derive(Deserialize)]
struct ApiTokenResponseEndpoints {
    api: String,
}

#[derive(Clone)]
struct ApiToken {
    api_key: String,
    expires_at: DateTime<chrono::Utc>,
    api_endpoint: String,
}

impl ApiToken {
    pub fn remaining_seconds(&self) -> i64 {
        self.expires_at
            .timestamp()
            .saturating_sub(chrono::Utc::now().timestamp())
    }
}

impl TryFrom<ApiTokenResponse> for ApiToken {
    type Error = anyhow::Error;

    fn try_from(response: ApiTokenResponse) -> Result<Self, Self::Error> {
        let expires_at =
            DateTime::from_timestamp(response.expires_at, 0).context("invalid expires_at")?;

        Ok(Self {
            api_key: response.token,
            expires_at,
            api_endpoint: response.endpoints.api,
        })
    }
}

struct GlobalCopilotChat(gpui::Entity<CopilotChat>);

impl Global for GlobalCopilotChat {}

pub struct CopilotChat {
    oauth_token: Option<String>,
    api_token: Option<ApiToken>,
    configuration: CopilotChatConfiguration,
    models: Option<Vec<Model>>,
    client: Arc<dyn HttpClient>,
}

pub fn init(
    fs: Arc<dyn Fs>,
    client: Arc<dyn HttpClient>,
    configuration: CopilotChatConfiguration,
    cx: &mut App,
) {
    let copilot_chat = cx.new(|cx| CopilotChat::new(fs, client, configuration, cx));
    cx.set_global(GlobalCopilotChat(copilot_chat));
}

pub fn copilot_chat_config_dir() -> &'static PathBuf {
    static COPILOT_CHAT_CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();

    COPILOT_CHAT_CONFIG_DIR.get_or_init(|| {
        let config_dir = if cfg!(target_os = "windows") {
            dirs::data_local_dir().expect("failed to determine LocalAppData directory")
        } else {
            std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir().join(".config"))
        };

        config_dir.join("github-copilot")
    })
}

fn copilot_chat_config_paths() -> [PathBuf; 2] {
    let base_dir = copilot_chat_config_dir();
    [base_dir.join("hosts.json"), base_dir.join("apps.json")]
}

impl CopilotChat {
    pub fn global(cx: &App) -> Option<gpui::Entity<Self>> {
        cx.try_global::<GlobalCopilotChat>()
            .map(|model| model.0.clone())
    }

    fn new(
        fs: Arc<dyn Fs>,
        client: Arc<dyn HttpClient>,
        configuration: CopilotChatConfiguration,
        cx: &mut Context<Self>,
    ) -> Self {
        let config_paths: HashSet<PathBuf> = copilot_chat_config_paths().into_iter().collect();
        let dir_path = copilot_chat_config_dir();

        cx.spawn(async move |this, cx| {
            let mut parent_watch_rx = watch_config_dir(
                cx.background_executor(),
                fs.clone(),
                dir_path.clone(),
                config_paths,
            );
            while let Some(contents) = parent_watch_rx.next().await {
                let oauth_domain =
                    this.read_with(cx, |this, _| this.configuration.oauth_domain())?;
                let oauth_token = extract_oauth_token(contents, &oauth_domain);

                this.update(cx, |this, cx| {
                    this.oauth_token = oauth_token.clone();
                    cx.notify();
                })?;

                if oauth_token.is_some() {
                    Self::update_models(&this, cx).await?;
                }
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let this = Self {
            oauth_token: std::env::var(COPILOT_OAUTH_ENV_VAR).ok(),
            api_token: None,
            models: None,
            configuration,
            client,
        };

        if this.oauth_token.is_some() {
            cx.spawn(async move |this, cx| Self::update_models(&this, cx).await)
                .detach_and_log_err(cx);
        }

        this
    }

    async fn update_models(this: &WeakEntity<Self>, cx: &mut AsyncApp) -> Result<()> {
        let (oauth_token, client, configuration) = this.read_with(cx, |this, _| {
            (
                this.oauth_token.clone(),
                this.client.clone(),
                this.configuration.clone(),
            )
        })?;

        let oauth_token = oauth_token
            .ok_or_else(|| anyhow!("OAuth token is missing while updating Copilot Chat models"))?;

        let token_url = configuration.token_url();
        let api_token = request_api_token(&oauth_token, token_url.into(), client.clone()).await?;

        let models_url = configuration.models_url_from_endpoint(&api_token.api_endpoint);
        let models =
            get_models(models_url.into(), api_token.api_key.clone(), client.clone()).await?;

        this.update(cx, |this, cx| {
            this.api_token = Some(api_token);
            this.models = Some(models);
            cx.notify();
        })?;
        anyhow::Ok(())
    }

    pub fn is_authenticated(&self) -> bool {
        self.oauth_token.is_some()
    }

    pub fn models(&self) -> Option<&[Model]> {
        self.models.as_deref()
    }

    pub async fn stream_completion(
        request: Request,
        is_user_initiated: bool,
        mut cx: AsyncApp,
    ) -> Result<BoxStream<'static, Result<ResponseEvent>>> {
        let this = cx
            .update(|cx| Self::global(cx))
            .ok()
            .flatten()
            .context("Copilot chat is not enabled")?;

        let (oauth_token, api_token, client, configuration) = this.read_with(&cx, |this, _| {
            (
                this.oauth_token.clone(),
                this.api_token.clone(),
                this.client.clone(),
                this.configuration.clone(),
            )
        })?;

        let oauth_token = oauth_token.context("No OAuth token available")?;

        let token = match api_token {
            Some(api_token) if api_token.remaining_seconds() > 5 * 60 => api_token.clone(),
            _ => {
                let token_url = configuration.token_url();
                let token =
                    request_api_token(&oauth_token, token_url.into(), client.clone()).await?;
                this.update(&mut cx, |this, cx| {
                    this.api_token = Some(token.clone());
                    cx.notify();
                })?;
                token
            }
        };

        let api_url =
            configuration.api_url_from_endpoint(&token.api_endpoint, request.use_responses_api);
        stream_completion(
            client.clone(),
            token.api_key,
            api_url.into(),
            request,
            is_user_initiated,
        )
        .await
    }

    pub fn set_configuration(
        &mut self,
        configuration: CopilotChatConfiguration,
        cx: &mut Context<Self>,
    ) {
        let same_configuration = self.configuration == configuration;
        self.configuration = configuration;
        if !same_configuration {
            self.api_token = None;
            cx.spawn(async move |this, cx| {
                Self::update_models(&this, cx).await?;
                Ok::<_, anyhow::Error>(())
            })
            .detach();
        }
    }
}

async fn get_models(
    models_url: Arc<str>,
    api_token: String,
    client: Arc<dyn HttpClient>,
) -> Result<Vec<Model>> {
    let all_models = request_models(models_url, api_token, client).await?;

    let mut models: Vec<Model> = all_models
        .into_iter()
        .filter(|model| {
            model.model_picker_enabled
                && model.capabilities.model_type.as_str() == "chat"
                && model
                    .policy
                    .as_ref()
                    .is_none_or(|policy| policy.state == "enabled")
        })
        .dedup_by(|a, b| a.capabilities.family == b.capabilities.family)
        .collect();

    if let Some(default_model_position) = models.iter().position(|model| model.is_chat_default) {
        let default_model = models.remove(default_model_position);
        models.insert(0, default_model);
    }

    Ok(models)
}

async fn request_models(
    models_url: Arc<str>,
    api_token: String,
    client: Arc<dyn HttpClient>,
) -> Result<Vec<Model>> {
    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(models_url.as_ref())
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .header("Copilot-Integration-Id", "vscode-chat")
        .header("Editor-Version", "vscode/1.103.2")
        .header("x-github-api-version", "2025-05-01");

    let request = request_builder.body(AsyncBody::empty())?;

    let mut response = client.send(request).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to request models: {}",
        response.status()
    );
    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;

    let body_str = std::str::from_utf8(&body)?;

    let models = serde_json::from_str::<ModelSchema>(body_str)?.data;

    Ok(models)
}

async fn request_api_token(
    oauth_token: &str,
    auth_url: Arc<str>,
    client: Arc<dyn HttpClient>,
) -> Result<ApiToken> {
    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(auth_url.as_ref())
        .header("Authorization", format!("token {}", oauth_token))
        .header("Accept", "application/json");

    let request = request_builder.body(AsyncBody::empty())?;

    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;

        let parsed: ApiTokenResponse = serde_json::from_str(body_str)?;
        ApiToken::try_from(parsed)
    } else {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;
        anyhow::bail!("Failed to request API token: {body_str}");
    }
}

fn extract_oauth_token(contents: String, domain: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(&contents)
        .map(|v| {
            v.as_object().and_then(|obj| {
                obj.iter().find_map(|(key, value)| {
                    if key.starts_with(domain) {
                        value["oauth_token"].as_str().map(|v| v.to_string())
                    } else {
                        None
                    }
                })
            })
        })
        .ok()
        .flatten()
}

async fn stream_completion(
    client: Arc<dyn HttpClient>,
    api_key: String,
    completion_url: Arc<str>,
    request: Request,
    is_user_initiated: bool,
) -> Result<BoxStream<'static, Result<ResponseEvent>>> {
    const MAX_RETRIES: usize = 3;

    for attempt in 0..MAX_RETRIES {
        match stream_completion_inner(
            client.clone(),
            api_key.clone(),
            completion_url.clone(),
            request.clone(),
            is_user_initiated,
        )
        .await
        {
            Ok(stream) => return Ok(stream),
            Err(e) => {
                if attempt == MAX_RETRIES - 1 {
                    return Err(e);
                }
                // Simple retry without delay for now
                log::warn!(
                    "Retrying stream completion after error (attempt {}/{}): {}",
                    attempt + 1,
                    MAX_RETRIES,
                    e
                );
            }
        }
    }

    unreachable!()
}

async fn stream_completion_inner(
    client: Arc<dyn HttpClient>,
    api_key: String,
    completion_url: Arc<str>,
    request: Request,
    is_user_initiated: bool,
) -> Result<BoxStream<'static, Result<ResponseEvent>>> {
    let is_vision_request = request.messages.iter().any(|message| match message {
      ChatMessage::User { content }
      | ChatMessage::Assistant { content, .. }
      | ChatMessage::Tool { content, .. } => {
          matches!(content, ChatMessageContent::Multipart(parts) if parts.iter().any(|part| matches!(part, ChatMessagePart::Image { .. })))
      }
      _ => false,
  });

    let request_initiator = if is_user_initiated { "user" } else { "agent" };

    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(completion_url.as_ref())
        .header(
            "Editor-Version",
            format!(
                "Zed/{}",
                option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
            ),
        )
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("Copilot-Integration-Id", "vscode-chat")
        .header("X-Initiator", request_initiator)
        .when(is_vision_request, |builder| {
            builder.header("Copilot-Vision-Request", is_vision_request.to_string())
        });

    let is_streaming = request.stream;

    // Models that require the Responses API need a bespoke payload format
    let json = if request.use_responses_api {
        create_responses_request_body(&request)?
    } else {
        // Use standard Chat Completions API format
        serde_json::to_string(&request)?
    };
    let http_request = request_builder.body(AsyncBody::from(json))?;
    let mut response = client.send(http_request).await?;

    if !response.status().is_success() {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;
        let body_str = std::str::from_utf8(&body)?;
        anyhow::bail!(
            "Failed to connect to API: {} {}",
            response.status(),
            body_str
        );
    }

    if is_streaming {
        let uses_responses_api = request.use_responses_api;
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(move |line| {
                async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ")?;
                        if line.starts_with("[DONE]") {
                            return None;
                        }

                        // Parse based on model - Responses API models use streaming format with
                        // SSE payloads that differ from Chat Completions
                        if uses_responses_api {
                            // Try Responses API streaming format first
                            match serde_json::from_str::<ResponsesApiStreamEvent>(line) {
                                Ok(stream_event) => match stream_event.into_response_event() {
                                    Some(Ok(response)) => {
                                        if response.choices.is_empty() {
                                            None
                                        } else {
                                            Some(Ok(response))
                                        }
                                    }
                                    Some(Err(err)) => Some(Err(err)),
                                    None => None,
                                },
                                Err(parse_error) => {
                                    // Log the detailed parse error and raw line for debugging
                                    log::warn!("GPT-5 Codex response parsing failed with Responses API format: {}", parse_error);
                                    log::debug!("Raw response line: {}", line);

                                    // Check if this is a raw JSON response (non-streaming completion)
                                    if line.contains("\"object\":\"responses.response\"") || line.contains("\"object\":\"chat.completion\"") {
                                        log::debug!("Attempting to parse as complete response object");
                                        // Try parsing as complete Responses API response
                                        match serde_json::from_str::<ResponsesApiWrapper>(line) {
                                            Ok(wrapper) => {
                                                let response = wrapper.response;
                                                log::debug!("Successfully parsed complete response: id={}", response.id);

                                                // Convert to ResponseEvent format
                                                let choices = response
                                                    .output
                                                    .unwrap_or_default()
                                                    .into_iter()
                                                    .enumerate()
                                                    .filter_map(|(index, output)| match output {
                                                        ResponsesApiOutput::Message { role, content, .. } => {
                                                            let text_content = content
                                                                .into_iter()
                                                                .filter_map(|c| c.text)
                                                                .collect::<Vec<_>>()
                                                                .join("");

                                                            let role = match role.as_str() {
                                                                "user" => Role::User,
                                                                "system" => Role::System,
                                                                _ => Role::Assistant,
                                                            };

                                                        let message_delta = if text_content.is_empty() {
                                                            None
                                                        } else {
                                                            Some(ResponseDelta {
                                                                content: Some(text_content),
                                                                role: Some(role),
                                                                tool_calls: vec![],
                                                            })
                                                        };

                                                        Some(ResponseChoice {
                                                            index,
                                                            finish_reason: Some("stop".to_string()),
                                                            delta: Some(ResponseDelta {
                                                                content: None,
                                                                role: Some(role),
                                                                tool_calls: vec![],
                                                            }),
                                                            message: message_delta,
                                                        })
                                                    }
                                                        ResponsesApiOutput::FunctionCall {
                                                            id,
                                                            call_id,
                                                            name,
                                                            function,
                                                            arguments,
                                                            arguments_delta,
                                                            arguments_json_patch,
                                                            ..
                                                        } => {
                                                            tool_chunk_from_function_call(
                                                                index,
                                                                id,
                                                                call_id,
                                                                name,
                                                                function,
                                                                arguments,
                                                                arguments_delta,
                                                                arguments_json_patch,
                                                            )
                                                            .map(|chunk| {
                                                                log::debug!(
                                                                    "Responses non-stream tool_call -> id={:?}, name={:?}, args={:?}",
                                                                    chunk.id,
                                                                    chunk
                                                                        .function
                                                                        .as_ref()
                                                                        .and_then(|f| f.name.as_ref())
                                                                        .map(|s| s.as_str()),
                                                                    chunk
                                                                        .function
                                                                        .as_ref()
                                                                        .and_then(|f| f.arguments.as_ref())
                                                                        .map(|s| s.as_str())
                                                                );

                                                                ResponseChoice {
                                                                    index,
                                                                    finish_reason: Some("tool_calls".to_string()),
                                                                    delta: Some(ResponseDelta {
                                                                        content: None,
                                                                        role: Some(Role::Assistant),
                                                                        tool_calls: vec![chunk],
                                                                    }),
                                                                    message: None,
                                                                }
                                                            })
                                                        }
                                                        ResponsesApiOutput::Reasoning { .. } => None,
                                                    })
                                                    .collect();

                                                Some(Ok(ResponseEvent {
                                                    choices,
                                                    id: response.id,
                                                    usage: response.usage,
                                                }))
                                            },
                                            Err(_) => {
                                                // Final fallback to Chat Completions format
                                                log::warn!("Trying Chat Completions format as final fallback");
                                                match serde_json::from_str::<ResponseEvent>(line) {
                                                    Ok(response) => {
                                                        if response.choices.is_empty() {
                                                            None
                                                        } else {
                                                            Some(Ok(response))
                                                        }
                                                    }
                                                    Err(error) => Some(Err(anyhow!("Failed to parse GPT-5 Codex response in all formats. Stream event error: {}, Complete response error: parse failed, Chat completion error: {}", parse_error, error))),
                                                }
                                            }
                                        }
                                    } else {
                                        // Not a complete response, return the original streaming parse error
                                        Some(Err(anyhow!("Failed to parse GPT-5 Codex streaming event: {}", parse_error)))
                                    }
                                }
                            }
                        } else {
                            // Parse Chat Completions API format
                            match serde_json::from_str::<ResponseEvent>(line) {
                                Ok(response) => {
                                    if response.choices.is_empty() {
                                        None
                                    } else {
                                        Some(Ok(response))
                                    }
                                }
                                Err(error) => Some(Err(anyhow!(error))),
                            }
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            }})
            .boxed())
    } else {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;
        let body_str = std::str::from_utf8(&body)?;

        // Parse response based on model - Responses API models use a different envelope
        let response: ResponseEvent = if request.use_responses_api {
            // Try Responses API response format first
            log::debug!(
                "Parsing GPT-5 Codex non-streaming response as Responses API format. Response length: {}",
                body_str.len()
            );
            log::debug!(
                "Response preview: {}",
                &body_str[..std::cmp::min(500, body_str.len())]
            );
            match serde_json::from_str::<ResponsesApiWrapper>(body_str) {
                Ok(wrapper) => {
                    let responses_response = wrapper.response;
                    let choices = responses_response
                        .output
                        .unwrap_or_default()
                        .into_iter()
                        .enumerate()
                        .filter_map(|(index, output)| {
                            match output {
                                ResponsesApiOutput::Message { role, content, .. } => {
                                    let text_content = content
                                        .into_iter()
                                        .filter_map(|c| c.text)
                                        .collect::<Vec<_>>()
                                        .join("");

                                    let role = match role.as_str() {
                                        "assistant" => Role::Assistant,
                                        "user" => Role::User,
                                        "system" => Role::System,
                                        _ => Role::Assistant,
                                    };

                                    let message_delta = if text_content.is_empty() {
                                        None
                                    } else {
                                        Some(ResponseDelta {
                                            content: Some(text_content),
                                            role: Some(role),
                                            tool_calls: vec![],
                                        })
                                    };

                                    Some(ResponseChoice {
                                        index,
                                        finish_reason: Some("stop".to_string()),
                                        delta: Some(ResponseDelta {
                                            content: None,
                                            role: Some(role),
                                            tool_calls: vec![],
                                        }),
                                        message: message_delta,
                                    })
                                }
                                ResponsesApiOutput::FunctionCall {
                                    id,
                                    call_id,
                                    name,
                                    function,
                                    arguments,
                                    arguments_delta,
                                    arguments_json_patch,
                                    ..
                                } => tool_chunk_from_function_call(
                                    index,
                                    id,
                                    call_id,
                                    name,
                                    function,
                                    arguments,
                                    arguments_delta,
                                    arguments_json_patch,
                                )
                                .map(|chunk| ResponseChoice {
                                    index,
                                    finish_reason: Some("tool_calls".to_string()),
                                    delta: None,
                                    message: Some(ResponseDelta {
                                        content: None,
                                        role: Some(Role::Assistant),
                                        tool_calls: vec![chunk],
                                    }),
                                }),
                                ResponsesApiOutput::Reasoning { .. } => {
                                    // Skip reasoning outputs as they are internal
                                    log::debug!(
                                        "Skipping reasoning output in GPT-5 Codex response"
                                    );
                                    None
                                }
                            }
                        })
                        .collect();

                    ResponseEvent {
                        choices,
                        id: responses_response.id,
                        usage: responses_response.usage,
                    }
                }
                Err(e) => {
                    // Fallback to Chat Completions format if Responses API parsing fails
                    log::warn!(
                        "GPT-5 Codex response parsing failed with Responses API format: {}. Trying Chat Completions format. Response: {}",
                        e,
                        body_str
                    );
                    match serde_json::from_str::<ResponseEvent>(body_str) {
                        Ok(chat_response) => chat_response,
                        Err(chat_error) => {
                            return Err(anyhow::anyhow!(
                                "Failed to parse GPT-5 Codex response in both formats. Responses API error: {}, Chat Completions error: {}",
                                e,
                                chat_error
                            ));
                        }
                    }
                }
            }
        } else {
            // Parse Chat Completions API format directly
            serde_json::from_str(body_str)?
        };

        Ok(futures::stream::once(async move { Ok(response) }).boxed())
    }
}

