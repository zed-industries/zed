#![deny(warnings)]

#[tokio::test]
async fn cookie() {
    let foo = warp::cookie::<String>("foo");

    let req = warp::test::request().header("cookie", "foo=bar");
    assert_eq!(req.filter(&foo).await.unwrap(), "bar");

    let req = warp::test::request().header("cookie", "abc=def; foo=baz");
    assert_eq!(req.filter(&foo).await.unwrap(), "baz");

    let req = warp::test::request().header("cookie", "abc=def");
    assert!(!req.matches(&foo).await);

    let req = warp::test::request().header("cookie", "foobar=quux");
    assert!(!req.matches(&foo).await);
}

#[tokio::test]
async fn optional() {
    let foo = warp::cookie::optional::<String>("foo");

    let req = warp::test::request().header("cookie", "foo=bar");
    assert_eq!(req.filter(&foo).await.unwrap().unwrap(), "bar");

    let req = warp::test::request().header("cookie", "abc=def; foo=baz");
    assert_eq!(req.filter(&foo).await.unwrap().unwrap(), "baz");

    let req = warp::test::request().header("cookie", "abc=def");
    assert!(req.matches(&foo).await);

    let req = warp::test::request().header("cookie", "foobar=quux");
    assert!(req.matches(&foo).await);
}

#[tokio::test]
async fn missing() {
    let _ = pretty_env_logger::try_init();

    let cookie = warp::cookie::<String>("foo");

    let res = warp::test::request()
        .header("cookie", "not=here")
        .reply(&cookie)
        .await;

    assert_eq!(res.status(), 400);
    assert_eq!(res.body(), "Missing request cookie \"foo\"");
}
