/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::Client;
use aws_smithy_http_client::test_util::capture_request;

#[tokio::test]
async fn recursion_detection_applied() {
    std::env::set_var("AWS_LAMBDA_FUNCTION_NAME", "some-function");
    std::env::set_var("_X_AMZN_TRACE_ID", "traceid");
    let (http_client, captured_request) = capture_request(None);
    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .build();
    let client = Client::new(&sdk_config);
    let _ = client.list_objects_v2().bucket("test-bucket").send().await;
    assert_eq!(
        captured_request
            .expect_request()
            .headers()
            .get("x-amzn-trace-id"),
        Some("traceid")
    );
}
