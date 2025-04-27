use anyhow::{Context as _, Result, anyhow};
use collections::{BTreeMap, HashMap};
use credentials_provider::CredentialsProvider;
use editor::{Editor, EditorElement, EditorStyle};
use futures::{FutureExt, Stream, StreamExt, future::BoxFuture};
use gpui::{
    AnyView, App, AsyncApp, Context, Entity, FontStyle, Subscription, Task, TextStyle, WhiteSpace,
};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolUse, MessageContent, RateLimiter, Role, StopReason,
};
use open_router::{Model, ResponseStreamEvent, get_models, stream_completion};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::pin::Pin;
use std::str::FromStr as _;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{Icon, IconName, List, Tooltip, prelude::*};
use util::ResultExt;

use crate::{AllLanguageModelSettings, ui::InstructionListItem};

const PROVIDER_ID: &str = "openrouter";
const PROVIDER_NAME: &str = "OpenRouter";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenRouterSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: usize,
    pub max_output_tokens: Option<u32>,
    pub max_completion_tokens: Option<u32>,
}

pub struct OpenRouterLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

pub struct State {
    api_key: Option<String>,
    api_key_from_env: bool,
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<open_router::Model>,
    fetch_models_task: Option<Task<Result<()>>>,
    _subscription: Subscription,
}

const OPENROUTER_API_KEY_VAR: &str = "OPENROUTER_API_KEY";

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn reset_api_key(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .open_router
            .api_url
            .clone();
        cx.spawn(async move |this, cx| {
            credentials_provider
                .delete_credentials(&api_url, &cx)
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.api_key = None;
                this.api_key_from_env = false;
                cx.notify();
            })
        })
    }

    fn set_api_key(&mut self, api_key: String, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .open_router
            .api_url
            .clone();
        cx.spawn(async move |this, cx| {
            credentials_provider
                .write_credentials(&api_url, "Bearer", api_key.as_bytes(), &cx)
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.api_key = Some(api_key);
                cx.notify();
            })
        })
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .open_router
            .api_url
            .clone();
        cx.spawn(async move |this, cx| {
            let (api_key, from_env) = if let Ok(api_key) = std::env::var(OPENROUTER_API_KEY_VAR) {
                (api_key, true)
            } else {
                let (_, api_key) = credentials_provider
                    .read_credentials(&api_url, &cx)
                    .await?
                    .ok_or(AuthenticateError::CredentialsNotFound)?;
                (
                    String::from_utf8(api_key)
                        .context(format!("invalid {} API key", PROVIDER_NAME))?,
                    false,
                )
            };
            this.update(cx, |this, cx| {
                this.api_key = Some(api_key);
                this.api_key_from_env = from_env;
                cx.notify();
            })?;

            Ok(())
        })
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let settings = &AllLanguageModelSettings::get_global(cx).open_router;
        let http_client = self.http_client.clone();
        let api_url = settings.api_url.clone();

        // As a proxy for the server being "authenticated", we'll check if its up by fetching the models
        cx.spawn(async move |this, cx| {
            let model_entries = get_models(http_client.as_ref(), &api_url).await?;

            // Convert ModelEntry objects to our Model objects
            let mut models: Vec<open_router::Model> = model_entries
                .into_iter()
                .map(|entry| {
                    let max_tokens = entry.context_length as usize;
                    open_router::Model::new(
                        &entry.id,
                        Some(&entry.name),
                        Some(max_tokens),
                        entry.supports_tool_calls,
                        entry.excels_at_coding,
                    )
                })
                .collect();

            // Sort models by name for better display
            models.sort_by(|a, b| a.name.cmp(&b.name));

            this.update(cx, |this, cx| {
                this.available_models = models;
                cx.notify();
            })
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        let task = self.fetch_models(cx);
        self.fetch_models_task.replace(task);
    }
}

impl OpenRouterLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            api_key: None,
            api_key_from_env: false,
            http_client: http_client.clone(),
            available_models: Vec::new(),
            fetch_models_task: None,
            _subscription: cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                // Restart the fetch_models task when settings change
                this.restart_fetch_models_task(cx);
                cx.notify();
            }),
        });

        Self { http_client, state }
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

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenRouterLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiOpenRouter
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_router::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_router::Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models: BTreeMap<String, open_router::Model> = BTreeMap::default();

        // Add models from the OpenRouter API
        for model in self.state.read(cx).available_models.iter() {
            models.insert(model.name.clone(), model.clone());
        }

        // Override with available models from settings
        for model in &AllLanguageModelSettings::get_global(cx)
            .open_router
            .available_models
        {
            models.insert(
                model.name.clone(),
                open_router::Model {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    supports_tool_calls: false,
                    excels_at_coding: false,
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
        self.state.update(cx, |state, cx| state.reset_api_key(cx))
    }
}

pub struct OpenRouterLanguageModel {
    id: LanguageModelId,
    model: open_router::Model,
    state: gpui::Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenRouterLanguageModel {
    fn stream_completion(
        &self,
        request: open_router::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>>>
    {
        let http_client = self.http_client.clone();
        let Ok((api_key, api_url)) = cx.read_entity(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).open_router;
            (state.api_key.clone(), settings.api_url.clone())
        }) else {
            return futures::future::ready(Err(anyhow!(
                "App state dropped: Unable to read API key or API URL from the application state"
            )))
            .boxed();
        };

        let future = self.request_limiter.stream(async move {
            let api_key = api_key.ok_or_else(|| anyhow!("Missing OpenRouter API Key"))?;
            let request = stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
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
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tool_calls()
    }

    fn telemetry_id(&self) -> String {
        format!("openrouter/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u32> {
        self.model.max_output_tokens()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        count_open_router_tokens(request, self.model.clone(), cx)
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
        >,
    > {
        let request = into_open_router(request, &self.model, self.max_output_tokens());
        let completions = self.stream_completion(request, cx);
        async move { Ok(map_to_language_model_completion_events(completions.await?).boxed()) }
            .boxed()
    }
}

pub fn into_open_router(
    request: LanguageModelRequest,
    model: &Model,
    max_output_tokens: Option<u32>,
) -> open_router::Request {
    let stream = !model.id().starts_with("meta-llama/") && !model.id().starts_with("google/");

    let mut messages = Vec::new();
    for req_message in request.messages {
        for content in req_message.content {
            match content {
                MessageContent::Text(text) | MessageContent::Thinking { text, .. } => messages
                    .push(match req_message.role {
                        Role::User => open_router::RequestMessage::User { content: text },
                        Role::Assistant => open_router::RequestMessage::Assistant {
                            content: Some(text),
                            tool_calls: Vec::new(),
                        },
                        Role::System => open_router::RequestMessage::System { content: text },
                    }),
                MessageContent::RedactedThinking(_) => {}
                MessageContent::Image(_) => {}
                MessageContent::ToolUse(tool_use) => {
                    let tool_call = open_router::ToolCall {
                        id: tool_use.id.to_string(),
                        content: open_router::ToolCallContent::Function {
                            function: open_router::FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                            },
                        },
                    };

                    if let Some(last_assistant_message) = messages.iter_mut().rfind(|message| {
                        matches!(message, open_router::RequestMessage::Assistant { .. })
                    }) {
                        if let open_router::RequestMessage::Assistant { tool_calls, .. } =
                            last_assistant_message
                        {
                            tool_calls.push(tool_call);
                        }
                    } else {
                        messages.push(open_router::RequestMessage::Assistant {
                            content: None,
                            tool_calls: vec![tool_call],
                        });
                    }
                }
                MessageContent::ToolResult(tool_result) => {
                    messages.push(open_router::RequestMessage::Tool {
                        content: tool_result.content.to_string(),
                        tool_call_id: tool_result.tool_use_id.to_string(),
                    });
                }
            }
        }
    }

    open_router::Request {
        model: model.id().into(),
        messages,
        stream,
        stop: request.stop,
        temperature: request.temperature.unwrap_or(1.0),
        max_tokens: max_output_tokens,
        parallel_tool_calls: if model.supports_parallel_tool_calls() && !request.tools.is_empty() {
            // Disable parallel tool calls, as the Agent currently expects a maximum of one per turn.
            Some(false)
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
        tool_choice: None,
        http_referer: Some("zed.dev".to_string()),
        http_user_agent: Some("Zed Editor".to_string()),
    }
}

pub fn map_to_language_model_completion_events(
    events: Pin<Box<dyn Send + Stream<Item = Result<ResponseStreamEvent>>>>,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
    #[derive(Default)]
    struct RawToolCall {
        id: String,
        name: String,
        arguments: String,
    }

    struct State {
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponseStreamEvent>>>>,
        tool_calls_by_index: HashMap<usize, RawToolCall>,
    }

    futures::stream::unfold(
        State {
            events,
            tool_calls_by_index: HashMap::default(),
        },
        |mut state| async move {
            if let Some(event) = state.events.next().await {
                match event {
                    Ok(event) => {
                        let Some(choice) = event.choices.first() else {
                            return Some((
                                vec![Err(LanguageModelCompletionError::Other(anyhow!(
                                    "Response contained no choices"
                                )))],
                                state,
                            ));
                        };

                        let mut events = Vec::new();
                        if let Some(content) = choice.delta.content.clone() {
                            events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                        }

                        if let Some(tool_calls) = choice.delta.tool_calls.as_ref() {
                            for tool_call in tool_calls {
                                let entry = state
                                    .tool_calls_by_index
                                    .entry(tool_call.index)
                                    .or_default();

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
                                }
                            }
                        }

                        match choice.finish_reason.as_deref() {
                            Some("stop") => {
                                events.push(Ok(LanguageModelCompletionEvent::Stop(
                                    StopReason::EndTurn,
                                )));
                            }
                            Some("tool_calls") => {
                                events.extend(state.tool_calls_by_index.drain().map(
                                    |(_, tool_call)| match serde_json::Value::from_str(
                                        &tool_call.arguments,
                                    ) {
                                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                                            LanguageModelToolUse {
                                                id: tool_call.id.clone().into(),
                                                name: tool_call.name.as_str().into(),
                                                is_input_complete: true,
                                                input,
                                                raw_input: tool_call.arguments.clone(),
                                            },
                                        )),
                                        Err(error) => {
                                            Err(LanguageModelCompletionError::BadInputJson {
                                                id: tool_call.id.into(),
                                                tool_name: tool_call.name.as_str().into(),
                                                raw_input: tool_call.arguments.into(),
                                                json_parse_error: error.to_string(),
                                            })
                                        }
                                    },
                                ));

                                events.push(Ok(LanguageModelCompletionEvent::Stop(
                                    StopReason::ToolUse,
                                )));
                            }
                            Some(stop_reason) => {
                                log::error!("Unexpected OpenRouter stop_reason: {stop_reason:?}",);
                                events.push(Ok(LanguageModelCompletionEvent::Stop(
                                    StopReason::EndTurn,
                                )));
                            }
                            None => {}
                        }

                        return Some((events, state));
                    }
                    Err(err) => {
                        return Some((vec![Err(LanguageModelCompletionError::Other(err))], state));
                    }
                }
            }

            None
        },
    )
    .flat_map(futures::stream::iter)
}

pub fn count_open_router_tokens(
    request: LanguageModelRequest,
    _model: open_router::Model,
    cx: &App,
) -> BoxFuture<'static, Result<usize>> {
    cx.background_spawn(async move {
        let messages = request
            .messages
            .into_iter()
            .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
                role: match message.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                    Role::System => "system".into(),
                },
                content: Some(message.string_contents()),
                name: None,
                function_call: None,
            })
            .collect::<Vec<_>>();

        // OpenRouter serves a variety of models, but we'll use o200k_base for token counting
        // This is the encoding used by modern OpenAI models like GPT-4o
        tiktoken_rs::num_tokens_from_messages("o200k_base", &messages)
    })
    .boxed()
}

struct ConfigurationView {
    api_key_editor: Entity<Editor>,
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor
                .set_placeholder_text("sk_or_000000000000000000000000000000000000000000000000", cx);
            editor
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
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
            api_key_editor,
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
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state.update(cx, |state, cx| state.reset_api_key(cx))?.await
        })
        .detach_and_log_err(cx);

        cx.notify();
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
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_from_env;

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials...")).into_any()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use Zed's assistant with OpenRouter, you need to add an API key. Follow these steps:"))
                .child(
                    List::new()
                        .child(InstructionListItem::new(
                            "Create an API key by visiting",
                            Some("OpenRouter's console"),
                            Some("https://openrouter.ai/keys"),
                        ))
                        .child(InstructionListItem::text_only(
                            "Ensure your OpenRouter account has credits",
                        ))
                        .child(InstructionListItem::text_only(
                            "Paste your API key below and hit enter to start using the assistant",
                        )),
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
                        format!("You can also assign the {OPENROUTER_API_KEY_VAR} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any()
        } else {
            h_flex()
                .mt_1()
                .p_1()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(if env_var_set {
                            format!("API key set in {OPENROUTER_API_KEY_VAR} environment variable.")
                        } else {
                            "API key configured.".to_string()
                        })),
                )
                .child(
                    Button::new("reset-key", "Reset Key")
                        .label_size(LabelSize::Small)
                        .icon(Some(IconName::Trash))
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .disabled(env_var_set)
                        .when(env_var_set, |this| {
                            this.tooltip(Tooltip::text(format!("To reset your API key, unset the {OPENROUTER_API_KEY_VAR} environment variable.")))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                )
                .into_any()
        }
    }
}
