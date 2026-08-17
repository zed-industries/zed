use crate::Timestamp;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::de::Visitor;
use serde::Serialize;

impl TryFrom<Timestamp> for DateTime<Utc> {
    type Error = &'static str;
    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        let Timestamp { seconds, nanos } = value;

        let dt = NaiveDateTime::from_timestamp_opt(
            seconds,
            nanos
                .try_into()
                .map_err(|_| "out of range integral type conversion attempted")?,
        )
        .ok_or("invalid or out-of-range datetime")?;
        Ok(Utc.from_utc_datetime(&dt))
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Self {
            seconds: value.timestamp(),
            nanos: value.timestamp_subsec_nanos() as i32,
        }
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let t: DateTime<Utc> = self.clone().try_into().map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(t.to_rfc3339().as_str())
    }
}

struct TimestampVisitor;

impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a date string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let d = DateTime::parse_from_rfc3339(s).map_err(serde::de::Error::custom)?;
        let d: DateTime<Utc> = d.into();
        Ok(d.into())
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TimestampVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};
    use serde::de::value::{BorrowedStrDeserializer, Error};
    use serde::Deserialize;

    #[test]
    fn test_date() {
        let datetime = FixedOffset::east_opt(5 * 3600)
            .expect("time zone offset should be valid")
            .with_ymd_and_hms(2016, 11, 8, 21, 7, 9)
            .unwrap();
        let encoded = datetime.to_rfc3339();
        assert_eq!(&encoded, "2016-11-08T21:07:09+05:00");

        let utc: DateTime<Utc> = datetime.into();
        let utc_encoded = utc.to_rfc3339();
        assert_eq!(&utc_encoded, "2016-11-08T16:07:09+00:00");

        let deserializer = BorrowedStrDeserializer::<'_, Error>::new(&encoded);
        let a: Timestamp = Timestamp::deserialize(deserializer).unwrap();
        assert_eq!(a.seconds, utc.timestamp());
        assert_eq!(a.nanos, utc.timestamp_subsec_nanos() as i32);

        let encoded = serde_json::to_string(&a).unwrap();
        assert_eq!(encoded, format!("\"{}\"", utc_encoded));
    }
}
