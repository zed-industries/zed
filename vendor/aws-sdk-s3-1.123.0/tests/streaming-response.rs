/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::error::DisplayErrorContext;
use aws_sdk_s3::Client;
use aws_smithy_http_client::test_util::infallible_client_fn;
use aws_smithy_runtime::assert_str_contains;
use aws_smithy_types::body::SdkBody;
use bytes::BytesMut;
use http_1x::header::CONTENT_LENGTH;
use std::future::Future;
use std::net::SocketAddr;
use std::time::Duration;
use tracing::debug;

#[tokio::test]
async fn test_too_short_body_causes_an_error() {
    // this is almost impossible to reproduce with Hyper—you need to do stuff like run each request
    // in its own async runtime. But there's no reason a customer couldn't run their _own_ HttpClient
    // that was more poorly behaved, so we'll do that here.
    let http_client = infallible_client_fn(|_req| {
        http_1x::Response::builder()
            .header(CONTENT_LENGTH, 5000)
            .body(SdkBody::from("definitely not 5000 characters"))
            .unwrap()
    });

    let client = aws_sdk_s3::Client::from_conf(
        aws_sdk_s3::Config::builder()
            .with_test_defaults()
            .region(Region::new("us-east-1"))
            .http_client(http_client)
            .build(),
    );

    let content = client
        .get_object()
        .bucket("some-test-bucket")
        .key("test.txt")
        .send()
        .await
        .unwrap()
        .body;
    let error = content.collect().await.expect_err("content too short");
    assert_str_contains!(
        format!("{}", DisplayErrorContext(error)),
        "Invalid Content-Length: Expected 5000 bytes but 30 bytes were received"
    );
}

// test will hang forever with the default (single-threaded) test executor
#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_response_fails_when_eof_comes_before_content_length_reached() {
    // We spawn a faulty server that will close the connection after
    // writing half of the response body.
    let (server, server_addr) = start_faulty_server().await;
    let _ = tokio::spawn(server);

    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::new("us-east-1"))
        .endpoint_url(format!("http://{server_addr}"))
        .build();

    let client = Client::new(&sdk_config);

    // This will succeed b/c the head of the response is fine.
    let res = client
        .get_object()
        .bucket("some-test-bucket")
        .key("test.txt")
        .send()
        .await
        .unwrap();

    // Should panic here when the body is read with an "UnexpectedEof" error
    if let Err(e) = res.body.collect().await {
        let message = format!("{}", DisplayErrorContext(e));
        let expected =
            "error reading a body from connection: end of file before message length reached";
        assert!(
            message.contains(expected),
            "Expected `{message}` to contain `{expected}`"
        );
    } else {
        panic!("response did not error")
    }
}

async fn start_faulty_server() -> (impl Future<Output = ()>, SocketAddr) {
    use tokio::net::{TcpListener, TcpStream};
    use tokio::time::sleep;

    let listener = TcpListener::bind("0.0.0.0:0")
        .await
        .expect("socket is free");
    let bind_addr = listener.local_addr().unwrap();

    async fn process_socket(socket: TcpStream) {
        let mut buf = BytesMut::new();
        let response: &[u8] = br#"HTTP/1.1 200 OK
x-amz-request-id: 4B4NGF0EAWN0GE63
content-length: 12
etag: 3e25960a79dbc69b674cd4ec67a72c62
content-type: application/octet-stream
server: AmazonS3
content-encoding:
last-modified: Tue, 21 Jun 2022 16:29:14 GMT
date: Tue, 21 Jun 2022 16:29:23 GMT
x-amz-id-2: kPl+IVVZAwsN8ePUyQJZ40WD9dzaqtr4eNESArqE68GSKtVvuvCTDe+SxhTT+JTUqXB1HL4OxNM=
accept-ranges: bytes

Hello"#;
        let mut time_to_respond = false;

        loop {
            match socket.try_read_buf(&mut buf) {
                Ok(0) => {
                    unreachable!(
                        "The connection will be closed before this branch is ever reached"
                    );
                }
                Ok(n) => {
                    debug!("read {n} bytes from the socket");
                    if let Ok(s) = std::str::from_utf8(&buf) {
                        debug!("buf currently looks like:\n{s:?}");
                    }

                    // Check for CRLF to see if we've received the entire HTTP request.
                    if buf.ends_with(b"\r\n\r\n") {
                        time_to_respond = true;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    debug!("reading would block, sleeping for 1ms and then trying again");
                    sleep(Duration::from_millis(1)).await;
                }
                Err(e) => {
                    panic!("{e}")
                }
            }

            if socket.writable().await.is_ok() && time_to_respond {
                // The content length is 12 but we'll only write 5 bytes
                socket.try_write(response).unwrap();
                // We break from the R/W loop after sending a partial response in order to
                // close the connection early.
                debug!("faulty server has written partial response, now closing connection");
                break;
            }
        }
    }

    let fut = async move {
        loop {
            let (socket, addr) = listener
                .accept()
                .await
                .expect("listener can accept new connections");
            debug!("server received new connection from {addr:?}");
            let start = std::time::Instant::now();
            process_socket(socket).await;
            debug!(
                "connection to {addr:?} closed after {:.02?}",
                start.elapsed()
            );
        }
    };

    (fut, bind_addr)
}
