use anyhow::{anyhow, Context, Result};
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, Stream, StreamExt};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use isahc::config::Configurable;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::{convert::TryFrom, future::Future, pin::Pin, time::Duration};
use strum::EnumIter;

pub const KIMI_AI_API_URL: &str = "https://api.moonshot.cn/v1";

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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[serde(rename = "moonshot-v1-8k", alias = "moonshot-v1-8k")]
    MoonShotV18K,
    #[serde(rename = "moonshot-v1-32k", alias = "moonshot-v1-32k")]
    MoonShotV132K,
    #[serde(rename = "moonshot-v1-128k", alias = "moonshot-v1-128k")]
    MoonShotV1128K,
    #[serde(rename = "moonshot-v1-auto", alias = "moonshot-v1-auto")]
    MoonShotV1Auto,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        max_tokens: usize,
        max_output_tokens: Option<u32>,
    },
}

impl Model {
    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "moonshot-v1-8k" => Ok(Self::MoonShotV18K),
            "moonshot-v1-32k" => Ok(Self::MoonShotV132K),
            "moonshot-v1-128k" => Ok(Self::MoonShotV1128K),
            "moonshot-v1-auto" => Ok(Self::MoonShotV1Auto),
            _ => Err(anyhow!("invalid model id")),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::MoonShotV18K => "moonshot-v1-8k",
            Self::MoonShotV132K => "moonshot-v1-32k",
            Self::MoonShotV1128K => "moonshot-v1-128k",
            Self::MoonShotV1Auto => "moonshot-v1-auto",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::MoonShotV18K => "moonshot-v1-8k",
            Self::MoonShotV132K => "moonshot-v1-32k",
            Self::MoonShotV1128K => "moonshot-v1-128k",
            Self::MoonShotV1Auto => "moonshot-v1-auto",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::MoonShotV18K => 8 * 1024,
            Self::MoonShotV132K => 32 * 1024,
            Self::MoonShotV1128K => 128 * 1024,
            Self::MoonShotV1Auto => 128 * 1024,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u32> {
        match self {
            Self::MoonShotV18K => Some(4 * 1024),
            Self::MoonShotV132K => Some(16 * 1024),
            Self::MoonShotV1128K => Some(64 * 1024),
            Self::MoonShotV1Auto => Some(64 * 1024),
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
    pub max_tokens: Option<u32>,
    pub stop: Vec<String>,
    pub temperature: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<KimiToolChoice>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<KimiToolDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KimiToolChoice {
    Auto,
    Required,
    None,
    Other(KimiToolDefinition),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KimiToolDefinition {
    #[allow(dead_code)]
    Function { function: KimiFunctionDefinition },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KimiFunctionDefinition {
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
pub enum KimiResponseStreamResult {
    Ok(KimiResponseStreamEvent),
    Err { error: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KimiResponseStreamEvent {
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
    low_speed_timeout: Option<Duration>,
) -> Result<BoxStream<'static, Result<KimiResponseStreamEvent>>> {
    let uri = format!("{api_url}/chat/completions");
    let mut request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri.clone())
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));
    if let Some(low_speed_timeout) = low_speed_timeout {
        request_builder = request_builder.low_speed_timeout(100, low_speed_timeout);
    };

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    log::info!("status: kimiai: {}", response.status());
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
                                Ok(KimiResponseStreamResult::Ok(response)) => Some(Ok(response)),
                                Ok(KimiResponseStreamResult::Err { error }) => {
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
        log::info!("{body}");
        #[derive(Deserialize)]
        struct KimiAiResponse {
            error: KimiAiError,
        }

        #[derive(Deserialize)]
        struct KimiAiError {
            message: String,
        }

        match serde_json::from_str::<KimiAiResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "Failed to connect to KimiAi API: {}",
                response.error.message,
            )),

            _ => Err(anyhow!(
                "Failed to connect to KimiAi API: {} {}",
                response.status(),
                body,
            )),
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum KimiAiEmbeddingModel {
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
}

#[derive(Serialize)]
struct KimiAiEmbeddingRequest<'a> {
    model: KimiAiEmbeddingModel,
    input: Vec<&'a str>,
}

#[derive(Deserialize)]
pub struct KimiAiEmbeddingResponse {
    pub data: Vec<KimiAiEmbedding>,
}

#[derive(Deserialize)]
pub struct KimiAiEmbedding {
    pub embedding: Vec<f32>,
}

pub fn embed<'a>(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    model: KimiAiEmbeddingModel,
    texts: impl IntoIterator<Item = &'a str>,
) -> impl 'static + Future<Output = Result<KimiAiEmbeddingResponse>> {
    let uri = format!("{api_url}/embeddings");

    let request = KimiAiEmbeddingRequest {
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
    log::info!("api_key: {}", api_key);
    async move {
        let mut response = request?.await?;
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        if response.status().is_success() {
            let response: KimiAiEmbeddingResponse =
                serde_json::from_str(&body).context("failed to parse KimiAi embedding response")?;
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

pub async fn extract_tool_args_from_events(
    tool_name: String,
    mut events: Pin<Box<dyn Send + Stream<Item = Result<KimiResponseStreamEvent>>>>,
) -> Result<impl Send + Stream<Item = Result<String>>> {
    let mut tool_use_index = None;
    let mut first_chunk = None;
    while let Some(event) = events.next().await {
        let call = event?.choices.into_iter().find_map(|choice| {
            choice.delta.tool_calls?.into_iter().find_map(|call| {
                if call.function.as_ref()?.name.as_deref()? == tool_name {
                    Some(call)
                } else {
                    None
                }
            })
        });
        if let Some(call) = call {
            tool_use_index = Some(call.index);
            first_chunk = call.function.and_then(|func| func.arguments);
            break;
        }
    }

    let Some(tool_use_index) = tool_use_index else {
        return Err(anyhow!("tool not used"));
    };

    Ok(events.filter_map(move |event| {
        let result = match event {
            Err(error) => Some(Err(error)),
            Ok(KimiResponseStreamEvent { choices, .. }) => choices.into_iter().find_map(|choice| {
                choice.delta.tool_calls?.into_iter().find_map(|call| {
                    if call.index == tool_use_index {
                        let func = call.function?;
                        let mut arguments = func.arguments?;
                        if let Some(mut first_chunk) = first_chunk.take() {
                            first_chunk.push_str(&arguments);
                            arguments = first_chunk
                        }
                        Some(Ok(arguments))
                    } else {
                        None
                    }
                })
            }),
        };

        async move { result }
    }))
}

pub fn extract_text_from_events(
    response: impl Stream<Item = Result<KimiResponseStreamEvent>>,
) -> impl Stream<Item = Result<String>> {
    response.filter_map(|response| async move {
        match response {
            Ok(mut response) => Some(Ok(response.choices.pop()?.delta.content?)),
            Err(error) => Some(Err(error)),
        }
    })
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}
