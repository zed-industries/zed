use anyhow::{anyhow, Context as _, Result};
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use editor::{Editor, EditorElement, EditorStyle};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{
    AnyView, AppContext as _, AsyncApp, Entity, FontStyle, Subscription, Task, TextStyle,
    WhiteSpace,
};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionEvent, LanguageModelId,
    LanguageModelName, LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, RateLimiter, Role,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{prelude::*, Icon, IconName};
use util::ResultExt;

use crate::AllLanguageModelSettings;

const PROVIDER_ID: &str = "deepseek";
const PROVIDER_NAME: &str = "DeepSeek";
const DEEPSEEK_API_KEY_VAR: &str = "DEEPSEEK_API_KEY";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct DeepSeekSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: usize,
    pub max_output_tokens: Option<u32>,
}

pub struct DeepSeekLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key: Option<String>,
    api_key_from_env: bool,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn reset_api_key(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .deepseek
            .api_url
            .clone();
        cx.spawn(|this, mut cx| async move {
            credentials_provider
                .delete_credentials(&api_url, &cx)
                .await
                .log_err();
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
            .deepseek
            .api_url
            .clone();
        cx.spawn(|this, mut cx| async move {
            credentials_provider
                .write_credentials(&api_url, "Bearer", api_key.as_bytes(), &cx)
                .await?;
            this.update(&mut cx, |this, cx| {
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
            .deepseek
            .api_url
            .clone();
        cx.spawn(|this, mut cx| async move {
            let (api_key, from_env) = if let Ok(api_key) = std::env::var(DEEPSEEK_API_KEY_VAR) {
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

impl DeepSeekLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            api_key: None,
            api_key_from_env: false,
            _subscription: cx.observe_global::<SettingsStore>(|_this: &mut State, cx| {
                cx.notify();
            }),
        });

        Self { http_client, state }
    }
}

impl LanguageModelProviderState for DeepSeekLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for DeepSeekLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiDeepSeek
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        models.insert("deepseek-chat", deepseek::Model::Chat);
        models.insert("deepseek-reasoner", deepseek::Model::Reasoner);

        for available_model in AllLanguageModelSettings::get_global(cx)
            .deepseek
            .available_models
            .iter()
        {
            models.insert(
                &available_model.name,
                deepseek::Model::Custom {
                    name: available_model.name.clone(),
                    display_name: available_model.display_name.clone(),
                    max_tokens: available_model.max_tokens,
                    max_output_tokens: available_model.max_output_tokens,
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(DeepSeekLanguageModel {
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

pub struct DeepSeekLanguageModel {
    id: LanguageModelId,
    model: deepseek::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl DeepSeekLanguageModel {
    fn stream_completion(
        &self,
        request: deepseek::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<deepseek::StreamResponse>>>> {
        let http_client = self.http_client.clone();
        let Ok((api_key, api_url)) = cx.read_entity(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).deepseek;
            (state.api_key.clone(), settings.api_url.clone())
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let future = self.request_limiter.stream(async move {
            let api_key = api_key.ok_or_else(|| anyhow!("Missing DeepSeek API Key"))?;
            let request =
                deepseek::stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for DeepSeekLanguageModel {
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
        format!("deepseek/{}", self.model.id())
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

            tiktoken_rs::num_tokens_from_messages("gpt-4", &messages)
        })
        .boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<LanguageModelCompletionEvent>>>> {
        let request = request.into_deepseek(self.model.id().to_string(), self.max_output_tokens());
        let stream = self.stream_completion(request, cx);

        async move {
            let stream = stream.await?;
            Ok(stream
                .map(|result| {
                    result.and_then(|response| {
                        response
                            .choices
                            .first()
                            .ok_or_else(|| anyhow!("Empty response"))
                            .map(|choice| {
                                choice
                                    .delta
                                    .content
                                    .clone()
                                    .unwrap_or_default()
                                    .map(LanguageModelCompletionEvent::Text)
                            })
                    })
                })
                .boxed())
        }
        .boxed()
    }

    fn use_any_tool(
        &self,
        request: LanguageModelRequest,
        name: String,
        description: String,
        schema: serde_json::Value,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<String>>>> {
        let mut deepseek_request =
            request.into_deepseek(self.model.id().to_string(), self.max_output_tokens());

        deepseek_request.tools = vec![deepseek::ToolDefinition::Function {
            function: deepseek::FunctionDefinition {
                name: name.clone(),
                description: Some(description),
                parameters: Some(schema),
            },
        }];

        let response_stream = self.stream_completion(deepseek_request, cx);

        self.request_limiter
            .run(async move {
                let stream = response_stream.await?;

                let tool_args_stream = stream
                    .filter_map(move |response| async move {
                        match response {
                            Ok(response) => {
                                for choice in response.choices {
                                    if let Some(tool_calls) = choice.delta.tool_calls {
                                        for tool_call in tool_calls {
                                            if let Some(function) = tool_call.function {
                                                if let Some(args) = function.arguments {
                                                    return Some(Ok(args));
                                                }
                                            }
                                        }
                                    }
                                }
                                None
                            }
                            Err(e) => Some(Err(e)),
                        }
                    })
                    .boxed();

                Ok(tool_args_stream)
            })
            .boxed()
    }
}

struct ConfigurationView {
    api_key_editor: Entity<Editor>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("sk-00000000000000000000000000000000", cx);
            editor
        });

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
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        let state = self.state.clone();
        cx.spawn(|_, mut cx| async move {
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
        cx.spawn(|_, mut cx| async move {
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
            background_color: None,
            underline: None,
            strikethrough: None,
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const DEEPSEEK_CONSOLE_URL: &str = "https://platform.deepseek.com/api_keys";
        const INSTRUCTIONS: [&str; 3] = [
            "To use DeepSeek in Zed, you need an API key:",
            "- Get your API key from:",
            "- Paste it below and press enter:",
        ];

        let env_var_set = self.state.read(cx).api_key_from_env;

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials...")).into_any()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(INSTRUCTIONS[0]))
                .child(
                    h_flex().child(Label::new(INSTRUCTIONS[1])).child(
                        Button::new("deepseek_console", DEEPSEEK_CONSOLE_URL)
                            .style(ButtonStyle::Subtle)
                            .icon(IconName::ArrowUpRight)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Muted)
                            .on_click(move |_, _window, cx| cx.open_url(DEEPSEEK_CONSOLE_URL)),
                    ),
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
                    Label::new(format!(
                        "Or set the {} environment variable.",
                        DEEPSEEK_API_KEY_VAR
                    ))
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
                            format!("API key set in {}", DEEPSEEK_API_KEY_VAR)
                        } else {
                            "API key configured".to_string()
                        })),
                )
                .child(
                    Button::new("reset-key", "Reset")
                        .icon(IconName::Trash)
                        .disabled(env_var_set)
                        .on_click(
                            cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)),
                        ),
                )
                .into_any()
        }
    }
}
