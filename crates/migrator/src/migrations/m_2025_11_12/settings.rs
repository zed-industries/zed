use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_NESTED_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[
    (
        SETTINGS_NESTED_KEY_VALUE_PATTERN,
        rename_open_file_on_paste_setting,
    ),
    (
        SETTINGS_NESTED_KEY_VALUE_PATTERN,
        replace_open_file_on_paste_setting_value,
    ),
];

fn rename_open_file_on_paste_setting(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    if !is_project_panel_open_file_on_paste(contents, mat, query) {
        return None;
    }

    let setting_name_ix = query.capture_index_for_name("setting_name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()?
        .byte_range();

    Some((setting_name_range, "auto_open".to_string()))
}

fn replace_open_file_on_paste_setting_value(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    if !is_project_panel_open_file_on_paste(contents, mat, query) {
        return None;
    }

    let value_ix = query.capture_index_for_name("setting_value")?;
    let value_node = mat.nodes_for_capture_index(value_ix).next()?;
    let value_range = value_node.byte_range();
    let value_text = contents.get(value_range.clone())?.trim();

    let normalized_value = match value_text {
        "true" => "true",
        "false" => "false",
        _ => return None,
    };

    Some((
        value_range,
        format!("{{ \"on_paste\": {normalized_value} }}"),
    ))
}

fn is_project_panel_open_file_on_paste(contents: &str, mat: &QueryMatch, query: &Query) -> bool {
    let parent_key_ix = match query.capture_index_for_name("parent_key") {
        Some(ix) => ix,
        None => return false,
    };
    let parent_range = match mat.nodes_for_capture_index(parent_key_ix).next() {
        Some(node) => node.byte_range(),
        None => return false,
    };
    if contents.get(parent_range) != Some("project_panel") {
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
    contents.get(setting_name_range) == Some("open_file_on_paste")
}
