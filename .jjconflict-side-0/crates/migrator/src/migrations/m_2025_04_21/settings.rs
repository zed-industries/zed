use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_ASSISTANT_TOOLS_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns =
    &[(SETTINGS_ASSISTANT_TOOLS_PATTERN, rename_tools)];

fn rename_tools(contents: &str, mat: &QueryMatch, query: &Query) -> Option<(Range<usize>, String)> {
    let tool_name_capture_ix = query.capture_index_for_name("tool_name")?;
    let tool_name_range = mat
        .nodes_for_capture_index(tool_name_capture_ix)
        .next()?
        .byte_range();
    let tool_name = contents.get(tool_name_range.clone())?;

    let new_name = match tool_name {
        "find_replace_file" => "edit_file",
        "regex_search" => "grep",
        _ => return None,
    };

    Some((tool_name_range, new_name.to_string()))
}
