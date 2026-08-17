/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sdk_s3 as s3;
use std::collections::HashMap;

use futures_util::future::FutureExt;
use futures_util::Future;
use http_1x::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http_1x::Uri;
use s3::config::{Credentials, Region};
use s3::operation::get_object::builders::GetObjectFluentBuilder;
use s3::operation::head_object::builders::HeadObjectFluentBuilder;
use s3::operation::put_object::builders::PutObjectFluentBuilder;
use s3::operation::upload_part::builders::UploadPartFluentBuilder;
use s3::presigning::{PresignedRequest, PresigningConfig};
use std::pin::Pin;
use std::time::{Duration, SystemTime};

trait TestOperation {
    fn presign_for_test(
        self,
        config: PresigningConfig,
    ) -> Pin<Box<dyn Future<Output = PresignedRequest>>>;
}

macro_rules! rig_operation {
    ($fluent_builder:ident) => {
        impl TestOperation for $fluent_builder {
            fn presign_for_test(
                self,
                config: PresigningConfig,
            ) -> Pin<Box<dyn Future<Output = PresignedRequest>>> {
                Box::pin($fluent_builder::presigned(self, config).map(|out| out.expect("success")))
            }
        }
    };
}

rig_operation!(GetObjectFluentBuilder);
rig_operation!(PutObjectFluentBuilder);
rig_operation!(UploadPartFluentBuilder);
rig_operation!(HeadObjectFluentBuilder);

/// Generates a `PresignedRequest` from the given input.
/// Assumes that that input has a `presigned` method on it.
async fn presign<O, F>(operation: O) -> PresignedRequest
where
    O: FnOnce(s3::Client) -> F,
    F: TestOperation,
{
    let creds = Credentials::for_tests_with_session_token();
    let config = s3::Config::builder()
        .credentials_provider(creds)
        .region(Region::new("us-east-1"))
        .build();
    let client = s3::Client::from_conf(config);

    operation(client)
        .presign_for_test(
            PresigningConfig::builder()
                .start_time(SystemTime::UNIX_EPOCH + Duration::from_secs(1234567891))
                .expires_in(Duration::from_secs(30))
                .build()
                .unwrap(),
        )
        .await
}

#[tokio::test]
async fn test_presigning() {
    let presigned =
        presign(|client| client.get_object().bucket("test-bucket").key("test-key")).await;
    let uri = presigned.uri().parse::<Uri>().unwrap();

    let pq = uri.path_and_query().unwrap();
    let path = pq.path();
    let query = pq.query().unwrap();
    let mut query_params: Vec<&str> = query.split('&').collect();
    query_params.sort();

    pretty_assertions::assert_eq!(
        "test-bucket.s3.us-east-1.amazonaws.com",
        uri.authority().unwrap()
    );
    assert_eq!("GET", presigned.method());
    assert_eq!("/test-key", path);
    pretty_assertions::assert_eq!(
        &[
            "X-Amz-Algorithm=AWS4-HMAC-SHA256",
            "X-Amz-Credential=ANOTREAL%2F20090213%2Fus-east-1%2Fs3%2Faws4_request",
            "X-Amz-Date=20090213T233131Z",
            "X-Amz-Expires=30",
            "X-Amz-Security-Token=notarealsessiontoken",
            "X-Amz-Signature=758353318739033a850182c7b3435076eebbbd095f8dcf311383a6a1e124c4cb",
            "X-Amz-SignedHeaders=host",
            "x-id=GetObject"
        ][..],
        &query_params
    );
    assert_eq!(presigned.headers().count(), 0);
    let headers = presigned.headers().collect::<HashMap<_, _>>();

    // Checksum headers should not be included by default in presigned requests
    assert_eq!(headers.get("x-amz-sdk-checksum-algorithm"), None);
    assert_eq!(headers.get("x-amz-checksum-crc32"), None);
    assert_eq!(headers.get("x-amz-checksum-mode"), None);
}

#[tokio::test]
async fn test_presigning_with_payload_headers() {
    let presigned = presign(|client| {
        client
            .put_object()
            .bucket("test-bucket")
            .key("test-key")
            .content_length(12345)
            .content_type("application/x-test")
    })
    .await;
    let uri = presigned.uri().parse::<Uri>().unwrap();

    let pq = uri.path_and_query().unwrap();
    let path = pq.path();
    let query = pq.query().unwrap();
    let mut query_params: Vec<&str> = query.split('&').collect();
    query_params.sort();

    pretty_assertions::assert_eq!(
        "test-bucket.s3.us-east-1.amazonaws.com",
        uri.authority().unwrap()
    );
    assert_eq!("PUT", presigned.method());
    assert_eq!("/test-key", path);
    pretty_assertions::assert_eq!(
        &[
            "X-Amz-Algorithm=AWS4-HMAC-SHA256",
            "X-Amz-Credential=ANOTREAL%2F20090213%2Fus-east-1%2Fs3%2Faws4_request",
            "X-Amz-Date=20090213T233131Z",
            "X-Amz-Expires=30",
            "X-Amz-Security-Token=notarealsessiontoken",
            "X-Amz-Signature=be1d41dc392f7019750e4f5e577234fb9059dd20d15f6a99734196becce55e52",
            "X-Amz-SignedHeaders=content-length%3Bcontent-type%3Bhost",
            "x-id=PutObject"
        ][..],
        &query_params
    );
    let headers = presigned.headers().collect::<HashMap<_, _>>();

    assert_eq!(
        headers.get(CONTENT_TYPE.as_str()),
        Some(&"application/x-test")
    );
    assert_eq!(headers.get(CONTENT_LENGTH.as_str()), Some(&"12345"));

    // Checksum headers should not be included by default in presigned requests
    assert_eq!(headers.get("x-amz-sdk-checksum-algorithm"), None);
    assert_eq!(headers.get("x-amz-checksum-crc32"), None);

    assert_eq!(headers.len(), 2);
}

#[tokio::test]
async fn test_presigned_upload_part() {
    let presigned = presign(|client| {
        client
            .upload_part()
            .content_length(12345)
            .bucket("bucket")
            .key("key")
            .part_number(0)
            .upload_id("upload-id")
    })
    .await;
    pretty_assertions::assert_eq!(
        "https://bucket.s3.us-east-1.amazonaws.com/key?x-id=UploadPart&partNumber=0&uploadId=upload-id&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ANOTREAL%2F20090213%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20090213T233131Z&X-Amz-Expires=30&X-Amz-SignedHeaders=content-length%3Bhost&X-Amz-Signature=a702867244f0bd1fb4d161e2a062520dcbefae3b9992d2e5366bcd61a60c6ddd&X-Amz-Security-Token=notarealsessiontoken",
        presigned.uri().to_string(),
    );
}

#[tokio::test]
async fn test_presigning_object_lambda() {
    let presigned = presign(|client| {
        client
            .get_object()
            .bucket("arn:aws:s3-object-lambda:us-west-2:123456789012:accesspoint:my-banner-ap-name")
            .key("test2.txt")
    })
    .await;
    // since the URI is `my-banner-api-name...` we know EP2 is working properly for presigning
    pretty_assertions::assert_eq!(
        "https://my-banner-ap-name-123456789012.s3-object-lambda.us-west-2.amazonaws.com/test2.txt?x-id=GetObject&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ANOTREAL%2F20090213%2Fus-west-2%2Fs3-object-lambda%2Faws4_request&X-Amz-Date=20090213T233131Z&X-Amz-Expires=30&X-Amz-SignedHeaders=host&X-Amz-Signature=027976453050b6f9cca7af80a59c05ee572b462e0fc1ef564c59412b903fcdf2&X-Amz-Security-Token=notarealsessiontoken",
        presigned.uri().to_string()
    );
}

#[tokio::test]
async fn test_presigned_head_object() {
    let presigned = presign(|client| client.head_object().bucket("bucket").key("key")).await;

    assert_eq!("HEAD", presigned.method());
    pretty_assertions::assert_eq!(
        "https://bucket.s3.us-east-1.amazonaws.com/key?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ANOTREAL%2F20090213%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20090213T233131Z&X-Amz-Expires=30&X-Amz-SignedHeaders=host&X-Amz-Signature=6b97012e70d5ee3528b5591e0e90c0f45e0fa303506f854eff50ff922751a193&X-Amz-Security-Token=notarealsessiontoken",
        presigned.uri().to_string(),
    );
}

#[tokio::test]
async fn test_presigned_user_provided_checksum() {
    let presigned = presign(|client| {
        client
            .put_object()
            .checksum_crc64_nvme("NotARealChecksum")
            .bucket("test-bucket")
            .key("test-key")
    })
    .await;

    // The x-amz-checksum-crc64nvme header is added to the signed headers
    pretty_assertions::assert_eq!(
        "https://test-bucket.s3.us-east-1.amazonaws.com/test-key?x-id=PutObject&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ANOTREAL%2F20090213%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20090213T233131Z&X-Amz-Expires=30&X-Amz-SignedHeaders=host%3Bx-amz-checksum-crc64nvme&X-Amz-Signature=40e6ea102769a53f440db587be0b6898893d9a0f8268d2f8d2315ca0abc42fee&X-Amz-Security-Token=notarealsessiontoken",
        presigned.uri().to_string(),
    );

    // Checksum value header is persisted into the request
    let headers = presigned.headers().collect::<HashMap<_, _>>();
    assert_eq!(
        headers.get("x-amz-checksum-crc64nvme"),
        Some(&"NotARealChecksum")
    );
}
