/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(feature = "test-util")]

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Error;
use aws_sdk_s3::{Client, Config};
use aws_smithy_http_client::test_util::capture_request;

#[tokio::test]
async fn test_s3_signer_query_string_with_all_valid_chars() {
    let (http_client, rcvr) = capture_request(None);
    let config = Config::builder()
        .credentials_provider(SharedCredentialsProvider::new(
            Credentials::for_tests_with_session_token(),
        ))
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .with_test_defaults()
        .build();
    let client = Client::from_conf(config);

    // Generate a string containing all printable ASCII chars
    let prefix: String = (32u8..127).map(char::from).collect();

    // The response from the fake connection won't return the expected XML but we don't care about
    // that error in this test
    let _ = client
        .list_objects_v2()
        .bucket("test-bucket")
        .prefix(&prefix)
        .send()
        .await;

    // As long as a request can be extracted and the `Authorization` header exits, we're good.
    // We cannot compare a signature in the `Authorization` header between expected and actual
    // because the signature is subject to change as we update the `x-amz-user-agent` header, e.g.
    // due to the introduction of a new metric.
    let expected_req = rcvr.expect_request();
    let _ = expected_req.headers().get("Authorization").unwrap();
}

// This test can help identify individual characters that break the signing of query strings. This
// test must be run against an actual bucket so we `ignore` it unless the runner specifically requests it
#[tokio::test]
#[ignore]
#[allow(deprecated)]
async fn test_query_strings_are_correctly_encoded() {
    use aws_smithy_runtime_api::client::result::SdkError;

    tracing_subscriber::fmt::init();
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let mut chars_that_break_signing = Vec::new();
    let mut chars_that_break_uri_parsing = Vec::new();
    let mut chars_that_are_invalid_arguments = Vec::new();

    // We test all possible bytes to check for issues with URL construction or signing
    for byte in u8::MIN..u8::MAX {
        let char = char::from(byte);
        let res = client
            .list_objects_v2()
            .bucket("a-bucket-to-test-with")
            .prefix(char)
            .send()
            .await;
        if let Err(SdkError::ServiceError(context)) = res {
            let err = context.err();
            let msg = err.to_string();
            let unhandled = matches!(err, ListObjectsV2Error::Unhandled(_));
            if unhandled && msg.contains("SignatureDoesNotMatch") {
                chars_that_break_signing.push(byte);
            } else if unhandled && msg.to_string().contains("InvalidUri") {
                chars_that_break_uri_parsing.push(byte);
            } else if unhandled && msg.to_string().contains("InvalidArgument") {
                chars_that_are_invalid_arguments.push(byte);
            } else if unhandled && msg.to_string().contains("InvalidToken") {
                panic!("refresh your credentials and run this test again");
            } else {
                todo!("unexpected error: {:?}", err);
            }
        }
    }

    if chars_that_break_signing.is_empty()
        && chars_that_break_uri_parsing.is_empty()
        && chars_that_are_invalid_arguments.is_empty()
    {
        return;
    }

    fn char_transform(c: u8) -> String {
        format!("byte {}: {}\n", c, char::from(c))
    }
    if !chars_that_break_signing.is_empty() {
        eprintln!(
            "The following characters caused a signature mismatch:\n{}(end)",
            chars_that_break_signing
                .clone()
                .into_iter()
                .map(char_transform)
                .collect::<String>()
        );
    }
    if !chars_that_break_uri_parsing.is_empty() {
        eprintln!(
            "The following characters caused a URI parse failure:\n{}(end)",
            chars_that_break_uri_parsing
                .clone()
                .into_iter()
                .map(char_transform)
                .collect::<String>()
        );
    }
    if !chars_that_are_invalid_arguments.is_empty() {
        eprintln!(
            "The following characters caused an \"Invalid Argument\" failure:\n{}(end)",
            chars_that_are_invalid_arguments
                .clone()
                .into_iter()
                .map(char_transform)
                .collect::<String>()
        );
    }

    panic!("test failed due to invalid characters")
}
