use std::ops::Range;

use tree_sitter::{Query, QueryMatch};

use crate::{patterns::SETTINGS_NESTED_KEY_VALUE_PATTERN, MigrationPatterns};

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_NESTED_KEY_VALUE_PATTERN,
    replace_hunk_style_setting_key,
)];

fn replace_hunk_style_setting_key(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let parent_object_capture_ix = query.capture_index_for_name("parent_key")?;
    let parent_object_range = mat
        .nodes_for_capture_index(parent_object_capture_ix)
        .next()?
        .byte_range();
    let parent_object_name = contents.get(parent_object_range.clone())?;

    let setting_name_ix = query.capture_index_for_name("setting_name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_name_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_name_range.clone())?;

    let setting_value_ix = query.capture_index_for_name("setting_value")?;
    let setting_value_range = mat
        .nodes_for_capture_index(setting_value_ix)
        .next()?
        .byte_range();
    let setting_value = contents.get(setting_value_range.clone())?;

    if parent_object_name == "git" && setting_name == "hunk_style" {
        match setting_value {
            "\"transparent\"" | "\"pattern\"" | "\"border\"" => {
                return Some((setting_value_range, "\"unstaged_hollow\"".to_string()))
            }
            "\"staged_transparent\"" | "\"staged_pattern\"" | "\"staged_border\"" => {
                return Some((setting_value_range, "\"staged_hollow\"".to_string()))
            }
            _ => {}
        }
    }

    None
}
