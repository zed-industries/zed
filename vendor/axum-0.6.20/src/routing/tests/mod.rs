use crate::{
    body::{Bytes, Empty},
    error_handling::HandleErrorLayer,
    extract::{self, DefaultBodyLimit, FromRef, Path, State},
    handler::{Handler, HandlerWithoutStateExt},
    response::IntoResponse,
    routing::{
        delete, get, get_service, on, on_service, patch, patch_service,
        path_router::path_for_nested_route, post, MethodFilter,
    },
    test_helpers::{
        tracing_helpers::{capture_tracing, TracingEvent},
        *,
    },
    BoxError, Extension, Json, Router,
};
use futures_util::stream::StreamExt;
use http::{
    header::CONTENT_LENGTH,
    header::{ALLOW, HOST},
    HeaderMap, Method, Request, Response, StatusCode, Uri,
};
use hyper::Body;
use serde::Deserialize;
use serde_json::json;
use std::{
    convert::Infallible,
    future::{ready, Ready},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    task::{Context, Poll},
    time::Duration,
};
use tower::{
    service_fn, timeout::TimeoutLayer, util::MapResponseLayer, ServiceBuilder, ServiceExt,
};
use tower_http::{limit::RequestBodyLimitLayer, validate_request::ValidateRequestHeaderLayer};
use tower_service::Service;

mod fallback;
mod get_to_head;
mod handle_error;
mod merge;
mod nest;

#[crate::test]
async fn hello_world() {
    async fn root(_: Request<Body>) -> &'static str {
        "Hello, World!"
    }

    async fn foo(_: Request<Body>) -> &'static str {
        "foo"
    }

    async fn users_create(_: Request<Body>) -> &'static str {
        "users#create"
    }

    let app = Router::new()
        .route("/", get(root).post(foo))
        .route("/users", post(users_create));

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    let body = res.text().await;
    assert_eq!(body, "Hello, World!");

    let res = client.post("/").send().await;
    let body = res.text().await;
    assert_eq!(body, "foo");

    let res = client.post("/users").send().await;
    let body = res.text().await;
    assert_eq!(body, "users#create");
}

#[crate::test]
async fn routing() {
    let app = Router::new()
        .route(
            "/users",
            get(|_: Request<Body>| async { "users#index" })
                .post(|_: Request<Body>| async { "users#create" }),
        )
        .route("/users/:id", get(|_: Request<Body>| async { "users#show" }))
        .route(
            "/users/:id/action",
            get(|_: Request<Body>| async { "users#action" }),
        );

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let res = client.get("/users").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "users#index");

    let res = client.post("/users").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "users#create");

    let res = client.get("/users/1").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "users#show");

    let res = client.get("/users/1/action").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "users#action");
}

#[crate::test]
async fn router_type_doesnt_change() {
    let app: Router = Router::new()
        .route(
            "/",
            on(MethodFilter::GET, |_: Request<Body>| async {
                "hi from GET"
            })
            .on(MethodFilter::POST, |_: Request<Body>| async {
                "hi from POST"
            }),
        )
        .layer(tower_http::compression::CompressionLayer::new());

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "hi from GET");

    let res = client.post("/").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "hi from POST");
}

#[crate::test]
async fn routing_between_services() {
    use std::convert::Infallible;
    use tower::service_fn;

    async fn handle(_: Request<Body>) -> &'static str {
        "handler"
    }

    let app = Router::new()
        .route(
            "/one",
            get_service(service_fn(|_: Request<Body>| async {
                Ok::<_, Infallible>(Response::new(Body::from("one get")))
            }))
            .post_service(service_fn(|_: Request<Body>| async {
                Ok::<_, Infallible>(Response::new(Body::from("one post")))
            }))
            .on_service(
                MethodFilter::PUT,
                service_fn(|_: Request<Body>| async {
                    Ok::<_, Infallible>(Response::new(Body::from("one put")))
                }),
            ),
        )
        .route("/two", on_service(MethodFilter::GET, handle.into_service()));

    let client = TestClient::new(app);

    let res = client.get("/one").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "one get");

    let res = client.post("/one").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "one post");

    let res = client.put("/one").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "one put");

    let res = client.get("/two").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "handler");
}

#[crate::test]
async fn middleware_on_single_route() {
    use tower::ServiceBuilder;
    use tower_http::{compression::CompressionLayer, trace::TraceLayer};

    async fn handle(_: Request<Body>) -> &'static str {
        "Hello, World!"
    }

    let app = Router::new().route(
        "/",
        get(handle.layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .into_inner(),
        )),
    );

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    let body = res.text().await;

    assert_eq!(body, "Hello, World!");
}

#[crate::test]
async fn service_in_bottom() {
    async fn handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
        Ok(Response::new(hyper::Body::empty()))
    }

    let app = Router::new().route("/", get_service(service_fn(handler)));

    TestClient::new(app);
}

#[crate::test]
async fn wrong_method_handler() {
    let app = Router::new()
        .route("/", get(|| async {}).post(|| async {}))
        .route("/foo", patch(|| async {}));

    let client = TestClient::new(app);

    let res = client.patch("/").send().await;
    assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(res.headers()[ALLOW], "GET,HEAD,POST");

    let res = client.patch("/foo").send().await;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client.post("/foo").send().await;
    assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(res.headers()[ALLOW], "PATCH");

    let res = client.get("/bar").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[crate::test]
async fn wrong_method_service() {
    #[derive(Clone)]
    struct Svc;

    impl<R> Service<R> for Svc {
        type Response = Response<Empty<Bytes>>;
        type Error = Infallible;
        type Future = Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: R) -> Self::Future {
            ready(Ok(Response::new(Empty::new())))
        }
    }

    let app = Router::new()
        .route("/", get_service(Svc).post_service(Svc))
        .route("/foo", patch_service(Svc));

    let client = TestClient::new(app);

    let res = client.patch("/").send().await;
    assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(res.headers()[ALLOW], "GET,HEAD,POST");

    let res = client.patch("/foo").send().await;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client.post("/foo").send().await;
    assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(res.headers()[ALLOW], "PATCH");

    let res = client.get("/bar").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[crate::test]
async fn multiple_methods_for_one_handler() {
    async fn root(_: Request<Body>) -> &'static str {
        "Hello, World!"
    }

    let app = Router::new().route("/", on(MethodFilter::GET | MethodFilter::POST, root));

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client.post("/").send().await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[crate::test]
async fn wildcard_sees_whole_url() {
    let app = Router::new().route("/api/*rest", get(|uri: Uri| async move { uri.to_string() }));

    let client = TestClient::new(app);

    let res = client.get("/api/foo/bar").send().await;
    assert_eq!(res.text().await, "/api/foo/bar");
}

#[crate::test]
async fn middleware_applies_to_routes_above() {
    let app = Router::new()
        .route("/one", get(std::future::pending::<()>))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async move {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::new(0, 0))),
        )
        .route("/two", get(|| async {}));

    let client = TestClient::new(app);

    let res = client.get("/one").send().await;
    assert_eq!(res.status(), StatusCode::REQUEST_TIMEOUT);

    let res = client.get("/two").send().await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[crate::test]
async fn not_found_for_extra_trailing_slash() {
    let app = Router::new().route("/foo", get(|| async {}));

    let client = TestClient::new(app);

    let res = client.get("/foo/").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let res = client.get("/foo").send().await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[crate::test]
async fn not_found_for_missing_trailing_slash() {
    let app = Router::new().route("/foo/", get(|| async {}));

    let client = TestClient::new(app);

    let res = client.get("/foo").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[crate::test]
async fn with_and_without_trailing_slash() {
    let app = Router::new()
        .route("/foo", get(|| async { "without tsr" }))
        .route("/foo/", get(|| async { "with tsr" }));

    let client = TestClient::new(app);

    let res = client.get("/foo/").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "with tsr");

    let res = client.get("/foo").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "without tsr");
}

// for https://github.com/tokio-rs/axum/issues/420
#[crate::test]
async fn wildcard_doesnt_match_just_trailing_slash() {
    let app = Router::new().route(
        "/x/*path",
        get(|Path(path): Path<String>| async move { path }),
    );

    let client = TestClient::new(app);

    let res = client.get("/x").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let res = client.get("/x/").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let res = client.get("/x/foo/bar").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "foo/bar");
}

#[crate::test]
async fn what_matches_wildcard() {
    let app = Router::new()
        .route("/*key", get(|| async { "root" }))
        .route("/x/*key", get(|| async { "x" }))
        .fallback(|| async { "fallback" });

    let client = TestClient::new(app);

    let get = |path| {
        let f = client.get(path).send();
        async move { f.await.text().await }
    };

    assert_eq!(get("/").await, "fallback");
    assert_eq!(get("/a").await, "root");
    assert_eq!(get("/a/").await, "root");
    assert_eq!(get("/a/b").await, "root");
    assert_eq!(get("/a/b/").await, "root");

    assert_eq!(get("/x").await, "root");
    assert_eq!(get("/x/").await, "root");
    assert_eq!(get("/x/a").await, "x");
    assert_eq!(get("/x/a/").await, "x");
    assert_eq!(get("/x/a/b").await, "x");
    assert_eq!(get("/x/a/b/").await, "x");
}

#[crate::test]
async fn static_and_dynamic_paths() {
    let app = Router::new()
        .route(
            "/:key",
            get(|Path(key): Path<String>| async move { format!("dynamic: {key}") }),
        )
        .route("/foo", get(|| async { "static" }));

    let client = TestClient::new(app);

    let res = client.get("/bar").send().await;
    assert_eq!(res.text().await, "dynamic: bar");

    let res = client.get("/foo").send().await;
    assert_eq!(res.text().await, "static");
}

#[crate::test]
#[should_panic(expected = "Paths must start with a `/`. Use \"/\" for root routes")]
async fn empty_route() {
    let app = Router::new().route("", get(|| async {}));
    TestClient::new(app);
}

#[crate::test]
async fn middleware_still_run_for_unmatched_requests() {
    #[derive(Clone)]
    struct CountMiddleware<S>(S);

    static COUNT: AtomicUsize = AtomicUsize::new(0);

    impl<R, S> Service<R> for CountMiddleware<S>
    where
        S: Service<R>,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = S::Future;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.0.poll_ready(cx)
        }

        fn call(&mut self, req: R) -> Self::Future {
            COUNT.fetch_add(1, Ordering::SeqCst);
            self.0.call(req)
        }
    }

    let app = Router::new()
        .route("/", get(|| async {}))
        .layer(tower::layer::layer_fn(CountMiddleware));

    let client = TestClient::new(app);

    assert_eq!(COUNT.load(Ordering::SeqCst), 0);

    client.get("/").send().await;
    assert_eq!(COUNT.load(Ordering::SeqCst), 1);

    client.get("/not-found").send().await;
    assert_eq!(COUNT.load(Ordering::SeqCst), 2);
}

#[crate::test]
#[should_panic(expected = "\
    Invalid route: `Router::route_service` cannot be used with `Router`s. \
    Use `Router::nest` instead\
")]
async fn routing_to_router_panics() {
    TestClient::new(Router::new().route_service("/", Router::new()));
}

#[crate::test]
async fn route_layer() {
    let app = Router::new()
        .route("/foo", get(|| async {}))
        .route_layer(ValidateRequestHeaderLayer::bearer("password"));

    let client = TestClient::new(app);

    let res = client
        .get("/foo")
        .header("authorization", "Bearer password")
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client.get("/foo").send().await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let res = client.get("/not-found").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    // it would be nice if this would return `405 Method Not Allowed`
    // but that requires knowing more about which method route we're calling, which we
    // don't know currently since its just a generic `Service`
    let res = client.post("/foo").send().await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[crate::test]
async fn different_methods_added_in_different_routes() {
    let app = Router::new()
        .route("/", get(|| async { "GET" }))
        .route("/", post(|| async { "POST" }));

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    let body = res.text().await;
    assert_eq!(body, "GET");

    let res = client.post("/").send().await;
    let body = res.text().await;
    assert_eq!(body, "POST");
}

#[crate::test]
#[should_panic(expected = "Cannot merge two `Router`s that both have a fallback")]
async fn merging_routers_with_fallbacks_panics() {
    async fn fallback() {}
    let one = Router::new().fallback(fallback);
    let two = Router::new().fallback(fallback);
    TestClient::new(one.merge(two));
}

#[test]
#[should_panic(expected = "Overlapping method route. Handler for `GET /foo/bar` already exists")]
fn routes_with_overlapping_method_routes() {
    async fn handler() {}
    let _: Router = Router::new()
        .route("/foo/bar", get(handler))
        .route("/foo/bar", get(handler));
}

#[test]
#[should_panic(expected = "Overlapping method route. Handler for `GET /foo/bar` already exists")]
fn merging_with_overlapping_method_routes() {
    async fn handler() {}
    let app: Router = Router::new().route("/foo/bar", get(handler));
    _ = app.clone().merge(app);
}

#[crate::test]
async fn merging_routers_with_same_paths_but_different_methods() {
    let one = Router::new().route("/", get(|| async { "GET" }));
    let two = Router::new().route("/", post(|| async { "POST" }));

    let client = TestClient::new(one.merge(two));

    let res = client.get("/").send().await;
    let body = res.text().await;
    assert_eq!(body, "GET");

    let res = client.post("/").send().await;
    let body = res.text().await;
    assert_eq!(body, "POST");
}

#[crate::test]
async fn head_content_length_through_hyper_server() {
    let app = Router::new()
        .route("/", get(|| async { "foo" }))
        .route("/json", get(|| async { Json(json!({ "foo": 1 })) }));

    let client = TestClient::new(app);

    let res = client.head("/").send().await;
    assert_eq!(res.headers()["content-length"], "3");
    assert!(res.text().await.is_empty());

    let res = client.head("/json").send().await;
    assert_eq!(res.headers()["content-length"], "9");
    assert!(res.text().await.is_empty());
}

#[crate::test]
async fn head_content_length_through_hyper_server_that_hits_fallback() {
    let app = Router::new().fallback(|| async { "foo" });

    let client = TestClient::new(app);

    let res = client.head("/").send().await;
    assert_eq!(res.headers()["content-length"], "3");
}

#[crate::test]
async fn head_with_middleware_applied() {
    use tower_http::compression::{predicate::SizeAbove, CompressionLayer};

    let app = Router::new()
        .nest(
            "/",
            Router::new().route("/", get(|| async { "Hello, World!" })),
        )
        .layer(CompressionLayer::new().compress_when(SizeAbove::new(0)));

    let client = TestClient::new(app);

    // send GET request
    let res = client
        .get("/")
        .header("accept-encoding", "gzip")
        .send()
        .await;
    assert_eq!(res.headers()["transfer-encoding"], "chunked");
    // cannot have `transfer-encoding: chunked` and `content-length`
    assert!(!res.headers().contains_key("content-length"));

    // send HEAD request
    let res = client
        .head("/")
        .header("accept-encoding", "gzip")
        .send()
        .await;
    // no response body so no `transfer-encoding`
    assert!(!res.headers().contains_key("transfer-encoding"));
    // no content-length since we cannot know it since the response
    // is compressed
    assert!(!res.headers().contains_key("content-length"));
}

#[crate::test]
#[should_panic(expected = "Paths must start with a `/`")]
async fn routes_must_start_with_slash() {
    let app = Router::new().route(":foo", get(|| async {}));
    TestClient::new(app);
}

#[crate::test]
async fn body_limited_by_default() {
    let app = Router::new()
        .route("/bytes", post(|_: Bytes| async {}))
        .route("/string", post(|_: String| async {}))
        .route("/json", post(|_: Json<serde_json::Value>| async {}));

    let client = TestClient::new(app);

    for uri in ["/bytes", "/string", "/json"] {
        println!("calling {uri}");

        let stream = futures_util::stream::repeat("a".repeat(1000)).map(Ok::<_, hyper::Error>);
        let body = Body::wrap_stream(stream);

        let res_future = client
            .post(uri)
            .header("content-type", "application/json")
            .body(body)
            .send();
        let res = tokio::time::timeout(Duration::from_secs(3), res_future)
            .await
            .expect("never got response");

        assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }
}

#[crate::test]
async fn disabling_the_default_limit() {
    let app = Router::new()
        .route("/", post(|_: Bytes| async {}))
        .layer(DefaultBodyLimit::disable());

    let client = TestClient::new(app);

    // `DEFAULT_LIMIT` is 2mb so make a body larger than that
    let body = Body::from("a".repeat(3_000_000));

    let res = client.post("/").body(body).send().await;

    assert_eq!(res.status(), StatusCode::OK);
}

#[crate::test]
async fn limited_body_with_content_length() {
    const LIMIT: usize = 3;

    let app = Router::new()
        .route(
            "/",
            post(|headers: HeaderMap, _body: Bytes| async move {
                assert!(headers.get(CONTENT_LENGTH).is_some());
            }),
        )
        .layer(RequestBodyLimitLayer::new(LIMIT));

    let client = TestClient::new(app);

    let res = client.post("/").body("a".repeat(LIMIT)).send().await;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client.post("/").body("a".repeat(LIMIT * 2)).send().await;
    assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[crate::test]
async fn changing_the_default_limit() {
    let new_limit = 2;

    let app = Router::new()
        .route("/", post(|_: Bytes| async {}))
        .layer(DefaultBodyLimit::max(new_limit));

    let client = TestClient::new(app);

    let res = client
        .post("/")
        .body(Body::from("a".repeat(new_limit)))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client
        .post("/")
        .body(Body::from("a".repeat(new_limit + 1)))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[crate::test]
async fn limited_body_with_streaming_body() {
    const LIMIT: usize = 3;

    let app = Router::new()
        .route(
            "/",
            post(|headers: HeaderMap, _body: Bytes| async move {
                assert!(headers.get(CONTENT_LENGTH).is_none());
            }),
        )
        .layer(RequestBodyLimitLayer::new(LIMIT));

    let client = TestClient::new(app);

    let stream = futures_util::stream::iter(vec![Ok::<_, hyper::Error>("a".repeat(LIMIT))]);
    let res = client
        .post("/")
        .body(Body::wrap_stream(stream))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let stream = futures_util::stream::iter(vec![Ok::<_, hyper::Error>("a".repeat(LIMIT * 2))]);
    let res = client
        .post("/")
        .body(Body::wrap_stream(stream))
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[crate::test]
async fn extract_state() {
    #[derive(Clone)]
    struct AppState {
        value: i32,
        inner: InnerState,
    }

    #[derive(Clone)]
    struct InnerState {
        value: i32,
    }

    impl FromRef<AppState> for InnerState {
        fn from_ref(state: &AppState) -> Self {
            state.inner.clone()
        }
    }

    async fn handler(State(outer): State<AppState>, State(inner): State<InnerState>) {
        assert_eq!(outer.value, 1);
        assert_eq!(inner.value, 2);
    }

    let state = AppState {
        value: 1,
        inner: InnerState { value: 2 },
    };

    let app = Router::new().route("/", get(handler)).with_state(state);
    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[crate::test]
async fn explicitly_set_state() {
    let app = Router::new()
        .route_service(
            "/",
            get(|State(state): State<&'static str>| async move { state }).with_state("foo"),
        )
        .with_state("...");

    let client = TestClient::new(app);
    let res = client.get("/").send().await;
    assert_eq!(res.text().await, "foo");
}

#[crate::test]
async fn layer_response_into_response() {
    fn map_response<B>(_res: Response<B>) -> Result<Response<B>, impl IntoResponse> {
        let headers = [("x-foo", "bar")];
        let status = StatusCode::IM_A_TEAPOT;
        Err((headers, status))
    }

    let app = Router::new()
        .route("/", get(|| async {}))
        .layer(MapResponseLayer::new(map_response));

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.headers()["x-foo"], "bar");
    assert_eq!(res.status(), StatusCode::IM_A_TEAPOT);
}

#[allow(dead_code)]
fn method_router_fallback_with_state() {
    async fn fallback(_: State<&'static str>) {}

    async fn not_found(_: State<&'static str>) {}

    let state = "foo";

    let _: Router = Router::new()
        .fallback(get(fallback).fallback(not_found))
        .with_state(state);
}

#[test]
fn test_path_for_nested_route() {
    assert_eq!(path_for_nested_route("/", "/"), "/");

    assert_eq!(path_for_nested_route("/a", "/"), "/a");
    assert_eq!(path_for_nested_route("/", "/b"), "/b");
    assert_eq!(path_for_nested_route("/a/", "/"), "/a/");
    assert_eq!(path_for_nested_route("/", "/b/"), "/b/");

    assert_eq!(path_for_nested_route("/a", "/b"), "/a/b");
    assert_eq!(path_for_nested_route("/a/", "/b"), "/a/b");
    assert_eq!(path_for_nested_route("/a", "/b/"), "/a/b/");
    assert_eq!(path_for_nested_route("/a/", "/b/"), "/a/b/");
}

#[crate::test]
async fn state_isnt_cloned_too_much() {
    static SETUP_DONE: AtomicBool = AtomicBool::new(false);
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    struct AppState;

    impl Clone for AppState {
        fn clone(&self) -> Self {
            #[rustversion::since(1.65)]
            #[track_caller]
            fn count() {
                if SETUP_DONE.load(Ordering::SeqCst) {
                    let bt = std::backtrace::Backtrace::force_capture();
                    let bt = bt
                        .to_string()
                        .lines()
                        .filter(|line| line.contains("axum") || line.contains("./src"))
                        .collect::<Vec<_>>()
                        .join("\n");
                    println!("AppState::Clone:\n===============\n{}\n", bt);
                    COUNT.fetch_add(1, Ordering::SeqCst);
                }
            }

            #[rustversion::not(since(1.65))]
            fn count() {
                if SETUP_DONE.load(Ordering::SeqCst) {
                    COUNT.fetch_add(1, Ordering::SeqCst);
                }
            }

            count();

            Self
        }
    }

    let app = Router::new()
        .route("/", get(|_: State<AppState>| async {}))
        .with_state(AppState);

    let client = TestClient::new(app);

    // ignore clones made during setup
    SETUP_DONE.store(true, Ordering::SeqCst);

    client.get("/").send().await;

    assert_eq!(COUNT.load(Ordering::SeqCst), 4);
}

#[crate::test]
async fn logging_rejections() {
    #[derive(Deserialize, Eq, PartialEq, Debug)]
    #[serde(deny_unknown_fields)]
    struct RejectionEvent {
        message: String,
        status: u16,
        body: String,
        rejection_type: String,
    }

    let events = capture_tracing::<RejectionEvent, _, _>(|| async {
        let app = Router::new()
            .route("/extension", get(|_: Extension<Infallible>| async {}))
            .route("/string", post(|_: String| async {}));

        let client = TestClient::new(app);

        assert_eq!(
            client.get("/extension").send().await.status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );

        assert_eq!(
            client
                .post("/string")
                .body(Vec::from([0, 159, 146, 150]))
                .send()
                .await
                .status(),
            StatusCode::BAD_REQUEST,
        );
    })
    .await;

    assert_eq!(
        dbg!(events),
        Vec::from([
            TracingEvent {
                fields: RejectionEvent {
                    message: "rejecting request".to_owned(),
                    status: 500,
                    body: "Missing request extension: Extension of \
                        type `core::convert::Infallible` was not found. \
                        Perhaps you forgot to add it? See `axum::Extension`."
                        .to_owned(),
                    rejection_type: "axum::extract::rejection::MissingExtension".to_owned(),
                },
                target: "axum::rejection".to_owned(),
                level: "TRACE".to_owned(),
            },
            TracingEvent {
                fields: RejectionEvent {
                    message: "rejecting request".to_owned(),
                    status: 400,
                    body: "Request body didn't contain valid UTF-8: \
                        invalid utf-8 sequence of 1 bytes from index 1"
                        .to_owned(),
                    rejection_type: "axum_core::extract::rejection::InvalidUtf8".to_owned(),
                },
                target: "axum::rejection".to_owned(),
                level: "TRACE".to_owned(),
            },
        ])
    )
}

// https://github.com/tokio-rs/axum/issues/1955
#[crate::test]
async fn connect_going_to_custom_fallback() {
    let app = Router::new().fallback(|| async { (StatusCode::NOT_FOUND, "custom fallback") });

    let req = Request::builder()
        .uri("example.com:443")
        .method(Method::CONNECT)
        .header(HOST, "example.com:443")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    let text = String::from_utf8(hyper::body::to_bytes(res).await.unwrap().to_vec()).unwrap();
    assert_eq!(text, "custom fallback");
}

// https://github.com/tokio-rs/axum/issues/1955
#[crate::test]
async fn connect_going_to_default_fallback() {
    let app = Router::new();

    let req = Request::builder()
        .uri("example.com:443")
        .method(Method::CONNECT)
        .header(HOST, "example.com:443")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    let body = hyper::body::to_bytes(res).await.unwrap();
    assert!(body.is_empty());
}

#[crate::test]
async fn impl_handler_for_into_response() {
    let app = Router::new().route("/things", post((StatusCode::CREATED, "thing created")));

    let client = TestClient::new(app);

    let res = client.post("/things").send().await;
    assert_eq!(res.status(), StatusCode::CREATED);
    assert_eq!(res.text().await, "thing created");
}
