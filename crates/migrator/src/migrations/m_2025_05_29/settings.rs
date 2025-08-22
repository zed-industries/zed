use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_NESTED_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_NESTED_KEY_VALUE_PATTERN,
    replace_preferred_completion_mode_value,
)];

fn replace_preferred_completion_mode_value(
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

    if parent_object_name != "agent" {
        return None;
    }

    let setting_name_capture_ix = query.capture_index_for_name("setting_name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_name_capture_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_name_range)?;

    if setting_name != "preferred_completion_mode" {
        return None;
    }

    let value_capture_ix = query.capture_index_for_name("setting_value")?;
    let value_range = mat
        .nodes_for_capture_index(value_capture_ix)
        .next()?
        .byte_range();
    let value = contents.get(value_range.clone())?;

    if value.trim() == "\"max\"" {
        Some((value_range, "\"burn\"".to_string()))
    } else {
        None
    }
}
