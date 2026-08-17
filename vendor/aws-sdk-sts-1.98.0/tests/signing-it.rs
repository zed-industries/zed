/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sdk_sts::config::{Credentials, Region};
use aws_smithy_http_client::test_util::capture_request;

#[tokio::test]
async fn assume_role_signed() {
    let creds = Credentials::for_tests();
    let (http_client, request) = capture_request(None);
    let conf = aws_sdk_sts::Config::builder()
        .behavior_version_latest()
        .credentials_provider(creds)
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .build();
    let client = aws_sdk_sts::Client::from_conf(conf);
    let _ = client.assume_role().send().await;
    // assume role should have an auth header
    assert_ne!(
        request.expect_request().headers().get("AUTHORIZATION"),
        None
    );
}

#[tokio::test]
async fn web_identity_unsigned() {
    let (http_client, request) = capture_request(None);
    let conf = aws_sdk_sts::Config::builder()
        .behavior_version_latest()
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .build();
    let client = aws_sdk_sts::Client::from_conf(conf);
    let _ = client.assume_role_with_web_identity().send().await;
    // web identity should be unsigned
    assert_eq!(
        request.expect_request().headers().get("AUTHORIZATION"),
        None
    );
}

#[tokio::test]
async fn assume_role_saml_unsigned() {
    let (http_client, request) = capture_request(None);
    let conf = aws_sdk_sts::Config::builder()
        .behavior_version_latest()
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .build();
    let client = aws_sdk_sts::Client::from_conf(conf);
    let _ = client.assume_role_with_saml().send().await;
    // web identity should be unsigned
    assert_eq!(
        request.expect_request().headers().get("AUTHORIZATION"),
        None
    );
}

#[tokio::test]
async fn web_identity_no_creds() {
    let (http_client, request) = capture_request(None);
    let conf = aws_sdk_sts::Config::builder()
        .behavior_version_latest()
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .build();
    let client = aws_sdk_sts::Client::from_conf(conf);
    let _ = client.assume_role_with_web_identity().send().await;
    // web identity should be unsigned and work without credentials
    assert_eq!(
        request.expect_request().headers().get("AUTHORIZATION"),
        None
    );
}
