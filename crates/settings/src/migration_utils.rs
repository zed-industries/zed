use convert_case::{Case, Casing};
use serde_json::Value;

fn snake_case_recursively(obj: &mut serde_json::Map<String, Value>) {
    let keys: Vec<String> = obj.keys().cloned().collect();
    for key in keys {
        let new_key = key.to_case(Case::Snake);
        if new_key != key {
            if let Some(value) = obj.remove(&key) {
                obj.insert(new_key, value);
            }
        }
    }
    for value in obj.values_mut() {
        if let Value::String(s) = value {
            *s = s.to_case(Case::Snake);
        } else if let Value::Object(inner_obj) = value {
            snake_case_recursively(inner_obj);
        }
    }
}
