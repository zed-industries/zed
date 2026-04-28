use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

const SETTINGS_KEY: &str = "settings";
const TITLE_BAR_KEY: &str = "title_bar";
const OLD_KEY: &str = "show_branch_icon";
const NEW_KEY: &str = "show_branch_status_icon";

pub fn promote_show_branch_icon_true_to_show_branch_status_icon(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(object: &mut serde_json::Map<String, Value>) -> Result<()> {
    migrate_title_bar_value(object);

    if let Some(settings) = object
        .get_mut(SETTINGS_KEY)
        .and_then(|value| value.as_object_mut())
    {
        migrate_title_bar_value(settings);
    }

    Ok(())
}

fn migrate_title_bar_value(object: &mut serde_json::Map<String, Value>) {
    let Some(title_bar) = object
        .get_mut(TITLE_BAR_KEY)
        .and_then(|value| value.as_object_mut())
    else {
        return;
    };

    let Some(old_value) = title_bar.remove(OLD_KEY) else {
        return;
    };

    if title_bar.contains_key(NEW_KEY) {
        return;
    }

    if old_value == Value::Bool(true) {
        title_bar.insert(NEW_KEY.to_string(), Value::Bool(true));
    }
}
