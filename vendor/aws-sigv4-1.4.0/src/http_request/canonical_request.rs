/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::date_time::{format_date, format_date_time};
use crate::http_request::error::CanonicalRequestError;
use crate::http_request::settings::SessionTokenMode;
use crate::http_request::settings::UriPathNormalizationMode;
use crate::http_request::sign::SignableRequest;
use crate::http_request::uri_path_normalization::normalize_uri_path;
use crate::http_request::url_escape::percent_encode_path;
use crate::http_request::{PayloadChecksumKind, SignableBody, SignatureLocation, SigningParams};
use crate::http_request::{PercentEncodingMode, SigningSettings};
use crate::sign::v4::{sha256_hex_string, HMAC_SHA256};
use crate::SignatureVersion;
use aws_smithy_http::query_writer::QueryWriter;
use http::header::{AsHeaderName, HeaderName, HOST};
use http::uri::{Port, Scheme};
use http::{HeaderMap, HeaderValue, Uri};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use std::time::SystemTime;

#[cfg(feature = "sigv4a")]
pub(crate) mod sigv4a;

pub(crate) mod header {
    pub(crate) const X_AMZ_CONTENT_SHA_256: &str = "x-amz-content-sha256";
    pub(crate) const X_AMZ_DATE: &str = "x-amz-date";
    pub(crate) const X_AMZ_SECURITY_TOKEN: &str = "x-amz-security-token";
    pub(crate) const X_AMZ_USER_AGENT: &str = "x-amz-user-agent";
    pub(crate) const X_AMZ_CHECKSUM_MODE: &str = "x-amz-checksum-mode";
}

pub(crate) mod param {
    pub(crate) const X_AMZ_ALGORITHM: &str = "X-Amz-Algorithm";
    pub(crate) const X_AMZ_CREDENTIAL: &str = "X-Amz-Credential";
    pub(crate) const X_AMZ_DATE: &str = "X-Amz-Date";
    pub(crate) const X_AMZ_EXPIRES: &str = "X-Amz-Expires";
    pub(crate) const X_AMZ_SECURITY_TOKEN: &str = "X-Amz-Security-Token";
    pub(crate) const X_AMZ_SIGNED_HEADERS: &str = "X-Amz-SignedHeaders";
    pub(crate) const X_AMZ_SIGNATURE: &str = "X-Amz-Signature";
}

const UNSIGNED_PAYLOAD: &str = "UNSIGNED-PAYLOAD";
const STREAMING_UNSIGNED_PAYLOAD_TRAILER: &str = "STREAMING-UNSIGNED-PAYLOAD-TRAILER";
const STREAMING_SIGNED_PAYLOAD_TRAILER: &str = "STREAMING-AWS4-HMAC-SHA256-PAYLOAD-TRAILER";

#[derive(Debug, PartialEq)]
pub(crate) struct HeaderValues<'a> {
    pub(crate) content_sha256: Cow<'a, str>,
    pub(crate) date_time: String,
    pub(crate) security_token: Option<&'a str>,
    pub(crate) signed_headers: SignedHeaders,
    #[cfg(feature = "sigv4a")]
    pub(crate) region_set: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct QueryParamValues<'a> {
    pub(crate) algorithm: &'static str,
    pub(crate) content_sha256: Cow<'a, str>,
    pub(crate) credential: String,
    pub(crate) date_time: String,
    pub(crate) expires: String,
    pub(crate) security_token: Option<&'a str>,
    pub(crate) signed_headers: SignedHeaders,
    #[cfg(feature = "sigv4a")]
    pub(crate) region_set: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum SignatureValues<'a> {
    Headers(HeaderValues<'a>),
    QueryParams(QueryParamValues<'a>),
}

impl<'a> SignatureValues<'a> {
    pub(crate) fn signed_headers(&self) -> &SignedHeaders {
        match self {
            SignatureValues::Headers(values) => &values.signed_headers,
            SignatureValues::QueryParams(values) => &values.signed_headers,
        }
    }

    fn content_sha256(&self) -> &str {
        match self {
            SignatureValues::Headers(values) => &values.content_sha256,
            SignatureValues::QueryParams(values) => &values.content_sha256,
        }
    }

    pub(crate) fn as_headers(&self) -> Option<&HeaderValues<'_>> {
        match self {
            SignatureValues::Headers(values) => Some(values),
            _ => None,
        }
    }

    #[allow(clippy::result_large_err)]
    /*
       QueryParams(QueryParamValues<'a>),
       --------------------------------- the largest variant contains at least 192 bytes

       Suppressing the Clippy warning, as the variant above is always returned wrapped in `Ok`.
    */
    pub(crate) fn into_query_params(self) -> Result<QueryParamValues<'a>, Self> {
        match self {
            SignatureValues::QueryParams(values) => Ok(values),
            _ => Err(self),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CanonicalRequest<'a> {
    pub(crate) method: &'a str,
    pub(crate) path: Cow<'a, str>,
    pub(crate) params: Option<String>,
    pub(crate) headers: HeaderMap,
    pub(crate) values: SignatureValues<'a>,
}

impl CanonicalRequest<'_> {
    /// Construct a CanonicalRequest from a [`SignableRequest`] and [`SigningParams`].
    ///
    /// The returned canonical request includes information required for signing as well
    /// as query parameters or header values that go along with the signature in a request.
    ///
    /// ## Behavior
    ///
    /// There are several settings which alter signing behavior:
    /// - If a `security_token` is provided as part of the credentials it will be included in the signed headers
    /// - If `settings.percent_encoding_mode` specifies double encoding, `%` in the URL will be re-encoded as `%25`
    /// - If `settings.payload_checksum_kind` is XAmzSha256, add a x-amz-content-sha256 with the body
    ///   checksum. This is the same checksum used as the "payload_hash" in the canonical request
    /// - If `settings.session_token_mode` specifies X-Amz-Security-Token to be
    ///   included before calculating the signature, add it, otherwise omit it.
    /// - `settings.signature_location` determines where the signature will be placed in a request,
    ///   and also alters the kinds of signing values that go along with it in the request.
    pub(crate) fn from<'b>(
        req: &'b SignableRequest<'b>,
        params: &'b SigningParams<'b>,
    ) -> Result<CanonicalRequest<'b>, CanonicalRequestError> {
        let creds = params
            .credentials()
            .map_err(|_| CanonicalRequestError::unsupported_identity_type())?;
        // Path encoding: if specified, re-encode % as %25
        // Set method and path into CanonicalRequest
        let path = req.uri().path();
        let path = match params.settings().uri_path_normalization_mode {
            UriPathNormalizationMode::Enabled => normalize_uri_path(path),
            UriPathNormalizationMode::Disabled => Cow::Borrowed(path),
        };
        let path = match params.settings().percent_encoding_mode {
            // The string is already URI encoded, we don't need to encode everything again, just `%`
            PercentEncodingMode::Double => Cow::Owned(percent_encode_path(&path)),
            PercentEncodingMode::Single => path,
        };
        let payload_hash = Self::payload_hash(req.body());

        let date_time = format_date_time(*params.time());
        let (signed_headers, canonical_headers) =
            Self::headers(req, params, &payload_hash, &date_time)?;
        let signed_headers = SignedHeaders::new(signed_headers);

        let security_token = match params.settings().session_token_mode {
            SessionTokenMode::Include => creds.session_token(),
            SessionTokenMode::Exclude => None,
        };

        let values = match params.settings().signature_location {
            SignatureLocation::Headers => SignatureValues::Headers(HeaderValues {
                content_sha256: payload_hash,
                date_time,
                security_token,
                signed_headers,
                #[cfg(feature = "sigv4a")]
                region_set: params.region_set(),
            }),
            SignatureLocation::QueryParams => {
                let credential = match params {
                    SigningParams::V4(params) => {
                        format!(
                            "{}/{}/{}/{}/aws4_request",
                            creds.access_key_id(),
                            format_date(params.time),
                            params.region,
                            params.name,
                        )
                    }
                    #[cfg(feature = "sigv4a")]
                    SigningParams::V4a(params) => {
                        format!(
                            "{}/{}/{}/aws4_request",
                            creds.access_key_id(),
                            format_date(params.time),
                            params.name,
                        )
                    }
                };

                SignatureValues::QueryParams(QueryParamValues {
                    algorithm: params.algorithm(),
                    content_sha256: payload_hash,
                    credential,
                    date_time,
                    expires: params
                        .settings()
                        .expires_in
                        .expect("presigning requires expires_in")
                        .as_secs()
                        .to_string(),
                    security_token,
                    signed_headers,
                    #[cfg(feature = "sigv4a")]
                    region_set: params.region_set(),
                })
            }
        };

        let creq = CanonicalRequest {
            method: req.method(),
            path,
            params: Self::params(req.uri(), &values, params.settings()),
            headers: canonical_headers,
            values,
        };
        Ok(creq)
    }

    fn headers(
        req: &SignableRequest<'_>,
        params: &SigningParams<'_>,
        payload_hash: &str,
        date_time: &str,
    ) -> Result<(Vec<CanonicalHeaderName>, HeaderMap), CanonicalRequestError> {
        // Header computation:
        // The canonical request will include headers not present in the input. We need to clone and
        // normalize the headers from the original request and add:
        // - host
        // - x-amz-date
        // - x-amz-security-token (if provided)
        // - x-amz-content-sha256 (if requested by signing settings)
        let mut canonical_headers = HeaderMap::with_capacity(req.headers().len());
        for (name, value) in req.headers().iter() {
            // Header names and values need to be normalized according to Step 4 of https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html
            // Using append instead of insert means this will not clobber headers that have the same lowercased name
            canonical_headers.append(
                HeaderName::from_str(&name.to_lowercase())?,
                normalize_header_value(value)?,
            );
        }

        Self::insert_host_header(&mut canonical_headers, req.uri());

        let token_header_name = params
            .settings()
            .session_token_name_override
            .unwrap_or(header::X_AMZ_SECURITY_TOKEN);

        if params.settings().signature_location == SignatureLocation::Headers {
            let creds = params
                .credentials()
                .map_err(|_| CanonicalRequestError::unsupported_identity_type())?;
            Self::insert_date_header(&mut canonical_headers, date_time);

            if let Some(security_token) = creds.session_token() {
                let mut sec_header = HeaderValue::from_str(security_token)?;
                sec_header.set_sensitive(true);
                canonical_headers.insert(token_header_name, sec_header);
            }

            if params.settings().payload_checksum_kind == PayloadChecksumKind::XAmzSha256 {
                let header = HeaderValue::from_str(payload_hash)?;
                canonical_headers.insert(header::X_AMZ_CONTENT_SHA_256, header);
            }

            #[cfg(feature = "sigv4a")]
            if let Some(region_set) = params.region_set() {
                let header = HeaderValue::from_str(region_set)?;
                canonical_headers.insert(sigv4a::header::X_AMZ_REGION_SET, header);
            }
        }

        let mut signed_headers = Vec::with_capacity(canonical_headers.len());
        for name in canonical_headers.keys() {
            if let Some(excluded_headers) = params.settings().excluded_headers.as_ref() {
                if excluded_headers.iter().any(|it| name.as_str() == it) {
                    continue;
                }
            }

            if params.settings().session_token_mode == SessionTokenMode::Exclude
                && name == HeaderName::from_static(token_header_name)
            {
                continue;
            }

            if params.settings().signature_location == SignatureLocation::QueryParams {
                // The X-Amz-User-Agent and x-amz-checksum-mode headers should not be signed if this is for a presigned URL
                if name == HeaderName::from_static(header::X_AMZ_USER_AGENT)
                    || name == HeaderName::from_static(header::X_AMZ_CHECKSUM_MODE)
                {
                    continue;
                }
            }
            signed_headers.push(CanonicalHeaderName(name.clone()));
        }

        Ok((signed_headers, canonical_headers))
    }

    fn payload_hash<'b>(body: &'b SignableBody<'b>) -> Cow<'b, str> {
        // Payload hash computation
        //
        // Based on the input body, set the payload_hash of the canonical request:
        // Either:
        // - compute a hash
        // - use the precomputed hash
        // - use `UnsignedPayload`
        // - use `UnsignedPayload` for streaming requests
        // - use `StreamingUnsignedPayloadTrailer` for streaming requests with trailers
        // - use `StreamingSignedPayloadTrailer` for streaming requests with trailers
        match body {
            SignableBody::Bytes(data) => Cow::Owned(sha256_hex_string(data)),
            SignableBody::Precomputed(digest) => Cow::Borrowed(digest.as_str()),
            SignableBody::UnsignedPayload => Cow::Borrowed(UNSIGNED_PAYLOAD),
            SignableBody::StreamingUnsignedPayloadTrailer => {
                Cow::Borrowed(STREAMING_UNSIGNED_PAYLOAD_TRAILER)
            }
            SignableBody::StreamingSignedPayloadTrailer => {
                Cow::Borrowed(STREAMING_SIGNED_PAYLOAD_TRAILER)
            }
        }
    }

    fn params(
        uri: &Uri,
        values: &SignatureValues<'_>,
        settings: &SigningSettings,
    ) -> Option<String> {
        let mut params: Vec<(Cow<'_, str>, Cow<'_, str>)> =
            form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect();
        fn add_param<'a>(params: &mut Vec<(Cow<'a, str>, Cow<'a, str>)>, k: &'a str, v: &'a str) {
            params.push((Cow::Borrowed(k), Cow::Borrowed(v)));
        }

        if let SignatureValues::QueryParams(values) = values {
            add_param(&mut params, param::X_AMZ_DATE, &values.date_time);
            add_param(&mut params, param::X_AMZ_EXPIRES, &values.expires);

            #[cfg(feature = "sigv4a")]
            if let Some(regions) = values.region_set {
                add_param(&mut params, sigv4a::param::X_AMZ_REGION_SET, regions);
            }

            add_param(&mut params, param::X_AMZ_ALGORITHM, values.algorithm);
            add_param(&mut params, param::X_AMZ_CREDENTIAL, &values.credential);
            add_param(
                &mut params,
                param::X_AMZ_SIGNED_HEADERS,
                values.signed_headers.as_str(),
            );

            if let Some(security_token) = values.security_token {
                add_param(
                    &mut params,
                    settings
                        .session_token_name_override
                        .unwrap_or(param::X_AMZ_SECURITY_TOKEN),
                    security_token,
                );
            }
        }

        // Sort on the _encoded_ key/value pairs
        let mut params: Vec<(String, String)> = params
            .into_iter()
            .map(|x| {
                use aws_smithy_http::query::fmt_string;
                let enc_k = fmt_string(&x.0);
                let enc_v = fmt_string(&x.1);
                (enc_k, enc_v)
            })
            .collect();

        params.sort();

        let mut query = QueryWriter::new(uri);
        query.clear_params();
        for (key, value) in params {
            query.insert_encoded(&key, &value);
        }

        let query = query.build_query();
        if query.is_empty() {
            None
        } else {
            Some(query)
        }
    }

    fn insert_host_header(
        canonical_headers: &mut HeaderMap<HeaderValue>,
        uri: &Uri,
    ) -> HeaderValue {
        match canonical_headers.get(&HOST) {
            Some(header) => header.clone(),
            None => {
                let port = uri.port();
                let scheme = uri.scheme();
                let authority = uri
                    .authority()
                    .expect("request uri authority must be set for signing")
                    .as_str();
                let host = uri
                    .host()
                    .expect("request uri host must be set for signing");

                // Check if port is default (80 for HTTP, 443 for HTTPS) and if so exclude it from the
                // Host header when signing since RFC 2616 indicates that the default port should not be
                // sent in the Host header (and Hyper strips default ports if they are present)
                // https://datatracker.ietf.org/doc/html/rfc2616#section-14.23
                // https://github.com/awslabs/aws-sdk-rust/issues/1244
                let header_value = if is_port_scheme_default(scheme, port) {
                    host
                } else {
                    authority
                };

                let header = HeaderValue::try_from(header_value)
                    .expect("endpoint must contain valid header characters");
                canonical_headers.insert(HOST, header.clone());
                header
            }
        }
    }

    fn insert_date_header(
        canonical_headers: &mut HeaderMap<HeaderValue>,
        date_time: &str,
    ) -> HeaderValue {
        let x_amz_date = HeaderName::from_static(header::X_AMZ_DATE);
        let date_header = HeaderValue::try_from(date_time).expect("date is valid header value");
        canonical_headers.insert(x_amz_date, date_header.clone());
        date_header
    }

    fn header_values_for(&self, key: impl AsHeaderName) -> String {
        let values: Vec<&str> = self
            .headers
            .get_all(key)
            .into_iter()
            .map(|value| {
                std::str::from_utf8(value.as_bytes())
                    .expect("SDK request header values are valid UTF-8")
            })
            .collect();
        values.join(",")
    }
}

impl fmt::Display for CanonicalRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.method)?;
        writeln!(f, "{}", self.path)?;
        writeln!(f, "{}", self.params.as_deref().unwrap_or(""))?;
        // write out _all_ the headers
        for header in &self.values.signed_headers().headers {
            write!(f, "{}:", header.0.as_str())?;
            writeln!(f, "{}", self.header_values_for(&header.0))?;
        }
        writeln!(f)?;
        // write out the signed headers
        writeln!(f, "{}", self.values.signed_headers().as_str())?;
        write!(f, "{}", self.values.content_sha256())?;
        Ok(())
    }
}

/// Removes excess spaces before and after a given byte string, and converts multiple sequential
/// spaces to a single space e.g. "  Some  example   text  " -> "Some example text".
///
/// This function ONLY affects spaces and not other kinds of whitespace.
fn trim_all(text: &str) -> Cow<'_, str> {
    let text = text.trim_matches(' ');
    let requires_filter = text
        .chars()
        .zip(text.chars().skip(1))
        .any(|(a, b)| a == ' ' && b == ' ');
    if !requires_filter {
        Cow::Borrowed(text)
    } else {
        // The normal trim function will trim non-breaking spaces and other various whitespace chars.
        // S3 ONLY trims spaces so we use trim_matches to trim spaces only
        Cow::Owned(
            text.chars()
                // Filter out consecutive spaces
                .zip(text.chars().skip(1).chain(std::iter::once('!')))
                .filter(|(a, b)| *a != ' ' || *b != ' ')
                .map(|(a, _)| a)
                .collect(),
        )
    }
}

/// Works just like [trim_all] but acts on HeaderValues instead of bytes.
/// Will ensure that the underlying bytes are valid UTF-8.
fn normalize_header_value(header_value: &str) -> Result<HeaderValue, CanonicalRequestError> {
    let trimmed_value = trim_all(header_value);
    HeaderValue::from_str(&trimmed_value).map_err(CanonicalRequestError::from)
}

#[inline]
fn is_port_scheme_default(scheme: Option<&Scheme>, port: Option<Port<&str>>) -> bool {
    if let (Some(scheme), Some(port)) = (scheme, port) {
        return [("http", "80"), ("https", "443")].contains(&(scheme.as_str(), port.as_str()));
    }

    false
}

#[derive(Debug, PartialEq, Default)]
pub(crate) struct SignedHeaders {
    headers: Vec<CanonicalHeaderName>,
    formatted: String,
}

impl SignedHeaders {
    fn new(mut headers: Vec<CanonicalHeaderName>) -> Self {
        headers.sort();
        let formatted = Self::fmt(&headers);
        SignedHeaders { headers, formatted }
    }

    fn fmt(headers: &[CanonicalHeaderName]) -> String {
        let mut value = String::new();
        let mut iter = headers.iter().peekable();
        while let Some(next) = iter.next() {
            value += next.0.as_str();
            if iter.peek().is_some() {
                value.push(';');
            }
        }
        value
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.formatted
    }
}

impl fmt::Display for SignedHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct CanonicalHeaderName(HeaderName);

impl PartialOrd for CanonicalHeaderName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CanonicalHeaderName {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.as_str().cmp(other.0.as_str())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct SigningScope<'a> {
    pub(crate) time: SystemTime,
    pub(crate) region: &'a str,
    pub(crate) service: &'a str,
}

impl SigningScope<'_> {
    pub(crate) fn v4a_display(&self) -> String {
        format!("{}/{}/aws4_request", format_date(self.time), self.service)
    }
}

impl fmt::Display for SigningScope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/aws4_request",
            format_date(self.time),
            self.region,
            self.service
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct StringToSign<'a> {
    pub(crate) algorithm: &'static str,
    pub(crate) scope: SigningScope<'a>,
    pub(crate) time: SystemTime,
    pub(crate) region: &'a str,
    pub(crate) service: &'a str,
    pub(crate) hashed_creq: &'a str,
    signature_version: SignatureVersion,
}

impl<'a> StringToSign<'a> {
    pub(crate) fn new_v4(
        time: SystemTime,
        region: &'a str,
        service: &'a str,
        hashed_creq: &'a str,
    ) -> Self {
        let scope = SigningScope {
            time,
            region,
            service,
        };
        Self {
            algorithm: HMAC_SHA256,
            scope,
            time,
            region,
            service,
            hashed_creq,
            signature_version: SignatureVersion::V4,
        }
    }

    #[cfg(feature = "sigv4a")]
    pub(crate) fn new_v4a(
        time: SystemTime,
        region_set: &'a str,
        service: &'a str,
        hashed_creq: &'a str,
    ) -> Self {
        use crate::sign::v4a::ECDSA_256;

        let scope = SigningScope {
            time,
            region: region_set,
            service,
        };
        Self {
            algorithm: ECDSA_256,
            scope,
            time,
            region: region_set,
            service,
            hashed_creq,
            signature_version: SignatureVersion::V4a,
        }
    }
}

impl fmt::Display for StringToSign<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}",
            self.algorithm,
            format_date_time(self.time),
            match self.signature_version {
                SignatureVersion::V4 => self.scope.to_string(),
                SignatureVersion::V4a => self.scope.v4a_display(),
            },
            self.hashed_creq
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::date_time::test_parsers::parse_date_time;
    use crate::http_request::canonical_request::{
        normalize_header_value, trim_all, CanonicalRequest, SigningScope, StringToSign,
    };
    use crate::http_request::test;
    use crate::http_request::test::SigningSuiteTest;
    use crate::http_request::{
        PayloadChecksumKind, SessionTokenMode, SignableBody, SignableRequest, SignatureLocation,
        SigningParams, SigningSettings,
    };
    use crate::sign::v4;
    use crate::sign::v4::sha256_hex_string;
    use aws_credential_types::Credentials;
    use aws_smithy_http::query_writer::QueryWriter;
    use aws_smithy_runtime_api::client::identity::Identity;
    use http::{HeaderValue, Uri};
    use pretty_assertions::assert_eq;
    use proptest::{prelude::*, proptest};
    use std::borrow::Cow;
    use std::time::Duration;

    fn signing_params(identity: &Identity, settings: SigningSettings) -> SigningParams<'_> {
        v4::signing_params::Builder::default()
            .identity(identity)
            .region("test-region")
            .name("testservicename")
            .time(parse_date_time("20210511T154045Z").unwrap())
            .settings(settings)
            .build()
            .unwrap()
            .into()
    }

    #[test]
    fn test_repeated_header() {
        let test = test::SigningSuiteTest::v4("get-vanilla-query-order-key-case");
        let mut req = test.request();
        req.headers.push((
            "x-amz-object-attributes".to_string(),
            "Checksum".to_string(),
        ));
        req.headers.push((
            "x-amz-object-attributes".to_string(),
            "ObjectSize".to_string(),
        ));
        let req = SignableRequest::from(&req);
        let settings = SigningSettings {
            payload_checksum_kind: PayloadChecksumKind::XAmzSha256,
            session_token_mode: SessionTokenMode::Exclude,
            ..Default::default()
        };
        let identity = Credentials::for_tests().into();
        let signing_params = signing_params(&identity, settings);
        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();

        assert_eq!(
            creq.values.signed_headers().to_string(),
            "host;x-amz-content-sha256;x-amz-date;x-amz-object-attributes"
        );
        assert_eq!(
            creq.header_values_for("x-amz-object-attributes"),
            "Checksum,ObjectSize",
        );
    }

    #[test]
    fn test_host_header_properly_handles_ports() {
        fn host_header_test_setup(endpoint: String) -> String {
            let test = SigningSuiteTest::v4("get-vanilla");
            let mut req = test.request();
            req.uri = endpoint;
            let req = SignableRequest::from(&req);
            let settings = SigningSettings {
                payload_checksum_kind: PayloadChecksumKind::XAmzSha256,
                session_token_mode: SessionTokenMode::Exclude,
                ..Default::default()
            };
            let identity = Credentials::for_tests().into();
            let signing_params = signing_params(&identity, settings);
            let creq = CanonicalRequest::from(&req, &signing_params).unwrap();
            creq.header_values_for("host")
        }

        // HTTP request with 80 port should not be signed with that port
        let http_80_host_header = host_header_test_setup("http://localhost:80".into());
        assert_eq!(http_80_host_header, "localhost",);

        // HTTP request with non-80 port should be signed with that port
        let http_1234_host_header = host_header_test_setup("http://localhost:1234".into());
        assert_eq!(http_1234_host_header, "localhost:1234",);

        // HTTPS request with 443 port should not be signed with that port
        let https_443_host_header = host_header_test_setup("https://localhost:443".into());
        assert_eq!(https_443_host_header, "localhost",);

        // HTTPS request with non-443 port should be signed with that port
        let https_1234_host_header = host_header_test_setup("https://localhost:1234".into());
        assert_eq!(https_1234_host_header, "localhost:1234",);
    }

    #[test]
    fn test_set_xamz_sha_256() {
        let test = SigningSuiteTest::v4("get-vanilla-query-order-key-case");
        let req = test.request();
        let req = SignableRequest::from(&req);
        let settings = SigningSettings {
            payload_checksum_kind: PayloadChecksumKind::XAmzSha256,
            session_token_mode: SessionTokenMode::Exclude,
            ..Default::default()
        };
        let identity = Credentials::for_tests().into();
        let mut signing_params = signing_params(&identity, settings);
        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();
        assert_eq!(
            creq.values.content_sha256(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        // assert that the sha256 header was added
        assert_eq!(
            creq.values.signed_headers().as_str(),
            "host;x-amz-content-sha256;x-amz-date"
        );

        signing_params.set_payload_checksum_kind(PayloadChecksumKind::NoHeader);
        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();
        assert_eq!(creq.values.signed_headers().as_str(), "host;x-amz-date");
    }

    #[test]
    fn test_unsigned_payload() {
        let test = SigningSuiteTest::v4("get-vanilla-query-order-key-case");
        let mut req = test.request();
        req.set_body(SignableBody::UnsignedPayload);
        let req: SignableRequest<'_> = SignableRequest::from(&req);

        let settings = SigningSettings {
            payload_checksum_kind: PayloadChecksumKind::XAmzSha256,
            ..Default::default()
        };
        let identity = Credentials::for_tests().into();
        let signing_params = signing_params(&identity, settings);
        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();
        assert_eq!(creq.values.content_sha256(), "UNSIGNED-PAYLOAD");
        assert!(creq.to_string().ends_with("UNSIGNED-PAYLOAD"));
    }

    #[test]
    fn test_precomputed_payload() {
        let payload_hash = "44ce7dd67c959e0d3524ffac1771dfbba87d2b6b4b4e99e42034a8b803f8b072";
        let test = SigningSuiteTest::v4("get-vanilla-query-order-key-case");
        let mut req = test.request();
        req.set_body(SignableBody::Precomputed(String::from(payload_hash)));
        let req = SignableRequest::from(&req);
        let settings = SigningSettings {
            payload_checksum_kind: PayloadChecksumKind::XAmzSha256,
            ..Default::default()
        };
        let identity = Credentials::for_tests().into();
        let signing_params = signing_params(&identity, settings);
        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();
        assert_eq!(creq.values.content_sha256(), payload_hash);
        assert!(creq.to_string().ends_with(payload_hash));
    }

    #[test]
    fn test_generate_scope() {
        let expected = "20150830/us-east-1/iam/aws4_request\n";
        let scope = SigningScope {
            time: parse_date_time("20150830T123600Z").unwrap(),
            region: "us-east-1",
            service: "iam",
        };
        assert_eq!(format!("{}\n", scope), expected);
    }

    #[test]
    fn test_string_to_sign() {
        let time = parse_date_time("20150830T123600Z").unwrap();
        let test = SigningSuiteTest::v4("get-vanilla-query-order-key-case");
        let creq = test.canonical_request(SignatureLocation::Headers);
        let expected_sts = test.string_to_sign(SignatureLocation::Headers);
        let encoded = sha256_hex_string(creq.as_bytes());

        let actual = StringToSign::new_v4(time, "us-east-1", "service", &encoded);
        assert_eq!(expected_sts, actual.to_string());
    }

    #[test]
    fn test_digest_of_canonical_request() {
        let test = SigningSuiteTest::v4("get-vanilla-query-order-key-case");
        let creq = test.canonical_request(SignatureLocation::Headers);
        let expected = "816cd5b414d056048ba4f7c5386d6e0533120fb1fcfa93762cf0fc39e2cf19e0";
        let actual = sha256_hex_string(creq.as_bytes());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_double_url_encode_path() {
        let test = SigningSuiteTest::v4("double-encode-path");
        let req = test.request();
        let req = SignableRequest::from(&req);
        let identity = Credentials::for_tests().into();
        let signing_params = signing_params(&identity, SigningSettings::default());
        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();

        let expected = test.canonical_request(SignatureLocation::Headers);
        let actual = format!("{}", creq);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_double_url_encode() {
        let test = SigningSuiteTest::v4("double-url-encode");
        let req = test.request();
        let req = SignableRequest::from(&req);
        let identity = Credentials::for_tests().into();
        let signing_params = signing_params(&identity, SigningSettings::default());
        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();
        let expected = test.canonical_request(SignatureLocation::Headers);
        let actual = format!("{}", creq);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_tilde_in_uri() {
        let req = http::Request::builder()
            .uri("https://s3.us-east-1.amazonaws.com/my-bucket?list-type=2&prefix=~objprefix&single&k=&unreserved=-_.~").body("").unwrap().into();
        let req = SignableRequest::from(&req);
        let identity = Credentials::for_tests().into();
        let signing_params = signing_params(&identity, SigningSettings::default());
        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();
        assert_eq!(
            Some("k=&list-type=2&prefix=~objprefix&single=&unreserved=-_.~"),
            creq.params.as_deref(),
        );
    }

    #[test]
    fn test_signing_urls_with_percent_encoded_query_strings() {
        let all_printable_ascii_chars: String = (32u8..127).map(char::from).collect();
        let uri = Uri::from_static("https://s3.us-east-1.amazonaws.com/my-bucket");

        let mut query_writer = QueryWriter::new(&uri);
        query_writer.insert("list-type", "2");
        query_writer.insert("prefix", &all_printable_ascii_chars);

        let req = http::Request::builder()
            .uri(query_writer.build_uri())
            .body("")
            .unwrap()
            .into();
        let req = SignableRequest::from(&req);
        let identity = Credentials::for_tests().into();
        let signing_params = signing_params(&identity, SigningSettings::default());
        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();

        let expected = "list-type=2&prefix=%20%21%22%23%24%25%26%27%28%29%2A%2B%2C-.%2F0123456789%3A%3B%3C%3D%3E%3F%40ABCDEFGHIJKLMNOPQRSTUVWXYZ%5B%5C%5D%5E_%60abcdefghijklmnopqrstuvwxyz%7B%7C%7D~";
        let actual = creq.params.unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_omit_session_token() {
        let test = SigningSuiteTest::v4("get-vanilla-query-order-key-case");
        let req = test.request();
        let req = SignableRequest::from(&req);
        let settings = SigningSettings {
            session_token_mode: SessionTokenMode::Include,
            ..Default::default()
        };
        let identity = Credentials::for_tests_with_session_token().into();
        let mut signing_params = signing_params(&identity, settings);

        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();
        assert_eq!(
            creq.values.signed_headers().as_str(),
            "host;x-amz-date;x-amz-security-token"
        );
        assert_eq!(
            creq.headers.get("x-amz-security-token").unwrap(),
            "notarealsessiontoken"
        );

        signing_params.set_session_token_mode(SessionTokenMode::Exclude);
        let creq = CanonicalRequest::from(&req, &signing_params).unwrap();
        assert_eq!(
            creq.headers.get("x-amz-security-token").unwrap(),
            "notarealsessiontoken"
        );
        assert_eq!(creq.values.signed_headers().as_str(), "host;x-amz-date");
    }

    // It should exclude authorization, user-agent, x-amzn-trace-id, and transfer-encoding headers from presigning
    #[test]
    fn non_presigning_header_exclusion() {
        let request = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com")
            .header("authorization", "test-authorization")
            .header("content-type", "application/xml")
            .header("content-length", "0")
            .header("user-agent", "test-user-agent")
            .header("x-amzn-trace-id", "test-trace-id")
            .header("x-amz-user-agent", "test-user-agent")
            .header("transfer-encoding", "chunked")
            .body("")
            .unwrap()
            .into();
        let request = SignableRequest::from(&request);

        let settings = SigningSettings {
            signature_location: SignatureLocation::Headers,
            ..Default::default()
        };

        let identity = Credentials::for_tests().into();
        let signing_params = signing_params(&identity, settings);
        let canonical = CanonicalRequest::from(&request, &signing_params).unwrap();

        let values = canonical.values.as_headers().unwrap();
        assert_eq!(
            "content-length;content-type;host;x-amz-date;x-amz-user-agent",
            values.signed_headers.as_str()
        );
    }

    // It should exclude authorization, user-agent, x-amz-user-agent, x-amzn-trace-id, and transfer-encoding headers from presigning
    #[test]
    fn presigning_header_exclusion() {
        let request = http::Request::builder()
            .uri("https://some-endpoint.some-region.amazonaws.com")
            .header("authorization", "test-authorization")
            .header("content-type", "application/xml")
            .header("content-length", "0")
            .header("user-agent", "test-user-agent")
            .header("x-amzn-trace-id", "test-trace-id")
            .header("x-amz-user-agent", "test-user-agent")
            .header("transfer-encoding", "chunked")
            .body("")
            .unwrap()
            .into();
        let request = SignableRequest::from(&request);

        let settings = SigningSettings {
            signature_location: SignatureLocation::QueryParams,
            expires_in: Some(Duration::from_secs(30)),
            ..Default::default()
        };

        let identity = Credentials::for_tests().into();
        let signing_params = signing_params(&identity, settings);
        let canonical = CanonicalRequest::from(&request, &signing_params).unwrap();

        let values = canonical.values.into_query_params().unwrap();
        assert_eq!(
            "content-length;content-type;host",
            values.signed_headers.as_str()
        );
    }

    #[allow(clippy::ptr_arg)] // The proptest macro requires this arg to be a Vec instead of a slice.
    fn valid_input(input: &Vec<String>) -> bool {
        [
            "content-length".to_owned(),
            "content-type".to_owned(),
            "host".to_owned(),
        ]
        .iter()
        .all(|element| !input.contains(element))
    }

    proptest! {
        #[test]
        fn presigning_header_exclusion_with_explicit_exclusion_list_specified(
            excluded_headers in prop::collection::vec("[a-z]{1,20}", 1..10).prop_filter(
                "`excluded_headers` should pass the `valid_input` check",
                valid_input,
            )
        ) {
            let mut request_builder = http::Request::builder()
                .uri("https://some-endpoint.some-region.amazonaws.com")
                .header("content-type", "application/xml")
                .header("content-length", "0");
            for key in &excluded_headers {
                request_builder = request_builder.header(key, "value");
            }
            let request = request_builder.body("").unwrap().into();

            let request = SignableRequest::from(&request);

            let settings = SigningSettings {
                signature_location: SignatureLocation::QueryParams,
                expires_in: Some(Duration::from_secs(30)),
                excluded_headers: Some(
                    excluded_headers
                        .into_iter()
                        .map(std::borrow::Cow::Owned)
                        .collect(),
                ),
                ..Default::default()
            };

        let identity = Credentials::for_tests().into();
        let signing_params = signing_params(&identity, settings);
            let canonical = CanonicalRequest::from(&request, &signing_params).unwrap();

            let values = canonical.values.into_query_params().unwrap();
            assert_eq!(
                "content-length;content-type;host",
                values.signed_headers.as_str()
            );
        }
    }

    #[test]
    fn test_trim_all_handles_spaces_correctly() {
        assert_eq!(Cow::Borrowed("don't touch me"), trim_all("don't touch me"));
        assert_eq!("trim left", trim_all("   trim left"));
        assert_eq!("trim right", trim_all("trim right "));
        assert_eq!("trim both", trim_all("   trim both  "));
        assert_eq!("", trim_all(" "));
        assert_eq!("", trim_all("  "));
        assert_eq!("a b", trim_all(" a   b "));
        assert_eq!("Some example text", trim_all("  Some  example   text  "));
    }

    #[test]
    fn test_trim_all_ignores_other_forms_of_whitespace() {
        // \xA0 is a non-breaking space character
        assert_eq!(
            "\t\u{A0}Some\u{A0} example \u{A0}text\u{A0}\n",
            trim_all("\t\u{A0}Some\u{A0}     example   \u{A0}text\u{A0}\n")
        );
    }

    #[test]
    fn trim_spaces_works_on_single_characters() {
        assert_eq!(trim_all("2").as_ref(), "2");
    }

    proptest! {
        #[test]
        fn test_trim_all_doesnt_elongate_strings(s in ".*") {
            assert!(trim_all(&s).len() <= s.len())
        }

        #[test]
        fn test_normalize_header_value_works_on_valid_header_value(v in (".*")) {
            assert_eq!(normalize_header_value(&v).is_ok(), HeaderValue::from_str(&v).is_ok());
        }

        #[test]
        fn test_trim_all_does_nothing_when_there_are_no_spaces(s in "[^ ]*") {
            assert_eq!(trim_all(&s).as_ref(), s);
        }
    }
}
