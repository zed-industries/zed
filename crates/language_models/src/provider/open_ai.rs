use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    ApiKeyConfiguration, ApiKeyState, AuthenticateError, EnvVar, FastModeConfirmation, IconOrSvg,
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelEffortLevel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, OPEN_AI_PROVIDER_ID, OPEN_AI_PROVIDER_NAME,
    ProviderSettingsView, RateLimiter, env_var,
};
use open_ai::{
    ResponseStreamEvent,
    responses::{Request as ResponseRequest, StreamEvent as ResponsesStreamEvent, stream_response},
    stream_completion,
};
use settings::{OpenAiAvailableModel as AvailableModel, Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::IconName;

pub use open_ai::completion::{
    ChatCompletionMaxTokensParameter, OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai,
    into_open_ai_response,
};

const PROVIDER_ID: LanguageModelProviderId = OPEN_AI_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = OPEN_AI_PROVIDER_NAME;

const API_KEY_ENV_VAR_NAME: &str = "OPENAI_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

pub struct OpenAiLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = OpenAiLanguageModelProvider::api_url(cx);
        self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = OpenAiLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }
}

impl OpenAiLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let credentials_provider = this.credentials_provider.clone();
                let api_url = Self::api_url(cx);
                this.api_key_state.handle_url_change(
                    api_url,
                    |this| &mut this.api_key_state,
                    credentials_provider,
                    cx,
                );
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                credentials_provider,
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: open_ai::Model) -> Arc<dyn LanguageModel> {
        Arc::new(OpenAiLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &OpenAiSettings {
        &crate::AllLanguageModelSettings::get_global(cx).openai
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            open_ai::OPEN_AI_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for OpenAiLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenAiLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAi)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_ai::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_ai::Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        // Add base models from open_ai::Model::iter()
        for model in open_ai::Model::iter() {
            if !matches!(model, open_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in &OpenAiLanguageModelProvider::settings(cx).available_models {
            models.insert(
                model.name.clone(),
                open_ai::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    max_completion_tokens: model.max_completion_tokens,
                    reasoning_effort: model.reasoning_effort,
                    supports_chat_completions: model.capabilities.chat_completions,
                    supports_images: model.capabilities.images,
                },
            );
        }

        models
            .into_values()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn settings_view(&self, cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.read(cx);
        Some(ProviderSettingsView::ApiKey(ApiKeyConfiguration::new(
            state.api_key_state.has_key(),
            state.api_key_state.is_from_env_var(),
            state.api_key_state.env_var_name().clone(),
            "https://platform.openai.com/api-keys".into(),
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }

    fn fast_mode_confirmation(&self, _cx: &App) -> Option<FastModeConfirmation> {
        Some(FastModeConfirmation {
            title: "Enable Fast Mode for OpenAI?".into(),
            message: "Fast mode sends requests using OpenAI's Priority processing tier, which \
                targets significantly lower latency than the standard tier and is billed at a \
                premium per-token rate."
                .into(),
        })
    }
}

fn default_thinking_reasoning_effort(model: &open_ai::Model) -> Option<open_ai::ReasoningEffort> {
    use open_ai::ReasoningEffort;

    model
        .reasoning_effort()
        .filter(|effort| open_ai_reasoning_effort_is_supported(*effort))
        .or_else(|| {
            let supported_efforts = model.supported_reasoning_efforts();
            if supported_efforts.contains(&ReasoningEffort::Medium) {
                Some(ReasoningEffort::Medium)
            } else {
                supported_efforts
                    .iter()
                    .copied()
                    .find(|effort| open_ai_reasoning_effort_is_supported(*effort))
            }
        })
}

fn open_ai_reasoning_effort_is_supported(effort: open_ai::ReasoningEffort) -> bool {
    effort != open_ai::ReasoningEffort::None
}

fn normalize_open_ai_response_thinking_effort(
    request: &mut LanguageModelRequest,
    model: &open_ai::Model,
) {
    let selected_effort_is_supported = request
        .thinking_effort
        .as_deref()
        .and_then(|effort| effort.parse::<open_ai::ReasoningEffort>().ok())
        .is_some_and(|effort| {
            open_ai_reasoning_effort_is_supported(effort)
                && model.supported_reasoning_efforts().contains(&effort)
        });

    if !selected_effort_is_supported {
        request.thinking_effort = None;
    }
}

fn supports_selectable_thinking_effort(model: &open_ai::Model) -> bool {
    model.uses_responses_api()
        && model
            .supported_reasoning_efforts()
            .iter()
            .any(|effort| open_ai_reasoning_effort_is_supported(*effort))
}

fn supported_thinking_effort_levels(model: &open_ai::Model) -> Vec<LanguageModelEffortLevel> {
    if !supports_selectable_thinking_effort(model) {
        return Vec::new();
    }

    let default_effort = default_thinking_reasoning_effort(model);
    model
        .supported_reasoning_efforts()
        .iter()
        .copied()
        .filter_map(|effort| {
            if !open_ai_reasoning_effort_is_supported(effort) {
                return None;
            }

            Some(LanguageModelEffortLevel {
                name: effort.label().into(),
                value: effort.value().into(),
                is_default: Some(effort) == default_effort,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_thinking_effort_levels_hide_none() {
        let effort_levels = supported_thinking_effort_levels(&open_ai::Model::FivePointTwo);
        let values = effort_levels
            .iter()
            .map(|level| level.value.as_ref())
            .collect::<Vec<_>>();

        assert_eq!(values, ["low", "medium", "high", "xhigh"]);
        assert_eq!(
            effort_levels
                .iter()
                .find(|level| level.is_default)
                .map(|level| level.value.as_ref()),
            Some("medium")
        );
    }

    #[test]
    fn models_supporting_only_none_have_no_selectable_thinking_effort() {
        let model = open_ai::Model::Custom {
            name: "custom-model".to_string(),
            display_name: None,
            max_tokens: 128_000,
            max_output_tokens: None,
            max_completion_tokens: None,
            reasoning_effort: Some(open_ai::ReasoningEffort::None),
            supports_chat_completions: false,
            supports_images: true,
        };

        assert!(!supports_selectable_thinking_effort(&model));
        assert!(supported_thinking_effort_levels(&model).is_empty());
        assert!(
            model
                .supported_reasoning_efforts()
                .contains(&open_ai::ReasoningEffort::None)
        );
    }
}

pub struct OpenAiLanguageModel {
    id: LanguageModelId,
    model: open_ai::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenAiLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>>>
    {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = OpenAiLanguageModelProvider::api_url(cx);
            let extra_headers = OpenAiLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let future = self.request_limiter.stream(async move {
            let provider = PROVIDER_NAME;
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

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = OpenAiLanguageModelProvider::api_url(cx);
            let extra_headers = OpenAiLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let provider = PROVIDER_NAME;
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

impl LanguageModel for OpenAiLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        use open_ai::Model;
        match &self.model {
            Model::FourOmniMini
            | Model::Five
            | Model::FiveMini
            | Model::FiveNano
            | Model::FivePointOne
            | Model::FivePointTwo
            | Model::FivePointThreeCodex
            | Model::FivePointFour
            | Model::FivePointFourMini
            | Model::FivePointFourNano
            | Model::FivePointFourPro
            | Model::FivePointFive
            | Model::FivePointFivePro
            | Model::FivePointSixSol
            | Model::FivePointSixTerra
            | Model::FivePointSixLuna
            | Model::O3 => true,
            Model::Four => false,
            Model::Custom {
                supports_images, ..
            } => *supports_images,
        }
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => true,
            LanguageModelToolChoice::Any => true,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        supports_selectable_thinking_effort(&self.model)
    }

    fn supports_fast_mode(&self) -> bool {
        self.model.supports_priority()
    }

    fn supports_server_side_compaction(&self) -> bool {
        self.model.supports_compaction()
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        supported_thinking_effort_levels(&self.model)
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("openai/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
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
        if !self.model.supports_priority() {
            request.speed = None;
        }
        if self.model.uses_responses_api() {
            normalize_open_ai_response_thinking_effort(&mut request, &self.model);
            let request = into_open_ai_response(
                request,
                self.model.id(),
                self.model.supports_parallel_tool_calls(),
                self.model.supports_prompt_cache_key(),
                self.max_output_tokens(),
                default_thinking_reasoning_effort(&self.model),
                self.model
                    .supported_reasoning_efforts()
                    .contains(&open_ai::ReasoningEffort::None),
            );
            let completions = self.stream_response(request, cx);
            async move {
                let mapper = OpenAiResponseEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        } else {
            let request = match into_open_ai(
                request,
                self.model.id(),
                self.model.supports_parallel_tool_calls(),
                self.model.supports_prompt_cache_key(),
                self.max_output_tokens(),
                ChatCompletionMaxTokensParameter::MaxCompletionTokens,
                None,
                false,
            ) {
                Ok(request) => request,
                Err(error) => return async move { Err(error.into()) }.boxed(),
            };
            let completions = self.stream_completion(request, cx);
            async move {
                let mapper = OpenAiEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        }
    }
}
