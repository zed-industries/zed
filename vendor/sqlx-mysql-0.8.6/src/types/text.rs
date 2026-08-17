use crate::{MySql, MySqlTypeInfo, MySqlValueRef};
use sqlx_core::decode::Decode;
use sqlx_core::encode::{Encode, IsNull};
use sqlx_core::error::BoxDynError;
use sqlx_core::types::{Text, Type};
use std::fmt::Display;
use std::str::FromStr;

impl<T> Type<MySql> for Text<T> {
    fn type_info() -> MySqlTypeInfo {
        <String as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <String as Type<MySql>>::compatible(ty)
    }
}

impl<'q, T> Encode<'q, MySql> for Text<T>
where
    T: Display,
{
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        // We can't really do the trick like with Postgres where we reserve the space for the
        // length up-front and then overwrite it later, because MySQL appears to enforce that
        // length-encoded integers use the smallest encoding for the value:
        // https://dev.mysql.com/doc/dev/mysql-server/latest/page_protocol_basic_dt_integers.html#sect_protocol_basic_dt_int_le
        //
        // So we'd have to reserve space for the max-width encoding, format into the buffer,
        // then figure out how many bytes our length-encoded integer needs to be and move the
        // value bytes down to use up the empty space.
        //
        // Copying from a completely separate buffer instead is easier. It may or may not be faster
        // or slower depending on a ton of different variables, but I don't currently have the time
        // to implement both approaches and compare their performance.
        Encode::<MySql>::encode(self.0.to_string(), buf)
    }
}

impl<'r, T> Decode<'r, MySql> for Text<T>
where
    T: FromStr,
    BoxDynError: From<<T as FromStr>::Err>,
{
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let s: &str = Decode::<MySql>::decode(value)?;
        Ok(Self(s.parse()?))
    }
}
