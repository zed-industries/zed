use anyhow::{Context as _, Result};
use futures::AsyncReadExt;
use serde::{Deserialize, Serialize};
use util::http::{HttpClient, HttpClientWithUrl};

pub const EMBEDDING_SIZE_TINY: usize = 768;
pub const EMBEDDING_SIZE_SMALL: usize = 1536;
pub const EMBEDDING_SIZE_LARGE: usize = 3072;

pub enum Embedding {
    Tiny([f32; EMBEDDING_SIZE_TINY]),
    Small([f32; EMBEDDING_SIZE_SMALL]),
    Large([f32; EMBEDDING_SIZE_LARGE]),
    None,
}

#[async_trait::async_trait]
pub trait EmbeddingProvider {
    async fn get_embedding<'a>(&self, text: &'a str) -> Result<Embedding>;
}

pub struct OllamaEmbeddingProvider {
    client: HttpClientWithUrl,
    endpoint: String,
    // Model should not change when creating embeddings, otherwise they're incompatible embeddings
    model: String,
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

pub fn normalize_embedding(embedding: Vec<f32>, size: usize) -> Embedding {
    // todo!(): Actually normalize the embedding, either using ndarray or writing the simd operations here
    match size {
        EMBEDDING_SIZE_TINY => Embedding::Tiny(embedding.try_into().unwrap()),
        EMBEDDING_SIZE_SMALL => Embedding::Small(embedding.try_into().unwrap()),
        EMBEDDING_SIZE_LARGE => Embedding::Large(embedding.try_into().unwrap()),
        _ => panic!("Invalid embedding size"),
    }
}

impl OllamaEmbeddingProvider {
    pub fn new(client: HttpClientWithUrl, endpoint: String, model: Option<String>) -> Self {
        Self {
            client,
            endpoint,
            model: model.unwrap_or("nomic-embed-text".to_string()),
        }
    }
}

impl EmbeddingProvider for OllamaEmbeddingProvider {
    async fn get_embedding(&self, text: &str) -> Result<Embedding> {
        let request = OllamaEmbeddingRequest {
            model: self.model.clone(),
            prompt: text.to_string(),
        };

        let request = serde_json::to_string(&request)?;
        let mut response = self
            .client
            .post_json("http://localhost:11434/api/embeddings", request.into())
            .await
            .context("failed to embed")?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await.ok();

        let response: OllamaEmbeddingResponse =
            serde_json::from_slice(body.as_slice()).context("Unable to pull response")?;

        let embedding_len = response.embedding.len();

        Ok(normalize_embedding(response.embedding, embedding_len))
    }
}
