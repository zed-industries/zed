/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, UNIX_EPOCH};

use aws_runtime::auth::PayloadSigningOverride;
use aws_runtime::content_encoding::header::X_AMZ_TRAILER_SIGNATURE;
use aws_runtime::content_encoding::{AwsChunkedBodyOptions, DeferredSigner};
use aws_sdk_s3::config::Region;
use aws_sdk_s3::error::DisplayErrorContext;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{Client, Config};
use aws_smithy_async::test_util::ManualTimeSource;
use aws_smithy_async::time::SharedTimeSource;
use aws_smithy_http_client::test_util::capture_request;
use aws_smithy_http_client::test_util::dvr::ReplayingClient;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::BeforeTransmitInterceptorContextMut;
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;
use bytes::Bytes;
use http_body_1x::{Body, SizeHint};
use pin_project_lite::pin_project;

// Interceptor that forces chunk signing for testing purposes.
//
// Chunk signing during AWS chunked content encoding only occurs when requests are sent
// without TLS. This interceptor overrides the `AwsChunkedContentEncodingInterceptor`
// configuration to enable chunk signing for testing.
#[derive(Debug)]
struct ForceChunkedSigningInterceptor {
    time_source: ManualTimeSource,
}

impl Intercept for ForceChunkedSigningInterceptor {
    fn name(&self) -> &'static str {
        "ForceChunkedSigningInterceptor"
    }

    fn modify_before_signing(
        &self,
        _context: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        // Grab existing options and update them with signing enabled
        let chunked_body_options = cfg
            .get_mut_from_interceptor_state::<AwsChunkedBodyOptions>()
            .expect("AwsChunkedBodyOptions should be set");

        let chunked_body_options = std::mem::take(chunked_body_options)
            .signed_chunked_encoding(true)
            .with_trailer_len((X_AMZ_TRAILER_SIGNATURE.len() + ":".len() + 64) as u64);

        cfg.interceptor_state().store_put(chunked_body_options);

        let (signer, sender) = DeferredSigner::new();
        cfg.interceptor_state().store_put(signer);
        cfg.interceptor_state().store_put(sender);

        cfg.interceptor_state()
            .store_put(PayloadSigningOverride::StreamingSignedPayloadTrailer);

        Ok(())
    }

    // Verifies the chunk signer uses a `StaticTimeSource` by advancing time by 1 second
    // before transmission. If a dynamic time source were used, the test would fail with
    // a chunk signature mismatch.
    fn modify_before_transmit(
        &self,
        _context: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        self.time_source.advance(Duration::from_secs(1));
        Ok(())
    }
}

// Custom streaming body
pin_project! {
    #[derive(Clone)]
    struct TestBody {
        data: Option<Bytes>,
    }
}

impl Body for TestBody {
    type Data = Bytes;
    type Error = aws_smithy_types::body::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body_1x::Frame<Self::Data>, Self::Error>>> {
        let this = self.project();

        if let Some(data) = this.data.take() {
            return Poll::Ready(Some(Ok(http_body_1x::Frame::data(data))));
        }

        Poll::Ready(None)
    }

    fn size_hint(&self) -> SizeHint {
        let mut size = SizeHint::default();
        size.set_lower(self.data.as_ref().map_or(0, |d| d.len() as u64));
        size.set_upper(self.data.as_ref().map_or(0, |d| d.len() as u64));
        size
    }
}

#[tokio::test]
async fn test_signing_for_aws_chunked_content_encoding() {
    let time_source = ManualTimeSource::new(UNIX_EPOCH + Duration::from_secs(1234567890));

    let http_client = ReplayingClient::from_file("tests/data/aws_chunked/chunk-signing.json")
        .expect("recorded HTTP communication exists");
    let config = Config::builder()
        .with_test_defaults()
        .http_client(http_client.clone())
        .region(Region::new("us-east-1"))
        .time_source(SharedTimeSource::new(time_source.clone()))
        .aws_chunked_encoding_chunk_size(Some(8 * 1024)) // 8 KiB chunk size
        .build();

    let client = Client::from_conf(config);

    // 10 KiB of 'a' characters. With a 8 KiB chunk size, the payload splits into four chunks:
    // 8 KiB, 2 KiB, 0 bytes, and the final chunk containing trailing headers.
    let data = "a".repeat(10 * 1024);
    let body = TestBody {
        data: Some(Bytes::from(data)),
    };
    let body = ByteStream::from_body_1_x(body);

    let _ = dbg!(client
        .put_object()
        .body(body)
        .bucket("test-bucket")
        .key("10KiBofA.txt")
        .customize()
        .config_override(
            Config::builder().interceptor(ForceChunkedSigningInterceptor { time_source })
        )
        .send()
        .await
        .unwrap());

    http_client
        .validate_body_and_headers(
            Some(&["content-encoding", "x-amz-content-sha256"]),
            "application/octet-stream",
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_aws_chunked_content_encoding_with_custom_chunk_size() {
    let http_client = ReplayingClient::from_file("tests/data/aws_chunked/custom-chunk-size.json")
        .expect("recorded HTTP communication exists");
    let config = Config::builder()
        .with_test_defaults()
        .http_client(http_client.clone())
        .region(Region::new("us-east-1"))
        .build();

    let client = Client::from_conf(config);

    // 10 KiB of 'a' characters
    let data = "a".repeat(10 * 1024);
    let body = TestBody {
        data: Some(Bytes::from(data)),
    };
    let body = ByteStream::from_body_1_x(body);

    // Demonstrate that chunk size can be overridden per-request
    let _ = dbg!(client
        .put_object()
        .body(body)
        .bucket("test-bucket")
        .key("10KiBofA.txt")
        .customize()
        .config_override(Config::builder().aws_chunked_encoding_chunk_size(Some(8 * 1024)))
        .send()
        .await
        .unwrap());

    http_client
        .validate_body_and_headers(
            Some(&["content-encoding", "x-amz-content-sha256"]),
            "application/octet-stream",
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_aws_chunked_content_encoding_with_no_chunking() {
    let http_client = ReplayingClient::from_file("tests/data/aws_chunked/no-chunking.json")
        .expect("recorded HTTP communication exists");
    let config = Config::builder()
        .with_test_defaults()
        .http_client(http_client.clone())
        .region(Region::new("us-east-1"))
        .aws_chunked_encoding_chunk_size(None) // No chunking
        .build();

    let client = Client::from_conf(config);

    // 65 KiB of 'a' characters. Without chunking, the payload splits into two chunks:
    // 65 KiB and the final chunk containg 0 bytes data and trailing headers.
    let data = "a".repeat(65 * 1024);
    let body = TestBody {
        data: Some(Bytes::from(data)),
    };
    let body = ByteStream::from_body_1_x(body);

    let _ = dbg!(client
        .put_object()
        .body(body)
        .bucket("test-bucket")
        .key("65KiBofA.txt")
        .send()
        .await
        .unwrap());

    http_client
        .validate_body_and_headers(
            Some(&["content-encoding", "x-amz-content-sha256"]),
            "application/octet-stream",
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_chunk_size_too_small_fails() {
    let (http_client, _rcvr) = capture_request(None);
    let config = Config::builder()
        .with_test_defaults()
        .http_client(http_client)
        .region(Region::new("us-east-1"))
        .aws_chunked_encoding_chunk_size(Some(4096)) // Too small - less than 8 KiB
        .build();

    let client = Client::from_conf(config);

    let data = "a".repeat(10 * 1024);
    let body = TestBody {
        data: Some(Bytes::from(data)),
    };

    let result = dbg!(
        client
            .put_object()
            .body(ByteStream::from_body_1_x(body.clone()))
            .bucket("test-bucket")
            .key("10KiBofA.txt")
            .send()
            .await
    );

    assert!(result.is_err());
    let err_msg = DisplayErrorContext(&result.unwrap_err()).to_string();
    assert!(
        err_msg.contains("Chunk size must be at least 8192 bytes, but 4096 was provided"),
        "Expected error about minimum chunk size, got: {}",
        err_msg
    );

    let result = dbg!(
        client
            .put_object()
            .body(ByteStream::from_body_1_x(body))
            .bucket("test-bucket")
            .key("10KiBofA.txt")
            .customize()
            .config_override(Config::builder().aws_chunked_encoding_chunk_size(Some(0))) // Test edge case of 0
            .send()
            .await
    );

    assert!(result.is_err());
    let err_msg = DisplayErrorContext(&result.unwrap_err()).to_string();
    assert!(
        err_msg.contains("Chunk size must be at least 8192 bytes, but 0 was provided"),
        "Expected error about minimum chunk size, got: {}",
        err_msg
    );
}
