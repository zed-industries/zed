/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(feature = "test-util")]

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region, StalledStreamProtectionConfig};
use aws_sdk_s3::operation::put_object::PutObjectOutput;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ChecksumMode;
use aws_sdk_s3::{operation::get_object::GetObjectOutput, types::ChecksumAlgorithm};
use aws_sdk_s3::{Client, Config};
use aws_smithy_http_client::test_util::{capture_request, ReplayEvent, StaticReplayClient};
use aws_smithy_mocks::RuleMode;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::http::{
    HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings, SharedHttpClient,
    SharedHttpConnector,
};
use aws_smithy_runtime_api::client::interceptors::context::BeforeTransmitInterceptorContextRef;
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::shared::IntoShared;
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::config_bag::ConfigBag;
use aws_smithy_types::retry::RetryConfig;
use http_1x::header::AUTHORIZATION;
use http_1x::{HeaderValue, Uri};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, UNIX_EPOCH};
use tracing_test::traced_test;

/// Test connection for the movies IT
/// headers are signed with actual creds, at some point we could replace them with verifiable test
/// credentials, but there are plenty of other tests that target signing
fn new_checksum_validated_response_test_connection(
    checksum_header_name: &'static str,
    checksum_header_value: &'static str,
) -> StaticReplayClient {
    StaticReplayClient::new(vec![ReplayEvent::new(
        http_1x::Request::builder()
            .header("x-amz-checksum-mode", "ENABLED")
            .header(
                "user-agent",
                "aws-sdk-rust/0.123.test os/windows/XPSP3 lang/rust/1.50.0",
            )
            .header("x-amz-date", "20090213T233130Z")
            .header(
                "x-amz-content-sha256",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            )
            .header(
                "x-amz-user-agent",
                "aws-sdk-rust/0.123.test api/test-service/0.123 os/windows/XPSP3 lang/rust/1.50.0",
            )
            .header("authorization", "not-relevant")
            .uri(Uri::from_static(
                "https://some-test-bucket.s3.us-east-1.amazonaws.com/test.txt?x-id=GetObject",
            ))
            .body(SdkBody::empty())
            .unwrap(),
        http_1x::Response::builder()
            .header("x-amz-request-id", "4B4NGF0EAWN0GE63")
            .header("content-length", "11")
            .header("etag", "\"3e25960a79dbc69b674cd4ec67a72c62\"")
            .header(checksum_header_name, checksum_header_value)
            .header("content-type", "application/octet-stream")
            .header("server", "AmazonS3")
            .header("content-encoding", "")
            .header("last-modified", "Tue, 21 Jun 2022 16:29:14 GMT")
            .header("date", "Tue, 21 Jun 2022 16:29:23 GMT")
            .header(
                "x-amz-id-2",
                "kPl+IVVZAwsN8ePUyQJZ40WD9dzaqtr4eNESArqE68GSKtVvuvCTDe+SxhTT+JTUqXB1HL4OxNM=",
            )
            .header("accept-ranges", "bytes")
            .status(http_1x::StatusCode::from_u16(200).unwrap())
            .body(SdkBody::from(r#"Hello world"#))
            .unwrap(),
    )])
}

async fn test_checksum_on_streaming_response(
    checksum_header_name: &'static str,
    checksum_header_value: &'static str,
) -> GetObjectOutput {
    let http_client = new_checksum_validated_response_test_connection(
        checksum_header_name,
        checksum_header_value,
    );
    let config = Config::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .time_source(UNIX_EPOCH + Duration::from_secs(1624036048))
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .with_test_defaults()
        .build();
    let client = Client::from_conf(config);

    let res = client
        .get_object()
        .bucket("some-test-bucket")
        .key("test.txt")
        .checksum_mode(aws_sdk_s3::types::ChecksumMode::Enabled)
        .send()
        .await
        .unwrap();

    http_client.assert_requests_match(&[
        "x-amz-checksum-mode",
        "x-amz-user-agent",
        AUTHORIZATION.as_str(),
    ]);

    res
}

#[tokio::test]
async fn test_crc32_checksum_on_streaming_response() {
    let res = test_checksum_on_streaming_response("x-amz-checksum-crc32", "i9aeUg==").await;

    // Header checksums are base64 encoded
    assert_eq!(res.checksum_crc32(), Some("i9aeUg=="));
    let body = collect_body_into_string(res.body.into_inner()).await;

    assert_eq!(body, "Hello world");
}

#[tokio::test]
async fn test_crc32c_checksum_on_streaming_response() {
    let res = test_checksum_on_streaming_response("x-amz-checksum-crc32c", "crUfeA==").await;

    // Header checksums are base64 encoded
    assert_eq!(res.checksum_crc32_c(), Some("crUfeA=="));
    let body = collect_body_into_string(res.body.into_inner()).await;

    assert_eq!(body, "Hello world");
}

#[tokio::test]
async fn test_sha1_checksum_on_streaming_response() {
    let res =
        test_checksum_on_streaming_response("x-amz-checksum-sha1", "e1AsOh9IyGCa4hLN+2Od7jlnP14=")
            .await;

    // Header checksums are base64 encoded
    assert_eq!(res.checksum_sha1(), Some("e1AsOh9IyGCa4hLN+2Od7jlnP14="));
    let body = collect_body_into_string(res.body.into_inner()).await;

    assert_eq!(body, "Hello world");
}

#[tokio::test]
async fn test_sha256_checksum_on_streaming_response() {
    let res = test_checksum_on_streaming_response(
        "x-amz-checksum-sha256",
        "ZOyIygCyaOW6GjVnihtTFtIS9PNmskdyMlNKiuyjfzw=",
    )
    .await;

    // Header checksums are base64 encoded
    assert_eq!(
        res.checksum_sha256(),
        Some("ZOyIygCyaOW6GjVnihtTFtIS9PNmskdyMlNKiuyjfzw=")
    );
    let body = collect_body_into_string(res.body.into_inner()).await;

    assert_eq!(body, "Hello world");
}

// The test structure is identical for all supported checksum algorithms
async fn test_checksum_on_streaming_request<'a>(
    body: &'static [u8],
    checksum_algorithm: ChecksumAlgorithm,
    checksum_header_name: &'static str,
    expected_decoded_content_length: &'a str,
    expected_encoded_content_length: &'a str,
    expected_aws_chunked_encoded_body: &'a str,
) {
    let (http_client, rcvr) = capture_request(None);
    let config = Config::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .with_test_defaults()
        .build();
    let client = Client::from_conf(config);

    // ByteStreams created from a file are streaming and have a known size
    let mut file = tempfile::NamedTempFile::new().unwrap();
    use std::io::Write;
    file.write_all(body).unwrap();

    let body = aws_sdk_s3::primitives::ByteStream::read_from()
        .path(file.path())
        .buffer_size(1024)
        .build()
        .await
        .unwrap();

    // The response from the fake connection won't return the expected XML but we don't care about
    // that error in this test
    let _ = client
        .put_object()
        .bucket("test-bucket")
        .key("test.txt")
        .content_encoding("custom")
        .body(body)
        .checksum_algorithm(checksum_algorithm)
        .send()
        .await
        .unwrap();

    let mut req = rcvr.expect_request();

    let headers = req.headers();
    let x_amz_content_sha256 = headers
        .get("x-amz-content-sha256")
        .expect("x-amz-content-sha256 header exists");
    let x_amz_trailer = headers
        .get("x-amz-trailer")
        .expect("x-amz-trailer header exists");
    let x_amz_decoded_content_length = headers
        .get("x-amz-decoded-content-length")
        .expect("x-amz-decoded-content-length header exists");
    let content_length = headers
        .get("Content-Length")
        .expect("Content-Length header exists");
    let content_encoding = headers.get_all("Content-Encoding").collect::<Vec<_>>();

    assert_eq!(
        HeaderValue::from_static("STREAMING-UNSIGNED-PAYLOAD-TRAILER"),
        x_amz_content_sha256,
        "signing header is incorrect"
    );
    assert_eq!(
        HeaderValue::from_static(checksum_header_name),
        x_amz_trailer,
        "x-amz-trailer is incorrect"
    );
    // The position for `aws-chunked` in `content_encoding` doesn't matter for the target service.
    // The expected here just reflects the current behavior of appending `aws-chunked` to the header.
    assert_eq!(vec!["custom", "aws-chunked"], content_encoding);

    // The length of the string "Hello world"
    assert_eq!(
        HeaderValue::from_str(expected_decoded_content_length).unwrap(),
        x_amz_decoded_content_length,
        "decoded content length was wrong"
    );
    // The sum of the length of the original body, chunk markers, and trailers
    assert_eq!(
        HeaderValue::from_str(expected_encoded_content_length).unwrap(),
        content_length,
        "content-length was expected to be {} but was {} instead",
        expected_encoded_content_length,
        content_length
    );

    let body = collect_body_into_string(req.take_body()).await;
    // When sending a streaming body with a checksum, the trailers are included as part of the body content
    assert_eq!(body.as_str(), expected_aws_chunked_encoded_body,);
}

#[tokio::test]
async fn test_crc32_checksum_on_streaming_request() {
    let expected_aws_chunked_encoded_body =
        "B\r\nHello world\r\n0\r\nx-amz-checksum-crc32:i9aeUg==\r\n\r\n";
    let expected_encoded_content_length = format!("{}", expected_aws_chunked_encoded_body.len());
    test_checksum_on_streaming_request(
        b"Hello world",
        ChecksumAlgorithm::Crc32,
        "x-amz-checksum-crc32",
        "11",
        &expected_encoded_content_length,
        expected_aws_chunked_encoded_body,
    )
    .await
}

// This test isn't a duplicate. It tests CRC32C (note the C) checksum request validation
#[tokio::test]
async fn test_crc32c_checksum_on_streaming_request() {
    let expected_aws_chunked_encoded_body =
        "B\r\nHello world\r\n0\r\nx-amz-checksum-crc32c:crUfeA==\r\n\r\n";
    let expected_encoded_content_length = format!("{}", expected_aws_chunked_encoded_body.len());
    test_checksum_on_streaming_request(
        b"Hello world",
        ChecksumAlgorithm::Crc32C,
        "x-amz-checksum-crc32c",
        "11",
        &expected_encoded_content_length,
        expected_aws_chunked_encoded_body,
    )
    .await
}

#[tokio::test]
async fn test_sha1_checksum_on_streaming_request() {
    let expected_aws_chunked_encoded_body =
        "B\r\nHello world\r\n0\r\nx-amz-checksum-sha1:e1AsOh9IyGCa4hLN+2Od7jlnP14=\r\n\r\n";
    let expected_encoded_content_length = format!("{}", expected_aws_chunked_encoded_body.len());
    test_checksum_on_streaming_request(
        b"Hello world",
        ChecksumAlgorithm::Sha1,
        "x-amz-checksum-sha1",
        "11",
        &expected_encoded_content_length,
        expected_aws_chunked_encoded_body,
    )
    .await
}

#[tokio::test]
async fn test_sha256_checksum_on_streaming_request() {
    let expected_aws_chunked_encoded_body = "B\r\nHello world\r\n0\r\nx-amz-checksum-sha256:ZOyIygCyaOW6GjVnihtTFtIS9PNmskdyMlNKiuyjfzw=\r\n\r\n";
    let expected_encoded_content_length = format!("{}", expected_aws_chunked_encoded_body.len());
    test_checksum_on_streaming_request(
        b"Hello world",
        ChecksumAlgorithm::Sha256,
        "x-amz-checksum-sha256",
        "11",
        &expected_encoded_content_length,
        expected_aws_chunked_encoded_body,
    )
    .await
}

async fn collect_body_into_string(body: aws_smithy_types::body::SdkBody) -> String {
    use bytes::Buf;
    use bytes_utils::SegmentedBuf;
    use std::io::Read;

    let mut stream = ByteStream::new(body);
    let mut output = SegmentedBuf::new();
    while let Some(buf) = stream.next().await {
        output.push(buf.unwrap());
    }

    let mut output_text = String::new();
    output
        .reader()
        .read_to_string(&mut output_text)
        .expect("Doesn't cause IO errors");

    output_text
}

#[tokio::test]
#[traced_test]
async fn test_get_multipart_upload_part_checksum_validation() {
    let expected_checksum = "cpjwid==-12";
    let (http_client, rcvr) = capture_request(Some(
        http_1x::Response::builder()
            .header("etag", "\"3e25960a79dbc69b674cd4ec67a72c62\"")
            .header("x-amz-checksum-crc32", expected_checksum)
            .body(SdkBody::empty())
            .unwrap(),
    ));
    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .build();
    let client = Client::new(&sdk_config);

    // The response from the fake connection won't return the expected XML but we don't care about
    // that error in this test
    let res = client
        .get_object()
        .bucket("test-bucket")
        .key("test.txt")
        .checksum_mode(ChecksumMode::Enabled)
        .send()
        .await
        .expect("request should succeed, despite the non-base64-decodable checksum");

    let _req = rcvr.expect_request();

    let actual_checksum = res.checksum_crc32().unwrap();
    assert_eq!(expected_checksum, actual_checksum);

    logs_assert(|lines: &[&str]| {
        let checksum_warning = lines.iter().find(|&&line| {
            line.contains("This checksum is a part-level checksum which can't be validated by the Rust SDK. Disable checksum validation for this request to fix this warning.")
        });

        match checksum_warning {
            Some(_) => Ok(()),
            None => Err("Checksum warning was not issued".to_string()),
        }
    });
}

#[tokio::test]
#[traced_test]
async fn test_response_checksum_ignores_invalid_base64() {
    let expected_checksum = "{}{!!#{})!{)@$(}";
    let (http_client, rcvr) = capture_request(Some(
        http_1x::Response::builder()
            .header("etag", "\"3e25960a79dbc69b674cd4ec67a72c62\"")
            .header("x-amz-checksum-crc32", expected_checksum)
            .body(SdkBody::empty())
            .unwrap(),
    ));
    let sdk_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .region(Region::new("us-east-1"))
        .http_client(http_client.clone())
        .build();
    let client = Client::new(&sdk_config);

    // The response from the fake connection won't return the expected XML but we don't care about
    // that error in this test
    let res = client
        .get_object()
        .bucket("test-bucket")
        .key("test.txt")
        .checksum_mode(ChecksumMode::Enabled)
        .send()
        .await
        .expect("request should succeed, despite the non-base64-decodable checksum");

    let _req = rcvr.expect_request();

    let actual_checksum = res.checksum_crc32().unwrap();
    assert_eq!(expected_checksum, actual_checksum);

    logs_assert(|lines: &[&str]| {
        let checksum_warning = lines.iter().find(|&&line| {
            line.contains("Checksum received from server could not be base64 decoded. No checksum validation will be performed.")
        });

        match checksum_warning {
            Some(_) => Ok(()),
            None => Err("Checksum error was not issued".to_string()),
        }
    });
}

#[derive(Debug, Clone)]
struct CaptureHttpClient {
    inner: SharedHttpClient,
    captured_requests: Arc<Mutex<Vec<CapturedRequest>>>,
}

impl CaptureHttpClient {
    fn new() -> Self {
        Self {
            inner: aws_smithy_mocks::create_mock_http_client(),
            captured_requests: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn take_captured_requests(&self) -> Vec<CapturedRequest> {
        let mut captured = self.captured_requests.lock().unwrap();
        std::mem::take(&mut *captured)
    }

    fn attempt(&self) -> usize {
        self.captured_requests.lock().unwrap().len() + 1
    }
}

#[derive(Debug)]
struct CaptureConnector {
    inner: SharedHttpConnector,
    captured_requests: Arc<Mutex<Vec<CapturedRequest>>>,
}

#[derive(Debug)]
struct CapturedRequest {
    headers: aws_smithy_runtime_api::http::Headers,
    body: Result<http_body_util::Collected<bytes::Bytes>, BoxError>,
}

impl CapturedRequest {
    fn headers(&self) -> &aws_smithy_runtime_api::http::Headers {
        &self.headers
    }
}

impl HttpConnector for CaptureConnector {
    fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
        let captured_requests = self.captured_requests.clone();
        let inner = self.inner.clone();
        HttpConnectorFuture::new(async move {
            let mut request = request;
            // the body isn't read by the inner connector so it's safe to take it here
            // we need to read it as if we were the actual server here and consume it to be able to
            // test retry behavior of streaming bodies which must be read for checksums to be triggered
            let body = request.take_body();

            let output = http_body_util::BodyExt::collect(body)
                .await
                .map_err(|e| BoxError::from(e));
            let captured = CapturedRequest {
                headers: request.headers().clone(),
                body: output,
            };

            {
                let mut captured_requests = captured_requests.lock().unwrap();
                captured_requests.push(captured);
            }

            inner.call(request).await
        })
    }
}

impl HttpClient for CaptureHttpClient {
    fn http_connector(
        &self,
        settings: &HttpConnectorSettings,
        components: &RuntimeComponents,
    ) -> SharedHttpConnector {
        let inner = self.inner.http_connector(settings, components);
        let connector = CaptureConnector {
            inner,
            captured_requests: self.captured_requests.clone(),
        };
        connector.into_shared()
    }
}

#[tokio::test]
#[traced_test]
async fn test_checksum_reuse_on_retry() {
    let retry_rule = aws_smithy_mocks::mock!(aws_sdk_s3::Client::put_object)
        .sequence()
        .http_status(503, None)
        .output(|| PutObjectOutput::builder().build())
        .build();

    let http_client = CaptureHttpClient::new();
    let client = aws_smithy_mocks::mock_client!(
        aws_sdk_s3,
        RuleMode::Sequential,
        &[retry_rule],
        |client_builder| {
            client_builder
                .http_client(http_client.clone())
                .retry_config(RetryConfig::standard().with_max_attempts(3))
        }
    );

    let http_client_clone = http_client.clone();
    let change_body = SdkBody::retryable(move || {
        let current_attempt = http_client_clone.attempt();
        tracing::info!("test body current attempt: {}", current_attempt);
        match current_attempt {
            1 => SdkBody::from("initial content"),
            _ => SdkBody::from("retry content"),
        }
    });

    let body = ByteStream::new(change_body);

    let _res = client
        .put_object()
        .body(body)
        .checksum_algorithm(ChecksumAlgorithm::Sha256)
        .bucket("test-bucket")
        .key("test.txt")
        .send()
        .await
        .expect("request should succeed, despite the non-base64-decodable checksum");

    let requests = http_client.take_captured_requests();
    assert_eq!(2, requests.len());
    let first_checksum = requests[0]
        .headers()
        .get("x-amz-checksum-sha256")
        .expect("x-amz-checksum-sha256 header exists");

    let second_checksum = requests[1]
        .headers()
        .get("x-amz-checksum-sha256")
        .expect("x-amz-checksum-sha256 header exists");

    let initial_content_checksum = "kWo/C8OkKOGhaPRAjfgs1bu4sIxtesVe/53/KYJRNN8=";
    assert_eq!(initial_content_checksum, first_checksum);
    assert_eq!(initial_content_checksum, second_checksum);
}

/// Swaps the file content on subsequent attempts
#[derive(Debug)]
struct ChangeBodyInterceptor {
    http_client: CaptureHttpClient,
    content_path: PathBuf,
    retry_content: &'static str,
}

impl Intercept for ChangeBodyInterceptor {
    fn name(&self) -> &'static str {
        "ChangeBodyInterceptor"
    }

    fn read_before_attempt(
        &self,
        _context: &BeforeTransmitInterceptorContextRef<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        if self.http_client.attempt() > 1 {
            std::fs::write(self.content_path.as_path(), self.retry_content.as_bytes())
                .expect("replace contents successful");
        }
        Ok(())
    }
}

async fn run_checksum_reuse_streaming_request_test(
    initial_content: &'static str,
    retry_content: &'static str,
    expected_checksum: &'static str,
) {
    let retry_rule = aws_smithy_mocks::mock!(aws_sdk_s3::Client::put_object)
        .sequence()
        .http_status(503, None)
        .output(|| PutObjectOutput::builder().build())
        .build();

    let mut file = tempfile::NamedTempFile::new().unwrap();
    let content_path = file.path().to_path_buf();
    use std::io::Write;
    file.write_all(initial_content.as_bytes()).unwrap();

    let http_client = CaptureHttpClient::new();
    let client = aws_smithy_mocks::mock_client!(
        aws_sdk_s3,
        RuleMode::Sequential,
        &[retry_rule],
        |client_builder| {
            let change_body_interceptor = ChangeBodyInterceptor {
                http_client: http_client.clone(),
                content_path: content_path.clone(),
                retry_content,
            };
            client_builder
                .http_client(http_client.clone())
                .retry_config(RetryConfig::standard().with_max_attempts(3))
                .interceptor(change_body_interceptor)
        }
    );

    let body = aws_sdk_s3::primitives::ByteStream::read_from()
        .path(file.path())
        .buffer_size(1024)
        .build()
        .await
        .unwrap();

    let _res = client
        .put_object()
        .body(body)
        .checksum_algorithm(ChecksumAlgorithm::Sha256)
        .bucket("test-bucket")
        .key("test.txt")
        .send()
        .await
        .expect("request should succeed, despite the non-base64-decodable checksum");

    let mut requests = http_client.take_captured_requests();
    assert_eq!(2, requests.len());

    assert_streaming_request_checksum(
        requests.remove(0),
        "x-amz-checksum-sha256",
        expected_checksum,
    )
    .await;
    assert_streaming_request_checksum(
        requests.remove(0),
        "x-amz-checksum-sha256",
        expected_checksum,
    )
    .await;
}

async fn assert_streaming_request_checksum(
    request: CapturedRequest,
    checksum_header_name: &'static str,
    expected_checksum: &'static str,
) {
    let headers = request.headers();
    let x_amz_content_sha256 = headers
        .get("x-amz-content-sha256")
        .expect("x-amz-content-sha256 header exists");
    let x_amz_trailer = headers
        .get("x-amz-trailer")
        .expect("x-amz-trailer header exists");
    let content_encoding = headers.get_all("Content-Encoding").collect::<Vec<_>>();

    assert_eq!(
        HeaderValue::from_static("STREAMING-UNSIGNED-PAYLOAD-TRAILER"),
        x_amz_content_sha256,
        "signing header is incorrect"
    );
    assert_eq!(
        HeaderValue::from_static(checksum_header_name),
        x_amz_trailer,
        "x-amz-trailer is incorrect"
    );

    assert!(content_encoding.contains(&"aws-chunked"));
    // let body = collect_body_into_string(request.take_body()).await;
    let body = request.body.expect("body collected").to_bytes();
    let body_str = bytes_utils::Str::try_from(body)
        .expect("body is utf-8")
        .to_string();
    let actual_checksum =
        extract_checksum_value(&body_str, checksum_header_name).expect("trailing checksum exists");
    assert_eq!(actual_checksum, expected_checksum);
}

fn extract_checksum_value<'a, 'b>(input: &'a str, checksum_name: &'b str) -> Option<&'a str> {
    input
        .find(&format!("{}:", checksum_name))
        .and_then(|start| {
            let value_start = start + checksum_name.len() + 1;
            input[value_start..]
                .find("\r\n\r\n")
                .map(|end| &input[value_start..value_start + end])
        })
}

/// Test checksum is re-used for aws-chunked/streaming content when the content changes
/// between retries but the content length differs
///
/// NOTE: Because we set the overall body size hint only once when the stream is constructed
///       we end up throwing an error :696
#[tokio::test]
#[traced_test]
#[should_panic(expected = "StreamLengthMismatch { actual: 13, expected: 15 }")]
async fn test_checksum_reuse_on_retry_streaming_content_len_differs() {
    run_checksum_reuse_streaming_request_test(
        "initial content",
        "retry content",
        "kWo/C8OkKOGhaPRAjfgs1bu4sIxtesVe/53/KYJRNN8=",
    )
    .await;
}

/// Test checksum is re-used for aws-chunked/streaming content when the content changes
/// between retries but the content length is the same
#[tokio::test]
#[traced_test]
async fn test_checksum_reuse_on_retry_streaming_content_len_same() {
    run_checksum_reuse_streaming_request_test(
        "initial content",
        "in1t1al content",
        "kWo/C8OkKOGhaPRAjfgs1bu4sIxtesVe/53/KYJRNN8=",
    )
    .await;

    logs_assert(|lines: &[&str]| {
        let checksum_warning = lines.iter().find(|&&line| {
            line.contains( r#"calculated checksum differs from cached checksum! cached={"x-amz-checksum-sha256": "kWo/C8OkKOGhaPRAjfgs1bu4sIxtesVe/53/KYJRNN8="} calculated={"x-amz-checksum-sha256": "pPv/1lYp3XTWCZXJWT1heRy9+ZQyPn99ZqMQn1MK3Bw="}"#)
        });

        match checksum_warning {
            Some(_) => Ok(()),
            None => Err("Checksum mismatch warning was not issued".to_string()),
        }
    });
}
