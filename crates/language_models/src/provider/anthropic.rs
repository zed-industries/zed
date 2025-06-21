use crate::AllLanguageModelSettings;
use crate::ui::InstructionListItem;
use anthropic::{
    AnthropicAuth, AnthropicError, AnthropicModelMode, ContentDelta, Event, ResponseContent,
    ToolResultContent, ToolResultPart, Usage,
};
use anyhow::{Context as _, Result, anyhow};
use collections::{BTreeMap, HashMap};
use credentials_provider::CredentialsProvider;
use editor::{Editor, EditorElement, EditorStyle};
use futures::Stream;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{
    Animation, AnimationExt, AnyView, App, AsyncApp, Context, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, FontStyle, InteractiveElement, IntoElement, MouseDownEvent,
    ParentElement, Styled, Subscription, Task, TextStyle, Transformation, WhiteSpace, Window, div,
    percentage,
};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCacheConfiguration,
    LanguageModelCompletionError, LanguageModelId, LanguageModelKnownError, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolResultContent, MessageContent, RateLimiter, Role,
};
use language_model::{LanguageModelCompletionEvent, LanguageModelToolUse, StopReason};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use strum::IntoEnumIterator;
use theme::ThemeSettings;
use ui::{Button, Icon, IconName, Label, List, Tooltip, Vector, VectorName, prelude::*};

use util::ResultExt;
use workspace::{ModalView, Workspace};

const PROVIDER_ID: &str = language_model::ANTHROPIC_PROVIDER_ID;
const PROVIDER_NAME: &str = "Anthropic";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AnthropicSettings {
    pub api_url: String,
    /// Extend Zed's list of Anthropic models.
    pub available_models: Vec<AvailableModel>,
    pub needs_setting_migration: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    /// The model's name in the Anthropic API. e.g. claude-3-5-sonnet-latest, claude-3-opus-20240229, etc
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the assistant panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// A model `name` to substitute when calling tools, in case the primary model doesn't support tool calling.
    pub tool_override: Option<String>,
    /// Configuration of Anthropic's caching API.
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
    pub max_output_tokens: Option<u64>,
    pub default_temperature: Option<f32>,
    #[serde(default)]
    pub extra_beta_headers: Vec<String>,
    /// The model's mode (e.g. thinking)
    pub mode: Option<ModelMode>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModelMode {
    #[default]
    Default,
    Thinking {
        /// The maximum number of tokens to use for reasoning. Must be lower than the model's `max_output_tokens`.
        budget_tokens: Option<u32>,
    },
}

impl From<ModelMode> for AnthropicModelMode {
    fn from(value: ModelMode) -> Self {
        match value {
            ModelMode::Default => AnthropicModelMode::Default,
            ModelMode::Thinking { budget_tokens } => AnthropicModelMode::Thinking { budget_tokens },
        }
    }
}

impl From<AnthropicModelMode> for ModelMode {
    fn from(value: AnthropicModelMode) -> Self {
        match value {
            AnthropicModelMode::Default => ModelMode::Default,
            AnthropicModelMode::Thinking { budget_tokens } => ModelMode::Thinking { budget_tokens },
        }
    }
}

pub struct AnthropicLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

const ANTHROPIC_API_KEY_VAR: &str = "ANTHROPIC_API_KEY";

pub struct State {
    api_key: Option<String>,
    api_key_from_env: bool,
    oauth: Option<AnthropicAuth>,
    _subscription: Subscription,
}

impl State {
    fn reset_auth(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .anthropic
            .api_url
            .clone();
        cx.spawn(async move |this, cx| {
            credentials_provider
                .delete_credentials(&api_url, &cx)
                .await
                .ok();
            this.update(cx, |this, cx| {
                this.api_key = None;
                this.api_key_from_env = false;
                this.oauth = None;
                cx.notify();
            })
        })
    }

    fn set_api_key(&mut self, api_key: String, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .anthropic
            .api_url
            .clone();
        cx.spawn(async move |this, cx| {
            credentials_provider
                .write_credentials(&api_url, "Bearer", api_key.as_bytes(), &cx)
                .await
                .ok();

            this.update(cx, |this, cx| {
                this.api_key = Some(api_key);
                this.oauth = None;
                cx.notify();
            })
        })
    }

    fn set_oauth(&mut self, oauth: AnthropicAuth, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = AllLanguageModelSettings::get_global(cx)
            .anthropic
            .api_url
            .clone();
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let oauth_json = format!(
            r#"{{"refresh_token":"{}","access_token":"{}","expires":{}}}"#,
            oauth.refresh_token, oauth.access_token, oauth.expires
        );

        cx.spawn(async move |this, cx| {
            if let Err(err) = credentials_provider
                .write_credentials(&api_url, "OAuth", oauth_json.as_bytes(), &cx)
                .await
            {
                log::error!("Failed to store OAuth credentials: {}", err);
                return Err(anyhow::anyhow!("Failed to store credentials"));
            }

            this.update(cx, |this, cx| {
                this.oauth = Some(oauth);
                this.api_key = None;
                this.api_key_from_env = false;
                cx.notify();
            })?;

            Ok(())
        })
    }

    fn is_authenticated(&self) -> bool {
        self.api_key.is_some() || self.oauth.is_some()
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let settings = AllLanguageModelSettings::get_global(cx).anthropic.clone();

        if self.is_authenticated() {
            if let Some(oauth) = &self.oauth {
                if oauth.is_expired() {
                    let oauth = oauth.clone();
                    let http_client = cx.http_client().clone();
                    return cx.spawn(async move |this, cx| {
                        let mut oauth_auth = oauth;
                        match oauth_auth.access_token(&*http_client).await {
                            Ok(Some(_)) => {
                                this.update(cx, |this, cx| {
                                    let settings = AllLanguageModelSettings::get_global(cx)
                                        .anthropic
                                        .clone();
                                    this.oauth = Some(oauth_auth.clone());

                                    let credentials_provider =
                                        <dyn CredentialsProvider>::global(cx);
                                    let oauth_json = format!(
                                        r#"{{"refresh_token":"{}","access_token":"{}","expires":{}}}"#,
                                        oauth_auth.refresh_token, oauth_auth.access_token, oauth_auth.expires
                                    );
                                    cx.spawn(async move |_, cx| {
                                        credentials_provider
                                            .write_credentials(
                                                &settings.api_url,
                                                "OAuth",
                                                oauth_json.as_bytes(),
                                                &cx,
                                            )
                                            .await
                                            .log_err();
                                        Ok::<(), anyhow::Error>(())
                                    })
                                    .detach();

                                    cx.notify();
                                })?;
                                Ok(())
                            }
                            Ok(None) | Err(_) => {
                                this.update(cx, |this, cx| {
                                    this.oauth = None;
                                    cx.notify();
                                })?;
                                Err(AuthenticateError::CredentialsNotFound)
                            }
                        }
                    });
                }
                return Task::ready(Ok(()));
            }

            if self.api_key.is_some() {
                return Task::ready(Ok(()));
            }
        }

        let credentials_provider = <dyn CredentialsProvider>::global(cx);

        cx.spawn(async move |this, cx| {
            if let Ok(api_key) = std::env::var(ANTHROPIC_API_KEY_VAR) {
                if !api_key.trim().is_empty() {
                    this.update(cx, |this, cx| {
                        this.api_key = Some(api_key);
                        this.api_key_from_env = true;
                        this.oauth = None;
                        cx.notify();
                    })?;
                    return Ok(());
                }
            }

            if let Ok(Some((username, credential_data))) = credentials_provider
                .read_credentials(&settings.api_url, &cx)
                .await
            {
                match username.as_str() {
                    "OAuth" => {
                        let oauth_str = String::from_utf8(credential_data)
                            .context("Invalid OAuth data format")?;

                        let oauth_value = serde_json::from_str::<serde_json::Value>(&oauth_str)
                            .context("Invalid OAuth JSON format")?;

                        let (refresh_token, access_token, expires) = (
                            oauth_value["refresh_token"]
                                .as_str()
                                .ok_or_else(|| anyhow::anyhow!("Missing refresh_token"))?,
                            oauth_value["access_token"]
                                .as_str()
                                .ok_or_else(|| anyhow::anyhow!("Missing access_token"))?,
                            oauth_value["expires"]
                                .as_u64()
                                .ok_or_else(|| anyhow::anyhow!("Missing expires"))?,
                        );

                        if !refresh_token.is_empty() && !access_token.is_empty() {
                            let oauth = AnthropicAuth::new(refresh_token, access_token, expires);
                            this.update(cx, |this, cx| {
                                this.oauth = Some(oauth);
                                this.api_key = None;
                                this.api_key_from_env = false;
                                cx.notify();
                            })?;
                            return Ok(());
                        }
                    }
                    "Bearer" => {
                        let api_key_str =
                            String::from_utf8(credential_data).context("Invalid API key format")?;
                        if !api_key_str.trim().is_empty() {
                            this.update(cx, |this, cx| {
                                this.api_key = Some(api_key_str);
                                this.api_key_from_env = false;
                                this.oauth = None;
                                cx.notify();
                            })?;
                            return Ok(());
                        }
                    }
                    _ => {}
                }
            }

            Err(AuthenticateError::CredentialsNotFound)
        })
    }
}

impl AnthropicLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            api_key: None,
            api_key_from_env: false,
            oauth: None,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: anthropic::Model) -> Arc<dyn LanguageModel> {
        Arc::new(AnthropicModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for AnthropicLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for AnthropicLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiAnthropic
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(anthropic::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(anthropic::Model::default_fast()))
    }

    fn recommended_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        [
            anthropic::Model::ClaudeSonnet4,
            anthropic::Model::ClaudeSonnet4Thinking,
        ]
        .into_iter()
        .map(|model| self.create_language_model(model))
        .collect()
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        // Add base models from anthropic::Model::iter()
        for model in anthropic::Model::iter() {
            if !matches!(model, anthropic::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in AllLanguageModelSettings::get_global(cx)
            .anthropic
            .available_models
            .iter()
        {
            models.insert(
                model.name.clone(),
                anthropic::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    tool_override: model.tool_override.clone(),
                    cache_configuration: model.cache_configuration.as_ref().map(|config| {
                        anthropic::AnthropicModelCacheConfiguration {
                            max_cache_anchors: config.max_cache_anchors,
                            should_speculate: config.should_speculate,
                            min_total_token: config.min_total_token,
                        }
                    }),
                    max_output_tokens: model.max_output_tokens,
                    default_temperature: model.default_temperature,
                    extra_beta_headers: model.extra_beta_headers.clone(),
                    mode: model.mode.clone().unwrap_or_default().into(),
                },
            );
        }

        models
            .into_values()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.reset_auth(cx))
    }
}

pub struct AnthropicModel {
    id: LanguageModelId,
    model: anthropic::Model,
    state: gpui::Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

pub fn count_anthropic_tokens(
    request: LanguageModelRequest,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
    cx.background_spawn(async move {
        let messages = request.messages;
        let mut tokens_from_images = 0;
        let mut string_messages = Vec::with_capacity(messages.len());

        for message in messages {
            use language_model::MessageContent;

            let mut string_contents = String::new();

            for content in message.content {
                match content {
                    MessageContent::Text(text) => {
                        string_contents.push_str(&text);
                    }
                    MessageContent::Thinking { .. } => {
                        // Thinking blocks are not included in the input token count.
                    }
                    MessageContent::RedactedThinking(_) => {
                        // Thinking blocks are not included in the input token count.
                    }
                    MessageContent::Image(image) => {
                        tokens_from_images += image.estimate_tokens();
                    }
                    MessageContent::ToolUse(_tool_use) => {
                        // TODO: Estimate token usage from tool uses.
                    }
                    MessageContent::ToolResult(tool_result) => match &tool_result.content {
                        LanguageModelToolResultContent::Text(text) => {
                            string_contents.push_str(text);
                        }
                        LanguageModelToolResultContent::Image(image) => {
                            tokens_from_images += image.estimate_tokens();
                        }
                    },
                }
            }

            if !string_contents.is_empty() {
                string_messages.push(tiktoken_rs::ChatCompletionRequestMessage {
                    role: match message.role {
                        Role::User => "user".into(),
                        Role::Assistant => "assistant".into(),
                        Role::System => "system".into(),
                    },
                    content: Some(string_contents),
                    name: None,
                    function_call: None,
                });
            }
        }

        // Tiktoken doesn't yet support these models, so we manually use the
        // same tokenizer as GPT-4.
        tiktoken_rs::num_tokens_from_messages("gpt-4", &string_messages)
            .map(|tokens| (tokens + tokens_from_images) as u64)
    })
    .boxed()
}

impl AnthropicModel {
    fn stream_completion(
        &self,
        request: anthropic::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<anthropic::Event, AnthropicError>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();

        let Ok((api_key, api_url, oauth)) = cx.read_entity(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).anthropic;
            (
                state.api_key.clone(),
                settings.api_url.clone(),
                state.oauth.clone(),
            )
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped").into())).boxed();
        };

        async move {
            let api_key = match (api_key, oauth) {
                (Some(key), _) => key,
                (None, Some(mut oauth)) => {
                    // TODO: When OAuth token is refreshed by access_token(), we should update the state
                    // to persist the new tokens. However, this requires an entity context (Context<State>)
                    // which isn't available in this async closure. The proper solution would be to:
                    // 1. Move the OAuth refresh logic to a method that has access to Context<State>
                    // 2. Use cx.spawn() to handle the async refresh with proper state updates
                    // 3. Call set_oauth() to persist the refreshed tokens
                    // For now, the tokens are refreshed in memory but not persisted to storage
                    oauth
                        .access_token(http_client.as_ref())
                        .await?
                        .ok_or_else(|| anyhow!("Failed to get OAuth access token"))?
                }
                (None, None) => return Err(anyhow!("No authentication configured").into()),
            };

            let request =
                anthropic::stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            request.await.map_err(|err| match err {
                AnthropicError::RateLimit(duration) => {
                    LanguageModelCompletionError::RateLimit(duration)
                }
                err @ (AnthropicError::ApiError(..) | AnthropicError::Other(..)) => {
                    LanguageModelCompletionError::Other(anthropic_err_to_anyhow(err))
                }
            })
        }
        .boxed()
    }
}

impl LanguageModel for AnthropicModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        true
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto
            | LanguageModelToolChoice::Any
            | LanguageModelToolChoice::None => true,
        }
    }

    fn telemetry_id(&self) -> String {
        format!("anthropic/{}", self.model.id())
    }

    fn api_key(&self, cx: &App) -> Option<String> {
        let state = self.state.read(cx);
        if let Some(api_key) = &state.api_key {
            Some(api_key.clone())
        } else if let Some(oauth) = &state.oauth {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .ok()?
                .as_millis() as u64;

            if oauth.expires > now {
                Some(oauth.access_token.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        Some(self.model.max_output_tokens())
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        count_anthropic_tokens(request, cx)
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let request = into_anthropic(
            request,
            self.model.request_id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
            self.model.mode(),
        );
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.await?;
            Ok(AnthropicEventMapper::new().map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        self.model
            .cache_configuration()
            .map(|config| LanguageModelCacheConfiguration {
                max_cache_anchors: config.max_cache_anchors,
                should_speculate: config.should_speculate,
                min_total_token: config.min_total_token,
            })
    }
}

pub fn into_anthropic(
    request: LanguageModelRequest,
    model: String,
    default_temperature: f32,
    max_output_tokens: u64,
    mode: AnthropicModelMode,
) -> anthropic::Request {
    let mut new_messages: Vec<anthropic::Message> = Vec::new();
    let mut system_message = String::new();

    for message in request.messages {
        if message.contents_empty() {
            continue;
        }

        match message.role {
            Role::User | Role::Assistant => {
                let mut anthropic_message_content: Vec<anthropic::RequestContent> = message
                    .content
                    .into_iter()
                    .filter_map(|content| match content {
                        MessageContent::Text(text) => {
                            if !text.is_empty() {
                                Some(anthropic::RequestContent::Text {
                                    text,
                                    cache_control: None,
                                })
                            } else {
                                None
                            }
                        }
                        MessageContent::Thinking {
                            text: thinking,
                            signature,
                        } => {
                            if !thinking.is_empty() {
                                Some(anthropic::RequestContent::Thinking {
                                    thinking,
                                    signature: signature.unwrap_or_default(),
                                    cache_control: None,
                                })
                            } else {
                                None
                            }
                        }
                        MessageContent::RedactedThinking(data) => {
                            if !data.is_empty() {
                                Some(anthropic::RequestContent::RedactedThinking {
                                    data: String::from_utf8(data).ok()?,
                                })
                            } else {
                                None
                            }
                        }
                        MessageContent::Image(image) => Some(anthropic::RequestContent::Image {
                            source: anthropic::ImageSource {
                                source_type: "base64".to_string(),
                                media_type: "image/png".to_string(),
                                data: image.source.to_string(),
                            },
                            cache_control: None,
                        }),
                        MessageContent::ToolUse(tool_use) => {
                            Some(anthropic::RequestContent::ToolUse {
                                id: tool_use.id.to_string(),
                                name: tool_use.name.to_string(),
                                input: tool_use.input,
                                cache_control: None,
                            })
                        }
                        MessageContent::ToolResult(tool_result) => {
                            Some(anthropic::RequestContent::ToolResult {
                                tool_use_id: tool_result.tool_use_id.to_string(),
                                is_error: tool_result.is_error,
                                content: match tool_result.content {
                                    LanguageModelToolResultContent::Text(text) => {
                                        ToolResultContent::Plain(text.to_string())
                                    }
                                    LanguageModelToolResultContent::Image(image) => {
                                        ToolResultContent::Multipart(vec![ToolResultPart::Image {
                                            source: anthropic::ImageSource {
                                                source_type: "base64".to_string(),
                                                media_type: "image/png".to_string(),
                                                data: image.source.to_string(),
                                            },
                                        }])
                                    }
                                },
                                cache_control: None,
                            })
                        }
                    })
                    .collect();
                let anthropic_role = match message.role {
                    Role::User => anthropic::Role::User,
                    Role::Assistant => anthropic::Role::Assistant,
                    Role::System => unreachable!("System role should never occur here"),
                };
                if let Some(last_message) = new_messages.last_mut() {
                    if last_message.role == anthropic_role {
                        last_message.content.extend(anthropic_message_content);
                        continue;
                    }
                }

                // Mark the last segment of the message as cached
                if message.cache {
                    let cache_control_value = Some(anthropic::CacheControl {
                        cache_type: anthropic::CacheControlType::Ephemeral,
                    });
                    for message_content in anthropic_message_content.iter_mut().rev() {
                        match message_content {
                            anthropic::RequestContent::RedactedThinking { .. } => {
                                // Caching is not possible, fallback to next message
                            }
                            anthropic::RequestContent::Text { cache_control, .. }
                            | anthropic::RequestContent::Thinking { cache_control, .. }
                            | anthropic::RequestContent::Image { cache_control, .. }
                            | anthropic::RequestContent::ToolUse { cache_control, .. }
                            | anthropic::RequestContent::ToolResult { cache_control, .. } => {
                                *cache_control = cache_control_value;
                                break;
                            }
                        }
                    }
                }

                new_messages.push(anthropic::Message {
                    role: anthropic_role,
                    content: anthropic_message_content,
                });
            }
            Role::System => {
                if !system_message.is_empty() {
                    system_message.push_str("\n\n");
                }
                system_message.push_str(&message.string_contents());
            }
        }
    }

    anthropic::Request {
        model,
        messages: new_messages,
        max_tokens: max_output_tokens,
        system: if system_message.is_empty() {
            None
        } else {
            Some(anthropic::StringOrContents::String(system_message))
        },
        thinking: if let AnthropicModelMode::Thinking { budget_tokens } = mode {
            Some(anthropic::Thinking::Enabled { budget_tokens })
        } else {
            None
        },
        tools: request
            .tools
            .into_iter()
            .map(|tool| anthropic::Tool {
                name: tool.name,
                description: tool.description,
                input_schema: tool.input_schema,
            })
            .collect(),
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => anthropic::ToolChoice::Auto,
            LanguageModelToolChoice::Any => anthropic::ToolChoice::Any,
            LanguageModelToolChoice::None => anthropic::ToolChoice::None,
        }),
        metadata: None,
        stop_sequences: Vec::new(),
        temperature: request.temperature.or(Some(default_temperature)),
        top_k: None,
        top_p: None,
    }
}

pub struct AnthropicEventMapper {
    tool_uses_by_index: HashMap<usize, RawToolUse>,
    usage: Usage,
    stop_reason: StopReason,
}

impl AnthropicEventMapper {
    pub fn new() -> Self {
        Self {
            tool_uses_by_index: HashMap::default(),
            usage: Usage::default(),
            stop_reason: StopReason::EndTurn,
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<Event, AnthropicError>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(LanguageModelCompletionError::Other(anyhow!(error)))],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: Event,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        match event {
            Event::ContentBlockStart {
                index,
                content_block,
            } => match content_block {
                ResponseContent::Text { text } => {
                    vec![Ok(LanguageModelCompletionEvent::Text(text))]
                }
                ResponseContent::Thinking { thinking } => {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: thinking,
                        signature: None,
                    })]
                }
                ResponseContent::RedactedThinking { .. } => {
                    // Redacted thinking is encrypted and not accessible to the user, see:
                    // https://docs.anthropic.com/en/docs/build-with-claude/extended-thinking#suggestions-for-handling-redacted-thinking-in-production
                    Vec::new()
                }
                ResponseContent::ToolUse { id, name, .. } => {
                    self.tool_uses_by_index.insert(
                        index,
                        RawToolUse {
                            id,
                            name,
                            input_json: String::new(),
                        },
                    );
                    Vec::new()
                }
            },
            Event::ContentBlockDelta { index, delta } => match delta {
                ContentDelta::TextDelta { text } => {
                    vec![Ok(LanguageModelCompletionEvent::Text(text))]
                }
                ContentDelta::ThinkingDelta { thinking } => {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: thinking,
                        signature: None,
                    })]
                }
                ContentDelta::SignatureDelta { signature } => {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: "".to_string(),
                        signature: Some(signature),
                    })]
                }
                ContentDelta::InputJsonDelta { partial_json } => {
                    if let Some(tool_use) = self.tool_uses_by_index.get_mut(&index) {
                        tool_use.input_json.push_str(&partial_json);

                        // Try to convert invalid (incomplete) JSON into
                        // valid JSON that serde can accept, e.g. by closing
                        // unclosed delimiters. This way, we can update the
                        // UI with whatever has been streamed back so far.
                        if let Ok(input) = serde_json::Value::from_str(
                            &partial_json_fixer::fix_json(&tool_use.input_json),
                        ) {
                            return vec![Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id: tool_use.id.clone().into(),
                                    name: tool_use.name.clone().into(),
                                    is_input_complete: false,
                                    raw_input: tool_use.input_json.clone(),
                                    input,
                                },
                            ))];
                        }
                    }
                    return vec![];
                }
            },
            Event::ContentBlockStop { index } => {
                if let Some(tool_use) = self.tool_uses_by_index.remove(&index) {
                    let input_json = tool_use.input_json.trim();
                    let input_value = if input_json.is_empty() {
                        Ok(serde_json::Value::Object(serde_json::Map::default()))
                    } else {
                        serde_json::Value::from_str(input_json)
                    };
                    let event_result = match input_value {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_use.id.into(),
                                name: tool_use.name.into(),
                                is_input_complete: true,
                                input,
                                raw_input: tool_use.input_json.clone(),
                            },
                        )),
                        Err(json_parse_err) => Err(LanguageModelCompletionError::BadInputJson {
                            id: tool_use.id.into(),
                            tool_name: tool_use.name.into(),
                            raw_input: input_json.into(),
                            json_parse_error: json_parse_err.to_string(),
                        }),
                    };

                    vec![event_result]
                } else {
                    Vec::new()
                }
            }
            Event::MessageStart { message } => {
                update_usage(&mut self.usage, &message.usage);
                vec![
                    Ok(LanguageModelCompletionEvent::UsageUpdate(convert_usage(
                        &self.usage,
                    ))),
                    Ok(LanguageModelCompletionEvent::StartMessage {
                        message_id: message.id,
                    }),
                ]
            }
            Event::MessageDelta { delta, usage } => {
                update_usage(&mut self.usage, &usage);
                if let Some(stop_reason) = delta.stop_reason.as_deref() {
                    self.stop_reason = match stop_reason {
                        "end_turn" => StopReason::EndTurn,
                        "max_tokens" => StopReason::MaxTokens,
                        "tool_use" => StopReason::ToolUse,
                        "refusal" => StopReason::Refusal,
                        _ => {
                            log::error!("Unexpected anthropic stop_reason: {stop_reason}");
                            StopReason::EndTurn
                        }
                    };
                }
                vec![Ok(LanguageModelCompletionEvent::UsageUpdate(
                    convert_usage(&self.usage),
                ))]
            }
            Event::MessageStop => {
                vec![Ok(LanguageModelCompletionEvent::Stop(self.stop_reason))]
            }
            Event::Error { error } => {
                vec![Err(LanguageModelCompletionError::Other(anyhow!(
                    AnthropicError::ApiError(error)
                )))]
            }
            _ => Vec::new(),
        }
    }
}

struct RawToolUse {
    id: String,
    name: String,
    input_json: String,
}

pub fn anthropic_err_to_anyhow(err: AnthropicError) -> anyhow::Error {
    if let AnthropicError::ApiError(api_err) = &err {
        if let Some(tokens) = api_err.match_window_exceeded() {
            return anyhow!(LanguageModelKnownError::ContextWindowLimitExceeded { tokens });
        }
    }

    anyhow!(err)
}

/// Updates usage data by preferring counts from `new`.
fn update_usage(usage: &mut Usage, new: &Usage) {
    if let Some(input_tokens) = new.input_tokens {
        usage.input_tokens = Some(input_tokens);
    }
    if let Some(output_tokens) = new.output_tokens {
        usage.output_tokens = Some(output_tokens);
    }
    if let Some(cache_creation_input_tokens) = new.cache_creation_input_tokens {
        usage.cache_creation_input_tokens = Some(cache_creation_input_tokens);
    }
    if let Some(cache_read_input_tokens) = new.cache_read_input_tokens {
        usage.cache_read_input_tokens = Some(cache_read_input_tokens);
    }
}

fn convert_usage(usage: &Usage) -> language_model::TokenUsage {
    language_model::TokenUsage {
        input_tokens: usage.input_tokens.unwrap_or(0),
        output_tokens: usage.output_tokens.unwrap_or(0),
        cache_creation_input_tokens: usage.cache_creation_input_tokens.unwrap_or(0),
        cache_read_input_tokens: usage.cache_read_input_tokens.unwrap_or(0),
    }
}

struct ConfigurationView {
    api_key_editor: Entity<Editor>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    const PLACEHOLDER_TEXT: &'static str = "sk-ant-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";

    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn({
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = state
                    .update(cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    // We don't log an error, because "not signed in" is also an error.
                    let _ = task.await;
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            api_key_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_TEXT, cx);
                editor
            }),
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(api_key, cx))?
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let state = self.state.clone();
        self.load_credentials_task = Some(cx.spawn(async move |this, cx| {
            if let Some(task) = state.update(cx, |state, cx| state.reset_auth(cx)).ok() {
                task.await.log_err();
            }
            this.update(cx, |this, cx| {
                this.load_credentials_task = None;
                cx.notify();
            })
            .ok();
        }));
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));
        cx.notify();
    }

    fn initiate_claude_sign_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let state = self.state.read(cx);
        if let Some(oauth) = &state.oauth {
            if !oauth.is_expired() {
                return;
            }
        }

        if let Some(workspace) = window.root::<Workspace>().flatten() {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    ClaudeSignIn::new(self.state.clone(), window, cx)
                });
            });
        }
    }

    fn render_api_key_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            white_space: WhiteSpace::Normal,
            ..Default::default()
        };
        EditorElement::new(
            &self.api_key_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        let state = self.state.read(cx);
        (state.api_key.is_none() && !state.api_key_from_env)
            && (state.oauth.is_none()
                || state
                    .oauth
                    .as_ref()
                    .map(|oauth| oauth.is_expired())
                    .unwrap_or(true))
    }

    fn get_auth_status_message(&self, cx: &mut Context<Self>) -> (String, bool) {
        let state = self.state.read(cx);

        if state.api_key_from_env {
            (
                format!("API key set in {ANTHROPIC_API_KEY_VAR} environment variable."),
                true,
            )
        } else if let Some(oauth) = &state.oauth {
            if oauth.is_expired() {
                ("OAuth token expired.".to_string(), false)
            } else {
                ("Signed in with Claude.".to_string(), true)
            }
        } else if state.api_key.is_some() {
            ("API key configured.".to_string(), true)
        } else {
            ("No authentication configured.".to_string(), false)
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_from_env;
        if self.load_credentials_task.is_some() {
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(Icon::new(IconName::Spinner).with_animation(
                    "spin",
                    Animation::new(Duration::from_secs(1)).repeat(),
                    |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                ))
                .child(Label::new("Loading credentials..."))
                .into_any()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .gap_6()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use Anthropic's AI models in Zed, you can either add an API key or sign in with your Claude account:"))
                .child(
                    v_flex()
                        .gap_4()
                        .child(
                            v_flex()
                                .gap_2()
                                .child(
                                    List::new()
                                        .child(
                                            InstructionListItem::new(
                                                "Get your API key from",
                                                Some("console.anthropic.com/settings/keys"),
                                                Some("https://console.anthropic.com/settings/keys")
                                            )
                                        )
                                        .child(
                                            InstructionListItem::text_only("Paste your API key below and press Enter to start using Claude")
                                        )
                                )
                                .child(
                                    h_flex()
                                        .w_full()
                                        .my_2()
                                        .px_2()
                                        .py_1()
                                        .bg(cx.theme().colors().editor_background)
                                        .border_1()
                                        .border_color(cx.theme().colors().border)
                                        .rounded_sm()
                                        .child(self.render_api_key_editor(cx)),
                                )
                                .child(
                                    Label::new(
                                        format!("Or set the {ANTHROPIC_API_KEY_VAR} environment variable and restart Zed"),
                                    )
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                                )
                        )
                        .child(
                            v_flex()
                                .gap_3()
                                .mt_4()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .flex_1()
                                                .h_px()
                                                .bg(cx.theme().colors().border_variant)
                                        )
                                        .child(
                                            Label::new("or")
                                                .color(Color::Muted)
                                                .size(LabelSize::Small)
                                        )
                                        .child(
                                            div()
                                                .flex_1()
                                                .h_px()
                                                .bg(cx.theme().colors().border_variant)
                                        )
                                )
                                .child(
                                    Button::new("sign-in-claude", "Sign in with Claude")
                                        .icon(Some(IconName::AiClaude))
                                        .icon_position(IconPosition::Start)
                                        .style(ButtonStyle::Subtle)
                                        .full_width()
                                        .on_click(cx.listener(|this, _, window, cx| this.initiate_claude_sign_in(window, cx)))
                                )
                        )
                )
                .into_any()
        } else {
            h_flex()
                .mt_1()
                .p_2()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    h_flex()
                        .gap_2()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child({
                            let (message, _) = self.get_auth_status_message(cx);
                            Label::new(message).color(Color::Muted)
                        }),
                )
                .child(
                    Button::new("reset-credentials", "Reset Credentials")
                        .label_size(LabelSize::Small)
                        .icon(Some(IconName::Trash))
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .disabled(env_var_set)
                        .when(env_var_set, |this| {
                            this.tooltip(Tooltip::text(format!("To reset your API key, unset the {ANTHROPIC_API_KEY_VAR} environment variable.")))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                )
                .into_any()
        }
    }
}

pub struct ClaudeSignIn {
    status: ClaudeSignInStatus,
    focus_handle: FocusHandle,
    state: Entity<State>,
    verification_code_input: Entity<Editor>,
    _subscription: Option<Subscription>,
}

#[derive(Debug, Clone)]
pub enum ClaudeSignInStatus {
    ShowSignInButton,
    ShowCodeInput { verification_url: String },
    Verifying,
    Success,
    Error(String),
}

impl Focusable for ClaudeSignIn {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ClaudeSignIn {}

impl ModalView for ClaudeSignIn {
    fn on_before_dismiss(
        &mut self,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        workspace::DismissDecision::Dismiss(true)
    }
}

impl ClaudeSignIn {
    pub fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let status = ClaudeSignInStatus::ShowSignInButton;

        let verification_code_input = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Paste verification code here...", cx);
            editor
        });

        let this = Self {
            status,
            focus_handle: cx.focus_handle(),
            state: state.clone(),
            verification_code_input,
            _subscription: None,
        };

        this
    }

    fn start_oauth_flow(&mut self, cx: &mut Context<Self>) {
        self.status = ClaudeSignInStatus::Verifying;
        cx.notify();

        cx.spawn(
            async move |this, cx| match AnthropicAuth::authorize().await {
                Ok(authorize_result) => {
                    if authorize_result.url.is_empty() {
                        this.update(cx, |this, cx| {
                            this.status = ClaudeSignInStatus::Error(
                                "Invalid authorization response".to_string(),
                            );
                            cx.notify();
                        })
                        .ok();
                        return;
                    }

                    // Automatically open the URL when we get it
                    let url = authorize_result.url.clone();
                    cx.update(|cx| {
                        cx.open_url(&url);
                    })
                    .ok();

                    this.update(cx, |this, cx| {
                        this.status = ClaudeSignInStatus::ShowCodeInput {
                            verification_url: authorize_result.url,
                        };
                        cx.notify();
                    })
                    .ok();
                }
                Err(err) => {
                    let error_message = if err.to_string().contains("network") {
                        "Network error. Please check your connection and try again.".to_string()
                    } else if err.to_string().contains("timeout") {
                        "Request timed out. Please try again.".to_string()
                    } else {
                        "Failed to start authentication. Please try again.".to_string()
                    };

                    this.update(cx, |this, cx| {
                        this.status = ClaudeSignInStatus::Error(error_message);
                        cx.notify();
                    })
                    .ok();
                    log::error!("Failed to initiate OAuth flow: {}", err);
                }
            },
        )
        .detach();
    }

    fn verify_code(&mut self, code: String, cx: &mut Context<Self>) {
        let code = code.trim().to_string();

        if code.is_empty() {
            self.status = ClaudeSignInStatus::Error("Please enter a verification code".to_string());
            cx.notify();
            return;
        }

        if code.len() < 4 {
            self.status = ClaudeSignInStatus::Error("Verification code is too short".to_string());
            cx.notify();
            return;
        }

        let state = self.state.clone();
        self.status = ClaudeSignInStatus::Verifying;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let http_client = match cx.update(|cx| cx.http_client().clone()) {
                Ok(client) => client,
                Err(_) => {
                    this.update(cx, |this, cx| {
                        this.status = ClaudeSignInStatus::Error("Connection error".to_string());
                        cx.notify();
                    })
                    .ok();
                    return;
                }
            };

            match AnthropicAuth::exchange(&*http_client, &code, "verifier").await {
                Ok(oauth_auth) => {
                    match state.update(cx, |state, cx| state.set_oauth(oauth_auth, cx)) {
                        Ok(save_task) => {
                            if let Err(_) = save_task.await {
                                this.update(cx, |this, cx| {
                                    this.status = ClaudeSignInStatus::Error(
                                        "Failed to save credentials".to_string(),
                                    );
                                    cx.notify();
                                })
                                .ok();
                                return;
                            }

                            state.update(cx, |_, cx| cx.notify()).ok();
                            this.update(cx, |this, cx| {
                                this.status = ClaudeSignInStatus::Success;
                                cx.notify();
                            })
                            .ok();
                        }
                        Err(_) => {
                            this.update(cx, |this, cx| {
                                this.status = ClaudeSignInStatus::Error(
                                    "Failed to save authentication".to_string(),
                                );
                                cx.notify();
                            })
                            .ok();
                        }
                    }
                }
                Err(err) => {
                    let error_message = match err.to_string().to_lowercase() {
                        s if s.contains("invalid") => {
                            "Invalid verification code. Please try again."
                        }
                        s if s.contains("expired") => "Code expired. Please restart sign-in.",
                        s if s.contains("network") => "Network error. Check your connection.",
                        _ => "Authentication failed. Please try again.",
                    };

                    this.update(cx, |this, cx| {
                        this.status = ClaudeSignInStatus::Error(error_message.to_string());
                        cx.notify();
                    })
                    .ok();
                }
            }
        })
        .detach();
    }

    fn render_success_modal(cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        Vector::new(VectorName::ZedXClaude, rems(6.), rems(3.))
                            .color(Color::Custom(cx.theme().colors().icon))
                    )
            )
            .child(Headline::new("Welcome to Claude in Zed!").size(HeadlineSize::Large))
            .child(
                Label::new("Your Claude account is connected and ready to use. You can now access Claude's powerful AI models directly in your editor.")
                    .size(LabelSize::Default)
                    .color(Color::Muted)
            )
            .child(
                Button::new("claude-success-done", "Start Using Claude")
                    .full_width()
                    .style(ButtonStyle::Filled)
                    .on_click(cx.listener(|_, _, _, cx| {
                        cx.emit(DismissEvent);
                    })),
            )
    }

    fn render_error_modal(error: String, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .items_center()
            .child(
                div().flex().items_center().justify_center().child(
                    Vector::new(VectorName::ZedXClaude, rems(6.), rems(3.))
                        .color(Color::Custom(cx.theme().colors().icon)),
                ),
            )
            .child(Headline::new("Something went wrong").size(HeadlineSize::Large))
            .child(
                Label::new(error)
                    .size(LabelSize::Default)
                    .color(Color::Muted),
            )
            .child(
                v_flex()
                    .gap_2()
                    .w_full()
                    .child(
                        Button::new("claude-error-retry", "Try Again")
                            .full_width()
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.status = ClaudeSignInStatus::ShowSignInButton;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("claude-error-cancel", "Cancel")
                            .full_width()
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(DismissEvent);
                            })),
                    ),
            )
    }

    fn render_sign_in_button(cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        Vector::new(VectorName::ZedXClaude, rems(6.), rems(3.))
                            .color(Color::Custom(cx.theme().colors().icon))
                    )
            )
            .child(
                v_flex()
                    .gap_3()
                    .items_center()
                    .child(Headline::new("Sign in to Claude").size(HeadlineSize::Large))
                    .child(
                        Label::new("Connect your Claude account to unlock Anthropic's most advanced AI models right inside Zed.")
                            .size(LabelSize::Default)
                            .color(Color::Muted)
                    )
            )
            .child(
                v_flex()
                    .gap_3()
                    .w_full()
                    .child(
                        Button::new("go-to-claude", "Continue with Claude")
                            .style(ButtonStyle::Filled)
                            .full_width()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.start_oauth_flow(cx);
                            })),
                    )
                    .child(
                        Button::new("claude-cancel", "Cancel")
                            .full_width()
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(DismissEvent);
                            })),
                    )
            )
    }

    fn render_code_input_modal(
        &self,
        verification_url: &str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let verification_url = verification_url.to_string();

        v_flex()
            .gap_6()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        Vector::new(VectorName::ZedXClaude, rems(6.), rems(3.))
                            .color(Color::Custom(cx.theme().colors().icon))
                    )
            )
            .child(
                v_flex()
                    .gap_2()
                    .items_center()
                    .child(Headline::new("Enter your verification code").size(HeadlineSize::Large))
                    .child(
                        Label::new("A browser window should have opened with your verification code. Copy the code and paste it below to complete the setup.")
                            .size(LabelSize::Default)
                            .color(Color::Muted),
                    ),
            )
            .child(
                v_flex()
                    .gap_3()
                    .w_full()
                    .child(
                        h_flex()
                            .w_full()
                            .px_2()
                            .py_1()
                            .bg(cx.theme().colors().editor_background)
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .rounded_sm()
                            .child(self.render_verification_code_editor(cx)),
                    )
                    .child(
                        Button::new("verify-code", "Complete Setup")
                            .style(ButtonStyle::Filled)
                            .full_width()
                            .on_click(cx.listener(|this, _, _, cx| {
                                let code = this
                                    .verification_code_input
                                    .read(cx)
                                    .text(cx);
                                if !code.trim().is_empty() {
                                    this.verify_code(code.trim().to_string(), cx);
                                }
                            })),
                    )
                    .child(
                        Button::new("open-browser-again", "Reopen Browser")
                            .style(ButtonStyle::Subtle)
                            .icon(Some(IconName::ExternalLink))
                            .icon_size(IconSize::Small)
                            .full_width()
                            .on_click({
                                let verification_url = verification_url.clone();
                                cx.listener(move |_, _, _, cx| {
                                    cx.open_url(&verification_url);
                                })
                            }),
                    )
                    .child(
                        Button::new("code-cancel", "Cancel")
                            .full_width()
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(DismissEvent);
                            })),
                    ),
            )
    }

    fn render_verifying_modal(cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        Vector::new(VectorName::ZedXClaude, rems(6.), rems(3.))
                            .color(Color::Custom(cx.theme().colors().icon))
                    )
            )
            .child(
                Icon::new(IconName::Spinner)
                    .size(IconSize::XLarge)
                    .with_animation(
                        "claude_verify_loading",
                        Animation::new(Duration::from_secs(1)).repeat(),
                        |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                    ),
            )
            .child(Headline::new("Connecting to Claude...").size(HeadlineSize::Large))
            .child(
                Label::new("We're securely connecting your Claude account to Zed. This will just take a moment.")
                    .size(LabelSize::Default)
                    .color(Color::Muted),
            )
    }

    fn render_verification_code_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            white_space: WhiteSpace::Normal,
            ..Default::default()
        };
        EditorElement::new(
            &self.verification_code_input,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}

impl Render for ClaudeSignIn {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match &self.status {
            ClaudeSignInStatus::ShowSignInButton => {
                Self::render_sign_in_button(cx).into_any_element()
            }
            ClaudeSignInStatus::ShowCodeInput { verification_url } => self
                .render_code_input_modal(verification_url, cx)
                .into_any_element(),
            ClaudeSignInStatus::Verifying => Self::render_verifying_modal(cx).into_any_element(),
            ClaudeSignInStatus::Success => Self::render_success_modal(cx).into_any_element(),
            ClaudeSignInStatus::Error(error) => {
                Self::render_error_modal(error.clone(), cx).into_any_element()
            }
        };

        div()
            .id("claude-sign-in-modal")
            .track_focus(&self.focus_handle(cx))
            .elevation_3(cx)
            .w(rems(24.))
            .bg(cx.theme().colors().elevated_surface_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_xl()
            .shadow_2xl()
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, _| {
                window.focus(&this.focus_handle);
            }))
            .child(v_flex().p_6().gap_4().items_center().child(content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anthropic::AnthropicModelMode;
    use language_model::{LanguageModelRequestMessage, MessageContent};

    #[test]
    fn test_cache_control_only_on_last_segment() {
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![
                    MessageContent::Text("Some prompt".to_string()),
                    MessageContent::Image(language_model::LanguageModelImage::empty()),
                    MessageContent::Image(language_model::LanguageModelImage::empty()),
                    MessageContent::Image(language_model::LanguageModelImage::empty()),
                    MessageContent::Image(language_model::LanguageModelImage::empty()),
                ],
                cache: true,
            }],
            thread_id: None,
            prompt_id: None,
            intent: None,
            mode: None,
            stop: vec![],
            temperature: None,
            tools: vec![],
            tool_choice: None,
        };

        let anthropic_request = into_anthropic(
            request,
            "claude-3-5-sonnet".to_string(),
            0.7,
            4096,
            AnthropicModelMode::Default,
        );

        assert_eq!(anthropic_request.messages.len(), 1);

        let message = &anthropic_request.messages[0];
        assert_eq!(message.content.len(), 5);

        assert!(matches!(
            message.content[0],
            anthropic::RequestContent::Text {
                cache_control: None,
                ..
            }
        ));
        for i in 1..3 {
            assert!(matches!(
                message.content[i],
                anthropic::RequestContent::Image {
                    cache_control: None,
                    ..
                }
            ));
        }

        assert!(matches!(
            message.content[4],
            anthropic::RequestContent::Image {
                cache_control: Some(anthropic::CacheControl {
                    cache_type: anthropic::CacheControlType::Ephemeral,
                }),
                ..
            }
        ));
    }
}
