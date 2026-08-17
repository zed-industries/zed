mod support;
use flate2::write::GzEncoder;
use flate2::Compression;
use support::server;

use std::io::Write;
use tokio::io::AsyncWriteExt;
use tokio::time::Duration;

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

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content.as_bytes()).unwrap();
    let gzipped_content = encoder.finish().unwrap();

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

            let body = reqwest::Body::wrap_stream(stream.map(Ok::<_, std::convert::Infallible>));

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

const COMPRESSED_RESPONSE_HEADERS: &[u8] = b"HTTP/1.1 200 OK\x0d\x0a\
            Content-Type: text/plain\x0d\x0a\
            Connection: keep-alive\x0d\x0a\
            Content-Encoding: gzip\x0d\x0a";

const RESPONSE_CONTENT: &str = "some message here";

fn gzip_compress(input: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input).unwrap();
    encoder.finish().unwrap()
}

#[tokio::test]
async fn test_non_chunked_non_fragmented_response() {
    let server = server::low_level_with_response(|_raw_request, client_socket| {
        Box::new(async move {
            let gzipped_content = gzip_compress(RESPONSE_CONTENT.as_bytes());
            let content_length_header =
                format!("Content-Length: {}\r\n\r\n", gzipped_content.len()).into_bytes();
            let response = [
                COMPRESSED_RESPONSE_HEADERS,
                &content_length_header,
                &gzipped_content,
            ]
            .concat();

            client_socket
                .write_all(response.as_slice())
                .await
                .expect("response write_all failed");
            client_socket.flush().await.expect("response flush failed");
        })
    });

    let res = reqwest::Client::new()
        .get(&format!("http://{}/", server.addr()))
        .send()
        .await
        .expect("response");

    assert_eq!(res.text().await.expect("text"), RESPONSE_CONTENT);
}

#[tokio::test]
async fn test_chunked_fragmented_response_1() {
    const DELAY_BETWEEN_RESPONSE_PARTS: tokio::time::Duration =
        tokio::time::Duration::from_millis(1000);
    const DELAY_MARGIN: tokio::time::Duration = tokio::time::Duration::from_millis(50);

    let server = server::low_level_with_response(|_raw_request, client_socket| {
        Box::new(async move {
            let gzipped_content = gzip_compress(RESPONSE_CONTENT.as_bytes());
            let response_first_part = [
                COMPRESSED_RESPONSE_HEADERS,
                format!(
                    "Transfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                    gzipped_content.len()
                )
                .as_bytes(),
                &gzipped_content,
            ]
            .concat();
            let response_second_part = b"\r\n0\r\n\r\n";

            client_socket
                .write_all(response_first_part.as_slice())
                .await
                .expect("response_first_part write_all failed");
            client_socket
                .flush()
                .await
                .expect("response_first_part flush failed");

            tokio::time::sleep(DELAY_BETWEEN_RESPONSE_PARTS).await;

            client_socket
                .write_all(response_second_part)
                .await
                .expect("response_second_part write_all failed");
            client_socket
                .flush()
                .await
                .expect("response_second_part flush failed");
        })
    });

    let start = tokio::time::Instant::now();
    let res = reqwest::Client::new()
        .get(&format!("http://{}/", server.addr()))
        .send()
        .await
        .expect("response");

    assert_eq!(res.text().await.expect("text"), RESPONSE_CONTENT);
    assert!(start.elapsed() >= DELAY_BETWEEN_RESPONSE_PARTS - DELAY_MARGIN);
}

#[tokio::test]
async fn test_chunked_fragmented_response_2() {
    const DELAY_BETWEEN_RESPONSE_PARTS: tokio::time::Duration =
        tokio::time::Duration::from_millis(1000);
    const DELAY_MARGIN: tokio::time::Duration = tokio::time::Duration::from_millis(50);

    let server = server::low_level_with_response(|_raw_request, client_socket| {
        Box::new(async move {
            let gzipped_content = gzip_compress(RESPONSE_CONTENT.as_bytes());
            let response_first_part = [
                COMPRESSED_RESPONSE_HEADERS,
                format!(
                    "Transfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                    gzipped_content.len()
                )
                .as_bytes(),
                &gzipped_content,
                b"\r\n",
            ]
            .concat();
            let response_second_part = b"0\r\n\r\n";

            client_socket
                .write_all(response_first_part.as_slice())
                .await
                .expect("response_first_part write_all failed");
            client_socket
                .flush()
                .await
                .expect("response_first_part flush failed");

            tokio::time::sleep(DELAY_BETWEEN_RESPONSE_PARTS).await;

            client_socket
                .write_all(response_second_part)
                .await
                .expect("response_second_part write_all failed");
            client_socket
                .flush()
                .await
                .expect("response_second_part flush failed");
        })
    });

    let start = tokio::time::Instant::now();
    let res = reqwest::Client::new()
        .get(&format!("http://{}/", server.addr()))
        .send()
        .await
        .expect("response");

    assert_eq!(res.text().await.expect("text"), RESPONSE_CONTENT);
    assert!(start.elapsed() >= DELAY_BETWEEN_RESPONSE_PARTS - DELAY_MARGIN);
}

#[tokio::test]
async fn test_chunked_fragmented_response_with_extra_bytes() {
    const DELAY_BETWEEN_RESPONSE_PARTS: tokio::time::Duration =
        tokio::time::Duration::from_millis(1000);
    const DELAY_MARGIN: tokio::time::Duration = tokio::time::Duration::from_millis(50);

    let server = server::low_level_with_response(|_raw_request, client_socket| {
        Box::new(async move {
            let gzipped_content = gzip_compress(RESPONSE_CONTENT.as_bytes());
            let response_first_part = [
                COMPRESSED_RESPONSE_HEADERS,
                format!(
                    "Transfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                    gzipped_content.len()
                )
                .as_bytes(),
                &gzipped_content,
            ]
            .concat();
            let response_second_part = b"\r\n2ab\r\n0\r\n\r\n";

            client_socket
                .write_all(response_first_part.as_slice())
                .await
                .expect("response_first_part write_all failed");
            client_socket
                .flush()
                .await
                .expect("response_first_part flush failed");

            tokio::time::sleep(DELAY_BETWEEN_RESPONSE_PARTS).await;

            client_socket
                .write_all(response_second_part)
                .await
                .expect("response_second_part write_all failed");
            client_socket
                .flush()
                .await
                .expect("response_second_part flush failed");
        })
    });

    let start = tokio::time::Instant::now();
    let res = reqwest::Client::new()
        .get(&format!("http://{}/", server.addr()))
        .send()
        .await
        .expect("response");

    let err = res.text().await.expect_err("there must be an error");
    assert!(err.is_decode());
    assert!(start.elapsed() >= DELAY_BETWEEN_RESPONSE_PARTS - DELAY_MARGIN);
}
