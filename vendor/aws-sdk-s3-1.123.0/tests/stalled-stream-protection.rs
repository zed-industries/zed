/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_credential_types::Credentials;
use aws_sdk_s3::{
    config::{Region, StalledStreamProtectionConfig},
    error::BoxError,
};
use aws_sdk_s3::{error::DisplayErrorContext, primitives::ByteStream};
use aws_sdk_s3::{Client, Config};
use aws_smithy_runtime::{assert_str_contains, test_util::capture_test_logs::capture_test_logs};
use aws_smithy_types::body::SdkBody;
use bytes::{Bytes, BytesMut};
use http_body_1x::Body;
use std::error::Error;
use std::time::Duration;
use std::{future::Future, task::Poll};
use std::{net::SocketAddr, pin::Pin, task::Context};
use tokio::{
    net::{TcpListener, TcpStream},
    time::sleep,
};
use tracing::debug;

enum SlowBodyState {
    Wait(Pin<Box<dyn std::future::Future<Output = ()> + Send + Sync + 'static>>),
    Send,
    Taken,
}

struct SlowBody {
    state: SlowBodyState,
}

impl SlowBody {
    fn new() -> Self {
        Self {
            state: SlowBodyState::Send,
        }
    }
}

impl Body for SlowBody {
    type Data = Bytes;
    type Error = BoxError;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body_1x::Frame<Self::Data>, Self::Error>>> {
        loop {
            let mut state = SlowBodyState::Taken;
            std::mem::swap(&mut state, &mut self.state);
            match state {
                SlowBodyState::Wait(mut fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(_) => self.state = SlowBodyState::Send,
                    Poll::Pending => {
                        self.state = SlowBodyState::Wait(fut);
                        return Poll::Pending;
                    }
                },
                SlowBodyState::Send => {
                    self.state = SlowBodyState::Wait(Box::pin(sleep(Duration::from_micros(100))));
                    return Poll::Ready(Some(Ok(http_body_1x::Frame::data(Bytes::from_static(
                        b"data_data_data_data_data_data_data_data_data_data_data_data_\
                          data_data_data_data_data_data_data_data_data_data_data_data_\
                          data_data_data_data_data_data_data_data_data_data_data_data_\
                          data_data_data_data_data_data_data_data_data_data_data_data_",
                    )))));
                }
                SlowBodyState::Taken => unreachable!(),
            }
        }
    }
}

#[tokio::test]
async fn test_stalled_stream_protection_defaults_for_upload() {
    let _logs = capture_test_logs();

    // We spawn a faulty server that will stop all request processing after reading half of the request body.
    let (server, server_addr) = start_faulty_upload_server().await;
    let _ = tokio::spawn(server);

    let conf = Config::builder()
        // Stalled stream protection MUST BE enabled by default. Do not configure it explicitly.
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .endpoint_url(format!("http://{server_addr}"))
        // The Body used here is odd and fails the body.size_hint().exact() check in the streaming branch of
        // the `RequestChecksumInterceptor`
        .request_checksum_calculation(aws_sdk_s3::config::RequestChecksumCalculation::WhenRequired)
        .build();
    let client = Client::from_conf(conf);

    let err = client
        .put_object()
        .bucket("a-test-bucket")
        .key("stalled-stream-test.txt")
        .body(ByteStream::new(SdkBody::from_body_1_x(SlowBody::new())))
        .send()
        .await
        .expect_err("upload stream stalled out");

    let err_msg = DisplayErrorContext(&err).to_string();
    assert_str_contains!(
        err_msg,
        "minimum throughput was specified at 1 B/s, but throughput of 0 B/s was observed"
    );
}

async fn start_faulty_upload_server() -> (impl Future<Output = ()>, SocketAddr) {
    let listener = TcpListener::bind("0.0.0.0:0")
        .await
        .expect("socket is free");
    let bind_addr = listener.local_addr().unwrap();

    async fn process_socket(socket: TcpStream) {
        let mut buf = BytesMut::new();
        let mut time_to_stall = false;

        while !time_to_stall {
            match socket.try_read_buf(&mut buf) {
                Ok(0) => {
                    unreachable!(
                        "The connection will be closed before this branch is ever reached"
                    );
                }
                Ok(n) => {
                    debug!("read {n} bytes from the socket");
                    if buf.len() >= 128 {
                        time_to_stall = true;
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
        }

        debug!("faulty server has read partial request, now getting stuck");
        loop {
            tokio::task::yield_now().await
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

#[tokio::test]
async fn test_explicitly_configured_stalled_stream_protection_for_downloads() {
    // We spawn a faulty server that will close the connection after
    // writing half of the response body.
    let (server, server_addr) = start_faulty_download_server().await;
    let _ = tokio::spawn(server);

    let conf = Config::builder()
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .endpoint_url(format!("http://{server_addr}"))
        .stalled_stream_protection(
            StalledStreamProtectionConfig::enabled()
                // Fail stalled streams immediately
                .grace_period(Duration::from_secs(0))
                .build(),
        )
        .build();
    let client = Client::from_conf(conf);

    let res = client
        .get_object()
        .bucket("a-test-bucket")
        .key("stalled-stream-test.txt")
        .send()
        .await
        .unwrap();

    let err = res
        .body
        .collect()
        .await
        .expect_err("download stream stalled out");
    let err = err.source().expect("inner error exists");
    assert_eq!(
        err.to_string(),
        "minimum throughput was specified at 1 B/s, but throughput of 0 B/s was observed"
    );
}

#[tokio::test]
async fn test_stalled_stream_protection_for_downloads_can_be_disabled() {
    // We spawn a faulty server that will close the connection after
    // writing half of the response body.
    let (server, server_addr) = start_faulty_download_server().await;
    let _ = tokio::spawn(server);

    let conf = Config::builder()
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .endpoint_url(format!("http://{server_addr}"))
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .build();
    let client = Client::from_conf(conf);

    let res = client
        .get_object()
        .bucket("a-test-bucket")
        .key("stalled-stream-test.txt")
        .send()
        .await
        .unwrap();

    let timeout_duration = Duration::from_secs(2);
    match tokio::time::timeout(timeout_duration, res.body.collect()).await {
        Ok(_) => panic!("stalled stream protection kicked in but it shouldn't have"),
        // If timeout elapses, then stalled stream protection didn't end the stream early.
        Err(elapsed) => assert_eq!("deadline has elapsed".to_owned(), elapsed.to_string()),
    }
}

// This test will always take as long as whatever grace period is set by default.
#[tokio::test]
async fn test_stalled_stream_protection_for_downloads_is_enabled_by_default() {
    // We spawn a faulty server that will close the connection after
    // writing half of the response body.
    let (server, server_addr) = start_faulty_download_server().await;
    let _ = tokio::spawn(server);

    // Stalled stream protection should be enabled by default.
    let sdk_config = aws_config::from_env()
        // Stalled stream protection MUST BE enabled by default. Do not configure it explicitly.
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .endpoint_url(format!("http://{server_addr}"))
        .load()
        .await;
    let client = Client::new(&sdk_config);

    let res = client
        .get_object()
        .bucket("a-test-bucket")
        .key("stalled-stream-test.txt")
        .send()
        .await
        .unwrap();

    let start = std::time::Instant::now();
    let err = res
        .body
        .collect()
        .await
        .expect_err("download stream stalled out");
    let err = err.source().expect("inner error exists");
    assert_eq!(
        err.to_string(),
        "minimum throughput was specified at 1 B/s, but throughput of 0 B/s was observed"
    );
    // 5s grace period
    let elapsed_secs = start.elapsed().as_secs();
    assert!(
        elapsed_secs == 5,
        "elapsed secs should be 5, but was {elapsed_secs}"
    )
}

async fn start_faulty_download_server() -> (impl Future<Output = ()>, SocketAddr) {
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

"#;
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
                debug!("faulty server has written partial response, now getting stuck");
                break;
            }
        }

        loop {
            tokio::task::yield_now().await
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
