/// When using prost_types::Struct, we need a custom serializer to convert
/// it to a serde_json::Value.
use prost_types;
use serde::ser::{SerializeMap, Serializer};
use serde_json::Value;

pub fn serialize_prost_struct_to_json_object<S>(
    wrapper: &Option<prost_types::Struct>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let wrapper = match wrapper {
        Some(wrapper) => wrapper,
        None => return serializer.serialize_none(),
    };

    let mut map = serializer.serialize_map(Some(wrapper.fields.len()))?;
    for (key, value) in &wrapper.fields {
        map.serialize_entry(key, &value_to_json(value))?;
    }
    map.end()
}

pub fn from_prost_struct(wrapper: Option<prost_types::Struct>) -> Value {
    match wrapper {
        Some(wrapper) => Value::Object(
            wrapper
                .fields
                .iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect(),
        ),
        None => Value::Null,
    }
}

pub fn value_to_json(value: &prost_types::Value) -> Value {
    match value.kind {
        Some(prost_types::value::Kind::NullValue(_)) => Value::Null,
        Some(prost_types::value::Kind::NumberValue(v)) => {
            let number = serde_json::Number::from_f64(v);

            number.map_or(Value::Null, Value::Number)
        }
        Some(prost_types::value::Kind::StringValue(ref v)) => Value::String(v.clone()),
        Some(prost_types::value::Kind::BoolValue(v)) => Value::Bool(v),
        Some(prost_types::value::Kind::StructValue(ref v)) => Value::Object(
            v.fields
                .iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect(),
        ),
        Some(prost_types::value::Kind::ListValue(ref v)) => {
            Value::Array(v.values.iter().map(value_to_json).collect())
        }
        None => Value::Null,
    }
}
