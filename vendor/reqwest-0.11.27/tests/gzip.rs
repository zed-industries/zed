mod support;
use support::server;

use std::io::Write;

#[tokio::test]
async fn gzip_response() {
    gzip_case(10_000, 4096).await;
}

#[tokio::test]
async fn gzip_single_byte_chunks() {
    gzip_case(10, 1).await;
}

#[tokio::test]
async fn test_gzip_empty_body() {
    let server = server::http(move |req| async move {
        assert_eq!(req.method(), "HEAD");

        http::Response::builder()
            .header("content-encoding", "gzip")
            .header("content-length", 100)
            .body(Default::default())
            .unwrap()
    });

    let client = reqwest::Client::new();
    let res = client
        .head(&format!("http://{}/gzip", server.addr()))
        .send()
        .await
        .unwrap();

    let body = res.text().await.unwrap();

    assert_eq!(body, "");
}

#[tokio::test]
async fn test_accept_header_is_not_changed_if_set() {
    let server = server::http(move |req| async move {
        assert_eq!(req.headers()["accept"], "application/json");
        assert!(req.headers()["accept-encoding"]
            .to_str()
            .unwrap()
            .contains("gzip"));
        http::Response::default()
    });

    let client = reqwest::Client::new();

    let res = client
        .get(&format!("http://{}/accept", server.addr()))
        .header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_accept_encoding_header_is_not_changed_if_set() {
    let server = server::http(move |req| async move {
        assert_eq!(req.headers()["accept"], "*/*");
        assert_eq!(req.headers()["accept-encoding"], "identity");
        http::Response::default()
    });

    let client = reqwest::Client::new();

    let res = client
        .get(&format!("http://{}/accept-encoding", server.addr()))
        .header(
            reqwest::header::ACCEPT_ENCODING,
            reqwest::header::HeaderValue::from_static("identity"),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), reqwest::StatusCode::OK);
}

async fn gzip_case(response_size: usize, chunk_size: usize) {
    use futures_util::stream::StreamExt;

    let content: String = (0..response_size)
        .into_iter()
        .map(|i| format!("test {i}"))
        .collect();
    let mut encoder = libflate::gzip::Encoder::new(Vec::new()).unwrap();
    match encoder.write(content.as_bytes()) {
        Ok(n) => assert!(n > 0, "Failed to write to encoder."),
        _ => panic!("Failed to gzip encode string."),
    };

    let gzipped_content = encoder.finish().into_result().unwrap();

    let mut response = format!(
        "\
         HTTP/1.1 200 OK\r\n\
         Server: test-accept\r\n\
         Content-Encoding: gzip\r\n\
         Content-Length: {}\r\n\
         \r\n",
        &gzipped_content.len()
    )
    .into_bytes();
    response.extend(&gzipped_content);

    let server = server::http(move |req| {
        assert!(req.headers()["accept-encoding"]
            .to_str()
            .unwrap()
            .contains("gzip"));

        let gzipped = gzipped_content.clone();
        async move {
            let len = gzipped.len();
            let stream =
                futures_util::stream::unfold((gzipped, 0), move |(gzipped, pos)| async move {
                    let chunk = gzipped.chunks(chunk_size).nth(pos)?.to_vec();

                    Some((chunk, (gzipped, pos + 1)))
                });

            let body = hyper::Body::wrap_stream(stream.map(Ok::<_, std::convert::Infallible>));

            http::Response::builder()
                .header("content-encoding", "gzip")
                .header("content-length", len)
                .body(body)
                .unwrap()
        }
    });

    let client = reqwest::Client::new();

    let res = client
        .get(&format!("http://{}/gzip", server.addr()))
        .send()
        .await
        .expect("response");

    let body = res.text().await.expect("text");
    assert_eq!(body, content);
}
