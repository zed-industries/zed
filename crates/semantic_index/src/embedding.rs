use anyhow::Result;
use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait EmbeddingProvider {
    async fn get_embedding(&self, text: &str) -> Result<Vec<f64>>;
}

pub struct OllamaEmbeddingProvider {
    client: Client,
    endpoint: String,
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
