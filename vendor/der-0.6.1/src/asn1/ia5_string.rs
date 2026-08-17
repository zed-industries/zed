//! ASN.1 `IA5String` support.

use crate::{
    asn1::AnyRef, ord::OrdIsValueOrd, ByteSlice, DecodeValue, EncodeValue, Error, FixedTag, Header,
    Length, Reader, Result, StrSlice, Tag, Writer,
};
use core::{fmt, ops::Deref, str};

/// ASN.1 `IA5String` type.
///
/// Supports the [International Alphabet No. 5 (IA5)] character encoding, i.e.
/// the lower 128 characters of the ASCII alphabet. (Note: IA5 is now
/// technically known as the International Reference Alphabet or IRA as
/// specified in the ITU-T's T.50 recommendation).
///
/// For UTF-8, use [`Utf8StringRef`][`crate::asn1::Utf8StringRef`].
///
/// This is a zero-copy reference type which borrows from the input data.
///
/// [International Alphabet No. 5 (IA5)]: https://en.wikipedia.org/wiki/T.50_%28standard%29
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Ia5StringRef<'a> {
    /// Inner value
    inner: StrSlice<'a>,
}

impl<'a> Ia5StringRef<'a> {
    /// Create a new `IA5String`.
    pub fn new<T>(input: &'a T) -> Result<Self>
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let input = input.as_ref();

        // Validate all characters are within IA5String's allowed set
        if input.iter().any(|&c| c > 0x7F) {
            return Err(Self::TAG.value_error());
        }

        StrSlice::from_bytes(input)
            .map(|inner| Self { inner })
            .map_err(|_| Self::TAG.value_error())
    }
}

impl<'a> Deref for Ia5StringRef<'a> {
    type Target = StrSlice<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<str> for Ia5StringRef<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Ia5StringRef<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> DecodeValue<'a> for Ia5StringRef<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        Self::new(ByteSlice::decode_value(reader, header)?.as_slice())
    }
}

impl EncodeValue for Ia5StringRef<'_> {
    fn value_len(&self) -> Result<Length> {
        self.inner.value_len()
    }

    fn encode_value(&self, writer: &mut dyn Writer) -> Result<()> {
        self.inner.encode_value(writer)
    }
}

impl<'a> FixedTag for Ia5StringRef<'a> {
    const TAG: Tag = Tag::Ia5String;
}

impl OrdIsValueOrd for Ia5StringRef<'_> {}

impl<'a> From<&Ia5StringRef<'a>> for Ia5StringRef<'a> {
    fn from(value: &Ia5StringRef<'a>) -> Ia5StringRef<'a> {
        *value
    }
}

impl<'a> TryFrom<AnyRef<'a>> for Ia5StringRef<'a> {
    type Error = Error;

    fn try_from(any: AnyRef<'a>) -> Result<Ia5StringRef<'a>> {
        any.decode_into()
    }
}

impl<'a> From<Ia5StringRef<'a>> for AnyRef<'a> {
    fn from(printable_string: Ia5StringRef<'a>) -> AnyRef<'a> {
        AnyRef::from_tag_and_value(Tag::Ia5String, printable_string.inner.into())
    }
}

impl<'a> fmt::Display for Ia5StringRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> fmt::Debug for Ia5StringRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ia5String({:?})", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::Ia5StringRef;
    use crate::Decode;
    use hex_literal::hex;

    #[test]
    fn parse_bytes() {
        let example_bytes = hex!("16 0d 74 65 73 74 31 40 72 73 61 2e 63 6f 6d");
        let printable_string = Ia5StringRef::from_der(&example_bytes).unwrap();
        assert_eq!(printable_string.as_str(), "test1@rsa.com");
    }
}
