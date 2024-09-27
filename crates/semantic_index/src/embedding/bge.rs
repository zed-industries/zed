use crate::{Embedding, EmbeddingProvider, TextToEmbed};
use anyhow::Result;
pub use bge::BgeEmbeddingModel;
use futures::{future::BoxFuture, FutureExt};
use http_client::HttpClient;
use std::sync::Arc;

pub struct BgeEmbeddingProvider {
    client: Arc<dyn HttpClient>,
    model: BgeEmbeddingModel,
    api_url: String,
    api_key: String,
}

impl BgeEmbeddingProvider {
    pub fn new(
        client: Arc<dyn HttpClient>,
        model: BgeEmbeddingModel,
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

impl EmbeddingProvider for BgeEmbeddingProvider {
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
