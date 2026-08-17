use prost::bytes::Bytes;

macro_rules! ser_scalar_value {
    ($typ: ty) => {
        impl serde::Serialize for $typ {
            fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.value.serialize(ser)
            }
        }
    };
}
macro_rules! deser_scalar_value {
    ($typ: ty) => {
        impl<'de> serde::Deserialize<'de> for $typ {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = serde::Deserialize::deserialize(deserializer)?;
                Ok(Self { value })
            }
        }
    };
}
macro_rules! ser_bytes_value {
    ($typ: ty) => {
        impl serde::Serialize for $typ {
            fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use pbjson::private::base64::engine::Engine;
                let value =
                    pbjson::private::base64::engine::general_purpose::STANDARD.encode(&self.value);
                value.serialize(ser)
            }
        }
    };
}
macro_rules! deser_bytes_value {
    ($typ: ty) => {
        impl<'de> serde::Deserialize<'de> for $typ {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = pbjson::private::BytesDeserialize::deserialize(deserializer)?.0;
                Ok(Self { value })
            }
        }
    };
}
macro_rules! ser_long_value {
    ($typ: ty) => {
        impl serde::Serialize for $typ {
            fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let value = self.value.to_string();
                value.serialize(ser)
            }
        }
    };
}
macro_rules! deser_number_value {
    ($typ: ty) => {
        impl<'de> serde::Deserialize<'de> for $typ {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = pbjson::private::NumberDeserialize::deserialize(deserializer)?.0;
                Ok(Self { value })
            }
        }
    };
}

macro_rules! convert_scalar_value {
    ($scalar: ty, $typ: ty) => {
        impl From<$scalar> for $typ {
            fn from(value: $scalar) -> Self {
                Self { value }
            }
        }
    };
}

ser_scalar_value!(crate::BoolValue);
deser_scalar_value!(crate::BoolValue);
ser_bytes_value!(crate::BytesValue);
deser_bytes_value!(crate::BytesValue);
ser_scalar_value!(crate::DoubleValue);
deser_number_value!(crate::DoubleValue);
ser_scalar_value!(crate::FloatValue);
deser_number_value!(crate::FloatValue);
ser_scalar_value!(crate::Int32Value);
deser_number_value!(crate::Int32Value);
ser_long_value!(crate::Int64Value);
deser_number_value!(crate::Int64Value);
ser_scalar_value!(crate::StringValue);
deser_scalar_value!(crate::StringValue);
ser_scalar_value!(crate::UInt32Value);
deser_number_value!(crate::UInt32Value);
ser_long_value!(crate::UInt64Value);
deser_number_value!(crate::UInt64Value);

convert_scalar_value!(bool, crate::BoolValue);
convert_scalar_value!(Bytes, crate::BytesValue);
convert_scalar_value!(f64, crate::DoubleValue);
convert_scalar_value!(f32, crate::FloatValue);
convert_scalar_value!(i32, crate::Int32Value);
convert_scalar_value!(i64, crate::Int64Value);
convert_scalar_value!(String, crate::StringValue);
convert_scalar_value!(u32, crate::UInt32Value);
convert_scalar_value!(u64, crate::UInt64Value);
