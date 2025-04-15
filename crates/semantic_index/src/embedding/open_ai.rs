use crate::{Embedding, EmbeddingProvider, TextToEmbed};
use anyhow::Result;
use futures::{FutureExt, future::BoxFuture};
use http_client::HttpClient;
pub use open_ai::OpenAiEmbeddingModel;
use std::sync::Arc;

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
    fn embed<'a>(&'a self, texts: &'a [TextToEmbed<'a>]) -> BoxFuture<'a, Result<Vec<Embedding>>> {
        let embed = open_ai::embed(
            self.client.as_ref(),
            &self.api_url,
            &self.api_key,
            self.model,
            texts.iter().map(|to_embed| to_embed.text),
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
