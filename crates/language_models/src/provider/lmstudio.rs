use anyhow::{Result, anyhow};
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Subscription, Task};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModelCompletionError, LanguageModelCompletionEvent,
};
use language_model::{
    LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, RateLimiter, Role,
};
use lmstudio::{
    ChatCompletionRequest, ChatMessage, ModelType, get_models, preload_model,
    stream_chat_completion,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::{collections::BTreeMap, sync::Arc};
use ui::{ButtonLike, Indicator, List, prelude::*};
use util::ResultExt;

use crate::AllLanguageModelSettings;
use crate::ui::InstructionListItem;

const LMSTUDIO_DOWNLOAD_URL: &str = "https://lmstudio.ai/download";
const LMSTUDIO_CATALOG_URL: &str = "https://lmstudio.ai/models";
const LMSTUDIO_SITE: &str = "https://lmstudio.ai/";

const PROVIDER_ID: &str = "lmstudio";
const PROVIDER_NAME: &str = "LM Studio";

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LmStudioSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    /// The model name in the LM Studio API. e.g. qwen2.5-coder-7b, phi-4, etc
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the assistant panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: usize,
}

pub struct LmStudioLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

pub struct State {
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<lmstudio::Model>,
    fetch_model_task: Option<Task<Result<()>>>,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        !self.available_models.is_empty()
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
        let http_client = self.http_client.clone();
        let api_url = settings.api_url.clone();

        // As a proxy for the server being "authenticated", we'll check if its up by fetching the models
        cx.spawn(async move |this, cx| {
            let models = get_models(http_client.as_ref(), &api_url, None).await?;

            let mut models: Vec<lmstudio::Model> = models
                .into_iter()
                .filter(|model| model.r#type != ModelType::Embeddings)
                .map(|model| lmstudio::Model::new(&model.id, None, None))
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
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let fetch_models_task = self.fetch_models(cx);
        cx.spawn(async move |_this, _cx| Ok(fetch_models_task.await?))
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
}

impl LanguageModelProviderState for LmStudioLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for LmStudioLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiLmStudio
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.provided_models(cx).into_iter().next()
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.default_model(cx)
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
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(LmStudioLanguageModel {
                    id: LanguageModelId::from(model.name.clone()),
                    model: model.clone(),
                    http_client: self.http_client.clone(),
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn load_model(&self, model: Arc<dyn LanguageModel>, cx: &App) {
        let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
        let http_client = self.http_client.clone();
        let api_url = settings.api_url.clone();
        let id = model.id().0.to_string();
        cx.spawn(async move |_| preload_model(http_client, &api_url, &id).await)
            .detach_and_log_err(cx);
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(&self, _window: &mut Window, cx: &mut App) -> AnyView {
        let state = self.state.clone();
        cx.new(|cx| ConfigurationView::new(state, cx)).into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.fetch_models(cx))
    }
}

pub struct LmStudioLanguageModel {
    id: LanguageModelId,
    model: lmstudio::Model,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl LmStudioLanguageModel {
    fn to_lmstudio_request(&self, request: LanguageModelRequest) -> ChatCompletionRequest {
        ChatCompletionRequest {
            model: self.model.name.clone(),
            messages: request
                .messages
                .into_iter()
                .map(|msg| match msg.role {
                    Role::User => ChatMessage::User {
                        content: msg.string_contents(),
                    },
                    Role::Assistant => ChatMessage::Assistant {
                        content: Some(msg.string_contents()),
                        tool_calls: None,
                    },
                    Role::System => ChatMessage::System {
                        content: msg.string_contents(),
                    },
                })
                .collect(),
            stream: true,
            max_tokens: Some(-1),
            stop: Some(request.stop),
            temperature: request.temperature.or(Some(0.0)),
            tools: vec![],
        }
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
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn supports_tools(&self) -> bool {
        false
    }

    fn telemetry_id(&self) -> String {
        format!("lmstudio/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        _cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        // Endpoint for this is coming soon. In the meantime, hacky estimation
        let token_count = request
            .messages
            .iter()
            .map(|msg| msg.string_contents().split_whitespace().count())
            .sum::<usize>();

        let estimated_tokens = (token_count as f64 * 0.75) as usize;
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
        >,
    > {
        let request = self.to_lmstudio_request(request);

        let http_client = self.http_client.clone();
        let Ok(api_url) = cx.update(|cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
            settings.api_url.clone()
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let future = self.request_limiter.stream(async move {
            let response = stream_chat_completion(http_client.as_ref(), &api_url, request).await?;

            // Create a stream mapper to handle content across multiple deltas
            let stream_mapper = LmStudioStreamMapper::new();

            let stream = response
                .map(move |response| {
                    response.and_then(|fragment| stream_mapper.process_fragment(fragment))
                })
                .filter_map(|result| async move {
                    match result {
                        Ok(Some(content)) => Some(Ok(content)),
                        Ok(None) => None,
                        Err(error) => Some(Err(error)),
                    }
                })
                .boxed();

            Ok(stream)
        });

        async move {
            Ok(future
                .await?
                .map(|result| {
                    result
                        .map(LanguageModelCompletionEvent::Text)
                        .map_err(LanguageModelCompletionError::Other)
                })
                .boxed())
        }
        .boxed()
    }
}

struct LmStudioStreamMapper {
    current_role: Option<String>,
    content_buffer: String,
}

impl LmStudioStreamMapper {
    fn new() -> Self {
        Self {
            current_role: None,
            content_buffer: String::new(),
        }
    }

    fn process_fragment(&self, fragment: lmstudio::ChatResponse) -> Result<Option<String>> {
        // Most of the time, there will be only one choice
        let Some(choice) = fragment.choices.first() else {
            return Ok(None);
        };

        // Extract the delta content
        if let Ok(delta) =
            serde_json::from_value::<lmstudio::ResponseMessageDelta>(choice.delta.clone())
        {
            if let Some(content) = delta.content {
                if !content.is_empty() {
                    return Ok(Some(content));
                }
            }
        }

        // If there's a finish_reason, we're done
        if choice.finish_reason.is_some() {
            return Ok(None);
        }

        Ok(None)
    }
}

struct ConfigurationView {
    state: gpui::Entity<State>,
    loading_models_task: Option<Task<()>>,
}

impl ConfigurationView {
    pub fn new(state: gpui::Entity<State>, cx: &mut Context<Self>) -> Self {
        let loading_models_task = Some(cx.spawn({
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = state
                    .update(cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    task.await.log_err();
                }
                this.update(cx, |this, cx| {
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

    fn retry_connection(&self, cx: &mut App) {
        self.state
            .update(cx, |state, cx| state.fetch_models(cx))
            .detach_and_log_err(cx);
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();

        let lmstudio_intro = "Run local LLMs like Llama, Phi, and Qwen.";

        if self.loading_models_task.is_some() {
            div().child(Label::new("Loading models...")).into_any()
        } else {
            v_flex()
                .gap_2()
                .child(
                    v_flex().gap_1().child(Label::new(lmstudio_intro)).child(
                        List::new()
                            .child(InstructionListItem::text_only(
                                "LM Studio needs to be running with at least one model downloaded.",
                            ))
                            .child(InstructionListItem::text_only(
                                "To get your first model, try running `lms get qwen2.5-coder-7b`",
                            )),
                    ),
                )
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
                                                .icon_size(IconSize::XSmall)
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
                                            .icon_size(IconSize::XSmall)
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
                                        .icon_size(IconSize::XSmall)
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
                                        .cursor_style(gpui::CursorStyle::Arrow)
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .child(Indicator::dot().color(Color::Success))
                                                .child(Label::new("Connected"))
                                                .into_any_element(),
                                        ),
                                )
                            } else {
                                this.child(
                                    Button::new("retry_lmstudio_models", "Connect")
                                        .icon_position(IconPosition::Start)
                                        .icon_size(IconSize::XSmall)
                                        .icon(IconName::Play)
                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                            this.retry_connection(cx)
                                        })),
                                )
                            }
                        }),
                )
                .into_any()
        }
    }
}
