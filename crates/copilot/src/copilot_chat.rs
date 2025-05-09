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
use itertools::Itertools;
use paths::home_dir;
use serde::{Deserialize, Serialize};
use settings::watch_config_dir;

pub const COPILOT_CHAT_COMPLETION_URL: &str = "https://api.githubcopilot.com/chat/completions";
pub const COPILOT_CHAT_AUTH_URL: &str = "https://api.github.com/copilot_internal/v2/token";
pub const COPILOT_CHAT_MODELS_URL: &str = "https://api.githubcopilot.com/models";

// Copilot's base model; defined by Microsoft in premium requests table
// This will be moved to the front of the Copilot model list, and will be used for
// 'fast' requests (e.g. title generation)
// https://docs.github.com/en/copilot/managing-copilot/monitoring-usage-and-entitlements/about-premium-requests
const DEFAULT_MODEL_ID: &str = "gpt-4.1";

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[serde_with::serde_as]
#[derive(Deserialize)]
struct ModelSchema {
    #[serde_as(as = "serde_with::VecSkipError<_>")]
    data: Vec<Model>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Model {
    capabilities: ModelCapabilities,
    id: String,
    name: String,
    policy: Option<ModelPolicy>,
    vendor: ModelVendor,
    model_picker_enabled: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ModelCapabilities {
    family: String,
    #[serde(default)]
    limits: ModelLimits,
    supports: ModelSupportedFeatures,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ModelLimits {
    #[serde(default)]
    max_context_window_tokens: usize,
    #[serde(default)]
    max_output_tokens: usize,
    #[serde(default)]
    max_prompt_tokens: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ModelPolicy {
    state: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ModelSupportedFeatures {
    #[serde(default)]
    streaming: bool,
    #[serde(default)]
    tool_calls: bool,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ModelVendor {
    // Azure OpenAI should have no functional difference from OpenAI in Copilot Chat
    #[serde(alias = "Azure OpenAI")]
    OpenAI,
    Google,
    Anthropic,
}

impl Model {
    pub fn uses_streaming(&self) -> bool {
        self.capabilities.supports.streaming
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn display_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn max_token_count(&self) -> usize {
        self.capabilities.limits.max_prompt_tokens
    }

    pub fn supports_tools(&self) -> bool {
        self.capabilities.supports.tool_calls
    }

    pub fn vendor(&self) -> ModelVendor {
        self.vendor
    }
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub intent: bool,
    pub n: usize,
    pub stream: bool,
    pub temperature: f32,
    pub model: String,
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
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Any,
    None,
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
    models: Option<Vec<Model>>,
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

        let client_async = client.clone();
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
                            this.oauth_token = oauth_token.clone();
                            cx.notify();
                        });
                    }
                })?;

                if let Some(ref oauth_token) = oauth_token {
                    let api_token = request_api_token(oauth_token, client_async.clone()).await?;
                    cx.update(|cx| {
                        if let Some(this) = Self::global(cx).as_ref() {
                            this.update(cx, |this, cx| {
                                this.api_token = Some(api_token.clone());
                                cx.notify();
                            });
                        }
                    })?;
                    let models = get_models(api_token.api_key, client_async.clone()).await?;
                    cx.update(|cx| {
                        if let Some(this) = Self::global(cx).as_ref() {
                            this.update(cx, |this, cx| {
                                this.models = Some(models);
                                cx.notify();
                            });
                        }
                    })?;
                }
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        Self {
            oauth_token: None,
            api_token: None,
            models: None,
            client,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.oauth_token.is_some()
    }

    pub fn models(&self) -> Option<&[Model]> {
        self.models.as_deref()
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

async fn get_models(api_token: String, client: Arc<dyn HttpClient>) -> Result<Vec<Model>> {
    let all_models = request_models(api_token, client).await?;

    let mut models: Vec<Model> = all_models
        .into_iter()
        .filter(|model| model.model_picker_enabled)
        .filter(|model| {
            // Ensure user has access to the model; Policy is present only for models that must be
            // enabled in the GitHub dashboard
            model
                .policy
                .as_ref()
                .is_none_or(|policy| policy.state == "enabled")
        })
        // The first model from the API response, in any given family, appear to be the non-tagged
        // models, which are likely the best choice (e.g. gpt-4o rather than gpt-4o-2024-11-20)
        .dedup_by(|a, b| a.capabilities.family == b.capabilities.family)
        .collect();

    if let Some(default_model_position) =
        models.iter().position(|model| model.id == DEFAULT_MODEL_ID)
    {
        let default_model = models.remove(default_model_position);
        models.insert(0, default_model);
    }

    Ok(models)
}

async fn request_models(api_token: String, client: Arc<dyn HttpClient>) -> Result<Vec<Model>> {
    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(COPILOT_CHAT_MODELS_URL)
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .header("Copilot-Integration-Id", "vscode-chat");

    let request = request_builder.body(AsyncBody::empty())?;

    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;

        let models = serde_json::from_str::<ModelSchema>(body_str)?.data;

        Ok(models)
    } else {
        Err(anyhow!("Failed to request models: {}", response.status()))
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
        .header("Copilot-Integration-Id", "vscode-chat");

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
