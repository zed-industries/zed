/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::{Client, Config};
use aws_smithy_http_client::test_util::capture_request;
use http_1x::HeaderValue;
use std::time::{Duration, SystemTime};

#[tokio::test]
async fn test_s3_ops_are_customizable() {
    let (http_client, rcvr) = capture_request(None);
    let config = Config::builder()
        .credentials_provider(SharedCredentialsProvider::new(
            Credentials::for_tests_with_session_token(),
        ))
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .build();

    let client = Client::from_conf(config);

    // The response from the fake connection won't return the expected XML but we don't care about
    // that error in this test
    let _ = assert_send(
        client
            .list_buckets()
            .customize()
            .mutate_request(|req| {
                req.headers_mut()
                    .append("test-header", HeaderValue::from_static("test-value"));
            })
            .send(),
    )
    .await
    .expect_err("this will fail due to not receiving a proper XML response.");

    let expected_req = rcvr.expect_request();
    let test_header = expected_req
        .headers()
        .get("test-header")
        .unwrap()
        .to_owned();

    assert_eq!("test-value", test_header);
}

#[tokio::test]
async fn customized_presigning() {
    let creds = Credentials::for_tests_with_session_token();
    let config = Config::builder()
        .credentials_provider(creds)
        .region(Region::new("us-east-1"))
        .build();
    let client = Client::from_conf(config);
    let static_ps_config = PresigningConfig::builder()
        .start_time(SystemTime::UNIX_EPOCH + Duration::from_secs(1234567891))
        .expires_in(Duration::from_secs(30))
        .build()
        .unwrap();
    let req = assert_send(
        client
            .get_object()
            .bucket("foo")
            .key("bar")
            .customize()
            .mutate_request(|req| {
                req.set_uri(req.uri().to_string() + "&a=b")
                    .expect("failed to update URI")
            })
            .presigned(static_ps_config),
    )
    .await
    .unwrap();
    let expect = "https://foo.s3.us-east-1.amazonaws.com/bar?x-id=GetObject&a=b&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ANOTREAL%2F20090213%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20090213T233131Z&X-Amz-Expires=30&X-Amz-SignedHeaders=host&X-Amz-Signature=2e1a459c206932ce53beb07028c711cf70f3a61dc876c6f9ce0aed5823f60234&X-Amz-Security-Token=notarealsessiontoken";
    assert_eq!(req.uri(), expect);
}

fn assert_send<T: Send>(t: T) -> T {
    t
}
