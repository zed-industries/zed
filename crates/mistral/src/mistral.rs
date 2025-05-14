use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use log;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

pub const MISTRAL_API_URL: &str = "https://api.mistral.ai/v1";

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

impl TryFrom<String> for Role {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "system" => Ok(Self::System),
            "tool" => Ok(Self::Tool),
            _ => Err(anyhow!("invalid role '{value}'")),
        }
    }
}

impl From<Role> for String {
    fn from(val: Role) -> Self {
        match val {
            Role::User => "user".to_owned(),
            Role::Assistant => "assistant".to_owned(),
            Role::System => "system".to_owned(),
            Role::Tool => "tool".to_owned(),
        }
    }
}

#[derive(Deserialize)]
struct ModelResponse {
    #[serde(deserialize_with = "deserialize_models_skip_errors")]
    data: Vec<ModelData>,
}

fn deserialize_models_skip_errors<'de, D>(deserializer: D) -> Result<Vec<ModelData>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw_values = Vec::<serde_json::Value>::deserialize(deserializer)?;
    let models = raw_values
        .into_iter()
        .filter_map(|value| match serde_json::from_value::<ModelData>(value) {
            Ok(model) => Some(model),
            Err(err) => {
                log::warn!("Mistral model failed to deserialize: {:?}", err);
                None
            }
        })
        .collect();

    Ok(models)
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Model {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: usize,
    pub max_output_tokens: Option<u32>,
    pub max_completion_tokens: Option<u32>,
    pub supports_tools: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ModelData {
    pub id: String,
    pub name: String,
    pub capabilities: ModelCapabilities,
    pub max_context_length: usize,
    pub description: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ModelCapabilities {
    pub completion_chat: bool,
    pub completion_fim: bool,
    pub function_calling: bool,
    pub fine_tuning: bool,
    pub vision: bool,
    #[serde(default)]
    pub classification: bool,
}

impl Model {
    pub fn new(
        name: &str,
        display_name: Option<&str>,
        max_tokens: Option<usize>,
        supports_tool_calls: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            display_name: display_name.map(|s| s.to_string()),
            max_tokens: max_tokens.unwrap_or(4096),
            max_output_tokens: None,
            max_completion_tokens: None,
            supports_tools: Some(supports_tool_calls),
        }
    }

    pub fn default_fast() -> Self {
        Self {
            display_name: Some("mistral-small-latest".to_string()),
            name: "mistral-small-latest".to_string(),
            max_tokens: 4096,
            max_output_tokens: None,
            max_completion_tokens: None,
            supports_tools: Some(false),
        }
    }

    pub fn default() -> Self {
        Self {
            display_name: Some("codestral-latest".to_string()),
            name: "codestral-latest".to_string(),
            max_tokens: 32768,
            max_output_tokens: None,
            max_completion_tokens: None,
            supports_tools: Some(true),
        }
    }

    pub fn id(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        &self.display_name.as_deref().unwrap_or(&self.name)
    }

    pub fn max_token_count(&self) -> usize {
        self.max_tokens
    }

    pub fn max_output_tokens(&self) -> Option<u32> {
        None
    }

    pub fn supports_tools(&self) -> bool {
        self.supports_tools.unwrap_or(false)
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    #[serde(rename = "json_object")]
    JsonObject,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    Function { function: FunctionDefinition },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: u32,
    pub temperature: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prediction: Option<Prediction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite_speculation: Option<bool>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Prediction {
    Content { content: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    Any,
    Function(ToolDefinition),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum RequestMessage {
    Assistant {
        content: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
    },
    User {
        content: String,
    },
    System {
        content: String,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCall {
    pub id: String,
    #[serde(flatten)]
    pub content: ToolCallContent,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolCallContent {
    Function { function: FunctionContent },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionContent {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionChoice {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: RequestMessage,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamChoice {
    pub index: u32,
    pub delta: StreamDelta,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDelta {
    pub role: Option<Role>,
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<StreamResponse>>> {
    let uri = format!("{api_url}/chat/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ")?;
                        if line == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str(line) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => Some(Err(anyhow!(error))),
                            }
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        Err(anyhow!(
            "Failed to connect to Mistral API: {} {}",
            response.status(),
            body,
        ))
    }
}

pub async fn fetch_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
) -> Result<Vec<Model>> {
    let uri = format!("{}/models", api_url);
    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));

    let request = request_builder.body(AsyncBody::empty())?;
    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;
        let model_response: ModelResponse = serde_json::from_str(body_str)?;

        let models = model_response
            .data
            .into_iter()
            .filter(|model| model.capabilities.completion_chat)
            .map(|model| Model {
                name: model.id,
                display_name: Some(model.name),
                max_tokens: model.max_context_length,
                max_output_tokens: None,
                max_completion_tokens: None,
                supports_tools: Some(model.capabilities.function_calling),
            })
            .collect::<Vec<_>>();

        Ok(models)
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        Err(anyhow!(
            "Failed to fetch mistral models: {} {}",
            response.status(),
            body
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resilient_model_schema_deserialize() {
        let json = r#"{
            "object": "list",
            "data": [
                {
                    "id": "mistral-large-latest",
                    "object": "model",
                    "created": 1747143320,
                    "owned_by": "mistralai",
                    "capabilities": {
                        "completion_chat": true,
                        "completion_fim": false,
                        "function_calling": true,
                        "fine_tuning": true,
                        "vision": false,
                        "classification": false
                    },
                    "name": "mistral-large-2411",
                    "description": "Official mistral-large-2411 Mistral AI model",
                    "max_context_length": 131072,
                    "aliases": [
                        "mistral-large-2411"
                    ],
                    "deprecation": null,
                    "default_model_temperature": 0.7,
                    "type": "base"
                },
                {
                    "id": "pixtral-large-latest",
                    "object": "model",
                    "created": 1747143320,
                    "owned_by": "mistralai",
                    "capabilities": {
                        "completion_chat": true,
                        "completion_fim": false,
                        "function_calling": true,
                        "fine_tuning": false,
                        "vision": true,
                        "classification": false
                    },
                    "name": "pixtral-large-2411",
                    "description": "Official pixtral-large-2411 Mistral AI model",
                    "max_context_length": 131072,
                    "aliases": [
                        "pixtral-large-2411",
                        "mistral-large-pixtral-2411"
                    ],
                    "deprecation": null,
                    "default_model_temperature": 0.7,
                    "type": "base"
                },
                {
                    "id": "codestral-latest",
                    "object": "model",
                    "created": 1747143320,
                    "owned_by": "mistralai",
                    "capabilities": {
                        "completion_chat": true,
                        "completion_fim": true,
                        "function_calling": true,
                        "fine_tuning": false,
                        "vision": false,
                        "classification": false
                    },
                    "name": "codestral-2501",
                    "description": "Official codestral-2501 Mistral AI model",
                    "max_context_length": 262144,
                    "aliases": [
                        "codestral-2501",
                        "codestral-2412",
                        "codestral-2411-rc5"
                    ],
                    "deprecation": null,
                    "default_model_temperature": 0.3,
                    "type": "base"
                },
                {
                    "id": "codestral-mamba-latest",
                    "object": "model",
                    "created": 1747143320,
                    "owned_by": "mistralai",
                    "capabilities": {
                        "completion_chat": true,
                        "completion_fim": false,
                        "function_calling": true,
                        "fine_tuning": false,
                        "vision": false,
                        "classification": false
                    },
                    "name": "codestral-mamba-2407",
                    "description": "Official codestral-mamba-2407 Mistral AI model",
                    "max_context_length": 262144,
                    "aliases": [
                        "codestral-mamba-2407",
                        "open-codestral-mamba"
                    ],
                    "deprecation": null,
                    "default_model_temperature": 0.7,
                    "type": "base"
                },
                {
                    "id": "pixtral-12b-latest",
                    "object": "model",
                    "created": 1747143320,
                    "owned_by": "mistralai",
                    "capabilities": {
                        "completion_chat": true,
                        "completion_fim": false,
                        "function_calling": true,
                        "fine_tuning": false,
                        "vision": true,
                        "classification": false
                    },
                    "name": "pixtral-12b-2409",
                    "description": "Official pixtral-12b-2409 Mistral AI model",
                    "max_context_length": 131072,
                    "aliases": [
                        "pixtral-12b-2409",
                        "pixtral-12b"
                    ],
                    "deprecation": null,
                    "default_model_temperature": 0.3,
                    "type": "base"
                },
                {
                    "id": "mistral-small-latest",
                    "object": "model",
                    "created": 1747143320,
                    "owned_by": "mistralai",
                    "capabilities": {
                        "completion_chat": true,
                        "completion_fim": false,
                        "function_calling": true,
                        "fine_tuning": false,
                        "vision": true,
                        "classification": false
                    },
                    "name": "mistral-small-2503",
                    "description": "Official mistral-small-2503 Mistral AI model",
                    "max_context_length": 131072,
                    "aliases": [
                        "mistral-small-2503"
                    ],
                    "deprecation": null,
                    "default_model_temperature": 0.3,
                    "type": "base"
                },
                {
                    "id": "mistral-medium-latest",
                    "object": "model",
                    "created": 1747143320,
                    "owned_by": "mistralai",
                    "capabilities": {
                        "completion_chat": true,
                        "completion_fim": false,
                        "function_calling": true,
                        "fine_tuning": false,
                        "vision": true,
                        "classification": false
                    },
                    "name": "mistral-medium-2505",
                    "description": "Official mistral-medium-2505 Mistral AI model",
                    "max_context_length": 131072,
                    "aliases": [
                        "mistral-medium-2505",
                        "mistral-medium"
                    ],
                    "deprecation": null,
                    "default_model_temperature": 0.3,
                    "type": "base"
                },
                {
                    "id": "mistral-ocr-latest",
                    "object": "model",
                    "created": 1747143320,
                    "owned_by": "mistralai",
                    "capabilities": {
                        "completion_chat": false,
                        "completion_fim": false,
                        "function_calling": false,
                        "fine_tuning": false,
                        "vision": false,
                        "classification": false
                    },
                    "name": "mistral-ocr-2503",
                    "description": "Official mistral-ocr-2503 Mistral AI model",
                    "max_context_length": 32768,
                    "aliases": [
                        "mistral-ocr-2503"
                    ],
                    "deprecation": null,
                    "default_model_temperature": null,
                    "type": "base"
                }
            ]
        }"#;

        let schema: ModelResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(schema.data.len(), 8);
        assert_eq!(schema.data[0].id, "mistral-large-latest");
        assert_eq!(schema.data[1].id, "pixtral-large-latest");

        assert!(schema.data[0].capabilities.function_calling);
        assert!(schema.data[0].capabilities.fine_tuning);
        assert!(!schema.data[0].capabilities.vision);

        assert!(schema.data[1].capabilities.vision);
    }

    #[test]
    fn test_resilient_model_deserialization() {
        let json = r#"{
            "object": "list",
            "data": [
                {
                    "id": "valid-model",
                    "name": "Valid Model",
                    "capabilities": {
                        "completion_chat": true,
                        "completion_fim": false,
                        "function_calling": true,
                        "fine_tuning": false,
                        "vision": false,
                        "classification": false
                    },
                    "max_context_length": 4000,
                    "description": "A test model",
                    "aliases": []
                },
                {
                    "id": "invalid-model",
                    "name": "Invalid Model"
                }
            ]
        }"#;

        let schema: ModelResponse = serde_json::from_str(json).unwrap();

        assert_eq!(schema.data.len(), 1);
        assert_eq!(schema.data[0].id, "valid-model");
        assert_eq!(schema.data[0].name, "Valid Model");
        assert!(schema.data[0].capabilities.completion_chat);
        assert!(schema.data[0].capabilities.function_calling);
    }

    #[test]
    fn test_empty_model_list() {
        let json = r#"{
            "object": "list",
            "data": []
        }"#;

        let schema: ModelResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(schema.data.len(), 0);
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "id": "resp-12345",
            "object": "chat.completion",
            "created": 1717612000,
            "model": "mistral-medium-latest",
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": "Hello, how can I help you today?"
                    },
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 8,
                "total_tokens": 18
            }
        }"#;

        let response: Response = serde_json::from_str(json).unwrap();

        assert_eq!(response.id, "resp-12345");
        assert_eq!(response.model, "mistral-medium-latest");
        assert_eq!(response.choices.len(), 1);

        let choice = &response.choices[0];
        assert_eq!(choice.index, 0);
        assert_eq!(choice.finish_reason, Some("stop".to_string()));

        if let RequestMessage::Assistant { content, .. } = &choice.message {
            assert_eq!(
                content,
                &Some("Hello, how can I help you today?".to_string())
            );
        } else {
            panic!("Expected Assistant message");
        }

        assert_eq!(response.usage.prompt_tokens, 10);
        assert_eq!(response.usage.completion_tokens, 8);
        assert_eq!(response.usage.total_tokens, 18);
    }

    #[test]
    fn test_stream_response_deserialization() {
        let json = r#"{
            "id": "cmpl-12345",
            "object": "chat.completion.chunk",
            "created": 1717612000,
            "model": "mistral-small-latest",
            "choices": [
                {
                    "index": 0,
                    "delta": {
                        "role": "assistant",
                        "content": "Hello"
                    },
                    "finish_reason": null
                }
            ]
        }"#;

        let response: StreamResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.id, "cmpl-12345");
        assert_eq!(response.model, "mistral-small-latest");
        assert_eq!(response.choices.len(), 1);

        let choice = &response.choices[0];
        assert_eq!(choice.index, 0);
        assert!(choice.finish_reason.is_none());

        let delta = &choice.delta;
        assert_eq!(delta.role, Some(Role::Assistant));
        assert_eq!(delta.content, Some("Hello".to_string()));
    }

    #[test]
    fn test_complex_request_serialization() {
        let request = Request {
            model: "mistral-medium-latest".to_string(),
            messages: vec![
                RequestMessage::System {
                    content: "You are a helpful assistant.".to_string(),
                },
                RequestMessage::User {
                    content: "Tell me about Rust programming.".to_string(),
                },
            ],
            stream: true,
            max_tokens: Some(100),
            temperature: Some(0.7),
            response_format: Some(ResponseFormat::Text),
            tool_choice: None,
            parallel_tool_calls: None,
            tools: vec![],
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"model\":\"mistral-medium-latest\""));
        assert!(json.contains("\"stream\":true"));
        assert!(json.contains("\"max_tokens\":100"));
        assert!(json.contains("\"temperature\":0.7"));
        assert!(json.contains("\"role\":\"system\""));
        assert!(json.contains("\"role\":\"user\""));
        assert!(json.contains("\"content\":\"You are a helpful assistant.\""));
        assert!(json.contains("\"content\":\"Tell me about Rust programming.\""));
    }

    #[test]
    fn test_tool_call_serialization() {
        let tool_call = ToolCall {
            id: "call_12345".to_string(),
            content: ToolCallContent::Function {
                function: FunctionContent {
                    name: "get_weather".to_string(),
                    arguments: r#"{"location":"New York"}"#.to_string(),
                },
            },
        };

        let json = serde_json::to_string(&tool_call).unwrap();

        assert!(json.contains("\"id\":\"call_12345\""));
        assert!(json.contains("\"type\":\"function\""));
        assert!(json.contains("\"name\":\"get_weather\""));
        assert!(json.contains("\"arguments\":\"{\\\"location\\\":\\\"New York\\\"}\""));

        let deserialized: ToolCall = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "call_12345");

        let ToolCallContent::Function { function } = &deserialized.content;
        assert_eq!(function.name, "get_weather");
        assert_eq!(function.arguments, r#"{"location":"New York"}"#);
    }
}
