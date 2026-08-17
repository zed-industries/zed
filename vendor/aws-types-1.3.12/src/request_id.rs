/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! AWS-specific request ID support

use aws_smithy_runtime_api::client::result::SdkError;
use aws_smithy_runtime_api::http::Headers;
use aws_smithy_runtime_api::http::Response;
use aws_smithy_types::error::metadata::{Builder as ErrorMetadataBuilder, ErrorMetadata};

/// Constant for the [`ErrorMetadata`] extra field that contains the request ID
const AWS_REQUEST_ID: &str = "aws_request_id";

/// Implementers add a function to return an AWS request ID
pub trait RequestId {
    /// Returns the request ID, or `None` if the service could not be reached.
    fn request_id(&self) -> Option<&str>;
}

impl<E> RequestId for SdkError<E, Response> {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::ResponseError(err) => err.raw().headers().request_id(),
            Self::ServiceError(err) => err.raw().headers().request_id(),
            _ => None,
        }
    }
}

impl RequestId for ErrorMetadata {
    fn request_id(&self) -> Option<&str> {
        self.extra(AWS_REQUEST_ID)
    }
}

impl<B> RequestId for Response<B> {
    fn request_id(&self) -> Option<&str> {
        self.headers().request_id()
    }
}

impl RequestId for Headers {
    fn request_id(&self) -> Option<&str> {
        self.get("x-amzn-requestid")
            .or(self.get("x-amz-request-id"))
    }
}

impl<O, E> RequestId for Result<O, E>
where
    O: RequestId,
    E: RequestId,
{
    fn request_id(&self) -> Option<&str> {
        match self {
            Ok(ok) => ok.request_id(),
            Err(err) => err.request_id(),
        }
    }
}

/// Applies a request ID to a generic error builder
pub fn apply_request_id(builder: ErrorMetadataBuilder, headers: &Headers) -> ErrorMetadataBuilder {
    if let Some(request_id) = headers.request_id() {
        builder.custom(AWS_REQUEST_ID, request_id)
    } else {
        builder
    }
}

#[cfg(test)]
mod tests {
    use crate::request_id::{apply_request_id, RequestId, AWS_REQUEST_ID};
    use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
    use aws_smithy_runtime_api::client::result::SdkError;
    use aws_smithy_runtime_api::http::Headers;
    use aws_smithy_types::body::SdkBody;
    use aws_smithy_types::error::ErrorMetadata;
    use http::{HeaderValue, Response};

    #[test]
    fn test_request_id_sdk_error() {
        let without_request_id =
            || HttpResponse::try_from(Response::builder().body(SdkBody::empty()).unwrap()).unwrap();
        let with_request_id = || {
            HttpResponse::try_from(
                Response::builder()
                    .header(
                        "x-amzn-requestid",
                        HeaderValue::from_static("some-request-id"),
                    )
                    .body(SdkBody::empty())
                    .unwrap(),
            )
            .unwrap()
        };
        assert_eq!(
            None,
            SdkError::<(), _>::response_error("test", without_request_id()).request_id()
        );
        assert_eq!(
            Some("some-request-id"),
            SdkError::<(), _>::response_error("test", with_request_id()).request_id()
        );
        assert_eq!(
            None,
            SdkError::service_error((), without_request_id()).request_id()
        );
        assert_eq!(
            Some("some-request-id"),
            SdkError::service_error((), with_request_id()).request_id()
        );
    }

    #[test]
    fn test_extract_request_id() {
        let mut headers = Headers::new();
        assert_eq!(None, headers.request_id());

        headers.append(
            "x-amzn-requestid",
            HeaderValue::from_static("some-request-id"),
        );
        assert_eq!(Some("some-request-id"), headers.request_id());

        headers.append(
            "x-amz-request-id",
            HeaderValue::from_static("other-request-id"),
        );
        assert_eq!(Some("some-request-id"), headers.request_id());

        headers.remove("x-amzn-requestid");
        assert_eq!(Some("other-request-id"), headers.request_id());
    }

    #[test]
    fn test_apply_request_id() {
        let mut headers = Headers::new();
        assert_eq!(
            ErrorMetadata::builder().build(),
            apply_request_id(ErrorMetadata::builder(), &headers).build(),
        );

        headers.append(
            "x-amzn-requestid",
            HeaderValue::from_static("some-request-id"),
        );
        assert_eq!(
            ErrorMetadata::builder()
                .custom(AWS_REQUEST_ID, "some-request-id")
                .build(),
            apply_request_id(ErrorMetadata::builder(), &headers).build(),
        );
    }

    #[test]
    fn test_error_metadata_request_id_impl() {
        let err = ErrorMetadata::builder()
            .custom(AWS_REQUEST_ID, "some-request-id")
            .build();
        assert_eq!(Some("some-request-id"), err.request_id());
    }
}
