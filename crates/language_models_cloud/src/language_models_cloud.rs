use anthropic::AnthropicModelMode;
use anyhow::{Context as _, Result};
use cloud_llm_client::{
    CLIENT_SUPPORTS_STATUS_MESSAGES_HEADER_NAME, CLIENT_SUPPORTS_STATUS_STREAM_ENDED_HEADER_NAME,
    CLIENT_SUPPORTS_X_AI_HEADER_NAME, CompletionBody, CompletionEvent, CompletionRequestStatus,
    EXPIRED_LLM_TOKEN_HEADER_NAME, ListModelsResponse, OUTDATED_LLM_TOKEN_HEADER_NAME,
    SERVER_SUPPORTS_STATUS_MESSAGES_HEADER_NAME, ZED_VERSION_HEADER_NAME,
};
use futures::{
    AsyncBufReadExt, AsyncReadExt as _, FutureExt, Stream, StreamExt,
    future::BoxFuture,
    io::BufReader,
    stream::{self, BoxStream},
};
use google_ai::GoogleModelMode;
use gpui::{AppContext, AsyncApp, BackgroundExecutor, Context, Task};
use http_client::http::{HeaderMap, HeaderValue};
use http_client::{
    AsyncBody, HttpClient, HttpClientWithUrl, HttpRequestExt, Method, Response, StatusCode,
};
use language_model::{
    ANTHROPIC_PROVIDER_ID, ANTHROPIC_PROVIDER_NAME, CompactionResult, DisabledReason,
    GOOGLE_PROVIDER_ID, GOOGLE_PROVIDER_NAME, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolSchemaFormat, OPEN_AI_PROVIDER_ID,
    OPEN_AI_PROVIDER_NAME, RateLimiter, X_AI_PROVIDER_ID, X_AI_PROVIDER_NAME,
    ZED_CLOUD_PROVIDER_ID, ZED_CLOUD_PROVIDER_NAME,
};

use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::VecDeque;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;
use thiserror::Error;

use anthropic::completion::{AnthropicEventMapper, AnthropicPromptCacheMode, into_anthropic};
use google_ai::completion::{GoogleEventMapper, into_google};
use open_ai::completion::{
    ChatCompletionMaxTokensParameter, OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai,
    into_open_ai_response, token_usage_from_response_usage,
};
use open_ai::responses_websocket::{self, SharedWebSocketChains, WebSocketChains};
use websocket_client::{AuthRequired, WebSocketClient, websocket_url_from_http};

const PROVIDER_ID: LanguageModelProviderId = ZED_CLOUD_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = ZED_CLOUD_PROVIDER_NAME;

/// Setting this environment variable (to any value) disables the WebSocket
/// transport for cloud completions, forcing the HTTP `/completions` endpoint.
const DISABLE_WEBSOCKET_ENV_VAR_NAME: &str = "ZED_DISABLE_CLOUD_WEBSOCKET";

fn websocket_streaming_disabled() -> bool {
    std::env::var_os(DISABLE_WEBSOCKET_ENV_VAR_NAME).is_some()
}

/// Trait for acquiring and refreshing LLM authentication tokens.
pub trait CloudLlmTokenProvider: Send + Sync {
    type AuthContext: Clone + Send + 'static;

    fn auth_context(&self, cx: &impl AppContext) -> Self::AuthContext;
    fn cached_token(&self, auth_context: Self::AuthContext) -> BoxFuture<'static, Result<String>>;
    fn refresh_token(&self, auth_context: Self::AuthContext) -> BoxFuture<'static, Result<String>>;

    /// Whether the user has consented to upstream providers retaining
    /// inference logs for models that require it (see
    /// [`LanguageModel::requires_data_retention`]).
    fn has_data_retention_consent(&self, cx: &impl AppContext) -> bool;
}

/// Sends an authenticated request to the Zed LLM service, retrying once with
/// a refreshed token if the server signals that the cached LLM token is
/// expired or otherwise rejected. Returns the raw response so callers can
/// inspect headers and stream the body.
pub async fn authenticated_llm_request<TP: CloudLlmTokenProvider>(
    http_client: &HttpClientWithUrl,
    token_provider: &TP,
    auth_context: TP::AuthContext,
    build_request: impl Fn(&str) -> Result<http_client::Request<AsyncBody>>,
) -> Result<Response<AsyncBody>> {
    let token = token_provider.cached_token(auth_context.clone()).await?;
    let response = http_client.send(build_request(&token)?).await?;
    if !needs_llm_token_refresh(&response) && response.status() != StatusCode::UNAUTHORIZED {
        return Ok(response);
    }
    log::info!("LLM token rejected; refreshing and retrying request");
    let token = token_provider.refresh_token(auth_context).await?;
    http_client.send(build_request(&token)?).await
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

pub struct CloudLanguageModel<TP: CloudLlmTokenProvider> {
    pub id: LanguageModelId,
    pub model: Arc<cloud_llm_client::LanguageModel>,
    pub token_provider: Arc<TP>,
    pub http_client: Arc<HttpClientWithUrl>,
    pub app_version: Option<Version>,
    pub request_limiter: RateLimiter,
    /// When present, OpenAI completions are streamed over a WebSocket
    /// session to the `/completions/session` endpoint, with the HTTP
    /// `/completions` endpoint as a fallback.
    pub websocket_client: Option<Arc<dyn WebSocketClient>>,
    pub websocket_chains: SharedWebSocketChains,
}

pub struct PerformLlmCompletionResponse {
    pub response: Response<AsyncBody>,
    pub includes_status_messages: bool,
}

impl<TP: CloudLlmTokenProvider> CloudLanguageModel<TP> {
    pub async fn perform_llm_completion(
        http_client: &HttpClientWithUrl,
        token_provider: &TP,
        auth_context: TP::AuthContext,
        app_version: Option<Version>,
        body: CompletionBody,
    ) -> Result<PerformLlmCompletionResponse, LanguageModelCompletionError> {
        Self::perform_llm_request(
            "/completions",
            true,
            http_client,
            token_provider,
            auth_context,
            app_version,
            body,
        )
        .await
    }

    async fn perform_llm_compaction(
        http_client: &HttpClientWithUrl,
        token_provider: &TP,
        auth_context: TP::AuthContext,
        app_version: Option<Version>,
        body: CompletionBody,
    ) -> Result<PerformLlmCompletionResponse, LanguageModelCompletionError> {
        Self::perform_llm_request(
            "/completions/compact",
            false,
            http_client,
            token_provider,
            auth_context,
            app_version,
            body,
        )
        .await
    }

    async fn perform_llm_request(
        path: &str,
        request_status_messages: bool,
        http_client: &HttpClientWithUrl,
        token_provider: &TP,
        auth_context: TP::AuthContext,
        app_version: Option<Version>,
        body: CompletionBody,
    ) -> Result<PerformLlmCompletionResponse, LanguageModelCompletionError> {
        let url = http_client
            .build_zed_llm_url(path, &[])
            .map_err(LanguageModelCompletionError::Other)?;
        let body = serde_json::to_string(&body).map_err(|error| {
            LanguageModelCompletionError::SerializeRequest {
                provider: PROVIDER_NAME,
                error,
            }
        })?;
        let mut response =
            authenticated_llm_request(http_client, token_provider, auth_context, |token| {
                let mut request = http_client::Request::builder()
                    .method(Method::POST)
                    .uri(url.as_ref())
                    .when_some(app_version.as_ref(), |builder, app_version| {
                        builder.header(ZED_VERSION_HEADER_NAME, app_version.to_string())
                    })
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {token}"));
                if request_status_messages {
                    request = request
                        .header(CLIENT_SUPPORTS_STATUS_MESSAGES_HEADER_NAME, "true")
                        .header(CLIENT_SUPPORTS_STATUS_STREAM_ENDED_HEADER_NAME, "true");
                }
                Ok(request.body(body.clone().into())?)
            })
            .await
            .map_err(|error| LanguageModelCompletionError::HttpSend {
                provider: PROVIDER_NAME,
                error,
            })?;

        let status = response.status();
        if status.is_success() {
            let includes_status_messages = request_status_messages
                && response
                    .headers()
                    .get(SERVER_SUPPORTS_STATUS_MESSAGES_HEADER_NAME)
                    .is_some();

            return Ok(PerformLlmCompletionResponse {
                response,
                includes_status_messages,
            });
        }

        if status == StatusCode::PAYMENT_REQUIRED {
            return Err(LanguageModelCompletionError::PaymentRequired);
        }

        let mut body = String::new();
        let headers = response.headers().clone();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|error| LanguageModelCompletionError::ApiReadResponseError {
                provider: PROVIDER_NAME,
                error,
            })?;
        Err(ApiError {
            status,
            body,
            headers,
        }
        .into())
    }

    /// Runs one OpenAI completion turn over a WebSocket session with the
    /// `/completions/session` endpoint, which forwards native Responses API
    /// events. The session machinery sends only new input items plus
    /// `previous_response_id` when a cached connection covers a prefix of
    /// the request.
    async fn stream_open_ai_websocket_completion(
        websocket_client: Arc<dyn WebSocketClient>,
        http_client: &HttpClientWithUrl,
        token_provider: &TP,
        auth_context: TP::AuthContext,
        app_version: Option<Version>,
        websocket_chains: SharedWebSocketChains,
        executor: BackgroundExecutor,
        thread_id: Option<String>,
        prompt_id: Option<String>,
        request: &open_ai::responses::Request,
    ) -> Result<BoxStream<'static, Result<open_ai::responses::StreamEvent>>> {
        let mut url =
            websocket_url_from_http(http_client.build_zed_llm_url("/completions/session", &[])?)?;
        // `build_zed_llm_url` leaves an empty query (a trailing `?`); drop
        // it so the upgrade request has a clean path.
        if url.query() == Some("") {
            url.set_query(None);
        }
        let token = token_provider.cached_token(auth_context.clone()).await?;
        let connection_scope = format!("{}\0{token}", url.as_str());
        let connect = async move {
            let headers = websocket_headers(&token, app_version.as_ref())?;
            match websocket_client.connect(url.as_str(), headers).await {
                Err(error) if error.downcast_ref::<AuthRequired>().is_some() => {
                    log::info!("LLM token rejected; refreshing and retrying WebSocket connection");
                    let token = token_provider.refresh_token(auth_context.clone()).await?;
                    let headers = websocket_headers(&token, app_version.as_ref())?;
                    websocket_client.connect(url.as_str(), headers).await
                }
                result => result,
            }
        };
        let model = request.model.clone();
        let envelope_turn = move |provider_request: serde_json::Map<String, serde_json::Value>| {
            Ok(serde_json::to_string(&CompletionBody {
                thread_id: thread_id.clone(),
                prompt_id: prompt_id.clone(),
                provider: cloud_llm_client::LanguageModelProvider::OpenAi,
                model: model.clone(),
                provider_request: serde_json::Value::Object(provider_request),
            })?)
        };
        responses_websocket::stream_websocket_response(
            request,
            connection_scope.as_bytes(),
            websocket_chains,
            connect,
            envelope_turn,
            move |future| executor.spawn(future).detach(),
        )
        .await
    }
}

fn websocket_headers(token: &str, app_version: Option<&Version>) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(
        http_client::http::header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}"))?,
    );
    if let Some(app_version) = app_version {
        headers.insert(
            ZED_VERSION_HEADER_NAME,
            HeaderValue::from_str(&app_version.to_string())?,
        );
    }
    Ok(headers)
}

fn needs_llm_token_refresh(response: &Response<AsyncBody>) -> bool {
    response
        .headers()
        .get(EXPIRED_LLM_TOKEN_HEADER_NAME)
        .is_some()
        || response
            .headers()
            .get(OUTDATED_LLM_TOKEN_HEADER_NAME)
            .is_some()
}

#[derive(Debug, Error)]
#[error("cloud language model request failed with status {status}: {body}")]
struct ApiError {
    status: StatusCode,
    body: String,
    headers: HeaderMap<HeaderValue>,
}

/// Represents error responses from Zed's cloud API.
///
/// Example JSON for an upstream HTTP error:
/// ```json
/// {
///   "code": "upstream_http_error",
///   "message": "Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers, reset reason: connection timeout",
///   "upstream_status": 503
/// }
/// ```
#[derive(Debug, serde::Deserialize)]
struct CloudApiError {
    code: String,
    message: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_status_code")]
    upstream_status: Option<StatusCode>,
    #[serde(default)]
    retry_after: Option<f64>,
}

fn deserialize_optional_status_code<'de, D>(deserializer: D) -> Result<Option<StatusCode>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<u16> = Option::deserialize(deserializer)?;
    Ok(opt.and_then(|code| StatusCode::from_u16(code).ok()))
}

impl From<ApiError> for LanguageModelCompletionError {
    fn from(error: ApiError) -> Self {
        if let Ok(cloud_error) = serde_json::from_str::<CloudApiError>(&error.body) {
            if cloud_error.code.starts_with("upstream_http_") {
                let status = if let Some(status) = cloud_error.upstream_status {
                    status
                } else if cloud_error.code.ends_with("_error") {
                    error.status
                } else {
                    // If there's a status code in the code string (e.g. "upstream_http_429")
                    // then use that; otherwise, see if the JSON contains a status code.
                    cloud_error
                        .code
                        .strip_prefix("upstream_http_")
                        .and_then(|code_str| code_str.parse::<u16>().ok())
                        .and_then(|code| StatusCode::from_u16(code).ok())
                        .unwrap_or(error.status)
                };

                return LanguageModelCompletionError::UpstreamProviderError {
                    message: cloud_error.message,
                    status,
                    retry_after: cloud_error.retry_after.map(Duration::from_secs_f64),
                };
            }

            return LanguageModelCompletionError::from_http_status(
                PROVIDER_NAME,
                error.status,
                cloud_error.message,
                None,
            );
        }

        let retry_after = None;
        LanguageModelCompletionError::from_http_status(
            PROVIDER_NAME,
            error.status,
            error.body,
            retry_after,
        )
    }
}

impl<TP: CloudLlmTokenProvider + 'static> LanguageModel for CloudLanguageModel<TP> {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name.clone())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn upstream_provider_id(&self) -> LanguageModelProviderId {
        use cloud_llm_client::LanguageModelProvider::*;
        match self.model.provider {
            Anthropic => ANTHROPIC_PROVIDER_ID,
            OpenAi => OPEN_AI_PROVIDER_ID,
            Google => GOOGLE_PROVIDER_ID,
            XAi => X_AI_PROVIDER_ID,
        }
    }

    fn upstream_provider_name(&self) -> LanguageModelProviderName {
        use cloud_llm_client::LanguageModelProvider::*;
        match self.model.provider {
            Anthropic => ANTHROPIC_PROVIDER_NAME,
            OpenAi => OPEN_AI_PROVIDER_NAME,
            Google => GOOGLE_PROVIDER_NAME,
            XAi => X_AI_PROVIDER_NAME,
        }
    }

    fn is_latest(&self) -> bool {
        self.model.is_latest
    }

    fn is_disabled(&self) -> Option<DisabledReason> {
        if self.model.is_disabled {
            self.model.disabled_reason.clone().map(DisabledReason::new)
        } else {
            None
        }
    }

    fn requires_data_retention(&self) -> bool {
        // Anthropic cannot offer Fable models with Zero Data Retention
        self.id
            .0
            .as_ref()
            .starts_with(anthropic::FABLE_MODEL_ID_PREFIX)
    }

    fn refusal_fallback_model_id(&self) -> Option<&'static str> {
        if self
            .id
            .0
            .as_ref()
            .starts_with(anthropic::FABLE_MODEL_ID_PREFIX)
        {
            Some(anthropic::FABLE_FALLBACK_MODEL_ID)
        } else {
            None
        }
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tools
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images
    }

    fn supports_thinking(&self) -> bool {
        self.model.supports_thinking
    }

    fn supports_disabling_thinking(&self) -> bool {
        self.model.supports_disabling_thinking
    }

    fn supports_fast_mode(&self) -> bool {
        self.model.supports_fast_mode
    }

    fn supports_server_side_compaction(&self) -> bool {
        self.model.supports_server_side_compaction
    }

    fn supports_explicit_compaction(&self) -> bool {
        self.model.provider == cloud_llm_client::LanguageModelProvider::OpenAi
            && self.model.supports_server_side_compaction
    }

    fn compact(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<CompactionResult, LanguageModelCompletionError>> {
        if !self.supports_explicit_compaction() {
            return async {
                Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                    "this cloud model does not support explicit compaction"
                )))
            }
            .boxed();
        }

        let thread_id = request.thread_id.clone();
        let prompt_id = request.prompt_id.clone();
        let app_version = self.app_version.clone();
        let model_provider = self.model.provider;
        let provider_name = provider_name(&self.model.provider);
        let supports_none_reasoning_effort =
            self.model.supported_effort_levels.iter().any(|effort| {
                open_ai::ReasoningEffort::from_str(&effort.value)
                    .is_ok_and(|effort| effort == open_ai::ReasoningEffort::None)
            });
        // Cloud proxies to OpenAI's own infrastructure, so the resulting
        // compaction state is owned by (and interchangeable with) OpenAI
        // proper, not by the cloud transport.
        let request = match into_open_ai_response(
            request,
            &self.model.id.0,
            self.model.supports_parallel_tool_calls,
            true,
            None,
            None,
            supports_none_reasoning_effort,
            &OPEN_AI_PROVIDER_ID,
        ) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let compact_request = request.into_compact_request();
        let http_client = self.http_client.clone();
        let token_provider = self.token_provider.clone();
        let auth_context = token_provider.auth_context(cx);
        let future = self.request_limiter.run(async move {
            let PerformLlmCompletionResponse {
                response,
                includes_status_messages,
            } = Self::perform_llm_compaction(
                &http_client,
                &*token_provider,
                auth_context,
                app_version,
                CompletionBody {
                    thread_id,
                    prompt_id,
                    provider: model_provider,
                    model: compact_request.model.clone(),
                    provider_request: serde_json::to_value(compact_request).map_err(|error| {
                        LanguageModelCompletionError::SerializeRequest {
                            provider: provider_name.clone(),
                            error,
                        }
                    })?,
                },
            )
            .await?;

            let events = response_lines::<open_ai::responses::CompactedResponse>(
                response,
                includes_status_messages,
            );
            futures::pin_mut!(events);
            while let Some(event) = events.next().await {
                match event.map_err(|error| error.into_completion_error(provider_name.clone()))? {
                    CompletionEvent::Event(response) => {
                        let usage = token_usage_from_response_usage(&response.usage);
                        let context = response
                            .into_compacted_context(OPEN_AI_PROVIDER_ID)
                            .map_err(LanguageModelCompletionError::Other)?;
                        return Ok(CompactionResult { context, usage });
                    }
                    CompletionEvent::Status(_) => {}
                }
            }

            Err(LanguageModelCompletionError::StreamEndedUnexpectedly {
                provider: provider_name,
            })
        });
        future.boxed()
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        self.model
            .supported_effort_levels
            .iter()
            .map(|effort_level| LanguageModelEffortLevel {
                name: effort_level.name.clone().into(),
                value: effort_level.value.clone().into(),
                is_default: effort_level.is_default.unwrap_or(false),
            })
            .collect()
    }

    fn supports_streaming_tools(&self) -> bool {
        self.model.supports_streaming_tools
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto
            | LanguageModelToolChoice::Any
            | LanguageModelToolChoice::None => true,
        }
    }

    fn supports_split_token_display(&self) -> bool {
        use cloud_llm_client::LanguageModelProvider::*;
        matches!(self.model.provider, OpenAi | XAi)
    }

    fn telemetry_id(&self) -> String {
        format!("zed.dev/{}", self.model.id)
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        match self.model.provider {
            cloud_llm_client::LanguageModelProvider::Anthropic
            | cloud_llm_client::LanguageModelProvider::OpenAi => {
                LanguageModelToolSchemaFormat::JsonSchema
            }
            cloud_llm_client::LanguageModelProvider::Google
            | cloud_llm_client::LanguageModelProvider::XAi => {
                LanguageModelToolSchemaFormat::JsonSchemaSubset
            }
        }
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count as u64
    }

    fn max_output_tokens(&self) -> Option<u64> {
        Some(self.model.max_output_tokens as u64)
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
        if self.requires_data_retention() && !self.token_provider.has_data_retention_consent(cx) {
            let model_name = self.model.display_name.clone();
            return async move {
                Err(LanguageModelCompletionError::DataRetentionConsentRequired { model_name })
            }
            .boxed();
        }

        let thread_id = request.thread_id.clone();
        let prompt_id = request.prompt_id.clone();
        let app_version = self.app_version.clone();
        let thinking_allowed = request.thinking_allowed;
        let enable_thinking = thinking_allowed && self.model.supports_thinking;
        let provider_name = provider_name(&self.model.provider);
        match self.model.provider {
            cloud_llm_client::LanguageModelProvider::Anthropic => {
                let effort = request
                    .thinking_effort
                    .as_ref()
                    .and_then(|effort| anthropic::Effort::from_str(effort).ok());

                let mut request = match into_anthropic(
                    request,
                    self.model.id.to_string(),
                    1.0,
                    self.model.max_output_tokens as u64,
                    if enable_thinking {
                        AnthropicModelMode::Thinking {
                            budget_tokens: Some(4_096),
                        }
                    } else {
                        AnthropicModelMode::Default
                    },
                    AnthropicPromptCacheMode::Automatic,
                    // Cloud proxies to Anthropic's own infrastructure, so
                    // compaction state is owned by (and interchangeable with)
                    // Anthropic proper, not by the cloud transport.
                    &ANTHROPIC_PROVIDER_ID,
                ) {
                    Ok(request) => request,
                    Err(error) => return async move { Err(error.into()) }.boxed(),
                };

                if enable_thinking && effort.is_some() {
                    request.thinking = Some(anthropic::Thinking::Adaptive {
                        display: Some(anthropic::AdaptiveThinkingDisplay::Summarized),
                    });
                    request.output_config = Some(anthropic::OutputConfig { effort });
                }

                if !self.model.supports_fast_mode {
                    request.speed = None;
                }

                let http_client = self.http_client.clone();
                let token_provider = self.token_provider.clone();
                let auth_context = token_provider.auth_context(cx);
                let future = self.request_limiter.stream(async move {
                    let PerformLlmCompletionResponse {
                        response,
                        includes_status_messages,
                    } = Self::perform_llm_completion(
                        &http_client,
                        &*token_provider,
                        auth_context,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            provider: cloud_llm_client::LanguageModelProvider::Anthropic,
                            model: request.model.clone(),
                            provider_request: serde_json::to_value(&request).map_err(|error| {
                                LanguageModelCompletionError::SerializeRequest {
                                    provider: provider_name.clone(),
                                    error,
                                }
                            })?,
                        },
                    )
                    .await?;

                    let mut mapper =
                        AnthropicEventMapper::new(provider_name.clone(), ANTHROPIC_PROVIDER_ID);
                    Ok(map_cloud_completion_events(
                        Box::pin(response_lines(response, includes_status_messages)),
                        &provider_name,
                        move |event| mapper.map_event(event),
                    ))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
            cloud_llm_client::LanguageModelProvider::OpenAi => {
                let http_client = self.http_client.clone();
                let token_provider = self.token_provider.clone();
                let effort = request
                    .thinking_effort
                    .as_ref()
                    .and_then(|effort| open_ai::ReasoningEffort::from_str(effort).ok())
                    .filter(|effort| *effort != open_ai::ReasoningEffort::None);
                let supports_none_reasoning_effort =
                    self.model.supported_effort_levels.iter().any(|effort| {
                        open_ai::ReasoningEffort::from_str(&effort.value)
                            .is_ok_and(|effort| effort == open_ai::ReasoningEffort::None)
                    });

                let mut request = match into_open_ai_response(
                    request,
                    &self.model.id.0,
                    self.model.supports_parallel_tool_calls,
                    true,
                    None,
                    None,
                    supports_none_reasoning_effort,
                    &OPEN_AI_PROVIDER_ID,
                ) {
                    Ok(request) => request,
                    Err(error) => return async move { Err(error.into()) }.boxed(),
                };

                if enable_thinking && let Some(effort) = effort {
                    request.reasoning = Some(open_ai::responses::ReasoningConfig {
                        effort,
                        summary: Some(open_ai::responses::ReasoningSummaryMode::Auto),
                    });
                }

                let auth_context = token_provider.auth_context(cx);
                let websocket_client = if websocket_streaming_disabled() {
                    log::debug!(
                        "Cloud OpenAI transport: HTTP because {} is set",
                        DISABLE_WEBSOCKET_ENV_VAR_NAME
                    );
                    None
                } else {
                    self.websocket_client.clone()
                };
                let websocket_chains = self.websocket_chains.clone();
                let executor = cx.background_executor().clone();
                let future = self.request_limiter.stream(async move {
                    // A WebSocket attempt that fails before streaming begins
                    // is retried over the HTTP endpoint, which also covers
                    // servers that don't expose `/completions/session` yet.
                    // Rejected credentials are surfaced directly since the
                    // fallback would fail the same way.
                    if let Some(websocket_client) = websocket_client {
                        match Self::stream_open_ai_websocket_completion(
                            websocket_client,
                            &http_client,
                            &*token_provider,
                            auth_context.clone(),
                            app_version.clone(),
                            websocket_chains,
                            executor,
                            thread_id.clone(),
                            prompt_id.clone(),
                            &request,
                        )
                        .await
                        {
                            Ok(events) => {
                                let mapper =
                                    OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
                                return Ok(mapper.map_stream(events).boxed());
                            }
                            Err(error) => {
                                if error.downcast_ref::<AuthRequired>().is_some() {
                                    return Err(
                                        LanguageModelCompletionError::AuthenticationError {
                                            provider: provider_name.clone(),
                                            message: error.to_string(),
                                        },
                                    );
                                }
                                log::info!(
                                    "Cloud OpenAI transport: falling back to HTTP; WebSocket request failed: {error:#}"
                                );
                            }
                        }
                    }

                    let PerformLlmCompletionResponse {
                        response,
                        includes_status_messages,
                    } = Self::perform_llm_completion(
                        &http_client,
                        &*token_provider,
                        auth_context,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            provider: cloud_llm_client::LanguageModelProvider::OpenAi,
                            model: request.model.clone(),
                            provider_request: serde_json::to_value(&request).map_err(|error| {
                                LanguageModelCompletionError::SerializeRequest {
                                    provider: provider_name.clone(),
                                    error,
                                }
                            })?,
                        },
                    )
                    .await?;

                    let mut mapper = OpenAiResponseEventMapper::new(OPEN_AI_PROVIDER_ID);
                    Ok(map_cloud_completion_events(
                        Box::pin(response_lines(response, includes_status_messages)),
                        &provider_name,
                        move |event| mapper.map_event(event),
                    ))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
            cloud_llm_client::LanguageModelProvider::XAi => {
                let http_client = self.http_client.clone();
                let token_provider = self.token_provider.clone();
                let request = match into_open_ai(
                    request,
                    &self.model.id.0,
                    self.model.supports_parallel_tool_calls,
                    false,
                    None,
                    ChatCompletionMaxTokensParameter::MaxCompletionTokens,
                    None,
                    false,
                ) {
                    Ok(request) => request,
                    Err(error) => return async move { Err(error.into()) }.boxed(),
                };
                let auth_context = token_provider.auth_context(cx);
                let future = self.request_limiter.stream(async move {
                    let PerformLlmCompletionResponse {
                        response,
                        includes_status_messages,
                    } = Self::perform_llm_completion(
                        &http_client,
                        &*token_provider,
                        auth_context,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            provider: cloud_llm_client::LanguageModelProvider::XAi,
                            model: request.model.clone(),
                            provider_request: serde_json::to_value(&request).map_err(|error| {
                                LanguageModelCompletionError::SerializeRequest {
                                    provider: provider_name.clone(),
                                    error,
                                }
                            })?,
                        },
                    )
                    .await?;

                    let mut mapper = OpenAiEventMapper::new();
                    Ok(map_cloud_completion_events(
                        Box::pin(response_lines(response, includes_status_messages)),
                        &provider_name,
                        move |event| mapper.map_event(event),
                    ))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
            cloud_llm_client::LanguageModelProvider::Google => {
                let http_client = self.http_client.clone();
                let token_provider = self.token_provider.clone();
                let request =
                    match into_google(request, self.model.id.to_string(), GoogleModelMode::Default)
                    {
                        Ok(request) => request,
                        Err(error) => return async move { Err(error.into()) }.boxed(),
                    };
                let auth_context = token_provider.auth_context(cx);
                let future = self.request_limiter.stream(async move {
                    let PerformLlmCompletionResponse {
                        response,
                        includes_status_messages,
                    } = Self::perform_llm_completion(
                        &http_client,
                        &*token_provider,
                        auth_context,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            provider: cloud_llm_client::LanguageModelProvider::Google,
                            model: request.model.model_id.clone(),
                            provider_request: serde_json::to_value(&request).map_err(|error| {
                                LanguageModelCompletionError::SerializeRequest {
                                    provider: provider_name.clone(),
                                    error,
                                }
                            })?,
                        },
                    )
                    .await?;

                    let mut mapper = GoogleEventMapper::new();
                    Ok(map_cloud_completion_events(
                        Box::pin(response_lines(response, includes_status_messages)),
                        &provider_name,
                        move |event| mapper.map_event(event),
                    ))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
        }
    }
}

pub struct CloudModelProvider<TP: CloudLlmTokenProvider> {
    token_provider: Arc<TP>,
    http_client: Arc<HttpClientWithUrl>,
    app_version: Option<Version>,
    websocket_client: Option<Arc<dyn WebSocketClient>>,
    websocket_chains: SharedWebSocketChains,
    models: Vec<Arc<cloud_llm_client::LanguageModel>>,
    default_model: Option<Arc<cloud_llm_client::LanguageModel>>,
    default_fast_model: Option<Arc<cloud_llm_client::LanguageModel>>,
    recommended_models: Vec<Arc<cloud_llm_client::LanguageModel>>,
}

impl<TP: CloudLlmTokenProvider + 'static> CloudModelProvider<TP> {
    pub fn new(
        token_provider: Arc<TP>,
        http_client: Arc<HttpClientWithUrl>,
        app_version: Option<Version>,
    ) -> Self {
        Self {
            token_provider,
            http_client,
            app_version,
            websocket_client: None,
            websocket_chains: WebSocketChains::new_shared(),
            models: Vec::new(),
            default_model: None,
            default_fast_model: None,
            recommended_models: Vec::new(),
        }
    }

    /// Enables the WebSocket transport for providers that support it (see
    /// [`CloudLanguageModel::websocket_client`]). Without this, all
    /// completions use the HTTP endpoints.
    pub fn with_websocket_client(mut self, websocket_client: Arc<dyn WebSocketClient>) -> Self {
        self.websocket_client = Some(websocket_client);
        self
    }

    /// Enables or disables the WebSocket transport after construction, e.g.
    /// when the user's eligibility is only known once the server responds.
    /// Affects models created afterwards; disabling also drops any cached
    /// connections.
    pub fn set_websocket_client(&mut self, websocket_client: Option<Arc<dyn WebSocketClient>>) {
        if websocket_client.is_none() {
            self.websocket_chains.lock().clear();
        }
        self.websocket_client = websocket_client;
    }

    pub fn refresh_models(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let http_client = self.http_client.clone();
        let token_provider = self.token_provider.clone();
        cx.spawn(async move |this, cx| {
            let auth_context = token_provider.auth_context(cx);
            let response =
                Self::fetch_models_request(&http_client, &*token_provider, auth_context).await?;
            this.update(cx, |this, cx| {
                this.update_models(response);
                cx.notify();
            })
        })
    }

    async fn fetch_models_request(
        http_client: &HttpClientWithUrl,
        token_provider: &TP,
        auth_context: TP::AuthContext,
    ) -> Result<ListModelsResponse> {
        let url = http_client.build_zed_llm_url("/models", &[])?;
        let mut response =
            authenticated_llm_request(http_client, token_provider, auth_context, |token| {
                Ok(http_client::Request::builder()
                    .method(Method::GET)
                    .header(CLIENT_SUPPORTS_X_AI_HEADER_NAME, "true")
                    .uri(url.as_ref())
                    .header("Authorization", format!("Bearer {token}"))
                    .body(AsyncBody::empty())?)
            })
            .await
            .context("failed to send list models request")?;

        if response.status().is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            Ok(serde_json::from_str(&body)?)
        } else {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            anyhow::bail!(
                "error listing models.\nStatus: {:?}\nBody: {body}",
                response.status(),
            );
        }
    }

    pub fn update_models(&mut self, response: ListModelsResponse) {
        let models: Vec<_> = response.models.into_iter().map(Arc::new).collect();

        self.default_model = models
            .iter()
            .find(|model| {
                response
                    .default_model
                    .as_ref()
                    .is_some_and(|default_model_id| &model.id == default_model_id)
            })
            .cloned();
        self.default_fast_model = models
            .iter()
            .find(|model| {
                response
                    .default_fast_model
                    .as_ref()
                    .is_some_and(|default_fast_model_id| &model.id == default_fast_model_id)
            })
            .cloned();
        self.recommended_models = response
            .recommended_models
            .iter()
            .filter_map(|id| models.iter().find(|model| &model.id == id))
            .cloned()
            .collect();
        self.models = models;
    }

    pub fn clear_models(&mut self) {
        self.models.clear();
        self.default_model = None;
        self.default_fast_model = None;
        self.recommended_models.clear();
    }

    pub fn create_model(
        &self,
        model: &Arc<cloud_llm_client::LanguageModel>,
    ) -> Arc<dyn LanguageModel> {
        Arc::new(CloudLanguageModel::<TP> {
            id: LanguageModelId::from(model.id.0.to_string()),
            model: model.clone(),
            token_provider: self.token_provider.clone(),
            http_client: self.http_client.clone(),
            app_version: self.app_version.clone(),
            request_limiter: RateLimiter::new(4),
            websocket_client: self.websocket_client.clone(),
            websocket_chains: self.websocket_chains.clone(),
        })
    }

    pub fn models(&self) -> &[Arc<cloud_llm_client::LanguageModel>] {
        &self.models
    }

    pub fn default_model(&self) -> Option<&Arc<cloud_llm_client::LanguageModel>> {
        self.default_model.as_ref()
    }

    pub fn default_fast_model(&self) -> Option<&Arc<cloud_llm_client::LanguageModel>> {
        self.default_fast_model.as_ref()
    }

    pub fn recommended_models(&self) -> &[Arc<cloud_llm_client::LanguageModel>] {
        &self.recommended_models
    }
}

pub fn map_cloud_completion_events<T, F>(
    stream: Pin<Box<dyn Stream<Item = Result<CompletionEvent<T>, ResponseStreamError>> + Send>>,
    provider: &LanguageModelProviderName,
    mut map_callback: F,
) -> BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
where
    T: DeserializeOwned + 'static,
    F: FnMut(T) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
        + Send
        + 'static,
{
    let provider = provider.clone();
    let mut stream = stream.fuse();

    let mut saw_stream_ended = false;

    let mut done = false;
    let mut pending = VecDeque::new();

    stream::poll_fn(move |cx| {
        loop {
            if let Some(item) = pending.pop_front() {
                return Poll::Ready(Some(item));
            }

            if done {
                return Poll::Ready(None);
            }

            match stream.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => {
                    let items = match event {
                        Err(error) => {
                            vec![Err(error.into_completion_error(provider.clone()))]
                        }
                        Ok(CompletionEvent::Status(CompletionRequestStatus::StreamEnded)) => {
                            saw_stream_ended = true;
                            vec![]
                        }
                        Ok(CompletionEvent::Status(status)) => {
                            LanguageModelCompletionEvent::from_completion_request_status(
                                status,
                                provider.clone(),
                            )
                            .transpose()
                            .map(|event| vec![event])
                            .unwrap_or_default()
                        }
                        Ok(CompletionEvent::Event(event)) => map_callback(event),
                    };
                    pending.extend(items);
                }
                Poll::Ready(None) => {
                    done = true;

                    if !saw_stream_ended {
                        return Poll::Ready(Some(Err(
                            LanguageModelCompletionError::StreamEndedUnexpectedly {
                                provider: provider.clone(),
                            },
                        )));
                    }
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    })
    .boxed()
}

pub fn provider_name(
    provider: &cloud_llm_client::LanguageModelProvider,
) -> LanguageModelProviderName {
    match provider {
        cloud_llm_client::LanguageModelProvider::Anthropic => ANTHROPIC_PROVIDER_NAME,
        cloud_llm_client::LanguageModelProvider::OpenAi => OPEN_AI_PROVIDER_NAME,
        cloud_llm_client::LanguageModelProvider::Google => GOOGLE_PROVIDER_NAME,
        cloud_llm_client::LanguageModelProvider::XAi => X_AI_PROVIDER_NAME,
    }
}

/// A failure while reading the streamed completion response body.
///
/// Kept as a typed error (rather than `anyhow::Error`) so the consumer can
/// attach the provider name and build a structured
/// [`LanguageModelCompletionError`] without a runtime downcast.
pub enum ResponseStreamError {
    Read(std::io::Error),
    Deserialize(serde_json::Error),
}

impl ResponseStreamError {
    fn into_completion_error(
        self,
        provider: LanguageModelProviderName,
    ) -> LanguageModelCompletionError {
        match self {
            ResponseStreamError::Read(error) => {
                LanguageModelCompletionError::ApiReadResponseError { provider, error }
            }
            ResponseStreamError::Deserialize(error) => {
                LanguageModelCompletionError::DeserializeResponse { provider, error }
            }
        }
    }
}

pub fn response_lines<T: DeserializeOwned>(
    response: Response<AsyncBody>,
    includes_status_messages: bool,
) -> impl Stream<Item = Result<CompletionEvent<T>, ResponseStreamError>> {
    futures::stream::try_unfold(
        (String::new(), BufReader::new(response.into_body())),
        move |(mut line, mut body)| async move {
            match body.read_line(&mut line).await {
                Ok(0) => Ok(None),
                Ok(_) => {
                    let event = if includes_status_messages {
                        serde_json::from_str::<CompletionEvent<T>>(&line)
                            .map_err(ResponseStreamError::Deserialize)?
                    } else {
                        CompletionEvent::Event(
                            serde_json::from_str::<T>(&line)
                                .map_err(ResponseStreamError::Deserialize)?,
                        )
                    };

                    line.clear();
                    Ok(Some((event, (line, body))))
                }
                Err(error) => Err(ResponseStreamError::Read(error)),
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_client::FakeHttpClient;
    use http_client::http::{HeaderMap, StatusCode};
    use language_model::{
        LanguageModelCompletionError, LanguageModelRequestMessage, MessageContent, Role, Speed,
    };
    use serde_json::json;
    use std::sync::Mutex;

    #[gpui::test]
    async fn cloud_explicit_compaction_forwards_supported_request_fields(
        cx: &mut gpui::TestAppContext,
    ) {
        let captured_request = Arc::new(Mutex::new(None));
        let captured_request_for_handler = captured_request.clone();
        let http_client = FakeHttpClient::create(move |request| {
            let captured_request = captured_request_for_handler.clone();
            async move {
                let method = request.method().clone();
                let uri = request.uri().to_string();
                let authorization = request
                    .headers()
                    .get("Authorization")
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_string);
                let requested_status_messages = request
                    .headers()
                    .contains_key(CLIENT_SUPPORTS_STATUS_MESSAGES_HEADER_NAME);
                let requested_stream_end = request
                    .headers()
                    .contains_key(CLIENT_SUPPORTS_STATUS_STREAM_ENDED_HEADER_NAME);
                let mut body = request.into_body();
                let mut body_text = String::new();
                body.read_to_string(&mut body_text).await?;
                *captured_request.lock().unwrap() = Some((
                    method,
                    uri,
                    authorization,
                    requested_status_messages,
                    requested_stream_end,
                    body_text,
                ));

                Ok(http_client::Response::builder()
                    .status(200)
                    .body(AsyncBody::from(format!(
                        "{}\n",
                        json!({
                            "id": "resp_compact",
                            "created_at": 1_700_000_000,
                            "object": "response.compaction",
                            "output": [{
                                "type": "compaction",
                                "id": "cmp_manual",
                                "encrypted_content": "opaque-state"
                            }],
                            "usage": {
                                "input_tokens": 100,
                                "input_tokens_details": {"cached_tokens": 20},
                                "output_tokens": 10,
                                "output_tokens_details": {"reasoning_tokens": 5},
                                "total_tokens": 110
                            }
                        })
                    )))?)
            }
        });
        let model = cloud_test_model(http_client);
        let request = compact_test_request();

        let result = model.compact(request, &cx.to_async()).await.unwrap();

        assert_eq!(
            result.usage,
            language_model::TokenUsage {
                input_tokens: 80,
                output_tokens: 10,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 20,
            }
        );
        let language_model::CompactedContext::ProviderState(state) = result.context else {
            panic!("expected provider compaction state");
        };
        assert_eq!(
            open_ai::responses::provider_compaction_items(&state, &OPEN_AI_PROVIDER_ID).unwrap(),
            Some(vec![json!({
                "type": "compaction",
                "id": "cmp_manual",
                "encrypted_content": "opaque-state"
            })])
        );
        let (method, uri, authorization, requested_status_messages, requested_stream_end, body) =
            captured_request.lock().unwrap().take().unwrap();
        assert_eq!(method, Method::POST);
        assert_eq!(uri, "http://test.example/completions/compact?");
        assert_eq!(authorization.as_deref(), Some("Bearer test-token"));
        assert!(!requested_status_messages);
        assert!(!requested_stream_end);
        let body = serde_json::from_str::<serde_json::Value>(&body).unwrap();
        assert_eq!(body["thread_id"], "thread-123");
        assert_eq!(body["provider"], "open_ai");
        assert_eq!(body["model"], "gpt-5.4");
        assert_eq!(
            body["provider_request"],
            json!({
                "model": "gpt-5.4",
                "input": [{
                    "type": "message",
                    "role": "user",
                    "content": [{
                        "type": "input_text",
                        "text": "Retain this context."
                    }]
                }],
                "prompt_cache_key": "thread-123",
                "service_tier": "priority"
            })
        );
    }

    #[gpui::test]
    async fn cloud_explicit_compaction_rejects_output_without_compaction_item(
        cx: &mut gpui::TestAppContext,
    ) {
        let http_client = FakeHttpClient::create(|_| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body(AsyncBody::from(format!(
                    "{}\n",
                    json!({
                        "id": "resp_compact",
                        "created_at": 1_700_000_000,
                        "object": "response.compaction",
                        "output": [{
                            "type": "message",
                            "role": "assistant",
                            "content": "This is not an opaque compaction item."
                        }],
                        "usage": {
                            "input_tokens": 100,
                            "input_tokens_details": {"cached_tokens": 20},
                            "output_tokens": 10,
                            "output_tokens_details": {"reasoning_tokens": 5},
                            "total_tokens": 110
                        }
                    })
                )))?)
        });
        let model = cloud_test_model(http_client);

        let error = model
            .compact(compact_test_request(), &cx.to_async())
            .await
            .unwrap_err();

        assert!(
            matches!(&error, LanguageModelCompletionError::Other(_)),
            "expected invalid canonical output to be rejected, got {error:?}"
        );
        assert!(error.to_string().contains("compaction item"));
    }

    use futures::future;
    use gpui::TestAppContext;
    use websocket_client::{WebSocketConnection, WebSocketMessage};

    #[gpui::test]
    async fn websocket_turns_send_completion_bodies_and_continue_incrementally(
        cx: &mut TestAppContext,
    ) {
        let http_client = FakeHttpClient::with_404_response();
        let sent_frames = Arc::new(Mutex::new(Vec::new()));
        let connects = Arc::new(Mutex::new(Vec::new()));
        let websocket_client = Arc::new(FakeWebSocketClient {
            connects: connects.clone(),
            connect_results: Mutex::new(vec![Ok(Box::new(ScriptedConnection {
                sent: sent_frames.clone(),
                incoming: vec![
                    text_event(r#"{"type":"response.created","response":{"id":"resp_1"}}"#),
                    text_event(
                        r#"{"type":"response.completed","response":{"id":"resp_1","output":[{"type":"message","id":"msg_1","role":"assistant","content":[{"type":"output_text","text":"Hello.","annotations":[]}]}]}}"#,
                    ),
                    text_event(r#"{"type":"response.created","response":{"id":"resp_2"}}"#),
                    text_event(
                        r#"{"type":"response.completed","response":{"id":"resp_2","output":[]}}"#,
                    ),
                ],
            }) as Box<dyn WebSocketConnection>)]),
        });
        let token_provider = FakeTokenProvider::default();
        let websocket_chains = WebSocketChains::new_shared();

        let events = CloudLanguageModel::<FakeTokenProvider>::stream_open_ai_websocket_completion(
            websocket_client.clone(),
            &http_client,
            &token_provider,
            (),
            None,
            websocket_chains.clone(),
            cx.executor().clone(),
            Some("thread-1".to_string()),
            None,
            &test_responses_request(vec![user_message("Find fizz_buzz")]),
        )
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;

        assert_eq!(events.len(), 2);
        assert!(matches!(
            events[1],
            Ok(open_ai::responses::StreamEvent::Completed { ref response })
                if response.id.as_deref() == Some("resp_1")
        ));
        {
            let connects = connects.lock().unwrap();
            assert_eq!(
                connects.as_slice(),
                &[(
                    "ws://test.example/completions/session".to_string(),
                    Some("cached-token".to_string())
                )]
            );
        }
        let first_frame: CompletionBody = {
            let sent_frames = sent_frames.lock().unwrap();
            assert_eq!(sent_frames.len(), 1);
            serde_json::from_str(&sent_frames[0]).unwrap()
        };
        assert_eq!(first_frame.thread_id.as_deref(), Some("thread-1"));
        assert_eq!(
            first_frame.provider,
            cloud_llm_client::LanguageModelProvider::OpenAi
        );
        assert_eq!(first_frame.model, "gpt-test");
        assert!(first_frame.provider_request.get("stream").is_none());
        assert!(first_frame.provider_request.get("type").is_none());
        assert!(
            first_frame
                .provider_request
                .get("previous_response_id")
                .is_none()
        );
        assert_eq!(
            first_frame.provider_request["input"]
                .as_array()
                .unwrap()
                .len(),
            1
        );

        // The next turn extends the conversation with the previous
        // response's replayed output and a new user message; only the new
        // message is sent, chained via `previous_response_id`, over the
        // cached connection.
        let events = CloudLanguageModel::<FakeTokenProvider>::stream_open_ai_websocket_completion(
            websocket_client,
            &http_client,
            &token_provider,
            (),
            None,
            websocket_chains,
            cx.executor().clone(),
            Some("thread-1".to_string()),
            None,
            &test_responses_request(vec![
                user_message("Find fizz_buzz"),
                assistant_message("Hello."),
                user_message("Now optimize it."),
            ]),
        )
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;

        assert_eq!(events.len(), 2);
        assert!(matches!(
            events[1],
            Ok(open_ai::responses::StreamEvent::Completed { ref response })
                if response.id.as_deref() == Some("resp_2")
        ));
        assert_eq!(
            connects.lock().unwrap().len(),
            1,
            "expected connection reuse"
        );
        let second_frame: CompletionBody = {
            let sent_frames = sent_frames.lock().unwrap();
            assert_eq!(sent_frames.len(), 2);
            serde_json::from_str(&sent_frames[1]).unwrap()
        };
        assert_eq!(
            second_frame.provider_request["previous_response_id"],
            "resp_1"
        );
        let input = second_frame.provider_request["input"].as_array().unwrap();
        assert_eq!(input.len(), 1);
        assert_eq!(input[0]["content"][0]["text"], "Now optimize it.");
    }

    #[gpui::test]
    async fn websocket_connect_refreshes_a_rejected_llm_token_once(cx: &mut TestAppContext) {
        let http_client = FakeHttpClient::with_404_response();
        let connects = Arc::new(Mutex::new(Vec::new()));
        let websocket_client = Arc::new(FakeWebSocketClient {
            connects: connects.clone(),
            connect_results: Mutex::new(vec![
                Err(anyhow::anyhow!(websocket_client::AuthRequired)),
                Ok(Box::new(ScriptedConnection {
                    sent: Arc::default(),
                    incoming: vec![text_event(
                        r#"{"type":"response.completed","response":{"id":"resp_1","output":[]}}"#,
                    )],
                }) as Box<dyn WebSocketConnection>),
            ]),
        });
        let token_provider = FakeTokenProvider::default();

        let events = CloudLanguageModel::<FakeTokenProvider>::stream_open_ai_websocket_completion(
            websocket_client,
            &http_client,
            &token_provider,
            (),
            None,
            WebSocketChains::new_shared(),
            cx.executor().clone(),
            None,
            None,
            &test_responses_request(vec![user_message("hi")]),
        )
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;

        assert_eq!(events.len(), 1);
        let connects = connects.lock().unwrap();
        assert_eq!(connects.len(), 2);
        assert_eq!(connects[0].1.as_deref(), Some("cached-token"));
        assert_eq!(connects[1].1.as_deref(), Some("refreshed-token"));
        assert_eq!(*token_provider.refresh_count.lock().unwrap(), 1);
    }

    fn text_event(json: &str) -> anyhow::Result<WebSocketMessage> {
        Ok(WebSocketMessage::Text(json.to_string()))
    }

    fn user_message(text: &str) -> open_ai::responses::ResponseInputItem {
        open_ai::responses::ResponseInputItem::Message(open_ai::responses::ResponseMessageItem {
            role: open_ai::Role::User,
            content: vec![open_ai::responses::ResponseInputContent::Text {
                text: text.to_string(),
            }],
            phase: None,
        })
    }

    fn assistant_message(text: &str) -> open_ai::responses::ResponseInputItem {
        open_ai::responses::ResponseInputItem::Message(open_ai::responses::ResponseMessageItem {
            role: open_ai::Role::Assistant,
            content: vec![open_ai::responses::ResponseInputContent::OutputText {
                text: text.to_string(),
                annotations: Vec::new(),
            }],
            phase: None,
        })
    }

    fn test_responses_request(
        input: Vec<open_ai::responses::ResponseInputItem>,
    ) -> open_ai::responses::Request {
        open_ai::responses::Request {
            model: "gpt-test".to_string(),
            instructions: None,
            input: open_ai::responses::ResponseInput::new(Vec::new(), input),
            include: Vec::new(),
            stream: true,
            temperature: None,
            top_p: None,
            max_output_tokens: None,
            parallel_tool_calls: None,
            tool_choice: None,
            tools: Vec::new(),
            prompt_cache_key: None,
            reasoning: None,
            store: Some(false),
            service_tier: None,
            context_management: None,
        }
    }

    #[derive(Default)]
    struct FakeTokenProvider {
        refresh_count: Arc<Mutex<usize>>,
    }

    impl CloudLlmTokenProvider for FakeTokenProvider {
        type AuthContext = ();

        fn auth_context(&self, _cx: &impl AppContext) -> Self::AuthContext {}

        fn cached_token(&self, _: ()) -> BoxFuture<'static, Result<String>> {
            future::ready(Ok("cached-token".to_string())).boxed()
        }

        fn refresh_token(&self, _: ()) -> BoxFuture<'static, Result<String>> {
            *self.refresh_count.lock().unwrap() += 1;
            future::ready(Ok("refreshed-token".to_string())).boxed()
        }

        fn has_data_retention_consent(&self, _cx: &impl AppContext) -> bool {
            true
        }
    }

    /// A [`WebSocketClient`] that records connection attempts and hands out
    /// scripted connections.
    struct FakeWebSocketClient {
        connects: Arc<Mutex<Vec<(String, Option<String>)>>>,
        connect_results: Mutex<Vec<Result<Box<dyn WebSocketConnection>>>>,
    }

    impl WebSocketClient for FakeWebSocketClient {
        fn connect(
            &self,
            url: &str,
            headers: HeaderMap,
        ) -> futures::future::BoxFuture<'static, Result<Box<dyn WebSocketConnection>>> {
            let auth_token = headers
                .get(http_client::http::header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.strip_prefix("Bearer "))
                .map(str::to_string);
            self.connects
                .lock()
                .unwrap()
                .push((url.to_string(), auth_token));
            let result = {
                let mut connect_results = self.connect_results.lock().unwrap();
                if connect_results.is_empty() {
                    Err(anyhow::anyhow!("no scripted connection left"))
                } else {
                    connect_results.remove(0)
                }
            };
            future::ready(result).boxed()
        }
    }

    /// A [`WebSocketConnection`] that records sent text frames and replays a
    /// fixed sequence of incoming messages, reporting closure once they run
    /// out.
    struct ScriptedConnection {
        sent: Arc<Mutex<Vec<String>>>,
        incoming: Vec<Result<WebSocketMessage>>,
    }

    impl WebSocketConnection for ScriptedConnection {
        fn send(
            &mut self,
            message: WebSocketMessage,
        ) -> futures::future::BoxFuture<'_, Result<()>> {
            if let WebSocketMessage::Text(text) = message {
                self.sent.lock().unwrap().push(text);
            }
            future::ready(Ok(())).boxed()
        }

        fn receive(&mut self) -> futures::future::BoxFuture<'_, Option<Result<WebSocketMessage>>> {
            let message = if self.incoming.is_empty() {
                None
            } else {
                Some(self.incoming.remove(0))
            };
            future::ready(message).boxed()
        }
    }

    #[test]
    fn test_api_error_conversion_with_upstream_http_error() {
        // upstream_http_error with 503 status should become ServerOverloaded
        let error_body = r#"{"code":"upstream_http_error","message":"Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers, reset reason: connection timeout","upstream_status":503}"#;

        let api_error = ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: error_body.to_string(),
            headers: HeaderMap::new(),
        };

        let completion_error: LanguageModelCompletionError = api_error.into();

        match completion_error {
            LanguageModelCompletionError::UpstreamProviderError { message, .. } => {
                assert_eq!(
                    message,
                    "Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers, reset reason: connection timeout"
                );
            }
            _ => panic!(
                "Expected UpstreamProviderError for upstream 503, got: {:?}",
                completion_error
            ),
        }

        // upstream_http_error with 500 status should become ApiInternalServerError
        let error_body = r#"{"code":"upstream_http_error","message":"Received an error from the OpenAI API: internal server error","upstream_status":500}"#;

        let api_error = ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: error_body.to_string(),
            headers: HeaderMap::new(),
        };

        let completion_error: LanguageModelCompletionError = api_error.into();

        match completion_error {
            LanguageModelCompletionError::UpstreamProviderError { message, .. } => {
                assert_eq!(
                    message,
                    "Received an error from the OpenAI API: internal server error"
                );
            }
            _ => panic!(
                "Expected UpstreamProviderError for upstream 500, got: {:?}",
                completion_error
            ),
        }

        // upstream_http_error with 429 status should become RateLimitExceeded
        let error_body = r#"{"code":"upstream_http_error","message":"Received an error from the Google API: rate limit exceeded","upstream_status":429}"#;

        let api_error = ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: error_body.to_string(),
            headers: HeaderMap::new(),
        };

        let completion_error: LanguageModelCompletionError = api_error.into();

        match completion_error {
            LanguageModelCompletionError::UpstreamProviderError { message, .. } => {
                assert_eq!(
                    message,
                    "Received an error from the Google API: rate limit exceeded"
                );
            }
            _ => panic!(
                "Expected UpstreamProviderError for upstream 429, got: {:?}",
                completion_error
            ),
        }

        // Regular 500 error without upstream_http_error should remain ApiInternalServerError for Zed
        let error_body = "Regular internal server error";

        let api_error = ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: error_body.to_string(),
            headers: HeaderMap::new(),
        };

        let completion_error: LanguageModelCompletionError = api_error.into();

        match completion_error {
            LanguageModelCompletionError::ApiInternalServerError { provider, message } => {
                assert_eq!(provider, PROVIDER_NAME);
                assert_eq!(message, "Regular internal server error");
            }
            _ => panic!(
                "Expected ApiInternalServerError for regular 500, got: {:?}",
                completion_error
            ),
        }

        // upstream_http_429 format should be converted to UpstreamProviderError
        let error_body = r#"{"code":"upstream_http_429","message":"Upstream Anthropic rate limit exceeded.","retry_after":30.5}"#;

        let api_error = ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: error_body.to_string(),
            headers: HeaderMap::new(),
        };

        let completion_error: LanguageModelCompletionError = api_error.into();

        match completion_error {
            LanguageModelCompletionError::UpstreamProviderError {
                message,
                status,
                retry_after,
            } => {
                assert_eq!(message, "Upstream Anthropic rate limit exceeded.");
                assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
                assert_eq!(retry_after, Some(Duration::from_secs_f64(30.5)));
            }
            _ => panic!(
                "Expected UpstreamProviderError for upstream_http_429, got: {:?}",
                completion_error
            ),
        }

        // Invalid JSON in error body should fall back to regular error handling
        let error_body = "Not JSON at all";

        let api_error = ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: error_body.to_string(),
            headers: HeaderMap::new(),
        };

        let completion_error: LanguageModelCompletionError = api_error.into();

        match completion_error {
            LanguageModelCompletionError::ApiInternalServerError { provider, .. } => {
                assert_eq!(provider, PROVIDER_NAME);
            }
            _ => panic!(
                "Expected ApiInternalServerError for invalid JSON, got: {:?}",
                completion_error
            ),
        }
    }

    #[test]
    fn test_response_stream_error_maps_to_structured_variant() {
        // Read/deserialize failures mid-stream must keep their structured
        // variant rather than collapsing into `Other` (the source of the
        // generic "Request failed." message).
        let read = ResponseStreamError::Read(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
            .into_completion_error(PROVIDER_NAME);
        assert!(
            matches!(
                read,
                LanguageModelCompletionError::ApiReadResponseError { .. }
            ),
            "Expected ApiReadResponseError, got: {read:?}"
        );

        let deserialize = ResponseStreamError::Deserialize(
            serde_json::from_str::<serde_json::Value>("not json").unwrap_err(),
        )
        .into_completion_error(PROVIDER_NAME);
        assert!(
            matches!(
                deserialize,
                LanguageModelCompletionError::DeserializeResponse { .. }
            ),
            "Expected DeserializeResponse, got: {deserialize:?}"
        );
    }

    fn compact_test_request() -> LanguageModelRequest {
        LanguageModelRequest {
            thread_id: Some("thread-123".to_string()),
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Retain this context.".to_string())],
                cache: false,
                reasoning_details: None,
            }],
            speed: Some(Speed::Fast),
            ..Default::default()
        }
    }

    fn cloud_test_model(
        http_client: Arc<HttpClientWithUrl>,
    ) -> CloudLanguageModel<TestTokenProvider> {
        CloudLanguageModel {
            id: LanguageModelId::from("gpt-5.4".to_string()),
            model: Arc::new(cloud_llm_client::LanguageModel {
                provider: cloud_llm_client::LanguageModelProvider::OpenAi,
                id: cloud_llm_client::LanguageModelId(Arc::from("gpt-5.4")),
                display_name: "GPT-5.4".to_string(),
                is_latest: true,
                max_token_count: 1_000_000,
                max_token_count_in_max_mode: None,
                max_output_tokens: 128_000,
                supports_tools: true,
                supports_images: true,
                supports_thinking: true,
                supports_disabling_thinking: true,
                supports_fast_mode: true,
                supports_server_side_compaction: true,
                supported_effort_levels: Vec::new(),
                supports_streaming_tools: true,
                supports_parallel_tool_calls: true,
                is_disabled: false,
                disabled_reason: None,
            }),
            token_provider: Arc::new(TestTokenProvider),
            http_client,
            app_version: None,
            request_limiter: RateLimiter::new(4),
            websocket_client: None,
            websocket_chains: WebSocketChains::new_shared(),
        }
    }

    struct TestTokenProvider;

    impl CloudLlmTokenProvider for TestTokenProvider {
        type AuthContext = ();

        fn auth_context(&self, _cx: &impl AppContext) -> Self::AuthContext {}

        fn cached_token(
            &self,
            _auth_context: Self::AuthContext,
        ) -> BoxFuture<'static, Result<String>> {
            async { Ok("test-token".to_string()) }.boxed()
        }

        fn refresh_token(
            &self,
            _auth_context: Self::AuthContext,
        ) -> BoxFuture<'static, Result<String>> {
            async { Ok("refreshed-test-token".to_string()) }.boxed()
        }

        fn has_data_retention_consent(&self, _cx: &impl AppContext) -> bool {
            false
        }
    }
}
