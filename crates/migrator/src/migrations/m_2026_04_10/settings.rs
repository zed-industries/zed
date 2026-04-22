use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

const AGENT_KEY: &str = "agent";
const PROFILES_KEY: &str = "profiles";
const TOOL_PERMISSIONS_KEY: &str = "tool_permissions";
const TOOLS_KEY: &str = "tools";
const OLD_TOOL_NAME: &str = "web_search";
const NEW_TOOL_NAME: &str = "search_web";

pub fn rename_web_search_to_search_web(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_agent_value)
}

fn migrate_agent_value(object: &mut serde_json::Map<String, Value>) -> Result<()> {
    let Some(agent) = object.get_mut(AGENT_KEY).and_then(|v| v.as_object_mut()) else {
        return Ok(());
    };

    if let Some(tools) = agent
        .get_mut(TOOL_PERMISSIONS_KEY)
        .and_then(|v| v.as_object_mut())
        .and_then(|tp| tp.get_mut(TOOLS_KEY))
        .and_then(|v| v.as_object_mut())
    {
        rename_key(tools);
    }

    if let Some(profiles) = agent.get_mut(PROFILES_KEY).and_then(|v| v.as_object_mut()) {
        for (_profile_name, profile) in profiles.iter_mut() {
            if let Some(tools) = profile
                .as_object_mut()
                .and_then(|p| p.get_mut(TOOLS_KEY))
                .and_then(|v| v.as_object_mut())
            {
                rename_key(tools);
            }
        }
    }

    Ok(())
}

fn rename_key(tools: &mut serde_json::Map<String, Value>) {
    if let Some(value) = tools.remove(OLD_TOOL_NAME) {
        tools.insert(NEW_TOOL_NAME.to_string(), value);
    }
}
