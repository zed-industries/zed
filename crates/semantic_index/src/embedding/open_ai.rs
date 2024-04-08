use crate::{Embedding, EmbeddingProvider};
use anyhow::Result;
use futures::{future::BoxFuture, FutureExt};
pub use open_ai::OpenAiEmbeddingModel;
use std::sync::Arc;
use util::http::HttpClient;

pub struct OpenAiEmbeddingProvider {
    client: Arc<dyn HttpClient>,
    model: OpenAiEmbeddingModel,
    api_url: String,
    api_key: String,
}

impl OpenAiEmbeddingProvider {
    pub fn new(
        client: Arc<dyn HttpClient>,
        model: OpenAiEmbeddingModel,
        api_url: String,
        api_key: String,
    ) -> Self {
        Self {
            client,
            model,
            api_url,
            api_key,
        }
    }
}

impl EmbeddingProvider for OpenAiEmbeddingProvider {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>> {
        let embed = open_ai::embed(
            self.client.as_ref(),
            &self.api_url,
            &self.api_key,
            self.model,
            texts.iter().map(|text| *text),
        );
        async move {
            let response = embed.await?;
            Ok(response
                .data
                .into_iter()
                .map(|data| Embedding::new(data.embedding))
                .collect())
        }
        .boxed()
    }

    fn batch_size(&self) -> usize {
        // From https://platform.openai.com/docs/api-reference/embeddings/create
        2048
    }
}
