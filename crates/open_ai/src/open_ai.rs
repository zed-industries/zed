use anyhow::{Context as _, Result, anyhow};
use futures::{
    AsyncBufReadExt, AsyncReadExt, StreamExt,
    io::BufReader,
    stream::{self, BoxStream},
};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    convert::TryFrom,
    future::{self, Future},
};
use strum::EnumIter;

pub const OPEN_AI_API_URL: &str = "https://api.openai.com/v1";

fn is_none_or_empty<T: AsRef<[U]>, U>(opt: &Option<T>) -> bool {
    opt.as_ref().map_or(true, |v| v.as_ref().is_empty())
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

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo", alias = "gpt-3.5-turbo")]
    ThreePointFiveTurbo,
    #[serde(rename = "gpt-4", alias = "gpt-4")]
    Four,
    #[serde(rename = "gpt-4-turbo", alias = "gpt-4-turbo")]
    FourTurbo,
    #[serde(rename = "gpt-4o", alias = "gpt-4o")]
    #[default]
    FourOmni,
    #[serde(rename = "gpt-4o-mini", alias = "gpt-4o-mini")]
    FourOmniMini,
    #[serde(rename = "gpt-4.1", alias = "gpt-4.1")]
    FourPointOne,
    #[serde(rename = "gpt-4.1-mini", alias = "gpt-4.1-mini")]
    FourPointOneMini,
    #[serde(rename = "gpt-4.1-nano", alias = "gpt-4.1-nano")]
    FourPointOneNano,
    #[serde(rename = "o1", alias = "o1")]
    O1,
    #[serde(rename = "o1-preview", alias = "o1-preview")]
    O1Preview,
    #[serde(rename = "o1-mini", alias = "o1-mini")]
    O1Mini,
    #[serde(rename = "o3-mini", alias = "o3-mini")]
    O3Mini,
    #[serde(rename = "o3", alias = "o3")]
    O3,
    #[serde(rename = "o4-mini", alias = "o4-mini")]
    O4Mini,

    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: usize,
        max_output_tokens: Option<u32>,
        max_completion_tokens: Option<u32>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
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
            "o1-preview" => Ok(Self::O1Preview),
            "o1-mini" => Ok(Self::O1Mini),
            "o3-mini" => Ok(Self::O3Mini),
            "o3" => Ok(Self::O3),
            "o4-mini" => Ok(Self::O4Mini),
            _ => Err(anyhow!("invalid model id")),
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
            Self::O1Preview => "o1-preview",
            Self::O1Mini => "o1-mini",
            Self::O3Mini => "o3-mini",
            Self::O3 => "o3",
            Self::O4Mini => "o4-mini",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn matches_id(&self, other_id: &str) -> bool {
        self.id() == other_id
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
            Self::O1Preview => "o1-preview",
            Self::O1Mini => "o1-mini",
            Self::O3Mini => "o3-mini",
            Self::O3 => "o3",
            Self::O4Mini => "o4-mini",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> usize {
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
            Self::O1Preview => 128_000,
            Self::O1Mini => 128_000,
            Self::O3Mini => 200_000,
            Self::O3 => 200_000,
            Self::O4Mini => 200_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u32> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
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
            | Self::O1
            | Self::O1Preview
            | Self::O1Mini => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
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
#[serde(untagged)]
pub enum ToolChoice {
    Auto,
    Required,
    None,
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
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceDelta {
    pub index: u32,
    pub delta: ResponseMessageDelta,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseStreamResult {
    Ok(ResponseStreamEvent),
    Err { error: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamEvent {
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
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
pub struct Choice {
    pub index: u32,
    pub message: RequestMessage,
    pub finish_reason: Option<String>,
}

pub async fn complete(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<Response> {
    let uri = format!("{api_url}/chat/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));

    let mut request_body = request;
    request_body.stream = false;

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request_body)?))?;
    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        let response: Response = serde_json::from_str(&body)?;
        Ok(response)
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct OpenAiResponse {
            error: OpenAiError,
        }

        #[derive(Deserialize)]
        struct OpenAiError {
            message: String,
        }

        match serde_json::from_str::<OpenAiResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "Failed to connect to OpenAI API: {}",
                response.error.message,
            )),

            _ => Err(anyhow!(
                "Failed to connect to OpenAI API: {} {}",
                response.status(),
                body,
            )),
        }
    }
}

pub async fn complete_text(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: CompletionRequest,
) -> Result<CompletionResponse> {
    let uri = format!("{api_url}/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        let response = serde_json::from_str(&body)?;
        Ok(response)
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct OpenAiResponse {
            error: OpenAiError,
        }

        #[derive(Deserialize)]
        struct OpenAiError {
            message: String,
        }

        match serde_json::from_str::<OpenAiResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "Failed to connect to OpenAI API: {}",
                response.error.message,
            )),

            _ => Err(anyhow!(
                "Failed to connect to OpenAI API: {} {}",
                response.status(),
                body,
            )),
        }
    }
}

fn adapt_response_to_stream(response: Response) -> ResponseStreamEvent {
    ResponseStreamEvent {
        created: response.created as u32,
        model: response.model,
        choices: response
            .choices
            .into_iter()
            .map(|choice| ChoiceDelta {
                index: choice.index,
                delta: ResponseMessageDelta {
                    role: Some(match choice.message {
                        RequestMessage::Assistant { .. } => Role::Assistant,
                        RequestMessage::User { .. } => Role::User,
                        RequestMessage::System { .. } => Role::System,
                        RequestMessage::Tool { .. } => Role::Tool,
                    }),
                    content: match choice.message {
                        RequestMessage::Assistant { content, .. } => content,
                        RequestMessage::User { content } => Some(content),
                        RequestMessage::System { content } => Some(content),
                        RequestMessage::Tool { content, .. } => Some(content),
                    },
                    tool_calls: None,
                },
                finish_reason: choice.finish_reason,
            })
            .collect(),
        usage: Some(response.usage),
    }
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent>>> {
    if request.model.starts_with("o1") {
        let response = complete(client, api_url, api_key, request).await;
        let response_stream_event = response.map(adapt_response_to_stream);
        return Ok(stream::once(future::ready(response_stream_event)).boxed());
    }

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
                                Ok(ResponseStreamResult::Ok(response)) => Some(Ok(response)),
                                Ok(ResponseStreamResult::Err { error }) => {
                                    Some(Err(anyhow!(error)))
                                }
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

        #[derive(Deserialize)]
        struct OpenAiResponse {
            error: OpenAiError,
        }

        #[derive(Deserialize)]
        struct OpenAiError {
            message: String,
        }

        match serde_json::from_str::<OpenAiResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "Failed to connect to OpenAI API: {}",
                response.error.message,
            )),

            _ => Err(anyhow!(
                "Failed to connect to OpenAI API: {} {}",
                response.status(),
                body,
            )),
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
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body)
        .map(|request| client.send(request));

    async move {
        let mut response = request?.await?;
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        if response.status().is_success() {
            let response: OpenAiEmbeddingResponse =
                serde_json::from_str(&body).context("failed to parse OpenAI embedding response")?;
            Ok(response)
        } else {
            Err(anyhow!(
                "error during embedding, status: {:?}, body: {:?}",
                response.status(),
                body
            ))
        }
    }
}
