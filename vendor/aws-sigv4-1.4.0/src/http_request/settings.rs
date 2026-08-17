/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use http::header::{AUTHORIZATION, TRANSFER_ENCODING, USER_AGENT};
use std::borrow::Cow;
use std::time::Duration;

const HEADER_NAME_X_RAY_TRACE_ID: &str = "x-amzn-trace-id";

/// HTTP-specific signing settings
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SigningSettings {
    /// Specifies how to encode the request URL when signing. Some services do not decode
    /// the path prior to checking the signature, requiring clients to actually _double-encode_
    /// the URI in creating the canonical request in order to pass a signature check.
    pub percent_encoding_mode: PercentEncodingMode,

    /// Add an additional checksum header
    pub payload_checksum_kind: PayloadChecksumKind,

    /// Where to put the signature
    pub signature_location: SignatureLocation,

    /// For presigned requests, how long the presigned request is valid for
    pub expires_in: Option<Duration>,

    /// Headers that should be excluded from the signing process
    pub excluded_headers: Option<Vec<Cow<'static, str>>>,

    /// Specifies whether the absolute path component of the URI should be normalized during signing.
    pub uri_path_normalization_mode: UriPathNormalizationMode,

    /// Some services require X-Amz-Security-Token to be included in the
    /// canonical request. Other services require only it to be added after
    /// calculating the signature.
    pub session_token_mode: SessionTokenMode,

    /// Some services require an alternative session token header or query param instead of
    /// `x-amz-security-token` or `X-Amz-Security-Token`.
    pub session_token_name_override: Option<&'static str>,
}

/// HTTP payload checksum type
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PayloadChecksumKind {
    /// Add x-amz-checksum-sha256 to the canonical request
    ///
    /// This setting is required for S3
    XAmzSha256,

    /// Do not add an additional header when creating the canonical request
    ///
    /// This is "normal mode" and will work for services other than S3
    NoHeader,
}

/// Config value to specify how to encode the request URL when signing.
///
/// We assume the URI will be encoded _once_ prior to transmission. Some services
/// do not decode the path prior to checking the signature, requiring clients to actually
/// _double-encode_ the URI in creating the canonical request in order to pass a signature check.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PercentEncodingMode {
    /// Re-encode the resulting URL (e.g. %30 becomes `%2530)
    Double,

    /// Take the resulting URL as-is
    Single,
}

/// Config value to specify whether the canonical request's URI path should be normalized.
/// <https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html>
///
/// URI path normalization is performed based on <https://www.rfc-editor.org/rfc/rfc3986>.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UriPathNormalizationMode {
    /// Normalize the URI path according to RFC3986
    Enabled,

    /// Don't normalize the URI path (S3, for example, rejects normalized paths in some instances)
    Disabled,
}

impl From<bool> for UriPathNormalizationMode {
    fn from(value: bool) -> Self {
        if value {
            UriPathNormalizationMode::Enabled
        } else {
            UriPathNormalizationMode::Disabled
        }
    }
}

/// Config value to specify whether X-Amz-Security-Token should be part of the canonical request.
/// <http://docs.aws.amazon.com/general/latest/gr/sigv4-add-signature-to-request.html#temporary-security-credentials>
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionTokenMode {
    /// Include in the canonical request before calculating the signature.
    Include,

    /// Exclude in the canonical request.
    Exclude,
}

impl Default for SigningSettings {
    fn default() -> Self {
        // Headers that are potentially altered by proxies or as a part of standard service operations.
        // Reference:
        // Go SDK: <https://github.com/aws/aws-sdk-go/blob/v1.44.289/aws/signer/v4/v4.go#L92>
        // Java SDK: <https://github.com/aws/aws-sdk-java-v2/blob/master/core/auth/src/main/java/software/amazon/awssdk/auth/signer/internal/AbstractAws4Signer.java#L70>
        // JS SDK: <https://github.com/aws/aws-sdk-js/blob/master/lib/signers/v4.js#L191>
        // There is no single source of truth for these available, so this uses the minimum common set of the excluded options.
        // Instantiate this every time, because SigningSettings takes a Vec (which cannot be const);
        let excluded_headers = Some(
            [
                // This header is calculated as part of the signing process, so if it's present, discard it
                Cow::Borrowed(AUTHORIZATION.as_str()),
                // Changes when sent by proxy
                Cow::Borrowed(USER_AGENT.as_str()),
                // Changes based on the request from the client
                Cow::Borrowed(HEADER_NAME_X_RAY_TRACE_ID),
                // Hop by hop header, can be erased by Cloudfront
                Cow::Borrowed(TRANSFER_ENCODING.as_str()),
            ]
            .to_vec(),
        );
        Self {
            percent_encoding_mode: PercentEncodingMode::Double,
            payload_checksum_kind: PayloadChecksumKind::NoHeader,
            signature_location: SignatureLocation::Headers,
            expires_in: None,
            excluded_headers,
            uri_path_normalization_mode: UriPathNormalizationMode::Enabled,
            session_token_mode: SessionTokenMode::Include,
            session_token_name_override: None,
        }
    }
}

/// Where to place signing values in the HTTP request
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SignatureLocation {
    /// Place the signature in the request headers
    Headers,
    /// Place the signature in the request query parameters
    QueryParams,
}
