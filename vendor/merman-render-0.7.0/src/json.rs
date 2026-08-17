use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub(crate) fn from_value_ref<T: DeserializeOwned>(value: &Value) -> Result<T, serde_json::Error> {
    T::deserialize(value)
}

pub(crate) fn clone_value_nonrecursive(value: &Value) -> Value {
    let mut cloned: HashMap<*const Value, Value> = HashMap::new();
    let mut stack = vec![(value, false)];

    while let Some((current, visited)) = stack.pop() {
        let current_ptr = std::ptr::from_ref(current);
        if visited {
            let value = match current {
                Value::Null => Value::Null,
                Value::Bool(v) => Value::Bool(*v),
                Value::Number(v) => Value::Number(v.clone()),
                Value::String(v) => Value::String(v.clone()),
                Value::Array(items) => Value::Array(
                    items
                        .iter()
                        .filter_map(|item| cloned.remove(&std::ptr::from_ref(item)))
                        .collect(),
                ),
                Value::Object(entries) => {
                    let mut out = Map::new();
                    for (key, child) in entries {
                        if let Some(value) = cloned.remove(&std::ptr::from_ref(child)) {
                            out.insert(key.clone(), value);
                        }
                    }
                    Value::Object(out)
                }
            };
            cloned.insert(current_ptr, value);
        } else {
            stack.push((current, true));
            match current {
                Value::Array(items) => {
                    for item in items.iter().rev() {
                        stack.push((item, false));
                    }
                }
                Value::Object(entries) => {
                    for child in entries.values().rev() {
                        stack.push((child, false));
                    }
                }
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
            }
        }
    }

    cloned
        .remove(&std::ptr::from_ref(value))
        .unwrap_or(Value::Null)
}
