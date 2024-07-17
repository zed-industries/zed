use std::{env, path::PathBuf, sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime};
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, StreamExt};
use http::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use isahc::config::Configurable;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::EnumIter;

pub const COPILOT_CHAT_API_URL: &'static str = "https://api.githubcopilot.com";
pub const COPILOT_CHAT_AUTH_URL: &'static str = "https://api.github.com";

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(alias = "gpt-4", rename = "gpt-4")]
    Gpt4,
    #[serde(alias = "gpt-3.5", rename = "gpt-3.5")]
    Gpt3_5,
}

impl Model {
    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-3.5" => Ok(Self::Gpt3_5),
            _ => Err(anyhow!("Invalid model id: {}", id)),
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::Gpt3_5 => "gpt-3.5",
            Self::Gpt4 => "gpt-4",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Gpt3_5 => "GPT-3.5",
            Self::Gpt4 => "GPT-4",
        }
    }

    pub fn max_token_count(&self) -> usize {
        8192
    }
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub intent: bool,
    pub n: usize,
    pub stream: bool,
    pub temperature: f32,
    pub model: Model,
    pub messages: Vec<ChatMessage>,
}

impl Request {
    pub fn new(model: Model, messages: Vec<ChatMessage>) -> Self {
        Self {
            intent: true,
            n: 1,
            stream: true,
            temperature: 0.1,
            model,
            messages,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct ResponseEvent {
    pub choices: Vec<ResponseChoice>,
    pub created: u64,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseChoice {
    pub index: usize,
    pub finish_reason: Option<String>,
    pub delta: ResponseDelta,
}

#[derive(Debug, Deserialize)]
pub struct ResponseDelta {
    pub content: Option<String>,
    pub role: Option<Role>,
}

pub async fn request_api_token(
    oauth_token: &str,
    api_url: &str,
    client: Arc<dyn HttpClient>,
    low_speed_timeout: Option<Duration>,
) -> Result<(String, NaiveDateTime), anyhow::Error> {
    let uri = format!("{api_url}/copilot_internal/v2/token");

    let mut request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Authorization", format!("token {}", oauth_token))
        .header("Accept", "application/json");

    if let Some(low_speed_timeout) = low_speed_timeout {
        request_builder = request_builder.low_speed_timeout(100, low_speed_timeout);
    }

    let request = request_builder.body(AsyncBody::empty())?;

    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;

        let parsed: Value = serde_json::from_str(body_str)?;
        Ok((
            parsed["token"].as_str().unwrap().to_string(),
            DateTime::from_timestamp(parsed["expires_at"].as_i64().unwrap(), 0)
                .unwrap()
                .naive_utc(),
        ))
    } else {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;

        Err(anyhow!("Failed to request API token: {}", body_str))
    }
}

pub async fn fetch_copilot_oauth_token() -> Result<String, anyhow::Error> {
    let path = if cfg!(target_os = "linux") {
        PathBuf::from(env::var_os("XDG_CONFIG_HOME").unwrap_or("~/.config".into()))
    } else if cfg!(target_os = "windows") {
        PathBuf::from(env::var_os("APPDATA").unwrap_or("~/AppData".into()))
    } else {
        PathBuf::from(env::var_os("HOME").unwrap_or("~/".into())).join(".config")
    }
    .join("github-copilot")
    .join("hosts.json");

    match async_fs::metadata(&path).await {
            Ok(_) => {
                let contents = async_fs::read_to_string(&path).await?;

                let hosts: serde_json::Value  = serde_json::from_str(&contents)?;

                let host = hosts.get("github.com").ok_or_else(|| anyhow::anyhow!("No host found for github.com"))?.clone();

                let token = host.get("oauth_token").ok_or_else(|| anyhow::anyhow!("No oauth token found for github.com"))?.as_str().unwrap();

                Ok(token.to_string())
            },
            Err(_) => Err(anyhow::anyhow!("Failed to determine whether or not you have a file with the necessary token. Make sure you have read permissions for your config directory."))
        }
}

pub async fn stream_completion(
    client: Arc<dyn HttpClient>,
    api_url: String,
    api_key: String,
    request: Request,
    low_speed_timeout: Option<Duration>,
) -> Result<BoxStream<'static, Result<ResponseEvent>>> {
    let uri = format!("{api_url}/chat/completions");
    let mut request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header(
            "Editor-Version",
            format!(
                "Zed/{}",
                option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
            ),
        )
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("Copilot-Integration-Id", "vscode-chat");

    if let Some(low_speed_timeout) = low_speed_timeout {
        request_builder = request_builder.low_speed_timeout(100, low_speed_timeout);
    }
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
                        if line.starts_with("[DONE]") {
                            return None;
                        }

                        match serde_json::from_str::<ResponseEvent>(line) {
                            Ok(response) => {
                                if response.choices.first().is_none()
                                    || response.choices.first().unwrap().finish_reason.is_some()
                                {
                                    None
                                } else {
                                    Some(Ok(response))
                                }
                            }
                            Err(error) => Some(Err(anyhow!(error))),
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;

        match serde_json::from_str::<ResponseEvent>(body_str) {
            Ok(_) => Err(anyhow!(
                "Unexpected success response while expecting an error: {}",
                body_str,
            )),
            Err(_) => Err(anyhow!(
                "Failed to connect to API: {} {}",
                response.status(),
                body_str,
            )),
        }
    }
}
