use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

pub fn migrate_tool_permission_defaults(value: &mut Value) -> Result<()> {
    migrate_settings(value, migrate_one)
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()> {
    if let Some(agent) = obj.get_mut("agent") {
        migrate_agent_with_profiles(agent)?;
    }
    Ok(())
}

fn migrate_agent_with_profiles(agent: &mut Value) -> Result<()> {
    migrate_agent_tool_permissions(agent)?;

    if let Some(agent_object) = agent.as_object_mut() {
        if let Some(profiles) = agent_object.get_mut("profiles") {
            if let Some(profiles_object) = profiles.as_object_mut() {
                for (_profile_name, profile) in profiles_object.iter_mut() {
                    migrate_agent_tool_permissions(profile)?;
                }
            }
        }
    }

    Ok(())
}

fn migrate_agent_tool_permissions(agent: &mut Value) -> Result<()> {
    let Some(agent_object) = agent.as_object_mut() else {
        return Ok(());
    };

    let should_migrate_always_allow = match agent_object.get("always_allow_tool_actions") {
        Some(Value::Bool(true)) => {
            agent_object.remove("always_allow_tool_actions");
            true
        }
        Some(Value::Bool(false)) | Some(Value::Null) | None => {
            agent_object.remove("always_allow_tool_actions");
            false
        }
        Some(_) => {
            // Non-boolean value — leave it in place so the schema validator
            // can report it, rather than silently dropping user data.
            false
        }
    };

    if should_migrate_always_allow {
        let tool_permissions = agent_object
            .entry("tool_permissions")
            .or_insert_with(|| Value::Object(Default::default()));

        // If tool_permissions exists but isn't an object (e.g. null), replace it
        // so we don't silently drop the user's always_allow preference.
        if !tool_permissions.is_object() {
            *tool_permissions = Value::Object(Default::default());
        }

        let Some(tool_permissions_object) = tool_permissions.as_object_mut() else {
            return Ok(());
        };

        if !tool_permissions_object.contains_key("default")
            && !tool_permissions_object.contains_key("default_mode")
        {
            tool_permissions_object
                .insert("default".to_string(), Value::String("allow".to_string()));
        }
    }

    if let Some(tool_permissions) = agent_object.get_mut("tool_permissions") {
        migrate_default_mode_to_default(tool_permissions)?;
    }

    Ok(())
}

fn migrate_default_mode_to_default(tool_permissions: &mut Value) -> Result<()> {
    let Some(tool_permissions_object) = tool_permissions.as_object_mut() else {
        return Ok(());
    };

    if let Some(default_mode) = tool_permissions_object.remove("default_mode") {
        if !tool_permissions_object.contains_key("default") {
            tool_permissions_object.insert("default".to_string(), default_mode);
        }
    }

    if let Some(tools) = tool_permissions_object.get_mut("tools") {
        if let Some(tools_object) = tools.as_object_mut() {
            for (_tool_name, tool_rules) in tools_object.iter_mut() {
                if let Some(tool_rules_object) = tool_rules.as_object_mut() {
                    if let Some(default_mode) = tool_rules_object.remove("default_mode") {
                        if !tool_rules_object.contains_key("default") {
                            tool_rules_object.insert("default".to_string(), default_mode);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
