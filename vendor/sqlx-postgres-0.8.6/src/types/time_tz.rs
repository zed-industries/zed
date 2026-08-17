use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::mem;

#[cfg(feature = "time")]
type DefaultTime = ::time::Time;

#[cfg(all(not(feature = "time"), feature = "chrono"))]
type DefaultTime = ::chrono::NaiveTime;

#[cfg(feature = "time")]
type DefaultOffset = ::time::UtcOffset;

#[cfg(all(not(feature = "time"), feature = "chrono"))]
type DefaultOffset = ::chrono::FixedOffset;

/// Represents a moment of time, in a specified timezone.
///
/// # Warning
///
/// `PgTimeTz` provides `TIMETZ` and is supported only for reading from legacy databases.
/// [PostgreSQL recommends] to use `TIMESTAMPTZ` instead.
///
/// [PostgreSQL recommends]: https://wiki.postgresql.org/wiki/Don't_Do_This#Don.27t_use_timetz
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PgTimeTz<Time = DefaultTime, Offset = DefaultOffset> {
    pub time: Time,
    pub offset: Offset,
}

impl<Time, Offset> PgHasArrayType for PgTimeTz<Time, Offset> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::TIMETZ_ARRAY
    }
}

#[cfg(feature = "chrono")]
mod chrono {
    use super::*;
    use ::chrono::{DateTime, Duration, FixedOffset, NaiveTime};

    impl Type<Postgres> for PgTimeTz<NaiveTime, FixedOffset> {
        fn type_info() -> PgTypeInfo {
            PgTypeInfo::TIMETZ
        }
    }

    impl Encode<'_, Postgres> for PgTimeTz<NaiveTime, FixedOffset> {
        fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
            let _: IsNull = <NaiveTime as Encode<'_, Postgres>>::encode(self.time, buf)?;
            let _: IsNull =
                <i32 as Encode<'_, Postgres>>::encode(self.offset.utc_minus_local(), buf)?;

            Ok(IsNull::No)
        }

        fn size_hint(&self) -> usize {
            mem::size_of::<i64>() + mem::size_of::<i32>()
        }
    }

    impl<'r> Decode<'r, Postgres> for PgTimeTz<NaiveTime, FixedOffset> {
        fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
            match value.format() {
                PgValueFormat::Binary => {
                    let mut buf = Cursor::new(value.as_bytes()?);

                    // TIME is encoded as the microseconds since midnight
                    let us = buf.read_i64::<BigEndian>()?;
                    // default is midnight, there is a canary test for this
                    // in `sqlx-postgres/src/types/chrono/time.rs`
                    let time = NaiveTime::default() + Duration::microseconds(us);

                    // OFFSET is encoded as seconds from UTC
                    let offset_seconds = buf.read_i32::<BigEndian>()?;

                    let offset = FixedOffset::west_opt(offset_seconds).ok_or_else(|| {
                        format!(
                            "server returned out-of-range offset for `TIMETZ`: {offset_seconds} seconds"
                        )
                    })?;

                    Ok(PgTimeTz { time, offset })
                }

                PgValueFormat::Text => try_parse_timetz(value.as_str()?),
            }
        }
    }

    fn try_parse_timetz(s: &str) -> Result<PgTimeTz<NaiveTime, FixedOffset>, BoxDynError> {
        let mut tmp = String::with_capacity(11 + s.len());
        tmp.push_str("2001-07-08 ");
        tmp.push_str(s);

        let mut err = None;

        for fmt in &["%Y-%m-%d %H:%M:%S%.f%#z", "%Y-%m-%d %H:%M:%S%.f"] {
            match DateTime::parse_from_str(&tmp, fmt) {
                Ok(dt) => {
                    let time = dt.time();
                    let offset = *dt.offset();

                    return Ok(PgTimeTz { time, offset });
                }

                Err(error) => {
                    err = Some(error);
                }
            }
        }

        Err(err
            .expect("BUG: loop should have set `err` to `Some()` before exiting")
            .into())
    }
}

#[cfg(feature = "time")]
mod time {
    use super::*;
    use ::time::{Duration, Time, UtcOffset};

    impl Type<Postgres> for PgTimeTz<Time, UtcOffset> {
        fn type_info() -> PgTypeInfo {
            PgTypeInfo::TIMETZ
        }
    }

    impl Encode<'_, Postgres> for PgTimeTz<Time, UtcOffset> {
        fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
            let _: IsNull = <Time as Encode<'_, Postgres>>::encode(self.time, buf)?;
            let _: IsNull =
                <i32 as Encode<'_, Postgres>>::encode(-self.offset.whole_seconds(), buf)?;

            Ok(IsNull::No)
        }

        fn size_hint(&self) -> usize {
            mem::size_of::<i64>() + mem::size_of::<i32>()
        }
    }

    impl<'r> Decode<'r, Postgres> for PgTimeTz<Time, UtcOffset> {
        fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
            match value.format() {
                PgValueFormat::Binary => {
                    let mut buf = Cursor::new(value.as_bytes()?);

                    // TIME is encoded as the microseconds since midnight
                    let us = buf.read_i64::<BigEndian>()?;
                    let time = Time::MIDNIGHT + Duration::microseconds(us);

                    // OFFSET is encoded as seconds from UTC
                    let seconds = buf.read_i32::<BigEndian>()?;

                    Ok(PgTimeTz {
                        time,
                        offset: -UtcOffset::from_whole_seconds(seconds)?,
                    })
                }

                PgValueFormat::Text => {
                    // the `time` crate has a limited ability to parse and can't parse the
                    // timezone format
                    Err("reading a `TIMETZ` value in text format is not supported.".into())
                }
            }
        }
    }
}
