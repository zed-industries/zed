use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_ROOT_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns =
    &[(SETTINGS_ROOT_KEY_VALUE_PATTERN, rename_agent_font_size)];

/// Renames the setting `agent_font_size` to `agent_ui_font_size`
fn rename_agent_font_size(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let setting_capture_ix = query.capture_index_for_name("name")?;

    let setting_name_range = mat
        .nodes_for_capture_index(setting_capture_ix)
        .next()?
        .byte_range();

    let setting_name = contents.get(setting_name_range.clone())?;

    if setting_name != "agent_font_size" {
        return None;
    }

    Some((setting_name_range, "agent_ui_font_size".to_string()))
}
