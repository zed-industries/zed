/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::operation::{RequestId, RequestIdExt};
use aws_sdk_s3::{config::Credentials, config::Region, Client, Config};
use aws_smithy_http_client::test_util::capture_request;
use aws_smithy_types::body::SdkBody;

#[tokio::test]
async fn get_request_id_from_modeled_error() {
    let (http_client, request) = capture_request(Some(
        http_1x::Response::builder()
            .header("x-amz-request-id", "correct-request-id")
            .header("x-amz-id-2", "correct-extended-request-id")
            .status(404)
            .body(SdkBody::from(
                r#"<?xml version="1.0" encoding="UTF-8"?>
                <Error>
                  <Code>NoSuchKey</Code>
                  <Message>The resource you requested does not exist</Message>
                  <Resource>/mybucket/myfoto.jpg</Resource>
                  <RequestId>incorrect-request-id</RequestId>
                </Error>"#,
            ))
            .unwrap(),
    ));
    let config = Config::builder()
        .http_client(http_client)
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .build();
    let client = Client::from_conf(config);
    let err = client
        .get_object()
        .key("dontcare")
        .bucket("dontcare")
        .send()
        .await
        .expect_err("status was 404, this is an error")
        .into_service_error();
    request.expect_request();
    assert!(
        matches!(err, GetObjectError::NoSuchKey(_)),
        "expected NoSuchKey, got {err:?}",
    );
    assert_eq!(Some("correct-request-id"), err.request_id());
    assert_eq!(Some("correct-request-id"), err.meta().request_id());
    assert_eq!(
        Some("correct-extended-request-id"),
        err.extended_request_id()
    );
    assert_eq!(
        Some("correct-extended-request-id"),
        err.meta().extended_request_id()
    );
}

#[tokio::test]
#[allow(deprecated)]
async fn get_request_id_from_unmodeled_error() {
    let (http_client, request) = capture_request(Some(
        http_1x::Response::builder()
            .header("x-amz-request-id", "correct-request-id")
            .header("x-amz-id-2", "correct-extended-request-id")
            .status(500)
            .body(SdkBody::from(
                r#"<?xml version="1.0" encoding="UTF-8"?>
                <Error>
                  <Code>SomeUnmodeledError</Code>
                  <Message>Something bad happened</Message>
                  <Resource>/mybucket/myfoto.jpg</Resource>
                  <RequestId>incorrect-request-id</RequestId>
                </Error>"#,
            ))
            .unwrap(),
    ));
    let config = Config::builder()
        .http_client(http_client)
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .retry_config(aws_smithy_types::retry::RetryConfig::disabled())
        .build();
    let client = Client::from_conf(config);
    let err = client
        .get_object()
        .bucket("dontcare")
        .key("dontcare")
        .send()
        .await
        .expect_err("status 500")
        .into_service_error();
    request.expect_request();
    assert!(matches!(err, GetObjectError::Unhandled(_)));
    assert_eq!(Some("correct-request-id"), err.request_id());
    assert_eq!(Some("correct-request-id"), err.meta().request_id());
    assert_eq!(
        Some("correct-extended-request-id"),
        err.extended_request_id()
    );
    assert_eq!(
        Some("correct-extended-request-id"),
        err.meta().extended_request_id()
    );
}

#[tokio::test]
async fn get_request_id_from_successful_nonstreaming_response() {
    let (http_client, request) = capture_request(Some(
        http_1x::Response::builder()
            .header("x-amz-request-id", "correct-request-id")
            .header("x-amz-id-2", "correct-extended-request-id")
            .status(200)
            .body(SdkBody::from(
                r#"<?xml version="1.0" encoding="UTF-8"?>
                <ListAllMyBucketsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
                  <Owner><ID>some-id</ID><DisplayName>some-display-name</DisplayName></Owner>
                  <Buckets></Buckets>
                </ListAllMyBucketsResult>"#,
            ))
            .unwrap(),
    ));
    let config = Config::builder()
        .http_client(http_client)
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .build();
    let client = Client::from_conf(config);
    let output = client
        .list_buckets()
        .send()
        .await
        .expect("valid successful response");
    request.expect_request();
    assert_eq!(Some("correct-request-id"), output.request_id());
    assert_eq!(
        Some("correct-extended-request-id"),
        output.extended_request_id()
    );
}

#[tokio::test]
async fn get_request_id_from_successful_streaming_response() {
    let (http_client, request) = capture_request(Some(
        http_1x::Response::builder()
            .header("x-amz-request-id", "correct-request-id")
            .header("x-amz-id-2", "correct-extended-request-id")
            .status(200)
            .body(SdkBody::from("some streaming file data"))
            .unwrap(),
    ));
    let config = Config::builder()
        .http_client(http_client)
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .build();
    let client = Client::from_conf(config);
    let output = client
        .get_object()
        .key("dontcare")
        .bucket("dontcare")
        .send()
        .await
        .expect("valid successful response");
    request.expect_request();
    assert_eq!(Some("correct-request-id"), output.request_id());
    assert_eq!(
        Some("correct-extended-request-id"),
        output.extended_request_id()
    );
}

// Verify that the conversion from operation error to the top-level service error maintains the request ID
#[tokio::test]
async fn conversion_to_service_error_maintains_request_id() {
    let (http_client, request) = capture_request(Some(
        http_1x::Response::builder()
            .header("x-amz-request-id", "correct-request-id")
            .header("x-amz-id-2", "correct-extended-request-id")
            .status(404)
            .body(SdkBody::from(
                r#"<?xml version="1.0" encoding="UTF-8"?>
                <Error>
                  <Code>NoSuchKey</Code>
                  <Message>The resource you requested does not exist</Message>
                  <Resource>/mybucket/myfoto.jpg</Resource>
                  <RequestId>incorrect-request-id</RequestId>
                </Error>"#,
            ))
            .unwrap(),
    ));
    let config = Config::builder()
        .http_client(http_client)
        .credentials_provider(Credentials::for_tests())
        .region(Region::new("us-east-1"))
        .build();
    let client = Client::from_conf(config);
    let err = client
        .get_object()
        .bucket("dontcare")
        .key("dontcare")
        .send()
        .await
        .expect_err("status was 404, this is an error");
    request.expect_request();
    let service_error: aws_sdk_s3::Error = err.into();
    assert_eq!(Some("correct-request-id"), service_error.request_id());
    assert_eq!(
        Some("correct-extended-request-id"),
        service_error.extended_request_id()
    );
}
