use anyhow::{anyhow, bail, Result};
use editor::{Editor, EditorElement, EditorStyle};
use fs::Fs;
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{div, AnyView, AppContext, AsyncAppContext, ModelContext, Subscription, Task, View};
use http_client::HttpClient;
use ollama::{
    get_models, preload_model, stream_chat_completion, ChatMessage, ChatOptions, ChatRequest,
    ChatResponseDelta, OllamaToolCall, OLLAMA_API_URL_DEFAULT, OLLAMA_API_URL_VAR,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{update_settings_file, Settings, SettingsStore};
use std::{collections::BTreeMap, sync::Arc, time::Duration};
use ui::{prelude::*, Button, ButtonLike, Color, IconName, Indicator, Tooltip};
use util::ResultExt;

use crate::{
    settings::AllLanguageModelSettings, LanguageModel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, RateLimiter, Role,
};

const OLLAMA_DOWNLOAD_URL: &str = "https://ollama.com/download";
const OLLAMA_LIBRARY_URL: &str = "https://ollama.com/library";
const OLLAMA_SITE: &str = "https://ollama.com/";

const PROVIDER_ID: &str = "ollama";
const PROVIDER_NAME: &str = "Ollama";

#[derive(Default, Debug, Clone, PartialEq)]
pub struct OllamaSettings {
    pub api_url: String,
    pub low_speed_timeout: Option<Duration>,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    /// The model name in the Ollama API (e.g. "llama3.1:latest")
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the assistant panel.
    pub display_name: Option<String>,
    /// The Context Length parameter to the model (aka num_ctx or n_ctx)
    pub max_tokens: usize,
}

pub struct OllamaLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Model<State>,
}

pub struct State {
    api_url: Option<String>,
    api_url_from_env: bool,
    api_key: Option<String>,
    api_key_from_env: bool,
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<ollama::Model>,
    _subscription: Subscription,
    fs: Arc<dyn Fs>,
}
const OLLAMA_API_KEY_VAR: &'static str = "OLLAMA_API_KEY";

impl State {
    fn is_authenticated(&self) -> bool {
        !self.available_models.is_empty()
    }

    fn reset_api_key(&self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let delete_credentials =
            cx.delete_credentials(&AllLanguageModelSettings::get_global(cx).ollama.api_url);
        cx.spawn(|this, mut cx| async move {
            delete_credentials.await.ok();
            this.update(&mut cx, |this, cx| {
                this.api_key = None;
                this.api_key_from_env = false;
                cx.notify();
            })
        })
    }

    #[allow(dead_code)]
    fn get_api_url(&self, cx: &mut ModelContext<Self>) -> String {
        if let Some(api_url) = &self.api_url {
            return api_url.clone();
        }
        let settings = AllLanguageModelSettings::get_global(cx);
        if !settings.ollama.api_url.is_empty() {
            return settings.ollama.api_url.clone();
        }
        OLLAMA_API_URL_DEFAULT.to_string()
    }

    // Common function to update the API URL
    fn update_api_url(
        &mut self,
        api_url: Option<String>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let api_url_clone = api_url.clone();
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            let _ = cx.update(|cx| {
                update_settings_file::<AllLanguageModelSettings>(fs, cx, move |settings, _| {
                    if let Some(ollama_settings) = &mut settings.ollama {
                        ollama_settings.api_url = api_url_clone;
                    }
                })
            });
            this.update(&mut cx, |this, _| {
                this.api_url = api_url;
            })?;
            Ok(())
        })
    }

    fn reset_api_url(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        self.api_url_from_env = false;
        self.update_api_url(Some(OLLAMA_API_URL_DEFAULT.to_string()), cx)
    }

    fn set_api_url(&mut self, api_url: String, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        self.update_api_url(Some(api_url), cx)
    }

    fn set_api_key(&mut self, api_key: String, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let settings = &AllLanguageModelSettings::get_global(cx).ollama;
        let write_credentials =
            cx.write_credentials(&settings.api_url, "Bearer", api_key.as_bytes());

        cx.spawn(|this, mut cx| async move {
            write_credentials.await?;
            this.update(&mut cx, |this, cx| {
                this.api_key = Some(api_key);
                cx.notify();
            })
        })
    }

    fn fetch_models(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let settings = &AllLanguageModelSettings::get_global(cx).ollama;
        let http_client = self.http_client.clone();
        let api_url = settings.api_url.clone();

        // As a proxy for the server being "authenticated", we'll check if its up by fetching the models
        cx.spawn(|this, mut cx| async move {
            let models = get_models(http_client.as_ref(), &api_url, None).await?;

            let mut models: Vec<ollama::Model> = models
                .into_iter()
                .filter(|model| !model.name.contains("-embed"))
                .map(|model| ollama::Model::new(&model.name, None, None))
                .collect();

            models.sort_by(|a, b| a.name.cmp(&b.name));

            this.update(&mut cx, |this, cx| {
                this.available_models = models;
                cx.notify();
            })
        })
    }

    fn authenticate(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if self.is_authenticated() {
            Task::ready(Ok(()))
        } else {
            cx.spawn(|this, mut cx| async move {
                let (api_key, from_env) = if let Ok(api_key) = std::env::var(OLLAMA_API_KEY_VAR) {
                    (Some(api_key), true)
                } else {
                    (None, false)
                };

                this.update(&mut cx, |this, cx| {
                    this.api_key = api_key;
                    this.api_key_from_env = from_env;
                    if from_env {
                        this.fetch_models(cx).detach();
                    }
                })
                .log_err();

                this.update(&mut cx, |this, cx| this.fetch_models(cx))?.await
            })
        }
    }
}

impl OllamaLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, fs: Arc<dyn Fs>, cx: &mut AppContext) -> Self {
        let settings = AllLanguageModelSettings::get_global(cx).ollama.clone();
        let this = Self {
            http_client: http_client.clone(),
            state: cx.new_model(|cx| State {
                api_url: Some(settings.api_url.clone()),
                api_url_from_env: false,
                api_key: None,
                api_key_from_env: false,
                http_client,
                available_models: Default::default(),
                _subscription: cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                    this.fetch_models(cx).detach();
                    cx.notify();
                }),
                fs,
            }),
        };
        this.state
            .update(cx, |state, cx| state.fetch_models(cx).detach());
        this
    }
}

impl LanguageModelProviderState for OllamaLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Model<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OllamaLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiOllama
    }

    fn provided_models(&self, cx: &AppContext) -> Vec<Arc<dyn LanguageModel>> {
        let mut models: BTreeMap<String, ollama::Model> = BTreeMap::default();

        // Add models from the Ollama API
        for model in self.state.read(cx).available_models.iter() {
            models.insert(model.name.clone(), model.clone());
        }

        // Override with available models from settings
        for model in AllLanguageModelSettings::get_global(cx)
            .ollama
            .available_models
            .iter()
        {
            models.insert(
                model.name.clone(),
                ollama::Model {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    keep_alive: None,
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(OllamaLanguageModel {
                    id: LanguageModelId::from(model.name.clone()),
                    model: model.clone(),
                    http_client: self.http_client.clone(),
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn load_model(&self, model: Arc<dyn LanguageModel>, cx: &AppContext) {
        let settings = &AllLanguageModelSettings::get_global(cx).ollama;
        let http_client = self.http_client.clone();
        let api_url = settings.api_url.clone();
        let id = model.id().0.to_string();
        cx.spawn(|_| async move { preload_model(http_client, &api_url, &id).await })
            .detach_and_log_err(cx);
    }

    fn is_authenticated(&self, cx: &AppContext) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut AppContext) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(&self, cx: &mut WindowContext) -> AnyView {
        let state = self.state.clone();
        cx.new_view(|cx| ConfigurationView::new(state, cx)).into()
    }

    fn reset_credentials(&self, cx: &mut AppContext) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.fetch_models(cx))
    }
}

pub struct OllamaLanguageModel {
    id: LanguageModelId,
    model: ollama::Model,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OllamaLanguageModel {
    fn to_ollama_request(&self, request: LanguageModelRequest) -> ChatRequest {
        ChatRequest {
            model: self.model.name.clone(),
            messages: request
                .messages
                .into_iter()
                .map(|msg| match msg.role {
                    Role::User => ChatMessage::User {
                        content: msg.string_contents(),
                    },
                    Role::Assistant => ChatMessage::Assistant {
                        content: msg.string_contents(),
                        tool_calls: None,
                    },
                    Role::System => ChatMessage::System {
                        content: msg.string_contents(),
                    },
                })
                .collect(),
            keep_alive: self.model.keep_alive.clone().unwrap_or_default(),
            stream: true,
            options: Some(ChatOptions {
                num_ctx: Some(self.model.max_tokens),
                stop: Some(request.stop),
                temperature: Some(request.temperature),
                ..Default::default()
            }),
            tools: vec![],
        }
    }
    fn request_completion(
        &self,
        request: ChatRequest,
        cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<ChatResponseDelta>> {
        let http_client = self.http_client.clone();

        let Ok(api_url) = cx.update(|cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).ollama;
            settings.api_url.clone()
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        async move { ollama::complete(http_client.as_ref(), &api_url, request).await }.boxed()
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
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn telemetry_id(&self) -> String {
        format!("ollama/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        _cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        // There is no endpoint for this _yet_ in Ollama
        // see: https://github.com/ollama/ollama/issues/1716 and https://github.com/ollama/ollama/issues/3582
        let token_count = request
            .messages
            .iter()
            .map(|msg| msg.string_contents().chars().count())
            .sum::<usize>()
            / 4;

        async move { Ok(token_count) }.boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let request = self.to_ollama_request(request);

        let http_client = self.http_client.clone();
        let Ok((api_url, low_speed_timeout)) = cx.update(|cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).ollama;
            (settings.api_url.clone(), settings.low_speed_timeout)
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let future = self.request_limiter.stream(async move {
            let response =
                stream_chat_completion(http_client.as_ref(), &api_url, request, low_speed_timeout)
                    .await?;
            let stream = response
                .filter_map(|response| async move {
                    match response {
                        Ok(delta) => {
                            let content = match delta.message {
                                ChatMessage::User { content } => content,
                                ChatMessage::Assistant { content, .. } => content,
                                ChatMessage::System { content } => content,
                            };
                            Some(Ok(content))
                        }
                        Err(error) => Some(Err(error)),
                    }
                })
                .boxed();
            Ok(stream)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn use_any_tool(
        &self,
        request: LanguageModelRequest,
        tool_name: String,
        tool_description: String,
        schema: serde_json::Value,
        cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        use ollama::{OllamaFunctionTool, OllamaTool};
        let function = OllamaFunctionTool {
            name: tool_name.clone(),
            description: Some(tool_description),
            parameters: Some(schema),
        };
        let tools = vec![OllamaTool::Function { function }];
        let request = self.to_ollama_request(request).with_tools(tools);
        let response = self.request_completion(request, cx);
        self.request_limiter
            .run(async move {
                let response = response.await?;
                let ChatMessage::Assistant { tool_calls, .. } = response.message else {
                    bail!("message does not have an assistant role");
                };
                if let Some(tool_calls) = tool_calls.filter(|calls| !calls.is_empty()) {
                    for call in tool_calls {
                        let OllamaToolCall::Function(function) = call;
                        if function.name == tool_name {
                            return Ok(futures::stream::once(async move {
                                Ok(function.arguments.to_string())
                            })
                            .boxed());
                        }
                    }
                } else {
                    bail!("assistant message does not have any tool calls");
                };

                bail!("tool not used")
            })
            .boxed()
    }
}

struct ConfigurationView {
    api_url_editor: View<Editor>,
    api_key_editor: View<Editor>,
    state: gpui::Model<State>,
    loading_models_task: Option<Task<()>>,
}

impl ConfigurationView {
    pub fn new(state: gpui::Model<State>, cx: &mut ViewContext<Self>) -> Self {
        let loading_models_task = Some(cx.spawn({
            let state = state.clone();
            |this, mut cx| async move {
                if let Some(task) = state
                    .update(&mut cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    task.await.log_err();
                }
                this.update(&mut cx, |this, cx| {
                    this.loading_models_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        // Load the API URL from settings
        let api_url = state.read(cx).api_url.clone().unwrap_or_default();

        Self {
            api_url_editor: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text(
                    if std::env::var(OLLAMA_API_URL_VAR).is_ok() {
                        format!("API URL set by {} environment variable", OLLAMA_API_URL_VAR)
                    } else {
                        OLLAMA_API_URL_DEFAULT.to_string()
                    },
                    cx,
                );
                editor.set_text(api_url, cx); // Set the loaded API URL
                editor
            }),
            api_key_editor: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text(
                    if std::env::var(OLLAMA_API_KEY_VAR).is_ok() {
                        format!("API key set by {} environment variable", OLLAMA_API_KEY_VAR)
                    } else {
                        "Enter optional API key here".to_string()
                    },
                    cx,
                );
                editor
            }),
            state,
            loading_models_task,
        }
    }

    fn save_api_url(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let api_url = self.api_url_editor.read(cx).text(cx);
        if api_url.is_empty() {
            return;
        }

        let state = self.state.clone();
        cx.spawn(|_, mut cx| async move {
            state
                .update(&mut cx, |state, cx| state.set_api_url(api_url, cx))?
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn save_api_key(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
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
    }

    fn reset_api_key(&self, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, cx| {
            state.reset_api_key(cx).detach_and_log_err(cx);
        });

        // Clear the API key editor
        self.api_key_editor.update(cx, |editor, cx| {
            editor.set_text("", cx);
        });

        cx.notify();
    }

    fn reset_api_url(&self, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, cx| {
            state.reset_api_url(cx).detach_and_log_err(cx);
        });
        self.api_url_editor.update(cx, |editor, cx| {
            editor.set_text(OLLAMA_API_URL_DEFAULT, cx);
        });
        cx.notify();
    }

    fn retry_connection(&self, cx: &mut WindowContext) {
        self.state
            .update(cx, |state, cx| state.fetch_models(cx))
            .detach_and_log_err(cx);
    }

    fn render_api_url_editor(&self) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(Label::new("API URL"))
            .child(h_flex().w_full().child(EditorElement::new(
                &self.api_url_editor,
                EditorStyle::default(),
            )))
    }

    fn render_api_key_editor(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let editor_style = EditorStyle::default();

        // Create the editor element
        let editor_element = EditorElement::new(&self.api_key_editor, editor_style);

        // Update the text of the Editor directly
        self.api_key_editor.update(cx, |editor, cx| {
            let api_key_text = editor.text(cx);
            editor.set_text(api_key_text.clone(), cx); // Show the actual API key
        });

        // Return the label and editor in a vertical layout
        v_flex().size_full().child(Label::new("API Key")).child(
            h_flex()
                .gap(ui::Pixels(8.0))
                .child(h_flex().w_full().child(editor_element)),
        )
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();
        let api_url_from_env = self.state.read(cx).api_url_from_env;
        let api_key_from_env = self.state.read(cx).api_key_from_env;

        let ollama_intro = "Get up and running with Llama 3.1, Mistral, Gemma 2, and other large language models with Ollama.";
        let ollama_reqs =
            "Ollama must be running with at least one model installed to use it in the assistant.";

        let mut inline_code_bg = cx.theme().colors().editor_background;
        inline_code_bg.fade_out(0.5);

        if self.loading_models_task.is_some() {
            div().child(Label::new("Loading models...")).into_any()
        } else {
            v_flex()
                .size_full()
                .gap_3()
                .child(
                    v_flex()
                        .size_full()
                        .gap_2()
                        .p_1()
                        .child(Label::new(ollama_intro))
                        .child(Label::new(ollama_reqs))
                        .child(
                            h_flex()
                                .gap_0p5()
                                .child(Label::new("Once installed, try "))
                                .child(
                                    div()
                                        .bg(inline_code_bg)
                                        .px_1p5()
                                        .rounded_md()
                                        .child(Label::new("ollama run llama3.1")),
                                ),
                        ),
                )
                .gap_4()
                .child(h_flex().gap_1().child(self.render_api_url_editor()).when(
                    !api_url_from_env,
                    |flex| {
                        flex.children(vec![
                            IconButton::new("save-api-url", IconName::Save)
                                .on_click(
                                    cx.listener(|this, _, cx| {
                                        this.save_api_url(&menu::Confirm, cx)
                                    }),
                                )
                                .tooltip(|cx| Tooltip::text("Save the custom API URL", cx)),
                            IconButton::new("reset-api-url", IconName::RotateCcw)
                                .on_click(cx.listener(|this, _, cx| this.reset_api_url(cx)))
                                .tooltip(|cx| Tooltip::text("Reset to the default API URL", cx)),
                        ])
                    },
                ))
                .child(h_flex().gap_1().child(self.render_api_key_editor(cx)).when(
                    !api_key_from_env,
                    |flex| {
                        flex.children(vec![
                            IconButton::new("save-api-key", IconName::Save)
                                .on_click(
                                    cx.listener(|this, _, cx| {
                                        this.save_api_key(&menu::Confirm, cx)
                                    }),
                                )
                                .tooltip(|cx| Tooltip::text("Save the custom API key", cx)),
                            IconButton::new("reset-api-key", IconName::RotateCcw)
                                .on_click(cx.listener(|this, _, cx| this.reset_api_key(cx)))
                                .tooltip(|cx| Tooltip::text("Remove the API key", cx)),
                        ])
                    },
                ))
                .child(
                    h_flex()
                        .w_full()
                        .pt_2()
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
                                                .icon(IconName::ExternalLink)
                                                .icon_size(IconSize::XSmall)
                                                .icon_color(Color::Muted)
                                                .on_click(move |_, cx| cx.open_url(OLLAMA_SITE))
                                                .into_any_element(),
                                        )
                                    } else {
                                        this.child(
                                            Button::new(
                                                "download_ollama_button",
                                                "Download Ollama",
                                            )
                                            .style(ButtonStyle::Subtle)
                                            .icon(IconName::ExternalLink)
                                            .icon_size(IconSize::XSmall)
                                            .icon_color(Color::Muted)
                                            .on_click(move |_, cx| cx.open_url(OLLAMA_DOWNLOAD_URL))
                                            .into_any_element(),
                                        )
                                    }
                                })
                                .child(
                                    Button::new("view-models", "All Models")
                                        .style(ButtonStyle::Subtle)
                                        .icon(IconName::ExternalLink)
                                        .icon_size(IconSize::XSmall)
                                        .icon_color(Color::Muted)
                                        .on_click(move |_, cx| cx.open_url(OLLAMA_LIBRARY_URL)),
                                ),
                        )
                        .child(if is_authenticated {
                            // This is only a button to ensure the spacing is correct
                            // it should stay disabled
                            ButtonLike::new("connected")
                                .disabled(true)
                                // Since this won't ever be clickable, we can use the arrow cursor
                                .cursor_style(gpui::CursorStyle::Arrow)
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(Indicator::dot().color(Color::Success))
                                        .child(Label::new("Connected"))
                                        .into_any_element(),
                                )
                                .into_any_element()
                        } else {
                            Button::new("retry_ollama_models", "Connect")
                                .icon_position(IconPosition::Start)
                                .icon(IconName::ArrowCircle)
                                .on_click(cx.listener(move |this, _, cx| this.retry_connection(cx)))
                                .into_any_element()
                        }),
                )
                .into_any()
        }
    }
}
