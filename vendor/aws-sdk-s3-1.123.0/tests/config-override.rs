/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::Client;
use aws_smithy_http_client::test_util::{capture_request, CaptureRequestReceiver};
use aws_types::SdkConfig;

fn test_client() -> (CaptureRequestReceiver, Client) {
    let (http_client, captured_request) = capture_request(None);
    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::new("us-west-2"))
        .http_client(http_client)
        .build();
    let client = Client::new(&sdk_config);
    (captured_request, client)
}

#[tokio::test]
async fn operation_overrides_force_path_style() {
    let (captured_request, client) = test_client();
    let _ = client
        .list_objects_v2()
        .bucket("test-bucket")
        .customize()
        .config_override(aws_sdk_s3::config::Config::builder().force_path_style(true))
        .send()
        .await;
    assert_eq!(
        captured_request.expect_request().uri().to_string(),
        "https://s3.us-west-2.amazonaws.com/test-bucket/?list-type=2"
    );
}

#[tokio::test]
async fn operation_overrides_fips() {
    let (captured_request, client) = test_client();
    let _ = client
        .list_objects_v2()
        .bucket("test-bucket")
        .customize()
        .config_override(aws_sdk_s3::config::Config::builder().use_fips(true))
        .send()
        .await;
    assert_eq!(
        captured_request.expect_request().uri().to_string(),
        "https://test-bucket.s3-fips.us-west-2.amazonaws.com/?list-type=2"
    );
}

#[tokio::test]
async fn operation_overrides_dual_stack() {
    let (captured_request, client) = test_client();
    let _ = client
        .list_objects_v2()
        .bucket("test-bucket")
        .customize()
        .config_override(aws_sdk_s3::config::Config::builder().use_dual_stack(true))
        .send()
        .await;
    assert_eq!(
        captured_request.expect_request().uri().to_string(),
        "https://test-bucket.s3.dualstack.us-west-2.amazonaws.com/?list-type=2"
    );
}
