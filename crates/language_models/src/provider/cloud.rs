use super::open_ai::count_open_ai_tokens;
use anthropic::AnthropicError;
use anyhow::{anyhow, Result};
use client::{
    zed_urls, Client, PerformCompletionParams, UserStore, EXPIRED_LLM_TOKEN_HEADER_NAME,
    MAX_LLM_MONTHLY_SPEND_REACHED_HEADER_NAME,
};
use collections::BTreeMap;
use feature_flags::{FeatureFlagAppExt, LlmClosedBeta, ZedPro};
use futures::{
    future::BoxFuture, stream::BoxStream, AsyncBufReadExt, FutureExt, Stream, StreamExt,
    TryStreamExt as _,
};
use gpui::{
    AnyElement, AnyView, App, AsyncApp, Context, Entity, EventEmitter, Global, ReadGlobal,
    Subscription, Task,
};
use http_client::{AsyncBody, HttpClient, Method, Response, StatusCode};
use language_model::{
    AuthenticateError, CloudModel, LanguageModel, LanguageModelCacheConfiguration, LanguageModelId,
    LanguageModelName, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelProviderTosView, LanguageModelRequest, RateLimiter,
    ZED_CLOUD_PROVIDER_ID,
};
use language_model::{
    LanguageModelAvailability, LanguageModelCompletionEvent, LanguageModelProvider,
};
use proto::TypedEnvelope;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::RawValue;
use settings::{Settings, SettingsStore};
use smol::{
    io::{AsyncReadExt, BufReader},
    lock::{RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard},
};
use std::fmt;
use std::{
    future,
    sync::{Arc, LazyLock},
};
use strum::IntoEnumIterator;
use thiserror::Error;
use ui::{prelude::*, TintColor};

use crate::provider::anthropic::map_to_language_model_completion_events;
use crate::AllLanguageModelSettings;

use super::anthropic::count_anthropic_tokens;

pub const PROVIDER_NAME: &str = "Zed";

const ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON: Option<&str> =
    option_env!("ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON");

fn zed_cloud_provider_additional_models() -> &'static [AvailableModel] {
    static ADDITIONAL_MODELS: LazyLock<Vec<AvailableModel>> = LazyLock::new(|| {
        ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON
            .map(|json| serde_json::from_str(json).unwrap())
            .unwrap_or_default()
    });
    ADDITIONAL_MODELS.as_slice()
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ZedDotDevSettings {
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AvailableProvider {
    Anthropic,
    OpenAi,
    Google,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    /// The provider of the language model.
    pub provider: AvailableProvider,
    /// The model's name in the provider's API. e.g. claude-3-5-sonnet-20240620
    pub name: String,
    /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
    pub display_name: Option<String>,
    /// The size of the context window, indicating the maximum number of tokens the model can process.
    pub max_tokens: usize,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u32>,
    /// The maximum number of completion tokens allowed by the model (o1-* only)
    pub max_completion_tokens: Option<u32>,
    /// Override this model with a different Anthropic model for tool calls.
    pub tool_override: Option<String>,
    /// Indicates whether this custom model supports caching.
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
    /// The default temperature to use for this model.
    pub default_temperature: Option<f32>,
    /// Any extra beta headers to provide when using the model.
    #[serde(default)]
    pub extra_beta_headers: Vec<String>,
}

struct GlobalRefreshLlmTokenListener(Entity<RefreshLlmTokenListener>);

impl Global for GlobalRefreshLlmTokenListener {}

pub struct RefreshLlmTokenEvent;

pub struct RefreshLlmTokenListener {
    _llm_token_subscription: client::Subscription,
}

impl EventEmitter<RefreshLlmTokenEvent> for RefreshLlmTokenListener {}

impl RefreshLlmTokenListener {
    pub fn register(client: Arc<Client>, cx: &mut App) {
        let listener = cx.new(|cx| RefreshLlmTokenListener::new(client, cx));
        cx.set_global(GlobalRefreshLlmTokenListener(listener));
    }

    pub fn global(cx: &App) -> Entity<Self> {
        GlobalRefreshLlmTokenListener::global(cx).0.clone()
    }

    fn new(client: Arc<Client>, cx: &mut Context<Self>) -> Self {
        Self {
            _llm_token_subscription: client
                .add_message_handler(cx.weak_entity(), Self::handle_refresh_llm_token),
        }
    }

    async fn handle_refresh_llm_token(
        this: Entity<Self>,
        _: TypedEnvelope<proto::RefreshLlmToken>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |_this, cx| cx.emit(RefreshLlmTokenEvent))
    }
}

pub struct CloudLanguageModelProvider {
    client: Arc<Client>,
    state: gpui::Entity<State>,
    _maintain_client_status: Task<()>,
}

pub struct State {
    client: Arc<Client>,
    llm_api_token: LlmApiToken,
    user_store: Entity<UserStore>,
    status: client::Status,
    accept_terms: Option<Task<Result<()>>>,
    _settings_subscription: Subscription,
    _llm_token_subscription: Subscription,
}

impl State {
    fn new(
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        status: client::Status,
        cx: &mut Context<Self>,
    ) -> Self {
        let refresh_llm_token_listener = RefreshLlmTokenListener::global(cx);

        Self {
            client: client.clone(),
            llm_api_token: LlmApiToken::default(),
            user_store,
            status,
            accept_terms: None,
            _settings_subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
            _llm_token_subscription: cx.subscribe(
                &refresh_llm_token_listener,
                |this, _listener, _event, cx| {
                    let client = this.client.clone();
                    let llm_api_token = this.llm_api_token.clone();
                    cx.spawn(|_this, _cx| async move {
                        llm_api_token.refresh(&client).await?;
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                },
            ),
        }
    }

    fn is_signed_out(&self) -> bool {
        self.status.is_signed_out()
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(move |this, mut cx| async move {
            client.authenticate_and_connect(true, &cx).await?;
            this.update(&mut cx, |_, cx| cx.notify())
        })
    }

    fn has_accepted_terms_of_service(&self, cx: &App) -> bool {
        self.user_store
            .read(cx)
            .current_user_has_accepted_terms()
            .unwrap_or(false)
    }

    fn accept_terms_of_service(&mut self, cx: &mut Context<Self>) {
        let user_store = self.user_store.clone();
        self.accept_terms = Some(cx.spawn(move |this, mut cx| async move {
            let _ = user_store
                .update(&mut cx, |store, cx| store.accept_terms_of_service(cx))?
                .await;
            this.update(&mut cx, |this, cx| {
                this.accept_terms = None;
                cx.notify()
            })
        }));
    }
}

impl CloudLanguageModelProvider {
    pub fn new(user_store: Entity<UserStore>, client: Arc<Client>, cx: &mut App) -> Self {
        let mut status_rx = client.status();
        let status = *status_rx.borrow();

        let state = cx.new(|cx| State::new(client.clone(), user_store.clone(), status, cx));

        let state_ref = state.downgrade();
        let maintain_client_status = cx.spawn(|mut cx| async move {
            while let Some(status) = status_rx.next().await {
                if let Some(this) = state_ref.upgrade() {
                    _ = this.update(&mut cx, |this, cx| {
                        if this.status != status {
                            this.status = status;
                            cx.notify();
                        }
                    });
                } else {
                    break;
                }
            }
        });

        Self {
            client,
            state: state.clone(),
            _maintain_client_status: maintain_client_status,
        }
    }
}

impl LanguageModelProviderState for CloudLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for CloudLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(ZED_CLOUD_PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiZed
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let llm_api_token = self.state.read(cx).llm_api_token.clone();
        let model = CloudModel::Anthropic(anthropic::Model::default());
        Some(Arc::new(CloudLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            llm_api_token: llm_api_token.clone(),
            client: self.client.clone(),
            request_limiter: RateLimiter::new(4),
        }))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        if cx.is_staff() {
            for model in anthropic::Model::iter() {
                if !matches!(model, anthropic::Model::Custom { .. }) {
                    models.insert(model.id().to_string(), CloudModel::Anthropic(model));
                }
            }
            for model in open_ai::Model::iter() {
                if !matches!(model, open_ai::Model::Custom { .. }) {
                    models.insert(model.id().to_string(), CloudModel::OpenAi(model));
                }
            }
            for model in google_ai::Model::iter() {
                if !matches!(model, google_ai::Model::Custom { .. }) {
                    models.insert(model.id().to_string(), CloudModel::Google(model));
                }
            }
        } else {
            models.insert(
                anthropic::Model::Claude3_5Sonnet.id().to_string(),
                CloudModel::Anthropic(anthropic::Model::Claude3_5Sonnet),
            );
        }

        let llm_closed_beta_models = if cx.has_flag::<LlmClosedBeta>() {
            zed_cloud_provider_additional_models()
        } else {
            &[]
        };

        // Override with available models from settings
        for model in AllLanguageModelSettings::get_global(cx)
            .zed_dot_dev
            .available_models
            .iter()
            .chain(llm_closed_beta_models)
            .cloned()
        {
            let model = match model.provider {
                AvailableProvider::Anthropic => CloudModel::Anthropic(anthropic::Model::Custom {
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
                    default_temperature: model.default_temperature,
                    max_output_tokens: model.max_output_tokens,
                    extra_beta_headers: model.extra_beta_headers.clone(),
                }),
                AvailableProvider::OpenAi => CloudModel::OpenAi(open_ai::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    max_completion_tokens: model.max_completion_tokens,
                }),
                AvailableProvider::Google => CloudModel::Google(google_ai::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                }),
            };
            models.insert(model.id().to_string(), model.clone());
        }

        let llm_api_token = self.state.read(cx).llm_api_token.clone();
        models
            .into_values()
            .map(|model| {
                Arc::new(CloudLanguageModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    llm_api_token: llm_api_token.clone(),
                    client: self.client.clone(),
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        !self.state.read(cx).is_signed_out()
    }

    fn authenticate(&self, _cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        Task::ready(Ok(()))
    }

    fn configuration_view(&self, _: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|_| ConfigurationView {
            state: self.state.clone(),
        })
        .into()
    }

    fn must_accept_terms(&self, cx: &App) -> bool {
        !self.state.read(cx).has_accepted_terms_of_service(cx)
    }

    fn render_accept_terms(
        &self,
        view: LanguageModelProviderTosView,
        cx: &mut App,
    ) -> Option<AnyElement> {
        render_accept_terms(self.state.clone(), view, cx)
    }

    fn reset_credentials(&self, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }
}

fn render_accept_terms(
    state: Entity<State>,
    view_kind: LanguageModelProviderTosView,
    cx: &mut App,
) -> Option<AnyElement> {
    if state.read(cx).has_accepted_terms_of_service(cx) {
        return None;
    }

    let accept_terms_disabled = state.read(cx).accept_terms.is_some();

    let terms_button = Button::new("terms_of_service", "Terms of Service")
        .style(ButtonStyle::Subtle)
        .icon(IconName::ArrowUpRight)
        .icon_color(Color::Muted)
        .icon_size(IconSize::XSmall)
        .on_click(move |_, _window, cx| cx.open_url("https://zed.dev/terms-of-service"));

    let text = "To start using Zed AI, please read and accept the";

    let form = v_flex()
        .w_full()
        .gap_2()
        .when(
            view_kind == LanguageModelProviderTosView::ThreadEmptyState,
            |form| form.items_center(),
        )
        .child(
            h_flex()
                .flex_wrap()
                .when(
                    view_kind == LanguageModelProviderTosView::ThreadEmptyState,
                    |form| form.justify_center(),
                )
                .child(Label::new(text))
                .child(terms_button),
        )
        .child({
            let button_container = h_flex().w_full().child(
                Button::new("accept_terms", "I accept the Terms of Service")
                    .style(ButtonStyle::Tinted(TintColor::Accent))
                    .disabled(accept_terms_disabled)
                    .on_click({
                        let state = state.downgrade();
                        move |_, _window, cx| {
                            state
                                .update(cx, |state, cx| state.accept_terms_of_service(cx))
                                .ok();
                        }
                    }),
            );

            match view_kind {
                LanguageModelProviderTosView::ThreadEmptyState => button_container.justify_center(),
                LanguageModelProviderTosView::PromptEditorPopup => button_container.justify_end(),
                LanguageModelProviderTosView::Configuration => button_container.justify_start(),
            }
        });

    Some(form.into_any())
}

pub struct CloudLanguageModel {
    id: LanguageModelId,
    model: CloudModel,
    llm_api_token: LlmApiToken,
    client: Arc<Client>,
    request_limiter: RateLimiter,
}

#[derive(Clone, Default)]
pub struct LlmApiToken(Arc<RwLock<Option<String>>>);

#[derive(Error, Debug)]
pub struct PaymentRequiredError;

impl fmt::Display for PaymentRequiredError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Payment required to use this language model. Please upgrade your account."
        )
    }
}

#[derive(Error, Debug)]
pub struct MaxMonthlySpendReachedError;

impl fmt::Display for MaxMonthlySpendReachedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Maximum spending limit reached for this month. For more usage, increase your spending limit."
        )
    }
}

impl CloudLanguageModel {
    async fn perform_llm_completion(
        client: Arc<Client>,
        llm_api_token: LlmApiToken,
        body: PerformCompletionParams,
    ) -> Result<Response<AsyncBody>> {
        let http_client = &client.http_client();

        let mut token = llm_api_token.acquire(&client).await?;
        let mut did_retry = false;

        let response = loop {
            let request_builder = http_client::Request::builder();
            let request = request_builder
                .method(Method::POST)
                .uri(http_client.build_zed_llm_url("/completion", &[])?.as_ref())
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {token}"))
                .body(serde_json::to_string(&body)?.into())?;
            let mut response = http_client.send(request).await?;
            if response.status().is_success() {
                break response;
            } else if !did_retry
                && response
                    .headers()
                    .get(EXPIRED_LLM_TOKEN_HEADER_NAME)
                    .is_some()
            {
                did_retry = true;
                token = llm_api_token.refresh(&client).await?;
            } else if response.status() == StatusCode::FORBIDDEN
                && response
                    .headers()
                    .get(MAX_LLM_MONTHLY_SPEND_REACHED_HEADER_NAME)
                    .is_some()
            {
                break Err(anyhow!(MaxMonthlySpendReachedError))?;
            } else if response.status() == StatusCode::PAYMENT_REQUIRED {
                break Err(anyhow!(PaymentRequiredError))?;
            } else {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                break Err(anyhow!(
                    "cloud language model completion failed with status {}: {body}",
                    response.status()
                ))?;
            }
        };

        Ok(response)
    }
}

impl LanguageModel for CloudLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn icon(&self) -> Option<IconName> {
        self.model.icon()
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(ZED_CLOUD_PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn telemetry_id(&self) -> String {
        format!("zed.dev/{}", self.model.id())
    }

    fn availability(&self) -> LanguageModelAvailability {
        self.model.availability()
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        match &self.model {
            CloudModel::Anthropic(model) => {
                model
                    .cache_configuration()
                    .map(|cache| LanguageModelCacheConfiguration {
                        max_cache_anchors: cache.max_cache_anchors,
                        should_speculate: cache.should_speculate,
                        min_total_token: cache.min_total_token,
                    })
            }
            CloudModel::OpenAi(_) | CloudModel::Google(_) => None,
        }
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        match self.model.clone() {
            CloudModel::Anthropic(_) => count_anthropic_tokens(request, cx),
            CloudModel::OpenAi(model) => count_open_ai_tokens(request, model, cx),
            CloudModel::Google(model) => {
                let client = self.client.clone();
                let request = request.into_google(model.id().into());
                let request = google_ai::CountTokensRequest {
                    contents: request.contents,
                };
                async move {
                    let request = serde_json::to_string(&request)?;
                    let response = client
                        .request(proto::CountLanguageModelTokens {
                            provider: proto::LanguageModelProvider::Google as i32,
                            request,
                        })
                        .await?;
                    Ok(response.token_count as usize)
                }
                .boxed()
            }
        }
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        _cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<LanguageModelCompletionEvent>>>> {
        match &self.model {
            CloudModel::Anthropic(model) => {
                let request = request.into_anthropic(
                    model.id().into(),
                    model.default_temperature(),
                    model.max_output_tokens(),
                );
                let client = self.client.clone();
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream(async move {
                    let response = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        PerformCompletionParams {
                            provider: client::LanguageModelProvider::Anthropic,
                            model: request.model.clone(),
                            provider_request: RawValue::from_string(serde_json::to_string(
                                &request,
                            )?)?,
                        },
                    )
                    .await?;
                    Ok(map_to_language_model_completion_events(Box::pin(
                        response_lines(response).map_err(AnthropicError::Other),
                    )))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
            CloudModel::OpenAi(model) => {
                let client = self.client.clone();
                let request = request.into_open_ai(model.id().into(), model.max_output_tokens());
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream(async move {
                    let response = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        PerformCompletionParams {
                            provider: client::LanguageModelProvider::OpenAi,
                            model: request.model.clone(),
                            provider_request: RawValue::from_string(serde_json::to_string(
                                &request,
                            )?)?,
                        },
                    )
                    .await?;
                    Ok(open_ai::extract_text_from_events(response_lines(response)))
                });
                async move {
                    Ok(future
                        .await?
                        .map(|result| result.map(LanguageModelCompletionEvent::Text))
                        .boxed())
                }
                .boxed()
            }
            CloudModel::Google(model) => {
                let client = self.client.clone();
                let request = request.into_google(model.id().into());
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream(async move {
                    let response = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        PerformCompletionParams {
                            provider: client::LanguageModelProvider::Google,
                            model: request.model.clone(),
                            provider_request: RawValue::from_string(serde_json::to_string(
                                &request,
                            )?)?,
                        },
                    )
                    .await?;
                    Ok(google_ai::extract_text_from_events(response_lines(
                        response,
                    )))
                });
                async move {
                    Ok(future
                        .await?
                        .map(|result| result.map(LanguageModelCompletionEvent::Text))
                        .boxed())
                }
                .boxed()
            }
        }
    }

    fn use_any_tool(
        &self,
        request: LanguageModelRequest,
        tool_name: String,
        tool_description: String,
        input_schema: serde_json::Value,
        _cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let client = self.client.clone();
        let llm_api_token = self.llm_api_token.clone();

        match &self.model {
            CloudModel::Anthropic(model) => {
                let mut request = request.into_anthropic(
                    model.tool_model_id().into(),
                    model.default_temperature(),
                    model.max_output_tokens(),
                );
                request.tool_choice = Some(anthropic::ToolChoice::Tool {
                    name: tool_name.clone(),
                });
                request.tools = vec![anthropic::Tool {
                    name: tool_name.clone(),
                    description: tool_description,
                    input_schema,
                }];

                self.request_limiter
                    .run(async move {
                        let response = Self::perform_llm_completion(
                            client.clone(),
                            llm_api_token,
                            PerformCompletionParams {
                                provider: client::LanguageModelProvider::Anthropic,
                                model: request.model.clone(),
                                provider_request: RawValue::from_string(serde_json::to_string(
                                    &request,
                                )?)?,
                            },
                        )
                        .await?;

                        Ok(anthropic::extract_tool_args_from_events(
                            tool_name,
                            Box::pin(response_lines(response)),
                        )
                        .await?
                        .boxed())
                    })
                    .boxed()
            }
            CloudModel::OpenAi(model) => {
                let mut request =
                    request.into_open_ai(model.id().into(), model.max_output_tokens());
                request.tool_choice = Some(open_ai::ToolChoice::Other(
                    open_ai::ToolDefinition::Function {
                        function: open_ai::FunctionDefinition {
                            name: tool_name.clone(),
                            description: None,
                            parameters: None,
                        },
                    },
                ));
                request.tools = vec![open_ai::ToolDefinition::Function {
                    function: open_ai::FunctionDefinition {
                        name: tool_name.clone(),
                        description: Some(tool_description),
                        parameters: Some(input_schema),
                    },
                }];

                self.request_limiter
                    .run(async move {
                        let response = Self::perform_llm_completion(
                            client.clone(),
                            llm_api_token,
                            PerformCompletionParams {
                                provider: client::LanguageModelProvider::OpenAi,
                                model: request.model.clone(),
                                provider_request: RawValue::from_string(serde_json::to_string(
                                    &request,
                                )?)?,
                            },
                        )
                        .await?;

                        Ok(open_ai::extract_tool_args_from_events(
                            tool_name,
                            Box::pin(response_lines(response)),
                        )
                        .await?
                        .boxed())
                    })
                    .boxed()
            }
            CloudModel::Google(_) => {
                future::ready(Err(anyhow!("tool use not implemented for Google AI"))).boxed()
            }
        }
    }
}

fn response_lines<T: DeserializeOwned>(
    response: Response<AsyncBody>,
) -> impl Stream<Item = Result<T>> {
    futures::stream::try_unfold(
        (String::new(), BufReader::new(response.into_body())),
        move |(mut line, mut body)| async {
            match body.read_line(&mut line).await {
                Ok(0) => Ok(None),
                Ok(_) => {
                    let event: T = serde_json::from_str(&line)?;
                    line.clear();
                    Ok(Some((event, (line, body))))
                }
                Err(e) => Err(e.into()),
            }
        },
    )
}

impl LlmApiToken {
    pub async fn acquire(&self, client: &Arc<Client>) -> Result<String> {
        let lock = self.0.upgradable_read().await;
        if let Some(token) = lock.as_ref() {
            Ok(token.to_string())
        } else {
            Self::fetch(RwLockUpgradableReadGuard::upgrade(lock).await, client).await
        }
    }

    pub async fn refresh(&self, client: &Arc<Client>) -> Result<String> {
        Self::fetch(self.0.write().await, client).await
    }

    async fn fetch<'a>(
        mut lock: RwLockWriteGuard<'a, Option<String>>,
        client: &Arc<Client>,
    ) -> Result<String> {
        let response = client.request(proto::GetLlmToken {}).await?;
        *lock = Some(response.token.clone());
        Ok(response.token.clone())
    }
}

struct ConfigurationView {
    state: gpui::Entity<State>,
}

impl ConfigurationView {
    fn authenticate(&mut self, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            state.authenticate(cx).detach_and_log_err(cx);
        });
        cx.notify();
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const ZED_AI_URL: &str = "https://zed.dev/ai";

        let is_connected = !self.state.read(cx).is_signed_out();
        let plan = self.state.read(cx).user_store.read(cx).current_plan();
        let has_accepted_terms = self.state.read(cx).has_accepted_terms_of_service(cx);

        let is_pro = plan == Some(proto::Plan::ZedPro);
        let subscription_text = Label::new(if is_pro {
            "You have full access to Zed's hosted LLMs, which include models from Anthropic, OpenAI, and Google. They come with faster speeds and higher limits through Zed Pro."
        } else {
            "You have basic access to models from Anthropic through the Zed AI Free plan."
        });
        let manage_subscription_button = if is_pro {
            Some(
                h_flex().child(
                    Button::new("manage_settings", "Manage Subscription")
                        .style(ButtonStyle::Tinted(TintColor::Accent))
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.open_url(&zed_urls::account_url(cx))),
                        ),
                ),
            )
        } else if cx.has_flag::<ZedPro>() {
            Some(
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("learn_more", "Learn more")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|_, _, _, cx| cx.open_url(ZED_AI_URL))),
                    )
                    .child(
                        Button::new("upgrade", "Upgrade")
                            .style(ButtonStyle::Subtle)
                            .color(Color::Accent)
                            .on_click(
                                cx.listener(|_, _, _, cx| cx.open_url(&zed_urls::account_url(cx))),
                            ),
                    ),
            )
        } else {
            None
        };

        if is_connected {
            v_flex()
                .gap_3()
                .w_full()
                .children(render_accept_terms(
                    self.state.clone(),
                    LanguageModelProviderTosView::Configuration,
                    cx,
                ))
                .when(has_accepted_terms, |this| {
                    this.child(subscription_text)
                        .children(manage_subscription_button)
                })
        } else {
            v_flex()
                .gap_2()
                .child(Label::new("Use Zed AI to access hosted language models."))
                .child(
                    Button::new("sign_in", "Sign In")
                        .icon_color(Color::Muted)
                        .icon(IconName::Github)
                        .icon_position(IconPosition::Start)
                        .on_click(cx.listener(move |this, _, _, cx| this.authenticate(cx))),
                )
        }
    }
}
