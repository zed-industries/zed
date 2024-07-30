use anyhow::Result;
use futures::Stream;
use smol::lock::{Semaphore, SemaphoreGuardArc};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

pub struct RateLimitGuard<T> {
    inner: T,
    _guard: SemaphoreGuardArc,
}

impl<T> Future for RateLimitGuard<T>
where
    T: Future,
{
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        unsafe { Pin::map_unchecked_mut(self, |this| &mut this.inner).poll(cx) }
    }
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
        f: impl 'a + FnOnce() -> Fut,
    ) -> impl 'a + Future<Output = Result<RateLimitGuard<T>>>
    where
        Fut: Future<Output = Result<T>>,
    {
        let guard = self.semaphore.acquire_arc();
        async move {
            let guard = guard.await;
            let inner = f().await?;
            Ok(RateLimitGuard {
                inner,
                _guard: guard,
            })
        }
    }
}
