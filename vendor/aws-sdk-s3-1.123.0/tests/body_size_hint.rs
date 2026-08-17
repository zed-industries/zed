/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Body wrappers must pass through size_hint

use aws_config::SdkConfig;
use aws_sdk_s3::{
    config::{Credentials, Region, SharedCredentialsProvider},
    primitives::{ByteStream, SdkBody},
    Client,
};
use aws_smithy_http_client::test_util::{capture_request, infallible_client_fn};
use http_body_1x::Body;

#[tokio::test]
async fn download_body_size_hint_check() {
    let test_body_content = b"hello";
    let test_body = || SdkBody::from(&test_body_content[..]);
    assert_eq!(
        Some(test_body_content.len() as u64),
        (test_body)().size_hint().exact(),
        "pre-condition check"
    );

    let http_client = infallible_client_fn(move |_| {
        http_1x::Response::builder()
            .status(200)
            .body((test_body)())
            .unwrap()
    });
    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .build();
    let client = Client::new(&sdk_config);
    let response = client
        .get_object()
        .bucket("foo")
        .key("foo")
        .send()
        .await
        .unwrap();
    assert_eq!(
        (
            test_body_content.len() as u64,
            Some(test_body_content.len() as u64),
        ),
        response.body.size_hint(),
        "the size hint should be passed through all the default body wrappers"
    );
}

#[tokio::test]
async fn upload_body_size_hint_check() {
    let test_body_content = b"hello";

    let (http_client, rx) = capture_request(None);
    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .build();
    let client = Client::new(&sdk_config);
    let body = ByteStream::from_static(test_body_content);
    assert_eq!(
        (
            test_body_content.len() as u64,
            Some(test_body_content.len() as u64),
        ),
        body.size_hint(),
        "pre-condition check"
    );
    let _response = client
        .put_object()
        .bucket("foo")
        .key("foo")
        .body(body)
        .send()
        .await;
    let captured_request = rx.expect_request();
    assert_eq!(
        Some(test_body_content.len() as u64),
        captured_request.body().size_hint().exact(),
        "the size hint should be passed through all the default body wrappers"
    );
}
