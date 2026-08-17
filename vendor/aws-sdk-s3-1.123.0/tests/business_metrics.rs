/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_config::Region;
use aws_runtime::{
    sdk_feature::AwsSdkFeature, user_agent::test_util::assert_ua_contains_metric_values,
};
use aws_sdk_s3::{
    config::{Intercept, IntoShared},
    primitives::ByteStream,
    Client, Config,
};
use aws_smithy_http_client::test_util::capture_request;

#[derive(Debug)]
struct TransferManagerFeatureInterceptor;

impl Intercept for TransferManagerFeatureInterceptor {
    fn name(&self) -> &'static str {
        "TransferManagerFeature"
    }

    fn read_before_execution(
        &self,
        _ctx: &aws_sdk_s3::config::interceptors::BeforeSerializationInterceptorContextRef<'_>,
        cfg: &mut aws_sdk_s3::config::ConfigBag,
    ) -> Result<(), aws_sdk_s3::error::BoxError> {
        cfg.interceptor_state()
            .store_append(AwsSdkFeature::S3Transfer);
        Ok(())
    }
}

#[tokio::test]
async fn test_track_metric_for_s3_transfer_manager() {
    let (http_client, captured_request) = capture_request(None);
    let mut conf_builder = Config::builder()
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .with_test_defaults();
    // The S3 Transfer Manager uses a passed-in S3 client SDK for operations.
    // By configuring an interceptor at the client level to track metrics,
    // all operations executed by the client will automatically include the metric.
    // This eliminates the need to apply `.config_override` on individual operations
    // to insert the `TransferManagerFeatureInterceptor`.
    conf_builder.push_interceptor(TransferManagerFeatureInterceptor.into_shared());
    let client = Client::from_conf(conf_builder.build());

    let _ = client
        .put_object()
        .bucket("doesnotmatter")
        .key("doesnotmatter")
        .body(ByteStream::from_static("Hello, world".as_bytes()))
        .send()
        .await
        .unwrap();

    let expected_req = captured_request.expect_request();
    let user_agent = expected_req.headers().get("x-amz-user-agent").unwrap();
    assert_ua_contains_metric_values(user_agent, &["G"]);
}
