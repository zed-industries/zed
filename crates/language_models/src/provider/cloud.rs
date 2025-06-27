use anthropic::{AnthropicModelMode, parse_prompt_too_long};
use anyhow::{Context as _, Result, anyhow};
use client::{Client, ModelRequestUsage, UserStore, zed_urls};
use futures::{
    AsyncBufReadExt, FutureExt, Stream, StreamExt, future::BoxFuture, stream::BoxStream,
};
use google_ai::GoogleModelMode;
use gpui::{
    AnyElement, AnyView, App, AsyncApp, Context, Entity, SemanticVersion, Subscription, Task,
};
use http_client::{AsyncBody, HttpClient, Method, Response, StatusCode};
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCacheConfiguration,
    LanguageModelCompletionError, LanguageModelId, LanguageModelKnownError, LanguageModelName,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelProviderTosView, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolSchemaFormat, ModelRequestLimitReachedError, RateLimiter,
    ZED_CLOUD_PROVIDER_ID,
};
use language_model::{
    LanguageModelCompletionEvent, LanguageModelProvider, LlmApiToken, PaymentRequiredError,
    RefreshLlmTokenListener,
};
use proto::Plan;
use release_channel::AppVersion;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use settings::SettingsStore;
use smol::Timer;
use smol::io::{AsyncReadExt, BufReader};
use std::pin::Pin;
use std::str::FromStr as _;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use ui::{TintColor, prelude::*};
use util::{ResultExt as _, maybe};
use zed_llm_client::{
    CLIENT_SUPPORTS_STATUS_MESSAGES_HEADER_NAME, CURRENT_PLAN_HEADER_NAME, CompletionBody,
    CompletionRequestStatus, CountTokensBody, CountTokensResponse, EXPIRED_LLM_TOKEN_HEADER_NAME,
    ListModelsResponse, MODEL_REQUESTS_RESOURCE_HEADER_VALUE,
    SERVER_SUPPORTS_STATUS_MESSAGES_HEADER_NAME, SUBSCRIPTION_LIMIT_RESOURCE_HEADER_NAME,
    TOOL_USE_LIMIT_REACHED_HEADER_NAME, ZED_VERSION_HEADER_NAME,
};

use crate::provider::anthropic::{AnthropicEventMapper, count_anthropic_tokens, into_anthropic};
use crate::provider::google::{GoogleEventMapper, into_google};
use crate::provider::open_ai::{OpenAiEventMapper, count_open_ai_tokens, into_open_ai};

pub const PROVIDER_NAME: &str = "Zed";

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
    pub max_output_tokens: Option<u64>,
    /// The maximum number of completion tokens allowed by the model (o1-* only)
    pub max_completion_tokens: Option<u64>,
    /// Override this model with a different Anthropic model for tool calls.
    pub tool_override: Option<String>,
    /// Indicates whether this custom model supports caching.
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
    /// The default temperature to use for this model.
    pub default_temperature: Option<f32>,
    /// Any extra beta headers to provide when using the model.
    #[serde(default)]
    pub extra_beta_headers: Vec<String>,
    /// The model's mode (e.g. thinking)
    pub mode: Option<ModelMode>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModelMode {
    #[default]
    Default,
    Thinking {
        /// The maximum number of tokens to use for reasoning. Must be lower than the model's `max_output_tokens`.
        budget_tokens: Option<u32>,
    },
}

impl From<ModelMode> for AnthropicModelMode {
    fn from(value: ModelMode) -> Self {
        match value {
            ModelMode::Default => AnthropicModelMode::Default,
            ModelMode::Thinking { budget_tokens } => AnthropicModelMode::Thinking { budget_tokens },
        }
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
    models: Vec<Arc<zed_llm_client::LanguageModel>>,
    default_model: Option<Arc<zed_llm_client::LanguageModel>>,
    default_fast_model: Option<Arc<zed_llm_client::LanguageModel>>,
    recommended_models: Vec<Arc<zed_llm_client::LanguageModel>>,
    _fetch_models_task: Task<()>,
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
            models: Vec::new(),
            default_model: None,
            default_fast_model: None,
            recommended_models: Vec::new(),
            _fetch_models_task: cx.spawn(async move |this, cx| {
                maybe!(async move {
                    let (client, llm_api_token) = this
                        .read_with(cx, |this, _cx| (client.clone(), this.llm_api_token.clone()))?;

                    loop {
                        let status = this.read_with(cx, |this, _cx| this.status)?;
                        if matches!(status, client::Status::Connected { .. }) {
                            break;
                        }

                        cx.background_executor()
                            .timer(Duration::from_millis(100))
                            .await;
                    }

                    let response = Self::fetch_models(client, llm_api_token).await?;
                    cx.update(|cx| {
                        this.update(cx, |this, cx| {
                            let mut models = Vec::new();

                            for model in response.models {
                                models.push(Arc::new(model.clone()));

                                // Right now we represent thinking variants of models as separate models on the client,
                                // so we need to insert variants for any model that supports thinking.
                                if model.supports_thinking {
                                    models.push(Arc::new(zed_llm_client::LanguageModel {
                                        id: zed_llm_client::LanguageModelId(
                                            format!("{}-thinking", model.id).into(),
                                        ),
                                        display_name: format!("{} Thinking", model.display_name),
                                        ..model
                                    }));
                                }
                            }

                            this.default_model = models
                                .iter()
                                .find(|model| model.id == response.default_model)
                                .cloned();
                            this.default_fast_model = models
                                .iter()
                                .find(|model| model.id == response.default_fast_model)
                                .cloned();
                            this.recommended_models = response
                                .recommended_models
                                .iter()
                                .filter_map(|id| models.iter().find(|model| &model.id == id))
                                .cloned()
                                .collect();
                            this.models = models;
                            cx.notify();
                        })
                    })??;

                    anyhow::Ok(())
                })
                .await
                .context("failed to fetch Zed models")
                .log_err();
            }),
            _settings_subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
            _llm_token_subscription: cx.subscribe(
                &refresh_llm_token_listener,
                |this, _listener, _event, cx| {
                    let client = this.client.clone();
                    let llm_api_token = this.llm_api_token.clone();
                    cx.spawn(async move |_this, _cx| {
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
        cx.spawn(async move |state, cx| {
            client
                .authenticate_and_connect(true, &cx)
                .await
                .into_response()?;
            state.update(cx, |_, cx| cx.notify())
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
        self.accept_terms = Some(cx.spawn(async move |this, cx| {
            let _ = user_store
                .update(cx, |store, cx| store.accept_terms_of_service(cx))?
                .await;
            this.update(cx, |this, cx| {
                this.accept_terms = None;
                cx.notify()
            })
        }));
    }

    async fn fetch_models(
        client: Arc<Client>,
        llm_api_token: LlmApiToken,
    ) -> Result<ListModelsResponse> {
        let http_client = &client.http_client();
        let token = llm_api_token.acquire(&client).await?;

        let request = http_client::Request::builder()
            .method(Method::GET)
            .uri(http_client.build_zed_llm_url("/models", &[])?.as_ref())
            .header("Authorization", format!("Bearer {token}"))
            .body(AsyncBody::empty())?;
        let mut response = http_client
            .send(request)
            .await
            .context("failed to send list models request")?;

        if response.status().is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            return Ok(serde_json::from_str(&body)?);
        } else {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            anyhow::bail!(
                "error listing models.\nStatus: {:?}\nBody: {body}",
                response.status(),
            );
        }
    }
}

impl CloudLanguageModelProvider {
    pub fn new(user_store: Entity<UserStore>, client: Arc<Client>, cx: &mut App) -> Self {
        let mut status_rx = client.status();
        let status = *status_rx.borrow();

        let state = cx.new(|cx| State::new(client.clone(), user_store.clone(), status, cx));

        let state_ref = state.downgrade();
        let maintain_client_status = cx.spawn(async move |cx| {
            while let Some(status) = status_rx.next().await {
                if let Some(this) = state_ref.upgrade() {
                    _ = this.update(cx, |this, cx| {
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

    fn create_language_model(
        &self,
        model: Arc<zed_llm_client::LanguageModel>,
        llm_api_token: LlmApiToken,
    ) -> Arc<dyn LanguageModel> {
        Arc::new(CloudLanguageModel {
            id: LanguageModelId(SharedString::from(model.id.0.clone())),
            model,
            llm_api_token: llm_api_token.clone(),
            client: self.client.clone(),
            request_limiter: RateLimiter::new(4),
        })
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
        let default_model = self.state.read(cx).default_model.clone()?;
        let llm_api_token = self.state.read(cx).llm_api_token.clone();
        Some(self.create_language_model(default_model, llm_api_token))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let default_fast_model = self.state.read(cx).default_fast_model.clone()?;
        let llm_api_token = self.state.read(cx).llm_api_token.clone();
        Some(self.create_language_model(default_fast_model, llm_api_token))
    }

    fn recommended_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let llm_api_token = self.state.read(cx).llm_api_token.clone();
        self.state
            .read(cx)
            .recommended_models
            .iter()
            .cloned()
            .map(|model| self.create_language_model(model, llm_api_token.clone()))
            .collect()
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let llm_api_token = self.state.read(cx).llm_api_token.clone();
        self.state
            .read(cx)
            .models
            .iter()
            .cloned()
            .map(|model| self.create_language_model(model, llm_api_token.clone()))
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

    let thread_fresh_start = matches!(view_kind, LanguageModelProviderTosView::ThreadFreshStart);
    let thread_empty_state = matches!(view_kind, LanguageModelProviderTosView::ThreadtEmptyState);

    let terms_button = Button::new("terms_of_service", "Terms of Service")
        .style(ButtonStyle::Subtle)
        .icon(IconName::ArrowUpRight)
        .icon_color(Color::Muted)
        .icon_size(IconSize::XSmall)
        .when(thread_empty_state, |this| this.label_size(LabelSize::Small))
        .on_click(move |_, _window, cx| cx.open_url("https://zed.dev/terms-of-service"));

    let button_container = h_flex().child(
        Button::new("accept_terms", "I accept the Terms of Service")
            .when(!thread_empty_state, |this| {
                this.full_width()
                    .style(ButtonStyle::Tinted(TintColor::Accent))
                    .icon(IconName::Check)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
            })
            .when(thread_empty_state, |this| {
                this.style(ButtonStyle::Tinted(TintColor::Warning))
                    .label_size(LabelSize::Small)
            })
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

    let form = if thread_empty_state {
        h_flex()
            .w_full()
            .flex_wrap()
            .justify_between()
            .child(
                h_flex()
                    .child(
                        Label::new("To start using Zed AI, please read and accept the")
                            .size(LabelSize::Small),
                    )
                    .child(terms_button),
            )
            .child(button_container)
    } else {
        v_flex()
            .w_full()
            .gap_2()
            .child(
                h_flex()
                    .flex_wrap()
                    .when(thread_fresh_start, |this| this.justify_center())
                    .child(Label::new(
                        "To start using Zed AI, please read and accept the",
                    ))
                    .child(terms_button),
            )
            .child({
                match view_kind {
                    LanguageModelProviderTosView::PromptEditorPopup => {
                        button_container.w_full().justify_end()
                    }
                    LanguageModelProviderTosView::Configuration => {
                        button_container.w_full().justify_start()
                    }
                    LanguageModelProviderTosView::ThreadFreshStart => {
                        button_container.w_full().justify_center()
                    }
                    LanguageModelProviderTosView::ThreadtEmptyState => div().w_0(),
                }
            })
    };

    Some(form.into_any())
}

pub struct CloudLanguageModel {
    id: LanguageModelId,
    model: Arc<zed_llm_client::LanguageModel>,
    llm_api_token: LlmApiToken,
    client: Arc<Client>,
    request_limiter: RateLimiter,
}

struct PerformLlmCompletionResponse {
    response: Response<AsyncBody>,
    usage: Option<ModelRequestUsage>,
    tool_use_limit_reached: bool,
    includes_status_messages: bool,
}

impl CloudLanguageModel {
    const MAX_RETRIES: usize = 3;

    async fn perform_llm_completion(
        client: Arc<Client>,
        llm_api_token: LlmApiToken,
        app_version: Option<SemanticVersion>,
        body: CompletionBody,
    ) -> Result<PerformLlmCompletionResponse> {
        let http_client = &client.http_client();

        let mut token = llm_api_token.acquire(&client).await?;
        let mut retries_remaining = Self::MAX_RETRIES;
        let mut retry_delay = Duration::from_secs(1);

        loop {
            let request_builder = http_client::Request::builder()
                .method(Method::POST)
                .uri(http_client.build_zed_llm_url("/completions", &[])?.as_ref());
            let request_builder = if let Some(app_version) = app_version {
                request_builder.header(ZED_VERSION_HEADER_NAME, app_version.to_string())
            } else {
                request_builder
            };

            let request = request_builder
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {token}"))
                .header(CLIENT_SUPPORTS_STATUS_MESSAGES_HEADER_NAME, "true")
                .body(serde_json::to_string(&body)?.into())?;
            let mut response = http_client.send(request).await?;
            let status = response.status();
            if status.is_success() {
                let includes_status_messages = response
                    .headers()
                    .get(SERVER_SUPPORTS_STATUS_MESSAGES_HEADER_NAME)
                    .is_some();

                let tool_use_limit_reached = response
                    .headers()
                    .get(TOOL_USE_LIMIT_REACHED_HEADER_NAME)
                    .is_some();

                let usage = if includes_status_messages {
                    None
                } else {
                    ModelRequestUsage::from_headers(response.headers()).ok()
                };

                return Ok(PerformLlmCompletionResponse {
                    response,
                    usage,
                    includes_status_messages,
                    tool_use_limit_reached,
                });
            } else if response
                .headers()
                .get(EXPIRED_LLM_TOKEN_HEADER_NAME)
                .is_some()
            {
                retries_remaining -= 1;
                token = llm_api_token.refresh(&client).await?;
            } else if status == StatusCode::FORBIDDEN
                && response
                    .headers()
                    .get(SUBSCRIPTION_LIMIT_RESOURCE_HEADER_NAME)
                    .is_some()
            {
                if let Some(MODEL_REQUESTS_RESOURCE_HEADER_VALUE) = response
                    .headers()
                    .get(SUBSCRIPTION_LIMIT_RESOURCE_HEADER_NAME)
                    .and_then(|resource| resource.to_str().ok())
                {
                    if let Some(plan) = response
                        .headers()
                        .get(CURRENT_PLAN_HEADER_NAME)
                        .and_then(|plan| plan.to_str().ok())
                        .and_then(|plan| zed_llm_client::Plan::from_str(plan).ok())
                    {
                        let plan = match plan {
                            zed_llm_client::Plan::ZedFree => Plan::Free,
                            zed_llm_client::Plan::ZedPro => Plan::ZedPro,
                            zed_llm_client::Plan::ZedProTrial => Plan::ZedProTrial,
                        };
                        return Err(anyhow!(ModelRequestLimitReachedError { plan }));
                    }
                }

                anyhow::bail!("Forbidden");
            } else if status.as_u16() >= 500 && status.as_u16() < 600 {
                // If we encounter an error in the 500 range, retry after a delay.
                // We've seen at least these in the wild from API providers:
                // * 500 Internal Server Error
                // * 502 Bad Gateway
                // * 529 Service Overloaded

                if retries_remaining == 0 {
                    let mut body = String::new();
                    response.body_mut().read_to_string(&mut body).await?;
                    anyhow::bail!(
                        "cloud language model completion failed after {} retries with status {status}: {body}",
                        Self::MAX_RETRIES
                    );
                }

                Timer::after(retry_delay).await;

                retries_remaining -= 1;
                retry_delay *= 2; // If it fails again, wait longer.
            } else if status == StatusCode::PAYMENT_REQUIRED {
                return Err(anyhow!(PaymentRequiredError));
            } else {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                return Err(anyhow!(ApiError { status, body }));
            }
        }
    }
}

#[derive(Debug, Error)]
#[error("cloud language model request failed with status {status}: {body}")]
struct ApiError {
    status: StatusCode,
    body: String,
}

impl LanguageModel for CloudLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name.clone())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(ZED_CLOUD_PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tools
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto
            | LanguageModelToolChoice::Any
            | LanguageModelToolChoice::None => true,
        }
    }

    fn supports_burn_mode(&self) -> bool {
        self.model.supports_max_mode
    }

    fn telemetry_id(&self) -> String {
        format!("zed.dev/{}", self.model.id)
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        match self.model.provider {
            zed_llm_client::LanguageModelProvider::Anthropic
            | zed_llm_client::LanguageModelProvider::OpenAi => {
                LanguageModelToolSchemaFormat::JsonSchema
            }
            zed_llm_client::LanguageModelProvider::Google => {
                LanguageModelToolSchemaFormat::JsonSchemaSubset
            }
        }
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count as u64
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        match &self.model.provider {
            zed_llm_client::LanguageModelProvider::Anthropic => {
                Some(LanguageModelCacheConfiguration {
                    min_total_token: 2_048,
                    should_speculate: true,
                    max_cache_anchors: 4,
                })
            }
            zed_llm_client::LanguageModelProvider::OpenAi
            | zed_llm_client::LanguageModelProvider::Google => None,
        }
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        match self.model.provider {
            zed_llm_client::LanguageModelProvider::Anthropic => count_anthropic_tokens(request, cx),
            zed_llm_client::LanguageModelProvider::OpenAi => {
                let model = match open_ai::Model::from_id(&self.model.id.0) {
                    Ok(model) => model,
                    Err(err) => return async move { Err(anyhow!(err)) }.boxed(),
                };
                count_open_ai_tokens(request, model, cx)
            }
            zed_llm_client::LanguageModelProvider::Google => {
                let client = self.client.clone();
                let llm_api_token = self.llm_api_token.clone();
                let model_id = self.model.id.to_string();
                let generate_content_request =
                    into_google(request, model_id.clone(), GoogleModelMode::Default);
                async move {
                    let http_client = &client.http_client();
                    let token = llm_api_token.acquire(&client).await?;

                    let request_body = CountTokensBody {
                        provider: zed_llm_client::LanguageModelProvider::Google,
                        model: model_id,
                        provider_request: serde_json::to_value(&google_ai::CountTokensRequest {
                            generate_content_request,
                        })?,
                    };
                    let request = http_client::Request::builder()
                        .method(Method::POST)
                        .uri(
                            http_client
                                .build_zed_llm_url("/count_tokens", &[])?
                                .as_ref(),
                        )
                        .header("Content-Type", "application/json")
                        .header("Authorization", format!("Bearer {token}"))
                        .body(serde_json::to_string(&request_body)?.into())?;
                    let mut response = http_client.send(request).await?;
                    let status = response.status();
                    let mut response_body = String::new();
                    response
                        .body_mut()
                        .read_to_string(&mut response_body)
                        .await?;

                    if status.is_success() {
                        let response_body: CountTokensResponse =
                            serde_json::from_str(&response_body)?;

                        Ok(response_body.tokens as u64)
                    } else {
                        Err(anyhow!(ApiError {
                            status,
                            body: response_body
                        }))
                    }
                }
                .boxed()
            }
        }
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
        let thread_id = request.thread_id.clone();
        let prompt_id = request.prompt_id.clone();
        let intent = request.intent;
        let mode = request.mode;
        let app_version = cx.update(|cx| AppVersion::global(cx)).ok();
        match self.model.provider {
            zed_llm_client::LanguageModelProvider::Anthropic => {
                let request = into_anthropic(
                    request,
                    self.model.id.to_string(),
                    1.0,
                    self.model.max_output_tokens as u64,
                    if self.model.id.0.ends_with("-thinking") {
                        AnthropicModelMode::Thinking {
                            budget_tokens: Some(4_096),
                        }
                    } else {
                        AnthropicModelMode::Default
                    },
                );
                let client = self.client.clone();
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream(async move {
                    let PerformLlmCompletionResponse {
                        response,
                        usage,
                        includes_status_messages,
                        tool_use_limit_reached,
                    } = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            intent,
                            mode,
                            provider: zed_llm_client::LanguageModelProvider::Anthropic,
                            model: request.model.clone(),
                            provider_request: serde_json::to_value(&request)
                                .map_err(|e| anyhow!(e))?,
                        },
                    )
                    .await
                    .map_err(|err| match err.downcast::<ApiError>() {
                        Ok(api_err) => {
                            if api_err.status == StatusCode::BAD_REQUEST {
                                if let Some(tokens) = parse_prompt_too_long(&api_err.body) {
                                    return anyhow!(
                                        LanguageModelKnownError::ContextWindowLimitExceeded {
                                            tokens
                                        }
                                    );
                                }
                            }
                            anyhow!(api_err)
                        }
                        Err(err) => anyhow!(err),
                    })?;

                    let mut mapper = AnthropicEventMapper::new();
                    Ok(map_cloud_completion_events(
                        Box::pin(
                            response_lines(response, includes_status_messages)
                                .chain(usage_updated_event(usage))
                                .chain(tool_use_limit_reached_event(tool_use_limit_reached)),
                        ),
                        move |event| mapper.map_event(event),
                    ))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
            zed_llm_client::LanguageModelProvider::OpenAi => {
                let client = self.client.clone();
                let model = match open_ai::Model::from_id(&self.model.id.0) {
                    Ok(model) => model,
                    Err(err) => return async move { Err(anyhow!(err).into()) }.boxed(),
                };
                let request = into_open_ai(
                    request,
                    model.id(),
                    model.supports_parallel_tool_calls(),
                    None,
                );
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream(async move {
                    let PerformLlmCompletionResponse {
                        response,
                        usage,
                        includes_status_messages,
                        tool_use_limit_reached,
                    } = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            intent,
                            mode,
                            provider: zed_llm_client::LanguageModelProvider::OpenAi,
                            model: request.model.clone(),
                            provider_request: serde_json::to_value(&request)
                                .map_err(|e| anyhow!(e))?,
                        },
                    )
                    .await?;

                    let mut mapper = OpenAiEventMapper::new();
                    Ok(map_cloud_completion_events(
                        Box::pin(
                            response_lines(response, includes_status_messages)
                                .chain(usage_updated_event(usage))
                                .chain(tool_use_limit_reached_event(tool_use_limit_reached)),
                        ),
                        move |event| mapper.map_event(event),
                    ))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
            zed_llm_client::LanguageModelProvider::Google => {
                let client = self.client.clone();
                let request =
                    into_google(request, self.model.id.to_string(), GoogleModelMode::Default);
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream(async move {
                    let PerformLlmCompletionResponse {
                        response,
                        usage,
                        includes_status_messages,
                        tool_use_limit_reached,
                    } = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            intent,
                            mode,
                            provider: zed_llm_client::LanguageModelProvider::Google,
                            model: request.model.model_id.clone(),
                            provider_request: serde_json::to_value(&request)
                                .map_err(|e| anyhow!(e))?,
                        },
                    )
                    .await?;

                    let mut mapper = GoogleEventMapper::new();
                    Ok(map_cloud_completion_events(
                        Box::pin(
                            response_lines(response, includes_status_messages)
                                .chain(usage_updated_event(usage))
                                .chain(tool_use_limit_reached_event(tool_use_limit_reached)),
                        ),
                        move |event| mapper.map_event(event),
                    ))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloudCompletionEvent<T> {
    Status(CompletionRequestStatus),
    Event(T),
}

fn map_cloud_completion_events<T, F>(
    stream: Pin<Box<dyn Stream<Item = Result<CloudCompletionEvent<T>>> + Send>>,
    mut map_callback: F,
) -> BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
where
    T: DeserializeOwned + 'static,
    F: FnMut(T) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
        + Send
        + 'static,
{
    stream
        .flat_map(move |event| {
            futures::stream::iter(match event {
                Err(error) => {
                    vec![Err(LanguageModelCompletionError::Other(error))]
                }
                Ok(CloudCompletionEvent::Status(event)) => {
                    vec![Ok(LanguageModelCompletionEvent::StatusUpdate(event))]
                }
                Ok(CloudCompletionEvent::Event(event)) => map_callback(event),
            })
        })
        .boxed()
}

fn usage_updated_event<T>(
    usage: Option<ModelRequestUsage>,
) -> impl Stream<Item = Result<CloudCompletionEvent<T>>> {
    futures::stream::iter(usage.map(|usage| {
        Ok(CloudCompletionEvent::Status(
            CompletionRequestStatus::UsageUpdated {
                amount: usage.amount as usize,
                limit: usage.limit,
            },
        ))
    }))
}

fn tool_use_limit_reached_event<T>(
    tool_use_limit_reached: bool,
) -> impl Stream<Item = Result<CloudCompletionEvent<T>>> {
    futures::stream::iter(tool_use_limit_reached.then(|| {
        Ok(CloudCompletionEvent::Status(
            CompletionRequestStatus::ToolUseLimitReached,
        ))
    }))
}

fn response_lines<T: DeserializeOwned>(
    response: Response<AsyncBody>,
    includes_status_messages: bool,
) -> impl Stream<Item = Result<CloudCompletionEvent<T>>> {
    futures::stream::try_unfold(
        (String::new(), BufReader::new(response.into_body())),
        move |(mut line, mut body)| async move {
            match body.read_line(&mut line).await {
                Ok(0) => Ok(None),
                Ok(_) => {
                    let event = if includes_status_messages {
                        serde_json::from_str::<CloudCompletionEvent<T>>(&line)?
                    } else {
                        CloudCompletionEvent::Event(serde_json::from_str::<T>(&line)?)
                    };

                    line.clear();
                    Ok(Some((event, (line, body))))
                }
                Err(e) => Err(e.into()),
            }
        },
    )
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
        const ZED_PRICING_URL: &str = "https://zed.dev/pricing";

        let is_connected = !self.state.read(cx).is_signed_out();
        let user_store = self.state.read(cx).user_store.read(cx);
        let plan = user_store.current_plan();
        let subscription_period = user_store.subscription_period();
        let eligible_for_trial = user_store.trial_started_at().is_none();
        let has_accepted_terms = self.state.read(cx).has_accepted_terms_of_service(cx);

        let is_pro = plan == Some(proto::Plan::ZedPro);
        let subscription_text = match (plan, subscription_period) {
            (Some(proto::Plan::ZedPro), Some(_)) => {
                "You have access to Zed's hosted LLMs through your Zed Pro subscription."
            }
            (Some(proto::Plan::ZedProTrial), Some(_)) => {
                "You have access to Zed's hosted LLMs through your Zed Pro trial."
            }
            (Some(proto::Plan::Free), Some(_)) => {
                "You have basic access to Zed's hosted LLMs through your Zed Free subscription."
            }
            _ => {
                if eligible_for_trial {
                    "Subscribe for access to Zed's hosted LLMs. Start with a 14 day free trial."
                } else {
                    "Subscribe for access to Zed's hosted LLMs."
                }
            }
        };
        let manage_subscription_buttons = if is_pro {
            h_flex().child(
                Button::new("manage_settings", "Manage Subscription")
                    .style(ButtonStyle::Tinted(TintColor::Accent))
                    .on_click(cx.listener(|_, _, _, cx| cx.open_url(&zed_urls::account_url(cx)))),
            )
        } else {
            h_flex()
                .gap_2()
                .child(
                    Button::new("learn_more", "Learn more")
                        .style(ButtonStyle::Subtle)
                        .on_click(cx.listener(|_, _, _, cx| cx.open_url(ZED_PRICING_URL))),
                )
                .child(
                    Button::new("upgrade", "Upgrade")
                        .style(ButtonStyle::Subtle)
                        .color(Color::Accent)
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.open_url(&zed_urls::account_url(cx))),
                        ),
                )
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
                        .child(manage_subscription_buttons)
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
