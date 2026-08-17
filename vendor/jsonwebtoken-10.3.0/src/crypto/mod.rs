//! The cryptography of the `jsonwebtoken` crate is decoupled behind
//! [`JwtSigner`] and [`JwtVerifier`] traits. These make use of `signature`'s
//! [`Signer`] and [`Verifier`] traits respectively.
//! Crypto provider selection is handled by [`CryptoProvider`].
//!
//! [`JwtSigner`]: crate::crypto::JwtSigner
//! [`JwtVerifier`]: crate::crypto::JwtVerifier
//! [`Signer`]: signature::Signer
//! [`Verifier`]: signature::Verifier
//! [`CryptoProvider`]: crate::crypto::CryptoProvider

use crate::algorithms::Algorithm;
use crate::errors::Result;
use crate::jwk::{EllipticCurve, ThumbprintHash};
use crate::{DecodingKey, EncodingKey};

/// `aws_lc_rs` based CryptoProvider.
#[cfg(feature = "aws_lc_rs")]
pub mod aws_lc;

/// `RustCrypto` based CryptoProvider.
#[cfg(feature = "rust_crypto")]
pub mod rust_crypto;

use crate::serialization::{b64_decode, b64_encode};
use signature::{Signer, Verifier};

/// Trait providing the functionality to sign a JWT.
///
/// Allows an arbitrary crypto backend to be provided.
pub trait JwtSigner: Signer<Vec<u8>> {
    /// Return the [`Algorithm`] corresponding to the signing module.
    fn algorithm(&self) -> Algorithm;
}

/// Trait providing the functionality to verify a JWT.
///
/// Allows an arbitrary crypto backend to be provided.
pub trait JwtVerifier: Verifier<Vec<u8>> {
    /// Return the [`Algorithm`] corresponding to the signing module.
    fn algorithm(&self) -> Algorithm;
}

/// Take the payload of a JWT, sign it using the algorithm given and return
/// the base64 url safe encoded of the result.
///
/// If you just want to encode a JWT, use `encode` instead.
pub fn sign(message: &[u8], key: &EncodingKey, algorithm: Algorithm) -> Result<String> {
    let provider = (CryptoProvider::get_default().signer_factory)(&algorithm, key)?;
    Ok(b64_encode(provider.try_sign(message)?))
}

/// Compares the signature given with a re-computed signature for HMAC or using the public key
/// for RSA/EC.
///
/// If you just want to decode a JWT, use `decode` instead.
///
/// `signature` is the signature part of a jwt (text after the second '.')
///
/// `message` is base64(header) + "." + base64(claims)
pub fn verify(
    signature: &str,
    message: &[u8],
    key: &DecodingKey,
    algorithm: Algorithm,
) -> Result<bool> {
    let provider = (CryptoProvider::get_default().verifier_factory)(&algorithm, key)?;
    Ok(provider.verify(message, &b64_decode(signature)?).is_ok())
}

/// Controls the cryptography used by jsonwebtoken.
///
/// You can either install one of the built-in options:
/// - [`crypto::aws_lc::DEFAULT_PROVIDER`]: (behind the `aws_lc_rs` crate feature).
///   This provider uses the [aws-lc-rs](https://github.com/aws/aws-lc-rs) crate.
/// - [`crypto::rust_crypto::DEFAULT_PROVIDER`]: (behind the `rust_crypto` crate feature)
///   This provider uses crates from the [Rust Crypto](https://github.com/RustCrypto) project.
///
/// or provide your own custom custom implementation of `CryptoProvider`.
// This implementation appropriates a good chunk of code from the `rustls` CryptoProvider,
// and is very much inspired by it.
#[derive(Clone, Debug)]
pub struct CryptoProvider {
    /// A function that produces a [`JwtSigner`] for a given [`Algorithm`]
    pub signer_factory: fn(&Algorithm, &EncodingKey) -> Result<Box<dyn JwtSigner>>,
    /// A function that produces a [`JwtVerifier`] for a given [`Algorithm`]
    pub verifier_factory: fn(&Algorithm, &DecodingKey) -> Result<Box<dyn JwtVerifier>>,
    /// Struct with utility functions for JWK processing.
    pub jwk_utils: JwkUtils,
}

impl CryptoProvider {
    /// Set this `CryptoProvider` as the default for this process.
    ///
    /// This can be called successfully at most once in any process execution.
    pub fn install_default(&'static self) -> std::result::Result<(), &'static Self> {
        static_default::install_default(self)
    }

    pub(crate) fn get_default() -> &'static Self {
        static_default::get_default()
    }

    fn from_crate_features() -> &'static Self {
        #[cfg(all(feature = "rust_crypto", not(feature = "aws_lc_rs")))]
        {
            return &rust_crypto::DEFAULT_PROVIDER;
        }

        #[cfg(all(feature = "aws_lc_rs", not(feature = "rust_crypto")))]
        {
            return &aws_lc::DEFAULT_PROVIDER;
        }

        #[allow(unreachable_code)]
        {
            const NOT_INSTALLED_ERROR: &str = r###"
Could not automatically determine the process-level CryptoProvider from jsonwebtoken crate features.
Call CryptoProvider::install_default() before this point to select a provider manually, or make sure exactly one of the 'rust_crypto' and 'aws_lc_rs' features is enabled.
See the documentation of the CryptoProvider type for more information.
"###;

            static INSTANCE: CryptoProvider = CryptoProvider {
                signer_factory: |_, _| panic!("{}", NOT_INSTALLED_ERROR),
                verifier_factory: |_, _| panic!("{}", NOT_INSTALLED_ERROR),
                jwk_utils: JwkUtils::new_unimplemented(),
            };

            &INSTANCE
        }
    }
}

/// Holds utility functions required for JWK processing.
/// Use the [`JwkUtils::new_unimplemented`] function to initialize all values to dummies.
#[derive(Clone, Debug)]
pub struct JwkUtils {
    /// Given a DER encoded private key, extract the RSA public key components (n, e)
    #[allow(clippy::type_complexity)]
    pub extract_rsa_public_key_components: fn(&[u8]) -> Result<(Vec<u8>, Vec<u8>)>,
    /// Given a DER encoded private key and an algorithm, extract the associated curve
    /// and the EC public key components (x, y)
    #[allow(clippy::type_complexity)]
    pub extract_ec_public_key_coordinates:
        fn(&[u8], Algorithm) -> Result<(EllipticCurve, Vec<u8>, Vec<u8>)>,
    /// Given some data and a name of a hash function, compute hash_function(data)
    pub compute_digest: fn(&[u8], ThumbprintHash) -> Vec<u8>,
}

impl JwkUtils {
    /// Initialises all values to dummies.
    /// Will lead to a panic when JWKs are required, so only use it if you don't want to support JWKs.
    pub const fn new_unimplemented() -> Self {
        const NOT_INSTALLED_OR_UNIMPLEMENTED_ERROR: &str = r###"
Could not automatically determine the process-level CryptoProvider from jsonwebtoken crate features, or your CryptoProvider does not support JWKs.
Call CryptoProvider::install_default() before this point to select a provider manually, or make sure exactly one of the 'rust_crypto' and 'aws_lc_rs' features is enabled.
See the documentation of the CryptoProvider type for more information.
"###;
        Self {
            extract_rsa_public_key_components: |_| {
                panic!("{}", NOT_INSTALLED_OR_UNIMPLEMENTED_ERROR)
            },
            extract_ec_public_key_coordinates: |_, _| {
                panic!("{}", NOT_INSTALLED_OR_UNIMPLEMENTED_ERROR)
            },
            compute_digest: |_, _| panic!("{}", NOT_INSTALLED_OR_UNIMPLEMENTED_ERROR),
        }
    }
}

mod static_default {
    use std::sync::OnceLock;

    use super::CryptoProvider;

    static PROCESS_DEFAULT_PROVIDER: OnceLock<&'static CryptoProvider> = OnceLock::new();

    pub(crate) fn install_default(
        default_provider: &'static CryptoProvider,
    ) -> Result<(), &'static CryptoProvider> {
        PROCESS_DEFAULT_PROVIDER.set(default_provider)
    }

    pub(crate) fn get_default() -> &'static CryptoProvider {
        PROCESS_DEFAULT_PROVIDER.get_or_init(CryptoProvider::from_crate_features)
    }
}
