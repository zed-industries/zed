use super::SpawnReady;
use futures_core::ready;
use pin_project_lite::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower_service::Service;

/// Builds [`SpawnReady`] instances with the result of an inner [`Service`].
#[derive(Clone, Debug)]
pub struct MakeSpawnReady<S> {
    inner: S,
}

impl<S> MakeSpawnReady<S> {
    /// Creates a new [`MakeSpawnReady`] wrapping `service`.
    pub fn new(service: S) -> Self {
        Self { inner: service }
    }
}

pin_project! {
    /// Builds a [`SpawnReady`] with the result of an inner [`Future`].
    #[derive(Debug)]
    pub struct MakeFuture<F> {
        #[pin]
        inner: F,
    }
}

impl<S, Target> Service<Target> for MakeSpawnReady<S>
where
    S: Service<Target>,
{
    type Response = SpawnReady<S::Response>;
    type Error = S::Error;
    type Future = MakeFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, target: Target) -> Self::Future {
        MakeFuture {
            inner: self.inner.call(target),
        }
    }
}

impl<F, T, E> Future for MakeFuture<F>
where
    F: Future<Output = Result<T, E>>,
{
    type Output = Result<SpawnReady<T>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let inner = ready!(this.inner.poll(cx))?;
        let svc = SpawnReady::new(inner);
        Poll::Ready(Ok(svc))
    }
}
