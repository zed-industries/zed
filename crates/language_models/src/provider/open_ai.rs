use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    ApiKeyConfiguration, ApiKeyState, AuthenticateError, CompactionResult, EnvVar,
    FastModeConfirmation, IconOrSvg, LanguageModel, LanguageModelClient,
    LanguageModelCompletionError, LanguageModelCompletionStream, LanguageModelEffortLevel,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoiceSupport, ModelRateLimiters, OPEN_AI_PROVIDER_ID, OPEN_AI_PROVIDER_NAME,
    ProviderSettingsView, RateLimiter, ReasoningMode, env_var, stream_in_background,
    unavailable_error,
};
use open_ai::{
    ResponseStreamEvent,
    responses::{
        CompactRequest, CompactedResponse, Request as ResponseRequest,
        StreamEvent as ResponsesStreamEvent, compact_response, stream_response,
    },
    stream_completion,
};
use settings::{OpenAiAvailableModel as AvailableModel, Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::IconName;

use language_model::chat_completion::ChatCompletionEventMapper;
use open_ai::completion::token_usage_from_response_usage;
pub use open_ai::completion::{
    ChatCompletionMaxTokensParameter, OpenAiResponseEventMapper, into_open_ai,
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
    request_limiters: ModelRateLimiters,
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

        Self {
            http_client,
            state,
            request_limiters: ModelRateLimiters::default(),
        }
    }

    /// Every model this provider offers, keyed by id: the built-in models,
    /// with settings entries added or overriding built-in ones.
    fn open_ai_models(&self, cx: &App) -> BTreeMap<String, open_ai::Model> {
        let mut models = BTreeMap::default();

        for model in open_ai::Model::iter() {
            if !matches!(model, open_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        for model in &Self::settings(cx).available_models {
            models.insert(
                model.name.clone(),
                open_ai::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    max_completion_tokens: model.max_completion_tokens,
                    reasoning_effort: model.reasoning_effort,
                    supports_pro_reasoning: model.capabilities.pro_reasoning,
                    supports_chat_completions: model.capabilities.chat_completions,
                    supports_images: model.capabilities.images,
                },
            );
        }

        models
    }

    /// The current configuration of `model`, if this provider still offers it.
    fn config(
        &self,
        model: &LanguageModel,
        cx: &App,
    ) -> Result<open_ai::Model, LanguageModelCompletionError> {
        self.open_ai_models(cx)
            .remove(model.id.0.as_ref())
            .ok_or_else(|| unavailable_error(model))
    }

    fn stream_chat_completion(
        &self,
        request_limiter: &RateLimiter,
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

        let future = request_limiter.stream(async move {
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
        request_limiter: &RateLimiter,
        request: ResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<ResponsesStreamEvent, open_ai::RequestError>,
            >,
        >,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = OpenAiLanguageModelProvider::api_url(cx);
            let extra_headers = OpenAiLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let provider = PROVIDER_NAME;
        let future = request_limiter.stream(async move {
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

    fn compact_response(
        &self,
        request_limiter: &RateLimiter,
        request: CompactRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<CompactedResponse, LanguageModelCompletionError>> {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = OpenAiLanguageModelProvider::api_url(cx);
            let extra_headers = OpenAiLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let provider = PROVIDER_NAME;
        let future = request_limiter.run(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            Ok(compact_response(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            )
            .await?)
        });

        future.boxed()
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

    fn default_model(&self, cx: &App) -> Option<LanguageModel> {
        self.open_ai_models(cx)
            .get(open_ai::Model::default().id())
            .map(language_model)
    }

    fn default_fast_model(&self, cx: &App) -> Option<LanguageModel> {
        self.open_ai_models(cx)
            .get(open_ai::Model::default_fast().id())
            .map(language_model)
    }

    fn recommended_models(&self, cx: &App) -> Vec<LanguageModel> {
        self.open_ai_models(cx)
            .get(open_ai::Model::FivePointSixSol.id())
            .map(language_model)
            .into_iter()
            .collect()
    }

    fn provided_models(&self, cx: &App) -> Vec<LanguageModel> {
        self.open_ai_models(cx)
            .values()
            .map(language_model)
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

impl LanguageModelClient for OpenAiLanguageModelProvider {
    fn stream_completion(
        &self,
        model: &LanguageModel,
        mut request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
    {
        let config = match cx.update(|cx| self.config(model, cx)) {
            Ok(config) => config,
            Err(error) => return async move { Err(error) }.boxed(),
        };
        let request_limiter = self.request_limiters.for_model(&model.id);
        if !config.supports_priority() {
            request.speed = None;
        }
        if config.uses_responses_api() {
            let request = match into_native_open_ai_response(request, &config) {
                Ok(request) => request,
                Err(error) => return async move { Err(error.into()) }.boxed(),
            };
            let completions = self.stream_response(&request_limiter, request, cx);
            let executor = cx.background_executor().clone();
            async move {
                let mapper = OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
                Ok(stream_in_background(
                    mapper.map_stream(completions.await?).boxed(),
                    executor,
                ))
            }
            .boxed()
        } else {
            let request = match into_open_ai(
                request,
                config.id(),
                config.supports_parallel_tool_calls(),
                config.supports_prompt_cache_key(),
                config.max_output_tokens(),
                ChatCompletionMaxTokensParameter::MaxCompletionTokens,
                None,
                false,
            ) {
                Ok(request) => request,
                Err(error) => return async move { Err(error.into()) }.boxed(),
            };
            let completions = self.stream_chat_completion(&request_limiter, request, cx);
            let executor = cx.background_executor().clone();
            async move {
                let mapper = ChatCompletionEventMapper::new();
                Ok(stream_in_background(
                    mapper.map_stream(completions.await?).boxed(),
                    executor,
                ))
            }
            .boxed()
        }
    }

    fn count_input_tokens(
        &self,
        model: &LanguageModel,
        mut request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<Option<u64>, LanguageModelCompletionError>> {
        let config = match cx.update(|cx| self.config(model, cx)) {
            Ok(config) => config,
            Err(error) => return async move { Err(error) }.boxed(),
        };
        let request_limiter = self.request_limiters.for_model(&model.id);
        if !config.uses_responses_api() {
            return async { Ok(None) }.boxed();
        }
        normalize_open_ai_response_thinking_effort(&mut request, &config);
        let request = into_open_ai_response(
            request,
            config.id(),
            config.supports_parallel_tool_calls(),
            config.supports_prompt_cache_key(),
            config.max_output_tokens(),
            default_thinking_reasoning_effort(&config),
            config
                .supported_reasoning_efforts()
                .contains(&open_ai::ReasoningEffort::None),
            &OPEN_AI_PROVIDER_ID,
        );
        let http_client = self.http_client.clone();
        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = OpenAiLanguageModelProvider::api_url(cx);
            let extra_headers = OpenAiLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });
        request_limiter
            .run(async move {
                let request = request?.into_count_tokens_request();
                let api_key = api_key.ok_or(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                })?;
                open_ai::responses::count_input_tokens(
                    http_client.as_ref(),
                    PROVIDER_NAME.0.as_str(),
                    &api_url,
                    &api_key,
                    request,
                    &extra_headers,
                )
                .await
                .map(Some)
                .map_err(Into::into)
            })
            .boxed()
    }

    fn compact(
        &self,
        model: &LanguageModel,
        mut request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<CompactionResult, LanguageModelCompletionError>> {
        let config = match cx.update(|cx| self.config(model, cx)) {
            Ok(config) => config,
            Err(error) => return async move { Err(error) }.boxed(),
        };
        let request_limiter = self.request_limiters.for_model(&model.id);
        if !config.supports_compaction() {
            return async {
                Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                    "this OpenAI model does not support explicit compaction"
                )))
            }
            .boxed();
        }

        normalize_open_ai_response_thinking_effort(&mut request, &config);
        let request = match into_open_ai_response(
            request,
            config.id(),
            config.supports_parallel_tool_calls(),
            config.supports_prompt_cache_key(),
            config.max_output_tokens(),
            default_thinking_reasoning_effort(&config),
            config
                .supported_reasoning_efforts()
                .contains(&open_ai::ReasoningEffort::None),
            &OPEN_AI_PROVIDER_ID,
        ) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let request = request.into_compact_request();
        let response = self.compact_response(&request_limiter, request, cx);
        async move {
            let response = response.await?;
            let usage = token_usage_from_response_usage(&response.usage);
            let context = response
                .into_compacted_context(OPEN_AI_PROVIDER_ID)
                .map_err(LanguageModelCompletionError::Other)?;
            Ok(CompactionResult { context, usage })
        }
        .boxed()
    }
}

fn into_native_open_ai_response(
    mut request: LanguageModelRequest,
    model: &open_ai::Model,
) -> Result<ResponseRequest> {
    normalize_open_ai_response_thinking_effort(&mut request, model);
    // Like effort, Pro only applies while thinking is on. Models without a thinking toggle can't
    // turn it off, so their `thinking_allowed` doesn't reflect a user choice.
    let thinking_turned_off =
        !request.thinking_allowed && supports_selectable_thinking_effort(model);
    let pro_reasoning = request.reasoning_mode == ReasoningMode::Pro
        && model.supports_pro_reasoning()
        && !thinking_turned_off;
    let mut request = into_open_ai_response(
        request,
        model.id(),
        model.supports_parallel_tool_calls(),
        model.supports_prompt_cache_key(),
        model.max_output_tokens(),
        default_thinking_reasoning_effort(model),
        model
            .supported_reasoning_efforts()
            .contains(&open_ai::ReasoningEffort::None),
        &OPEN_AI_PROVIDER_ID,
    )?;
    if pro_reasoning {
        let reasoning = request
            .reasoning
            .get_or_insert(open_ai::responses::ReasoningConfig {
                effort: None,
                summary: None,
                mode: None,
            });
        reasoning.mode = Some(ReasoningMode::Pro);
    }
    Ok(request)
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
                supports_disabling_thinking: None,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn native_standard_reasoning_preserves_payload() -> Result<()> {
        let standard = into_native_open_ai_response(
            LanguageModelRequest {
                thinking_allowed: true,
                thinking_effort: Some("high".into()),
                reasoning_mode: ReasoningMode::Standard,
                ..Default::default()
            },
            &open_ai::Model::SixSol,
        )?;
        assert_eq!(
            serde_json::to_value(standard)?.get("reasoning"),
            Some(&json!({ "effort": "high", "summary": "auto" })),
        );
        Ok(())
    }

    #[test]
    fn native_pro_reasoning_preserves_effort_summary_and_include() -> Result<()> {
        assert!(language_model(&open_ai::Model::SixSol).supports_pro_reasoning);
        for effort in ["low", "high", "max"] {
            let response = into_native_open_ai_response(
                LanguageModelRequest {
                    thinking_allowed: true,
                    thinking_effort: Some(effort.into()),
                    reasoning_mode: ReasoningMode::Pro,
                    ..Default::default()
                },
                &open_ai::Model::SixSol,
            )?;
            let serialized = serde_json::to_value(response)?;
            assert_eq!(
                serialized.get("reasoning"),
                Some(&json!({ "effort": effort, "summary": "auto", "mode": "pro" })),
            );
            assert_eq!(
                serialized.get("include"),
                Some(&json!(["reasoning.encrypted_content"])),
            );
        }
        Ok(())
    }

    #[test]
    fn native_pro_reasoning_is_dropped_while_thinking_is_off() -> Result<()> {
        for (model, reasoning) in [
            (open_ai::Model::SixSol, Some(json!({ "effort": "none" }))),
            (open_ai::Model::SixPointOneSol, None),
        ] {
            let response = into_native_open_ai_response(
                LanguageModelRequest {
                    thinking_allowed: false,
                    thinking_effort: Some("high".into()),
                    reasoning_mode: ReasoningMode::Pro,
                    ..Default::default()
                },
                &model,
            )?;
            assert_eq!(
                serde_json::to_value(response)?.get("reasoning"),
                reasoning.as_ref(),
                "{}",
                model.id(),
            );
        }
        Ok(())
    }

    #[test]
    fn native_unsupported_models_ignore_stale_pro_reasoning() -> Result<()> {
        for model in [
            open_ai::Model::Four,
            open_ai::Model::FivePointFourPro,
            open_ai::Model::FivePointFivePro,
        ] {
            let request = LanguageModelRequest {
                thinking_allowed: true,
                thinking_effort: Some("high".into()),
                ..Default::default()
            };
            let baseline = into_native_open_ai_response(request.clone(), &model)?;
            let response = into_native_open_ai_response(
                LanguageModelRequest {
                    reasoning_mode: ReasoningMode::Pro,
                    ..request
                },
                &model,
            )?;
            assert_eq!(
                serde_json::to_value(response)?,
                serde_json::to_value(baseline)?
            );
            assert!(!language_model(&model).supports_pro_reasoning);
        }
        Ok(())
    }

    #[test]
    fn native_custom_pro_reasoning_requires_opt_in_and_responses() -> Result<()> {
        for supports_pro_reasoning in [false, true] {
            for supports_chat_completions in [false, true] {
                let model = open_ai::Model::Custom {
                    name: "gpt-6-sol".into(),
                    display_name: None,
                    max_tokens: 128_000,
                    max_output_tokens: None,
                    max_completion_tokens: None,
                    reasoning_effort: None,
                    supports_pro_reasoning,
                    supports_chat_completions,
                    supports_images: true,
                };
                let supported = supports_pro_reasoning && !supports_chat_completions;
                assert_eq!(language_model(&model).supports_pro_reasoning, supported);
                assert!(!language_model(&model).supports_thinking);
                // Without a thinking toggle, the stored thinking state must not suppress Pro.
                for thinking_allowed in [false, true] {
                    let response = into_native_open_ai_response(
                        LanguageModelRequest {
                            thinking_allowed,
                            reasoning_mode: ReasoningMode::Pro,
                            ..Default::default()
                        },
                        &model,
                    )?;
                    let serialized = serde_json::to_value(response)?;
                    assert_eq!(
                        serialized.get("reasoning"),
                        supported.then(|| json!({ "mode": "pro" })).as_ref(),
                    );
                }
            }
        }
        Ok(())
    }

    /// Run with `OPENAI_API_KEY=... cargo test -p language_models live_gpt_6_1_sol -- --ignored`.
    #[test]
    #[ignore = "sends paid requests to the OpenAI API"]
    fn live_gpt_6_1_sol_runs_in_requested_reasoning_mode() -> Result<()> {
        use anyhow::Context as _;
        use futures::AsyncReadExt as _;
        use http_client::{AsyncBody, HttpRequestExt as _, Method, Request as HttpRequest};
        use language_model::{LanguageModelCompletionEvent, LanguageModelRequestMessage, Role};
        use std::time::Duration;

        let api_key = std::env::var(API_KEY_ENV_VAR_NAME).context("OPENAI_API_KEY is not set")?;
        let client = reqwest_client::ReqwestClient::user_agent("Zed reasoning mode test")?;
        for (thinking_allowed, expected_mode) in [(true, "pro"), (false, "standard")] {
            let request = into_native_open_ai_response(
                LanguageModelRequest {
                    messages: vec![LanguageModelRequestMessage {
                        role: Role::User,
                        content: vec!["Reply with the single word OK.".into()],
                        cache: false,
                        reasoning_details: None,
                    }],
                    thinking_allowed,
                    thinking_effort: Some("low".into()),
                    reasoning_mode: ReasoningMode::Pro,
                    ..Default::default()
                },
                &open_ai::Model::SixPointOneSol,
            )?;
            let http_request = HttpRequest::builder()
                .method(Method::POST)
                .uri(format!("{}/responses", open_ai::OPEN_AI_API_URL))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_key.trim()))
                .timeout(Duration::from_secs(300))
                .body(AsyncBody::from(serde_json::to_string(&request)?))?;
            let (status, body) = futures::executor::block_on(async {
                let mut response = client.send(http_request).await?;
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                anyhow::Ok((response.status(), body))
            })?;
            anyhow::ensure!(status.is_success(), "{status}: {body}");

            // Zed's own mapper must accept every event a Pro stream emits.
            let mut mapper = OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
            let mut completed_response = None;
            let mut text = String::new();
            for data in body
                .lines()
                .filter_map(|line| line.strip_prefix("data: "))
                .filter(|data| *data != "[DONE]")
            {
                let event: serde_json::Value = serde_json::from_str(data)?;
                if event["type"] == "response.completed" {
                    completed_response = Some(event["response"].clone());
                }
                for mapped in mapper.map_event(serde_json::from_value(event)?) {
                    if let LanguageModelCompletionEvent::Text(chunk) = mapped? {
                        text.push_str(&chunk);
                    }
                }
            }
            let completed_response =
                completed_response.context("stream ended without response.completed")?;
            assert_eq!(
                completed_response["reasoning"]["mode"], expected_mode,
                "thinking_allowed: {thinking_allowed}, reasoning: {}",
                completed_response["reasoning"],
            );
            assert!(!text.trim().is_empty(), "no text in the mapped stream");
        }
        Ok(())
    }

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
            supports_pro_reasoning: false,
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

fn language_model(model: &open_ai::Model) -> LanguageModel {
    use open_ai::Model;
    let supports_images = match model {
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
        | Model::SixAstra
        | Model::SixSol
        | Model::SixLuna
        | Model::SixPointOneSol
        | Model::O3 => true,
        Model::Four => false,
        Model::Custom {
            supports_images, ..
        } => *supports_images,
    };
    LanguageModel {
        supports_tools: true,
        supports_images,
        tool_choice_support: LanguageModelToolChoiceSupport::ALL,
        supports_streaming_tools: true,
        supports_thinking: supports_selectable_thinking_effort(model),
        supports_fast_mode: model.supports_priority(),
        supports_pro_reasoning: model.supports_pro_reasoning(),
        supports_server_side_compaction: model.supports_compaction(),
        supports_explicit_compaction: model.supports_compaction(),
        supported_effort_levels: supported_thinking_effort_levels(model).into(),
        supports_split_token_display: true,
        max_output_tokens: model.max_output_tokens(),
        ..LanguageModel::new(
            LanguageModelId::from(model.id().to_string()),
            LanguageModelName::from(model.display_name().to_string()),
            PROVIDER_ID,
            PROVIDER_NAME,
            format!("openai/{}", model.id()),
            model.max_token_count(),
        )
    }
}
