//! PKCS#1 version identifier.

use crate::Error;
use der::{Decode, Encode, FixedTag, Reader, Tag, Writer};

/// Version identifier for PKCS#1 documents as defined in
/// [RFC 8017 Appendix 1.2].
///
/// > version is the version number, for compatibility with future
/// > revisions of this document.  It SHALL be 0 for this version of the
/// > document, unless multi-prime is used; in which case, it SHALL be 1.
///
/// ```text
/// Version ::= INTEGER { two-prime(0), multi(1) }
///    (CONSTRAINED BY
///    {-- version must be multi if otherPrimeInfos present --})
/// ```
///
/// [RFC 8017 Appendix 1.2]: https://datatracker.ietf.org/doc/html/rfc8017#appendix-A.1.2
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Version {
    /// Denotes a `two-prime` key
    TwoPrime = 0,

    /// Denotes a `multi` (i.e. multi-prime) key
    Multi = 1,
}

impl Version {
    /// Is this a multi-prime RSA key?
    pub fn is_multi(self) -> bool {
        self == Self::Multi
    }
}

impl From<Version> for u8 {
    fn from(version: Version) -> Self {
        version as u8
    }
}

impl TryFrom<u8> for Version {
    type Error = Error;
    fn try_from(byte: u8) -> Result<Version, Error> {
        match byte {
            0 => Ok(Version::TwoPrime),
            1 => Ok(Version::Multi),
            _ => Err(Error::Version),
        }
    }
}

impl<'a> Decode<'a> for Version {
    fn decode<R: Reader<'a>>(decoder: &mut R) -> der::Result<Self> {
        Version::try_from(u8::decode(decoder)?).map_err(|_| Self::TAG.value_error())
    }
}

impl Encode for Version {
    fn encoded_len(&self) -> der::Result<der::Length> {
        der::Length::ONE.for_tlv()
    }

    fn encode(&self, writer: &mut impl Writer) -> der::Result<()> {
        u8::from(*self).encode(writer)
    }
}

impl FixedTag for Version {
    const TAG: Tag = Tag::Integer;
}
