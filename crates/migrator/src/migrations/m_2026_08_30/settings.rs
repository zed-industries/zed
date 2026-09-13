use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

const MARKDOWN_PREVIEW_KEY: &str = "markdown_preview";
const KEY_MAPPINGS: &[(&str, &str)] = &[
    ("markdown_preview_font_size", "font_size"),
    ("markdown_preview_font_family", "font_family"),
    ("markdown_preview_code_font_family", "code_font_family"),
    ("markdown_preview_theme", "theme"),
];

pub fn nest_markdown_preview_settings(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_markdown_preview_settings)
}

fn migrate_markdown_preview_settings(object: &mut serde_json::Map<String, Value>) -> Result<()> {
    let has_legacy_settings = KEY_MAPPINGS
        .iter()
        .any(|(old_key, _)| object.contains_key(*old_key));
    match object.get(MARKDOWN_PREVIEW_KEY) {
        Some(Value::Null) if has_legacy_settings => {
            object.remove(MARKDOWN_PREVIEW_KEY);
        }
        Some(markdown_preview) if !markdown_preview.is_object() => return Ok(()),
        _ => {}
    }

    let migrated_settings = KEY_MAPPINGS
        .iter()
        .filter_map(|(old_key, new_key)| {
            object
                .remove(*old_key)
                .map(|value| ((*new_key).to_string(), value))
        })
        .collect::<Vec<_>>();

    if migrated_settings.is_empty() {
        return Ok(());
    }

    let Some(markdown_preview) = object
        .entry(MARKDOWN_PREVIEW_KEY)
        .or_insert_with(|| Value::Object(serde_json::Map::new()))
        .as_object_mut()
    else {
        return Ok(());
    };

    for (key, value) in migrated_settings {
        markdown_preview.entry(key).or_insert(value);
    }

    Ok(())
}

const AGENT_KEY: &str = "agent";
const OLD_KEY: &str = "sidebar_side";
const THREAD_SIDEBAR_KEY: &str = "thread_sidebar";
const POSITION_KEY: &str = "position";

pub fn nest_agent_sidebar_side_under_thread_sidebar_position(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_agent_sidebar_side)
}

fn migrate_agent_sidebar_side(object: &mut serde_json::Map<String, Value>) -> Result<()> {
    let Some(agent) = object.get_mut(AGENT_KEY).and_then(Value::as_object_mut) else {
        return Ok(());
    };

    if agent
        .get(THREAD_SIDEBAR_KEY)
        .is_some_and(|thread_sidebar| !thread_sidebar.is_object())
    {
        return Ok(());
    }

    let Some(sidebar_side) = agent.remove(OLD_KEY) else {
        return Ok(());
    };

    if let Some(thread_sidebar) = agent
        .get_mut(THREAD_SIDEBAR_KEY)
        .and_then(Value::as_object_mut)
    {
        thread_sidebar.entry(POSITION_KEY).or_insert(sidebar_side);
    } else {
        let mut thread_sidebar = serde_json::Map::new();
        thread_sidebar.insert(POSITION_KEY.to_string(), sidebar_side);
        agent.insert(
            THREAD_SIDEBAR_KEY.to_string(),
            Value::Object(thread_sidebar),
        );
    }

    Ok(())
}
