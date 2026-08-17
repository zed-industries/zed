use super::Handler;
use crate::response::Response;
use http::Request;
use std::{
    convert::Infallible,
    fmt,
    marker::PhantomData,
    task::{Context, Poll},
};
use tower_service::Service;

pub(crate) struct IntoServiceStateInExtension<H, T, S, B> {
    handler: H,
    _marker: PhantomData<fn() -> (T, S, B)>,
}

#[test]
fn traits() {
    use crate::test_helpers::*;
    assert_send::<IntoServiceStateInExtension<(), NotSendSync, (), NotSendSync>>();
    assert_sync::<IntoServiceStateInExtension<(), NotSendSync, (), NotSendSync>>();
}

impl<H, T, S, B> IntoServiceStateInExtension<H, T, S, B> {
    pub(crate) fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S, B> fmt::Debug for IntoServiceStateInExtension<H, T, S, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntoServiceStateInExtension")
            .finish_non_exhaustive()
    }
}

impl<H, T, S, B> Clone for IntoServiceStateInExtension<H, T, S, B>
where
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S, B> Service<Request<B>> for IntoServiceStateInExtension<H, T, S, B>
where
    H: Handler<T, S, B> + Clone + Send + 'static,
    B: Send + 'static,
    S: Send + Sync + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future = super::future::IntoServiceFuture<H::Future>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // `IntoServiceStateInExtension` can only be constructed from async functions which are always ready, or
        // from `Layered` which buffers in `<Layered as Handler>::call` and is therefore
        // also always ready.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        use futures_util::future::FutureExt;

        let state = req
            .extensions_mut()
            .remove::<S>()
            .expect("state extension missing. This is a bug in axum, please file an issue");

        let handler = self.handler.clone();
        let future = Handler::call(handler, req, state);
        let future = future.map(Ok as _);

        super::future::IntoServiceFuture::new(future)
    }
}
