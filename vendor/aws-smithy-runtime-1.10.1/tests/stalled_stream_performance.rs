/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(all(
    feature = "client",
    any(feature = "test-util", feature = "legacy-test-util")
))]

use aws_smithy_async::rt::sleep::TokioSleep;
use aws_smithy_async::time::{SystemTimeSource, TimeSource};
use aws_smithy_runtime::client::http::body::minimum_throughput::MinimumThroughputDownloadBody;
use aws_smithy_runtime_api::client::stalled_stream_protection::StalledStreamProtectionConfig;
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::byte_stream::ByteStream;
use bytes::{BufMut, Bytes, BytesMut};
use hyper_0_14::server::conn::AddrStream;
use hyper_0_14::service::{make_service_fn, service_fn, Service};
use hyper_0_14::Server;
use std::convert::Infallible;
use std::net::TcpListener;
use std::time::Duration;

fn make_block(sz: usize) -> Bytes {
    let mut b = BytesMut::with_capacity(sz);
    b.put_bytes(1, sz);
    b.freeze()
}

// TODO(postGA): convert this to an actual benchmark
// This test evaluates streaming 1GB of data over the loopback with and without the body wrapper
// enabled. After optimizations, the body wrapper seems to make minimal differences
// NOTE: make sure you run this test in release mode to get a sensible result
#[tokio::test]
#[ignore]
async fn stalled_stream_performance() {
    // 1GB
    let data_size = 1_000_000_000;
    // observed block size during actual HTTP requests
    let block_size = 16384;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let make_service = make_service_fn(move |_connection: &AddrStream| async move {
        Ok::<_, Infallible>(service_fn(
            move |_: http_02x::Request<hyper_0_14::Body>| async move {
                let (mut sender, body) = hyper_0_14::Body::channel();
                tokio::task::spawn(async move {
                    for _i in 0..(data_size / block_size) {
                        sender
                            .send_data(make_block(block_size))
                            .await
                            .expect("failed to write data");
                    }
                });
                Ok::<_, Infallible>(http_02x::Response::new(body))
            },
        ))
    });
    let addr = format!("http://localhost:{}", listener.local_addr().unwrap().port());
    let server = Server::from_tcp(listener).unwrap().serve(make_service);
    tokio::spawn(server);

    let mut no_wrapping = vec![];
    let mut wrapping = vec![];
    let runs = 10;
    for _i in 0..runs {
        no_wrapping.push(make_request(&addr, false).await);
        wrapping.push(make_request(&addr, true).await);
    }
    println!(
        "Average w/ wrapping: {}",
        wrapping.iter().map(|it| it.as_millis() as f64).sum::<f64>() / runs as f64
    );
    println!(
        "Average w/o wrapping: {}",
        no_wrapping
            .iter()
            .map(|it: &Duration| it.as_millis() as f64)
            .sum::<f64>()
            / runs as f64
    )
}

async fn make_request(address: &str, wrap_body: bool) -> Duration {
    let mut client = hyper_0_14::Client::new();
    let req = ::http_02x::Request::builder()
        .uri(address)
        .body(hyper_0_14::Body::empty())
        .unwrap();
    let resp = client.call(req).await;
    let body = resp.unwrap().into_body();
    let mut body = SdkBody::from_body_0_4(body);
    if wrap_body {
        body = body.map_preserve_contents(|body| {
            let time_source = SystemTimeSource::new();
            let sleep = TokioSleep::new();
            let opts = StalledStreamProtectionConfig::enabled().build();
            let mtb = MinimumThroughputDownloadBody::new(time_source, sleep, body, opts.into());
            SdkBody::from_body_0_4(mtb)
        });
    }

    let sdk_body = ByteStream::new(body);
    let ts = SystemTimeSource::new();
    let start = ts.now();
    // this a slow way to count total bytes, but we need to actually read the bytes into segments
    // otherwise some of our code seems to be optimized away
    let total_bytes = sdk_body
        .collect()
        .await
        .unwrap()
        .into_segments()
        .map(|seg| seg.len())
        .sum::<usize>();
    println!("total: {:?}", total_bytes);
    let end = ts.now();
    end.duration_since(start).unwrap()
}
