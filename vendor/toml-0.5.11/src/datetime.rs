use std::error;
use std::fmt;
use std::str::{self, FromStr};

use serde::{de, ser};

/// A parsed TOML datetime value
///
/// This structure is intended to represent the datetime primitive type that can
/// be encoded into TOML documents. This type is a parsed version that contains
/// all metadata internally.
///
/// Currently this type is intentionally conservative and only supports
/// `to_string` as an accessor. Over time though it's intended that it'll grow
/// more support!
///
/// Note that if you're using `Deserialize` to deserialize a TOML document, you
/// can use this as a placeholder for where you're expecting a datetime to be
/// specified.
///
/// Also note though that while this type implements `Serialize` and
/// `Deserialize` it's only recommended to use this type with the TOML format,
/// otherwise encoded in other formats it may look a little odd.
///
/// Depending on how the option values are used, this struct will correspond
/// with one of the following four datetimes from the [TOML v1.0.0 spec]:
///
/// | `date`    | `time`    | `offset`  | TOML type          |
/// | --------- | --------- | --------- | ------------------ |
/// | `Some(_)` | `Some(_)` | `Some(_)` | [Offset Date-Time] |
/// | `Some(_)` | `Some(_)` | `None`    | [Local Date-Time]  |
/// | `Some(_)` | `None`    | `None`    | [Local Date]       |
/// | `None`    | `Some(_)` | `None`    | [Local Time]       |
///
/// **1. Offset Date-Time**: If all the optional values are used, `Datetime`
/// corresponds to an [Offset Date-Time]. From the TOML v1.0.0 spec:
///
/// > To unambiguously represent a specific instant in time, you may use an
/// > RFC 3339 formatted date-time with offset.
/// >
/// > ```toml
/// > odt1 = 1979-05-27T07:32:00Z
/// > odt2 = 1979-05-27T00:32:00-07:00
/// > odt3 = 1979-05-27T00:32:00.999999-07:00
/// > ```
/// >
/// > For the sake of readability, you may replace the T delimiter between date
/// > and time with a space character (as permitted by RFC 3339 section 5.6).
/// >
/// > ```toml
/// > odt4 = 1979-05-27 07:32:00Z
/// > ```
///
/// **2. Local Date-Time**: If `date` and `time` are given but `offset` is
/// `None`, `Datetime` corresponds to a [Local Date-Time]. From the spec:
///
/// > If you omit the offset from an RFC 3339 formatted date-time, it will
/// > represent the given date-time without any relation to an offset or
/// > timezone. It cannot be converted to an instant in time without additional
/// > information. Conversion to an instant, if required, is implementation-
/// > specific.
/// >
/// > ```toml
/// > ldt1 = 1979-05-27T07:32:00
/// > ldt2 = 1979-05-27T00:32:00.999999
/// > ```
///
/// **3. Local Date**: If only `date` is given, `Datetime` corresponds to a
/// [Local Date]; see the docs for [`Date`].
///
/// **4. Local Time**: If only `time` is given, `Datetime` corresponds to a
/// [Local Time]; see the docs for [`Time`].
///
/// [TOML v1.0.0 spec]: https://toml.io/en/v1.0.0
/// [Offset Date-Time]: https://toml.io/en/v1.0.0#offset-date-time
/// [Local Date-Time]: https://toml.io/en/v1.0.0#local-date-time
/// [Local Date]: https://toml.io/en/v1.0.0#local-date
/// [Local Time]: https://toml.io/en/v1.0.0#local-time
#[derive(PartialEq, Eq, Clone)]
pub struct Datetime {
    /// Optional date.
    /// Required for: *Offset Date-Time*, *Local Date-Time*, *Local Date*.
    pub date: Option<Date>,

    /// Optional time.
    /// Required for: *Offset Date-Time*, *Local Date-Time*, *Local Time*.
    pub time: Option<Time>,

    /// Optional offset.
    /// Required for: *Offset Date-Time*.
    pub offset: Option<Offset>,
}

/// Error returned from parsing a `Datetime` in the `FromStr` implementation.
#[derive(Debug, Clone)]
pub struct DatetimeParseError {
    _private: (),
}

// Currently serde itself doesn't have a datetime type, so we map our `Datetime`
// to a special valid in the serde data model. Namely one with these special
// fields/struct names.
//
// In general the TOML encoder/decoder will catch this and not literally emit
// these strings but rather emit datetimes as they're intended.
pub const FIELD: &str = "$__toml_private_datetime";
pub const NAME: &str = "$__toml_private_Datetime";

/// A parsed TOML date value
///
/// May be part of a [`Datetime`]. Alone, `Date` corresponds to a [Local Date].
/// From the TOML v1.0.0 spec:
///
/// > If you include only the date portion of an RFC 3339 formatted date-time,
/// > it will represent that entire day without any relation to an offset or
/// > timezone.
/// >
/// > ```toml
/// > ld1 = 1979-05-27
/// > ```
///
/// [Local Date]: https://toml.io/en/v1.0.0#local-date
#[derive(PartialEq, Eq, Clone)]
pub struct Date {
    /// Year: four digits
    pub year: u16,
    /// Month: 1 to 12
    pub month: u8,
    /// Day: 1 to {28, 29, 30, 31} (based on month/year)
    pub day: u8,
}

/// A parsed TOML time value
///
/// May be part of a [`Datetime`]. Alone, `Time` corresponds to a [Local Time].
/// From the TOML v1.0.0 spec:
///
/// > If you include only the time portion of an RFC 3339 formatted date-time,
/// > it will represent that time of day without any relation to a specific
/// > day or any offset or timezone.
/// >
/// > ```toml
/// > lt1 = 07:32:00
/// > lt2 = 00:32:00.999999
/// > ```
/// >
/// > Millisecond precision is required. Further precision of fractional
/// > seconds is implementation-specific. If the value contains greater
/// > precision than the implementation can support, the additional precision
/// > must be truncated, not rounded.
///
/// [Local Time]: https://toml.io/en/v1.0.0#local-time
#[derive(PartialEq, Eq, Clone)]
pub struct Time {
    /// Hour: 0 to 23
    pub hour: u8,
    /// Minute: 0 to 59
    pub minute: u8,
    /// Second: 0 to {58, 59, 60} (based on leap second rules)
    pub second: u8,
    /// Nanosecond: 0 to 999_999_999
    pub nanosecond: u32,
}

/// A parsed TOML time offset
///
#[derive(PartialEq, Eq, Clone)]
pub enum Offset {
    /// > A suffix which, when applied to a time, denotes a UTC offset of 00:00;
    /// > often spoken "Zulu" from the ICAO phonetic alphabet representation of
    /// > the letter "Z". --- [RFC 3339 section 2]
    ///
    /// [RFC 3339 section 2]: https://datatracker.ietf.org/doc/html/rfc3339#section-2
    Z,

    /// Offset between local time and UTC
    Custom {
        /// Hours: -12 to +12
        hours: i8,

        /// Minutes: 0 to 59
        minutes: u8,
    },
}

impl fmt::Debug for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref date) = self.date {
            write!(f, "{}", date)?;
        }
        if let Some(ref time) = self.time {
            if self.date.is_some() {
                write!(f, "T")?;
            }
            write!(f, "{}", time)?;
        }
        if let Some(ref offset) = self.offset {
            write!(f, "{}", offset)?;
        }
        Ok(())
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)?;
        if self.nanosecond != 0 {
            let s = format!("{:09}", self.nanosecond);
            write!(f, ".{}", s.trim_end_matches('0'))?;
        }
        Ok(())
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Offset::Z => write!(f, "Z"),
            Offset::Custom { hours, minutes } => write!(f, "{:+03}:{:02}", hours, minutes),
        }
    }
}

impl FromStr for Datetime {
    type Err = DatetimeParseError;

    fn from_str(date: &str) -> Result<Datetime, DatetimeParseError> {
        // Accepted formats:
        //
        // 0000-00-00T00:00:00.00Z
        // 0000-00-00T00:00:00.00
        // 0000-00-00
        // 00:00:00.00
        if date.len() < 3 {
            return Err(DatetimeParseError { _private: () });
        }
        let mut offset_allowed = true;
        let mut chars = date.chars();

        // First up, parse the full date if we can
        let full_date = if chars.clone().nth(2) == Some(':') {
            offset_allowed = false;
            None
        } else {
            let y1 = u16::from(digit(&mut chars)?);
            let y2 = u16::from(digit(&mut chars)?);
            let y3 = u16::from(digit(&mut chars)?);
            let y4 = u16::from(digit(&mut chars)?);

            match chars.next() {
                Some('-') => {}
                _ => return Err(DatetimeParseError { _private: () }),
            }

            let m1 = digit(&mut chars)?;
            let m2 = digit(&mut chars)?;

            match chars.next() {
                Some('-') => {}
                _ => return Err(DatetimeParseError { _private: () }),
            }

            let d1 = digit(&mut chars)?;
            let d2 = digit(&mut chars)?;

            let date = Date {
                year: y1 * 1000 + y2 * 100 + y3 * 10 + y4,
                month: m1 * 10 + m2,
                day: d1 * 10 + d2,
            };

            if date.month < 1 || date.month > 12 {
                return Err(DatetimeParseError { _private: () });
            }
            if date.day < 1 || date.day > 31 {
                return Err(DatetimeParseError { _private: () });
            }

            Some(date)
        };

        // Next parse the "partial-time" if available
        let next = chars.clone().next();
        let partial_time = if full_date.is_some()
            && (next == Some('T') || next == Some('t') || next == Some(' '))
        {
            chars.next();
            true
        } else {
            full_date.is_none()
        };

        let time = if partial_time {
            let h1 = digit(&mut chars)?;
            let h2 = digit(&mut chars)?;
            match chars.next() {
                Some(':') => {}
                _ => return Err(DatetimeParseError { _private: () }),
            }
            let m1 = digit(&mut chars)?;
            let m2 = digit(&mut chars)?;
            match chars.next() {
                Some(':') => {}
                _ => return Err(DatetimeParseError { _private: () }),
            }
            let s1 = digit(&mut chars)?;
            let s2 = digit(&mut chars)?;

            let mut nanosecond = 0;
            if chars.clone().next() == Some('.') {
                chars.next();
                let whole = chars.as_str();

                let mut end = whole.len();
                for (i, byte) in whole.bytes().enumerate() {
                    match byte {
                        b'0'..=b'9' => {
                            if i < 9 {
                                let p = 10_u32.pow(8 - i as u32);
                                nanosecond += p * u32::from(byte - b'0');
                            }
                        }
                        _ => {
                            end = i;
                            break;
                        }
                    }
                }
                if end == 0 {
                    return Err(DatetimeParseError { _private: () });
                }
                chars = whole[end..].chars();
            }

            let time = Time {
                hour: h1 * 10 + h2,
                minute: m1 * 10 + m2,
                second: s1 * 10 + s2,
                nanosecond,
            };

            if time.hour > 24 {
                return Err(DatetimeParseError { _private: () });
            }
            if time.minute > 59 {
                return Err(DatetimeParseError { _private: () });
            }
            if time.second > 59 {
                return Err(DatetimeParseError { _private: () });
            }
            if time.nanosecond > 999_999_999 {
                return Err(DatetimeParseError { _private: () });
            }

            Some(time)
        } else {
            offset_allowed = false;
            None
        };

        // And finally, parse the offset
        let offset = if offset_allowed {
            let next = chars.clone().next();
            if next == Some('Z') || next == Some('z') {
                chars.next();
                Some(Offset::Z)
            } else if next.is_none() {
                None
            } else {
                let sign = match next {
                    Some('+') => 1,
                    Some('-') => -1,
                    _ => return Err(DatetimeParseError { _private: () }),
                };
                chars.next();
                let h1 = digit(&mut chars)? as i8;
                let h2 = digit(&mut chars)? as i8;
                match chars.next() {
                    Some(':') => {}
                    _ => return Err(DatetimeParseError { _private: () }),
                }
                let m1 = digit(&mut chars)?;
                let m2 = digit(&mut chars)?;

                Some(Offset::Custom {
                    hours: sign * (h1 * 10 + h2),
                    minutes: m1 * 10 + m2,
                })
            }
        } else {
            None
        };

        // Return an error if we didn't hit eof, otherwise return our parsed
        // date
        if chars.next().is_some() {
            return Err(DatetimeParseError { _private: () });
        }

        Ok(Datetime {
            date: full_date,
            time,
            offset,
        })
    }
}

fn digit(chars: &mut str::Chars<'_>) -> Result<u8, DatetimeParseError> {
    match chars.next() {
        Some(c) if ('0'..='9').contains(&c) => Ok(c as u8 - b'0'),
        _ => Err(DatetimeParseError { _private: () }),
    }
}

impl ser::Serialize for Datetime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct(NAME, 1)?;
        s.serialize_field(FIELD, &self.to_string())?;
        s.end()
    }
}

impl<'de> de::Deserialize<'de> for Datetime {
    fn deserialize<D>(deserializer: D) -> Result<Datetime, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct DatetimeVisitor;

        impl<'de> de::Visitor<'de> for DatetimeVisitor {
            type Value = Datetime;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a TOML datetime")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Datetime, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let value = visitor.next_key::<DatetimeKey>()?;
                if value.is_none() {
                    return Err(de::Error::custom("datetime key not found"));
                }
                let v: DatetimeFromString = visitor.next_value()?;
                Ok(v.value)
            }
        }

        static FIELDS: [&str; 1] = [FIELD];
        deserializer.deserialize_struct(NAME, &FIELDS, DatetimeVisitor)
    }
}

struct DatetimeKey;

impl<'de> de::Deserialize<'de> for DatetimeKey {
    fn deserialize<D>(deserializer: D) -> Result<DatetimeKey, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a valid datetime field")
            }

            fn visit_str<E>(self, s: &str) -> Result<(), E>
            where
                E: de::Error,
            {
                if s == FIELD {
                    Ok(())
                } else {
                    Err(de::Error::custom("expected field with custom name"))
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)?;
        Ok(DatetimeKey)
    }
}

pub struct DatetimeFromString {
    pub value: Datetime,
}

impl<'de> de::Deserialize<'de> for DatetimeFromString {
    fn deserialize<D>(deserializer: D) -> Result<DatetimeFromString, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DatetimeFromString;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("string containing a datetime")
            }

            fn visit_str<E>(self, s: &str) -> Result<DatetimeFromString, E>
            where
                E: de::Error,
            {
                match s.parse() {
                    Ok(date) => Ok(DatetimeFromString { value: date }),
                    Err(e) => Err(de::Error::custom(e)),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl fmt::Display for DatetimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "failed to parse datetime".fmt(f)
    }
}

impl error::Error for DatetimeParseError {}
