use convert_case::{Case, Casing};
use serde_json::Value;

use crate::{
    keymap_file::KeymapAction,
    migration_static::{CONTEXT_REPLACE, STRING_REPLACE, TRANSFORM_ARRAY, UNWRAP_OBJECTS},
};

// Returns new context if migration is needed, otherwise None
pub fn get_migrated_context(existing_context: &String) -> Option<String> {
    for (old_context, new_context) in CONTEXT_REPLACE.iter() {
        if existing_context.as_str().contains(old_context) {
            return Some(existing_context.replace(old_context, new_context));
        }
    }
    None
}

// Returns new action if migration is needed, otherwise None
pub fn get_migrated_action(existing_action: &KeymapAction) -> Option<KeymapAction> {
    match &existing_action.0 {
        Value::Array(items) => {
            if items.len() != 2 {
                return None;
            }
            let Some(Value::String(old_action_name)) = items.get(0) else {
                return None;
            };
            match items.get(1) {
                Some(Value::String(value)) => {
                    let new_action =
                        TRANSFORM_ARRAY.get(&(old_action_name.as_str(), value.as_str()))?;
                    Some(KeymapAction(Value::String(new_action.to_string())))
                }
                Some(Value::Object(value)) => {
                    match UNWRAP_OBJECTS
                        .get(old_action_name.as_str())
                        .and_then(|fields| {
                            fields.iter().find_map(|(field, new_action_name)| {
                                value
                                    .get(*field)
                                    .map(|inner_val| (inner_val, new_action_name))
                            })
                        }) {
                        Some((inner_value, new_action)) => {
                            let new_value = match inner_value {
                                Value::Object(ref obj) => {
                                    let mut new_obj = obj.clone();
                                    snake_case_recursively(&mut new_obj);
                                    Value::Object(new_obj)
                                }
                                _ => inner_value.clone(),
                            };

                            Some(KeymapAction(Value::Array(vec![
                                Value::String(new_action.to_string()),
                                new_value,
                            ])))
                        }
                        None => {
                            let mut new_value = value.clone();
                            snake_case_recursively(&mut new_value);

                            if &new_value == value {
                                return None;
                            }

                            Some(KeymapAction(Value::Array(vec![
                                Value::String(old_action_name.to_string()),
                                Value::Object(new_value),
                            ])))
                        }
                    }
                }
                _ => None,
            }
        }
        Value::String(value) => {
            let new_value = STRING_REPLACE.get(value.as_str())?;
            Some(KeymapAction(Value::String(new_value.to_string())))
        }
        _ => None,
    }
}

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

#[cfg(test)]
mod tests {
    use crate::{
        keymap_file::KeymapAction,
        migration_utils::{get_migrated_action, snake_case_recursively},
    };
    use serde_json::{json, Value};

    #[test]
    fn test_array_to_string_migration() {
        let input = KeymapAction(json!(["workspace::ActivatePaneInDirection", "Up"]));
        let result = get_migrated_action(&input);
        assert_eq!(
            result.unwrap().0,
            Value::String("workspace::ActivatePaneUp".to_string())
        );
    }

    #[test]
    fn test_unwrap_object_migration() {
        let input = KeymapAction(json!([
            "editor::FoldAtLevel",
            {"level": 2}
        ]));
        let result = get_migrated_action(&input);
        assert_eq!(result.unwrap().0, json!(["editor::FoldAtLevel", 2]));

        let input = KeymapAction(json!([
            "vim::PushOperator",
            {"Object": {"around": false}}
        ]));
        let result = get_migrated_action(&input);
        assert_eq!(
            result.unwrap().0,
            json!(["vim::PushObject", {"around": false}])
        );
    }

    #[test]
    fn test_snake_case_conversion() {
        let mut map = serde_json::Map::new();
        map.insert("camelCase".to_string(), json!("someValue"));
        map.insert(
            "nestedObject".to_string(),
            json!({
                "innerCamel": "value"
            }),
        );

        snake_case_recursively(&mut map);

        assert!(map.contains_key("camel_case"));
        assert!(!map.contains_key("camelCase"));

        if let Some(Value::Object(nested)) = map.get("nested_object") {
            assert!(nested.contains_key("inner_camel"));
            assert!(!nested.contains_key("innerCamel"));
        } else {
            panic!("Expected nested object");
        }
    }
}
