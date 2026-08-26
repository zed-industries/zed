use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_ASSISTANT_TOOLS_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns =
    &[(SETTINGS_ASSISTANT_TOOLS_PATTERN, rename_path_search_tool)];

fn rename_path_search_tool(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let tool_name_capture_ix = query.capture_index_for_name("tool_name")?;
    let tool_name_range = mat
        .nodes_for_capture_index(tool_name_capture_ix)
        .next()?
        .byte_range();
    let tool_name = contents.get(tool_name_range.clone())?;

    if tool_name == "path_search" {
        return Some((tool_name_range, "find_path".to_string()));
    }

    None
}
