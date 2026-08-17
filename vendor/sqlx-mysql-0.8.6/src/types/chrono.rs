use bytes::Buf;
use chrono::{
    DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc,
};
use sqlx_core::database::Database;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::{BoxDynError, UnexpectedNullError};
use crate::protocol::text::ColumnType;
use crate::type_info::MySqlTypeInfo;
use crate::types::{MySqlTime, MySqlTimeSign, Type};
use crate::{MySql, MySqlValueFormat, MySqlValueRef};

impl Type<MySql> for DateTime<Utc> {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Timestamp)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        matches!(ty.r#type, ColumnType::Datetime | ColumnType::Timestamp)
    }
}

/// Note: assumes the connection's `time_zone` is set to `+00:00` (UTC).
impl Encode<'_, MySql> for DateTime<Utc> {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        Encode::<MySql>::encode(self.naive_utc(), buf)
    }
}

/// Note: assumes the connection's `time_zone` is set to `+00:00` (UTC).
impl<'r> Decode<'r, MySql> for DateTime<Utc> {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let naive: NaiveDateTime = Decode::<MySql>::decode(value)?;

        Ok(Utc.from_utc_datetime(&naive))
    }
}

impl Type<MySql> for DateTime<Local> {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Timestamp)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        matches!(ty.r#type, ColumnType::Datetime | ColumnType::Timestamp)
    }
}

/// Note: assumes the connection's `time_zone` is set to `+00:00` (UTC).
impl Encode<'_, MySql> for DateTime<Local> {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        Encode::<MySql>::encode(self.naive_utc(), buf)
    }
}

/// Note: assumes the connection's `time_zone` is set to `+00:00` (UTC).
impl<'r> Decode<'r, MySql> for DateTime<Local> {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(<DateTime<Utc> as Decode<'r, MySql>>::decode(value)?.with_timezone(&Local))
    }
}

impl Type<MySql> for NaiveTime {
    fn type_info() -> MySqlTypeInfo {
        MySqlTime::type_info()
    }
}

impl Encode<'_, MySql> for NaiveTime {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        let len = naive_time_encoded_len(self);
        buf.push(len);

        // NaiveTime is not negative
        buf.push(0);

        // Number of days in the interval; always 0 for time-of-day values.
        // https://mariadb.com/kb/en/resultset-row/#teimstamp-binary-encoding
        buf.extend_from_slice(&[0_u8; 4]);

        encode_time(self, len > 8, buf);

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        naive_time_encoded_len(self) as usize + 1 // plus length byte
    }
}

/// Decode from a `TIME` value.
///
/// ### Errors
/// Returns an error if the `TIME` value is negative or exceeds `23:59:59.999999`.
impl<'r> Decode<'r, MySql> for NaiveTime {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            MySqlValueFormat::Binary => {
                // Covers most possible failure modes.
                MySqlTime::decode(value)?.try_into()
            }
            // Retaining this parsing for now as it allows us to cross-check our impl.
            MySqlValueFormat::Text => {
                let s = value.as_str()?;
                NaiveTime::parse_from_str(s, "%H:%M:%S%.f").map_err(Into::into)
            }
        }
    }
}

impl TryFrom<MySqlTime> for NaiveTime {
    type Error = BoxDynError;

    fn try_from(time: MySqlTime) -> Result<Self, Self::Error> {
        NaiveTime::from_hms_micro_opt(
            time.hours(),
            time.minutes() as u32,
            time.seconds() as u32,
            time.microseconds(),
        )
        .ok_or_else(|| format!("Cannot convert `MySqlTime` value to `NaiveTime`: {time}").into())
    }
}

impl From<MySqlTime> for chrono::TimeDelta {
    fn from(time: MySqlTime) -> Self {
        chrono::TimeDelta::new(time.whole_seconds_signed(), time.subsec_nanos())
            .expect("BUG: chrono::TimeDelta should have a greater range than MySqlTime")
    }
}

impl TryFrom<chrono::TimeDelta> for MySqlTime {
    type Error = BoxDynError;

    fn try_from(value: chrono::TimeDelta) -> Result<Self, Self::Error> {
        let sign = if value < chrono::TimeDelta::zero() {
            MySqlTimeSign::Negative
        } else {
            MySqlTimeSign::Positive
        };

        Ok(
            // `std::time::Duration` has a greater positive range than `TimeDelta`
            // which makes it a great intermediate if you ignore the sign.
            MySqlTime::try_from(value.abs().to_std()?)?.with_sign(sign),
        )
    }
}

impl Type<MySql> for chrono::TimeDelta {
    fn type_info() -> MySqlTypeInfo {
        MySqlTime::type_info()
    }
}

impl<'r> Decode<'r, MySql> for chrono::TimeDelta {
    fn decode(value: <MySql as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(MySqlTime::decode(value)?.into())
    }
}

impl Type<MySql> for NaiveDate {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Date)
    }
}

impl Encode<'_, MySql> for NaiveDate {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.push(4);

        encode_date(self, buf)?;

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        5
    }
}

impl<'r> Decode<'r, MySql> for NaiveDate {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            MySqlValueFormat::Binary => {
                let buf = value.as_bytes()?;

                // Row decoding should have left the length prefix.
                if buf.is_empty() {
                    return Err("empty buffer".into());
                }

                decode_date(&buf[1..])?.ok_or_else(|| UnexpectedNullError.into())
            }

            MySqlValueFormat::Text => {
                let s = value.as_str()?;
                NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(Into::into)
            }
        }
    }
}

impl Type<MySql> for NaiveDateTime {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Datetime)
    }
}

impl Encode<'_, MySql> for NaiveDateTime {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        let len = naive_dt_encoded_len(self);
        buf.push(len);

        encode_date(&self.date(), buf)?;

        if len > 4 {
            encode_time(&self.time(), len > 7, buf);
        }

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        naive_dt_encoded_len(self) as usize + 1 // plus length byte
    }
}

impl<'r> Decode<'r, MySql> for NaiveDateTime {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            MySqlValueFormat::Binary => {
                let buf = value.as_bytes()?;

                if buf.is_empty() {
                    return Err("empty buffer".into());
                }

                let len = buf[0];
                let date = decode_date(&buf[1..])?.ok_or(UnexpectedNullError)?;

                let dt = if len > 4 {
                    date.and_time(decode_time(len - 4, &buf[5..])?)
                } else {
                    date.and_hms_opt(0, 0, 0)
                        .expect("expected `NaiveDate::and_hms_opt(0, 0, 0)` to be valid")
                };

                Ok(dt)
            }

            MySqlValueFormat::Text => {
                let s = value.as_str()?;
                NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f").map_err(Into::into)
            }
        }
    }
}

fn encode_date(date: &NaiveDate, buf: &mut Vec<u8>) -> Result<(), BoxDynError> {
    // MySQL supports years 1000 - 9999
    let year = u16::try_from(date.year())
        .map_err(|_| format!("NaiveDateTime out of range for Mysql: {date}"))?;

    buf.extend_from_slice(&year.to_le_bytes());

    // `NaiveDate` guarantees the ranges of these values
    #[allow(clippy::cast_possible_truncation)]
    {
        buf.push(date.month() as u8);
        buf.push(date.day() as u8);
    }

    Ok(())
}

fn decode_date(mut buf: &[u8]) -> Result<Option<NaiveDate>, BoxDynError> {
    match buf.len() {
        // MySQL specifies that if there are no bytes, this is all zeros
        0 => Ok(None),
        4.. => {
            let year = buf.get_u16_le() as i32;
            let month = buf[0] as u32;
            let day = buf[1] as u32;

            let date = NaiveDate::from_ymd_opt(year, month, day)
                .ok_or_else(|| format!("server returned invalid date: {year}/{month}/{day}"))?;

            Ok(Some(date))
        }
        len => Err(format!("expected at least 4 bytes for date, got {len}").into()),
    }
}

fn encode_time(time: &NaiveTime, include_micros: bool, buf: &mut Vec<u8>) {
    // `NaiveTime` API guarantees the ranges of these values
    #[allow(clippy::cast_possible_truncation)]
    {
        buf.push(time.hour() as u8);
        buf.push(time.minute() as u8);
        buf.push(time.second() as u8);
    }

    if include_micros {
        buf.extend((time.nanosecond() / 1000).to_le_bytes());
    }
}

fn decode_time(len: u8, mut buf: &[u8]) -> Result<NaiveTime, BoxDynError> {
    let hour = buf.get_u8();
    let minute = buf.get_u8();
    let seconds = buf.get_u8();

    let micros = if len > 3 {
        // microseconds : int<EOF>
        buf.get_uint_le(buf.len())
    } else {
        0
    };

    let micros = u32::try_from(micros)
        .map_err(|_| format!("server returned microseconds out of range: {micros}"))?;

    NaiveTime::from_hms_micro_opt(hour as u32, minute as u32, seconds as u32, micros)
        .ok_or_else(|| format!("server returned invalid time: {hour:02}:{minute:02}:{seconds:02}; micros: {micros}").into())
}

#[inline(always)]
fn naive_dt_encoded_len(time: &NaiveDateTime) -> u8 {
    // to save space the packet can be compressed:
    match (
        time.hour(),
        time.minute(),
        time.second(),
        #[allow(deprecated)]
        time.timestamp_subsec_nanos(),
    ) {
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
fn naive_time_encoded_len(time: &NaiveTime) -> u8 {
    if time.nanosecond() == 0 {
        // if micro_seconds is 0, length is 8 and micro_seconds is not sent
        8
    } else {
        // otherwise length is 12
        12
    }
}
