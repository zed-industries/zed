/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(feature = "test-util")]

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::Config;
use aws_sdk_s3::{config::Credentials, config::Region, types::ObjectAttributes, Client};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

const RESPONSE_BODY_XML: &[u8] = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<GetObjectAttributesResponse xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\"><Checksum><ChecksumSHA1>e1AsOh9IyGCa4hLN+2Od7jlnP14=</ChecksumSHA1></Checksum></GetObjectAttributesResponse>";

#[tokio::test]
async fn ignore_invalid_xml_body_root() {
    let http_client = StaticReplayClient::new(vec![
        ReplayEvent::new(http_1x::Request::builder()
             .header("x-amz-object-attributes", "Checksum")
             .header("x-amz-user-agent", "aws-sdk-rust/0.123.test api/test-service/0.123 os/windows/XPSP3 lang/rust/1.50.0")
             .header("x-amz-date", "20090213T233130Z")
             .header("authorization", "AWS4-HMAC-SHA256 Credential=ANOTREAL/20210618/us-east-1/s3/aws4_request, SignedHeaders=host;x-amz-content-sha256;x-amz-date;x-amz-object-attributes;x-amz-security-token;x-amz-user-agent, Signature=0e6ec749db5a0af07890a83f553319eda95be0e498d058c64880471a474c5378")
             .header("x-amz-content-sha256", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
             .uri(http_1x::Uri::from_static("https://some-test-bucket.s3.us-east-1.amazonaws.com/test.txt?attributes"))
             .body(SdkBody::empty())
             .unwrap(),
         http_1x::Response::builder()
             .header(
                 "x-amz-id-2",
                 "rbipIUyF3YKPIcqpz6hrP9x9mzYMSqkHzDEp6TEN/STcKvylDIE/LLN6x9t6EKJRrgctNsdNHWk=",
             )
             .header("x-amz-request-id", "K8036R3D4NZNMMVC")
             .header("date", "Tue, 23 Aug 2022 18:17:23 GMT")
             .header("last-modified", "Tue, 21 Jun 2022 16:30:01 GMT")
             .header("server", "AmazonS3")
             .header("content-length", "224")
             .status(200)
             .body(SdkBody::from(RESPONSE_BODY_XML))
             .unwrap())
    ]);

    let config = Config::builder()
        .credentials_provider(SharedCredentialsProvider::new(
            Credentials::for_tests_with_session_token(),
        ))
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .with_test_defaults()
        .build();
    let client = Client::from_conf(config);

    let _ = client
        .get_object_attributes()
        .bucket("some-test-bucket")
        .key("test.txt")
        .object_attributes(ObjectAttributes::Checksum)
        .send()
        .await
        .unwrap();

    http_client.relaxed_requests_match();
}
