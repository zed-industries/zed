pub const SETTINGS_ROOT_KEY_VALUE_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @name)
            value: (_)  @value
        )
    )
)"#;

pub const SETTINGS_NESTED_KEY_VALUE_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @parent_key)
            value: (object
                (pair
                    key: (string (string_content) @setting_name)
                    value: (_) @setting_value
                )
            )
        )
    )
)"#;

pub const SETTINGS_LANGUAGES_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @languages)
            value: (object
            (pair
                key: (string)
                value: (object
                    (pair
                        key: (string (string_content) @setting_name)
                        value: (_) @value
                    )
                )
            ))
        )
    )
    (#eq? @languages "languages")
)"#;

pub const SETTINGS_ASSISTANT_TOOLS_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @assistant)
            value: (object
                (pair
                    key: (string (string_content) @profiles)
                    value: (object
                        (pair
                            key: (_)
                            value: (object
                                (pair
                                    key: (string (string_content) @tools_key)
                                    value: (object
                                        (pair
                                            key: (string (string_content) @tool_name)
                                            value: (_) @tool_value
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            )
        )
    )
    (#eq? @assistant "assistant")
    (#eq? @profiles "profiles")
    (#eq? @tools_key "tools")
)"#;

pub const SETTINGS_ASSISTANT_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @key)
        )
    )
    (#eq? @key "assistant")
)"#;

pub const SETTINGS_EDIT_PREDICTIONS_ASSISTANT_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @edit_predictions)
            value: (object
                (pair key: (string (string_content) @enabled_in_assistant))
            )
        )
    )
    (#eq? @edit_predictions "edit_predictions")
    (#eq? @enabled_in_assistant "enabled_in_assistant")
)"#;

pub const SETTINGS_DUPLICATED_AGENT_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @agent1)
            value: (_)
        ) @pair1
        (pair
            key: (string (string_content) @agent2)
            value: (_)
        )
    )
    (#eq? @agent1 "agent")
    (#eq? @agent2 "agent")
)"#;

/// Migrate language settings,
/// calls `migrate_fn` with the top level object as well as all language settings under the "languages" key
/// Fails early if `migrate_fn` returns an error at any point
pub fn migrate_language_setting(
    value: &mut serde_json::Value,
    migrate_fn: fn(&mut serde_json::Value, path: &[&str]) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    migrate_fn(value, &[])?;
    let languages = value
        .as_object_mut()
        .and_then(|obj| obj.get_mut("languages"))
        .and_then(|languages| languages.as_object_mut());
    if let Some(languages) = languages {
        for (language_name, language) in languages.iter_mut() {
            let path = vec!["languages", language_name];
            migrate_fn(language, &path)?;
        }
    }
    Ok(())
}
