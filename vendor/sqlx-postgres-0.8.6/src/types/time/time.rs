use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};
use std::mem;
use time::macros::format_description;
use time::{Duration, Time};

impl Type<Postgres> for Time {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::TIME
    }
}

impl PgHasArrayType for Time {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::TIME_ARRAY
    }
}

impl Encode<'_, Postgres> for Time {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // TIME is encoded as the microseconds since midnight.
        //
        // A truncating cast is fine because `self - Time::MIDNIGHT` cannot exceed a span of 24 hours.
        #[allow(clippy::cast_possible_truncation)]
        let micros: i64 = (*self - Time::MIDNIGHT).whole_microseconds() as i64;
        Encode::<Postgres>::encode(micros, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<u64>()
    }
}

impl<'r> Decode<'r, Postgres> for Time {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIME is encoded as the microseconds since midnight
                let us = Decode::<Postgres>::decode(value)?;
                Time::MIDNIGHT + Duration::microseconds(us)
            }

            PgValueFormat::Text => Time::parse(
                value.as_str()?,
                // Postgres will not include the subsecond part if it's zero.
                &format_description!("[hour]:[minute]:[second][optional [.[subsecond]]]"),
            )?,
        })
    }
}
