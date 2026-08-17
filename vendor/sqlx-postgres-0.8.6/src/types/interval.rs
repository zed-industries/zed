use std::mem;

use byteorder::{NetworkEndian, ReadBytesExt};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};

// `PgInterval` is available for direct access to the INTERVAL type

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Default)]
pub struct PgInterval {
    pub months: i32,
    pub days: i32,
    pub microseconds: i64,
}

impl Type<Postgres> for PgInterval {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::INTERVAL
    }
}

impl PgHasArrayType for PgInterval {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::INTERVAL_ARRAY
    }
}

impl<'de> Decode<'de, Postgres> for PgInterval {
    fn decode(value: PgValueRef<'de>) -> Result<Self, BoxDynError> {
        match value.format() {
            PgValueFormat::Binary => {
                let mut buf = value.as_bytes()?;
                let microseconds = buf.read_i64::<NetworkEndian>()?;
                let days = buf.read_i32::<NetworkEndian>()?;
                let months = buf.read_i32::<NetworkEndian>()?;

                Ok(PgInterval {
                    months,
                    days,
                    microseconds,
                })
            }

            // TODO: Implement parsing of text mode
            PgValueFormat::Text => {
                Err("not implemented: decode `INTERVAL` in text mode (unprepared queries)".into())
            }
        }
    }
}

impl Encode<'_, Postgres> for PgInterval {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.microseconds.to_be_bytes());
        buf.extend(&self.days.to_be_bytes());
        buf.extend(&self.months.to_be_bytes());

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        2 * mem::size_of::<i64>()
    }
}

// We then implement Encode + Type for std Duration, chrono Duration, and time Duration
// This is to enable ease-of-use for encoding when its simple

impl Type<Postgres> for std::time::Duration {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::INTERVAL
    }
}

impl PgHasArrayType for std::time::Duration {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::INTERVAL_ARRAY
    }
}

impl Encode<'_, Postgres> for std::time::Duration {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        PgInterval::try_from(*self)?.encode_by_ref(buf)
    }

    fn size_hint(&self) -> usize {
        2 * mem::size_of::<i64>()
    }
}

impl TryFrom<std::time::Duration> for PgInterval {
    type Error = BoxDynError;

    /// Convert a `std::time::Duration` to a `PgInterval`
    ///
    /// This returns an error if there is a loss of precision using nanoseconds or if there is a
    /// microsecond overflow.
    fn try_from(value: std::time::Duration) -> Result<Self, BoxDynError> {
        if value.as_nanos() % 1000 != 0 {
            return Err("PostgreSQL `INTERVAL` does not support nanoseconds precision".into());
        }

        Ok(Self {
            months: 0,
            days: 0,
            microseconds: value.as_micros().try_into()?,
        })
    }
}

#[cfg(feature = "chrono")]
impl Type<Postgres> for chrono::Duration {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::INTERVAL
    }
}

#[cfg(feature = "chrono")]
impl PgHasArrayType for chrono::Duration {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::INTERVAL_ARRAY
    }
}

#[cfg(feature = "chrono")]
impl Encode<'_, Postgres> for chrono::Duration {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let pg_interval = PgInterval::try_from(*self)?;
        pg_interval.encode_by_ref(buf)
    }

    fn size_hint(&self) -> usize {
        2 * mem::size_of::<i64>()
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<chrono::Duration> for PgInterval {
    type Error = BoxDynError;

    /// Convert a `chrono::Duration` to a `PgInterval`.
    ///
    /// This returns an error if there is a loss of precision using nanoseconds or if there is a
    /// nanosecond overflow.
    fn try_from(value: chrono::Duration) -> Result<Self, BoxDynError> {
        value
            .num_nanoseconds()
            .map_or::<Result<_, Self::Error>, _>(
                Err("Overflow has occurred for PostgreSQL `INTERVAL`".into()),
                |nanoseconds| {
                    if nanoseconds % 1000 != 0 {
                        return Err(
                            "PostgreSQL `INTERVAL` does not support nanoseconds precision".into(),
                        );
                    }
                    Ok(())
                },
            )?;

        value.num_microseconds().map_or(
            Err("Overflow has occurred for PostgreSQL `INTERVAL`".into()),
            |microseconds| {
                Ok(Self {
                    months: 0,
                    days: 0,
                    microseconds,
                })
            },
        )
    }
}

#[cfg(feature = "time")]
impl Type<Postgres> for time::Duration {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::INTERVAL
    }
}

#[cfg(feature = "time")]
impl PgHasArrayType for time::Duration {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::INTERVAL_ARRAY
    }
}

#[cfg(feature = "time")]
impl Encode<'_, Postgres> for time::Duration {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let pg_interval = PgInterval::try_from(*self)?;
        pg_interval.encode_by_ref(buf)
    }

    fn size_hint(&self) -> usize {
        2 * mem::size_of::<i64>()
    }
}

#[cfg(feature = "time")]
impl TryFrom<time::Duration> for PgInterval {
    type Error = BoxDynError;

    /// Convert a `time::Duration` to a `PgInterval`.
    ///
    /// This returns an error if there is a loss of precision using nanoseconds or if there is a
    /// microsecond overflow.
    fn try_from(value: time::Duration) -> Result<Self, BoxDynError> {
        if value.whole_nanoseconds() % 1000 != 0 {
            return Err("PostgreSQL `INTERVAL` does not support nanoseconds precision".into());
        }

        Ok(Self {
            months: 0,
            days: 0,
            microseconds: value.whole_microseconds().try_into()?,
        })
    }
}

#[test]
fn test_encode_interval() {
    let mut buf = PgArgumentBuffer::default();

    let interval = PgInterval {
        months: 0,
        days: 0,
        microseconds: 0,
    };
    assert!(matches!(
        Encode::<Postgres>::encode(&interval, &mut buf),
        Ok(IsNull::No)
    ));
    assert_eq!(&**buf, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    let interval = PgInterval {
        months: 0,
        days: 0,
        microseconds: 1_000,
    };
    assert!(matches!(
        Encode::<Postgres>::encode(&interval, &mut buf),
        Ok(IsNull::No)
    ));
    assert_eq!(&**buf, [0, 0, 0, 0, 0, 0, 3, 232, 0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    let interval = PgInterval {
        months: 0,
        days: 0,
        microseconds: 1_000_000,
    };
    assert!(matches!(
        Encode::<Postgres>::encode(&interval, &mut buf),
        Ok(IsNull::No)
    ));
    assert_eq!(&**buf, [0, 0, 0, 0, 0, 15, 66, 64, 0, 0, 0, 0, 0, 0, 0, 0]);
    buf.clear();

    let interval = PgInterval {
        months: 0,
        days: 0,
        microseconds: 3_600_000_000,
    };
    assert!(matches!(
        Encode::<Postgres>::encode(&interval, &mut buf),
        Ok(IsNull::No)
    ));
    assert_eq!(
        &**buf,
        [0, 0, 0, 0, 214, 147, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    buf.clear();

    let interval = PgInterval {
        months: 0,
        days: 1,
        microseconds: 0,
    };
    assert!(matches!(
        Encode::<Postgres>::encode(&interval, &mut buf),
        Ok(IsNull::No)
    ));
    assert_eq!(&**buf, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0]);
    buf.clear();

    let interval = PgInterval {
        months: 1,
        days: 0,
        microseconds: 0,
    };
    assert!(matches!(
        Encode::<Postgres>::encode(&interval, &mut buf),
        Ok(IsNull::No)
    ));
    assert_eq!(&**buf, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    buf.clear();

    assert_eq!(
        PgInterval::default(),
        PgInterval {
            months: 0,
            days: 0,
            microseconds: 0,
        }
    );
}

#[test]
fn test_pginterval_std() {
    // Case for positive duration
    let interval = PgInterval {
        days: 0,
        months: 0,
        microseconds: 27_000,
    };
    assert_eq!(
        &PgInterval::try_from(std::time::Duration::from_micros(27_000)).unwrap(),
        &interval
    );

    // Case when precision loss occurs
    assert!(PgInterval::try_from(std::time::Duration::from_nanos(27_000_001)).is_err());

    // Case when microsecond overflow occurs
    assert!(PgInterval::try_from(std::time::Duration::from_secs(20_000_000_000_000)).is_err());
}

#[test]
#[cfg(feature = "chrono")]
fn test_pginterval_chrono() {
    // Case for positive duration
    let interval = PgInterval {
        days: 0,
        months: 0,
        microseconds: 27_000,
    };
    assert_eq!(
        &PgInterval::try_from(chrono::Duration::microseconds(27_000)).unwrap(),
        &interval
    );

    // Case for negative duration
    let interval = PgInterval {
        days: 0,
        months: 0,
        microseconds: -27_000,
    };
    assert_eq!(
        &PgInterval::try_from(chrono::Duration::microseconds(-27_000)).unwrap(),
        &interval
    );

    // Case when precision loss occurs
    assert!(PgInterval::try_from(chrono::Duration::nanoseconds(27_000_001)).is_err());
    assert!(PgInterval::try_from(chrono::Duration::nanoseconds(-27_000_001)).is_err());

    // Case when nanosecond overflow occurs
    assert!(PgInterval::try_from(chrono::Duration::seconds(10_000_000_000)).is_err());
    assert!(PgInterval::try_from(chrono::Duration::seconds(-10_000_000_000)).is_err());
}

#[test]
#[cfg(feature = "time")]
fn test_pginterval_time() {
    // Case for positive duration
    let interval = PgInterval {
        days: 0,
        months: 0,
        microseconds: 27_000,
    };
    assert_eq!(
        &PgInterval::try_from(time::Duration::microseconds(27_000)).unwrap(),
        &interval
    );

    // Case for negative duration
    let interval = PgInterval {
        days: 0,
        months: 0,
        microseconds: -27_000,
    };
    assert_eq!(
        &PgInterval::try_from(time::Duration::microseconds(-27_000)).unwrap(),
        &interval
    );

    // Case when precision loss occurs
    assert!(PgInterval::try_from(time::Duration::nanoseconds(27_000_001)).is_err());
    assert!(PgInterval::try_from(time::Duration::nanoseconds(-27_000_001)).is_err());

    // Case when microsecond overflow occurs
    assert!(PgInterval::try_from(time::Duration::seconds(10_000_000_000_000)).is_err());
    assert!(PgInterval::try_from(time::Duration::seconds(-10_000_000_000_000)).is_err());
}
