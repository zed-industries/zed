#![deny(warnings)]
use std::fs;

#[tokio::test]
async fn file() {
    let _ = pretty_env_logger::try_init();

    let file = warp::fs::file("README.md");

    let req = warp::test::request();
    let res = req.reply(&file).await;

    assert_eq!(res.status(), 200);

    let contents = fs::read("README.md").expect("fs::read README.md");
    assert_eq!(res.headers()["content-length"], contents.len().to_string());
    assert_eq!(res.headers()["accept-ranges"], "bytes");

    let ct = &res.headers()["content-type"];
    assert!(
        ct == "text/x-markdown" || ct == "text/markdown",
        "content-type is not markdown: {:?}",
        ct,
    );

    assert_eq!(res.body(), &*contents);
}

#[tokio::test]
async fn dir() {
    let _ = pretty_env_logger::try_init();

    let file = warp::fs::dir("examples");

    let req = warp::test::request().path("/todos.rs");
    let res = req.reply(&file).await;

    assert_eq!(res.status(), 200);

    let contents = fs::read("examples/todos.rs").expect("fs::read");
    assert_eq!(res.headers()["content-length"], contents.len().to_string());
    assert_eq!(res.headers()["content-type"], "text/x-rust");
    assert_eq!(res.headers()["accept-ranges"], "bytes");

    assert_eq!(res.body(), &*contents);

    let malformed_req = warp::test::request().path("todos.rs");
    assert_eq!(malformed_req.reply(&file).await.status(), 404);
}

#[tokio::test]
async fn dir_encoded() {
    let _ = pretty_env_logger::try_init();

    let file = warp::fs::dir("examples");

    let req = warp::test::request().path("/todos%2ers");
    let res = req.reply(&file).await;

    assert_eq!(res.status(), 200);

    let contents = fs::read("examples/todos.rs").expect("fs::read");
    assert_eq!(res.headers()["content-length"], contents.len().to_string());

    assert_eq!(res.body(), &*contents);
}

#[tokio::test]
async fn dir_not_found() {
    let _ = pretty_env_logger::try_init();

    let file = warp::fs::dir("examples");

    let req = warp::test::request().path("/definitely-not-found");
    let res = req.reply(&file).await;

    assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn dir_bad_path() {
    let _ = pretty_env_logger::try_init();

    let file = warp::fs::dir("examples");

    let req = warp::test::request().path("/../README.md");
    let res = req.reply(&file).await;

    assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn dir_bad_encoded_path() {
    let _ = pretty_env_logger::try_init();

    let file = warp::fs::dir("examples");

    let req = warp::test::request().path("/%2E%2e/README.md");
    let res = req.reply(&file).await;

    assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn dir_fallback_index_on_dir() {
    let _ = pretty_env_logger::try_init();

    let file = warp::fs::dir("examples");
    let req = warp::test::request().path("/dir");
    let res = req.reply(&file).await;
    let contents = fs::read("examples/dir/index.html").expect("fs::read");
    assert_eq!(res.headers()["content-length"], contents.len().to_string());
    assert_eq!(res.status(), 200);
    let req = warp::test::request().path("/dir/");
    let res = req.reply(&file).await;
    assert_eq!(res.headers()["content-length"], contents.len().to_string());
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn not_modified() {
    let _ = pretty_env_logger::try_init();

    let file = warp::fs::file("README.md");

    let req = warp::test::request();
    let body = fs::read("README.md").unwrap();
    let res1 = req.reply(&file).await;
    assert_eq!(res1.status(), 200);
    assert_eq!(res1.headers()["content-length"], body.len().to_string());

    // if-modified-since
    let res = warp::test::request()
        .header("if-modified-since", &res1.headers()["last-modified"])
        .reply(&file)
        .await;
    assert_eq!(res.headers().get("content-length"), None);
    assert_eq!(res.status(), 304);
    assert_eq!(res.body(), "");

    // clearly too old
    let res = warp::test::request()
        .header("if-modified-since", "Mon, 07 Nov 1994 01:00:00 GMT")
        .reply(&file)
        .await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), &body);
    assert_eq!(res1.headers()["content-length"], body.len().to_string());
}

#[tokio::test]
async fn precondition() {
    let _ = pretty_env_logger::try_init();

    let file = warp::fs::file("README.md");

    let req = warp::test::request();
    let res1 = req.reply(&file).await;
    assert_eq!(res1.status(), 200);

    // if-unmodified-since
    let res = warp::test::request()
        .header("if-unmodified-since", &res1.headers()["last-modified"])
        .reply(&file)
        .await;
    assert_eq!(res.status(), 200);

    // clearly too old
    let res = warp::test::request()
        .header("if-unmodified-since", "Mon, 07 Nov 1994 01:00:00 GMT")
        .reply(&file)
        .await;
    assert_eq!(res.status(), 412);
    assert_eq!(res.body(), "");
}

#[tokio::test]
async fn byte_ranges() {
    let _ = pretty_env_logger::try_init();

    let contents = fs::read("README.md").expect("fs::read README.md");
    let file = warp::fs::file("README.md");

    let res = warp::test::request()
        .header("range", "bytes=100-200")
        .reply(&file)
        .await;
    assert_eq!(res.status(), 206);
    assert_eq!(
        res.headers()["content-range"],
        format!("bytes 100-200/{}", contents.len())
    );
    assert_eq!(res.headers()["content-length"], "101");
    assert_eq!(res.body(), &contents[100..=200]);

    // bad range
    let res = warp::test::request()
        .header("range", "bytes=100-10")
        .reply(&file)
        .await;
    assert_eq!(res.status(), 416);
    assert_eq!(
        res.headers()["content-range"],
        format!("bytes */{}", contents.len())
    );
    assert_eq!(res.headers().get("content-length"), None);
    assert_eq!(res.body(), "");

    // out of range
    let res = warp::test::request()
        .header("range", "bytes=100-100000")
        .reply(&file)
        .await;
    assert_eq!(res.status(), 416);
    assert_eq!(
        res.headers()["content-range"],
        format!("bytes */{}", contents.len())
    );
    assert_eq!(res.headers().get("content-length"), None);
    assert_eq!(res.body(), "");

    // if-range too old
    let res = warp::test::request()
        .header("range", "bytes=100-200")
        .header("if-range", "Mon, 07 Nov 1994 01:00:00 GMT")
        .reply(&file)
        .await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.headers()["content-length"], contents.len().to_string());
    assert_eq!(res.headers().get("content-range"), None);
}

#[tokio::test]
async fn byte_ranges_with_excluded_file_size() {
    let _ = pretty_env_logger::try_init();

    let contents = fs::read("README.md").expect("fs::read README.md");
    let file = warp::fs::file("README.md");

    // range including end of file (non-inclusive result)
    let res = warp::test::request()
        .header("range", format!("bytes=100-{}", contents.len()))
        .reply(&file)
        .await;
    assert_eq!(res.status(), 206);
    assert_eq!(
        res.headers()["content-range"],
        format!("bytes 100-{}/{}", contents.len() - 1, contents.len())
    );
    assert_eq!(
        res.headers()["content-length"],
        format!("{}", contents.len() - 100)
    );
    assert_eq!(res.body(), &contents[100..=contents.len() - 1]);

    // range with 1 byte to end yields same result as above. (inclusive result)
    let res = warp::test::request()
        .header("range", format!("bytes=100-{}", contents.len() - 1))
        .reply(&file)
        .await;
    assert_eq!(res.status(), 206);
    assert_eq!(
        res.headers()["content-range"],
        format!("bytes 100-{}/{}", contents.len() - 1, contents.len())
    );
    assert_eq!(
        res.headers()["content-length"],
        format!("{}", contents.len() - 100)
    );
    assert_eq!(res.body(), &contents[100..=contents.len() - 1]);
}
