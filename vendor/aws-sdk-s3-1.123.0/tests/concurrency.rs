/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use aws_smithy_types::timeout::TimeoutConfig;
use aws_types::region::Region;
use aws_types::SdkConfig;
use bytes::BytesMut;
use hdrhistogram::sync::SyncHistogram;
use hdrhistogram::Histogram;
use std::future::Future;
use std::iter::repeat_with;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, Instant};
use tracing::debug;

// WARNING:
// When testing this on your own computer, be sure to run the tests in several different terminals.
// Depending on the terminal used, you may run into errors related to "Too many open files".

const TASK_COUNT: usize = 1_000;
// Larger requests take longer to send, which means we'll consume more network resources per
// request, which means we can't support as many concurrent connections to S3.
const TASK_PAYLOAD_LENGTH: usize = 5_000;
// At 130 and above, this test will fail with a `ConnectorError` from `hyper`. I've seen:
// - ConnectorError { kind: Io, source: hyper::Error(Canceled, hyper::Error(Io, Os { code: 54, kind: ConnectionReset, message: "Connection reset by peer" })) }
// - ConnectorError { kind: Io, source: hyper::Error(BodyWrite, Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }) }
// These errors don't necessarily occur when actually running against S3 with concurrency levels
// above 129. You can test it for yourself by running the
// `test_concurrency_put_object_against_live` test that appears at the bottom of this file.
const CONCURRENCY_LIMIT: usize = 50;

#[tokio::test(flavor = "multi_thread")]
async fn test_concurrency_on_multi_thread_against_dummy_server() {
    let (server, server_addr) = start_agreeable_server().await;
    let _ = tokio::spawn(server);
    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::new("us-east-1"))
        .endpoint_url(format!("http://{server_addr}"))
        .build();

    test_concurrency(sdk_config).await;
}

#[tokio::test(flavor = "current_thread")]
async fn test_concurrency_on_single_thread_against_dummy_server() {
    let (server, server_addr) = start_agreeable_server().await;
    let _ = tokio::spawn(server);
    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::new("us-east-1"))
        .endpoint_url(format!("http://{server_addr}"))
        .build();

    test_concurrency(sdk_config).await;
}

#[ignore = "this test runs against S3 and requires credentials"]
#[tokio::test(flavor = "multi_thread")]
async fn test_concurrency_on_multi_thread_against_s3() {
    let sdk_config = aws_config::from_env()
        .timeout_config(
            TimeoutConfig::builder()
                .connect_timeout(Duration::from_secs(30))
                .read_timeout(Duration::from_secs(30))
                .build(),
        )
        .load()
        .await;

    test_concurrency(sdk_config).await;
}

#[derive(Clone, Copy)]
enum State {
    Listening,
    Speaking,
}

// This server is agreeable because it always replies with `OK`
async fn start_agreeable_server() -> (impl Future<Output = ()>, SocketAddr) {
    use tokio::net::{TcpListener, TcpStream};
    use tokio::time::sleep;

    let listener = TcpListener::bind("0.0.0.0:0")
        .await
        .expect("socket is free");
    let bind_addr = listener.local_addr().unwrap();
    async fn handle_tcp_stream(tcp_stream: TcpStream) {
        let mut buf = BytesMut::new();
        let mut state = State::Listening;

        let response: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
        let mut bytes_left_to_write = response.len();

        loop {
            match state {
                State::Listening => {
                    match tcp_stream.try_read_buf(&mut buf) {
                        Ok(_) => {
                            // Check for CRLF to see if we've received the entire HTTP request.
                            let s = String::from_utf8_lossy(&buf);
                            if let Some(content_length) = discern_content_length(&s) {
                                if let Some(body_length) = discern_body_length(&s) {
                                    if body_length == content_length {
                                        state = State::Speaking;
                                    }
                                }
                            }
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            // reading would block, sleeping for 1ms and then trying again
                            sleep(Duration::from_millis(1)).await;
                        }
                        Err(err) => {
                            panic!("{}", err)
                        }
                    }
                }
                State::Speaking => {
                    if tcp_stream.writable().await.is_ok() {
                        let bytes_written = tcp_stream.try_write(response).unwrap();
                        bytes_left_to_write -= bytes_written;
                        if bytes_left_to_write == 0 {
                            break;
                        }
                    }
                }
            }
        }
    }

    let fut = async move {
        loop {
            let (tcp_stream, _addr) = listener
                .accept()
                .await
                .expect("listener can accept new connections");
            handle_tcp_stream(tcp_stream).await;
        }
    };

    (fut, bind_addr)
}

fn discern_content_length(s: &str) -> Option<usize> {
    // split on newlines
    s.split("\r\n")
        // throw out all lines that aren't the content-length header
        .find(|s| s.contains("content-length: "))
        // attempt to parse the numeric part of the header as a usize
        .and_then(|s| s.trim_start_matches("content-length: ").parse().ok())
}

fn discern_body_length(s: &str) -> Option<usize> {
    // If the request doesn't have a double CRLF, then we haven't finished reading it yet
    if !s.contains("\r\n\r\n") {
        return None;
    }
    // starting from end, split on the double CRLF that separates the body from the header
    s.rsplit("\r\n\r\n")
        // get the body, which must be the first element (we don't send trailers with PutObject requests)
        .next()
        // get the length of the body, in bytes, being sure to trim off the final newline
        .map(|s| s.trim_end().len())
}

async fn test_concurrency(sdk_config: SdkConfig) {
    let client = Client::new(&sdk_config);

    let histogram =
        Histogram::new_with_bounds(1, Duration::from_secs(60 * 60).as_nanos() as u64, 3)
            .unwrap()
            .into_sync();

    debug!("creating futures");
    // This semaphore ensures we only run up to <CONCURRENCY_LIMIT> requests at once.
    let semaphore = Arc::new(Semaphore::new(CONCURRENCY_LIMIT));
    let futures = (0..TASK_COUNT).map(|i| {
        let client = client.clone();
        let key = format!("concurrency/test_object_{:05}", i);
        let body: Vec<_> = repeat_with(fastrand::alphanumeric)
            .take(TASK_PAYLOAD_LENGTH)
            .map(|c| c as u8)
            .collect();
        let fut = client
            .put_object()
            .bucket("your-test-bucket-here")
            .key(key)
            .body(body.into())
            .send();
        // make a clone of the semaphore and the recorder that can live in the future
        let semaphore = semaphore.clone();
        let mut histogram_recorder = histogram.recorder();

        // because we wait on a permit from the semaphore, only <CONCURRENCY_LIMIT> futures
        // will be run at once. Otherwise, we'd quickly get rate-limited by S3.
        async move {
            let permit = semaphore
                .acquire()
                .await
                .expect("we'll get one if we wait long enough");
            let start = Instant::now();
            let res = fut.await.expect("request should succeed");
            histogram_recorder.saturating_record(start.elapsed().as_nanos() as u64);
            drop(permit);
            res
        }
    });

    debug!("joining futures");
    let res: Vec<_> = ::futures_util::future::join_all(futures).await;
    // Assert we ran all the tasks
    assert_eq!(TASK_COUNT, res.len());

    display_metrics(
        "Request Latency",
        histogram,
        "s",
        Duration::from_secs(1).as_nanos() as f64,
    );
}

fn display_metrics(name: &str, mut h: SyncHistogram<u64>, unit: &str, scale: f64) {
    // Refreshing is required or else we won't see any results at all
    h.refresh();
    debug!("displaying {} results from {name} histogram", h.len());
    debug!(
        "{name}\n\
        \tmean:\t{:.1}{unit},\n\
        \tp50:\t{:.1}{unit},\n\
        \tp90:\t{:.1}{unit},\n\
        \tp99:\t{:.1}{unit},\n\
        \tmax:\t{:.1}{unit}",
        h.mean() / scale,
        h.value_at_quantile(0.5) as f64 / scale,
        h.value_at_quantile(0.9) as f64 / scale,
        h.value_at_quantile(0.99) as f64 / scale,
        h.max() as f64 / scale,
    );
}
