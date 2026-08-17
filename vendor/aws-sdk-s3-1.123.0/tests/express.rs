/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::time::{Duration, SystemTime};

use aws_config::timeout::TimeoutConfig;
use aws_config::Region;
use aws_runtime::user_agent::test_util::{
    assert_ua_contains_metric_values, assert_ua_does_not_contain_metric_values,
};
use aws_sdk_s3::config::endpoint::{EndpointFuture, Params, ResolveEndpoint};
use aws_sdk_s3::config::{Builder, Credentials};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::SdkBody;
use aws_sdk_s3::types::ChecksumAlgorithm;
use aws_sdk_s3::{Client, Config};
use aws_smithy_http_client::test_util::dvr::ReplayingClient;
use aws_smithy_http_client::test_util::{capture_request, ReplayEvent, StaticReplayClient};
use aws_smithy_runtime::test_util::capture_test_logs::capture_test_logs;
use aws_smithy_types::endpoint::Endpoint;
use http_1x::Uri;

async fn test_client<F>(update_builder: F) -> Client
where
    F: Fn(Builder) -> Builder,
{
    let sdk_config = aws_config::from_env().region("us-west-2").load().await;
    let config = Config::from(&sdk_config).to_builder().with_test_defaults();
    aws_sdk_s3::Client::from_conf(update_builder(config).build())
}

#[tokio::test]
async fn create_session_request_should_not_include_x_amz_s3session_token() {
    let (http_client, request) = capture_request(None);
    // There was a bug where a regular SigV4 session token was overwritten by an express session token
    // even for CreateSession API request.
    // To exercise that code path, it is important to include credentials with a session token below.
    let conf = Config::builder()
        .http_client(http_client)
        .region(Region::new("us-west-2"))
        .credentials_provider(::aws_credential_types::Credentials::for_tests_with_session_token())
        .build();
    let client = Client::from_conf(conf);

    let _ = client
        .list_objects_v2()
        .bucket("s3express-test-bucket--usw2-az1--x-s3")
        .send()
        .await;

    let req = request.expect_request();
    assert!(
        req.headers().get("x-amz-create-session-mode").is_none(),
        "`x-amz-create-session-mode` should not appear in headers of the first request when an express bucket is specified"
    );
    assert!(req.headers().get("x-amz-security-token").is_some());
    assert!(req.headers().get("x-amz-s3session-token").is_none());

    // The first request uses regular SigV4 credentials (for CreateSession), not S3 Express credentials,
    // so metric "J" should NOT be present yet. It will appear on subsequent requests that use S3 Express credentials.
    let user_agent = req.headers().get("x-amz-user-agent").unwrap();
    assert_ua_does_not_contain_metric_values(user_agent, &["J"]);
}

#[tokio::test]
async fn mixed_auths() {
    let _logs = capture_test_logs();

    let http_client = ReplayingClient::from_file("tests/data/express/mixed-auths.json").unwrap();
    let client = test_client(|b| b.http_client(http_client.clone())).await;

    // A call to an S3 Express bucket where we should see two request/response pairs,
    // one for the `create_session` API and the other for `list_objects_v2` in S3 Express bucket.
    let result = client
        .list_objects_v2()
        .bucket("s3express-test-bucket--usw2-az1--x-s3")
        .send()
        .await;
    dbg!(result).expect("success");

    // A call to a regular bucket, and request headers should not contain `x-amz-s3session-token`.
    let result = client
        .list_objects_v2()
        .bucket("regular-test-bucket")
        .send()
        .await;
    dbg!(result).expect("success");

    // A call to another S3 Express bucket where we should again see two request/response pairs,
    // one for the `create_session` API and the other for `list_objects_v2` in S3 Express bucket.
    let result = client
        .list_objects_v2()
        .bucket("s3express-test-bucket-2--usw2-az3--x-s3")
        .send()
        .await;
    dbg!(result).expect("success");

    // This call should be an identity cache hit for the first S3 Express bucket,
    // thus no HTTP request should be sent to the `create_session` API.
    let result = client
        .list_objects_v2()
        .bucket("s3express-test-bucket--usw2-az1--x-s3")
        .send()
        .await;
    dbg!(result).expect("success");

    http_client
        .validate_body_and_headers(Some(&["x-amz-s3session-token"]), "application/xml")
        .await
        .unwrap();
}

fn create_session_request() -> http_1x::Request<SdkBody> {
    http_1x::Request::builder()
        .uri("https://s3express-test-bucket--usw2-az1--x-s3.s3express-usw2-az1.us-west-2.amazonaws.com/?session")
        .method("GET")
        .body(SdkBody::empty())
        .unwrap()
}

fn create_session_response() -> http_1x::Response<SdkBody> {
    http_1x::Response::builder()
        .status(200)
        .body(SdkBody::from(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <CreateSessionResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
                <Credentials>
                    <SessionToken>TESTSESSIONTOKEN</SessionToken>
                    <SecretAccessKey>TESTSECRETKEY</SecretAccessKey>
                    <AccessKeyId>ASIARTESTID</AccessKeyId>
                    <Expiration>2024-01-29T18:53:01Z</Expiration>
                </Credentials>
            </CreateSessionResult>
            "#,
        ))
        .unwrap()
}

#[tokio::test]
async fn presigning() {
    let http_client = StaticReplayClient::new(vec![ReplayEvent::new(
        create_session_request(),
        create_session_response(),
    )]);

    let client = test_client(|b| b.http_client(http_client.clone())).await;

    let presigning_config = PresigningConfig::builder()
        .start_time(SystemTime::UNIX_EPOCH + Duration::from_secs(1234567891))
        .expires_in(Duration::from_secs(30))
        .build()
        .unwrap();

    let presigned = client
        .get_object()
        .bucket("s3express-test-bucket--usw2-az1--x-s3")
        .key("ferris.png")
        .presigned(presigning_config)
        .await
        .unwrap();

    let uri = presigned.uri().parse::<Uri>().unwrap();

    let pq = uri.path_and_query().unwrap();
    let path = pq.path();
    let query = pq.query().unwrap();
    let mut query_params: Vec<&str> = query.split('&').collect();
    query_params.sort();

    pretty_assertions::assert_eq!(
        "s3express-test-bucket--usw2-az1--x-s3.s3express-usw2-az1.us-west-2.amazonaws.com",
        uri.authority().unwrap()
    );
    assert_eq!("GET", presigned.method());
    assert_eq!("/ferris.png", path);
    pretty_assertions::assert_eq!(
        &[
            "X-Amz-Algorithm=AWS4-HMAC-SHA256",
            "X-Amz-Credential=ASIARTESTID%2F20090213%2Fus-west-2%2Fs3express%2Faws4_request",
            "X-Amz-Date=20090213T233131Z",
            "X-Amz-Expires=30",
            "X-Amz-S3session-Token=TESTSESSIONTOKEN",
            "X-Amz-Signature=c09c93c7878184492cb960d59e148af932dff6b19609e63e3484599903d97e44",
            "X-Amz-SignedHeaders=host",
            "x-id=GetObject"
        ][..],
        &query_params
    );
    // Presigned request has no headers by default
    assert_eq!(presigned.headers().count(), 0);
}

fn operation_request_with_checksum(
    query: &str,
    kv: Option<(&str, &str)>,
) -> http_1x::Request<SdkBody> {
    let mut b = http_1x::Request::builder()
        .uri(&format!("https://s3express-test-bucket--usw2-az1--x-s3.s3express-usw2-az1.us-west-2.amazonaws.com/{query}"))
        .method("GET");
    if let Some((key, value)) = kv {
        b = b.header(key, value);
    }
    b.body(SdkBody::empty()).unwrap()
}

fn response_ok() -> http_1x::Response<SdkBody> {
    http_1x::Response::builder()
        .status(200)
        .body(SdkBody::empty())
        .unwrap()
}

#[tokio::test]
async fn user_specified_checksum_should_be_respected() {
    async fn runner(checksum: ChecksumAlgorithm, value: &str) {
        let http_client = StaticReplayClient::new(vec![
            ReplayEvent::new(create_session_request(), create_session_response()),
            ReplayEvent::new(
                operation_request_with_checksum(
                    "test?x-id=PutObject",
                    Some((
                        &format!("x-amz-checksum-{}", checksum.as_str().to_lowercase()),
                        &format!("{value}"),
                    )),
                ),
                response_ok(),
            ),
        ]);
        let client = test_client(|b| b.http_client(http_client.clone())).await;

        let _ = client
            .put_object()
            .bucket("s3express-test-bucket--usw2-az1--x-s3")
            .key("test")
            .body(SdkBody::empty().into())
            .checksum_algorithm(checksum)
            .send()
            .await;

        http_client.assert_requests_match(&[""]);
    }

    let checksum_value_pairs = &[
        (ChecksumAlgorithm::Crc32, "AAAAAA=="),
        (ChecksumAlgorithm::Crc32C, "AAAAAA=="),
        (ChecksumAlgorithm::Sha1, "2jmj7l5rSw0yVb/vlWAYkK/YBwk="),
        (
            ChecksumAlgorithm::Sha256,
            "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=",
        ),
    ];
    for (checksum, value) in checksum_value_pairs {
        runner(checksum.clone(), *value).await;
    }
}

#[tokio::test]
async fn default_checksum_should_be_crc32_for_operation_requiring_checksum() {
    let http_client = StaticReplayClient::new(vec![
        ReplayEvent::new(create_session_request(), create_session_response()),
        ReplayEvent::new(
            operation_request_with_checksum("?delete", Some(("x-amz-checksum-crc32", "AAAAAA=="))),
            response_ok(),
        ),
    ]);
    let client = test_client(|b| b.http_client(http_client.clone())).await;

    let _ = client
        .delete_objects()
        .bucket("s3express-test-bucket--usw2-az1--x-s3")
        .send()
        .await;

    let checksum_headers: Vec<_> = http_client
        .actual_requests()
        .last()
        .unwrap()
        .headers()
        .iter()
        .filter(|(key, _)| key.starts_with("x-amz-checksum"))
        .collect();
    assert_eq!(1, checksum_headers.len());
    assert_eq!("x-amz-checksum-crc32", checksum_headers[0].0);

    http_client.assert_requests_match(&[""]);
}

#[tokio::test]
async fn default_checksum_should_be_none() {
    let http_client = StaticReplayClient::new(vec![
        ReplayEvent::new(create_session_request(), create_session_response()),
        ReplayEvent::new(
            operation_request_with_checksum("test?x-id=PutObject", None),
            response_ok(),
        ),
    ]);
    let client = test_client(|b| b.http_client(http_client.clone())).await;

    let _ = client
        .put_object()
        .bucket("s3express-test-bucket--usw2-az1--x-s3")
        .key("test")
        .body(SdkBody::empty().into())
        .send()
        .await;

    http_client.assert_requests_match(&[""]);

    let mut all_checksums = ChecksumAlgorithm::values()
        .iter()
        .map(|checksum| format!("amz-checksum-{}", checksum.to_lowercase()))
        .chain(std::iter::once("content-md5".to_string()));
    assert!(!all_checksums.any(|checksum| http_client
        .actual_requests()
        .any(|req| req.headers().iter().any(|(key, _)| key == checksum))));
}

#[tokio::test]
async fn disable_s3_express_session_auth_at_service_client_level() {
    let (http_client, request) = capture_request(None);
    let conf = Config::builder()
        .http_client(http_client)
        .region(Region::new("us-west-2"))
        .with_test_defaults()
        .disable_s3_express_session_auth(true)
        .build();
    let client = Client::from_conf(conf);

    let _ = client
        .list_objects_v2()
        .bucket("s3express-test-bucket--usw2-az1--x-s3")
        .send()
        .await;

    let req = request.expect_request();
    assert!(
        req.headers().get("x-amz-create-session-mode").is_none(),
        "x-amz-create-session-mode should not appear in headers when S3 Express session auth is disabled"
    );

    // Verify that the User-Agent does NOT contain the S3ExpressBucket metric "J" when session auth is disabled
    let user_agent = req.headers().get("x-amz-user-agent").unwrap();
    assert_ua_does_not_contain_metric_values(user_agent, &["J"]);
}

#[tokio::test]
async fn disable_s3_express_session_auth_at_operation_level() {
    let (http_client, request) = capture_request(None);
    let conf = Config::builder()
        .http_client(http_client)
        .region(Region::new("us-west-2"))
        .with_test_defaults()
        .build();
    let client = Client::from_conf(conf);

    let _ = client
        .list_objects_v2()
        .bucket("s3express-test-bucket--usw2-az1--x-s3")
        .customize()
        .config_override(Config::builder().disable_s3_express_session_auth(true))
        .send()
        .await;

    let req = request.expect_request();
    assert!(
        req.headers().get("x-amz-create-session-mode").is_none(),
        "x-amz-create-session-mode should not appear in headers when S3 Express session auth is disabled"
    );
}

#[tokio::test]
async fn support_customer_overriding_express_credentials_provider() {
    let expected_session_token = "testsessiontoken";
    let client_overriding_express_credentials_provider = || async move {
        let (http_client, rx) = capture_request(None);
        let client = test_client(|b| {
            let credentials = Credentials::new(
                "testaccess",
                "testsecret",
                Some(expected_session_token.to_owned()),
                None,
                "test",
            );
            b.http_client(http_client.clone())
                // Pass a credential with a session token so that
                // `x-amz-s3session-token` should appear in the request header
                // when s3 session auth is enabled.
                .express_credentials_provider(credentials.clone())
                // Pass a credential with a session token so that
                // `x-amz-security-token` should appear in the request header
                // when s3 session auth is disabled.
                .credentials_provider(credentials)
        })
        .await;
        (client, rx)
    };

    // Test `x-amz-s3session-token` should be present with `expected_session_token`.
    let (client, rx) = client_overriding_express_credentials_provider().await;
    let _ = client
        .list_objects_v2()
        .bucket("s3express-test-bucket--usw2-az1--x-s3")
        .send()
        .await;

    let req = rx.expect_request();
    let actual_session_token = req
        .headers()
        .get("x-amz-s3session-token")
        .expect("x-amz-s3session-token should be present");
    assert_eq!(expected_session_token, actual_session_token);
    assert!(req.headers().get("x-amz-security-token").is_none());

    // With a regular S3 bucket, test `x-amz-security-token` should be present with `expected_session_token`,
    // instead of `x-amz-s3session-token`.
    let (client, rx) = client_overriding_express_credentials_provider().await;
    let _ = client
        .list_objects_v2()
        .bucket("regular-test-bucket")
        .send()
        .await;

    let req = rx.expect_request();
    let actual_session_token = req
        .headers()
        .get("x-amz-security-token")
        .expect("x-amz-security-token should be present");
    assert_eq!(expected_session_token, actual_session_token);
    assert!(req.headers().get("x-amz-s3session-token").is_none());

    // Verify that the User-Agent does NOT contain the S3ExpressBucket metric "J" for regular buckets
    let user_agent = req.headers().get("x-amz-user-agent").unwrap();
    assert_ua_does_not_contain_metric_values(user_agent, &["J"]);
}

#[tokio::test]
async fn s3_express_auth_flow_should_not_be_reached_with_no_auth_schemes() {
    #[derive(Debug)]
    struct TestResolver {
        url: String,
    }
    impl ResolveEndpoint for TestResolver {
        fn resolve_endpoint(&self, _params: &Params) -> EndpointFuture<'_> {
            EndpointFuture::ready(Ok(Endpoint::builder().url(self.url.clone()).build()))
        }
    }

    let (http_client, request) = capture_request(None);
    let conf = Config::builder()
        .http_client(http_client)
        .region(Region::new("us-west-2"))
        .endpoint_resolver(TestResolver {
            url: "http://127.0.0.1".to_owned(),
        })
        .with_test_defaults()
        .timeout_config(
            TimeoutConfig::builder()
                .operation_attempt_timeout(Duration::from_secs(1))
                .build(),
        )
        .build();
    let client = Client::from_conf(conf);

    // Note that we pass a regular bucket; when the bug was present, it still went through s3 Express auth flow.
    let _ = client.list_objects_v2().bucket("test-bucket").send().await;
    // If s3 Express auth flow were exercised, no request would be received, most likely due to `TimeoutError`.
    let _ = request.expect_request();
}

#[tokio::test]
async fn s3_express_request_contains_metric_j() {
    let _logs = capture_test_logs();

    let http_client = ReplayingClient::from_file("tests/data/express/mixed-auths.json").unwrap();
    let client = test_client(|b| b.http_client(http_client.clone())).await;

    let _result = client
        .list_objects_v2()
        .bucket("s3express-test-bucket--usw2-az1--x-s3")
        .send()
        .await
        .expect("Request should succeed");

    let requests = http_client.take_requests().await;

    let s3_express_request = requests
        .iter()
        .find(|req| req.headers().get("x-amz-s3session-token").is_some())
        .expect("Should have at least one S3 Express request with session token");

    let user_agent = s3_express_request
        .headers()
        .get("x-amz-user-agent")
        .expect("User-Agent header should be present")
        .to_str()
        .expect("User-Agent should be valid UTF-8");

    assert_ua_contains_metric_values(user_agent, &["J"]);
}
