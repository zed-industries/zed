//! Implementations of the [`JwtSigner`] and [`JwtVerifier`] traits for the
//! HMAC family of algorithms using [`aws_lc_rs`]

use aws_lc_rs::hmac;
use signature::{Signer, Verifier};

use crate::crypto::{JwtSigner, JwtVerifier};
use crate::errors::Result;
use crate::{Algorithm, DecodingKey, EncodingKey};

macro_rules! define_hmac_signer {
    ($name:ident, $alg:expr, $hmac_alg:expr) => {
        pub struct $name(hmac::Key);

        impl $name {
            pub(crate) fn new(encoding_key: &EncodingKey) -> Result<Self> {
                Ok(Self(hmac::Key::new($hmac_alg, encoding_key.try_get_hmac_secret()?)))
            }
        }

        impl Signer<Vec<u8>> for $name {
            fn try_sign(&self, msg: &[u8]) -> std::result::Result<Vec<u8>, signature::Error> {
                Ok(hmac::sign(&self.0, msg).as_ref().to_vec())
            }
        }

        impl JwtSigner for $name {
            fn algorithm(&self) -> Algorithm {
                $alg
            }
        }
    };
}

macro_rules! define_hmac_verifier {
    ($name:ident, $alg:expr, $hmac_alg:expr) => {
        pub struct $name(hmac::Key);

        impl $name {
            pub(crate) fn new(decoding_key: &DecodingKey) -> Result<Self> {
                Ok(Self(hmac::Key::new($hmac_alg, decoding_key.try_get_hmac_secret()?)))
            }
        }

        impl Verifier<Vec<u8>> for $name {
            fn verify(
                &self,
                msg: &[u8],
                signature: &Vec<u8>,
            ) -> std::result::Result<(), signature::Error> {
                hmac::verify(&self.0, msg, signature).map_err(signature::Error::from_source)
            }
        }

        impl JwtVerifier for $name {
            fn algorithm(&self) -> Algorithm {
                $alg
            }
        }
    };
}

define_hmac_signer!(Hs256Signer, Algorithm::HS256, hmac::HMAC_SHA256);
define_hmac_signer!(Hs384Signer, Algorithm::HS384, hmac::HMAC_SHA384);
define_hmac_signer!(Hs512Signer, Algorithm::HS512, hmac::HMAC_SHA512);

define_hmac_verifier!(Hs256Verifier, Algorithm::HS256, hmac::HMAC_SHA256);
define_hmac_verifier!(Hs384Verifier, Algorithm::HS384, hmac::HMAC_SHA384);
define_hmac_verifier!(Hs512Verifier, Algorithm::HS512, hmac::HMAC_SHA512);
