use anyhow::{anyhow, Context as _, Result};
use futures::{future::BoxFuture, AsyncReadExt, FutureExt};
use serde::{Deserialize, Serialize};
use std::{future, sync::Arc};
use util::http::HttpClient;

/// Ollama's embedding via nomic-embed-text is of length 768
pub const EMBEDDING_SIZE_TINY: usize = 768;
/// Ollama's embedding via mxbai-embed-large is of length 1024
pub const EMBEDDING_SIZE_XSMALL: usize = 1024;
/// OpenAI's text small and Voyage Large/Code are of length 1536
pub const EMBEDDING_SIZE_SMALL: usize = 1536;
/// OpenAI's text large embeddings are of length 3072
pub const EMBEDDING_SIZE_LARGE: usize = 3072;

#[derive(Clone, Copy)]
pub enum EmbeddingModel {
    OllamaNomicEmbedText,
    OllamaMxbaiEmbedLarge,
    OpenaiTextEmbedding3Small,
    OpenaiTextEmbedding3Large,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Embedding(Vec<f32>);

impl Embedding {
    fn new(mut embedding: Vec<f32>) -> Self {
        let len = embedding.len();
        let mut norm = 0f32;

        for i in 0..len {
            norm += embedding[i] * embedding[i];
        }

        norm = norm.sqrt();
        for dimension in &mut embedding {
            *dimension /= norm;
        }

        Self(embedding)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

/// Trait for embedding providers. Texts in, vectors out.
pub trait EmbeddingProvider {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>>;
    fn batch_size(&self) -> usize;
}

pub struct FakeEmbeddingProvider {}

impl FakeEmbeddingProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl EmbeddingProvider for FakeEmbeddingProvider {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>> {
        let embeddings = texts
            .iter()
            .map(|text| {
                let mut embedding = vec![0f32; 2];
                for i in 0..2 {
                    embedding[i] = i as f32;
                }
                Embedding::new(embedding)
            })
            .collect();
        future::ready(Ok(embeddings)).boxed()
    }

    fn batch_size(&self) -> usize {
        16
    }
}

pub struct OllamaEmbeddingProvider {
    client: Arc<dyn HttpClient>,
    model: EmbeddingModel,
}

#[derive(Serialize)]
struct OllamaEmbeddingRequest {
    model: String,
    prompt: String,
}

#[derive(Deserialize)]
struct OllamaEmbeddingResponse {
    embedding: Vec<f32>,
}

impl OllamaEmbeddingProvider {
    pub fn new(client: Arc<dyn HttpClient>, model: EmbeddingModel) -> Self {
        Self { client, model }
    }
}

impl EmbeddingProvider for OllamaEmbeddingProvider {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>> {
        //
        let model = match self.model {
            EmbeddingModel::OllamaNomicEmbedText => "nomic-embed-text".to_string(),
            EmbeddingModel::OllamaMxbaiEmbedLarge => "mxbai-embed-large".to_string(),
            _ => return future::ready(Err(anyhow!("Invalid model"))).boxed(),
        };

        futures::future::try_join_all(texts.into_iter().map(|text| {
            let request = OllamaEmbeddingRequest {
                model,
                prompt: text.to_string(),
            };

            let request = serde_json::to_string(&request).unwrap();

            async {
                let response = self
                    .client
                    .post_json("http://localhost:11434/api/embeddings", request.into())
                    .await?;

                let mut body = String::new();
                response.into_body().read_to_string(&mut body).await?;

                let response: OllamaEmbeddingResponse =
                    serde_json::from_str(&body).context("Unable to pull response")?;

                Ok(Embedding::new(response.embedding))
            }
        }))
        .boxed()
    }

    fn batch_size(&self) -> usize {
        // TODO: Figure out decent value
        10
    }
}

pub struct OpenaiEmbeddingProvider {
    client: Arc<dyn HttpClient>,
    model: EmbeddingModel,
    api_key: String,
}

#[derive(Serialize)]
struct OpenaiEmbeddingRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct OpenaiEmbeddingData {
    embedding: Vec<f32>,
}

#[derive(Deserialize)]
struct OpenaiEmbeddingResponse {
    object: String,
    data: Vec<OpenaiEmbeddingData>,
    model: String,
}

impl OpenaiEmbeddingProvider {
    pub fn new(client: Arc<dyn HttpClient>, model: EmbeddingModel, api_key: String) -> Self {
        Self {
            client,
            model,
            api_key,
        }
    }
}

impl EmbeddingProvider for OpenaiEmbeddingProvider {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>> {
        todo!();
        // let request = OpenaiEmbeddingRequest {
        //     model: match self.model {
        //         EmbeddingModel::OpenaiTextEmbedding3Small => "text-embedding-3-small".to_string(),
        //         EmbeddingModel::OpenaiTextEmbedding3Large => "text-embedding-3-large".to_string(),
        //         _ => return Err(anyhow!("Invalid model")),
        //     },
        //     input: texts,
        // };

        // let api_url = "https://api.openai.com/v1/";

        // let uri = format!("{api_url}/embeddings");

        // let request = HttpRequest::builder()
        //     .method(Method::POST)
        //     .uri(uri)
        //     .header("Content-Type", "application/json")
        //     .header("Authorization", format!("Bearer {}", self.api_key))
        //     .body(AsyncBody::from(serde_json::to_string(&request)?))?;

        // let mut response = self.client.send(request).await.context("Failed to embed")?;

        // let mut body = Vec::new();
        // response.body_mut().read_to_end(&mut body).await.ok();

        // let mut response: OpenaiEmbeddingResponse =
        //     serde_json::from_slice(body.as_slice()).context("Unable to pull response")?;

        // let data = response
        //     .data
        //     .pop()
        //     .context("No embedding data found in response")?;
        // Ok(Embedding::new(data.embedding))
    }

    fn batch_size(&self) -> usize {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::BackgroundExecutor;

    use util::http::HttpClientWithUrl;

    #[gpui::test]
    async fn test_ollama_embedding_provider(executor: BackgroundExecutor) {
        executor.allow_parking();

        let client = Arc::new(HttpClientWithUrl::new("http://localhost:11434/"));
        let provider =
            OllamaEmbeddingProvider::new(client.clone(), EmbeddingModel::OllamaNomicEmbedText);
        let embedding = provider.embed(&[&"Hello, world!"]).await.unwrap();
        assert_eq!(embedding.len(), EMBEDDING_SIZE_TINY);
    }

    #[gpui::test]
    async fn test_ollama_embedding_not_exactly_a_benchmark(executor: BackgroundExecutor) {
        executor.allow_parking();

        let client = Arc::new(HttpClientWithUrl::new("http://localhost:11434/"));
        let provider =
            OllamaEmbeddingProvider::new(client.clone(), EmbeddingModel::OllamaNomicEmbedText);

        let t_nomic = std::time::Instant::now();
        for i in 0..100 {
            let embedding = provider
                .embed(&[&format!("Hello, world! {}", i)])
                .await
                .unwrap();
            assert_eq!(embedding.len(), EMBEDDING_SIZE_TINY);
        }
        dbg!(t_nomic.elapsed());

        let client = Arc::new(HttpClientWithUrl::new("http://localhost:11434/"));
        let provider =
            OllamaEmbeddingProvider::new(client.clone(), EmbeddingModel::OllamaMxbaiEmbedLarge);

        let t_mxbai = std::time::Instant::now();

        for i in 0..100 {
            let embedding = provider
                .embed(&[&format!("Hello, world! {}", i)])
                .await
                .unwrap();
            assert_eq!(embedding.len(), EMBEDDING_SIZE_XSMALL);
        }
        dbg!(t_mxbai.elapsed());
    }

    #[gpui::test]
    async fn test_openai_embedding(executor: BackgroundExecutor) {
        executor.allow_parking();

        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

        let client = Arc::new(HttpClientWithUrl::new("http://localhost:11434/"));
        let provider = OpenaiEmbeddingProvider::new(
            client.clone(),
            EmbeddingModel::OpenaiTextEmbedding3Small,
            api_key,
        );

        let t_openai_small = std::time::Instant::now();
        for i in 0..100 {
            let embedding = provider
                .embed(&[&format!("Hello, world! {}", i)])
                .await
                .unwrap();
            assert_eq!(embedding.len(), EMBEDDING_SIZE_SMALL);
        }
        dbg!(t_openai_small.elapsed());
    }

    #[gpui::test]
    fn test_normalize_embedding() {
        let normalized = Embedding::new(vec![1.0, 1.0, 1.0]);
        let value: f32 = 1.0 / 3.0_f32.sqrt();
        assert_eq!(normalized, Embedding(vec![value; 3]));
    }
}
