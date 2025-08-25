use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[
    (SETTINGS_VERSION_PATTERN, remove_version_fields),
    (
        SETTINGS_NESTED_VERSION_PATTERN,
        remove_nested_version_fields,
    ),
];

const SETTINGS_VERSION_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @key)
            value: (object
                (pair
                    key: (string (string_content) @version_key)
                    value: (_) @version_value
                ) @version_pair
            )
        )
    )
    (#eq? @key "agent")
    (#eq? @version_key "version")
)"#;

const SETTINGS_NESTED_VERSION_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @language_models)
            value: (object
                (pair
                    key: (string (string_content) @provider)
                    value: (object
                        (pair
                            key: (string (string_content) @version_key)
                            value: (_) @version_value
                        ) @version_pair
                    )
                )
            )
        )
    )
    (#eq? @language_models "language_models")
    (#match? @provider "^(anthropic|openai)$")
    (#eq? @version_key "version")
)"#;

fn remove_version_fields(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let version_pair_ix = query.capture_index_for_name("version_pair")?;
    let version_pair_node = mat.nodes_for_capture_index(version_pair_ix).next()?;

    remove_pair_with_whitespace(contents, version_pair_node)
}

fn remove_nested_version_fields(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let version_pair_ix = query.capture_index_for_name("version_pair")?;
    let version_pair_node = mat.nodes_for_capture_index(version_pair_ix).next()?;

    remove_pair_with_whitespace(contents, version_pair_node)
}

fn remove_pair_with_whitespace(
    contents: &str,
    pair_node: tree_sitter::Node,
) -> Option<(Range<usize>, String)> {
    let mut range_to_remove = pair_node.byte_range();

    // Check if there's a comma after this pair
    if let Some(next_sibling) = pair_node.next_sibling() {
        if next_sibling.kind() == "," {
            range_to_remove.end = next_sibling.end_byte();
        }
    } else {
        // If no next sibling, check if there's a comma before
        if let Some(prev_sibling) = pair_node.prev_sibling()
            && prev_sibling.kind() == ","
        {
            range_to_remove.start = prev_sibling.start_byte();
        }
    }

    // Include any leading whitespace/newline, including comments
    let text_before = &contents[..range_to_remove.start];
    if let Some(last_newline) = text_before.rfind('\n') {
        let whitespace_start = last_newline + 1;
        let potential_whitespace = &contents[whitespace_start..range_to_remove.start];

        // Check if it's only whitespace or comments
        let mut is_whitespace_or_comment = true;
        let mut in_comment = false;
        let mut chars = potential_whitespace.chars().peekable();

        while let Some(ch) = chars.next() {
            if in_comment {
                if ch == '\n' {
                    in_comment = false;
                }
            } else if ch == '/' && chars.peek() == Some(&'/') {
                in_comment = true;
                chars.next(); // Skip the second '/'
            } else if !ch.is_whitespace() {
                is_whitespace_or_comment = false;
                break;
            }
        }

        if is_whitespace_or_comment {
            range_to_remove.start = whitespace_start;
        }
    }

    // Also check if we need to include trailing whitespace up to the next line
    let text_after = &contents[range_to_remove.end..];
    if let Some(newline_pos) = text_after.find('\n')
        && text_after[..newline_pos].chars().all(|c| c.is_whitespace())
    {
        range_to_remove.end += newline_pos + 1;
    }

    Some((range_to_remove, String::new()))
}
