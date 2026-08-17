use super::*;
use std::future::{pending, ready};
use tower::{timeout::TimeoutLayer, ServiceBuilder};

async fn unit() {}

async fn forever() {
    pending().await
}

fn timeout() -> TimeoutLayer {
    TimeoutLayer::new(Duration::from_millis(10))
}

#[derive(Clone)]
struct Svc;

impl<R> Service<R> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: R) -> Self::Future {
        ready(Ok(Response::new(Body::empty())))
    }
}

#[crate::test]
async fn handler() {
    let app = Router::new().route(
        "/",
        get(forever.layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(timeout()),
        )),
    );

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::REQUEST_TIMEOUT);
}

#[crate::test]
async fn handler_multiple_methods_first() {
    let app = Router::new().route(
        "/",
        get(forever.layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(timeout()),
        ))
        .post(unit),
    );

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::REQUEST_TIMEOUT);
}

#[crate::test]
async fn handler_multiple_methods_middle() {
    let app = Router::new().route(
        "/",
        delete(unit)
            .get(
                forever.layer(
                    ServiceBuilder::new()
                        .layer(HandleErrorLayer::new(|_: BoxError| async {
                            StatusCode::REQUEST_TIMEOUT
                        }))
                        .layer(timeout()),
                ),
            )
            .post(unit),
    );

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::REQUEST_TIMEOUT);
}

#[crate::test]
async fn handler_multiple_methods_last() {
    let app = Router::new().route(
        "/",
        delete(unit).get(
            forever.layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(|_: BoxError| async {
                        StatusCode::REQUEST_TIMEOUT
                    }))
                    .layer(timeout()),
            ),
        ),
    );

    let client = TestClient::new(app);

    let res = client.get("/").send().await;
    assert_eq!(res.status(), StatusCode::REQUEST_TIMEOUT);
}
