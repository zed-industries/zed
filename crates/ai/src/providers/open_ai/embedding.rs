use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::future::BoxFuture;
use futures::AsyncReadExt;
use futures::FutureExt;
use gpui::AppContext;
use gpui::BackgroundExecutor;
use isahc::http::StatusCode;
use isahc::prelude::Configurable;
use isahc::{AsyncBody, Response};
use parking_lot::{Mutex, RwLock};
use parse_duration::parse;
use postage::watch;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::ops::Add;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use tiktoken_rs::{cl100k_base, CoreBPE};
use util::http::{HttpClient, Request};
use util::ResultExt;

use crate::auth::{CredentialProvider, ProviderCredential};
use crate::embedding::{Embedding, EmbeddingProvider};
use crate::models::LanguageModel;
use crate::providers::open_ai::OpenAiLanguageModel;

use crate::providers::open_ai::OPEN_AI_API_URL;

pub(crate) fn open_ai_bpe_tokenizer() -> &'static CoreBPE {
    static OPEN_AI_BPE_TOKENIZER: OnceLock<CoreBPE> = OnceLock::new();
    OPEN_AI_BPE_TOKENIZER.get_or_init(|| cl100k_base().unwrap())
}

#[derive(Clone)]
pub struct OpenAiEmbeddingProvider {
    api_url: String,
    model: OpenAiLanguageModel,
    credential: Arc<RwLock<ProviderCredential>>,
    pub client: Arc<dyn HttpClient>,
    pub executor: BackgroundExecutor,
    rate_limit_count_rx: watch::Receiver<Option<Instant>>,
    rate_limit_count_tx: Arc<Mutex<watch::Sender<Option<Instant>>>>,
}

#[derive(Serialize)]
struct OpenAiEmbeddingRequest<'a> {
    model: &'static str,
    input: Vec<&'a str>,
}

#[derive(Deserialize)]
struct OpenAiEmbeddingResponse {
    data: Vec<OpenAiEmbedding>,
    usage: OpenAiEmbeddingUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAiEmbedding {
    embedding: Vec<f32>,
    index: usize,
    object: String,
}

#[derive(Deserialize)]
struct OpenAiEmbeddingUsage {
    prompt_tokens: usize,
    total_tokens: usize,
}

impl OpenAiEmbeddingProvider {
    pub async fn new(
        api_url: String,
        client: Arc<dyn HttpClient>,
        executor: BackgroundExecutor,
    ) -> Self {
        let (rate_limit_count_tx, rate_limit_count_rx) = watch::channel_with(None);
        let rate_limit_count_tx = Arc::new(Mutex::new(rate_limit_count_tx));

        // Loading the model is expensive, so ensure this runs off the main thread.
        let model = executor
            .spawn(async move { OpenAiLanguageModel::load("text-embedding-ada-002") })
            .await;
        let credential = Arc::new(RwLock::new(ProviderCredential::NoCredentials));

        OpenAiEmbeddingProvider {
            api_url,
            model,
            credential,
            client,
            executor,
            rate_limit_count_rx,
            rate_limit_count_tx,
        }
    }

    fn get_api_key(&self) -> Result<String> {
        match self.credential.read().clone() {
            ProviderCredential::Credentials { api_key } => Ok(api_key),
            _ => Err(anyhow!("api credentials not provided")),
        }
    }

    fn resolve_rate_limit(&self) {
        let reset_time = *self.rate_limit_count_tx.lock().borrow();

        if let Some(reset_time) = reset_time {
            if Instant::now() >= reset_time {
                *self.rate_limit_count_tx.lock().borrow_mut() = None
            }
        }

        log::trace!(
            "resolving reset time: {:?}",
            *self.rate_limit_count_tx.lock().borrow()
        );
    }

    fn update_reset_time(&self, reset_time: Instant) {
        let original_time = *self.rate_limit_count_tx.lock().borrow();

        let updated_time = if let Some(original_time) = original_time {
            if reset_time < original_time {
                Some(reset_time)
            } else {
                Some(original_time)
            }
        } else {
            Some(reset_time)
        };

        log::trace!("updating rate limit time: {:?}", updated_time);

        *self.rate_limit_count_tx.lock().borrow_mut() = updated_time;
    }
    async fn send_request(
        &self,
        api_url: &str,
        api_key: &str,
        spans: Vec<&str>,
        request_timeout: u64,
    ) -> Result<Response<AsyncBody>> {
        let request = Request::post(format!("{api_url}/embeddings"))
            .redirect_policy(isahc::config::RedirectPolicy::Follow)
            .timeout(Duration::from_secs(request_timeout))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .body(
                serde_json::to_string(&OpenAiEmbeddingRequest {
                    input: spans.clone(),
                    model: "text-embedding-ada-002",
                })
                .unwrap()
                .into(),
            )?;

        Ok(self.client.send(request).await?)
    }
}

impl CredentialProvider for OpenAiEmbeddingProvider {
    fn has_credentials(&self) -> bool {
        match *self.credential.read() {
            ProviderCredential::Credentials { .. } => true,
            _ => false,
        }
    }

    fn retrieve_credentials(&self, cx: &mut AppContext) -> BoxFuture<ProviderCredential> {
        let existing_credential = self.credential.read().clone();
        let retrieved_credential = match existing_credential {
            ProviderCredential::Credentials { .. } => {
                return async move { existing_credential }.boxed()
            }
            _ => {
                if let Some(api_key) = env::var("OPENAI_API_KEY").log_err() {
                    async move { ProviderCredential::Credentials { api_key } }.boxed()
                } else {
                    let credentials = cx.read_credentials(OPEN_AI_API_URL);
                    async move {
                        if let Some(Some((_, api_key))) = credentials.await.log_err() {
                            if let Some(api_key) = String::from_utf8(api_key).log_err() {
                                ProviderCredential::Credentials { api_key }
                            } else {
                                ProviderCredential::NoCredentials
                            }
                        } else {
                            ProviderCredential::NoCredentials
                        }
                    }
                    .boxed()
                }
            }
        };

        async move {
            let retrieved_credential = retrieved_credential.await;
            *self.credential.write() = retrieved_credential.clone();
            retrieved_credential
        }
        .boxed()
    }

    fn save_credentials(
        &self,
        cx: &mut AppContext,
        credential: ProviderCredential,
    ) -> BoxFuture<()> {
        *self.credential.write() = credential.clone();
        let credential = credential.clone();
        let write_credentials = match credential {
            ProviderCredential::Credentials { api_key } => {
                Some(cx.write_credentials(OPEN_AI_API_URL, "Bearer", api_key.as_bytes()))
            }
            _ => None,
        };

        async move {
            if let Some(write_credentials) = write_credentials {
                write_credentials.await.log_err();
            }
        }
        .boxed()
    }

    fn delete_credentials(&self, cx: &mut AppContext) -> BoxFuture<()> {
        *self.credential.write() = ProviderCredential::NoCredentials;
        let delete_credentials = cx.delete_credentials(OPEN_AI_API_URL);
        async move {
            delete_credentials.await.log_err();
        }
        .boxed()
    }
}

#[async_trait]
impl EmbeddingProvider for OpenAiEmbeddingProvider {
    fn base_model(&self) -> Box<dyn LanguageModel> {
        let model: Box<dyn LanguageModel> = Box::new(self.model.clone());
        model
    }

    fn max_tokens_per_batch(&self) -> usize {
        50000
    }

    fn rate_limit_expiration(&self) -> Option<Instant> {
        *self.rate_limit_count_rx.borrow()
    }

    async fn embed_batch(&self, spans: Vec<String>) -> Result<Vec<Embedding>> {
        const BACKOFF_SECONDS: [usize; 4] = [3, 5, 15, 45];
        const MAX_RETRIES: usize = 4;

        let api_url = self.api_url.as_str();
        let api_key = self.get_api_key()?;

        let mut request_number = 0;
        let mut rate_limiting = false;
        let mut request_timeout: u64 = 15;
        let mut response: Response<AsyncBody>;
        while request_number < MAX_RETRIES {
            response = self
                .send_request(
                    &api_url,
                    &api_key,
                    spans.iter().map(|x| &**x).collect(),
                    request_timeout,
                )
                .await?;

            request_number += 1;

            match response.status() {
                StatusCode::REQUEST_TIMEOUT => {
                    request_timeout += 5;
                }
                StatusCode::OK => {
                    let mut body = String::new();
                    response.body_mut().read_to_string(&mut body).await?;
                    let response: OpenAiEmbeddingResponse = serde_json::from_str(&body)?;

                    log::trace!(
                        "openai embedding completed. tokens: {:?}",
                        response.usage.total_tokens
                    );

                    // If we complete a request successfully that was previously rate_limited
                    // resolve the rate limit
                    if rate_limiting {
                        self.resolve_rate_limit()
                    }

                    return Ok(response
                        .data
                        .into_iter()
                        .map(|embedding| Embedding::from(embedding.embedding))
                        .collect());
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    rate_limiting = true;
                    let mut body = String::new();
                    response.body_mut().read_to_string(&mut body).await?;

                    let delay_duration = {
                        let delay = Duration::from_secs(BACKOFF_SECONDS[request_number - 1] as u64);
                        if let Some(time_to_reset) =
                            response.headers().get("x-ratelimit-reset-tokens")
                        {
                            if let Ok(time_str) = time_to_reset.to_str() {
                                parse(time_str).unwrap_or(delay)
                            } else {
                                delay
                            }
                        } else {
                            delay
                        }
                    };

                    // If we've previously rate limited, increment the duration but not the count
                    let reset_time = Instant::now().add(delay_duration);
                    self.update_reset_time(reset_time);

                    log::trace!(
                        "openai rate limiting: waiting {:?} until lifted",
                        &delay_duration
                    );

                    self.executor.timer(delay_duration).await;
                }
                _ => {
                    let mut body = String::new();
                    response.body_mut().read_to_string(&mut body).await?;
                    return Err(anyhow!(
                        "open ai bad request: {:?} {:?}",
                        &response.status(),
                        body
                    ));
                }
            }
        }
        Err(anyhow!("openai max retries"))
    }
}
