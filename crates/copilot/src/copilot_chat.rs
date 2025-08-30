use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;

use anyhow::Context as _;
use anyhow::{Result, anyhow};
use chrono::DateTime;
use collections::HashSet;
use fs::Fs;
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use gpui::WeakEntity;
use gpui::{App, AsyncApp, Global, prelude::*};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use itertools::Itertools;
use paths::home_dir;
use serde::{Deserialize, Serialize};
use settings::watch_config_dir;

pub const COPILOT_OAUTH_ENV_VAR: &str = "GH_COPILOT_TOKEN";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct CopilotChatConfiguration {
    pub enterprise_uri: Option<String>,
}

impl CopilotChatConfiguration {
    pub fn token_url(&self) -> String {
        if let Some(enterprise_uri) = &self.enterprise_uri {
            let domain = Self::parse_domain(enterprise_uri);
            format!("https://api.{}/copilot_internal/v2/token", domain)
        } else {
            "https://api.github.com/copilot_internal/v2/token".to_string()
        }
    }

    pub fn oauth_domain(&self) -> String {
        if let Some(enterprise_uri) = &self.enterprise_uri {
            Self::parse_domain(enterprise_uri)
        } else {
            "github.com".to_string()
        }
    }

    pub fn api_url_from_endpoint(&self, endpoint: &str) -> String {
        format!("{}/chat/completions", endpoint)
    }

    pub fn models_url_from_endpoint(&self, endpoint: &str) -> String {
        format!("{}/models", endpoint)
    }

    fn parse_domain(enterprise_uri: &str) -> String {
        let uri = enterprise_uri.trim_end_matches('/');

        if let Some(domain) = uri.strip_prefix("https://") {
            domain.split('/').next().unwrap_or(domain).to_string()
        } else if let Some(domain) = uri.strip_prefix("http://") {
            domain.split('/').next().unwrap_or(domain).to_string()
        } else {
            uri.split('/').next().unwrap_or(uri).to_string()
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Deserialize)]
struct ModelSchema {
    #[serde(deserialize_with = "deserialize_models_skip_errors")]
    data: Vec<Model>,
}

fn deserialize_models_skip_errors<'de, D>(deserializer: D) -> Result<Vec<Model>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw_values = Vec::<serde_json::Value>::deserialize(deserializer)?;
    let models = raw_values
        .into_iter()
        .filter_map(|value| match serde_json::from_value::<Model>(value) {
            Ok(model) => Some(model),
            Err(err) => {
                log::warn!("GitHub Copilot Chat model failed to deserialize: {:?}", err);
                None
            }
        })
        .collect();

    Ok(models)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Model {
    billing: ModelBilling,
    capabilities: ModelCapabilities,
    id: String,
    name: String,
    policy: Option<ModelPolicy>,
    vendor: ModelVendor,
    is_chat_default: bool,
    // The model with this value true is selected by VSCode copilot if a premium request limit is
    // reached. Zed does not currently implement this behaviour
    is_chat_fallback: bool,
    model_picker_enabled: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct ModelBilling {
    is_premium: bool,
    multiplier: f64,
    // List of plans a model is restricted to
    // Field is not present if a model is available for all plans
    #[serde(default)]
    restricted_to: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ModelCapabilities {
    family: String,
    #[serde(default)]
    limits: ModelLimits,
    supports: ModelSupportedFeatures,
    #[serde(rename = "type")]
    model_type: String,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ModelLimits {
    #[serde(default)]
    max_context_window_tokens: usize,
    #[serde(default)]
    max_output_tokens: usize,
    #[serde(default)]
    max_prompt_tokens: u64,
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
    #[serde(default)]
    parallel_tool_calls: bool,
    #[serde(default)]
    vision: bool,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ModelVendor {
    // Azure OpenAI should have no functional difference from OpenAI in Copilot Chat
    #[serde(alias = "Azure OpenAI")]
    OpenAI,
    Google,
    Anthropic,
    #[serde(rename = "xAI")]
    XAI,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum ChatMessagePart {
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
    pub fn uses_streaming(&self) -> bool {
        self.capabilities.supports.streaming
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn display_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn max_token_count(&self) -> u64 {
        self.capabilities.limits.max_prompt_tokens
    }

    pub fn supports_tools(&self) -> bool {
        self.capabilities.supports.tool_calls
    }

    pub fn vendor(&self) -> ModelVendor {
        self.vendor
    }

    pub fn supports_vision(&self) -> bool {
        self.capabilities.supports.vision
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        self.capabilities.supports.parallel_tool_calls
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    Assistant {
        content: ChatMessageContent,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
    },
    User {
        content: ChatMessageContent,
    },
    System {
        content: String,
    },
    Tool {
        content: ChatMessageContent,
        tool_call_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMessageContent {
    Plain(String),
    Multipart(Vec<ChatMessagePart>),
}

impl ChatMessageContent {
    pub fn empty() -> Self {
        ChatMessageContent::Multipart(vec![])
    }
}

impl From<Vec<ChatMessagePart>> for ChatMessageContent {
    fn from(mut parts: Vec<ChatMessagePart>) -> Self {
        if let [ChatMessagePart::Text { text }] = parts.as_mut_slice() {
            ChatMessageContent::Plain(std::mem::take(text))
        } else {
            ChatMessageContent::Multipart(parts)
        }
    }
}

impl From<String> for ChatMessageContent {
    fn from(text: String) -> Self {
        ChatMessageContent::Plain(text)
    }
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
    pub id: String,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub completion_tokens: u64,
    pub prompt_tokens: u64,
    pub total_tokens: u64,
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
    endpoints: ApiTokenResponseEndpoints,
}

#[derive(Deserialize)]
struct ApiTokenResponseEndpoints {
    api: String,
}

#[derive(Clone)]
struct ApiToken {
    api_key: String,
    expires_at: DateTime<chrono::Utc>,
    api_endpoint: String,
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
        let expires_at =
            DateTime::from_timestamp(response.expires_at, 0).context("invalid expires_at")?;

        Ok(Self {
            api_key: response.token,
            expires_at,
            api_endpoint: response.endpoints.api,
        })
    }
}

struct GlobalCopilotChat(gpui::Entity<CopilotChat>);

impl Global for GlobalCopilotChat {}

pub struct CopilotChat {
    oauth_token: Option<String>,
    api_token: Option<ApiToken>,
    configuration: CopilotChatConfiguration,
    models: Option<Vec<Model>>,
    client: Arc<dyn HttpClient>,
}

pub fn init(
    fs: Arc<dyn Fs>,
    client: Arc<dyn HttpClient>,
    configuration: CopilotChatConfiguration,
    cx: &mut App,
) {
    let copilot_chat = cx.new(|cx| CopilotChat::new(fs, client, configuration, cx));
    cx.set_global(GlobalCopilotChat(copilot_chat));
}

pub fn copilot_chat_config_dir() -> &'static PathBuf {
    static COPILOT_CHAT_CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();

    COPILOT_CHAT_CONFIG_DIR.get_or_init(|| {
        let config_dir = if cfg!(target_os = "windows") {
            dirs::data_local_dir().expect("failed to determine LocalAppData directory")
        } else {
            std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir().join(".config"))
        };

        config_dir.join("github-copilot")
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

    fn new(
        fs: Arc<dyn Fs>,
        client: Arc<dyn HttpClient>,
        configuration: CopilotChatConfiguration,
        cx: &mut Context<Self>,
    ) -> Self {
        let config_paths: HashSet<PathBuf> = copilot_chat_config_paths().into_iter().collect();
        let dir_path = copilot_chat_config_dir();

        cx.spawn(async move |this, cx| {
            let mut parent_watch_rx = watch_config_dir(
                cx.background_executor(),
                fs.clone(),
                dir_path.clone(),
                config_paths,
            );
            while let Some(contents) = parent_watch_rx.next().await {
                let oauth_domain =
                    this.read_with(cx, |this, _| this.configuration.oauth_domain())?;
                let oauth_token = extract_oauth_token(contents, &oauth_domain);

                this.update(cx, |this, cx| {
                    this.oauth_token = oauth_token.clone();
                    cx.notify();
                })?;

                if oauth_token.is_some() {
                    Self::update_models(&this, cx).await?;
                }
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let this = Self {
            oauth_token: std::env::var(COPILOT_OAUTH_ENV_VAR).ok(),
            api_token: None,
            models: None,
            configuration,
            client,
        };

        if this.oauth_token.is_some() {
            cx.spawn(async move |this, cx| Self::update_models(&this, cx).await)
                .detach_and_log_err(cx);
        }

        this
    }

    async fn update_models(this: &WeakEntity<Self>, cx: &mut AsyncApp) -> Result<()> {
        let (oauth_token, client, configuration) = this.read_with(cx, |this, _| {
            (
                this.oauth_token.clone(),
                this.client.clone(),
                this.configuration.clone(),
            )
        })?;

        let oauth_token = oauth_token
            .ok_or_else(|| anyhow!("OAuth token is missing while updating Copilot Chat models"))?;

        let token_url = configuration.token_url();
        let api_token = request_api_token(&oauth_token, token_url.into(), client.clone()).await?;

        let models_url = configuration.models_url_from_endpoint(&api_token.api_endpoint);
        let models =
            get_models(models_url.into(), api_token.api_key.clone(), client.clone()).await?;

        this.update(cx, |this, cx| {
            this.api_token = Some(api_token);
            this.models = Some(models);
            cx.notify();
        })?;
        anyhow::Ok(())
    }

    pub fn is_authenticated(&self) -> bool {
        self.oauth_token.is_some()
    }

    pub fn models(&self) -> Option<&[Model]> {
        self.models.as_deref()
    }

    pub async fn stream_completion(
        request: Request,
        is_user_initiated: bool,
        mut cx: AsyncApp,
    ) -> Result<BoxStream<'static, Result<ResponseEvent>>> {
        let this = cx
            .update(|cx| Self::global(cx))
            .ok()
            .flatten()
            .context("Copilot chat is not enabled")?;

        let (oauth_token, api_token, client, configuration) = this.read_with(&cx, |this, _| {
            (
                this.oauth_token.clone(),
                this.api_token.clone(),
                this.client.clone(),
                this.configuration.clone(),
            )
        })?;

        let oauth_token = oauth_token.context("No OAuth token available")?;

        let token = match api_token {
            Some(api_token) if api_token.remaining_seconds() > 5 * 60 => api_token.clone(),
            _ => {
                let token_url = configuration.token_url();
                let token =
                    request_api_token(&oauth_token, token_url.into(), client.clone()).await?;
                this.update(&mut cx, |this, cx| {
                    this.api_token = Some(token.clone());
                    cx.notify();
                })?;
                token
            }
        };

        let api_url = configuration.api_url_from_endpoint(&token.api_endpoint);
        stream_completion(
            client.clone(),
            token.api_key,
            api_url.into(),
            request,
            is_user_initiated,
        )
        .await
    }

    pub fn set_configuration(
        &mut self,
        configuration: CopilotChatConfiguration,
        cx: &mut Context<Self>,
    ) {
        let same_configuration = self.configuration == configuration;
        self.configuration = configuration;
        if !same_configuration {
            self.api_token = None;
            cx.spawn(async move |this, cx| {
                Self::update_models(&this, cx).await?;
                Ok::<_, anyhow::Error>(())
            })
            .detach();
        }
    }
}

async fn get_models(
    models_url: Arc<str>,
    api_token: String,
    client: Arc<dyn HttpClient>,
) -> Result<Vec<Model>> {
    let all_models = request_models(models_url, api_token, client).await?;

    let mut models: Vec<Model> = all_models
        .into_iter()
        .filter(|model| {
            model.model_picker_enabled
                && model.capabilities.model_type.as_str() == "chat"
                && model
                    .policy
                    .as_ref()
                    .is_none_or(|policy| policy.state == "enabled")
        })
        .dedup_by(|a, b| a.capabilities.family == b.capabilities.family)
        .collect();

    if let Some(default_model_position) = models.iter().position(|model| model.is_chat_default) {
        let default_model = models.remove(default_model_position);
        models.insert(0, default_model);
    }

    Ok(models)
}

async fn request_models(
    models_url: Arc<str>,
    api_token: String,
    client: Arc<dyn HttpClient>,
) -> Result<Vec<Model>> {
    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(models_url.as_ref())
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .header("Copilot-Integration-Id", "vscode-chat")
        .header("Editor-Version", "vscode/1.103.2")
        .header("x-github-api-version", "2025-05-01");

    let request = request_builder.body(AsyncBody::empty())?;

    let mut response = client.send(request).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to request models: {}",
        response.status()
    );
    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;

    let body_str = std::str::from_utf8(&body)?;

    let models = serde_json::from_str::<ModelSchema>(body_str)?.data;

    Ok(models)
}

async fn request_api_token(
    oauth_token: &str,
    auth_url: Arc<str>,
    client: Arc<dyn HttpClient>,
) -> Result<ApiToken> {
    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(auth_url.as_ref())
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
        anyhow::bail!("Failed to request API token: {body_str}");
    }
}

fn extract_oauth_token(contents: String, domain: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(&contents)
        .map(|v| {
            v.as_object().and_then(|obj| {
                obj.iter().find_map(|(key, value)| {
                    if key.starts_with(domain) {
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
    completion_url: Arc<str>,
    request: Request,
    is_user_initiated: bool,
) -> Result<BoxStream<'static, Result<ResponseEvent>>> {
    let is_vision_request = request.messages.iter().any(|message| match message {
      ChatMessage::User { content }
      | ChatMessage::Assistant { content, .. }
      | ChatMessage::Tool { content, .. } => {
          matches!(content, ChatMessageContent::Multipart(parts) if parts.iter().any(|part| matches!(part, ChatMessagePart::Image { .. })))
      }
      _ => false,
  });

    let request_initiator = if is_user_initiated { "user" } else { "agent" };

    let mut request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(completion_url.as_ref())
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
        .header("X-Initiator", request_initiator);

    if is_vision_request {
        request_builder =
            request_builder.header("Copilot-Vision-Request", is_vision_request.to_string());
    }

    let is_streaming = request.stream;

    let json = serde_json::to_string(&request)?;
    let request = request_builder.body(AsyncBody::from(json))?;
    let mut response = client.send(request).await?;

    if !response.status().is_success() {
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;
        let body_str = std::str::from_utf8(&body)?;
        anyhow::bail!(
            "Failed to connect to API: {} {}",
            response.status(),
            body_str
        );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resilient_model_schema_deserialize() {
        let json = r#"{
              "data": [
                {
                  "billing": {
                    "is_premium": false,
                    "multiplier": 0
                  },
                  "capabilities": {
                    "family": "gpt-4",
                    "limits": {
                      "max_context_window_tokens": 32768,
                      "max_output_tokens": 4096,
                      "max_prompt_tokens": 32768
                    },
                    "object": "model_capabilities",
                    "supports": { "streaming": true, "tool_calls": true },
                    "tokenizer": "cl100k_base",
                    "type": "chat"
                  },
                  "id": "gpt-4",
                  "is_chat_default": false,
                  "is_chat_fallback": false,
                  "model_picker_enabled": false,
                  "name": "GPT 4",
                  "object": "model",
                  "preview": false,
                  "vendor": "Azure OpenAI",
                  "version": "gpt-4-0613"
                },
                {
                    "some-unknown-field": 123
                },
                {
                  "billing": {
                    "is_premium": true,
                    "multiplier": 1,
                    "restricted_to": [
                      "pro",
                      "pro_plus",
                      "business",
                      "enterprise"
                    ]
                  },
                  "capabilities": {
                    "family": "claude-3.7-sonnet",
                    "limits": {
                      "max_context_window_tokens": 200000,
                      "max_output_tokens": 16384,
                      "max_prompt_tokens": 90000,
                      "vision": {
                        "max_prompt_image_size": 3145728,
                        "max_prompt_images": 1,
                        "supported_media_types": ["image/jpeg", "image/png", "image/webp"]
                      }
                    },
                    "object": "model_capabilities",
                    "supports": {
                      "parallel_tool_calls": true,
                      "streaming": true,
                      "tool_calls": true,
                      "vision": true
                    },
                    "tokenizer": "o200k_base",
                    "type": "chat"
                  },
                  "id": "claude-3.7-sonnet",
                  "is_chat_default": false,
                  "is_chat_fallback": false,
                  "model_picker_enabled": true,
                  "name": "Claude 3.7 Sonnet",
                  "object": "model",
                  "policy": {
                    "state": "enabled",
                    "terms": "Enable access to the latest Claude 3.7 Sonnet model from Anthropic. [Learn more about how GitHub Copilot serves Claude 3.7 Sonnet](https://docs.github.com/copilot/using-github-copilot/using-claude-sonnet-in-github-copilot)."
                  },
                  "preview": false,
                  "vendor": "Anthropic",
                  "version": "claude-3.7-sonnet"
                }
              ],
              "object": "list"
            }"#;

        let schema: ModelSchema = serde_json::from_str(json).unwrap();

        assert_eq!(schema.data.len(), 2);
        assert_eq!(schema.data[0].id, "gpt-4");
        assert_eq!(schema.data[1].id, "claude-3.7-sonnet");
    }
}
