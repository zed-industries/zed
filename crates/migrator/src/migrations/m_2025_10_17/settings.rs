use anyhow::Result;
use serde_json::Value;

pub fn make_file_finder_include_ignored_an_enum(value: &mut Value) -> Result<()> {
    let Some(file_finder) = value.get_mut("file_finder") else {
        return Ok(());
    };

    let Some(file_finder_obj) = file_finder.as_object_mut() else {
        anyhow::bail!("Expected file_finder to be an object");
    };

    let Some(include_ignored) = file_finder_obj.get_mut("include_ignored") else {
        return Ok(());
    };
    *include_ignored = match include_ignored {
        Value::Bool(true) => Value::String("all".to_string()),
        Value::Bool(false) => Value::String("indexed".to_string()),
        Value::Null => Value::String("smart".to_string()),
        Value::String(s) if s == "all" || s == "indexed" || s == "smart" => return Ok(()),
        _ => anyhow::bail!("Expected include_ignored to be a boolean or null"),
    };
    Ok(())
}
