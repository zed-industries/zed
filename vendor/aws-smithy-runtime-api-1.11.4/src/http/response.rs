/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Http Response Types

use crate::http::extensions::Extensions;
use crate::http::{Headers, HttpError};
use aws_smithy_types::body::SdkBody;
use std::fmt;

/// HTTP response status code
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct StatusCode(u16);

impl StatusCode {
    /// True if this is a successful response code (200, 201, etc)
    pub fn is_success(self) -> bool {
        (200..300).contains(&self.0)
    }

    /// True if this response code is a client error (4xx)
    pub fn is_client_error(self) -> bool {
        (400..500).contains(&self.0)
    }

    /// True if this response code is a server error (5xx)
    pub fn is_server_error(self) -> bool {
        (500..600).contains(&self.0)
    }

    /// Return the value of this status code as a `u16`.
    pub fn as_u16(self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = HttpError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (100..1000).contains(&value) {
            Ok(StatusCode(value))
        } else {
            Err(HttpError::invalid_status_code())
        }
    }
}

#[cfg(feature = "http-02x")]
impl From<http_02x::StatusCode> for StatusCode {
    fn from(value: http_02x::StatusCode) -> Self {
        Self(value.as_u16())
    }
}

#[cfg(feature = "http-02x")]
impl From<StatusCode> for http_02x::StatusCode {
    fn from(value: StatusCode) -> Self {
        Self::from_u16(value.0).unwrap()
    }
}

#[cfg(feature = "http-1x")]
impl From<http_1x::StatusCode> for StatusCode {
    fn from(value: http_1x::StatusCode) -> Self {
        Self(value.as_u16())
    }
}

#[cfg(feature = "http-1x")]
impl From<StatusCode> for http_1x::StatusCode {
    fn from(value: StatusCode) -> Self {
        Self::from_u16(value.0).unwrap()
    }
}

impl From<StatusCode> for u16 {
    fn from(value: StatusCode) -> Self {
        value.0
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// An HTTP Response Type
#[derive(Debug)]
pub struct Response<B = SdkBody> {
    status: StatusCode,
    headers: Headers,
    body: B,
    extensions: Extensions,
}

impl<B> Response<B> {
    /// Converts this response into an http 0.x response.
    ///
    /// Depending on the internal storage type, this operation may be free or it may have an internal
    /// cost.
    #[cfg(feature = "http-02x")]
    pub fn try_into_http02x(self) -> Result<http_02x::Response<B>, HttpError> {
        let mut res = http_02x::Response::builder()
            .status(
                http_02x::StatusCode::from_u16(self.status.into())
                    .expect("validated upon construction"),
            )
            .body(self.body)
            .expect("known valid");
        *res.headers_mut() = self.headers.http0_headermap();
        *res.extensions_mut() = self.extensions.try_into()?;
        Ok(res)
    }

    /// Converts this response into an http 1.x response.
    ///
    /// Depending on the internal storage type, this operation may be free or it may have an internal
    /// cost.
    #[cfg(feature = "http-1x")]
    pub fn try_into_http1x(self) -> Result<http_1x::Response<B>, HttpError> {
        let mut res = http_1x::Response::builder()
            .status(
                http_1x::StatusCode::from_u16(self.status.into())
                    .expect("validated upon construction"),
            )
            .body(self.body)
            .expect("known valid");
        *res.headers_mut() = self.headers.http1_headermap();
        *res.extensions_mut() = self.extensions.try_into()?;
        Ok(res)
    }

    /// Update the body of this response to be a new body.
    pub fn map<U>(self, f: impl Fn(B) -> U) -> Response<U> {
        Response {
            status: self.status,
            body: f(self.body),
            extensions: self.extensions,
            headers: self.headers,
        }
    }

    /// Returns a response with the given status and body
    pub fn new(status: StatusCode, body: B) -> Self {
        Self {
            status,
            body,
            extensions: Default::default(),
            headers: Default::default(),
        }
    }

    /// Returns the status code
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Returns a mutable reference to the status code
    pub fn status_mut(&mut self) -> &mut StatusCode {
        &mut self.status
    }

    /// Returns a reference to the header map
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Returns a mutable reference to the header map
    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    /// Returns the body associated with the request
    pub fn body(&self) -> &B {
        &self.body
    }

    /// Returns a mutable reference to the body
    pub fn body_mut(&mut self) -> &mut B {
        &mut self.body
    }

    /// Converts this response into the response body.
    pub fn into_body(self) -> B {
        self.body
    }

    /// Adds an extension to the response extensions
    pub fn add_extension<T: Send + Sync + Clone + 'static>(&mut self, extension: T) {
        self.extensions.insert(extension);
    }
}

impl Response<SdkBody> {
    /// Replaces this response's body with [`SdkBody::taken()`]
    pub fn take_body(&mut self) -> SdkBody {
        std::mem::replace(self.body_mut(), SdkBody::taken())
    }
}

#[cfg(feature = "http-02x")]
impl<B> TryFrom<http_02x::Response<B>> for Response<B> {
    type Error = HttpError;

    fn try_from(value: http_02x::Response<B>) -> Result<Self, Self::Error> {
        let (parts, body) = value.into_parts();
        let headers = Headers::try_from(parts.headers)?;
        Ok(Self {
            status: StatusCode::try_from(parts.status.as_u16()).expect("validated by http 0.x"),
            body,
            extensions: parts.extensions.into(),
            headers,
        })
    }
}

#[cfg(feature = "http-1x")]
impl<B> TryFrom<http_1x::Response<B>> for Response<B> {
    type Error = HttpError;

    fn try_from(value: http_1x::Response<B>) -> Result<Self, Self::Error> {
        let (parts, body) = value.into_parts();
        let headers = Headers::try_from(parts.headers)?;
        Ok(Self {
            status: StatusCode::try_from(parts.status.as_u16()).expect("validated by http 1.x"),
            body,
            extensions: parts.extensions.into(),
            headers,
        })
    }
}

#[cfg(all(test, feature = "http-02x", feature = "http-1x"))]
mod test {
    use super::*;
    use aws_smithy_types::body::SdkBody;

    #[test]
    fn non_ascii_responses() {
        let response = http_02x::Response::builder()
            .status(200)
            .header("k", "😹")
            .body(SdkBody::empty())
            .unwrap();
        let response: Response = response
            .try_into()
            .expect("failed to convert a non-string header");
        assert_eq!(response.headers().get("k"), Some("😹"))
    }

    #[test]
    fn response_can_be_created() {
        let req = http_02x::Response::builder()
            .status(200)
            .body(SdkBody::from("hello"))
            .unwrap();
        let mut rsp = super::Response::try_from(req).unwrap();
        rsp.headers_mut().insert("a", "b");
        assert_eq!("b", rsp.headers().get("a").unwrap());
        rsp.headers_mut().append("a", "c");
        assert_eq!("b", rsp.headers().get("a").unwrap());
        let http0 = rsp.try_into_http02x().unwrap();
        assert_eq!(200, http0.status().as_u16());
    }

    macro_rules! resp_eq {
        ($a: expr, $b: expr) => {{
            assert_eq!($a.status(), $b.status(), "status code mismatch");
            assert_eq!($a.headers(), $b.headers(), "header mismatch");
            assert_eq!($a.body().bytes(), $b.body().bytes(), "data mismatch");
            assert_eq!(
                $a.extensions().len(),
                $b.extensions().len(),
                "extensions size mismatch"
            );
        }};
    }

    #[track_caller]
    fn check_roundtrip(req: impl Fn() -> http_02x::Response<SdkBody>) {
        let mut container = super::Response::try_from(req()).unwrap();
        container.add_extension(5_u32);
        let mut h1 = container
            .try_into_http1x()
            .expect("failed converting to http_1x");
        assert_eq!(h1.extensions().get::<u32>(), Some(&5));
        h1.extensions_mut().remove::<u32>();

        let mut container = super::Response::try_from(h1).expect("failed converting from http1x");
        container.add_extension(5_u32);
        let mut h0 = container
            .try_into_http02x()
            .expect("failed converting back to http_02x");
        assert_eq!(h0.extensions().get::<u32>(), Some(&5));
        h0.extensions_mut().remove::<u32>();
        resp_eq!(h0, req());
    }

    #[test]
    fn valid_round_trips() {
        let response = || {
            http_02x::Response::builder()
                .status(200)
                .header("k", "v")
                .header("multi", "v1")
                .header("multi", "v2")
                .body(SdkBody::from("12345"))
                .unwrap()
        };
        check_roundtrip(response);
    }

    #[test]
    #[should_panic]
    fn header_panics() {
        let res = http_02x::Response::builder()
            .status(200)
            .body(SdkBody::from("hello"))
            .unwrap();
        let mut res = Response::try_from(res).unwrap();
        let _ = res
            .headers_mut()
            .try_insert("a\nb", "a\nb")
            .expect_err("invalid header");
        let _ = res.headers_mut().insert("a\nb", "a\nb");
    }

    #[test]
    fn cant_cross_convert_with_extensions_h0_h1() {
        let resp_h0 = || {
            http_02x::Response::builder()
                .status(200)
                .extension(5_u32)
                .body(SdkBody::from("hello"))
                .unwrap()
        };

        let _ = Response::try_from(resp_h0())
            .unwrap()
            .try_into_http1x()
            .expect_err("cant copy extension");

        let _ = Response::try_from(resp_h0())
            .unwrap()
            .try_into_http02x()
            .expect("allowed to cross-copy");
    }

    #[test]
    fn cant_cross_convert_with_extensions_h1_h0() {
        let resp_h1 = || {
            http_1x::Response::builder()
                .status(200)
                .extension(5_u32)
                .body(SdkBody::from("hello"))
                .unwrap()
        };

        let _ = Response::try_from(resp_h1())
            .unwrap()
            .try_into_http02x()
            .expect_err("cant copy extension");

        let _ = Response::try_from(resp_h1())
            .unwrap()
            .try_into_http1x()
            .expect("allowed to cross-copy");
    }
}
