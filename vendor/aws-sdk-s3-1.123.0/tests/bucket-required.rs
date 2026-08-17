/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region, StalledStreamProtectionConfig};
use aws_sdk_s3::Client;
use aws_smithy_http_client::test_util::capture_request;

#[tokio::test]
async fn dont_dispatch_when_bucket_is_unset() {
    let (http_client, rcvr) = capture_request(None);
    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::new("us-east-1"))
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .http_client(http_client.clone())
        .build();
    let client = Client::new(&sdk_config);
    let err = client
        .list_objects_v2()
        .send()
        .await
        .expect_err("bucket not set");
    assert_eq!(format!("{}", err), "failed to construct request");
    rcvr.expect_no_request();
}
