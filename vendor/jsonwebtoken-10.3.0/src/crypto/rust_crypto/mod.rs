use ::rsa::{RsaPrivateKey, pkcs1::DecodeRsaPrivateKey, traits::PublicKeyParts};
use p256::{ecdsa::SigningKey as P256SigningKey, pkcs8::DecodePrivateKey};
use p384::ecdsa::SigningKey as P384SigningKey;
use sha2::{Digest, Sha256, Sha384, Sha512};

use crate::{
    Algorithm, DecodingKey, EncodingKey,
    crypto::{CryptoProvider, JwkUtils, JwtSigner, JwtVerifier},
    errors::{self, Error, ErrorKind},
    jwk::{EllipticCurve, ThumbprintHash},
};

mod ecdsa;
mod eddsa;
mod hmac;
mod rsa;

fn extract_rsa_public_key_components(key_content: &[u8]) -> errors::Result<(Vec<u8>, Vec<u8>)> {
    let private_key = RsaPrivateKey::from_pkcs1_der(key_content)
        .map_err(|e| ErrorKind::InvalidRsaKey(e.to_string()))?;
    let public_key = private_key.to_public_key();
    Ok((public_key.n().to_bytes_be(), public_key.e().to_bytes_be()))
}

fn extract_ec_public_key_coordinates(
    key_content: &[u8],
    alg: Algorithm,
) -> errors::Result<(EllipticCurve, Vec<u8>, Vec<u8>)> {
    match alg {
        Algorithm::ES256 => {
            let signing_key = P256SigningKey::from_pkcs8_der(key_content)
                .map_err(|_| ErrorKind::InvalidEcdsaKey)?;
            let public_key = signing_key.verifying_key();
            let encoded = public_key.to_encoded_point(false);
            match encoded.coordinates() {
                p256::elliptic_curve::sec1::Coordinates::Uncompressed { x, y } => {
                    Ok((EllipticCurve::P256, x.to_vec(), y.to_vec()))
                }
                _ => Err(ErrorKind::InvalidEcdsaKey.into()),
            }
        }
        Algorithm::ES384 => {
            let signing_key = P384SigningKey::from_pkcs8_der(key_content)
                .map_err(|_| ErrorKind::InvalidEcdsaKey)?;
            let public_key = signing_key.verifying_key();
            let encoded = public_key.to_encoded_point(false);
            match encoded.coordinates() {
                p384::elliptic_curve::sec1::Coordinates::Uncompressed { x, y } => {
                    Ok((EllipticCurve::P384, x.to_vec(), y.to_vec()))
                }
                _ => Err(ErrorKind::InvalidEcdsaKey.into()),
            }
        }
        _ => Err(ErrorKind::InvalidEcdsaKey.into()),
    }
}

fn compute_digest(data: &[u8], hash_function: ThumbprintHash) -> Vec<u8> {
    match hash_function {
        ThumbprintHash::SHA256 => Sha256::digest(data).to_vec(),
        ThumbprintHash::SHA384 => Sha384::digest(data).to_vec(),
        ThumbprintHash::SHA512 => Sha512::digest(data).to_vec(),
    }
}

fn new_signer(algorithm: &Algorithm, key: &EncodingKey) -> Result<Box<dyn JwtSigner>, Error> {
    let jwt_signer = match algorithm {
        Algorithm::HS256 => Box::new(hmac::Hs256Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::HS384 => Box::new(hmac::Hs384Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::HS512 => Box::new(hmac::Hs512Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::ES256 => Box::new(ecdsa::Es256Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::ES384 => Box::new(ecdsa::Es384Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::RS256 => Box::new(rsa::Rsa256Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::RS384 => Box::new(rsa::Rsa384Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::RS512 => Box::new(rsa::Rsa512Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::PS256 => Box::new(rsa::RsaPss256Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::PS384 => Box::new(rsa::RsaPss384Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::PS512 => Box::new(rsa::RsaPss512Signer::new(key)?) as Box<dyn JwtSigner>,
        Algorithm::EdDSA => Box::new(eddsa::EdDSASigner::new(key)?) as Box<dyn JwtSigner>,
    };

    Ok(jwt_signer)
}

fn new_verifier(
    algorithm: &Algorithm,
    key: &DecodingKey,
) -> Result<Box<dyn super::JwtVerifier>, Error> {
    let jwt_verifier = match algorithm {
        Algorithm::HS256 => Box::new(hmac::Hs256Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::HS384 => Box::new(hmac::Hs384Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::HS512 => Box::new(hmac::Hs512Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::ES256 => Box::new(ecdsa::Es256Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::ES384 => Box::new(ecdsa::Es384Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::RS256 => Box::new(rsa::Rsa256Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::RS384 => Box::new(rsa::Rsa384Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::RS512 => Box::new(rsa::Rsa512Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::PS256 => Box::new(rsa::RsaPss256Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::PS384 => Box::new(rsa::RsaPss384Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::PS512 => Box::new(rsa::RsaPss512Verifier::new(key)?) as Box<dyn JwtVerifier>,
        Algorithm::EdDSA => Box::new(eddsa::EdDSAVerifier::new(key)?) as Box<dyn JwtVerifier>,
    };

    Ok(jwt_verifier)
}

/// The default [`CryptoProvider`] backed by [`rust_crypto`](https://github.com/RustCrypto).
pub static DEFAULT_PROVIDER: CryptoProvider = CryptoProvider {
    signer_factory: new_signer,
    verifier_factory: new_verifier,
    jwk_utils: JwkUtils {
        extract_rsa_public_key_components,
        extract_ec_public_key_coordinates,
        compute_digest,
    },
};
