//! Implementations of the [`JwtSigner`] and [`JwtVerifier`] traits for EdDSA using RustCrypto.

use crate::algorithms::AlgorithmFamily;
use crate::crypto::{JwtSigner, JwtVerifier};
use crate::errors::{ErrorKind, Result, new_error};
use crate::{Algorithm, DecodingKey, EncodingKey};
use ed25519_dalek::pkcs8::DecodePrivateKey;
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use signature::{Error, Signer, Verifier};

pub struct EdDSASigner(SigningKey);

impl EdDSASigner {
    pub(crate) fn new(encoding_key: &EncodingKey) -> Result<Self> {
        if encoding_key.family() != AlgorithmFamily::Ed {
            return Err(new_error(ErrorKind::InvalidKeyFormat));
        }

        Ok(Self(
            SigningKey::from_pkcs8_der(encoding_key.inner())
                .map_err(|_| ErrorKind::InvalidEddsaKey)?,
        ))
    }
}

impl Signer<Vec<u8>> for EdDSASigner {
    fn try_sign(&self, msg: &[u8]) -> std::result::Result<Vec<u8>, Error> {
        Ok(self.0.sign(msg).to_bytes().to_vec())
    }
}

impl JwtSigner for EdDSASigner {
    fn algorithm(&self) -> Algorithm {
        Algorithm::EdDSA
    }
}

pub struct EdDSAVerifier(VerifyingKey);

impl EdDSAVerifier {
    pub(crate) fn new(decoding_key: &DecodingKey) -> Result<Self> {
        if decoding_key.family() != AlgorithmFamily::Ed {
            return Err(new_error(ErrorKind::InvalidKeyFormat));
        }

        Ok(Self(
            VerifyingKey::from_bytes(
                <&[u8; 32]>::try_from(&decoding_key.as_bytes()[..32])
                    .map_err(|_| ErrorKind::InvalidEddsaKey)?,
            )
            .map_err(|_| ErrorKind::InvalidEddsaKey)?,
        ))
    }
}

impl Verifier<Vec<u8>> for EdDSAVerifier {
    fn verify(&self, msg: &[u8], signature: &Vec<u8>) -> std::result::Result<(), Error> {
        self.0.verify(msg, &Signature::from_slice(signature)?)?;
        Ok(())
    }
}

impl JwtVerifier for EdDSAVerifier {
    fn algorithm(&self) -> Algorithm {
        Algorithm::EdDSA
    }
}
