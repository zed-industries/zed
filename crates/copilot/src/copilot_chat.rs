use std::path::PathBuf;
use std::sync::OnceLock;
use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use chrono::DateTime;
use fs::Fs;
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, StreamExt};
use gpui::{AppContext, AsyncAppContext, Global};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use isahc::config::Configurable;
use paths::home_dir;
use serde::{Deserialize, Serialize};
use settings::watch_config_file;
use strum::EnumIter;
use ui::Context;

pub const COPILOT_CHAT_COMPLETION_URL: &'static str =
    "https://api.githubcopilot.com/chat/completions";
pub const COPILOT_CHAT_AUTH_URL: &'static str = "https://api.github.com/copilot_internal/v2/token";

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
    #[serde(alias = "gpt-4o", rename = "gpt-4o-2024-05-13")]
    Gpt4o,
    #[serde(alias = "gpt-4", rename = "gpt-4")]
    Gpt4,
    #[serde(alias = "gpt-3.5-turbo", rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
}

impl Model {
    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "gpt-4o" => Ok(Self::Gpt4o),
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-3.5-turbo" => Ok(Self::Gpt3_5Turbo),
            _ => Err(anyhow!("Invalid model id: {}", id)),
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::Gpt3_5Turbo => "gpt-3.5-turbo",
            Self::Gpt4 => "gpt-4",
            Self::Gpt4o => "gpt-4o",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Gpt3_5Turbo => "GPT-3.5",
            Self::Gpt4 => "GPT-4",
            Self::Gpt4o => "GPT-4o",
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Gpt4o => 128000,
            Self::Gpt4 => 8192,
            Self::Gpt3_5Turbo => 16385,
        }
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

#[derive(Deserialize)]
struct ApiTokenResponse {
    token: String,
    expires_at: i64,
}

#[derive(Clone)]
struct ApiToken {
    api_key: String,
    expires_at: DateTime<chrono::Utc>,
}

impl ApiToken {
    pub fn remaining_seconds(&self) -> i64 {
        self.expires_at
            .timestamp()
            .saturating_sub(chrono::Utc::now().timestamp())
    }
}

impl TryFrom<ApiTokenResponse> for ApiToken {
    type Error = anyhow::Error;

    fn try_from(response: ApiTokenResponse) -> Result<Self, Self::Error> {
        let expires_at = DateTime::from_timestamp(response.expires_at, 0)
            .ok_or_else(|| anyhow!("invalid expires_at"))?;

        Ok(Self {
            api_key: response.token,
            expires_at,
        })
    }
}

struct GlobalCopilotChat(gpui::Model<CopilotChat>);

impl Global for GlobalCopilotChat {}

pub struct CopilotChat {
    oauth_token: Option<String>,
    api_token: Option<ApiToken>,
    client: Arc<dyn HttpClient>,
}

pub fn init(fs: Arc<dyn Fs>, client: Arc<dyn HttpClient>, cx: &mut AppContext) {
    let copilot_chat = cx.new_model(|cx| CopilotChat::new(fs, client, cx));
    cx.set_global(GlobalCopilotChat(copilot_chat));
}

fn copilot_chat_config_path() -> &'static PathBuf {
    static COPILOT_CHAT_CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();

    COPILOT_CHAT_CONFIG_DIR.get_or_init(|| {
        if cfg!(target_os = "windows") {
            home_dir().join("AppData").join("Local")
        } else {
            home_dir().join(".config")
        }
        .join("github-copilot")
        .join("hosts.json")
    })
}

impl CopilotChat {
    pub fn global(cx: &AppContext) -> Option<gpui::Model<Self>> {
        cx.try_global::<GlobalCopilotChat>()
            .map(|model| model.0.clone())
    }

    pub fn new(fs: Arc<dyn Fs>, client: Arc<dyn HttpClient>, cx: &AppContext) -> Self {
        let mut config_file_rx = watch_config_file(
            cx.background_executor(),
            fs,
            copilot_chat_config_path().clone(),
        );

        cx.spawn(|cx| async move {
            while let Some(contents) = config_file_rx.next().await {
                let oauth_token = extract_oauth_token(contents);

                cx.update(|cx| {
                    if let Some(this) = Self::global(cx).as_ref() {
                        this.update(cx, |this, cx| {
                            this.oauth_token = oauth_token;
                            cx.notify();
                        });
                    }
                })?;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        Self {
            oauth_token: None,
            api_token: None,
            client,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.oauth_token.is_some()
    }

    pub async fn stream_completion(
        request: Request,
        low_speed_timeout: Option<Duration>,
        mut cx: AsyncAppContext,
    ) -> Result<BoxStream<'static, Result<ResponseEvent>>> {
        let Some(this) = cx.update(|cx| Self::global(cx)).ok().flatten() else {
            return Err(anyhow!("Copilot chat is not enabled"));
        };

        let (oauth_token, api_token, client) = this.read_with(&cx, |this, _| {
            (
                this.oauth_token.clone(),
                this.api_token.clone(),
                this.client.clone(),
            )
        })?;

        let oauth_token = oauth_token.ok_or_else(|| anyhow!("No OAuth token available"))?;

        let token = match api_token {
            Some(api_token) if api_token.remaining_seconds() > 5 * 60 => api_token.clone(),
            _ => {
                let token =
                    request_api_token(&oauth_token, client.clone(), low_speed_timeout).await?;
                this.update(&mut cx, |this, cx| {
                    this.api_token = Some(token.clone());
                    cx.notify();
                })?;
                token
            }
        };

        stream_completion(client.clone(), token.api_key, request, low_speed_timeout).await
    }
}

async fn request_api_token(
    oauth_token: &str,
    client: Arc<dyn HttpClient>,
    low_speed_timeout: Option<Duration>,
) -> Result<ApiToken> {
    let mut request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(COPILOT_CHAT_AUTH_URL)
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

        let parsed: ApiTokenResponse = serde_json::from_str(body_str)?;
        ApiToken::try_from(parsed)
    } else {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;

        Err(anyhow!("Failed to request API token: {}", body_str))
    }
}

fn extract_oauth_token(contents: String) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(&contents)
        .map(|v| {
            v["github.com"]["oauth_token"]
                .as_str()
                .map(|v| v.to_string())
        })
        .ok()
        .flatten()
}

async fn stream_completion(
    client: Arc<dyn HttpClient>,
    api_key: String,
    request: Request,
    low_speed_timeout: Option<Duration>,
) -> Result<BoxStream<'static, Result<ResponseEvent>>> {
    let mut request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(COPILOT_CHAT_COMPLETION_URL)
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
