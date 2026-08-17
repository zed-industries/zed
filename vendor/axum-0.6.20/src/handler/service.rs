use super::Handler;
#[cfg(feature = "tokio")]
use crate::extract::connect_info::IntoMakeServiceWithConnectInfo;
use crate::response::Response;
use crate::routing::IntoMakeService;
use http::Request;
use std::{
    convert::Infallible,
    fmt,
    marker::PhantomData,
    task::{Context, Poll},
};
use tower_service::Service;

/// An adapter that makes a [`Handler`] into a [`Service`].
///
/// Created with [`Handler::with_state`] or [`HandlerWithoutStateExt::into_service`].
///
/// [`HandlerWithoutStateExt::into_service`]: super::HandlerWithoutStateExt::into_service
pub struct HandlerService<H, T, S, B> {
    handler: H,
    state: S,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, S, B> HandlerService<H, T, S, B> {
    /// Get a reference to the state.
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Convert the handler into a [`MakeService`].
    ///
    /// This allows you to serve a single handler if you don't need any routing:
    ///
    /// ```rust
    /// use axum::{
    ///     Server,
    ///     handler::Handler,
    ///     extract::State,
    ///     http::{Uri, Method},
    ///     response::IntoResponse,
    /// };
    /// use std::net::SocketAddr;
    ///
    /// #[derive(Clone)]
    /// struct AppState {}
    ///
    /// async fn handler(State(state): State<AppState>) {
    ///     // ...
    /// }
    ///
    /// let app = handler.with_state(AppState {});
    ///
    /// # async {
    /// Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
    ///     .serve(app.into_make_service())
    ///     .await?;
    /// # Ok::<_, hyper::Error>(())
    /// # };
    /// ```
    ///
    /// [`MakeService`]: tower::make::MakeService
    pub fn into_make_service(self) -> IntoMakeService<HandlerService<H, T, S, B>> {
        IntoMakeService::new(self)
    }

    /// Convert the handler into a [`MakeService`] which stores information
    /// about the incoming connection.
    ///
    /// See [`Router::into_make_service_with_connect_info`] for more details.
    ///
    /// ```rust
    /// use axum::{
    ///     Server,
    ///     handler::Handler,
    ///     response::IntoResponse,
    ///     extract::{ConnectInfo, State},
    /// };
    /// use std::net::SocketAddr;
    ///
    /// #[derive(Clone)]
    /// struct AppState {};
    ///
    /// async fn handler(
    ///     ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ///     State(state): State<AppState>,
    /// ) -> String {
    ///     format!("Hello {}", addr)
    /// }
    ///
    /// let app = handler.with_state(AppState {});
    ///
    /// # async {
    /// Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
    ///     .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    ///     .await?;
    /// # Ok::<_, hyper::Error>(())
    /// # };
    /// ```
    ///
    /// [`MakeService`]: tower::make::MakeService
    /// [`Router::into_make_service_with_connect_info`]: crate::routing::Router::into_make_service_with_connect_info
    #[cfg(feature = "tokio")]
    pub fn into_make_service_with_connect_info<C>(
        self,
    ) -> IntoMakeServiceWithConnectInfo<HandlerService<H, T, S, B>, C> {
        IntoMakeServiceWithConnectInfo::new(self)
    }
}

#[test]
fn traits() {
    use crate::test_helpers::*;
    assert_send::<HandlerService<(), NotSendSync, (), NotSendSync>>();
    assert_sync::<HandlerService<(), NotSendSync, (), NotSendSync>>();
}

impl<H, T, S, B> HandlerService<H, T, S, B> {
    pub(super) fn new(handler: H, state: S) -> Self {
        Self {
            handler,
            state,
            _marker: PhantomData,
        }
    }
}

impl<H, T, S, B> fmt::Debug for HandlerService<H, T, S, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntoService").finish_non_exhaustive()
    }
}

impl<H, T, S, B> Clone for HandlerService<H, T, S, B>
where
    H: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            state: self.state.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, S, B> Service<Request<B>> for HandlerService<H, T, S, B>
where
    H: Handler<T, S, B> + Clone + Send + 'static,
    B: Send + 'static,
    S: Clone + Send + Sync,
{
    type Response = Response;
    type Error = Infallible;
    type Future = super::future::IntoServiceFuture<H::Future>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // `IntoService` can only be constructed from async functions which are always ready, or
        // from `Layered` which buffers in `<Layered as Handler>::call` and is therefore
        // also always ready.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        use futures_util::future::FutureExt;

        let handler = self.handler.clone();
        let future = Handler::call(handler, req, self.state.clone());
        let future = future.map(Ok as _);

        super::future::IntoServiceFuture::new(future)
    }
}
