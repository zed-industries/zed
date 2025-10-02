use anyhow::Result;
use serde_json::Value;

pub fn remove_formatters_on_save(value: &mut Value) -> Result<()> {
    remove_formatters_on_save_inner(value)?;
    let languages = value
        .as_object_mut()
        .and_then(|obj| obj.get_mut("languages"))
        .and_then(|languages| languages.as_object_mut());
    if let Some(languages) = languages {
        for (_, language) in languages.iter_mut() {
            remove_formatters_on_save_inner(language)?
        }
    }
    Ok(())
}

fn remove_formatters_on_save_inner(value: &mut Value) -> Result<()> {
    let Some(obj) = value.as_object_mut() else {
        return Ok(());
    };
    let Some(format_on_save) = obj.get("format_on_save").cloned() else {
        return Ok(());
    };
    let is_format_on_save_set_to_formatter = format_on_save
        .as_str()
        .map_or(true, |s| s != "on" && s != "off");
    if !is_format_on_save_set_to_formatter {
        return Ok(());
    }

    anyhow::ensure!(
        obj.get("formatter").is_none(),
        "Setting formatters in both format_on_save and formatters is deprecated. Please migrate the formatters from `format_on_save` into `formatters`"
    );

    obj.insert("format_on_save".to_string(), serde_json::json!("on"));
    obj.insert("formatter".to_string(), format_on_save);

    Ok(())
}
