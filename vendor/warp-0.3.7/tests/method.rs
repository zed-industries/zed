#![deny(warnings)]
use warp::Filter;

#[tokio::test]
async fn method() {
    let _ = pretty_env_logger::try_init();
    let get = warp::get().map(warp::reply);

    let req = warp::test::request();
    assert!(req.matches(&get).await);

    let req = warp::test::request().method("POST");
    assert!(!req.matches(&get).await);

    let req = warp::test::request().method("POST");
    let resp = req.reply(&get).await;
    assert_eq!(resp.status(), 405);
}

#[tokio::test]
async fn method_not_allowed_trumps_not_found() {
    let _ = pretty_env_logger::try_init();
    let get = warp::get().and(warp::path("hello").map(warp::reply));
    let post = warp::post().and(warp::path("bye").map(warp::reply));

    let routes = get.or(post);

    let req = warp::test::request().method("GET").path("/bye");

    let resp = req.reply(&routes).await;
    // GET was allowed, but only for /hello, so POST returning 405 is fine.
    assert_eq!(resp.status(), 405);
}

#[tokio::test]
async fn bad_request_trumps_method_not_allowed() {
    let _ = pretty_env_logger::try_init();
    let get = warp::get()
        .and(warp::path("hello"))
        .and(warp::header::exact("foo", "bar"))
        .map(warp::reply);
    let post = warp::post().and(warp::path("bye")).map(warp::reply);

    let routes = get.or(post);

    let req = warp::test::request().method("GET").path("/hello");

    let resp = req.reply(&routes).await;
    // GET was allowed, but header rejects with 400, should not
    // assume POST was the appropriate method.
    assert_eq!(resp.status(), 400);
}
