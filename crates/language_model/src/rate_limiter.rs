use futures::Stream;
use smol::lock::{Semaphore, SemaphoreGuardArc};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use crate::LanguageModelCompletionError;

#[derive(Clone)]
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

pub struct RateLimitGuard<T> {
    inner: T,
    _guard: Option<SemaphoreGuardArc>,
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
                _guard: Some(guard),
            })
        }
    }

    /// Like `stream`, but conditionally bypasses the rate limiter based on the flag.
    /// Used for nested requests (like edit agent requests) that are already "part of"
    /// a rate-limited request to avoid deadlocks.
    pub fn stream_with_bypass<'a, Fut, T>(
        &self,
        future: Fut,
        bypass: bool,
    ) -> impl 'a
    + Future<
        Output = Result<impl Stream<Item = T::Item> + use<Fut, T>, LanguageModelCompletionError>,
    >
    where
        Fut: 'a + Future<Output = Result<T, LanguageModelCompletionError>>,
        T: Stream,
    {
        let semaphore = self.semaphore.clone();
        async move {
            let guard = if bypass {
                None
            } else {
                Some(semaphore.acquire_arc().await)
            };
            let inner = future.await?;
            Ok(RateLimitGuard {
                inner,
                _guard: guard,
            })
        }
    }
}
