#![deny(warnings)]
use warp::http::header::{HeaderMap, HeaderValue};
use warp::Filter;

#[tokio::test]
async fn header() {
    let header = warp::reply::with::header("foo", "bar");

    let no_header = warp::any().map(warp::reply).with(&header);

    let req = warp::test::request();
    let resp = req.reply(&no_header).await;
    assert_eq!(resp.headers()["foo"], "bar");

    let prev_header = warp::reply::with::header("foo", "sean");
    let yes_header = warp::any().map(warp::reply).with(prev_header).with(header);

    let req = warp::test::request();
    let resp = req.reply(&yes_header).await;
    assert_eq!(resp.headers()["foo"], "bar", "replaces header");
}

#[tokio::test]
async fn headers() {
    let mut headers = HeaderMap::new();
    headers.insert("server", HeaderValue::from_static("warp"));
    headers.insert("foo", HeaderValue::from_static("bar"));

    let headers = warp::reply::with::headers(headers);

    let no_header = warp::any().map(warp::reply).with(&headers);

    let req = warp::test::request();
    let resp = req.reply(&no_header).await;
    assert_eq!(resp.headers()["foo"], "bar");
    assert_eq!(resp.headers()["server"], "warp");

    let prev_header = warp::reply::with::header("foo", "sean");
    let yes_header = warp::any().map(warp::reply).with(prev_header).with(headers);

    let req = warp::test::request();
    let resp = req.reply(&yes_header).await;
    assert_eq!(resp.headers()["foo"], "bar", "replaces header");
}

#[tokio::test]
async fn default_header() {
    let def_header = warp::reply::with::default_header("foo", "bar");

    let no_header = warp::any().map(warp::reply).with(&def_header);

    let req = warp::test::request();
    let resp = req.reply(&no_header).await;

    assert_eq!(resp.headers()["foo"], "bar");

    let header = warp::reply::with::header("foo", "sean");
    let yes_header = warp::any().map(warp::reply).with(header).with(def_header);

    let req = warp::test::request();
    let resp = req.reply(&yes_header).await;

    assert_eq!(resp.headers()["foo"], "sean", "doesn't replace header");
}
