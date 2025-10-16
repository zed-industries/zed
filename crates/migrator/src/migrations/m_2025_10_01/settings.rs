use crate::patterns::migrate_language_setting;
use anyhow::Result;
use serde_json::Value;

pub fn flatten_code_actions_formatters(value: &mut Value) -> Result<()> {
    migrate_language_setting(value, |value, _path| {
        let Some(obj) = value.as_object_mut() else {
            return Ok(());
        };
        for key in ["formatter", "format_on_save"] {
            let Some(formatter) = obj.get_mut(key) else {
                continue;
            };
            let new_formatter = match formatter {
                Value::Array(arr) => {
                    let mut new_arr = Vec::new();
                    let mut found_code_actions = false;
                    for item in arr {
                        let Some(obj) = item.as_object() else {
                            new_arr.push(item.clone());
                            continue;
                        };
                        let code_actions_obj = obj
                            .get("code_actions")
                            .and_then(|code_actions| code_actions.as_object());
                        let Some(code_actions) = code_actions_obj else {
                            new_arr.push(item.clone());
                            continue;
                        };
                        found_code_actions = true;
                        for (name, enabled) in code_actions {
                            if !enabled.as_bool().unwrap_or(true) {
                                continue;
                            }
                            new_arr.push(serde_json::json!({
                                "code_action": name
                            }));
                        }
                    }
                    if !found_code_actions {
                        continue;
                    }
                    Value::Array(new_arr)
                }
                Value::Object(obj) => {
                    let mut new_arr = Vec::new();
                    let code_actions_obj = obj
                        .get("code_actions")
                        .and_then(|code_actions| code_actions.as_object());
                    let Some(code_actions) = code_actions_obj else {
                        continue;
                    };
                    for (name, enabled) in code_actions {
                        if !enabled.as_bool().unwrap_or(true) {
                            continue;
                        }
                        new_arr.push(serde_json::json!({
                            "code_action": name
                        }));
                    }
                    if new_arr.len() == 1 {
                        new_arr.pop().unwrap()
                    } else {
                        Value::Array(new_arr)
                    }
                }
                _ => continue,
            };

            obj.insert(key.to_string(), new_formatter);
        }
        return Ok(());
    })
}
