use std::error::Error as StdError;
use std::fmt;
use std::str::{Chars, FromStr};
use std::time::Duration;

/// Error parsing human-friendly duration
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// Invalid character during parsing
    ///
    /// More specifically anything that is not alphanumeric is prohibited
    ///
    /// The field is an byte offset of the character in the string.
    InvalidCharacter(usize),
    /// Non-numeric value where number is expected
    ///
    /// This usually means that either time unit is broken into words,
    /// e.g. `m sec` instead of `msec`, or just number is omitted,
    /// for example `2 hours min` instead of `2 hours 1 min`
    ///
    /// The field is an byte offset of the errorneous character
    /// in the string.
    NumberExpected(usize),
    /// Unit in the number is not one of allowed units
    ///
    /// See documentation of `parse_duration` for the list of supported
    /// time units.
    ///
    /// The two fields are start and end (exclusive) of the slice from
    /// the original string, containing errorneous value
    UnknownUnit {
        /// Start of the invalid unit inside the original string
        start: usize,
        /// End of the invalid unit inside the original string
        end: usize,
        /// The unit verbatim
        unit: String,
        /// A number associated with the unit
        value: u64,
    },
    /// The numeric value exceeds the limits of this library.
    ///
    /// This can mean two things:
    /// - The value is too large to be useful.
    ///   For instance, the maximum duration written with subseconds unit is about 3000 years.
    /// - The attempted precision is not supported.
    ///   For instance, a duration of `0.5ns` is not supported,
    ///   because durations below one nanosecond cannot be represented.
    // NOTE: it would be more logical to create a separate `NumberPrecisionLimit` error,
    // but that would be a breaking change. Reconsider this for the next major version.
    NumberOverflow,
    /// The value was an empty string (or consists only whitespace)
    Empty,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidCharacter(offset) => write!(f, "invalid character at {}", offset),
            Error::NumberExpected(offset) => write!(f, "expected number at {}", offset),
            Error::UnknownUnit { unit, value, .. } if unit.is_empty() => {
                write!(f, "time unit needed, for example {0}sec or {0}ms", value)
            }
            Error::UnknownUnit { unit, .. } => {
                write!(
                    f,
                    "unknown time unit {:?}, \
                    supported units: ns, us/µs, ms, sec, min, hours, days, \
                    weeks, months, years (and few variations)",
                    unit
                )
            }
            Error::NumberOverflow => write!(f, "number is too large or cannot be represented without a lack of precision (values below 1ns are not supported)"),
            Error::Empty => write!(f, "value was empty"),
        }
    }
}

/// A wrapper type that allows you to Display a Duration
#[derive(Debug, Clone)]
pub struct FormattedDuration(Duration);

trait OverflowOp: Sized {
    fn mul(self, other: Self) -> Result<Self, Error>;
    fn add(self, other: Self) -> Result<Self, Error>;
    fn div(self, other: Self) -> Result<Self, Error>;
}

impl OverflowOp for u64 {
    fn mul(self, other: Self) -> Result<Self, Error> {
        self.checked_mul(other).ok_or(Error::NumberOverflow)
    }
    fn add(self, other: Self) -> Result<Self, Error> {
        self.checked_add(other).ok_or(Error::NumberOverflow)
    }
    fn div(self, other: Self) -> Result<Self, Error> {
        match self % other {
            0 => Ok(self / other),
            _ => Err(Error::NumberOverflow),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Fraction {
    numerator: u64,
    denominator: u64,
}

struct Parser<'a> {
    iter: Chars<'a>,
    src: &'a str,
}

impl Parser<'_> {
    fn parse(mut self) -> Result<Duration, Error> {
        let mut n = self.parse_first_char()?.ok_or(Error::Empty)?; // integer part
        let mut out = Duration::ZERO;
        'outer: loop {
            let mut frac = None; // fractional part
            let mut off = self.off();
            while let Some(c) = self.iter.next() {
                match c {
                    '0'..='9' => {
                        n = n
                            .checked_mul(10)
                            .and_then(|x| x.checked_add(c as u64 - '0' as u64))
                            .ok_or(Error::NumberOverflow)?;
                    }
                    c if c.is_whitespace() => {}
                    'a'..='z' | 'A'..='Z' | 'µ' => {
                        break;
                    }
                    '.' => {
                        // decimal separator, the fractional part begins now
                        frac = Some(self.parse_fractional_part(&mut off)?);
                        break;
                    }
                    _ => {
                        return Err(Error::InvalidCharacter(off));
                    }
                }
                off = self.off();
            }
            let start = off;
            let mut off = self.off();
            while let Some(c) = self.iter.next() {
                match c {
                    '0'..='9' => {
                        self.parse_unit(n, frac, start, off, &mut out)?;
                        n = c as u64 - '0' as u64;
                        continue 'outer;
                    }
                    c if c.is_whitespace() => break,
                    'a'..='z' | 'A'..='Z' | 'µ' => {}
                    _ => {
                        return Err(Error::InvalidCharacter(off));
                    }
                }
                off = self.off();
            }

            self.parse_unit(n, frac, start, off, &mut out)?;
            n = match self.parse_first_char()? {
                Some(n) => n,
                None => return Ok(out),
            };
        }
    }

    fn parse_first_char(&mut self) -> Result<Option<u64>, Error> {
        let off = self.off();
        for c in self.iter.by_ref() {
            match c {
                '0'..='9' => {
                    return Ok(Some(c as u64 - '0' as u64));
                }
                c if c.is_whitespace() => continue,
                _ => {
                    return Err(Error::NumberExpected(off));
                }
            }
        }
        Ok(None)
    }

    fn parse_fractional_part(&mut self, off: &mut usize) -> Result<Fraction, Error> {
        let mut numerator = 0u64;
        let mut denominator = 1u64;
        let mut zeros = true;
        while let Some(c) = self.iter.next() {
            match c {
                '0' => {
                    denominator = denominator.checked_mul(10).ok_or(Error::NumberOverflow)?;
                    if !zeros {
                        numerator = numerator.checked_mul(10).ok_or(Error::NumberOverflow)?;
                    }
                }
                '1'..='9' => {
                    zeros = false;
                    denominator = denominator.checked_mul(10).ok_or(Error::NumberOverflow)?;
                    numerator = numerator
                        .checked_mul(10)
                        .and_then(|x| x.checked_add(c as u64 - '0' as u64))
                        .ok_or(Error::NumberOverflow)?;
                }
                c if c.is_whitespace() => {}
                'a'..='z' | 'A'..='Z' | 'µ' => {
                    break;
                }
                _ => {
                    return Err(Error::InvalidCharacter(*off));
                }
            };
            // update the offset used by the parsing loop
            *off = self.off();
        }
        if denominator == 1 {
            // no digits were given after the separator, e.g. "1."
            return Err(Error::InvalidCharacter(*off));
        }
        Ok(Fraction {
            numerator,
            denominator,
        })
    }

    fn off(&self) -> usize {
        self.src.len() - self.iter.as_str().len()
    }

    fn parse_unit(
        &mut self,
        n: u64,
        frac: Option<Fraction>,
        start: usize,
        end: usize,
        out: &mut Duration,
    ) -> Result<(), Error> {
        let unit = match Unit::from_str(&self.src[start..end]) {
            Ok(u) => u,
            Err(()) => {
                return Err(Error::UnknownUnit {
                    start,
                    end,
                    unit: self.src[start..end].to_owned(),
                    value: n,
                });
            }
        };

        // add the integer part
        let (sec, nsec) = match unit {
            Unit::Nanosecond => (0u64, n),
            Unit::Microsecond => (0u64, n.mul(1000)?),
            Unit::Millisecond => (0u64, n.mul(1_000_000)?),
            Unit::Second => (n, 0),
            Unit::Minute => (n.mul(60)?, 0),
            Unit::Hour => (n.mul(3600)?, 0),
            Unit::Day => (n.mul(86400)?, 0),
            Unit::Week => (n.mul(86400 * 7)?, 0),
            Unit::Month => (n.mul(2_630_016)?, 0), // 30.44d
            Unit::Year => (n.mul(31_557_600)?, 0), // 365.25d
        };
        add_current(sec, nsec, out)?;

        // add the fractional part
        if let Some(Fraction {
            numerator: n,
            denominator: d,
        }) = frac
        {
            let (sec, nsec) = match unit {
                Unit::Nanosecond => return Err(Error::NumberOverflow),
                Unit::Microsecond => (0, n.mul(1000)?.div(d)?),
                Unit::Millisecond => (0, n.mul(1_000_000)?.div(d)?),
                Unit::Second => (0, n.mul(1_000_000_000)?.div(d)?),
                Unit::Minute => (0, n.mul(60_000_000_000)?.div(d)?),
                Unit::Hour => (n.mul(3600)?.div(d)?, 0),
                Unit::Day => (n.mul(86400)?.div(d)?, 0),
                Unit::Week => (n.mul(86400 * 7)?.div(d)?, 0),
                Unit::Month => (n.mul(2_630_016)?.div(d)?, 0), // 30.44d
                Unit::Year => (n.mul(31_557_600)?.div(d)?, 0), // 365.25d
            };
            add_current(sec, nsec, out)?;
        }

        Ok(())
    }
}

fn add_current(mut sec: u64, nsec: u64, out: &mut Duration) -> Result<(), Error> {
    let mut nsec = (out.subsec_nanos() as u64).add(nsec)?;
    if nsec > 1_000_000_000 {
        sec = sec.add(nsec / 1_000_000_000)?;
        nsec %= 1_000_000_000;
    }
    sec = out.as_secs().add(sec)?;
    *out = Duration::new(sec, nsec as u32);
    Ok(())
}

enum Unit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl FromStr for Unit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nanos" | "nsec" | "ns" => Ok(Self::Nanosecond),
            "usec" | "us" | "µs" => Ok(Self::Microsecond),
            "millis" | "msec" | "ms" => Ok(Self::Millisecond),
            "seconds" | "second" | "secs" | "sec" | "s" => Ok(Self::Second),
            "minutes" | "minute" | "min" | "mins" | "m" => Ok(Self::Minute),
            "hours" | "hour" | "hr" | "hrs" | "h" => Ok(Self::Hour),
            "days" | "day" | "d" => Ok(Self::Day),
            "weeks" | "week" | "wk" | "wks" | "w" => Ok(Self::Week),
            "months" | "month" | "M" => Ok(Self::Month),
            "years" | "year" | "yr" | "yrs" | "y" => Ok(Self::Year),
            _ => Err(()),
        }
    }
}

/// Parse duration object `1hour 12min 5s`
///
/// The duration object is a concatenation of time spans. Where each time
/// span is an integer number and a suffix. Supported suffixes:
///
/// * `nsec`, `ns` -- nanoseconds
/// * `usec`, `us`, `µs` -- microseconds
/// * `msec`, `ms` -- milliseconds
/// * `seconds`, `second`, `sec`, `s`
/// * `minutes`, `minute`, `min`, `m`
/// * `hours`, `hour`, `hr`, `hrs`, `h`
/// * `days`, `day`, `d`
/// * `weeks`, `week`, `wk`, `wks`, `w`
/// * `months`, `month`, `M` -- defined as 30.44 days
/// * `years`, `year`, `yr`, `yrs`, `y` -- defined as 365.25 days
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use humantime::parse_duration;
///
/// assert_eq!(parse_duration("2h 37min"), Ok(Duration::new(9420, 0)));
/// assert_eq!(parse_duration("32ms"), Ok(Duration::new(0, 32_000_000)));
/// assert_eq!(parse_duration("4.2s"), Ok(Duration::new(4, 200_000_000)));
/// ```
pub fn parse_duration(s: &str) -> Result<Duration, Error> {
    if s == "0" {
        return Ok(Duration::ZERO);
    }
    Parser {
        iter: s.chars(),
        src: s,
    }
    .parse()
}

/// Formats duration into a human-readable string
///
/// Note: this format is guaranteed to have same value when using
/// parse_duration, but we can change some details of the exact composition
/// of the value.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use humantime::format_duration;
///
/// let val1 = Duration::new(9420, 0);
/// assert_eq!(format_duration(val1).to_string(), "2h 37m");
/// let val2 = Duration::new(0, 32_000_000);
/// assert_eq!(format_duration(val2).to_string(), "32ms");
/// ```
pub fn format_duration(val: Duration) -> FormattedDuration {
    FormattedDuration(val)
}

fn item_plural(f: &mut fmt::Formatter, started: &mut bool, name: &str, value: u64) -> fmt::Result {
    if value > 0 {
        if *started {
            f.write_str(" ")?;
        }
        write!(f, "{}{}", value, name)?;
        if value > 1 {
            f.write_str("s")?;
        }
        *started = true;
    }
    Ok(())
}
fn item(f: &mut fmt::Formatter, started: &mut bool, name: &str, value: u32) -> fmt::Result {
    if value > 0 {
        if *started {
            f.write_str(" ")?;
        }
        write!(f, "{}{}", value, name)?;
        *started = true;
    }
    Ok(())
}

impl FormattedDuration {
    /// Returns a reference to the [`Duration`][] that is being formatted.
    pub fn get_ref(&self) -> &Duration {
        &self.0
    }
}

impl fmt::Display for FormattedDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let secs = self.0.as_secs();
        let nanos = self.0.subsec_nanos();

        if secs == 0 && nanos == 0 {
            f.write_str("0s")?;
            return Ok(());
        }

        let years = secs / 31_557_600; // 365.25d
        let ydays = secs % 31_557_600;
        let months = ydays / 2_630_016; // 30.44d
        let mdays = ydays % 2_630_016;
        let days = mdays / 86400;
        let day_secs = mdays % 86400;
        let hours = day_secs / 3600;
        let minutes = day_secs % 3600 / 60;
        let seconds = day_secs % 60;

        let millis = nanos / 1_000_000;
        let micros = nanos / 1000 % 1000;
        let nanosec = nanos % 1000;

        let started = &mut false;
        item_plural(f, started, "year", years)?;
        item_plural(f, started, "month", months)?;
        item_plural(f, started, "day", days)?;
        item(f, started, "h", hours as u32)?;
        item(f, started, "m", minutes as u32)?;
        item(f, started, "s", seconds as u32)?;
        item(f, started, "ms", millis)?;
        #[cfg(feature = "mu")]
        item(f, started, "µs", micros)?;
        #[cfg(not(feature = "mu"))]
        item(f, started, "us", micros)?;
        item(f, started, "ns", nanosec)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use rand::Rng;

    use super::Error;
    use super::{format_duration, parse_duration};

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_units() {
        assert_eq!(parse_duration("17nsec"), Ok(Duration::new(0, 17)));
        assert_eq!(parse_duration("17nanos"), Ok(Duration::new(0, 17)));
        assert_eq!(parse_duration("33ns"), Ok(Duration::new(0, 33)));
        assert_eq!(parse_duration("3usec"), Ok(Duration::new(0, 3000)));
        assert_eq!(parse_duration("78us"), Ok(Duration::new(0, 78000)));
        assert_eq!(parse_duration("163µs"), Ok(Duration::new(0, 163000)));
        assert_eq!(parse_duration("31msec"), Ok(Duration::new(0, 31_000_000)));
        assert_eq!(parse_duration("31millis"), Ok(Duration::new(0, 31_000_000)));
        assert_eq!(parse_duration("6ms"), Ok(Duration::new(0, 6_000_000)));
        assert_eq!(parse_duration("3000s"), Ok(Duration::new(3000, 0)));
        assert_eq!(parse_duration("300sec"), Ok(Duration::new(300, 0)));
        assert_eq!(parse_duration("300secs"), Ok(Duration::new(300, 0)));
        assert_eq!(parse_duration("50seconds"), Ok(Duration::new(50, 0)));
        assert_eq!(parse_duration("1second"), Ok(Duration::new(1, 0)));
        assert_eq!(parse_duration("100m"), Ok(Duration::new(6000, 0)));
        assert_eq!(parse_duration("12min"), Ok(Duration::new(720, 0)));
        assert_eq!(parse_duration("12mins"), Ok(Duration::new(720, 0)));
        assert_eq!(parse_duration("1minute"), Ok(Duration::new(60, 0)));
        assert_eq!(parse_duration("7minutes"), Ok(Duration::new(420, 0)));
        assert_eq!(parse_duration("2h"), Ok(Duration::new(7200, 0)));
        assert_eq!(parse_duration("7hr"), Ok(Duration::new(25200, 0)));
        assert_eq!(parse_duration("7hrs"), Ok(Duration::new(25200, 0)));
        assert_eq!(parse_duration("1hour"), Ok(Duration::new(3600, 0)));
        assert_eq!(parse_duration("24hours"), Ok(Duration::new(86400, 0)));
        assert_eq!(parse_duration("1day"), Ok(Duration::new(86400, 0)));
        assert_eq!(parse_duration("2days"), Ok(Duration::new(172_800, 0)));
        assert_eq!(parse_duration("365d"), Ok(Duration::new(31_536_000, 0)));
        assert_eq!(parse_duration("1week"), Ok(Duration::new(604_800, 0)));
        assert_eq!(parse_duration("7weeks"), Ok(Duration::new(4_233_600, 0)));
        assert_eq!(
            parse_duration("104wks"),
            Ok(Duration::new(2 * 31_449_600, 0))
        );
        assert_eq!(parse_duration("100wk"), Ok(Duration::new(60_480_000, 0)));
        assert_eq!(parse_duration("52w"), Ok(Duration::new(31_449_600, 0)));
        assert_eq!(parse_duration("1month"), Ok(Duration::new(2_630_016, 0)));
        assert_eq!(
            parse_duration("3months"),
            Ok(Duration::new(3 * 2_630_016, 0))
        );
        assert_eq!(parse_duration("12M"), Ok(Duration::new(31_560_192, 0)));
        assert_eq!(parse_duration("1year"), Ok(Duration::new(31_557_600, 0)));
        assert_eq!(
            parse_duration("7years"),
            Ok(Duration::new(7 * 31_557_600, 0))
        );
        assert_eq!(
            parse_duration("15yrs"),
            Ok(Duration::new(15 * 31_557_600, 0))
        );
        assert_eq!(
            parse_duration("10yr"),
            Ok(Duration::new(10 * 31_557_600, 0))
        );
        assert_eq!(parse_duration("17y"), Ok(Duration::new(536_479_200, 0)));
    }

    #[test]
    fn test_fractional_bad_input() {
        assert!(matches!(
            parse_duration("1.s"),
            Err(Error::InvalidCharacter(_))
        ));
        assert!(matches!(
            parse_duration("1..s"),
            Err(Error::InvalidCharacter(_))
        ));
        assert!(matches!(
            parse_duration(".1s"),
            Err(Error::NumberExpected(_))
        ));
        assert!(matches!(parse_duration("."), Err(Error::NumberExpected(_))));
        assert_eq!(
            parse_duration("0.000123456789s"),
            Err(Error::NumberOverflow)
        );
    }

    #[test]
    fn test_fractional_units() {
        // nanos
        for input in &["17.5nsec", "5.1nanos", "0.0005ns"] {
            let bad_ns_frac = parse_duration(input);
            assert!(
                matches!(bad_ns_frac, Err(Error::NumberOverflow)),
                "fractions of nanoseconds should fail, but got {bad_ns_frac:?}"
            );
        }

        // micros
        assert_eq!(parse_duration("3.1usec"), Ok(Duration::new(0, 3100)));
        assert_eq!(parse_duration("3.1us"), Ok(Duration::new(0, 3100)));
        assert_eq!(parse_duration("3.01us"), Ok(Duration::new(0, 3010)));
        assert_eq!(parse_duration("3.001us"), Ok(Duration::new(0, 3001)));
        for input in &["3.0001us", "0.0001us", "0.123456us"] {
            let bad_ms_frac = parse_duration(input);
            assert!(
                matches!(bad_ms_frac, Err(Error::NumberOverflow)),
                "too small fractions of microseconds should fail, but got {bad_ms_frac:?}"
            );
        }

        // millis
        assert_eq!(parse_duration("31.1msec"), Ok(Duration::new(0, 31_100_000)));
        assert_eq!(
            parse_duration("31.1millis"),
            Ok(Duration::new(0, 31_100_000))
        );
        assert_eq!(parse_duration("31.1ms"), Ok(Duration::new(0, 31_100_000)));
        assert_eq!(parse_duration("31.01ms"), Ok(Duration::new(0, 31_010_000)));
        assert_eq!(parse_duration("31.001ms"), Ok(Duration::new(0, 31_001_000)));
        assert_eq!(
            parse_duration("31.0001ms"),
            Ok(Duration::new(0, 31_000_100))
        );
        assert_eq!(
            parse_duration("31.00001ms"),
            Ok(Duration::new(0, 31_000_010))
        );
        assert_eq!(
            parse_duration("31.000001ms"),
            Ok(Duration::new(0, 31_000_001))
        );
        assert!(matches!(
            parse_duration("31.0000001ms"),
            Err(Error::NumberOverflow)
        ));

        // seconds
        assert_eq!(parse_duration("300.0sec"), Ok(Duration::new(300, 0)));
        assert_eq!(parse_duration("300.0secs"), Ok(Duration::new(300, 0)));
        assert_eq!(parse_duration("300.0seconds"), Ok(Duration::new(300, 0)));
        assert_eq!(parse_duration("300.0s"), Ok(Duration::new(300, 0)));
        assert_eq!(parse_duration("0.0s"), Ok(Duration::new(0, 0)));
        assert_eq!(parse_duration("0.2s"), Ok(Duration::new(0, 200_000_000)));
        assert_eq!(parse_duration("1.2s"), Ok(Duration::new(1, 200_000_000)));
        assert_eq!(parse_duration("1.02s"), Ok(Duration::new(1, 20_000_000)));
        assert_eq!(parse_duration("1.002s"), Ok(Duration::new(1, 2_000_000)));
        assert_eq!(parse_duration("1.0002s"), Ok(Duration::new(1, 200_000)));
        assert_eq!(parse_duration("1.00002s"), Ok(Duration::new(1, 20_000)));
        assert_eq!(parse_duration("1.000002s"), Ok(Duration::new(1, 2_000)));
        assert_eq!(parse_duration("1.0000002s"), Ok(Duration::new(1, 200)));
        assert_eq!(parse_duration("1.00000002s"), Ok(Duration::new(1, 20)));
        assert_eq!(parse_duration("1.000000002s"), Ok(Duration::new(1, 2)));
        assert_eq!(
            parse_duration("1.123456789s"),
            Ok(Duration::new(1, 123_456_789))
        );
        assert!(matches!(
            parse_duration("1.0000000002s"),
            Err(Error::NumberOverflow)
        ));
        assert!(matches!(
            parse_duration("0.0000000002s"),
            Err(Error::NumberOverflow)
        ));

        // minutes
        assert_eq!(parse_duration("100.0m"), Ok(Duration::new(6000, 0)));
        assert_eq!(parse_duration("12.1min"), Ok(Duration::new(726, 0)));
        assert_eq!(parse_duration("12.1mins"), Ok(Duration::new(726, 0)));
        assert_eq!(parse_duration("1.5minute"), Ok(Duration::new(90, 0)));
        assert_eq!(parse_duration("1.5minutes"), Ok(Duration::new(90, 0)));

        // hours
        assert_eq!(parse_duration("2.0h"), Ok(Duration::new(7200, 0)));
        assert_eq!(parse_duration("2.0hr"), Ok(Duration::new(7200, 0)));
        assert_eq!(parse_duration("2.0hrs"), Ok(Duration::new(7200, 0)));
        assert_eq!(parse_duration("2.0hours"), Ok(Duration::new(7200, 0)));
        assert_eq!(parse_duration("2.5h"), Ok(Duration::new(9000, 0)));
        assert_eq!(parse_duration("0.5h"), Ok(Duration::new(1800, 0)));

        // days
        assert_eq!(
            parse_duration("1.5day"),
            Ok(Duration::new(86400 + 86400 / 2, 0))
        );
        assert_eq!(
            parse_duration("1.5days"),
            Ok(Duration::new(86400 + 86400 / 2, 0))
        );
        assert_eq!(
            parse_duration("1.5d"),
            Ok(Duration::new(86400 + 86400 / 2, 0))
        );
        assert!(matches!(
            parse_duration("0.00000005d"),
            Err(Error::NumberOverflow)
        ));
    }

    #[test]
    fn test_fractional_combined() {
        assert_eq!(parse_duration("7.120us 3ns"), Ok(Duration::new(0, 7123)));
        assert_eq!(parse_duration("7.123us 4ns"), Ok(Duration::new(0, 7127)));
        assert_eq!(
            parse_duration("1.234s 789ns"),
            Ok(Duration::new(1, 234_000_789))
        );
        assert_eq!(
            parse_duration("1.234s 0.789us"),
            Ok(Duration::new(1, 234_000_789))
        );
        assert_eq!(
            parse_duration("1.234567s 0.789us"),
            Ok(Duration::new(1, 234_567_789))
        );
        assert_eq!(
            parse_duration("1.234s 1.345ms 1.678us 1ns"),
            Ok(Duration::new(1, 235_346_679))
        );
        assert_eq!(
            parse_duration("1.234s 0.345ms 0.678us 0ns"),
            Ok(Duration::new(1, 234_345_678))
        );
        assert_eq!(
            parse_duration("1.234s0.345ms0.678us0ns"),
            Ok(Duration::new(1, 234_345_678))
        );
    }

    #[test]
    fn allow_0_with_no_unit() {
        assert_eq!(parse_duration("0"), Ok(Duration::new(0, 0)));
    }

    #[test]
    fn test_combo() {
        assert_eq!(
            parse_duration("20 min 17 nsec "),
            Ok(Duration::new(1200, 17))
        );
        assert_eq!(parse_duration("2h 15m"), Ok(Duration::new(8100, 0)));
    }

    #[test]
    fn all_86400_seconds() {
        for second in 0..86400 {
            // scan leap year and non-leap year
            let d = Duration::new(second, 0);
            assert_eq!(d, parse_duration(&format_duration(d).to_string()).unwrap());
        }
    }

    #[test]
    fn random_second() {
        for _ in 0..10000 {
            let sec = rand::rng().random_range(0..253_370_764_800);
            let d = Duration::new(sec, 0);
            assert_eq!(d, parse_duration(&format_duration(d).to_string()).unwrap());
        }
    }

    #[test]
    fn random_any() {
        for _ in 0..10000 {
            let sec = rand::rng().random_range(0..253_370_764_800);
            let nanos = rand::rng().random_range(0..1_000_000_000);
            let d = Duration::new(sec, nanos);
            assert_eq!(d, parse_duration(&format_duration(d).to_string()).unwrap());
        }
    }

    #[test]
    fn test_overlow() {
        // Overflow on subseconds is earlier because of how we do conversion
        // we could fix it, but I don't see any good reason for this
        assert_eq!(
            parse_duration("100000000000000000000ns"),
            Err(Error::NumberOverflow)
        );
        assert_eq!(
            parse_duration("100000000000000000us"),
            Err(Error::NumberOverflow)
        );
        assert_eq!(
            parse_duration("100000000000000ms"),
            Err(Error::NumberOverflow)
        );

        assert_eq!(
            parse_duration("100000000000000000000s"),
            Err(Error::NumberOverflow)
        );
        assert_eq!(
            parse_duration("10000000000000000000m"),
            Err(Error::NumberOverflow)
        );
        assert_eq!(
            parse_duration("1000000000000000000h"),
            Err(Error::NumberOverflow)
        );
        assert_eq!(
            parse_duration("100000000000000000d"),
            Err(Error::NumberOverflow)
        );
        assert_eq!(
            parse_duration("10000000000000000w"),
            Err(Error::NumberOverflow)
        );
        assert_eq!(
            parse_duration("1000000000000000M"),
            Err(Error::NumberOverflow)
        );
        assert_eq!(
            parse_duration("10000000000000y"),
            Err(Error::NumberOverflow)
        );
    }

    #[test]
    fn test_nice_error_message() {
        assert_eq!(
            parse_duration("123").unwrap_err().to_string(),
            "time unit needed, for example 123sec or 123ms"
        );
        assert_eq!(
            parse_duration("10 months 1").unwrap_err().to_string(),
            "time unit needed, for example 1sec or 1ms"
        );
        assert_eq!(
            parse_duration("10nights").unwrap_err().to_string(),
            "unknown time unit \"nights\", supported units: \
            ns, us/µs, ms, sec, min, hours, days, weeks, months, \
            years (and few variations)"
        );
    }

    #[cfg(feature = "mu")]
    #[test]
    fn test_format_micros() {
        assert_eq!(
            format_duration(Duration::from_micros(123)).to_string(),
            "123µs"
        );
    }

    #[cfg(not(feature = "mu"))]
    #[test]
    fn test_format_micros() {
        assert_eq!(
            format_duration(Duration::from_micros(123)).to_string(),
            "123us"
        );
    }

    #[test]
    fn test_error_cases() {
        assert_eq!(
            parse_duration("\0").unwrap_err().to_string(),
            "expected number at 0"
        );
        assert_eq!(
            parse_duration("\r").unwrap_err().to_string(),
            "value was empty"
        );
        assert_eq!(
            parse_duration("1~").unwrap_err().to_string(),
            "invalid character at 1"
        );
        assert_eq!(
            parse_duration("1Nå").unwrap_err().to_string(),
            "invalid character at 2"
        );
        assert_eq!(parse_duration("222nsec221nanosmsec7s5msec572s").unwrap_err().to_string(),
                   "unknown time unit \"nanosmsec\", supported units: ns, us/µs, ms, sec, min, hours, days, weeks, months, years (and few variations)");
    }
}
