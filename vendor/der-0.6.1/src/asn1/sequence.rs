//! The [`Sequence`] trait simplifies writing decoders/encoders which map ASN.1
//! `SEQUENCE`s to Rust structs.

use crate::{
    ByteSlice, Decode, DecodeValue, Encode, EncodeValue, FixedTag, Header, Length, Reader, Result,
    Tag, Writer,
};

/// ASN.1 `SEQUENCE` trait.
///
/// Types which impl this trait receive blanket impls for the [`Decode`],
/// [`Encode`], and [`FixedTag`] traits.
pub trait Sequence<'a>: Decode<'a> {
    /// Call the provided function with a slice of [`Encode`] trait objects
    /// representing the fields of this `SEQUENCE`.
    ///
    /// This method uses a callback because structs with fields which aren't
    /// directly [`Encode`] may need to construct temporary values from
    /// their fields prior to encoding.
    fn fields<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&[&dyn Encode]) -> Result<T>;
}

impl<'a, M> EncodeValue for M
where
    M: Sequence<'a>,
{
    fn value_len(&self) -> Result<Length> {
        self.fields(|fields| {
            fields
                .iter()
                .try_fold(Length::ZERO, |acc, field| acc + field.encoded_len()?)
        })
    }

    fn encode_value(&self, writer: &mut dyn Writer) -> Result<()> {
        self.fields(|fields| {
            for &field in fields {
                field.encode(writer)?;
            }

            Ok(())
        })
    }
}

impl<'a, M> FixedTag for M
where
    M: Sequence<'a>,
{
    const TAG: Tag = Tag::Sequence;
}

/// The [`SequenceRef`] type provides raw access to the octets which comprise a
/// DER-encoded `SEQUENCE`.
///
/// This is a zero-copy reference type which borrows from the input data.
pub struct SequenceRef<'a> {
    /// Body of the `SEQUENCE`.
    body: ByteSlice<'a>,
}

impl<'a> DecodeValue<'a> for SequenceRef<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        Ok(Self {
            body: ByteSlice::decode_value(reader, header)?,
        })
    }
}

impl EncodeValue for SequenceRef<'_> {
    fn value_len(&self) -> Result<Length> {
        Ok(self.body.len())
    }

    fn encode_value(&self, writer: &mut dyn Writer) -> Result<()> {
        self.body.encode_value(writer)
    }
}

impl<'a> FixedTag for SequenceRef<'a> {
    const TAG: Tag = Tag::Sequence;
}
