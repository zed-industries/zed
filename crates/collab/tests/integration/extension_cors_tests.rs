use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header};
use tower::ServiceExt as _;

#[tokio::test]
async fn extension_downloads_allow_delta_web_origin() {
    let router = collab::api::extensions::router();
    for path in [
        "/extensions/updates",
        "/extensions/toml/download",
        "/extensions/toml/1.1.0/download",
    ] {
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri(path)
                    .header(header::ORIGIN, "https://delta.dev")
                    .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN),
            Some(&"https://delta.dev".parse().unwrap())
        );
    }

    let response = router
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/extensions/toml/download")
                .header(header::ORIGIN, "https://unrelated.example")
                .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        !response
            .headers()
            .contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN)
    );
}
