use collections::HashSet;
use std::{ops::Range, sync::LazyLock};
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::KEYMAP_ACTION_ARRAY_ARGUMENT_AS_OBJECT_PATTERN;

pub const KEYMAP_PATTERNS: MigrationPatterns = &[(
    KEYMAP_ACTION_ARRAY_ARGUMENT_AS_OBJECT_PATTERN,
    replace_array_with_single_string,
)];

fn replace_array_with_single_string(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let array_ix = query.capture_index_for_name("array")?;
    let action_name_ix = query.capture_index_for_name("action_name")?;

    let action_name = contents.get(
        mat.nodes_for_capture_index(action_name_ix)
            .next()?
            .byte_range(),
    )?;

    if TRANSFORM_ARRAY.contains(&action_name) {
        let replacement_as_string = format!("\"{action_name}\"");
        let range_to_replace = mat.nodes_for_capture_index(array_ix).next()?.byte_range();
        return Some((range_to_replace, replacement_as_string));
    }

    None
}

/// ["editor::GoToPreviousHunk", { "center_cursor": true }] -> "editor::GoToPreviousHunk"
static TRANSFORM_ARRAY: LazyLock<HashSet<&str>> =
    LazyLock::new(|| HashSet::from_iter(["editor::GoToHunk", "editor::GoToPreviousHunk"]));
