use anyhow::Result;
use client::{Client, EditPredictionUsage};
use cloud_llm_client::predict_edits_v3::{
    PredictEditsV3Request, PredictEditsV3Response, RawCompletionRequest, RawCompletionResponse,
};
use cloud_llm_client::{EXPIRED_LLM_TOKEN_HEADER_NAME, MINIMUM_REQUIRED_VERSION_HEADER_NAME};
use futures::AsyncReadExt as _;
use gpui::http_client::{self, AsyncBody, Method, Url};
use language_model::LlmApiToken;
use semver::Version;
use serde::de::DeserializeOwned;
use std::env;
use std::sync::{Arc, LazyLock};
use thiserror::Error;
use zeta_prompt::{ZetaPromptInput, ZetaVersion};

pub(crate) static RAW_MODEL_ID: LazyLock<Option<String>> =
    LazyLock::new(|| env::var("ZED_ZETA_MODEL").ok());

pub const ZED_VERSION_HEADER_NAME: &str = cloud_llm_client::ZED_VERSION_HEADER_NAME;

#[derive(Error, Debug)]
#[error(
    "You must update to Zed version {minimum_version} or higher to continue using edit predictions."
)]
pub struct ZedUpdateRequiredError {
    pub minimum_version: Version,
}

pub async fn send_raw_llm_request(
    request: RawCompletionRequest,
    client: Arc<Client>,
    custom_url: Option<Arc<Url>>,
    llm_token: LlmApiToken,
    app_version: Version,
) -> Result<(RawCompletionResponse, Option<EditPredictionUsage>)> {
    let url = if let Some(custom_url) = custom_url {
        custom_url.as_ref().clone()
    } else {
        client
            .http_client()
            .build_zed_llm_url("/predict_edits/raw", &[])?
    };

    send_api_request(
        |builder| {
            let req = builder
                .uri(url.as_ref())
                .body(serde_json::to_string(&request)?.into());
            Ok(req?)
        },
        client,
        llm_token,
        app_version,
        true,
    )
    .await
}

pub async fn send_v3_request(
    input: ZetaPromptInput,
    prompt_version: ZetaVersion,
    client: Arc<Client>,
    llm_token: LlmApiToken,
    app_version: Version,
) -> Result<(PredictEditsV3Response, Option<EditPredictionUsage>)> {
    let url = client
        .http_client()
        .build_zed_llm_url("/predict_edits/v3", &[])?;

    let request = PredictEditsV3Request {
        input,
        model: RAW_MODEL_ID.clone(),
        prompt_version,
    };

    send_api_request(
        |builder| {
            let req = builder
                .uri(url.as_ref())
                .body(serde_json::to_string(&request)?.into());
            Ok(req?)
        },
        client,
        llm_token,
        app_version,
        true,
    )
    .await
}

pub async fn send_api_request<Res>(
    build: impl Fn(http_client::http::request::Builder) -> Result<http_client::Request<AsyncBody>>,
    client: Arc<Client>,
    llm_token: LlmApiToken,
    app_version: Version,
    require_auth: bool,
) -> Result<(Res, Option<EditPredictionUsage>)>
where
    Res: DeserializeOwned,
{
    let http_client = client.http_client();

    let mut token = if let Ok(custom_token) = std::env::var("ZED_PREDICT_EDITS_TOKEN") {
        Some(custom_token)
    } else if require_auth {
        Some(llm_token.acquire(&client).await?)
    } else {
        llm_token.acquire(&client).await.ok()
    };
    let mut did_retry = false;

    loop {
        let request_builder = http_client::Request::builder().method(Method::POST);

        let mut request_builder = request_builder
            .header("Content-Type", "application/json")
            .header(ZED_VERSION_HEADER_NAME, app_version.to_string());

        // Only add Authorization header if we have a token
        if let Some(ref token_value) = token {
            request_builder =
                request_builder.header("Authorization", format!("Bearer {}", token_value));
        }

        let request = build(request_builder)?;

        let mut response = http_client.send(request).await?;

        if let Some(minimum_required_version) = response
            .headers()
            .get(MINIMUM_REQUIRED_VERSION_HEADER_NAME)
            .and_then(|version| Version::parse(version.to_str().ok()?).ok())
        {
            anyhow::ensure!(
                app_version >= minimum_required_version,
                ZedUpdateRequiredError {
                    minimum_version: minimum_required_version
                }
            );
        }

        if response.status().is_success() {
            let usage = EditPredictionUsage::from_headers(response.headers()).ok();

            let mut body = Vec::new();
            response.body_mut().read_to_end(&mut body).await?;
            return Ok((serde_json::from_slice(&body)?, usage));
        } else if !did_retry
            && token.is_some()
            && response
                .headers()
                .get(EXPIRED_LLM_TOKEN_HEADER_NAME)
                .is_some()
        {
            did_retry = true;
            token = Some(llm_token.refresh(&client).await?);
        } else {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            anyhow::bail!(
                "Request failed with status: {:?}\nBody: {}",
                response.status(),
                body
            );
        }
    }
}
