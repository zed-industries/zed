//! X.509 `SubjectPublicKeyInfo`

use crate::{AlgorithmIdentifier, Error, Result};
use core::cmp::Ordering;
use der::{
    asn1::{AnyRef, BitStringRef},
    Choice, Decode, DecodeValue, DerOrd, Encode, EncodeValue, FixedTag, Header, Length, Reader,
    Sequence, ValueOrd, Writer,
};

#[cfg(feature = "alloc")]
use der::{
    asn1::{Any, BitString},
    Document,
};

#[cfg(feature = "fingerprint")]
use crate::{fingerprint, FingerprintBytes};

#[cfg(feature = "pem")]
use der::pem::PemLabel;

/// [`SubjectPublicKeyInfo`] with [`AnyRef`] algorithm parameters, and [`BitStringRef`] params.
pub type SubjectPublicKeyInfoRef<'a> = SubjectPublicKeyInfo<AnyRef<'a>, BitStringRef<'a>>;

/// [`SubjectPublicKeyInfo`] with [`Any`] algorithm parameters, and [`BitString`] params.
#[cfg(feature = "alloc")]
pub type SubjectPublicKeyInfoOwned = SubjectPublicKeyInfo<Any, BitString>;

/// X.509 `SubjectPublicKeyInfo` (SPKI) as defined in [RFC 5280 § 4.1.2.7].
///
/// ASN.1 structure containing an [`AlgorithmIdentifier`] and public key
/// data in an algorithm specific format.
///
/// ```text
///    SubjectPublicKeyInfo  ::=  SEQUENCE  {
///         algorithm            AlgorithmIdentifier,
///         subjectPublicKey     BIT STRING  }
/// ```
///
/// [RFC 5280 § 4.1.2.7]: https://tools.ietf.org/html/rfc5280#section-4.1.2.7
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubjectPublicKeyInfo<Params, Key> {
    /// X.509 [`AlgorithmIdentifier`] for the public key type
    pub algorithm: AlgorithmIdentifier<Params>,

    /// Public key data
    pub subject_public_key: Key,
}

impl<'a, Params, Key> SubjectPublicKeyInfo<Params, Key>
where
    Params: Choice<'a> + Encode,
    // TODO: replace FixedTag with FixedTag<TAG = { Tag::BitString }> once
    // https://github.com/rust-lang/rust/issues/92827 is fixed
    Key: Decode<'a> + Encode + FixedTag,
{
    /// Calculate the SHA-256 fingerprint of this [`SubjectPublicKeyInfo`] and
    /// encode it as a Base64 string.
    ///
    /// See [RFC7469 § 2.1.1] for more information.
    ///
    /// [RFC7469 § 2.1.1]: https://datatracker.ietf.org/doc/html/rfc7469#section-2.1.1
    #[cfg(all(feature = "fingerprint", feature = "alloc", feature = "base64"))]
    pub fn fingerprint_base64(&self) -> Result<alloc::string::String> {
        use base64ct::{Base64, Encoding};
        Ok(Base64::encode_string(&self.fingerprint_bytes()?))
    }

    /// Calculate the SHA-256 fingerprint of this [`SubjectPublicKeyInfo`] as
    /// a raw byte array.
    ///
    /// See [RFC7469 § 2.1.1] for more information.
    ///
    /// [RFC7469 § 2.1.1]: https://datatracker.ietf.org/doc/html/rfc7469#section-2.1.1
    #[cfg(feature = "fingerprint")]
    pub fn fingerprint_bytes(&self) -> Result<FingerprintBytes> {
        let mut builder = fingerprint::Builder::new();
        self.encode(&mut builder)?;
        Ok(builder.finish())
    }
}

impl<'a: 'k, 'k, Params, Key: 'k> DecodeValue<'a> for SubjectPublicKeyInfo<Params, Key>
where
    Params: Choice<'a> + Encode,
    Key: Decode<'a>,
{
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> der::Result<Self> {
        reader.read_nested(header.length, |reader| {
            Ok(Self {
                algorithm: reader.decode()?,
                subject_public_key: Key::decode(reader)?,
            })
        })
    }
}

impl<'a, Params, Key> EncodeValue for SubjectPublicKeyInfo<Params, Key>
where
    Params: Choice<'a> + Encode,
    Key: Encode,
{
    fn value_len(&self) -> der::Result<Length> {
        self.algorithm.encoded_len()? + self.subject_public_key.encoded_len()?
    }

    fn encode_value(&self, writer: &mut impl Writer) -> der::Result<()> {
        self.algorithm.encode(writer)?;
        self.subject_public_key.encode(writer)?;
        Ok(())
    }
}

impl<'a, Params, Key> Sequence<'a> for SubjectPublicKeyInfo<Params, Key>
where
    Params: Choice<'a> + Encode,
    Key: Decode<'a> + Encode + FixedTag,
{
}

impl<'a, Params, Key> TryFrom<&'a [u8]> for SubjectPublicKeyInfo<Params, Key>
where
    Params: Choice<'a> + Encode,
    Key: Decode<'a> + Encode + FixedTag,
{
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self> {
        Ok(Self::from_der(bytes)?)
    }
}

impl<'a, Params, Key> ValueOrd for SubjectPublicKeyInfo<Params, Key>
where
    Params: Choice<'a> + DerOrd + Encode,
    Key: ValueOrd,
{
    fn value_cmp(&self, other: &Self) -> der::Result<Ordering> {
        match self.algorithm.der_cmp(&other.algorithm)? {
            Ordering::Equal => self.subject_public_key.value_cmp(&other.subject_public_key),
            other => Ok(other),
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a: 'k, 'k, Params, Key: 'k> TryFrom<SubjectPublicKeyInfo<Params, Key>> for Document
where
    Params: Choice<'a> + Encode,
    Key: Decode<'a> + Encode + FixedTag,
    BitStringRef<'a>: From<&'k Key>,
{
    type Error = Error;

    fn try_from(spki: SubjectPublicKeyInfo<Params, Key>) -> Result<Document> {
        Self::try_from(&spki)
    }
}

#[cfg(feature = "alloc")]
impl<'a: 'k, 'k, Params, Key: 'k> TryFrom<&SubjectPublicKeyInfo<Params, Key>> for Document
where
    Params: Choice<'a> + Encode,
    Key: Decode<'a> + Encode + FixedTag,
    BitStringRef<'a>: From<&'k Key>,
{
    type Error = Error;

    fn try_from(spki: &SubjectPublicKeyInfo<Params, Key>) -> Result<Document> {
        Ok(Self::encode_msg(spki)?)
    }
}

#[cfg(feature = "pem")]
impl<Params, Key> PemLabel for SubjectPublicKeyInfo<Params, Key> {
    const PEM_LABEL: &'static str = "PUBLIC KEY";
}

#[cfg(feature = "alloc")]
mod allocating {
    use super::*;
    use crate::EncodePublicKey;
    use der::referenced::*;

    impl<'a> RefToOwned<'a> for SubjectPublicKeyInfoRef<'a> {
        type Owned = SubjectPublicKeyInfoOwned;
        fn ref_to_owned(&self) -> Self::Owned {
            SubjectPublicKeyInfo {
                algorithm: self.algorithm.ref_to_owned(),
                subject_public_key: self.subject_public_key.ref_to_owned(),
            }
        }
    }

    impl OwnedToRef for SubjectPublicKeyInfoOwned {
        type Borrowed<'a> = SubjectPublicKeyInfoRef<'a>;
        fn owned_to_ref(&self) -> Self::Borrowed<'_> {
            SubjectPublicKeyInfo {
                algorithm: self.algorithm.owned_to_ref(),
                subject_public_key: self.subject_public_key.owned_to_ref(),
            }
        }
    }

    impl SubjectPublicKeyInfoOwned {
        /// Create a [`SubjectPublicKeyInfoOwned`] from any object that implements
        /// [`EncodePublicKey`].
        pub fn from_key<T>(source: T) -> Result<Self>
        where
            T: EncodePublicKey,
        {
            Ok(source.to_public_key_der()?.decode_msg::<Self>()?)
        }
    }
}
