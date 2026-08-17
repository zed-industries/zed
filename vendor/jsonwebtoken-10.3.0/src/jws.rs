//! JSON Web Signatures data type.
use std::marker::PhantomData;

use crate::crypto::{CryptoProvider, sign};
use crate::errors::{ErrorKind, Result, new_error};
use crate::serialization::{DecodedJwtPartClaims, b64_encode_part};
use crate::validation::validate;
use crate::{DecodingKey, EncodingKey, Header, TokenData, Validation};

use crate::decoding::verify_signature_body;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// This is a serde-compatible JSON Web Signature structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jws<C> {
    /// The base64 encoded header data.
    ///
    /// Defined in [RFC7515#3.2](https://tools.ietf.org/html/rfc7515#section-3.2).
    pub protected: String,
    /// The base64 encoded claims data.
    ///
    /// Defined in [RFC7515#3.2](https://tools.ietf.org/html/rfc7515#section-3.2).
    pub payload: String,
    /// The signature on the other fields.
    ///
    /// Defined in [RFC7515#3.2](https://tools.ietf.org/html/rfc7515#section-3.2).
    pub signature: String,
    /// Unused, for associating type metadata.
    #[serde(skip)]
    pub _pd: PhantomData<C>,
}

/// Encode the header and claims given and sign the payload using the algorithm from the header and the key.
/// If the algorithm given is RSA or EC, the key needs to be in the PEM format. This produces a JWS instead of
/// a JWT -- usage is similar to `encode`, see that for more details.
pub fn encode<T: Serialize>(
    header: &Header,
    claims: Option<&T>,
    key: &EncodingKey,
) -> Result<Jws<T>> {
    if key.family() != header.alg.family() {
        return Err(new_error(ErrorKind::InvalidAlgorithm));
    }
    let encoded_header = b64_encode_part(header)?;
    let encoded_claims = match claims {
        Some(claims) => b64_encode_part(claims)?,
        None => "".to_string(),
    };
    let message = [encoded_header.as_str(), encoded_claims.as_str()].join(".");
    let signature = sign(message.as_bytes(), key, header.alg)?;

    Ok(Jws {
        protected: encoded_header,
        payload: encoded_claims,
        signature,
        _pd: Default::default(),
    })
}

/// Validate a received JWS and decode into the header and claims.
pub fn decode<T: DeserializeOwned>(
    jws: &Jws<T>,
    key: &DecodingKey,
    validation: &Validation,
) -> Result<TokenData<T>> {
    let header = Header::from_encoded(&jws.protected)?;
    let message = [jws.protected.as_str(), jws.payload.as_str()].join(".");

    let verifying_provider = (CryptoProvider::get_default().verifier_factory)(&header.alg, key)?;
    verify_signature_body(
        message.as_bytes(),
        jws.signature.as_bytes(),
        &header,
        validation,
        verifying_provider,
    )?;

    let decoded_claims = DecodedJwtPartClaims::from_jwt_part_claims(&jws.payload)?;
    let claims = decoded_claims.deserialize()?;
    validate(decoded_claims.deserialize()?, validation)?;

    Ok(TokenData { header, claims })
}
