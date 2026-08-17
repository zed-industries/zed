use super::safe_string::SafeString;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(String::from(v))
    }
    type Value = String;
}

impl Serialize for SafeString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.deref())
    }
}

impl<'de> Deserialize<'de> for SafeString {
    fn deserialize<D>(deserializer: D) -> Result<SafeString, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_string(StringVisitor)
            .map(|parsed_value| SafeString::from_string(parsed_value))
    }
}

mod test {
    use super::SafeString;
    use serde::{Deserialize, Serialize};

    #[test]
    fn safe_string_serialization() {
        let s = SafeString::from_string(String::from("blabla"));

        match serde_json::to_string(&s) {
            Ok(json) => assert_eq!("\"blabla\"", json),
            Err(_) => panic!("Serialization failed, somehow"),
        }
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    pub struct TestStruct {
        password: SafeString,
    }

    #[test]
    fn safe_string_within_struct_serialization() {
        let ts = TestStruct {
            password: SafeString::from_string(String::from("blabla")),
        };

        match serde_json::to_string(&ts) {
            Ok(json) => assert_eq!("{\"password\":\"blabla\"}", json),
            Err(_) => panic!("Serialization failed, somehow"),
        }
    }

    #[test]
    fn safe_string_deserialization() {
        let s = "\"blabla\"";

        let res: Result<SafeString, serde_json::Error> = serde_json::from_str(s);

        match res {
            Ok(ss) => assert_eq!(ss, SafeString::from_string(String::from("blabla"))),
            Err(_) => panic!("Deserialization failed"),
        }
    }

    #[test]
    fn safe_string_within_struct_deserialization() {
        let json = "{\"password\":\"blabla\"}";
        let res: Result<TestStruct, serde_json::Error> = serde_json::from_str(json);
        match res {
            Ok(ts) => assert_eq!(
                ts,
                TestStruct {
                    password: SafeString::from_string(String::from("blabla"))
                }
            ),
            Err(_) => panic!("Deserialization failed"),
        }
    }
}
