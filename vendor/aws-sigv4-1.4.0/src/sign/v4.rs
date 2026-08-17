/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::{
    date_time::{format_date, format_date_time},
    http_request::SigningError,
    SigningOutput,
};
use aws_credential_types::Credentials;
use aws_smithy_runtime_api::{client::identity::Identity, http::Headers};
use bytes::Bytes;
use hmac::{digest::FixedOutput, Hmac, Mac};
use sha2::{Digest, Sha256};
use std::time::SystemTime;

/// HashedPayload = Lowercase(HexEncode(Hash(requestPayload)))
#[allow(dead_code)] // Unused when compiling without certain features
pub(crate) fn sha256_hex_string(bytes: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize_fixed())
}

/// Calculates a Sigv4 signature
pub fn calculate_signature(signing_key: impl AsRef<[u8]>, string_to_sign: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(signing_key.as_ref())
        .expect("HMAC can take key of any size");
    mac.update(string_to_sign);
    hex::encode(mac.finalize_fixed())
}

/// Generates a signing key for Sigv4
pub fn generate_signing_key(
    secret: &str,
    time: SystemTime,
    region: &str,
    service: &str,
) -> impl AsRef<[u8]> {
    // kSecret = your secret access key
    // kDate = HMAC("AWS4" + kSecret, Date)
    // kRegion = HMAC(kDate, Region)
    // kService = HMAC(kRegion, Service)
    // kSigning = HMAC(kService, "aws4_request")

    let secret = format!("AWS4{secret}");
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_ref()).expect("HMAC can take key of any size");
    mac.update(format_date(time).as_bytes());
    let tag = mac.finalize_fixed();

    // sign region
    let mut mac = Hmac::<Sha256>::new_from_slice(&tag).expect("HMAC can take key of any size");
    mac.update(region.as_bytes());
    let tag = mac.finalize_fixed();

    // sign service
    let mut mac = Hmac::<Sha256>::new_from_slice(&tag).expect("HMAC can take key of any size");
    mac.update(service.as_bytes());
    let tag = mac.finalize_fixed();

    // sign request
    let mut mac = Hmac::<Sha256>::new_from_slice(&tag).expect("HMAC can take key of any size");
    mac.update("aws4_request".as_bytes());
    mac.finalize_fixed()
}

/// Parameters to use when signing.
#[derive(Debug)]
#[non_exhaustive]
pub struct SigningParams<'a, S> {
    /// The identity to use when signing a request
    pub(crate) identity: &'a Identity,

    /// Region to sign for.
    pub(crate) region: &'a str,
    /// Service Name to sign for.
    ///
    /// NOTE: Endpoint resolution rules may specify a name that differs from the typical service name.
    pub(crate) name: &'a str,
    /// Timestamp to use in the signature (should be `SystemTime::now()` unless testing).
    pub(crate) time: SystemTime,

    /// Additional signing settings. These differ between HTTP and Event Stream.
    pub(crate) settings: S,
}

pub(crate) const HMAC_SHA256: &str = "AWS4-HMAC-SHA256";
const HMAC_SHA256_PAYLOAD: &str = "AWS4-HMAC-SHA256-PAYLOAD";
const HMAC_SHA256_TRAILER: &str = "AWS4-HMAC-SHA256-TRAILER";

impl<S> SigningParams<'_, S> {
    /// Returns the region that will be used to sign SigV4 requests
    pub fn region(&self) -> &str {
        self.region
    }

    /// Returns the signing name that will be used to sign requests
    pub fn name(&self) -> &str {
        self.name
    }

    /// Return the name of the algorithm used to sign requests
    pub fn algorithm(&self) -> &'static str {
        HMAC_SHA256
    }
}

impl<'a, S: Default> SigningParams<'a, S> {
    /// Returns a builder that can create new `SigningParams`.
    pub fn builder() -> signing_params::Builder<'a, S> {
        Default::default()
    }
}

/// Builder and error for creating [`SigningParams`]
pub mod signing_params {
    use super::SigningParams;
    use aws_smithy_runtime_api::client::identity::Identity;
    use std::error::Error;
    use std::fmt;
    use std::time::SystemTime;

    /// [`SigningParams`] builder error
    #[derive(Debug)]
    pub struct BuildError {
        reason: &'static str,
    }
    impl BuildError {
        fn new(reason: &'static str) -> Self {
            Self { reason }
        }
    }

    impl fmt::Display for BuildError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.reason)
        }
    }

    impl Error for BuildError {}

    /// Builder that can create new [`SigningParams`]
    #[derive(Debug, Default)]
    pub struct Builder<'a, S> {
        identity: Option<&'a Identity>,
        region: Option<&'a str>,
        name: Option<&'a str>,
        time: Option<SystemTime>,
        settings: Option<S>,
    }

    impl<'a, S> Builder<'a, S> {
        builder_methods!(
            set_identity,
            identity,
            &'a Identity,
            "Sets the identity (required)",
            set_region,
            region,
            &'a str,
            "Sets the region (required)",
            set_name,
            name,
            &'a str,
            "Sets the name (required)",
            set_time,
            time,
            SystemTime,
            "Sets the time to be used in the signature (required)",
            set_settings,
            settings,
            S,
            "Sets additional signing settings (required)"
        );

        /// Builds an instance of [`SigningParams`]. Will yield a [`BuildError`] if
        /// a required argument was not given.
        pub fn build(self) -> Result<SigningParams<'a, S>, BuildError> {
            Ok(SigningParams {
                identity: self
                    .identity
                    .ok_or_else(|| BuildError::new("identity is required"))?,
                region: self
                    .region
                    .ok_or_else(|| BuildError::new("region is required"))?,
                name: self
                    .name
                    .ok_or_else(|| BuildError::new("name is required"))?,
                time: self
                    .time
                    .ok_or_else(|| BuildError::new("time is required"))?,
                settings: self
                    .settings
                    .ok_or_else(|| BuildError::new("settings are required"))?,
            })
        }
    }
}

/// Signs `chunk` with the given `running_signature` and `params`.
///
/// See [signature calculation details](https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-streaming.html#sigv4-chunked-body-definition).
pub fn sign_chunk<'a, S>(
    chunk: &Bytes,
    running_signature: &'a str,
    params: &'a SigningParams<'a, S>,
) -> Result<SigningOutput<()>, SigningError> {
    let payload_hash = format!("{}\n{}", sha256_hex_string([]), sha256_hex_string(chunk));
    sign_streaming_payload(
        HMAC_SHA256_PAYLOAD,
        running_signature,
        params,
        &payload_hash,
    )
}

/// Signs trailing headers with the given `running_signature` and `params`.
///
/// See [signature calculation details](https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-streaming-trailers.html#example-signature-calculations-trailing-header).
pub fn sign_trailer<'a, S>(
    headers: &'a Headers,
    running_signature: &'a str,
    params: &'a SigningParams<'a, S>,
) -> Result<SigningOutput<()>, SigningError> {
    fn canonical_headers(headers: &Headers) -> Vec<u8> {
        let mut sorted_headers: Vec<_> = headers.iter().collect();
        sorted_headers.sort_by_key(|(name, _)| name.to_lowercase());
        let mut buf = Vec::with_capacity(sorted_headers.len());
        for (name, value) in sorted_headers.iter() {
            buf.extend_from_slice(name.to_lowercase().as_bytes());
            buf.extend_from_slice(b":");
            buf.extend_from_slice(value.trim().as_bytes());
            buf.extend_from_slice(b"\n");
        }
        buf
    }

    let payload_hash = sha256_hex_string(canonical_headers(headers));
    sign_streaming_payload(
        HMAC_SHA256_TRAILER,
        running_signature,
        params,
        &payload_hash,
    )
}

fn sign_streaming_payload<'a, S>(
    algorithm: &str,
    running_signature: &'a str,
    params: &'a SigningParams<'a, S>,
    payload_hash: &str,
) -> Result<SigningOutput<()>, SigningError> {
    let creds = params
        .identity
        .data::<Credentials>()
        .expect("identity must contain credentials");

    let signing_key = generate_signing_key(
        creds.secret_access_key(),
        params.time,
        params.region,
        params.name,
    );

    let scope = format!(
        "{}/{}/{}/aws4_request",
        format_date(params.time),
        params.region,
        params.name
    );

    let string_to_sign = format!(
        "{}\n{}\n{}\n{}\n{}",
        algorithm,
        format_date_time(params.time),
        scope,
        running_signature,
        payload_hash,
    );

    let signature = calculate_signature(signing_key, string_to_sign.as_bytes());
    Ok(SigningOutput::new((), signature))
}

#[cfg(test)]
mod tests {
    use super::{calculate_signature, generate_signing_key, sha256_hex_string};
    use crate::date_time::test_parsers::parse_date_time;

    #[test]
    fn test_signature_calculation() {
        let secret = "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY";
        let creq = r#"AWS4-HMAC-SHA256
20150830T123600Z
20150830/us-east-1/iam/aws4_request
f536975d06c0309214f805bb90ccff089219ecd68b2577efef23edd43b7e1a59"#;
        let time = parse_date_time("20150830T123600Z").unwrap();

        let derived_key = generate_signing_key(secret, time, "us-east-1", "iam");
        let signature = calculate_signature(derived_key, creq.as_bytes());

        let expected = "5d672d79c15b13162d9279b0855cfba6789a8edb4c82c400e06b5924a6f2b5d7";
        assert_eq!(expected, &signature);
    }

    #[test]
    fn sign_payload_empty_string() {
        let expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let actual = sha256_hex_string([]);
        assert_eq!(expected, actual);
    }
}
