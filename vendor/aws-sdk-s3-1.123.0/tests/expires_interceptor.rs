/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(feature = "test-util")]
use aws_sdk_s3::{config::Region, Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::date_time::{DateTime, Format};

fn make_client(expires_val: &str) -> Client {
    let http_client = StaticReplayClient::new(vec![ReplayEvent::new(
        http_1x::Request::builder()
            .uri(http_1x::Uri::from_static(
                "https://some-test-bucket.s3.us-east-1.amazonaws.com/test.txt?attributes",
            ))
            .body(SdkBody::empty())
            .unwrap(),
        http_1x::Response::builder()
            .header("Expires", expires_val)
            .status(200)
            .body(SdkBody::empty())
            .unwrap(),
    )]);

    let config = Config::builder()
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .with_test_defaults()
        .build();

    Client::from_conf(config)
}

#[allow(deprecated)]
#[tokio::test]
async fn expires_customization_works_with_non_date_value() {
    let client = make_client("foo");

    let out = client
        .get_object()
        .bucket("some-test-bucket")
        .key("test.txt")
        .send()
        .await
        .unwrap();

    assert_eq!(out.expires, None);
    assert_eq!(out.expires_string.unwrap(), "foo".to_string())
}

#[allow(deprecated)]
#[tokio::test]
async fn expires_customization_works_with_valid_date_format() {
    let date = "Tue, 29 Apr 2014 18:30:38 GMT";
    let date_time = DateTime::from_str(date, Format::HttpDate).unwrap();

    let client = make_client(date);

    let out = client
        .get_object()
        .bucket("some-test-bucket")
        .key("test.txt")
        .send()
        .await
        .unwrap();

    assert_eq!(out.expires.unwrap(), date_time);
    assert_eq!(out.expires_string.unwrap(), date);
}

#[allow(deprecated)]
#[tokio::test]
async fn expires_customization_works_with_non_http_date_format() {
    let date = "1985-04-12T23:20:50.52Z";

    let client = make_client(date);

    let out = client
        .get_object()
        .bucket("some-test-bucket")
        .key("test.txt")
        .send()
        .await
        .unwrap();

    assert_eq!(out.expires, None);
    assert_eq!(out.expires_string.unwrap(), date);
}
