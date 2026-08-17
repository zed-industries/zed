/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sdk_s3::config::Region;
use aws_sdk_s3::{Client, Config};
use aws_smithy_http_client::test_util::capture_request;
use aws_smithy_http_client::test_util::dvr::ReplayingClient;
use aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID;
use aws_smithy_runtime::test_util::capture_test_logs::capture_test_logs;
use aws_smithy_runtime_api::client::auth::AuthSchemeId;

#[tokio::test]
async fn list_objects() {
    let _logs = capture_test_logs();

    let http_client = ReplayingClient::from_file("tests/data/no_auth/list-objects.json").unwrap();
    let config = aws_config::from_env()
        .http_client(http_client.clone())
        .no_credentials()
        .region("us-east-1")
        .load()
        .await;
    let config = Config::from(&config)
        .to_builder()
        .with_test_defaults()
        .build();
    let client = aws_sdk_s3::Client::from_conf(config);

    let result = client
        .list_objects()
        .bucket("gdc-organoid-pancreatic-phs001611-2-open")
        .max_keys(3)
        .send()
        .await;
    dbg!(result).expect("success");

    http_client
        .relaxed_validate("application/xml")
        .await
        .unwrap();
}

#[tokio::test]
async fn list_objects_v2() {
    let _logs = capture_test_logs();

    let http_client =
        ReplayingClient::from_file("tests/data/no_auth/list-objects-v2.json").unwrap();
    let config = aws_config::from_env()
        .http_client(http_client.clone())
        .no_credentials()
        .region("us-east-1")
        .load()
        .await;
    let config = Config::from(&config)
        .to_builder()
        .with_test_defaults()
        .build();
    let client = Client::from_conf(config);

    let result = client
        .list_objects_v2()
        .bucket("gdc-organoid-pancreatic-phs001611-2-open")
        .max_keys(3)
        .send()
        .await;
    dbg!(result).expect("success");

    http_client
        .relaxed_validate("application/xml")
        .await
        .unwrap();
}

#[tokio::test]
async fn head_object() {
    let _logs = capture_test_logs();

    let http_client = ReplayingClient::from_file("tests/data/no_auth/head-object.json").unwrap();
    let config = aws_config::from_env()
        .http_client(http_client.clone())
        .no_credentials()
        .region("us-east-1")
        .load()
        .await;
    let config = Config::from(&config)
        .to_builder()
        .with_test_defaults()
        .build();
    let client = Client::from_conf(config);

    let result = client
        .head_object()
        .bucket("gdc-organoid-pancreatic-phs001611-2-open")
        .key("0431cddc-a418-4a79-a34d-6c041394e8e4/a6ddcc84-8e4d-4c68-885c-2d51168eec97.FPKM-UQ.txt.gz")
        .send()
        .await;
    dbg!(result).expect("success");

    http_client
        .relaxed_validate("application/xml")
        .await
        .unwrap();
}

#[tokio::test]
async fn get_object() {
    let _logs = capture_test_logs();

    let http_client = ReplayingClient::from_file("tests/data/no_auth/get-object.json").unwrap();
    let config = aws_config::from_env()
        .http_client(http_client.clone())
        .no_credentials()
        .region("us-east-1")
        .load()
        .await;
    let config = Config::from(&config)
        .to_builder()
        .with_test_defaults()
        .build();
    let client = Client::from_conf(config);

    let result = client
        .get_object()
        .bucket("gdc-organoid-pancreatic-phs001611-2-open")
        .key("0431cddc-a418-4a79-a34d-6c041394e8e4/a6ddcc84-8e4d-4c68-885c-2d51168eec97.FPKM-UQ.txt.gz")
        .send()
        .await;
    dbg!(result).expect("success");

    http_client
        .relaxed_validate("application/xml")
        .await
        .unwrap();
}

#[tracing_test::traced_test]
#[tokio::test]
async fn no_auth_should_be_selected_when_no_credentials_is_configured() {
    let (http_client, _) = capture_request(None);
    let config = aws_config::from_env()
        .http_client(http_client)
        .region(Region::new("us-east-2"))
        .no_credentials()
        .load()
        .await;

    let client = Client::new(&config);
    let _ = dbg!(
        client
            .list_objects_v2()
            .bucket("doesnotmatter")
            .send()
            .await
    );

    assert!(logs_contain(&format!(
        "resolving identity scheme_id=AuthSchemeId {{ scheme_id: \"{auth_scheme_id_str}\" }}",
        auth_scheme_id_str = NO_AUTH_SCHEME_ID.inner(),
    )));
}

#[tracing_test::traced_test]
#[tokio::test]
async fn auth_scheme_preference_specifying_legacy_no_auth_scheme_id_should_be_supported() {
    let (http_client, _) = capture_request(None);
    let conf = Config::builder()
        .http_client(http_client)
        .region(Region::new("us-east-2"))
        .with_test_defaults()
        .auth_scheme_preference([AuthSchemeId::from("no_auth")])
        .build();
    let client = Client::from_conf(conf);
    let _ = client
        .get_object()
        .bucket("arn:aws:s3::123456789012:accesspoint/mfzwi23gnjvgw.mrap")
        .key("doesnotmatter")
        .send()
        .await;

    // What should appear in the log is the updated no auth scheme ID, not the legacy one.
    // The legacy no auth scheme ID passed to the auth scheme preference merely reprioritizes
    // ones supported in the runtime, which should contain the updated no auth scheme ID.
    assert!(logs_contain(&format!(
        "resolving identity scheme_id=AuthSchemeId {{ scheme_id: \"{auth_scheme_id_str}\" }}",
        auth_scheme_id_str = NO_AUTH_SCHEME_ID.inner(),
    )));
}
