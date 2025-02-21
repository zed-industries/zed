use crate::AllLanguageModelSettings;
use anthropic::{AnthropicError, ContentDelta, Event, ResponseContent};
use anyhow::{anyhow, Context as _, Result};
use collections::{BTreeMap, HashMap};
use credentials_provider::CredentialsProvider;
use editor::{Editor, EditorElement, EditorStyle};
use futures::Stream;
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt, TryStreamExt as _};
use gpui::{
    AnyView, App, AsyncApp, Context, Entity, FontStyle, Subscription, Task, TextStyle, WhiteSpace,
};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCacheConfiguration, LanguageModelId,
    LanguageModelName, LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, RateLimiter, Role,
};
use language_model::{LanguageModelCompletionEvent, LanguageModelToolUse, StopReason};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use strum::IntoEnumIterator;
use theme::ThemeSettings;
use ui::{prelude::*, Icon, IconName, Tooltip};
use util::{maybe, ResultExt};

pub const PROVIDER_ID: &str = "anthropic";
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
    pub max_tokens: usize,
    /// A model `name` to substitute when calling tools, in case the primary model doesn't support tool calling.
    pub tool_override: Option<String>,
    /// Configuration of Anthropic's caching API.
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
    pub max_output_tokens: Option<u32>,
    pub default_temperature: Option<f32>,
    #[serde(default)]
    pub extra_beta_headers: Vec<String>,
}

pub struct AnthropicLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

const ANTHROPIC_API_KEY_VAR: &str = "ANTHROPIC_API_KEY";

pub struct State {
    api_key: Option<String>,
    api_key_from_env: bool,
    _subscription: Subscription,
}

impl State {
    fn reset_api_key(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .anthropic
            .api_url
            .clone();
        cx.spawn(|this, mut cx| async move {
            credentials_provider
                .delete_credentials(&api_url, &cx)
                .await
                .ok();
            this.update(&mut cx, |this, cx| {
                this.api_key = None;
                this.api_key_from_env = false;
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
        cx.spawn(|this, mut cx| async move {
            credentials_provider
                .write_credentials(&api_url, "Bearer", api_key.as_bytes(), &cx)
                .await
                .ok();

            this.update(&mut cx, |this, cx| {
                this.api_key = Some(api_key);
                cx.notify();
            })
        })
    }

    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .anthropic
            .api_url
            .clone();

        cx.spawn(|this, mut cx| async move {
            let (api_key, from_env) = if let Ok(api_key) = std::env::var(ANTHROPIC_API_KEY_VAR) {
                (api_key, true)
            } else {
                let (_, api_key) = credentials_provider
                    .read_credentials(&api_url, &cx)
                    .await?
                    .ok_or(AuthenticateError::CredentialsNotFound)?;
                (
                    String::from_utf8(api_key).context("invalid {PROVIDER_NAME} API key")?,
                    false,
                )
            };

            this.update(&mut cx, |this, cx| {
                this.api_key = Some(api_key);
                this.api_key_from_env = from_env;
                cx.notify();
            })?;

            Ok(())
        })
    }
}

impl AnthropicLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            api_key: None,
            api_key_from_env: false,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        Self { http_client, state }
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
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(AnthropicModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    state: self.state.clone(),
                    http_client: self.http_client.clone(),
                    request_limiter: RateLimiter::new(4),
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

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.reset_api_key(cx))
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
) -> BoxFuture<'static, Result<usize>> {
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
                    MessageContent::Image(image) => {
                        tokens_from_images += image.estimate_tokens();
                    }
                    MessageContent::ToolUse(_tool_use) => {
                        // TODO: Estimate token usage from tool uses.
                    }
                    MessageContent::ToolResult(tool_result) => {
                        string_contents.push_str(&tool_result.content);
                    }
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
            .map(|tokens| tokens + tokens_from_images)
    })
    .boxed()
}

impl AnthropicModel {
    fn stream_completion(
        &self,
        request: anthropic::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<anthropic::Event, AnthropicError>>>>
    {
        let http_client = self.http_client.clone();

        let Ok((api_key, api_url)) = cx.read_entity(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).anthropic;
            (state.api_key.clone(), settings.api_url.clone())
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        async move {
            let api_key = api_key.ok_or_else(|| anyhow!("Missing Anthropic API Key"))?;
            let request =
                anthropic::stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            request.await.context("failed to stream completion")
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

    fn telemetry_id(&self) -> String {
        format!("anthropic/{}", self.model.id())
    }

    fn api_key(&self, cx: &App) -> Option<String> {
        self.state.read(cx).api_key.clone()
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u32> {
        Some(self.model.max_output_tokens())
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        count_anthropic_tokens(request, cx)
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<LanguageModelCompletionEvent>>>> {
        let request = request.into_anthropic(
            self.model.id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
        );
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.await.map_err(|err| anyhow!(err))?;
            Ok(map_to_language_model_completion_events(response))
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

    fn use_any_tool(
        &self,
        request: LanguageModelRequest,
        tool_name: String,
        tool_description: String,
        input_schema: serde_json::Value,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let mut request = request.into_anthropic(
            self.model.tool_model_id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
        );
        request.tool_choice = Some(anthropic::ToolChoice::Tool {
            name: tool_name.clone(),
        });
        request.tools = vec![anthropic::Tool {
            name: tool_name.clone(),
            description: tool_description,
            input_schema,
        }];

        let response = self.stream_completion(request, cx);
        self.request_limiter
            .run(async move {
                let response = response.await?;
                Ok(anthropic::extract_tool_args_from_events(
                    tool_name,
                    Box::pin(response.map_err(|e| anyhow!(e))),
                )
                .await?
                .boxed())
            })
            .boxed()
    }
}

pub fn map_to_language_model_completion_events(
    events: Pin<Box<dyn Send + Stream<Item = Result<Event, AnthropicError>>>>,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent>> {
    struct RawToolUse {
        id: String,
        name: String,
        input_json: String,
    }

    struct State {
        events: Pin<Box<dyn Send + Stream<Item = Result<Event, AnthropicError>>>>,
        tool_uses_by_index: HashMap<usize, RawToolUse>,
    }

    futures::stream::unfold(
        State {
            events,
            tool_uses_by_index: HashMap::default(),
        },
        |mut state| async move {
            while let Some(event) = state.events.next().await {
                match event {
                    Ok(event) => match event {
                        Event::ContentBlockStart {
                            index,
                            content_block,
                        } => match content_block {
                            ResponseContent::Text { text } => {
                                return Some((
                                    Some(Ok(LanguageModelCompletionEvent::Text(text))),
                                    state,
                                ));
                            }
                            ResponseContent::ToolUse { id, name, .. } => {
                                state.tool_uses_by_index.insert(
                                    index,
                                    RawToolUse {
                                        id,
                                        name,
                                        input_json: String::new(),
                                    },
                                );

                                return Some((None, state));
                            }
                        },
                        Event::ContentBlockDelta { index, delta } => match delta {
                            ContentDelta::TextDelta { text } => {
                                return Some((
                                    Some(Ok(LanguageModelCompletionEvent::Text(text))),
                                    state,
                                ));
                            }
                            ContentDelta::InputJsonDelta { partial_json } => {
                                if let Some(tool_use) = state.tool_uses_by_index.get_mut(&index) {
                                    tool_use.input_json.push_str(&partial_json);
                                    return Some((None, state));
                                }
                            }
                        },
                        Event::ContentBlockStop { index } => {
                            if let Some(tool_use) = state.tool_uses_by_index.remove(&index) {
                                return Some((
                                    Some(maybe!({
                                        Ok(LanguageModelCompletionEvent::ToolUse(
                                            LanguageModelToolUse {
                                                id: tool_use.id.into(),
                                                name: tool_use.name,
                                                input: if tool_use.input_json.is_empty() {
                                                    serde_json::Value::Null
                                                } else {
                                                    serde_json::Value::from_str(
                                                        &tool_use.input_json,
                                                    )
                                                    .map_err(|err| anyhow!(err))?
                                                },
                                            },
                                        ))
                                    })),
                                    state,
                                ));
                            }
                        }
                        Event::MessageStart { message } => {
                            return Some((
                                Some(Ok(LanguageModelCompletionEvent::StartMessage {
                                    message_id: message.id,
                                })),
                                state,
                            ))
                        }
                        Event::MessageDelta { delta, .. } => {
                            if let Some(stop_reason) = delta.stop_reason.as_deref() {
                                let stop_reason = match stop_reason {
                                    "end_turn" => StopReason::EndTurn,
                                    "max_tokens" => StopReason::MaxTokens,
                                    "tool_use" => StopReason::ToolUse,
                                    _ => StopReason::EndTurn,
                                };

                                return Some((
                                    Some(Ok(LanguageModelCompletionEvent::Stop(stop_reason))),
                                    state,
                                ));
                            }
                        }
                        Event::Error { error } => {
                            return Some((
                                Some(Err(anyhow!(AnthropicError::ApiError(error)))),
                                state,
                            ));
                        }
                        _ => {}
                    },
                    Err(err) => {
                        return Some((Some(Err(anyhow!(err))), state));
                    }
                }
            }

            None
        },
    )
    .filter_map(|event| async move { event })
}

struct ConfigurationView {
    api_key_editor: Entity<Editor>,
    state: gpui::Entity<State>,
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
            |this, mut cx| async move {
                if let Some(task) = state
                    .update(&mut cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    // We don't log an error, because "not signed in" is also an error.
                    let _ = task.await;
                }
                this.update(&mut cx, |this, cx| {
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
        cx.spawn_in(window, |_, mut cx| async move {
            state
                .update(&mut cx, |state, cx| state.set_api_key(api_key, cx))?
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, |_, mut cx| async move {
            state
                .update(&mut cx, |state, cx| state.reset_api_key(cx))?
                .await
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
        const ANTHROPIC_CONSOLE_URL: &str = "https://console.anthropic.com/settings/keys";
        const INSTRUCTIONS: [&str; 3] = [
            "To use Zed's assistant with Anthropic, you need to add an API key. Follow these steps:",
            "- Create one at:",
            "- Paste your API key below and hit enter to use the assistant:",
        ];
        let env_var_set = self.state.read(cx).api_key_from_env;

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials...")).into_any()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(INSTRUCTIONS[0]))
                .child(h_flex().child(Label::new(INSTRUCTIONS[1])).child(
                    Button::new("anthropic_console", ANTHROPIC_CONSOLE_URL)
                        .style(ButtonStyle::Subtle)
                        .icon(IconName::ArrowUpRight)
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .on_click(move |_, _, cx| cx.open_url(ANTHROPIC_CONSOLE_URL))
                    )
                )
                .child(Label::new(INSTRUCTIONS[2]))
                .child(
                    h_flex()
                        .w_full()
                        .my_2()
                        .px_2()
                        .py_1()
                        .bg(cx.theme().colors().editor_background)
                        .border_1()
                        .border_color(cx.theme().colors().border_variant)
                        .rounded_md()
                        .child(self.render_api_key_editor(cx)),
                )
                .child(
                    Label::new(
                        format!("You can also assign the {ANTHROPIC_API_KEY_VAR} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small),
                )
                .into_any()
        } else {
            h_flex()
                .size_full()
                .justify_between()
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(if env_var_set {
                            format!("API key set in {ANTHROPIC_API_KEY_VAR} environment variable.")
                        } else {
                            "API key configured.".to_string()
                        })),
                )
                .child(
                    Button::new("reset-key", "Reset key")
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
