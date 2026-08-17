//! Routing between [`Service`]s and handlers.

use self::{future::RouteFuture, not_found::NotFound, path_router::PathRouter};
#[cfg(feature = "tokio")]
use crate::extract::connect_info::IntoMakeServiceWithConnectInfo;
use crate::{
    body::{Body, HttpBody},
    boxed::BoxedIntoRoute,
    handler::Handler,
    util::try_downcast,
};
use axum_core::response::{IntoResponse, Response};
use http::Request;
use std::{
    convert::Infallible,
    fmt,
    task::{Context, Poll},
};
use sync_wrapper::SyncWrapper;
use tower_layer::Layer;
use tower_service::Service;

pub mod future;
pub mod method_routing;

mod into_make_service;
mod method_filter;
mod not_found;
pub(crate) mod path_router;
mod route;
mod strip_prefix;
pub(crate) mod url_params;

#[cfg(test)]
mod tests;

pub use self::{into_make_service::IntoMakeService, method_filter::MethodFilter, route::Route};

pub use self::method_routing::{
    any, any_service, delete, delete_service, get, get_service, head, head_service, on, on_service,
    options, options_service, patch, patch_service, post, post_service, put, put_service, trace,
    trace_service, MethodRouter,
};

macro_rules! panic_on_err {
    ($expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(err) => panic!("{err}"),
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RouteId(u32);

/// The router type for composing handlers and services.
#[must_use]
pub struct Router<S = (), B = Body> {
    path_router: PathRouter<S, B, false>,
    fallback_router: PathRouter<S, B, true>,
    default_fallback: bool,
    catch_all_fallback: Fallback<S, B>,
}

impl<S, B> Clone for Router<S, B> {
    fn clone(&self) -> Self {
        Self {
            path_router: self.path_router.clone(),
            fallback_router: self.fallback_router.clone(),
            default_fallback: self.default_fallback,
            catch_all_fallback: self.catch_all_fallback.clone(),
        }
    }
}

impl<S, B> Default for Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> fmt::Debug for Router<S, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("path_router", &self.path_router)
            .field("fallback_router", &self.fallback_router)
            .field("default_fallback", &self.default_fallback)
            .field("catch_all_fallback", &self.catch_all_fallback)
            .finish()
    }
}

pub(crate) const NEST_TAIL_PARAM: &str = "__private__axum_nest_tail_param";
pub(crate) const NEST_TAIL_PARAM_CAPTURE: &str = "/*__private__axum_nest_tail_param";
pub(crate) const FALLBACK_PARAM: &str = "__private__axum_fallback";
pub(crate) const FALLBACK_PARAM_PATH: &str = "/*__private__axum_fallback";

impl<S, B> Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    /// Create a new `Router`.
    ///
    /// Unless you add additional routes this will respond with `404 Not Found` to
    /// all requests.
    pub fn new() -> Self {
        Self {
            path_router: Default::default(),
            fallback_router: PathRouter::new_fallback(),
            default_fallback: true,
            catch_all_fallback: Fallback::Default(Route::new(NotFound)),
        }
    }

    #[doc = include_str!("../docs/routing/route.md")]
    #[track_caller]
    pub fn route(mut self, path: &str, method_router: MethodRouter<S, B>) -> Self {
        panic_on_err!(self.path_router.route(path, method_router));
        self
    }

    #[doc = include_str!("../docs/routing/route_service.md")]
    pub fn route_service<T>(mut self, path: &str, service: T) -> Self
    where
        T: Service<Request<B>, Error = Infallible> + Clone + Send + 'static,
        T::Response: IntoResponse,
        T::Future: Send + 'static,
    {
        let service = match try_downcast::<Router<S, B>, _>(service) {
            Ok(_) => {
                panic!(
                    "Invalid route: `Router::route_service` cannot be used with `Router`s. \
                     Use `Router::nest` instead"
                );
            }
            Err(service) => service,
        };

        panic_on_err!(self.path_router.route_service(path, service));
        self
    }

    #[doc = include_str!("../docs/routing/nest.md")]
    #[track_caller]
    pub fn nest(mut self, path: &str, router: Router<S, B>) -> Self {
        let Router {
            path_router,
            fallback_router,
            default_fallback,
            // we don't need to inherit the catch-all fallback. It is only used for CONNECT
            // requests with an empty path. If we were to inherit the catch-all fallback
            // it would end up matching `/{path}/*` which doesn't match empty paths.
            catch_all_fallback: _,
        } = router;

        panic_on_err!(self.path_router.nest(path, path_router));

        if !default_fallback {
            panic_on_err!(self.fallback_router.nest(path, fallback_router));
        }

        self
    }

    /// Like [`nest`](Self::nest), but accepts an arbitrary `Service`.
    #[track_caller]
    pub fn nest_service<T>(mut self, path: &str, service: T) -> Self
    where
        T: Service<Request<B>, Error = Infallible> + Clone + Send + 'static,
        T::Response: IntoResponse,
        T::Future: Send + 'static,
    {
        panic_on_err!(self.path_router.nest_service(path, service));
        self
    }

    #[doc = include_str!("../docs/routing/merge.md")]
    #[track_caller]
    pub fn merge<R>(mut self, other: R) -> Self
    where
        R: Into<Router<S, B>>,
    {
        const PANIC_MSG: &str =
            "Failed to merge fallbacks. This is a bug in axum. Please file an issue";

        let Router {
            path_router,
            fallback_router: mut other_fallback,
            default_fallback,
            catch_all_fallback,
        } = other.into();

        panic_on_err!(self.path_router.merge(path_router));

        match (self.default_fallback, default_fallback) {
            // both have the default fallback
            // use the one from other
            (true, true) => {
                self.fallback_router.merge(other_fallback).expect(PANIC_MSG);
            }
            // self has default fallback, other has a custom fallback
            (true, false) => {
                self.fallback_router.merge(other_fallback).expect(PANIC_MSG);
                self.default_fallback = false;
            }
            // self has a custom fallback, other has a default
            (false, true) => {
                let fallback_router = std::mem::take(&mut self.fallback_router);
                other_fallback.merge(fallback_router).expect(PANIC_MSG);
                self.fallback_router = other_fallback;
            }
            // both have a custom fallback, not allowed
            (false, false) => {
                panic!("Cannot merge two `Router`s that both have a fallback")
            }
        };

        self.catch_all_fallback = self
            .catch_all_fallback
            .merge(catch_all_fallback)
            .unwrap_or_else(|| panic!("Cannot merge two `Router`s that both have a fallback"));

        self
    }

    #[doc = include_str!("../docs/routing/layer.md")]
    pub fn layer<L, NewReqBody>(self, layer: L) -> Router<S, NewReqBody>
    where
        L: Layer<Route<B>> + Clone + Send + 'static,
        L::Service: Service<Request<NewReqBody>> + Clone + Send + 'static,
        <L::Service as Service<Request<NewReqBody>>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request<NewReqBody>>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request<NewReqBody>>>::Future: Send + 'static,
        NewReqBody: HttpBody + 'static,
    {
        Router {
            path_router: self.path_router.layer(layer.clone()),
            fallback_router: self.fallback_router.layer(layer.clone()),
            default_fallback: self.default_fallback,
            catch_all_fallback: self.catch_all_fallback.map(|route| route.layer(layer)),
        }
    }

    #[doc = include_str!("../docs/routing/route_layer.md")]
    #[track_caller]
    pub fn route_layer<L>(self, layer: L) -> Self
    where
        L: Layer<Route<B>> + Clone + Send + 'static,
        L::Service: Service<Request<B>> + Clone + Send + 'static,
        <L::Service as Service<Request<B>>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request<B>>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request<B>>>::Future: Send + 'static,
    {
        Router {
            path_router: self.path_router.route_layer(layer),
            fallback_router: self.fallback_router,
            default_fallback: self.default_fallback,
            catch_all_fallback: self.catch_all_fallback,
        }
    }

    #[track_caller]
    #[doc = include_str!("../docs/routing/fallback.md")]
    pub fn fallback<H, T>(mut self, handler: H) -> Self
    where
        H: Handler<T, S, B>,
        T: 'static,
    {
        self.catch_all_fallback =
            Fallback::BoxedHandler(BoxedIntoRoute::from_handler(handler.clone()));
        self.fallback_endpoint(Endpoint::MethodRouter(any(handler)))
    }

    /// Add a fallback [`Service`] to the router.
    ///
    /// See [`Router::fallback`] for more details.
    pub fn fallback_service<T>(mut self, service: T) -> Self
    where
        T: Service<Request<B>, Error = Infallible> + Clone + Send + 'static,
        T::Response: IntoResponse,
        T::Future: Send + 'static,
    {
        let route = Route::new(service);
        self.catch_all_fallback = Fallback::Service(route.clone());
        self.fallback_endpoint(Endpoint::Route(route))
    }

    fn fallback_endpoint(mut self, endpoint: Endpoint<S, B>) -> Self {
        self.fallback_router.set_fallback(endpoint);
        self.default_fallback = false;
        self
    }

    #[doc = include_str!("../docs/routing/with_state.md")]
    pub fn with_state<S2>(self, state: S) -> Router<S2, B> {
        Router {
            path_router: self.path_router.with_state(state.clone()),
            fallback_router: self.fallback_router.with_state(state.clone()),
            default_fallback: self.default_fallback,
            catch_all_fallback: self.catch_all_fallback.with_state(state),
        }
    }

    pub(crate) fn call_with_state(
        &mut self,
        mut req: Request<B>,
        state: S,
    ) -> RouteFuture<B, Infallible> {
        // required for opaque routers to still inherit the fallback
        // TODO(david): remove this feature in 0.7
        if !self.default_fallback {
            req.extensions_mut().insert(SuperFallback(SyncWrapper::new(
                self.fallback_router.clone(),
            )));
        }

        match self.path_router.call_with_state(req, state) {
            Ok(future) => future,
            Err((mut req, state)) => {
                let super_fallback = req
                    .extensions_mut()
                    .remove::<SuperFallback<S, B>>()
                    .map(|SuperFallback(path_router)| path_router.into_inner());

                if let Some(mut super_fallback) = super_fallback {
                    match super_fallback.call_with_state(req, state) {
                        Ok(future) => return future,
                        Err((req, state)) => {
                            return self.catch_all_fallback.call_with_state(req, state);
                        }
                    }
                }

                match self.fallback_router.call_with_state(req, state) {
                    Ok(future) => future,
                    Err((req, state)) => self.catch_all_fallback.call_with_state(req, state),
                }
            }
        }
    }
}

impl<B> Router<(), B>
where
    B: HttpBody + Send + 'static,
{
    /// Convert this router into a [`MakeService`], that is a [`Service`] whose
    /// response is another service.
    ///
    /// This is useful when running your application with hyper's
    /// [`Server`](hyper::server::Server):
    ///
    /// ```
    /// use axum::{
    ///     routing::get,
    ///     Router,
    /// };
    ///
    /// let app = Router::new().route("/", get(|| async { "Hi!" }));
    ///
    /// # async {
    /// axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    ///     .serve(app.into_make_service())
    ///     .await
    ///     .expect("server failed");
    /// # };
    /// ```
    ///
    /// [`MakeService`]: tower::make::MakeService
    pub fn into_make_service(self) -> IntoMakeService<Self> {
        // call `Router::with_state` such that everything is turned into `Route` eagerly
        // rather than doing that per request
        IntoMakeService::new(self.with_state(()))
    }

    #[doc = include_str!("../docs/routing/into_make_service_with_connect_info.md")]
    #[cfg(feature = "tokio")]
    pub fn into_make_service_with_connect_info<C>(self) -> IntoMakeServiceWithConnectInfo<Self, C> {
        // call `Router::with_state` such that everything is turned into `Route` eagerly
        // rather than doing that per request
        IntoMakeServiceWithConnectInfo::new(self.with_state(()))
    }
}

impl<B> Service<Request<B>> for Router<(), B>
where
    B: HttpBody + Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future = RouteFuture<B, Infallible>;

    #[inline]
    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: Request<B>) -> Self::Future {
        self.call_with_state(req, ())
    }
}

enum Fallback<S, B, E = Infallible> {
    Default(Route<B, E>),
    Service(Route<B, E>),
    BoxedHandler(BoxedIntoRoute<S, B, E>),
}

impl<S, B, E> Fallback<S, B, E>
where
    S: Clone,
{
    fn merge(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Self::Default(_), pick @ Self::Default(_)) => Some(pick),
            (Self::Default(_), pick) | (pick, Self::Default(_)) => Some(pick),
            _ => None,
        }
    }

    fn map<F, B2, E2>(self, f: F) -> Fallback<S, B2, E2>
    where
        S: 'static,
        B: 'static,
        E: 'static,
        F: FnOnce(Route<B, E>) -> Route<B2, E2> + Clone + Send + 'static,
        B2: HttpBody + 'static,
        E2: 'static,
    {
        match self {
            Self::Default(route) => Fallback::Default(f(route)),
            Self::Service(route) => Fallback::Service(f(route)),
            Self::BoxedHandler(handler) => Fallback::BoxedHandler(handler.map(f)),
        }
    }

    fn with_state<S2>(self, state: S) -> Fallback<S2, B, E> {
        match self {
            Fallback::Default(route) => Fallback::Default(route),
            Fallback::Service(route) => Fallback::Service(route),
            Fallback::BoxedHandler(handler) => Fallback::Service(handler.into_route(state)),
        }
    }

    fn call_with_state(&mut self, req: Request<B>, state: S) -> RouteFuture<B, E> {
        match self {
            Fallback::Default(route) | Fallback::Service(route) => {
                RouteFuture::from_future(route.oneshot_inner(req))
            }
            Fallback::BoxedHandler(handler) => {
                let mut route = handler.clone().into_route(state);
                RouteFuture::from_future(route.oneshot_inner(req))
            }
        }
    }
}

impl<S, B, E> Clone for Fallback<S, B, E> {
    fn clone(&self) -> Self {
        match self {
            Self::Default(inner) => Self::Default(inner.clone()),
            Self::Service(inner) => Self::Service(inner.clone()),
            Self::BoxedHandler(inner) => Self::BoxedHandler(inner.clone()),
        }
    }
}

impl<S, B, E> fmt::Debug for Fallback<S, B, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default(inner) => f.debug_tuple("Default").field(inner).finish(),
            Self::Service(inner) => f.debug_tuple("Service").field(inner).finish(),
            Self::BoxedHandler(_) => f.debug_tuple("BoxedHandler").finish(),
        }
    }
}

#[allow(clippy::large_enum_variant)]
enum Endpoint<S, B> {
    MethodRouter(MethodRouter<S, B>),
    Route(Route<B>),
}

impl<S, B> Endpoint<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    fn layer<L, NewReqBody>(self, layer: L) -> Endpoint<S, NewReqBody>
    where
        L: Layer<Route<B>> + Clone + Send + 'static,
        L::Service: Service<Request<NewReqBody>> + Clone + Send + 'static,
        <L::Service as Service<Request<NewReqBody>>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request<NewReqBody>>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request<NewReqBody>>>::Future: Send + 'static,
        NewReqBody: HttpBody + 'static,
    {
        match self {
            Endpoint::MethodRouter(method_router) => {
                Endpoint::MethodRouter(method_router.layer(layer))
            }
            Endpoint::Route(route) => Endpoint::Route(route.layer(layer)),
        }
    }
}

impl<S, B> Clone for Endpoint<S, B> {
    fn clone(&self) -> Self {
        match self {
            Self::MethodRouter(inner) => Self::MethodRouter(inner.clone()),
            Self::Route(inner) => Self::Route(inner.clone()),
        }
    }
}

impl<S, B> fmt::Debug for Endpoint<S, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MethodRouter(method_router) => {
                f.debug_tuple("MethodRouter").field(method_router).finish()
            }
            Self::Route(route) => f.debug_tuple("Route").field(route).finish(),
        }
    }
}

struct SuperFallback<S, B>(SyncWrapper<PathRouter<S, B, true>>);

#[test]
#[allow(warnings)]
fn traits() {
    use crate::test_helpers::*;
    assert_send::<Router<(), ()>>();
}
