use crate::{Embedding, EmbeddingProvider, TextToEmbed};
use anyhow::{anyhow, Context, Result};
use client::{proto, Client};
use collections::HashMap;
use futures::{future::BoxFuture, FutureExt};
use std::sync::Arc;

pub struct CloudEmbeddingProvider {
    model: String,
    client: Arc<Client>,
}

impl CloudEmbeddingProvider {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            model: "openai/text-embedding-3-small".into(),
            client,
        }
    }
}

impl EmbeddingProvider for CloudEmbeddingProvider {
    fn embed<'a>(&'a self, texts: &'a [TextToEmbed<'a>]) -> BoxFuture<'a, Result<Vec<Embedding>>> {
        // First, fetch any embeddings that are cached based on the requested texts' digests
        // Then compute any embeddings that are missing.
        async move {
            if !self.client.status().borrow().is_connected() {
                return Err(anyhow!("sign in required"));
            }

            let cached_embeddings = self.client.request(proto::GetCachedEmbeddings {
                model: self.model.clone(),
                digests: texts
                    .iter()
                    .map(|to_embed| to_embed.digest.to_vec())
                    .collect(),
            });
            let mut embeddings = cached_embeddings
                .await
                .context("failed to fetch cached embeddings via cloud model")?
                .embeddings
                .into_iter()
                .map(|embedding| {
                    let digest: [u8; 32] = embedding
                        .digest
                        .try_into()
                        .map_err(|_| anyhow!("invalid digest for cached embedding"))?;
                    Ok((digest, embedding.dimensions))
                })
                .collect::<Result<HashMap<_, _>>>()?;

            let compute_embeddings_request = proto::ComputeEmbeddings {
                model: self.model.clone(),
                texts: texts
                    .iter()
                    .filter_map(|to_embed| {
                        if embeddings.contains_key(&to_embed.digest) {
                            None
                        } else {
                            Some(to_embed.text.to_string())
                        }
                    })
                    .collect(),
            };
            if !compute_embeddings_request.texts.is_empty() {
                let missing_embeddings = self.client.request(compute_embeddings_request).await?;
                for embedding in missing_embeddings.embeddings {
                    let digest: [u8; 32] = embedding
                        .digest
                        .try_into()
                        .map_err(|_| anyhow!("invalid digest for cached embedding"))?;
                    embeddings.insert(digest, embedding.dimensions);
                }
            }

            texts
                .iter()
                .map(|to_embed| {
                    let embedding =
                        embeddings.get(&to_embed.digest).cloned().with_context(|| {
                            format!("server did not return an embedding for {:?}", to_embed)
                        })?;
                    Ok(Embedding::new(embedding))
                })
                .collect()
        }
        .boxed()
    }

    fn batch_size(&self) -> usize {
        2048
    }
}
