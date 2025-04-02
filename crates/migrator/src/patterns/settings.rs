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
