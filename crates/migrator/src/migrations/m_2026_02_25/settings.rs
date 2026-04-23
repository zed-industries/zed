use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

const AGENT_SERVERS_KEY: &str = "agent_servers";

struct BuiltinMapping {
    old_key: &'static str,
    registry_key: &'static str,
}

const BUILTIN_MAPPINGS: &[BuiltinMapping] = &[
    BuiltinMapping {
        old_key: "gemini",
        registry_key: "gemini",
    },
    BuiltinMapping {
        old_key: "claude",
        registry_key: "claude-acp",
    },
    BuiltinMapping {
        old_key: "codex",
        registry_key: "codex-acp",
    },
];

const REGISTRY_COMPATIBLE_FIELDS: &[&str] = &[
    "env",
    "default_mode",
    "default_model",
    "favorite_models",
    "default_config_options",
    "favorite_config_option_values",
];

pub fn migrate_builtin_agent_servers_to_registry(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()> {
    let Some(agent_servers) = obj.get_mut(AGENT_SERVERS_KEY) else {
        return Ok(());
    };
    let Some(servers_map) = agent_servers.as_object_mut() else {
        return Ok(());
    };

    for mapping in BUILTIN_MAPPINGS {
        migrate_builtin_entry(servers_map, mapping);
    }

    Ok(())
}

fn migrate_builtin_entry(
    servers_map: &mut serde_json::Map<String, Value>,
    mapping: &BuiltinMapping,
) {
    // Check if the old key exists and needs migration before taking ownership.
    let needs_migration = servers_map
        .get(mapping.old_key)
        .and_then(|v| v.as_object())
        .is_some_and(|obj| !obj.contains_key("type"));

    if !needs_migration {
        return;
    }

    // When the registry key differs from the old key and the target already
    // exists, just remove the stale old entry to avoid overwriting user data.
    if mapping.old_key != mapping.registry_key && servers_map.contains_key(mapping.registry_key) {
        servers_map.remove(mapping.old_key);
        return;
    }

    let Some(old_entry) = servers_map.remove(mapping.old_key) else {
        return;
    };
    let Some(old_obj) = old_entry.as_object() else {
        return;
    };

    let has_command = old_obj.contains_key("command");
    let ignore_system_version = old_obj
        .get("ignore_system_version")
        .and_then(|v| v.as_bool());

    // A custom entry is needed when the user configured a custom binary
    // or explicitly opted into using the system version via
    // `ignore_system_version: false` (only meaningful for gemini).
    let needs_custom = has_command
        || (mapping.old_key == "gemini" && matches!(ignore_system_version, Some(false)));

    if needs_custom {
        let local_key = format!("{}-custom", mapping.registry_key);

        // Don't overwrite an existing `-custom` entry.
        if servers_map.contains_key(&local_key) {
            return;
        }

        let mut custom_obj = serde_json::Map::new();
        custom_obj.insert("type".to_string(), Value::String("custom".to_string()));

        if has_command {
            if let Some(command) = old_obj.get("command") {
                custom_obj.insert("command".to_string(), command.clone());
            }
            if let Some(args) = old_obj.get("args") {
                if !args.as_array().is_some_and(|a| a.is_empty()) {
                    custom_obj.insert("args".to_string(), args.clone());
                }
            }
        } else {
            // ignore_system_version: false — the user wants the binary from $PATH
            custom_obj.insert(
                "command".to_string(),
                Value::String(mapping.old_key.to_string()),
            );
        }

        // Carry over all compatible fields to the custom entry.
        for &field in REGISTRY_COMPATIBLE_FIELDS {
            if let Some(value) = old_obj.get(field) {
                match value {
                    Value::Array(arr) if arr.is_empty() => continue,
                    Value::Object(map) if map.is_empty() => continue,
                    Value::Null => continue,
                    _ => {
                        custom_obj.insert(field.to_string(), value.clone());
                    }
                }
            }
        }

        servers_map.insert(local_key, Value::Object(custom_obj));
    } else {
        // Build a registry entry with compatible fields only.
        let mut registry_obj = serde_json::Map::new();
        registry_obj.insert("type".to_string(), Value::String("registry".to_string()));

        for &field in REGISTRY_COMPATIBLE_FIELDS {
            if let Some(value) = old_obj.get(field) {
                match value {
                    Value::Array(arr) if arr.is_empty() => continue,
                    Value::Object(map) if map.is_empty() => continue,
                    Value::Null => continue,
                    _ => {
                        registry_obj.insert(field.to_string(), value.clone());
                    }
                }
            }
        }

        servers_map.insert(
            mapping.registry_key.to_string(),
            Value::Object(registry_obj),
        );
    }
}
