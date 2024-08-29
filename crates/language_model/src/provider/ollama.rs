use anyhow::{anyhow, bail, Result};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AnyView, AppContext, AsyncAppContext, ModelContext, Subscription, Task};
use http_client::HttpClient;
use ollama::{
    get_models, preload_model, stream_chat_completion, ChatMessage, ChatOptions, ChatRequest,
    ChatResponseDelta, OllamaToolCall,
};
use settings::{Settings, SettingsStore};
use std::{sync::Arc, time::Duration};
use ui::{prelude::*, ButtonLike, Indicator};
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
}

pub struct OllamaLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Model<State>,
}

pub struct State {
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<ollama::Model>,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        !self.available_models.is_empty()
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
                // Since there is no metadata from the Ollama API
                // indicating which models are embedding models,
                // simply filter out models with "-embed" in their name
                .filter(|model| !model.name.contains("-embed"))
                .map(|model| ollama::Model::new(&model.name))
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
            self.fetch_models(cx)
        }
    }
}

impl OllamaLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut AppContext) -> Self {
        let this = Self {
            http_client: http_client.clone(),
            state: cx.new_model(|cx| State {
                http_client,
                available_models: Default::default(),
                _subscription: cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                    this.fetch_models(cx).detach();
                    cx.notify();
                }),
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
        self.state
            .read(cx)
            .available_models
            .iter()
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

        Self {
            state,
            loading_models_task,
        }
    }

    fn retry_connection(&self, cx: &mut WindowContext) {
        self.state
            .update(cx, |state, cx| state.fetch_models(cx))
            .detach_and_log_err(cx);
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();

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
