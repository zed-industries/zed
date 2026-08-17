use aws_lc_rs::{
    digest,
    signature::{
        self as aws_sig, ECDSA_P256_SHA256_FIXED_SIGNING, ECDSA_P384_SHA384_FIXED_SIGNING,
        EcdsaKeyPair, KeyPair,
    },
};

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
    let key_pair = aws_sig::RsaKeyPair::from_der(key_content)
        .map_err(|e| ErrorKind::InvalidRsaKey(e.to_string()))?;
    let public = key_pair.public_key();
    let components = aws_sig::RsaPublicKeyComponents::<Vec<u8>>::from(public);
    Ok((components.n, components.e))
}

fn extract_ec_public_key_coordinates(
    key_content: &[u8],
    alg: Algorithm,
) -> errors::Result<(EllipticCurve, Vec<u8>, Vec<u8>)> {
    let (signing_alg, curve, pub_elem_bytes) = match alg {
        Algorithm::ES256 => (&ECDSA_P256_SHA256_FIXED_SIGNING, EllipticCurve::P256, 32),
        Algorithm::ES384 => (&ECDSA_P384_SHA384_FIXED_SIGNING, EllipticCurve::P384, 48),
        _ => return Err(ErrorKind::InvalidEcdsaKey.into()),
    };

    let key_pair = EcdsaKeyPair::from_pkcs8(signing_alg, key_content)
        .map_err(|_| ErrorKind::InvalidEcdsaKey)?;

    let pub_bytes = key_pair.public_key().as_ref();
    if pub_bytes[0] != 4 {
        return Err(ErrorKind::InvalidEcdsaKey.into());
    }

    let (x, y) = pub_bytes[1..].split_at(pub_elem_bytes);
    Ok((curve, x.to_vec(), y.to_vec()))
}

fn compute_digest(data: &[u8], hash_function: ThumbprintHash) -> Vec<u8> {
    let algorithm = match hash_function {
        ThumbprintHash::SHA256 => &digest::SHA256,
        ThumbprintHash::SHA384 => &digest::SHA384,
        ThumbprintHash::SHA512 => &digest::SHA512,
    };
    digest::digest(algorithm, data).as_ref().to_vec()
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

/// The default [`CryptoProvider`] backed by [`aws_lc_rs`].
pub static DEFAULT_PROVIDER: CryptoProvider = CryptoProvider {
    signer_factory: new_signer,
    verifier_factory: new_verifier,
    jwk_utils: JwkUtils {
        extract_rsa_public_key_components,
        extract_ec_public_key_coordinates,
        compute_digest,
    },
};
