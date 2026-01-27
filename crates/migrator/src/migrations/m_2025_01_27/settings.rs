use anyhow::Result;
use serde_json::Value;

use crate::patterns::migrate_language_setting;

pub fn make_auto_indent_an_enum(value: &mut Value) -> Result<()> {
    migrate_language_setting(value, migrate_auto_indent)
}

fn migrate_auto_indent(value: &mut Value, _path: &[&str]) -> Result<()> {
    let Some(auto_indent) = value
        .as_object_mut()
        .and_then(|obj| obj.get_mut("auto_indent"))
    else {
        return Ok(());
    };

    *auto_indent = match auto_indent {
        Value::Bool(true) => Value::String("full".to_string()),
        Value::Bool(false) => Value::String("none".to_string()),
        Value::String(s) if s == "full" || s == "preserve_indent" || s == "none" => return Ok(()),
        _ => anyhow::bail!("Expected auto_indent to be a boolean or valid enum value"),
    };
    Ok(())
}
