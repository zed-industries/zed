use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A timestamp with a serialized representation in RFC 3339 format.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Timestamp(pub DateTime<Utc>);

impl Timestamp {
    pub fn new(datetime: DateTime<Utc>) -> Self {
        Self(datetime)
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<NaiveDateTime> for Timestamp {
    fn from(value: NaiveDateTime) -> Self {
        Self(value.and_utc())
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rfc3339_string = self.0.to_rfc3339_opts(SecondsFormat::Millis, true);
        serializer.serialize_str(&rfc3339_string)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let datetime = DateTime::parse_from_rfc3339(&value)
            .map_err(serde::de::Error::custom)?
            .to_utc();
        Ok(Self(datetime))
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_timestamp_serialization() {
        let datetime = DateTime::parse_from_rfc3339("2023-12-25T14:30:45.123Z")
            .unwrap()
            .to_utc();
        let timestamp = Timestamp::new(datetime);

        let json = serde_json::to_string(&timestamp).unwrap();
        assert_eq!(json, "\"2023-12-25T14:30:45.123Z\"");
    }

    #[test]
    fn test_timestamp_deserialization() {
        let json = "\"2023-12-25T14:30:45.123Z\"";
        let timestamp: Timestamp = serde_json::from_str(json).unwrap();

        let expected = DateTime::parse_from_rfc3339("2023-12-25T14:30:45.123Z")
            .unwrap()
            .to_utc();

        assert_eq!(timestamp.0, expected);
    }

    #[test]
    fn test_timestamp_roundtrip() {
        let original = DateTime::parse_from_rfc3339("2023-12-25T14:30:45.123Z")
            .unwrap()
            .to_utc();

        let timestamp = Timestamp::new(original);
        let json = serde_json::to_string(&timestamp).unwrap();
        let deserialized: Timestamp = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.0, original);
    }

    #[test]
    fn test_timestamp_from_datetime_utc() {
        let datetime = DateTime::parse_from_rfc3339("2023-12-25T14:30:45.123Z")
            .unwrap()
            .to_utc();

        let timestamp = Timestamp::from(datetime);
        assert_eq!(timestamp.0, datetime);
    }

    #[test]
    fn test_timestamp_from_naive_datetime() {
        let naive_dt = NaiveDate::from_ymd_opt(2023, 12, 25)
            .unwrap()
            .and_hms_milli_opt(14, 30, 45, 123)
            .unwrap();

        let timestamp = Timestamp::from(naive_dt);
        let expected = naive_dt.and_utc();

        assert_eq!(timestamp.0, expected);
    }

    #[test]
    fn test_timestamp_serialization_with_microseconds() {
        // Test that microseconds are truncated to milliseconds
        let datetime = NaiveDate::from_ymd_opt(2023, 12, 25)
            .unwrap()
            .and_hms_micro_opt(14, 30, 45, 123456)
            .unwrap()
            .and_utc();

        let timestamp = Timestamp::new(datetime);
        let json = serde_json::to_string(&timestamp).unwrap();

        // Should be truncated to milliseconds
        assert_eq!(json, "\"2023-12-25T14:30:45.123Z\"");
    }

    #[test]
    fn test_timestamp_deserialization_without_milliseconds() {
        let json = "\"2023-12-25T14:30:45Z\"";
        let timestamp: Timestamp = serde_json::from_str(json).unwrap();

        let expected = NaiveDate::from_ymd_opt(2023, 12, 25)
            .unwrap()
            .and_hms_opt(14, 30, 45)
            .unwrap()
            .and_utc();

        assert_eq!(timestamp.0, expected);
    }

    #[test]
    fn test_timestamp_deserialization_with_timezone() {
        let json = "\"2023-12-25T14:30:45.123+05:30\"";
        let timestamp: Timestamp = serde_json::from_str(json).unwrap();

        // Should be converted to UTC
        let expected = NaiveDate::from_ymd_opt(2023, 12, 25)
            .unwrap()
            .and_hms_milli_opt(9, 0, 45, 123) // 14:30:45 + 5:30 = 20:00:45, but we want UTC so subtract 5:30
            .unwrap()
            .and_utc();

        assert_eq!(timestamp.0, expected);
    }

    #[test]
    fn test_timestamp_deserialization_with_invalid_format() {
        let json = "\"invalid-date\"";
        let result: Result<Timestamp, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
