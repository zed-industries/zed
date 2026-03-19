use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_NESTED_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_NESTED_KEY_VALUE_PATTERN,
    rename_restore_on_startup_values,
)];

fn rename_restore_on_startup_values(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    if !is_restore_on_startup_setting(contents, mat, query) {
        return None;
    }

    let setting_value_ix = query.capture_index_for_name("setting_value")?;
    let setting_value_range = mat
        .nodes_for_capture_index(setting_value_ix)
        .next()?
        .byte_range();
    let setting_value = contents.get(setting_value_range.clone())?;

    // The value includes quotes, so we check for the quoted string
    let new_value = match setting_value.trim() {
        "\"none\"" => "\"empty_tab\"",
        "\"welcome\"" => "\"launchpad\"",
        _ => return None,
    };

    Some((setting_value_range, new_value.to_string()))
}

fn is_restore_on_startup_setting(contents: &str, mat: &QueryMatch, query: &Query) -> bool {
    // Check that the parent key is "workspace" (since restore_on_startup is under workspace settings)
    // Actually, restore_on_startup can be at the root level too, so we need to handle both cases
    // The SETTINGS_NESTED_KEY_VALUE_PATTERN captures parent_key and setting_name

    let setting_name_ix = match query.capture_index_for_name("setting_name") {
        Some(ix) => ix,
        None => return false,
    };
    let setting_name_range = match mat.nodes_for_capture_index(setting_name_ix).next() {
        Some(node) => node.byte_range(),
        None => return false,
    };
    contents.get(setting_name_range) == Some("restore_on_startup")
}
