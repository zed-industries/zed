mod authorization;
pub mod db;
mod token;

use crate::api::events::SnowflakeRow;
use crate::api::CloudflareIpCountryHeader;
use crate::build_kinesis_client;
use crate::{db::UserId, executor::Executor, Cents, Config, Error, Result};
use anyhow::{anyhow, Context as _};
use authorization::authorize_access_to_language_model;
use axum::routing::get;
use axum::{
    body::Body,
    http::{self, HeaderName, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Extension, Json, Router, TypedHeader,
};
use chrono::{DateTime, Duration, Utc};
use collections::HashMap;
use db::TokenUsage;
use db::{usage_measure::UsageMeasure, ActiveUserCount, LlmDatabase};
use futures::{Stream, StreamExt as _};
use reqwest_client::ReqwestClient;
use rpc::{
    proto::Plan, LanguageModelProvider, PerformCompletionParams, EXPIRED_LLM_TOKEN_HEADER_NAME,
};
use rpc::{ListModelsResponse, MAX_LLM_MONTHLY_SPEND_REACHED_HEADER_NAME};
use serde_json::json;
use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use strum::IntoEnumIterator;
use tokio::sync::RwLock;
use util::ResultExt;

pub use token::*;

const ACTIVE_USER_COUNT_CACHE_DURATION: Duration = Duration::seconds(30);

pub struct LlmState {
    pub config: Config,
    pub executor: Executor,
    pub db: Arc<LlmDatabase>,
    pub http_client: ReqwestClient,
    pub kinesis_client: Option<aws_sdk_kinesis::Client>,
    active_user_count_by_model:
        RwLock<HashMap<(LanguageModelProvider, String), (DateTime<Utc>, ActiveUserCount)>>,
}

impl LlmState {
    pub async fn new(config: Config, executor: Executor) -> Result<Arc<Self>> {
        let database_url = config
            .llm_database_url
            .as_ref()
            .ok_or_else(|| anyhow!("missing LLM_DATABASE_URL"))?;
        let max_connections = config
            .llm_database_max_connections
            .ok_or_else(|| anyhow!("missing LLM_DATABASE_MAX_CONNECTIONS"))?;

        let mut db_options = db::ConnectOptions::new(database_url);
        db_options.max_connections(max_connections);
        let mut db = LlmDatabase::new(db_options, executor.clone()).await?;
        db.initialize().await?;

        let db = Arc::new(db);

        let user_agent = format!("Zed Server/{}", env!("CARGO_PKG_VERSION"));
        let http_client =
            ReqwestClient::user_agent(&user_agent).context("failed to construct http client")?;

        let this = Self {
            executor,
            db,
            http_client,
            kinesis_client: if config.kinesis_access_key.is_some() {
                build_kinesis_client(&config).await.log_err()
            } else {
                None
            },
            active_user_count_by_model: RwLock::new(HashMap::default()),
            config,
        };

        Ok(Arc::new(this))
    }

    pub async fn get_active_user_count(
        &self,
        provider: LanguageModelProvider,
        model: &str,
    ) -> Result<ActiveUserCount> {
        let now = Utc::now();

        {
            let active_user_count_by_model = self.active_user_count_by_model.read().await;
            if let Some((last_updated, count)) =
                active_user_count_by_model.get(&(provider, model.to_string()))
            {
                if now - *last_updated < ACTIVE_USER_COUNT_CACHE_DURATION {
                    return Ok(*count);
                }
            }
        }

        let mut cache = self.active_user_count_by_model.write().await;
        let new_count = self.db.get_active_user_count(provider, model, now).await?;
        cache.insert((provider, model.to_string()), (now, new_count));
        Ok(new_count)
    }
}

pub fn routes() -> Router<(), Body> {
    Router::new()
        .route("/models", get(list_models))
        .route("/completion", post(perform_completion))
        .layer(middleware::from_fn(validate_api_token))
}

async fn validate_api_token<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::http(
                StatusCode::BAD_REQUEST,
                "missing authorization header".to_string(),
            )
        })?
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            Error::http(
                StatusCode::BAD_REQUEST,
                "invalid authorization header".to_string(),
            )
        })?;

    let state = req.extensions().get::<Arc<LlmState>>().unwrap();
    match LlmTokenClaims::validate(token, &state.config) {
        Ok(claims) => {
            if state.db.is_access_token_revoked(&claims.jti).await? {
                return Err(Error::http(
                    StatusCode::UNAUTHORIZED,
                    "unauthorized".to_string(),
                ));
            }

            tracing::Span::current()
                .record("user_id", claims.user_id)
                .record("login", claims.github_user_login.clone())
                .record("authn.jti", &claims.jti)
                .record("is_staff", claims.is_staff);

            req.extensions_mut().insert(claims);
            Ok::<_, Error>(next.run(req).await.into_response())
        }
        Err(ValidateLlmTokenError::Expired) => Err(Error::Http(
            StatusCode::UNAUTHORIZED,
            "unauthorized".to_string(),
            [(
                HeaderName::from_static(EXPIRED_LLM_TOKEN_HEADER_NAME),
                HeaderValue::from_static("true"),
            )]
            .into_iter()
            .collect(),
        )),
        Err(_err) => Err(Error::http(
            StatusCode::UNAUTHORIZED,
            "unauthorized".to_string(),
        )),
    }
}

async fn list_models(
    Extension(state): Extension<Arc<LlmState>>,
    Extension(claims): Extension<LlmTokenClaims>,
    country_code_header: Option<TypedHeader<CloudflareIpCountryHeader>>,
) -> Result<Json<ListModelsResponse>> {
    let country_code = country_code_header.map(|header| header.to_string());

    let mut accessible_models = Vec::new();

    for (provider, model) in state.db.all_models() {
        let authorize_result = authorize_access_to_language_model(
            &state.config,
            &claims,
            country_code.as_deref(),
            provider,
            &model.name,
        );

        if authorize_result.is_ok() {
            accessible_models.push(rpc::LanguageModel {
                provider,
                name: model.name,
            });
        }
    }

    Ok(Json(ListModelsResponse {
        models: accessible_models,
    }))
}

async fn perform_completion(
    Extension(state): Extension<Arc<LlmState>>,
    Extension(claims): Extension<LlmTokenClaims>,
    country_code_header: Option<TypedHeader<CloudflareIpCountryHeader>>,
    Json(params): Json<PerformCompletionParams>,
) -> Result<impl IntoResponse> {
    let model = normalize_model_name(
        state.db.model_names_for_provider(params.provider),
        params.model,
    );

    authorize_access_to_language_model(
        &state.config,
        &claims,
        country_code_header
            .map(|header| header.to_string())
            .as_deref(),
        params.provider,
        &model,
    )?;

    check_usage_limit(&state, params.provider, &model, &claims).await?;

    let stream = match params.provider {
        LanguageModelProvider::Anthropic => {
            let api_key = if claims.is_staff {
                state
                    .config
                    .anthropic_staff_api_key
                    .as_ref()
                    .context("no Anthropic AI staff API key configured on the server")?
            } else {
                state
                    .config
                    .anthropic_api_key
                    .as_ref()
                    .context("no Anthropic AI API key configured on the server")?
            };

            let mut request: anthropic::Request =
                serde_json::from_str(params.provider_request.get())?;

            // Override the model on the request with the latest version of the model that is
            // known to the server.
            //
            // Right now, we use the version that's defined in `model.id()`, but we will likely
            // want to change this code once a new version of an Anthropic model is released,
            // so that users can use the new version, without having to update Zed.
            request.model = match model.as_str() {
                "claude-3-5-sonnet" => anthropic::Model::Claude3_5Sonnet.id().to_string(),
                "claude-3-7-sonnet" => anthropic::Model::Claude3_7Sonnet.id().to_string(),
                "claude-3-opus" => anthropic::Model::Claude3Opus.id().to_string(),
                "claude-3-haiku" => anthropic::Model::Claude3Haiku.id().to_string(),
                "claude-3-sonnet" => anthropic::Model::Claude3Sonnet.id().to_string(),
                _ => request.model,
            };

            let (chunks, rate_limit_info) = anthropic::stream_completion_with_rate_limit_info(
                &state.http_client,
                anthropic::ANTHROPIC_API_URL,
                api_key,
                request,
            )
            .await
            .map_err(|err| match err {
                anthropic::AnthropicError::ApiError(ref api_error) => match api_error.code() {
                    Some(anthropic::ApiErrorCode::RateLimitError) => {
                        tracing::info!(
                            target: "upstream rate limit exceeded",
                            user_id = claims.user_id,
                            login = claims.github_user_login,
                            authn.jti = claims.jti,
                            is_staff = claims.is_staff,
                            provider = params.provider.to_string(),
                            model = model
                        );

                        Error::http(
                            StatusCode::TOO_MANY_REQUESTS,
                            "Upstream Anthropic rate limit exceeded.".to_string(),
                        )
                    }
                    Some(anthropic::ApiErrorCode::InvalidRequestError) => {
                        Error::http(StatusCode::BAD_REQUEST, api_error.message.clone())
                    }
                    Some(anthropic::ApiErrorCode::OverloadedError) => {
                        Error::http(StatusCode::SERVICE_UNAVAILABLE, api_error.message.clone())
                    }
                    Some(_) => {
                        Error::http(StatusCode::INTERNAL_SERVER_ERROR, api_error.message.clone())
                    }
                    None => Error::Internal(anyhow!(err)),
                },
                anthropic::AnthropicError::Other(err) => Error::Internal(err),
            })?;

            if let Some(rate_limit_info) = rate_limit_info {
                tracing::info!(
                    target: "upstream rate limit",
                    is_staff = claims.is_staff,
                    provider = params.provider.to_string(),
                    model = model,
                    tokens_remaining = rate_limit_info.tokens_remaining,
                    requests_remaining = rate_limit_info.requests_remaining,
                    requests_reset = ?rate_limit_info.requests_reset,
                    tokens_reset = ?rate_limit_info.tokens_reset,
                );
            }

            chunks
                .map(move |event| {
                    let chunk = event?;
                    let (
                        input_tokens,
                        output_tokens,
                        cache_creation_input_tokens,
                        cache_read_input_tokens,
                    ) = match &chunk {
                        anthropic::Event::MessageStart {
                            message: anthropic::Response { usage, .. },
                        }
                        | anthropic::Event::MessageDelta { usage, .. } => (
                            usage.input_tokens.unwrap_or(0) as usize,
                            usage.output_tokens.unwrap_or(0) as usize,
                            usage.cache_creation_input_tokens.unwrap_or(0) as usize,
                            usage.cache_read_input_tokens.unwrap_or(0) as usize,
                        ),
                        _ => (0, 0, 0, 0),
                    };

                    anyhow::Ok(CompletionChunk {
                        bytes: serde_json::to_vec(&chunk).unwrap(),
                        input_tokens,
                        output_tokens,
                        cache_creation_input_tokens,
                        cache_read_input_tokens,
                    })
                })
                .boxed()
        }
        LanguageModelProvider::OpenAi => {
            let api_key = state
                .config
                .openai_api_key
                .as_ref()
                .context("no OpenAI API key configured on the server")?;
            let chunks = open_ai::stream_completion(
                &state.http_client,
                open_ai::OPEN_AI_API_URL,
                api_key,
                serde_json::from_str(params.provider_request.get())?,
            )
            .await?;

            chunks
                .map(|event| {
                    event.map(|chunk| {
                        let input_tokens =
                            chunk.usage.as_ref().map_or(0, |u| u.prompt_tokens) as usize;
                        let output_tokens =
                            chunk.usage.as_ref().map_or(0, |u| u.completion_tokens) as usize;
                        CompletionChunk {
                            bytes: serde_json::to_vec(&chunk).unwrap(),
                            input_tokens,
                            output_tokens,
                            cache_creation_input_tokens: 0,
                            cache_read_input_tokens: 0,
                        }
                    })
                })
                .boxed()
        }
        LanguageModelProvider::Google => {
            let api_key = state
                .config
                .google_ai_api_key
                .as_ref()
                .context("no Google AI API key configured on the server")?;
            let chunks = google_ai::stream_generate_content(
                &state.http_client,
                google_ai::API_URL,
                api_key,
                serde_json::from_str(params.provider_request.get())?,
            )
            .await?;

            chunks
                .map(|event| {
                    event.map(|chunk| {
                        // TODO - implement token counting for Google AI
                        CompletionChunk {
                            bytes: serde_json::to_vec(&chunk).unwrap(),
                            input_tokens: 0,
                            output_tokens: 0,
                            cache_creation_input_tokens: 0,
                            cache_read_input_tokens: 0,
                        }
                    })
                })
                .boxed()
        }
    };

    Ok(Response::new(Body::wrap_stream(TokenCountingStream {
        state,
        claims,
        provider: params.provider,
        model,
        tokens: TokenUsage::default(),
        inner_stream: stream,
    })))
}

fn normalize_model_name(known_models: Vec<String>, name: String) -> String {
    if let Some(known_model_name) = known_models
        .iter()
        .filter(|known_model_name| name.starts_with(known_model_name.as_str()))
        .max_by_key(|known_model_name| known_model_name.len())
    {
        known_model_name.to_string()
    } else {
        name
    }
}

/// The maximum monthly spending an individual user can reach on the free tier
/// before they have to pay.
pub const FREE_TIER_MONTHLY_SPENDING_LIMIT: Cents = Cents::from_dollars(10);

/// The default value to use for maximum spend per month if the user did not
/// explicitly set a maximum spend.
///
/// Used to prevent surprise bills.
pub const DEFAULT_MAX_MONTHLY_SPEND: Cents = Cents::from_dollars(10);

async fn check_usage_limit(
    state: &Arc<LlmState>,
    provider: LanguageModelProvider,
    model_name: &str,
    claims: &LlmTokenClaims,
) -> Result<()> {
    if claims.is_staff {
        return Ok(());
    }

    let user_id = UserId::from_proto(claims.user_id);
    let model = state.db.model(provider, model_name)?;
    let free_tier = claims.free_tier_monthly_spending_limit();

    let spending_this_month = state
        .db
        .get_user_spending_for_month(user_id, Utc::now())
        .await?;
    if spending_this_month >= free_tier {
        if !claims.has_llm_subscription {
            return Err(Error::http(
                StatusCode::PAYMENT_REQUIRED,
                "Maximum spending limit reached for this month.".to_string(),
            ));
        }

        let monthly_spend = spending_this_month.saturating_sub(free_tier);
        if monthly_spend >= Cents(claims.max_monthly_spend_in_cents) {
            return Err(Error::Http(
                StatusCode::FORBIDDEN,
                "Maximum spending limit reached for this month.".to_string(),
                [(
                    HeaderName::from_static(MAX_LLM_MONTHLY_SPEND_REACHED_HEADER_NAME),
                    HeaderValue::from_static("true"),
                )]
                .into_iter()
                .collect(),
            ));
        }
    }

    let active_users = state.get_active_user_count(provider, model_name).await?;

    let users_in_recent_minutes = active_users.users_in_recent_minutes.max(1);
    let users_in_recent_days = active_users.users_in_recent_days.max(1);

    let per_user_max_requests_per_minute =
        model.max_requests_per_minute as usize / users_in_recent_minutes;
    let per_user_max_tokens_per_minute =
        model.max_tokens_per_minute as usize / users_in_recent_minutes;
    let per_user_max_tokens_per_day = model.max_tokens_per_day as usize / users_in_recent_days;

    let usage = state
        .db
        .get_usage(user_id, provider, model_name, Utc::now())
        .await?;

    let checks = [
        (
            usage.requests_this_minute,
            per_user_max_requests_per_minute,
            UsageMeasure::RequestsPerMinute,
        ),
        (
            usage.tokens_this_minute,
            per_user_max_tokens_per_minute,
            UsageMeasure::TokensPerMinute,
        ),
        (
            usage.tokens_this_day,
            per_user_max_tokens_per_day,
            UsageMeasure::TokensPerDay,
        ),
    ];

    for (used, limit, usage_measure) in checks {
        if used > limit {
            let resource = match usage_measure {
                UsageMeasure::RequestsPerMinute => "requests_per_minute",
                UsageMeasure::TokensPerMinute => "tokens_per_minute",
                UsageMeasure::TokensPerDay => "tokens_per_day",
            };

            tracing::info!(
                target: "user rate limit",
                user_id = claims.user_id,
                login = claims.github_user_login,
                authn.jti = claims.jti,
                is_staff = claims.is_staff,
                provider = provider.to_string(),
                model = model.name,
                requests_this_minute = usage.requests_this_minute,
                tokens_this_minute = usage.tokens_this_minute,
                tokens_this_day = usage.tokens_this_day,
                users_in_recent_minutes = users_in_recent_minutes,
                users_in_recent_days = users_in_recent_days,
                max_requests_per_minute = per_user_max_requests_per_minute,
                max_tokens_per_minute = per_user_max_tokens_per_minute,
                max_tokens_per_day = per_user_max_tokens_per_day,
            );

            SnowflakeRow::new(
                "Language Model Rate Limited",
                claims.metrics_id,
                claims.is_staff,
                claims.system_id.clone(),
                json!({
                    "usage": usage,
                    "users_in_recent_minutes": users_in_recent_minutes,
                    "users_in_recent_days": users_in_recent_days,
                    "max_requests_per_minute": per_user_max_requests_per_minute,
                    "max_tokens_per_minute": per_user_max_tokens_per_minute,
                    "max_tokens_per_day": per_user_max_tokens_per_day,
                    "plan": match claims.plan {
                        Plan::Free => "free".to_string(),
                        Plan::ZedPro => "zed_pro".to_string(),
                    },
                    "model": model.name.clone(),
                    "provider": provider.to_string(),
                    "usage_measure": resource.to_string(),
                }),
            )
            .write(&state.kinesis_client, &state.config.kinesis_stream)
            .await
            .log_err();

            return Err(Error::http(
                StatusCode::TOO_MANY_REQUESTS,
                format!("Rate limit exceeded. Maximum {} reached.", resource),
            ));
        }
    }

    Ok(())
}

struct CompletionChunk {
    bytes: Vec<u8>,
    input_tokens: usize,
    output_tokens: usize,
    cache_creation_input_tokens: usize,
    cache_read_input_tokens: usize,
}

struct TokenCountingStream<S> {
    state: Arc<LlmState>,
    claims: LlmTokenClaims,
    provider: LanguageModelProvider,
    model: String,
    tokens: TokenUsage,
    inner_stream: S,
}

impl<S> Stream for TokenCountingStream<S>
where
    S: Stream<Item = Result<CompletionChunk, anyhow::Error>> + Unpin,
{
    type Item = Result<Vec<u8>, anyhow::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner_stream).poll_next(cx) {
            Poll::Ready(Some(Ok(mut chunk))) => {
                chunk.bytes.push(b'\n');
                self.tokens.input += chunk.input_tokens;
                self.tokens.output += chunk.output_tokens;
                self.tokens.input_cache_creation += chunk.cache_creation_input_tokens;
                self.tokens.input_cache_read += chunk.cache_read_input_tokens;
                Poll::Ready(Some(Ok(chunk.bytes)))
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<S> Drop for TokenCountingStream<S> {
    fn drop(&mut self) {
        let state = self.state.clone();
        let claims = self.claims.clone();
        let provider = self.provider;
        let model = std::mem::take(&mut self.model);
        let tokens = self.tokens;
        self.state.executor.spawn_detached(async move {
            let usage = state
                .db
                .record_usage(
                    UserId::from_proto(claims.user_id),
                    claims.is_staff,
                    provider,
                    &model,
                    tokens,
                    claims.has_llm_subscription,
                    Cents(claims.max_monthly_spend_in_cents),
                    claims.free_tier_monthly_spending_limit(),
                    Utc::now(),
                )
                .await
                .log_err();

            if let Some(usage) = usage {
                tracing::info!(
                    target: "user usage",
                    user_id = claims.user_id,
                    login = claims.github_user_login,
                    authn.jti = claims.jti,
                    is_staff = claims.is_staff,
                    requests_this_minute = usage.requests_this_minute,
                    tokens_this_minute = usage.tokens_this_minute,
                );

                let properties = json!({
                    "has_llm_subscription": claims.has_llm_subscription,
                    "max_monthly_spend_in_cents": claims.max_monthly_spend_in_cents,
                    "plan": match claims.plan {
                        Plan::Free => "free".to_string(),
                        Plan::ZedPro => "zed_pro".to_string(),
                    },
                    "model": model,
                    "provider": provider,
                    "usage": usage,
                    "tokens": tokens
                });
                SnowflakeRow::new(
                    "Language Model Used",
                    claims.metrics_id,
                    claims.is_staff,
                    claims.system_id.clone(),
                    properties,
                )
                .write(&state.kinesis_client, &state.config.kinesis_stream)
                .await
                .log_err();
            }
        })
    }
}

pub fn log_usage_periodically(state: Arc<LlmState>) {
    state.executor.clone().spawn_detached(async move {
        loop {
            state
                .executor
                .sleep(std::time::Duration::from_secs(30))
                .await;

            for provider in LanguageModelProvider::iter() {
                for model in state.db.model_names_for_provider(provider) {
                    if let Some(active_user_count) = state
                        .get_active_user_count(provider, &model)
                        .await
                        .log_err()
                    {
                        tracing::info!(
                            target: "active user counts",
                            provider = provider.to_string(),
                            model = model,
                            users_in_recent_minutes = active_user_count.users_in_recent_minutes,
                            users_in_recent_days = active_user_count.users_in_recent_days,
                        );
                    }
                }
            }

            if let Some(usages) = state
                .db
                .get_application_wide_usages_by_model(Utc::now())
                .await
                .log_err()
            {
                for usage in usages {
                    tracing::info!(
                        target: "computed usage",
                        provider = usage.provider.to_string(),
                        model = usage.model,
                        requests_this_minute = usage.requests_this_minute,
                        tokens_this_minute = usage.tokens_this_minute,
                    );
                }
            }
        }
    })
}
