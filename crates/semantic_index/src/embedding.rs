use std::sync::Arc;

use anyhow::{Context as _, Result};
use futures::AsyncReadExt;
use serde::{Deserialize, Serialize};
use util::http::{HttpClient, HttpClientWithUrl};

/// Ollama's embedding via nomic-embed-text is of length 768
pub const EMBEDDING_SIZE_TINY: usize = 768;
/// OpenAI's text small embeddings are of length 1536
pub const EMBEDDING_SIZE_SMALL: usize = 1536;
/// OpenAI's text large embeddings are of length 3072
pub const EMBEDDING_SIZE_LARGE: usize = 3072;

// TODO: Check out Voyage

pub enum Embedding {
    Tiny([f32; EMBEDDING_SIZE_TINY]),
    Small([f32; EMBEDDING_SIZE_SMALL]),
    Large([f32; EMBEDDING_SIZE_LARGE]),
    None,
}

pub trait EmbeddingProvider {
    async fn get_embedding(&self, text: String) -> Result<Embedding>;
}

pub struct OllamaEmbeddingProvider {
    client: Arc<HttpClientWithUrl>,
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

pub(crate) fn normalize_vector(embedding: Vec<f32>) -> Vec<f32> {
    // TODO: Either use ndarray directly like this:
    //         let array = ndarray::Array1::from(self.embedding.clone());
    //         let norm = array.dot(&array).sqrt();
    //         array / norm
    // OR: use simd operations directly to calculate the norm and normalize the embedding
    let len = embedding.len();
    let mut norm = 0f32;

    for i in 0..len {
        norm += embedding[i] * embedding[i];
    }

    norm = norm.sqrt();

    embedding.iter().map(|x| x / norm).collect::<Vec<f32>>()
}

pub fn normalize_embedding(embedding: Vec<f32>) -> Embedding {
    let embedding = normalize_vector(embedding);

    match embedding.len() {
        EMBEDDING_SIZE_TINY => Embedding::Tiny(embedding.try_into().unwrap()),
        EMBEDDING_SIZE_SMALL => Embedding::Small(embedding.try_into().unwrap()),
        EMBEDDING_SIZE_LARGE => Embedding::Large(embedding.try_into().unwrap()),
        _ => panic!("Invalid embedding size"),
    }
}

impl OllamaEmbeddingProvider {
    pub fn new(client: Arc<HttpClientWithUrl>, model: Option<String>) -> Self {
        Self {
            client,
            model: model.unwrap_or("nomic-embed-text".to_string()),
        }
    }
}

impl EmbeddingProvider for OllamaEmbeddingProvider {
    async fn get_embedding(&self, text: String) -> Result<Embedding> {
        let request = OllamaEmbeddingRequest {
            model: self.model.clone(),
            prompt: text,
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

        Ok(normalize_embedding(response.embedding))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::BackgroundExecutor;

    #[gpui::test]
    async fn test_ollama_embedding_provider(executor: BackgroundExecutor) {
        executor.allow_parking();

        let client = Arc(HttpClientWithUrl::new("http://localhost:11434/"));
        let provider = OllamaEmbeddingProvider::new(client.into(), None);
        let embedding = provider
            .get_embedding("Hello, world!".to_string())
            .await
            .unwrap();

        match embedding {
            Embedding::Tiny(e) => assert_eq!(e.len(), EMBEDDING_SIZE_TINY),
            _ => panic!("Invalid embedding size"),
        }
    }

    #[gpui::test]
    fn test_normalize_embedding() {
        // Create an vector of size EMBEDDING_SIZE_TINY with all values set to 1.0
        let embedding = vec![1.0, 1.0, 1.0];

        let normalized = normalize_vector(embedding);

        let value: f32 = 1.0 / 3.0_f32.sqrt();

        assert_eq!(normalized, vec![value; 3]);
    }
}
