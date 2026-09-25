use anthropic::completion::{AnthropicEventMapper, AnthropicPromptCacheMode, into_anthropic};
use anthropic::{AnthropicError, AnthropicModelMode};
use anyhow::Result;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{App, AppContext, AsyncApp, Entity, Task};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionStream, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoiceSupport, ModelRateLimiters, ProviderSettingsView,
    SubPageProviderSettings, unavailable_error,
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
    request_limiters: ModelRateLimiters,
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
        settings::ModelMode::Adaptive => AnthropicModelMode::AdaptiveThinking,
    };
    let supports_thinking = matches!(
        mode,
        AnthropicModelMode::Thinking { .. } | AnthropicModelMode::AdaptiveThinking
    );
    let supports_adaptive_thinking = matches!(mode, AnthropicModelMode::AdaptiveThinking { .. });

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
        supports_adaptive_thinking,
        supports_images: available.capabilities.images,
        supports_speed: false,
        supports_compaction: false,
        supported_effort_levels: if supports_adaptive_thinking {
            vec![
                anthropic::Effort::Low,
                anthropic::Effort::Medium,
                anthropic::Effort::High,
                anthropic::Effort::XHigh,
                anthropic::Effort::Max,
            ]
        } else {
            Vec::new()
        },
        tool_override: available.tool_override.clone(),
        extra_beta_headers: available.extra_beta_headers.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_available_model(json: &str) -> AvailableModel {
        serde_json::from_str(json).expect("test fixture should parse")
    }

    #[test]
    fn adaptive_mode_maps_to_adaptive_thinking_with_all_effort_levels() {
        let available = parse_available_model(
            r#"{
                "name": "claude-opus-4-7",
                "max_tokens": 1000000,
                "max_output_tokens": 128000,
                "mode": { "type": "adaptive" }
            }"#,
        );
        let model = available_model_to_anthropic_model(&available);

        assert_eq!(model.mode, AnthropicModelMode::AdaptiveThinking);
        assert!(model.supports_thinking);
        assert!(model.supports_adaptive_thinking);
        assert_eq!(
            model.supported_effort_levels,
            vec![
                anthropic::Effort::Low,
                anthropic::Effort::Medium,
                anthropic::Effort::High,
                anthropic::Effort::XHigh,
                anthropic::Effort::Max,
            ]
        );
    }

    #[test]
    fn thinking_mode_does_not_enable_adaptive() {
        let available = parse_available_model(
            r#"{
                "name": "claude-sonnet-4-5",
                "max_tokens": 200000,
                "mode": { "type": "thinking", "budget_tokens": 4096 }
            }"#,
        );
        let model = available_model_to_anthropic_model(&available);

        assert!(matches!(model.mode, AnthropicModelMode::Thinking { .. }));
        assert!(model.supports_thinking);
        assert!(!model.supports_adaptive_thinking);
        assert!(model.supported_effort_levels.is_empty());
    }

    #[test]
    fn default_mode_disables_thinking() {
        let available = parse_available_model(
            r#"{
                "name": "claude-3-5-haiku",
                "max_tokens": 200000
            }"#,
        );
        let model = available_model_to_anthropic_model(&available);

        assert_eq!(model.mode, AnthropicModelMode::Default);
        assert!(!model.supports_thinking);
        assert!(!model.supports_adaptive_thinking);
        assert!(model.supported_effort_levels.is_empty());
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
            request_limiters: ModelRateLimiters::default(),
        }
    }

    /// Every model this provider offers: the settings entries, in order.
    fn available_models<'a>(&self, cx: &'a App) -> &'a [AvailableModel] {
        &self.state.read(cx).settings.available_models
    }

    fn create_language_model(&self, available: &AvailableModel) -> LanguageModel {
        let capabilities = &available.capabilities;
        let model = available_model_to_anthropic_model(available);
        LanguageModel {
            supports_tools: capabilities.tools,
            supports_images: capabilities.images,
            supports_streaming_tools: capabilities.tools,
            tool_choice_support: LanguageModelToolChoiceSupport {
                auto: capabilities.tools,
                any: capabilities.tools,
                none: true,
            },
            supports_thinking: model.supports_thinking,
            supported_effort_levels: model
                .supported_effort_levels
                .iter()
                .map(|effort| {
                    let is_default = matches!(effort, anthropic::Effort::High);
                    let (name, value) = match effort {
                        anthropic::Effort::Low => ("Low".into(), "low".into()),
                        anthropic::Effort::Medium => ("Medium".into(), "medium".into()),
                        anthropic::Effort::High => ("High".into(), "high".into()),
                        anthropic::Effort::XHigh => ("XHigh".into(), "xhigh".into()),
                        anthropic::Effort::Max => ("Max".into(), "max".into()),
                    };
                    language_model::LanguageModelEffortLevel {
                        name,
                        value,
                        is_default,
                    }
                })
                .collect(),
            max_output_tokens: Some(model.max_output_tokens),
            ..LanguageModel::new(
                LanguageModelId::from(model.id.clone()),
                LanguageModelName::from(model.display_name.clone()),
                self.id.clone(),
                self.name.clone(),
                format!("anthropic/{}", model.id),
                model.max_input_tokens,
            )
        }
    }

    /// The current configuration of `model` and the prompt cache mode to
    /// request it with, if this provider still offers it.
    fn config(
        &self,
        model: &LanguageModel,
        cx: &App,
    ) -> Result<(anthropic::Model, AnthropicPromptCacheMode), LanguageModelCompletionError> {
        let available = self
            .available_models(cx)
            .iter()
            .find(|available| available.name == model.id.0.as_ref())
            .ok_or_else(|| unavailable_error(model))?;
        // Compatible providers may not support Anthropic's automatic prompt
        // caching; only request explicit (legacy) cache breakpoints when the
        // user has opted in via the `prompt_caching` capability.
        let cache_mode = if available.capabilities.prompt_caching {
            AnthropicPromptCacheMode::Legacy
        } else {
            AnthropicPromptCacheMode::Disabled
        };
        Ok((available_model_to_anthropic_model(available), cache_mode))
    }

    fn stream_anthropic_request(
        &self,
        config: &anthropic::Model,
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
        let provider_name = self.name.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, _cx| {
            let api_url = state.settings.api_url.clone();
            (
                state.api_key_state.key(&api_url),
                api_url,
                state.settings.custom_headers.clone(),
            )
        });

        let beta_headers = config.beta_headers();

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

    fn default_model(&self, cx: &App) -> Option<LanguageModel> {
        self.available_models(cx)
            .first()
            .map(|model| self.create_language_model(model))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<LanguageModel> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<LanguageModel> {
        self.available_models(cx)
            .iter()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn settings_view(&self, _cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.clone();
        Some(ProviderSettingsView::SubPage(SubPageProviderSettings::new(
            move |window, cx| {
                cx.new(|cx| {
                    ApiCompatibleProviderConfigurationView::new(
                        state.clone(),
                        "Anthropic",
                        API_KEY_PLACEHOLDER,
                        window,
                        cx,
                    )
                })
                .into()
            },
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }

    fn stream_completion(
        &self,
        model: &LanguageModel,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
    {
        let (config, cache_mode) = match cx.update(|cx| self.config(model, cx)) {
            Ok(config) => config,
            Err(error) => return async move { Err(error) }.boxed(),
        };
        let request_limiter = self.request_limiters.for_model(&model.id);
        let has_tools = !request.tools.is_empty();
        let request_id = config.request_id(has_tools).to_string();
        let mut request = match into_anthropic(
            request,
            request_id,
            config.default_temperature,
            config.max_output_tokens,
            config.mode.clone(),
            cache_mode,
            &self.id,
        ) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        if !config.supports_speed {
            request.speed = None;
        }
        let completion_request = self.stream_anthropic_request(&config, request, cx);
        let provider_name = self.name.clone();
        let provider_id = self.id.clone();
        let executor = cx.background_executor().clone();
        let future = request_limiter.stream(async move {
            let response = completion_request.await?;
            let events = AnthropicEventMapper::new(provider_name, provider_id).map_stream(response);
            Ok(language_model::stream_in_background(
                events.boxed(),
                executor,
            ))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}
