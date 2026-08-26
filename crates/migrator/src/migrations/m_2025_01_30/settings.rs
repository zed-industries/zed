use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_NESTED_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[
    (
        SETTINGS_NESTED_KEY_VALUE_PATTERN,
        replace_tab_close_button_setting_key,
    ),
    (
        SETTINGS_NESTED_KEY_VALUE_PATTERN,
        replace_tab_close_button_setting_value,
    ),
];

fn replace_tab_close_button_setting_key(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let parent_object_capture_ix = query.capture_index_for_name("parent_key")?;
    let parent_object_range = mat
        .nodes_for_capture_index(parent_object_capture_ix)
        .next()?
        .byte_range();
    let parent_object_name = contents.get(parent_object_range)?;

    let setting_name_ix = query.capture_index_for_name("setting_name")?;
    let setting_range = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_range.clone())?;

    if parent_object_name == "tabs" && setting_name == "always_show_close_button" {
        return Some((setting_range, "show_close_button".into()));
    }

    None
}

fn replace_tab_close_button_setting_value(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let parent_object_capture_ix = query.capture_index_for_name("parent_key")?;
    let parent_object_range = mat
        .nodes_for_capture_index(parent_object_capture_ix)
        .next()?
        .byte_range();
    let parent_object_name = contents.get(parent_object_range)?;

    let setting_name_ix = query.capture_index_for_name("setting_name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_name_range)?;

    let setting_value_ix = query.capture_index_for_name("setting_value")?;
    let setting_value_range = mat
        .nodes_for_capture_index(setting_value_ix)
        .next()?
        .byte_range();
    let setting_value = contents.get(setting_value_range.clone())?;

    if parent_object_name == "tabs" && setting_name == "always_show_close_button" {
        match setting_value {
            "true" => {
                return Some((setting_value_range, "\"always\"".to_string()));
            }
            "false" => {
                return Some((setting_value_range, "\"hover\"".to_string()));
            }
            _ => {}
        }
    }

    None
}
