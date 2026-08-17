/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_config::retry::RetryConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region, StalledStreamProtectionConfig};
use aws_sdk_s3::types::{
    CompressionType, CsvInput, CsvOutput, ExpressionType, FileHeaderInfo, InputSerialization,
    OutputSerialization,
};
use aws_sdk_s3::{Client, Config};
use aws_smithy_async::assert_elapsed;
use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep, Sleep};
use aws_smithy_http_client::test_util::NeverClient;
use aws_smithy_runtime::test_util::capture_test_logs::capture_test_logs;
use aws_smithy_runtime_api::client::result::SdkError;
use aws_smithy_types::error::display::DisplayErrorContext;
use aws_smithy_types::timeout::TimeoutConfig;
use std::fmt::Debug;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct SmolSleep;

impl AsyncSleep for SmolSleep {
    fn sleep(&self, duration: Duration) -> Sleep {
        Sleep::new(async move {
            smol::Timer::after(duration).await;
        })
    }
}

#[test]
fn test_smol_runtime_timeouts() {
    let _guard = capture_test_logs();

    if let Err(err) = smol::block_on(async { timeout_test(SharedAsyncSleep::new(SmolSleep)).await })
    {
        println!("{err}");
        panic!();
    }
}

#[test]
fn test_smol_runtime_retry() {
    let _guard = capture_test_logs();

    if let Err(err) = smol::block_on(async { retry_test(SharedAsyncSleep::new(SmolSleep)).await }) {
        println!("{err}");
        panic!();
    }
}

#[derive(Debug)]
struct AsyncStdSleep;

impl AsyncSleep for AsyncStdSleep {
    fn sleep(&self, duration: Duration) -> Sleep {
        Sleep::new(async move { async_std::task::sleep(duration).await })
    }
}

#[test]
fn test_async_std_runtime_timeouts() {
    let _guard = capture_test_logs();

    if let Err(err) = async_std::task::block_on(async {
        timeout_test(SharedAsyncSleep::new(AsyncStdSleep)).await
    }) {
        println!("{err}");
        panic!();
    }
}

#[test]
fn test_async_std_runtime_retry() {
    let _guard = capture_test_logs();

    if let Err(err) =
        async_std::task::block_on(async { retry_test(SharedAsyncSleep::new(AsyncStdSleep)).await })
    {
        println!("{err}");
        panic!();
    }
}

async fn timeout_test(sleep_impl: SharedAsyncSleep) -> Result<(), Box<dyn std::error::Error>> {
    let http_client = NeverClient::new();
    let region = Region::from_static("us-east-2");
    let timeout_config = TimeoutConfig::builder()
        .operation_timeout(Duration::from_secs_f32(0.5))
        .build();
    let config = Config::builder()
        .region(region)
        .http_client(http_client.clone())
        .credentials_provider(Credentials::for_tests())
        .timeout_config(timeout_config)
        .sleep_impl(sleep_impl)
        .build();
    let client = Client::from_conf(config);

    let now = Instant::now();

    let err = client
        .select_object_content()
        .bucket("aws-rust-sdk")
        .key("sample_data.csv")
        .expression_type(ExpressionType::Sql)
        .expression("SELECT * FROM s3object s WHERE s.\"Name\" = 'Jane'")
        .input_serialization(
            InputSerialization::builder()
                .csv(
                    CsvInput::builder()
                        .file_header_info(FileHeaderInfo::Use)
                        .build(),
                )
                .compression_type(CompressionType::None)
                .build(),
        )
        .output_serialization(
            OutputSerialization::builder()
                .csv(CsvOutput::builder().build())
                .build(),
        )
        .send()
        .await
        .unwrap_err();

    let expected = "operation timeout (all attempts including retries) occurred after 500ms";
    let message = format!("{}", DisplayErrorContext(err));
    assert!(
        message.contains(expected),
        "expected '{message}' to contain '{expected}'"
    );
    // Assert 500ms have passed with a 150ms margin of error
    assert_elapsed!(now, Duration::from_millis(500), Duration::from_millis(150));

    Ok(())
}

async fn retry_test(sleep_impl: SharedAsyncSleep) -> Result<(), Box<dyn std::error::Error>> {
    let http_client = NeverClient::new();
    let conf = aws_types::SdkConfig::builder()
        .region(Region::new("us-east-2"))
        .http_client(http_client.clone())
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .retry_config(RetryConfig::standard().with_max_attempts(3))
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .timeout_config(
            TimeoutConfig::builder()
                .operation_attempt_timeout(Duration::from_secs_f64(0.1))
                .build(),
        )
        .sleep_impl(sleep_impl)
        .build();
    let client = Client::new(&conf);
    let resp = client
        .list_buckets()
        .send()
        .await
        .expect_err("call should fail");
    assert!(
        matches!(resp, SdkError::TimeoutError { .. }),
        "expected a timeout error, got: {:?}",
        resp
    );
    assert_eq!(
        3,
        http_client.num_calls(),
        "client level timeouts should be retried"
    );

    Ok(())
}
