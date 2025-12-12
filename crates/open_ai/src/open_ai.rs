use anyhow::{Context as _, Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{
    AsyncBody, HttpClient, Method, Request as HttpRequest, StatusCode,
    http::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use settings::OpenAiReasoningEffort as ReasoningEffort;
use std::{convert::TryFrom, future::Future};
use strum::EnumIter;
use thiserror::Error;

pub const OPEN_AI_API_URL: &str = "https://api.openai.com/v1";

fn is_none_or_empty<T: AsRef<[U]>, U>(opt: &Option<T>) -> bool {
    opt.as_ref().is_none_or(|v| v.as_ref().is_empty())
}

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
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo")]
    ThreePointFiveTurbo,
    #[serde(rename = "gpt-4")]
    Four,
    #[serde(rename = "gpt-4-turbo")]
    FourTurbo,
    #[serde(rename = "gpt-4o")]
    #[default]
    FourOmni,
    #[serde(rename = "gpt-4o-mini")]
    FourOmniMini,
    #[serde(rename = "gpt-4.1")]
    FourPointOne,
    #[serde(rename = "gpt-4.1-mini")]
    FourPointOneMini,
    #[serde(rename = "gpt-4.1-nano")]
    FourPointOneNano,
    #[serde(rename = "o1")]
    O1,
    #[serde(rename = "o3-mini")]
    O3Mini,
    #[serde(rename = "o3")]
    O3,
    #[serde(rename = "o4-mini")]
    O4Mini,
    #[serde(rename = "gpt-5")]
    Five,
    #[serde(rename = "gpt-5-mini")]
    FiveMini,
    #[serde(rename = "gpt-5-nano")]
    FiveNano,
    #[serde(rename = "gpt-5.1")]
    FivePointOne,
    #[serde(rename = "gpt-5.2")]
    FivePointTwo,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        max_completion_tokens: Option<u64>,
        reasoning_effort: Option<ReasoningEffort>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        // TODO: Replace with FiveMini since all other models are deprecated
        Self::FourPointOneMini
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "gpt-3.5-turbo" => Ok(Self::ThreePointFiveTurbo),
            "gpt-4" => Ok(Self::Four),
            "gpt-4-turbo-preview" => Ok(Self::FourTurbo),
            "gpt-4o" => Ok(Self::FourOmni),
            "gpt-4o-mini" => Ok(Self::FourOmniMini),
            "gpt-4.1" => Ok(Self::FourPointOne),
            "gpt-4.1-mini" => Ok(Self::FourPointOneMini),
            "gpt-4.1-nano" => Ok(Self::FourPointOneNano),
            "o1" => Ok(Self::O1),
            "o3-mini" => Ok(Self::O3Mini),
            "o3" => Ok(Self::O3),
            "o4-mini" => Ok(Self::O4Mini),
            "gpt-5" => Ok(Self::Five),
            "gpt-5-mini" => Ok(Self::FiveMini),
            "gpt-5-nano" => Ok(Self::FiveNano),
            "gpt-5.1" => Ok(Self::FivePointOne),
            "gpt-5.2" => Ok(Self::FivePointTwo),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::ThreePointFiveTurbo => "gpt-3.5-turbo",
            Self::Four => "gpt-4",
            Self::FourTurbo => "gpt-4-turbo",
            Self::FourOmni => "gpt-4o",
            Self::FourOmniMini => "gpt-4o-mini",
            Self::FourPointOne => "gpt-4.1",
            Self::FourPointOneMini => "gpt-4.1-mini",
            Self::FourPointOneNano => "gpt-4.1-nano",
            Self::O1 => "o1",
            Self::O3Mini => "o3-mini",
            Self::O3 => "o3",
            Self::O4Mini => "o4-mini",
            Self::Five => "gpt-5",
            Self::FiveMini => "gpt-5-mini",
            Self::FiveNano => "gpt-5-nano",
            Self::FivePointOne => "gpt-5.1",
            Self::FivePointTwo => "gpt-5.2",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::ThreePointFiveTurbo => "gpt-3.5-turbo",
            Self::Four => "gpt-4",
            Self::FourTurbo => "gpt-4-turbo",
            Self::FourOmni => "gpt-4o",
            Self::FourOmniMini => "gpt-4o-mini",
            Self::FourPointOne => "gpt-4.1",
            Self::FourPointOneMini => "gpt-4.1-mini",
            Self::FourPointOneNano => "gpt-4.1-nano",
            Self::O1 => "o1",
            Self::O3Mini => "o3-mini",
            Self::O3 => "o3",
            Self::O4Mini => "o4-mini",
            Self::Five => "gpt-5",
            Self::FiveMini => "gpt-5-mini",
            Self::FiveNano => "gpt-5-nano",
            Self::FivePointOne => "gpt-5.1",
            Self::FivePointTwo => "gpt-5.2",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::ThreePointFiveTurbo => 16_385,
            Self::Four => 8_192,
            Self::FourTurbo => 128_000,
            Self::FourOmni => 128_000,
            Self::FourOmniMini => 128_000,
            Self::FourPointOne => 1_047_576,
            Self::FourPointOneMini => 1_047_576,
            Self::FourPointOneNano => 1_047_576,
            Self::O1 => 200_000,
            Self::O3Mini => 200_000,
            Self::O3 => 200_000,
            Self::O4Mini => 200_000,
            Self::Five => 272_000,
            Self::FiveMini => 272_000,
            Self::FiveNano => 272_000,
            Self::FivePointOne => 400_000,
            Self::FivePointTwo => 400_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
            Self::ThreePointFiveTurbo => Some(4_096),
            Self::Four => Some(8_192),
            Self::FourTurbo => Some(4_096),
            Self::FourOmni => Some(16_384),
            Self::FourOmniMini => Some(16_384),
            Self::FourPointOne => Some(32_768),
            Self::FourPointOneMini => Some(32_768),
            Self::FourPointOneNano => Some(32_768),
            Self::O1 => Some(100_000),
            Self::O3Mini => Some(100_000),
            Self::O3 => Some(100_000),
            Self::O4Mini => Some(100_000),
            Self::Five => Some(128_000),
            Self::FiveMini => Some(128_000),
            Self::FiveNano => Some(128_000),
            Self::FivePointOne => Some(128_000),
            Self::FivePointTwo => Some(128_000),
        }
    }

    pub fn reasoning_effort(&self) -> Option<ReasoningEffort> {
        match self {
            Self::Custom {
                reasoning_effort, ..
            } => reasoning_effort.to_owned(),
            _ => None,
        }
    }

    /// Returns whether the given model supports the `parallel_tool_calls` parameter.
    ///
    /// If the model does not support the parameter, do not pass it up, or the API will return an error.
    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::ThreePointFiveTurbo
            | Self::Four
            | Self::FourTurbo
            | Self::FourOmni
            | Self::FourOmniMini
            | Self::FourPointOne
            | Self::FourPointOneMini
            | Self::FourPointOneNano
            | Self::Five
            | Self::FiveMini
            | Self::FivePointOne
            | Self::FivePointTwo
            | Self::FiveNano => true,
            Self::O1 | Self::O3 | Self::O3Mini | Self::O4Mini | Model::Custom { .. } => false,
        }
    }

    /// Returns whether the given model supports the `prompt_cache_key` parameter.
    ///
    /// If the model does not support the parameter, do not pass it up.
    pub fn supports_prompt_cache_key(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Whether to enable parallel function calling during tool use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    #[serde(untagged)]
    Other(ToolDefinition),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    #[allow(dead_code)]
    Function { function: FunctionDefinition },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum RequestMessage {
    Assistant {
        content: Option<MessageContent>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
    },
    User {
        content: MessageContent,
    },
    System {
        content: MessageContent,
    },
    Tool {
        content: MessageContent,
        tool_call_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    Plain(String),
    Multipart(Vec<MessagePart>),
}

impl MessageContent {
    pub fn empty() -> Self {
        MessageContent::Multipart(vec![])
    }

    pub fn push_part(&mut self, part: MessagePart) {
        match self {
            MessageContent::Plain(text) => {
                *self =
                    MessageContent::Multipart(vec![MessagePart::Text { text: text.clone() }, part]);
            }
            MessageContent::Multipart(parts) if parts.is_empty() => match part {
                MessagePart::Text { text } => *self = MessageContent::Plain(text),
                MessagePart::Image { .. } => *self = MessageContent::Multipart(vec![part]),
            },
            MessageContent::Multipart(parts) => parts.push(part),
        }
    }
}

impl From<Vec<MessagePart>> for MessageContent {
    fn from(mut parts: Vec<MessagePart>) -> Self {
        if let [MessagePart::Text { text }] = parts.as_mut_slice() {
            MessageContent::Plain(std::mem::take(text))
        } else {
            MessageContent::Multipart(parts)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum MessagePart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    Image { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCall {
    pub id: String,
    #[serde(flatten)]
    pub content: ToolCallContent,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolCallContent {
    Function { function: FunctionContent },
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionContent {
    pub name: String,
    pub arguments: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: RequestMessage,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ResponseMessageDelta {
    pub role: Option<Role>,
    pub content: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "is_none_or_empty",
        deserialize_with = "deserialize_tool_calls"
    )]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
    #[serde(default, skip_serializing_if = "is_none_or_empty")]
    pub reasoning_content: Option<String>,
}

#[allow(clippy::redundant_clone)]
fn deserialize_tool_calls<'de, D>(deserializer: D) -> Result<Option<Vec<ToolCallChunk>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;

    // Handle empty arrays explicitly
    if value.is_array() && value.as_array().map_or(false, |arr| arr.is_empty()) {
        return Ok(None);
    }

    // Try to deserialize as standard format first
    if let Ok(vec) = serde_json::from_value::<Vec<ToolCallChunk>>(value.clone()) {
        if vec.is_empty() {
            return Ok(None);
        } else {
            // Validate that all tool calls have required fields
            // For streaming scenarios, we're more lenient about missing id fields
            for tool_call in &vec {
                if tool_call.r#type.is_none() || tool_call.function.is_none() {
                    return Ok(None); // Malformed - missing required type/function fields
                }
                // Note: We allow missing id fields for streaming chunks
                // They will be handled by the conversion logic
            }
            return Ok(Some(vec));
        }
    }

    // If standard deserialization fails, try non-standard formats
    if let Ok(values) = serde_json::from_value::<Vec<serde_json::Value>>(value) {
        // Try to convert non-standard tool calls to our format
        let mut converted_tool_calls = Vec::new();

        for (index, value) in values.into_iter().enumerate() {
            if let Some(tool_call) = convert_non_standard_tool_call(value, index) {
                converted_tool_calls.push(tool_call);
            }
        }

        if converted_tool_calls.is_empty() {
            return Ok(None);
        } else {
            return Ok(Some(converted_tool_calls));
        }
    }

    // If all deserialization attempts fail, treat as malformed and return None
    Ok(None)
}

#[allow(clippy::redundant_clone)]
/// Convert non-standard tool call formats to our standard ToolCallChunk format
fn convert_non_standard_tool_call(value: serde_json::Value, index: usize) -> Option<ToolCallChunk> {
    // Handle the DeepSeek-style format: {"type": "function", "function": {...}}
    if let Some(tool_type) = value.get("type").and_then(|v| v.as_str()) {
        if tool_type == "function" {
            if let Some(function_value) = value.get("function") {
                // Validate that function_value has the required fields
                let function_obj = function_value.as_object()?;

                // Check if we have at least a name (required field)
                if function_obj.get("name").is_none() {
                    return None; // Malformed - no name field
                }

                let function_chunk =
                    serde_json::from_value::<FunctionChunk>(function_value.clone()).ok()?;

                // For streaming chunks, generate a temporary ID if missing
                // This handles DeepSeek's format where id comes in separate chunks
                let tool_call_id = value
                    .get("id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .or_else(|| Some(format!("temp-id-{}", index)));

                return Some(ToolCallChunk {
                    index,
                    id: tool_call_id,
                    r#type: Some("function".to_string()),
                    function: Some(function_chunk),
                });
            }
        }
    }

    // Try to deserialize as standard format as fallback
    // But validate that it has required fields
    if let Ok(tool_call) = serde_json::from_value::<ToolCallChunk>(value.clone()) {
        // Check if the tool call is valid (has type, function, and id)
        if tool_call.r#type.is_some() && tool_call.function.is_some() && tool_call.id.is_some() {
            return Some(tool_call);
        } else {
            // Missing required fields, treat as malformed
            return None;
        }
    }

    // If we can't convert it to a valid tool call, return None
    None
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    #[serde(default)]
    pub index: usize,
    pub id: Option<String>,

    // There is also an optional `type` field that would determine if a
    // function is there. Sometimes this streams in with the `function` before
    // it streams in the `type`
    #[serde(default)]
    pub r#type: Option<String>,

    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Usage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceDelta {
    pub index: u32,
    pub delta: Option<ResponseMessageDelta>,
    pub finish_reason: Option<String>,
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("HTTP response error from {provider}'s API: status {status_code} - {body:?}")]
    HttpResponseError {
        provider: String,
        status_code: StatusCode,
        body: String,
        headers: HeaderMap<HeaderValue>,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamError {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseStreamResult {
    Ok(ResponseStreamEvent),
    Err { error: ResponseStreamError },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamEvent {
    pub choices: Vec<ChoiceDelta>,
    #[serde(default)]
    pub usage: Option<Usage>,
    #[serde(flatten)]
    pub additional_fields: std::collections::HashMap<String, serde_json::Value>,
}

// Parse a response line with more lenient error handling for different providers
fn parse_response_stream_result(line: &str) -> Result<ResponseStreamResult, serde_json::Error> {
    // First try the standard parsing
    match serde_json::from_str(line) {
        Ok(result) => return Ok(result),
        Err(_) => {
            // If standard parsing fails, try a more lenient approach
            let value: serde_json::Value = serde_json::from_str(line)?;

            // Check if this looks like a success response (has choices)
            if let Some(choices) = value.get("choices") {
                // Try to build a ResponseStreamEvent manually with robust error handling
                let choices_result: Result<Vec<ChoiceDelta>, _> =
                    serde_json::from_value(choices.clone());

                let choices = match choices_result {
                    Ok(choices) => choices,
                    Err(_) => {
                        // If standard deserialization fails, try to handle it more gracefully
                        // This can happen with malformed data from some providers
                        if let Some(choices_array) = choices.as_array() {
                            let mut fixed_choices = Vec::new();

                            for choice_value in choices_array {
                                // Try to deserialize each choice individually
                                if let Ok(choice) =
                                    serde_json::from_value::<ChoiceDelta>(choice_value.clone())
                                {
                                    fixed_choices.push(choice);
                                } else {
                                    // If a choice fails to deserialize, try to create a minimal valid choice
                                    // but handle malformed tool calls gracefully
                                    let delta = if let Some(delta_value) = choice_value.get("delta")
                                    {
                                        // Try to deserialize delta, but if it fails, create a minimal valid delta
                                        match serde_json::from_value::<ResponseMessageDelta>(
                                            delta_value.clone(),
                                        ) {
                                            Ok(delta) => Some(delta),
                                            Err(_) => {
                                                // Create a minimal valid delta with malformed tool calls treated as None
                                                Some(ResponseMessageDelta {
                                                    role: delta_value.get("role").and_then(|v| {
                                                        serde_json::from_value(v.clone()).ok()
                                                    }),
                                                    content: delta_value
                                                        .get("content")
                                                        .and_then(|v| v.as_str())
                                                        .map(|s| s.to_string()),
                                                    tool_calls: None, // Treat malformed tool calls as None
                                                    reasoning_content: None,
                                                })
                                            }
                                        }
                                    } else {
                                        None
                                    };

                                    let fixed_choice = ChoiceDelta {
                                        index: choice_value
                                            .get("index")
                                            .and_then(|v| v.as_u64())
                                            .map(|v| v as u32)
                                            .unwrap_or(0),
                                        delta,
                                        finish_reason: choice_value
                                            .get("finish_reason")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string()),
                                    };
                                    fixed_choices.push(fixed_choice);
                                }
                            }

                            if !fixed_choices.is_empty() {
                                fixed_choices
                            } else {
                                // If we couldn't fix any choices, fall back to empty vector
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        }
                    }
                };

                let usage = value
                    .get("usage")
                    .and_then(|u| serde_json::from_value::<Usage>(u.clone()).ok());

                let mut additional_fields = std::collections::HashMap::new();
                if let Some(obj) = value.as_object() {
                    for (key, val) in obj {
                        if key != "choices" && key != "usage" {
                            additional_fields.insert(key.clone(), val.clone());
                        }
                    }
                }

                let event = ResponseStreamEvent {
                    choices,
                    usage,
                    additional_fields,
                };

                return Ok(ResponseStreamResult::Ok(event));
            }

            // Check if this looks like an error response (has error)
            if let Some(error_obj) = value.get("error") {
                let error = serde_json::from_value::<ResponseStreamError>(error_obj.clone())?;
                return Ok(ResponseStreamResult::Err { error });
            }

            // If we can't determine the structure, fall back to standard parsing
            // This will likely fail, but at least we tried
            serde_json::from_str(line)
        }
    }
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    provider_name: &str,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent>>, RequestError> {
    let uri = format!("{api_url}/chat/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()));

    let request = request_builder
        .body(AsyncBody::from(
            serde_json::to_string(&request).map_err(|e| RequestError::Other(e.into()))?,
        ))
        .map_err(|e| RequestError::Other(e.into()))?;

    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ").or_else(|| line.strip_prefix("data:"))?;
                        if line == "[DONE]" {
                            None
                        } else {
                            // Try to parse the response with more lenient error handling
                            match parse_response_stream_result(line) {
                                Ok(ResponseStreamResult::Ok(response)) => Some(Ok(response)),
                                Ok(ResponseStreamResult::Err { error }) => {
                                    Some(Err(anyhow!(error.message)))
                                }
                                Err(error) => {
                                    log::error!(
                                        "Failed to parse OpenAI response into ResponseStreamResult: `{}`\n\
                                        Response: `{}`",
                                        error,
                                        line,
                                    );
                                    Some(Err(anyhow!(error)))
                                }
                            }
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        Err(RequestError::HttpResponseError {
            provider: provider_name.to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum OpenAiEmbeddingModel {
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
}

#[derive(Serialize)]
struct OpenAiEmbeddingRequest<'a> {
    model: OpenAiEmbeddingModel,
    input: Vec<&'a str>,
}

#[derive(Deserialize)]
pub struct OpenAiEmbeddingResponse {
    pub data: Vec<OpenAiEmbedding>,
}

#[derive(Deserialize)]
pub struct OpenAiEmbedding {
    pub embedding: Vec<f32>,
}

pub fn embed<'a>(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    model: OpenAiEmbeddingModel,
    texts: impl IntoIterator<Item = &'a str>,
) -> impl 'static + Future<Output = Result<OpenAiEmbeddingResponse>> {
    let uri = format!("{api_url}/embeddings");

    let request = OpenAiEmbeddingRequest {
        model,
        input: texts.into_iter().collect(),
    };
    let body = AsyncBody::from(serde_json::to_string(&request).unwrap());
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .body(body)
        .map(|request| client.send(request));

    async move {
        let mut response = request?.await?;
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        anyhow::ensure!(
            response.status().is_success(),
            "error during embedding, status: {:?}, body: {:?}",
            response.status(),
            body
        );
        let response: OpenAiEmbeddingResponse =
            serde_json::from_str(&body).context("failed to parse OpenAI embedding response")?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_deepseek_response() {
        // Test a DeepSeek-style response with extend_fields
        let deepseek_response = json!({
            "id": "21705079-0372-4995-8176-8556a50d7951",
            "object": "chat.completion.chunk",
            "created": 1765468032,
            "model": "deepseek-v3.2",
            "usage": {
                "prompt_tokens": 10772,
                "completion_tokens": 1,
                "total_tokens": 10773
            },
            "extend_fields": {
                "traceId": "21010f9017654680283934182e2513",
                "requestId": "8e2021a4be88b53de0fbbe3655fbad29"
            },
            "choices": [{
                "index": 0,
                "delta": {
                    "role": "assistant",
                    "content": "**",
                    "tool_calls": []
                }
            }]
        });

        let response_str = deepseek_response.to_string();
        let result = parse_response_stream_result(&response_str);

        match result {
            Ok(ResponseStreamResult::Ok(event)) => {
                assert_eq!(event.choices.len(), 1);
                assert!(event.usage.is_some());
                assert!(event.additional_fields.contains_key("id"));
                assert!(event.additional_fields.contains_key("object"));
                assert!(event.additional_fields.contains_key("created"));
                assert!(event.additional_fields.contains_key("model"));
                assert!(event.additional_fields.contains_key("extend_fields"));
            }
            Ok(ResponseStreamResult::Err { error }) => {
                panic!("Expected success response, got error: {}", error.message);
            }
            Err(e) => {
                panic!("Failed to parse DeepSeek response: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_standard_openai_response() {
        // Test a standard OpenAI-style response
        let openai_response = json!({
            "choices": [{
                "index": 0,
                "delta": {
                    "content": "hello"
                }
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 5,
                "total_tokens": 15
            }
        });

        let response_str = openai_response.to_string();
        let result = parse_response_stream_result(&response_str);

        match result {
            Ok(ResponseStreamResult::Ok(event)) => {
                assert_eq!(event.choices.len(), 1);
                assert!(event.usage.is_some());
            }
            Ok(ResponseStreamResult::Err { error }) => {
                panic!("Expected success response, got error: {}", error.message);
            }
            Err(e) => {
                panic!("Failed to parse OpenAI response: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_malformed_tool_calls() {
        // Test parsing responses with tool calls missing 'id' field (common in streaming)
        let malformed_tool_calls_response = json!({
            "choices": [{
                "index": 0,
                "delta": {
                    "role": "assistant",
                    "content": "Some content",
                    "tool_calls": [
                        {
                            "type": "function",
                            "function": {
                                "arguments": "malformed json",
                                "name": "grep"
                            }
                        }
                    ]
                }
            }]
        });

        let response_str = malformed_tool_calls_response.to_string();
        let result = parse_response_stream_result(&response_str);

        match result {
            Ok(ResponseStreamResult::Ok(event)) => {
                assert_eq!(event.choices.len(), 1);
                // With our lenient streaming handling, tool calls without id fields should be accepted
                // and given temporary IDs
                if let Some(choice) = event.choices.first() {
                    if let Some(delta) = &choice.delta {
                        assert!(
                            delta.tool_calls.is_some(),
                            "Tool calls without id should be accepted with temporary IDs for streaming"
                        );
                        if let Some(tool_calls) = &delta.tool_calls {
                            assert_eq!(tool_calls.len(), 1, "Should have one tool call");
                            assert!(tool_calls[0].id.is_some(), "Should have a generated ID");
                            assert!(
                                tool_calls[0].id.as_ref().unwrap().starts_with("temp-id-"),
                                "Should have temporary ID"
                            );
                        }
                    }
                }
            }
            Ok(ResponseStreamResult::Err { error }) => {
                panic!("Expected success response, got error: {}", error.message);
            }
            Err(e) => {
                panic!("Failed to parse response with malformed tool calls: {}", e);
            }
        }
    }

    #[test]
    fn test_deepseek_response_with_empty_tool_calls() {
        // Test the DeepSeek response format with empty tool calls arrays
        let deepseek_response = json!({
            "id": "7fc265af-e548-44c8-b7e6-8732d764b6ac",
            "object": "chat.completion.chunk",
            "created": 1765515413,
            "model": "deepseek-v3.2",
            "usage": {
                "prompt_tokens": 11759,
                "completion_tokens": 1,
                "total_tokens": 11760
            },
            "extend_fields": {
                "traceId": "21010ca817655154089108071e1c01",
                "requestId": "379b33d83dc0c03fda2ba8328ec97fca"
            },
            "choices": [
                {
                    "index": 0,
                    "delta": {
                        "role": "assistant",
                        "content": "**",
                        "tool_calls": []
                    }
                }
            ]
        });

        let response_str = deepseek_response.to_string();
        let result = parse_response_stream_result(&response_str);

        match result {
            Ok(ResponseStreamResult::Ok(event)) => {
                assert_eq!(event.choices.len(), 1);
                // Empty tool calls should be treated as None
                if let Some(choice) = event.choices.first() {
                    if let Some(delta) = &choice.delta {
                        assert!(
                            delta.tool_calls.is_none(),
                            "Empty tool calls should be treated as None"
                        );
                    }
                }
            }
            Ok(ResponseStreamResult::Err { error }) => {
                panic!("Expected success response, got error: {}", error.message);
            }
            Err(e) => {
                panic!("Failed to parse DeepSeek response: {}", e);
            }
        }
    }

    #[test]
    fn test_deepseek_response_with_malformed_tool_calls() {
        // Test the DeepSeek response format with malformed tool calls
        let deepseek_response = json!({
            "id": "7fc265af-e548-44c8-b7e6-8732d764b6ac",
            "object": "chat.completion.chunk",
            "created": 1765515413,
            "model": "deepseek-v3.2",
            "usage": {
                "prompt_tokens": 11759,
                "completion_tokens": 1,
                "total_tokens": 11760
            },
            "extend_fields": {
                "traceId": "21010ca817655154089108071e1c01",
                "requestId": "379b33d83dc0c03fda2ba8328ec97fca"
            },
            "choices": [
                {
                    "index": 0,
                    "delta": {
                        "role": "assistant",
                        "content": "**",
                        "tool_calls": [
                            {
                                "type": "function",
                                "function": {
                                    "arguments": "malformed json",
                                    "name": "grep"
                                }
                            }
                        ]
                    }
                }
            ]
        });

        let response_str = deepseek_response.to_string();
        let result = parse_response_stream_result(&response_str);

        match result {
            Ok(ResponseStreamResult::Ok(event)) => {
                assert_eq!(event.choices.len(), 1);
                // Malformed tool calls should be treated as None
                if let Some(choice) = event.choices.first() {
                    if let Some(delta) = &choice.delta {
                        assert!(
                            delta.tool_calls.is_none(),
                            "Malformed tool calls should be treated as None"
                        );
                    }
                }
            }
            Ok(ResponseStreamResult::Err { error }) => {
                panic!("Expected success response, got error: {}", error.message);
            }
            Err(e) => {
                panic!(
                    "Failed to parse DeepSeek response with malformed tool calls: {}",
                    e
                );
            }
        }
    }
}
