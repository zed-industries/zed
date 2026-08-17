use std::ops::RangeInclusive;

use crate::parser::error::CustomError;
use crate::parser::prelude::*;
use crate::parser::trivia::from_utf8_unchecked;

use toml_datetime::{Date, Datetime, Offset, Time};
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::opt;
use winnow::combinator::preceded;
use winnow::combinator::trace;
use winnow::stream::Stream as _;
use winnow::token::one_of;
use winnow::token::take_while;

// ;; Date and Time (as defined in RFC 3339)

// date-time = offset-date-time / local-date-time / local-date / local-time
// offset-date-time = full-date time-delim full-time
// local-date-time = full-date time-delim partial-time
// local-date = full-date
// local-time = partial-time
// full-time = partial-time time-offset
pub(crate) fn date_time(input: &mut Input<'_>) -> ModalResult<Datetime> {
    trace(
        "date-time",
        alt((
            (full_date, opt((time_delim, partial_time, opt(time_offset))))
                .map(|(date, opt)| {
                    match opt {
                        // Offset Date-Time
                        Some((_, time, offset)) => Datetime {
                            date: Some(date),
                            time: Some(time),
                            offset,
                        },
                        // Local Date
                        None => Datetime {
                            date: Some(date),
                            time: None,
                            offset: None,
                        },
                    }
                })
                .context(StrContext::Label("date-time")),
            partial_time
                .map(|t| t.into())
                .context(StrContext::Label("time")),
        )),
    )
    .parse_next(input)
}

// full-date      = date-fullyear "-" date-month "-" date-mday
fn full_date(input: &mut Input<'_>) -> ModalResult<Date> {
    trace("full-date", full_date_).parse_next(input)
}

fn full_date_(input: &mut Input<'_>) -> ModalResult<Date> {
    let year = date_fullyear.parse_next(input)?;
    let _ = b'-'.parse_next(input)?;
    let month = cut_err(date_month).parse_next(input)?;
    let _ = cut_err(b'-').parse_next(input)?;
    let day_start = input.checkpoint();
    let day = cut_err(date_mday).parse_next(input)?;

    let is_leap_year = (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0));
    let max_days_in_month = match month {
        2 if is_leap_year => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    if max_days_in_month < day {
        input.reset(&day_start);
        return Err(
            winnow::error::ErrMode::from_external_error(input, CustomError::OutOfRange).cut(),
        );
    }

    Ok(Date { year, month, day })
}

// partial-time   = time-hour ":" time-minute ":" time-second [time-secfrac]
fn partial_time(input: &mut Input<'_>) -> ModalResult<Time> {
    trace(
        "partial-time",
        (
            time_hour,
            b':',
            cut_err((time_minute, b':', time_second, opt(time_secfrac))),
        )
            .map(|(hour, _, (minute, _, second, nanosecond))| Time {
                hour,
                minute,
                second,
                nanosecond: nanosecond.unwrap_or_default(),
            }),
    )
    .parse_next(input)
}

// time-offset    = "Z" / time-numoffset
// time-numoffset = ( "+" / "-" ) time-hour ":" time-minute
fn time_offset(input: &mut Input<'_>) -> ModalResult<Offset> {
    trace(
        "time-offset",
        alt((
            one_of((b'Z', b'z')).value(Offset::Z),
            (
                one_of((b'+', b'-')),
                cut_err((time_hour, b':', time_minute)),
            )
                .map(|(sign, (hours, _, minutes))| {
                    let sign = match sign {
                        b'+' => 1,
                        b'-' => -1,
                        _ => unreachable!("Parser prevents this"),
                    };
                    sign * (hours as i16 * 60 + minutes as i16)
                })
                .verify(|minutes| ((-24 * 60)..=(24 * 60)).contains(minutes))
                .map(|minutes| Offset::Custom { minutes }),
        ))
        .context(StrContext::Label("time offset")),
    )
    .parse_next(input)
}

// date-fullyear  = 4DIGIT
fn date_fullyear(input: &mut Input<'_>) -> ModalResult<u16> {
    unsigned_digits::<4, 4>
        .map(|s: &str| s.parse::<u16>().expect("4DIGIT should match u8"))
        .parse_next(input)
}

// date-month     = 2DIGIT  ; 01-12
fn date_month(input: &mut Input<'_>) -> ModalResult<u8> {
    unsigned_digits::<2, 2>
        .try_map(|s: &str| {
            let d = s.parse::<u8>().expect("2DIGIT should match u8");
            if (1..=12).contains(&d) {
                Ok(d)
            } else {
                Err(CustomError::OutOfRange)
            }
        })
        .parse_next(input)
}

// date-mday      = 2DIGIT  ; 01-28, 01-29, 01-30, 01-31 based on month/year
fn date_mday(input: &mut Input<'_>) -> ModalResult<u8> {
    unsigned_digits::<2, 2>
        .try_map(|s: &str| {
            let d = s.parse::<u8>().expect("2DIGIT should match u8");
            if (1..=31).contains(&d) {
                Ok(d)
            } else {
                Err(CustomError::OutOfRange)
            }
        })
        .parse_next(input)
}

// time-delim     = "T" / %x20 ; T, t, or space
fn time_delim(input: &mut Input<'_>) -> ModalResult<u8> {
    one_of(TIME_DELIM).parse_next(input)
}

const TIME_DELIM: (u8, u8, u8) = (b'T', b't', b' ');

// time-hour      = 2DIGIT  ; 00-23
fn time_hour(input: &mut Input<'_>) -> ModalResult<u8> {
    unsigned_digits::<2, 2>
        .try_map(|s: &str| {
            let d = s.parse::<u8>().expect("2DIGIT should match u8");
            if (0..=23).contains(&d) {
                Ok(d)
            } else {
                Err(CustomError::OutOfRange)
            }
        })
        .parse_next(input)
}

// time-minute    = 2DIGIT  ; 00-59
fn time_minute(input: &mut Input<'_>) -> ModalResult<u8> {
    unsigned_digits::<2, 2>
        .try_map(|s: &str| {
            let d = s.parse::<u8>().expect("2DIGIT should match u8");
            if (0..=59).contains(&d) {
                Ok(d)
            } else {
                Err(CustomError::OutOfRange)
            }
        })
        .parse_next(input)
}

// time-second    = 2DIGIT  ; 00-58, 00-59, 00-60 based on leap second rules
fn time_second(input: &mut Input<'_>) -> ModalResult<u8> {
    unsigned_digits::<2, 2>
        .try_map(|s: &str| {
            let d = s.parse::<u8>().expect("2DIGIT should match u8");
            if (0..=60).contains(&d) {
                Ok(d)
            } else {
                Err(CustomError::OutOfRange)
            }
        })
        .parse_next(input)
}

// time-secfrac   = "." 1*DIGIT
fn time_secfrac(input: &mut Input<'_>) -> ModalResult<u32> {
    static SCALE: [u32; 10] = [
        0,
        100_000_000,
        10_000_000,
        1_000_000,
        100_000,
        10_000,
        1_000,
        100,
        10,
        1,
    ];
    const INF: usize = usize::MAX;
    preceded(b'.', unsigned_digits::<1, INF>)
        .try_map(|mut repr: &str| -> Result<u32, CustomError> {
            let max_digits = SCALE.len() - 1;
            if max_digits < repr.len() {
                // Millisecond precision is required. Further precision of fractional seconds is
                // implementation-specific. If the value contains greater precision than the
                // implementation can support, the additional precision must be truncated, not rounded.
                repr = &repr[0..max_digits];
            }

            let v = repr.parse::<u32>().map_err(|_| CustomError::OutOfRange)?;
            let num_digits = repr.len();

            // scale the number accordingly.
            let scale = SCALE.get(num_digits).ok_or(CustomError::OutOfRange)?;
            let v = v.checked_mul(*scale).ok_or(CustomError::OutOfRange)?;
            Ok(v)
        })
        .parse_next(input)
}

fn unsigned_digits<'i, const MIN: usize, const MAX: usize>(
    input: &mut Input<'i>,
) -> ModalResult<&'i str> {
    take_while(MIN..=MAX, DIGIT)
        .map(|b: &[u8]| unsafe { from_utf8_unchecked(b, "`is_ascii_digit` filters out on-ASCII") })
        .parse_next(input)
}

// DIGIT = %x30-39 ; 0-9
const DIGIT: RangeInclusive<u8> = b'0'..=b'9';

#[cfg(test)]
#[cfg(feature = "parse")]
#[cfg(feature = "display")]
mod test {
    use super::*;

    #[test]
    fn offset_date_time() {
        let inputs = [
            (
                "1979-05-27T07:32:00Z",
                Datetime {
                    date: Some(Date {
                        year: 1979,
                        month: 5,
                        day: 27,
                    }),
                    time: Some(Time {
                        hour: 7,
                        minute: 32,
                        second: 0,
                        nanosecond: 0,
                    }),
                    offset: Some(Offset::Z),
                },
            ),
            (
                "1979-05-27T00:32:00-07:00",
                Datetime {
                    date: Some(Date {
                        year: 1979,
                        month: 5,
                        day: 27,
                    }),
                    time: Some(Time {
                        hour: 0,
                        minute: 32,
                        second: 0,
                        nanosecond: 0,
                    }),
                    offset: Some(Offset::Custom { minutes: -7 * 60 }),
                },
            ),
            (
                "1979-05-27T00:32:00-00:36",
                Datetime {
                    date: Some(Date {
                        year: 1979,
                        month: 5,
                        day: 27,
                    }),
                    time: Some(Time {
                        hour: 0,
                        minute: 32,
                        second: 0,
                        nanosecond: 0,
                    }),
                    offset: Some(Offset::Custom { minutes: -36 }),
                },
            ),
            (
                "1979-05-27T00:32:00.999999",
                Datetime {
                    date: Some(Date {
                        year: 1979,
                        month: 5,
                        day: 27,
                    }),
                    time: Some(Time {
                        hour: 0,
                        minute: 32,
                        second: 0,
                        nanosecond: 999999000,
                    }),
                    offset: None,
                },
            ),
        ];
        for (input, expected) in inputs {
            dbg!(input);
            let actual = date_time.parse(new_input(input)).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn local_date_time() {
        let inputs = [
            (
                "1979-05-27T07:32:00",
                Datetime {
                    date: Some(Date {
                        year: 1979,
                        month: 5,
                        day: 27,
                    }),
                    time: Some(Time {
                        hour: 7,
                        minute: 32,
                        second: 0,
                        nanosecond: 0,
                    }),
                    offset: None,
                },
            ),
            (
                "1979-05-27T00:32:00.999999",
                Datetime {
                    date: Some(Date {
                        year: 1979,
                        month: 5,
                        day: 27,
                    }),
                    time: Some(Time {
                        hour: 0,
                        minute: 32,
                        second: 0,
                        nanosecond: 999999000,
                    }),
                    offset: None,
                },
            ),
        ];
        for (input, expected) in inputs {
            dbg!(input);
            let actual = date_time.parse(new_input(input)).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn local_date() {
        let inputs = [
            (
                "1979-05-27",
                Datetime {
                    date: Some(Date {
                        year: 1979,
                        month: 5,
                        day: 27,
                    }),
                    time: None,
                    offset: None,
                },
            ),
            (
                "2017-07-20",
                Datetime {
                    date: Some(Date {
                        year: 2017,
                        month: 7,
                        day: 20,
                    }),
                    time: None,
                    offset: None,
                },
            ),
        ];
        for (input, expected) in inputs {
            dbg!(input);
            let actual = date_time.parse(new_input(input)).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn local_time() {
        let inputs = [
            (
                "07:32:00",
                Datetime {
                    date: None,
                    time: Some(Time {
                        hour: 7,
                        minute: 32,
                        second: 0,
                        nanosecond: 0,
                    }),
                    offset: None,
                },
            ),
            (
                "00:32:00.999999",
                Datetime {
                    date: None,
                    time: Some(Time {
                        hour: 0,
                        minute: 32,
                        second: 0,
                        nanosecond: 999999000,
                    }),
                    offset: None,
                },
            ),
        ];
        for (input, expected) in inputs {
            dbg!(input);
            let actual = date_time.parse(new_input(input)).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn time_fraction_truncated() {
        let input = "1987-07-05T17:45:00.123456789012345Z";
        date_time.parse(new_input(input)).unwrap();
    }
}
