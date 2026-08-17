use crate::ServiceExt;
use tower_layer::{layer_fn, LayerFn};
use tower_service::Service;

use std::fmt;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A boxed `Service + Send` trait object.
///
/// [`BoxService`] turns a service into a trait object, allowing the response
/// future type to be dynamic. This type requires both the service and the
/// response future to be [`Send`].
///
/// If you need a boxed [`Service`] that implements [`Clone`] consider using
/// [`BoxCloneService`](crate::util::BoxCloneService).
///
/// See module level documentation for more details.
pub struct BoxService<T, U, E> {
    inner: Box<dyn Service<T, Response = U, Error = E, Future = BoxFuture<U, E>> + Send>,
}

/// A boxed `Future + Send` trait object.
///
/// This type alias represents a boxed future that is [`Send`] and can be moved
/// across threads.
type BoxFuture<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;

impl<T, U, E> BoxService<T, U, E> {
    #[allow(missing_docs)]
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<T, Response = U, Error = E> + Send + 'static,
        S::Future: Send + 'static,
    {
        let inner = Box::new(inner.map_future(|f: S::Future| Box::pin(f) as _));
        BoxService { inner }
    }

    /// Returns a [`Layer`] for wrapping a [`Service`] in a [`BoxService`]
    /// middleware.
    ///
    /// [`Layer`]: crate::Layer
    pub fn layer<S>() -> LayerFn<fn(S) -> Self>
    where
        S: Service<T, Response = U, Error = E> + Send + 'static,
        S::Future: Send + 'static,
    {
        layer_fn(Self::new)
    }
}

impl<T, U, E> Service<T> for BoxService<T, U, E> {
    type Response = U;
    type Error = E;
    type Future = BoxFuture<U, E>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), E>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: T) -> BoxFuture<U, E> {
        self.inner.call(request)
    }
}

impl<T, U, E> fmt::Debug for BoxService<T, U, E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BoxService").finish()
    }
}
