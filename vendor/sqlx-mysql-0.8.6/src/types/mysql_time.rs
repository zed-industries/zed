//! The [`MysqlTime`] type.

use crate::protocol::text::ColumnType;
use crate::{MySql, MySqlTypeInfo, MySqlValueFormat};
use bytes::{Buf, BufMut};
use sqlx_core::database::Database;
use sqlx_core::decode::Decode;
use sqlx_core::encode::{Encode, IsNull};
use sqlx_core::error::BoxDynError;
use sqlx_core::types::Type;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Write};
use std::time::Duration;

// Similar to `PgInterval`
/// Container for a MySQL `TIME` value, which may be an interval or a time-of-day.
///
/// Allowed range is `-838:59:59.0` to `838:59:59.0`.
///
/// If this value is used for a time-of-day, the range should be `00:00:00.0` to `23:59:59.999999`.
/// You can use [`Self::is_valid_time_of_day()`] to check this easily.
///
/// * [MySQL Manual 13.2.3: The TIME Type](https://dev.mysql.com/doc/refman/8.3/en/time.html)
/// * [MariaDB Manual: TIME](https://mariadb.com/kb/en/time/)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MySqlTime {
    pub(crate) sign: MySqlTimeSign,
    pub(crate) magnitude: TimeMagnitude,
}

// By using a subcontainer for the actual time magnitude,
// we can still use a derived `Ord` implementation and just flip the comparison for negative values.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct TimeMagnitude {
    pub(crate) hours: u32,
    pub(crate) minutes: u8,
    pub(crate) seconds: u8,
    pub(crate) microseconds: u32,
}

const MAGNITUDE_ZERO: TimeMagnitude = TimeMagnitude {
    hours: 0,
    minutes: 0,
    seconds: 0,
    microseconds: 0,
};

/// Maximum magnitude (positive or negative).
const MAGNITUDE_MAX: TimeMagnitude = TimeMagnitude {
    hours: MySqlTime::HOURS_MAX,
    minutes: 59,
    seconds: 59,
    // Surprisingly this is not 999_999 which is why `MySqlTimeError::SubsecondExcess`.
    microseconds: 0,
};

/// The sign for a [`MySqlTime`] type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MySqlTimeSign {
    // The protocol actually specifies negative as 1 and positive as 0,
    // but by specifying variants this way we can derive `Ord` and it works as expected.
    /// The interval is negative (invalid for time-of-day values).
    Negative,
    /// The interval is positive, or represents a time-of-day.
    Positive,
}

/// Errors returned by [`MySqlTime::new()`].
#[derive(Debug, thiserror::Error)]
pub enum MySqlTimeError {
    /// A field of [`MySqlTime`] exceeded its max range.
    #[error("`MySqlTime` field `{field}` cannot exceed {max}, got {value}")]
    FieldRange {
        field: &'static str,
        max: u32,
        value: u64,
    },
    /// Error returned for time magnitudes (positive or negative) between `838:59:59.0` and `839:00:00.0`.
    ///
    /// Other range errors should be covered by [`Self::FieldRange`] for the `hours` field.
    ///
    /// For applications which can tolerate rounding, a valid truncated value is provided.
    #[error(
        "`MySqlTime` cannot exceed +/-838:59:59.000000; got {sign}838:59:59.{microseconds:06}"
    )]
    SubsecondExcess {
        /// The sign of the magnitude.
        sign: MySqlTimeSign,
        /// The number of microseconds over the maximum.
        microseconds: u32,
        /// The truncated value,
        /// either [`MySqlTime::MIN`] if negative or [`MySqlTime::MAX`] if positive.
        truncated: MySqlTime,
    },
    /// MySQL coerces `-00:00:00` to `00:00:00` but this API considers that an error.
    ///
    /// For applications which can tolerate coercion, you can convert this error to [`MySqlTime::ZERO`].
    #[error("attempted to construct a `MySqlTime` value of negative zero")]
    NegativeZero,
}

impl MySqlTime {
    /// The `MySqlTime` value corresponding to `TIME '0:00:00.0'` (zero).
    pub const ZERO: Self = MySqlTime {
        sign: MySqlTimeSign::Positive,
        magnitude: MAGNITUDE_ZERO,
    };

    /// The `MySqlTime` value corresponding to `TIME '838:59:59.0'` (max value).
    pub const MAX: Self = MySqlTime {
        sign: MySqlTimeSign::Positive,
        magnitude: MAGNITUDE_MAX,
    };

    /// The `MySqlTime` value corresponding to `TIME '-838:59:59.0'` (min value).
    pub const MIN: Self = MySqlTime {
        sign: MySqlTimeSign::Negative,
        // Same magnitude, opposite sign.
        magnitude: MAGNITUDE_MAX,
    };

    // The maximums for the other values are self-evident, but not necessarily this one.
    pub(crate) const HOURS_MAX: u32 = 838;

    /// Construct a [`MySqlTime`] that is valid for use as a `TIME` value.
    ///
    /// ### Errors
    /// * [`MySqlTimeError::NegativeZero`] if all fields are 0 but `sign` is [`MySqlTimeSign::Negative`].
    /// * [`MySqlTimeError::FieldRange`] if any field is out of range:
    ///     * `hours > 838`
    ///     * `minutes > 59`
    ///     * `seconds > 59`
    ///     * `microseconds > 999_999`
    /// * [`MySqlTimeError::SubsecondExcess`] if the magnitude is less than one second over the maximum.
    ///     * Durations 839 hours or greater are covered by `FieldRange`.
    pub fn new(
        sign: MySqlTimeSign,
        hours: u32,
        minutes: u8,
        seconds: u8,
        microseconds: u32,
    ) -> Result<Self, MySqlTimeError> {
        macro_rules! check_fields {
            ($($name:ident: $max:expr),+ $(,)?) => {
                $(
                    if $name > $max {
                        return Err(MySqlTimeError::FieldRange {
                            field: stringify!($name),
                            max: $max as u32,
                            value: $name as u64
                        })
                    }
                )+
            }
        }

        check_fields!(
            hours: Self::HOURS_MAX,
            minutes: 59,
            seconds: 59,
            microseconds: 999_999
        );

        let values = TimeMagnitude {
            hours,
            minutes,
            seconds,
            microseconds,
        };

        if sign.is_negative() && values == MAGNITUDE_ZERO {
            return Err(MySqlTimeError::NegativeZero);
        }

        // This is only `true` if less than 1 second over the maximum magnitude
        if values > MAGNITUDE_MAX {
            return Err(MySqlTimeError::SubsecondExcess {
                sign,
                microseconds,
                truncated: if sign.is_positive() {
                    Self::MAX
                } else {
                    Self::MIN
                },
            });
        }

        Ok(Self {
            sign,
            magnitude: values,
        })
    }

    /// Update the `sign` of this value.
    pub fn with_sign(self, sign: MySqlTimeSign) -> Self {
        Self { sign, ..self }
    }

    /// Return the sign (positive or negative) for this TIME value.
    pub fn sign(&self) -> MySqlTimeSign {
        self.sign
    }

    /// Returns `true` if `self` is zero (equal to [`Self::ZERO`]).
    pub fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }

    /// Returns `true` if `self` is positive or zero, `false` if negative.
    pub fn is_positive(&self) -> bool {
        self.sign.is_positive()
    }

    /// Returns `true` if `self` is negative, `false` if positive or zero.
    pub fn is_negative(&self) -> bool {
        self.sign.is_positive()
    }

    /// Returns `true` if this interval is a valid time-of-day.
    ///
    /// If `true`, the sign is positive and `hours` is not greater than 23.
    pub fn is_valid_time_of_day(&self) -> bool {
        self.sign.is_positive() && self.hours() < 24
    }

    /// Get the total number of hours in this interval, from 0 to 838.
    ///
    /// If this value represents a time-of-day, the range is 0 to 23.
    pub fn hours(&self) -> u32 {
        self.magnitude.hours
    }

    /// Get the number of minutes in this interval, from 0 to 59.
    pub fn minutes(&self) -> u8 {
        self.magnitude.minutes
    }

    /// Get the number of seconds in this interval, from 0 to 59.
    pub fn seconds(&self) -> u8 {
        self.magnitude.seconds
    }

    /// Get the number of seconds in this interval, from 0 to 999,999.
    pub fn microseconds(&self) -> u32 {
        self.magnitude.microseconds
    }

    /// Convert this TIME value to a [`std::time::Duration`].
    ///
    /// Returns `None` if this value is negative (cannot be represented).
    pub fn to_duration(&self) -> Option<Duration> {
        self.is_positive()
            .then(|| Duration::new(self.whole_seconds() as u64, self.subsec_nanos()))
    }

    /// Get the whole number of seconds (`seconds + (minutes * 60) + (hours * 3600)`) in this time.
    ///
    /// Sign is ignored.
    pub(crate) fn whole_seconds(&self) -> u32 {
        // If `hours` does not exceed 838 then this cannot overflow.
        self.hours() * 3600 + self.minutes() as u32 * 60 + self.seconds() as u32
    }

    #[cfg_attr(not(any(feature = "time", feature = "chrono")), allow(dead_code))]
    pub(crate) fn whole_seconds_signed(&self) -> i64 {
        self.whole_seconds() as i64 * self.sign.signum() as i64
    }

    pub(crate) fn subsec_nanos(&self) -> u32 {
        self.microseconds() * 1000
    }

    fn encoded_len(&self) -> u8 {
        if self.is_zero() {
            0
        } else if self.microseconds() == 0 {
            8
        } else {
            12
        }
    }
}

impl PartialOrd<MySqlTime> for MySqlTime {
    fn partial_cmp(&self, other: &MySqlTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MySqlTime {
    fn cmp(&self, other: &Self) -> Ordering {
        // If the sides have different signs, we just need to compare those.
        if self.sign != other.sign {
            return self.sign.cmp(&other.sign);
        }

        // We've checked that both sides have the same sign
        match self.sign {
            MySqlTimeSign::Positive => self.magnitude.cmp(&other.magnitude),
            // Reverse the comparison for negative values (smaller negative magnitude = greater)
            MySqlTimeSign::Negative => other.magnitude.cmp(&self.magnitude),
        }
    }
}

impl Display for MySqlTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let TimeMagnitude {
            hours,
            minutes,
            seconds,
            microseconds,
        } = self.magnitude;

        // Obeys the `+` flag.
        Display::fmt(&self.sign(), f)?;

        write!(f, "{hours}:{minutes:02}:{seconds:02}")?;

        // Write microseconds if not zero or a nonzero precision was explicitly requested.
        if f.precision().map_or(microseconds != 0, |it| it != 0) {
            f.write_char('.')?;

            let mut remaining_precision = f.precision();
            let mut remainder = microseconds;
            let mut power_of_10 = 10u32.pow(5);

            // Write digits from most-significant to least, up to the requested precision.
            while remainder > 0 && remaining_precision != Some(0) {
                let digit = remainder / power_of_10;
                // 1 % 1 = 0
                remainder %= power_of_10;
                power_of_10 /= 10;

                write!(f, "{digit}")?;

                if let Some(remaining_precision) = &mut remaining_precision {
                    *remaining_precision = remaining_precision.saturating_sub(1);
                }
            }

            // If any requested precision remains, pad with zeroes.
            if let Some(precision) = remaining_precision.filter(|it| *it != 0) {
                write!(f, "{:0precision$}", 0)?;
            }
        }

        Ok(())
    }
}

impl Type<MySql> for MySqlTime {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Time)
    }
}

impl<'r> Decode<'r, MySql> for MySqlTime {
    fn decode(value: <MySql as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            MySqlValueFormat::Binary => {
                let mut buf = value.as_bytes()?;

                // Row decoding should have left the length byte on the front.
                if buf.is_empty() {
                    return Err("empty buffer".into());
                }

                let length = buf.get_u8();

                // MySQL specifies that if all fields are 0 then the length is 0 and no further data is sent
                // https://dev.mysql.com/doc/internals/en/binary-protocol-value.html
                if length == 0 {
                    return Ok(Self::ZERO);
                }

                if !matches!(buf.len(), 8 | 12) {
                    return Err(format!(
                        "expected 8 or 12 bytes for TIME value, got {}",
                        buf.len()
                    )
                    .into());
                }

                let sign = MySqlTimeSign::from_byte(buf.get_u8())?;
                // The wire protocol includes days but the text format doesn't. Isn't that crazy?
                let days = buf.get_u32_le();
                let hours = buf.get_u8();
                let minutes = buf.get_u8();
                let seconds = buf.get_u8();

                let microseconds = if !buf.is_empty() { buf.get_u32_le() } else { 0 };

                let whole_hours = days
                    .checked_mul(24)
                    .and_then(|days_to_hours| days_to_hours.checked_add(hours as u32))
                    .ok_or("overflow calculating whole hours from `days * 24 + hours`")?;

                Ok(Self::new(
                    sign,
                    whole_hours,
                    minutes,
                    seconds,
                    microseconds,
                )?)
            }
            MySqlValueFormat::Text => parse(value.as_str()?),
        }
    }
}

impl<'q> Encode<'q, MySql> for MySqlTime {
    fn encode_by_ref(
        &self,
        buf: &mut <MySql as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        if self.is_zero() {
            buf.put_u8(0);
            return Ok(IsNull::No);
        }

        buf.put_u8(self.encoded_len());
        buf.put_u8(self.sign.to_byte());

        let TimeMagnitude {
            hours: whole_hours,
            minutes,
            seconds,
            microseconds,
        } = self.magnitude;

        let days = whole_hours / 24;
        let hours = (whole_hours % 24) as u8;

        buf.put_u32_le(days);
        buf.put_u8(hours);
        buf.put_u8(minutes);
        buf.put_u8(seconds);

        if microseconds != 0 {
            buf.put_u32_le(microseconds);
        }

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        self.encoded_len() as usize + 1
    }
}

/// Convert [`MySqlTime`] from [`std::time::Duration`].
///
/// ### Note: Precision Truncation
/// [`Duration`] supports nanosecond precision, but MySQL `TIME` values only support microsecond
/// precision.
///
/// For simplicity, higher precision values are truncated when converting.
/// If you prefer another rounding mode instead, you should apply that to the `Duration` first.
///
/// See also: [MySQL Manual, section 13.2.6: Fractional Seconds in Time Values](https://dev.mysql.com/doc/refman/8.3/en/fractional-seconds.html)
///
/// ### Errors:
/// Returns [`MySqlTimeError::FieldRange`] if the given duration is longer than `838:59:59.999999`.
///
impl TryFrom<Duration> for MySqlTime {
    type Error = MySqlTimeError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        let hours = value.as_secs() / 3600;
        let rem_seconds = value.as_secs() % 3600;
        let minutes = (rem_seconds / 60) as u8;
        let seconds = (rem_seconds % 60) as u8;

        // Simply divides by 1000
        let microseconds = value.subsec_micros();

        Self::new(
            MySqlTimeSign::Positive,
            hours.try_into().map_err(|_| MySqlTimeError::FieldRange {
                field: "hours",
                max: Self::HOURS_MAX,
                value: hours,
            })?,
            minutes,
            seconds,
            microseconds,
        )
    }
}

impl MySqlTimeSign {
    fn from_byte(b: u8) -> Result<Self, BoxDynError> {
        match b {
            0 => Ok(Self::Positive),
            1 => Ok(Self::Negative),
            other => Err(format!("expected 0 or 1 for TIME sign byte, got {other}").into()),
        }
    }

    fn to_byte(self) -> u8 {
        match self {
            // We can't use `#[repr(u8)]` because this is opposite of the ordering we want from `Ord`
            Self::Negative => 1,
            Self::Positive => 0,
        }
    }

    fn signum(&self) -> i32 {
        match self {
            Self::Negative => -1,
            Self::Positive => 1,
        }
    }

    /// Returns `true` if positive, `false` if negative.
    pub fn is_positive(&self) -> bool {
        matches!(self, Self::Positive)
    }

    /// Returns `true` if negative, `false` if positive.
    pub fn is_negative(&self) -> bool {
        matches!(self, Self::Negative)
    }
}

impl Display for MySqlTimeSign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positive if f.sign_plus() => f.write_char('+'),
            Self::Negative => f.write_char('-'),
            _ => Ok(()),
        }
    }
}

impl Type<MySql> for Duration {
    fn type_info() -> MySqlTypeInfo {
        MySqlTime::type_info()
    }
}

impl<'r> Decode<'r, MySql> for Duration {
    fn decode(value: <MySql as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let time = MySqlTime::decode(value)?;

        time.to_duration().ok_or_else(|| {
            format!("`std::time::Duration` can only decode positive TIME values; got {time}").into()
        })
    }
}

// Not exposing this as a `FromStr` impl currently because `MySqlTime` is not designed to be
// a general interchange type.
fn parse(text: &str) -> Result<MySqlTime, BoxDynError> {
    let mut segments = text.split(':');

    let hours = segments
        .next()
        .ok_or("expected hours segment, got nothing")?;

    let minutes = segments
        .next()
        .ok_or("expected minutes segment, got nothing")?;

    let seconds = segments
        .next()
        .ok_or("expected seconds segment, got nothing")?;

    // Include the sign in parsing for convenience;
    // the allowed range of whole hours is much smaller than `i32`'s positive range.
    let hours: i32 = hours
        .parse()
        .map_err(|e| format!("error parsing hours from {text:?} (segment {hours:?}): {e}"))?;

    let sign = if hours.is_negative() {
        MySqlTimeSign::Negative
    } else {
        MySqlTimeSign::Positive
    };

    let hours = hours.unsigned_abs();

    let minutes: u8 = minutes
        .parse()
        .map_err(|e| format!("error parsing minutes from {text:?} (segment {minutes:?}): {e}"))?;

    let (seconds, microseconds): (u8, u32) = if let Some((seconds, microseconds)) =
        seconds.split_once('.')
    {
        (
            seconds.parse().map_err(|e| {
                format!("error parsing seconds from {text:?} (segment {seconds:?}): {e}")
            })?,
            parse_microseconds(microseconds).map_err(|e| {
                format!("error parsing microseconds from {text:?} (segment {microseconds:?}): {e}")
            })?,
        )
    } else {
        (
            seconds.parse().map_err(|e| {
                format!("error parsing seconds from {text:?} (segment {seconds:?}): {e}")
            })?,
            0,
        )
    };

    Ok(MySqlTime::new(sign, hours, minutes, seconds, microseconds)?)
}

/// Parse microseconds from a fractional seconds string.
fn parse_microseconds(micros: &str) -> Result<u32, BoxDynError> {
    const EXPECTED_DIGITS: usize = 6;

    match micros.len() {
        0 => Err("empty string".into()),
        len @ ..=EXPECTED_DIGITS => {
            // Fewer than 6 digits, multiply to the correct magnitude
            let micros: u32 = micros.parse()?;
            // cast cannot overflow
            #[allow(clippy::cast_possible_truncation)]
            Ok(micros * 10u32.pow((EXPECTED_DIGITS - len) as u32))
        }
        // More digits than expected, truncate
        _ => Ok(micros[..EXPECTED_DIGITS].parse()?),
    }
}

#[cfg(test)]
mod tests {
    use super::MySqlTime;
    use crate::types::MySqlTimeSign;

    use super::parse_microseconds;

    #[test]
    fn test_display() {
        assert_eq!(MySqlTime::ZERO.to_string(), "0:00:00");

        assert_eq!(format!("{:.0}", MySqlTime::ZERO), "0:00:00");

        assert_eq!(format!("{:.3}", MySqlTime::ZERO), "0:00:00.000");

        assert_eq!(format!("{:.6}", MySqlTime::ZERO), "0:00:00.000000");

        assert_eq!(format!("{:.9}", MySqlTime::ZERO), "0:00:00.000000000");

        assert_eq!(format!("{:.0}", MySqlTime::MAX), "838:59:59");

        assert_eq!(format!("{:.3}", MySqlTime::MAX), "838:59:59.000");

        assert_eq!(format!("{:.6}", MySqlTime::MAX), "838:59:59.000000");

        assert_eq!(format!("{:.9}", MySqlTime::MAX), "838:59:59.000000000");

        assert_eq!(format!("{:+.0}", MySqlTime::MAX), "+838:59:59");

        assert_eq!(format!("{:+.3}", MySqlTime::MAX), "+838:59:59.000");

        assert_eq!(format!("{:+.6}", MySqlTime::MAX), "+838:59:59.000000");

        assert_eq!(format!("{:+.9}", MySqlTime::MAX), "+838:59:59.000000000");

        assert_eq!(format!("{:.0}", MySqlTime::MIN), "-838:59:59");

        assert_eq!(format!("{:.3}", MySqlTime::MIN), "-838:59:59.000");

        assert_eq!(format!("{:.6}", MySqlTime::MIN), "-838:59:59.000000");

        assert_eq!(format!("{:.9}", MySqlTime::MIN), "-838:59:59.000000000");

        let positive = MySqlTime::new(MySqlTimeSign::Positive, 123, 45, 56, 890011).unwrap();

        assert_eq!(positive.to_string(), "123:45:56.890011");
        assert_eq!(format!("{positive:.0}"), "123:45:56");
        assert_eq!(format!("{positive:.3}"), "123:45:56.890");
        assert_eq!(format!("{positive:.6}"), "123:45:56.890011");
        assert_eq!(format!("{positive:.9}"), "123:45:56.890011000");

        assert_eq!(format!("{positive:+.0}"), "+123:45:56");
        assert_eq!(format!("{positive:+.3}"), "+123:45:56.890");
        assert_eq!(format!("{positive:+.6}"), "+123:45:56.890011");
        assert_eq!(format!("{positive:+.9}"), "+123:45:56.890011000");

        let negative = MySqlTime::new(MySqlTimeSign::Negative, 123, 45, 56, 890011).unwrap();

        assert_eq!(negative.to_string(), "-123:45:56.890011");
        assert_eq!(format!("{negative:.0}"), "-123:45:56");
        assert_eq!(format!("{negative:.3}"), "-123:45:56.890");
        assert_eq!(format!("{negative:.6}"), "-123:45:56.890011");
        assert_eq!(format!("{negative:.9}"), "-123:45:56.890011000");
    }

    #[test]
    fn test_parse_microseconds() {
        assert_eq!(parse_microseconds("010").unwrap(), 10_000);

        assert_eq!(parse_microseconds("0100000000").unwrap(), 10_000);

        assert_eq!(parse_microseconds("890").unwrap(), 890_000);

        assert_eq!(parse_microseconds("0890").unwrap(), 89_000);

        assert_eq!(
            // Case in point about not exposing this:
            // we always truncate excess precision because it's simpler than rounding
            // and MySQL should never return a higher precision.
            parse_microseconds("123456789").unwrap(),
            123456,
        );
    }
}
