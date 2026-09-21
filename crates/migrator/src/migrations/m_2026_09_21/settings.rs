use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_language_setting;

const SOFT_WRAP_KEY: &str = "soft_wrap";
const OLD_VALUE: &str = "prefer_line";
const NEW_VALUE: &str = "none";

pub fn replace_prefer_line_soft_wrap(value: &mut Value) -> Result<()> {
    migrate_language_setting(value, migrate_soft_wrap)
}

fn migrate_soft_wrap(value: &mut Value, _path: &[&str]) -> Result<()> {
    let Some(soft_wrap) = value
        .as_object_mut()
        .and_then(|object| object.get_mut(SOFT_WRAP_KEY))
    else {
        return Ok(());
    };
    if soft_wrap.as_str() == Some(OLD_VALUE) {
        *soft_wrap = Value::String(NEW_VALUE.to_string());
    }
    Ok(())
}
