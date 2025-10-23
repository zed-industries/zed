use anyhow::{Result, anyhow};
use fs::Fs;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use futures::{Stream, TryFutureExt, stream};
use gpui::{AnyView, App, AsyncApp, Context, CursorStyle, Entity, Task};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelRequestTool, LanguageModelToolChoice, LanguageModelToolUse,
    LanguageModelToolUseId, MessageContent, RateLimiter, Role, StopReason, TokenUsage,
};
use menu;
use ollama::{
    ChatMessage, ChatOptions, ChatRequest, ChatResponseDelta, OLLAMA_API_URL, OllamaFunctionCall,
    OllamaFunctionTool, OllamaToolCall, get_models, show_model, stream_chat_completion,
};
pub use settings::OllamaAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore, update_settings_file};
use std::pin::Pin;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{collections::HashMap, sync::Arc};
use ui::{ButtonLike, ElevationIndex, List, Tooltip, prelude::*};
use ui_input::InputField;
use zed_env_vars::{EnvVar, env_var};

use crate::AllLanguageModelSettings;
use crate::api_key::ApiKeyState;
use crate::ui::InstructionListItem;

const OLLAMA_DOWNLOAD_URL: &str = "https://ollama.com/download";
const OLLAMA_LIBRARY_URL: &str = "https://ollama.com/library";
const OLLAMA_SITE: &str = "https://ollama.com/";

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("ollama");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Ollama");

const API_KEY_ENV_VAR_NAME: &str = "OLLAMA_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Debug, Clone, PartialEq)]
pub struct OllamaSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct OllamaLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    http_client: Arc<dyn HttpClient>,
    fetched_models: Vec<ollama::Model>,
    fetch_model_task: Option<Task<Result<()>>>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        !self.fetched_models.is_empty()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = OllamaLanguageModelProvider::api_url(cx);
        let task = self
            .api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx);

        self.fetched_models.clear();
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = OllamaLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.load_if_needed(
            api_url,
            &API_KEY_ENV_VAR,
            |this| &mut this.api_key_state,
            cx,
        );

        // Always try to fetch models - if no API key is needed (local Ollama), it will work
        // If API key is needed and provided, it will work
        // If API key is needed and not provided, it will fail gracefully
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let http_client = Arc::clone(&self.http_client);
        let api_url = OllamaLanguageModelProvider::api_url(cx);
        let api_key = self.api_key_state.key(&api_url);

        // As a proxy for the server being "authenticated", we'll check if its up by fetching the models
        cx.spawn(async move |this, cx| {
            let models = get_models(http_client.as_ref(), &api_url, api_key.as_deref()).await?;

            let tasks = models
                .into_iter()
                // Since there is no metadata from the Ollama API
                // indicating which models are embedding models,
                // simply filter out models with "-embed" in their name
                .filter(|model| !model.name.contains("-embed"))
                .map(|model| {
                    let http_client = Arc::clone(&http_client);
                    let api_url = api_url.clone();
                    let api_key = api_key.clone();
                    async move {
                        let name = model.name.as_str();
                        let model =
                            show_model(http_client.as_ref(), &api_url, api_key.as_deref(), name)
                                .await?;
                        let ollama_model = ollama::Model::new(
                            name,
                            None,
                            model.context_length,
                            Some(model.supports_tools()),
                            Some(model.supports_vision()),
                            Some(model.supports_thinking()),
                        );
                        Ok(ollama_model)
                    }
                });

            // Rate-limit capability fetches
            // since there is an arbitrary number of models available
            let mut ollama_models: Vec<_> = futures::stream::iter(tasks)
                .buffer_unordered(5)
                .collect::<Vec<Result<_>>>()
                .await
                .into_iter()
                .collect::<Result<Vec<_>>>()?;

            ollama_models.sort_by(|a, b| a.name.cmp(&b.name));

            this.update(cx, |this, cx| {
                this.fetched_models = ollama_models;
                cx.notify();
            })
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        let task = self.fetch_models(cx);
        self.fetch_model_task.replace(task);
    }
}

impl OllamaLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let this = Self {
            http_client: http_client.clone(),
            state: cx.new(|cx| {
                cx.observe_global::<SettingsStore>({
                    let mut last_settings = OllamaLanguageModelProvider::settings(cx).clone();
                    move |this: &mut State, cx| {
                        let current_settings = OllamaLanguageModelProvider::settings(cx);
                        let settings_changed = current_settings != &last_settings;
                        if settings_changed {
                            let url_changed = last_settings.api_url != current_settings.api_url;
                            last_settings = current_settings.clone();
                            if url_changed {
                                this.fetched_models.clear();
                                this.authenticate(cx).detach();
                            }
                            cx.notify();
                        }
                    }
                })
                .detach();

                State {
                    http_client,
                    fetched_models: Default::default(),
                    fetch_model_task: None,
                    api_key_state: ApiKeyState::new(Self::api_url(cx)),
                }
            }),
        };
        this
    }

    fn settings(cx: &App) -> &OllamaSettings {
        &AllLanguageModelSettings::get_global(cx).ollama
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            OLLAMA_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for OllamaLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OllamaLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconName {
        IconName::AiOllama
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
        let mut models: HashMap<String, ollama::Model> = HashMap::new();

        // Add models from the Ollama API
        for model in self.state.read(cx).fetched_models.iter() {
            models.insert(model.name.clone(), model.clone());
        }

        // Override with available models from settings
        for setting_model in &OllamaLanguageModelProvider::settings(cx).available_models {
            let setting_base = setting_model.name.split(':').next().unwrap();
            if let Some(model) = models
                .values_mut()
                .find(|m| m.name.split(':').next().unwrap() == setting_base)
            {
                model.max_tokens = setting_model.max_tokens;
                model.display_name = setting_model.display_name.clone();
                model.keep_alive = setting_model.keep_alive.clone();
                model.supports_tools = setting_model.supports_tools;
                model.supports_vision = setting_model.supports_images;
                model.supports_thinking = setting_model.supports_thinking;
            } else {
                models.insert(
                    setting_model.name.clone(),
                    ollama::Model {
                        name: setting_model.name.clone(),
                        display_name: setting_model.display_name.clone(),
                        max_tokens: setting_model.max_tokens,
                        keep_alive: setting_model.keep_alive.clone(),
                        supports_tools: setting_model.supports_tools,
                        supports_vision: setting_model.supports_images,
                        supports_thinking: setting_model.supports_thinking,
                    },
                );
            }
        }

        let mut models = models
            .into_values()
            .map(|model| {
                Arc::new(OllamaLanguageModel {
                    id: LanguageModelId::from(model.name.clone()),
                    model,
                    http_client: self.http_client.clone(),
                    request_limiter: RateLimiter::new(4),
                    state: self.state.clone(),
                }) as Arc<dyn LanguageModel>
            })
            .collect::<Vec<_>>();
        models.sort_by_key(|model| model.name());
        models
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
        let state = self.state.clone();
        cx.new(|cx| ConfigurationView::new(state, window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct OllamaLanguageModel {
    id: LanguageModelId,
    model: ollama::Model,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
    state: Entity<State>,
}

impl OllamaLanguageModel {
    fn to_ollama_request(&self, request: LanguageModelRequest) -> ChatRequest {
        let supports_vision = self.model.supports_vision.unwrap_or(false);

        let mut messages = Vec::with_capacity(request.messages.len());

        for mut msg in request.messages.into_iter() {
            let images = if supports_vision {
                msg.content
                    .iter()
                    .filter_map(|content| match content {
                        MessageContent::Image(image) => Some(image.source.to_string()),
                        _ => None,
                    })
                    .collect::<Vec<String>>()
            } else {
                vec![]
            };

            match msg.role {
                Role::User => {
                    for tool_result in msg
                        .content
                        .extract_if(.., |x| matches!(x, MessageContent::ToolResult(..)))
                    {
                        match tool_result {
                            MessageContent::ToolResult(tool_result) => {
                                messages.push(ChatMessage::Tool {
                                    tool_name: tool_result.tool_name.to_string(),
                                    content: tool_result.content.to_str().unwrap_or("").to_string(),
                                })
                            }
                            _ => unreachable!("Only tool result should be extracted"),
                        }
                    }
                    if !msg.content.is_empty() {
                        messages.push(ChatMessage::User {
                            content: msg.string_contents(),
                            images: if images.is_empty() {
                                None
                            } else {
                                Some(images)
                            },
                        })
                    }
                }
                Role::Assistant => {
                    let content = msg.string_contents();
                    let mut thinking = None;
                    let mut tool_calls = Vec::new();
                    for content in msg.content.into_iter() {
                        match content {
                            MessageContent::Thinking { text, .. } if !text.is_empty() => {
                                thinking = Some(text)
                            }
                            MessageContent::ToolUse(tool_use) => {
                                tool_calls.push(OllamaToolCall::Function(OllamaFunctionCall {
                                    name: tool_use.name.to_string(),
                                    arguments: tool_use.input,
                                }));
                            }
                            _ => (),
                        }
                    }
                    messages.push(ChatMessage::Assistant {
                        content,
                        tool_calls: Some(tool_calls),
                        images: if images.is_empty() {
                            None
                        } else {
                            Some(images)
                        },
                        thinking,
                    })
                }
                Role::System => messages.push(ChatMessage::System {
                    content: msg.string_contents(),
                }),
            }
        }
        ChatRequest {
            model: self.model.name.clone(),
            messages,
            keep_alive: self.model.keep_alive.clone().unwrap_or_default(),
            stream: true,
            options: Some(ChatOptions {
                num_ctx: Some(self.model.max_tokens),
                stop: Some(request.stop),
                temperature: request.temperature.or(Some(1.0)),
                ..Default::default()
            }),
            think: self
                .model
                .supports_thinking
                .map(|supports_thinking| supports_thinking && request.thinking_allowed),
            tools: if self.model.supports_tools.unwrap_or(false) {
                request.tools.into_iter().map(tool_into_ollama).collect()
            } else {
                vec![]
            },
        }
    }
}

impl LanguageModel for OllamaLanguageModel {
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
        self.model.supports_tools.unwrap_or(false)
    }

    fn supports_images(&self) -> bool {
        self.model.supports_vision.unwrap_or(false)
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => false,
            LanguageModelToolChoice::Any => false,
            LanguageModelToolChoice::None => false,
        }
    }

    fn telemetry_id(&self) -> String {
        format!("ollama/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        _cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        // There is no endpoint for this _yet_ in Ollama
        // see: https://github.com/ollama/ollama/issues/1716 and https://github.com/ollama/ollama/issues/3582
        let token_count = request
            .messages
            .iter()
            .map(|msg| msg.string_contents().chars().count())
            .sum::<usize>()
            / 4;

        async move { Ok(token_count as u64) }.boxed()
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
        let request = self.to_ollama_request(request);

        let http_client = self.http_client.clone();
        let Ok((api_key, api_url)) = self.state.read_with(cx, |state, cx| {
            let api_url = OllamaLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped").into())).boxed();
        };

        let future = self.request_limiter.stream(async move {
            let stream =
                stream_chat_completion(http_client.as_ref(), &api_url, api_key.as_deref(), request)
                    .await?;
            let stream = map_to_language_model_completion_events(stream);
            Ok(stream)
        });

        future.map_ok(|f| f.boxed()).boxed()
    }
}

fn map_to_language_model_completion_events(
    stream: Pin<Box<dyn Stream<Item = anyhow::Result<ChatResponseDelta>> + Send>>,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
    // Used for creating unique tool use ids
    static TOOL_CALL_COUNTER: AtomicU64 = AtomicU64::new(0);

    struct State {
        stream: Pin<Box<dyn Stream<Item = anyhow::Result<ChatResponseDelta>> + Send>>,
        used_tools: bool,
    }

    // We need to create a ToolUse and Stop event from a single
    // response from the original stream
    let stream = stream::unfold(
        State {
            stream,
            used_tools: false,
        },
        async move |mut state| {
            let response = state.stream.next().await?;

            let delta = match response {
                Ok(delta) => delta,
                Err(e) => {
                    let event = Err(LanguageModelCompletionError::from(anyhow!(e)));
                    return Some((vec![event], state));
                }
            };

            let mut events = Vec::new();

            match delta.message {
                ChatMessage::User { content, images: _ } => {
                    events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                }
                ChatMessage::System { content } => {
                    events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                }
                ChatMessage::Tool { content, .. } => {
                    events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                }
                ChatMessage::Assistant {
                    content,
                    tool_calls,
                    images: _,
                    thinking,
                } => {
                    if let Some(text) = thinking {
                        events.push(Ok(LanguageModelCompletionEvent::Thinking {
                            text,
                            signature: None,
                        }));
                    }

                    if let Some(tool_call) = tool_calls.and_then(|v| v.into_iter().next()) {
                        match tool_call {
                            OllamaToolCall::Function(function) => {
                                let tool_id = format!(
                                    "{}-{}",
                                    &function.name,
                                    TOOL_CALL_COUNTER.fetch_add(1, Ordering::Relaxed)
                                );
                                let event =
                                    LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                                        id: LanguageModelToolUseId::from(tool_id),
                                        name: Arc::from(function.name),
                                        raw_input: function.arguments.to_string(),
                                        input: function.arguments,
                                        is_input_complete: true,
                                    });
                                events.push(Ok(event));
                                state.used_tools = true;
                            }
                        }
                    } else if !content.is_empty() {
                        events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                    }
                }
            };

            if delta.done {
                events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: delta.prompt_eval_count.unwrap_or(0),
                    output_tokens: delta.eval_count.unwrap_or(0),
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                })));
                if state.used_tools {
                    state.used_tools = false;
                    events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
                } else {
                    events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
                }
            }

            Some((events, state))
        },
    );

    stream.flat_map(futures::stream::iter)
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    api_url_editor: Entity<InputField>,
    state: Entity<State>,
}

impl ConfigurationView {
    pub fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| InputField::new(window, cx, "63e02e...").label("API key"));

        let api_url_editor = cx.new(|cx| {
            let input = InputField::new(window, cx, OLLAMA_API_URL).label("API URL");
            input.set_text(OllamaLanguageModelProvider::api_url(cx), window, cx);
            input
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        Self {
            api_key_editor,
            api_url_editor,
            state,
        }
    }

    fn retry_connection(&self, cx: &mut App) {
        self.state
            .update(cx, |state, cx| state.restart_fetch_models_task(cx));
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        // url changes can cause the editor to be displayed again
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))?
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn save_api_url(&mut self, cx: &mut Context<Self>) {
        let api_url = self.api_url_editor.read(cx).text(cx).trim().to_string();
        let current_url = OllamaLanguageModelProvider::api_url(cx);
        if !api_url.is_empty() && &api_url != &current_url {
            let fs = <dyn Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                settings
                    .language_models
                    .get_or_insert_default()
                    .ollama
                    .get_or_insert_default()
                    .api_url = Some(api_url);
            });
        }
    }

    fn reset_api_url(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_url_editor
            .update(cx, |input, cx| input.set_text("", window, cx));
        let fs = <dyn Fs>::global(cx);
        update_settings_file(fs, cx, |settings, _cx| {
            if let Some(settings) = settings
                .language_models
                .as_mut()
                .and_then(|models| models.ollama.as_mut())
            {
                settings.api_url = Some(OLLAMA_API_URL.into());
            }
        });
        cx.notify();
    }

    fn render_instructions() -> Div {
        v_flex()
            .gap_2()
            .child(Label::new(
                "Run LLMs locally on your machine with Ollama, or connect to an Ollama server. \
                Can provide access to Llama, Mistral, Gemma, and hundreds of other models.",
            ))
            .child(Label::new("To use local Ollama:"))
            .child(
                List::new()
                    .child(InstructionListItem::new(
                        "Download and install Ollama from",
                        Some("ollama.com"),
                        Some("https://ollama.com/download"),
                    ))
                    .child(InstructionListItem::text_only(
                        "Start Ollama and download a model: `ollama run gpt-oss:20b`",
                    ))
                    .child(InstructionListItem::text_only(
                        "Click 'Connect' below to start using Ollama in Zed",
                    )),
            )
            .child(Label::new(
                "Alternatively, you can connect to an Ollama server by specifying its \
                URL and API key (may not be required):",
            ))
    }

    fn render_api_key_editor(&self, cx: &Context<Self>) -> Div {
        let state = self.state.read(cx);
        let env_var_set = state.api_key_state.is_from_env_var();

        if !state.api_key_state.has_key() {
            v_flex()
              .on_action(cx.listener(Self::save_api_key))
              .child(self.api_key_editor.clone())
              .child(
                  Label::new(
                      format!("You can also assign the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed.")
                  )
                  .size(LabelSize::Small)
                  .color(Color::Muted),
              )
        } else {
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
                        .child(
                            Label::new(
                                if env_var_set {
                                    format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable.")
                                } else {
                                    "API key configured".to_string()
                                }
                            )
                        )
                )
                .child(
                    Button::new("reset-api-key", "Reset API Key")
                        .label_size(LabelSize::Small)
                        .icon(IconName::Undo)
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .layer(ElevationIndex::ModalSurface)
                        .when(env_var_set, |this| {
                            this.tooltip(Tooltip::text(format!("To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable.")))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                )
        }
    }

    fn render_api_url_editor(&self, cx: &Context<Self>) -> Div {
        let api_url = OllamaLanguageModelProvider::api_url(cx);
        let custom_api_url_set = api_url != OLLAMA_API_URL;

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
                            cx.listener(|this, _, window, cx| this.reset_api_url(window, cx)),
                        ),
                )
        } else {
            v_flex()
                .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| {
                    this.save_api_url(cx);
                    cx.notify();
                }))
                .gap_2()
                .child(self.api_url_editor.clone())
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();

        v_flex()
            .gap_2()
            .child(Self::render_instructions())
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
                                        Button::new("ollama-site", "Ollama")
                                            .style(ButtonStyle::Subtle)
                                            .icon(IconName::ArrowUpRight)
                                            .icon_size(IconSize::XSmall)
                                            .icon_color(Color::Muted)
                                            .on_click(move |_, _, cx| cx.open_url(OLLAMA_SITE))
                                            .into_any_element(),
                                    )
                                } else {
                                    this.child(
                                        Button::new("download_ollama_button", "Download Ollama")
                                            .style(ButtonStyle::Subtle)
                                            .icon(IconName::ArrowUpRight)
                                            .icon_size(IconSize::XSmall)
                                            .icon_color(Color::Muted)
                                            .on_click(move |_, _, cx| {
                                                cx.open_url(OLLAMA_DOWNLOAD_URL)
                                            })
                                            .into_any_element(),
                                    )
                                }
                            })
                            .child(
                                Button::new("view-models", "View All Models")
                                    .style(ButtonStyle::Subtle)
                                    .icon(IconName::ArrowUpRight)
                                    .icon_size(IconSize::XSmall)
                                    .icon_color(Color::Muted)
                                    .on_click(move |_, _, cx| cx.open_url(OLLAMA_LIBRARY_URL)),
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
                                    ),
                            )
                        } else {
                            this.child(
                                Button::new("retry_ollama_models", "Connect")
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::XSmall)
                                    .icon(IconName::PlayOutlined)
                                    .on_click(
                                        cx.listener(move |this, _, _, cx| {
                                            this.retry_connection(cx)
                                        }),
                                    ),
                            )
                        }
                    }),
            )
    }
}

fn tool_into_ollama(tool: LanguageModelRequestTool) -> ollama::OllamaTool {
    ollama::OllamaTool::Function {
        function: OllamaFunctionTool {
            name: tool.name,
            description: Some(tool.description),
            parameters: Some(tool.input_schema),
        },
    }
}
