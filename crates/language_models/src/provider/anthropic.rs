use anthropic::{
    ANTHROPIC_API_URL, AnthropicError, AnthropicModelMode, ContentDelta, CountTokensRequest, Event,
    ResponseContent, ToolResultContent, ToolResultPart, Usage,
};
use anyhow::Result;
use collections::{BTreeMap, HashMap};
use futures::{FutureExt, Stream, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Entity, Task};
use http_client::HttpClient;
use language_model::{
    ApiKeyState, AuthenticateError, ConfigurationViewTargetAgent, EnvVar, IconOrSvg, LanguageModel,
    LanguageModelCacheConfiguration, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolResultContent, LanguageModelToolUse, MessageContent,
    RateLimiter, Role, StopReason, env_var,
};
use settings::{Settings, SettingsStore};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::{ButtonLink, ConfiguredApiCard, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

pub use settings::AnthropicAvailableModel as AvailableModel;

const PROVIDER_ID: LanguageModelProviderId = language_model::ANTHROPIC_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = language_model::ANTHROPIC_PROVIDER_NAME;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AnthropicSettings {
    pub api_url: String,
    /// Extend Zed's list of Anthropic models.
    pub available_models: Vec<AvailableModel>,
}

pub struct AnthropicLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

const API_KEY_ENV_VAR_NAME: &str = "ANTHROPIC_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

pub struct State {
    api_key_state: ApiKeyState,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = AnthropicLanguageModelProvider::api_url(cx);
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = AnthropicLanguageModelProvider::api_url(cx);
        self.api_key_state
            .load_if_needed(api_url, |this| &mut this.api_key_state, cx)
    }
}

impl AnthropicLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let api_url = Self::api_url(cx);
                this.api_key_state
                    .handle_url_change(api_url, |this| &mut this.api_key_state, cx);
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
            }
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

    fn settings(cx: &App) -> &AnthropicSettings {
        &crate::AllLanguageModelSettings::get_global(cx).anthropic
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            ANTHROPIC_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for AnthropicLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for AnthropicLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiAnthropic)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(anthropic::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(anthropic::Model::default_fast()))
    }

    fn recommended_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        [
            anthropic::Model::ClaudeSonnet4_5,
            anthropic::Model::ClaudeSonnet4_5Thinking,
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
        for model in &AnthropicLanguageModelProvider::settings(cx).available_models {
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
                    mode: model.mode.unwrap_or_default().into(),
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

    fn configuration_view(
        &self,
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), target_agent, window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct AnthropicModel {
    id: LanguageModelId,
    model: anthropic::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

/// Convert a LanguageModelRequest to an Anthropic CountTokensRequest.
pub fn into_anthropic_count_tokens_request(
    request: LanguageModelRequest,
    model: String,
    mode: AnthropicModelMode,
) -> CountTokensRequest {
    let mut new_messages: Vec<anthropic::Message> = Vec::new();
    let mut system_message = String::new();

    for message in request.messages {
        if message.contents_empty() {
            continue;
        }

        match message.role {
            Role::User | Role::Assistant => {
                let anthropic_message_content: Vec<anthropic::RequestContent> = message
                    .content
                    .into_iter()
                    .filter_map(|content| match content {
                        MessageContent::Text(text) => {
                            let text = if text.chars().last().is_some_and(|c| c.is_whitespace()) {
                                text.trim_end().to_string()
                            } else {
                                text
                            };
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
                                Some(anthropic::RequestContent::RedactedThinking { data })
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
                if let Some(last_message) = new_messages.last_mut()
                    && last_message.role == anthropic_role
                {
                    last_message.content.extend(anthropic_message_content);
                    continue;
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

    CountTokensRequest {
        model,
        messages: new_messages,
        system: if system_message.is_empty() {
            None
        } else {
            Some(anthropic::StringOrContents::String(system_message))
        },
        thinking: if request.thinking_allowed
            && let AnthropicModelMode::Thinking { budget_tokens } = mode
        {
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
    }
}

/// Estimate tokens using tiktoken. Used as a fallback when the API is unavailable,
/// or by providers (like Zed Cloud) that don't have direct Anthropic API access.
pub fn count_anthropic_tokens_with_tiktoken(request: LanguageModelRequest) -> Result<u64> {
    let messages = request.messages;
    let mut tokens_from_images = 0;
    let mut string_messages = Vec::with_capacity(messages.len());

    for message in messages {
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

        let (api_key, api_url) = self.state.read_with(cx, |state, cx| {
            let api_url = AnthropicLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        });

        let beta_headers = self.model.beta_headers();

        async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = anthropic::stream_completion(
                http_client.as_ref(),
                &api_url,
                &api_key,
                request,
                beta_headers,
            );
            request.await.map_err(Into::into)
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
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        true
    }

    fn supports_streaming_tools(&self) -> bool {
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
        self.state.read_with(cx, |state, cx| {
            let api_url = AnthropicLanguageModelProvider::api_url(cx);
            state.api_key_state.key(&api_url).map(|key| key.to_string())
        })
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
        let http_client = self.http_client.clone();
        let model_id = self.model.request_id().to_string();
        let mode = self.model.mode();

        let (api_key, api_url) = self.state.read_with(cx, |state, cx| {
            let api_url = AnthropicLanguageModelProvider::api_url(cx);
            (
                state.api_key_state.key(&api_url).map(|k| k.to_string()),
                api_url.to_string(),
            )
        });

        async move {
            // If no API key, fall back to tiktoken estimation
            let Some(api_key) = api_key else {
                return count_anthropic_tokens_with_tiktoken(request);
            };

            let count_request =
                into_anthropic_count_tokens_request(request.clone(), model_id, mode);

            match anthropic::count_tokens(http_client.as_ref(), &api_url, &api_key, count_request)
                .await
            {
                Ok(response) => Ok(response.input_tokens),
                Err(err) => {
                    log::error!(
                        "Anthropic count_tokens API failed, falling back to tiktoken: {err:?}"
                    );
                    count_anthropic_tokens_with_tiktoken(request)
                }
            }
        }
        .boxed()
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
        let bypass_rate_limit = request.bypass_rate_limit;
        let request = into_anthropic(
            request,
            self.model.request_id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
            self.model.mode(),
        );
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream_with_bypass(
            async move {
                let response = request.await?;
                Ok(AnthropicEventMapper::new().map_stream(response))
            },
            bypass_rate_limit,
        );
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
                            let text = if text.chars().last().is_some_and(|c| c.is_whitespace()) {
                                text.trim_end().to_string()
                            } else {
                                text
                            };
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
                                Some(anthropic::RequestContent::RedactedThinking { data })
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
                if let Some(last_message) = new_messages.last_mut()
                    && last_message.role == anthropic_role
                {
                    last_message.content.extend(anthropic_message_content);
                    continue;
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
        thinking: if request.thinking_allowed
            && let AnthropicModelMode::Thinking { budget_tokens } = mode
        {
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
                Err(error) => vec![Err(error.into())],
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
                ResponseContent::RedactedThinking { data } => {
                    vec![Ok(LanguageModelCompletionEvent::RedactedThinking { data })]
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
                                    thought_signature: None,
                                },
                            ))];
                        }
                    }
                    vec![]
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
                                thought_signature: None,
                            },
                        )),
                        Err(json_parse_err) => {
                            Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                                id: tool_use.id.into(),
                                tool_name: tool_use.name.into(),
                                raw_input: input_json.into(),
                                json_parse_error: json_parse_err.to_string(),
                            })
                        }
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
                vec![Err(error.into())]
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
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
    target_agent: ConfigurationViewTargetAgent,
}

impl ConfigurationView {
    const PLACEHOLDER_TEXT: &'static str = "sk-ant-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";

    fn new(
        state: Entity<State>,
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn({
            let state = state.clone();
            async move |this, cx| {
                let task = state.update(cx, |state, cx| state.authenticate(cx));
                // We don't log an error, because "not signed in" is also an error.
                let _ = task.await;
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            api_key_editor: cx.new(|cx| InputField::new(window, cx, Self::PLACEHOLDER_TEXT)),
            state,
            load_credentials_task,
            target_agent,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        // url changes can cause the editor to be displayed again
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable")
        } else {
            let api_url = AnthropicLanguageModelProvider::api_url(cx);
            if api_url == ANTHROPIC_API_URL {
                "API key configured".to_string()
            } else {
                format!("API key configured for {}", api_url)
            }
        };

        if self.load_credentials_task.is_some() {
            div()
                .child(Label::new("Loading credentials..."))
                .into_any_element()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(format!("To use {}, you need to add an API key. Follow these steps:", match &self.target_agent {
                    ConfigurationViewTargetAgent::ZedAgent => "Zed's agent with Anthropic".into(),
                    ConfigurationViewTargetAgent::Other(agent) => agent.clone(),
                })))
                .child(
                    List::new()
                        .child(
                            ListBulletItem::new("")
                                .child(Label::new("Create one by visiting"))
                                .child(ButtonLink::new("Anthropic's settings", "https://console.anthropic.com/settings/keys"))
                        )
                        .child(
                            ListBulletItem::new("Paste your API key below and hit enter to start using the agent")
                        )
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(
                        format!("You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_0p5(),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .when(env_var_set, |this| {
                    this.tooltip_label(format!(
                    "To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."
                ))
                })
                .into_any_element()
        }
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
                reasoning_details: None,
            }],
            thread_id: None,
            prompt_id: None,
            intent: None,
            stop: vec![],
            temperature: None,
            tools: vec![],
            tool_choice: None,
            thinking_allowed: true,
            bypass_rate_limit: false,
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
