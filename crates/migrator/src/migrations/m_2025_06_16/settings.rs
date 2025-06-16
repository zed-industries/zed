use std::ops::Range;

use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

const SETTINGS_EMPTY_CONTEXT_SERVER_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @context_servers)
            value: (object
                (pair
                    key: (string (string_content) @server_name)
                    value: (object) @server_settings
                )
            )
        )
    )
    (#eq? @context_servers "context_servers")
    (#eq? @server_settings "{}")
)"#;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_EMPTY_CONTEXT_SERVER_PATTERN,
    migrate_empty_context_server_settings,
)];

fn migrate_empty_context_server_settings(
    _contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let server_setting_index = query.capture_index_for_name("server_settings")?;
    let server_setting_range = mat
        .nodes_for_capture_index(server_setting_index)
        .next()?
        .byte_range();

    Some((
        server_setting_range,
        r#"{
            "source": "extension",
            "settings": {}
        }"#
        .to_string(),
    ))
}
