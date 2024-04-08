use crate::{Embedding, EmbeddingProvider};
use anyhow::{Context, Result};
use client::{proto, Client};
use futures::{future::BoxFuture, FutureExt};
use std::sync::Arc;

pub struct ZedEmbeddingProvider {
    client: Arc<Client>,
}

impl EmbeddingProvider for ZedEmbeddingProvider {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>> {
        let client = self.client.clone();
        let response = client.request(proto::EmbedTexts {
            texts: texts.iter().map(ToString::to_string).collect(),
        });
        async move {
            let response = response
                .await
                .context("failed to embed texts via zed provider")?;
            Ok(response
                .embeddings
                .into_iter()
                .map(|embedding| Embedding::new(embedding.dimensions))
                .collect())
        }
        .boxed()
    }

    fn batch_size(&self) -> usize {
        2048
    }
}
