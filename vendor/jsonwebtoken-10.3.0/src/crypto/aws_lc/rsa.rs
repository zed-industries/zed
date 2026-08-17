//! Implementations of the [`JwtSigner`] and [`JwtVerifier`] traits for the
//! RSA family of algorithms using [`aws_lc_rs`]

use aws_lc_rs::{rand, signature as crypto_sig};
use signature::{Signer, Verifier};

use crate::algorithms::AlgorithmFamily;
use crate::crypto::{JwtSigner, JwtVerifier};
use crate::decoding::DecodingKeyKind;
use crate::errors::{ErrorKind, Result, new_error};
use crate::{Algorithm, DecodingKey, EncodingKey};

/// Try to sign the `message` using an `RSA` `algorithm`.
fn try_sign_rsa(
    algorithm: &'static dyn crypto_sig::RsaEncoding,
    encoding_key: &EncodingKey,
    msg: &[u8],
) -> std::result::Result<Vec<u8>, signature::Error> {
    let key_pair = crypto_sig::RsaKeyPair::from_der(encoding_key.inner())
        .map_err(signature::Error::from_source)?;

    let mut signature = vec![0; key_pair.public_modulus_len()];
    let rng = rand::SystemRandom::new();
    key_pair.sign(algorithm, &rng, msg, &mut signature).map_err(signature::Error::from_source)?;

    Ok(signature)
}

/// Return a `aws_lc_rs` RSA public key from a [`DecodingKey`]
///
/// # Errors
///
/// - If `decoding_key` is not from the RSA family.
fn verify_rsa(
    algorithm: &'static crypto_sig::RsaParameters,
    decoding_key: &DecodingKey,
    msg: &[u8],
    signature: &[u8],
) -> std::result::Result<(), signature::Error> {
    match decoding_key.kind() {
        DecodingKeyKind::SecretOrDer(bytes) => {
            let public_key = crypto_sig::UnparsedPublicKey::new(algorithm, bytes);
            public_key.verify(msg, signature).map_err(signature::Error::from_source)?;
        }
        DecodingKeyKind::RsaModulusExponent { n, e } => {
            let public_key = crypto_sig::RsaPublicKeyComponents { n, e };
            public_key.verify(algorithm, msg, signature).map_err(signature::Error::from_source)?;
        }
    };

    Ok(())
}

macro_rules! define_rsa_signer {
    ($name:ident, $alg:expr, $signing_alg:expr) => {
        pub struct $name(EncodingKey);

        impl $name {
            pub(crate) fn new(encoding_key: &EncodingKey) -> Result<Self> {
                if encoding_key.family() != AlgorithmFamily::Rsa {
                    return Err(new_error(ErrorKind::InvalidKeyFormat));
                }

                Ok(Self(encoding_key.clone()))
            }
        }

        impl Signer<Vec<u8>> for $name {
            fn try_sign(&self, msg: &[u8]) -> std::result::Result<Vec<u8>, signature::Error> {
                try_sign_rsa($signing_alg, &self.0, msg)
            }
        }

        impl JwtSigner for $name {
            fn algorithm(&self) -> Algorithm {
                $alg
            }
        }
    };
}

macro_rules! define_rsa_verifier {
    ($name:ident, $alg:expr, $verification_alg:expr) => {
        pub struct $name(DecodingKey);

        impl $name {
            pub(crate) fn new(decoding_key: &DecodingKey) -> Result<Self> {
                if decoding_key.family() != AlgorithmFamily::Rsa {
                    return Err(new_error(ErrorKind::InvalidKeyFormat));
                }

                Ok(Self(decoding_key.clone()))
            }
        }

        impl Verifier<Vec<u8>> for $name {
            fn verify(
                &self,
                msg: &[u8],
                signature: &Vec<u8>,
            ) -> std::result::Result<(), signature::Error> {
                verify_rsa($verification_alg, &self.0, msg, signature)
            }
        }

        impl JwtVerifier for $name {
            fn algorithm(&self) -> Algorithm {
                $alg
            }
        }
    };
}

define_rsa_signer!(Rsa256Signer, Algorithm::RS256, &crypto_sig::RSA_PKCS1_SHA256);
define_rsa_signer!(Rsa384Signer, Algorithm::RS384, &crypto_sig::RSA_PKCS1_SHA384);
define_rsa_signer!(Rsa512Signer, Algorithm::RS512, &crypto_sig::RSA_PKCS1_SHA512);
define_rsa_signer!(RsaPss256Signer, Algorithm::PS256, &crypto_sig::RSA_PSS_SHA256);
define_rsa_signer!(RsaPss384Signer, Algorithm::PS384, &crypto_sig::RSA_PSS_SHA384);
define_rsa_signer!(RsaPss512Signer, Algorithm::PS512, &crypto_sig::RSA_PSS_SHA512);

define_rsa_verifier!(Rsa256Verifier, Algorithm::RS256, &crypto_sig::RSA_PKCS1_2048_8192_SHA256);
define_rsa_verifier!(Rsa384Verifier, Algorithm::RS384, &crypto_sig::RSA_PKCS1_2048_8192_SHA384);
define_rsa_verifier!(Rsa512Verifier, Algorithm::RS512, &crypto_sig::RSA_PKCS1_2048_8192_SHA512);
define_rsa_verifier!(RsaPss256Verifier, Algorithm::PS256, &crypto_sig::RSA_PSS_2048_8192_SHA256);
define_rsa_verifier!(RsaPss384Verifier, Algorithm::PS384, &crypto_sig::RSA_PSS_2048_8192_SHA384);
define_rsa_verifier!(RsaPss512Verifier, Algorithm::PS512, &crypto_sig::RSA_PSS_2048_8192_SHA512);
