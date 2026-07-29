use std::ops::Range;

use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

pub const KEYMAP_PATTERNS: MigrationPatterns =
    &[(crate::patterns::KEYMAP_CONTEXT_PATTERN, rename_context_key)];

fn rename_context_key(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let context_predicate_ix = query.capture_index_for_name("context_predicate")?;
    let context_predicate_range = mat
        .nodes_for_capture_index(context_predicate_ix)
        .next()?
        .byte_range();
    let old_predicate = contents.get(context_predicate_range.clone())?.to_string();
    let mut new_predicate = old_predicate.clone();

    const REPLACEMENTS: &[(&str, &str)] = &[
        (
            "edit_prediction_conflict && !showing_completions",
            "(edit_prediction && in_leading_whitespace)",
        ),
        (
            "edit_prediction_conflict && showing_completions",
            "(edit_prediction && showing_completions)",
        ),
        (
            "edit_prediction_conflict",
            "(edit_prediction && (showing_completions || in_leading_whitespace))",
        ),
    ];

    for (old, new) in REPLACEMENTS {
        new_predicate = new_predicate.replace(old, new);
    }

    if new_predicate != old_predicate {
        Some((context_predicate_range, new_predicate))
    } else {
        None
    }
}
