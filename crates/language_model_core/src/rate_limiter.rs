use async_lock::{Semaphore, SemaphoreGuardArc};
use futures::Stream;
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use crate::{LanguageModelCompletionError, LanguageModelId};
use collections::HashMap;
use std::sync::Mutex;

/// Default number of concurrent requests a provider allows per model.
///
/// One limit is shared by every request to a model across the app: threads,
/// subagents, summaries, and inline assists. It is set well above normal
/// parallel use so it only stops runaway fan-out.
pub const DEFAULT_MODEL_CONCURRENCY: usize = 16;

/// A provider's local concurrency caps, one [`RateLimiter`] per model id.
///
/// Upstream rate limits are per credential and model family, and exceeding
/// them surfaces as retryable errors. This only bounds how many requests Zed
/// has in flight to one model at once.
pub struct ModelRateLimiters {
    limit: usize,
    limiters: Mutex<HashMap<LanguageModelId, RateLimiter>>,
}

impl ModelRateLimiters {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            limiters: Mutex::default(),
        }
    }

    /// Returns the limiter shared by every request to `model_id`.
    pub fn for_model(&self, model_id: &LanguageModelId) -> RateLimiter {
        let mut limiters = self
            .limiters
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        limiters
            .entry(model_id.clone())
            .or_insert_with(|| RateLimiter::new(self.limit))
            .clone()
    }
}

impl Default for ModelRateLimiters {
    fn default() -> Self {
        Self::new(DEFAULT_MODEL_CONCURRENCY)
    }
}

#[derive(Clone)]
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

pub struct RateLimitGuard<T> {
    inner: T,
    _guard: SemaphoreGuardArc,
}

impl<T> Stream for RateLimitGuard<T>
where
    T: Stream,
{
    type Item = T::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        unsafe { Pin::map_unchecked_mut(self, |this| &mut this.inner).poll_next(cx) }
    }
}

impl RateLimiter {
    pub fn new(limit: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(limit)),
        }
    }

    pub fn run<'a, Fut, T>(
        &self,
        future: Fut,
    ) -> impl 'a + Future<Output = Result<T, LanguageModelCompletionError>>
    where
        Fut: 'a + Future<Output = Result<T, LanguageModelCompletionError>>,
    {
        let guard = self.semaphore.acquire_arc();
        async move {
            let guard = guard.await;
            let result = future.await?;
            drop(guard);
            Ok(result)
        }
    }

    pub fn stream<'a, Fut, T>(
        &self,
        future: Fut,
    ) -> impl 'a
    + Future<
        Output = Result<impl Stream<Item = T::Item> + use<Fut, T>, LanguageModelCompletionError>,
    >
    where
        Fut: 'a + Future<Output = Result<T, LanguageModelCompletionError>>,
        T: Stream,
    {
        let guard = self.semaphore.acquire_arc();
        async move {
            let guard = guard.await;
            let inner = future.await?;
            Ok(RateLimitGuard {
                inner,
                _guard: guard,
            })
        }
    }
}
