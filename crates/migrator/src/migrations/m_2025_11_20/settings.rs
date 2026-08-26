use std::ops::Range;

use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_AGENT_SERVERS_CUSTOM_PATTERN,
    migrate_custom_agent_settings,
)];

const SETTINGS_AGENT_SERVERS_CUSTOM_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @agent-servers)
            value: (object
                (pair
                    key: (string (string_content) @server-name)
                    value: (object) @server-settings
                )
            )
        )
    )
    (#eq? @agent-servers "agent_servers")
)"#;

fn migrate_custom_agent_settings(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let server_name_index = query.capture_index_for_name("server-name")?;
    let server_name = mat.nodes_for_capture_index(server_name_index).next()?;
    let server_name_text = &contents[server_name.byte_range()];

    if matches!(server_name_text, "gemini" | "claude" | "codex") {
        return None;
    }

    let server_settings_index = query.capture_index_for_name("server-settings")?;
    let server_settings = mat.nodes_for_capture_index(server_settings_index).next()?;

    let mut column = None;

    // Parse the server settings to check what keys it contains
    let mut cursor = server_settings.walk();
    for child in server_settings.children(&mut cursor) {
        if child.kind() == "pair" {
            if let Some(key_node) = child.child_by_field_name("key") {
                if let (None, Some(quote_content)) = (column, key_node.child(0)) {
                    column = Some(quote_content.start_position().column);
                }
                if let Some(string_content) = key_node.child(1) {
                    let key = &contents[string_content.byte_range()];
                    match key {
                        // If it already has a type key, don't modify it
                        "type" => return None,
                        _ => {}
                    }
                }
            }
        }
    }

    // Insert the type key at the beginning of the object
    let start = server_settings.start_byte() + 1;
    let indent = " ".repeat(column.unwrap_or(12));

    Some((
        start..start,
        format!(
            r#"
{indent}"type": "custom","#
        ),
    ))
}
