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
    #[serde(rename = "gpt-5-codex")]
    FiveCodex,
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
        #[serde(default = "default_supports_chat_completions")]
        supports_chat_completions: bool,
    },
}

const fn default_supports_chat_completions() -> bool {
    true
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
            "gpt-5-codex" => Ok(Self::FiveCodex),
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
            Self::FiveCodex => "gpt-5-codex",
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
            Self::FiveCodex => "gpt-5-codex",
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
            Self::FiveCodex => 272_000,
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
            Self::FiveCodex => Some(128_000),
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

    pub fn supports_chat_completions(&self) -> bool {
        match self {
            Self::Custom {
                supports_chat_completions,
                ..
            } => *supports_chat_completions,
            Self::FiveCodex => false,
            _ => true,
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
            | Self::FiveCodex
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
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
    pub usage: Option<Usage>,
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

pub mod responses {
    use anyhow::{Result, anyhow};
    use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
    use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use crate::RequestError;

    #[derive(Serialize, Debug)]
    pub struct Request {
        pub model: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub input: Vec<Value>,
        #[serde(default)]
        pub stream: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub temperature: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub top_p: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_output_tokens: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parallel_tool_calls: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tool_choice: Option<super::ToolChoice>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub tools: Vec<ToolDefinition>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prompt_cache_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reasoning: Option<ReasoningConfig>,
    }

    #[derive(Serialize, Debug)]
    pub struct ReasoningConfig {
        pub effort: super::ReasoningEffort,
    }

    #[derive(Serialize, Debug)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum ToolDefinition {
        Function {
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            parameters: Option<Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            strict: Option<bool>,
        },
    }

    #[derive(Deserialize, Debug)]
    pub struct Error {
        pub message: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(tag = "type")]
    pub enum StreamEvent {
        #[serde(rename = "response.created")]
        Created { response: ResponseSummary },
        #[serde(rename = "response.in_progress")]
        InProgress { response: ResponseSummary },
        #[serde(rename = "response.output_item.added")]
        OutputItemAdded {
            output_index: usize,
            #[serde(default)]
            sequence_number: Option<u64>,
            item: ResponseOutputItem,
        },
        #[serde(rename = "response.output_item.done")]
        OutputItemDone {
            output_index: usize,
            #[serde(default)]
            sequence_number: Option<u64>,
            item: ResponseOutputItem,
        },
        #[serde(rename = "response.content_part.added")]
        ContentPartAdded {
            item_id: String,
            output_index: usize,
            content_index: usize,
            part: Value,
        },
        #[serde(rename = "response.content_part.done")]
        ContentPartDone {
            item_id: String,
            output_index: usize,
            content_index: usize,
            part: Value,
        },
        #[serde(rename = "response.output_text.delta")]
        OutputTextDelta {
            item_id: String,
            output_index: usize,
            #[serde(default)]
            content_index: Option<usize>,
            delta: String,
        },
        #[serde(rename = "response.output_text.done")]
        OutputTextDone {
            item_id: String,
            output_index: usize,
            #[serde(default)]
            content_index: Option<usize>,
            text: String,
        },
        #[serde(rename = "response.function_call_arguments.delta")]
        FunctionCallArgumentsDelta {
            item_id: String,
            output_index: usize,
            delta: String,
            #[serde(default)]
            sequence_number: Option<u64>,
        },
        #[serde(rename = "response.function_call_arguments.done")]
        FunctionCallArgumentsDone {
            item_id: String,
            output_index: usize,
            arguments: String,
            #[serde(default)]
            sequence_number: Option<u64>,
        },
        #[serde(rename = "response.completed")]
        Completed { response: ResponseSummary },
        #[serde(rename = "response.incomplete")]
        Incomplete { response: ResponseSummary },
        #[serde(rename = "response.failed")]
        Failed { response: ResponseSummary },
        #[serde(rename = "response.error")]
        Error { error: Error },
        #[serde(rename = "error")]
        GenericError { error: Error },
        #[serde(other)]
        Unknown,
    }

    #[derive(Deserialize, Debug, Default, Clone)]
    pub struct ResponseSummary {
        #[serde(default)]
        pub id: Option<String>,
        #[serde(default)]
        pub status: Option<String>,
        #[serde(default)]
        pub status_details: Option<ResponseStatusDetails>,
        #[serde(default)]
        pub usage: Option<ResponseUsage>,
        #[serde(default)]
        pub output: Vec<ResponseOutputItem>,
    }

    #[derive(Deserialize, Debug, Default, Clone)]
    pub struct ResponseStatusDetails {
        #[serde(default)]
        pub reason: Option<String>,
        #[serde(default)]
        pub r#type: Option<String>,
        #[serde(default)]
        pub error: Option<Value>,
    }

    #[derive(Deserialize, Debug, Default, Clone)]
    pub struct ResponseUsage {
        #[serde(default)]
        pub input_tokens: Option<u64>,
        #[serde(default)]
        pub output_tokens: Option<u64>,
        #[serde(default)]
        pub total_tokens: Option<u64>,
    }

    #[derive(Deserialize, Debug, Clone)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum ResponseOutputItem {
        Message(ResponseOutputMessage),
        FunctionCall(ResponseFunctionToolCall),
        #[serde(other)]
        Unknown,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct ResponseOutputMessage {
        #[serde(default)]
        pub id: Option<String>,
        #[serde(default)]
        pub content: Vec<Value>,
        #[serde(default)]
        pub role: Option<String>,
        #[serde(default)]
        pub status: Option<String>,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct ResponseFunctionToolCall {
        #[serde(default)]
        pub id: Option<String>,
        #[serde(default)]
        pub arguments: String,
        #[serde(default)]
        pub call_id: Option<String>,
        #[serde(default)]
        pub name: Option<String>,
        #[serde(default)]
        pub status: Option<String>,
    }

    pub async fn stream_response(
        client: &dyn HttpClient,
        provider_name: &str,
        api_url: &str,
        api_key: &str,
        request: Request,
    ) -> Result<BoxStream<'static, Result<StreamEvent>>, RequestError> {
        let uri = format!("{api_url}/responses");
        let request_builder = HttpRequest::builder()
            .method(Method::POST)
            .uri(uri)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key.trim()));

        let is_streaming = request.stream;
        let request = request_builder
            .body(AsyncBody::from(
                serde_json::to_string(&request).map_err(|e| RequestError::Other(e.into()))?,
            ))
            .map_err(|e| RequestError::Other(e.into()))?;

        let mut response = client.send(request).await?;
        if response.status().is_success() {
            if is_streaming {
                let reader = BufReader::new(response.into_body());
                Ok(reader
                    .lines()
                    .filter_map(|line| async move {
                        match line {
                            Ok(line) => {
                                let line = line
                                    .strip_prefix("data: ")
                                    .or_else(|| line.strip_prefix("data:"))?;
                                if line == "[DONE]" || line.is_empty() {
                                    None
                                } else {
                                    match serde_json::from_str::<StreamEvent>(line) {
                                        Ok(event) => Some(Ok(event)),
                                        Err(error) => {
                                            log::error!(
                                                "Failed to parse OpenAI responses stream event: `{}`\nResponse: `{}`",
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

                match serde_json::from_str::<ResponseSummary>(&body) {
                    Ok(response_summary) => {
                        let events = vec![
                            StreamEvent::Created {
                                response: response_summary.clone(),
                            },
                            StreamEvent::InProgress {
                                response: response_summary.clone(),
                            },
                        ];

                        let mut all_events = events;
                        for (output_index, item) in response_summary.output.iter().enumerate() {
                            all_events.push(StreamEvent::OutputItemAdded {
                                output_index,
                                sequence_number: None,
                                item: item.clone(),
                            });

                            match item {
                                ResponseOutputItem::Message(message) => {
                                    for content_item in &message.content {
                                        if let Some(text) = content_item.get("text") {
                                            if let Some(text_str) = text.as_str() {
                                                if let Some(ref item_id) = message.id {
                                                    all_events.push(StreamEvent::OutputTextDelta {
                                                        item_id: item_id.clone(),
                                                        output_index,
                                                        content_index: None,
                                                        delta: text_str.to_string(),
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                                ResponseOutputItem::FunctionCall(function_call) => {
                                    if let Some(ref item_id) = function_call.id {
                                        all_events.push(StreamEvent::FunctionCallArgumentsDone {
                                            item_id: item_id.clone(),
                                            output_index,
                                            arguments: function_call.arguments.clone(),
                                            sequence_number: None,
                                        });
                                    }
                                }
                                ResponseOutputItem::Unknown => {}
                            }

                            all_events.push(StreamEvent::OutputItemDone {
                                output_index,
                                sequence_number: None,
                                item: item.clone(),
                            });
                        }

                        all_events.push(StreamEvent::Completed {
                            response: response_summary,
                        });

                        Ok(futures::stream::iter(all_events.into_iter().map(Ok)).boxed())
                    }
                    Err(error) => {
                        log::error!(
                            "Failed to parse OpenAI non-streaming response: `{}`\nResponse: `{}`",
                            error,
                            body,
                        );
                        Err(RequestError::Other(anyhow!(error)))
                    }
                }
            }
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
}
