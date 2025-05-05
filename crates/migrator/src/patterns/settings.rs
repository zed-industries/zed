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
)"#;
