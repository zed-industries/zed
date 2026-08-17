use crate::decode::Decode;
use crate::error::BoxDynError;
use crate::types::PgRecordDecoder;
use crate::types::Type;
use crate::{PgHasArrayType, PgTypeInfo, PgValueRef, Postgres};

macro_rules! impl_type_for_tuple {
    ($( $idx:ident : $T:ident ),*) => {
        impl<$($T,)*> Type<Postgres> for ($($T,)*) {


            #[inline]
            fn type_info() -> PgTypeInfo {
                PgTypeInfo::RECORD
            }
        }

        impl<$($T,)*> PgHasArrayType for ($($T,)*) {


            #[inline]
            fn array_type_info() -> PgTypeInfo {
                PgTypeInfo::RECORD_ARRAY
            }
        }

        impl<'r, $($T,)*> Decode<'r, Postgres> for ($($T,)*)
        where
            $($T: 'r,)*
            $($T: Type<Postgres>,)*
            $($T: for<'a> Decode<'a, Postgres>,)*
        {


            fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
                #[allow(unused)]
                let mut decoder = PgRecordDecoder::new(value)?;

                $(let $idx: $T = decoder.try_decode()?;)*

                Ok(($($idx,)*))
            }
        }
    };
}

impl_type_for_tuple!(_1: T1);

impl_type_for_tuple!(_1: T1, _2: T2);

impl_type_for_tuple!(_1: T1, _2: T2, _3: T3);

impl_type_for_tuple!(_1: T1, _2: T2, _3: T3, _4: T4);

impl_type_for_tuple!(_1: T1, _2: T2, _3: T3, _4: T4, _5: T5);

impl_type_for_tuple!(_1: T1, _2: T2, _3: T3, _4: T4, _5: T5, _6: T6);

impl_type_for_tuple!(_1: T1, _2: T2, _3: T3, _4: T4, _5: T5, _6: T6, _7: T7);

impl_type_for_tuple!(
    _1: T1,
    _2: T2,
    _3: T3,
    _4: T4,
    _5: T5,
    _6: T6,
    _7: T7,
    _8: T8
);

impl_type_for_tuple!(
    _1: T1,
    _2: T2,
    _3: T3,
    _4: T4,
    _5: T5,
    _6: T6,
    _7: T7,
    _8: T8,
    _9: T9
);
