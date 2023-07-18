use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::AsyncReadExt;
use gpui::executor::Background;
use gpui::serde_json;
use isahc::http::StatusCode;
use isahc::prelude::Configurable;
use isahc::{AsyncBody, Response};
use lazy_static::lazy_static;
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
    async fn embed_batch(&self, spans: Vec<&str>) -> Result<Vec<Vec<f32>>>;
}

pub struct DummyEmbeddings {}

#[async_trait]
impl EmbeddingProvider for DummyEmbeddings {
    async fn embed_batch(&self, spans: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        // 1024 is the OpenAI Embeddings size for ada models.
        // the model we will likely be starting with.
        let dummy_vec = vec![0.32 as f32; 1536];
        return Ok(vec![dummy_vec; spans.len()]);
    }
}

const OPENAI_INPUT_LIMIT: usize = 8190;

impl OpenAIEmbeddings {
    pub fn new(client: Arc<dyn HttpClient>, executor: Arc<Background>) -> Self {
        Self { client, executor }
    }

    fn truncate(span: String) -> String {
        let mut tokens = OPENAI_BPE_TOKENIZER.encode_with_special_tokens(span.as_ref());
        if tokens.len() > OPENAI_INPUT_LIMIT {
            tokens.truncate(OPENAI_INPUT_LIMIT);
            let result = OPENAI_BPE_TOKENIZER.decode(tokens.clone());
            if result.is_ok() {
                let transformed = result.unwrap();
                // assert_ne!(transformed, span);
                return transformed;
            }
        }

        span
    }

    async fn send_request(&self, api_key: &str, spans: Vec<&str>) -> Result<Response<AsyncBody>> {
        let request = Request::post("https://api.openai.com/v1/embeddings")
            .redirect_policy(isahc::config::RedirectPolicy::Follow)
            .timeout(Duration::from_secs(4))
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
    async fn embed_batch(&self, spans: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        const BACKOFF_SECONDS: [usize; 3] = [65, 180, 360];
        const MAX_RETRIES: usize = 3;

        let api_key = OPENAI_API_KEY
            .as_ref()
            .ok_or_else(|| anyhow!("no api key"))?;

        let mut request_number = 0;
        let mut truncated = false;
        let mut response: Response<AsyncBody>;
        let mut spans: Vec<String> = spans.iter().map(|x| x.to_string()).collect();
        while request_number < MAX_RETRIES {
            response = self
                .send_request(api_key, spans.iter().map(|x| &**x).collect())
                .await?;
            request_number += 1;

            if request_number + 1 == MAX_RETRIES && response.status() != StatusCode::OK {
                return Err(anyhow!(
                    "openai max retries, error: {:?}",
                    &response.status()
                ));
            }

            match response.status() {
                StatusCode::TOO_MANY_REQUESTS => {
                    let delay = Duration::from_secs(BACKOFF_SECONDS[request_number - 1] as u64);
                    self.executor.timer(delay).await;
                }
                StatusCode::BAD_REQUEST => {
                    // Only truncate if it hasnt been truncated before
                    if !truncated {
                        for span in spans.iter_mut() {
                            *span = Self::truncate(span.clone());
                        }
                        truncated = true;
                    } else {
                        // If failing once already truncated, log the error and break the loop
                        let mut body = String::new();
                        response.body_mut().read_to_string(&mut body).await?;
                        log::trace!("open ai bad request: {:?} {:?}", &response.status(), body);
                        break;
                    }
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
                _ => {
                    return Err(anyhow!("openai embedding failed {}", response.status()));
                }
            }
        }

        Err(anyhow!("openai embedding failed"))
    }
}
