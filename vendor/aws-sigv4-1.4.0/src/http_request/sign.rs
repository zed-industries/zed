/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use super::error::SigningError;
use super::{PayloadChecksumKind, SignatureLocation};
use crate::http_request::canonical_request::header;
use crate::http_request::canonical_request::param;
use crate::http_request::canonical_request::{CanonicalRequest, StringToSign};
use crate::http_request::error::CanonicalRequestError;
use crate::http_request::SigningParams;
use crate::sign::v4;
#[cfg(feature = "sigv4a")]
use crate::sign::v4a;
use crate::{SignatureVersion, SigningOutput};
use http::Uri;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::str;

const LOG_SIGNABLE_BODY: &str = "LOG_SIGNABLE_BODY";

/// Represents all of the information necessary to sign an HTTP request.
#[derive(Debug)]
#[non_exhaustive]
pub struct SignableRequest<'a> {
    method: &'a str,
    uri: Uri,
    headers: Vec<(&'a str, &'a str)>,
    body: SignableBody<'a>,
}

impl<'a> SignableRequest<'a> {
    /// Creates a new `SignableRequest`.
    ///
    /// NOTE: The `uri` is expected to already in encoded form.
    pub fn new(
        method: &'a str,
        uri: impl Into<Cow<'a, str>>,
        headers: impl Iterator<Item = (&'a str, &'a str)>,
        body: SignableBody<'a>,
    ) -> Result<Self, SigningError> {
        let uri = uri
            .into()
            .parse()
            .map_err(|e| SigningError::from(CanonicalRequestError::from(e)))?;
        let headers = headers.collect();
        Ok(Self {
            method,
            uri,
            headers,
            body,
        })
    }

    /// Returns the signable URI
    pub(crate) fn uri(&self) -> &Uri {
        &self.uri
    }

    /// Returns the signable HTTP method
    pub(crate) fn method(&self) -> &str {
        self.method
    }

    /// Returns the request headers
    pub(crate) fn headers(&self) -> &[(&str, &str)] {
        self.headers.as_slice()
    }

    /// Returns the signable body
    pub fn body(&self) -> &SignableBody<'_> {
        &self.body
    }
}

/// A signable HTTP request body
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SignableBody<'a> {
    /// A body composed of a slice of bytes
    Bytes(&'a [u8]),

    /// An unsigned payload
    ///
    /// UnsignedPayload is used for streaming requests where the contents of the body cannot be
    /// known prior to signing
    UnsignedPayload,

    /// A precomputed body checksum. The checksum should be a SHA256 checksum of the body,
    /// lowercase hex encoded. Eg:
    /// `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
    Precomputed(String),

    /// Set when an unsigned streaming body has checksum trailers.
    StreamingUnsignedPayloadTrailer,

    /// Set when a signed streaming body has checksum trailers.
    StreamingSignedPayloadTrailer,
}

/// Formats the value using the given formatter. To print the body data, set the environment variable `LOG_SIGNABLE_BODY=true`.
impl Debug for SignableBody<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let should_log_signable_body = std::env::var(LOG_SIGNABLE_BODY)
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or_default();
        match self {
            Self::Bytes(arg0) => {
                if should_log_signable_body {
                    f.debug_tuple("Bytes").field(arg0).finish()
                } else {
                    let redacted = format!("** REDACTED **. To print {body_size} bytes of raw data, set environment variable `LOG_SIGNABLE_BODY=true`", body_size = arg0.len());
                    f.debug_tuple("Bytes").field(&redacted).finish()
                }
            }
            Self::UnsignedPayload => write!(f, "UnsignedPayload"),
            Self::Precomputed(arg0) => f.debug_tuple("Precomputed").field(arg0).finish(),
            Self::StreamingUnsignedPayloadTrailer => {
                write!(f, "StreamingUnsignedPayloadTrailer")
            }
            Self::StreamingSignedPayloadTrailer => {
                write!(f, "StreamingSignedPayloadTrailer")
            }
        }
    }
}

impl SignableBody<'_> {
    /// Create a new empty signable body
    pub fn empty() -> SignableBody<'static> {
        SignableBody::Bytes(&[])
    }
}

/// Instructions for applying a signature to an HTTP request.
#[derive(Debug)]
pub struct SigningInstructions {
    headers: Vec<Header>,
    params: Vec<(&'static str, Cow<'static, str>)>,
}

/// Header representation for use in [`SigningInstructions`]
pub struct Header {
    key: &'static str,
    value: String,
    sensitive: bool,
}

impl Debug for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Header");
        fmt.field("key", &self.key);
        let value = if self.sensitive {
            "** REDACTED **"
        } else {
            &self.value
        };
        fmt.field("value", &value);
        fmt.finish()
    }
}

impl Header {
    /// The name of this header
    pub fn name(&self) -> &'static str {
        self.key
    }

    /// The value of this header
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Whether this header has a sensitive value
    pub fn sensitive(&self) -> bool {
        self.sensitive
    }
}

impl SigningInstructions {
    fn new(headers: Vec<Header>, params: Vec<(&'static str, Cow<'static, str>)>) -> Self {
        Self { headers, params }
    }

    /// Returns the headers and query params that should be applied to this request
    pub fn into_parts(self) -> (Vec<Header>, Vec<(&'static str, Cow<'static, str>)>) {
        (self.headers, self.params)
    }

    /// Returns a reference to the headers that should be added to the request.
    pub fn headers(&self) -> impl Iterator<Item = (&str, &str)> {
        self.headers
            .iter()
            .map(|header| (header.key, header.value.as_str()))
    }

    /// Returns a reference to the query parameters that should be added to the request.
    pub fn params(&self) -> &[(&str, Cow<'static, str>)] {
        self.params.as_slice()
    }

    #[cfg(any(feature = "http0-compat", test))]
    /// Applies the instructions to the given `request`.
    pub fn apply_to_request_http0x<B>(self, request: &mut http0::Request<B>) {
        let (new_headers, new_query) = self.into_parts();
        for header in new_headers.into_iter() {
            let mut value = http0::HeaderValue::from_str(&header.value).unwrap();
            value.set_sensitive(header.sensitive);
            request.headers_mut().insert(header.key, value);
        }

        if !new_query.is_empty() {
            let mut query = aws_smithy_http::query_writer::QueryWriter::new_from_string(
                &request.uri().to_string(),
            )
            .expect("unreachable: URI is valid");
            for (name, value) in new_query {
                query.insert(name, &value);
            }
            let query_uri = query.build_uri().to_string();
            let query_http0 = query_uri.parse::<http0::Uri>().expect("URI is valid");
            *request.uri_mut() = query_http0;
        }
    }

    #[cfg(any(feature = "http1", test))]
    /// Applies the instructions to the given `request`.
    pub fn apply_to_request_http1x<B>(self, request: &mut http::Request<B>) {
        // TODO(https://github.com/smithy-lang/smithy-rs/issues/3367): Update query writer to reduce
        // allocations
        let (new_headers, new_query) = self.into_parts();
        for header in new_headers.into_iter() {
            let mut value = http::HeaderValue::from_str(&header.value).unwrap();
            value.set_sensitive(header.sensitive);
            request.headers_mut().insert(header.key, value);
        }

        if !new_query.is_empty() {
            let mut query = aws_smithy_http::query_writer::QueryWriter::new_from_string(
                &request.uri().to_string(),
            )
            .expect("unreachable: URI is valid");
            for (name, value) in new_query {
                query.insert(name, &value);
            }
            *request.uri_mut() = query
                .build_uri()
                .to_string()
                .parse()
                .expect("unreachable: URI is valid");
        }
    }
}

/// Produces a signature for the given `request` and returns instructions
/// that can be used to apply that signature to an HTTP request.
pub fn sign<'a>(
    request: SignableRequest<'a>,
    params: &'a SigningParams<'a>,
) -> Result<SigningOutput<SigningInstructions>, SigningError> {
    tracing::trace!(request = ?request, params = ?params, "signing request");
    match params.settings().signature_location {
        SignatureLocation::Headers => {
            let (signing_headers, signature) =
                calculate_signing_headers(&request, params)?.into_parts();
            Ok(SigningOutput::new(
                SigningInstructions::new(signing_headers, vec![]),
                signature,
            ))
        }
        SignatureLocation::QueryParams => {
            let (params, signature) = calculate_signing_params(&request, params)?;
            Ok(SigningOutput::new(
                SigningInstructions::new(vec![], params),
                signature,
            ))
        }
    }
}

type CalculatedParams = Vec<(&'static str, Cow<'static, str>)>;

fn calculate_signing_params<'a>(
    request: &'a SignableRequest<'a>,
    params: &'a SigningParams<'a>,
) -> Result<(CalculatedParams, String), SigningError> {
    let creds = params.credentials()?;
    let creq = CanonicalRequest::from(request, params)?;
    let encoded_creq = &v4::sha256_hex_string(creq.to_string().as_bytes());

    let (signature, string_to_sign) = match params {
        SigningParams::V4(params) => {
            let string_to_sign =
                StringToSign::new_v4(params.time, params.region, params.name, encoded_creq)
                    .to_string();
            let signing_key = v4::generate_signing_key(
                creds.secret_access_key(),
                params.time,
                params.region,
                params.name,
            );
            let signature = v4::calculate_signature(signing_key, string_to_sign.as_bytes());
            (signature, string_to_sign)
        }
        #[cfg(feature = "sigv4a")]
        SigningParams::V4a(params) => {
            let string_to_sign =
                StringToSign::new_v4a(params.time, params.region_set, params.name, encoded_creq)
                    .to_string();

            let secret_key =
                v4a::generate_signing_key(creds.access_key_id(), creds.secret_access_key());
            let signature = v4a::calculate_signature(&secret_key, string_to_sign.as_bytes());
            (signature, string_to_sign)
        }
    };
    tracing::trace!(canonical_request = %creq, string_to_sign = %string_to_sign, "calculated signing parameters");

    let values = creq.values.into_query_params().expect("signing with query");
    let mut signing_params = vec![
        (param::X_AMZ_ALGORITHM, Cow::Borrowed(values.algorithm)),
        (param::X_AMZ_CREDENTIAL, Cow::Owned(values.credential)),
        (param::X_AMZ_DATE, Cow::Owned(values.date_time)),
        (param::X_AMZ_EXPIRES, Cow::Owned(values.expires)),
        (
            param::X_AMZ_SIGNED_HEADERS,
            Cow::Owned(values.signed_headers.as_str().into()),
        ),
        (param::X_AMZ_SIGNATURE, Cow::Owned(signature.clone())),
    ];

    #[cfg(feature = "sigv4a")]
    if let Some(region_set) = params.region_set() {
        if params.signature_version() == SignatureVersion::V4a {
            signing_params.push((
                crate::http_request::canonical_request::sigv4a::param::X_AMZ_REGION_SET,
                Cow::Owned(region_set.to_owned()),
            ));
        }
    }

    if let Some(security_token) = creds.session_token() {
        signing_params.push((
            params
                .settings()
                .session_token_name_override
                .unwrap_or(param::X_AMZ_SECURITY_TOKEN),
            Cow::Owned(security_token.to_string()),
        ));
    }

    Ok((signing_params, signature))
}

/// Calculates the signature headers that need to get added to the given `request`.
///
/// `request` MUST NOT contain any of the following headers:
/// - x-amz-date
/// - x-amz-content-sha-256
/// - x-amz-security-token
fn calculate_signing_headers<'a>(
    request: &'a SignableRequest<'a>,
    params: &'a SigningParams<'a>,
) -> Result<SigningOutput<Vec<Header>>, SigningError> {
    let creds = params.credentials()?;

    // Step 1: https://docs.aws.amazon.com/en_pv/general/latest/gr/sigv4-create-canonical-request.html.
    let creq = CanonicalRequest::from(request, params)?;
    // Step 2: https://docs.aws.amazon.com/en_pv/general/latest/gr/sigv4-create-string-to-sign.html.
    let encoded_creq = v4::sha256_hex_string(creq.to_string().as_bytes());
    tracing::trace!(canonical_request = %creq);
    let mut headers = vec![];

    let signature = match params {
        SigningParams::V4(params) => {
            let sts = StringToSign::new_v4(
                params.time,
                params.region,
                params.name,
                encoded_creq.as_str(),
            );

            // Step 3: https://docs.aws.amazon.com/en_pv/general/latest/gr/sigv4-calculate-signature.html
            let signing_key = v4::generate_signing_key(
                creds.secret_access_key(),
                params.time,
                params.region,
                params.name,
            );
            let signature = v4::calculate_signature(signing_key, sts.to_string().as_bytes());

            // Step 4: https://docs.aws.amazon.com/en_pv/general/latest/gr/sigv4-add-signature-to-request.html
            let values = creq.values.as_headers().expect("signing with headers");
            add_header(&mut headers, header::X_AMZ_DATE, &values.date_time, false);
            headers.push(Header {
                key: "authorization",
                value: build_authorization_header(
                    creds.access_key_id(),
                    &creq,
                    sts,
                    &signature,
                    SignatureVersion::V4,
                ),
                sensitive: false,
            });
            if params.settings.payload_checksum_kind == PayloadChecksumKind::XAmzSha256 {
                add_header(
                    &mut headers,
                    header::X_AMZ_CONTENT_SHA_256,
                    &values.content_sha256,
                    false,
                );
            }

            if let Some(security_token) = creds.session_token() {
                add_header(
                    &mut headers,
                    params
                        .settings
                        .session_token_name_override
                        .unwrap_or(header::X_AMZ_SECURITY_TOKEN),
                    security_token,
                    true,
                );
            }
            signature
        }
        #[cfg(feature = "sigv4a")]
        SigningParams::V4a(params) => {
            let sts = StringToSign::new_v4a(
                params.time,
                params.region_set,
                params.name,
                encoded_creq.as_str(),
            );

            let signing_key =
                v4a::generate_signing_key(creds.access_key_id(), creds.secret_access_key());
            let signature = v4a::calculate_signature(&signing_key, sts.to_string().as_bytes());

            let values = creq.values.as_headers().expect("signing with headers");
            add_header(&mut headers, header::X_AMZ_DATE, &values.date_time, false);
            add_header(
                &mut headers,
                crate::http_request::canonical_request::sigv4a::header::X_AMZ_REGION_SET,
                params.region_set,
                false,
            );

            headers.push(Header {
                key: "authorization",
                value: build_authorization_header(
                    creds.access_key_id(),
                    &creq,
                    sts,
                    &signature,
                    SignatureVersion::V4a,
                ),
                sensitive: false,
            });
            if params.settings.payload_checksum_kind == PayloadChecksumKind::XAmzSha256 {
                add_header(
                    &mut headers,
                    header::X_AMZ_CONTENT_SHA_256,
                    &values.content_sha256,
                    false,
                );
            }

            if let Some(security_token) = creds.session_token() {
                add_header(
                    &mut headers,
                    header::X_AMZ_SECURITY_TOKEN,
                    security_token,
                    true,
                );
            }
            signature
        }
    };

    Ok(SigningOutput::new(headers, signature))
}

fn add_header(map: &mut Vec<Header>, key: &'static str, value: &str, sensitive: bool) {
    map.push(Header {
        key,
        value: value.to_string(),
        sensitive,
    });
}

// add signature to authorization header
// Authorization: algorithm Credential=access key ID/credential scope, SignedHeaders=SignedHeaders, Signature=signature
fn build_authorization_header(
    access_key: &str,
    creq: &CanonicalRequest<'_>,
    sts: StringToSign<'_>,
    signature: &str,
    signature_version: SignatureVersion,
) -> String {
    let scope = match signature_version {
        SignatureVersion::V4 => sts.scope.to_string(),
        SignatureVersion::V4a => sts.scope.v4a_display(),
    };
    format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        sts.algorithm,
        access_key,
        scope,
        creq.values.signed_headers().as_str(),
        signature
    )
}
#[cfg(test)]
mod tests {
    use crate::date_time::test_parsers::parse_date_time;
    use crate::http_request::sign::{add_header, SignableRequest};
    use crate::http_request::test::SigningSuiteTest;
    use crate::http_request::{
        sign, SessionTokenMode, SignableBody, SignatureLocation, SigningInstructions,
        SigningSettings,
    };
    use crate::sign::v4;
    use aws_credential_types::Credentials;
    use http::{HeaderValue, Request};
    use pretty_assertions::assert_eq;
    use proptest::proptest;
    use std::borrow::Cow;
    use std::iter;

    macro_rules! assert_req_eq {
        (http: $expected:expr, $actual:expr) => {
            let mut expected = ($expected).map(|_b|"body");
            let mut actual = ($actual).map(|_b|"body");
            make_headers_comparable(&mut expected);
            make_headers_comparable(&mut actual);
            assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
        };
        ($expected:tt, $actual:tt) => {
            assert_req_eq!(http: ($expected).as_http_request(), $actual);
        };
    }

    pub(crate) fn make_headers_comparable<B>(request: &mut Request<B>) {
        for (_name, value) in request.headers_mut() {
            value.set_sensitive(false);
        }
    }

    // Sigv4A suite tests
    #[cfg(feature = "sigv4a")]
    mod v4a_suite {
        use crate::http_request::test::v4a::run_test_suite_v4a;

        #[test]
        fn test_get_header_key_duplicate() {
            run_test_suite_v4a("get-header-key-duplicate")
        }

        #[test]
        #[ignore = "httpparse doesn't support parsing multiline headers since they are deprecated in RFC7230"]
        fn test_get_header_value_multiline() {
            run_test_suite_v4a("get-header-value-multiline")
        }

        #[test]
        fn test_get_header_value_order() {
            run_test_suite_v4a("get-header-value-order")
        }

        #[test]
        fn test_get_header_value_trim() {
            run_test_suite_v4a("get-header-value-trim");
        }

        #[test]
        fn test_get_relative_normalized() {
            run_test_suite_v4a("get-relative-normalized");
        }

        #[test]
        fn test_get_relative_relative_normalized() {
            run_test_suite_v4a("get-relative-relative-normalized");
        }

        #[test]
        fn test_get_relative_relative_unnormalized() {
            run_test_suite_v4a("get-relative-relative-unnormalized");
        }

        #[test]
        fn test_get_relative_unnormalized() {
            run_test_suite_v4a("get-relative-unnormalized");
        }

        #[test]
        fn test_get_slash_dot_slash_normalized() {
            run_test_suite_v4a("get-slash-dot-slash-normalized");
        }

        #[test]
        fn test_get_slash_dot_slash_unnormalized() {
            run_test_suite_v4a("get-slash-dot-slash-unnormalized");
        }

        #[test]
        fn test_get_slash_normalized() {
            run_test_suite_v4a("get-slash-normalized");
        }

        #[test]
        fn test_get_slash_pointless_dot_normalized() {
            run_test_suite_v4a("get-slash-pointless-dot-normalized");
        }

        #[test]
        fn test_get_slash_pointless_dot_unnormalized() {
            run_test_suite_v4a("get-slash-pointless-dot-unnormalized");
        }

        #[test]
        fn test_get_slash_unnormalized() {
            run_test_suite_v4a("get-slash-unnormalized");
        }

        #[test]
        fn test_get_slashes_normalized() {
            run_test_suite_v4a("get-slashes-normalized");
        }

        #[test]
        fn test_get_slashes_unnormalized() {
            run_test_suite_v4a("get-slashes-unnormalized");
        }

        #[test]
        #[ignore = "relies on single encode of path segments"]
        // rely on single encoding of path segments, i.e. string-to-sign contains %20 for spaces rather than %25%20 as it should.
        // skipped until we add control over double_uri_encode in context.json
        fn test_get_space_normalized() {
            run_test_suite_v4a("get-space-normalized");
        }

        #[test]
        #[ignore = "httpparse fails on unencoded spaces in path"]
        // the input request has unencoded space ' ' in the path which fails to parse
        fn test_get_space_unnormalized() {
            run_test_suite_v4a("get-space-unnormalized");
        }

        #[test]
        fn test_get_unreserved() {
            run_test_suite_v4a("get-unreserved");
        }

        #[test]
        #[ignore = "httparse fails on invalid uri character"]
        // relies on /ሴ canonicalized as /%E1%88%B4 when it should be /%25%E1%25%88%25%B4
        fn test_get_utf8() {
            run_test_suite_v4a("get-utf8");
        }

        #[test]
        fn test_get_vanilla() {
            run_test_suite_v4a("get-vanilla");
        }

        #[test]
        fn test_get_vanilla_empty_query_key() {
            run_test_suite_v4a("get-vanilla-empty-query-key");
        }

        #[test]
        fn test_get_vanilla_query() {
            run_test_suite_v4a("get-vanilla-query");
        }

        #[test]
        fn test_get_vanilla_query_order_encoded() {
            run_test_suite_v4a("get-vanilla-query-order-encoded");
        }

        #[test]
        fn test_get_vanilla_query_order_key_case() {
            run_test_suite_v4a("get-vanilla-query-order-key-case");
        }

        #[test]
        fn test_get_vanilla_query_unreserved() {
            run_test_suite_v4a("get-vanilla-query-unreserved");
        }

        #[test]
        #[ignore = "httparse fails on invalid uri character"]
        // relies on /ሴ canonicalized as /%E1%88%B4 when it should be /%25%E1%25%88%25%B4
        fn test_get_vanilla_utf8_query() {
            run_test_suite_v4a("get-vanilla-utf8-query");
        }

        #[test]
        fn test_get_vanilla_with_session_token() {
            run_test_suite_v4a("get-vanilla-with-session-token")
        }

        #[test]
        fn test_post_header_key_case() {
            run_test_suite_v4a("post-header-key-case");
        }

        #[test]
        fn test_post_header_key_sort() {
            run_test_suite_v4a("post-header-key-sort");
        }

        #[test]
        fn test_post_header_value_case() {
            run_test_suite_v4a("post-header-value-case");
        }

        #[test]
        fn test_post_sts_header_after() {
            run_test_suite_v4a("post-sts-header-after");
        }

        #[test]
        fn test_post_sts_header_before() {
            run_test_suite_v4a("post-sts-header-before");
        }

        #[test]
        fn test_post_vanilla() {
            run_test_suite_v4a("post-vanilla");
        }

        #[test]
        fn test_post_vanilla_empty_query_value() {
            run_test_suite_v4a("post-vanilla-empty-query-value");
        }

        #[test]
        fn test_post_vanilla_query() {
            run_test_suite_v4a("post-vanilla-query");
        }

        #[test]
        fn test_post_x_www_form_urlencoded() {
            run_test_suite_v4a("post-x-www-form-urlencoded");
        }

        #[test]
        fn test_post_x_www_form_urlencoded_parameters() {
            run_test_suite_v4a("post-x-www-form-urlencoded-parameters");
        }
    }

    #[test]
    fn test_sign_url_escape() {
        let test = SigningSuiteTest::v4("double-encode-path");
        let settings = SigningSettings::default();
        let identity = &Credentials::for_tests().into();
        let params = v4::SigningParams {
            identity,
            region: "us-east-1",
            name: "service",
            time: parse_date_time("20150830T123600Z").unwrap(),
            settings,
        }
        .into();

        let original = test.request();
        let signable = SignableRequest::from(&original);
        let out = sign(signable, &params).unwrap();
        assert_eq!(
            "57d157672191bac40bae387e48bbe14b15303c001fdbb01f4abf295dccb09705",
            out.signature
        );

        let mut signed = original.as_http_request();
        out.output.apply_to_request_http1x(&mut signed);

        let expected = test.signed_request(SignatureLocation::Headers);
        assert_req_eq!(expected, signed);
    }

    #[test]
    fn test_sign_headers_utf8() {
        let settings = SigningSettings::default();
        let identity = &Credentials::for_tests().into();
        let params = v4::SigningParams {
            identity,
            region: "us-east-1",
            name: "service",
            time: parse_date_time("20150830T123600Z").unwrap(),
            settings,
        }
        .into();

        let original = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com")
            .header("some-header", HeaderValue::from_str("テスト").unwrap())
            .body("")
            .unwrap()
            .into();
        let signable = SignableRequest::from(&original);
        let out = sign(signable, &params).unwrap();
        assert_eq!(
            "55e16b31f9bde5fd04f9d3b780dd2b5e5f11a5219001f91a8ca9ec83eaf1618f",
            out.signature
        );

        let mut signed = original.as_http_request();
        out.output.apply_to_request_http1x(&mut signed);

        let expected = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com")
            .header("some-header", HeaderValue::from_str("テスト").unwrap())
            .header(
                "x-amz-date",
                HeaderValue::from_str("20150830T123600Z").unwrap(),
            )
            .header(
                "authorization",
                HeaderValue::from_str(
                    "AWS4-HMAC-SHA256 \
                        Credential=ANOTREAL/20150830/us-east-1/service/aws4_request, \
                        SignedHeaders=host;some-header;x-amz-date, \
                        Signature=55e16b31f9bde5fd04f9d3b780dd2b5e5f11a5219001f91a8ca9ec83eaf1618f",
                )
                .unwrap(),
            )
            .body("")
            .unwrap();
        assert_req_eq!(http: expected, signed);
    }

    #[test]
    fn test_sign_headers_excluding_session_token() {
        let settings = SigningSettings {
            session_token_mode: SessionTokenMode::Exclude,
            ..Default::default()
        };
        let identity = &Credentials::for_tests_with_session_token().into();
        let params = v4::SigningParams {
            identity,
            region: "us-east-1",
            name: "service",
            time: parse_date_time("20150830T123600Z").unwrap(),
            settings,
        }
        .into();

        let original = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com")
            .body("")
            .unwrap()
            .into();
        let out_without_session_token = sign(SignableRequest::from(&original), &params).unwrap();

        let out_with_session_token_but_excluded =
            sign(SignableRequest::from(&original), &params).unwrap();
        assert_eq!(
            "ab32de057edf094958d178b3c91f3c8d5c296d526b11da991cd5773d09cea560",
            out_with_session_token_but_excluded.signature
        );
        assert_eq!(
            out_with_session_token_but_excluded.signature,
            out_without_session_token.signature
        );

        let mut signed = original.as_http_request();
        out_with_session_token_but_excluded
            .output
            .apply_to_request_http1x(&mut signed);

        let expected = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com")
            .header(
                "x-amz-date",
                HeaderValue::from_str("20150830T123600Z").unwrap(),
            )
            .header(
                "authorization",
                HeaderValue::from_str(
                    "AWS4-HMAC-SHA256 \
                        Credential=ANOTREAL/20150830/us-east-1/service/aws4_request, \
                        SignedHeaders=host;x-amz-date, \
                        Signature=ab32de057edf094958d178b3c91f3c8d5c296d526b11da991cd5773d09cea560",
                )
                .unwrap(),
            )
            .header(
                "x-amz-security-token",
                HeaderValue::from_str("notarealsessiontoken").unwrap(),
            )
            .body(b"")
            .unwrap();
        assert_req_eq!(http: expected, signed);
    }

    #[test]
    fn test_sign_headers_space_trimming() {
        let settings = SigningSettings::default();
        let identity = &Credentials::for_tests().into();
        let params = v4::SigningParams {
            identity,
            region: "us-east-1",
            name: "service",
            time: parse_date_time("20150830T123600Z").unwrap(),
            settings,
        }
        .into();

        let original = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com")
            .header(
                "some-header",
                HeaderValue::from_str("  test  test   ").unwrap(),
            )
            .body("")
            .unwrap()
            .into();
        let signable = SignableRequest::from(&original);
        let out = sign(signable, &params).unwrap();
        assert_eq!(
            "244f2a0db34c97a528f22715fe01b2417b7750c8a95c7fc104a3c48d81d84c08",
            out.signature
        );

        let mut signed = original.as_http_request();
        out.output.apply_to_request_http1x(&mut signed);

        let expected = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com")
            .header(
                "some-header",
                HeaderValue::from_str("  test  test   ").unwrap(),
            )
            .header(
                "x-amz-date",
                HeaderValue::from_str("20150830T123600Z").unwrap(),
            )
            .header(
                "authorization",
                HeaderValue::from_str(
                    "AWS4-HMAC-SHA256 \
                        Credential=ANOTREAL/20150830/us-east-1/service/aws4_request, \
                        SignedHeaders=host;some-header;x-amz-date, \
                        Signature=244f2a0db34c97a528f22715fe01b2417b7750c8a95c7fc104a3c48d81d84c08",
                )
                .unwrap(),
            )
            .body("")
            .unwrap();
        assert_req_eq!(http: expected, signed);
    }

    proptest! {
        #[test]
        // Only byte values between 32 and 255 (inclusive) are permitted, excluding byte 127, for
        // [HeaderValue](https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.from_bytes).
        fn test_sign_headers_no_panic(
            header in ".*"
        ) {
            let settings = SigningSettings::default();
        let identity = &Credentials::for_tests().into();
        let params = v4::SigningParams {
            identity,
                region: "us-east-1",
                name: "foo",
                time: std::time::SystemTime::UNIX_EPOCH,
                settings,
            }.into();

            let req = SignableRequest::new(
                "GET",
                "https://foo.com",
                iter::once(("x-sign-me", header.as_str())),
                SignableBody::Bytes(&[])
            );

            if let Ok(req) = req {
                // The test considered a pass if the creation of `creq` does not panic.
                let _creq = crate::http_request::sign(req, &params);
            }
        }
    }

    #[test]
    fn apply_signing_instructions_headers() {
        let mut headers = vec![];
        add_header(&mut headers, "some-header", "foo", false);
        add_header(&mut headers, "some-other-header", "bar", false);
        let instructions = SigningInstructions::new(headers, vec![]);

        let mut request = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com")
            .body("")
            .unwrap();

        instructions.apply_to_request_http1x(&mut request);

        let get_header = |n: &str| request.headers().get(n).unwrap().to_str().unwrap();
        assert_eq!("foo", get_header("some-header"));
        assert_eq!("bar", get_header("some-other-header"));
    }

    #[test]
    fn apply_signing_instructions_query_params() {
        let params = vec![
            ("some-param", Cow::Borrowed("f&o?o")),
            ("some-other-param?", Cow::Borrowed("bar")),
        ];
        let instructions = SigningInstructions::new(vec![], params);

        let mut request = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com/some/path")
            .body("")
            .unwrap();

        instructions.apply_to_request_http1x(&mut request);

        assert_eq!(
            "/some/path?some-param=f%26o%3Fo&some-other-param%3F=bar",
            request.uri().path_and_query().unwrap().to_string()
        );
    }

    #[test]
    fn apply_signing_instructions_query_params_http_1x() {
        let params = vec![
            ("some-param", Cow::Borrowed("f&o?o")),
            ("some-other-param?", Cow::Borrowed("bar")),
        ];
        let instructions = SigningInstructions::new(vec![], params);

        let mut request = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com/some/path")
            .body("")
            .unwrap();

        instructions.apply_to_request_http1x(&mut request);

        assert_eq!(
            "/some/path?some-param=f%26o%3Fo&some-other-param%3F=bar",
            request.uri().path_and_query().unwrap().to_string()
        );
    }

    #[test]
    fn test_debug_signable_body() {
        let sut = SignableBody::Bytes(b"hello signable body");
        assert_eq!(
            "Bytes(\"** REDACTED **. To print 19 bytes of raw data, set environment variable `LOG_SIGNABLE_BODY=true`\")",
            format!("{sut:?}")
        );

        let sut = SignableBody::UnsignedPayload;
        assert_eq!("UnsignedPayload", format!("{sut:?}"));

        let sut = SignableBody::Precomputed("precomputed".to_owned());
        assert_eq!("Precomputed(\"precomputed\")", format!("{sut:?}"));

        let sut = SignableBody::StreamingUnsignedPayloadTrailer;
        assert_eq!("StreamingUnsignedPayloadTrailer", format!("{sut:?}"));
    }

    // v4 test suite
    mod v4_suite {
        use crate::http_request::test::run_test_suite_v4;

        #[test]
        fn test_get_header_key_duplicate() {
            run_test_suite_v4("get-header-key-duplicate");
        }

        #[test]
        #[ignore = "httpparse doesn't support parsing multiline headers since they are deprecated in RFC7230"]
        fn test_get_header_value_multiline() {
            run_test_suite_v4("get-header-value-multiline");
        }

        #[test]
        fn test_get_header_value_order() {
            run_test_suite_v4("get-header-value-order");
        }

        #[test]
        fn test_get_header_value_trim() {
            run_test_suite_v4("get-header-value-trim");
        }

        #[test]
        fn test_get_relative_normalized() {
            run_test_suite_v4("get-relative-normalized");
        }

        #[test]
        fn test_get_relative_relative_normalized() {
            run_test_suite_v4("get-relative-relative-normalized");
        }

        #[test]
        fn test_get_relative_relative_unnormalized() {
            run_test_suite_v4("get-relative-relative-unnormalized");
        }

        #[test]
        fn test_get_relative_unnormalized() {
            run_test_suite_v4("get-relative-unnormalized");
        }

        #[test]
        fn test_get_slash_dot_slash_normalized() {
            run_test_suite_v4("get-slash-dot-slash-normalized");
        }

        #[test]
        fn test_get_slash_dot_slash_unnormalized() {
            run_test_suite_v4("get-slash-dot-slash-unnormalized");
        }

        #[test]
        fn test_get_slash_normalized() {
            run_test_suite_v4("get-slash-normalized");
        }

        #[test]
        fn test_get_slash_pointless_dot_normalized() {
            run_test_suite_v4("get-slash-pointless-dot-normalized");
        }

        #[test]
        fn test_get_slash_pointless_dot_unnormalized() {
            run_test_suite_v4("get-slash-pointless-dot-unnormalized");
        }

        #[test]
        fn test_get_slash_unnormalized() {
            run_test_suite_v4("get-slash-unnormalized");
        }

        #[test]
        fn test_get_slashes_normalized() {
            run_test_suite_v4("get-slashes-normalized");
        }

        #[test]
        fn test_get_slashes_unnormalized() {
            run_test_suite_v4("get-slashes-unnormalized");
        }

        #[test]
        #[ignore = "relies on single encode of path segments"]
        // rely on single encoding of path segments, i.e. string-to-sign contains %20 for spaces rather than %25%20 as it should.
        // skipped until we add control over double_uri_encode in context.json
        fn test_get_space_normalized() {
            run_test_suite_v4("get-space-normalized");
        }

        #[test]
        #[ignore = "httpparse fails on unencoded spaces in path"]
        // the input request has unencoded space ' ' in the path which fails to parse
        fn test_get_space_unnormalized() {
            run_test_suite_v4("get-space-unnormalized");
        }

        #[test]
        fn test_get_unreserved() {
            run_test_suite_v4("get-unreserved");
        }

        #[test]
        #[ignore = "httparse fails on invalid uri character"]
        // relies on /ሴ canonicalized as /%E1%88%B4 when it should be /%25%E1%25%88%25%B4
        fn test_get_utf8() {
            run_test_suite_v4("get-utf8");
        }

        #[test]
        fn test_get_vanilla() {
            run_test_suite_v4("get-vanilla");
        }

        #[test]
        fn test_get_vanilla_empty_query_key() {
            run_test_suite_v4("get-vanilla-empty-query-key");
        }

        #[test]
        fn test_get_vanilla_query() {
            run_test_suite_v4("get-vanilla-query");
        }

        #[test]
        fn test_get_vanilla_query_order_encoded() {
            run_test_suite_v4("get-vanilla-query-order-encoded");
        }

        #[test]
        fn test_get_vanilla_query_order_key_case() {
            run_test_suite_v4("get-vanilla-query-order-key-case");
        }

        #[test]
        fn test_get_vanilla_query_unreserved() {
            run_test_suite_v4("get-vanilla-query-unreserved");
        }

        #[test]
        #[ignore = "httparse fails on invalid uri character"]
        // relies on /ሴ canonicalized as /%E1%88%B4 when it should be /%25%E1%25%88%25%B4
        fn test_get_vanilla_utf8_query() {
            run_test_suite_v4("get-vanilla-utf8-query");
        }

        #[test]
        fn test_get_vanilla_with_session_token() {
            run_test_suite_v4("get-vanilla-with-session-token");
        }

        #[test]
        fn test_post_header_key_case() {
            run_test_suite_v4("post-header-key-case");
        }

        #[test]
        fn test_post_header_key_sort() {
            run_test_suite_v4("post-header-key-sort");
        }

        #[test]
        fn test_post_header_value_case() {
            run_test_suite_v4("post-header-value-case");
        }

        #[test]
        fn test_post_sts_header_after() {
            run_test_suite_v4("post-sts-header-after");
        }

        #[test]
        fn test_post_sts_header_before() {
            run_test_suite_v4("post-sts-header-before");
        }

        #[test]
        fn test_post_vanilla() {
            run_test_suite_v4("post-vanilla");
        }

        #[test]
        fn test_post_vanilla_empty_query_value() {
            run_test_suite_v4("post-vanilla-empty-query-value");
        }

        #[test]
        fn test_post_vanilla_query() {
            run_test_suite_v4("post-vanilla-query");
        }

        #[test]
        fn test_post_x_www_form_urlencoded() {
            run_test_suite_v4("post-x-www-form-urlencoded");
        }

        #[test]
        fn test_post_x_www_form_urlencoded_parameters() {
            run_test_suite_v4("post-x-www-form-urlencoded-parameters");
        }
    }
}
