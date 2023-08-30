use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::AsyncReadExt;
use gpui::executor::Background;
use gpui::serde_json;
use isahc::http::StatusCode;
use isahc::prelude::Configurable;
use isahc::{AsyncBody, Response};
use lazy_static::lazy_static;
use parse_duration::parse;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tiktoken_rs::{cl100k_base, CoreBPE};
use util::http::{HttpClient, Request};

lazy_static! {
    static ref OPENAI_API_KEY: Option<String> = env::var("OPENAI_API_KEY").ok();
    static ref OPENAI_BPE_TOKENIZER: CoreBPE = cl100k_base().unwrap();
}

#[derive(Clone)]
pub struct OpenAIEmbeddings {
    pub client: Arc<dyn HttpClient>,
    pub executor: Arc<Background>,
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

#[async_trait]
pub trait EmbeddingProvider: Sync + Send {
    async fn embed_batch(&self, spans: Vec<String>) -> Result<Vec<Vec<f32>>>;
    fn max_tokens_per_batch(&self) -> usize;
    fn truncate(&self, span: &str) -> (String, usize);
}

pub struct DummyEmbeddings {}

#[async_trait]
impl EmbeddingProvider for DummyEmbeddings {
    async fn embed_batch(&self, spans: Vec<String>) -> Result<Vec<Vec<f32>>> {
        // 1024 is the OpenAI Embeddings size for ada models.
        // the model we will likely be starting with.
        let dummy_vec = vec![0.32 as f32; 1536];
        return Ok(vec![dummy_vec; spans.len()]);
    }

    fn max_tokens_per_batch(&self) -> usize {
        OPENAI_INPUT_LIMIT
    }

    fn truncate(&self, span: &str) -> (String, usize) {
        let mut tokens = OPENAI_BPE_TOKENIZER.encode_with_special_tokens(span);
        let token_count = tokens.len();
        let output = if token_count > OPENAI_INPUT_LIMIT {
            tokens.truncate(OPENAI_INPUT_LIMIT);
            OPENAI_BPE_TOKENIZER
                .decode(tokens)
                .ok()
                .unwrap_or_else(|| span.to_string())
        } else {
            span.to_string()
        };

        (output, token_count)
    }
}

const OPENAI_INPUT_LIMIT: usize = 8190;

impl OpenAIEmbeddings {
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
impl EmbeddingProvider for OpenAIEmbeddings {
    fn max_tokens_per_batch(&self) -> usize {
        OPENAI_INPUT_LIMIT
    }

    fn truncate(&self, span: &str) -> (String, usize) {
        let mut tokens = OPENAI_BPE_TOKENIZER.encode_with_special_tokens(span);
        let token_count = tokens.len();
        let output = if token_count > OPENAI_INPUT_LIMIT {
            tokens.truncate(OPENAI_INPUT_LIMIT);
            OPENAI_BPE_TOKENIZER
                .decode(tokens)
                .ok()
                .unwrap_or_else(|| span.to_string())
        } else {
            span.to_string()
        };

        (output, token_count)
    }

    async fn embed_batch(&self, spans: Vec<String>) -> Result<Vec<Vec<f32>>> {
        const BACKOFF_SECONDS: [usize; 4] = [3, 5, 15, 45];
        const MAX_RETRIES: usize = 4;

        let api_key = OPENAI_API_KEY
            .as_ref()
            .ok_or_else(|| anyhow!("no api key"))?;

        let mut request_number = 0;
        let mut request_timeout: u64 = 10;
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

                    return Ok(response
                        .data
                        .into_iter()
                        .map(|embedding| embedding.embedding)
                        .collect());
                }
                StatusCode::TOO_MANY_REQUESTS => {
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
