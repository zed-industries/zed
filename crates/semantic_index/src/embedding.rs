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
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::ToSql;
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

#[derive(Debug, PartialEq, Clone)]
pub struct Embedding(Vec<f32>);

impl From<Vec<f32>> for Embedding {
    fn from(value: Vec<f32>) -> Self {
        Embedding(value)
    }
}

impl Embedding {
    pub fn similarity(&self, other: &Self) -> f32 {
        let len = self.0.len();
        assert_eq!(len, other.0.len());

        let mut result = 0.0;
        unsafe {
            matrixmultiply::sgemm(
                1,
                len,
                1,
                1.0,
                self.0.as_ptr(),
                len as isize,
                1,
                other.0.as_ptr(),
                1,
                len as isize,
                0.0,
                &mut result as *mut f32,
                1,
                1,
            );
        }
        result
    }
}

impl FromSql for Embedding {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        let bytes = value.as_blob()?;
        let embedding: Result<Vec<f32>, Box<bincode::ErrorKind>> = bincode::deserialize(bytes);
        if embedding.is_err() {
            return Err(rusqlite::types::FromSqlError::Other(embedding.unwrap_err()));
        }
        Ok(Embedding(embedding.unwrap()))
    }
}

impl ToSql for Embedding {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        let bytes = bincode::serialize(&self.0)
            .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Blob(bytes)))
    }
}

#[derive(Clone)]
pub struct OpenAIEmbeddings {
    pub client: Arc<dyn HttpClient>,
    pub executor: Arc<Background>,
    rate_limit_count_rx: watch::Receiver<(Duration, usize)>,
    rate_limit_count_tx: Arc<Mutex<watch::Sender<(Duration, usize)>>>,
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
    async fn embed_batch(&self, spans: Vec<String>) -> Result<Vec<Embedding>>;
    fn max_tokens_per_batch(&self) -> usize;
    fn truncate(&self, span: &str) -> (String, usize);
    fn rate_limit_expiration(&self) -> Duration;
}

pub struct DummyEmbeddings {}

#[async_trait]
impl EmbeddingProvider for DummyEmbeddings {
    fn rate_limit_expiration(&self) -> Duration {
        Duration::ZERO
    }
    async fn embed_batch(&self, spans: Vec<String>) -> Result<Vec<Embedding>> {
        // 1024 is the OpenAI Embeddings size for ada models.
        // the model we will likely be starting with.
        let dummy_vec = Embedding::from(vec![0.32 as f32; 1536]);
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
            let new_input = OPENAI_BPE_TOKENIZER.decode(tokens.clone());
            new_input.ok().unwrap_or_else(|| span.to_string())
        } else {
            span.to_string()
        };

        (output, tokens.len())
    }
}

const OPENAI_INPUT_LIMIT: usize = 8190;

impl OpenAIEmbeddings {
    pub fn new(client: Arc<dyn HttpClient>, executor: Arc<Background>) -> Self {
        let (rate_limit_count_tx, rate_limit_count_rx) = watch::channel_with((Duration::ZERO, 0));
        let rate_limit_count_tx = Arc::new(Mutex::new(rate_limit_count_tx));

        OpenAIEmbeddings {
            client,
            executor,
            rate_limit_count_rx,
            rate_limit_count_tx,
        }
    }

    fn resolve_rate_limit(&self) {
        let (current_delay, delay_count) = *self.rate_limit_count_tx.lock().borrow();
        let updated_count = delay_count - 1;
        let updated_duration = if updated_count == 0 {
            Duration::ZERO
        } else {
            current_delay
        };

        log::trace!(
            "resolving rate limit: Count: {:?} Duration: {:?}",
            updated_count,
            updated_duration
        );

        *self.rate_limit_count_tx.lock().borrow_mut() = (updated_duration, updated_count);
    }

    fn update_rate_limit(&self, delay_duration: Duration, count_increase: usize) {
        let (current_delay, delay_count) = *self.rate_limit_count_tx.lock().borrow();
        let updated_count = delay_count + count_increase;
        let updated_duration = if current_delay < delay_duration {
            delay_duration
        } else {
            current_delay
        };

        log::trace!(
            "updating rate limit: Count: {:?} Duration: {:?}",
            updated_count,
            updated_duration
        );

        *self.rate_limit_count_tx.lock().borrow_mut() = (updated_duration, updated_count);
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
impl EmbeddingProvider for OpenAIEmbeddings {
    fn max_tokens_per_batch(&self) -> usize {
        50000
    }

    fn rate_limit_expiration(&self) -> Duration {
        let (duration, _) = *self.rate_limit_count_rx.borrow();
        duration
    }
    fn truncate(&self, span: &str) -> (String, usize) {
        let mut tokens = OPENAI_BPE_TOKENIZER.encode_with_special_tokens(span);
        let output = if tokens.len() > OPENAI_INPUT_LIMIT {
            tokens.truncate(OPENAI_INPUT_LIMIT);
            OPENAI_BPE_TOKENIZER
                .decode(tokens.clone())
                .ok()
                .unwrap_or_else(|| span.to_string())
        } else {
            span.to_string()
        };

        (output, tokens.len())
    }

    async fn embed_batch(&self, spans: Vec<String>) -> Result<Vec<Embedding>> {
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
                    if rate_limiting {
                        self.update_rate_limit(delay_duration, 0);
                    } else {
                        self.update_rate_limit(delay_duration, 1);
                    }

                    rate_limiting = true;

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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[gpui::test]
    fn test_similarity(mut rng: StdRng) {
        assert_eq!(
            Embedding::from(vec![1., 0., 0., 0., 0.])
                .similarity(&Embedding::from(vec![0., 1., 0., 0., 0.])),
            0.
        );
        assert_eq!(
            Embedding::from(vec![2., 0., 0., 0., 0.])
                .similarity(&Embedding::from(vec![3., 1., 0., 0., 0.])),
            6.
        );

        for _ in 0..100 {
            let size = 1536;
            let mut a = vec![0.; size];
            let mut b = vec![0.; size];
            for (a, b) in a.iter_mut().zip(b.iter_mut()) {
                *a = rng.gen();
                *b = rng.gen();
            }
            let a = Embedding::from(a);
            let b = Embedding::from(b);

            assert_eq!(
                round_to_decimals(a.similarity(&b), 1),
                round_to_decimals(reference_dot(&a.0, &b.0), 1)
            );
        }

        fn round_to_decimals(n: f32, decimal_places: i32) -> f32 {
            let factor = (10.0 as f32).powi(decimal_places);
            (n * factor).round() / factor
        }

        fn reference_dot(a: &[f32], b: &[f32]) -> f32 {
            a.iter().zip(b.iter()).map(|(a, b)| a * b).sum()
        }
    }
}
