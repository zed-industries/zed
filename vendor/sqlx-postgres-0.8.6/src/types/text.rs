use crate::{PgArgumentBuffer, PgTypeInfo, PgValueRef, Postgres};
use sqlx_core::decode::Decode;
use sqlx_core::encode::{Encode, IsNull};
use sqlx_core::error::BoxDynError;
use sqlx_core::types::{Text, Type};
use std::fmt::Display;
use std::str::FromStr;

use std::io::Write;

impl<T> Type<Postgres> for Text<T> {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <String as Type<Postgres>>::compatible(ty)
    }
}

impl<'q, T> Encode<'q, Postgres> for Text<T>
where
    T: Display,
{
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        write!(**buf, "{}", self.0)?;
        Ok(IsNull::No)
    }
}

impl<'r, T> Decode<'r, Postgres> for Text<T>
where
    T: FromStr,
    BoxDynError: From<<T as FromStr>::Err>,
{
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let s: &str = Decode::<Postgres>::decode(value)?;
        Ok(Self(s.parse()?))
    }
}
