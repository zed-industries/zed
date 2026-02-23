use std::ops::Range;

use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_NESTED_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_NESTED_KEY_VALUE_PATTERN,
    migrate_diagnostics_include_warnings_to_max_severity,
)];

fn migrate_diagnostics_include_warnings_to_max_severity(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    if !is_diagnostics_include_warnings_setting(contents, mat, query) {
        return None;
    }

    let setting_name_ix = query.capture_index_for_name("setting_name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()?
        .byte_range();

    let setting_value_ix = query.capture_index_for_name("setting_value")?;
    let setting_value_range = mat
        .nodes_for_capture_index(setting_value_ix)
        .next()?
        .byte_range();
    let setting_value = contents.get(setting_value_range.clone())?;

    let max_severity_value = match setting_value.trim() {
        "true" => "\"warning\"",
        "false" => "\"error\"",
        _ => return None,
    };

    let replacement = format!("max_severity\": {max_severity_value}");
    Some((
        setting_name_range.start..setting_value_range.end,
        replacement,
    ))
}

fn is_diagnostics_include_warnings_setting(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> bool {
    let Some(parent_key_ix) = query.capture_index_for_name("parent_key") else {
        return false;
    };
    let Some(setting_name_ix) = query.capture_index_for_name("setting_name") else {
        return false;
    };

    let Some(parent_key_range) = mat
        .nodes_for_capture_index(parent_key_ix)
        .next()
        .map(|node| node.byte_range())
    else {
        return false;
    };

    let Some(setting_name_range) = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()
        .map(|node| node.byte_range())
    else {
        return false;
    };

    contents.get(parent_key_range) == Some("diagnostics")
        && contents.get(setting_name_range) == Some("include_warnings")
}
