use crate::{Embedding, EmbeddingProvider};
use anyhow::Result;
use client::Client;
use futures::future::BoxFuture;
use std::sync::Arc;

pub struct ZedEmbeddingProvider {
    client: Arc<Client>,
}

impl EmbeddingProvider for ZedEmbeddingProvider {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>> {
        todo!()
    }

    fn batch_size(&self) -> usize {
        2048
    }
}
