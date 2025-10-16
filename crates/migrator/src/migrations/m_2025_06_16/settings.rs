use std::ops::Range;

use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_CONTEXT_SERVER_PATTERN,
    migrate_context_server_settings,
)];

const SETTINGS_CONTEXT_SERVER_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @context-servers)
            value: (object
                (pair
                    key: (string (string_content) @server-name)
                    value: (object) @server-settings
                )
            )
        )
    )
    (#eq? @context-servers "context_servers")
)"#;

fn migrate_context_server_settings(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let server_settings_index = query.capture_index_for_name("server-settings")?;
    let server_settings = mat.nodes_for_capture_index(server_settings_index).next()?;

    let mut has_command = false;
    let mut has_settings = false;
    let mut other_keys = 0;
    let mut column = None;

    // Parse the server settings to check what keys it contains
    let mut cursor = server_settings.walk();
    for child in server_settings.children(&mut cursor) {
        if child.kind() == "pair"
            && let Some(key_node) = child.child_by_field_name("key")
        {
            if let (None, Some(quote_content)) = (column, key_node.child(0)) {
                column = Some(quote_content.start_position().column);
            }
            if let Some(string_content) = key_node.child(1) {
                let key = &contents[string_content.byte_range()];
                match key {
                    // If it already has a source key, don't modify it
                    "source" => return None,
                    "command" => has_command = true,
                    "settings" => has_settings = true,
                    _ => other_keys += 1,
                }
            }
        }
    }

    let source_type = if has_command { "custom" } else { "extension" };

    // Insert the source key at the beginning of the object
    let start = server_settings.start_byte() + 1;
    let indent = " ".repeat(column.unwrap_or(12));

    if !has_command && !has_settings {
        return Some((
            start..start,
            format!(
                r#"
{indent}"source": "{}",
{indent}"settings": {{}}{}
        "#,
                source_type,
                if other_keys > 0 { "," } else { "" }
            ),
        ));
    }

    Some((
        start..start,
        format!(
            r#"
{indent}"source": "{}","#,
            source_type
        ),
    ))
}
