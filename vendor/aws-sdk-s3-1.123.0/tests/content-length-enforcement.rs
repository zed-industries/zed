/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sdk_s3::{
    config::retry::RetryConfig, config::Region, error::DisplayErrorContext, Client, Config,
};
use aws_smithy_http_client::test_util::dvr::ReplayingClient;

#[tokio::test]
async fn test_content_length_enforcement_is_not_applied_to_head_request() {
    let http_client =
        ReplayingClient::from_file("tests/data/content-length-enforcement/head-object.json")
            .expect("recorded HTTP communication exists");
    let config = Config::builder()
        .with_test_defaults()
        .http_client(http_client.clone())
        .region(Region::new("us-east-1"))
        .retry_config(RetryConfig::disabled()) // Disable retries for replay test
        .build();
    let client = Client::from_conf(config);
    let _resp = client
        .head_object()
        .key("dontcare.json")
        .bucket("dontcare")
        .send()
        .await
        .expect("content length enforcement must not apply to HEAD requests");

    // The body returned will be empty, so we pass an empty string for `media_type` to
    // `validate_body_and_headers_except`. That way, it'll do a string equality check on the empty
    // strings.
    http_client.relaxed_validate("").await.unwrap();
}

#[tokio::test]
async fn test_content_length_enforcement_get_request_short() {
    let http_client =
        ReplayingClient::from_file("tests/data/content-length-enforcement/get-object-short.json")
            .expect("recorded HTTP communication exists");
    let config = Config::builder()
        .with_test_defaults()
        .http_client(http_client.clone())
        .region(Region::new("us-east-1"))
        .retry_config(RetryConfig::disabled()) // Disable retries for replay test
        .build();
    let client = Client::from_conf(config);
    // The file we're fetching is exactly 10,000 bytes long, but we've set the
    // response's content-length to 9,999 bytes. This should trigger the
    // content-length enforcement.

    // This will succeed.
    let output = client
        .get_object()
        .key("1000-lines.txt")
        .bucket("dontcare")
        .send()
        .await
        .unwrap();

    // This will fail with a content-length mismatch error.
    let content_length_err = output.body.collect().await.unwrap_err();

    http_client
        .relaxed_validate("application/text")
        .await
        .unwrap();
    assert_eq!(
        DisplayErrorContext(content_length_err).to_string(),
        "streaming error: Invalid Content-Length: Expected 9999 bytes but 10000 bytes were received (Error { kind: StreamingError(ContentLengthError { expected: 9999, received: 10000 }) })"
    );
}

#[tokio::test]
async fn test_content_length_enforcement_get_request_long() {
    let http_client =
        ReplayingClient::from_file("tests/data/content-length-enforcement/get-object-long.json")
            .expect("recorded HTTP communication exists");
    let config = Config::builder()
        .with_test_defaults()
        .http_client(http_client.clone())
        .region(Region::new("us-east-1"))
        .retry_config(RetryConfig::disabled()) // Disable retries for replay test
        .build();
    let client = Client::from_conf(config);
    // The file we're fetching is exactly 10,000 bytes long, but we've set the
    // response's content-length to 9,999 bytes. This should trigger the
    // content-length enforcement.

    // This will succeed.
    let output = client
        .get_object()
        .key("1000-lines.txt")
        .bucket("dontcare")
        .send()
        .await
        .unwrap();

    // This will fail with a content-length mismatch error.
    let content_length_err = output.body.collect().await.unwrap_err();

    http_client
        .relaxed_validate("application/text")
        .await
        .unwrap();
    assert_eq!(
        DisplayErrorContext(content_length_err).to_string(),
        "streaming error: Invalid Content-Length: Expected 10001 bytes but 10000 bytes were received (Error { kind: StreamingError(ContentLengthError { expected: 10001, received: 10000 }) })"
    );
}
