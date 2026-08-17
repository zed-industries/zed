mod support;
use support::server;
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn zstd_response() {
    zstd_case(10_000, 4096).await;
}

#[tokio::test]
async fn zstd_single_byte_chunks() {
    zstd_case(10, 1).await;
}

#[tokio::test]
async fn test_zstd_empty_body() {
    let server = server::http(move |req| async move {
        assert_eq!(req.method(), "HEAD");

        http::Response::builder()
            .header("content-encoding", "zstd")
            .body(Default::default())
            .unwrap()
    });

    let client = reqwest::Client::new();
    let res = client
        .head(&format!("http://{}/zstd", server.addr()))
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
            .contains("zstd"));
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

async fn zstd_case(response_size: usize, chunk_size: usize) {
    use futures_util::stream::StreamExt;

    let content: String = (0..response_size)
        .into_iter()
        .map(|i| format!("test {i}"))
        .collect();

    let zstded_content = zstd_crate::encode_all(content.as_bytes(), 3).unwrap();

    let mut response = format!(
        "\
         HTTP/1.1 200 OK\r\n\
         Server: test-accept\r\n\
         Content-Encoding: zstd\r\n\
         Content-Length: {}\r\n\
         \r\n",
        &zstded_content.len()
    )
    .into_bytes();
    response.extend(&zstded_content);

    let server = server::http(move |req| {
        assert!(req.headers()["accept-encoding"]
            .to_str()
            .unwrap()
            .contains("zstd"));

        let zstded = zstded_content.clone();
        async move {
            let len = zstded.len();
            let stream =
                futures_util::stream::unfold((zstded, 0), move |(zstded, pos)| async move {
                    let chunk = zstded.chunks(chunk_size).nth(pos)?.to_vec();

                    Some((chunk, (zstded, pos + 1)))
                });

            let body = reqwest::Body::wrap_stream(stream.map(Ok::<_, std::convert::Infallible>));

            http::Response::builder()
                .header("content-encoding", "zstd")
                .header("content-length", len)
                .body(body)
                .unwrap()
        }
    });

    let client = reqwest::Client::new();

    let res = client
        .get(&format!("http://{}/zstd", server.addr()))
        .send()
        .await
        .expect("response");

    let body = res.text().await.expect("text");
    assert_eq!(body, content);
}

const COMPRESSED_RESPONSE_HEADERS: &[u8] = b"HTTP/1.1 200 OK\x0d\x0a\
            Content-Type: text/plain\x0d\x0a\
            Connection: keep-alive\x0d\x0a\
            Content-Encoding: zstd\x0d\x0a";

const RESPONSE_CONTENT: &str = "some message here";

fn zstd_compress(input: &[u8]) -> Vec<u8> {
    zstd_crate::encode_all(input, 3).unwrap()
}

#[tokio::test]
async fn test_non_chunked_non_fragmented_response() {
    let server = server::low_level_with_response(|_raw_request, client_socket| {
        Box::new(async move {
            let zstded_content = zstd_compress(RESPONSE_CONTENT.as_bytes());
            let content_length_header =
                format!("Content-Length: {}\r\n\r\n", zstded_content.len()).into_bytes();
            let response = [
                COMPRESSED_RESPONSE_HEADERS,
                &content_length_header,
                &zstded_content,
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

// Big response can have multiple ZSTD frames in it
#[tokio::test]
async fn test_non_chunked_non_fragmented_multiple_frames_response() {
    let server = server::low_level_with_response(|_raw_request, client_socket| {
        Box::new(async move {
            // Split the content into two parts
            let content_bytes = RESPONSE_CONTENT.as_bytes();
            let mid = content_bytes.len() / 2;
            // Compress each part separately to create multiple ZSTD frames
            let compressed_part1 = zstd_crate::encode_all(&content_bytes[0..mid], 3).unwrap();
            let compressed_part2 = zstd_crate::encode_all(&content_bytes[mid..], 3).unwrap();
            // Concatenate the compressed frames
            let mut zstded_content = compressed_part1;
            zstded_content.extend_from_slice(&compressed_part2);
            // Set Content-Length to the total length of the concatenated frames
            let content_length_header =
                format!("Content-Length: {}\r\n\r\n", zstded_content.len()).into_bytes();
            let response = [
                COMPRESSED_RESPONSE_HEADERS,
                &content_length_header,
                &zstded_content,
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
        .get(format!("http://{}/", server.addr()))
        .send()
        .await
        .expect("response");

    assert_eq!(res.text().await.expect("text"), RESPONSE_CONTENT);
}

#[tokio::test]
async fn test_chunked_fragmented_multiple_frames_in_one_chunk() {
    // Define constants for delay and timing margin
    const DELAY_BETWEEN_RESPONSE_PARTS: tokio::time::Duration =
        tokio::time::Duration::from_millis(1000); // 1-second delay
    const DELAY_MARGIN: tokio::time::Duration = tokio::time::Duration::from_millis(50); // Margin for timing assertions

    // Set up a low-level server
    let server = server::low_level_with_response(|_raw_request, client_socket| {
        Box::new(async move {
            // Split RESPONSE_CONTENT into two parts
            let mid = RESPONSE_CONTENT.len() / 2;
            let part1 = &RESPONSE_CONTENT[0..mid];
            let part2 = &RESPONSE_CONTENT[mid..];

            // Compress each part separately to create two ZSTD frames
            let compressed_part1 = zstd_compress(part1.as_bytes());
            let compressed_part2 = zstd_compress(part2.as_bytes());

            // Concatenate the frames into a single chunk's data
            let chunk_data = [compressed_part1.as_slice(), compressed_part2.as_slice()].concat();

            // Calculate the chunk size in bytes
            let chunk_size = chunk_data.len();

            // Prepare the initial response part: headers + chunk size
            let headers = [
                COMPRESSED_RESPONSE_HEADERS, // e.g., "HTTP/1.1 200 OK\r\nContent-Encoding: zstd\r\n"
                b"Transfer-Encoding: chunked\r\n\r\n", // Indicate chunked encoding
                format!("{:x}\r\n", chunk_size).as_bytes(), // Chunk size in hex
            ]
            .concat();

            // Send headers + chunk size + chunk data
            client_socket
                .write_all([headers.as_slice(), &chunk_data].concat().as_slice())
                .await
                .expect("write_all failed");
            client_socket.flush().await.expect("flush failed");

            // Introduce a delay to simulate fragmentation
            tokio::time::sleep(DELAY_BETWEEN_RESPONSE_PARTS).await;

            // Send chunk terminator + final chunk
            client_socket
                .write_all(b"\r\n0\r\n\r\n")
                .await
                .expect("write_all failed");
            client_socket.flush().await.expect("flush failed");
        })
    });

    // Record the start time for delay verification
    let start = tokio::time::Instant::now();

    let res = reqwest::Client::new()
        .get(format!("http://{}/", server.addr()))
        .send()
        .await
        .expect("Failed to get response");

    // Verify the decompressed response matches the original content
    assert_eq!(
        res.text().await.expect("Failed to read text"),
        RESPONSE_CONTENT
    );
    assert!(start.elapsed() >= DELAY_BETWEEN_RESPONSE_PARTS - DELAY_MARGIN);
}

#[tokio::test]
async fn test_connection_reuse_with_chunked_fragmented_multiple_frames_in_one_chunk() {
    // Define constants for delay and timing margin
    const DELAY_BETWEEN_RESPONSE_PARTS: tokio::time::Duration =
        tokio::time::Duration::from_millis(1000); // 1-second delay
    const DELAY_MARGIN: tokio::time::Duration = tokio::time::Duration::from_millis(50); // Margin for timing assertions

    // We will record the peer addresses of each client request here
    let peer_addrs = std::sync::Arc::new(std::sync::Mutex::new(Vec::<std::net::SocketAddr>::new()));
    let peer_addrs_clone = peer_addrs.clone();

    // Set up a low-level server (it will reuse existing client connection, executing callback for each client request)
    let server = server::low_level_with_response(move |_raw_request, client_socket| {
        let peer_addrs = peer_addrs_clone.clone();
        Box::new(async move {
            // Split RESPONSE_CONTENT into two parts
            let mid = RESPONSE_CONTENT.len() / 2;
            let part1 = &RESPONSE_CONTENT[0..mid];
            let part2 = &RESPONSE_CONTENT[mid..];

            // Compress each part separately to create two ZSTD frames
            let compressed_part1 = zstd_compress(part1.as_bytes());
            let compressed_part2 = zstd_compress(part2.as_bytes());

            // Concatenate the frames into a single chunk's data
            let chunk_data = [compressed_part1.as_slice(), compressed_part2.as_slice()].concat();

            // Calculate the chunk size in bytes
            let chunk_size = chunk_data.len();

            // Prepare the initial response part: headers + chunk size
            let headers = [
                COMPRESSED_RESPONSE_HEADERS, // e.g., "HTTP/1.1 200 OK\r\nContent-Encoding: zstd\r\n"
                b"Transfer-Encoding: chunked\r\n\r\n", // Indicate chunked encoding
                format!("{:x}\r\n", chunk_size).as_bytes(), // Chunk size in hex
            ]
            .concat();

            // Send headers + chunk size + chunk data
            client_socket
                .write_all([headers.as_slice(), &chunk_data].concat().as_slice())
                .await
                .expect("write_all failed");
            client_socket.flush().await.expect("flush failed");

            // Introduce a delay to simulate fragmentation
            tokio::time::sleep(DELAY_BETWEEN_RESPONSE_PARTS).await;

            peer_addrs
                .lock()
                .unwrap()
                .push(client_socket.peer_addr().unwrap());

            // Send chunk terminator + final chunk
            client_socket
                .write_all(b"\r\n0\r\n\r\n")
                .await
                .expect("write_all failed");
            client_socket.flush().await.expect("flush failed");
        })
    });

    let client = reqwest::Client::builder()
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .pool_max_idle_per_host(1)
        .build()
        .unwrap();

    const NUMBER_OF_REQUESTS: usize = 5;

    for _ in 0..NUMBER_OF_REQUESTS {
        // Record the start time for delay verification
        let start = tokio::time::Instant::now();

        let res = client
            .get(format!("http://{}/", server.addr()))
            .send()
            .await
            .expect("Failed to get response");

        // Verify the decompressed response matches the original content
        assert_eq!(
            res.text().await.expect("Failed to read text"),
            RESPONSE_CONTENT
        );
        assert!(start.elapsed() >= DELAY_BETWEEN_RESPONSE_PARTS - DELAY_MARGIN);
    }

    drop(client);

    // Check that all peer addresses are the same
    let peer_addrs = peer_addrs.lock().unwrap();
    assert_eq!(
        peer_addrs.len(),
        NUMBER_OF_REQUESTS,
        "Expected {} peer addresses, but got {}",
        NUMBER_OF_REQUESTS,
        peer_addrs.len()
    );
    let first_addr = peer_addrs[0];
    assert!(
        peer_addrs.iter().all(|addr| addr == &first_addr),
        "All peer addresses should be the same, but found differences: {:?}",
        peer_addrs
    );
}

#[tokio::test]
async fn test_chunked_fragmented_response_1() {
    const DELAY_BETWEEN_RESPONSE_PARTS: tokio::time::Duration =
        tokio::time::Duration::from_millis(1000);
    const DELAY_MARGIN: tokio::time::Duration = tokio::time::Duration::from_millis(50);

    let server = server::low_level_with_response(|_raw_request, client_socket| {
        Box::new(async move {
            let zstded_content = zstd_compress(RESPONSE_CONTENT.as_bytes());
            let response_first_part = [
                COMPRESSED_RESPONSE_HEADERS,
                format!(
                    "Transfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                    zstded_content.len()
                )
                .as_bytes(),
                &zstded_content,
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
            let zstded_content = zstd_compress(RESPONSE_CONTENT.as_bytes());
            let response_first_part = [
                COMPRESSED_RESPONSE_HEADERS,
                format!(
                    "Transfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                    zstded_content.len()
                )
                .as_bytes(),
                &zstded_content,
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
            let zstded_content = zstd_compress(RESPONSE_CONTENT.as_bytes());
            let response_first_part = [
                COMPRESSED_RESPONSE_HEADERS,
                format!(
                    "Transfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                    zstded_content.len()
                )
                .as_bytes(),
                &zstded_content,
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
