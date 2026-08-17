pub use crate::pb::google::protobuf::value::Kind;

use serde::{
    de::{self, MapAccess, SeqAccess},
    ser, Deserialize, Deserializer, Serialize, Serializer,
};

macro_rules! from {
    ($($typ: ty [$id:ident] => {$($from_type:ty => $exp:expr),+ $(,)?})+) => {
        $($(
            impl From<$from_type> for $typ {
                #[allow(unused_variables)]
                fn from($id: $from_type) -> Self {
                    $exp
                }
            }
        )+)+
    }
}

from! {
    crate::Value[value] => {
        &'static str => Kind::from(value).into(),
        () => Kind::NullValue(0).into(),
        Kind => Self { kind: Some(value) },
        Option<Kind> => Self { kind: value },
        String => Kind::from(value).into(),
        Vec<Self> => Kind::from(value).into(),
        bool => Kind::from(value).into(),
        crate::ListValue => Kind::from(value).into(),
        crate::Struct => Kind::from(value).into(),
        f64 => Kind::from(value).into(),
        std::collections::HashMap<String, Self> => Kind::from(value).into(),
    }

    Kind[value] => {
        &'static str => Self::StringValue(value.into()),
        () => Self::NullValue(0),
        String => Self::StringValue(value),
        Vec<crate::Value> => Self::ListValue(value.into()),
        bool => Self::BoolValue(value),
        crate::ListValue => Self::ListValue(value),
        crate::Struct => Self::StructValue(value),
        f64 => Self::NumberValue(value),
        std::collections::HashMap<String, crate::Value> => Self::StructValue(value.into()),
    }
}

impl<const N: usize> From<[Self; N]> for crate::Value {
    fn from(value: [Self; N]) -> Self {
        crate::ListValue::from(value).into()
    }
}

impl Serialize for crate::Value {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.kind.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for crate::Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            kind: <_>::deserialize(deserializer)?,
        })
    }
}

impl Serialize for Kind {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NullValue(_) => ().serialize(ser),
            Self::StringValue(value) => value.serialize(ser),
            Self::BoolValue(value) => value.serialize(ser),
            Self::StructValue(value) => value.serialize(ser),
            Self::ListValue(list) => list.serialize(ser),
            Self::NumberValue(value) => {
                // Kind does not allow NaN's or Infinity as they are
                // indistinguishable from strings.
                if value.is_nan() {
                    Err(ser::Error::custom(
                        "Cannot serialize NaN as google.protobuf.Value.number_value",
                    ))
                } else if value.is_infinite() {
                    Err(ser::Error::custom(
                        "Cannot serialize infinity as google.protobuf.Value.number_value",
                    ))
                } else {
                    value.serialize(ser)
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(KindVisitor)
    }
}

struct KindVisitor;

impl<'de> serde::de::Visitor<'de> for KindVisitor {
    type Value = Kind;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("google.protobuf.Value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::BoolValue(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::NumberValue(v.into()))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::NumberValue(v.into()))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::NumberValue(v.into()))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v > -(1 << f64::MANTISSA_DIGITS) && v < 1 << f64::MANTISSA_DIGITS {
            return Ok(Kind::NumberValue(v as f64));
        }

        Err(de::Error::custom(
            "out of range integral type conversion attempted",
        ))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v.try_into().map_err(de::Error::custom)?)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::NumberValue(v.into()))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::NumberValue(v.into()))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::NumberValue(v.into()))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v < 1 << f64::MANTISSA_DIGITS {
            return Ok(Kind::NumberValue(v as f64));
        }

        Err(de::Error::custom(
            "out of range integral type conversion attempted",
        ))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(v.try_into().map_err(de::Error::custom)?)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::NumberValue(v.into()))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::NumberValue(v))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::StringValue(v.into()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::StringValue(v.into()))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::StringValue(v.into()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::StringValue(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::NullValue(0))
    }

    fn visit_some<D>(self, de: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(de)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Kind::NullValue(0))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::new();

        while let Some(value) = seq.next_element()? {
            list.push(value);
        }

        Ok(Kind::ListValue(list.into()))
    }

    fn visit_map<A>(self, mut map_access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map = std::collections::HashMap::new();

        while let Some((key, value)) = map_access.next_entry()? {
            map.insert(key, value);
        }

        Ok(Kind::StructValue(map.into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;

    #[test]
    fn boolean() {
        assert_eq!(
            serde_json::to_value(Value::from(false)).unwrap(),
            serde_json::json!(false)
        );
        assert_eq!(
            serde_json::to_value(Value::from(true)).unwrap(),
            serde_json::json!(true)
        );
    }

    #[test]
    fn number() {
        assert_eq!(
            serde_json::to_value(Value::from(5.0)).unwrap(),
            serde_json::json!(5.0)
        );
    }

    #[test]
    fn string() {
        assert_eq!(
            serde_json::to_value(Value::from("string")).unwrap(),
            serde_json::json!("string")
        );
    }

    #[test]
    fn float_special_cases() {
        assert!(serde_json::to_value(Value::from(f64::NAN)).is_err());
        assert!(serde_json::to_value(Value::from(f64::INFINITY)).is_err());
        assert!(serde_json::to_value(Value::from(f64::NEG_INFINITY)).is_err());
    }

    #[test]
    fn parse_max_safe_integer() {
        let max_safe_integer: i64 = 9007199254740991;
        let json = serde_json::json!(max_safe_integer);
        let vec = serde_json::to_vec(&json).unwrap();
        let pb = serde_json::from_slice::<Value>(&vec).unwrap();
        assert_eq!(
            serde_json::to_value(pb).unwrap(),
            serde_json::json!(max_safe_integer as f64)
        );
    }

    #[test]
    fn parse_min_safe_integer() {
        let min_safe_integer: i64 = -9007199254740991;
        let json = serde_json::json!(min_safe_integer);
        let vec = serde_json::to_vec(&json).unwrap();
        let pb = serde_json::from_slice::<Value>(&vec).unwrap();
        assert_eq!(
            serde_json::to_value(pb).unwrap(),
            serde_json::json!(min_safe_integer as f64)
        );
    }
}
