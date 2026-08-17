/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_runtime::env_config::EnvConfigValue;
use aws_runtime::user_agent::test_util::{
    assert_ua_contains_metric_values, assert_ua_does_not_contain_metric_values,
};
use aws_sdk_bedrockruntime::config::{Region, Token};
use aws_sdk_bedrockruntime::error::DisplayErrorContext;
use aws_smithy_http_client::test_util::capture_request;
use aws_smithy_runtime::assert_str_contains;
use aws_smithy_runtime_api::client::auth::http::HTTP_BEARER_AUTH_SCHEME_ID;
use aws_types::origin::Origin;
use aws_types::os_shim_internal::Env;
use aws_types::service_config::{LoadServiceConfig, ServiceConfigKey};
use aws_types::SdkConfig;

#[derive(Debug)]
struct TestEnv {
    env: Env,
}

impl LoadServiceConfig for TestEnv {
    fn load_config(&self, key: ServiceConfigKey<'_>) -> Option<String> {
        let (value, _source) = EnvConfigValue::new()
            .env(key.env())
            .profile(key.profile())
            .service_id(key.service_id())
            .load(&self.env, None)?;

        Some(value.to_string())
    }
}

#[tokio::test]
async fn test_valid_service_specific_token_configured() {
    let (http_client, captured_request) = capture_request(None);
    let expected_token = "bedrock-token";
    let shared_config = SdkConfig::builder()
        .region(Region::new("us-west-2"))
        .http_client(http_client)
        .service_config(TestEnv {
            env: Env::from_slice(&[("AWS_BEARER_TOKEN_BEDROCK", expected_token)]),
        })
        .build();
    let client = aws_sdk_bedrockruntime::Client::new(&shared_config);
    let _ = client
        .get_async_invoke()
        .invocation_arn("arn:aws:bedrock:us-west-2:123456789012:invoke/ExampleModel")
        .send()
        .await;
    let request = captured_request.expect_request();
    let authorization_header = request.headers().get("authorization").unwrap();
    assert!(authorization_header.starts_with(&format!("Bearer {expected_token}")));

    // Verify that the user agent contains the expected metric value (BEARER_SERVICE_ENV_VARS: 3)
    let user_agent = request.headers().get("x-amz-user-agent").unwrap();
    assert_ua_contains_metric_values(user_agent, &["3"]);
}

#[tokio::test]
async fn test_token_configured_for_different_service() {
    let (http_client, _) = capture_request(None);
    let shared_config = SdkConfig::builder()
        .region(Region::new("us-west-2"))
        .http_client(http_client)
        .service_config(TestEnv {
            env: Env::from_slice(&[("AWS_BEARER_TOKEN_FOO", "foo-token")]),
        })
        .build();
    let client = aws_sdk_bedrockruntime::Client::new(&shared_config);
    let err = client
        .get_async_invoke()
        .invocation_arn("arn:aws:bedrock:us-west-2:123456789012:invoke/ExampleModel")
        .send()
        .await
        .unwrap_err();
    assert_str_contains!(
        format!("{}", DisplayErrorContext(err)),
        "failed to select an auth scheme to sign the request with."
    );
}

#[tokio::test]
async fn test_token_configured_with_auth_scheme_preference_also_set_in_env() {
    let (http_client, captured_request) = capture_request(None);
    let expected_token = "bedrock-token";
    let mut shared_config = SdkConfig::builder()
        .region(Region::new("us-west-2"))
        .http_client(http_client)
        .service_config(TestEnv {
            env: Env::from_slice(&[("AWS_BEARER_TOKEN_BEDROCK", expected_token)]),
        })
        .auth_scheme_preference([
            aws_runtime::auth::sigv4::SCHEME_ID,
            HTTP_BEARER_AUTH_SCHEME_ID,
        ]);
    // Pretend as if the auth scheme preference were set through the environment variable
    shared_config.insert_origin(
        "auth_scheme_preference",
        Origin::shared_environment_variable(),
    );
    let shared_config = shared_config.build();
    let client = aws_sdk_bedrockruntime::Client::new(&shared_config);
    let _ = client
        .get_async_invoke()
        .invocation_arn("arn:aws:bedrock:us-west-2:123456789012:invoke/ExampleModel")
        .send()
        .await;
    let request = captured_request.expect_request();
    let authorization_header = request.headers().get("authorization").unwrap();
    assert!(authorization_header.starts_with(&format!("Bearer {expected_token}")));

    // Verify that the user agent contains the expected metric value (BEARER_SERVICE_ENV_VARS: 3)
    let user_agent = request.headers().get("x-amz-user-agent").unwrap();
    assert_ua_contains_metric_values(user_agent, &["3"]);
}

#[tokio::test]
async fn test_explicit_service_config_takes_precedence() {
    let (http_client, captured_request) = capture_request(None);
    let shared_config = SdkConfig::builder()
        .region(Region::new("us-west-2"))
        .http_client(http_client)
        .service_config(TestEnv {
            env: Env::from_slice(&[("AWS_BEARER_TOKEN_BEDROCK", "bedrock-token")]),
        })
        .build();
    let expected_token = "explicit-code-token";
    let conf = aws_sdk_bedrockruntime::config::Builder::from(&shared_config)
        .token_provider(Token::new(expected_token, None))
        .build();
    let client = aws_sdk_bedrockruntime::Client::from_conf(conf);
    let _ = client
        .get_async_invoke()
        .invocation_arn("arn:aws:bedrock:us-west-2:123456789012:invoke/ExampleModel")
        .send()
        .await;
    let request = captured_request.expect_request();
    let authorization_header = request.headers().get("authorization").unwrap();
    assert!(authorization_header.starts_with(&format!("Bearer {expected_token}")));

    // Verify that the user agent does NOT contain the expected metric value (BEARER_SERVICE_ENV_VARS: 3)
    // since the token explicitly set in code was used.
    let user_agent = request.headers().get("x-amz-user-agent").unwrap();
    assert_ua_does_not_contain_metric_values(user_agent, &["3"]);
}
