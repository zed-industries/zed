use anyhow::Result;
use futures::Stream;
use smol::lock::{Semaphore, SemaphoreGuardArc};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

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

    pub fn run<'a, Fut, T>(&self, future: Fut) -> impl 'a + Future<Output = Result<T>>
    where
        Fut: 'a + Future<Output = Result<T>>,
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
    ) -> impl 'a + Future<Output = Result<impl Stream<Item = T::Item> + use<Fut, T>>>
    where
        Fut: 'a + Future<Output = Result<T>>,
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
