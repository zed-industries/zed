use anyhow::{Context as _, Result};
use futures::{future::BoxFuture, AsyncReadExt, FutureExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::http::HttpClient;

use crate::{Embedding, EmbeddingProvider};

pub enum OllamaEmbeddingModel {
    NomicEmbedText,
    MxbaiEmbedLarge,
}

pub struct OllamaEmbeddingProvider {
    client: Arc<dyn HttpClient>,
    model: OllamaEmbeddingModel,
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

impl OllamaEmbeddingProvider {
    pub fn new(client: Arc<dyn HttpClient>, model: OllamaEmbeddingModel) -> Self {
        Self { client, model }
    }
}

impl EmbeddingProvider for OllamaEmbeddingProvider {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>> {
        //
        let model = match self.model {
            OllamaEmbeddingModel::NomicEmbedText => "nomic-embed-text",
            OllamaEmbeddingModel::MxbaiEmbedLarge => "mxbai-embed-large",
        };

        futures::future::try_join_all(texts.into_iter().map(|text| {
            let request = OllamaEmbeddingRequest {
                model: model.to_string(),
                prompt: text.to_string(),
            };

            let request = serde_json::to_string(&request).unwrap();

            async {
                let response = self
                    .client
                    .post_json("http://localhost:11434/api/embeddings", request.into())
                    .await?;

                let mut body = String::new();
                response.into_body().read_to_string(&mut body).await?;

                let response: OllamaEmbeddingResponse =
                    serde_json::from_str(&body).context("Unable to pull response")?;

                Ok(Embedding::new(response.embedding))
            }
        }))
        .boxed()
    }

    fn batch_size(&self) -> usize {
        // TODO: Figure out decent value
        10
    }
}
