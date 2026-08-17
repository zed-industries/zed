//! Rules defined in [ISO 8601].
//!
//! [ISO 8601]: https://www.iso.org/iso-8601-date-and-time-format.html

use core::num::NonZero;

use num_conv::prelude::*;

use crate::parsing::ParsedItem;
use crate::parsing::combinator::{ExactlyNDigits, Sign, any_digit, sign};
use crate::{Month, Weekday};

/// What kind of format is being parsed. This is used to ensure each part of the format (date, time,
/// offset) is the same kind.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ExtendedKind {
    /// The basic format.
    Basic,
    /// The extended format.
    Extended,
    /// ¯\_(ツ)_/¯
    Unknown,
}

impl ExtendedKind {
    /// Is it possible that the format is extended?
    #[inline]
    pub(crate) const fn maybe_extended(self) -> bool {
        matches!(self, Self::Extended | Self::Unknown)
    }

    /// Is the format known for certain to be extended?
    #[inline]
    pub(crate) const fn is_extended(self) -> bool {
        matches!(self, Self::Extended)
    }

    /// If the kind is `Unknown`, make it `Basic`. Otherwise, do nothing. Returns `Some` if and only
    /// if the kind is now `Basic`.
    #[inline]
    pub(crate) const fn coerce_basic(&mut self) -> Option<()> {
        match self {
            Self::Basic => Some(()),
            Self::Extended => None,
            Self::Unknown => {
                *self = Self::Basic;
                Some(())
            }
        }
    }

    /// If the kind is `Unknown`, make it `Extended`. Otherwise, do nothing. Returns `Some` if and
    /// only if the kind is now `Extended`.
    #[inline]
    pub(crate) const fn coerce_extended(&mut self) -> Option<()> {
        match self {
            Self::Basic => None,
            Self::Extended => Some(()),
            Self::Unknown => {
                *self = Self::Extended;
                Some(())
            }
        }
    }
}

/// Parse a possibly expanded year.
#[inline]
pub(crate) fn year(input: &[u8]) -> Option<ParsedItem<'_, i32>> {
    Some(match sign(input) {
        Some(ParsedItem(input, sign)) => ExactlyNDigits::<6>::parse(input)?.map(|val| {
            let val = val.cast_signed();
            match sign {
                Sign::Negative => -val,
                Sign::Positive => val,
            }
        }),
        None => ExactlyNDigits::<4>::parse(input)?.map(|val| val.cast_signed().extend()),
    })
}

/// Parse a month.
#[inline]
pub(crate) fn month(input: &[u8]) -> Option<ParsedItem<'_, Month>> {
    match input {
        [b'0', b'1', remaining @ ..] => Some(ParsedItem(remaining, Month::January)),
        [b'0', b'2', remaining @ ..] => Some(ParsedItem(remaining, Month::February)),
        [b'0', b'3', remaining @ ..] => Some(ParsedItem(remaining, Month::March)),
        [b'0', b'4', remaining @ ..] => Some(ParsedItem(remaining, Month::April)),
        [b'0', b'5', remaining @ ..] => Some(ParsedItem(remaining, Month::May)),
        [b'0', b'6', remaining @ ..] => Some(ParsedItem(remaining, Month::June)),
        [b'0', b'7', remaining @ ..] => Some(ParsedItem(remaining, Month::July)),
        [b'0', b'8', remaining @ ..] => Some(ParsedItem(remaining, Month::August)),
        [b'0', b'9', remaining @ ..] => Some(ParsedItem(remaining, Month::September)),
        [b'1', b'0', remaining @ ..] => Some(ParsedItem(remaining, Month::October)),
        [b'1', b'1', remaining @ ..] => Some(ParsedItem(remaining, Month::November)),
        [b'1', b'2', remaining @ ..] => Some(ParsedItem(remaining, Month::December)),
        _ => None,
    }
}

/// Parse a week number.
#[inline]
pub(crate) fn week(input: &[u8]) -> Option<ParsedItem<'_, NonZero<u8>>> {
    ExactlyNDigits::<2>::parse(input).and_then(|parsed| parsed.flat_map(NonZero::new))
}

/// Parse a day of the month.
#[inline]
pub(crate) fn day(input: &[u8]) -> Option<ParsedItem<'_, NonZero<u8>>> {
    ExactlyNDigits::<2>::parse(input).and_then(|parsed| parsed.flat_map(NonZero::new))
}

/// Parse a day of the week.
#[inline]
pub(crate) fn dayk(input: &[u8]) -> Option<ParsedItem<'_, Weekday>> {
    match input {
        [b'1', remaining @ ..] => Some(ParsedItem(remaining, Weekday::Monday)),
        [b'2', remaining @ ..] => Some(ParsedItem(remaining, Weekday::Tuesday)),
        [b'3', remaining @ ..] => Some(ParsedItem(remaining, Weekday::Wednesday)),
        [b'4', remaining @ ..] => Some(ParsedItem(remaining, Weekday::Thursday)),
        [b'5', remaining @ ..] => Some(ParsedItem(remaining, Weekday::Friday)),
        [b'6', remaining @ ..] => Some(ParsedItem(remaining, Weekday::Saturday)),
        [b'7', remaining @ ..] => Some(ParsedItem(remaining, Weekday::Sunday)),
        _ => None,
    }
}

/// Parse a day of the year.
#[inline]
pub(crate) fn dayo(input: &[u8]) -> Option<ParsedItem<'_, NonZero<u16>>> {
    ExactlyNDigits::<3>::parse(input).and_then(|parsed| parsed.flat_map(NonZero::new))
}

/// Parse the hour.
#[inline]
pub(crate) const fn hour(input: &[u8]) -> Option<ParsedItem<'_, u8>> {
    ExactlyNDigits::<2>::parse(input)
}

/// Parse the minute.
#[inline]
pub(crate) const fn min(input: &[u8]) -> Option<ParsedItem<'_, u8>> {
    ExactlyNDigits::<2>::parse(input)
}

/// Parse a floating point number as its integer and optional fractional parts.
///
/// The number must have two digits before the decimal point. If a decimal point is present, at
/// least one digit must follow.
///
/// The return type is a tuple of the integer part and optional fraction part.
#[inline]
pub(crate) fn float(input: &[u8]) -> Option<ParsedItem<'_, (u8, Option<f64>)>> {
    // Two digits before the decimal.
    let ParsedItem(input, integer_part) = match input {
        [
            first_digit @ b'0'..=b'9',
            second_digit @ b'0'..=b'9',
            input @ ..,
        ] => ParsedItem(input, (first_digit - b'0') * 10 + (second_digit - b'0')),
        _ => return None,
    };

    if let Some(ParsedItem(input, ())) = decimal_sign(input) {
        // Mandatory post-decimal digit.
        let ParsedItem(mut input, mut fractional_part) =
            any_digit(input)?.map(|digit| ((digit - b'0') as f64) / 10.);

        let mut divisor = 10.;
        // Any number of subsequent digits.
        while let Some(ParsedItem(new_input, digit)) = any_digit(input) {
            input = new_input;
            divisor *= 10.;
            fractional_part += (digit - b'0') as f64 / divisor;
        }

        Some(ParsedItem(input, (integer_part, Some(fractional_part))))
    } else {
        Some(ParsedItem(input, (integer_part, None)))
    }
}

/// Parse a "decimal sign", which is either a comma or a period.
#[inline]
fn decimal_sign(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    match input {
        [b'.' | b',', remaining @ ..] => Some(ParsedItem(remaining, ())),
        _ => None,
    }
}
