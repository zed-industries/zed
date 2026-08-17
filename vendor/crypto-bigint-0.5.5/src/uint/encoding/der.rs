//! Support for decoding/encoding [`Uint`] as an ASN.1 DER `INTEGER`.

use crate::{generic_array::GenericArray, ArrayEncoding, Uint};
use ::der::{
    asn1::{AnyRef, UintRef},
    DecodeValue, EncodeValue, FixedTag, Length, Tag,
};

impl<'a, const LIMBS: usize> TryFrom<AnyRef<'a>> for Uint<LIMBS>
where
    Uint<LIMBS>: ArrayEncoding,
{
    type Error = der::Error;

    fn try_from(any: AnyRef<'a>) -> der::Result<Uint<LIMBS>> {
        UintRef::try_from(any)?.try_into()
    }
}

impl<'a, const LIMBS: usize> TryFrom<UintRef<'a>> for Uint<LIMBS>
where
    Uint<LIMBS>: ArrayEncoding,
{
    type Error = der::Error;

    fn try_from(bytes: UintRef<'a>) -> der::Result<Uint<LIMBS>> {
        let mut array = GenericArray::default();
        let offset = array.len().saturating_sub(bytes.len().try_into()?);
        array[offset..].copy_from_slice(bytes.as_bytes());
        Ok(Uint::from_be_byte_array(array))
    }
}

impl<'a, const LIMBS: usize> DecodeValue<'a> for Uint<LIMBS>
where
    Uint<LIMBS>: ArrayEncoding,
{
    fn decode_value<R: der::Reader<'a>>(reader: &mut R, header: der::Header) -> der::Result<Self> {
        UintRef::decode_value(reader, header)?.try_into()
    }
}

impl<const LIMBS: usize> EncodeValue for Uint<LIMBS>
where
    Uint<LIMBS>: ArrayEncoding,
{
    fn value_len(&self) -> der::Result<Length> {
        // TODO(tarcieri): more efficient length calculation
        let array = self.to_be_byte_array();
        UintRef::new(&array)?.value_len()
    }

    fn encode_value(&self, encoder: &mut impl der::Writer) -> der::Result<()> {
        let array = self.to_be_byte_array();
        UintRef::new(&array)?.encode_value(encoder)
    }
}

impl<const LIMBS: usize> FixedTag for Uint<LIMBS>
where
    Uint<LIMBS>: ArrayEncoding,
{
    const TAG: Tag = Tag::Integer;
}
