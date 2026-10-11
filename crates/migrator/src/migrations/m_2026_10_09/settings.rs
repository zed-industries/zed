use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_language_setting;

pub fn replace_prefer_line_soft_wrap(value: &mut Value) -> Result<()> {
    migrate_language_setting(value, migrate_soft_wrap)
}

fn migrate_soft_wrap(value: &mut Value, _path: &[&str]) -> Result<()> {
    if let Some(soft_wrap) = value.get_mut("soft_wrap")
        && *soft_wrap == "prefer_line"
    {
        *soft_wrap = Value::from("none");
    }
    Ok(())
}
