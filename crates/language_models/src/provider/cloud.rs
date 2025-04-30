use anthropic::{AnthropicError, AnthropicModelMode, parse_prompt_too_long};
use anyhow::{Result, anyhow};
use client::{Client, UserStore, zed_urls};
use collections::BTreeMap;
use feature_flags::{FeatureFlagAppExt, LlmClosedBetaFeatureFlag, ZedProFeatureFlag};
use futures::{
    AsyncBufReadExt, FutureExt, Stream, StreamExt, TryStreamExt as _, future::BoxFuture,
    stream::BoxStream,
};
use gpui::{AnyElement, AnyView, App, AsyncApp, Context, Entity, Subscription, Task};
use http_client::{AsyncBody, HttpClient, Method, Response, StatusCode};
use language_model::{
    AuthenticateError, CloudModel, LanguageModel, LanguageModelCacheConfiguration,
    LanguageModelCompletionError, LanguageModelId, LanguageModelKnownError, LanguageModelName,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelProviderTosView, LanguageModelRequest, LanguageModelToolSchemaFormat,
    ModelRequestLimitReachedError, RateLimiter, RequestUsage, ZED_CLOUD_PROVIDER_ID,
};
use language_model::{
    LanguageModelAvailability, LanguageModelCompletionEvent, LanguageModelProvider, LlmApiToken,
    MaxMonthlySpendReachedError, PaymentRequiredError, RefreshLlmTokenListener,
};
use proto::Plan;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use settings::{Settings, SettingsStore};
use smol::Timer;
use smol::io::{AsyncReadExt, BufReader};
use std::str::FromStr as _;
use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};
use strum::IntoEnumIterator;
use thiserror::Error;
use ui::{TintColor, prelude::*};
use zed_llm_client::{
    CURRENT_PLAN_HEADER_NAME, CompletionBody, CountTokensBody, CountTokensResponse,
    EXPIRED_LLM_TOKEN_HEADER_NAME, MAX_LLM_MONTHLY_SPEND_REACHED_HEADER_NAME,
    MODEL_REQUESTS_RESOURCE_HEADER_VALUE, SUBSCRIPTION_LIMIT_RESOURCE_HEADER_NAME,
};

use crate::AllLanguageModelSettings;
use crate::provider::anthropic::{count_anthropic_tokens, into_anthropic};
use crate::provider::google::into_google;
use crate::provider::open_ai::{count_open_ai_tokens, into_open_ai};

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
        cx.spawn(async move |this, cx| {
            client.authenticate_and_connect(true, &cx).await?;
            this.update(cx, |_, cx| cx.notify())
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
        model: CloudModel,
        llm_api_token: LlmApiToken,
    ) -> Arc<dyn LanguageModel> {
        Arc::new(CloudLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
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
        let llm_api_token = self.state.read(cx).llm_api_token.clone();
        let model = CloudModel::Anthropic(anthropic::Model::Claude3_7Sonnet);
        Some(self.create_language_model(model, llm_api_token))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let llm_api_token = self.state.read(cx).llm_api_token.clone();
        let model = CloudModel::Anthropic(anthropic::Model::Claude3_5Sonnet);
        Some(self.create_language_model(model, llm_api_token))
    }

    fn recommended_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let llm_api_token = self.state.read(cx).llm_api_token.clone();
        [
            CloudModel::Anthropic(anthropic::Model::Claude3_7Sonnet),
            CloudModel::Anthropic(anthropic::Model::Claude3_7SonnetThinking),
        ]
        .into_iter()
        .map(|model| self.create_language_model(model, llm_api_token.clone()))
        .collect()
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
            models.insert(
                anthropic::Model::Claude3_7Sonnet.id().to_string(),
                CloudModel::Anthropic(anthropic::Model::Claude3_7Sonnet),
            );
            models.insert(
                anthropic::Model::Claude3_7SonnetThinking.id().to_string(),
                CloudModel::Anthropic(anthropic::Model::Claude3_7SonnetThinking),
            );
        }

        let llm_closed_beta_models = if cx.has_flag::<LlmClosedBetaFeatureFlag>() {
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
                    mode: model.mode.unwrap_or_default().into(),
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
                    caching: model.cache_configuration.is_some(),
                }),
            };
            models.insert(model.id().to_string(), model.clone());
        }

        let llm_api_token = self.state.read(cx).llm_api_token.clone();
        models
            .into_values()
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
    model: CloudModel,
    llm_api_token: LlmApiToken,
    client: Arc<Client>,
    request_limiter: RateLimiter,
}

impl CloudLanguageModel {
    const MAX_RETRIES: usize = 3;

    async fn perform_llm_completion(
        client: Arc<Client>,
        llm_api_token: LlmApiToken,
        body: CompletionBody,
    ) -> Result<(Response<AsyncBody>, Option<RequestUsage>)> {
        let http_client = &client.http_client();

        let mut token = llm_api_token.acquire(&client).await?;
        let mut retries_remaining = Self::MAX_RETRIES;
        let mut retry_delay = Duration::from_secs(1);

        loop {
            let request_builder = http_client::Request::builder().method(Method::POST);
            let request_builder = if let Ok(completions_url) = std::env::var("ZED_COMPLETIONS_URL")
            {
                request_builder.uri(completions_url)
            } else {
                request_builder.uri(http_client.build_zed_llm_url("/completions", &[])?.as_ref())
            };
            let request = request_builder
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {token}"))
                .body(serde_json::to_string(&body)?.into())?;
            let mut response = http_client.send(request).await?;
            let status = response.status();
            if status.is_success() {
                let usage = RequestUsage::from_headers(response.headers()).ok();

                return Ok((response, usage));
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
                    .get(MAX_LLM_MONTHLY_SPEND_REACHED_HEADER_NAME)
                    .is_some()
            {
                return Err(anyhow!(MaxMonthlySpendReachedError));
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
                            zed_llm_client::Plan::Free => Plan::Free,
                            zed_llm_client::Plan::ZedPro => Plan::ZedPro,
                            zed_llm_client::Plan::ZedProTrial => Plan::ZedProTrial,
                        };
                        return Err(anyhow!(ModelRequestLimitReachedError { plan }));
                    }
                }

                return Err(anyhow!("Forbidden"));
            } else if status.as_u16() >= 500 && status.as_u16() < 600 {
                // If we encounter an error in the 500 range, retry after a delay.
                // We've seen at least these in the wild from API providers:
                // * 500 Internal Server Error
                // * 502 Bad Gateway
                // * 529 Service Overloaded

                if retries_remaining == 0 {
                    let mut body = String::new();
                    response.body_mut().read_to_string(&mut body).await?;
                    return Err(anyhow!(
                        "cloud language model completion failed after {} retries with status {status}: {body}",
                        Self::MAX_RETRIES
                    ));
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
#[error("cloud language model completion failed with status {status}: {body}")]
struct ApiError {
    status: StatusCode,
    body: String,
}

impl LanguageModel for CloudLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn matches_id(&self, other_id: &LanguageModelId) -> bool {
        match &self.model {
            CloudModel::Anthropic(model) => model.matches_id(&other_id.0),
            CloudModel::Google(model) => model.matches_id(&other_id.0),
            CloudModel::OpenAi(model) => model.matches_id(&other_id.0),
        }
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(ZED_CLOUD_PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn supports_tools(&self) -> bool {
        match self.model {
            CloudModel::Anthropic(_) => true,
            CloudModel::Google(_) => true,
            CloudModel::OpenAi(_) => true,
        }
    }

    fn telemetry_id(&self) -> String {
        format!("zed.dev/{}", self.model.id())
    }

    fn availability(&self) -> LanguageModelAvailability {
        self.model.availability()
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        self.model.tool_input_format()
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
                let llm_api_token = self.llm_api_token.clone();
                let model_id = model.id().to_string();
                let generate_content_request = into_google(request, model_id.clone());
                async move {
                    let http_client = &client.http_client();
                    let token = llm_api_token.acquire(&client).await?;

                    let request_builder = http_client::Request::builder().method(Method::POST);
                    let request_builder =
                        if let Ok(completions_url) = std::env::var("ZED_COUNT_TOKENS_URL") {
                            request_builder.uri(completions_url)
                        } else {
                            request_builder.uri(
                                http_client
                                    .build_zed_llm_url("/count_tokens", &[])?
                                    .as_ref(),
                            )
                        };
                    let request_body = CountTokensBody {
                        provider: zed_llm_client::LanguageModelProvider::Google,
                        model: model_id,
                        provider_request: serde_json::to_value(&google_ai::CountTokensRequest {
                            generate_content_request,
                        })?,
                    };
                    let request = request_builder
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

                        Ok(response_body.tokens)
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
        >,
    > {
        self.stream_completion_with_usage(request, cx)
            .map(|result| result.map(|(stream, _)| stream))
            .boxed()
    }

    fn stream_completion_with_usage(
        &self,
        request: LanguageModelRequest,
        _cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<(
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            Option<RequestUsage>,
        )>,
    > {
        let thread_id = request.thread_id.clone();
        let prompt_id = request.prompt_id.clone();
        let mode = request.mode;
        match &self.model {
            CloudModel::Anthropic(model) => {
                let request = into_anthropic(
                    request,
                    model.request_id().into(),
                    model.default_temperature(),
                    model.max_output_tokens(),
                    model.mode(),
                );
                let client = self.client.clone();
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream_with_usage(async move {
                    let (response, usage) = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            mode,
                            provider: zed_llm_client::LanguageModelProvider::Anthropic,
                            model: request.model.clone(),
                            provider_request: serde_json::to_value(&request)?,
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

                    Ok((
                        crate::provider::anthropic::map_to_language_model_completion_events(
                            Box::pin(response_lines(response).map_err(AnthropicError::Other)),
                        ),
                        usage,
                    ))
                });
                async move {
                    let (stream, usage) = future.await?;
                    Ok((stream.boxed(), usage))
                }
                .boxed()
            }
            CloudModel::OpenAi(model) => {
                let client = self.client.clone();
                let request = into_open_ai(request, model, model.max_output_tokens());
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream_with_usage(async move {
                    let (response, usage) = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            mode,
                            provider: zed_llm_client::LanguageModelProvider::OpenAi,
                            model: request.model.clone(),
                            provider_request: serde_json::to_value(&request)?,
                        },
                    )
                    .await?;
                    Ok((
                        crate::provider::open_ai::map_to_language_model_completion_events(
                            Box::pin(response_lines(response)),
                        ),
                        usage,
                    ))
                });
                async move {
                    let (stream, usage) = future.await?;
                    Ok((stream.boxed(), usage))
                }
                .boxed()
            }
            CloudModel::Google(model) => {
                let client = self.client.clone();
                let request = into_google(request, model.id().into());
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream_with_usage(async move {
                    let (response, usage) = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            mode,
                            provider: zed_llm_client::LanguageModelProvider::Google,
                            model: request.model.model_id.clone(),
                            provider_request: serde_json::to_value(&request)?,
                        },
                    )
                    .await?;
                    Ok((
                        crate::provider::google::map_to_language_model_completion_events(Box::pin(
                            response_lines(response),
                        )),
                        usage,
                    ))
                });
                async move {
                    let (stream, usage) = future.await?;
                    Ok((stream.boxed(), usage))
                }
                .boxed()
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
        } else if cx.has_flag::<ZedProFeatureFlag>() {
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
