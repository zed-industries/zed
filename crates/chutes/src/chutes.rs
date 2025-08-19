use anyhow::{Context as _, Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{convert::TryFrom, future::Future};
use strum::EnumIter;

pub const CHUTES_API_URL: &str = "https://api.chutes.ai/v1";

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
    #[serde(rename = "llama-3-8b")]
    Llama3_8B,
    #[serde(rename = "llama-3-70b")]
    Llama3_70B,
    #[serde(rename = "llama-3.1-8b")]
    Llama3_1_8B,
    #[serde(rename = "llama-3.1-70b")]
    Llama3_1_70B,
    #[serde(rename = "llama-3.1-405b")]
    #[default]
    Llama3_1_405B,
    #[serde(rename = "mistral-7b")]
    Mistral7B,
    #[serde(rename = "mixtral-8x7b")]
    Mixtral8x7B,
    #[serde(rename = "claude-3-sonnet")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku")]
    Claude3Haiku,
    #[serde(rename = "gpt-4")]
    GPT4,
    #[serde(rename = "gpt-3.5-turbo")]
    GPT3_5Turbo,
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
        Self::Llama3_1_8B
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "llama-3-8b" => Ok(Self::Llama3_8B),
            "llama-3-70b" => Ok(Self::Llama3_70B),
            "llama-3.1-8b" => Ok(Self::Llama3_1_8B),
            "llama-3.1-70b" => Ok(Self::Llama3_1_70B),
            "llama-3.1-405b" => Ok(Self::Llama3_1_405B),
            "mistral-7b" => Ok(Self::Mistral7B),
            "mixtral-8x7b" => Ok(Self::Mixtral8x7B),
            "claude-3-sonnet" => Ok(Self::Claude3Sonnet),
            "claude-3-haiku" => Ok(Self::Claude3Haiku),
            "gpt-4" => Ok(Self::GPT4),
            "gpt-3.5-turbo" => Ok(Self::GPT3_5Turbo),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Llama3_8B => "llama-3-8b",
            Self::Llama3_70B => "llama-3-70b",
            Self::Llama3_1_8B => "llama-3.1-8b",
            Self::Llama3_1_70B => "llama-3.1-70b",
            Self::Llama3_1_405B => "llama-3.1-405b",
            Self::Mistral7B => "mistral-7b",
            Self::Mixtral8x7B => "mixtral-8x7b",
            Self::Claude3Sonnet => "claude-3-sonnet",
            Self::Claude3Haiku => "claude-3-haiku",
            Self::GPT4 => "gpt-4",
            Self::GPT3_5Turbo => "gpt-3.5-turbo",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Llama3_8B => "Llama 3 8B",
            Self::Llama3_70B => "Llama 3 70B",
            Self::Llama3_1_8B => "Llama 3.1 8B",
            Self::Llama3_1_70B => "Llama 3.1 70B",
            Self::Llama3_1_405B => "Llama 3.1 405B",
            Self::Mistral7B => "Mistral 7B",
            Self::Mixtral8x7B => "Mixtral 8x7B",
            Self::Claude3Sonnet => "Claude 3 Sonnet",
            Self::Claude3Haiku => "Claude 3 Haiku",
            Self::GPT4 => "GPT-4",
            Self::GPT3_5Turbo => "GPT-3.5 Turbo",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Llama3_8B => 8_192,
            Self::Llama3_70B => 8_192,
            Self::Llama3_1_8B => 128_000,
            Self::Llama3_1_70B => 128_000,
            Self::Llama3_1_405B => 128_000,
            Self::Mistral7B => 32_768,
            Self::Mixtral8x7B => 32_768,
            Self::Claude3Sonnet => 200_000,
            Self::Claude3Haiku => 200_000,
            Self::GPT4 => 128_000,
            Self::GPT3_5Turbo => 16_385,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
            Self::Llama3_8B => Some(8_192),
            Self::Llama3_70B => Some(8_192),
            Self::Llama3_1_8B => Some(128_000),
            Self::Llama3_1_70B => Some(128_000),
            Self::Llama3_1_405B => Some(128_000),
            Self::Mistral7B => Some(32_768),
            Self::Mixtral8x7B => Some(32_768),
            Self::Claude3Sonnet => Some(4_096),
            Self::Claude3Haiku => Some(4_096),
            Self::GPT4 => Some(4_096),
            Self::GPT3_5Turbo => Some(4_096),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u64>,
    pub temperature: Option<f32>,
    pub stream: bool,
}

#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Deserialize)]
pub struct Choice {
    pub index: u64,
    pub delta: Option<Delta>,
    pub message: Option<Message>,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize)]
pub struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
pub struct ChutesError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<String>,
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<Response>>> {
    let uri = format!("{api_url}/chat/completions");
    let body = AsyncBody::from(serde_json::to_string(&request)?);
    
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body)?;

    let mut response = client.send(request).await?;

    if !response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct ChutesResponse {
            error: ChutesError,
        }

        match serde_json::from_str::<ChutesResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "API request failed: {}",
                response.error.message,
            )),
            _ => anyhow::bail!(
                "API request failed with status {}: {}",
                response.status(),
                body,
            ),
        }
    } else {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.trim();
                        if line.starts_with("data: ") {
                            let content = &line[6..];
                            if content == "[DONE]" {
                                return None;
                            }
                            match serde_json::from_str::<Response>(content) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => {
                                    log::error!("Error parsing Chutes response: {error}, line: {line}");
                                    Some(Err(anyhow!(error)))
                                }
                            }
                        } else {
                            None
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    }
}

pub async fn complete(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<Response> {
    let uri = format!("{api_url}/chat/completions");
    let body = AsyncBody::from(serde_json::to_string(&request)?);
    
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body)?;

    let mut response = client.send(request).await?;
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    if response.status().is_success() {
        let response: Response = serde_json::from_str(&body)
            .context("failed to parse Chutes response")?;
        Ok(response)
    } else {
        #[derive(Deserialize)]
        struct ChutesResponse {
            error: ChutesError,
        }

        match serde_json::from_str::<ChutesResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "API request failed: {}",
                response.error.message,
            )),
            _ => anyhow::bail!(
                "API request failed with status {}: {}",
                response.status(),
                body,
            ),
        }
    }
}