use anyhow::{anyhow, Context, Result};
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, StreamExt};
use http::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use isahc::config::Configurable;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{convert::TryFrom, future::Future, time::Duration};
use strum::EnumIter;

pub const OPEN_AI_API_URL: &str = "https://api.openai.com/v1";

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
    #[serde(rename = "gpt-3.5-turbo", alias = "gpt-3.5-turbo-0613")]
    ThreePointFiveTurbo,
    #[serde(rename = "gpt-4", alias = "gpt-4-0613")]
    Four,
    #[serde(rename = "gpt-4-turbo-preview", alias = "gpt-4-1106-preview")]
    FourTurbo,
    #[serde(rename = "gpt-4o", alias = "gpt-4o-2024-05-13")]
    #[default]
    FourOmni,
}

impl Model {
    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "gpt-3.5-turbo" => Ok(Self::ThreePointFiveTurbo),
            "gpt-4" => Ok(Self::Four),
            "gpt-4-turbo-preview" => Ok(Self::FourTurbo),
            "gpt-4o" => Ok(Self::FourOmni),
            _ => Err(anyhow!("invalid model id")),
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::ThreePointFiveTurbo => "gpt-3.5-turbo",
            Self::Four => "gpt-4",
            Self::FourTurbo => "gpt-4-turbo-preview",
            Self::FourOmni => "gpt-4o",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::ThreePointFiveTurbo => "gpt-3.5-turbo",
            Self::Four => "gpt-4",
            Self::FourTurbo => "gpt-4-turbo",
            Self::FourOmni => "gpt-4o",
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Model::ThreePointFiveTurbo => 4096,
            Model::Four => 8192,
            Model::FourTurbo => 128000,
            Model::FourOmni => 128000,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Request {
    pub model: Model,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    pub stop: Vec<String>,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
}

#[derive(Debug, Serialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Map<String, Value>>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    #[allow(dead_code)]
    Function { function: FunctionDefinition },
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCallChunk>,
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

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct ChoiceDelta {
    pub index: u32,
    pub delta: ResponseMessageDelta,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseStreamEvent {
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
) -> Result<BoxStream<'static, Result<ResponseStreamEvent>>> {
    let uri = format!("{api_url}/chat/completions");
    let mut request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));

    if let Some(low_speed_timeout) = low_speed_timeout {
        request_builder = request_builder.low_speed_timeout(100, low_speed_timeout);
    };

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
