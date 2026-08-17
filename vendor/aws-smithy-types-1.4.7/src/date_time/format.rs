/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::borrow::Cow;
use std::error::Error;
use std::fmt;

const NANOS_PER_SECOND: u32 = 1_000_000_000;

#[derive(Debug)]
pub(super) enum DateTimeParseErrorKind {
    /// The given date-time string was invalid.
    Invalid(Cow<'static, str>),
    /// Failed to parse an integer inside the given date-time string.
    IntParseError,
}

/// Error returned when date-time parsing fails.
#[derive(Debug)]
pub struct DateTimeParseError {
    kind: DateTimeParseErrorKind,
}

impl Error for DateTimeParseError {}

impl fmt::Display for DateTimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DateTimeParseErrorKind::*;
        match &self.kind {
            Invalid(msg) => write!(f, "invalid date-time: {msg}"),
            IntParseError => write!(f, "failed to parse int"),
        }
    }
}

impl From<DateTimeParseErrorKind> for DateTimeParseError {
    fn from(kind: DateTimeParseErrorKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug)]
enum DateTimeFormatErrorKind {
    /// The given date-time cannot be represented in the requested date format.
    OutOfRange(Cow<'static, str>),
}

/// Error returned when date-time formatting fails.
#[derive(Debug)]
pub struct DateTimeFormatError {
    kind: DateTimeFormatErrorKind,
}

impl Error for DateTimeFormatError {}

impl fmt::Display for DateTimeFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            DateTimeFormatErrorKind::OutOfRange(msg) => write!(
                f,
                "date-time cannot be formatted since it is out of range: {msg}"
            ),
        }
    }
}

impl From<DateTimeFormatErrorKind> for DateTimeFormatError {
    fn from(kind: DateTimeFormatErrorKind) -> Self {
        DateTimeFormatError { kind }
    }
}

fn remove_trailing_zeros(string: &mut String) {
    while let Some(b'0') = string.as_bytes().last() {
        string.pop();
    }
}

pub(crate) mod epoch_seconds {
    use super::remove_trailing_zeros;
    use super::{DateTimeParseError, DateTimeParseErrorKind};
    use crate::DateTime;
    use std::str::FromStr;

    /// Formats a `DateTime` into the Smithy epoch seconds date-time format.
    pub(crate) fn format(date_time: &DateTime) -> String {
        if date_time.subsecond_nanos == 0 {
            format!("{}", date_time.seconds)
        } else {
            let mut result = format!("{}.{:0>9}", date_time.seconds, date_time.subsecond_nanos);
            remove_trailing_zeros(&mut result);
            result
        }
    }

    /// Parses the Smithy epoch seconds date-time format into a `DateTime`.
    pub(crate) fn parse(value: &str) -> Result<DateTime, DateTimeParseError> {
        let mut parts = value.splitn(2, '.');
        let (mut whole, mut decimal) = (0i64, 0u32);
        if let Some(whole_str) = parts.next() {
            whole =
                <i64>::from_str(whole_str).map_err(|_| DateTimeParseErrorKind::IntParseError)?;
        }
        if let Some(decimal_str) = parts.next() {
            if decimal_str.starts_with('+') || decimal_str.starts_with('-') {
                return Err(DateTimeParseErrorKind::Invalid(
                    "invalid epoch-seconds timestamp".into(),
                )
                .into());
            }
            if decimal_str.len() > 9 {
                return Err(DateTimeParseErrorKind::Invalid(
                    "decimal is longer than 9 digits".into(),
                )
                .into());
            }
            let missing_places = 9 - decimal_str.len() as isize;
            decimal =
                <u32>::from_str(decimal_str).map_err(|_| DateTimeParseErrorKind::IntParseError)?;
            for _ in 0..missing_places {
                decimal *= 10;
            }
        }
        Ok(DateTime::from_secs_and_nanos(whole, decimal))
    }
}

pub(crate) mod http_date {
    use crate::date_time::format::{
        DateTimeFormatError, DateTimeFormatErrorKind, DateTimeParseError, DateTimeParseErrorKind,
        NANOS_PER_SECOND,
    };
    use crate::DateTime;
    use std::str::FromStr;
    use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

    // This code is taken from https://github.com/pyfisch/httpdate and modified under an
    // Apache 2.0 License. Modifications:
    // - Removed use of unsafe
    // - Add deserialization of subsecond nanos
    //
    /// Format a `DateTime` in the HTTP date format (imf-fixdate)
    ///
    /// Example: "Mon, 16 Dec 2019 23:48:18 GMT"
    ///
    /// Some notes:
    /// - HTTP date does not support years before `0001`—this will cause a panic.
    /// - Subsecond nanos are not emitted
    pub(crate) fn format(date_time: &DateTime) -> Result<String, DateTimeFormatError> {
        fn out_of_range<E: std::fmt::Display>(cause: E) -> DateTimeFormatError {
            DateTimeFormatErrorKind::OutOfRange(
                format!(
                    "HTTP dates support dates between Mon, 01 Jan 0001 00:00:00 GMT \
                            and Fri, 31 Dec 9999 23:59:59.999 GMT. {cause}"
                )
                .into(),
            )
            .into()
        }
        let structured = OffsetDateTime::from_unix_timestamp_nanos(date_time.as_nanos())
            .map_err(out_of_range)?;
        let weekday = match structured.weekday() {
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
            Weekday::Sunday => "Sun",
        };
        let month = match structured.month() {
            Month::January => "Jan",
            Month::February => "Feb",
            Month::March => "Mar",
            Month::April => "Apr",
            Month::May => "May",
            Month::June => "Jun",
            Month::July => "Jul",
            Month::August => "Aug",
            Month::September => "Sep",
            Month::October => "Oct",
            Month::November => "Nov",
            Month::December => "Dec",
        };
        let mut out = String::with_capacity(32);
        fn push_digit(out: &mut String, digit: u8) {
            debug_assert!(digit < 10);
            out.push((b'0' + digit) as char);
        }

        out.push_str(weekday);
        out.push_str(", ");
        let day = structured.day();
        push_digit(&mut out, day / 10);
        push_digit(&mut out, day % 10);

        out.push(' ');
        out.push_str(month);

        out.push(' ');

        let year = structured.year();
        // HTTP date does not support years before 0001
        let year = if year < 1 {
            return Err(out_of_range("HTTP dates cannot be before the year 0001"));
        } else {
            year as u32
        };

        // Extract the individual digits from year
        push_digit(&mut out, (year / 1000) as u8);
        push_digit(&mut out, (year / 100 % 10) as u8);
        push_digit(&mut out, (year / 10 % 10) as u8);
        push_digit(&mut out, (year % 10) as u8);

        out.push(' ');

        let hour = structured.hour();

        // Extract the individual digits from hour
        push_digit(&mut out, hour / 10);
        push_digit(&mut out, hour % 10);

        out.push(':');

        // Extract the individual digits from minute
        let minute = structured.minute();
        push_digit(&mut out, minute / 10);
        push_digit(&mut out, minute % 10);

        out.push(':');

        let second = structured.second();
        push_digit(&mut out, second / 10);
        push_digit(&mut out, second % 10);

        out.push_str(" GMT");
        Ok(out)
    }

    /// Parse an IMF-fixdate formatted date into a DateTime
    ///
    /// This function has a few caveats:
    /// 1. It DOES NOT support the "deprecated" formats supported by HTTP date
    /// 2. It supports up to 3 digits of subsecond precision
    ///
    /// Ok: "Mon, 16 Dec 2019 23:48:18 GMT"
    /// Ok: "Mon, 16 Dec 2019 23:48:18.123 GMT"
    /// Ok: "Mon, 16 Dec 2019 23:48:18.12 GMT"
    /// Not Ok: "Mon, 16 Dec 2019 23:48:18.1234 GMT"
    pub(crate) fn parse(s: &str) -> Result<DateTime, DateTimeParseError> {
        if !s.is_ascii() {
            return Err(DateTimeParseErrorKind::Invalid("date-time must be ASCII".into()).into());
        }
        let x = s.trim().as_bytes();
        parse_imf_fixdate(x)
    }

    pub(crate) fn read(s: &str) -> Result<(DateTime, &str), DateTimeParseError> {
        if !s.is_ascii() {
            return Err(DateTimeParseErrorKind::Invalid("date-time must be ASCII".into()).into());
        }
        let (first_date, rest) = match find_subsequence(s.as_bytes(), b" GMT") {
            // split_at is correct because we asserted that this date is only valid ASCII so the byte index is
            // the same as the char index
            Some(idx) => s.split_at(idx),
            None => {
                return Err(DateTimeParseErrorKind::Invalid("date-time is not GMT".into()).into())
            }
        };
        Ok((parse(first_date)?, rest))
    }

    fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
            .map(|idx| idx + needle.len())
    }

    fn parse_imf_fixdate(s: &[u8]) -> Result<DateTime, DateTimeParseError> {
        // Example: `Sun, 06 Nov 1994 08:49:37 GMT`
        if s.len() < 29
            || s.len() > 33
            || !s.ends_with(b" GMT")
            || s[16] != b' '
            || s[19] != b':'
            || s[22] != b':'
        {
            return Err(DateTimeParseErrorKind::Invalid("incorrectly shaped string".into()).into());
        }
        let nanos: u32 = match &s[25] {
            b'.' => {
                // The date must end with " GMT", so read from the character after the `.`
                // to 4 from the end
                let fraction_slice = &s[26..s.len() - 4];
                if fraction_slice.len() > 3 {
                    // Only thousandths are supported
                    return Err(DateTimeParseErrorKind::Invalid(
                        "Smithy http-date only supports millisecond precision".into(),
                    )
                    .into());
                }
                let fraction: u32 = parse_slice(fraction_slice)?;
                // We need to convert the fractional second to nanoseconds, so we need to scale
                // according the the number of decimals provided
                let multiplier = [10, 100, 1000];
                fraction * (NANOS_PER_SECOND / multiplier[fraction_slice.len() - 1])
            }
            b' ' => 0,
            _ => {
                return Err(
                    DateTimeParseErrorKind::Invalid("incorrectly shaped string".into()).into(),
                )
            }
        };

        let hours = parse_slice(&s[17..19])?;
        let minutes = parse_slice(&s[20..22])?;
        let seconds = parse_slice(&s[23..25])?;
        let time = Time::from_hms_nano(hours, minutes, seconds, nanos).map_err(|err| {
            DateTimeParseErrorKind::Invalid(
                format!("time components are out of range: {err}").into(),
            )
        })?;

        let month = match &s[7..12] {
            b" Jan " => Month::January,
            b" Feb " => Month::February,
            b" Mar " => Month::March,
            b" Apr " => Month::April,
            b" May " => Month::May,
            b" Jun " => Month::June,
            b" Jul " => Month::July,
            b" Aug " => Month::August,
            b" Sep " => Month::September,
            b" Oct " => Month::October,
            b" Nov " => Month::November,
            b" Dec " => Month::December,
            month => {
                return Err(DateTimeParseErrorKind::Invalid(
                    format!(
                        "invalid month: {}",
                        std::str::from_utf8(month).unwrap_or_default()
                    )
                    .into(),
                )
                .into())
            }
        };
        let year = parse_slice(&s[12..16])?;
        let day = parse_slice(&s[5..7])?;
        let date = Date::from_calendar_date(year, month, day).map_err(|err| {
            DateTimeParseErrorKind::Invalid(
                format!("date components are out of range: {err}").into(),
            )
        })?;
        let date_time = PrimitiveDateTime::new(date, time).assume_offset(UtcOffset::UTC);

        Ok(DateTime::from_nanos(date_time.unix_timestamp_nanos())
            .expect("this date format cannot produce out of range date-times"))
    }

    fn parse_slice<T>(ascii_slice: &[u8]) -> Result<T, DateTimeParseError>
    where
        T: FromStr,
    {
        let as_str =
            std::str::from_utf8(ascii_slice).expect("should only be called on ascii strings");
        Ok(as_str
            .parse::<T>()
            .map_err(|_| DateTimeParseErrorKind::IntParseError)?)
    }
}

pub(crate) mod rfc3339 {
    use crate::date_time::format::{
        DateTimeFormatError, DateTimeFormatErrorKind, DateTimeParseError, DateTimeParseErrorKind,
    };
    use crate::DateTime;
    use time::format_description::well_known::Rfc3339;
    use time::OffsetDateTime;

    #[derive(Debug, PartialEq)]
    pub(crate) enum AllowOffsets {
        OffsetsAllowed,
        OffsetsForbidden,
    }

    // OK: 1985-04-12T23:20:50.52Z
    // OK: 1985-04-12T23:20:50Z
    //
    // Timezones not supported:
    // Not OK: 1985-04-12T23:20:50-02:00
    pub(crate) fn parse(
        s: &str,
        allow_offsets: AllowOffsets,
    ) -> Result<DateTime, DateTimeParseError> {
        if allow_offsets == AllowOffsets::OffsetsForbidden && !matches!(s.chars().last(), Some('Z'))
        {
            return Err(DateTimeParseErrorKind::Invalid(
                "Smithy does not support timezone offsets in RFC-3339 date times".into(),
            )
            .into());
        }
        if s.len() > 10 && !matches!(s.as_bytes()[10], b'T' | b't') {
            return Err(DateTimeParseErrorKind::Invalid(
                "RFC-3339 only allows `T` as a separator for date-time values".into(),
            )
            .into());
        }
        let date_time = OffsetDateTime::parse(s, &Rfc3339).map_err(|err| {
            DateTimeParseErrorKind::Invalid(format!("invalid RFC-3339 date-time: {err}").into())
        })?;
        Ok(DateTime::from_nanos(date_time.unix_timestamp_nanos())
            .expect("this date format cannot produce out of range date-times"))
    }

    /// Read 1 RFC-3339 date from &str and return the remaining str
    pub(crate) fn read(
        s: &str,
        allow_offests: AllowOffsets,
    ) -> Result<(DateTime, &str), DateTimeParseError> {
        let delim = s.find('Z').map(|idx| idx + 1).unwrap_or_else(|| s.len());
        let (head, rest) = s.split_at(delim);
        Ok((parse(head, allow_offests)?, rest))
    }

    /// Format a [DateTime] in the RFC-3339 date format
    pub(crate) fn format(date_time: &DateTime) -> Result<String, DateTimeFormatError> {
        use std::fmt::Write;
        fn out_of_range<E: std::fmt::Display>(cause: E) -> DateTimeFormatError {
            DateTimeFormatErrorKind::OutOfRange(
                format!(
                    "RFC-3339 timestamps support dates between 0001-01-01T00:00:00.000Z \
                            and 9999-12-31T23:59:59.999Z. {cause}"
                )
                .into(),
            )
            .into()
        }
        let (year, month, day, hour, minute, second, micros) = {
            let s = OffsetDateTime::from_unix_timestamp_nanos(date_time.as_nanos())
                .map_err(out_of_range)?;
            (
                s.year(),
                u8::from(s.month()),
                s.day(),
                s.hour(),
                s.minute(),
                s.second(),
                s.microsecond(),
            )
        };

        // This is stated in the assumptions for RFC-3339. ISO-8601 allows for years
        // between -99,999 and 99,999 inclusive, but RFC-3339 is bound between 0 and 9,999.
        if !(1..=9_999).contains(&year) {
            return Err(out_of_range(""));
        }

        let mut out = String::with_capacity(33);
        write!(
            out,
            "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}"
        )
        .unwrap();
        format_subsecond_fraction(&mut out, micros);
        out.push('Z');
        Ok(out)
    }

    /// Formats sub-second fraction for RFC-3339 (including the '.').
    /// Expects to be called with a number of `micros` between 0 and 999_999 inclusive.
    fn format_subsecond_fraction(into: &mut String, micros: u32) {
        debug_assert!(micros < 1_000_000);
        if micros > 0 {
            into.push('.');
            let (mut remaining, mut place) = (micros, 100_000);
            while remaining > 0 {
                let digit = (remaining / place) % 10;
                into.push(char::from(b'0' + (digit as u8)));
                remaining -= digit * place;
                place /= 10;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date_time::format::rfc3339::AllowOffsets;
    use crate::DateTime;
    use lazy_static::lazy_static;
    use proptest::prelude::*;
    use std::fs::File;
    use std::io::Read;
    use std::str::FromStr;

    #[derive(Debug, serde::Deserialize)]
    struct TestCase {
        canonical_seconds: String,
        canonical_nanos: u32,
        #[allow(dead_code)]
        iso8601: String,
        #[allow(dead_code)]
        error: bool,
        smithy_format_value: Option<String>,
    }
    impl TestCase {
        fn time(&self) -> DateTime {
            DateTime::from_secs_and_nanos(
                <i64>::from_str(&self.canonical_seconds).unwrap(),
                self.canonical_nanos,
            )
        }
    }

    #[derive(serde::Deserialize)]
    struct TestCases {
        format_date_time: Vec<TestCase>,
        format_http_date: Vec<TestCase>,
        format_epoch_seconds: Vec<TestCase>,
        parse_date_time: Vec<TestCase>,
        parse_http_date: Vec<TestCase>,
        parse_epoch_seconds: Vec<TestCase>,
    }

    lazy_static! {
        static ref TEST_CASES: TestCases = {
            // This test suite can be regenerated by the following Kotlin class:
            // `codegen/src/test/kotlin/software/amazon/smithy/rust/tool/TimeTestSuiteGenerator.kt`
            let mut json = Vec::new();
            let mut file = File::open("test_data/date_time_format_test_suite.json").expect("open test data file");
            file.read_to_end(&mut json).expect("read test data");
            serde_json::from_slice(&json).expect("valid test data")
        };
    }

    fn format_test<F>(test_cases: &[TestCase], format: F)
    where
        F: Fn(&DateTime) -> Result<String, DateTimeFormatError>,
    {
        for test_case in test_cases {
            if let Some(expected) = test_case.smithy_format_value.as_ref() {
                let actual = format(&test_case.time()).expect("failed to format");
                assert_eq!(expected, &actual, "Additional context:\n{test_case:#?}");
            } else {
                format(&test_case.time()).expect_err("date should fail to format");
            }
        }
    }

    fn parse_test<F>(test_cases: &[TestCase], parse: F)
    where
        F: Fn(&str) -> Result<DateTime, DateTimeParseError>,
    {
        for test_case in test_cases {
            let expected = test_case.time();
            let to_parse = test_case
                .smithy_format_value
                .as_ref()
                .expect("parse test cases should always have a formatted value");
            let actual = parse(to_parse);

            assert!(
                actual.is_ok(),
                "Failed to parse `{}`: {}\nAdditional context:\n{:#?}",
                to_parse,
                actual.err().unwrap(),
                test_case
            );
            assert_eq!(
                expected,
                actual.unwrap(),
                "Additional context:\n{test_case:#?}"
            );
        }
    }

    #[test]
    fn format_epoch_seconds() {
        format_test(&TEST_CASES.format_epoch_seconds, |dt| {
            Ok(epoch_seconds::format(dt))
        });
    }

    #[test]
    fn parse_epoch_seconds() {
        parse_test(&TEST_CASES.parse_epoch_seconds, epoch_seconds::parse);
    }

    #[test]
    fn format_http_date() {
        format_test(&TEST_CASES.format_http_date, http_date::format);
    }

    #[test]
    fn parse_http_date() {
        parse_test(&TEST_CASES.parse_http_date, http_date::parse);
    }

    #[test]
    fn date_time_out_of_range() {
        assert_eq!(
            "0001-01-01T00:00:00Z",
            rfc3339::format(&DateTime::from_secs(-62_135_596_800)).unwrap()
        );
        assert_eq!(
            "9999-12-31T23:59:59.999999Z",
            rfc3339::format(&DateTime::from_secs_and_nanos(253402300799, 999_999_999)).unwrap()
        );

        assert!(matches!(
            rfc3339::format(&DateTime::from_secs(-62_135_596_800 - 1)),
            Err(DateTimeFormatError {
                kind: DateTimeFormatErrorKind::OutOfRange(_)
            })
        ));
        assert!(matches!(
            rfc3339::format(&DateTime::from_secs(253402300799 + 1)),
            Err(DateTimeFormatError {
                kind: DateTimeFormatErrorKind::OutOfRange(_)
            })
        ));
    }

    #[test]
    fn format_date_time() {
        format_test(&TEST_CASES.format_date_time, rfc3339::format);
    }

    #[test]
    fn parse_date_time() {
        parse_test(&TEST_CASES.parse_date_time, |date| {
            rfc3339::parse(date, AllowOffsets::OffsetsForbidden)
        });
    }

    #[test]
    fn epoch_seconds_invalid_cases() {
        assert!(epoch_seconds::parse("").is_err());
        assert!(epoch_seconds::parse("123.+456").is_err());
        assert!(epoch_seconds::parse("123.-456").is_err());
        assert!(epoch_seconds::parse("123.456.789").is_err());
        assert!(epoch_seconds::parse("123 . 456").is_err());
        assert!(epoch_seconds::parse("123.456  ").is_err());
        assert!(epoch_seconds::parse("  123.456").is_err());
        assert!(epoch_seconds::parse("a.456").is_err());
        assert!(epoch_seconds::parse("123.a").is_err());
        assert!(epoch_seconds::parse("123..").is_err());
        assert!(epoch_seconds::parse(".123").is_err());
    }

    #[test]
    fn read_rfc3339_date_comma_split() {
        let date = "1985-04-12T23:20:50Z,1985-04-12T23:20:51Z";
        let (e1, date) =
            rfc3339::read(date, AllowOffsets::OffsetsForbidden).expect("should succeed");
        let (e2, date2) =
            rfc3339::read(&date[1..], AllowOffsets::OffsetsForbidden).expect("should succeed");
        assert_eq!(date2, "");
        assert_eq!(date, ",1985-04-12T23:20:51Z");
        let expected = DateTime::from_secs_and_nanos(482196050, 0);
        assert_eq!(e1, expected);
        let expected = DateTime::from_secs_and_nanos(482196051, 0);
        assert_eq!(e2, expected);
    }

    #[test]
    fn parse_rfc3339_with_timezone() {
        let dt = rfc3339::parse("1985-04-12T21:20:51-02:00", AllowOffsets::OffsetsAllowed);
        assert_eq!(dt.unwrap(), DateTime::from_secs_and_nanos(482196051, 0));
    }

    #[test]
    fn parse_rfc3339_timezone_forbidden() {
        let dt = rfc3339::parse("1985-04-12T23:20:50-02:00", AllowOffsets::OffsetsForbidden);
        assert!(matches!(
            dt.unwrap_err(),
            DateTimeParseError {
                kind: DateTimeParseErrorKind::Invalid(_)
            }
        ));
    }

    #[test]
    fn http_date_out_of_range() {
        assert_eq!(
            "Mon, 01 Jan 0001 00:00:00 GMT",
            http_date::format(&DateTime::from_secs(-62_135_596_800)).unwrap()
        );
        assert_eq!(
            "Fri, 31 Dec 9999 23:59:59 GMT",
            http_date::format(&DateTime::from_secs_and_nanos(253402300799, 999_999_999)).unwrap()
        );

        assert!(matches!(
            http_date::format(&DateTime::from_secs(-62_135_596_800 - 1)),
            Err(DateTimeFormatError {
                kind: DateTimeFormatErrorKind::OutOfRange(_)
            })
        ));
        assert!(matches!(
            http_date::format(&DateTime::from_secs(253402300799 + 1)),
            Err(DateTimeFormatError {
                kind: DateTimeFormatErrorKind::OutOfRange(_)
            })
        ));
    }

    #[test]
    fn http_date_too_much_fraction() {
        let fractional = "Mon, 16 Dec 2019 23:48:18.1212 GMT";
        assert!(matches!(
            http_date::parse(fractional),
            Err(DateTimeParseError {
                kind: DateTimeParseErrorKind::Invalid(_)
            })
        ));
    }

    #[test]
    fn http_date_bad_fraction() {
        let fractional = "Mon, 16 Dec 2019 23:48:18. GMT";
        assert!(matches!(
            http_date::parse(fractional),
            Err(DateTimeParseError {
                kind: DateTimeParseErrorKind::IntParseError
            })
        ));
    }

    #[test]
    fn http_date_read_date() {
        let fractional = "Mon, 16 Dec 2019 23:48:18.123 GMT,some more stuff";
        let ts = 1576540098;
        let expected = DateTime::from_fractional_secs(ts, 0.123);
        let (actual, rest) = http_date::read(fractional).expect("valid");
        assert_eq!(rest, ",some more stuff");
        assert_eq!(expected, actual);
        http_date::read(rest).expect_err("invalid date");
    }

    #[track_caller]
    fn http_date_check_roundtrip(epoch_secs: i64, subsecond_nanos: u32) {
        let date_time = DateTime::from_secs_and_nanos(epoch_secs, subsecond_nanos);
        let formatted = http_date::format(&date_time).unwrap();
        let parsed = http_date::parse(&formatted);
        let read = http_date::read(&formatted);
        match parsed {
            Err(failure) => panic!("Date failed to parse {failure:?}"),
            Ok(date) => {
                assert!(read.is_ok());
                if date.subsecond_nanos != subsecond_nanos {
                    assert_eq!(http_date::format(&date_time).unwrap(), formatted);
                } else {
                    assert_eq!(date, date_time)
                }
            }
        }
    }

    #[test]
    fn http_date_roundtrip() {
        for epoch_secs in -1000..1000 {
            http_date_check_roundtrip(epoch_secs, 1);
        }

        http_date_check_roundtrip(1576540098, 0);
        http_date_check_roundtrip(9999999999, 0);
    }

    #[test]
    fn parse_rfc3339_invalid_separator() {
        let test_cases = [
            ("1985-04-12 23:20:50Z", AllowOffsets::OffsetsForbidden),
            ("1985-04-12x23:20:50Z", AllowOffsets::OffsetsForbidden),
            ("1985-04-12 23:20:50-02:00", AllowOffsets::OffsetsAllowed),
            ("1985-04-12a23:20:50-02:00", AllowOffsets::OffsetsAllowed),
        ];
        for (date, offset) in test_cases.into_iter() {
            let dt = rfc3339::parse(date, offset);
            assert!(matches!(
                dt.unwrap_err(),
                DateTimeParseError {
                    kind: DateTimeParseErrorKind::Invalid(_)
                }
            ));
        }
    }
    #[test]
    fn parse_rfc3339_t_separator() {
        let test_cases = [
            ("1985-04-12t23:20:50Z", AllowOffsets::OffsetsForbidden),
            ("1985-04-12T23:20:50Z", AllowOffsets::OffsetsForbidden),
            ("1985-04-12t23:20:50-02:00", AllowOffsets::OffsetsAllowed),
            ("1985-04-12T23:20:50-02:00", AllowOffsets::OffsetsAllowed),
        ];
        for (date, offset) in test_cases.into_iter() {
            let dt = rfc3339::parse(date, offset);
            assert!(
                dt.is_ok(),
                "failed to parse date: '{}' with error: {:?}",
                date,
                dt.err().unwrap()
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]

        #[test]
        fn round_trip(secs in -10000000..9999999999i64, nanos in 0..1_000_000_000u32) {
            http_date_check_roundtrip(secs, nanos);
        }
    }
}
