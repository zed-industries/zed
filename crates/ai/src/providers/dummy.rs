use std::time::Instant;

use crate::{
    completion::CompletionRequest,
    embedding::{Embedding, EmbeddingProvider},
    models::{LanguageModel, TruncationDirection},
};
use async_trait::async_trait;
use serde::Serialize;

pub struct DummyLanguageModel {}

impl LanguageModel for DummyLanguageModel {
    fn name(&self) -> String {
        "dummy".to_string()
    }
    fn capacity(&self) -> anyhow::Result<usize> {
        anyhow::Ok(1000)
    }
    fn truncate(
        &self,
        content: &str,
        length: usize,
        direction: crate::models::TruncationDirection,
    ) -> anyhow::Result<String> {
        if content.len() < length {
            return anyhow::Ok(content.to_string());
        }

        let truncated = match direction {
            TruncationDirection::End => content.chars().collect::<Vec<char>>()[..length]
                .iter()
                .collect::<String>(),
            TruncationDirection::Start => content.chars().collect::<Vec<char>>()[..length]
                .iter()
                .collect::<String>(),
        };

        anyhow::Ok(truncated)
    }
    fn count_tokens(&self, content: &str) -> anyhow::Result<usize> {
        anyhow::Ok(content.chars().collect::<Vec<char>>().len())
    }
}

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
    fn base_model(&self) -> Box<dyn LanguageModel> {
        Box::new(DummyLanguageModel {})
    }
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
}
