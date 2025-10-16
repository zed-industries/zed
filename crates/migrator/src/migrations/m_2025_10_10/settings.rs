use anyhow::Result;
use serde_json::Value;
use settings::merge_from::MergeFrom;

// todo! 1 - if formatter set at root, that is the new default formatter
// todo! 2 - if no default formatter for language, use previous default
// order:
//  4. root from defaults
//  3. language from defaults
//  2. root from user
//  1. language from user
pub fn remove_code_actions_on_format(value: &mut Value) -> Result<()> {
    let defaults =
        settings::parse_json_with_comments::<Value>(settings::default_settings().as_ref()).unwrap();
    let default_root_formatter = defaults.get("formatter");

    let root_code_actions =
        remove_code_actions_on_format_inner(value, default_root_formatter, &[], &[])?;
    let user_default_formatter = value.get("formatter").cloned();

    let user_languages = value
        .as_object_mut()
        .and_then(|obj| obj.get_mut("languages"))
        .and_then(|languages| languages.as_object_mut());
    let default_languages = defaults
        .as_object()
        .and_then(|obj| obj.get("languages"))
        .and_then(|languages| languages.as_object());

    if let Some(languages) = user_languages {
        for (language_name, language) in languages.iter_mut() {
            let path = vec!["languages", language_name];
            let language_default_formatter = default_languages
                .and_then(|langs| langs.get(language_name))
                .and_then(|lang| lang.get("formatter"));

            let default_formatter_for_language = user_default_formatter
                .as_ref()
                .or(language_default_formatter)
                .or(default_root_formatter);
            remove_code_actions_on_format_inner(
                language,
                default_formatter_for_language,
                &root_code_actions,
                &path,
            )?;
        }
    }
    Ok(())
}

// todo! include parent code_actions_on_format
fn remove_code_actions_on_format_inner(
    value: &mut Value,
    default_formatters: Option<&Value>,
    parent_code_actions: &[String],
    path: &[&str],
) -> Result<Vec<String>> {
    let Some(obj) = value.as_object_mut() else {
        return Ok(vec![]);
    };
    let Some(code_actions_on_format) = obj.get("code_actions_on_format").cloned() else {
        return Ok(vec![]);
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
    code_actions.extend(parent_code_actions.iter().cloned());

    let mut formatter_array = vec![];
    if let Some(formatter) = obj.get("formatter") {
        if let Some(array) = formatter.as_array() {
            formatter_array = array.clone();
        } else {
            formatter_array.insert(0, formatter.clone());
        }
    } else if let Some(formatter) = default_formatters {
        if let Some(array) = formatter.as_array() {
            formatter_array = array.clone();
        } else {
            formatter_array.push(formatter.clone());
        }
    };
    let found_code_actions = !code_actions.is_empty();
    formatter_array.splice(
        0..0,
        code_actions
            .iter()
            .map(|code_action| serde_json::json!({"code_action": code_action})),
    );

    obj.remove("code_actions_on_format");
    if found_code_actions {
        obj.insert("formatter".to_string(), Value::Array(formatter_array));
    }

    Ok(code_actions)
}
