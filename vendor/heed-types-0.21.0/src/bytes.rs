use std::borrow::Cow;

use heed_traits::{BoxedError, BytesDecode, BytesEncode};

/// Describes a byte slice `[u8]` that is totally borrowed and doesn't depend on
/// any [memory alignment].
///
/// [memory alignment]: std::mem::align_of()
pub enum Bytes {}

impl<'a> BytesEncode<'a> for Bytes {
    type EItem = [u8];

    fn bytes_encode(item: &'a Self::EItem) -> Result<Cow<'a, [u8]>, BoxedError> {
        Ok(Cow::Borrowed(item))
    }
}

impl<'a> BytesDecode<'a> for Bytes {
    type DItem = &'a [u8];

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, BoxedError> {
        Ok(bytes)
    }
}
