use anyhow::{Context as _, Result};
use futures::{future::BoxFuture, AsyncReadExt as _, FutureExt};
use http_client::HttpClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{Embedding, EmbeddingProvider, TextToEmbed};

pub enum LMStudioEmbeddingModel {
    NomicEmbedText,
}

pub struct LMStudioEmbeddingProvider {
    client: Arc<dyn HttpClient>,
    model: LMStudioEmbeddingModel,
}

#[derive(Serialize)]
struct LMStudioEmbeddingRequest {
    model: String,
    prompt: String,
}

#[derive(Deserialize)]
struct LMStudioEmbeddingResponse {
    embedding: Vec<f32>,
}

impl LMStudioEmbeddingProvider {
    pub fn new(client: Arc<dyn HttpClient>, model: LMStudioEmbeddingModel) -> Self {
        Self { client, model }
    }
}

impl EmbeddingProvider for LMStudioEmbeddingProvider {
    fn embed<'a>(&'a self, texts: &'a [TextToEmbed<'a>]) -> BoxFuture<'a, Result<Vec<Embedding>>> {
        //
        let model = match self.model {
            LMStudioEmbeddingModel::NomicEmbedText => "nomic-embed-text",
        };

        futures::future::try_join_all(texts.iter().map(|to_embed| {
            let request = LMStudioEmbeddingRequest {
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

                let response: LMStudioEmbeddingResponse =
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
