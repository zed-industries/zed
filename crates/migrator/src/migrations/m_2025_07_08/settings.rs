use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_ROOT_KEY_VALUE_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(
    SETTINGS_ROOT_KEY_VALUE_PATTERN,
    migrate_drag_and_drop_selection,
)];

fn migrate_drag_and_drop_selection(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let name_ix = query.capture_index_for_name("name")?;
    let name_range = mat.nodes_for_capture_index(name_ix).next()?.byte_range();
    let name = contents.get(name_range)?;

    if name != "drag_and_drop_selection" {
        return None;
    }

    let value_ix = query.capture_index_for_name("value")?;
    let value_node = mat.nodes_for_capture_index(value_ix).next()?;
    let value_range = value_node.byte_range();
    let value = contents.get(value_range.clone())?;

    match value {
        "true" | "false" => {
            let replacement = format!("{{\n    \"enabled\": {}\n  }}", value);
            Some((value_range, replacement))
        }
        _ => None,
    }
}
