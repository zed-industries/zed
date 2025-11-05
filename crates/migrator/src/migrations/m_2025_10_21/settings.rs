use anyhow::Result;
use serde_json::Value;

pub fn make_relative_line_numbers_an_enum(value: &mut Value) -> Result<()> {
    let Some(relative_line_numbers) = value.get_mut("relative_line_numbers") else {
        return Ok(());
    };

    *relative_line_numbers = match relative_line_numbers {
        Value::Bool(true) => Value::String("enabled".to_string()),
        Value::Bool(false) => Value::String("disabled".to_string()),
        Value::String(s) if s == "enabled" || s == "disabled" || s == "wrapped" => return Ok(()),
        _ => anyhow::bail!("Expected relative_line_numbers to be a boolean"),
    };
    Ok(())
}
