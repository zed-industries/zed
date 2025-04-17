use collections::HashMap;
use std::{ops::Range, sync::LazyLock};
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::KEYMAP_ACTION_STRING_PATTERN;

pub const KEYMAP_PATTERNS: MigrationPatterns =
    &[(KEYMAP_ACTION_STRING_PATTERN, replace_string_action)];

fn replace_string_action(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let action_name_node = mat.nodes_for_capture_index(action_name_ix).next()?;
    let action_name_range = action_name_node.byte_range();
    let action_name = contents.get(action_name_range.clone())?;

    if let Some(new_action_name) = STRING_REPLACE.get(&action_name) {
        return Some((action_name_range, new_action_name.to_string()));
    }

    None
}

/// "ctrl-k ctrl-1": "inline_completion::ToggleMenu" -> "edit_prediction::ToggleMenu"
static STRING_REPLACE: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([("outline_panel::Open", "outline_panel::OpenSelectedEntry")])
});
