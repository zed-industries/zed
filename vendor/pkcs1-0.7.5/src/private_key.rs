//! PKCS#1 RSA Private Keys.

#[cfg(feature = "alloc")]
pub(crate) mod other_prime_info;

use crate::{Error, Result, RsaPublicKey, Version};
use core::fmt;
use der::{
    asn1::UintRef, Decode, DecodeValue, Encode, EncodeValue, Header, Length, Reader, Sequence, Tag,
    Writer,
};

#[cfg(feature = "alloc")]
use {self::other_prime_info::OtherPrimeInfo, alloc::vec::Vec, der::SecretDocument};

#[cfg(feature = "pem")]
use der::pem::PemLabel;

/// PKCS#1 RSA Private Keys as defined in [RFC 8017 Appendix 1.2].
///
/// ASN.1 structure containing a serialized RSA private key:
///
/// ```text
/// RSAPrivateKey ::= SEQUENCE {
///     version           Version,
///     modulus           INTEGER,  -- n
///     publicExponent    INTEGER,  -- e
///     privateExponent   INTEGER,  -- d
///     prime1            INTEGER,  -- p
///     prime2            INTEGER,  -- q
///     exponent1         INTEGER,  -- d mod (p-1)
///     exponent2         INTEGER,  -- d mod (q-1)
///     coefficient       INTEGER,  -- (inverse of q) mod p
///     otherPrimeInfos   OtherPrimeInfos OPTIONAL
/// }
/// ```
///
/// Note: the `version` field is selected automatically based on the absence or
/// presence of the `other_prime_infos` field.
///
/// [RFC 8017 Appendix 1.2]: https://datatracker.ietf.org/doc/html/rfc8017#appendix-A.1.2
#[derive(Clone)]
pub struct RsaPrivateKey<'a> {
    /// `n`: RSA modulus.
    pub modulus: UintRef<'a>,

    /// `e`: RSA public exponent.
    pub public_exponent: UintRef<'a>,

    /// `d`: RSA private exponent.
    pub private_exponent: UintRef<'a>,

    /// `p`: first prime factor of `n`.
    pub prime1: UintRef<'a>,

    /// `q`: Second prime factor of `n`.
    pub prime2: UintRef<'a>,

    /// First exponent: `d mod (p-1)`.
    pub exponent1: UintRef<'a>,

    /// Second exponent: `d mod (q-1)`.
    pub exponent2: UintRef<'a>,

    /// CRT coefficient: `(inverse of q) mod p`.
    pub coefficient: UintRef<'a>,

    /// Additional primes `r_3`, ..., `r_u`, in order, if this is a multi-prime
    /// RSA key (i.e. `version` is `multi`).
    pub other_prime_infos: Option<OtherPrimeInfos<'a>>,
}

impl<'a> RsaPrivateKey<'a> {
    /// Get the public key that corresponds to this [`RsaPrivateKey`].
    pub fn public_key(&self) -> RsaPublicKey<'a> {
        RsaPublicKey {
            modulus: self.modulus,
            public_exponent: self.public_exponent,
        }
    }

    /// Get the [`Version`] for this key.
    ///
    /// Determined by the presence or absence of the
    /// [`RsaPrivateKey::other_prime_infos`] field.
    pub fn version(&self) -> Version {
        if self.other_prime_infos.is_some() {
            Version::Multi
        } else {
            Version::TwoPrime
        }
    }
}

impl<'a> DecodeValue<'a> for RsaPrivateKey<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> der::Result<Self> {
        reader.read_nested(header.length, |reader| {
            let version = Version::decode(reader)?;

            let result = Self {
                modulus: reader.decode()?,
                public_exponent: reader.decode()?,
                private_exponent: reader.decode()?,
                prime1: reader.decode()?,
                prime2: reader.decode()?,
                exponent1: reader.decode()?,
                exponent2: reader.decode()?,
                coefficient: reader.decode()?,
                other_prime_infos: reader.decode()?,
            };

            // Ensure version is set correctly for two-prime vs multi-prime key.
            if version.is_multi() != result.other_prime_infos.is_some() {
                return Err(reader.error(der::ErrorKind::Value { tag: Tag::Integer }));
            }

            Ok(result)
        })
    }
}

impl EncodeValue for RsaPrivateKey<'_> {
    fn value_len(&self) -> der::Result<Length> {
        self.version().encoded_len()?
            + self.modulus.encoded_len()?
            + self.public_exponent.encoded_len()?
            + self.private_exponent.encoded_len()?
            + self.prime1.encoded_len()?
            + self.prime2.encoded_len()?
            + self.exponent1.encoded_len()?
            + self.exponent2.encoded_len()?
            + self.coefficient.encoded_len()?
            + self.other_prime_infos.encoded_len()?
    }

    fn encode_value(&self, writer: &mut impl Writer) -> der::Result<()> {
        self.version().encode(writer)?;
        self.modulus.encode(writer)?;
        self.public_exponent.encode(writer)?;
        self.private_exponent.encode(writer)?;
        self.prime1.encode(writer)?;
        self.prime2.encode(writer)?;
        self.exponent1.encode(writer)?;
        self.exponent2.encode(writer)?;
        self.coefficient.encode(writer)?;
        self.other_prime_infos.encode(writer)?;
        Ok(())
    }
}

impl<'a> Sequence<'a> for RsaPrivateKey<'a> {}

impl<'a> From<RsaPrivateKey<'a>> for RsaPublicKey<'a> {
    fn from(private_key: RsaPrivateKey<'a>) -> RsaPublicKey<'a> {
        private_key.public_key()
    }
}

impl<'a> From<&RsaPrivateKey<'a>> for RsaPublicKey<'a> {
    fn from(private_key: &RsaPrivateKey<'a>) -> RsaPublicKey<'a> {
        private_key.public_key()
    }
}

impl<'a> TryFrom<&'a [u8]> for RsaPrivateKey<'a> {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self> {
        Ok(Self::from_der(bytes)?)
    }
}

impl fmt::Debug for RsaPrivateKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RsaPrivateKey")
            .field("version", &self.version())
            .field("modulus", &self.modulus)
            .field("public_exponent", &self.public_exponent)
            .finish_non_exhaustive()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<RsaPrivateKey<'_>> for SecretDocument {
    type Error = Error;

    fn try_from(private_key: RsaPrivateKey<'_>) -> Result<SecretDocument> {
        SecretDocument::try_from(&private_key)
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&RsaPrivateKey<'_>> for SecretDocument {
    type Error = Error;

    fn try_from(private_key: &RsaPrivateKey<'_>) -> Result<SecretDocument> {
        Ok(Self::encode_msg(private_key)?)
    }
}

#[cfg(feature = "pem")]
impl PemLabel for RsaPrivateKey<'_> {
    const PEM_LABEL: &'static str = "RSA PRIVATE KEY";
}

/// Placeholder struct for `OtherPrimeInfos` in the no-`alloc` case.
///
/// This type is unconstructable by design, but supports the same traits.
#[cfg(not(feature = "alloc"))]
#[derive(Clone)]
#[non_exhaustive]
pub struct OtherPrimeInfos<'a> {
    _lifetime: core::marker::PhantomData<&'a ()>,
}

#[cfg(not(feature = "alloc"))]
impl<'a> DecodeValue<'a> for OtherPrimeInfos<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, _header: Header) -> der::Result<Self> {
        // Placeholder decoder that always returns an error.
        // Uses `Tag::Integer` to signal an unsupported version.
        Err(reader.error(der::ErrorKind::Value { tag: Tag::Integer }))
    }
}

#[cfg(not(feature = "alloc"))]
impl EncodeValue for OtherPrimeInfos<'_> {
    fn value_len(&self) -> der::Result<Length> {
        // Placeholder decoder that always returns an error.
        // Uses `Tag::Integer` to signal an unsupported version.
        Err(der::ErrorKind::Value { tag: Tag::Integer }.into())
    }

    fn encode_value(&self, _writer: &mut impl Writer) -> der::Result<()> {
        // Placeholder decoder that always returns an error.
        // Uses `Tag::Integer` to signal an unsupported version.
        Err(der::ErrorKind::Value { tag: Tag::Integer }.into())
    }
}

#[cfg(not(feature = "alloc"))]
impl<'a> der::FixedTag for OtherPrimeInfos<'a> {
    const TAG: Tag = Tag::Sequence;
}

/// Additional RSA prime info in a multi-prime RSA key.
#[cfg(feature = "alloc")]
pub type OtherPrimeInfos<'a> = Vec<OtherPrimeInfo<'a>>;
