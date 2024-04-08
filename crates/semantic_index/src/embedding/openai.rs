use anyhow::{anyhow, Context as _, Result};
use futures::{future::BoxFuture, AsyncReadExt, FutureExt};
use serde::{Deserialize, Serialize};
use std::{future, sync::Arc};
use util::http::{AsyncBody, HttpClient, Method, Request as HttpRequest};

use crate::{Embedding, EmbeddingProvider};

pub enum OpenaiEmbeddingModel {
    TextEmbedding3Small,
    TextEmbedding3Large,
}

pub struct OpenaiEmbeddingProvider {
    client: Arc<dyn HttpClient>,
    model: OpenaiEmbeddingModel,
    api_key: String,
}

#[derive(Serialize)]
struct OpenaiEmbeddingRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct OpenaiEmbeddingData {
    embedding: Vec<f32>,
}

#[derive(Deserialize)]
struct OpenaiEmbeddingResponse {
    object: String,
    data: Vec<OpenaiEmbeddingData>,
    model: String,
}

impl OpenaiEmbeddingProvider {
    pub fn new(client: Arc<dyn HttpClient>, model: OpenaiEmbeddingModel, api_key: String) -> Self {
        Self {
            client,
            model,
            api_key,
        }
    }
}

impl EmbeddingProvider for OpenaiEmbeddingProvider {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>> {
        let model = match self.model {
            OpenaiEmbeddingModel::TextEmbedding3Small => "text-embedding-3-small",
            OpenaiEmbeddingModel::TextEmbedding3Large => "text-embedding-3-large",
        };

        // Unlike the Ollama model, we can send `texts` as a batch directly

        let api_url = "https://api.openai.com/v1/";

        let uri = format!("{api_url}/embeddings");

        let request = OpenaiEmbeddingRequest {
            model: model.to_string(),
            input: texts.iter().map(|text| text.to_string()).collect(),
        };
        let request = serde_json::to_string(&request).unwrap();
        let body = AsyncBody::from(request);

        let request = HttpRequest::builder()
            .method(Method::POST)
            .uri(uri)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(body);

        let request = if let Ok(request) = request {
            request
        } else {
            return future::ready(Err(anyhow!("Failed to build request"))).boxed();
        };

        async {
            let response = self.client.send(request).await?;

            let mut body = String::new();
            response.into_body().read_to_string(&mut body).await?;

            // todo(): check for errors in response (likely due to rate limiting or invalid API key)
            let response: OpenaiEmbeddingResponse = serde_json::from_str(&body)
                .context("Response format from OpenAI did not match struct")?;

            let embeddings = response
                .data
                .into_iter()
                .map(|data| Embedding::new(data.embedding))
                .collect();

            Ok(embeddings)
        }
        .boxed()
    }

    fn batch_size(&self) -> usize {
        // From https://platform.openai.com/docs/api-reference/embeddings/create
        2048
    }
}
