//! [`Type`], [`Encode`], and [`Decode`] implementations for the various [`NonZero*`][non-zero]
//! types from the standard library.
//!
//! [non-zero]: core::num::NonZero

use std::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8,
};

use crate::database::Database;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::types::Type;

macro_rules! impl_non_zero {
    ($($int:ty => $non_zero:ty),* $(,)?) => {
        $(impl<DB> Type<DB> for $non_zero
        where
            DB: Database,
            $int: Type<DB>,
        {
            fn type_info() -> <DB as Database>::TypeInfo {
                <$int as Type<DB>>::type_info()
            }

            fn compatible(ty: &<DB as Database>::TypeInfo) -> bool {
                <$int as Type<DB>>::compatible(ty)
            }
        }

        impl<'q, DB> Encode<'q, DB> for $non_zero
        where
            DB: Database,
            $int: Encode<'q, DB>,
        {
            fn encode_by_ref(&self, buf: &mut <DB as Database>::ArgumentBuffer<'q>) -> Result<IsNull, crate::error::BoxDynError> {
                <$int as Encode<'q, DB>>::encode_by_ref(&self.get(), buf)
            }

            fn encode(self, buf: &mut <DB as Database>::ArgumentBuffer<'q>) -> Result<IsNull, crate::error::BoxDynError>
            where
                Self: Sized,
            {
                <$int as Encode<'q, DB>>::encode(self.get(), buf)
            }

            fn produces(&self) -> Option<<DB as Database>::TypeInfo> {
                <$int as Encode<'q, DB>>::produces(&self.get())
            }
        }

        impl<'r, DB> Decode<'r, DB> for $non_zero
        where
            DB: Database,
            $int: Decode<'r, DB>,
        {
            fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, crate::error::BoxDynError> {
                let int = <$int as Decode<'r, DB>>::decode(value)?;
                let non_zero = Self::try_from(int)?;

                Ok(non_zero)
            }
        })*
    };
}

impl_non_zero! {
    i8 => NonZeroI8,
    u8 => NonZeroU8,
    i16 => NonZeroI16,
    u16 => NonZeroU16,
    i32 => NonZeroI32,
    u32 => NonZeroU32,
    i64 => NonZeroI64,
    u64 => NonZeroU64,
}
