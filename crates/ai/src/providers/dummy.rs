use std::time::Instant;

use crate::{
    completion::CompletionRequest,
    embedding::{Embedding, EmbeddingProvider},
};
use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize)]
pub struct DummyCompletionRequest {
    pub name: String,
}

impl CompletionRequest for DummyCompletionRequest {
    fn data(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

pub struct DummyEmbeddingProvider {}

#[async_trait]
impl EmbeddingProvider for DummyEmbeddingProvider {
    fn is_authenticated(&self) -> bool {
        true
    }
    fn rate_limit_expiration(&self) -> Option<Instant> {
        None
    }
    async fn embed_batch(&self, spans: Vec<String>) -> anyhow::Result<Vec<Embedding>> {
        // 1024 is the OpenAI Embeddings size for ada models.
        // the model we will likely be starting with.
        let dummy_vec = Embedding::from(vec![0.32 as f32; 1536]);
        return Ok(vec![dummy_vec; spans.len()]);
    }

    fn max_tokens_per_batch(&self) -> usize {
        8190
    }

    fn truncate(&self, span: &str) -> (String, usize) {
        let truncated = span.chars().collect::<Vec<char>>()[..8190]
            .iter()
            .collect::<String>();
        (truncated, 8190)
    }
}
