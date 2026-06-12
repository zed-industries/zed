use anthropic::completion::{AnthropicEventMapper, AnthropicPromptCacheMode, into_anthropic};
use anthropic::{AnthropicError, AnthropicModelMode};
use anyhow::Result;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AppContext, AsyncApp, Entity, Task, Window};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, RateLimiter,
};
use settings::Settings;
use std::sync::Arc;
use ui::IconName;

use crate::provider::api_compatible::{
    ApiCompatibleProviderConfigurationView, ApiCompatibleProviderSettings,
    ApiCompatibleProviderState,
};

pub use settings::AnthropicCompatibleAvailableModel as AvailableModel;
pub use settings::AnthropicCompatibleModelCapabilities as ModelCapabilities;

const API_KEY_PLACEHOLDER: &str = "sk-ant-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AnthropicCompatibleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

pub struct AnthropicCompatibleLanguageModelProvider {
    id: LanguageModelProviderId,
    name: LanguageModelProviderName,
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

impl ApiCompatibleProviderSettings for AnthropicCompatibleSettings {
    fn api_url(&self) -> &str {
        &self.api_url
    }
}

pub type State = ApiCompatibleProviderState<AnthropicCompatibleSettings>;

fn available_model_to_anthropic_model(available: &AvailableModel) -> anthropic::Model {
    let mode = match available.mode.unwrap_or_default() {
        settings::ModelMode::Default => AnthropicModelMode::Default,
        settings::ModelMode::Thinking { budget_tokens } => {
            AnthropicModelMode::Thinking { budget_tokens }
        }
    };
    let supports_thinking = matches!(mode, AnthropicModelMode::Thinking { .. });

    anthropic::Model {
        display_name: available
            .display_name
            .clone()
            .unwrap_or_else(|| available.name.clone()),
        id: available.name.clone(),
        max_input_tokens: available.max_tokens,
        max_output_tokens: available.max_output_tokens.unwrap_or(4_096),
        default_temperature: available.default_temperature.unwrap_or(1.0),
        mode,
        supports_thinking,
        supports_adaptive_thinking: false,
        supports_images: available.capabilities.images,
        supports_speed: false,
        supported_effort_levels: Vec::new(),
        tool_override: available.tool_override.clone(),
        extra_beta_headers: available.extra_beta_headers.clone(),
    }
}

impl AnthropicCompatibleLanguageModelProvider {
    pub fn new(
        id: Arc<str>,
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = State::new(
            id.clone(),
            credentials_provider,
            |id, cx| {
                crate::AllLanguageModelSettings::get_global(cx)
                    .anthropic_compatible
                    .get(id)
            },
            cx,
        );

        Self {
            id: id.clone().into(),
            name: id.into(),
            http_client,
            state,
        }
    }

    fn create_language_model(&self, model: AvailableModel) -> Arc<dyn LanguageModel> {
        let capabilities = model.capabilities.clone();
        // Compatible providers may not support Anthropic's automatic prompt
        // caching; only request explicit (legacy) cache breakpoints when the
        // user has opted in via the `prompt_caching` capability.
        let cache_mode = if capabilities.prompt_caching {
            AnthropicPromptCacheMode::Legacy
        } else {
            AnthropicPromptCacheMode::Disabled
        };
        let model = available_model_to_anthropic_model(&model);

        Arc::new(AnthropicCompatibleLanguageModel {
            id: LanguageModelId::from(model.id.clone()),
            provider_id: self.id.clone(),
            provider_name: self.name.clone(),
            model,
            capabilities,
            cache_mode,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for AnthropicCompatibleLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for AnthropicCompatibleLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelProviderName {
        self.name.clone()
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiAnthropicCompat)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.state
            .read(cx)
            .settings
            .available_models
            .first()
            .map(|model| self.create_language_model(model.clone()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        self.state
            .read(cx)
            .settings
            .available_models
            .iter()
            .map(|model| self.create_language_model(model.clone()))
            .collect()
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
        cx.new(|cx| {
            ApiCompatibleProviderConfigurationView::new(
                self.state.clone(),
                "Anthropic",
                API_KEY_PLACEHOLDER,
                window,
                cx,
            )
        })
        .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct AnthropicCompatibleLanguageModel {
    id: LanguageModelId,
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
    model: anthropic::Model,
    capabilities: ModelCapabilities,
    cache_mode: AnthropicPromptCacheMode,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl AnthropicCompatibleLanguageModel {
    fn stream_completion(
        &self,
        request: anthropic::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<anthropic::Event, AnthropicError>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();
        let provider_name = self.provider_name.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, _cx| {
            let api_url = state.settings.api_url.clone();
            (
                state.api_key_state.key(&api_url),
                api_url,
                state.settings.custom_headers.clone(),
            )
        });

        let beta_headers = self.model.beta_headers();

        async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: provider_name,
                });
            };

            let request = anthropic::stream_completion(
                http_client.as_ref(),
                &api_url,
                &api_key,
                request,
                beta_headers,
                &extra_headers,
            );

            request
                .await
                .map_err(|error| anthropic::completion_error_from_anthropic(error, provider_name))
        }
        .boxed()
    }
}

impl LanguageModel for AnthropicCompatibleLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name.clone())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        self.provider_id.clone()
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        self.provider_name.clone()
    }

    fn supports_tools(&self) -> bool {
        self.capabilities.tools
    }

    fn supports_images(&self) -> bool {
        self.capabilities.images
    }

    fn supports_streaming_tools(&self) -> bool {
        self.capabilities.tools
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto | LanguageModelToolChoice::Any => self.capabilities.tools,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_thinking(&self) -> bool {
        self.model.supports_thinking
    }

    fn telemetry_id(&self) -> String {
        format!("anthropic/{}", self.model.id)
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_input_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        Some(self.model.max_output_tokens)
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
        let has_tools = !request.tools.is_empty();
        let request_id = self.model.request_id(has_tools).to_string();
        let mut request = into_anthropic(
            request,
            request_id,
            self.model.default_temperature,
            self.model.max_output_tokens,
            self.model.mode.clone(),
            self.cache_mode,
        );
        if !self.model.supports_speed {
            request.speed = None;
        }
        let completion_request = self.stream_completion(request, cx);
        let provider_name = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let response = completion_request.await?;
            Ok(AnthropicEventMapper::new(provider_name).map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}
