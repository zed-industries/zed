use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::time::PG_EPOCH;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};
use std::mem;
use time::macros::format_description;
use time::{Date, Duration};

impl Type<Postgres> for Date {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::DATE
    }
}

impl PgHasArrayType for Date {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::DATE_ARRAY
    }
}

impl Encode<'_, Postgres> for Date {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // DATE is encoded as number of days since epoch (2000-01-01)
        let days: i32 = (*self - PG_EPOCH).whole_days().try_into().map_err(|_| {
            format!("value {self:?} would overflow binary encoding for Postgres DATE")
        })?;
        Encode::<Postgres>::encode(days, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i32>()
    }
}

impl<'r> Decode<'r, Postgres> for Date {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // DATE is encoded as the days since epoch
                let days: i32 = Decode::<Postgres>::decode(value)?;
                PG_EPOCH + Duration::days(days.into())
            }

            PgValueFormat::Text => Date::parse(
                value.as_str()?,
                &format_description!("[year]-[month]-[day]"),
            )?,
        })
    }
}
