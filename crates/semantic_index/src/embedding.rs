mod lmstudio;
mod ollama;
mod open_ai;

pub use lmstudio::*;
pub use ollama::*;
pub use open_ai::*;
use sha2::{Digest, Sha256};

use anyhow::Result;
use futures::{FutureExt, future::BoxFuture};
use serde::{Deserialize, Serialize};
use std::{fmt, future};

/// Trait for embedding providers. Texts in, vectors out.
pub trait EmbeddingProvider: Sync + Send {
    fn embed<'a>(&'a self, texts: &'a [TextToEmbed<'a>]) -> BoxFuture<'a, Result<Vec<Embedding>>>;
    fn batch_size(&self) -> usize;
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Embedding(Vec<f32>);

impl Embedding {
    pub fn new(mut embedding: Vec<f32>) -> Self {
        let len = embedding.len();
        let mut norm = 0f32;

        for i in 0..len {
            norm += embedding[i] * embedding[i];
        }

        norm = norm.sqrt();
        for dimension in &mut embedding {
            *dimension /= norm;
        }

        Self(embedding)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    pub fn similarity(&self, others: &[Embedding]) -> (f32, usize) {
        debug_assert!(others.iter().all(|other| self.0.len() == other.0.len()));
        others
            .iter()
            .enumerate()
            .map(|(index, other)| {
                let dot_product: f32 = self
                    .0
                    .iter()
                    .copied()
                    .zip(other.0.iter().copied())
                    .map(|(a, b)| a * b)
                    .sum();
                (dot_product, index)
            })
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((0.0, 0))
    }
}

impl fmt::Display for Embedding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digits_to_display = 3;

        // Start the Embedding display format
        write!(f, "Embedding(sized: {}; values: [", self.len())?;

        for (index, value) in self.0.iter().enumerate().take(digits_to_display) {
            // Lead with comma if not the first element
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.3}", value)?;
        }
        if self.len() > digits_to_display {
            write!(f, "...")?;
        }
        write!(f, "])")
    }
}

#[derive(Debug)]
pub struct TextToEmbed<'a> {
    pub text: &'a str,
    pub digest: [u8; 32],
}

impl<'a> TextToEmbed<'a> {
    pub fn new(text: &'a str) -> Self {
        let digest = Sha256::digest(text.as_bytes());
        Self {
            text,
            digest: digest.into(),
        }
    }
}

pub struct FakeEmbeddingProvider;

impl EmbeddingProvider for FakeEmbeddingProvider {
    fn embed<'a>(&'a self, texts: &'a [TextToEmbed<'a>]) -> BoxFuture<'a, Result<Vec<Embedding>>> {
        let embeddings = texts
            .iter()
            .map(|_text| {
                let mut embedding = vec![0f32; 1536];
                for i in 0..embedding.len() {
                    embedding[i] = i as f32;
                }
                Embedding::new(embedding)
            })
            .collect();
        future::ready(Ok(embeddings)).boxed()
    }

    fn batch_size(&self) -> usize {
        16
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[gpui::test]
    fn test_normalize_embedding() {
        let normalized = Embedding::new(vec![1.0, 1.0, 1.0]);
        let value: f32 = 1.0 / 3.0_f32.sqrt();
        assert_eq!(normalized, Embedding(vec![value; 3]));
    }
}
