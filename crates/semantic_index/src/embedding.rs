mod ollama;
mod openai;
mod zed;

pub use ollama::*;
pub use openai::*;
pub use zed::*;

use anyhow::Result;
use futures::{future::BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use std::{fmt, future};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Embedding(Vec<f32>);

impl Embedding {
    fn new(mut embedding: Vec<f32>) -> Self {
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

    pub fn similarity(self, other: &Embedding) -> f32 {
        debug_assert_eq!(self.0.len(), other.0.len());
        self.0
            .iter()
            .copied()
            .zip(other.0.iter().copied())
            .map(|(a, b)| a * b)
            .sum()
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

/// Trait for embedding providers. Texts in, vectors out.
pub trait EmbeddingProvider: Sync + Send {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>>;
    fn batch_size(&self) -> usize;
}

pub struct FakeEmbeddingProvider;

impl EmbeddingProvider for FakeEmbeddingProvider {
    fn embed(&self, texts: &[&str]) -> BoxFuture<Result<Vec<Embedding>>> {
        let embeddings = texts
            .iter()
            .map(|text| {
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
