use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

pub fn make_git_gutter_width_an_enum(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()> {
    let Some(gutter) = obj
        .get_mut("gutter")
        .and_then(|gutter| gutter.as_object_mut())
    else {
        return Ok(());
    };

    let Some(git_gutter_width) = gutter.get_mut("git_gutter_width") else {
        return Ok(());
    };

    *git_gutter_width = match git_gutter_width {
        Value::Number(n) => {
            let width = n
                .as_f64()
                .ok_or_else(|| anyhow::anyhow!("Expected git_gutter_width to be a number"))?;
            Value::Object(
                [("custom".to_string(), Value::from(width))]
                    .into_iter()
                    .collect(),
            )
        }
        _ => return Ok(()),
    };

    Ok(())
}
