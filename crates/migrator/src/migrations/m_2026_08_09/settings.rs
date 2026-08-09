use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

const PANEL_KEYS: &[&str] = &["project_panel", "outline_panel", "git_panel"];
const OLD_KEY: &str = "folder_icons";
const NEW_KEY: &str = "folder_indicator";

pub fn rename_folder_icons_to_folder_indicator(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(object: &mut serde_json::Map<String, Value>) -> Result<()> {
    for panel_key in PANEL_KEYS {
        let Some(panel) = object.get_mut(*panel_key).and_then(Value::as_object_mut) else {
            continue;
        };

        // Anything other than a boolean never deserialized as `folder_icons`, so leave
        // it in place rather than guessing which indicator was meant.
        let indicator = match panel.get(OLD_KEY) {
            Some(Value::Bool(true)) => "icon",
            Some(Value::Bool(false)) => "chevron",
            _ => continue,
        };

        panel.remove(OLD_KEY);
        panel
            .entry(NEW_KEY)
            .or_insert_with(|| Value::String(indicator.to_string()));
    }

    Ok(())
}
