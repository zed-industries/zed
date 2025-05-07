use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;

use anyhow::{Result, anyhow};
use chrono::DateTime;
use collections::HashSet;
use fs::Fs;
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use gpui::{App, AsyncApp, Global, prelude::*};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use paths::home_dir;
use serde::{Deserialize, Serialize};
use settings::watch_config_dir;
use strum::EnumIter;

pub const COPILOT_CHAT_COMPLETION_URL: &str = "https://api.githubcopilot.com/chat/completions";
pub const COPILOT_CHAT_AUTH_URL: &str = "https://api.github.com/copilot_internal/v2/token";

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
    #[serde(alias = "gpt-4.1", rename = "gpt-4.1")]
    Gpt4_1,
    #[serde(alias = "gpt-3.5-turbo", rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
    #[serde(alias = "o1", rename = "o1")]
    O1,
    #[serde(alias = "o1-mini", rename = "o3-mini")]
    O3Mini,
    #[serde(alias = "o3", rename = "o3")]
    O3,
    #[serde(alias = "o4-mini", rename = "o4-mini")]
    O4Mini,
    #[serde(alias = "claude-3-5-sonnet", rename = "claude-3.5-sonnet")]
    Claude3_5Sonnet,
    #[serde(alias = "claude-3-7-sonnet", rename = "claude-3.7-sonnet")]
    Claude3_7Sonnet,
    #[serde(
        alias = "claude-3.7-sonnet-thought",
        rename = "claude-3.7-sonnet-thought"
    )]
    Claude3_7SonnetThinking,
    #[serde(alias = "gemini-2.0-flash", rename = "gemini-2.0-flash-001")]
    Gemini20Flash,
    #[serde(alias = "gemini-2.5-pro", rename = "gemini-2.5-pro")]
    Gemini25Pro,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum ChatMessageContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    Image { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ImageUrl {
    pub url: String,
}

impl Model {
    pub fn default_fast() -> Self {
        Self::Claude3_7Sonnet
    }

    pub fn uses_streaming(&self) -> bool {
        match self {
            Self::Gpt4o
            | Self::Gpt4
            | Self::Gpt4_1
            | Self::Gpt3_5Turbo
            | Self::O3
            | Self::O4Mini
            | Self::Claude3_5Sonnet
            | Self::Claude3_7Sonnet
            | Self::Claude3_7SonnetThinking => true,
            Self::O3Mini | Self::O1 | Self::Gemini20Flash | Self::Gemini25Pro => false,
        }
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "gpt-4o" => Ok(Self::Gpt4o),
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-4.1" => Ok(Self::Gpt4_1),
            "gpt-3.5-turbo" => Ok(Self::Gpt3_5Turbo),
            "o1" => Ok(Self::O1),
            "o3-mini" => Ok(Self::O3Mini),
            "o3" => Ok(Self::O3),
            "o4-mini" => Ok(Self::O4Mini),
            "claude-3-5-sonnet" => Ok(Self::Claude3_5Sonnet),
            "claude-3-7-sonnet" => Ok(Self::Claude3_7Sonnet),
            "claude-3.7-sonnet-thought" => Ok(Self::Claude3_7SonnetThinking),
            "gemini-2.0-flash-001" => Ok(Self::Gemini20Flash),
            "gemini-2.5-pro" => Ok(Self::Gemini25Pro),
            _ => Err(anyhow!("Invalid model id: {}", id)),
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::Gpt3_5Turbo => "gpt-3.5-turbo",
            Self::Gpt4 => "gpt-4",
            Self::Gpt4_1 => "gpt-4.1",
            Self::Gpt4o => "gpt-4o",
            Self::O3Mini => "o3-mini",
            Self::O1 => "o1",
            Self::O3 => "o3",
            Self::O4Mini => "o4-mini",
            Self::Claude3_5Sonnet => "claude-3-5-sonnet",
            Self::Claude3_7Sonnet => "claude-3-7-sonnet",
            Self::Claude3_7SonnetThinking => "claude-3.7-sonnet-thought",
            Self::Gemini20Flash => "gemini-2.0-flash-001",
            Self::Gemini25Pro => "gemini-2.5-pro",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Gpt3_5Turbo => "GPT-3.5",
            Self::Gpt4 => "GPT-4",
            Self::Gpt4_1 => "GPT-4.1",
            Self::Gpt4o => "GPT-4o",
            Self::O3Mini => "o3-mini",
            Self::O1 => "o1",
            Self::O3 => "o3",
            Self::O4Mini => "o4-mini",
            Self::Claude3_5Sonnet => "Claude 3.5 Sonnet",
            Self::Claude3_7Sonnet => "Claude 3.7 Sonnet",
            Self::Claude3_7SonnetThinking => "Claude 3.7 Sonnet Thinking",
            Self::Gemini20Flash => "Gemini 2.0 Flash",
            Self::Gemini25Pro => "Gemini 2.5 Pro",
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Gpt4o => 64_000,
            Self::Gpt4 => 32_768,
            Self::Gpt4_1 => 128_000,
            Self::Gpt3_5Turbo => 12_288,
            Self::O3Mini => 64_000,
            Self::O1 => 20_000,
            Self::O3 => 128_000,
            Self::O4Mini => 128_000,
            Self::Claude3_5Sonnet => 200_000,
            Self::Claude3_7Sonnet => 90_000,
            Self::Claude3_7SonnetThinking => 90_000,
            Self::Gemini20Flash => 128_000,
            Self::Gemini25Pro => 128_000,
        }
    }

    pub fn supports_vision(&self) -> bool {
        match self {
            Self::Gpt4o
            | Self::Gpt4_1
            | Self::O3
            | Self::O4Mini
            | Self::Gemini20Flash
            | Self::Gemini25Pro
            | Self::Claude3_5Sonnet
            | Self::Claude3_7Sonnet
            | Self::Claude3_7SonnetThinking => true,
            _ => false,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

#[derive(Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
    Function { function: Function },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Any,
    Tool { name: String },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    Assistant {
        content: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
    },
    User {
        content: Vec<ChatMessageContent>,
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
    pub delta: Option<ResponseDelta>,
    pub message: Option<ResponseDelta>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseDelta {
    pub content: Option<String>,
    pub role: Option<Role>,
    #[serde(default)]
    pub tool_calls: Vec<ToolCallChunk>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
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

struct GlobalCopilotChat(gpui::Entity<CopilotChat>);

impl Global for GlobalCopilotChat {}

pub struct CopilotChat {
    oauth_token: Option<String>,
    api_token: Option<ApiToken>,
    client: Arc<dyn HttpClient>,
}

pub fn init(fs: Arc<dyn Fs>, client: Arc<dyn HttpClient>, cx: &mut App) {
    let copilot_chat = cx.new(|cx| CopilotChat::new(fs, client, cx));
    cx.set_global(GlobalCopilotChat(copilot_chat));
}

pub fn copilot_chat_config_dir() -> &'static PathBuf {
    static COPILOT_CHAT_CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();

    COPILOT_CHAT_CONFIG_DIR.get_or_init(|| {
        if cfg!(target_os = "windows") {
            home_dir().join("AppData").join("Local")
        } else {
            home_dir().join(".config")
        }
        .join("github-copilot")
    })
}

fn copilot_chat_config_paths() -> [PathBuf; 2] {
    let base_dir = copilot_chat_config_dir();
    [base_dir.join("hosts.json"), base_dir.join("apps.json")]
}

impl CopilotChat {
    pub fn global(cx: &App) -> Option<gpui::Entity<Self>> {
        cx.try_global::<GlobalCopilotChat>()
            .map(|model| model.0.clone())
    }

    pub fn new(fs: Arc<dyn Fs>, client: Arc<dyn HttpClient>, cx: &App) -> Self {
        let config_paths: HashSet<PathBuf> = copilot_chat_config_paths().into_iter().collect();
        let dir_path = copilot_chat_config_dir();

        cx.spawn(async move |cx| {
            let mut parent_watch_rx = watch_config_dir(
                cx.background_executor(),
                fs.clone(),
                dir_path.clone(),
                config_paths,
            );
            while let Some(contents) = parent_watch_rx.next().await {
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
        mut cx: AsyncApp,
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
                let token = request_api_token(&oauth_token, client.clone()).await?;
                this.update(&mut cx, |this, cx| {
                    this.api_token = Some(token.clone());
                    cx.notify();
                })?;
                token
            }
        };

        stream_completion(client.clone(), token.api_key, request).await
    }
}

async fn request_api_token(oauth_token: &str, client: Arc<dyn HttpClient>) -> Result<ApiToken> {
    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(COPILOT_CHAT_AUTH_URL)
        .header("Authorization", format!("token {}", oauth_token))
        .header("Accept", "application/json");

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
            v.as_object().and_then(|obj| {
                obj.iter().find_map(|(key, value)| {
                    if key.starts_with("github.com") {
                        value["oauth_token"].as_str().map(|v| v.to_string())
                    } else {
                        None
                    }
                })
            })
        })
        .ok()
        .flatten()
}

async fn stream_completion(
    client: Arc<dyn HttpClient>,
    api_key: String,
    request: Request,
) -> Result<BoxStream<'static, Result<ResponseEvent>>> {
    let request_builder = HttpRequest::builder()
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
        .header("Copilot-Integration-Id", "vscode-chat")
        .header("Copilot-Vision-Request", "true");

    let is_streaming = request.stream;

    let json = serde_json::to_string(&request)?;
    let request = request_builder.body(AsyncBody::from(json))?;
    let mut response = client.send(request).await?;

    if !response.status().is_success() {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;
        let body_str = std::str::from_utf8(&body)?;
        return Err(anyhow!(
            "Failed to connect to API: {} {}",
            response.status(),
            body_str
        ));
    }

    if is_streaming {
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
                                if response.choices.is_empty() {
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
        let response: ResponseEvent = serde_json::from_str(body_str)?;

        Ok(futures::stream::once(async move { Ok(response) }).boxed())
    }
}
