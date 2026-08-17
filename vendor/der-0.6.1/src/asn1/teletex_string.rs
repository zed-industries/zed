//! ASN.1 `TeletexString` support.

use crate::{
    asn1::AnyRef, ord::OrdIsValueOrd, ByteSlice, DecodeValue, EncodeValue, Error, FixedTag, Header,
    Length, Reader, Result, StrSlice, Tag, Writer,
};
use core::{fmt, ops::Deref, str};

/// ASN.1 `TeletexString` type.
///
/// Supports a subset the ASCII character set (described below).
///
/// For UTF-8, use [`Utf8StringRef`][`crate::asn1::Utf8StringRef`] instead.
/// For the full ASCII character set, use
/// [`Ia5StringRef`][`crate::asn1::Ia5StringRef`].
///
/// This is a zero-copy reference type which borrows from the input data.
///
/// # Supported characters
///
/// The standard defines a complex character set allowed in this type. However, quoting the ASN.1
/// mailing list, "a sizable volume of software in the world treats TeletexString (T61String) as a
/// simple 8-bit string with mostly Windows Latin 1 (superset of iso-8859-1) encoding".
///
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TeletexStringRef<'a> {
    /// Inner value
    inner: StrSlice<'a>,
}

impl<'a> TeletexStringRef<'a> {
    /// Create a new ASN.1 `TeletexString`.
    pub fn new<T>(input: &'a T) -> Result<Self>
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let input = input.as_ref();

        // FIXME: support higher part of the charset
        if input.iter().any(|&c| c > 0x7F) {
            return Err(Self::TAG.value_error());
        }

        StrSlice::from_bytes(input)
            .map(|inner| Self { inner })
            .map_err(|_| Self::TAG.value_error())
    }
}

impl<'a> Deref for TeletexStringRef<'a> {
    type Target = StrSlice<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<str> for TeletexStringRef<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for TeletexStringRef<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> DecodeValue<'a> for TeletexStringRef<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        Self::new(ByteSlice::decode_value(reader, header)?.as_slice())
    }
}

impl<'a> EncodeValue for TeletexStringRef<'a> {
    fn value_len(&self) -> Result<Length> {
        self.inner.value_len()
    }

    fn encode_value(&self, writer: &mut dyn Writer) -> Result<()> {
        self.inner.encode_value(writer)
    }
}

impl FixedTag for TeletexStringRef<'_> {
    const TAG: Tag = Tag::TeletexString;
}

impl OrdIsValueOrd for TeletexStringRef<'_> {}

impl<'a> From<&TeletexStringRef<'a>> for TeletexStringRef<'a> {
    fn from(value: &TeletexStringRef<'a>) -> TeletexStringRef<'a> {
        *value
    }
}

impl<'a> TryFrom<AnyRef<'a>> for TeletexStringRef<'a> {
    type Error = Error;

    fn try_from(any: AnyRef<'a>) -> Result<TeletexStringRef<'a>> {
        any.decode_into()
    }
}

impl<'a> From<TeletexStringRef<'a>> for AnyRef<'a> {
    fn from(teletex_string: TeletexStringRef<'a>) -> AnyRef<'a> {
        AnyRef::from_tag_and_value(Tag::TeletexString, teletex_string.inner.into())
    }
}

impl<'a> fmt::Display for TeletexStringRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> fmt::Debug for TeletexStringRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TeletexString({:?})", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::TeletexStringRef;
    use crate::Decode;
    use crate::SliceWriter;

    #[test]
    fn parse_bytes() {
        let example_bytes = &[
            0x14, 0x0b, 0x54, 0x65, 0x73, 0x74, 0x20, 0x55, 0x73, 0x65, 0x72, 0x20, 0x31,
        ];

        let teletex_string = TeletexStringRef::from_der(example_bytes).unwrap();
        assert_eq!(teletex_string.as_str(), "Test User 1");
        let mut out = [0_u8; 30];
        let mut writer = SliceWriter::new(&mut out);
        writer.encode(&teletex_string).unwrap();
        let encoded = writer.finish().unwrap();
        assert_eq!(encoded, example_bytes);
    }
}
