use anyhow::{Result, anyhow};
use collections::HashMap;
use fs::Fs;
use futures::Stream;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, CursorStyle, Entity, Subscription, Task};
use http_client::HttpClient;
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelToolChoice, LanguageModelToolResultContent,
    LanguageModelToolUse, MessageContent, StopReason, TokenUsage, env_var,
};
use language_model::{
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest, RateLimiter, Role,
};
use lmstudio::{LMSTUDIO_API_URL, ModelType, get_models};

pub use settings::LmStudioAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore, update_settings_file};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::LazyLock;
use std::{collections::BTreeMap, sync::Arc};
use ui::{
    ButtonLike, ConfiguredApiCard, ElevationIndex, List, ListBulletItem, Tooltip, prelude::*,
};
use ui_input::InputField;

use crate::AllLanguageModelSettings;

const LMSTUDIO_DOWNLOAD_URL: &str = "https://lmstudio.ai/download";
const LMSTUDIO_CATALOG_URL: &str = "https://lmstudio.ai/models";
const LMSTUDIO_SITE: &str = "https://lmstudio.ai/";

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("lmstudio");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("LM Studio");

const API_KEY_ENV_VAR_NAME: &str = "LMSTUDIO_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LmStudioSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct LmStudioLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<lmstudio::Model>,
    fetch_model_task: Option<Task<Result<()>>>,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        !self.available_models.is_empty()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = LmStudioLanguageModelProvider::api_url(cx).into();
        let task = self
            .api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx);
        self.restart_fetch_models_task(cx);
        task
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
        let http_client = self.http_client.clone();
        let api_url = settings.api_url.clone();
        let api_key = self.api_key_state.key(&api_url);

        // As a proxy for the server being "authenticated", we'll check if its up by fetching the models
        cx.spawn(async move |this, cx| {
            let models =
                get_models(http_client.as_ref(), &api_url, api_key.as_deref(), None).await?;

            let mut models: Vec<lmstudio::Model> = models
                .into_iter()
                .filter(|model| model.r#type != ModelType::Embeddings)
                .map(|model| {
                    lmstudio::Model::new(
                        &model.id,
                        None,
                        model
                            .loaded_context_length
                            .or_else(|| model.max_context_length),
                        model.capabilities.supports_tool_calls(),
                        model.capabilities.supports_images() || model.r#type == ModelType::Vlm,
                    )
                })
                .collect();

            models.sort_by(|a, b| a.name.cmp(&b.name));

            this.update(cx, |this, cx| {
                this.available_models = models;
                cx.notify();
            })
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        let task = self.fetch_models(cx);
        self.fetch_model_task.replace(task);
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = LmStudioLanguageModelProvider::api_url(cx).into();
        let _task = self
            .api_key_state
            .load_if_needed(api_url, |this| &mut this.api_key_state, cx);

        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let fetch_models_task = self.fetch_models(cx);
        cx.spawn(async move |_this, _cx| {
            match fetch_models_task.await {
                Ok(()) => Ok(()),
                Err(err) => {
                    // If any cause in the error chain is an std::io::Error with
                    // ErrorKind::ConnectionRefused, treat this as "credentials not found"
                    // (i.e. LM Studio not running).
                    let mut connection_refused = false;
                    for cause in err.chain() {
                        if let Some(io_err) = cause.downcast_ref::<std::io::Error>() {
                            if io_err.kind() == std::io::ErrorKind::ConnectionRefused {
                                connection_refused = true;
                                break;
                            }
                        }
                    }
                    if connection_refused {
                        Err(AuthenticateError::ConnectionRefused)
                    } else {
                        Err(AuthenticateError::Other(err))
                    }
                }
            }
        })
    }
}

impl LmStudioLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let this = Self {
            http_client: http_client.clone(),
            state: cx.new(|cx| {
                let subscription = cx.observe_global::<SettingsStore>({
                    let mut settings = AllLanguageModelSettings::get_global(cx).lmstudio.clone();
                    move |this: &mut State, cx| {
                        let new_settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
                        if &settings != new_settings {
                            settings = new_settings.clone();
                            this.restart_fetch_models_task(cx);
                            cx.notify();
                        }
                    }
                });

                State {
                    api_key_state: ApiKeyState::new(
                        Self::api_url(cx).into(),
                        (*API_KEY_ENV_VAR).clone(),
                    ),
                    http_client,
                    available_models: Default::default(),
                    fetch_model_task: None,
                    _subscription: subscription,
                }
            }),
        };
        this.state
            .update(cx, |state, cx| state.restart_fetch_models_task(cx));
        this
    }

    fn api_url(cx: &App) -> String {
        AllLanguageModelSettings::get_global(cx)
            .lmstudio
            .api_url
            .clone()
    }

    fn has_custom_url(cx: &App) -> bool {
        Self::api_url(cx) != LMSTUDIO_API_URL
    }
}

impl LanguageModelProviderState for LmStudioLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for LmStudioLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiLmStudio)
    }

    fn default_model(&self, _: &App) -> Option<Arc<dyn LanguageModel>> {
        // We shouldn't try to select default model, because it might lead to a load call for an unloaded model.
        // In a constrained environment where user might not have enough resources it'll be a bad UX to select something
        // to load by default.
        None
    }

    fn default_fast_model(&self, _: &App) -> Option<Arc<dyn LanguageModel>> {
        // See explanation for default_model.
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models: BTreeMap<String, lmstudio::Model> = BTreeMap::default();

        // Add models from the LM Studio API
        for model in self.state.read(cx).available_models.iter() {
            models.insert(model.name.clone(), model.clone());
        }

        // Override with available models from settings
        for model in AllLanguageModelSettings::get_global(cx)
            .lmstudio
            .available_models
            .iter()
        {
            models.insert(
                model.name.clone(),
                lmstudio::Model {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    supports_tool_calls: model.supports_tool_calls,
                    supports_images: model.supports_images,
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(LmStudioLanguageModel {
                    id: LanguageModelId::from(model.name.clone()),
                    model,
                    http_client: self.http_client.clone(),
                    request_limiter: RateLimiter::new(4),
                    state: self.state.clone(),
                }) as Arc<dyn LanguageModel>
            })
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
        _window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), _window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct LmStudioLanguageModel {
    id: LanguageModelId,
    model: lmstudio::Model,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
    state: Entity<State>,
}

impl LmStudioLanguageModel {
    fn to_lmstudio_request(
        &self,
        request: LanguageModelRequest,
    ) -> lmstudio::ChatCompletionRequest {
        let mut messages = Vec::new();

        for message in request.messages {
            for content in message.content {
                match content {
                    MessageContent::Text(text) => add_message_content_part(
                        lmstudio::MessagePart::Text { text },
                        message.role,
                        &mut messages,
                    ),
                    MessageContent::Thinking { .. } => {}
                    MessageContent::RedactedThinking(_) => {}
                    MessageContent::Image(image) => {
                        add_message_content_part(
                            lmstudio::MessagePart::Image {
                                image_url: lmstudio::ImageUrl {
                                    url: image.to_base64_url(),
                                    detail: None,
                                },
                            },
                            message.role,
                            &mut messages,
                        );
                    }
                    MessageContent::ToolUse(tool_use) => {
                        let tool_call = lmstudio::ToolCall {
                            id: tool_use.id.to_string(),
                            content: lmstudio::ToolCallContent::Function {
                                function: lmstudio::FunctionContent {
                                    name: tool_use.name.to_string(),
                                    arguments: serde_json::to_string(&tool_use.input)
                                        .unwrap_or_default(),
                                },
                            },
                        };

                        if let Some(lmstudio::ChatMessage::Assistant { tool_calls, .. }) =
                            messages.last_mut()
                        {
                            tool_calls.push(tool_call);
                        } else {
                            messages.push(lmstudio::ChatMessage::Assistant {
                                content: None,
                                tool_calls: vec![tool_call],
                            });
                        }
                    }
                    MessageContent::ToolResult(tool_result) => {
                        let content = match &tool_result.content {
                            LanguageModelToolResultContent::Text(text) => {
                                vec![lmstudio::MessagePart::Text {
                                    text: text.to_string(),
                                }]
                            }
                            LanguageModelToolResultContent::Image(image) => {
                                vec![lmstudio::MessagePart::Image {
                                    image_url: lmstudio::ImageUrl {
                                        url: image.to_base64_url(),
                                        detail: None,
                                    },
                                }]
                            }
                        };

                        messages.push(lmstudio::ChatMessage::Tool {
                            content: content.into(),
                            tool_call_id: tool_result.tool_use_id.to_string(),
                        });
                    }
                }
            }
        }

        lmstudio::ChatCompletionRequest {
            model: self.model.name.clone(),
            messages,
            stream: true,
            max_tokens: Some(-1),
            stop: Some(request.stop),
            // In LM Studio you can configure specific settings you'd like to use for your model.
            // For example Qwen3 is recommended to be used with 0.7 temperature.
            // It would be a bad UX to silently override these settings from Zed, so we pass no temperature as a default.
            temperature: request.temperature.or(None),
            tools: request
                .tools
                .into_iter()
                .map(|tool| lmstudio::ToolDefinition::Function {
                    function: lmstudio::FunctionDefinition {
                        name: tool.name,
                        description: Some(tool.description),
                        parameters: Some(tool.input_schema),
                    },
                })
                .collect(),
            tool_choice: request.tool_choice.map(|choice| match choice {
                LanguageModelToolChoice::Auto => lmstudio::ToolChoice::Auto,
                LanguageModelToolChoice::Any => lmstudio::ToolChoice::Required,
                LanguageModelToolChoice::None => lmstudio::ToolChoice::None,
            }),
        }
    }

    fn stream_completion(
        &self,
        request: lmstudio::ChatCompletionRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<lmstudio::ResponseStreamEvent>>>,
    > {
        let http_client = self.http_client.clone();
        let (api_key, api_url) = self.state.read_with(cx, |state, cx| {
            let api_url = LmStudioLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        });

        let future = self.request_limiter.stream(async move {
            let stream = lmstudio::stream_chat_completion(
                http_client.as_ref(),
                &api_url,
                api_key.as_deref(),
                request,
            )
            .await?;
            Ok(stream)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for LmStudioLanguageModel {
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

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        self.supports_tools()
            && match choice {
                LanguageModelToolChoice::Auto => true,
                LanguageModelToolChoice::Any => true,
                LanguageModelToolChoice::None => true,
            }
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images
    }

    fn telemetry_id(&self) -> String {
        format!("lmstudio/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        _cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        // Endpoint for this is coming soon. In the meantime, hacky estimation
        let token_count = request
            .messages
            .iter()
            .map(|msg| msg.string_contents().split_whitespace().count())
            .sum::<usize>();

        let estimated_tokens = (token_count as f64 * 0.75) as u64;
        async move { Ok(estimated_tokens) }.boxed()
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
        let request = self.to_lmstudio_request(request);
        let completions = self.stream_completion(request, cx);
        async move {
            let mapper = LmStudioEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

struct LmStudioEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
}

impl LmStudioEventMapper {
    fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<lmstudio::ResponseStreamEvent>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(LanguageModelCompletionError::from(error))],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: lmstudio::ResponseStreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let Some(choice) = event.choices.into_iter().next() else {
            return vec![Err(LanguageModelCompletionError::from(anyhow!(
                "Response contained no choices"
            )))];
        };

        let mut events = Vec::new();
        if let Some(content) = choice.delta.content {
            events.push(Ok(LanguageModelCompletionEvent::Text(content)));
        }

        if let Some(reasoning_content) = choice.delta.reasoning_content {
            events.push(Ok(LanguageModelCompletionEvent::Thinking {
                text: reasoning_content,
                signature: None,
            }));
        }

        if let Some(tool_calls) = choice.delta.tool_calls {
            for tool_call in tool_calls {
                let entry = self.tool_calls_by_index.entry(tool_call.index).or_default();

                if let Some(tool_id) = tool_call.id {
                    entry.id = tool_id;
                }

                if let Some(function) = tool_call.function {
                    if let Some(name) = function.name {
                        // At the time of writing this code LM Studio (0.3.15) is incompatible with the OpenAI API:
                        // 1. It sends function name in the first chunk
                        // 2. It sends empty string in the function name field in all subsequent chunks for arguments
                        // According to https://platform.openai.com/docs/guides/function-calling?api-mode=responses#streaming
                        // function name field should be sent only inside the first chunk.
                        if !name.is_empty() {
                            entry.name = name;
                        }
                    }

                    if let Some(arguments) = function.arguments {
                        entry.arguments.push_str(&arguments);
                    }
                }
            }
        }

        if let Some(usage) = event.usage {
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens: usage.prompt_tokens,
                output_tokens: usage.completion_tokens,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            })));
        }

        match choice.finish_reason.as_deref() {
            Some("stop") => {
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            Some("tool_calls") => {
                events.extend(self.tool_calls_by_index.drain().map(|(_, tool_call)| {
                    match serde_json::Value::from_str(&tool_call.arguments) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_call.id.into(),
                                name: tool_call.name.into(),
                                is_input_complete: true,
                                input,
                                raw_input: tool_call.arguments,
                                thought_signature: None,
                            },
                        )),
                        Err(error) => Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                            id: tool_call.id.into(),
                            tool_name: tool_call.name.into(),
                            raw_input: tool_call.arguments.into(),
                            json_parse_error: error.to_string(),
                        }),
                    }
                }));

                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
            }
            Some(stop_reason) => {
                log::error!("Unexpected LMStudio stop_reason: {stop_reason:?}",);
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
}

fn add_message_content_part(
    new_part: lmstudio::MessagePart,
    role: Role,
    messages: &mut Vec<lmstudio::ChatMessage>,
) {
    match (role, messages.last_mut()) {
        (Role::User, Some(lmstudio::ChatMessage::User { content }))
        | (
            Role::Assistant,
            Some(lmstudio::ChatMessage::Assistant {
                content: Some(content),
                ..
            }),
        )
        | (Role::System, Some(lmstudio::ChatMessage::System { content })) => {
            content.push_part(new_part);
        }
        _ => {
            messages.push(match role {
                Role::User => lmstudio::ChatMessage::User {
                    content: lmstudio::MessageContent::from(vec![new_part]),
                },
                Role::Assistant => lmstudio::ChatMessage::Assistant {
                    content: Some(lmstudio::MessageContent::from(vec![new_part])),
                    tool_calls: Vec::new(),
                },
                Role::System => lmstudio::ChatMessage::System {
                    content: lmstudio::MessageContent::from(vec![new_part]),
                },
            });
        }
    }
}

struct ConfigurationView {
    state: Entity<State>,
    api_key_editor: Entity<InputField>,
    api_url_editor: Entity<InputField>,
}

impl ConfigurationView {
    pub fn new(state: Entity<State>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| InputField::new(_window, cx, "sk-...").label("API key"));

        let api_url_editor = cx.new(|cx| {
            let input = InputField::new(_window, cx, LMSTUDIO_API_URL).label("API URL");
            input.set_text(&LmStudioLanguageModelProvider::api_url(cx), _window, cx);
            input
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        Self {
            state,
            api_key_editor,
            api_url_editor,
        }
    }

    fn retry_connection(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let has_api_url = LmStudioLanguageModelProvider::has_custom_url(cx);
        let has_api_key = self
            .state
            .read_with(cx, |state, _| state.api_key_state.has_key());
        if !has_api_url {
            self.save_api_url(cx);
        }
        if !has_api_key {
            self.save_api_key(&Default::default(), _window, cx);
        }

        self.state.update(cx, |state, cx| {
            state.restart_fetch_models_task(cx);
        });
    }

    fn save_api_key(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", _window, cx));

        let state = self.state.clone();
        cx.spawn_in(_window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", _window, cx));

        let state = self.state.clone();
        cx.spawn_in(_window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn save_api_url(&self, cx: &mut Context<Self>) {
        let api_url = self.api_url_editor.read(cx).text(cx).trim().to_string();
        let current_url = LmStudioLanguageModelProvider::api_url(cx);
        if !api_url.is_empty() && &api_url != &current_url {
            self.state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .detach_and_log_err(cx);

            let fs = <dyn Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                settings
                    .language_models
                    .get_or_insert_default()
                    .lmstudio
                    .get_or_insert_default()
                    .api_url = Some(api_url);
            });
        }
    }

    fn reset_api_url(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.api_url_editor
            .update(cx, |input, cx| input.set_text("", _window, cx));

        // Clear API key when URL changes since keys are URL-specific
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
            .detach_and_log_err(cx);

        let fs = <dyn Fs>::global(cx);
        update_settings_file(fs, cx, |settings, _cx| {
            if let Some(settings) = settings
                .language_models
                .as_mut()
                .and_then(|models| models.lmstudio.as_mut())
            {
                settings.api_url = Some(LMSTUDIO_API_URL.into());
            }
        });
        cx.notify();
    }

    fn render_api_url_editor(&self, cx: &Context<Self>) -> impl IntoElement {
        let api_url = LmStudioLanguageModelProvider::api_url(cx);
        let custom_api_url_set = api_url != LMSTUDIO_API_URL;

        if custom_api_url_set {
            h_flex()
                .p_3()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().elevated_surface_background)
                .child(
                    h_flex()
                        .gap_2()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(v_flex().gap_1().child(Label::new(api_url))),
                )
                .child(
                    Button::new("reset-api-url", "Reset API URL")
                        .label_size(LabelSize::Small)
                        .icon(IconName::Undo)
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .layer(ElevationIndex::ModalSurface)
                        .on_click(
                            cx.listener(|this, _, _window, cx| this.reset_api_url(_window, cx)),
                        ),
                )
                .into_any_element()
        } else {
            v_flex()
                .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| {
                    this.save_api_url(cx);
                    cx.notify();
                }))
                .gap_2()
                .child(self.api_url_editor.clone())
                .into_any_element()
        }
    }

    fn render_api_key_editor(&self, cx: &Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable.")
        } else {
            "API key configured".to_string()
        };

        if !state.api_key_state.has_key() {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(format!(
                        "You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, _window, cx| this.reset_api_key(_window, cx)))
                .when(env_var_set, |this| {
                    this.tooltip_label(format!(
                        "To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."
                    ))
                })
                .into_any_element()
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();

        v_flex()
            .gap_2()
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new("Run local LLMs like Llama, Phi, and Qwen."))
                    .child(
                        List::new()
                            .child(ListBulletItem::new(
                                "LM Studio needs to be running with at least one model downloaded.",
                            ))
                            .child(
                                ListBulletItem::new("")
                                    .child(Label::new("To get your first model, try running"))
                                    .child(Label::new("lms get qwen2.5-coder-7b").inline_code(cx)),
                            ),
                    ),
            )
            .child(self.render_api_url_editor(cx))
            .child(self.render_api_key_editor(cx))
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .gap_2()
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2()
                            .map(|this| {
                                if is_authenticated {
                                    this.child(
                                        Button::new("lmstudio-site", "LM Studio")
                                            .style(ButtonStyle::Subtle)
                                            .icon(IconName::ArrowUpRight)
                                            .icon_size(IconSize::Small)
                                            .icon_color(Color::Muted)
                                            .on_click(move |_, _window, cx| {
                                                cx.open_url(LMSTUDIO_SITE)
                                            })
                                            .into_any_element(),
                                    )
                                } else {
                                    this.child(
                                        Button::new(
                                            "download_lmstudio_button",
                                            "Download LM Studio",
                                        )
                                        .style(ButtonStyle::Subtle)
                                        .icon(IconName::ArrowUpRight)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .on_click(move |_, _window, cx| {
                                            cx.open_url(LMSTUDIO_DOWNLOAD_URL)
                                        })
                                        .into_any_element(),
                                    )
                                }
                            })
                            .child(
                                Button::new("view-models", "Model Catalog")
                                    .style(ButtonStyle::Subtle)
                                    .icon(IconName::ArrowUpRight)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .on_click(move |_, _window, cx| {
                                        cx.open_url(LMSTUDIO_CATALOG_URL)
                                    }),
                            ),
                    )
                    .map(|this| {
                        if is_authenticated {
                            this.child(
                                ButtonLike::new("connected")
                                    .disabled(true)
                                    .cursor_style(CursorStyle::Arrow)
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(Icon::new(IconName::Check).color(Color::Success))
                                            .child(Label::new("Connected"))
                                            .into_any_element(),
                                    )
                                    .child(
                                        IconButton::new("refresh-models", IconName::RotateCcw)
                                            .tooltip(Tooltip::text("Refresh Models"))
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.state.update(cx, |state, _| {
                                                    state.available_models.clear();
                                                });
                                                this.retry_connection(_window, cx);
                                            })),
                                    ),
                            )
                        } else {
                            this.child(
                                Button::new("retry_lmstudio_models", "Connect")
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::XSmall)
                                    .icon(IconName::PlayFilled)
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.retry_connection(_window, cx)
                                    })),
                            )
                        }
                    }),
            )
    }
}
