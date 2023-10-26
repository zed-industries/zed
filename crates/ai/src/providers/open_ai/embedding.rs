use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::AsyncReadExt;
use gpui::executor::Background;
use gpui::serde_json;
use isahc::http::StatusCode;
use isahc::prelude::Configurable;
use isahc::{AsyncBody, Response};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use parse_duration::parse;
use postage::watch;
use serde::{Deserialize, Serialize};
use std::env;
use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tiktoken_rs::{cl100k_base, CoreBPE};
use util::http::{HttpClient, Request};

use crate::auth::{CredentialProvider, ProviderCredential};
use crate::embedding::{Embedding, EmbeddingProvider};
use crate::models::LanguageModel;
use crate::providers::open_ai::OpenAILanguageModel;

use crate::providers::open_ai::auth::OpenAICredentialProvider;

lazy_static! {
    static ref OPENAI_API_KEY: Option<String> = env::var("OPENAI_API_KEY").ok();
    static ref OPENAI_BPE_TOKENIZER: CoreBPE = cl100k_base().unwrap();
}

#[derive(Clone)]
pub struct OpenAIEmbeddingProvider {
    model: OpenAILanguageModel,
    credential_provider: OpenAICredentialProvider,
    pub client: Arc<dyn HttpClient>,
    pub executor: Arc<Background>,
    rate_limit_count_rx: watch::Receiver<Option<Instant>>,
    rate_limit_count_tx: Arc<Mutex<watch::Sender<Option<Instant>>>>,
}

#[derive(Serialize)]
struct OpenAIEmbeddingRequest<'a> {
    model: &'static str,
    input: Vec<&'a str>,
}

#[derive(Deserialize)]
struct OpenAIEmbeddingResponse {
    data: Vec<OpenAIEmbedding>,
    usage: OpenAIEmbeddingUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbedding {
    embedding: Vec<f32>,
    index: usize,
    object: String,
}

#[derive(Deserialize)]
struct OpenAIEmbeddingUsage {
    prompt_tokens: usize,
    total_tokens: usize,
}

impl OpenAIEmbeddingProvider {
    pub fn new(client: Arc<dyn HttpClient>, executor: Arc<Background>) -> Self {
        let (rate_limit_count_tx, rate_limit_count_rx) = watch::channel_with(None);
        let rate_limit_count_tx = Arc::new(Mutex::new(rate_limit_count_tx));

        let model = OpenAILanguageModel::load("text-embedding-ada-002");

        OpenAIEmbeddingProvider {
            model,
            credential_provider: OpenAICredentialProvider {},
            client,
            executor,
            rate_limit_count_rx,
            rate_limit_count_tx,
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
        api_key: &str,
        spans: Vec<&str>,
        request_timeout: u64,
    ) -> Result<Response<AsyncBody>> {
        let request = Request::post("https://api.openai.com/v1/embeddings")
            .redirect_policy(isahc::config::RedirectPolicy::Follow)
            .timeout(Duration::from_secs(request_timeout))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .body(
                serde_json::to_string(&OpenAIEmbeddingRequest {
                    input: spans.clone(),
                    model: "text-embedding-ada-002",
                })
                .unwrap()
                .into(),
            )?;

        Ok(self.client.send(request).await?)
    }
}

#[async_trait]
impl EmbeddingProvider for OpenAIEmbeddingProvider {
    fn base_model(&self) -> Box<dyn LanguageModel> {
        let model: Box<dyn LanguageModel> = Box::new(self.model.clone());
        model
    }

    fn credential_provider(&self) -> Box<dyn CredentialProvider> {
        let credential_provider: Box<dyn CredentialProvider> =
            Box::new(self.credential_provider.clone());
        credential_provider
    }

    fn max_tokens_per_batch(&self) -> usize {
        50000
    }

    fn rate_limit_expiration(&self) -> Option<Instant> {
        *self.rate_limit_count_rx.borrow()
    }

    async fn embed_batch(
        &self,
        spans: Vec<String>,
        _credential: ProviderCredential,
    ) -> Result<Vec<Embedding>> {
        const BACKOFF_SECONDS: [usize; 4] = [3, 5, 15, 45];
        const MAX_RETRIES: usize = 4;

        let api_key = OPENAI_API_KEY
            .as_ref()
            .ok_or_else(|| anyhow!("no api key"))?;

        let mut request_number = 0;
        let mut rate_limiting = false;
        let mut request_timeout: u64 = 15;
        let mut response: Response<AsyncBody>;
        while request_number < MAX_RETRIES {
            response = self
                .send_request(
                    api_key,
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
                    let response: OpenAIEmbeddingResponse = serde_json::from_str(&body)?;

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
