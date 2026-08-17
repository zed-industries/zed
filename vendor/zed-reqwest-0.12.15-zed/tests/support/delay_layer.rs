use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use pin_project_lite::pin_project;
use tokio::time::Sleep;
use tower::{BoxError, Layer, Service};

/// This tower layer injects an arbitrary delay before calling downstream layers.
#[derive(Clone)]
pub struct DelayLayer {
    delay: Duration,
}

impl DelayLayer {
    pub const fn new(delay: Duration) -> Self {
        DelayLayer { delay }
    }
}

impl<S> Layer<S> for DelayLayer {
    type Service = Delay<S>;
    fn layer(&self, service: S) -> Self::Service {
        Delay::new(service, self.delay)
    }
}

impl std::fmt::Debug for DelayLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DelayLayer")
            .field("delay", &self.delay)
            .finish()
    }
}

/// This tower service injects an arbitrary delay before calling downstream layers.
#[derive(Debug, Clone)]
pub struct Delay<S> {
    inner: S,
    delay: Duration,
}
impl<S> Delay<S> {
    pub fn new(inner: S, delay: Duration) -> Self {
        Delay { inner, delay }
    }
}

impl<S, Request> Service<Request> for Delay<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;

    type Error = BoxError;

    type Future = ResponseFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => Poll::Ready(r.map_err(Into::into)),
        }
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let response = self.inner.call(req);
        let sleep = tokio::time::sleep(self.delay);

        ResponseFuture::new(response, sleep)
    }
}

// `Delay` response future
pin_project! {
    #[derive(Debug)]
    pub struct ResponseFuture<S> {
        #[pin]
        response: S,
        #[pin]
        sleep: Sleep,
    }
}

impl<S> ResponseFuture<S> {
    pub(crate) fn new(response: S, sleep: Sleep) -> Self {
        ResponseFuture { response, sleep }
    }
}

impl<F, S, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<S, E>>,
    E: Into<BoxError>,
{
    type Output = Result<S, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        // First poll the sleep until complete
        match this.sleep.poll(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(_) => {}
        }

        // Then poll the inner future
        match this.response.poll(cx) {
            Poll::Ready(v) => Poll::Ready(v.map_err(Into::into)),
            Poll::Pending => Poll::Pending,
        }
    }
}
