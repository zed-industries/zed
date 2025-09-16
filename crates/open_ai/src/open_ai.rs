use anyhow::{Context as _, Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{convert::TryFrom, future::Future};
use strum::EnumIter;

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
    pub temperature: f32,
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

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Minimal,
    Low,
    Medium,
    High,
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ResponseMessageDelta {
    pub role: Option<Role>,
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "is_none_or_empty")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,

    // There is also an optional `type` field that would determine if a
    // function is there. Sometimes this streams in with the `function` before
    // it streams in the `type`
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceDelta {
    pub index: u32,
    pub delta: ResponseMessageDelta,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiError {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseStreamResult {
    Ok(ResponseStreamEvent),
    Err { error: OpenAiError },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamEvent {
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent>>> {
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
                        let line = line.strip_prefix("data: ").or_else(|| line.strip_prefix("data:"))?;
                        if line == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str(line) {
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
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct OpenAiResponse {
            error: OpenAiError,
        }

        match serde_json::from_str::<OpenAiResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "API request to {} failed: {}",
                api_url,
                response.error.message,
            )),

            _ => anyhow::bail!(
                "API request to {} failed with status {}: {}",
                api_url,
                response.status(),
                body,
            ),
        }
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
