/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

// Some of the functions in this file are unused when disabling certain features
#![allow(dead_code)]

use std::time::SystemTime;
use time::{OffsetDateTime, Time};

/// Truncates the subseconds from the given `SystemTime` to zero.
pub(crate) fn truncate_subsecs(time: SystemTime) -> SystemTime {
    let date_time = OffsetDateTime::from(time);
    let time = date_time.time();
    date_time
        .replace_time(
            Time::from_hms(time.hour(), time.minute(), time.second()).expect("was already a time"),
        )
        .into()
}

/// Formats a `SystemTime` in `YYYYMMDD` format.
pub(crate) fn format_date(time: SystemTime) -> String {
    let time = OffsetDateTime::from(time);
    format!(
        "{:04}{:02}{:02}",
        time.year(),
        u8::from(time.month()),
        time.day()
    )
}

/// Formats a `SystemTime` in `YYYYMMDD'T'HHMMSS'Z'` format.
pub(crate) fn format_date_time(time: SystemTime) -> String {
    let time = OffsetDateTime::from(time);
    format!(
        "{:04}{:02}{:02}T{:02}{:02}{:02}Z",
        time.year(),
        u8::from(time.month()),
        time.day(),
        time.hour(),
        time.minute(),
        time.second()
    )
}

/// Parse functions that are only needed for unit tests.
#[cfg(test)]
pub(crate) mod test_parsers {
    use std::{borrow::Cow, error::Error, fmt, time::SystemTime};
    use time::format_description;
    use time::{Date, PrimitiveDateTime, Time};

    const DATE_TIME_FORMAT: &str = "[year][month][day]T[hour][minute][second]Z";
    const DATE_FORMAT: &str = "[year][month][day]";

    /// Parses `YYYYMMDD'T'HHMMSS'Z'` formatted dates into a `SystemTime`.
    pub(crate) fn parse_date_time(date_time_str: &str) -> Result<SystemTime, ParseError> {
        let date_time = PrimitiveDateTime::parse(
            date_time_str,
            &format_description::parse(DATE_TIME_FORMAT).unwrap(),
        )
        .map_err(|err| ParseError(err.to_string().into()))?
        .assume_utc();
        Ok(date_time.into())
    }

    /// Parses `YYYYMMDD` formatted dates into a `SystemTime`.
    pub(crate) fn parse_date(date_str: &str) -> Result<SystemTime, ParseError> {
        let date_time = PrimitiveDateTime::new(
            Date::parse(date_str, &format_description::parse(DATE_FORMAT).unwrap())
                .map_err(|err| ParseError(err.to_string().into()))?,
            Time::from_hms(0, 0, 0).unwrap(),
        )
        .assume_utc();
        Ok(date_time.into())
    }

    #[derive(Debug)]
    pub(crate) struct ParseError(Cow<'static, str>);

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "failed to parse time: {}", self.0)
        }
    }

    impl Error for ParseError {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date_time::test_parsers::{parse_date, parse_date_time};
    use time::format_description::well_known::Rfc3339;

    // TODO(https://github.com/smithy-lang/smithy-rs/issues/1857)
    #[cfg(not(any(target_arch = "powerpc", target_arch = "x86")))]
    #[test]
    fn date_format() {
        let time: SystemTime = OffsetDateTime::parse("2039-02-04T23:01:09.104Z", &Rfc3339)
            .unwrap()
            .into();
        assert_eq!("20390204", format_date(time));
        let time: SystemTime = OffsetDateTime::parse("0100-01-02T00:00:00.000Z", &Rfc3339)
            .unwrap()
            .into();
        assert_eq!("01000102", format_date(time));
    }

    // TODO(https://github.com/smithy-lang/smithy-rs/issues/1857)
    #[cfg(not(any(target_arch = "powerpc", target_arch = "x86")))]
    #[test]
    fn date_time_format() {
        let time: SystemTime = OffsetDateTime::parse("2039-02-04T23:01:09.104Z", &Rfc3339)
            .unwrap()
            .into();
        assert_eq!("20390204T230109Z", format_date_time(time));
        let time: SystemTime = OffsetDateTime::parse("0100-01-02T00:00:00.000Z", &Rfc3339)
            .unwrap()
            .into();
        assert_eq!("01000102T000000Z", format_date_time(time));
    }

    #[test]
    fn date_time_roundtrip() {
        let time = parse_date_time("20150830T123600Z").unwrap();
        assert_eq!("20150830T123600Z", format_date_time(time));
    }

    #[test]
    fn date_roundtrip() {
        let time = parse_date("20150830").unwrap();
        assert_eq!("20150830", format_date(time));
    }

    // TODO(https://github.com/smithy-lang/smithy-rs/issues/1857)
    #[cfg(not(any(target_arch = "powerpc", target_arch = "x86")))]
    #[test]
    fn test_truncate_subsecs() {
        let time: SystemTime = OffsetDateTime::parse("2039-02-04T23:01:09.104Z", &Rfc3339)
            .unwrap()
            .into();
        let expected: SystemTime = OffsetDateTime::parse("2039-02-04T23:01:09.000Z", &Rfc3339)
            .unwrap()
            .into();
        assert_eq!(expected, truncate_subsecs(time));
    }
}
