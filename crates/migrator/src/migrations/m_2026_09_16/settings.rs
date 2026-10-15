use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

const THREADS_SIDEBAR_KEY: &str = "threads_sidebar";
const KEY_MAPPINGS: &[(&str, &str)] = &[
    ("sidebar_side", "position"),
    ("threads_sidebar_default_width", "default_width"),
    ("threads_sidebar_auto_open", "auto_open"),
];

pub fn nest_agent_threads_sidebar_settings(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(object: &mut serde_json::Map<String, Value>) -> Result<()> {
    let Some(agent) = object.get_mut("agent").and_then(Value::as_object_mut) else {
        return Ok(());
    };
    let has_legacy_settings = KEY_MAPPINGS
        .iter()
        .any(|(old_key, _)| agent.contains_key(*old_key));
    if !has_legacy_settings {
        return Ok(());
    }

    let mut threads_sidebar = match agent.remove(THREADS_SIDEBAR_KEY) {
        None | Some(Value::Null) => serde_json::Map::new(),
        Some(Value::Object(settings)) => settings,
        Some(value) => {
            agent.insert(THREADS_SIDEBAR_KEY.into(), value);
            return Ok(());
        }
    };

    for (old_key, new_key) in KEY_MAPPINGS {
        if let Some(value) = agent.remove(*old_key) {
            threads_sidebar.entry(*new_key).or_insert(value);
        }
    }
    agent.insert(THREADS_SIDEBAR_KEY.into(), Value::Object(threads_sidebar));

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn migrate(value: &mut Value) {
        nest_agent_threads_sidebar_settings(value).expect("migration should succeed");
    }

    #[test]
    fn nest_agent_threads_sidebar_settings_moves_keys_and_preserves_unrelated_fields() {
        let mut value = json!({
            "agent": {
                "sidebar_side": "right",
                "threads_sidebar_default_width": 420,
                "threads_sidebar_auto_open": false,
                "button": false
            }
        });
        migrate(&mut value);
        assert_eq!(
            value,
            json!({
                "agent": {
                    "threads_sidebar": { "position": "right", "default_width": 420, "auto_open": false },
                    "button": false
                }
            })
        );
        let migrated = value.clone();
        migrate(&mut value);
        assert_eq!(value, migrated);
    }

    #[test]
    fn nest_agent_threads_sidebar_settings_preserves_nested_values_and_fills_missing_fields() {
        let mut value = json!({
            "agent": {
                "sidebar_side": "left",
                "threads_sidebar_default_width": 360,
                "threads_sidebar_auto_open": true,
                "threads_sidebar": { "position": null, "auto_open": false }
            }
        });
        migrate(&mut value);
        assert_eq!(
            value,
            json!({
                "agent": {
                    "threads_sidebar": { "position": null, "default_width": 360, "auto_open": false }
                }
            })
        );
    }

    #[test]
    fn nest_agent_threads_sidebar_settings_handles_null_destinations_only_when_needed() {
        let mut with_legacy = json!({
            "agent": {
                "sidebar_side": "right",
                "threads_sidebar": null
            }
        });
        migrate(&mut with_legacy);
        assert_eq!(
            with_legacy,
            json!({ "agent": { "threads_sidebar": { "position": "right" } } })
        );
    }

    #[test]
    fn nest_agent_threads_sidebar_settings_leaves_unrelated_and_malformed_values_untouched() {
        for mut value in [
            json!({ "agent": { "threads_sidebar": null } }),
            json!({ "agent": false }),
            json!({
                "agent": {
                    "sidebar_side": "left",
                    "threads_sidebar_default_width": 360,
                    "threads_sidebar": false
                }
            }),
        ] {
            let original = value.clone();
            migrate(&mut value);
            assert_eq!(value, original);
        }
    }

    #[test]
    fn nest_agent_threads_sidebar_settings_migrates_all_supported_scopes() {
        let legacy_agent = json!({
            "sidebar_side": "right",
            "threads_sidebar_default_width": 420,
            "threads_sidebar_auto_open": false
        });
        let mut value = json!({
            "agent": legacy_agent,
            "macos": { "agent": legacy_agent },
            "preview": { "agent": legacy_agent },
            "profiles": {
                "direct": { "agent": legacy_agent },
                "wrapped": { "settings": { "agent": legacy_agent } }
            }
        });
        migrate(&mut value);

        for agent in [
            &value["agent"],
            &value["macos"]["agent"],
            &value["preview"]["agent"],
            &value["profiles"]["direct"]["agent"],
            &value["profiles"]["wrapped"]["settings"]["agent"],
        ] {
            assert_eq!(
                agent,
                &json!({
                    "threads_sidebar": { "position": "right", "default_width": 420, "auto_open": false }
                })
            );
        }
    }
}
