use std::{convert::Infallible, fmt};

use http::Request;
use tower::Service;

use crate::{
    body::HttpBody,
    handler::Handler,
    routing::{future::RouteFuture, Route},
    Router,
};

pub(crate) struct BoxedIntoRoute<S, B, E>(Box<dyn ErasedIntoRoute<S, B, E>>);

impl<S, B> BoxedIntoRoute<S, B, Infallible>
where
    S: Clone + Send + Sync + 'static,
    B: Send + 'static,
{
    pub(crate) fn from_handler<H, T>(handler: H) -> Self
    where
        H: Handler<T, S, B>,
        T: 'static,
        B: HttpBody,
    {
        Self(Box::new(MakeErasedHandler {
            handler,
            into_route: |handler, state| Route::new(Handler::with_state(handler, state)),
        }))
    }
}

impl<S, B, E> BoxedIntoRoute<S, B, E> {
    pub(crate) fn map<F, B2, E2>(self, f: F) -> BoxedIntoRoute<S, B2, E2>
    where
        S: 'static,
        B: 'static,
        E: 'static,
        F: FnOnce(Route<B, E>) -> Route<B2, E2> + Clone + Send + 'static,
        B2: HttpBody + 'static,
        E2: 'static,
    {
        BoxedIntoRoute(Box::new(Map {
            inner: self.0,
            layer: Box::new(f),
        }))
    }

    pub(crate) fn into_route(self, state: S) -> Route<B, E> {
        self.0.into_route(state)
    }
}

impl<S, B, E> Clone for BoxedIntoRoute<S, B, E> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

impl<S, B, E> fmt::Debug for BoxedIntoRoute<S, B, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BoxedIntoRoute").finish()
    }
}

pub(crate) trait ErasedIntoRoute<S, B, E>: Send {
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<S, B, E>>;

    fn into_route(self: Box<Self>, state: S) -> Route<B, E>;

    fn call_with_state(self: Box<Self>, request: Request<B>, state: S) -> RouteFuture<B, E>;
}

pub(crate) struct MakeErasedHandler<H, S, B> {
    pub(crate) handler: H,
    pub(crate) into_route: fn(H, S) -> Route<B>,
}

impl<H, S, B> ErasedIntoRoute<S, B, Infallible> for MakeErasedHandler<H, S, B>
where
    H: Clone + Send + 'static,
    S: 'static,
    B: HttpBody + 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<S, B, Infallible>> {
        Box::new(self.clone())
    }

    fn into_route(self: Box<Self>, state: S) -> Route<B> {
        (self.into_route)(self.handler, state)
    }

    fn call_with_state(
        self: Box<Self>,
        request: Request<B>,
        state: S,
    ) -> RouteFuture<B, Infallible> {
        self.into_route(state).call(request)
    }
}

impl<H, S, B> Clone for MakeErasedHandler<H, S, B>
where
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            into_route: self.into_route,
        }
    }
}

pub(crate) struct MakeErasedRouter<S, B> {
    pub(crate) router: Router<S, B>,
    pub(crate) into_route: fn(Router<S, B>, S) -> Route<B>,
}

impl<S, B> ErasedIntoRoute<S, B, Infallible> for MakeErasedRouter<S, B>
where
    S: Clone + Send + Sync + 'static,
    B: HttpBody + Send + 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<S, B, Infallible>> {
        Box::new(self.clone())
    }

    fn into_route(self: Box<Self>, state: S) -> Route<B> {
        (self.into_route)(self.router, state)
    }

    fn call_with_state(
        mut self: Box<Self>,
        request: Request<B>,
        state: S,
    ) -> RouteFuture<B, Infallible> {
        self.router.call_with_state(request, state)
    }
}

impl<S, B> Clone for MakeErasedRouter<S, B>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            router: self.router.clone(),
            into_route: self.into_route,
        }
    }
}

pub(crate) struct Map<S, B, E, B2, E2> {
    pub(crate) inner: Box<dyn ErasedIntoRoute<S, B, E>>,
    pub(crate) layer: Box<dyn LayerFn<B, E, B2, E2>>,
}

impl<S, B, E, B2, E2> ErasedIntoRoute<S, B2, E2> for Map<S, B, E, B2, E2>
where
    S: 'static,
    B: 'static,
    E: 'static,
    B2: HttpBody + 'static,
    E2: 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<S, B2, E2>> {
        Box::new(Self {
            inner: self.inner.clone_box(),
            layer: self.layer.clone_box(),
        })
    }

    fn into_route(self: Box<Self>, state: S) -> Route<B2, E2> {
        (self.layer)(self.inner.into_route(state))
    }

    fn call_with_state(self: Box<Self>, request: Request<B2>, state: S) -> RouteFuture<B2, E2> {
        (self.layer)(self.inner.into_route(state)).call(request)
    }
}

pub(crate) trait LayerFn<B, E, B2, E2>: FnOnce(Route<B, E>) -> Route<B2, E2> + Send {
    fn clone_box(&self) -> Box<dyn LayerFn<B, E, B2, E2>>;
}

impl<F, B, E, B2, E2> LayerFn<B, E, B2, E2> for F
where
    F: FnOnce(Route<B, E>) -> Route<B2, E2> + Clone + Send + 'static,
{
    fn clone_box(&self) -> Box<dyn LayerFn<B, E, B2, E2>> {
        Box::new(self.clone())
    }
}
