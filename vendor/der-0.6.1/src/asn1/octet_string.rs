//! ASN.1 `OCTET STRING` support.

use crate::{
    asn1::AnyRef, ord::OrdIsValueOrd, ByteSlice, DecodeValue, EncodeValue, Error, ErrorKind,
    FixedTag, Header, Length, Reader, Result, Tag, Writer,
};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// ASN.1 `OCTET STRING` type: borrowed form.
///
/// Octet strings represent contiguous sequences of octets, a.k.a. bytes.
///
/// This is a zero-copy reference type which borrows from the input data.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct OctetStringRef<'a> {
    /// Inner value
    inner: ByteSlice<'a>,
}

impl<'a> OctetStringRef<'a> {
    /// Create a new ASN.1 `OCTET STRING` from a byte slice.
    pub fn new(slice: &'a [u8]) -> Result<Self> {
        ByteSlice::new(slice)
            .map(|inner| Self { inner })
            .map_err(|_| ErrorKind::Length { tag: Self::TAG }.into())
    }

    /// Borrow the inner byte slice.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.inner.as_slice()
    }

    /// Get the length of the inner byte slice.
    pub fn len(&self) -> Length {
        self.inner.len()
    }

    /// Is the inner byte slice empty?
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl AsRef<[u8]> for OctetStringRef<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> DecodeValue<'a> for OctetStringRef<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        let inner = ByteSlice::decode_value(reader, header)?;
        Ok(Self { inner })
    }
}

impl EncodeValue for OctetStringRef<'_> {
    fn value_len(&self) -> Result<Length> {
        self.inner.value_len()
    }

    fn encode_value(&self, writer: &mut dyn Writer) -> Result<()> {
        self.inner.encode_value(writer)
    }
}

impl FixedTag for OctetStringRef<'_> {
    const TAG: Tag = Tag::OctetString;
}

impl OrdIsValueOrd for OctetStringRef<'_> {}

impl<'a> From<&OctetStringRef<'a>> for OctetStringRef<'a> {
    fn from(value: &OctetStringRef<'a>) -> OctetStringRef<'a> {
        *value
    }
}

impl<'a> TryFrom<AnyRef<'a>> for OctetStringRef<'a> {
    type Error = Error;

    fn try_from(any: AnyRef<'a>) -> Result<OctetStringRef<'a>> {
        any.decode_into()
    }
}

impl<'a> From<OctetStringRef<'a>> for AnyRef<'a> {
    fn from(octet_string: OctetStringRef<'a>) -> AnyRef<'a> {
        AnyRef::from_tag_and_value(Tag::OctetString, octet_string.inner)
    }
}

impl<'a> From<OctetStringRef<'a>> for &'a [u8] {
    fn from(octet_string: OctetStringRef<'a>) -> &'a [u8] {
        octet_string.as_bytes()
    }
}

/// ASN.1 `OCTET STRING` type: owned form..
///
/// Octet strings represent contiguous sequences of octets, a.k.a. bytes.
///
/// This type provides the same functionality as [`OctetStringRef`] but owns
/// the backing data.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct OctetString {
    /// Bitstring represented as a slice of bytes.
    inner: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl OctetString {
    /// Create a new ASN.1 `OCTET STRING`.
    pub fn new(bytes: impl Into<Vec<u8>>) -> Result<Self> {
        let inner = bytes.into();

        // Ensure the bytes parse successfully as an `OctetStringRef`
        OctetStringRef::new(&inner)?;

        Ok(Self { inner })
    }

    /// Borrow the inner byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_slice()
    }

    /// Get the length of the inner byte slice.
    pub fn len(&self) -> Length {
        self.value_len().expect("invalid OCTET STRING length")
    }

    /// Is the inner byte slice empty?
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(feature = "alloc")]
impl AsRef<[u8]> for OctetString {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(feature = "alloc")]
impl<'a> DecodeValue<'a> for OctetString {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        Self::new(reader.read_vec(header.length)?)
    }
}

#[cfg(feature = "alloc")]
impl EncodeValue for OctetString {
    fn value_len(&self) -> Result<Length> {
        self.inner.len().try_into()
    }

    fn encode_value(&self, writer: &mut dyn Writer) -> Result<()> {
        writer.write(&self.inner)
    }
}

#[cfg(feature = "alloc")]
impl FixedTag for OctetString {
    const TAG: Tag = Tag::OctetString;
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a OctetString> for OctetStringRef<'a> {
    fn from(octet_string: &'a OctetString) -> OctetStringRef<'a> {
        // Ensured to parse successfully in constructor
        OctetStringRef::new(&octet_string.inner).expect("invalid OCTET STRING")
    }
}

#[cfg(feature = "alloc")]
impl OrdIsValueOrd for OctetString {}
