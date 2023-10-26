use std::{
    sync::atomic::{self, AtomicUsize, Ordering},
    time::Instant,
};

use async_trait::async_trait;

use crate::{
    auth::{CredentialProvider, NullCredentialProvider, ProviderCredential},
    embedding::{Embedding, EmbeddingProvider},
    models::{LanguageModel, TruncationDirection},
};

#[derive(Clone)]
pub struct FakeLanguageModel {
    pub capacity: usize,
}

impl LanguageModel for FakeLanguageModel {
    fn name(&self) -> String {
        "dummy".to_string()
    }
    fn count_tokens(&self, content: &str) -> anyhow::Result<usize> {
        anyhow::Ok(content.chars().collect::<Vec<char>>().len())
    }
    fn truncate(
        &self,
        content: &str,
        length: usize,
        direction: TruncationDirection,
    ) -> anyhow::Result<String> {
        if length > self.count_tokens(content)? {
            return anyhow::Ok(content.to_string());
        }

        anyhow::Ok(match direction {
            TruncationDirection::End => content.chars().collect::<Vec<char>>()[..length]
                .into_iter()
                .collect::<String>(),
            TruncationDirection::Start => content.chars().collect::<Vec<char>>()[length..]
                .into_iter()
                .collect::<String>(),
        })
    }
    fn capacity(&self) -> anyhow::Result<usize> {
        anyhow::Ok(self.capacity)
    }
}

pub struct FakeEmbeddingProvider {
    pub embedding_count: AtomicUsize,
    pub credential_provider: NullCredentialProvider,
}

impl Clone for FakeEmbeddingProvider {
    fn clone(&self) -> Self {
        FakeEmbeddingProvider {
            embedding_count: AtomicUsize::new(self.embedding_count.load(Ordering::SeqCst)),
            credential_provider: self.credential_provider.clone(),
        }
    }
}

impl Default for FakeEmbeddingProvider {
    fn default() -> Self {
        FakeEmbeddingProvider {
            embedding_count: AtomicUsize::default(),
            credential_provider: NullCredentialProvider {},
        }
    }
}

impl FakeEmbeddingProvider {
    pub fn embedding_count(&self) -> usize {
        self.embedding_count.load(atomic::Ordering::SeqCst)
    }

    pub fn embed_sync(&self, span: &str) -> Embedding {
        let mut result = vec![1.0; 26];
        for letter in span.chars() {
            let letter = letter.to_ascii_lowercase();
            if letter as u32 >= 'a' as u32 {
                let ix = (letter as u32) - ('a' as u32);
                if ix < 26 {
                    result[ix as usize] += 1.0;
                }
            }
        }

        let norm = result.iter().map(|x| x * x).sum::<f32>().sqrt();
        for x in &mut result {
            *x /= norm;
        }

        result.into()
    }
}

#[async_trait]
impl EmbeddingProvider for FakeEmbeddingProvider {
    fn base_model(&self) -> Box<dyn LanguageModel> {
        Box::new(FakeLanguageModel { capacity: 1000 })
    }
    fn credential_provider(&self) -> Box<dyn CredentialProvider> {
        let credential_provider: Box<dyn CredentialProvider> =
            Box::new(self.credential_provider.clone());
        credential_provider
    }
    fn max_tokens_per_batch(&self) -> usize {
        1000
    }

    fn rate_limit_expiration(&self) -> Option<Instant> {
        None
    }

    async fn embed_batch(
        &self,
        spans: Vec<String>,
        _credential: ProviderCredential,
    ) -> anyhow::Result<Vec<Embedding>> {
        self.embedding_count
            .fetch_add(spans.len(), atomic::Ordering::SeqCst);

        anyhow::Ok(spans.iter().map(|span| self.embed_sync(span)).collect())
    }
}
