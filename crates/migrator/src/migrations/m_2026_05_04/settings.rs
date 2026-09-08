use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_ROOT_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns =
    &[(SETTINGS_ROOT_KEY_VALUE_PATTERN, replace_setting_value)];

fn replace_setting_value(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let setting_capture_ix = query.capture_index_for_name("name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_capture_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_name_range)?;

    if setting_name != "hide_mouse" {
        return None;
    }

    let value_capture_ix = query.capture_index_for_name("value")?;
    let value_range = mat
        .nodes_for_capture_index(value_capture_ix)
        .next()?
        .byte_range();
    let value = contents.get(value_range.clone())?;

    if value.trim() != "\"on_typing_and_movement\"" {
        return None;
    }

    Some((value_range, "\"on_typing_and_action\"".to_string()))
}
