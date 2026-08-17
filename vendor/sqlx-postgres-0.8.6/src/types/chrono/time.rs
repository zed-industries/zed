use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};
use chrono::{Duration, NaiveTime};
use std::mem;

impl Type<Postgres> for NaiveTime {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::TIME
    }
}

impl PgHasArrayType for NaiveTime {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::TIME_ARRAY
    }
}

impl Encode<'_, Postgres> for NaiveTime {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // TIME is encoded as the microseconds since midnight
        let micros = (*self - NaiveTime::default())
            .num_microseconds()
            .ok_or_else(|| format!("Time out of range for PostgreSQL: {self}"))?;

        Encode::<Postgres>::encode(micros, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<u64>()
    }
}

impl<'r> Decode<'r, Postgres> for NaiveTime {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIME is encoded as the microseconds since midnight
                let us: i64 = Decode::<Postgres>::decode(value)?;
                NaiveTime::default() + Duration::microseconds(us)
            }

            PgValueFormat::Text => NaiveTime::parse_from_str(value.as_str()?, "%H:%M:%S%.f")?,
        })
    }
}

#[test]
fn check_naive_time_default_is_midnight() {
    // Just a canary in case this changes.
    assert_eq!(
        NaiveTime::from_hms_opt(0, 0, 0),
        Some(NaiveTime::default()),
        "implementation assumes `NaiveTime::default()` equals midnight"
    );
}
