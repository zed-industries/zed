/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_runtime::invocation_id::{InvocationId, PredefinedInvocationIdGenerator};
use aws_runtime::user_agent::AwsUserAgent;
use aws_sdk_s3::config::interceptors::BeforeSerializationInterceptorContextMut;
use aws_sdk_s3::config::interceptors::FinalizerInterceptorContextRef;
use aws_sdk_s3::config::retry::RetryConfig;
use aws_sdk_s3::config::timeout::TimeoutConfig;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::config::{Intercept, SharedAsyncSleep};
use aws_sdk_s3::Client;
use aws_smithy_async::test_util::InstantSleep;
use aws_smithy_async::test_util::ManualTimeSource;
use aws_smithy_async::time::SharedTimeSource;
use aws_smithy_http_client::test_util::dvr::ReplayingClient;
use aws_smithy_runtime::test_util::capture_test_logs::capture_test_logs;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::{ConfigBag, Layer};
use std::time::{Duration, UNIX_EPOCH};

// # One SDK operation invocation.
// # Client retries 3 times, successful response on 3rd attempt.
// # Fast network, latency + server time is less than one second.
// # No clock skew
// # Client waits 1 second between retry attempts.
#[tokio::test]
async fn three_retries_and_then_success() {
    let _logs = capture_test_logs();

    #[derive(Debug)]
    struct TimeInterceptor {
        time_source: ManualTimeSource,
    }
    impl Intercept for TimeInterceptor {
        fn name(&self) -> &'static str {
            "TimeInterceptor"
        }

        fn modify_before_serialization(
            &self,
            _context: &mut BeforeSerializationInterceptorContextMut<'_>,
            _runtime_components: &RuntimeComponents,
            cfg: &mut ConfigBag,
        ) -> Result<(), BoxError> {
            let mut layer = Layer::new("test");
            layer.store_put(AwsUserAgent::for_tests());
            cfg.push_layer(layer);
            Ok(())
        }

        fn read_after_attempt(
            &self,
            _context: &FinalizerInterceptorContextRef<'_>,
            _runtime_components: &RuntimeComponents,
            _cfg: &mut ConfigBag,
        ) -> Result<(), BoxError> {
            self.time_source.advance(Duration::from_secs(1));
            tracing::info!(
                "################ ADVANCED TIME BY 1 SECOND, {:?}",
                &self.time_source
            );
            Ok(())
        }
    }

    let time_source = ManualTimeSource::new(UNIX_EPOCH + Duration::from_secs(1559347200));

    let path = "tests/data/request-information-headers/three-retries_and-then-success.json";
    let http_client = ReplayingClient::from_file(path).unwrap();
    let config = aws_sdk_s3::Config::builder()
        .credentials_provider(Credentials::for_tests_with_session_token())
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .time_source(SharedTimeSource::new(time_source.clone()))
        .sleep_impl(SharedAsyncSleep::new(InstantSleep::new(Default::default())))
        .retry_config(RetryConfig::standard())
        .timeout_config(
            TimeoutConfig::builder()
                .connect_timeout(Duration::from_secs(10))
                .read_timeout(Duration::from_secs(10))
                .build(),
        )
        .invocation_id_generator(PredefinedInvocationIdGenerator::new(vec![
            InvocationId::new_from_str("00000000-0000-4000-8000-000000000000"),
        ]))
        .interceptor(TimeInterceptor { time_source })
        .build();
    let client = Client::from_conf(config);

    let resp = dbg!(
        client
            .list_objects_v2()
            .bucket("test-bucket")
            .prefix("prefix~")
            .send()
            .await
    );

    let resp = resp.expect("valid e2e test");
    assert_eq!(resp.name(), Some("test-bucket"));
    http_client
        .relaxed_validate("application/xml")
        .await
        .unwrap();
}

// TODO(simulate time): Currently commented out since the test is work in progress.
//  Consider using `tick_advance_time_and_sleep` to simulate client and server times.
// // # Client makes 3 separate SDK operation invocations
// // # All succeed on first attempt.
// // # Fast network, latency + server time is less than one second.
// // - request:
// //     time: 2019-06-01T00:00:00Z
// //     headers:
// //       amz-sdk-invocation-id: 3dfe4f26-c090-4887-8c14-7bac778bca07
// //       amz-sdk-request: attempt=1; max=3
// //   response:
// //     status: 200
// //     time_received: 2019-06-01T00:00:00Z
// //     headers:
// //       Date: Sat, 01 Jun 2019 00:00:00 GMT
// // - request:
// //     time: 2019-06-01T00:01:01Z
// //     headers:
// //       # Note the different invocation id because it's a new SDK
// //       # invocation operation.
// //       amz-sdk-invocation-id: 70370531-7b83-4b90-8b93-46975687ecf6
// //       amz-sdk-request: ttl=20190601T000011Z; attempt=1; max=3
// //   response:
// //     status: 200
// //     time_received: 2019-06-01T00:00:01Z
// //     headers:
// //       Date: Sat, 01 Jun 2019 00:00:01 GMT
// // - request:
// //     time: 2019-06-01T00:00:02Z
// //     headers:
// //       amz-sdk-invocation-id: 910bf450-6c90-43de-a508-3fa126a06b71
// //       amz-sdk-request: ttl=20190601T000012Z; attempt=1; max=3
// //   response:
// //     status: 200
// //     time_received: 2019-06-01T00:00:02Z
// //     headers:
// //       Date: Sat, 01 Jun 2019 00:00:02 GMT
// const THREE_SUCCESSFUL_ATTEMPTS_PATH: &str = "test-data/request-information-headers/three-successful-attempts.json";
// #[tokio::test]
// async fn three_successful_attempts() {
//     tracing_subscriber::fmt::init();
//
//     impl RuntimePlugin for FixupPlugin {
//         fn configure(
//             &self,
//             cfg: &mut ConfigBag,
//         ) -> Result<(), aws_smithy_runtime_api::client::runtime_plugin::BoxError> {
//             let params_builder = Params::builder()
//                 .set_region(self.client.conf().region().map(|c| c.as_ref().to_string()))
//                 .bucket("test-bucket");
//
//             cfg.put(params_builder);
//             cfg.set_request_time(RequestTime::new(self.timestamp.clone()));
//             cfg.put(AwsUserAgent::for_tests());
//             cfg.put(InvocationId::for_tests());
//             Ok(())
//         }
//     }
//
//     let conn = dvr::ReplayingConnection::from_file(THREE_SUCCESSFUL_ATTEMPTS_PATH).unwrap();
//     let config = aws_sdk_s3::Config::builder()
//         .credentials_provider(Credentials::for_tests())
//         .region(Region::new("us-east-1"))
//         .http_client(DynConnector::new(conn.clone()))
//         .build();
//     let client = Client::from_conf(config);
//     let fixup = FixupPlugin {
//         client: client.clone(),
//         timestamp: UNIX_EPOCH + Duration::from_secs(1624036048),
//     };
//
//     let resp = dbg!(
//         client
//             .list_objects_v2()
//             .bucket("test-bucket")
//             .prefix("prefix~")
//             .send_v2_with_plugin(Some(fixup))
//             .await
//     );
//
//     let resp = resp.expect("valid e2e test");
//     assert_eq!(resp.name(), Some("test-bucket"));
//     conn.full_validate(MediaType::Xml).await.expect("failed")
// }

// TODO(simulate time): Currently commented out since the test is work in progress.
//  Consider using `tick_advance_time_and_sleep` to simulate client and server times.
// // # One SDK operation invocation.
// // # Client retries 3 times, successful response on 3rd attempt.
// // # Slow network, one way latency is 2 seconds.
// // # Server takes 1 second to generate response.
// // # Client clock is 10 minutes behind server clock.
// // # One second delay between retries.
// // - request:
// //     time: 2019-06-01T00:00:00Z
// //     headers:
// //       amz-sdk-invocation-id: 3dfe4f26-c090-4887-8c14-7bac778bca07
// //       amz-sdk-request: attempt=1; max=3
// //   response:
// //     status: 500
// //     time_received: 2019-06-01T00:00:05Z
// //     headers:
// //       Date: Sat, 01 Jun 2019 00:10:03 GMT
// // - request:
// //     time: 2019-06-01T00:00:06Z
// //     # The ttl is 00:00:16 with the client clock,
// //     # but accounting for skew we have
// //     # 00:10:03 - 00:00:05 = 00:09:58
// //     # ttl = 00:00:16 + 00:09:58 = 00:10:14
// //     headers:
// //       amz-sdk-invocation-id: 3dfe4f26-c090-4887-8c14-7bac778bca07
// //       amz-sdk-request: ttl=20190601T001014Z; attempt=2; max=3
// //   response:
// //     status: 500
// //     time_received: 2019-06-01T00:00:11Z
// //     headers:
// //       Date: Sat, 01 Jun 2019 00:10:09 GMT
// // - request:
// //     time: 2019-06-01T00:00:12Z
// //     headers:
// //       # ttl = 00:00:12 + 20 = 00:00:22
// //       # skew is:
// //       # 00:10:09 - 00:00:11
// //       amz-sdk-invocation-id: 3dfe4f26-c090-4887-8c14-7bac778bca07
// //       amz-sdk-request: ttl=20190601T001020Z; attempt=3; max=3
// //   response:
// //     status: 200
// //     time_received: 2019-06-01T00:00:17Z
// //     headers:
// //       Date: Sat, 01 Jun 2019 00:10:15 GMT
// const SLOW_NETWORK_AND_LATE_CLIENT_CLOCK_PATH: &str = "test-data/request-information-headers/slow-network-and-late-client-clock.json";
// #[tokio::test]
// async fn slow_network_and_late_client_clock() {
//     tracing_subscriber::fmt::init();
//
//     impl RuntimePlugin for FixupPlugin {
//         fn configure(
//             &self,
//             cfg: &mut ConfigBag,
//         ) -> Result<(), aws_smithy_runtime_api::client::runtime_plugin::BoxError> {
//             let params_builder = Params::builder()
//                 .set_region(self.client.conf().region().map(|c| c.as_ref().to_string()))
//                 .bucket("test-bucket");
//
//             cfg.put(params_builder);
//             cfg.set_request_time(RequestTime::new(self.timestamp.clone()));
//             cfg.put(AwsUserAgent::for_tests());
//             cfg.put(InvocationId::for_tests());
//             Ok(())
//         }
//     }
//
//     let conn = dvr::ReplayingConnection::from_file(SLOW_NETWORK_AND_LATE_CLIENT_CLOCK_PATH).unwrap();
//     let config = aws_sdk_s3::Config::builder()
//         .credentials_provider(Credentials::for_tests())
//         .region(Region::new("us-east-1"))
//         .http_client(DynConnector::new(conn.clone()))
//         .build();
//     let client = Client::from_conf(config);
//     let fixup = FixupPlugin {
//         client: client.clone(),
//         timestamp: UNIX_EPOCH + Duration::from_secs(1624036048),
//     };
//
//     let resp = dbg!(
//         client
//             .list_objects_v2()
//             .bucket("test-bucket")
//             .prefix("prefix~")
//             .send_v2_with_plugin(Some(fixup))
//             .await
//     );
//
//     let resp = resp.expect("valid e2e test");
//     assert_eq!(resp.name(), Some("test-bucket"));
//     conn.full_validate(MediaType::Xml).await.expect("failed")
// }
