use anyhow::Result;
use serde_json::Value;

pub fn migrate_always_allow_tool_actions_to_default_mode(value: &mut Value) -> Result<()> {
    let Some(obj) = value.as_object_mut() else {
        return Ok(());
    };

    let Some(agent) = obj.get_mut("agent") else {
        return Ok(());
    };

    let Some(agent_obj) = agent.as_object_mut() else {
        return Ok(());
    };

    // Check if always_allow_tool_actions exists and is true
    let should_migrate_always_allow = agent_obj
        .get("always_allow_tool_actions")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Remove the old setting regardless of its value
    agent_obj.remove("always_allow_tool_actions");

    // Get or create tool_permissions if we need to set default from always_allow_tool_actions
    if should_migrate_always_allow {
        let tool_permissions = agent_obj
            .entry("tool_permissions")
            .or_insert_with(|| Value::Object(Default::default()));

        let Some(tool_permissions_obj) = tool_permissions.as_object_mut() else {
            anyhow::bail!("Expected tool_permissions to be an object");
        };

        // Only set default if neither default nor default_mode is already set
        if !tool_permissions_obj.contains_key("default")
            && !tool_permissions_obj.contains_key("default_mode")
        {
            tool_permissions_obj.insert("default".to_string(), Value::String("allow".to_string()));
        }
    }

    // Now migrate any existing default_mode to default in tool_permissions
    if let Some(tool_permissions) = agent_obj.get_mut("tool_permissions") {
        migrate_default_mode_to_default(tool_permissions)?;
    }

    Ok(())
}

fn migrate_default_mode_to_default(tool_permissions: &mut Value) -> Result<()> {
    let Some(tool_permissions_obj) = tool_permissions.as_object_mut() else {
        return Ok(());
    };

    // Rename top-level default_mode to default (if default doesn't already exist)
    if let Some(default_mode) = tool_permissions_obj.remove("default_mode") {
        if !tool_permissions_obj.contains_key("default") {
            tool_permissions_obj.insert("default".to_string(), default_mode);
        }
    }

    // Rename default_mode to default in each tool's rules
    if let Some(tools) = tool_permissions_obj.get_mut("tools") {
        if let Some(tools_obj) = tools.as_object_mut() {
            for (_tool_name, tool_rules) in tools_obj.iter_mut() {
                if let Some(tool_rules_obj) = tool_rules.as_object_mut() {
                    if let Some(default_mode) = tool_rules_obj.remove("default_mode") {
                        if !tool_rules_obj.contains_key("default") {
                            tool_rules_obj.insert("default".to_string(), default_mode);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
