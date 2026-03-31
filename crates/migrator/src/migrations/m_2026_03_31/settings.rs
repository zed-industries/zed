use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

pub fn remove_text_thread_settings(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()> {
    // Remove `agent.default_view`
    if let Some(agent) = obj.get_mut("agent") {
        if let Some(agent_obj) = agent.as_object_mut() {
            agent_obj.remove("default_view");
        }
    }

    // Remove `edit_predictions.enabled_in_text_threads`
    if let Some(edit_predictions) = obj.get_mut("edit_predictions") {
        if let Some(edit_predictions_obj) = edit_predictions.as_object_mut() {
            edit_predictions_obj.remove("enabled_in_text_threads");
        }
    }

    // Remove top-level `slash_commands`
    obj.remove("slash_commands");

    Ok(())
}
