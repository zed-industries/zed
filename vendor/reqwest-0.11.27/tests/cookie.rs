mod support;
use support::server;

#[tokio::test]
async fn cookie_response_accessor() {
    let server = server::http(move |_req| async move {
        http::Response::builder()
            .header("Set-Cookie", "key=val")
            .header(
                "Set-Cookie",
                "expires=1; Expires=Wed, 21 Oct 2015 07:28:00 GMT",
            )
            .header("Set-Cookie", "path=1; Path=/the-path")
            .header("Set-Cookie", "maxage=1; Max-Age=100")
            .header("Set-Cookie", "domain=1; Domain=mydomain")
            .header("Set-Cookie", "secure=1; Secure")
            .header("Set-Cookie", "httponly=1; HttpOnly")
            .header("Set-Cookie", "samesitelax=1; SameSite=Lax")
            .header("Set-Cookie", "samesitestrict=1; SameSite=Strict")
            .body(Default::default())
            .unwrap()
    });

    let client = reqwest::Client::new();

    let url = format!("http://{}/", server.addr());
    let res = client.get(&url).send().await.unwrap();

    let cookies = res.cookies().collect::<Vec<_>>();

    // key=val
    assert_eq!(cookies[0].name(), "key");
    assert_eq!(cookies[0].value(), "val");

    // expires
    assert_eq!(cookies[1].name(), "expires");
    assert_eq!(
        cookies[1].expires().unwrap(),
        std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_445_412_480)
    );

    // path
    assert_eq!(cookies[2].name(), "path");
    assert_eq!(cookies[2].path().unwrap(), "/the-path");

    // max-age
    assert_eq!(cookies[3].name(), "maxage");
    assert_eq!(
        cookies[3].max_age().unwrap(),
        std::time::Duration::from_secs(100)
    );

    // domain
    assert_eq!(cookies[4].name(), "domain");
    assert_eq!(cookies[4].domain().unwrap(), "mydomain");

    // secure
    assert_eq!(cookies[5].name(), "secure");
    assert_eq!(cookies[5].secure(), true);

    // httponly
    assert_eq!(cookies[6].name(), "httponly");
    assert_eq!(cookies[6].http_only(), true);

    // samesitelax
    assert_eq!(cookies[7].name(), "samesitelax");
    assert!(cookies[7].same_site_lax());

    // samesitestrict
    assert_eq!(cookies[8].name(), "samesitestrict");
    assert!(cookies[8].same_site_strict());
}

#[tokio::test]
async fn cookie_store_simple() {
    let server = server::http(move |req| async move {
        if req.uri() == "/2" {
            assert_eq!(req.headers()["cookie"], "key=val");
        }
        http::Response::builder()
            .header("Set-Cookie", "key=val; HttpOnly")
            .body(Default::default())
            .unwrap()
    });

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let url = format!("http://{}/", server.addr());
    client.get(&url).send().await.unwrap();

    let url = format!("http://{}/2", server.addr());
    client.get(&url).send().await.unwrap();
}

#[tokio::test]
async fn cookie_store_overwrite_existing() {
    let server = server::http(move |req| async move {
        if req.uri() == "/" {
            http::Response::builder()
                .header("Set-Cookie", "key=val")
                .body(Default::default())
                .unwrap()
        } else if req.uri() == "/2" {
            assert_eq!(req.headers()["cookie"], "key=val");
            http::Response::builder()
                .header("Set-Cookie", "key=val2")
                .body(Default::default())
                .unwrap()
        } else {
            assert_eq!(req.uri(), "/3");
            assert_eq!(req.headers()["cookie"], "key=val2");
            http::Response::default()
        }
    });

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let url = format!("http://{}/", server.addr());
    client.get(&url).send().await.unwrap();

    let url = format!("http://{}/2", server.addr());
    client.get(&url).send().await.unwrap();

    let url = format!("http://{}/3", server.addr());
    client.get(&url).send().await.unwrap();
}

#[tokio::test]
async fn cookie_store_max_age() {
    let server = server::http(move |req| async move {
        assert_eq!(req.headers().get("cookie"), None);
        http::Response::builder()
            .header("Set-Cookie", "key=val; Max-Age=0")
            .body(Default::default())
            .unwrap()
    });

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let url = format!("http://{}/", server.addr());
    client.get(&url).send().await.unwrap();
    client.get(&url).send().await.unwrap();
}

#[tokio::test]
async fn cookie_store_expires() {
    let server = server::http(move |req| async move {
        assert_eq!(req.headers().get("cookie"), None);
        http::Response::builder()
            .header(
                "Set-Cookie",
                "key=val; Expires=Wed, 21 Oct 2015 07:28:00 GMT",
            )
            .body(Default::default())
            .unwrap()
    });

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let url = format!("http://{}/", server.addr());
    client.get(&url).send().await.unwrap();
    client.get(&url).send().await.unwrap();
}

#[tokio::test]
async fn cookie_store_path() {
    let server = server::http(move |req| async move {
        if req.uri() == "/" {
            assert_eq!(req.headers().get("cookie"), None);
            http::Response::builder()
                .header("Set-Cookie", "key=val; Path=/subpath")
                .body(Default::default())
                .unwrap()
        } else {
            assert_eq!(req.uri(), "/subpath");
            assert_eq!(req.headers()["cookie"], "key=val");
            http::Response::default()
        }
    });

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let url = format!("http://{}/", server.addr());
    client.get(&url).send().await.unwrap();
    client.get(&url).send().await.unwrap();

    let url = format!("http://{}/subpath", server.addr());
    client.get(&url).send().await.unwrap();
}
