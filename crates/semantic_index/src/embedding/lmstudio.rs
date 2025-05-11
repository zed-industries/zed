use anyhow::{Context as _, Result};
use futures::{AsyncReadExt as _, FutureExt, future::BoxFuture};
use http_client::HttpClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{Embedding, EmbeddingProvider, TextToEmbed};

pub enum LmStudioEmbeddingModel {
    NomicEmbedText,
}

pub struct LmStudioEmbeddingProvider {
    client: Arc<dyn HttpClient>,
    model: LmStudioEmbeddingModel,
}

#[derive(Serialize)]
struct LmStudioEmbeddingRequest {
    model: String,
    prompt: String,
}

#[derive(Deserialize)]
struct LmStudioEmbeddingResponse {
    embedding: Vec<f32>,
}

impl LmStudioEmbeddingProvider {
    pub fn new(client: Arc<dyn HttpClient>, model: LmStudioEmbeddingModel) -> Self {
        Self { client, model }
    }
}

impl EmbeddingProvider for LmStudioEmbeddingProvider {
    fn embed<'a>(&'a self, texts: &'a [TextToEmbed<'a>]) -> BoxFuture<'a, Result<Vec<Embedding>>> {
        let model = match self.model {
            LmStudioEmbeddingModel::NomicEmbedText => "nomic-embed-text",
        };

        futures::future::try_join_all(texts.iter().map(|to_embed| {
            let request = LmStudioEmbeddingRequest {
                model: model.to_string(),
                prompt: to_embed.text.to_string(),
            };

            let request = serde_json::to_string(&request).unwrap();

            async {
                let response = self
                    .client
                    .post_json("http://localhost:1234/api/v0/embeddings", request.into())
                    .await?;

                let mut body = String::new();
                response.into_body().read_to_string(&mut body).await?;

                let response: LmStudioEmbeddingResponse =
                    serde_json::from_str(&body).context("Unable to parse response")?;

                Ok(Embedding::new(response.embedding))
            }
        }))
        .boxed()
    }

    fn batch_size(&self) -> usize {
        256
    }
}
