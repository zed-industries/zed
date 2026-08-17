use byteorder::{ByteOrder, LittleEndian};
use bytes::Buf;
use sqlx_core::database::Database;
use time::macros::format_description;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::{BoxDynError, UnexpectedNullError};
use crate::protocol::text::ColumnType;
use crate::type_info::MySqlTypeInfo;
use crate::types::{MySqlTime, MySqlTimeSign, Type};
use crate::{MySql, MySqlValueFormat, MySqlValueRef};

impl Type<MySql> for OffsetDateTime {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Timestamp)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        matches!(ty.r#type, ColumnType::Datetime | ColumnType::Timestamp)
    }
}

impl Encode<'_, MySql> for OffsetDateTime {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        let utc_dt = self.to_offset(UtcOffset::UTC);
        let primitive_dt = PrimitiveDateTime::new(utc_dt.date(), utc_dt.time());

        Encode::<MySql>::encode(primitive_dt, buf)
    }
}

impl<'r> Decode<'r, MySql> for OffsetDateTime {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let primitive: PrimitiveDateTime = Decode::<MySql>::decode(value)?;

        Ok(primitive.assume_utc())
    }
}

impl Type<MySql> for Time {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Time)
    }
}

impl Encode<'_, MySql> for Time {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        let len = time_encoded_len(self);
        buf.push(len);

        // sign byte: Time is never negative
        buf.push(0);

        // Number of days in the interval; always 0 for time-of-day values.
        // https://mariadb.com/kb/en/resultset-row/#teimstamp-binary-encoding
        buf.extend_from_slice(&[0_u8; 4]);

        encode_time(self, len > 8, buf);

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        time_encoded_len(self) as usize + 1 // plus length byte
    }
}

impl<'r> Decode<'r, MySql> for Time {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            MySqlValueFormat::Binary => {
                // Should never panic.
                MySqlTime::decode(value)?.try_into()
            }

            // Retaining this parsing for now as it allows us to cross-check our impl.
            MySqlValueFormat::Text => Time::parse(
                value.as_str()?,
                &format_description!("[hour]:[minute]:[second].[subsecond]"),
            )
            .map_err(Into::into),
        }
    }
}

impl TryFrom<MySqlTime> for Time {
    type Error = BoxDynError;

    fn try_from(time: MySqlTime) -> Result<Self, Self::Error> {
        if !time.is_valid_time_of_day() {
            return Err(format!("MySqlTime value out of range for `time::Time`: {time}").into());
        }

        #[allow(clippy::cast_possible_truncation)]
        Ok(Time::from_hms_micro(
            // `is_valid_time_of_day()` ensures this won't overflow
            time.hours() as u8,
            time.minutes(),
            time.seconds(),
            time.microseconds(),
        )?)
    }
}

impl From<MySqlTime> for time::Duration {
    fn from(time: MySqlTime) -> Self {
        // `subsec_nanos()` is guaranteed to be between 0 and 10^9
        #[allow(clippy::cast_possible_wrap)]
        time::Duration::new(time.whole_seconds_signed(), time.subsec_nanos() as i32)
    }
}

impl TryFrom<time::Duration> for MySqlTime {
    type Error = BoxDynError;

    fn try_from(value: time::Duration) -> Result<Self, Self::Error> {
        let sign = if value.is_negative() {
            MySqlTimeSign::Negative
        } else {
            MySqlTimeSign::Positive
        };

        // Similar to `TryFrom<chrono::TimeDelta>`, use `std::time::Duration` as an intermediate.
        Ok(MySqlTime::try_from(std::time::Duration::try_from(value.abs())?)?.with_sign(sign))
    }
}

impl Type<MySql> for time::Duration {
    fn type_info() -> MySqlTypeInfo {
        MySqlTime::type_info()
    }
}

impl<'r> Decode<'r, MySql> for time::Duration {
    fn decode(value: <MySql as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(MySqlTime::decode(value)?.into())
    }
}

impl Type<MySql> for Date {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Date)
    }
}

impl Encode<'_, MySql> for Date {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.push(4);

        encode_date(self, buf)?;

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        5
    }
}

impl<'r> Decode<'r, MySql> for Date {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            MySqlValueFormat::Binary => {
                let buf = value.as_bytes()?;

                // Row decoding should leave the length byte on the front.
                if buf.is_empty() {
                    return Err("empty buffer".into());
                }

                Ok(decode_date(&buf[1..])?.ok_or(UnexpectedNullError)?)
            }
            MySqlValueFormat::Text => {
                let s = value.as_str()?;
                Date::parse(s, &format_description!("[year]-[month]-[day]")).map_err(Into::into)
            }
        }
    }
}

impl Type<MySql> for PrimitiveDateTime {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Datetime)
    }
}

impl Encode<'_, MySql> for PrimitiveDateTime {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        let len = primitive_dt_encoded_len(self);
        buf.push(len);

        encode_date(&self.date(), buf)?;

        if len > 4 {
            encode_time(&self.time(), len > 7, buf);
        }

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        primitive_dt_encoded_len(self) as usize + 1 // plus length byte
    }
}

impl<'r> Decode<'r, MySql> for PrimitiveDateTime {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            MySqlValueFormat::Binary => {
                let mut buf = value.as_bytes()?;

                if buf.is_empty() {
                    return Err("empty buffer".into());
                }

                let len = buf.get_u8();

                let date = decode_date(buf)?.ok_or(UnexpectedNullError)?;

                let dt = if len > 4 {
                    date.with_time(decode_time(&buf[4..])?)
                } else {
                    date.midnight()
                };

                Ok(dt)
            }

            MySqlValueFormat::Text => {
                let s = value.as_str()?;

                // If there are no nanoseconds parse without them
                if s.contains('.') {
                    PrimitiveDateTime::parse(
                        s,
                        &format_description!(
                            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
                        ),
                    )
                    .map_err(Into::into)
                } else {
                    PrimitiveDateTime::parse(
                        s,
                        &format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
                    )
                    .map_err(Into::into)
                }
            }
        }
    }
}

fn encode_date(date: &Date, buf: &mut Vec<u8>) -> Result<(), BoxDynError> {
    // MySQL supports years from 1000 - 9999
    let year =
        u16::try_from(date.year()).map_err(|_| format!("Date out of range for Mysql: {date}"))?;

    buf.extend_from_slice(&year.to_le_bytes());
    buf.push(date.month().into());
    buf.push(date.day());

    Ok(())
}

fn decode_date(buf: &[u8]) -> Result<Option<Date>, BoxDynError> {
    if buf.is_empty() {
        // zero buffer means a zero date (null)
        return Ok(None);
    }

    Date::from_calendar_date(
        LittleEndian::read_u16(buf) as i32,
        time::Month::try_from(buf[2])?,
        buf[3],
    )
    .map_err(Into::into)
    .map(Some)
}

fn encode_time(time: &Time, include_micros: bool, buf: &mut Vec<u8>) {
    buf.push(time.hour());
    buf.push(time.minute());
    buf.push(time.second());

    if include_micros {
        buf.extend(&(time.nanosecond() / 1000).to_le_bytes());
    }
}

fn decode_time(mut buf: &[u8]) -> Result<Time, BoxDynError> {
    let hour = buf.get_u8();
    let minute = buf.get_u8();
    let seconds = buf.get_u8();

    let micros = if !buf.is_empty() {
        // microseconds : int<EOF>
        buf.get_uint_le(buf.len())
    } else {
        0
    };

    let micros = u32::try_from(micros)
        .map_err(|_| format!("MySQL returned microseconds out of range: {micros}"))?;

    Time::from_hms_micro(hour, minute, seconds, micros)
        .map_err(|e| format!("Time out of range for MySQL: {e}").into())
}

#[inline(always)]
fn primitive_dt_encoded_len(time: &PrimitiveDateTime) -> u8 {
    // to save space the packet can be compressed:
    match (time.hour(), time.minute(), time.second(), time.nanosecond()) {
        // if hour, minutes, seconds and micro_seconds are all 0,
        // length is 4 and no other field is sent
        (0, 0, 0, 0) => 4,

        // if micro_seconds is 0, length is 7
        // and micro_seconds is not sent
        (_, _, _, 0) => 7,

        // otherwise length is 11
        (_, _, _, _) => 11,
    }
}

#[inline(always)]
fn time_encoded_len(time: &Time) -> u8 {
    if time.nanosecond() == 0 {
        // if micro_seconds is 0, length is 8 and micro_seconds is not sent
        8
    } else {
        // otherwise length is 12
        12
    }
}
