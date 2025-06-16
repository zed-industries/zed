use std::ops::Range;

use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[
    (
        SETTINGS_EXTENSION_CONTEXT_SERVER_PATTERN,
        migrate_extension_context_server_settings,
    ),
    (
        SETTINGS_EMPTY_CONTEXT_SERVER_PATTERN,
        migrate_empty_context_server_settings,
    ),
];

const SETTINGS_EXTENSION_CONTEXT_SERVER_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @context_servers)
            value: (object
                (pair
                    key: (string)
                    value: (object
                        (pair
                            key: (string (string_content) @key)
                            value: (object)
                        )
                    ) @server_settings
                )
            )
        )
    )
    (#eq? @context_servers "context_servers")
    (#eq? @key "settings")
)"#;

fn migrate_extension_context_server_settings(
    _contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let server_settings_index = query.capture_index_for_name("server_settings")?;
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
            key: (string (string_content) @context_servers)
            value: (object
                (pair
                    key: (string)
                    value: (object) @server_settings
                )
            )
        )
    )
    (#eq? @context_servers "context_servers")
    (#eq? @server_settings "{}")
)"#;

fn migrate_empty_context_server_settings(
    _contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let server_settings_index = query.capture_index_for_name("server_settings")?;
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
