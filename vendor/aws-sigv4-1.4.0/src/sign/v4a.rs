/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::identity::Identity;
use bytes::{BufMut, BytesMut};
use crypto_bigint::{CheckedAdd, CheckedSub, Encoding, U256};
use p256::ecdsa::signature::Signer;
use p256::ecdsa::{Signature, SigningKey};
use std::io::Write;
use std::sync::LazyLock;
use std::time::SystemTime;
use zeroize::Zeroizing;

const ALGORITHM: &[u8] = b"AWS4-ECDSA-P256-SHA256";
static BIG_N_MINUS_2: LazyLock<U256> = LazyLock::new(|| {
    // The N value from section 3.2.1.3 of https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186.pdf
    // Used as the N value for the algorithm described in section A.2.2 of https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-5.pdf
    // *(Basically a prime number blessed by the NSA for use in p256)*
    const ORDER: U256 =
        U256::from_be_hex("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");
    ORDER.checked_sub(&U256::from(2u32)).unwrap()
});

/// Calculates a Sigv4a signature
pub fn calculate_signature(signing_key: impl AsRef<[u8]>, string_to_sign: &[u8]) -> String {
    let signing_key = SigningKey::from_bytes(signing_key.as_ref()).unwrap();
    let signature: Signature = signing_key.sign(string_to_sign);
    // This conversion sucks but we have to do it afaict. Because we also use
    // the HMAC crate, we have to use a compatible (and therefore older) version
    // of the p256 crate. That older version requires us to convert between
    // signature types instead of using DER-encoded signatures directly.
    let signature = signature.to_der();
    hex::encode(signature.as_ref())
}

/// Generates a signing key for Sigv4a signing.
pub fn generate_signing_key(access_key: &str, secret_access_key: &str) -> impl AsRef<[u8]> {
    // Capacity is the secret access key length plus the length of "AWS4A"
    let mut input_key = Zeroizing::new(Vec::with_capacity(secret_access_key.len() + 5));
    write!(input_key, "AWS4A{secret_access_key}").unwrap();

    // Capacity is the access key length plus the counter byte
    let mut kdf_context = Zeroizing::new(Vec::with_capacity(access_key.len() + 1));
    let mut counter = Zeroizing::new(1u8);
    let key = loop {
        write!(kdf_context, "{access_key}").unwrap();
        kdf_context.push(*counter);

        let mut fis = ALGORITHM.to_vec();
        fis.push(0);
        fis.append(&mut kdf_context);
        fis.put_i32(256);

        let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, &input_key);

        let mut buf = BytesMut::new();
        buf.put_i32(1);
        buf.put_slice(&fis);
        let tag = ring::hmac::sign(&key, &buf);
        let tag = &tag.as_ref()[0..32];

        let k0 = U256::from_be_bytes(tag.try_into().expect("convert to [u8; 32]"));

        // It would be more secure for this to be a constant time comparison, but because this
        // is for client usage, that's not as big a deal.
        if k0 <= *BIG_N_MINUS_2 {
            let pk = k0
                .checked_add(&U256::ONE)
                .expect("k0 is always less than U256::MAX");
            let d = Zeroizing::new(pk.to_be_bytes());
            break SigningKey::from_bytes(d.as_ref()).unwrap();
        }

        *counter = counter
            .checked_add(1)
            .expect("counter will never get to 255");
    };

    key.to_bytes()
}

/// Parameters to use when signing.
#[derive(Debug)]
#[non_exhaustive]
pub struct SigningParams<'a, S> {
    /// The identity to use when signing a request
    pub(crate) identity: &'a Identity,

    /// Region set to sign for.
    pub(crate) region_set: &'a str,
    /// Service Name to sign for.
    ///
    /// NOTE: Endpoint resolution rules may specify a name that differs from the typical service name.
    pub(crate) name: &'a str,
    /// Timestamp to use in the signature (should be `SystemTime::now()` unless testing).
    pub(crate) time: SystemTime,

    /// Additional signing settings. These differ between HTTP and Event Stream.
    pub(crate) settings: S,
}

pub(crate) const ECDSA_256: &str = "AWS4-ECDSA-P256-SHA256";

impl<S> SigningParams<'_, S> {
    /// Returns the region that will be used to sign SigV4a requests
    pub fn region_set(&self) -> &str {
        self.region_set
    }

    /// Returns the service name that will be used to sign requests
    pub fn name(&self) -> &str {
        self.name
    }

    /// Return the name of the algorithm used to sign requests
    pub fn algorithm(&self) -> &'static str {
        ECDSA_256
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
        region_set: Option<&'a str>,
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
            set_region_set,
            region_set,
            &'a str,
            "Sets the region set (required)",
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
                region_set: self
                    .region_set
                    .ok_or_else(|| BuildError::new("region_set is required"))?,
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
