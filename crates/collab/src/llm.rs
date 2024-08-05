use crate::{executor::Executor, Config, Result};
use anyhow::Context as _;
use axum::{
    body::Body,
    response::{IntoResponse, Response},
    routing::post,
    Extension, Json, Router,
};
use futures::StreamExt as _;
use http_client::IsahcHttpClient;
use rpc::PerformCompletionParams;
use std::sync::Arc;

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
    Router::new().route("/completion", post(perform_completion))
}

async fn perform_completion(
    Extension(state): Extension<Arc<LlmState>>,
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
