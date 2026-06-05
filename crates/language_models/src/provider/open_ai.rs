use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use feature_flags::{FeatureFlagAppExt as _, HandoffFeatureFlag};
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, TaskExt, Window};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, FastModeConfirmation, IconOrSvg, LanguageModel,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelEffortLevel,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, OPEN_AI_PROVIDER_ID, OPEN_AI_PROVIDER_NAME, RateLimiter, env_var,
};
use menu;
use open_ai::{
    OPEN_AI_API_URL, RequestError, ResponseStreamEvent,
    responses::{
        ContextManagement, Request as ResponseRequest, StreamEvent as ResponsesStreamEvent,
        stream_response,
    },
    stream_completion,
};
use settings::{OpenAiAvailableModel as AvailableModel, Settings, SettingsStore};
use std::collections::HashSet;
use std::sync::{Arc, LazyLock, RwLock};
use strum::IntoEnumIterator;
use ui::{ButtonLink, ConfiguredApiCard, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

pub use open_ai::completion::{
    OpenAiEventMapper, OpenAiResponseEventMapper, ResponsesRequestConfig, into_open_ai,
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

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
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

/// Overrides the computed native-compaction `compact_threshold` with a fixed
/// token count. Intended for manual testing: set it low (e.g. `2000`) to make
/// OpenAI compact after a message or two instead of near the context limit.
static NATIVE_COMPACTION_THRESHOLD_ENV_VAR: LazyLock<EnvVar> =
    env_var!("ZED_NATIVE_COMPACTION_THRESHOLD");

/// Token threshold at which we ask OpenAI to run server-side compaction. We
/// reserve room for a full-length response so compaction triggers before the
/// model would otherwise run out of context. Shared by the OpenAI and
/// ChatGPT-subscription providers.
pub(crate) fn native_compaction_threshold(
    max_token_count: u64,
    max_output_tokens: Option<u64>,
) -> u64 {
    if let Some(override_threshold) = NATIVE_COMPACTION_THRESHOLD_ENV_VAR
        .value
        .as_ref()
        .and_then(|value| value.parse().ok())
    {
        return override_threshold;
    }
    let reserved = max_output_tokens.unwrap_or(max_token_count / 4);
    max_token_count.saturating_sub(reserved).max(1)
}

/// Process-wide set of model telemetry ids that returned a 400
/// `unsupported_parameter` for `compact_threshold`. OpenAI publishes no static
/// per-model capability list, so we learn it at runtime: once a model lands
/// here we stop requesting native compaction for it and fall back to summary
/// compaction.
static NATIVE_COMPACTION_UNSUPPORTED: LazyLock<RwLock<HashSet<String>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

/// Whether a model previously rejected native compaction at runtime.
pub(crate) fn native_compaction_unsupported(model_telemetry_id: &str) -> bool {
    NATIVE_COMPACTION_UNSUPPORTED
        .read()
        .is_ok_and(|set| set.contains(model_telemetry_id))
}

fn mark_native_compaction_unsupported(model_telemetry_id: &str) {
    if let Ok(mut set) = NATIVE_COMPACTION_UNSUPPORTED.write() {
        set.insert(model_telemetry_id.to_string());
    }
}

/// Clears the process-wide native-compaction cache so a test can't leak runtime
/// state (the `static` outlives individual tests) into later tests.
#[cfg(test)]
pub(crate) fn clear_native_compaction_unsupported_for_test() {
    if let Ok(mut set) = NATIVE_COMPACTION_UNSUPPORTED.write() {
        set.clear();
    }
}

fn is_unsupported_compaction_error(error: &RequestError) -> bool {
    let RequestError::HttpResponseError {
        status_code, body, ..
    } = error
    else {
        return false;
    };
    if status_code.as_u16() != 400 {
        return false;
    }

    #[derive(serde::Deserialize)]
    struct ErrorBody {
        error: ErrorDetails,
    }

    #[derive(serde::Deserialize)]
    struct ErrorDetails {
        code: Option<String>,
        param: Option<String>,
    }

    match serde_json::from_str::<ErrorBody>(body) {
        Ok(parsed) => {
            parsed.error.code.as_deref() == Some("unsupported_parameter")
                && parsed.error.param.as_deref() == Some("compact_threshold")
        }
        // Fall back to a substring heuristic when the body isn't the JSON shape
        // we expect, so a format change still degrades gracefully.
        Err(_) => body.contains("compact_threshold") && body.contains("unsupported_parameter"),
    }
}

/// Sends a Responses request, transparently retrying once without
/// `context_management` if the model rejects native compaction. On rejection we
/// record the model (keyed by telemetry id) so subsequent turns skip native
/// compaction and use summary compaction instead.
///
/// Note the rejected turn itself runs with neither native nor summary
/// compaction: `uses_native_compaction()` was still true when the request was
/// built, so the agent skipped its summary pass, and the retry here strips
/// `context_management` entirely. That turn therefore sends the full,
/// uncompacted history, which can itself overflow the context window if we were
/// near the limit. We accept this one-turn degradation because the model is now
/// marked unsupported, so the *next* turn's `compaction_message_target_ix`
/// summary pass brings the window back under control.
pub(crate) async fn stream_response_with_compaction_fallback(
    client: &dyn HttpClient,
    provider_name: &str,
    api_url: &str,
    api_key: &str,
    mut request: ResponseRequest,
    extra_headers: &CustomHeaders,
    model_telemetry_id: &str,
) -> Result<futures::stream::BoxStream<'static, Result<ResponsesStreamEvent>>, RequestError> {
    match stream_response(
        client,
        provider_name,
        api_url,
        api_key,
        &request,
        extra_headers,
    )
    .await
    {
        Err(error)
            if !request.context_management.is_empty()
                && is_unsupported_compaction_error(&error) =>
        {
            log::warn!(
                "{model_telemetry_id} rejected native compaction; retrying without it and falling back to summary compaction"
            );
            // Remember this model can't compact so future turns fall back to
            // summary compaction (see the function-level note on the one-turn
            // gap this leaves).
            mark_native_compaction_unsupported(model_telemetry_id);
            request.context_management.clear();
            stream_response(
                client,
                provider_name,
                api_url,
                api_key,
                &request,
                extra_headers,
            )
            .await
        }
        result => result,
    }
}

fn default_thinking_reasoning_effort(model: &open_ai::Model) -> Option<open_ai::ReasoningEffort> {
    use open_ai::ReasoningEffort;

    model
        .reasoning_effort()
        .filter(|effort| *effort != ReasoningEffort::None)
        .or_else(|| {
            let supported_efforts = model.supported_reasoning_efforts();
            if supported_efforts.contains(&ReasoningEffort::Medium) {
                Some(ReasoningEffort::Medium)
            } else {
                supported_efforts
                    .iter()
                    .copied()
                    .find(|effort| *effort != ReasoningEffort::None)
            }
        })
}

fn supports_selectable_thinking_effort(model: &open_ai::Model) -> bool {
    model.uses_responses_api()
        && model
            .supported_reasoning_efforts()
            .iter()
            .any(|effort| *effort != open_ai::ReasoningEffort::None)
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
            let (name, value) = match effort {
                open_ai::ReasoningEffort::None => return None,
                open_ai::ReasoningEffort::Minimal => ("Minimal", "minimal"),
                open_ai::ReasoningEffort::Low => ("Low", "low"),
                open_ai::ReasoningEffort::Medium => ("Medium", "medium"),
                open_ai::ReasoningEffort::High => ("High", "high"),
                open_ai::ReasoningEffort::XHigh => ("Extra High", "xhigh"),
            };

            Some(LanguageModelEffortLevel {
                name: name.into(),
                value: value.into(),
                is_default: Some(effort) == default_effort,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_unsupported_compaction_error() {
        use http_client::StatusCode;
        use http_client::http::HeaderMap;

        let http_error = |status: StatusCode, body: &str| RequestError::HttpResponseError {
            provider: "openai".into(),
            status_code: status,
            body: body.to_string(),
            headers: HeaderMap::new(),
        };

        // The exact 400 OpenAI returns when a model can't compact.
        assert!(is_unsupported_compaction_error(&http_error(
            StatusCode::BAD_REQUEST,
            r#"{"error":{"message":"compact_threshold is not enabled.","type":"invalid_request_error","param":"compact_threshold","code":"unsupported_parameter"}}"#,
        )));

        // Same `unsupported_parameter` code for a different `param` must not
        // trigger the fallback. Matching on `code` alone would be too coarse.
        assert!(!is_unsupported_compaction_error(&http_error(
            StatusCode::BAD_REQUEST,
            r#"{"error":{"message":"max_output_tokens is not supported.","type":"invalid_request_error","param":"max_output_tokens","code":"unsupported_parameter"}}"#,
        )));

        // Unrelated 400s, other statuses, and non-HTTP errors must not trigger
        // the fallback.
        assert!(!is_unsupported_compaction_error(&http_error(
            StatusCode::BAD_REQUEST,
            r#"{"error":{"message":"unknown model","code":"invalid_value"}}"#,
        )));
        assert!(!is_unsupported_compaction_error(&http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "compact_threshold unsupported_parameter",
        )));
        assert!(!is_unsupported_compaction_error(&RequestError::Other(
            anyhow::anyhow!("boom")
        )));

        // A non-JSON body still falls back to the substring heuristic so we
        // degrade gracefully if the error shape ever changes.
        assert!(is_unsupported_compaction_error(&http_error(
            StatusCode::BAD_REQUEST,
            "compact_threshold unsupported_parameter",
        )));
    }

    #[test]
    fn native_compaction_threshold_reserves_output_headroom() {
        // Threshold leaves room for a full-length response.
        assert_eq!(native_compaction_threshold(272_000, Some(128_000)), 144_000);
        // Falls back to reserving a quarter of the window when max output is unknown.
        assert_eq!(native_compaction_threshold(400_000, None), 300_000);
        // Never underflows to zero.
        assert_eq!(native_compaction_threshold(1_000, Some(8_000)), 1);
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

    /// A non-streaming Responses request that opts in to native compaction, so
    /// the fallback path has something to strip on retry.
    fn native_compaction_request() -> ResponseRequest {
        let mut request = into_open_ai_response(
            LanguageModelRequest::default(),
            ResponsesRequestConfig {
                model_id: "gpt-5",
                provider_id: "openai",
                supports_parallel_tool_calls: true,
                supports_prompt_cache_key: true,
                max_output_tokens: None,
                default_reasoning_effort: None,
                supports_none_reasoning_effort: true,
            },
        );
        request.stream = false;
        request.context_management = vec![ContextManagement::compaction(1000)];
        request
    }

    #[test]
    fn stream_response_with_compaction_fallback_retries_without_native_compaction() {
        use futures::AsyncReadExt as _;
        use std::sync::atomic::{AtomicUsize, Ordering};

        // The exact 400 OpenAI returns when a model can't compact.
        const UNSUPPORTED_BODY: &str = r#"{"error":{"message":"compact_threshold is not enabled.","type":"invalid_request_error","param":"compact_threshold","code":"unsupported_parameter"}}"#;
        // Minimal non-streaming Responses body that `stream_response` parses when
        // `request.stream == false`.
        const SUCCESS_BODY: &str = r#"{"output":[],"status":"completed"}"#;
        const MODEL_TELEMETRY_ID: &str = "openai/test-native-compaction";

        clear_native_compaction_unsupported_for_test();
        assert!(!native_compaction_unsupported(MODEL_TELEMETRY_ID));

        let call_count = Arc::new(AtomicUsize::new(0));
        let request_bodies = Arc::new(parking_lot::Mutex::new(Vec::<String>::new()));

        let client: Arc<dyn HttpClient> = http_client::FakeHttpClient::create({
            let call_count = call_count.clone();
            let request_bodies = request_bodies.clone();
            move |req| {
                let call_count = call_count.clone();
                let request_bodies = request_bodies.clone();
                let mut body = req.into_body();
                async move {
                    let mut body_string = String::new();
                    body.read_to_string(&mut body_string).await?;
                    request_bodies.lock().push(body_string);

                    // First attempt rejects native compaction; the retry succeeds.
                    if call_count.fetch_add(1, Ordering::SeqCst) == 0 {
                        Ok(http_client::Response::builder()
                            .status(400)
                            .body(http_client::AsyncBody::from(UNSUPPORTED_BODY))?)
                    } else {
                        Ok(http_client::Response::builder()
                            .status(200)
                            .body(http_client::AsyncBody::from(SUCCESS_BODY))?)
                    }
                }
            }
        });

        let events = smol::block_on(async {
            let stream = stream_response_with_compaction_fallback(
                client.as_ref(),
                "openai",
                "https://api.openai.com/v1",
                "test-key",
                native_compaction_request(),
                &CustomHeaders::default(),
                MODEL_TELEMETRY_ID,
            )
            .await
            .expect("native compaction fallback should retry and succeed");
            stream.collect::<Vec<_>>().await
        });

        // The retried, non-streaming 200 body parsed into events without error.
        assert!(events.iter().all(|event| event.is_ok()));

        // Sent exactly two requests: the rejected one and the retry.
        assert_eq!(call_count.load(Ordering::SeqCst), 2);

        let request_bodies = request_bodies.lock();
        assert_eq!(request_bodies.len(), 2);
        // The first request asked for native compaction...
        assert!(request_bodies[0].contains("context_management"));
        assert!(request_bodies[0].contains("compact_threshold"));
        // ...and the retry dropped it.
        assert!(!request_bodies[1].contains("context_management"));

        // The model is now remembered as unable to do native compaction.
        assert!(native_compaction_unsupported(MODEL_TELEMETRY_ID));

        clear_native_compaction_unsupported_for_test();
    }

    #[test]
    fn stream_response_with_compaction_fallback_does_not_retry_on_success() {
        use futures::AsyncReadExt as _;
        use std::sync::atomic::{AtomicUsize, Ordering};

        const SUCCESS_BODY: &str = r#"{"output":[],"status":"completed"}"#;
        // Distinct id from the fallback test, and this test never clears the
        // shared cache, so the two can run concurrently without racing on it.
        const MODEL_TELEMETRY_ID: &str = "openai/test-native-compaction-success";

        assert!(!native_compaction_unsupported(MODEL_TELEMETRY_ID));

        let call_count = Arc::new(AtomicUsize::new(0));
        let client: Arc<dyn HttpClient> = http_client::FakeHttpClient::create({
            let call_count = call_count.clone();
            move |req| {
                let call_count = call_count.clone();
                let mut body = req.into_body();
                async move {
                    // Drain the body so the request is fully "sent".
                    let mut body_string = String::new();
                    body.read_to_string(&mut body_string).await?;
                    call_count.fetch_add(1, Ordering::SeqCst);
                    Ok(http_client::Response::builder()
                        .status(200)
                        .body(http_client::AsyncBody::from(SUCCESS_BODY))?)
                }
            }
        });

        let result = smol::block_on(stream_response_with_compaction_fallback(
            client.as_ref(),
            "openai",
            "https://api.openai.com/v1",
            "test-key",
            native_compaction_request(),
            &CustomHeaders::default(),
            MODEL_TELEMETRY_ID,
        ));

        assert!(result.is_ok());
        // A successful first attempt must not retry...
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
        // ...and must not mark the model as unsupported.
        assert!(!native_compaction_unsupported(MODEL_TELEMETRY_ID));
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
        let model_telemetry_id = self.telemetry_id();

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
            let response = stream_response_with_compaction_fallback(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
                &model_telemetry_id,
            )
            .await?;
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

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        supported_thinking_effort_levels(&self.model)
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn uses_native_compaction(&self) -> bool {
        self.model.supports_native_compaction()
            && !native_compaction_unsupported(&self.telemetry_id())
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
            let mut request = into_open_ai_response(
                request,
                ResponsesRequestConfig {
                    model_id: self.model.id(),
                    provider_id: PROVIDER_ID.0.as_ref(),
                    supports_parallel_tool_calls: self.model.supports_parallel_tool_calls(),
                    supports_prompt_cache_key: self.model.supports_prompt_cache_key(),
                    max_output_tokens: self.max_output_tokens(),
                    default_reasoning_effort: default_thinking_reasoning_effort(&self.model),
                    supports_none_reasoning_effort: self
                        .model
                        .supported_reasoning_efforts()
                        .contains(&open_ai::ReasoningEffort::None),
                },
            );
            let native_compaction_enabled = self.uses_native_compaction()
                && cx.update(|cx| cx.has_flag::<HandoffFeatureFlag>());
            if native_compaction_enabled {
                let compact_threshold = native_compaction_threshold(
                    self.model.max_token_count(),
                    self.model.max_output_tokens(),
                );
                log::debug!(
                    "Requesting OpenAI native compaction for {} (compact_threshold={compact_threshold})",
                    self.telemetry_id()
                );
                request.context_management = vec![ContextManagement::compaction(compact_threshold)];
            }
            let completions = self.stream_response(request, cx);
            async move {
                let mapper = OpenAiResponseEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        } else {
            let request = into_open_ai(
                request,
                self.model.id(),
                self.model.supports_parallel_tool_calls(),
                self.model.supports_prompt_cache_key(),
                self.max_output_tokens(),
                None,
                false,
            );
            let completions = self.stream_completion(request, cx);
            async move {
                let mapper = OpenAiEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        }
    }
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            InputField::new(
                window,
                cx,
                "sk-000000000000000000000000000000000000000000000000",
            )
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = Some(state.update(cx, |state, cx| state.authenticate(cx))) {
                    // We don't log an error, because "not signed in" is also an error.
                    let _ = task.await;
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        // url changes can cause the editor to be displayed again
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable")
        } else {
            let api_url = OpenAiLanguageModelProvider::api_url(cx);
            if api_url == OPEN_AI_API_URL {
                "API key configured".to_string()
            } else {
                format!("API key configured for {}", api_url)
            }
        };

        let api_key_section = if self.should_render_editor(cx) {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use Zed's agent with OpenAI, you need to add an API key. Follow these steps:"))
                .child(
                    List::new()
                        .child(
                            ListBulletItem::new("")
                                .child(Label::new("Create one by visiting"))
                                .child(ButtonLink::new("OpenAI's console", "https://platform.openai.com/api-keys"))
                        )
                        .child(
                            ListBulletItem::new("Ensure your OpenAI account has credits")
                        )
                        .child(
                            ListBulletItem::new("Paste your API key below and hit enter to start using the agent")
                        ),
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(format!(
                        "You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .child(
                    Label::new(
                        "Note that having a subscription for another service like GitHub Copilot won't work.",
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .when(env_var_set, |this| {
                    this.tooltip_label(format!("To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."))
                })
                .into_any_element()
        };

        let compatible_api_section = h_flex()
            .mt_1p5()
            .gap_0p5()
            .flex_wrap()
            .when(self.should_render_editor(cx), |this| {
                this.pt_1p5()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
            })
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Icon::new(IconName::Info)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(Label::new("Zed also supports OpenAI-compatible models.")),
            )
            .child(
                Button::new("docs", "Learn More")
                    .end_icon(
                        Icon::new(IconName::ArrowUpRight)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .on_click(move |_, _window, cx| {
                        cx.open_url("https://zed.dev/docs/ai/llm-providers#openai-api-compatible")
                    }),
            );

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials…")).into_any()
        } else {
            v_flex()
                .size_full()
                .child(api_key_section)
                .child(compatible_api_section)
                .into_any()
        }
    }
}
