use std::borrow::Cow;
use std::str;

use heed_traits::{BoxedError, BytesDecode, BytesEncode};

/// Describes a [`prim@str`].
pub enum Str {}

impl BytesEncode<'_> for Str {
    type EItem = str;

    fn bytes_encode(item: &Self::EItem) -> Result<Cow<[u8]>, BoxedError> {
        Ok(Cow::Borrowed(item.as_bytes()))
    }
}

impl<'a> BytesDecode<'a> for Str {
    type DItem = &'a str;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, BoxedError> {
        str::from_utf8(bytes).map_err(Into::into)
    }
}
