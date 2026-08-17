/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_credential_types::Credentials;
use aws_smithy_async::assert_elapsed;
use aws_smithy_async::rt::sleep::{SharedAsyncSleep, TokioSleep};
use aws_smithy_http_client::test_util::NeverClient;
use aws_smithy_runtime::test_util::capture_test_logs::capture_test_logs;
use aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;
use aws_smithy_runtime_api::client::result::SdkError;
use aws_smithy_types::retry::RetryConfig;
use aws_smithy_types::timeout::TimeoutConfig;
use aws_types::region::Region;
use aws_types::SdkConfig;
use std::time::Duration;
use tokio::time::Instant;

/// Use a 5 second operation timeout on SdkConfig and a 0ms operation timeout on the service config
#[tokio::test]
async fn timeouts_can_be_set_by_service() {
    let (_guard, _) = capture_test_logs();
    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::from_static("us-east-1"))
        .sleep_impl(SharedAsyncSleep::new(TokioSleep::new()))
        .retry_config(RetryConfig::disabled())
        .timeout_config(
            TimeoutConfig::builder()
                .operation_timeout(Duration::from_secs(5))
                .read_timeout(Duration::from_secs(1))
                .build(),
        )
        .http_client(NeverClient::new())
        // ip that
        .endpoint_url(
            // Emulate a connect timeout error by hitting an unroutable IP
            "http://172.255.255.0:18104",
        )
        .build();
    let config = aws_sdk_s3::config::Builder::from(&sdk_config)
        .timeout_config(
            TimeoutConfig::builder()
                .operation_timeout(Duration::from_secs(0))
                .build(),
        )
        .build();

    let client = aws_sdk_s3::Client::from_conf(config);
    let start = Instant::now();
    let err = client
        .get_object()
        .key("foo")
        .bucket("bar")
        .send()
        .await
        .expect_err("unroutable IP should timeout");
    match err {
        SdkError::TimeoutError(_err) => { /* ok */ }
        // if the connect timeout is not respected, this times out after 5 seconds because of the operation timeout with `SdkError::Timeout`
        _other => panic!("unexpected error: {:?}", _other),
    }
    // there should be a 0ms timeout, we gotta set some stuff up. Just want to make sure
    // it's shorter than the 5 second timeout if the test is broken
    assert!(start.elapsed() < Duration::from_millis(500));
}

/// Ensures that a default timeout from aws-config is still persisted even if an operation_timeout
/// is set.
#[tokio::test]
async fn default_connect_timeout_set() {
    let (_guard, _) = capture_test_logs();
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .test_credentials()
        .region(Region::from_static("us-east-1"))
        .timeout_config(
            TimeoutConfig::builder()
                .operation_timeout(Duration::from_secs(10))
                .build(),
        )
        .retry_config(RetryConfig::disabled())
        // ip that
        .endpoint_url(
            // Emulate a connect timeout error by hitting an unroutable IP
            "http://172.255.255.0:18104",
        )
        .load()
        .await;
    assert_eq!(
        sdk_config.timeout_config(),
        Some(
            &TimeoutConfig::builder()
                .connect_timeout(Duration::from_millis(3100))
                .operation_timeout(Duration::from_secs(10))
                .build()
        )
    );
    assert_eq!(
        sdk_config.endpoint_url(),
        Some("http://172.255.255.0:18104")
    );

    let config = aws_sdk_s3::config::Builder::from(&sdk_config)
        // .endpoint_url("http://172.255.255.0:18104")
        .timeout_config(
            TimeoutConfig::builder()
                .operation_attempt_timeout(Duration::from_secs(8))
                .build(),
        )
        .build();

    let client = aws_sdk_s3::Client::from_conf(config);
    let start = Instant::now();
    let err = client
        .get_object()
        .key("foo")
        .bucket("bar")
        .send()
        .await
        .expect_err("unroutable IP should timeout");
    assert!(
        matches!(err, SdkError::DispatchFailure { .. }),
        "expected DispatchFailure got {}",
        err
    );
    // ensure that of the three timeouts, the one we hit is connect timeout.
    assert_elapsed!(
        start,
        Duration::from_millis(3100),
        Duration::from_millis(1000)
    );
}
