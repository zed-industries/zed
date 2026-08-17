//! ASN.1 `VideotexString` support.

use crate::{
    asn1::AnyRef, ord::OrdIsValueOrd, ByteSlice, DecodeValue, EncodeValue, Error, FixedTag, Header,
    Length, Reader, Result, StrSlice, Tag, Writer,
};
use core::{fmt, ops::Deref, str};

/// ASN.1 `VideotexString` type.
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
/// For the practical purposes VideotexString is treated as IA5string, disallowing non-ASCII chars.
///
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct VideotexStringRef<'a> {
    /// Inner value
    inner: StrSlice<'a>,
}

impl<'a> VideotexStringRef<'a> {
    /// Create a new ASN.1 `VideotexString`.
    pub fn new<T>(input: &'a T) -> Result<Self>
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let input = input.as_ref();

        // Validate all characters are within VideotexString's allowed set
        // FIXME: treat as if it were IA5String
        if input.iter().any(|&c| c > 0x7F) {
            return Err(Self::TAG.value_error());
        }

        StrSlice::from_bytes(input)
            .map(|inner| Self { inner })
            .map_err(|_| Self::TAG.value_error())
    }
}

impl<'a> Deref for VideotexStringRef<'a> {
    type Target = StrSlice<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<str> for VideotexStringRef<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for VideotexStringRef<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> DecodeValue<'a> for VideotexStringRef<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        Self::new(ByteSlice::decode_value(reader, header)?.as_slice())
    }
}

impl<'a> EncodeValue for VideotexStringRef<'a> {
    fn value_len(&self) -> Result<Length> {
        self.inner.value_len()
    }

    fn encode_value(&self, writer: &mut dyn Writer) -> Result<()> {
        self.inner.encode_value(writer)
    }
}

impl FixedTag for VideotexStringRef<'_> {
    const TAG: Tag = Tag::VideotexString;
}

impl OrdIsValueOrd for VideotexStringRef<'_> {}

impl<'a> From<&VideotexStringRef<'a>> for VideotexStringRef<'a> {
    fn from(value: &VideotexStringRef<'a>) -> VideotexStringRef<'a> {
        *value
    }
}

impl<'a> TryFrom<AnyRef<'a>> for VideotexStringRef<'a> {
    type Error = Error;

    fn try_from(any: AnyRef<'a>) -> Result<VideotexStringRef<'a>> {
        any.decode_into()
    }
}

impl<'a> From<VideotexStringRef<'a>> for AnyRef<'a> {
    fn from(printable_string: VideotexStringRef<'a>) -> AnyRef<'a> {
        AnyRef::from_tag_and_value(Tag::VideotexString, printable_string.inner.into())
    }
}

impl<'a> From<VideotexStringRef<'a>> for &'a [u8] {
    fn from(printable_string: VideotexStringRef<'a>) -> &'a [u8] {
        printable_string.as_bytes()
    }
}

impl<'a> fmt::Display for VideotexStringRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> fmt::Debug for VideotexStringRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VideotexString({:?})", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::VideotexStringRef;
    use crate::Decode;

    #[test]
    fn parse_bytes() {
        let example_bytes = &[
            0x15, 0x0b, 0x54, 0x65, 0x73, 0x74, 0x20, 0x55, 0x73, 0x65, 0x72, 0x20, 0x31,
        ];

        let printable_string = VideotexStringRef::from_der(example_bytes).unwrap();
        assert_eq!(printable_string.as_str(), "Test User 1");
    }
}
