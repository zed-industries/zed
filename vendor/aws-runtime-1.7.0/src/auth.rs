/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sigv4::http_request::{
    PayloadChecksumKind, PercentEncodingMode, SessionTokenMode, SignableBody, SignatureLocation,
    SigningInstructions, SigningSettings, UriPathNormalizationMode,
};
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::auth::AuthSchemeEndpointConfig;
use aws_smithy_runtime_api::client::identity::Identity;
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
use aws_smithy_types::config_bag::{ConfigBag, FrozenLayer, Layer, Storable, StoreReplace};
use aws_smithy_types::Document;
use aws_types::region::{Region, SigningRegion, SigningRegionSet};
use aws_types::SigningName;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;
use std::time::Duration;

/// Auth implementations for SigV4.
pub mod sigv4;

#[cfg(feature = "sigv4a")]
/// Auth implementations for SigV4a.
pub mod sigv4a;

/// Type of SigV4 signature.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum HttpSignatureType {
    /// A signature for a full http request should be computed, with header updates applied to the signing result.
    HttpRequestHeaders,

    /// A signature for a full http request should be computed, with query param updates applied to the signing result.
    ///
    /// This is typically used for presigned URLs.
    HttpRequestQueryParams,
}

/// Signing options for SigV4.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct SigningOptions {
    /// Apply URI encoding twice.
    pub double_uri_encode: bool,
    /// Apply a SHA-256 payload checksum.
    pub content_sha256_header: bool,
    /// Normalize the URI path before signing.
    pub normalize_uri_path: bool,
    /// Omit the session token from the signature.
    pub omit_session_token: bool,
    /// Optional override for the payload to be used in signing.
    pub payload_override: Option<SignableBody<'static>>,
    /// Signature type.
    pub signature_type: HttpSignatureType,
    /// Whether or not the signature is optional.
    pub signing_optional: bool,
    /// Optional expiration (for presigning)
    pub expires_in: Option<Duration>,
}

impl Default for SigningOptions {
    fn default() -> Self {
        Self {
            double_uri_encode: true,
            content_sha256_header: false,
            normalize_uri_path: true,
            omit_session_token: false,
            payload_override: None,
            signature_type: HttpSignatureType::HttpRequestHeaders,
            signing_optional: false,
            expires_in: None,
        }
    }
}

pub(crate) type SessionTokenNameOverrideFn = Box<
    dyn Fn(&SigningSettings, &ConfigBag) -> Result<Option<&'static str>, BoxError>
        + Send
        + Sync
        + 'static,
>;

/// Custom config that provides the alternative session token name for [`SigningSettings`]
pub struct SigV4SessionTokenNameOverride {
    name_override: SessionTokenNameOverrideFn,
}

impl SigV4SessionTokenNameOverride {
    /// Creates a new `SigV4SessionTokenNameOverride`
    pub fn new<F>(name_override: F) -> Self
    where
        F: Fn(&SigningSettings, &ConfigBag) -> Result<Option<&'static str>, BoxError>
            + Send
            + Sync
            + 'static,
    {
        Self {
            name_override: Box::new(name_override),
        }
    }

    /// Provides a session token name override
    pub fn name_override(
        &self,
        settings: &SigningSettings,
        config_bag: &ConfigBag,
    ) -> Result<Option<&'static str>, BoxError> {
        (self.name_override)(settings, config_bag)
    }
}

impl fmt::Debug for SigV4SessionTokenNameOverride {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionTokenNameOverride").finish()
    }
}

impl Storable for SigV4SessionTokenNameOverride {
    type Storer = StoreReplace<Self>;
}

/// SigV4 signing configuration for an operation
///
/// Although these fields MAY be customized on a per request basis, they are generally static
/// for a given operation
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SigV4OperationSigningConfig {
    /// AWS region to sign for.
    ///
    /// For an up-to-date list of AWS regions, see <https://docs.aws.amazon.com/general/latest/gr/rande.html>
    pub region: Option<SigningRegion>,
    /// AWS region set to sign for.
    ///
    /// A comma-separated list of AWS regions. Examples include typical AWS regions as well as 'wildcard' regions
    pub region_set: Option<SigningRegionSet>,
    /// AWS service to sign for.
    pub name: Option<SigningName>,
    /// Signing options.
    pub signing_options: SigningOptions,
}

// TODO(AuthRefactoring): Consider implementing a dedicated struct, similar to `MergeTimeoutConfig`, that allows
// us to implement a custom merge logic for `impl Store`, enabling fold-style merging of `SigV4OperationSigningConfig`.
impl Storable for SigV4OperationSigningConfig {
    type Storer = StoreReplace<Self>;
}

fn settings(operation_config: &SigV4OperationSigningConfig) -> SigningSettings {
    let mut settings = SigningSettings::default();
    settings.percent_encoding_mode = if operation_config.signing_options.double_uri_encode {
        PercentEncodingMode::Double
    } else {
        PercentEncodingMode::Single
    };
    settings.payload_checksum_kind = if operation_config.signing_options.content_sha256_header {
        PayloadChecksumKind::XAmzSha256
    } else {
        PayloadChecksumKind::NoHeader
    };
    settings.uri_path_normalization_mode = if operation_config.signing_options.normalize_uri_path {
        UriPathNormalizationMode::Enabled
    } else {
        UriPathNormalizationMode::Disabled
    };
    settings.session_token_mode = if operation_config.signing_options.omit_session_token {
        SessionTokenMode::Exclude
    } else {
        SessionTokenMode::Include
    };
    settings.signature_location = match operation_config.signing_options.signature_type {
        HttpSignatureType::HttpRequestHeaders => SignatureLocation::Headers,
        HttpSignatureType::HttpRequestQueryParams => SignatureLocation::QueryParams,
    };
    settings.expires_in = operation_config.signing_options.expires_in;
    settings
}

#[derive(Debug)]
enum SigV4SigningError {
    MissingOperationSigningConfig,
    MissingSigningRegion,
    #[cfg(feature = "sigv4a")]
    MissingSigningRegionSet,
    MissingSigningName,
    WrongIdentityType(Identity),
    BadTypeInEndpointAuthSchemeConfig(&'static str),
}

impl fmt::Display for SigV4SigningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SigV4SigningError::*;
        let mut w = |s| f.write_str(s);
        match self {
            MissingOperationSigningConfig => w("missing operation signing config"),
            MissingSigningRegion => w("missing signing region"),
            #[cfg(feature = "sigv4a")]
            MissingSigningRegionSet => w("missing signing region set"),
            MissingSigningName => w("missing signing name"),
            WrongIdentityType(identity) => {
                write!(f, "wrong identity type for SigV4/sigV4a. Expected AWS credentials but got `{identity:?}`")
            }
            BadTypeInEndpointAuthSchemeConfig(field_name) => {
                write!(
                    f,
                    "unexpected type for `{field_name}` in endpoint auth scheme config",
                )
            }
        }
    }
}

impl StdError for SigV4SigningError {}

fn extract_endpoint_auth_scheme_signing_name(
    endpoint_config: &AuthSchemeEndpointConfig<'_>,
) -> Result<Option<SigningName>, SigV4SigningError> {
    match extract_field_from_endpoint_config("signingName", endpoint_config) {
        Some(Document::String(s)) => Ok(Some(SigningName::from(s.to_string()))),
        None => Ok(None),
        _ => Err(SigV4SigningError::BadTypeInEndpointAuthSchemeConfig(
            "signingName",
        )),
    }
}

fn extract_endpoint_auth_scheme_signing_region(
    endpoint_config: &AuthSchemeEndpointConfig<'_>,
) -> Result<Option<SigningRegion>, SigV4SigningError> {
    match extract_field_from_endpoint_config("signingRegion", endpoint_config) {
        Some(Document::String(s)) => Ok(Some(SigningRegion::from(Region::new(s.clone())))),
        None => Ok(None),
        _ => Err(SigV4SigningError::BadTypeInEndpointAuthSchemeConfig(
            "signingRegion",
        )),
    }
}

// Extract `SigningOptions` from the given `AuthSchemeEndpointConfig`
//
// The implementation may need to be extended in the future to support additional fields
// of the `authSchemes` endpoint list property.
fn extract_endpoint_auth_scheme_signing_options<'a>(
    endpoint_config: &AuthSchemeEndpointConfig<'_>,
    signing_options: &'a SigningOptions,
) -> Result<Cow<'a, SigningOptions>, SigV4SigningError> {
    let double_uri_encode =
        match extract_field_from_endpoint_config("disableDoubleEncoding", endpoint_config) {
            Some(Document::Bool(b)) => Some(!b),
            None => None,
            _ => {
                return Err(SigV4SigningError::BadTypeInEndpointAuthSchemeConfig(
                    "disableDoubleEncoding",
                ))
            }
        };
    let normalize_uri_path =
        match extract_field_from_endpoint_config("disableNormalizePath", endpoint_config) {
            Some(Document::Bool(b)) => Some(!b),
            None => None,
            _ => {
                return Err(SigV4SigningError::BadTypeInEndpointAuthSchemeConfig(
                    "disableNormalizePath",
                ))
            }
        };
    match (double_uri_encode, normalize_uri_path) {
        (None, None) => Ok(Cow::Borrowed(signing_options)),
        (double_uri_encode, normalize_uri_path) => {
            let mut signing_options = signing_options.clone();
            signing_options.double_uri_encode =
                double_uri_encode.unwrap_or(signing_options.double_uri_encode);
            signing_options.normalize_uri_path =
                normalize_uri_path.unwrap_or(signing_options.normalize_uri_path);
            Ok(Cow::Owned(signing_options))
        }
    }
}

fn extract_field_from_endpoint_config<'a>(
    field_name: &'static str,
    endpoint_config: &'a AuthSchemeEndpointConfig<'_>,
) -> Option<&'a Document> {
    endpoint_config
        .as_document()
        .and_then(Document::as_object)
        .and_then(|config| config.get(field_name))
}

fn apply_signing_instructions(
    instructions: SigningInstructions,
    request: &mut HttpRequest,
) -> Result<(), BoxError> {
    let (new_headers, new_query) = instructions.into_parts();
    for header in new_headers.into_iter() {
        let mut value = http_1x::HeaderValue::from_str(header.value()).unwrap();
        value.set_sensitive(header.sensitive());
        request.headers_mut().insert(header.name(), value);
    }

    if !new_query.is_empty() {
        let mut query = aws_smithy_http::query_writer::QueryWriter::new_from_string(request.uri())?;
        for (name, value) in new_query {
            query.insert(name, &value);
        }
        request.set_uri(query.build_uri())?;
    }
    Ok(())
}

/// When present in the config bag, this type will signal that the default
/// payload signing should be overridden.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum PayloadSigningOverride {
    /// An unsigned payload
    ///
    /// UnsignedPayload is used for streaming requests where the contents of the body cannot be
    /// known prior to signing
    UnsignedPayload,

    /// A precomputed body checksum. The checksum should be a SHA256 checksum of the body,
    /// lowercase hex encoded. Eg:
    /// `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
    Precomputed(String),

    /// Set when a streaming body has checksum trailers.
    StreamingUnsignedPayloadTrailer,

    /// Set when a signed streaming body has checksum trailers
    StreamingSignedPayloadTrailer,
}

impl PayloadSigningOverride {
    /// Create a payload signing override that will prevent the payload from
    /// being signed.
    pub fn unsigned_payload() -> Self {
        Self::UnsignedPayload
    }

    /// Convert this type into the type used by the signer to determine how a
    /// request body should be signed.
    pub fn to_signable_body(self) -> SignableBody<'static> {
        match self {
            Self::UnsignedPayload => SignableBody::UnsignedPayload,
            Self::Precomputed(checksum) => SignableBody::Precomputed(checksum),
            Self::StreamingUnsignedPayloadTrailer => SignableBody::StreamingUnsignedPayloadTrailer,
            Self::StreamingSignedPayloadTrailer => SignableBody::StreamingSignedPayloadTrailer,
        }
    }
}

impl Storable for PayloadSigningOverride {
    type Storer = StoreReplace<Self>;
}

/// A runtime plugin that, when set, will override how the signer signs request payloads.
#[derive(Debug)]
pub struct PayloadSigningOverrideRuntimePlugin {
    inner: FrozenLayer,
}

impl PayloadSigningOverrideRuntimePlugin {
    /// Create a new runtime plugin that will force the signer to skip signing
    /// the request payload when signing an HTTP request.
    pub fn unsigned() -> Self {
        let mut layer = Layer::new("PayloadSigningOverrideRuntimePlugin");
        layer.store_put(PayloadSigningOverride::UnsignedPayload);

        Self {
            inner: layer.freeze(),
        }
    }
}

impl RuntimePlugin for PayloadSigningOverrideRuntimePlugin {
    fn config(&self) -> Option<FrozenLayer> {
        Some(self.inner.clone())
    }
}
