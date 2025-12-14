use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_NESTED_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_NESTED_KEY_VALUE_PATTERN,
    rename_enable_preview_from_code_navigation_setting,
)];

fn rename_enable_preview_from_code_navigation_setting(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    if !is_enable_preview_from_code_navigation(contents, mat, query) {
        return None;
    }

    let setting_name_ix = query.capture_index_for_name("setting_name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()?
        .byte_range();

    Some((
        setting_name_range,
        "enable_keep_preview_on_code_navigation".to_string(),
    ))
}

fn is_enable_preview_from_code_navigation(contents: &str, mat: &QueryMatch, query: &Query) -> bool {
    let parent_key_ix = match query.capture_index_for_name("parent_key") {
        Some(ix) => ix,
        None => return false,
    };
    let parent_range = match mat.nodes_for_capture_index(parent_key_ix).next() {
        Some(node) => node.byte_range(),
        None => return false,
    };
    if contents.get(parent_range) != Some("preview_tabs") {
        return false;
    }

    let setting_name_ix = match query.capture_index_for_name("setting_name") {
        Some(ix) => ix,
        None => return false,
    };
    let setting_name_range = match mat.nodes_for_capture_index(setting_name_ix).next() {
        Some(node) => node.byte_range(),
        None => return false,
    };
    contents.get(setting_name_range) == Some("enable_preview_from_code_navigation")
}
