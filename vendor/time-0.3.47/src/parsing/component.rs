//! Parsing implementations for all [`Component`](crate::format_description::Component)s.

use core::num::NonZero;

use num_conv::prelude::*;

use crate::convert::*;
use crate::format_description::{Period, modifier};
use crate::parsing::ParsedItem;
use crate::parsing::combinator::{
    ExactlyNDigits, Sign, any_digit, exactly_n_digits_padded, n_to_m_digits, n_to_m_digits_padded,
    opt, sign,
};
use crate::{Month, Weekday};

/// Parse the "year" component of a `Date`.
pub(crate) fn parse_year(
    input: &[u8],
    modifiers: modifier::Year,
) -> Option<ParsedItem<'_, (i32, bool)>> {
    match modifiers.repr {
        modifier::YearRepr::Full => {
            let ParsedItem(input, sign) = opt(sign)(input);

            if let Some(sign) = sign {
                let ParsedItem(input, year) = if cfg!(feature = "large-dates")
                    && modifiers.range == modifier::YearRange::Extended
                {
                    n_to_m_digits_padded::<4, 6, u32>(modifiers.padding)(input)?
                } else {
                    exactly_n_digits_padded::<4, u32>(modifiers.padding)(input)?
                };

                Some(ParsedItem(
                    input,
                    match sign {
                        Sign::Negative => (-year.cast_signed(), true),
                        Sign::Positive => (year.cast_signed(), false),
                    },
                ))
            } else if modifiers.sign_is_mandatory {
                None
            } else {
                let ParsedItem(input, year) =
                    exactly_n_digits_padded::<4, u32>(modifiers.padding)(input)?;
                Some(ParsedItem(input, (year.cast_signed(), false)))
            }
        }
        modifier::YearRepr::Century => {
            let ParsedItem(input, sign) = opt(sign)(input);

            if let Some(sign) = sign {
                let ParsedItem(input, year) = if cfg!(feature = "large-dates")
                    && modifiers.range == modifier::YearRange::Extended
                {
                    n_to_m_digits_padded::<2, 4, u32>(modifiers.padding)(input)?
                } else {
                    exactly_n_digits_padded::<2, u32>(modifiers.padding)(input)?
                };

                Some(ParsedItem(
                    input,
                    match sign {
                        Sign::Negative => (-year.cast_signed(), true),
                        Sign::Positive => (year.cast_signed(), false),
                    },
                ))
            } else if modifiers.sign_is_mandatory {
                None
            } else {
                let ParsedItem(input, year) =
                    n_to_m_digits_padded::<1, 2, u32>(modifiers.padding)(input)?;
                Some(ParsedItem(input, (year.cast_signed(), false)))
            }
        }
        modifier::YearRepr::LastTwo => Some(
            exactly_n_digits_padded::<2, u32>(modifiers.padding)(input)?
                .map(|v| (v.cast_signed(), false)),
        ),
    }
}

/// Parse the "month" component of a `Date`.
#[inline]
pub(crate) fn parse_month(
    input: &[u8],
    modifiers: modifier::Month,
) -> Option<ParsedItem<'_, Month>> {
    use Month::*;
    match modifiers.repr {
        modifier::MonthRepr::Numerical => {
            exactly_n_digits_padded::<2, _>(modifiers.padding)(input)?
                .flat_map(|n| Month::from_number(NonZero::new(n)?).ok())
        }
        modifier::MonthRepr::Long | modifier::MonthRepr::Short => {
            let [first, second, third, rest @ ..] = input else {
                return None;
            };
            let byte = if modifiers.case_sensitive {
                u32::from_ne_bytes([0, *first, *second, *third])
            } else {
                u32::from_ne_bytes([
                    0,
                    first.to_ascii_uppercase(),
                    second.to_ascii_lowercase(),
                    third.to_ascii_lowercase(),
                ])
            };
            const WEEKDAYS: [u32; 12] = [
                u32::from_ne_bytes([0, b'J', b'a', b'n']),
                u32::from_ne_bytes([0, b'F', b'e', b'b']),
                u32::from_ne_bytes([0, b'M', b'a', b'r']),
                u32::from_ne_bytes([0, b'A', b'p', b'r']),
                u32::from_ne_bytes([0, b'M', b'a', b'y']),
                u32::from_ne_bytes([0, b'J', b'u', b'n']),
                u32::from_ne_bytes([0, b'J', b'u', b'l']),
                u32::from_ne_bytes([0, b'A', b'u', b'g']),
                u32::from_ne_bytes([0, b'S', b'e', b'p']),
                u32::from_ne_bytes([0, b'O', b'c', b't']),
                u32::from_ne_bytes([0, b'N', b'o', b'v']),
                u32::from_ne_bytes([0, b'D', b'e', b'c']),
            ];

            let bitmask = ((WEEKDAYS[0] == byte) as u32) << 1
                | ((WEEKDAYS[1] == byte) as u32) << 2
                | ((WEEKDAYS[2] == byte) as u32) << 3
                | ((WEEKDAYS[3] == byte) as u32) << 4
                | ((WEEKDAYS[4] == byte) as u32) << 5
                | ((WEEKDAYS[5] == byte) as u32) << 6
                | ((WEEKDAYS[6] == byte) as u32) << 7
                | ((WEEKDAYS[7] == byte) as u32) << 8
                | ((WEEKDAYS[8] == byte) as u32) << 9
                | ((WEEKDAYS[9] == byte) as u32) << 10
                | ((WEEKDAYS[10] == byte) as u32) << 11
                | ((WEEKDAYS[11] == byte) as u32) << 12;
            if bitmask == 0 {
                return None;
            }
            let index = if cfg!(target_endian = "little") {
                bitmask.trailing_zeros() as u8
            } else {
                31 - bitmask.leading_zeros() as u8
            };

            // Safety: `index` cannot be greater than 12 because there are only 12 elements in the
            // array that is converted to a bitmask. We know at least one element matched because
            // the bitmask is non-zero.
            let month = unsafe { Month::from_number(NonZero::new(index)?).unwrap_unchecked() };

            // For the "short" repr, we've already validated the full text expected. For the "long"
            // repr, we need to validate the remaining characters.
            if modifiers.repr == modifier::MonthRepr::Short {
                return Some(ParsedItem(rest, month));
            }

            let expected_remaining = match month {
                January => b"uary".as_slice(),
                February => b"ruary".as_slice(),
                March => b"ch".as_slice(),
                April => b"il".as_slice(),
                May => b"".as_slice(),
                June => b"e".as_slice(),
                July => b"y".as_slice(),
                August => b"ust".as_slice(),
                September => b"tember".as_slice(),
                October => b"ober".as_slice(),
                November | December => b"ember".as_slice(),
            };

            if modifiers.case_sensitive {
                rest.strip_prefix(expected_remaining)
                    .map(|remaining| ParsedItem(remaining, month))
            } else {
                let (head, tail) = rest.split_at_checked(expected_remaining.len())?;
                core::iter::zip(head, expected_remaining)
                    .all(|(a, b)| a.eq_ignore_ascii_case(b))
                    .then_some(ParsedItem(tail, month))
            }
        }
    }
}

/// Parse the "week number" component of a `Date`.
pub(crate) fn parse_week_number(
    input: &[u8],
    modifiers: modifier::WeekNumber,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<2, _>(modifiers.padding)(input)
}

/// Parse the "weekday" component of a `Date`.
#[inline]
pub(crate) fn parse_weekday(
    input: &[u8],
    modifiers: modifier::Weekday,
) -> Option<ParsedItem<'_, Weekday>> {
    match modifiers.repr {
        modifier::WeekdayRepr::Long | modifier::WeekdayRepr::Short => {
            let [first, second, third, rest @ ..] = input else {
                return None;
            };
            let byte = if modifiers.case_sensitive {
                u32::from_ne_bytes([0, *first, *second, *third])
            } else {
                u32::from_ne_bytes([
                    0,
                    first.to_ascii_uppercase(),
                    second.to_ascii_lowercase(),
                    third.to_ascii_lowercase(),
                ])
            };
            const WEEKDAYS: [u32; 7] = [
                u32::from_ne_bytes([0, b'M', b'o', b'n']),
                u32::from_ne_bytes([0, b'T', b'u', b'e']),
                u32::from_ne_bytes([0, b'W', b'e', b'd']),
                u32::from_ne_bytes([0, b'T', b'h', b'u']),
                u32::from_ne_bytes([0, b'F', b'r', b'i']),
                u32::from_ne_bytes([0, b'S', b'a', b't']),
                u32::from_ne_bytes([0, b'S', b'u', b'n']),
            ];

            let bitmask = ((WEEKDAYS[0] == byte) as u32)
                | ((WEEKDAYS[1] == byte) as u32) << 1
                | ((WEEKDAYS[2] == byte) as u32) << 2
                | ((WEEKDAYS[3] == byte) as u32) << 3
                | ((WEEKDAYS[4] == byte) as u32) << 4
                | ((WEEKDAYS[5] == byte) as u32) << 5
                | ((WEEKDAYS[6] == byte) as u32) << 6;
            if bitmask == 0 {
                return None;
            }
            let index = if cfg!(target_endian = "little") {
                bitmask.trailing_zeros()
            } else {
                31 - bitmask.leading_zeros()
            };

            if index > 6 {
                return None;
            }
            // Safety: Values zero thru six are valid variants, while values greater than six have
            // already been excluded above. We know at least one element matched because the bitmask
            // is non-zero.
            let weekday = unsafe { core::mem::transmute::<u8, Weekday>(index.truncate()) };

            // For the "short" repr, we've already validated the full text expected. For the "long"
            // repr, we need to validate the remaining characters.
            if modifiers.repr == modifier::WeekdayRepr::Short {
                return Some(ParsedItem(rest, weekday));
            }

            let expected_remaining = match weekday {
                Weekday::Monday | Weekday::Friday | Weekday::Sunday => b"day".as_slice(),
                Weekday::Tuesday => b"sday".as_slice(),
                Weekday::Wednesday => b"nesday".as_slice(),
                Weekday::Thursday => b"rsday".as_slice(),
                Weekday::Saturday => b"urday".as_slice(),
            };

            if modifiers.case_sensitive {
                rest.strip_prefix(expected_remaining)
                    .map(|remaining| ParsedItem(remaining, weekday))
            } else {
                let (head, tail) = rest.split_at_checked(expected_remaining.len())?;
                core::iter::zip(head, expected_remaining)
                    .all(|(a, b)| a.eq_ignore_ascii_case(b))
                    .then_some(ParsedItem(tail, weekday))
            }
        }
        modifier::WeekdayRepr::Sunday | modifier::WeekdayRepr::Monday => {
            let [digit, rest @ ..] = input else {
                return None;
            };
            let mut digit = digit
                .wrapping_sub(b'0')
                .wrapping_sub(u8::from(modifiers.one_indexed));
            if digit > 6 {
                return None;
            }

            if modifiers.repr == modifier::WeekdayRepr::Sunday {
                // Remap so that Sunday comes after Saturday, not before Monday.
                digit = (digit + 6) % 7;
            }
            // Safety: Values zero thru six are valid variants.
            let weekday = unsafe { core::mem::transmute::<u8, Weekday>(digit) };
            Some(ParsedItem(rest, weekday))
        }
    }
}

/// Parse the "ordinal" component of a `Date`.
#[inline]
pub(crate) fn parse_ordinal(
    input: &[u8],
    modifiers: modifier::Ordinal,
) -> Option<ParsedItem<'_, NonZero<u16>>> {
    exactly_n_digits_padded::<3, _>(modifiers.padding)(input)
        .and_then(|parsed| parsed.flat_map(NonZero::new))
}

/// Parse the "day" component of a `Date`.
#[inline]
pub(crate) fn parse_day(
    input: &[u8],
    modifiers: modifier::Day,
) -> Option<ParsedItem<'_, NonZero<u8>>> {
    exactly_n_digits_padded::<2, _>(modifiers.padding)(input)
        .and_then(|parsed| parsed.flat_map(NonZero::new))
}

/// Parse the "hour" component of a `Time`.
#[inline]
pub(crate) fn parse_hour(input: &[u8], modifiers: modifier::Hour) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<2, _>(modifiers.padding)(input)
}

/// Parse the "minute" component of a `Time`.
#[inline]
pub(crate) fn parse_minute(
    input: &[u8],
    modifiers: modifier::Minute,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<2, _>(modifiers.padding)(input)
}

/// Parse the "second" component of a `Time`.
#[inline]
pub(crate) fn parse_second(
    input: &[u8],
    modifiers: modifier::Second,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<2, _>(modifiers.padding)(input)
}

/// Parse the "period" component of a `Time`. Required if the hour is on a 12-hour clock.
#[inline]
pub(crate) fn parse_period(
    input: &[u8],
    modifiers: modifier::Period,
) -> Option<ParsedItem<'_, Period>> {
    let [first, second, rest @ ..] = input else {
        return None;
    };
    let mut first = *first;
    let mut second = *second;

    if modifiers.is_uppercase && modifiers.case_sensitive {
        match [first, second].as_slice() {
            b"AM" => Some(ParsedItem(rest, Period::Am)),
            b"PM" => Some(ParsedItem(rest, Period::Pm)),
            _ => None,
        }
    } else {
        first = first.to_ascii_lowercase();
        second = second.to_ascii_lowercase();

        match &[first, second] {
            b"am" => Some(ParsedItem(rest, Period::Am)),
            b"pm" => Some(ParsedItem(rest, Period::Pm)),
            _ => None,
        }
    }
}

/// Parse the "subsecond" component of a `Time`.
pub(crate) fn parse_subsecond(
    input: &[u8],
    modifiers: modifier::Subsecond,
) -> Option<ParsedItem<'_, u32>> {
    use modifier::SubsecondDigits::*;
    Some(match modifiers.digits {
        One => ExactlyNDigits::<1>::parse(input)?.map(|v| v.extend::<u32>() * 100_000_000),
        Two => ExactlyNDigits::<2>::parse(input)?.map(|v| v.extend::<u32>() * 10_000_000),
        Three => ExactlyNDigits::<3>::parse(input)?.map(|v| v.extend::<u32>() * 1_000_000),
        Four => ExactlyNDigits::<4>::parse(input)?.map(|v| v.extend::<u32>() * 100_000),
        Five => ExactlyNDigits::<5>::parse(input)?.map(|v| v * 10_000),
        Six => ExactlyNDigits::<6>::parse(input)?.map(|v| v * 1_000),
        Seven => ExactlyNDigits::<7>::parse(input)?.map(|v| v * 100),
        Eight => ExactlyNDigits::<8>::parse(input)?.map(|v| v * 10),
        Nine => ExactlyNDigits::<9>::parse(input)?,
        OneOrMore => {
            let ParsedItem(mut input, mut value) =
                any_digit(input)?.map(|v| (v - b'0').extend::<u32>() * 100_000_000);

            let mut multiplier = 10_000_000;
            while let Some(ParsedItem(new_input, digit)) = any_digit(input) {
                value += (digit - b'0').extend::<u32>() * multiplier;
                input = new_input;
                multiplier /= 10;
            }

            ParsedItem(input, value)
        }
    })
}

/// Parse the "hour" component of a `UtcOffset`.
///
/// Returns the value and whether the value is negative. This is used for when "-0" is parsed.
#[inline]
pub(crate) fn parse_offset_hour(
    input: &[u8],
    modifiers: modifier::OffsetHour,
) -> Option<ParsedItem<'_, (i8, bool)>> {
    let ParsedItem(input, sign) = opt(sign)(input);
    let ParsedItem(input, hour) = exactly_n_digits_padded::<2, u8>(modifiers.padding)(input)?;
    match sign {
        Some(Sign::Negative) => Some(ParsedItem(input, (-hour.cast_signed(), true))),
        None if modifiers.sign_is_mandatory => None,
        _ => Some(ParsedItem(input, (hour.cast_signed(), false))),
    }
}

/// Parse the "minute" component of a `UtcOffset`.
#[inline]
pub(crate) fn parse_offset_minute(
    input: &[u8],
    modifiers: modifier::OffsetMinute,
) -> Option<ParsedItem<'_, i8>> {
    Some(
        exactly_n_digits_padded::<2, u8>(modifiers.padding)(input)?
            .map(|offset_minute| offset_minute.cast_signed()),
    )
}

/// Parse the "second" component of a `UtcOffset`.
#[inline]
pub(crate) fn parse_offset_second(
    input: &[u8],
    modifiers: modifier::OffsetSecond,
) -> Option<ParsedItem<'_, i8>> {
    Some(
        exactly_n_digits_padded::<2, u8>(modifiers.padding)(input)?
            .map(|offset_second| offset_second.cast_signed()),
    )
}

/// Ignore the given number of bytes.
#[inline]
pub(crate) fn parse_ignore(
    input: &[u8],
    modifiers: modifier::Ignore,
) -> Option<ParsedItem<'_, ()>> {
    let modifier::Ignore { count } = modifiers;
    let input = input.get((count.get().extend())..)?;
    Some(ParsedItem(input, ()))
}

/// Parse the Unix timestamp component.
pub(crate) fn parse_unix_timestamp(
    input: &[u8],
    modifiers: modifier::UnixTimestamp,
) -> Option<ParsedItem<'_, i128>> {
    let ParsedItem(input, sign) = opt(sign)(input);
    let ParsedItem(input, nano_timestamp) = match modifiers.precision {
        modifier::UnixTimestampPrecision::Second => {
            n_to_m_digits::<1, 14, u128>(input)?.map(|val| val * Nanosecond::per_t::<u128>(Second))
        }
        modifier::UnixTimestampPrecision::Millisecond => n_to_m_digits::<1, 17, u128>(input)?
            .map(|val| val * Nanosecond::per_t::<u128>(Millisecond)),
        modifier::UnixTimestampPrecision::Microsecond => n_to_m_digits::<1, 20, u128>(input)?
            .map(|val| val * Nanosecond::per_t::<u128>(Microsecond)),
        modifier::UnixTimestampPrecision::Nanosecond => n_to_m_digits::<1, 23, _>(input)?,
    };

    match sign {
        Some(Sign::Negative) => Some(ParsedItem(input, -nano_timestamp.cast_signed())),
        None if modifiers.sign_is_mandatory => None,
        _ => Some(ParsedItem(input, nano_timestamp.cast_signed())),
    }
}

/// Parse the `end` component, which represents the end of input. If any input is remaining _and_
/// trailing input is prohibited, `None` is returned. If trailing input is permitted, it is
/// discarded.
#[inline]
pub(crate) fn parse_end(input: &[u8], end: modifier::End) -> Option<ParsedItem<'_, ()>> {
    let modifier::End { trailing_input } = end;

    if trailing_input == modifier::TrailingInput::Discard || input.is_empty() {
        Some(ParsedItem(b"", ()))
    } else {
        None
    }
}
