use std::{
    sync::atomic::{self, AtomicUsize, Ordering},
    time::Instant,
};

use async_trait::async_trait;
use futures::{channel::mpsc, future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::AppContext;
use parking_lot::Mutex;

use crate::{
    auth::{CredentialProvider, ProviderCredential},
    completion::{CompletionProvider, CompletionRequest},
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
        println!("TRYING TO TRUNCATE: {:?}", length.clone());

        if length > self.count_tokens(content)? {
            println!("NOT TRUNCATING");
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

#[derive(Default)]
pub struct FakeEmbeddingProvider {
    pub embedding_count: AtomicUsize,
}

impl Clone for FakeEmbeddingProvider {
    fn clone(&self) -> Self {
        FakeEmbeddingProvider {
            embedding_count: AtomicUsize::new(self.embedding_count.load(Ordering::SeqCst)),
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

impl CredentialProvider for FakeEmbeddingProvider {
    fn has_credentials(&self) -> bool {
        true
    }

    fn retrieve_credentials(&self, _cx: &mut AppContext) -> BoxFuture<ProviderCredential> {
        async { ProviderCredential::NotNeeded }.boxed()
    }

    fn save_credentials(
        &self,
        _cx: &mut AppContext,
        _credential: ProviderCredential,
    ) -> BoxFuture<()> {
        async {}.boxed()
    }

    fn delete_credentials(&self, _cx: &mut AppContext) -> BoxFuture<()> {
        async {}.boxed()
    }
}

#[async_trait]
impl EmbeddingProvider for FakeEmbeddingProvider {
    fn base_model(&self) -> Box<dyn LanguageModel> {
        Box::new(FakeLanguageModel { capacity: 1000 })
    }
    fn max_tokens_per_batch(&self) -> usize {
        1000
    }

    fn rate_limit_expiration(&self) -> Option<Instant> {
        None
    }

    async fn embed_batch(&self, spans: Vec<String>) -> anyhow::Result<Vec<Embedding>> {
        self.embedding_count
            .fetch_add(spans.len(), atomic::Ordering::SeqCst);

        anyhow::Ok(spans.iter().map(|span| self.embed_sync(span)).collect())
    }
}

pub struct FakeCompletionProvider {
    last_completion_tx: Mutex<Option<mpsc::Sender<String>>>,
}

impl Clone for FakeCompletionProvider {
    fn clone(&self) -> Self {
        Self {
            last_completion_tx: Mutex::new(None),
        }
    }
}

impl FakeCompletionProvider {
    pub fn new() -> Self {
        Self {
            last_completion_tx: Mutex::new(None),
        }
    }

    pub fn send_completion(&self, completion: impl Into<String>) {
        let mut tx = self.last_completion_tx.lock();
        tx.as_mut().unwrap().try_send(completion.into()).unwrap();
    }

    pub fn finish_completion(&self) {
        self.last_completion_tx.lock().take().unwrap();
    }
}

impl CredentialProvider for FakeCompletionProvider {
    fn has_credentials(&self) -> bool {
        true
    }

    fn retrieve_credentials(&self, _cx: &mut AppContext) -> BoxFuture<ProviderCredential> {
        async { ProviderCredential::NotNeeded }.boxed()
    }

    fn save_credentials(
        &self,
        _cx: &mut AppContext,
        _credential: ProviderCredential,
    ) -> BoxFuture<()> {
        async {}.boxed()
    }

    fn delete_credentials(&self, _cx: &mut AppContext) -> BoxFuture<()> {
        async {}.boxed()
    }
}

impl CompletionProvider for FakeCompletionProvider {
    fn base_model(&self) -> Box<dyn LanguageModel> {
        let model: Box<dyn LanguageModel> = Box::new(FakeLanguageModel { capacity: 8190 });
        model
    }
    fn complete(
        &self,
        _prompt: Box<dyn CompletionRequest>,
    ) -> BoxFuture<'static, anyhow::Result<BoxStream<'static, anyhow::Result<String>>>> {
        let (tx, rx) = mpsc::channel(1);
        *self.last_completion_tx.lock() = Some(tx);
        async move { Ok(rx.map(|rx| Ok(rx)).boxed()) }.boxed()
    }
    fn box_clone(&self) -> Box<dyn CompletionProvider> {
        Box::new((*self).clone())
    }
}
