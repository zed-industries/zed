/// Conversions between `bstr` types and SQL types.
use crate::database::Database;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;

#[doc(no_inline)]
pub use bstr::{BStr, BString, ByteSlice};

impl<DB> Type<DB> for BString
where
    DB: Database,
    [u8]: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <&[u8] as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <&[u8] as Type<DB>>::compatible(ty)
    }
}

impl<'r, DB> Decode<'r, DB> for BString
where
    DB: Database,
    Vec<u8>: Decode<'r, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        <Vec<u8> as Decode<DB>>::decode(value).map(BString::from)
    }
}

impl<'q, DB: Database> Encode<'q, DB> for &'q BStr
where
    DB: Database,
    &'q [u8]: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        <&[u8] as Encode<DB>>::encode(self.as_bytes(), buf)
    }
}

impl<'q, DB: Database> Encode<'q, DB> for BString
where
    DB: Database,
    Vec<u8>: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        <Vec<u8> as Encode<DB>>::encode(self.as_bytes().to_vec(), buf)
    }
}
