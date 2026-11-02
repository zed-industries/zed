use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

const CLOSING_KEY: &str = "window_closing";
const KEY_MAPPINGS: &[(&str, &str)] = &[
    ("when_closing_with_no_tabs", "on_close_with_no_tabs"),
    ("on_last_window_closed", "on_last_window_closed"),
];

pub fn nest_window_closing_settings(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(object: &mut serde_json::Map<String, Value>) -> Result<()> {
    if !KEY_MAPPINGS
        .iter()
        .any(|(key, _)| object.contains_key(*key))
    {
        return Ok(());
    }

    // Do not discard old values when the destination cannot hold an object.
    match object.get(CLOSING_KEY) {
        Some(value) if !value.is_object() && !value.is_null() => return Ok(()),
        _ => {}
    }

    if object.get(CLOSING_KEY).is_some_and(Value::is_null) {
        object.remove(CLOSING_KEY);
    }

    let mut closing = object
        .remove(CLOSING_KEY)
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default();

    for (old_key, new_key) in KEY_MAPPINGS {
        if let Some(value) = object.remove(*old_key) {
            closing.entry(*new_key).or_insert(value);
        }
    }
    object.insert(CLOSING_KEY.into(), Value::Object(closing));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn moves_existing_settings_and_preserves_new_values() {
        let mut value = json!({
            "when_closing_with_no_tabs": "close_window",
            "on_last_window_closed": "quit_app",
            "window_closing": {"on_last_window_closed": "platform_default", "on_last_tab_closed": "close_window_if_no_project"},
            "confirm_quit": true
        });
        nest_window_closing_settings(&mut value).unwrap();
        assert_eq!(
            value,
            json!({
                "window_closing": {
                    "on_close_with_no_tabs": "close_window",
                    "on_last_tab_closed": "close_window_if_no_project",
                    "on_last_window_closed": "platform_default"
                },
                "confirm_quit": true
            })
        );
        let unchanged = value.clone();
        nest_window_closing_settings(&mut value).unwrap();
        assert_eq!(value, unchanged);
    }

    #[test]
    fn migrates_platform_release_and_profile_scopes() {
        let old = json!({"on_last_window_closed": "quit_app"});
        let mut value = json!({
            "when_closing_with_no_tabs": "keep_window_open",
            "macos": old.clone(),
            "preview": old.clone(),
            "profiles": {
                "direct": old.clone(),
                "wrapped": {"settings": old}
            }
        });
        nest_window_closing_settings(&mut value).unwrap();
        assert_eq!(
            value["window_closing"]["on_close_with_no_tabs"],
            "keep_window_open"
        );
        for closing in [
            &value["macos"]["window_closing"],
            &value["preview"]["window_closing"],
            &value["profiles"]["direct"]["window_closing"],
            &value["profiles"]["wrapped"]["settings"]["window_closing"],
        ] {
            assert_eq!(closing["on_last_window_closed"], "quit_app");
        }
    }

    #[test]
    fn does_not_discard_settings_when_destination_is_invalid() {
        let mut value = json!({
            "window_closing": false,
            "when_closing_with_no_tabs": "close_window"
        });
        let unchanged = value.clone();
        nest_window_closing_settings(&mut value).unwrap();
        assert_eq!(value, unchanged);

        let mut null_destination = json!({
            "window_closing": null,
            "on_last_window_closed": "quit_app"
        });
        nest_window_closing_settings(&mut null_destination).unwrap();
        assert_eq!(
            null_destination,
            json!({
                "window_closing": { "on_last_window_closed": "quit_app" }
            })
        );
    }

    #[test]
    fn registration_migrates_existing_settings() {
        let migrated = crate::migrate_settings(r#"{"when_closing_with_no_tabs":"close_window"}"#)
            .unwrap()
            .expect("expected migration");
        let value: Value = serde_json::from_str(&migrated).unwrap();
        assert_eq!(
            value["window_closing"]["on_close_with_no_tabs"],
            "close_window"
        );
        assert_eq!(crate::migrate_settings(&migrated).unwrap(), None);
    }
}
