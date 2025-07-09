use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_DRAG_AND_DROP_SELECTION_PATTERN,
    add_delay_to_drag_and_drop_selection,
)];

const SETTINGS_DRAG_AND_DROP_SELECTION_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @key)
            value: [(true) (false)] @value
        )
    )
    (#eq? @key "drag_and_drop_selection")
)"#;

fn add_delay_to_drag_and_drop_selection(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let enabled_index = query.capture_index_for_name("value")?;
    let enabled_node = mat.nodes_for_capture_index(enabled_index).next()?;
    let enabled_value = &contents[enabled_node.byte_range()];

    // Find the pair node to get the proper indentation and range
    let mut current_node = enabled_node.parent()?;
    while current_node.kind() != "pair" {
        current_node = current_node.parent()?;
    }

    // Get the proper indentation from the pair
    let pair_start = current_node.start_byte();
    let line_start = contents[..pair_start]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);
    let indent = &contents[line_start..pair_start];

    // Build the replacement string
    let mut replacement = String::from("\"drag_and_drop_selection\": {\n");
    replacement.push_str(indent);
    replacement.push_str(&format!("  \"enabled\": {},\n", enabled_value));
    replacement.push_str(indent);
    replacement.push_str("  \"delay\": 300\n");
    replacement.push_str(indent);
    replacement.push_str("}");

    let range_to_replace = current_node.byte_range();
    Some((range_to_replace, replacement))
}
