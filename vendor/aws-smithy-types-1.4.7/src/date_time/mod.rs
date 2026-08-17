/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! DateTime type for representing Smithy timestamps.

use crate::date_time::format::rfc3339::AllowOffsets;
use crate::date_time::format::DateTimeParseErrorKind;
use num_integer::div_mod_floor;
use num_integer::Integer;
use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[cfg(all(aws_sdk_unstable, feature = "serde-deserialize"))]
mod de;
mod format;
#[cfg(all(aws_sdk_unstable, feature = "serde-serialize"))]
mod ser;

pub use self::format::DateTimeFormatError;
pub use self::format::DateTimeParseError;

const MILLIS_PER_SECOND: i64 = 1000;
const NANOS_PER_MILLI: u32 = 1_000_000;
const NANOS_PER_SECOND: i128 = 1_000_000_000;
const NANOS_PER_SECOND_U32: u32 = 1_000_000_000;

/* ANCHOR: date_time */

/// DateTime in time.
///
/// DateTime in time represented as seconds and sub-second nanos since
/// the Unix epoch (January 1, 1970 at midnight UTC/GMT).
///
/// This type can be converted to/from the standard library's [`SystemTime`]:
/// ```rust
/// # fn doc_fn() -> Result<(), aws_smithy_types::date_time::ConversionError> {
/// # use aws_smithy_types::date_time::DateTime;
/// # use std::time::SystemTime;
///
/// let the_millennium_as_system_time = SystemTime::try_from(DateTime::from_secs(946_713_600))?;
/// let now_as_date_time = DateTime::from(SystemTime::now());
/// # Ok(())
/// # }
/// ```
///
/// The [`aws-smithy-types-convert`](https://crates.io/crates/aws-smithy-types-convert) crate
/// can be used for conversions to/from other libraries, such as
/// [`time`](https://crates.io/crates/time) or [`chrono`](https://crates.io/crates/chrono).
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct DateTime {
    pub(crate) seconds: i64,
    /// Subsecond nanos always advances the wallclock time, even for times where seconds is negative
    ///
    /// Bigger subsecond nanos => later time
    pub(crate) subsecond_nanos: u32,
}

/* ANCHOR_END: date_time */

impl DateTime {
    /// Creates a `DateTime` from a number of seconds since the Unix epoch.
    pub fn from_secs(epoch_seconds: i64) -> Self {
        DateTime {
            seconds: epoch_seconds,
            subsecond_nanos: 0,
        }
    }

    /// Creates a `DateTime` from a number of milliseconds since the Unix epoch.
    pub fn from_millis(epoch_millis: i64) -> DateTime {
        let (seconds, millis) = div_mod_floor(epoch_millis, MILLIS_PER_SECOND);
        DateTime::from_secs_and_nanos(seconds, millis as u32 * NANOS_PER_MILLI)
    }

    /// Creates a `DateTime` from a number of nanoseconds since the Unix epoch.
    pub fn from_nanos(epoch_nanos: i128) -> Result<Self, ConversionError> {
        let (seconds, subsecond_nanos) = epoch_nanos.div_mod_floor(&NANOS_PER_SECOND);
        let seconds = i64::try_from(seconds).map_err(|_| {
            ConversionError("given epoch nanos are too large to fit into a DateTime")
        })?;
        let subsecond_nanos = subsecond_nanos as u32; // safe cast because of the modulus
        Ok(DateTime {
            seconds,
            subsecond_nanos,
        })
    }

    /// Returns the number of nanoseconds since the Unix epoch that this `DateTime` represents.
    pub fn as_nanos(&self) -> i128 {
        let seconds = self.seconds as i128 * NANOS_PER_SECOND;
        seconds + self.subsecond_nanos as i128
    }

    /// Creates a `DateTime` from a number of seconds and a fractional second since the Unix epoch.
    ///
    /// # Example
    /// ```
    /// # use aws_smithy_types::DateTime;
    /// assert_eq!(
    ///     DateTime::from_secs_and_nanos(1, 500_000_000u32),
    ///     DateTime::from_fractional_secs(1, 0.5),
    /// );
    /// ```
    pub fn from_fractional_secs(mut epoch_seconds: i64, fraction: f64) -> Self {
        // Because of floating point issues, `fraction` can end up being 1.0 leading to
        // a full second of subsecond nanos. In that case, rollover the subsecond into the second.
        let mut subsecond_nanos = (fraction * 1_000_000_000_f64) as u32;
        if subsecond_nanos == 1_000_000_000 {
            epoch_seconds += 1;
            subsecond_nanos = 0;
        }
        DateTime::from_secs_and_nanos(epoch_seconds, subsecond_nanos)
    }

    /// Creates a `DateTime` from a number of seconds and sub-second nanos since the Unix epoch.
    ///
    /// # Panics
    /// This function will panic if `subsecond_nanos` is >= 1_000_000_000
    ///
    /// # Example
    /// ```
    /// # use aws_smithy_types::DateTime;
    /// assert_eq!(
    ///     DateTime::from_fractional_secs(1, 0.5),
    ///     DateTime::from_secs_and_nanos(1, 500_000_000u32),
    /// );
    /// ```
    pub fn from_secs_and_nanos(seconds: i64, subsecond_nanos: u32) -> Self {
        if subsecond_nanos >= 1_000_000_000 {
            panic!("{subsecond_nanos} is > 1_000_000_000")
        }
        DateTime {
            seconds,
            subsecond_nanos,
        }
    }

    /// Returns the `DateTime` value as an `f64` representing the seconds since the Unix epoch.
    ///
    /// _Note: This conversion will lose precision due to the nature of floating point numbers._
    pub fn as_secs_f64(&self) -> f64 {
        self.seconds as f64 + self.subsecond_nanos as f64 / 1_000_000_000_f64
    }

    /// Creates a `DateTime` from an `f64` representing the number of seconds since the Unix epoch.
    ///
    /// # Example
    /// ```
    /// # use aws_smithy_types::DateTime;
    /// assert_eq!(
    ///     DateTime::from_fractional_secs(1, 0.5),
    ///     DateTime::from_secs_f64(1.5),
    /// );
    /// ```
    pub fn from_secs_f64(epoch_seconds: f64) -> Self {
        let seconds = epoch_seconds.floor() as i64;
        let rem = epoch_seconds - epoch_seconds.floor();
        DateTime::from_fractional_secs(seconds, rem)
    }

    /// Parses a `DateTime` from a string using the given `format`.
    pub fn from_str(s: &str, format: Format) -> Result<Self, DateTimeParseError> {
        match format {
            Format::DateTime => format::rfc3339::parse(s, AllowOffsets::OffsetsForbidden),
            Format::DateTimeWithOffset => format::rfc3339::parse(s, AllowOffsets::OffsetsAllowed),
            Format::HttpDate => format::http_date::parse(s),
            Format::EpochSeconds => format::epoch_seconds::parse(s),
        }
    }

    /// Returns true if sub-second nanos is greater than zero.
    pub fn has_subsec_nanos(&self) -> bool {
        self.subsecond_nanos != 0
    }

    /// Returns the epoch seconds component of the `DateTime`.
    ///
    /// _Note: this does not include the sub-second nanos._
    pub fn secs(&self) -> i64 {
        self.seconds
    }

    /// Set the seconds component of this `DateTime`.
    pub fn set_seconds(&mut self, seconds: i64) -> &mut Self {
        self.seconds = seconds;
        self
    }

    /// Returns the sub-second nanos component of the `DateTime`.
    ///
    /// _Note: this does not include the number of seconds since the epoch._
    pub fn subsec_nanos(&self) -> u32 {
        self.subsecond_nanos
    }

    /// Set the "sub-second" nanoseconds of this `DateTime`.
    pub fn set_subsec_nanos(&mut self, subsec_nanos: u32) -> &mut Self {
        self.subsecond_nanos = subsec_nanos;
        self
    }

    /// Converts the `DateTime` to the number of milliseconds since the Unix epoch.
    ///
    /// This is fallible since `DateTime` holds more precision than an `i64`, and will
    /// return a `ConversionError` for `DateTime` values that can't be converted.
    pub fn to_millis(self) -> Result<i64, ConversionError> {
        let subsec_millis =
            Integer::div_floor(&i64::from(self.subsecond_nanos), &(NANOS_PER_MILLI as i64));
        if self.seconds < 0 {
            self.seconds
                .checked_add(1)
                .and_then(|seconds| seconds.checked_mul(MILLIS_PER_SECOND))
                .and_then(|millis| millis.checked_sub(1000 - subsec_millis))
        } else {
            self.seconds
                .checked_mul(MILLIS_PER_SECOND)
                .and_then(|millis| millis.checked_add(subsec_millis))
        }
        .ok_or(ConversionError(
            "DateTime value too large to fit into i64 epoch millis",
        ))
    }

    /// Read 1 date of `format` from `s`, expecting either `delim` or EOF
    ///
    /// Enable parsing multiple dates from the same string
    pub fn read(s: &str, format: Format, delim: char) -> Result<(Self, &str), DateTimeParseError> {
        let (inst, next) = match format {
            Format::DateTime => format::rfc3339::read(s, AllowOffsets::OffsetsForbidden)?,
            Format::DateTimeWithOffset => format::rfc3339::read(s, AllowOffsets::OffsetsAllowed)?,
            Format::HttpDate => format::http_date::read(s)?,
            Format::EpochSeconds => {
                let split_point = s.find(delim).unwrap_or(s.len());
                let (s, rest) = s.split_at(split_point);
                (Self::from_str(s, format)?, rest)
            }
        };
        if next.is_empty() {
            Ok((inst, next))
        } else if next.starts_with(delim) {
            Ok((inst, &next[1..]))
        } else {
            Err(DateTimeParseErrorKind::Invalid("didn't find expected delimiter".into()).into())
        }
    }

    /// Formats the `DateTime` to a string using the given `format`.
    ///
    /// Returns an error if the given `DateTime` cannot be represented by the desired format.
    pub fn fmt(&self, format: Format) -> Result<String, DateTimeFormatError> {
        match format {
            Format::DateTime | Format::DateTimeWithOffset => format::rfc3339::format(self),
            Format::EpochSeconds => Ok(format::epoch_seconds::format(self)),
            Format::HttpDate => format::http_date::format(self),
        }
    }
}

/// Tries to convert a [`DateTime`] into a [`SystemTime`].
///
/// This can fail if the the `DateTime` value is larger or smaller than what the `SystemTime`
/// can represent on the operating system it's compiled for. On Linux, for example, it will only
/// fail on `Instant::from_secs(i64::MIN)` (with any nanoseconds value). On Windows, however,
/// Rust's standard library uses a smaller precision type for `SystemTime`, and it will fail
/// conversion for a much larger range of date-times. This is only an issue if dealing with
/// date-times beyond several thousands of years from now.
impl TryFrom<DateTime> for SystemTime {
    type Error = ConversionError;

    fn try_from(date_time: DateTime) -> Result<Self, Self::Error> {
        if date_time.secs() < 0 {
            let mut secs = date_time.secs().unsigned_abs();
            let mut nanos = date_time.subsec_nanos();
            if date_time.has_subsec_nanos() {
                // This is safe because we just went from a negative number to a positive and are subtracting
                secs -= 1;
                // This is safe because nanos are < 999,999,999
                nanos = NANOS_PER_SECOND_U32 - nanos;
            }
            UNIX_EPOCH
                .checked_sub(Duration::new(secs, nanos))
                .ok_or(ConversionError(
                    "overflow occurred when subtracting duration from UNIX_EPOCH",
                ))
        } else {
            UNIX_EPOCH
                .checked_add(Duration::new(
                    date_time.secs().unsigned_abs(),
                    date_time.subsec_nanos(),
                ))
                .ok_or(ConversionError(
                    "overflow occurred when adding duration to UNIX_EPOCH",
                ))
        }
    }
}

impl From<SystemTime> for DateTime {
    fn from(time: SystemTime) -> Self {
        if time < UNIX_EPOCH {
            let duration = UNIX_EPOCH.duration_since(time).expect("time < UNIX_EPOCH");
            let mut secs = -(duration.as_secs() as i128);
            let mut nanos = duration.subsec_nanos() as i128;
            if nanos != 0 {
                secs -= 1;
                nanos = NANOS_PER_SECOND - nanos;
            }
            DateTime::from_nanos(secs * NANOS_PER_SECOND + nanos)
                .expect("SystemTime has same precision as DateTime")
        } else {
            let duration = time.duration_since(UNIX_EPOCH).expect("UNIX_EPOCH <= time");
            DateTime::from_secs_and_nanos(
                i64::try_from(duration.as_secs())
                    .expect("SystemTime has same precision as DateTime"),
                duration.subsec_nanos(),
            )
        }
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_nanos().cmp(&other.as_nanos())
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Some dates are out of range to be serialized with `DateTime`.
        // In these cases, fallback to using epoch seconds which always works
        let date = match self.fmt(Format::DateTime) {
            Ok(date) => date,
            Err(_err) => format::epoch_seconds::format(self),
        };
        write!(f, "{date}")
    }
}

impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
/// Failure to convert a `DateTime` to or from another type.
#[derive(Debug)]
#[non_exhaustive]
pub struct ConversionError(&'static str);

impl StdError for ConversionError {}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Formats for representing a `DateTime` in the Smithy protocols.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    /// RFC-3339 Date Time. If the date time has an offset, an error will be returned.
    /// e.g. `2019-12-16T23:48:18Z`
    DateTime,

    /// RFC-3339 Date Time. Offsets are supported.
    /// e.g. `2019-12-16T23:48:18+01:00`
    DateTimeWithOffset,

    /// Date format used by the HTTP `Date` header, specified in RFC-7231.
    /// e.g. `Mon, 16 Dec 2019 23:48:18 GMT`
    HttpDate,

    /// Number of seconds since the Unix epoch formatted as a floating point.
    /// e.g. `1576540098.52`
    EpochSeconds,
}

#[cfg(test)]
mod test {
    use crate::date_time::Format;
    use crate::DateTime;
    use proptest::proptest;
    use std::time::SystemTime;
    use time::format_description::well_known::Rfc3339;
    use time::OffsetDateTime;

    #[test]
    fn test_display_date_time() {
        let date_time = DateTime::from_secs(1576540098);
        assert_eq!(format!("{date_time}"), "2019-12-16T23:48:18Z");

        let date_time = DateTime::from_fractional_secs(1576540098, 0.52);
        assert_eq!(format!("{date_time}"), "2019-12-16T23:48:18.52Z");

        let date_time = DateTime::from_secs(1699942527);
        assert_eq!(format!("{date_time}"), "2023-11-14T06:15:27Z");

        let date_time = DateTime::from_secs(16995123);
        assert_eq!(format!("{date_time}"), "1970-07-16T16:52:03Z");
    }

    #[test]
    fn test_debug_date_time() {
        let date_time = DateTime::from_secs(1576540098);
        assert_eq!(format!("{date_time:?}"), "2019-12-16T23:48:18Z");

        let date_time = DateTime::from_fractional_secs(1576540098, 0.52);
        assert_eq!(format!("{date_time:?}"), "2019-12-16T23:48:18.52Z");

        let date_time = DateTime::from_secs(1699942527);
        assert_eq!(format!("{date_time:?}"), "2023-11-14T06:15:27Z");

        let date_time = DateTime::from_secs(16995123);
        assert_eq!(format!("{date_time:?}"), "1970-07-16T16:52:03Z");
    }

    #[test]
    fn test_fmt() {
        let date_time = DateTime::from_secs(1576540098);
        assert_eq!(
            date_time.fmt(Format::DateTime).unwrap(),
            "2019-12-16T23:48:18Z"
        );
        assert_eq!(date_time.fmt(Format::EpochSeconds).unwrap(), "1576540098");
        assert_eq!(
            date_time.fmt(Format::HttpDate).unwrap(),
            "Mon, 16 Dec 2019 23:48:18 GMT"
        );

        let date_time = DateTime::from_fractional_secs(1576540098, 0.52);
        assert_eq!(
            date_time.fmt(Format::DateTime).unwrap(),
            "2019-12-16T23:48:18.52Z"
        );
        assert_eq!(
            date_time.fmt(Format::EpochSeconds).unwrap(),
            "1576540098.52"
        );
        assert_eq!(
            date_time.fmt(Format::HttpDate).unwrap(),
            "Mon, 16 Dec 2019 23:48:18 GMT"
        );
    }

    #[test]
    fn test_fmt_zero_seconds() {
        let date_time = DateTime::from_secs(1576540080);
        assert_eq!(
            date_time.fmt(Format::DateTime).unwrap(),
            "2019-12-16T23:48:00Z"
        );
        assert_eq!(date_time.fmt(Format::EpochSeconds).unwrap(), "1576540080");
        assert_eq!(
            date_time.fmt(Format::HttpDate).unwrap(),
            "Mon, 16 Dec 2019 23:48:00 GMT"
        );
    }

    #[test]
    fn test_read_single_http_date() {
        let s = "Mon, 16 Dec 2019 23:48:18 GMT";
        let (_, next) = DateTime::read(s, Format::HttpDate, ',').expect("valid");
        assert_eq!(next, "");
    }

    #[test]
    fn test_read_single_float() {
        let s = "1576540098.52";
        let (_, next) = DateTime::read(s, Format::EpochSeconds, ',').expect("valid");
        assert_eq!(next, "");
    }

    #[test]
    fn test_read_many_float() {
        let s = "1576540098.52,1576540098.53";
        let (_, next) = DateTime::read(s, Format::EpochSeconds, ',').expect("valid");
        assert_eq!(next, "1576540098.53");
    }

    #[test]
    fn test_ready_many_http_date() {
        let s = "Mon, 16 Dec 2019 23:48:18 GMT,Tue, 17 Dec 2019 23:48:18 GMT";
        let (_, next) = DateTime::read(s, Format::HttpDate, ',').expect("valid");
        assert_eq!(next, "Tue, 17 Dec 2019 23:48:18 GMT");
    }

    #[derive(Debug)]
    struct EpochMillisTestCase {
        _rfc3339: &'static str,
        epoch_millis: i64,
        epoch_seconds: i64,
        epoch_subsec_nanos: u32,
    }

    // These test case values were generated from the following Kotlin JVM code:
    // ```kotlin
    // val date_time = DateTime.ofEpochMilli(<epoch milli value>);
    // println(DateTimeFormatter.ISO_DATE_TIME.format(date_time.atOffset(ZoneOffset.UTC)))
    // println(date_time.epochSecond)
    // println(date_time.nano)
    // ```
    const EPOCH_MILLIS_TEST_CASES: &[EpochMillisTestCase] = &[
        EpochMillisTestCase {
            _rfc3339: "2021-07-30T21:20:04.123Z",
            epoch_millis: 1627680004123,
            epoch_seconds: 1627680004,
            epoch_subsec_nanos: 123000000,
        },
        EpochMillisTestCase {
            _rfc3339: "1918-06-04T02:39:55.877Z",
            epoch_millis: -1627680004123,
            epoch_seconds: -1627680005,
            epoch_subsec_nanos: 877000000,
        },
        EpochMillisTestCase {
            _rfc3339: "+292278994-08-17T07:12:55.807Z",
            epoch_millis: i64::MAX,
            epoch_seconds: 9223372036854775,
            epoch_subsec_nanos: 807000000,
        },
        EpochMillisTestCase {
            _rfc3339: "-292275055-05-16T16:47:04.192Z",
            epoch_millis: i64::MIN,
            epoch_seconds: -9223372036854776,
            epoch_subsec_nanos: 192000000,
        },
    ];

    #[test]
    fn to_millis() {
        for test_case in EPOCH_MILLIS_TEST_CASES {
            println!("Test case: {test_case:?}");
            let date_time = DateTime::from_secs_and_nanos(
                test_case.epoch_seconds,
                test_case.epoch_subsec_nanos,
            );
            assert_eq!(test_case.epoch_seconds, date_time.secs());
            assert_eq!(test_case.epoch_subsec_nanos, date_time.subsec_nanos());
            assert_eq!(test_case.epoch_millis, date_time.to_millis().unwrap());
        }

        assert!(DateTime::from_secs_and_nanos(i64::MAX, 0)
            .to_millis()
            .is_err());
    }

    #[test]
    fn from_millis() {
        for test_case in EPOCH_MILLIS_TEST_CASES {
            println!("Test case: {test_case:?}");
            let date_time = DateTime::from_millis(test_case.epoch_millis);
            assert_eq!(test_case.epoch_seconds, date_time.secs());
            assert_eq!(test_case.epoch_subsec_nanos, date_time.subsec_nanos());
        }
    }

    #[test]
    fn to_from_millis_round_trip() {
        for millis in &[0, 1627680004123, -1627680004123, i64::MAX, i64::MIN] {
            assert_eq!(*millis, DateTime::from_millis(*millis).to_millis().unwrap());
        }
    }

    #[test]
    fn as_nanos() {
        assert_eq!(
            -9_223_372_036_854_775_807_000_000_001_i128,
            DateTime::from_secs_and_nanos(i64::MIN, 999_999_999).as_nanos()
        );
        assert_eq!(
            -10_876_543_211,
            DateTime::from_secs_and_nanos(-11, 123_456_789).as_nanos()
        );
        assert_eq!(0, DateTime::from_secs_and_nanos(0, 0).as_nanos());
        assert_eq!(
            11_123_456_789,
            DateTime::from_secs_and_nanos(11, 123_456_789).as_nanos()
        );
        assert_eq!(
            9_223_372_036_854_775_807_999_999_999_i128,
            DateTime::from_secs_and_nanos(i64::MAX, 999_999_999).as_nanos()
        );
    }

    #[test]
    fn from_nanos() {
        assert_eq!(
            DateTime::from_secs_and_nanos(i64::MIN, 999_999_999),
            DateTime::from_nanos(-9_223_372_036_854_775_807_000_000_001_i128).unwrap(),
        );
        assert_eq!(
            DateTime::from_secs_and_nanos(-11, 123_456_789),
            DateTime::from_nanos(-10_876_543_211).unwrap(),
        );
        assert_eq!(
            DateTime::from_secs_and_nanos(0, 0),
            DateTime::from_nanos(0).unwrap(),
        );
        assert_eq!(
            DateTime::from_secs_and_nanos(11, 123_456_789),
            DateTime::from_nanos(11_123_456_789).unwrap(),
        );
        assert_eq!(
            DateTime::from_secs_and_nanos(i64::MAX, 999_999_999),
            DateTime::from_nanos(9_223_372_036_854_775_807_999_999_999_i128).unwrap(),
        );
        assert!(DateTime::from_nanos(-10_000_000_000_000_000_000_999_999_999_i128).is_err());
        assert!(DateTime::from_nanos(10_000_000_000_000_000_000_999_999_999_i128).is_err());
    }

    // TODO(https://github.com/smithy-lang/smithy-rs/issues/1857)
    #[cfg(not(any(target_arch = "powerpc", target_arch = "x86")))]
    #[test]
    fn system_time_conversions() {
        // Check agreement
        let date_time = DateTime::from_str("1000-01-02T01:23:10.123Z", Format::DateTime).unwrap();
        let off_date_time = OffsetDateTime::parse("1000-01-02T01:23:10.123Z", &Rfc3339).unwrap();
        assert_eq!(
            SystemTime::from(off_date_time),
            SystemTime::try_from(date_time).unwrap()
        );

        let date_time = DateTime::from_str("2039-10-31T23:23:10.456Z", Format::DateTime).unwrap();
        let off_date_time = OffsetDateTime::parse("2039-10-31T23:23:10.456Z", &Rfc3339).unwrap();
        assert_eq!(
            SystemTime::from(off_date_time),
            SystemTime::try_from(date_time).unwrap()
        );
    }

    #[test]
    fn formatting_of_early_dates() {
        let date: DateTime =
            DateTime::from_str("Mon, 16 Dec -019 23:48:18 GMT", Format::HttpDate).unwrap();
        assert_eq!(format!("{date}"), "-62736509502");
    }

    #[test]
    fn ord() {
        let first = DateTime::from_secs_and_nanos(-1, 0);
        let second = DateTime::from_secs_and_nanos(-1, 1);
        let third = DateTime::from_secs_and_nanos(0, 0);
        let fourth = DateTime::from_secs_and_nanos(0, 1);
        let fifth = DateTime::from_secs_and_nanos(1, 0);

        assert!(first == first);
        assert!(first < second);
        assert!(first < third);
        assert!(first < fourth);
        assert!(first < fifth);

        assert!(second > first);
        assert!(second == second);
        assert!(second < third);
        assert!(second < fourth);
        assert!(second < fifth);

        assert!(third > first);
        assert!(third > second);
        assert!(third == third);
        assert!(third < fourth);
        assert!(third < fifth);

        assert!(fourth > first);
        assert!(fourth > second);
        assert!(fourth > third);
        assert!(fourth == fourth);
        assert!(fourth < fifth);

        assert!(fifth > first);
        assert!(fifth > second);
        assert!(fifth > third);
        assert!(fifth > fourth);
        assert!(fifth == fifth);
    }

    /// https://github.com/smithy-lang/smithy-rs/issues/3805
    #[test]
    fn panic_in_fromsecs_f64() {
        assert_eq!(DateTime::from_secs_f64(-1.0), DateTime::from_secs(-1));

        assert_eq!(
            DateTime::from_secs_f64(-1.95877825437922e-309),
            DateTime::from_secs(0)
        );
    }

    const MIN_RFC_3339_MILLIS: i64 = -62135596800000;
    const MAX_RFC_3339_MILLIS: i64 = 253402300799999;

    // This test uses milliseconds, because `Format::DateTime` does not support nanoseconds.
    proptest! {
        #[test]
        fn ord_proptest(
            left_millis in MIN_RFC_3339_MILLIS..MAX_RFC_3339_MILLIS,
            right_millis in MIN_RFC_3339_MILLIS..MAX_RFC_3339_MILLIS,
        ) {
            let left = DateTime::from_millis(left_millis);
            let right = DateTime::from_millis(right_millis);

            let left_str = left.fmt(Format::DateTime).unwrap();
            let right_str = right.fmt(Format::DateTime).unwrap();

            assert_eq!(left.cmp(&right), left_str.cmp(&right_str));
        }
    }

    proptest! {
        #[test]
        fn from_secs_f64_proptest(secs: f64) {
            let _date = DateTime::from_secs_f64(secs);
        }
    }
}
