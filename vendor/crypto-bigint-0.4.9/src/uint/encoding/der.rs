//! Support for decoding/encoding [`UInt`] as an ASN.1 DER `INTEGER`.

use crate::{generic_array::GenericArray, ArrayEncoding, UInt};
use ::der::{
    asn1::{AnyRef, UIntRef},
    DecodeValue, EncodeValue, FixedTag, Length, Tag,
};

#[cfg_attr(docsrs, doc(cfg(feature = "der")))]
impl<'a, const LIMBS: usize> TryFrom<AnyRef<'a>> for UInt<LIMBS>
where
    UInt<LIMBS>: ArrayEncoding,
{
    type Error = der::Error;

    fn try_from(any: AnyRef<'a>) -> der::Result<UInt<LIMBS>> {
        UIntRef::try_from(any)?.try_into()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "der")))]
impl<'a, const LIMBS: usize> TryFrom<UIntRef<'a>> for UInt<LIMBS>
where
    UInt<LIMBS>: ArrayEncoding,
{
    type Error = der::Error;

    fn try_from(bytes: UIntRef<'a>) -> der::Result<UInt<LIMBS>> {
        let mut array = GenericArray::default();
        let offset = array.len().saturating_sub(bytes.len().try_into()?);
        array[offset..].copy_from_slice(bytes.as_bytes());
        Ok(UInt::from_be_byte_array(array))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "der")))]
impl<'a, const LIMBS: usize> DecodeValue<'a> for UInt<LIMBS>
where
    UInt<LIMBS>: ArrayEncoding,
{
    fn decode_value<R: der::Reader<'a>>(reader: &mut R, header: der::Header) -> der::Result<Self> {
        UIntRef::decode_value(reader, header)?.try_into()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "der")))]
impl<const LIMBS: usize> EncodeValue for UInt<LIMBS>
where
    UInt<LIMBS>: ArrayEncoding,
{
    fn value_len(&self) -> der::Result<Length> {
        // TODO(tarcieri): more efficient length calculation
        let array = self.to_be_byte_array();
        UIntRef::new(&array)?.value_len()
    }

    fn encode_value(&self, encoder: &mut dyn der::Writer) -> der::Result<()> {
        let array = self.to_be_byte_array();
        UIntRef::new(&array)?.encode_value(encoder)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "der")))]
impl<const LIMBS: usize> FixedTag for UInt<LIMBS>
where
    UInt<LIMBS>: ArrayEncoding,
{
    const TAG: Tag = Tag::Integer;
}
