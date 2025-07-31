pub const KEYMAP_ACTION_ARRAY_PATTERN: &str = r#"(document
    (array
   	    (object
            (pair
                key: (string (string_content) @name)
                value: (
                    (object
                        (pair
                            key: (string)
                            value: ((array
                                . (string (string_content) @action_name)
                                . (string (string_content) @argument)
                                .)) @array
                        )
                    )
                )
            )
        )
    )
    (#eq? @name "bindings")
)"#;

pub const KEYMAP_ACTION_STRING_PATTERN: &str = r#"(document
    (array
        (object
            (pair
                key: (string (string_content) @name)
                value: (
                    (object
                        (pair
                            key: (string)
                            value: (string (string_content) @action_name)
                        )
                    )
                )
            )
        )
    )
    (#eq? @name "bindings")
)"#;

pub const KEYMAP_CONTEXT_PATTERN: &str = r#"(document
    (array
        (object
            (pair
                key: (string (string_content) @name)
                value: (string (string_content) @context_predicate)
            )
        )
    )
    (#eq? @name "context")
)"#;

pub const KEYMAP_ACTION_ARRAY_ARGUMENT_AS_OBJECT_PATTERN: &str = r#"(document
    (array
        (object
            (pair
                key: (string (string_content) @name)
                value: (
                    (object
                        (pair
                            key: (string)
                            value: ((array
                                . (string (string_content) @action_name)
                                . (object
                                    (pair
                                    key: (string (string_content) @argument_key)
                                    value: (_)  @argument_value))
                                . ) @array
                            ))
                        )
                    )
                )
            )
        )
    (#eq? @name "bindings")
)"#;
