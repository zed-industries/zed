use super::{rejection::*, FromRequestParts};
use crate::routing::{RouteId, NEST_TAIL_PARAM_CAPTURE};
use async_trait::async_trait;
use http::request::Parts;
use std::{collections::HashMap, sync::Arc};

/// Access the path in the router that matches the request.
///
/// ```
/// use axum::{
///     Router,
///     extract::MatchedPath,
///     routing::get,
/// };
///
/// let app = Router::new().route(
///     "/users/:id",
///     get(|path: MatchedPath| async move {
///         let path = path.as_str();
///         // `path` will be "/users/:id"
///     })
/// );
/// # async {
/// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
/// # };
/// ```
///
/// # Accessing `MatchedPath` via extensions
///
/// `MatchedPath` can also be accessed from middleware via request extensions.
///
/// This is useful for example with [`Trace`](tower_http::trace::Trace) to
/// create a span that contains the matched path:
///
/// ```
/// use axum::{
///     Router,
///     extract::MatchedPath,
///     http::Request,
///     routing::get,
/// };
/// use tower_http::trace::TraceLayer;
///
/// let app = Router::new()
///     .route("/users/:id", get(|| async { /* ... */ }))
///     .layer(
///         TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
///             let path = if let Some(path) = req.extensions().get::<MatchedPath>() {
///                 path.as_str()
///             } else {
///                 req.uri().path()
///             };
///             tracing::info_span!("http-request", %path)
///         }),
///     );
/// # let _: Router = app;
/// ```
///
/// # Matched path in nested routers
///
/// Because of how [nesting] works `MatchedPath` isn't accessible in middleware on nested routes:
///
/// ```
/// use axum::{
///     Router,
///     RequestExt,
///     routing::get,
///     extract::{MatchedPath, rejection::MatchedPathRejection},
///     middleware::map_request,
///     http::Request,
///     body::Body,
/// };
///
/// async fn access_matched_path(mut request: Request<Body>) -> Request<Body> {
///     // if `/foo/bar` is called this will be `Err(_)` since that matches
///     // a nested route
///     let matched_path: Result<MatchedPath, MatchedPathRejection> =
///         request.extract_parts::<MatchedPath>().await;
///
///     request
/// }
///
/// // `MatchedPath` is always accessible on handlers added via `Router::route`
/// async fn handler(matched_path: MatchedPath) {}
///
/// let app = Router::new()
///     .nest(
///         "/foo",
///         Router::new().route("/bar", get(handler)),
///     )
///     .layer(map_request(access_matched_path));
/// # let _: Router = app;
/// ```
///
/// [nesting]: crate::Router::nest
#[cfg_attr(docsrs, doc(cfg(feature = "matched-path")))]
#[derive(Clone, Debug)]
pub struct MatchedPath(pub(crate) Arc<str>);

impl MatchedPath {
    /// Returns a `str` representation of the path.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for MatchedPath
where
    S: Send + Sync,
{
    type Rejection = MatchedPathRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let matched_path = parts
            .extensions
            .get::<Self>()
            .ok_or(MatchedPathRejection::MatchedPathMissing(MatchedPathMissing))?
            .clone();

        Ok(matched_path)
    }
}

#[derive(Clone, Debug)]
struct MatchedNestedPath(Arc<str>);

pub(crate) fn set_matched_path_for_request(
    id: RouteId,
    route_id_to_path: &HashMap<RouteId, Arc<str>>,
    extensions: &mut http::Extensions,
) {
    let matched_path = if let Some(matched_path) = route_id_to_path.get(&id) {
        matched_path
    } else {
        #[cfg(debug_assertions)]
        panic!("should always have a matched path for a route id");
        #[cfg(not(debug_assertions))]
        return;
    };

    let matched_path = append_nested_matched_path(matched_path, extensions);

    if matched_path.ends_with(NEST_TAIL_PARAM_CAPTURE) {
        extensions.insert(MatchedNestedPath(matched_path));
        debug_assert!(extensions.remove::<MatchedPath>().is_none());
    } else {
        extensions.insert(MatchedPath(matched_path));
        extensions.remove::<MatchedNestedPath>();
    }
}

// a previous `MatchedPath` might exist if we're inside a nested Router
fn append_nested_matched_path(matched_path: &Arc<str>, extensions: &http::Extensions) -> Arc<str> {
    if let Some(previous) = extensions
        .get::<MatchedPath>()
        .map(|matched_path| matched_path.as_str())
        .or_else(|| Some(&extensions.get::<MatchedNestedPath>()?.0))
    {
        let previous = previous
            .strip_suffix(NEST_TAIL_PARAM_CAPTURE)
            .unwrap_or(previous);

        let matched_path = format!("{previous}{matched_path}");
        matched_path.into()
    } else {
        Arc::clone(matched_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        body::Body,
        handler::HandlerWithoutStateExt,
        middleware::map_request,
        routing::{any, get},
        test_helpers::*,
        Router,
    };
    use http::{Request, StatusCode};

    #[crate::test]
    async fn extracting_on_handler() {
        let app = Router::new().route(
            "/:a",
            get(|path: MatchedPath| async move { path.as_str().to_owned() }),
        );

        let client = TestClient::new(app);

        let res = client.get("/foo").send().await;
        assert_eq!(res.text().await, "/:a");
    }

    #[crate::test]
    async fn extracting_on_handler_in_nested_router() {
        let app = Router::new().nest(
            "/:a",
            Router::new().route(
                "/:b",
                get(|path: MatchedPath| async move { path.as_str().to_owned() }),
            ),
        );

        let client = TestClient::new(app);

        let res = client.get("/foo/bar").send().await;
        assert_eq!(res.text().await, "/:a/:b");
    }

    #[crate::test]
    async fn extracting_on_handler_in_deeply_nested_router() {
        let app = Router::new().nest(
            "/:a",
            Router::new().nest(
                "/:b",
                Router::new().route(
                    "/:c",
                    get(|path: MatchedPath| async move { path.as_str().to_owned() }),
                ),
            ),
        );

        let client = TestClient::new(app);

        let res = client.get("/foo/bar/baz").send().await;
        assert_eq!(res.text().await, "/:a/:b/:c");
    }

    #[crate::test]
    async fn cannot_extract_nested_matched_path_in_middleware() {
        async fn extract_matched_path<B>(
            matched_path: Option<MatchedPath>,
            req: Request<B>,
        ) -> Request<B> {
            assert!(matched_path.is_none());
            req
        }

        let app = Router::new()
            .nest_service("/:a", Router::new().route("/:b", get(|| async move {})))
            .layer(map_request(extract_matched_path));

        let client = TestClient::new(app);

        let res = client.get("/foo/bar").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[crate::test]
    async fn can_extract_nested_matched_path_in_middleware_using_nest() {
        async fn extract_matched_path<B>(
            matched_path: Option<MatchedPath>,
            req: Request<B>,
        ) -> Request<B> {
            assert_eq!(matched_path.unwrap().as_str(), "/:a/:b");
            req
        }

        let app = Router::new()
            .nest("/:a", Router::new().route("/:b", get(|| async move {})))
            .layer(map_request(extract_matched_path));

        let client = TestClient::new(app);

        let res = client.get("/foo/bar").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[crate::test]
    async fn cannot_extract_nested_matched_path_in_middleware_via_extension() {
        async fn assert_no_matched_path<B>(req: Request<B>) -> Request<B> {
            assert!(req.extensions().get::<MatchedPath>().is_none());
            req
        }

        let app = Router::new()
            .nest_service("/:a", Router::new().route("/:b", get(|| async move {})))
            .layer(map_request(assert_no_matched_path));

        let client = TestClient::new(app);

        let res = client.get("/foo/bar").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn can_extract_nested_matched_path_in_middleware_via_extension_using_nest() {
        async fn assert_matched_path<B>(req: Request<B>) -> Request<B> {
            assert!(req.extensions().get::<MatchedPath>().is_some());
            req
        }

        let app = Router::new()
            .nest("/:a", Router::new().route("/:b", get(|| async move {})))
            .layer(map_request(assert_matched_path));

        let client = TestClient::new(app);

        let res = client.get("/foo/bar").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[crate::test]
    async fn can_extract_nested_matched_path_in_middleware_on_nested_router() {
        async fn extract_matched_path<B>(matched_path: MatchedPath, req: Request<B>) -> Request<B> {
            assert_eq!(matched_path.as_str(), "/:a/:b");
            req
        }

        let app = Router::new().nest(
            "/:a",
            Router::new()
                .route("/:b", get(|| async move {}))
                .layer(map_request(extract_matched_path)),
        );

        let client = TestClient::new(app);

        let res = client.get("/foo/bar").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[crate::test]
    async fn can_extract_nested_matched_path_in_middleware_on_nested_router_via_extension() {
        async fn extract_matched_path<B>(req: Request<B>) -> Request<B> {
            let matched_path = req.extensions().get::<MatchedPath>().unwrap();
            assert_eq!(matched_path.as_str(), "/:a/:b");
            req
        }

        let app = Router::new().nest(
            "/:a",
            Router::new()
                .route("/:b", get(|| async move {}))
                .layer(map_request(extract_matched_path)),
        );

        let client = TestClient::new(app);

        let res = client.get("/foo/bar").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[crate::test]
    async fn extracting_on_nested_handler() {
        async fn handler(path: Option<MatchedPath>) {
            assert!(path.is_none());
        }

        let app = Router::new().nest_service("/:a", handler.into_service());

        let client = TestClient::new(app);

        let res = client.get("/foo/bar").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    // https://github.com/tokio-rs/axum/issues/1579
    #[crate::test]
    async fn doesnt_panic_if_router_called_from_wildcard_route() {
        use tower::ServiceExt;

        let app = Router::new().route(
            "/*path",
            any(|req: Request<Body>| {
                Router::new()
                    .nest("/", Router::new().route("/foo", get(|| async {})))
                    .oneshot(req)
            }),
        );

        let client = TestClient::new(app);

        let res = client.get("/foo").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[crate::test]
    async fn cant_extract_in_fallback() {
        async fn handler(path: Option<MatchedPath>, req: Request<Body>) {
            assert!(path.is_none());
            assert!(req.extensions().get::<MatchedPath>().is_none());
        }

        let app = Router::new().fallback(handler);

        let client = TestClient::new(app);

        let res = client.get("/foo/bar").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
