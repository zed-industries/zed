mod token;

use crate::{executor::Executor, Config, Error, Result};
use anyhow::Context as _;
use axum::{
    body::Body,
    http::{self, HeaderName, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Extension, Json, Router,
};
use futures::StreamExt as _;
use http_client::IsahcHttpClient;
use rpc::{PerformCompletionParams, EXPIRED_LLM_TOKEN_HEADER_NAME};
use std::sync::Arc;

pub use token::*;

pub struct LlmState {
    pub config: Config,
    pub executor: Executor,
    pub http_client: IsahcHttpClient,
}

impl LlmState {
    pub async fn new(config: Config, executor: Executor) -> Result<Arc<Self>> {
        let user_agent = format!("Zed Server/{}", env!("CARGO_PKG_VERSION"));
        let http_client = IsahcHttpClient::builder()
            .default_header("User-Agent", user_agent)
            .build()
            .context("failed to construct http client")?;

        let this = Self {
            config,
            executor,
            http_client,
        };

        Ok(Arc::new(this))
    }
}

pub fn routes() -> Router<(), Body> {
    Router::new()
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
    match LlmTokenClaims::validate(&token, &state.config) {
        Ok(claims) => {
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

async fn perform_completion(
    Extension(state): Extension<Arc<LlmState>>,
    Extension(_claims): Extension<LlmTokenClaims>,
    Json(params): Json<PerformCompletionParams>,
) -> Result<impl IntoResponse> {
    let api_key = state
        .config
        .anthropic_api_key
        .as_ref()
        .context("no Anthropic AI API key configured on the server")?;
    let chunks = anthropic::stream_completion(
        &state.http_client,
        anthropic::ANTHROPIC_API_URL,
        api_key,
        serde_json::from_str(&params.provider_request.get())?,
        None,
    )
    .await?;

    let stream = chunks.map(|event| {
        let mut buffer = Vec::new();
        event.map(|chunk| {
            buffer.clear();
            serde_json::to_writer(&mut buffer, &chunk).unwrap();
            buffer.push(b'\n');
            buffer
        })
    });

    Ok(Response::new(Body::wrap_stream(stream)))
}
