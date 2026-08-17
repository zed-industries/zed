//! Implementations of the [`JwtSigner`] and [`JwtVerifier`] traits for the
//! HMAC family of algorithms using `RustCrypto`'s [`hmac`].

use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha384, Sha512};
use signature::{Signer, Verifier};

use crate::crypto::{JwtSigner, JwtVerifier};
use crate::errors::Result;
use crate::{Algorithm, DecodingKey, EncodingKey};

type HmacSha256 = Hmac<Sha256>;
type HmacSha384 = Hmac<Sha384>;
type HmacSha512 = Hmac<Sha512>;

/// Macro to define an HMAC signer for a specific algorithm
macro_rules! define_hmac_signer {
    ($name:ident, $alg:expr, $hmac_type:ty) => {
        pub struct $name($hmac_type);

        impl $name {
            pub(crate) fn new(encoding_key: &EncodingKey) -> Result<Self> {
                let inner = <$hmac_type>::new_from_slice(encoding_key.try_get_hmac_secret()?)
                    .map_err(|_e| crate::errors::ErrorKind::InvalidKeyFormat)?;

                Ok(Self(inner))
            }
        }

        impl Signer<Vec<u8>> for $name {
            fn try_sign(&self, msg: &[u8]) -> std::result::Result<Vec<u8>, signature::Error> {
                let mut signer = self.0.clone();
                signer.reset();
                signer.update(msg);

                Ok(signer.finalize().into_bytes().to_vec())
            }
        }

        impl JwtSigner for $name {
            fn algorithm(&self) -> Algorithm {
                $alg
            }
        }
    };
}

/// Macro to define an HMAC verifier for a specific algorithm
macro_rules! define_hmac_verifier {
    ($name:ident, $alg:expr, $hmac_type:ty) => {
        pub struct $name($hmac_type);

        impl $name {
            pub(crate) fn new(decoding_key: &DecodingKey) -> Result<Self> {
                let inner = <$hmac_type>::new_from_slice(decoding_key.try_get_hmac_secret()?)
                    .map_err(|_e| crate::errors::ErrorKind::InvalidKeyFormat)?;

                Ok(Self(inner))
            }
        }

        impl Verifier<Vec<u8>> for $name {
            fn verify(
                &self,
                msg: &[u8],
                signature: &Vec<u8>,
            ) -> std::result::Result<(), signature::Error> {
                let mut verifier = self.0.clone();
                verifier.reset();
                verifier.update(msg);

                verifier.verify_slice(signature).map_err(signature::Error::from_source)
            }
        }

        impl JwtVerifier for $name {
            fn algorithm(&self) -> Algorithm {
                $alg
            }
        }
    };
}

// Define HMAC signers using the macro
define_hmac_signer!(Hs256Signer, Algorithm::HS256, HmacSha256);
define_hmac_signer!(Hs384Signer, Algorithm::HS384, HmacSha384);
define_hmac_signer!(Hs512Signer, Algorithm::HS512, HmacSha512);

// Define HMAC verifiers using the macro
define_hmac_verifier!(Hs256Verifier, Algorithm::HS256, HmacSha256);
define_hmac_verifier!(Hs384Verifier, Algorithm::HS384, HmacSha384);
define_hmac_verifier!(Hs512Verifier, Algorithm::HS512, HmacSha512);
