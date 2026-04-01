use anthropic::AnthropicModelMode;
use anyhow::{Result, anyhow};
use cloud_llm_client::{
    CLIENT_SUPPORTS_STATUS_MESSAGES_HEADER_NAME, CLIENT_SUPPORTS_STATUS_STREAM_ENDED_HEADER_NAME,
    CompletionBody, CompletionEvent, CompletionRequestStatus, CountTokensBody, CountTokensResponse,
    EXPIRED_LLM_TOKEN_HEADER_NAME, OUTDATED_LLM_TOKEN_HEADER_NAME,
    SERVER_SUPPORTS_STATUS_MESSAGES_HEADER_NAME, ZED_VERSION_HEADER_NAME,
};
use futures::{
    AsyncBufReadExt, FutureExt, Stream, StreamExt,
    future::BoxFuture,
    stream::{self, BoxStream},
};
use google_ai::GoogleModelMode;
use gpui::{App, AppContext, AsyncApp};
use http_client::http::{HeaderMap, HeaderValue};
use http_client::{
    AsyncBody, HttpClient, HttpClientWithUrl, HttpRequestExt, Method, Response, StatusCode,
};
use language_model::{
    ANTHROPIC_PROVIDER_ID, ANTHROPIC_PROVIDER_NAME, GOOGLE_PROVIDER_ID, GOOGLE_PROVIDER_NAME,
    LanguageModel, LanguageModelCacheConfiguration, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolSchemaFormat, OPEN_AI_PROVIDER_ID,
    OPEN_AI_PROVIDER_NAME, PaymentRequiredError, RateLimiter, X_AI_PROVIDER_ID, X_AI_PROVIDER_NAME,
    ZED_CLOUD_PROVIDER_ID, ZED_CLOUD_PROVIDER_NAME,
};

use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use smol::io::{AsyncReadExt, BufReader};
use std::collections::VecDeque;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;
use thiserror::Error;

use anthropic::completion::{
    AnthropicEventMapper, count_anthropic_tokens_with_tiktoken, into_anthropic,
};
use google_ai::completion::{GoogleEventMapper, into_google};
use open_ai::completion::{
    OpenAiEventMapper, OpenAiResponseEventMapper, count_open_ai_tokens, into_open_ai,
    into_open_ai_response,
};
use x_ai::completion::count_xai_tokens;

const PROVIDER_ID: LanguageModelProviderId = ZED_CLOUD_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = ZED_CLOUD_PROVIDER_NAME;

/// Trait for acquiring and refreshing LLM authentication tokens.
pub trait CloudLlmTokenProvider: Send + Sync {
    fn acquire_token(&self) -> BoxFuture<'_, Result<String>>;
    fn refresh_token(&self) -> BoxFuture<'_, Result<String>>;
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

pub struct CloudLanguageModel {
    pub id: LanguageModelId,
    pub model: Arc<cloud_llm_client::LanguageModel>,
    pub token_provider: Arc<dyn CloudLlmTokenProvider>,
    pub http_client: Arc<HttpClientWithUrl>,
    pub app_version: Option<Version>,
    pub request_limiter: RateLimiter,
}

pub struct PerformLlmCompletionResponse {
    pub response: Response<AsyncBody>,
    pub includes_status_messages: bool,
}

impl CloudLanguageModel {
    pub async fn perform_llm_completion(
        http_client: &HttpClientWithUrl,
        token_provider: &dyn CloudLlmTokenProvider,
        app_version: Option<Version>,
        body: CompletionBody,
    ) -> Result<PerformLlmCompletionResponse> {
        let mut token = token_provider.acquire_token().await?;
        let mut refreshed_token = false;

        loop {
            let request = http_client::Request::builder()
                .method(Method::POST)
                .uri(http_client.build_zed_llm_url("/completions", &[])?.as_ref())
                .when_some(app_version.as_ref(), |builder, app_version| {
                    builder.header(ZED_VERSION_HEADER_NAME, app_version.to_string())
                })
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {token}"))
                .header(CLIENT_SUPPORTS_STATUS_MESSAGES_HEADER_NAME, "true")
                .header(CLIENT_SUPPORTS_STATUS_STREAM_ENDED_HEADER_NAME, "true")
                .body(serde_json::to_string(&body)?.into())?;

            let mut response = http_client.send(request).await?;
            let status = response.status();
            if status.is_success() {
                let includes_status_messages = response
                    .headers()
                    .get(SERVER_SUPPORTS_STATUS_MESSAGES_HEADER_NAME)
                    .is_some();

                return Ok(PerformLlmCompletionResponse {
                    response,
                    includes_status_messages,
                });
            }

            if !refreshed_token && needs_llm_token_refresh(&response) {
                token = token_provider.refresh_token().await?;
                refreshed_token = true;
                continue;
            }

            if status == StatusCode::PAYMENT_REQUIRED {
                return Err(anyhow!(PaymentRequiredError));
            }

            let mut body = String::new();
            let headers = response.headers().clone();
            response.body_mut().read_to_string(&mut body).await?;
            return Err(anyhow!(ApiError {
                status,
                body,
                headers
            }));
        }
    }
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

impl LanguageModel for CloudLanguageModel {
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

    fn supports_tools(&self) -> bool {
        self.model.supports_tools
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images
    }

    fn supports_thinking(&self) -> bool {
        self.model.supports_thinking
    }

    fn supports_fast_mode(&self) -> bool {
        self.model.supports_fast_mode
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

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        match &self.model.provider {
            cloud_llm_client::LanguageModelProvider::Anthropic => {
                Some(LanguageModelCacheConfiguration {
                    min_total_token: 2_048,
                    should_speculate: true,
                    max_cache_anchors: 4,
                })
            }
            cloud_llm_client::LanguageModelProvider::OpenAi
            | cloud_llm_client::LanguageModelProvider::XAi
            | cloud_llm_client::LanguageModelProvider::Google => None,
        }
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        match self.model.provider {
            cloud_llm_client::LanguageModelProvider::Anthropic => cx
                .background_spawn(async move { count_anthropic_tokens_with_tiktoken(request) })
                .boxed(),
            cloud_llm_client::LanguageModelProvider::OpenAi => {
                let model = match open_ai::Model::from_id(&self.model.id.0) {
                    Ok(model) => model,
                    Err(err) => return async move { Err(anyhow!(err)) }.boxed(),
                };
                cx.background_spawn(async move { count_open_ai_tokens(request, model) })
                    .boxed()
            }
            cloud_llm_client::LanguageModelProvider::XAi => {
                let model = match x_ai::Model::from_id(&self.model.id.0) {
                    Ok(model) => model,
                    Err(err) => return async move { Err(anyhow!(err)) }.boxed(),
                };
                cx.background_spawn(async move { count_xai_tokens(request, model) })
                    .boxed()
            }
            cloud_llm_client::LanguageModelProvider::Google => {
                let http_client = self.http_client.clone();
                let token_provider = self.token_provider.clone();
                let model_id = self.model.id.to_string();
                let generate_content_request =
                    into_google(request, model_id.clone(), GoogleModelMode::Default);
                async move {
                    let token = token_provider.acquire_token().await?;

                    let request_body = CountTokensBody {
                        provider: cloud_llm_client::LanguageModelProvider::Google,
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
                    let headers = response.headers().clone();
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
                            body: response_body,
                            headers
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
        _cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
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

                let mut request = into_anthropic(
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
                );

                if enable_thinking && effort.is_some() {
                    request.thinking = Some(anthropic::Thinking::Adaptive);
                    request.output_config = Some(anthropic::OutputConfig { effort });
                }

                let http_client = self.http_client.clone();
                let token_provider = self.token_provider.clone();
                let future = self.request_limiter.stream(async move {
                    let PerformLlmCompletionResponse {
                        response,
                        includes_status_messages,
                    } = Self::perform_llm_completion(
                        &http_client,
                        &*token_provider,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            provider: cloud_llm_client::LanguageModelProvider::Anthropic,
                            model: request.model.clone(),
                            provider_request: serde_json::to_value(&request)
                                .map_err(|e| anyhow!(e))?,
                        },
                    )
                    .await
                    .map_err(|err| match err.downcast::<ApiError>() {
                        Ok(api_err) => anyhow!(LanguageModelCompletionError::from(api_err)),
                        Err(err) => anyhow!(err),
                    })?;

                    let mut mapper = AnthropicEventMapper::new();
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
                    .and_then(|effort| open_ai::ReasoningEffort::from_str(effort).ok());

                let mut request = into_open_ai_response(
                    request,
                    &self.model.id.0,
                    self.model.supports_parallel_tool_calls,
                    true,
                    None,
                    None,
                );

                if enable_thinking && let Some(effort) = effort {
                    request.reasoning = Some(open_ai::responses::ReasoningConfig {
                        effort,
                        summary: Some(open_ai::responses::ReasoningSummaryMode::Auto),
                    });
                }

                let future = self.request_limiter.stream(async move {
                    let PerformLlmCompletionResponse {
                        response,
                        includes_status_messages,
                    } = Self::perform_llm_completion(
                        &http_client,
                        &*token_provider,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            provider: cloud_llm_client::LanguageModelProvider::OpenAi,
                            model: request.model.clone(),
                            provider_request: serde_json::to_value(&request)
                                .map_err(|e| anyhow!(e))?,
                        },
                    )
                    .await?;

                    let mut mapper = OpenAiResponseEventMapper::new();
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
                let request = into_open_ai(
                    request,
                    &self.model.id.0,
                    self.model.supports_parallel_tool_calls,
                    false,
                    None,
                    None,
                );
                let future = self.request_limiter.stream(async move {
                    let PerformLlmCompletionResponse {
                        response,
                        includes_status_messages,
                    } = Self::perform_llm_completion(
                        &http_client,
                        &*token_provider,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            provider: cloud_llm_client::LanguageModelProvider::XAi,
                            model: request.model.clone(),
                            provider_request: serde_json::to_value(&request)
                                .map_err(|e| anyhow!(e))?,
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
                    into_google(request, self.model.id.to_string(), GoogleModelMode::Default);
                let future = self.request_limiter.stream(async move {
                    let PerformLlmCompletionResponse {
                        response,
                        includes_status_messages,
                    } = Self::perform_llm_completion(
                        &http_client,
                        &*token_provider,
                        app_version,
                        CompletionBody {
                            thread_id,
                            prompt_id,
                            provider: cloud_llm_client::LanguageModelProvider::Google,
                            model: request.model.model_id.clone(),
                            provider_request: serde_json::to_value(&request)
                                .map_err(|e| anyhow!(e))?,
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

pub fn map_cloud_completion_events<T, F>(
    stream: Pin<Box<dyn Stream<Item = Result<CompletionEvent<T>>> + Send>>,
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
                            vec![Err(LanguageModelCompletionError::from(error))]
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

pub fn response_lines<T: DeserializeOwned>(
    response: Response<AsyncBody>,
    includes_status_messages: bool,
) -> impl Stream<Item = Result<CompletionEvent<T>>> {
    futures::stream::try_unfold(
        (String::new(), BufReader::new(response.into_body())),
        move |(mut line, mut body)| async move {
            match body.read_line(&mut line).await {
                Ok(0) => Ok(None),
                Ok(_) => {
                    let event = if includes_status_messages {
                        serde_json::from_str::<CompletionEvent<T>>(&line)?
                    } else {
                        CompletionEvent::Event(serde_json::from_str::<T>(&line)?)
                    };

                    line.clear();
                    Ok(Some((event, (line, body))))
                }
                Err(e) => Err(e.into()),
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_client::http::{HeaderMap, StatusCode};
    use language_model::LanguageModelCompletionError;

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
}
