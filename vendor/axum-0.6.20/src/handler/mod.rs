//! Async functions that can be used to handle requests.
//!
#![doc = include_str!("../docs/handlers_intro.md")]
//!
//! Some examples of handlers:
//!
//! ```rust
//! use axum::{body::Bytes, http::StatusCode};
//!
//! // Handler that immediately returns an empty `200 OK` response.
//! async fn unit_handler() {}
//!
//! // Handler that immediately returns an empty `200 OK` response with a plain
//! // text body.
//! async fn string_handler() -> String {
//!     "Hello, World!".to_string()
//! }
//!
//! // Handler that buffers the request body and returns it.
//! //
//! // This works because `Bytes` implements `FromRequest`
//! // and therefore can be used as an extractor.
//! //
//! // `String` and `StatusCode` both implement `IntoResponse` and
//! // therefore `Result<String, StatusCode>` also implements `IntoResponse`
//! async fn echo(body: Bytes) -> Result<String, StatusCode> {
//!     if let Ok(string) = String::from_utf8(body.to_vec()) {
//!         Ok(string)
//!     } else {
//!         Err(StatusCode::BAD_REQUEST)
//!     }
//! }
//! ```
//!
//! Instead of a direct `StatusCode`, it makes sense to use intermediate error type
//! that can ultimately be converted to `Response`. This allows using `?` operator
//! in handlers. See those examples:
//!
//! * [`anyhow-error-response`][anyhow] for generic boxed errors
//! * [`error-handling-and-dependency-injection`][ehdi] for application-specific detailed errors
//!
//! [anyhow]: https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs
//! [ehdi]: https://github.com/tokio-rs/axum/blob/main/examples/error-handling-and-dependency-injection/src/main.rs
//!
#![doc = include_str!("../docs/debugging_handler_type_errors.md")]

#[cfg(feature = "tokio")]
use crate::extract::connect_info::IntoMakeServiceWithConnectInfo;
use crate::{
    body::Body,
    extract::{FromRequest, FromRequestParts},
    response::{IntoResponse, Response},
    routing::IntoMakeService,
};
use http::Request;
use std::{convert::Infallible, fmt, future::Future, marker::PhantomData, pin::Pin};
use tower::ServiceExt;
use tower_layer::Layer;
use tower_service::Service;

pub mod future;
mod service;

pub use self::service::HandlerService;

/// Trait for async functions that can be used to handle requests.
///
/// You shouldn't need to depend on this trait directly. It is automatically
/// implemented to closures of the right types.
///
/// See the [module docs](crate::handler) for more details.
///
/// # Converting `Handler`s into [`Service`]s
///
/// To convert `Handler`s into [`Service`]s you have to call either
/// [`HandlerWithoutStateExt::into_service`] or [`Handler::with_state`]:
///
/// ```
/// use tower::Service;
/// use axum::{
///     extract::State,
///     body::Body,
///     http::Request,
///     handler::{HandlerWithoutStateExt, Handler},
/// };
///
/// // this handler doesn't require any state
/// async fn one() {}
/// // so it can be converted to a service with `HandlerWithoutStateExt::into_service`
/// assert_service(one.into_service());
///
/// // this handler requires state
/// async fn two(_: State<String>) {}
/// // so we have to provide it
/// let handler_with_state = two.with_state(String::new());
/// // which gives us a `Service`
/// assert_service(handler_with_state);
///
/// // helper to check that a value implements `Service`
/// fn assert_service<S>(service: S)
/// where
///     S: Service<Request<Body>>,
/// {}
/// ```
#[doc = include_str!("../docs/debugging_handler_type_errors.md")]
///
/// # Handlers that aren't functions
///
/// The `Handler` trait is also implemented for `T: IntoResponse`. That allows easily returning
/// fixed data for routes:
///
/// ```
/// use axum::{
///     Router,
///     routing::{get, post},
///     Json,
///     http::StatusCode,
/// };
/// use serde_json::json;
///
/// let app = Router::new()
///     // respond with a fixed string
///     .route("/", get("Hello, World!"))
///     // or return some mock data
///     .route("/users", post((
///         StatusCode::CREATED,
///         Json(json!({ "id": 1, "username": "alice" })),
///     )));
/// # let _: Router = app;
/// ```
#[cfg_attr(
    nightly_error_messages,
    rustc_on_unimplemented(
        note = "Consider using `#[axum::debug_handler]` to improve the error message"
    )
)]
pub trait Handler<T, S, B = Body>: Clone + Send + Sized + 'static {
    /// The type of future calling this handler returns.
    type Future: Future<Output = Response> + Send + 'static;

    /// Call the handler with the given request.
    fn call(self, req: Request<B>, state: S) -> Self::Future;

    /// Apply a [`tower::Layer`] to the handler.
    ///
    /// All requests to the handler will be processed by the layer's
    /// corresponding middleware.
    ///
    /// This can be used to add additional processing to a request for a single
    /// handler.
    ///
    /// Note this differs from [`routing::Router::layer`](crate::routing::Router::layer)
    /// which adds a middleware to a group of routes.
    ///
    /// If you're applying middleware that produces errors you have to handle the errors
    /// so they're converted into responses. You can learn more about doing that
    /// [here](crate::error_handling).
    ///
    /// # Example
    ///
    /// Adding the [`tower::limit::ConcurrencyLimit`] middleware to a handler
    /// can be done like so:
    ///
    /// ```rust
    /// use axum::{
    ///     routing::get,
    ///     handler::Handler,
    ///     Router,
    /// };
    /// use tower::limit::{ConcurrencyLimitLayer, ConcurrencyLimit};
    ///
    /// async fn handler() { /* ... */ }
    ///
    /// let layered_handler = handler.layer(ConcurrencyLimitLayer::new(64));
    /// let app = Router::new().route("/", get(layered_handler));
    /// # async {
    /// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    /// # };
    /// ```
    fn layer<L, NewReqBody>(self, layer: L) -> Layered<L, Self, T, S, B, NewReqBody>
    where
        L: Layer<HandlerService<Self, T, S, B>> + Clone,
        L::Service: Service<Request<NewReqBody>>,
    {
        Layered {
            layer,
            handler: self,
            _marker: PhantomData,
        }
    }

    /// Convert the handler into a [`Service`] by providing the state
    fn with_state(self, state: S) -> HandlerService<Self, T, S, B> {
        HandlerService::new(self, state)
    }
}

impl<F, Fut, Res, S, B> Handler<((),), S, B> for F
where
    F: FnOnce() -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    B: Send + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, _req: Request<B>, _state: S) -> Self::Future {
        Box::pin(async move { self().await.into_response() })
    }
}

macro_rules! impl_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, Fut, S, B, Res, M, $($ty,)* $last> Handler<(M, $($ty,)* $last,), S, B> for F
        where
            F: FnOnce($($ty,)* $last,) -> Fut + Clone + Send + 'static,
            Fut: Future<Output = Res> + Send,
            B: Send + 'static,
            S: Send + Sync + 'static,
            Res: IntoResponse,
            $( $ty: FromRequestParts<S> + Send, )*
            $last: FromRequest<S, B, M> + Send,
        {
            type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

            fn call(self, req: Request<B>, state: S) -> Self::Future {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*

                    let req = Request::from_parts(parts, body);

                    let $last = match $last::from_request(req, state).await {
                        Ok(value) => value,
                        Err(rejection) => return rejection.into_response(),
                    };

                    let res = self($($ty,)* $last,).await;

                    res.into_response()
                })
            }
        }
    };
}

all_the_tuples!(impl_handler);

mod private {
    // Marker type for `impl<T: IntoResponse> Handler for T`
    #[allow(missing_debug_implementations)]
    pub enum IntoResponseHandler {}
}

impl<T, S, B> Handler<private::IntoResponseHandler, S, B> for T
where
    T: IntoResponse + Clone + Send + 'static,
    B: Send + 'static,
{
    type Future = std::future::Ready<Response>;

    fn call(self, _req: Request<B>, _state: S) -> Self::Future {
        std::future::ready(self.into_response())
    }
}

/// A [`Service`] created from a [`Handler`] by applying a Tower middleware.
///
/// Created with [`Handler::layer`]. See that method for more details.
pub struct Layered<L, H, T, S, B, B2> {
    layer: L,
    handler: H,
    _marker: PhantomData<fn() -> (T, S, B, B2)>,
}

impl<L, H, T, S, B, B2> fmt::Debug for Layered<L, H, T, S, B, B2>
where
    L: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Layered")
            .field("layer", &self.layer)
            .finish()
    }
}

impl<L, H, T, S, B, B2> Clone for Layered<L, H, T, S, B, B2>
where
    L: Clone,
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            layer: self.layer.clone(),
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, S, T, L, B, B2> Handler<T, S, B2> for Layered<L, H, T, S, B, B2>
where
    L: Layer<HandlerService<H, T, S, B>> + Clone + Send + 'static,
    H: Handler<T, S, B>,
    L::Service: Service<Request<B2>, Error = Infallible> + Clone + Send + 'static,
    <L::Service as Service<Request<B2>>>::Response: IntoResponse,
    <L::Service as Service<Request<B2>>>::Future: Send,
    T: 'static,
    S: 'static,
    B: Send + 'static,
    B2: Send + 'static,
{
    type Future = future::LayeredFuture<B2, L::Service>;

    fn call(self, req: Request<B2>, state: S) -> Self::Future {
        use futures_util::future::{FutureExt, Map};

        let svc = self.handler.with_state(state);
        let svc = self.layer.layer(svc);

        let future: Map<
            _,
            fn(
                Result<
                    <L::Service as Service<Request<B2>>>::Response,
                    <L::Service as Service<Request<B2>>>::Error,
                >,
            ) -> _,
        > = svc.oneshot(req).map(|result| match result {
            Ok(res) => res.into_response(),
            Err(err) => match err {},
        });

        future::LayeredFuture::new(future)
    }
}

/// Extension trait for [`Handler`]s that don't have state.
///
/// This provides convenience methods to convert the [`Handler`] into a [`Service`] or [`MakeService`].
///
/// [`MakeService`]: tower::make::MakeService
pub trait HandlerWithoutStateExt<T, B>: Handler<T, (), B> {
    /// Convert the handler into a [`Service`] and no state.
    fn into_service(self) -> HandlerService<Self, T, (), B>;

    /// Convert the handler into a [`MakeService`] and no state.
    ///
    /// See [`HandlerService::into_make_service`] for more details.
    ///
    /// [`MakeService`]: tower::make::MakeService
    fn into_make_service(self) -> IntoMakeService<HandlerService<Self, T, (), B>>;

    /// Convert the handler into a [`MakeService`] which stores information
    /// about the incoming connection and has no state.
    ///
    /// See [`HandlerService::into_make_service_with_connect_info`] for more details.
    ///
    /// [`MakeService`]: tower::make::MakeService
    #[cfg(feature = "tokio")]
    fn into_make_service_with_connect_info<C>(
        self,
    ) -> IntoMakeServiceWithConnectInfo<HandlerService<Self, T, (), B>, C>;
}

impl<H, T, B> HandlerWithoutStateExt<T, B> for H
where
    H: Handler<T, (), B>,
{
    fn into_service(self) -> HandlerService<Self, T, (), B> {
        self.with_state(())
    }

    fn into_make_service(self) -> IntoMakeService<HandlerService<Self, T, (), B>> {
        self.into_service().into_make_service()
    }

    #[cfg(feature = "tokio")]
    fn into_make_service_with_connect_info<C>(
        self,
    ) -> IntoMakeServiceWithConnectInfo<HandlerService<Self, T, (), B>, C> {
        self.into_service().into_make_service_with_connect_info()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{body, extract::State, test_helpers::*};
    use http::StatusCode;
    use std::time::Duration;
    use tower_http::{
        compression::CompressionLayer, limit::RequestBodyLimitLayer,
        map_request_body::MapRequestBodyLayer, map_response_body::MapResponseBodyLayer,
        timeout::TimeoutLayer,
    };

    #[crate::test]
    async fn handler_into_service() {
        async fn handle(body: String) -> impl IntoResponse {
            format!("you said: {body}")
        }

        let client = TestClient::new(handle.into_service());

        let res = client.post("/").body("hi there!").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "you said: hi there!");
    }

    #[crate::test]
    async fn with_layer_that_changes_request_body_and_state() {
        async fn handle(State(state): State<&'static str>) -> &'static str {
            state
        }

        let svc = handle
            .layer((
                RequestBodyLimitLayer::new(1024),
                TimeoutLayer::new(Duration::from_secs(10)),
                MapResponseBodyLayer::new(body::boxed),
                CompressionLayer::new(),
            ))
            .layer(MapRequestBodyLayer::new(body::boxed))
            .with_state("foo");

        let client = TestClient::new(svc);
        let res = client.get("/").send().await;
        assert_eq!(res.text().await, "foo");
    }
}
