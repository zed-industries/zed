use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_NESTED_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_NESTED_KEY_VALUE_PATTERN,
    rename_show_branch_icon_to_show_branch_status_icon,
)];

fn rename_show_branch_icon_to_show_branch_status_icon(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let parent_key_ix = query.capture_index_for_name("parent_key")?;
    let parent_range = mat
        .nodes_for_capture_index(parent_key_ix)
        .next()?
        .byte_range();
    if contents.get(parent_range) != Some("title_bar") {
        return None;
    }

    let setting_name_ix = query.capture_index_for_name("setting_name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()?
        .byte_range();
    if contents.get(setting_name_range.clone()) != Some("show_branch_icon") {
        return None;
    }

    Some((setting_name_range, "show_branch_status_icon".to_string()))
}
