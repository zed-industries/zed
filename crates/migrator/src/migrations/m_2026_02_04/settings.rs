use anyhow::{Result, bail};
use serde_json::Value;

use crate::migrations::migrate_settings;

const AGENT_KEY: &str = "agent";
const ALWAYS_ALLOW_TOOL_ACTIONS: &str = "always_allow_tool_actions";
const DEFAULT_KEY: &str = "default";
const DEFAULT_MODE_KEY: &str = "default_mode";
const PROFILES_KEY: &str = "profiles";
const TOOL_PERMISSIONS_KEY: &str = "tool_permissions";
const TOOLS_KEY: &str = "tools";

pub fn migrate_tool_permission_defaults(value: &mut Value) -> Result<()> {
    migrate_settings(value, migrate_one)
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()> {
    if let Some(agent) = obj.get_mut(AGENT_KEY) {
        migrate_agent_with_profiles(agent)?;
    }

    Ok(())
}

fn migrate_agent_with_profiles(agent: &mut Value) -> Result<()> {
    migrate_agent_tool_permissions(agent)?;

    if let Some(agent_object) = agent.as_object_mut() {
        if let Some(profiles) = agent_object.get_mut(PROFILES_KEY) {
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

    let should_migrate_always_allow = match agent_object.get(ALWAYS_ALLOW_TOOL_ACTIONS) {
        Some(Value::Bool(true)) => {
            agent_object.remove(ALWAYS_ALLOW_TOOL_ACTIONS);
            true
        }
        Some(Value::Bool(false)) | Some(Value::Null) | None => {
            agent_object.remove(ALWAYS_ALLOW_TOOL_ACTIONS);
            false
        }
        Some(_) => {
            // Non-boolean value — leave it in place so the schema validator
            // can report it, rather than silently dropping user data.
            false
        }
    };

    if should_migrate_always_allow {
        if matches!(
            agent_object.get(TOOL_PERMISSIONS_KEY),
            None | Some(Value::Null)
        ) {
            agent_object.insert(
                TOOL_PERMISSIONS_KEY.to_string(),
                Value::Object(Default::default()),
            );
        }

        let Some(Value::Object(tool_permissions_object)) =
            agent_object.get_mut(TOOL_PERMISSIONS_KEY)
        else {
            bail!(
                "agent.tool_permissions should be an object or null when migrating \
                 always_allow_tool_actions"
            );
        };

        if !tool_permissions_object.contains_key(DEFAULT_KEY)
            && !tool_permissions_object.contains_key(DEFAULT_MODE_KEY)
        {
            tool_permissions_object
                .insert(DEFAULT_KEY.to_string(), Value::String("allow".to_string()));
        }
    }

    if let Some(tool_permissions) = agent_object.get_mut(TOOL_PERMISSIONS_KEY) {
        migrate_default_mode_to_default(tool_permissions)?;
    }

    Ok(())
}

fn migrate_default_mode_to_default(tool_permissions: &mut Value) -> Result<()> {
    let Some(tool_permissions_object) = tool_permissions.as_object_mut() else {
        return Ok(());
    };

    if let Some(default_mode) = tool_permissions_object.remove(DEFAULT_MODE_KEY) {
        if !tool_permissions_object.contains_key(DEFAULT_KEY) {
            tool_permissions_object.insert(DEFAULT_KEY.to_string(), default_mode);
        }
    }

    if let Some(tools) = tool_permissions_object.get_mut(TOOLS_KEY) {
        if let Some(tools_object) = tools.as_object_mut() {
            for (_tool_name, tool_rules) in tools_object.iter_mut() {
                if let Some(tool_rules_object) = tool_rules.as_object_mut() {
                    if let Some(default_mode) = tool_rules_object.remove(DEFAULT_MODE_KEY) {
                        if !tool_rules_object.contains_key(DEFAULT_KEY) {
                            tool_rules_object.insert(DEFAULT_KEY.to_string(), default_mode);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
