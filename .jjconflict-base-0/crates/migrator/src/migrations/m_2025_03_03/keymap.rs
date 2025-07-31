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

    if let Some((new_action_name, options)) = STRING_TO_ARRAY_REPLACE.get(action_name) {
        let full_string_range = action_name_node.parent()?.byte_range();
        let mut options_parts = Vec::new();
        for (key, value) in options.iter() {
            options_parts.push(format!("\"{}\": {}", key, value));
        }
        let options_str = options_parts.join(", ");
        let replacement = format!("[\"{}\", {{ {} }}]", new_action_name, options_str);
        return Some((full_string_range, replacement));
    }

    None
}

static STRING_REPLACE: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        (
            "editor::GoToPrevDiagnostic",
            "editor::GoToPreviousDiagnostic",
        ),
        ("editor::ContextMenuPrev", "editor::ContextMenuPrevious"),
        ("search::SelectPrevMatch", "search::SelectPreviousMatch"),
        ("file_finder::SelectPrev", "file_finder::SelectPrevious"),
        ("menu::SelectPrev", "menu::SelectPrevious"),
        ("editor::TabPrev", "editor::Backtab"),
        ("pane::ActivatePrevItem", "pane::ActivatePreviousItem"),
        ("vim::MoveToPrev", "vim::MoveToPrevious"),
        ("vim::MoveToPrevMatch", "vim::MoveToPreviousMatch"),
    ])
});

/// "editor::GoToPrevHunk" -> ["editor::GoToPreviousHunk", { "center_cursor": true }]
static STRING_TO_ARRAY_REPLACE: LazyLock<HashMap<&str, (&str, HashMap<&str, bool>)>> =
    LazyLock::new(|| {
        HashMap::from_iter([
            (
                "editor::GoToHunk",
                (
                    "editor::GoToHunk",
                    HashMap::from_iter([("center_cursor", true)]),
                ),
            ),
            (
                "editor::GoToPrevHunk",
                (
                    "editor::GoToPreviousHunk",
                    HashMap::from_iter([("center_cursor", true)]),
                ),
            ),
        ])
    });
