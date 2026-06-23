use anyhow::Result;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AppContext, AsyncApp, Entity, Task, Window};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolSchemaFormat, RateLimiter,
};
use open_ai::{
    ResponseStreamEvent,
    responses::{Request as ResponseRequest, StreamEvent as ResponsesStreamEvent, stream_response},
    stream_completion,
};
use settings::Settings;
use std::sync::Arc;
use ui::IconName;

use crate::provider::api_compatible::{
    ApiCompatibleProviderConfigurationView, ApiCompatibleProviderSettings,
    ApiCompatibleProviderState,
};
use crate::provider::open_ai::{
    OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai, into_open_ai_response,
};
pub use settings::OpenAiCompatibleAvailableModel as AvailableModel;
pub use settings::OpenAiCompatibleModelCapabilities as ModelCapabilities;

const API_KEY_PLACEHOLDER: &str = "000000000000000000000000000000000000000000000000000";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiCompatibleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

impl ApiCompatibleProviderSettings for OpenAiCompatibleSettings {
    fn api_url(&self) -> &str {
        &self.api_url
    }
}

pub type State = ApiCompatibleProviderState<OpenAiCompatibleSettings>;

pub struct OpenAiCompatibleLanguageModelProvider {
    id: LanguageModelProviderId,
    name: LanguageModelProviderName,
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

impl OpenAiCompatibleLanguageModelProvider {
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
                    .openai_compatible
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
        Arc::new(OpenAiCompatibleLanguageModel {
            id: LanguageModelId::from(model.name.clone()),
            provider_id: self.id.clone(),
            provider_name: self.name.clone(),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for OpenAiCompatibleLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenAiCompatibleLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelProviderName {
        self.name.clone()
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAiCompat)
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
                "OpenAI",
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

pub struct OpenAiCompatibleLanguageModel {
    id: LanguageModelId,
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
    model: AvailableModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenAiCompatibleLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, _cx| {
            let api_url = &state.settings.api_url;
            (
                state.api_key_state.key(api_url),
                state.settings.api_url.clone(),
                state.settings.custom_headers.clone(),
            )
        });

        let provider = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = stream_completion(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn stream_response(
        &self,
        request: ResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponsesStreamEvent>>>>
    {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, _cx| {
            let api_url = &state.settings.api_url;
            (
                state.api_key_state.key(api_url),
                state.settings.api_url.clone(),
                state.settings.custom_headers.clone(),
            )
        });

        let provider = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = stream_response(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

fn default_thinking_reasoning_effort(model: &AvailableModel) -> Option<open_ai::ReasoningEffort> {
    model
        .reasoning_effort
        .filter(|effort| *effort != open_ai::ReasoningEffort::None)
}

fn supported_thinking_effort_levels(model: &AvailableModel) -> Vec<LanguageModelEffortLevel> {
    let Some(default_effort) = default_thinking_reasoning_effort(model) else {
        return Vec::new();
    };

    open_ai::ReasoningEffort::OPENAI_COMPATIBLE_SELECTABLE
        .into_iter()
        .map(|effort| LanguageModelEffortLevel {
            name: effort.label().into(),
            value: effort.value().into(),
            is_default: effort == default_effort,
        })
        .collect()
}

fn selected_thinking_reasoning_effort(
    request: &LanguageModelRequest,
) -> Option<open_ai::ReasoningEffort> {
    request
        .thinking_effort
        .as_deref()
        .and_then(|effort| effort.parse::<open_ai::ReasoningEffort>().ok())
        .filter(|effort| *effort != open_ai::ReasoningEffort::None)
}

fn chat_completion_max_tokens_parameter(
    model: &AvailableModel,
) -> crate::provider::open_ai::ChatCompletionMaxTokensParameter {
    if model.capabilities.max_tokens_parameter {
        crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxTokens
    } else {
        crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxCompletionTokens
    }
}

fn supports_none_reasoning_effort(model: &AvailableModel) -> bool {
    model.reasoning_effort.is_some()
}

fn chat_completion_reasoning_effort(
    request: &LanguageModelRequest,
    model: &AvailableModel,
) -> Option<open_ai::ReasoningEffort> {
    if model.reasoning_effort == Some(open_ai::ReasoningEffort::None) {
        return Some(open_ai::ReasoningEffort::None);
    }

    if request.thinking_allowed {
        selected_thinking_reasoning_effort(request)
            .or_else(|| default_thinking_reasoning_effort(model))
    } else if supports_none_reasoning_effort(model) {
        Some(open_ai::ReasoningEffort::None)
    } else {
        None
    }
}

fn disable_response_thinking_for_none_effort(
    request: &mut LanguageModelRequest,
    model: &AvailableModel,
) {
    if model.reasoning_effort == Some(open_ai::ReasoningEffort::None) {
        request.thinking_allowed = false;
        request.thinking_effort = None;
    }
}

impl LanguageModel for OpenAiCompatibleLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(
            self.model
                .display_name
                .clone()
                .unwrap_or_else(|| self.model.name.clone()),
        )
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        self.provider_id.clone()
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        self.provider_name.clone()
    }

    fn supports_tools(&self) -> bool {
        self.model.capabilities.tools
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchemaSubset
    }

    fn supports_images(&self) -> bool {
        self.model.capabilities.images
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => self.model.capabilities.tools,
            LanguageModelToolChoice::Any => self.model.capabilities.tools,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        default_thinking_reasoning_effort(&self.model).is_some()
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        supported_thinking_effort_levels(&self.model)
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("openai/{}", self.model.name)
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens
    }

    fn stream_completion(
        &self,
        mut request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        // `speed` can leak in from a parent thread's model; this provider never
        // supports fast mode, and arbitrary compatible endpoints reject `service_tier`.
        if !self.supports_fast_mode() {
            request.speed = None;
        }

        if self.model.capabilities.chat_completions {
            let reasoning_effort = chat_completion_reasoning_effort(&request, &self.model);
            let request = into_open_ai(
                request,
                &self.model.name,
                self.model.capabilities.parallel_tool_calls,
                self.model.capabilities.prompt_cache_key,
                self.max_output_tokens(),
                chat_completion_max_tokens_parameter(&self.model),
                reasoning_effort,
                self.model.capabilities.interleaved_reasoning,
            );
            let completions = self.stream_completion(request, cx);
            async move {
                let mapper = OpenAiEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        } else {
            disable_response_thinking_for_none_effort(&mut request, &self.model);
            let request = into_open_ai_response(
                request,
                &self.model.name,
                self.model.capabilities.parallel_tool_calls,
                self.model.capabilities.prompt_cache_key,
                self.max_output_tokens(),
                default_thinking_reasoning_effort(&self.model),
                supports_none_reasoning_effort(&self.model),
            );
            let completions = self.stream_response(request, cx);
            async move {
                let mapper = OpenAiResponseEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    fn available_model(reasoning_effort: Option<open_ai::ReasoningEffort>) -> AvailableModel {
        AvailableModel {
            name: "custom-model".to_string(),
            display_name: None,
            max_tokens: 128_000,
            max_output_tokens: None,
            max_completion_tokens: None,
            reasoning_effort,
            capabilities: ModelCapabilities {
                chat_completions: false,
                ..Default::default()
            },
        }
    }

    #[test]
    fn configured_reasoning_effort_supports_thinking() {
        assert_eq!(
            default_thinking_reasoning_effort(&available_model(Some(
                open_ai::ReasoningEffort::High
            ))),
            Some(open_ai::ReasoningEffort::High)
        );
    }

    #[test]
    fn missing_or_none_reasoning_effort_does_not_support_thinking() {
        assert_eq!(
            default_thinking_reasoning_effort(&available_model(None)),
            None
        );
        assert_eq!(
            default_thinking_reasoning_effort(&available_model(Some(
                open_ai::ReasoningEffort::None
            ))),
            None
        );
    }

    #[test]
    fn supported_thinking_effort_levels_use_configured_effort_as_default() {
        let effort_levels = supported_thinking_effort_levels(&available_model(Some(
            open_ai::ReasoningEffort::High,
        )));
        let values = effort_levels
            .iter()
            .map(|level| level.value.as_ref())
            .collect::<Vec<_>>();

        assert_eq!(values, ["minimal", "low", "medium", "high", "xhigh"]);
        assert_eq!(
            effort_levels
                .iter()
                .find(|level| level.is_default)
                .map(|level| level.value.as_ref()),
            Some("high")
        );
    }

    #[test]
    fn supported_thinking_effort_levels_hide_missing_or_none_effort() {
        assert!(supported_thinking_effort_levels(&available_model(None)).is_empty());
        assert!(
            supported_thinking_effort_levels(&available_model(Some(
                open_ai::ReasoningEffort::None
            )))
            .is_empty()
        );
    }

    #[test]
    fn chat_completion_reasoning_effort_honors_request_and_configured_effort() {
        let model = available_model(Some(open_ai::ReasoningEffort::Medium));
        let mut request = LanguageModelRequest {
            thinking_allowed: true,
            ..Default::default()
        };

        assert_eq!(
            chat_completion_reasoning_effort(&request, &model),
            Some(open_ai::ReasoningEffort::Medium)
        );

        request.thinking_effort = Some("high".to_string());
        assert_eq!(
            chat_completion_reasoning_effort(&request, &model),
            Some(open_ai::ReasoningEffort::High)
        );

        request.thinking_effort = Some("not-supported".to_string());
        assert_eq!(
            chat_completion_reasoning_effort(&request, &model),
            Some(open_ai::ReasoningEffort::Medium)
        );

        request.thinking_allowed = false;
        assert_eq!(
            chat_completion_reasoning_effort(&request, &model),
            Some(open_ai::ReasoningEffort::None)
        );
    }

    #[test]
    fn chat_completion_reasoning_effort_omits_missing_effort() {
        let model = available_model(None);
        let request = LanguageModelRequest {
            thinking_allowed: false,
            ..Default::default()
        };

        assert_eq!(chat_completion_reasoning_effort(&request, &model), None);
    }

    #[test]
    fn chat_completion_reasoning_effort_preserves_explicit_none() {
        let model = available_model(Some(open_ai::ReasoningEffort::None));
        let request = LanguageModelRequest {
            thinking_allowed: true,
            thinking_effort: Some("high".to_string()),
            ..Default::default()
        };

        assert_eq!(
            chat_completion_reasoning_effort(&request, &model),
            Some(open_ai::ReasoningEffort::None)
        );
    }

    #[test]
    fn chat_completion_max_tokens_parameter_defaults_to_max_completion_tokens() {
        let model = available_model(Some(open_ai::ReasoningEffort::Medium));

        assert_eq!(
            chat_completion_max_tokens_parameter(&model),
            crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxCompletionTokens
        );
    }

    #[test]
    fn chat_completion_max_tokens_parameter_uses_max_tokens_when_configured() {
        let mut model = available_model(Some(open_ai::ReasoningEffort::Medium));
        model.capabilities.max_tokens_parameter = true;

        assert_eq!(
            chat_completion_max_tokens_parameter(&model),
            crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxTokens
        );
    }

    #[test]
    fn response_request_includes_reasoning_when_effort_is_configured() {
        let model = available_model(Some(open_ai::ReasoningEffort::High));
        let request = LanguageModelRequest {
            thinking_allowed: true,
            ..Default::default()
        };

        let request = into_open_ai_response(
            request,
            &model.name,
            model.capabilities.parallel_tool_calls,
            model.capabilities.prompt_cache_key,
            model.max_output_tokens,
            default_thinking_reasoning_effort(&model),
            supports_none_reasoning_effort(&model),
        );
        let serialized = serde_json::to_value(request).unwrap();

        assert_eq!(
            serialized["reasoning"],
            json!({ "effort": "high", "summary": "auto" })
        );
        assert_eq!(
            serialized["include"],
            json!(["reasoning.encrypted_content"])
        );
    }

    #[test]
    fn response_request_omits_reasoning_when_effort_is_missing() {
        let model = available_model(None);
        let request = LanguageModelRequest {
            thinking_allowed: true,
            ..Default::default()
        };

        let request = into_open_ai_response(
            request,
            &model.name,
            model.capabilities.parallel_tool_calls,
            model.capabilities.prompt_cache_key,
            model.max_output_tokens,
            default_thinking_reasoning_effort(&model),
            supports_none_reasoning_effort(&model),
        );
        let serialized = serde_json::to_value(request).unwrap();

        assert_eq!(serialized.get("reasoning"), None);
        assert_eq!(serialized.get("include"), None);
    }

    #[test]
    fn chat_completion_request_includes_selected_reasoning_effort() {
        let mut model = available_model(Some(open_ai::ReasoningEffort::Medium));
        model.capabilities.chat_completions = true;
        let request = LanguageModelRequest {
            thinking_allowed: true,
            thinking_effort: Some("high".to_string()),
            ..Default::default()
        };
        let reasoning_effort = chat_completion_reasoning_effort(&request, &model);

        let request = into_open_ai(
            request,
            &model.name,
            model.capabilities.parallel_tool_calls,
            model.capabilities.prompt_cache_key,
            model.max_output_tokens,
            chat_completion_max_tokens_parameter(&model),
            reasoning_effort,
            model.capabilities.interleaved_reasoning,
        );
        let serialized = serde_json::to_value(request).unwrap();

        assert_eq!(serialized["reasoning_effort"], json!("high"));
    }

    #[test]
    fn configured_reasoning_effort_supports_none_reasoning_effort() {
        assert!(supports_none_reasoning_effort(&available_model(Some(
            open_ai::ReasoningEffort::Medium
        ))));
        assert!(supports_none_reasoning_effort(&available_model(Some(
            open_ai::ReasoningEffort::None
        ))));
        assert!(!supports_none_reasoning_effort(&available_model(None)));
    }

    #[test]
    fn response_thinking_effort_preserves_explicit_none() {
        let model = available_model(Some(open_ai::ReasoningEffort::None));
        let mut request = LanguageModelRequest {
            thinking_allowed: true,
            thinking_effort: Some("high".to_string()),
            ..Default::default()
        };

        disable_response_thinking_for_none_effort(&mut request, &model);
        assert!(!request.thinking_allowed);
        assert_eq!(request.thinking_effort, None);
    }
}
