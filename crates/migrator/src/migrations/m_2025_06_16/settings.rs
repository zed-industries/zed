use std::ops::Range;

use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[
    (
        SETTINGS_CUSTOM_CONTEXT_SERVER_PATTERN,
        migrate_custom_context_server_settings,
    ),
    (
        SETTINGS_EXTENSION_CONTEXT_SERVER_PATTERN,
        migrate_extension_context_server_settings,
    ),
    (
        SETTINGS_EMPTY_CONTEXT_SERVER_PATTERN,
        migrate_empty_context_server_settings,
    ),
];

const SETTINGS_CUSTOM_CONTEXT_SERVER_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @context-servers)
            value: (object
                (pair
                    key: (string)
                    value: (object
                        (pair
                            key: (string (string_content) @previous-key)
                            value: (object)
                        )*
                        (pair
                            key: (string (string_content) @key)
                            value: (object)
                        )
                        (pair
                            key: (string (string_content) @next-key)
                            value: (object)
                        )*
                    ) @server-settings
                )
            )
        )
    )
    (#eq? @context-servers "context_servers")
    (#eq? @key "command")
    (#not-eq? @previous-key "source")
    (#not-eq? @next-key "source")
)"#;

fn migrate_custom_context_server_settings(
    _contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let server_settings_index = query.capture_index_for_name("server-settings")?;
    let server_settings = mat.nodes_for_capture_index(server_settings_index).next()?;
    // Move forward 1 to get inside the object
    let start = server_settings.start_byte() + 1;

    Some((
        start..start,
        r#"
            "source": "custom","#
            .to_string(),
    ))
}

const SETTINGS_EXTENSION_CONTEXT_SERVER_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @context-servers)
            value: (object
                (pair
                    key: (string)
                    value: (object
                        (pair
                            key: (string (string_content) @previous-key)
                            value: (object)
                        )*
                        (pair
                            key: (string (string_content) @key)
                            value: (object)
                        )
                        (pair
                            key: (string (string_content) @next-key)
                            value: (object)
                        )*
                    ) @server-settings
                )
            )
        )
    )
    (#eq? @context-servers "context_servers")
    (#eq? @key "settings")
    (#not-match? @previous-key "^command|source$")
    (#not-match? @next-key "^command|source$")
)"#;

fn migrate_extension_context_server_settings(
    _contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let server_settings_index = query.capture_index_for_name("server-settings")?;
    let server_settings = mat.nodes_for_capture_index(server_settings_index).next()?;
    // Move forward 1 to get inside the object
    let start = server_settings.start_byte() + 1;

    Some((
        start..start,
        r#"
            "source": "extension","#
            .to_string(),
    ))
}

const SETTINGS_EMPTY_CONTEXT_SERVER_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @context-servers)
            value: (object
                (pair
                    key: (string)
                    value: (object) @server-settings
                )
            )
        )
    )
    (#eq? @context-servers "context_servers")
    (#eq? @server-settings "{}")
)"#;

fn migrate_empty_context_server_settings(
    _contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let server_settings_index = query.capture_index_for_name("server-settings")?;
    let server_settings = mat.nodes_for_capture_index(server_settings_index).next()?;

    Some((
        server_settings.byte_range(),
        r#"{
            "source": "extension",
            "settings": {}
        }"#
        .to_string(),
    ))
}
