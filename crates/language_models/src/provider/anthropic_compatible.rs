use anthropic::{AnthropicError, AnthropicModelMode};
use anyhow::Result;
use convert_case::{Case, Casing};
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AppContext, AsyncApp, Entity, Task, Window};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCacheConfiguration,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice, RateLimiter,
};
use settings::{Settings, SettingsStore};
use std::sync::Arc;
use ui::IconName;

use crate::provider::anthropic::{
    AnthropicEventMapper, count_anthropic_tokens_with_tiktoken, into_anthropic,
};
use crate::provider::util::{
    ApiCompatibleProviderConfigurationView, ApiCompatibleProviderSettings,
    ApiCompatibleProviderState,
};

pub use settings::AnthropicCompatibleAvailableModel as AvailableModel;
pub use settings::AnthropicCompatibleModelCapabilities as ModelCapabilities;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AnthropicCompatibleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
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

impl AnthropicCompatibleLanguageModelProvider {
    pub fn new(id: Arc<str>, http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        fn resolve_settings<'a>(
            id: &'a str,
            cx: &'a App,
        ) -> Option<&'a AnthropicCompatibleSettings> {
            crate::AllLanguageModelSettings::get_global(cx)
                .anthropic_compatible
                .get(id)
        }

        let api_key_env_var_name = format!("{}_API_KEY", id).to_case(Case::UpperSnake).into();
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let Some(settings) = resolve_settings(&this.id, cx).cloned() else {
                    return;
                };
                this.update_settings(settings, cx);
            })
            .detach();

            let settings = resolve_settings(&id, cx).cloned().unwrap_or_default();
            State::new(id.clone(), settings, EnvVar::new(api_key_env_var_name))
        });

        Self {
            id: id.clone().into(),
            name: id.into(),
            http_client,
            state,
        }
    }

    fn create_language_model(&self, model: AvailableModel) -> Arc<dyn LanguageModel> {
        let capabilities = model.capabilities.clone();
        let model = anthropic::Model::Custom {
            name: model.name,
            display_name: model.display_name,
            max_tokens: model.max_tokens,
            tool_override: model.tool_override,
            cache_configuration: model.cache_configuration.as_ref().map(|configuration| {
                anthropic::AnthropicModelCacheConfiguration {
                    max_cache_anchors: configuration.max_cache_anchors,
                    should_speculate: configuration.should_speculate,
                    min_total_token: configuration.min_total_token,
                }
            }),
            max_output_tokens: model.max_output_tokens,
            default_temperature: model.default_temperature,
            extra_beta_headers: model.extra_beta_headers,
            mode: model.mode.unwrap_or_default().into(),
        };

        Arc::new(AnthropicCompatibleLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            provider_id: self.id.clone(),
            provider_name: self.name.clone(),
            model,
            capabilities,
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
        IconOrSvg::Icon(IconName::AiAnthropic)
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
                "sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
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

        let (api_key, api_url) = self.state.read_with(cx, |state, _cx| {
            let api_url = state.settings.api_url.clone();
            (state.api_key_state.key(&api_url), api_url)
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
            );

            request
                .await
                .map_err(|error| LanguageModelCompletionError::from_anthropic(error, provider_name))
        }
        .boxed()
    }
}

impl LanguageModel for AnthropicCompatibleLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
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
        matches!(self.model.mode(), AnthropicModelMode::Thinking { .. })
    }

    fn telemetry_id(&self) -> String {
        format!("anthropic/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        Some(self.model.max_output_tokens())
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        // Unlike the first-party Anthropic provider, we don't call the count_tokens API here,
        // since compatible providers may not implement it. Estimate locally instead.
        cx.background_spawn(async move { count_anthropic_tokens_with_tiktoken(request) })
            .boxed()
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
        let request = into_anthropic(
            request,
            self.model.request_id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
            self.model.mode(),
        );
        let completion_request = self.stream_completion(request, cx);
        let provider_name = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let response = completion_request.await?;
            Ok(AnthropicEventMapper::new(provider_name).map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        self.model
            .cache_configuration()
            .map(|configuration| LanguageModelCacheConfiguration {
                max_cache_anchors: configuration.max_cache_anchors,
                should_speculate: configuration.should_speculate,
                min_total_token: configuration.min_total_token,
            })
    }
}
