/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(feature = "test-util")]

use aws_config::retry::RetryConfig;
use aws_sdk_s3::config::retry::RetryPartition;
use aws_sdk_s3::{config::Region, Client, Config};
use aws_smithy_async::test_util::ManualTimeSource;
use aws_smithy_async::time::SharedTimeSource;
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_runtime::client::retries::TokenBucket;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::config_bag::ConfigBag;
use std::sync::Mutex;
use std::sync::{Arc, LazyLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static THE_TIME: LazyLock<SystemTime> =
    LazyLock::new(|| UNIX_EPOCH + Duration::from_secs(12344321));

#[derive(Debug)]
struct TimeSourceValidationInterceptor {
    current_attempt: Arc<Mutex<u32>>,
}

impl Intercept for TimeSourceValidationInterceptor {
    fn name(&self) -> &'static str {
        "TimeSourceValidationInterceptor"
    }

    fn read_before_attempt(
        &self,
        _context: &aws_sdk_s3::config::interceptors::BeforeTransmitInterceptorContextRef<'_>,
        _runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        if let Some(token_bucket) = cfg.load::<TokenBucket>() {
            *self.current_attempt.lock().unwrap() += 1;

            if *self.current_attempt.lock().unwrap() == 1 {
                let last_refill = token_bucket
                    .last_refill_time_secs()
                    .load(std::sync::atomic::Ordering::Relaxed);
                assert_eq!(last_refill, 0);
            } else if *self.current_attempt.lock().unwrap() == 2 {
                let last_refill = token_bucket
                    .last_refill_time_secs()
                    .load(std::sync::atomic::Ordering::Relaxed);
                assert_eq!(last_refill, 12344321);
            } else {
                panic!("No attempts past the second should happen");
            }
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_token_bucket_gets_time_source_from_config() {
    let time_source = ManualTimeSource::new(*THE_TIME);
    let shared_time_source = SharedTimeSource::new(time_source);

    let http_client = StaticReplayClient::new(vec![
        ReplayEvent::new(
            http_1x::Request::builder()
                .uri("https://www.doesntmatter.com")
                .body(SdkBody::empty())
                .unwrap(),
            http_1x::Response::builder()
                .status(500)
                .body(SdkBody::from("This was an error"))
                .unwrap(),
        ),
        ReplayEvent::new(
            http_1x::Request::builder()
                .uri("https://www.doesntmatter.com")
                .body(SdkBody::empty())
                .unwrap(),
            http_1x::Response::builder()
                .status(200)
                .body(SdkBody::from("<ListBucketResult></ListBucketResult>"))
                .unwrap(),
        ),
    ]);

    let config = Config::builder()
        .region(Region::new("us-east-1"))
        .http_client(http_client)
        .time_source(shared_time_source)
        .interceptor(TimeSourceValidationInterceptor {
            current_attempt: Arc::new(Mutex::new(0)),
        })
        .retry_config(RetryConfig::standard())
        .retry_partition(
            RetryPartition::custom("test")
                .token_bucket(TokenBucket::builder().refill_rate(100.0).build())
                .build(),
        )
        .build();

    let client = Client::from_conf(config);

    let _result = client
        .list_objects_v2()
        .bucket("test-bucket")
        .send()
        .await
        .unwrap();
}
