/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(feature = "test-util")]

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{Client, Config};
use aws_smithy_http_client::test_util::{capture_request, ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;
use http_1x::header::AUTHORIZATION;

#[tokio::test]
async fn test_signer() {
    let http_client = StaticReplayClient::new(vec![ReplayEvent::new(
        http_1x::Request::builder()
            .header("authorization", "AWS4-HMAC-SHA256 Credential=ANOTREAL/20090213/us-east-1/s3/aws4_request, SignedHeaders=host;x-amz-content-sha256;x-amz-date;x-amz-user-agent, Signature=27e3f59ec3cffaa10e4f1c92112e8fb62d468a04cd32be39e68215f830404dbb")
            .uri("https://test-bucket.s3.us-east-1.amazonaws.com/?list-type=2&prefix=prefix~")
            .body(SdkBody::empty())
            .unwrap(),
        http_1x::Response::builder().status(200).body(SdkBody::empty()).unwrap(),
    )]);
    let config = Config::builder()
        .credentials_provider(SharedCredentialsProvider::new(
            Credentials::for_tests_with_session_token(),
        ))
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .with_test_defaults()
        .build();
    let client = Client::from_conf(config);
    let _ = client
        .list_objects_v2()
        .bucket("test-bucket")
        .prefix("prefix~")
        .send()
        .await;

    http_client.assert_requests_match(&[AUTHORIZATION.as_str()]);
}

#[tokio::test]
async fn disable_payload_signing_works() {
    let (http_client, request) = capture_request(None);
    let conf = aws_sdk_s3::Config::builder()
        .with_test_defaults()
        .behavior_version_latest()
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .build();
    let client = aws_sdk_s3::Client::from_conf(conf);
    let _ = client
        .put_object()
        .bucket("XXXXXXXXXXX")
        .key("test-key")
        .body(ByteStream::from_static(b"Hello, world!"))
        .customize()
        .disable_payload_signing()
        .send()
        .await;

    let request = request.expect_request();
    let x_amz_content_sha256 = request
        .headers()
        .get("x-amz-content-sha256")
        .expect("x-amz-content-sha256 is set")
        .to_owned();
    assert_eq!("UNSIGNED-PAYLOAD", x_amz_content_sha256);
}

// This test ensures that the request checksum interceptor payload signing
// override takes priority over the runtime plugin's override. If it didn't,
// then disabling payload signing would cause requests to incorrectly omit
// trailers.
#[tokio::test]
async fn disable_payload_signing_works_with_checksums() {
    let (http_client, request) = capture_request(None);
    let conf = aws_sdk_s3::Config::builder()
        .with_test_defaults()
        .behavior_version_latest()
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .build();
    let client = aws_sdk_s3::Client::from_conf(conf);

    // ByteStreams created from a file are streaming and have a known size
    let mut file = tempfile::NamedTempFile::new().unwrap();
    use std::io::Write;
    file.write_all(b"Hello, world!").unwrap();

    let body = aws_sdk_s3::primitives::ByteStream::read_from()
        .path(file.path())
        .buffer_size(1024)
        .build()
        .await
        .unwrap();

    let _ = client
        .put_object()
        .bucket("XXXXXXXXXXX")
        .key("test-key")
        .body(body)
        .checksum_algorithm(aws_sdk_s3::types::ChecksumAlgorithm::Crc32)
        .customize()
        .disable_payload_signing()
        .send()
        .await;

    let request = request.expect_request();
    let x_amz_content_sha256 = request
        .headers()
        .get("x-amz-content-sha256")
        .expect("x-amz-content-sha256 is set")
        .to_owned();
    // The checksum interceptor sets this.
    assert_eq!("STREAMING-UNSIGNED-PAYLOAD-TRAILER", x_amz_content_sha256);
}
