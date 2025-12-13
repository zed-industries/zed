use anyhow::{Result, anyhow};
use futures::{
    AsyncBufReadExt, AsyncReadExt,
    io::BufReader,
    stream::{BoxStream, StreamExt},
};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

pub const DEEPSEEK_API_URL: &str = "https://api.deepseek.com/v1";

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
            _ => anyhow::bail!("invalid role '{value}'"),
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

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Model {
    #[serde(rename = "deepseek-chat")]
    #[default]
    Chat,
    #[serde(rename = "deepseek-reasoner")]
    Reasoner,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Model::Chat
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "deepseek-chat" => Ok(Self::Chat),
            "deepseek-reasoner" => Ok(Self::Reasoner),
            _ => anyhow::bail!("invalid model id {id}"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Chat => "deepseek-chat",
            Self::Reasoner => "deepseek-reasoner",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Chat => "DeepSeek Chat",
            Self::Reasoner => "DeepSeek Reasoner",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name).as_str(),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Chat | Self::Reasoner => 128_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Chat => Some(8_192),
            Self::Reasoner => Some(64_000),
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum RequestMessage {
    Assistant {
        content: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reasoning_content: Option<String>,
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
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    #[serde(default)]
    pub prompt_cache_hit_tokens: u64,
    #[serde(default)]
    pub prompt_cache_miss_tokens: u64,
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
    pub usage: Option<Usage>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolCallChunk {
    #[serde(default)]
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Debug)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

impl<'de> Deserialize<'de> for FunctionChunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawFunctionChunk {
            name: Option<String>,
            arguments: Option<String>,
        }

        let raw = RawFunctionChunk::deserialize(deserializer)?;

        // Filter empty strings to None, similar to OpenAI implementation
        let name = raw.name.filter(|s| !s.is_empty());
        let arguments = raw.arguments.filter(|s| !s.is_empty());

        Ok(FunctionChunk { name, arguments })
    }
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
        .header("Authorization", format!("Bearer {}", api_key.trim()));

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
        anyhow::bail!(
            "Failed to connect to DeepSeek API: {} {}",
            response.status(),
            body,
        );
    }
}

// Helper function for testing DeepSeek streaming responses
#[allow(dead_code)]
fn parse_response_stream_result(line: &str) -> Result<StreamResponse, serde_json::Error> {
    serde_json::from_str(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_function_chunk_empty_string_filtering() {
        // Test that empty strings are filtered to None
        let json_with_empty = r#"{"name": "", "arguments": ""}"#;
        let chunk: FunctionChunk = serde_json::from_str(json_with_empty).unwrap();

        assert_eq!(chunk.name, None);
        assert_eq!(chunk.arguments, None);

        // Test that non-empty strings are preserved
        let json_with_content = r#"{"name": "grep", "arguments": "{\"pattern\": \"test\"}"}"#;
        let chunk: FunctionChunk = serde_json::from_str(json_with_content).unwrap();

        assert_eq!(chunk.name, Some("grep".to_string()));
        assert_eq!(chunk.arguments, Some("{\"pattern\": \"test\"}".to_string()));

        // Test mixed case
        let json_mixed = r#"{"name": "grep", "arguments": ""}"#;
        let chunk: FunctionChunk = serde_json::from_str(json_mixed).unwrap();

        assert_eq!(chunk.name, Some("grep".to_string()));
        assert_eq!(chunk.arguments, None);
    }

    #[test]
    fn test_deepseek_streaming_response_parsing() {
        // This simulates a real DeepSeek streaming response with tool calls
        let response_json = r#"{
            "id": "chat-123",
            "object": "chat.completion.chunk",
            "created": 1699012345,
            "model": "deepseek-chat",
            "choices": [{
                "index": 0,
                "delta": {
                    "role": "assistant",
                    "tool_calls": [{
                        "index": 0,
                        "id": "call_123",
                        "function": {
                            "name": "grep",
                            "arguments": "{\"pattern\":"
                        }
                    }]
                },
                "finish_reason": null
            }]
        }"#;

        let response: StreamResponse = serde_json::from_str(response_json).unwrap();

        assert_eq!(response.choices.len(), 1);
        let choice = &response.choices[0];
        assert_eq!(choice.delta.tool_calls.as_ref().unwrap().len(), 1);

        let tool_call = &choice.delta.tool_calls.as_ref().unwrap()[0];
        assert_eq!(tool_call.index, 0);
        assert_eq!(tool_call.id, Some("call_123".to_string()));

        let function = tool_call.function.as_ref().unwrap();
        assert_eq!(function.name, Some("grep".to_string()));
        assert_eq!(function.arguments, Some("{\"pattern\":".to_string()));
    }

    #[test]
    fn test_deepseek_streaming_response_empty_name() {
        // Test that empty names in subsequent chunks are filtered out
        let response_json = r#"{
            "id": "chat-123",
            "object": "chat.completion.chunk",
            "created": 1699012345,
            "model": "deepseek-chat",
            "choices": [{
                "index": 0,
                "delta": {
                    "tool_calls": [{
                        "index": 0,
                        "function": {
                            "name": "",
                            "arguments": "\"test\"}"
                        }
                    }]
                },
                "finish_reason": "tool_calls"
            }]
        }"#;

        let response: StreamResponse = serde_json::from_str(response_json).unwrap();

        let choice = &response.choices[0];
        let tool_call = &choice.delta.tool_calls.as_ref().unwrap()[0];

        // Empty name should be filtered to None
        assert_eq!(tool_call.function.as_ref().unwrap().name, None);
        assert_eq!(
            tool_call.function.as_ref().unwrap().arguments,
            Some("\"test\"}".to_string())
        );
    }

    #[test]
    fn test_deepseek_actual_streaming_response() {
        // Test with actual DeepSeek streaming response chunks
        // First chunk: has id, name but empty arguments
        let first_chunk = r#"{"id":"345aef33-13a5-44d2-aa7b-c34d7bf0c90c","object":"chat.completion.chunk","created":1765620432,"model":"deepseek-chat","choices":[{"index":0,"delta":{"role":"assistant","content":"","tool_calls":[{"index":0,"id":"chatcmpl-tool-5d8d905545f643c38be37e55bbec5e3c","function":{"arguments":"","name":"grep"}}]}}]}"#;

        let result = parse_response_stream_result(first_chunk);
        match result {
            Ok(event) => {
                assert_eq!(event.choices.len(), 1);
                let choice = &event.choices[0];
                assert!(choice.delta.tool_calls.is_some());
                let tool_calls = choice.delta.tool_calls.as_ref().unwrap();
                assert_eq!(tool_calls.len(), 1);
                assert_eq!(tool_calls[0].index, 0);
                assert_eq!(
                    tool_calls[0].id,
                    Some("chatcmpl-tool-5d8d905545f643c38be37e55bbec5e3c".to_string())
                );
                // The name should be Some("grep") after filtering
                assert_eq!(
                    tool_calls[0].function.as_ref().unwrap().name,
                    Some("grep".to_string())
                );
                assert_eq!(tool_calls[0].function.as_ref().unwrap().arguments, None); // Empty string filtered out
            }
            Err(_) => panic!("Expected success response for first chunk"),
        }

        // Second chunk: partial arguments, empty name
        let second_chunk = r#"{"id":"345aef33-13a5-44d2-aa7b-c34d7bf0c90c","object":"chat.completion.chunk","created":1765620432,"model":"deepseek-chat","choices":[{"index":0,"delta":{"role":"assistant","content":"","tool_calls":[{"index":0,"function":{"arguments":"{\"regex","name":""}}]}}]}"#;

        let result = parse_response_stream_result(second_chunk);
        match result {
            Ok(event) => {
                assert_eq!(event.choices.len(), 1);
                let choice = &event.choices[0];
                assert!(choice.delta.tool_calls.is_some());
                let tool_calls = choice.delta.tool_calls.as_ref().unwrap();
                assert_eq!(tool_calls.len(), 1);
                assert!(tool_calls[0].id.is_none()); // No ID in this chunk
                assert_eq!(tool_calls[0].function.as_ref().unwrap().name, None); // Empty string filtered out
                assert_eq!(
                    tool_calls[0].function.as_ref().unwrap().arguments,
                    Some("{\"regex".to_string())
                );
            }
            Err(_) => panic!("Expected success response for second chunk"),
        }

        // Third chunk: more arguments
        let third_chunk = r#"{"id":"345aef33-13a5-44d2-aa7b-c34d7bf0c90c","object":"chat.completion.chunk","created":1765620433,"model":"deepseek-chat","choices":[{"index":0,"delta":{"role":"assistant","content":"","tool_calls":[{"index":0,"function":{"arguments":"\": \"","name":""}}]}}]}"#;

        let result = parse_response_stream_result(third_chunk);
        match result {
            Ok(event) => {
                assert_eq!(event.choices.len(), 1);
                let choice = &event.choices[0];
                assert!(choice.delta.tool_calls.is_some());
                let tool_calls = choice.delta.tool_calls.as_ref().unwrap();
                assert_eq!(tool_calls.len(), 1);
                assert!(tool_calls[0].id.is_none());
                assert_eq!(tool_calls[0].function.as_ref().unwrap().name, None);
                assert_eq!(
                    tool_calls[0].function.as_ref().unwrap().arguments,
                    Some("\": \"".to_string())
                );
            }
            Err(_) => panic!("Expected success response for third chunk"),
        }

        // Last chunk with finish_reason
        let last_chunk = r#"{"id":"345aef33-13a5-44d2-aa7b-c34d7bf0c90c","object":"chat.completion.chunk","created":1765620434,"model":"deepseek-chat","choices":[{"index":0,"finish_reason":"tool_calls","delta":{"role":"assistant","content":""}}]}"#;

        let result = parse_response_stream_result(last_chunk);
        match result {
            Ok(event) => {
                assert_eq!(event.choices.len(), 1);
                let choice = &event.choices[0];
                assert_eq!(choice.finish_reason, Some("tool_calls".to_string()));
                // No tool calls in the final chunk
                if let Some(tool_calls) = &choice.delta.tool_calls {
                    assert!(tool_calls.is_empty());
                }
            }
            Err(_) => panic!("Expected success response for last chunk"),
        }
    }
}
