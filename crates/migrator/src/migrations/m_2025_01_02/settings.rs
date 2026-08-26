use collections::HashMap;
use std::{ops::Range, sync::LazyLock};
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_NESTED_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_NESTED_KEY_VALUE_PATTERN,
    replace_deprecated_settings_values,
)];

fn replace_deprecated_settings_values(
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

    UPDATED_SETTINGS
        .get(&(parent_object_name, setting_name))
        .and_then(|new_values| {
            new_values
                .iter()
                .find_map(|(old_value, new_value)| {
                    (*old_value == setting_value).then(|| new_value.to_string())
                })
                .map(|new_value| (setting_value_range, new_value))
        })
}

static UPDATED_SETTINGS: LazyLock<HashMap<(&str, &str), Vec<(&str, &str)>>> = LazyLock::new(|| {
    HashMap::from_iter([
        (
            ("chat_panel", "button"),
            vec![("true", "\"always\""), ("false", "\"never\"")],
        ),
        (
            ("scrollbar", "diagnostics"),
            vec![("true", "\"all\""), ("false", "\"none\"")],
        ),
    ])
});
