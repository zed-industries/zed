mod support;
use std::io::Read;
use support::server;
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn brotli_response() {
    brotli_case(10_000, 4096).await;
}

#[tokio::test]
async fn brotli_single_byte_chunks() {
    brotli_case(10, 1).await;
}

#[tokio::test]
async fn test_brotli_empty_body() {
    let server = server::http(move |req| async move {
        assert_eq!(req.method(), "HEAD");

        http::Response::builder()
            .header("content-encoding", "br")
            .body(Default::default())
            .unwrap()
    });

    let client = reqwest::Client::new();
    let res = client
        .head(&format!("http://{}/brotli", server.addr()))
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
            .contains("br"));
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

async fn brotli_case(response_size: usize, chunk_size: usize) {
    use futures_util::stream::StreamExt;

    let content: String = (0..response_size)
        .into_iter()
        .map(|i| format!("test {i}"))
        .collect();

    let mut encoder = brotli_crate::CompressorReader::new(content.as_bytes(), 4096, 5, 20);
    let mut brotlied_content = Vec::new();
    encoder.read_to_end(&mut brotlied_content).unwrap();

    let mut response = format!(
        "\
         HTTP/1.1 200 OK\r\n\
         Server: test-accept\r\n\
         Content-Encoding: br\r\n\
         Content-Length: {}\r\n\
         \r\n",
        &brotlied_content.len()
    )
    .into_bytes();
    response.extend(&brotlied_content);

    let server = server::http(move |req| {
        assert!(req.headers()["accept-encoding"]
            .to_str()
            .unwrap()
            .contains("br"));

        let brotlied = brotlied_content.clone();
        async move {
            let len = brotlied.len();
            let stream =
                futures_util::stream::unfold((brotlied, 0), move |(brotlied, pos)| async move {
                    let chunk = brotlied.chunks(chunk_size).nth(pos)?.to_vec();

                    Some((chunk, (brotlied, pos + 1)))
                });

            let body = reqwest::Body::wrap_stream(stream.map(Ok::<_, std::convert::Infallible>));

            http::Response::builder()
                .header("content-encoding", "br")
                .header("content-length", len)
                .body(body)
                .unwrap()
        }
    });

    let client = reqwest::Client::new();

    let res = client
        .get(&format!("http://{}/brotli", server.addr()))
        .send()
        .await
        .expect("response");

    let body = res.text().await.expect("text");
    assert_eq!(body, content);
}

const COMPRESSED_RESPONSE_HEADERS: &[u8] = b"HTTP/1.1 200 OK\x0d\x0a\
            Content-Type: text/plain\x0d\x0a\
            Connection: keep-alive\x0d\x0a\
            Content-Encoding: br\x0d\x0a";

const RESPONSE_CONTENT: &str = "some message here";

fn brotli_compress(input: &[u8]) -> Vec<u8> {
    let mut encoder = brotli_crate::CompressorReader::new(input, 4096, 5, 20);
    let mut brotlied_content = Vec::new();
    encoder.read_to_end(&mut brotlied_content).unwrap();
    brotlied_content
}

#[tokio::test]
async fn test_non_chunked_non_fragmented_response() {
    let server = server::low_level_with_response(|_raw_request, client_socket| {
        Box::new(async move {
            let brotlied_content = brotli_compress(RESPONSE_CONTENT.as_bytes());
            let content_length_header =
                format!("Content-Length: {}\r\n\r\n", brotlied_content.len()).into_bytes();
            let response = [
                COMPRESSED_RESPONSE_HEADERS,
                &content_length_header,
                &brotlied_content,
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
            let brotlied_content = brotli_compress(RESPONSE_CONTENT.as_bytes());
            let response_first_part = [
                COMPRESSED_RESPONSE_HEADERS,
                format!(
                    "Transfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                    brotlied_content.len()
                )
                .as_bytes(),
                &brotlied_content,
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
            let brotlied_content = brotli_compress(RESPONSE_CONTENT.as_bytes());
            let response_first_part = [
                COMPRESSED_RESPONSE_HEADERS,
                format!(
                    "Transfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                    brotlied_content.len()
                )
                .as_bytes(),
                &brotlied_content,
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
            let brotlied_content = brotli_compress(RESPONSE_CONTENT.as_bytes());
            let response_first_part = [
                COMPRESSED_RESPONSE_HEADERS,
                format!(
                    "Transfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                    brotlied_content.len()
                )
                .as_bytes(),
                &brotlied_content,
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
