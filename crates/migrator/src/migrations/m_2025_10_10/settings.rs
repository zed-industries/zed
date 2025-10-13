use anyhow::Result;
use serde_json::Value;

pub fn remove_code_actions_on_format(value: &mut Value) -> Result<()> {
    remove_code_actions_on_format_inner(value, &[])?;
    let languages = value
        .as_object_mut()
        .and_then(|obj| obj.get_mut("languages"))
        .and_then(|languages| languages.as_object_mut());
    if let Some(languages) = languages {
        for (language_name, language) in languages.iter_mut() {
            let path = vec!["languages", language_name];
            remove_code_actions_on_format_inner(language, &path)?;
        }
    }
    Ok(())
}

fn remove_code_actions_on_format_inner(value: &mut Value, path: &[&str]) -> Result<()> {
    let Some(obj) = value.as_object_mut() else {
        return Ok(());
    };
    let Some(code_actions_on_format) = obj.get("code_actions_on_format").cloned() else {
        return Ok(());
    };

    fn fmt_path(path: &[&str], key: &str) -> String {
        let mut path = path.to_vec();
        path.push(key);
        path.join(".")
    }

    anyhow::ensure!(
        code_actions_on_format.is_object(),
        r#"The `code_actions_on_format` setting is deprecated, but it is in an invalid state and cannot be migrated at {}. Please ensure the code_actions_on_format setting is a Map<String, bool>"#,
        fmt_path(path, "code_actions_on_format"),
    );

    let code_actions_map = code_actions_on_format.as_object().unwrap();
    let mut code_actions = vec![];
    for (code_action, code_action_enabled) in code_actions_map {
        if code_action_enabled.as_bool().map_or(false, |b| !b) {
            continue;
        }
        code_actions.push(code_action.clone());
    }

    let mut formatter_array = vec![];
    if let Some(formatter) = obj.get("formatter") {
        if let Some(array) = formatter.as_array() {
            formatter_array = array.clone();
        } else {
            formatter_array.insert(0, formatter.clone());
        }
    };
    let found_code_actions = !code_actions.is_empty();
    formatter_array.splice(
        0..0,
        code_actions
            .into_iter()
            .map(|code_action| serde_json::json!({"code_action": code_action})),
    );

    obj.remove("code_actions_on_format");
    if found_code_actions {
        obj.insert("formatter".to_string(), Value::Array(formatter_array));
    }

    Ok(())
}
