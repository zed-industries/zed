/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Config;
use aws_sdk_s3::{config::Credentials, config::Region, Client};
use aws_smithy_http_client::test_util::capture_request;

#[tokio::test]
async fn test_operation_should_not_normalize_uri_path() {
    let (http_client, rx) = capture_request(None);
    let config = Config::builder()
        .credentials_provider(SharedCredentialsProvider::new(
            Credentials::for_tests_with_session_token(),
        ))
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .with_test_defaults()
        .build();
    let client = Client::from_conf(config);

    let bucket_name = "test-bucket-ad7c9f01-7f7b-4669-b550-75cc6d4df0f1";

    client
        .put_object()
        .bucket(bucket_name)
        .key("a/.././b.txt") // object key with dot segments
        .body(ByteStream::from_static("Hello, world".as_bytes()))
        .send()
        .await
        .unwrap();

    let request = rx.expect_request();
    let actual_uri = request.uri();
    let expected_uri = "https://test-bucket-ad7c9f01-7f7b-4669-b550-75cc6d4df0f1.s3.us-east-1.amazonaws.com/a/.././b.txt?x-id=PutObject";
    assert_eq!(expected_uri, actual_uri);
}
