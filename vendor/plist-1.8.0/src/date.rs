use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};

/// A UTC timestamp used for serialization to and from the plist date type.
///
/// Note that while this type implements `Serialize` and `Deserialize` it will behave strangely if
/// used with serializers from outside this crate.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Date {
    inner: SystemTime,
}

/// An error indicating that a string was not a valid XML plist date.
#[derive(Debug)]
#[non_exhaustive]
pub struct InvalidXmlDate;

pub(crate) struct InfiniteOrNanDate;

impl Date {
    /// The unix timestamp of the plist epoch.
    const PLIST_EPOCH_UNIX_TIMESTAMP: Duration = Duration::from_secs(978_307_200);

    /// Converts an XML plist date string to a `Date`.
    pub fn from_xml_format(date: &str) -> Result<Self, InvalidXmlDate> {
        let offset: OffsetDateTime = OffsetDateTime::parse(date, &Rfc3339)
            .map_err(|_| InvalidXmlDate)?
            .to_offset(UtcOffset::UTC);
        Ok(Date {
            inner: offset.into(),
        })
    }

    /// Converts the `Date` to an XML plist date string.
    pub fn to_xml_format(&self) -> String {
        let datetime: OffsetDateTime = self.inner.into();
        datetime.format(&Rfc3339).unwrap()
    }

    pub(crate) fn from_seconds_since_plist_epoch(
        timestamp: f64,
    ) -> Result<Date, InfiniteOrNanDate> {
        // `timestamp` is the number of seconds since the plist epoch of 1/1/2001 00:00:00.
        let plist_epoch = UNIX_EPOCH + Date::PLIST_EPOCH_UNIX_TIMESTAMP;

        if !timestamp.is_finite() {
            return Err(InfiniteOrNanDate);
        }

        let is_negative = timestamp < 0.0;
        let timestamp = timestamp.abs();
        let seconds = timestamp.floor() as u64;
        let subsec_nanos = (timestamp.fract() * 1e9) as u32;

        let dur_since_plist_epoch = Duration::new(seconds, subsec_nanos);

        let inner = if is_negative {
            plist_epoch.checked_sub(dur_since_plist_epoch)
        } else {
            plist_epoch.checked_add(dur_since_plist_epoch)
        };

        let inner = inner.ok_or(InfiniteOrNanDate)?;

        Ok(Date { inner })
    }

    pub(crate) fn as_seconds_since_plist_epoch(&self) -> f64 {
        // needed until #![feature(duration_float)] is stabilized
        fn as_secs_f64(d: Duration) -> f64 {
            const NANOS_PER_SEC: f64 = 1_000_000_000.00;
            (d.as_secs() as f64) + f64::from(d.subsec_nanos()) / NANOS_PER_SEC
        }

        let plist_epoch = UNIX_EPOCH + Date::PLIST_EPOCH_UNIX_TIMESTAMP;
        match self.inner.duration_since(plist_epoch) {
            Ok(dur_since_plist_epoch) => as_secs_f64(dur_since_plist_epoch),
            Err(err) => -as_secs_f64(err.duration()),
        }
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_xml_format())
    }
}

impl From<SystemTime> for Date {
    fn from(date: SystemTime) -> Self {
        Date { inner: date }
    }
}

impl From<Date> for SystemTime {
    fn from(val: Date) -> Self {
        val.inner
    }
}

impl fmt::Display for InvalidXmlDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("String was not a valid XML plist date")
    }
}

impl std::error::Error for InvalidXmlDate {}

#[cfg(feature = "serde")]
pub mod serde_impls {
    use serde::{
        de::{Deserialize, Deserializer, Error, Unexpected, Visitor},
        ser::{Serialize, Serializer},
    };
    use std::fmt;

    use crate::Date;

    pub const DATE_NEWTYPE_STRUCT_NAME: &str = "PLIST-DATE";

    impl Serialize for Date {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let date_str = self.to_xml_format();
            serializer.serialize_newtype_struct(DATE_NEWTYPE_STRUCT_NAME, &date_str)
        }
    }

    struct DateNewtypeVisitor;

    impl<'de> Visitor<'de> for DateNewtypeVisitor {
        type Value = Date;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a plist date newtype")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            DateStrVisitor.visit_str(v)
        }

        fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(DateStrVisitor)
        }
    }

    struct DateStrVisitor;

    impl Visitor<'_> for DateStrVisitor {
        type Value = Date;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a plist date string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Date::from_xml_format(v).map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    impl<'de> Deserialize<'de> for Date {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_newtype_struct(DATE_NEWTYPE_STRUCT_NAME, DateNewtypeVisitor)
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn date_roundtrip() {
        let date_str = "1981-05-16T11:32:06Z";

        let date = Date::from_xml_format(date_str).expect("should parse");

        let generated_str = date.to_xml_format();

        assert_eq!(date_str, generated_str);
    }

    #[test]
    fn far_past_date() {
        let date_str = "1920-01-01T00:00:00Z";
        Date::from_xml_format(date_str).expect("should parse");
    }
}
