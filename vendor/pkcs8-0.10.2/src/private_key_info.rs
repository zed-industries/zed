//! PKCS#8 `PrivateKeyInfo`.

use crate::{AlgorithmIdentifierRef, Error, Result, Version};
use core::fmt;
use der::{
    asn1::{AnyRef, BitStringRef, ContextSpecific, OctetStringRef},
    Decode, DecodeValue, Encode, EncodeValue, Header, Length, Reader, Sequence, TagMode, TagNumber,
    Writer,
};

#[cfg(feature = "alloc")]
use der::SecretDocument;

#[cfg(feature = "encryption")]
use {
    crate::EncryptedPrivateKeyInfo,
    der::zeroize::Zeroizing,
    pkcs5::pbes2,
    rand_core::{CryptoRng, RngCore},
};

#[cfg(feature = "pem")]
use der::pem::PemLabel;

#[cfg(feature = "subtle")]
use subtle::{Choice, ConstantTimeEq};

/// Context-specific tag number for the public key.
const PUBLIC_KEY_TAG: TagNumber = TagNumber::N1;

/// PKCS#8 `PrivateKeyInfo`.
///
/// ASN.1 structure containing an `AlgorithmIdentifier`, private key
/// data in an algorithm specific format, and optional attributes
/// (ignored by this implementation).
///
/// Supports PKCS#8 v1 as described in [RFC 5208] and PKCS#8 v2 as described
/// in [RFC 5958]. PKCS#8 v2 keys include an additional public key field.
///
/// # PKCS#8 v1 `PrivateKeyInfo`
///
/// Described in [RFC 5208 Section 5]:
///
/// ```text
/// PrivateKeyInfo ::= SEQUENCE {
///         version                   Version,
///         privateKeyAlgorithm       PrivateKeyAlgorithmIdentifier,
///         privateKey                PrivateKey,
///         attributes           [0]  IMPLICIT Attributes OPTIONAL }
///
/// Version ::= INTEGER
///
/// PrivateKeyAlgorithmIdentifier ::= AlgorithmIdentifier
///
/// PrivateKey ::= OCTET STRING
///
/// Attributes ::= SET OF Attribute
/// ```
///
/// # PKCS#8 v2 `OneAsymmetricKey`
///
/// PKCS#8 `OneAsymmetricKey` as described in [RFC 5958 Section 2]:
///
/// ```text
/// PrivateKeyInfo ::= OneAsymmetricKey
///
/// OneAsymmetricKey ::= SEQUENCE {
///     version                   Version,
///     privateKeyAlgorithm       PrivateKeyAlgorithmIdentifier,
///     privateKey                PrivateKey,
///     attributes            [0] Attributes OPTIONAL,
///     ...,
///     [[2: publicKey        [1] PublicKey OPTIONAL ]],
///     ...
///   }
///
/// Version ::= INTEGER { v1(0), v2(1) } (v1, ..., v2)
///
/// PrivateKeyAlgorithmIdentifier ::= AlgorithmIdentifier
///
/// PrivateKey ::= OCTET STRING
///
/// Attributes ::= SET OF Attribute
///
/// PublicKey ::= BIT STRING
/// ```
///
/// [RFC 5208]: https://tools.ietf.org/html/rfc5208
/// [RFC 5958]: https://datatracker.ietf.org/doc/html/rfc5958
/// [RFC 5208 Section 5]: https://tools.ietf.org/html/rfc5208#section-5
/// [RFC 5958 Section 2]: https://datatracker.ietf.org/doc/html/rfc5958#section-2
#[derive(Clone)]
pub struct PrivateKeyInfo<'a> {
    /// X.509 `AlgorithmIdentifier` for the private key type.
    pub algorithm: AlgorithmIdentifierRef<'a>,

    /// Private key data.
    pub private_key: &'a [u8],

    /// Public key data, optionally available if version is V2.
    pub public_key: Option<&'a [u8]>,
}

impl<'a> PrivateKeyInfo<'a> {
    /// Create a new PKCS#8 [`PrivateKeyInfo`] message.
    ///
    /// This is a helper method which initializes `attributes` and `public_key`
    /// to `None`, helpful if you aren't using those.
    pub fn new(algorithm: AlgorithmIdentifierRef<'a>, private_key: &'a [u8]) -> Self {
        Self {
            algorithm,
            private_key,
            public_key: None,
        }
    }

    /// Get the PKCS#8 [`Version`] for this structure.
    ///
    /// [`Version::V1`] if `public_key` is `None`, [`Version::V2`] if `Some`.
    pub fn version(&self) -> Version {
        if self.public_key.is_some() {
            Version::V2
        } else {
            Version::V1
        }
    }

    /// Encrypt this private key using a symmetric encryption key derived
    /// from the provided password.
    ///
    /// Uses the following algorithms for encryption:
    /// - PBKDF: scrypt with default parameters:
    ///   - log₂(N): 15
    ///   - r: 8
    ///   - p: 1
    /// - Cipher: AES-256-CBC (best available option for PKCS#5 encryption)
    #[cfg(feature = "encryption")]
    pub fn encrypt(
        &self,
        rng: impl CryptoRng + RngCore,
        password: impl AsRef<[u8]>,
    ) -> Result<SecretDocument> {
        let der = Zeroizing::new(self.to_der()?);
        EncryptedPrivateKeyInfo::encrypt(rng, password, der.as_ref())
    }

    /// Encrypt this private key using a symmetric encryption key derived
    /// from the provided password and [`pbes2::Parameters`].
    #[cfg(feature = "encryption")]
    pub fn encrypt_with_params(
        &self,
        pbes2_params: pbes2::Parameters<'_>,
        password: impl AsRef<[u8]>,
    ) -> Result<SecretDocument> {
        let der = Zeroizing::new(self.to_der()?);
        EncryptedPrivateKeyInfo::encrypt_with(pbes2_params, password, der.as_ref())
    }

    /// Get a `BIT STRING` representation of the public key, if present.
    fn public_key_bit_string(&self) -> der::Result<Option<ContextSpecific<BitStringRef<'a>>>> {
        self.public_key
            .map(|pk| {
                BitStringRef::from_bytes(pk).map(|value| ContextSpecific {
                    tag_number: PUBLIC_KEY_TAG,
                    tag_mode: TagMode::Implicit,
                    value,
                })
            })
            .transpose()
    }
}

impl<'a> DecodeValue<'a> for PrivateKeyInfo<'a> {
    fn decode_value<R: Reader<'a>>(
        reader: &mut R,
        header: Header,
    ) -> der::Result<PrivateKeyInfo<'a>> {
        reader.read_nested(header.length, |reader| {
            // Parse and validate `version` INTEGER.
            let version = Version::decode(reader)?;
            let algorithm = reader.decode()?;
            let private_key = OctetStringRef::decode(reader)?.into();
            let public_key = reader
                .context_specific::<BitStringRef<'_>>(PUBLIC_KEY_TAG, TagMode::Implicit)?
                .map(|bs| {
                    bs.as_bytes()
                        .ok_or_else(|| der::Tag::BitString.value_error())
                })
                .transpose()?;

            if version.has_public_key() != public_key.is_some() {
                return Err(reader.error(
                    der::Tag::ContextSpecific {
                        constructed: true,
                        number: PUBLIC_KEY_TAG,
                    }
                    .value_error()
                    .kind(),
                ));
            }

            // Ignore any remaining extension fields
            while !reader.is_finished() {
                reader.decode::<ContextSpecific<AnyRef<'_>>>()?;
            }

            Ok(Self {
                algorithm,
                private_key,
                public_key,
            })
        })
    }
}

impl EncodeValue for PrivateKeyInfo<'_> {
    fn value_len(&self) -> der::Result<Length> {
        self.version().encoded_len()?
            + self.algorithm.encoded_len()?
            + OctetStringRef::new(self.private_key)?.encoded_len()?
            + self.public_key_bit_string()?.encoded_len()?
    }

    fn encode_value(&self, writer: &mut impl Writer) -> der::Result<()> {
        self.version().encode(writer)?;
        self.algorithm.encode(writer)?;
        OctetStringRef::new(self.private_key)?.encode(writer)?;
        self.public_key_bit_string()?.encode(writer)?;
        Ok(())
    }
}

impl<'a> Sequence<'a> for PrivateKeyInfo<'a> {}

impl<'a> TryFrom<&'a [u8]> for PrivateKeyInfo<'a> {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self> {
        Ok(Self::from_der(bytes)?)
    }
}

impl<'a> fmt::Debug for PrivateKeyInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrivateKeyInfo")
            .field("version", &self.version())
            .field("algorithm", &self.algorithm)
            .field("public_key", &self.public_key)
            .finish_non_exhaustive()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<PrivateKeyInfo<'_>> for SecretDocument {
    type Error = Error;

    fn try_from(private_key: PrivateKeyInfo<'_>) -> Result<SecretDocument> {
        SecretDocument::try_from(&private_key)
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&PrivateKeyInfo<'_>> for SecretDocument {
    type Error = Error;

    fn try_from(private_key: &PrivateKeyInfo<'_>) -> Result<SecretDocument> {
        Ok(Self::encode_msg(private_key)?)
    }
}

#[cfg(feature = "pem")]
impl PemLabel for PrivateKeyInfo<'_> {
    const PEM_LABEL: &'static str = "PRIVATE KEY";
}

#[cfg(feature = "subtle")]
impl<'a> ConstantTimeEq for PrivateKeyInfo<'a> {
    fn ct_eq(&self, other: &Self) -> Choice {
        // NOTE: public fields are not compared in constant time
        let public_fields_eq =
            self.algorithm == other.algorithm && self.public_key == other.public_key;

        self.private_key.ct_eq(other.private_key) & Choice::from(public_fields_eq as u8)
    }
}

#[cfg(feature = "subtle")]
impl<'a> Eq for PrivateKeyInfo<'a> {}

#[cfg(feature = "subtle")]
impl<'a> PartialEq for PrivateKeyInfo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}
