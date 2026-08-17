//! Helpers for implementing formatting for ISO 8601.

use std::io;

use crate::convert::*;
use crate::error;
use crate::format_description::well_known::Iso8601;
use crate::format_description::well_known::iso8601::{
    DateKind, EncodedConfig, OffsetPrecision, TimePrecision,
};
use crate::formatting::{
    ComponentProvider, format_float, format_number_pad_zero, write, write_if, write_if_else,
};

/// Format the date portion of ISO 8601.
pub(super) fn format_date<V, const CONFIG: EncodedConfig>(
    output: &mut (impl io::Write + ?Sized),
    value: &V,
    state: &mut V::State,
) -> Result<usize, error::Format>
where
    V: ComponentProvider,
{
    let mut bytes = 0;

    match Iso8601::<CONFIG>::DATE_KIND {
        DateKind::Calendar => {
            let year = value.calendar_year(state);

            if Iso8601::<CONFIG>::YEAR_IS_SIX_DIGITS {
                bytes += write_if_else(output, year < 0, b"-", b"+")?;
                bytes += format_number_pad_zero::<6>(output, year.unsigned_abs())?;
            } else if !(0..=9999).contains(&year) {
                return Err(error::Format::InvalidComponent("year"));
            } else {
                bytes += format_number_pad_zero::<4>(output, year.cast_unsigned())?;
            }
            bytes += write_if(output, Iso8601::<CONFIG>::USE_SEPARATORS, b"-")?;
            bytes += format_number_pad_zero::<2>(output, u8::from(value.month(state)))?;
            bytes += write_if(output, Iso8601::<CONFIG>::USE_SEPARATORS, b"-")?;
            bytes += format_number_pad_zero::<2>(output, value.day(state))?;
        }
        DateKind::Week => {
            let year = value.iso_year(state);

            if Iso8601::<CONFIG>::YEAR_IS_SIX_DIGITS {
                bytes += write_if_else(output, year < 0, b"-", b"+")?;
                bytes += format_number_pad_zero::<6>(output, year.unsigned_abs())?;
            } else if !(0..=9999).contains(&year) {
                return Err(error::Format::InvalidComponent("year"));
            } else {
                bytes += format_number_pad_zero::<4>(output, year.cast_unsigned())?;
            }
            bytes += write_if_else(output, Iso8601::<CONFIG>::USE_SEPARATORS, b"-W", b"W")?;
            bytes += format_number_pad_zero::<2>(output, value.iso_week_number(state))?;
            bytes += write_if(output, Iso8601::<CONFIG>::USE_SEPARATORS, b"-")?;
            bytes +=
                format_number_pad_zero::<1>(output, value.weekday(state).number_from_monday())?;
        }
        DateKind::Ordinal => {
            let year = value.calendar_year(state);

            if Iso8601::<CONFIG>::YEAR_IS_SIX_DIGITS {
                bytes += write_if_else(output, year < 0, b"-", b"+")?;
                bytes += format_number_pad_zero::<6>(output, year.unsigned_abs())?;
            } else if !(0..=9999).contains(&year) {
                return Err(error::Format::InvalidComponent("year"));
            } else {
                bytes += format_number_pad_zero::<4>(output, year.cast_unsigned())?;
            }
            bytes += write_if(output, Iso8601::<CONFIG>::USE_SEPARATORS, b"-")?;
            bytes += format_number_pad_zero::<3>(output, value.ordinal(state))?;
        }
    }

    Ok(bytes)
}

/// Format the time portion of ISO 8601.
#[inline]
pub(super) fn format_time<V, const CONFIG: EncodedConfig>(
    output: &mut (impl io::Write + ?Sized),
    value: &V,
    state: &mut V::State,
) -> Result<usize, error::Format>
where
    V: ComponentProvider,
{
    let mut bytes = 0;

    // The "T" can only be omitted in extended format where there is no date being formatted.
    bytes += write_if(
        output,
        Iso8601::<CONFIG>::USE_SEPARATORS || Iso8601::<CONFIG>::FORMAT_DATE,
        b"T",
    )?;

    match Iso8601::<CONFIG>::TIME_PRECISION {
        TimePrecision::Hour { decimal_digits } => {
            let hours = (value.hour(state) as f64)
                + (value.minute(state) as f64) / Minute::per_t::<f64>(Hour)
                + (value.second(state) as f64) / Second::per_t::<f64>(Hour)
                + (value.nanosecond(state) as f64) / Nanosecond::per_t::<f64>(Hour);
            format_float(output, hours, 2, decimal_digits)?;
        }
        TimePrecision::Minute { decimal_digits } => {
            bytes += format_number_pad_zero::<2>(output, value.hour(state))?;
            bytes += write_if(output, Iso8601::<CONFIG>::USE_SEPARATORS, b":")?;
            let minutes = (value.minute(state) as f64)
                + (value.second(state) as f64) / Second::per_t::<f64>(Minute)
                + (value.nanosecond(state) as f64) / Nanosecond::per_t::<f64>(Minute);
            bytes += format_float(output, minutes, 2, decimal_digits)?;
        }
        TimePrecision::Second { decimal_digits } => {
            bytes += format_number_pad_zero::<2>(output, value.hour(state))?;
            bytes += write_if(output, Iso8601::<CONFIG>::USE_SEPARATORS, b":")?;
            bytes += format_number_pad_zero::<2>(output, value.minute(state))?;
            bytes += write_if(output, Iso8601::<CONFIG>::USE_SEPARATORS, b":")?;
            let seconds = (value.second(state) as f64)
                + (value.nanosecond(state) as f64) / Nanosecond::per_t::<f64>(Second);
            bytes += format_float(output, seconds, 2, decimal_digits)?;
        }
    }

    Ok(bytes)
}

/// Format the UTC offset portion of ISO 8601.
#[inline]
pub(super) fn format_offset<V, const CONFIG: EncodedConfig>(
    output: &mut (impl io::Write + ?Sized),
    value: &V,
    state: &mut V::State,
) -> Result<usize, error::Format>
where
    V: ComponentProvider,
{
    if Iso8601::<CONFIG>::FORMAT_TIME && value.offset_is_utc(state) {
        return Ok(write(output, b"Z")?);
    }

    let mut bytes = 0;

    if value.offset_second(state) != 0 {
        return Err(error::Format::InvalidComponent("offset_second"));
    }
    bytes += write_if_else(output, value.offset_is_negative(state), b"-", b"+")?;
    bytes += format_number_pad_zero::<2>(output, value.offset_hour(state).unsigned_abs())?;

    let minutes = value.offset_minute(state);

    if Iso8601::<CONFIG>::OFFSET_PRECISION == OffsetPrecision::Hour && minutes != 0 {
        return Err(error::Format::InvalidComponent("offset_minute"));
    } else if Iso8601::<CONFIG>::OFFSET_PRECISION == OffsetPrecision::Minute {
        bytes += write_if(output, Iso8601::<CONFIG>::USE_SEPARATORS, b":")?;
        bytes += format_number_pad_zero::<2>(output, minutes.unsigned_abs())?;
    }

    Ok(bytes)
}
