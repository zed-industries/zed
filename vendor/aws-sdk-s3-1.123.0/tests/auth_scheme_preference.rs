/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sdk_s3::config::Region;
use aws_sdk_s3::{Client, Config};
use aws_smithy_http_client::test_util::capture_request;

// S3 is one of the servies that relies on endpoint-based auth scheme resolution.
// An auth scheme preference should not be overridden by other resolution methods.

#[tracing_test::traced_test]
#[tokio::test]
async fn auth_scheme_preference_at_client_level_should_take_the_highest_priority() {
    let (http_client, _) = capture_request(None);
    let conf = Config::builder()
        .http_client(http_client)
        .region(Region::new("us-east-2"))
        .with_test_defaults()
        // Explicitly set a preference that favors `sigv4`, otherwise `sigv4a`
        // would normally be resolved based on the endpoint authSchemes property.
        .auth_scheme_preference([aws_runtime::auth::sigv4::SCHEME_ID])
        .build();
    let client = Client::from_conf(conf);
    let _ = client
        .get_object()
        .bucket("arn:aws:s3::123456789012:accesspoint/mfzwi23gnjvgw.mrap")
        .key("doesnotmatter")
        .send()
        .await;

    assert!(logs_contain(&format!(
        "resolving identity scheme_id=AuthSchemeId {{ scheme_id: \"{auth_scheme_id_str}\" }}",
        auth_scheme_id_str = aws_runtime::auth::sigv4::SCHEME_ID.inner(),
    )));
}

#[tracing_test::traced_test]
#[tokio::test]
async fn auth_scheme_preference_at_operation_level_should_take_the_highest_priority() {
    let (http_client, _) = capture_request(None);
    let conf = Config::builder()
        .http_client(http_client)
        .region(Region::new("us-east-2"))
        .with_test_defaults()
        .build();
    let client = Client::from_conf(conf);
    let _ = client
        .get_object()
        .bucket("arn:aws:s3::123456789012:accesspoint/mfzwi23gnjvgw.mrap")
        .key("doesnotmatter")
        .customize()
        .config_override(
            // Explicitly set a preference that favors `sigv4`, otherwise `sigv4a`
            // would normally be resolved based on the endpoint authSchemes property.
            Config::builder().auth_scheme_preference([aws_runtime::auth::sigv4::SCHEME_ID]),
        )
        .send()
        .await;

    assert!(logs_contain(&format!(
        "resolving identity scheme_id=AuthSchemeId {{ scheme_id: \"{auth_scheme_id_str}\" }}",
        auth_scheme_id_str = aws_runtime::auth::sigv4::SCHEME_ID.inner(),
    )));
}
