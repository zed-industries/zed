/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Http Request Types

use crate::http::extensions::Extensions;
use crate::http::Headers;
use crate::http::HttpError;
use aws_smithy_types::body::SdkBody;
use std::borrow::Cow;

/// Parts struct useful for structural decomposition that the [`Request`] type can be converted into.
#[non_exhaustive]
pub struct RequestParts<B = SdkBody> {
    /// Request URI.
    pub uri: Uri,
    /// Request headers.
    pub headers: Headers,
    /// Request body.
    pub body: B,
}

#[derive(Debug)]
/// An HTTP Request Type
pub struct Request<B = SdkBody> {
    body: B,
    uri: Uri,
    method: http_02x::Method,
    extensions: Extensions,
    headers: Headers,
}

/// A Request URI
#[derive(Debug, Clone)]
pub struct Uri {
    as_string: String,
    parsed: ParsedUri,
}

#[derive(Debug, Clone)]
enum ParsedUri {
    H0(http_02x::Uri),
    H1(http_1x::Uri),
}

impl ParsedUri {
    fn path_and_query(&self) -> &str {
        match &self {
            ParsedUri::H0(u) => u.path_and_query().map(|pq| pq.as_str()).unwrap_or(""),
            ParsedUri::H1(u) => u.path_and_query().map(|pq| pq.as_str()).unwrap_or(""),
        }
    }

    fn path(&self) -> &str {
        match &self {
            ParsedUri::H0(u) => u.path(),
            ParsedUri::H1(u) => u.path(),
        }
    }

    fn query(&self) -> Option<&str> {
        match &self {
            ParsedUri::H0(u) => u.query(),
            ParsedUri::H1(u) => u.query(),
        }
    }
}

impl Uri {
    /// Sets `endpoint` as the endpoint for a URL.
    ///
    /// An `endpoint` MUST contain a scheme and authority.
    /// An `endpoint` MAY contain a port and path.
    ///
    /// An `endpoint` MUST NOT contain a query
    pub fn set_endpoint(&mut self, endpoint: &str) -> Result<(), HttpError> {
        let endpoint: http_02x::Uri = endpoint.parse().map_err(HttpError::invalid_uri)?;
        let endpoint = endpoint.into_parts();
        let authority = endpoint
            .authority
            .ok_or_else(HttpError::missing_authority)?;
        let scheme = endpoint.scheme.ok_or_else(HttpError::missing_scheme)?;
        let new_uri = http_02x::Uri::builder()
            .authority(authority)
            .scheme(scheme)
            .path_and_query(merge_paths(endpoint.path_and_query, &self.parsed).as_ref())
            .build()
            .map_err(HttpError::invalid_uri_parts)?;
        self.as_string = new_uri.to_string();
        self.parsed = ParsedUri::H0(new_uri);
        Ok(())
    }

    /// Returns the URI path.
    pub fn path(&self) -> &str {
        self.parsed.path()
    }

    /// Returns the URI query string.
    pub fn query(&self) -> Option<&str> {
        self.parsed.query()
    }

    fn from_http0x_uri(uri: http_02x::Uri) -> Self {
        Self {
            as_string: uri.to_string(),
            parsed: ParsedUri::H0(uri),
        }
    }

    #[allow(dead_code)]
    fn from_http1x_uri(uri: http_1x::Uri) -> Self {
        Self {
            as_string: uri.to_string(),
            parsed: ParsedUri::H1(uri),
        }
    }

    #[allow(dead_code)]
    fn into_h0(self) -> http_02x::Uri {
        match self.parsed {
            ParsedUri::H0(uri) => uri,
            ParsedUri::H1(_uri) => self.as_string.parse().unwrap(),
        }
    }
}

fn merge_paths(
    endpoint_path: Option<http_02x::uri::PathAndQuery>,
    uri: &ParsedUri,
) -> Cow<'_, str> {
    let uri_path_and_query = uri.path_and_query();
    let endpoint_path = match endpoint_path {
        None => return Cow::Borrowed(uri_path_and_query),
        Some(path) => path,
    };
    if let Some(query) = endpoint_path.query() {
        tracing::warn!(query = %query, "query specified in endpoint will be ignored during endpoint resolution");
    }
    let endpoint_path = endpoint_path.path();
    if endpoint_path.is_empty() {
        Cow::Borrowed(uri_path_and_query)
    } else {
        let ep_no_slash = endpoint_path.strip_suffix('/').unwrap_or(endpoint_path);
        let uri_path_no_slash = uri_path_and_query
            .strip_prefix('/')
            .unwrap_or(uri_path_and_query);
        Cow::Owned(format!("{ep_no_slash}/{uri_path_no_slash}"))
    }
}

impl TryFrom<String> for Uri {
    type Error = HttpError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed = ParsedUri::H0(value.parse().map_err(HttpError::invalid_uri)?);
        Ok(Uri {
            as_string: value,
            parsed,
        })
    }
}

impl<'a> TryFrom<&'a str> for Uri {
    type Error = HttpError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

#[cfg(feature = "http-02x")]
impl From<http_02x::Uri> for Uri {
    fn from(value: http_02x::Uri) -> Self {
        Uri::from_http0x_uri(value)
    }
}

#[cfg(feature = "http-02x")]
impl<B> TryInto<http_02x::Request<B>> for Request<B> {
    type Error = HttpError;

    fn try_into(self) -> Result<http_02x::Request<B>, Self::Error> {
        self.try_into_http02x()
    }
}

#[cfg(feature = "http-1x")]
impl From<http_1x::Uri> for Uri {
    fn from(value: http_1x::Uri) -> Self {
        Uri::from_http1x_uri(value)
    }
}

#[cfg(feature = "http-1x")]
impl<B> TryInto<http_1x::Request<B>> for Request<B> {
    type Error = HttpError;

    fn try_into(self) -> Result<http_1x::Request<B>, Self::Error> {
        self.try_into_http1x()
    }
}

impl<B> Request<B> {
    /// Converts this request into an http 0.x request.
    ///
    /// Depending on the internal storage type, this operation may be free or it may have an internal
    /// cost.
    #[cfg(feature = "http-02x")]
    pub fn try_into_http02x(self) -> Result<http_02x::Request<B>, HttpError> {
        let mut req = http_02x::Request::builder()
            .uri(self.uri.into_h0())
            .method(self.method)
            .body(self.body)
            .expect("known valid");
        *req.headers_mut() = self.headers.http0_headermap();
        *req.extensions_mut() = self.extensions.try_into()?;
        Ok(req)
    }

    /// Converts this request into an http 1.x request.
    ///
    /// Depending on the internal storage type, this operation may be free or it may have an internal
    /// cost.
    #[cfg(feature = "http-1x")]
    pub fn try_into_http1x(self) -> Result<http_1x::Request<B>, HttpError> {
        let mut req = http_1x::Request::builder()
            .uri(self.uri.as_string)
            .method(self.method.as_str())
            .body(self.body)
            .expect("known valid");
        *req.headers_mut() = self.headers.http1_headermap();
        *req.extensions_mut() = self.extensions.try_into()?;
        Ok(req)
    }

    /// Update the body of this request to be a new body.
    pub fn map<U>(self, f: impl Fn(B) -> U) -> Request<U> {
        Request {
            body: f(self.body),
            uri: self.uri,
            method: self.method,
            extensions: self.extensions,
            headers: self.headers,
        }
    }

    /// Returns a GET request with no URI
    pub fn new(body: B) -> Self {
        Self {
            body,
            uri: Uri::from_http0x_uri(http_02x::Uri::from_static("/")),
            method: http_02x::Method::GET,
            extensions: Default::default(),
            headers: Default::default(),
        }
    }

    /// Convert this request into its parts.
    pub fn into_parts(self) -> RequestParts<B> {
        RequestParts {
            uri: self.uri,
            headers: self.headers,
            body: self.body,
        }
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

    /// Converts this request into the request body.
    pub fn into_body(self) -> B {
        self.body
    }

    /// Returns the method associated with this request
    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    /// Returns the URI associated with this request
    pub fn uri(&self) -> &str {
        &self.uri.as_string
    }

    /// Returns a mutable reference the the URI of this http::Request
    pub fn uri_mut(&mut self) -> &mut Uri {
        &mut self.uri
    }

    /// Sets the URI of this request
    pub fn set_uri<U>(&mut self, uri: U) -> Result<(), U::Error>
    where
        U: TryInto<Uri>,
    {
        let uri = uri.try_into()?;
        self.uri = uri;
        Ok(())
    }

    /// Adds an extension to the request extensions
    pub fn add_extension<T: Send + Sync + Clone + 'static>(&mut self, extension: T) {
        self.extensions.insert(extension.clone());
    }
}

impl Request<SdkBody> {
    /// Attempts to clone this request
    ///
    /// On clone, any extensions will be cleared.
    ///
    /// If the body is cloneable, this will clone the request. Otherwise `None` will be returned
    pub fn try_clone(&self) -> Option<Self> {
        let body = self.body().try_clone()?;
        Some(Self {
            body,
            uri: self.uri.clone(),
            method: self.method.clone(),
            extensions: Extensions::new(),
            headers: self.headers.clone(),
        })
    }

    /// Replaces this request's body with [`SdkBody::taken()`]
    pub fn take_body(&mut self) -> SdkBody {
        std::mem::replace(self.body_mut(), SdkBody::taken())
    }

    /// Create a GET request to `/` with an empty body
    pub fn empty() -> Self {
        Self::new(SdkBody::empty())
    }

    /// Creates a GET request to `uri` with an empty body
    pub fn get(uri: impl AsRef<str>) -> Result<Self, HttpError> {
        let mut req = Self::new(SdkBody::empty());
        req.set_uri(uri.as_ref())?;
        Ok(req)
    }
}

#[cfg(feature = "http-02x")]
impl<B> TryFrom<http_02x::Request<B>> for Request<B> {
    type Error = HttpError;

    fn try_from(value: http_02x::Request<B>) -> Result<Self, Self::Error> {
        let (parts, body) = value.into_parts();
        let headers = Headers::try_from(parts.headers)?;
        Ok(Self {
            body,
            uri: parts.uri.into(),
            method: parts.method,
            extensions: parts.extensions.into(),
            headers,
        })
    }
}

#[cfg(feature = "http-1x")]
impl<B> TryFrom<http_1x::Request<B>> for Request<B> {
    type Error = HttpError;

    fn try_from(value: http_1x::Request<B>) -> Result<Self, Self::Error> {
        let (parts, body) = value.into_parts();
        let headers = Headers::try_from(parts.headers)?;
        Ok(Self {
            body,
            uri: Uri::from_http1x_uri(parts.uri),
            method: http_02x::Method::from_bytes(parts.method.as_str().as_bytes()).expect("valid"),
            extensions: parts.extensions.into(),
            headers,
        })
    }
}

#[cfg(all(test, feature = "http-02x", feature = "http-1x"))]
mod test {
    use aws_smithy_types::body::SdkBody;
    use http_02x::header::{AUTHORIZATION, CONTENT_LENGTH};

    #[test]
    fn non_ascii_requests() {
        let request = http_02x::Request::builder()
            .header("k", "😹")
            .body(SdkBody::empty())
            .unwrap();
        let request: super::Request = request
            .try_into()
            .expect("failed to convert a non-string header");
        assert_eq!(request.headers().get("k"), Some("😹"))
    }

    #[test]
    fn request_can_be_created() {
        let req = http_02x::Request::builder()
            .uri("http://foo.com")
            .body(SdkBody::from("hello"))
            .unwrap();
        let mut req = super::Request::try_from(req).unwrap();
        req.headers_mut().insert("a", "b");
        assert_eq!(req.headers().get("a").unwrap(), "b");
        req.headers_mut().append("a", "c");
        assert_eq!(req.headers().get("a").unwrap(), "b");
        let http0 = req.try_into_http02x().unwrap();
        assert_eq!(http0.uri(), "http://foo.com");
    }

    #[test]
    fn uri_mutations() {
        let req = http_02x::Request::builder()
            .uri("http://foo.com")
            .body(SdkBody::from("hello"))
            .unwrap();
        let mut req = super::Request::try_from(req).unwrap();
        assert_eq!(req.uri(), "http://foo.com/");
        req.set_uri("http://bar.com").unwrap();
        assert_eq!(req.uri(), "http://bar.com");
        let http0 = req.try_into_http02x().unwrap();
        assert_eq!(http0.uri(), "http://bar.com");
    }

    #[test]
    #[should_panic]
    fn header_panics() {
        let req = http_02x::Request::builder()
            .uri("http://foo.com")
            .body(SdkBody::from("hello"))
            .unwrap();
        let mut req = super::Request::try_from(req).unwrap();
        let _ = req
            .headers_mut()
            .try_insert("a\nb", "a\nb")
            .expect_err("invalid header");
        let _ = req.headers_mut().insert("a\nb", "a\nb");
    }

    #[test]
    fn try_clone_clones_all_data() {
        let request = http_02x::Request::builder()
            .uri(http_02x::Uri::from_static("https://www.amazon.com"))
            .method("POST")
            .header(CONTENT_LENGTH, 456)
            .header(AUTHORIZATION, "Token: hello")
            .body(SdkBody::from("hello world!"))
            .expect("valid request");

        let request: super::Request = request.try_into().unwrap();
        let cloned = request.try_clone().expect("request is cloneable");

        assert_eq!("https://www.amazon.com/", cloned.uri());
        assert_eq!("POST", cloned.method());
        assert_eq!(2, cloned.headers().len());
        assert_eq!("Token: hello", cloned.headers().get(AUTHORIZATION).unwrap(),);
        assert_eq!("456", cloned.headers().get(CONTENT_LENGTH).unwrap());
        assert_eq!("hello world!".as_bytes(), cloned.body().bytes().unwrap());
    }

    #[test]
    fn valid_round_trips() {
        let request = || {
            http_02x::Request::builder()
                .uri(http_02x::Uri::from_static("https://www.amazon.com"))
                .method("POST")
                .header(CONTENT_LENGTH, 456)
                .header(AUTHORIZATION, "Token: hello")
                .header("multi", "v1")
                .header("multi", "v2")
                .body(SdkBody::from("hello world!"))
                .expect("valid request")
        };

        check_roundtrip(request);
    }

    macro_rules! req_eq {
        ($a: expr, $b: expr) => {{
            assert_eq!($a.uri(), $b.uri(), "status code mismatch");
            assert_eq!($a.headers(), $b.headers(), "header mismatch");
            assert_eq!($a.method(), $b.method(), "header mismatch");
            assert_eq!($a.body().bytes(), $b.body().bytes(), "data mismatch");
            assert_eq!(
                $a.extensions().len(),
                $b.extensions().len(),
                "extensions size mismatch"
            );
        }};
    }

    #[track_caller]
    fn check_roundtrip(req: impl Fn() -> http_02x::Request<SdkBody>) {
        let mut container = super::Request::try_from(req()).unwrap();
        container.add_extension(5_u32);
        let mut h1 = container
            .try_into_http1x()
            .expect("failed converting to http1x");
        assert_eq!(h1.extensions().get::<u32>(), Some(&5));
        h1.extensions_mut().remove::<u32>();

        let mut container = super::Request::try_from(h1).expect("failed converting from http1x");
        container.add_extension(5_u32);
        let mut h0 = container
            .try_into_http02x()
            .expect("failed converting back to http0x");
        assert_eq!(h0.extensions().get::<u32>(), Some(&5));
        h0.extensions_mut().remove::<u32>();
        req_eq!(h0, req());
    }
}
