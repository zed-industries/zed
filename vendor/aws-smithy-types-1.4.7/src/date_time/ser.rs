/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use super::*;
use serde::ser::SerializeTuple;

impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            match self.fmt(Format::DateTime) {
                Ok(val) => serializer.serialize_str(&val),
                Err(e) => Err(serde::ser::Error::custom(e)),
            }
        } else {
            let mut tup_ser = serializer.serialize_tuple(2)?;
            tup_ser.serialize_element(&self.seconds)?;
            tup_ser.serialize_element(&self.subsecond_nanos)?;
            tup_ser.end()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// check for human redable format
    #[test]
    fn ser_human_readable_datetime() {
        use serde::{Deserialize, Serialize};

        let datetime = DateTime::from_secs(1576540098);
        #[derive(Serialize, Deserialize, PartialEq)]
        struct Test {
            datetime: DateTime,
        }
        let datetime_json = r#"{"datetime":"2019-12-16T23:48:18Z"}"#;
        assert!(serde_json::to_string(&Test { datetime }).ok() == Some(datetime_json.to_string()));
    }

    /// check for non-human redable format
    #[test]
    fn ser_not_human_readable_datetime() {
        {
            let cbor = ciborium::value::Value::Array(vec![
                ciborium::value::Value::Integer(1576540098i64.into()),
                ciborium::value::Value::Integer(0u32.into()),
            ]);
            let mut buf = vec![];
            let mut buf2 = vec![];
            let _ = ciborium::ser::into_writer(&cbor, &mut buf);
            let _ = ciborium::ser::into_writer(&cbor, &mut buf2);
            assert_eq!(buf, buf2);
        };

        {
            let cbor = ciborium::value::Value::Array(vec![
                ciborium::value::Value::Integer(0i64.into()),
                ciborium::value::Value::Integer(0u32.into()),
            ]);
            let mut buf = vec![];
            let mut buf2 = vec![];
            let _ = ciborium::ser::into_writer(&cbor, &mut buf);
            let _ = ciborium::ser::into_writer(&cbor, &mut buf2);
            assert_eq!(buf, buf2);
        };

        {
            let cbor = ciborium::value::Value::Array(vec![
                ciborium::value::Value::Integer(i64::MAX.into()),
                ciborium::value::Value::Integer(u32::MAX.into()),
            ]);
            let mut buf = vec![];
            let mut buf2 = vec![];
            let _ = ciborium::ser::into_writer(&cbor, &mut buf);
            let _ = ciborium::ser::into_writer(&cbor, &mut buf2);
            assert_eq!(buf, buf2);
        };
    }
}
