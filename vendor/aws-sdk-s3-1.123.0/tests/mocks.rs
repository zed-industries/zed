/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Integration tests for `aws-smithy-mocks`. These tests are not necessarily specific to S3 but
//! we need to test the macros against an actual SDK.

use aws_sdk_s3::config::retry::RetryConfig;
use aws_sdk_s3::operation::get_object::{GetObjectError, GetObjectOutput};
use aws_sdk_s3::operation::list_buckets::ListBucketsError;
use aws_smithy_mocks::{mock, mock_client, RuleMode};
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
use aws_smithy_runtime_api::http::StatusCode;
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::byte_stream::ByteStream;
use aws_smithy_types::error::metadata::ProvideErrorMetadata;
use aws_smithy_types::error::ErrorMetadata;

const S3_NO_SUCH_KEY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Error>
  <Code>NoSuchKey</Code>
  <Message>The resource you requested does not exist</Message>
  <Resource>/mybucket/myfoto.jpg</Resource>
  <RequestId>4442587FB7D0A2F9</RequestId>
</Error>"#;

#[tokio::test]
async fn test_mock_client() {
    let s3_404 = mock!(aws_sdk_s3::Client::get_object)
        .match_requests(|inp| {
            inp.bucket() == Some("test-bucket") && inp.key() != Some("correct-key")
        })
        .then_http_response(|| {
            HttpResponse::new(
                StatusCode::try_from(400).unwrap(),
                SdkBody::from(S3_NO_SUCH_KEY),
            )
        });

    let s3_real_object = mock!(aws_sdk_s3::Client::get_object)
        .match_requests(|inp| {
            inp.bucket() == Some("test-bucket") && inp.key() == Some("correct-key")
        })
        .then_output(|| {
            GetObjectOutput::builder()
                .body(ByteStream::from_static(b"test-test-test"))
                .build()
        });

    let modeled_error = mock!(aws_sdk_s3::Client::list_buckets).then_error(|| {
        ListBucketsError::generic(ErrorMetadata::builder().code("InvalidAccessKey").build())
    });

    let s3 = mock_client!(aws_sdk_s3, &[&s3_404, &s3_real_object, &modeled_error]);

    let error = s3
        .get_object()
        .bucket("test-bucket")
        .key("foo")
        .send()
        .await
        .expect_err("404");

    assert!(matches!(
        error.into_service_error(),
        GetObjectError::NoSuchKey(_)
    ));

    assert_eq!(s3_404.num_calls(), 1);

    let data = s3
        .get_object()
        .bucket("test-bucket")
        .key("correct-key")
        .send()
        .await
        .expect("success response")
        .body
        .collect()
        .await
        .expect("successful read")
        .to_vec();

    assert_eq!(data, b"test-test-test");
    assert_eq!(s3_real_object.num_calls(), 1);

    let err = s3.list_buckets().send().await.expect_err("bad access key");
    assert_eq!(err.code(), Some("InvalidAccessKey"));
}

#[tokio::test]
async fn test_mock_client_compute() {
    let s3_computed = mock!(aws_sdk_s3::Client::get_object)
        .match_requests(|inp| {
            inp.bucket() == Some("test-bucket") && inp.key() == Some("correct-key")
        })
        .then_compute_output(|input| {
            let content =
                format!("{}.{}", input.bucket().unwrap(), input.key().unwrap()).into_bytes();
            GetObjectOutput::builder()
                .body(ByteStream::from(content))
                .build()
        });

    let s3 = mock_client!(aws_sdk_s3, &[&s3_computed]);

    let data = s3
        .get_object()
        .bucket("test-bucket")
        .key("correct-key")
        .send()
        .await
        .expect("success response")
        .body
        .collect()
        .await
        .expect("successful read")
        .to_vec();

    assert_eq!(data, b"test-bucket.correct-key");
    assert_eq!(s3_computed.num_calls(), 1);
}

#[tokio::test]
async fn test_mock_client_sequence() {
    let rule = mock!(aws_sdk_s3::Client::get_object)
        .sequence()
        .http_status(400, Some(S3_NO_SUCH_KEY.to_string()))
        .output(|| {
            GetObjectOutput::builder()
                .body(ByteStream::from_static(b"test-test-test"))
                .build()
        })
        .build();

    // test client builder override
    let s3 = mock_client!(
        aws_sdk_s3,
        RuleMode::Sequential,
        [&rule],
        |client_builder| { client_builder.endpoint_url("http://localhost:9000") }
    );

    let error = s3
        .get_object()
        .bucket("test-bucket")
        .key("foo")
        .send()
        .await
        .expect_err("404");

    assert!(matches!(
        error.into_service_error(),
        GetObjectError::NoSuchKey(_)
    ));
    assert_eq!(1, rule.num_calls());
    let data = s3
        .get_object()
        .bucket("test-bucket")
        .key("correct-key")
        .send()
        .await
        .expect("success response")
        .body
        .collect()
        .await
        .expect("successful read")
        .to_vec();

    assert_eq!(data, b"test-test-test");
    assert_eq!(2, rule.num_calls());
}

#[tokio::test]
async fn test_mock_client_retries() {
    let rule = mock!(aws_sdk_s3::Client::get_object)
        .sequence()
        .http_status(503, None)
        .times(2)
        .output(|| {
            GetObjectOutput::builder()
                .body(ByteStream::from_static(b"test-test-test"))
                .build()
        })
        .build();

    // test client builder override
    let s3 = mock_client!(
        aws_sdk_s3,
        RuleMode::Sequential,
        [&rule],
        |client_builder| {
            client_builder.retry_config(RetryConfig::standard().with_max_attempts(3))
        }
    );

    let data = s3
        .get_object()
        .bucket("test-bucket")
        .key("correct-key")
        .send()
        .await
        .expect("success response")
        .body
        .collect()
        .await
        .expect("successful read")
        .to_vec();

    assert_eq!(data, b"test-test-test");
    assert_eq!(3, rule.num_calls());
}
