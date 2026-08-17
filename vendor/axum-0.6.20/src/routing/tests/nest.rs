use super::*;
use crate::{body::boxed, extract::Extension};
use std::collections::HashMap;
use tower_http::services::ServeDir;

#[crate::test]
async fn nesting_apps() {
    let api_routes = Router::new()
        .route(
            "/users",
            get(|| async { "users#index" }).post(|| async { "users#create" }),
        )
        .route(
            "/users/:id",
            get(
                |params: extract::Path<HashMap<String, String>>| async move {
                    format!(
                        "{}: users#show ({})",
                        params.get("version").unwrap(),
                        params.get("id").unwrap()
                    )
                },
            ),
        )
        .route(
            "/games/:id",
            get(
                |params: extract::Path<HashMap<String, String>>| async move {
                    format!(
                        "{}: games#show ({})",
                        params.get("version").unwrap(),
                        params.get("id").unwrap()
                    )
                },
            ),
        );

    let app = Router::new()
        .route("/", get(|| async { "hi" }))
        .nest("/:version/api", api_routes);

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "hi");

    let res = client.get("/v0/api/users").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "users#index");

    let res = client.get("/v0/api/users/123").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "v0: users#show (123)");

    let res = client.get("/v0/api/games/123").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "v0: games#show (123)");
}

#[crate::test]
async fn wrong_method_nest() {
    let nested_app = Router::new().route("/", get(|| async {}));
    let app = Router::new().nest("/", nested_app);

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client.post("/").send().await;
    assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(res.headers()[ALLOW], "GET,HEAD");

    let res = client.patch("/foo").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[crate::test]
async fn nesting_router_at_root() {
    let nested = Router::new().route("/foo", get(|uri: Uri| async move { uri.to_string() }));
    let app = Router::new().nest("/", nested);

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let res = client.get("/foo").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "/foo");

    let res = client.get("/foo/bar").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[crate::test]
async fn nesting_router_at_empty_path() {
    let nested = Router::new().route("/foo", get(|uri: Uri| async move { uri.to_string() }));
    let app = Router::new().nest("", nested);

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let res = client.get("/foo").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "/foo");

    let res = client.get("/foo/bar").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[crate::test]
async fn nesting_handler_at_root() {
    let app = Router::new().nest_service("/", get(|uri: Uri| async move { uri.to_string() }));

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "/");

    let res = client.get("/foo").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "/foo");

    let res = client.get("/foo/bar").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "/foo/bar");
}

#[crate::test]
async fn nested_url_extractor() {
    let app = Router::new().nest(
        "/foo",
        Router::new().nest(
            "/bar",
            Router::new()
                .route("/baz", get(|uri: Uri| async move { uri.to_string() }))
                .route(
                    "/qux",
                    get(|req: Request<Body>| async move { req.uri().to_string() }),
                ),
        ),
    );

    let client = TestClient::new(app);

    let res = client.get("/foo/bar/baz").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "/baz");

    let res = client.get("/foo/bar/qux").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "/qux");
}

#[crate::test]
async fn nested_url_original_extractor() {
    let app = Router::new().nest(
        "/foo",
        Router::new().nest(
            "/bar",
            Router::new().route(
                "/baz",
                get(|uri: extract::OriginalUri| async move { uri.0.to_string() }),
            ),
        ),
    );

    let client = TestClient::new(app);

    let res = client.get("/foo/bar/baz").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "/foo/bar/baz");
}

#[crate::test]
async fn nested_service_sees_stripped_uri() {
    let app = Router::new().nest(
        "/foo",
        Router::new().nest(
            "/bar",
            Router::new().route_service(
                "/baz",
                service_fn(|req: Request<Body>| async move {
                    let body = boxed(Body::from(req.uri().to_string()));
                    Ok::<_, Infallible>(Response::new(body))
                }),
            ),
        ),
    );

    let client = TestClient::new(app);

    let res = client.get("/foo/bar/baz").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "/baz");
}

#[crate::test]
async fn nest_static_file_server() {
    let app = Router::new().nest_service("/static", ServeDir::new("."));

    let client = TestClient::new(app);

    let res = client.get("/static/README.md").send().await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[crate::test]
async fn nested_multiple_routes() {
    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/users", get(|| async { "users" }))
                .route("/teams", get(|| async { "teams" })),
        )
        .route("/", get(|| async { "root" }));

    let client = TestClient::new(app);

    assert_eq!(client.get("/").send().await.text().await, "root");
    assert_eq!(client.get("/api/users").send().await.text().await, "users");
    assert_eq!(client.get("/api/teams").send().await.text().await, "teams");
}

#[test]
#[should_panic = "Invalid route \"/\": insertion failed due to conflict with previously registered route: /*__private__axum_nest_tail_param"]
fn nested_service_at_root_with_other_routes() {
    let _: Router = Router::new()
        .nest_service("/", Router::new().route("/users", get(|| async {})))
        .route("/", get(|| async {}));
}

#[test]
fn nested_at_root_with_other_routes() {
    let _: Router = Router::new()
        .nest("/", Router::new().route("/users", get(|| async {})))
        .route("/", get(|| async {}));
}

#[crate::test]
async fn multiple_top_level_nests() {
    let app = Router::new()
        .nest(
            "/one",
            Router::new().route("/route", get(|| async { "one" })),
        )
        .nest(
            "/two",
            Router::new().route("/route", get(|| async { "two" })),
        );

    let client = TestClient::new(app);

    assert_eq!(client.get("/one/route").send().await.text().await, "one");
    assert_eq!(client.get("/two/route").send().await.text().await, "two");
}

#[crate::test]
#[should_panic(expected = "Invalid route: nested routes cannot contain wildcards (*)")]
async fn nest_cannot_contain_wildcards() {
    _ = Router::<(), Body>::new().nest("/one/*rest", Router::new());
}

#[crate::test]
async fn outer_middleware_still_see_whole_url() {
    #[derive(Clone)]
    struct SetUriExtension<S>(S);

    #[derive(Clone)]
    struct Uri(http::Uri);

    impl<S, B> Service<Request<B>> for SetUriExtension<S>
    where
        S: Service<Request<B>>,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = S::Future;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.0.poll_ready(cx)
        }

        fn call(&mut self, mut req: Request<B>) -> Self::Future {
            let uri = Uri(req.uri().clone());
            req.extensions_mut().insert(uri);
            self.0.call(req)
        }
    }

    async fn handler(Extension(Uri(middleware_uri)): Extension<Uri>) -> impl IntoResponse {
        middleware_uri.to_string()
    }

    let app = Router::new()
        .route("/", get(handler))
        .route("/foo", get(handler))
        .route("/foo/bar", get(handler))
        .nest("/one", Router::new().route("/two", get(handler)))
        .fallback(handler)
        .layer(tower::layer::layer_fn(SetUriExtension));

    let client = TestClient::new(app);

    assert_eq!(client.get("/").send().await.text().await, "/");
    assert_eq!(client.get("/foo").send().await.text().await, "/foo");
    assert_eq!(client.get("/foo/bar").send().await.text().await, "/foo/bar");
    assert_eq!(
        client.get("/not-found").send().await.text().await,
        "/not-found"
    );
    assert_eq!(client.get("/one/two").send().await.text().await, "/one/two");
}

#[crate::test]
async fn nest_at_capture() {
    let api_routes = Router::new().route(
        "/:b",
        get(|Path((a, b)): Path<(String, String)>| async move { format!("a={a} b={b}") }),
    );

    let app = Router::new().nest("/:a", api_routes);

    let client = TestClient::new(app);

    let res = client.get("/foo/bar").send().await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await, "a=foo b=bar");
}

#[crate::test]
async fn nest_with_and_without_trailing() {
    let app = Router::new().nest_service("/foo", get(|| async {}));

    let client = TestClient::new(app);

    let res = client.get("/foo").send().await;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client.get("/foo/").send().await;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client.get("/foo/bar").send().await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn nesting_with_root_inner_router() {
    let app = Router::new()
        .nest_service("/service", Router::new().route("/", get(|| async {})))
        .nest("/router", Router::new().route("/", get(|| async {})))
        .nest("/router-slash/", Router::new().route("/", get(|| async {})));

    let client = TestClient::new(app);

    // `/service/` does match the `/service` prefix and the remaining path is technically
    // empty, which is the same as `/` which matches `.route("/", _)`
    let res = client.get("/service").send().await;
    assert_eq!(res.status(), StatusCode::OK);

    // `/service/` does match the `/service` prefix and the remaining path is `/`
    // which matches `.route("/", _)`
    //
    // this is perhaps a little surprising but don't think there is much we can do
    let res = client.get("/service/").send().await;
    assert_eq!(res.status(), StatusCode::OK);

    // at least it does work like you'd expect when using `nest`

    let res = client.get("/router").send().await;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client.get("/router/").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let res = client.get("/router-slash").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let res = client.get("/router-slash/").send().await;
    assert_eq!(res.status(), StatusCode::OK);
}

macro_rules! nested_route_test {
    (
        $name:ident,
        // the path we nest the inner router at
        nest = $nested_path:literal,
        // the route the inner router accepts
        route = $route_path:literal,
        // the route we expect to be able to call
        expected = $expected_path:literal $(,)?
    ) => {
        #[crate::test]
        async fn $name() {
            let inner = Router::new().route($route_path, get(|| async {}));
            let app = Router::new().nest($nested_path, inner);
            let client = TestClient::new(app);
            let res = client.get($expected_path).send().await;
            let status = res.status();
            assert_eq!(status, StatusCode::OK, "Router");
        }
    };
}

// test cases taken from https://github.com/tokio-rs/axum/issues/714#issuecomment-1058144460
nested_route_test!(nest_1, nest = "", route = "/", expected = "/");
nested_route_test!(nest_2, nest = "", route = "/a", expected = "/a");
nested_route_test!(nest_3, nest = "", route = "/a/", expected = "/a/");
nested_route_test!(nest_4, nest = "/", route = "/", expected = "/");
nested_route_test!(nest_5, nest = "/", route = "/a", expected = "/a");
nested_route_test!(nest_6, nest = "/", route = "/a/", expected = "/a/");
nested_route_test!(nest_7, nest = "/a", route = "/", expected = "/a");
nested_route_test!(nest_8, nest = "/a", route = "/a", expected = "/a/a");
nested_route_test!(nest_9, nest = "/a", route = "/a/", expected = "/a/a/");
nested_route_test!(nest_11, nest = "/a/", route = "/", expected = "/a/");
nested_route_test!(nest_12, nest = "/a/", route = "/a", expected = "/a/a");
nested_route_test!(nest_13, nest = "/a/", route = "/a/", expected = "/a/a/");
