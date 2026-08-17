/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use super::*;
use serde::de::{Error, Visitor};
use serde::Deserialize;

struct DateTimeVisitor;

struct NonHumanReadableDateTimeVisitor;

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = DateTime;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("expected RFC-3339 Date Time")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match DateTime::from_str(v, Format::DateTime) {
            Ok(e) => Ok(e),
            Err(e) => Err(Error::custom(e)),
        }
    }
}

impl<'de> Visitor<'de> for NonHumanReadableDateTimeVisitor {
    type Value = DateTime;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("DateTime type expects a tuple of i64 and u32 when deserializing from non human readable format like CBOR or AVRO, i.e. (i64, u32)")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        match seq.size_hint() {
            Some(2) | None => match (seq.next_element()?, seq.next_element()?) {
                (Some(seconds), Some(subsecond_nanos)) => Ok(DateTime {
                    seconds,
                    subsecond_nanos,
                }),
                _ => return Err(Error::custom("datatype mismatch")),
            },
            _ => Err(Error::custom("Size mismatch")),
        }
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(DateTimeVisitor)
        } else {
            deserializer.deserialize_tuple(2, NonHumanReadableDateTimeVisitor)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// check for human redable format
    #[test]
    fn de_human_readable_datetime() {
        use serde::{Deserialize, Serialize};

        let datetime = DateTime::from_secs(1576540098);
        #[derive(Serialize, Deserialize, PartialEq)]
        struct Test {
            datetime: DateTime,
        }
        let datetime_json = r#"{"datetime":"2019-12-16T23:48:18Z"}"#;
        let test = serde_json::from_str::<Test>(&datetime_json).ok();
        assert!(test == Some(Test { datetime }));
    }

    /// check for non-human redable format
    #[test]
    fn de_not_human_readable_datetime() {
        {
            let cbor = ciborium::value::Value::Array(vec![
                ciborium::value::Value::Integer(1576540098i64.into()),
                ciborium::value::Value::Integer(0u32.into()),
            ]);
            let mut buf = vec![];
            let _ = ciborium::ser::into_writer(&cbor, &mut buf);
            let cbor_dt: DateTime = ciborium::de::from_reader(std::io::Cursor::new(buf)).unwrap();
            assert_eq!(
                cbor_dt,
                DateTime {
                    seconds: 1576540098i64,
                    subsecond_nanos: 0
                }
            );
        };

        {
            let cbor = ciborium::value::Value::Array(vec![
                ciborium::value::Value::Integer(0i64.into()),
                ciborium::value::Value::Integer(0u32.into()),
            ]);
            let mut buf = vec![];
            let _ = ciborium::ser::into_writer(&cbor, &mut buf);
            let cbor_dt: DateTime = ciborium::de::from_reader(std::io::Cursor::new(buf)).unwrap();
            assert_eq!(
                cbor_dt,
                DateTime {
                    seconds: 0,
                    subsecond_nanos: 0
                }
            );
        };

        {
            let cbor = ciborium::value::Value::Array(vec![
                ciborium::value::Value::Integer(i64::MAX.into()),
                ciborium::value::Value::Integer(u32::MAX.into()),
            ]);
            let mut buf = vec![];
            let _ = ciborium::ser::into_writer(&cbor, &mut buf);
            let cbor_dt: DateTime = ciborium::de::from_reader(std::io::Cursor::new(buf)).unwrap();
            assert_eq!(
                cbor_dt,
                DateTime {
                    seconds: i64::MAX,
                    subsecond_nanos: u32::MAX
                }
            );
        };
    }
}
