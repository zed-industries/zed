use crate::{extract::rejection::*, response::IntoResponseParts};
use async_trait::async_trait;
use axum_core::{
    extract::FromRequestParts,
    response::{IntoResponse, Response, ResponseParts},
};
use http::{request::Parts, Request};
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tower_service::Service;

/// Extractor and response for extensions.
///
/// # As extractor
///
/// This is commonly used to share state across handlers.
///
/// ```rust,no_run
/// use axum::{
///     Router,
///     Extension,
///     routing::get,
/// };
/// use std::sync::Arc;
///
/// // Some shared state used throughout our application
/// struct State {
///     // ...
/// }
///
/// async fn handler(state: Extension<Arc<State>>) {
///     // ...
/// }
///
/// let state = Arc::new(State { /* ... */ });
///
/// let app = Router::new().route("/", get(handler))
///     // Add middleware that inserts the state into all incoming request's
///     // extensions.
///     .layer(Extension(state));
/// # async {
/// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
/// # };
/// ```
///
/// If the extension is missing it will reject the request with a `500 Internal
/// Server Error` response.
///
/// # As response
///
/// Response extensions can be used to share state with middleware.
///
/// ```rust
/// use axum::{
///     Extension,
///     response::IntoResponse,
/// };
///
/// async fn handler() -> (Extension<Foo>, &'static str) {
///     (
///         Extension(Foo("foo")),
///         "Hello, World!"
///     )
/// }
///
/// #[derive(Clone)]
/// struct Foo(&'static str);
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct Extension<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Extension<T>
where
    T: Clone + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = ExtensionRejection;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let value = req
            .extensions
            .get::<T>()
            .ok_or_else(|| {
                MissingExtension::from_err(format!(
                    "Extension of type `{}` was not found. Perhaps you forgot to add it? See `axum::Extension`.",
                    std::any::type_name::<T>()
                ))
            })
            .map(|x| x.clone())?;

        Ok(Extension(value))
    }
}

axum_core::__impl_deref!(Extension);

impl<T> IntoResponseParts for Extension<T>
where
    T: Send + Sync + 'static,
{
    type Error = Infallible;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.extensions_mut().insert(self.0);
        Ok(res)
    }
}

impl<T> IntoResponse for Extension<T>
where
    T: Send + Sync + 'static,
{
    fn into_response(self) -> Response {
        let mut res = ().into_response();
        res.extensions_mut().insert(self.0);
        res
    }
}

impl<S, T> tower_layer::Layer<S> for Extension<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Service = AddExtension<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        AddExtension {
            inner,
            value: self.0.clone(),
        }
    }
}

/// Middleware for adding some shareable value to [request extensions].
///
/// See [Sharing state with handlers](index.html#sharing-state-with-handlers)
/// for more details.
///
/// [request extensions]: https://docs.rs/http/latest/http/struct.Extensions.html
#[derive(Clone, Copy, Debug)]
pub struct AddExtension<S, T> {
    pub(crate) inner: S,
    pub(crate) value: T,
}

impl<ResBody, S, T> Service<Request<ResBody>> for AddExtension<S, T>
where
    S: Service<Request<ResBody>>,
    T: Clone + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ResBody>) -> Self::Future {
        req.extensions_mut().insert(self.value.clone());
        self.inner.call(req)
    }
}
