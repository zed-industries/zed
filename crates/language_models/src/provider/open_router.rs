use anyhow::Result;
use collections::HashMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, Stream, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, TaskExt};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolResultContent,
    LanguageModelToolSchemaFormat, LanguageModelToolUse, MessageContent, ProviderConfigurationView,
    RateLimiter, Role, StopReason, TokenUsage, env_var,
};
use open_router::{
    Model, ModelMode as OpenRouterModelMode, OPEN_ROUTER_API_URL, ResponseStreamEvent, list_models,
};
use settings::{OpenRouterAvailableModel as AvailableModel, Settings, SettingsStore};
use std::pin::Pin;
use std::sync::{Arc, LazyLock};
use ui::{ButtonLink, ConfiguredApiCard, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

use language_model::util::{fix_streamed_json, parse_tool_arguments};

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("openrouter");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("OpenRouter");

const API_KEY_ENV_VAR_NAME: &str = "OPENROUTER_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);
pub(crate) const RESERVED_HEADER_NAMES: &[&str] = &["HTTP-Referer", "X-Title"];
const MAX_OPEN_ROUTER_SESSION_ID_LENGTH: usize = 256;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenRouterSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

pub struct OpenRouterLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<open_router::Model>,
    fetch_models_task: Option<Task<Result<(), LanguageModelCompletionError>>>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = OpenRouterLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        cx.spawn(async move |this, cx| {
            let result = task.await?;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            Ok(result)
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = OpenRouterLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn fetch_models(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Task<Result<(), LanguageModelCompletionError>> {
        let http_client = self.http_client.clone();
        let api_url = OpenRouterLanguageModelProvider::api_url(cx);
        let extra_headers = OpenRouterLanguageModelProvider::settings(cx)
            .custom_headers
            .clone();
        let Some(api_key) = self.api_key_state.key(&api_url) else {
            return Task::ready(Err(LanguageModelCompletionError::NoApiKey {
                provider: PROVIDER_NAME,
            }));
        };
        cx.spawn(async move |this, cx| {
            let models = list_models(http_client.as_ref(), &api_url, &api_key, &extra_headers)
                .await
                .map_err(LanguageModelCompletionError::from)?;

            this.update(cx, |this, cx| {
                this.available_models = models;
                cx.notify();
            })
            .map_err(|e| LanguageModelCompletionError::Other(e))?;

            Ok(())
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        if self.is_authenticated() {
            let task = self.fetch_models(cx);
            self.fetch_models_task.replace(task);
        } else {
            self.available_models.clear();
        }
    }
}

impl OpenRouterLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>({
                let mut last_settings = OpenRouterLanguageModelProvider::settings(cx).clone();
                move |this: &mut State, cx| {
                    let current_settings = OpenRouterLanguageModelProvider::settings(cx);
                    let settings_changed = current_settings != &last_settings;
                    if settings_changed {
                        last_settings = current_settings.clone();
                        this.authenticate(cx).detach();
                        cx.notify();
                    }
                }
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                credentials_provider,
                http_client: http_client.clone(),
                available_models: Vec::new(),
                fetch_models_task: None,
            }
        });

        Self { http_client, state }
    }

    fn settings(cx: &App) -> &OpenRouterSettings {
        &crate::AllLanguageModelSettings::get_global(cx).open_router
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            OPEN_ROUTER_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }

    fn create_language_model(&self, model: open_router::Model) -> Arc<dyn LanguageModel> {
        Arc::new(OpenRouterLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for OpenRouterLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenRouterLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenRouter)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_router::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models_from_api = self.state.read(cx).available_models.clone();
        let mut settings_models = Vec::new();

        for model in &Self::settings(cx).available_models {
            settings_models.push(open_router::Model {
                name: model.name.clone(),
                display_name: model.display_name.clone(),
                max_tokens: model.max_tokens,
                supports_tools: model.supports_tools,
                supports_images: model.supports_images,
                mode: model.mode.unwrap_or_default(),
                provider: model.provider.clone(),
            });
        }

        for settings_model in &settings_models {
            if let Some(pos) = models_from_api
                .iter()
                .position(|m| m.name == settings_model.name)
            {
                models_from_api[pos] = settings_model.clone();
            } else {
                models_from_api.push(settings_model.clone());
            }
        }

        models_from_api
            .into_iter()
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
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }

    fn configuration_view_v2(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> ProviderConfigurationView {
        let state = self.state.clone();
        ProviderConfigurationView::Inline(
            cx.new(|cx| {
                crate::ApiKeyEditor::new(
                    state,
                    "https://openrouter.ai/keys",
                    "sk-or-...",
                    |state, _cx| crate::api_key_status(&state.api_key_state),
                    |state, key, cx| state.update(cx, |state, cx| state.set_api_key(Some(key), cx)),
                    |state, cx| state.update(cx, |state, cx| state.set_api_key(None, cx)),
                    window,
                    cx,
                )
            })
            .into(),
        )
    }
}

pub struct OpenRouterLanguageModel {
    id: LanguageModelId,
    model: open_router::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenRouterLanguageModel {
    fn stream_completion(
        &self,
        request: open_router::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<ResponseStreamEvent, open_router::OpenRouterError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();
        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = OpenRouterLanguageModelProvider::api_url(cx);
            let extra_headers = OpenRouterLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = open_router::stream_completion(
                http_client.as_ref(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            );
            request.await.map_err(Into::into)
        }
        .boxed()
    }
}

impl LanguageModel for OpenRouterLanguageModel {
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
        self.model.supports_tool_calls()
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        matches!(self.model.mode, OpenRouterModelMode::Thinking { .. })
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        let model_id = self.model.id().trim().to_lowercase();
        if model_id.contains("gemini") || model_id.contains("grok") {
            LanguageModelToolSchemaFormat::JsonSchemaSubset
        } else {
            LanguageModelToolSchemaFormat::JsonSchema
        }
    }

    fn telemetry_id(&self) -> String {
        format!("openrouter/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => true,
            LanguageModelToolChoice::Any => true,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images.unwrap_or(false)
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        let openrouter_request = into_open_router(request, &self.model, self.max_output_tokens());
        let request = self.stream_completion(openrouter_request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.await?;
            Ok(OpenRouterEventMapper::new().map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

pub fn into_open_router(
    request: LanguageModelRequest,
    model: &Model,
    max_output_tokens: Option<u64>,
) -> open_router::Request {
    // Anthropic models via OpenRouter don't accept reasoning_details being echoed back
    // in requests - it's an output-only field for them. However, Gemini models require
    // the thought signatures to be echoed back for proper reasoning chain continuity.
    // Note: OpenRouter's model API provides an `architecture.tokenizer` field (e.g. "Claude",
    // "Gemini") which could replace this ID prefix check, but since this is the only place
    // we need this distinction, we're just using this less invasive check instead.
    // If we ever have a more formal distionction between the models in the future,
    // we should revise this to use that instead.
    let is_anthropic_model = model.id().starts_with("anthropic/");
    let session_id = open_router_session_id(request.thread_id);

    let mut messages = Vec::new();
    let mut any_message_wants_cache = false;
    let mut last_cache_message_index: Option<usize> = None;

    for message in request.messages {
        let mut message_added_content = false;
        let reasoning_details_for_message = if is_anthropic_model {
            None
        } else {
            message.reasoning_details.clone()
        };

        let message_wants_cache = message.cache;
        if message_wants_cache {
            any_message_wants_cache = true;
        }

        for content in message.content {
            match content {
                MessageContent::Text(text) => {
                    add_message_content_part(
                        open_router::MessagePart::Text {
                            text,
                            cache_control: None,
                        },
                        message.role,
                        &mut messages,
                        reasoning_details_for_message.clone(),
                    );
                    message_added_content = true;
                }
                MessageContent::Thinking { .. } => {}
                MessageContent::RedactedThinking(_) => {}
                MessageContent::Compaction(_) => {}
                MessageContent::Image(image) => {
                    add_message_content_part(
                        open_router::MessagePart::Image {
                            image_url: image.to_base64_url(),
                        },
                        message.role,
                        &mut messages,
                        reasoning_details_for_message.clone(),
                    );
                    message_added_content = true;
                }
                MessageContent::ToolUse(tool_use) => {
                    let tool_call = open_router::ToolCall {
                        id: tool_use.id.to_string(),
                        content: open_router::ToolCallContent::Function {
                            function: open_router::FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                                thought_signature: tool_use.thought_signature.clone(),
                            },
                        },
                    };

                    if let Some(open_router::RequestMessage::Assistant { tool_calls, .. }) =
                        messages.last_mut()
                    {
                        tool_calls.push(tool_call);
                    } else {
                        messages.push(open_router::RequestMessage::Assistant {
                            content: None,
                            tool_calls: vec![tool_call],
                            reasoning_details: reasoning_details_for_message.clone(),
                        });
                    }
                    message_added_content = true;
                }
                MessageContent::ToolResult(tool_result) => {
                    let content: Vec<open_router::MessagePart> = tool_result
                        .content
                        .iter()
                        .map(|part| match part {
                            LanguageModelToolResultContent::Text(text) => {
                                open_router::MessagePart::Text {
                                    text: text.to_string(),
                                    cache_control: None,
                                }
                            }
                            LanguageModelToolResultContent::Image(image) => {
                                open_router::MessagePart::Image {
                                    image_url: image.to_base64_url(),
                                }
                            }
                        })
                        .collect();

                    messages.push(open_router::RequestMessage::Tool {
                        content: content.into(),
                        tool_call_id: tool_result.tool_use_id.to_string(),
                    });
                    message_added_content = true;
                }
            }
        }

        if message_wants_cache && message_added_content {
            last_cache_message_index = messages.len().checked_sub(1);
        }
    }

    if is_anthropic_model && any_message_wants_cache {
        // OpenRouter's top-level automatic cache_control restricts routing to
        // Anthropic direct; explicit block breakpoints also work on Bedrock and Vertex.
        if let Some(content) = last_cache_message_index
            .and_then(|index| messages.get_mut(index))
            .and_then(request_message_content_mut)
        {
            set_last_text_cache_control(content, cache_control(None));
        }

        if let Some(content) = messages.iter_mut().find_map(|message| match message {
            open_router::RequestMessage::System { content } => Some(content),
            _ => None,
        }) {
            set_last_text_cache_control(
                content,
                cache_control(Some(open_router::CacheTtl::OneHour)),
            );
        }
    }

    open_router::Request {
        model: model.id().into(),
        messages,
        stream: true,
        session_id,
        stop: request.stop,
        temperature: request.temperature.unwrap_or(0.4),
        max_tokens: max_output_tokens,
        parallel_tool_calls: if model.supports_parallel_tool_calls() && !request.tools.is_empty() {
            Some(false)
        } else {
            None
        },
        usage: open_router::RequestUsage { include: true },
        reasoning: if request.thinking_allowed
            && let OpenRouterModelMode::Thinking { budget_tokens } = model.mode
        {
            Some(open_router::Reasoning {
                effort: None,
                max_tokens: budget_tokens,
                exclude: Some(false),
                enabled: Some(true),
            })
        } else {
            None
        },
        tools: request
            .tools
            .into_iter()
            .map(|tool| open_router::ToolDefinition::Function {
                function: open_router::FunctionDefinition {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect(),
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => open_router::ToolChoice::Auto,
            LanguageModelToolChoice::Any => open_router::ToolChoice::Required,
            LanguageModelToolChoice::None => open_router::ToolChoice::None,
        }),
        provider: model.provider.clone(),
    }
}

fn open_router_session_id(thread_id: Option<String>) -> Option<String> {
    thread_id.map(|thread_id| {
        thread_id
            .chars()
            .take(MAX_OPEN_ROUTER_SESSION_ID_LENGTH)
            .collect()
    })
}

fn cache_control(ttl: Option<open_router::CacheTtl>) -> open_router::CacheControl {
    open_router::CacheControl {
        cache_type: open_router::CacheControlType::Ephemeral,
        ttl,
    }
}

fn request_message_content_mut(
    message: &mut open_router::RequestMessage,
) -> Option<&mut open_router::MessageContent> {
    match message {
        open_router::RequestMessage::User { content }
        | open_router::RequestMessage::System { content }
        | open_router::RequestMessage::Tool { content, .. } => Some(content),
        open_router::RequestMessage::Assistant {
            content: Some(content),
            ..
        } => Some(content),
        open_router::RequestMessage::Assistant { content: None, .. } => None,
    }
}

fn set_last_text_cache_control(
    content: &mut open_router::MessageContent,
    cache_control: open_router::CacheControl,
) {
    match content {
        open_router::MessageContent::Plain(text) => {
            let text = std::mem::take(text);
            *content =
                open_router::MessageContent::Multipart(vec![open_router::MessagePart::Text {
                    text,
                    cache_control: Some(cache_control),
                }]);
        }
        open_router::MessageContent::Multipart(parts) => {
            for part in parts.iter_mut().rev() {
                if let open_router::MessagePart::Text {
                    cache_control: target,
                    ..
                } = part
                {
                    *target = Some(cache_control);
                    break;
                }
            }
        }
    }
}

fn add_message_content_part(
    new_part: open_router::MessagePart,
    role: Role,
    messages: &mut Vec<open_router::RequestMessage>,
    reasoning_details: Option<Arc<serde_json::Value>>,
) {
    match (role, messages.last_mut()) {
        (Role::User, Some(open_router::RequestMessage::User { content }))
        | (Role::System, Some(open_router::RequestMessage::System { content })) => {
            content.push_part(new_part);
        }
        (
            Role::Assistant,
            Some(open_router::RequestMessage::Assistant {
                content: Some(content),
                ..
            }),
        ) => {
            content.push_part(new_part);
        }
        _ => {
            messages.push(match role {
                Role::User => open_router::RequestMessage::User {
                    content: open_router::MessageContent::from(vec![new_part]),
                },
                Role::Assistant => open_router::RequestMessage::Assistant {
                    content: Some(open_router::MessageContent::from(vec![new_part])),
                    tool_calls: Vec::new(),
                    reasoning_details,
                },
                Role::System => open_router::RequestMessage::System {
                    content: open_router::MessageContent::from(vec![new_part]),
                },
            });
        }
    }
}

pub struct OpenRouterEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
    reasoning_details: Option<serde_json::Value>,
}

impl OpenRouterEventMapper {
    pub fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
            reasoning_details: None,
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<
            Box<
                dyn Send + Stream<Item = Result<ResponseStreamEvent, open_router::OpenRouterError>>,
            >,
        >,
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
        event: ResponseStreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();

        if let Some(usage) = event.usage {
            let cache_creation_input_tokens = usage
                .prompt_tokens_details
                .as_ref()
                .map_or(0, |details| details.cache_write_tokens);
            let cache_read_input_tokens = usage
                .prompt_tokens_details
                .as_ref()
                .map_or(0, |details| details.cached_tokens);
            let input_tokens = usage.prompt_tokens.saturating_sub(
                cache_creation_input_tokens.saturating_add(cache_read_input_tokens),
            );

            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens,
                output_tokens: usage.completion_tokens,
                cache_creation_input_tokens,
                cache_read_input_tokens,
            })));
        }

        let Some(choice) = event.choices.first() else {
            return events;
        };

        if let Some(details) = choice.delta.reasoning_details.clone() {
            // Emit reasoning_details immediately
            events.push(Ok(LanguageModelCompletionEvent::ReasoningDetails(
                details.clone(),
            )));
            self.reasoning_details = Some(details);
        }

        if let Some(reasoning) = choice.delta.reasoning.clone() {
            events.push(Ok(LanguageModelCompletionEvent::Thinking {
                text: reasoning,
                signature: None,
            }));
        }

        if let Some(content) = choice.delta.content.clone() {
            // OpenRouter send empty content string with the reasoning content
            // This is a workaround for the OpenRouter API bug
            if !content.is_empty() {
                events.push(Ok(LanguageModelCompletionEvent::Text(content)));
            }
        }

        if let Some(tool_calls) = choice.delta.tool_calls.as_ref() {
            for tool_call in tool_calls {
                let entry = self.tool_calls_by_index.entry(tool_call.index).or_default();

                if let Some(tool_id) = tool_call.id.clone() {
                    entry.id = tool_id;
                }

                if let Some(function) = tool_call.function.as_ref() {
                    if let Some(name) = function.name.clone() {
                        entry.name = name;
                    }

                    if let Some(arguments) = function.arguments.clone() {
                        entry.arguments.push_str(&arguments);
                    }

                    if let Some(signature) = function.thought_signature.clone() {
                        entry.thought_signature = Some(signature);
                    }
                }

                if !entry.id.is_empty() && !entry.name.is_empty() {
                    if let Ok(input) = serde_json::from_str::<serde_json::Value>(
                        &fix_streamed_json(&entry.arguments),
                    ) {
                        events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: entry.id.clone().into(),
                                name: entry.name.as_str().into(),
                                is_input_complete: false,
                                input,
                                raw_input: entry.arguments.clone(),
                                thought_signature: entry.thought_signature.clone(),
                            },
                        )));
                    }
                }
            }
        }

        match choice.finish_reason.as_deref() {
            Some("stop") => {
                // Don't emit reasoning_details here - already emitted immediately when captured
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            Some("tool_calls") => {
                events.extend(self.tool_calls_by_index.drain().map(|(_, tool_call)| {
                    match parse_tool_arguments(&tool_call.arguments) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_call.id.clone().into(),
                                name: tool_call.name.as_str().into(),
                                is_input_complete: true,
                                input,
                                raw_input: tool_call.arguments.clone(),
                                thought_signature: tool_call.thought_signature.clone(),
                            },
                        )),
                        Err(error) => Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                            id: tool_call.id.clone().into(),
                            tool_name: tool_call.name.as_str().into(),
                            raw_input: tool_call.arguments.clone().into(),
                            json_parse_error: error.to_string(),
                        }),
                    }
                }));

                // Don't emit reasoning_details here - already emitted immediately when captured
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
            }
            Some(stop_reason) => {
                log::error!("Unexpected OpenRouter stop_reason: {stop_reason:?}",);
                // Don't emit reasoning_details here - already emitted immediately when captured
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            None => {}
        }

        events
    }
}

#[derive(Default)]
struct RawToolCall {
    id: String,
    name: String,
    arguments: String,
    thought_signature: Option<String>,
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            InputField::new(
                window,
                cx,
                "sk_or_000000000000000000000000000000000000000000000000",
            )
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = Some(state.update(cx, |state, cx| state.authenticate(cx))) {
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
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
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
            let api_url = OpenRouterLanguageModelProvider::api_url(cx);
            if api_url == OPEN_ROUTER_API_URL {
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
                .child(Label::new("To use Zed's agent with OpenRouter, you need to add an API key. Follow these steps:"))
                .child(
                    List::new()
                        .child(
                            ListBulletItem::new("")
                                .child(Label::new("Create an API key by visiting"))
                                .child(ButtonLink::new("OpenRouter's console", "https://openrouter.ai/keys"))
                        )
                        .child(ListBulletItem::new("Ensure your OpenRouter account has credits")
                        )
                        .child(ListBulletItem::new("Paste your API key below and hit enter to start using the assistant")
                        ),
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(
                        format!("You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .when(env_var_set, |this| {
                    this.tooltip_label(format!("To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."))
                })
                .into_any_element()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use open_router::{ChoiceDelta, FunctionChunk, ResponseMessageDelta, ToolCallChunk};

    #[gpui::test]
    async fn test_reasoning_details_preservation_with_tool_calls() {
        // This test verifies that reasoning_details are properly captured and preserved
        // when a model uses tool calling with reasoning/thinking tokens.
        //
        // The key regression this prevents:
        // - OpenRouter sends multiple reasoning_details updates during streaming
        // - First with actual content (encrypted reasoning data)
        // - Then with empty array on completion
        // - We must NOT overwrite the real data with the empty array

        let mut mapper = OpenRouterEventMapper::new();

        // Simulate the streaming events as they come from OpenRouter/Gemini
        let events = vec![
            // Event 1: Initial reasoning details with text
            ResponseStreamEvent {
                id: Some("response_123".into()),
                created: 1234567890,
                model: "google/gemini-3.1-pro-preview".into(),
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: ResponseMessageDelta {
                        role: None,
                        content: None,
                        reasoning: None,
                        tool_calls: None,
                        reasoning_details: Some(serde_json::json!([
                            {
                                "type": "reasoning.text",
                                "text": "Let me analyze this request...",
                                "format": "google-gemini-v1",
                                "index": 0
                            }
                        ])),
                    },
                    finish_reason: None,
                }],
                usage: None,
            },
            // Event 2: More reasoning details
            ResponseStreamEvent {
                id: Some("response_123".into()),
                created: 1234567890,
                model: "google/gemini-3.1-pro-preview".into(),
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: ResponseMessageDelta {
                        role: None,
                        content: None,
                        reasoning: None,
                        tool_calls: None,
                        reasoning_details: Some(serde_json::json!([
                            {
                                "type": "reasoning.encrypted",
                                "data": "EtgDCtUDAdHtim9OF5jm4aeZSBAtl/randomized123",
                                "format": "google-gemini-v1",
                                "index": 0,
                                "id": "tool_call_abc123"
                            }
                        ])),
                    },
                    finish_reason: None,
                }],
                usage: None,
            },
            // Event 3: Tool call starts
            ResponseStreamEvent {
                id: Some("response_123".into()),
                created: 1234567890,
                model: "google/gemini-3.1-pro-preview".into(),
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: ResponseMessageDelta {
                        role: None,
                        content: None,
                        reasoning: None,
                        tool_calls: Some(vec![ToolCallChunk {
                            index: 0,
                            id: Some("tool_call_abc123".into()),
                            function: Some(FunctionChunk {
                                name: Some("list_directory".into()),
                                arguments: Some("{\"path\":\"test\"}".into()),
                                thought_signature: Some("sha256:test_signature_xyz789".into()),
                            }),
                        }]),
                        reasoning_details: None,
                    },
                    finish_reason: None,
                }],
                usage: None,
            },
            // Event 4: Empty reasoning_details on tool_calls finish
            // This is the critical event - we must not overwrite with this empty array!
            ResponseStreamEvent {
                id: Some("response_123".into()),
                created: 1234567890,
                model: "google/gemini-3.1-pro-preview".into(),
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: ResponseMessageDelta {
                        role: None,
                        content: None,
                        reasoning: None,
                        tool_calls: None,
                        reasoning_details: Some(serde_json::json!([])),
                    },
                    finish_reason: Some("tool_calls".into()),
                }],
                usage: None,
            },
        ];

        // Process all events
        let mut collected_events = Vec::new();
        for event in events {
            let mapped = mapper.map_event(event);
            collected_events.extend(mapped);
        }

        // Verify we got the expected events
        let mut has_tool_use = false;
        let mut reasoning_details_events = Vec::new();
        let mut thought_signature_value = None;

        for event_result in collected_events {
            match event_result {
                Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) => {
                    has_tool_use = true;
                    assert_eq!(tool_use.id.to_string(), "tool_call_abc123");
                    assert_eq!(tool_use.name.as_ref(), "list_directory");
                    thought_signature_value = tool_use.thought_signature.clone();
                }
                Ok(LanguageModelCompletionEvent::ReasoningDetails(details)) => {
                    reasoning_details_events.push(details);
                }
                _ => {}
            }
        }

        // Assertions
        assert!(has_tool_use, "Should have emitted ToolUse event");
        assert!(
            !reasoning_details_events.is_empty(),
            "Should have emitted ReasoningDetails events"
        );

        // We should have received multiple reasoning_details events (text, encrypted, empty)
        // The agent layer is responsible for keeping only the first non-empty one
        assert!(
            reasoning_details_events.len() >= 2,
            "Should have multiple reasoning_details events from streaming"
        );

        // Verify at least one contains the encrypted data
        let has_encrypted = reasoning_details_events.iter().any(|details| {
            if let serde_json::Value::Array(arr) = details {
                arr.iter().any(|item| {
                    item["type"] == "reasoning.encrypted"
                        && item["data"]
                            .as_str()
                            .map_or(false, |s| s.contains("EtgDCtUDAdHtim9OF5jm4aeZSBAtl"))
                })
            } else {
                false
            }
        });
        assert!(
            has_encrypted,
            "Should have at least one reasoning_details with encrypted data"
        );

        // Verify thought_signature was captured
        assert!(
            thought_signature_value.is_some(),
            "Tool use should have thought_signature"
        );
        assert_eq!(
            thought_signature_value.unwrap(),
            "sha256:test_signature_xyz789"
        );
    }

    #[gpui::test]
    async fn test_usage_only_chunk_with_empty_choices_does_not_error() {
        let mut mapper = OpenRouterEventMapper::new();

        let events = mapper.map_event(ResponseStreamEvent {
            id: Some("response_123".into()),
            created: 1234567890,
            model: "google/gemini-3-flash-preview".into(),
            choices: Vec::new(),
            usage: Some(open_router::Usage {
                prompt_tokens: 12,
                completion_tokens: 7,
                total_tokens: 19,
                prompt_tokens_details: Some(open_router::PromptTokensDetails {
                    cached_tokens: 5,
                    cache_write_tokens: 3,
                }),
            }),
        });

        assert_eq!(events.len(), 1);
        match events.into_iter().next() {
            Some(Ok(LanguageModelCompletionEvent::UsageUpdate(usage))) => {
                assert_eq!(usage.input_tokens, 4);
                assert_eq!(usage.output_tokens, 7);
                assert_eq!(usage.cache_creation_input_tokens, 3);
                assert_eq!(usage.cache_read_input_tokens, 5);
                assert_eq!(usage.total_tokens(), 19);
            }
            other => panic!("Expected usage update event, got: {other:?}"),
        }
    }

    #[gpui::test]
    async fn test_session_id_uses_thread_id() {
        let model = open_router::Model::new(
            "openai/gpt-4o",
            Some("GPT-4o"),
            Some(128000),
            Some(true),
            Some(false),
            None,
            None,
        );
        let expected_session_id = "a".repeat(MAX_OPEN_ROUTER_SESSION_ID_LENGTH);
        let request = LanguageModelRequest {
            thread_id: Some(format!("{expected_session_id}extra")),
            messages: vec![language_model::LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello".to_string())],
                cache: false,
                reasoning_details: None,
            }],
            ..Default::default()
        };

        let result = into_open_router(request, &model, None);

        assert_eq!(
            result.session_id.as_deref(),
            Some(expected_session_id.as_str())
        );
    }

    #[gpui::test]
    async fn test_agent_prevents_empty_reasoning_details_overwrite() {
        // This test verifies that the agent layer prevents empty reasoning_details
        // from overwriting non-empty ones, even though the mapper emits all events.

        // Simulate what the agent does when it receives multiple ReasoningDetails events
        let mut agent_reasoning_details: Option<serde_json::Value> = None;

        let events = vec![
            // First event: non-empty reasoning_details
            serde_json::json!([
                {
                    "type": "reasoning.encrypted",
                    "data": "real_data_here",
                    "format": "google-gemini-v1"
                }
            ]),
            // Second event: empty array (should not overwrite)
            serde_json::json!([]),
        ];

        for details in events {
            // This mimics the agent's logic: only store if we don't already have it
            if agent_reasoning_details.is_none() {
                agent_reasoning_details = Some(details);
            }
        }

        // Verify the agent kept the first non-empty reasoning_details
        assert!(agent_reasoning_details.is_some());
        let final_details = agent_reasoning_details.unwrap();
        if let serde_json::Value::Array(arr) = &final_details {
            assert!(
                !arr.is_empty(),
                "Agent should have kept the non-empty reasoning_details"
            );
            assert_eq!(arr[0]["data"], "real_data_here");
        } else {
            panic!("Expected array");
        }
    }

    #[gpui::test]
    async fn test_anthropic_model_caching_two_tier() {
        let model = open_router::Model::new(
            "anthropic/claude-sonnet-4-5",
            Some("Claude Sonnet"),
            Some(200000),
            Some(true),
            Some(false),
            None,
            None,
        );

        let request = LanguageModelRequest {
            messages: vec![
                language_model::LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("You are helpful.".to_string())],
                    cache: false,
                    reasoning_details: None,
                },
                language_model::LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Hello".to_string())],
                    cache: false,
                    reasoning_details: None,
                },
                language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::Text("Hi there!".to_string())],
                    cache: false,
                    reasoning_details: None,
                },
                language_model::LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("What is 2+2?".to_string())],
                    cache: true,
                    reasoning_details: None,
                },
            ],
            stop: vec![],
            temperature: None,
            tools: vec![],
            tool_choice: None,
            thinking_allowed: false,
            thinking_effort: None,
            speed: None,
            thread_id: None,
            prompt_id: None,
            intent: None,
            compact_at_tokens: None,
        };

        let result = into_open_router(request, &model, None);

        let system_cache = result.messages.iter().find_map(|m| {
            if let open_router::RequestMessage::System { content } = m {
                if let open_router::MessageContent::Multipart(parts) = content {
                    parts.iter().last().and_then(|p| {
                        if let open_router::MessagePart::Text { cache_control, .. } = p {
                            *cache_control
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            } else {
                None
            }
        });
        assert!(
            matches!(
                system_cache,
                Some(open_router::CacheControl {
                    cache_type: open_router::CacheControlType::Ephemeral,
                    ttl: Some(open_router::CacheTtl::OneHour),
                })
            ),
            "System message should have 1h cache_control, got: {system_cache:?}"
        );

        let tail_cache = result.messages.last().and_then(|last_message| {
            if let open_router::RequestMessage::User { content } = last_message {
                if let open_router::MessageContent::Multipart(parts) = content {
                    parts.iter().last().and_then(|part| {
                        if let open_router::MessagePart::Text { cache_control, .. } = part {
                            *cache_control
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            } else {
                None
            }
        });
        assert!(
            matches!(
                tail_cache,
                Some(open_router::CacheControl {
                    cache_type: open_router::CacheControlType::Ephemeral,
                    ttl: None,
                })
            ),
            "Last cache:true message should have 5min cache_control, got: {tail_cache:?}"
        );

        for (i, message) in result.messages.iter().enumerate() {
            let is_system = matches!(message, open_router::RequestMessage::System { .. });
            let is_last = i == result.messages.len() - 1;
            if is_system || is_last {
                continue;
            }
            let parts: Option<&Vec<open_router::MessagePart>> = match message {
                open_router::RequestMessage::User { content }
                | open_router::RequestMessage::System { content }
                | open_router::RequestMessage::Tool { content, .. } => {
                    if let open_router::MessageContent::Multipart(parts) = content {
                        Some(parts)
                    } else {
                        None
                    }
                }
                open_router::RequestMessage::Assistant {
                    content: Some(content),
                    ..
                } => {
                    if let open_router::MessageContent::Multipart(parts) = content {
                        Some(parts)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(parts) = parts {
                for part in parts {
                    if let open_router::MessagePart::Text { cache_control, .. } = part {
                        assert!(
                            cache_control.is_none(),
                            "Message {i} should not have cache_control"
                        );
                    }
                }
            }
        }
    }

    #[gpui::test]
    async fn test_anthropic_model_no_cache_when_no_cache_flag() {
        let model = open_router::Model::new(
            "anthropic/claude-sonnet-4-5",
            Some("Claude Sonnet"),
            Some(200000),
            Some(true),
            Some(false),
            None,
            None,
        );

        let request = LanguageModelRequest {
            messages: vec![
                language_model::LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("You are helpful.".to_string())],
                    cache: false,
                    reasoning_details: None,
                },
                language_model::LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Hello".to_string())],
                    cache: false,
                    reasoning_details: None,
                },
            ],
            stop: vec![],
            temperature: None,
            tools: vec![],
            tool_choice: None,
            thinking_allowed: false,
            thinking_effort: None,
            speed: None,
            thread_id: None,
            prompt_id: None,
            intent: None,
            compact_at_tokens: None,
        };

        let result = into_open_router(request, &model, None);

        for message in &result.messages {
            let content = match message {
                open_router::RequestMessage::User { content }
                | open_router::RequestMessage::System { content } => Some(content),
                _ => None,
            };
            if let Some(content) = content {
                if let open_router::MessageContent::Multipart(parts) = content {
                    for part in parts {
                        if let open_router::MessagePart::Text { cache_control, .. } = part {
                            assert!(
                                cache_control.is_none(),
                                "No message should have cache_control when no cache:true flags"
                            );
                        }
                    }
                }
            }
        }
    }

    #[gpui::test]
    async fn test_non_anthropic_model_no_cache_control() {
        let model = open_router::Model::new(
            "openai/gpt-4o",
            Some("GPT-4o"),
            Some(128000),
            Some(true),
            Some(false),
            None,
            None,
        );

        let request = LanguageModelRequest {
            messages: vec![
                language_model::LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("You are helpful.".to_string())],
                    cache: false,
                    reasoning_details: None,
                },
                language_model::LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Hello".to_string())],
                    cache: true,
                    reasoning_details: None,
                },
            ],
            stop: vec![],
            temperature: None,
            tools: vec![],
            tool_choice: None,
            thinking_allowed: false,
            thinking_effort: None,
            speed: None,
            thread_id: None,
            prompt_id: None,
            intent: None,
            compact_at_tokens: None,
        };

        let result = into_open_router(request, &model, None);

        for message in &result.messages {
            let content = match message {
                open_router::RequestMessage::User { content }
                | open_router::RequestMessage::System { content } => Some(content),
                _ => None,
            };
            if let Some(content) = content {
                if let open_router::MessageContent::Multipart(parts) = content {
                    for part in parts {
                        if let open_router::MessagePart::Text { cache_control, .. } = part {
                            assert!(
                                cache_control.is_none(),
                                "Non-Anthropic model should never have cache_control"
                            );
                        }
                    }
                }
            }
        }
    }
}
