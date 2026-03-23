use std::ops::Range;

use anyhow::Result;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;

pub fn replace_edit_prediction_conflict_with_expanded_version(
    json: &mut serde_json::Value,
) -> Result<()> {
    let Some(bindings) = json.as_array_mut() else {
        anyhow::bail!("Expected an array of key bindings")
    };

    for binding in bindings {
        let Some(context) = binding.get_mut("context") else {
            continue;
        };
        let Some(context_str) = context.as_str() else {
            continue;
        };
        if !context_str.contains("edit_prediction_conflict") {
            continue;
        }
        *context = serde_json::Value::String(context_str.replace(
            "edit_prediction_conflict",
            "(edit_prediction && (showing_completions || in_leading_whitespace))",
        ));
    }

    return Ok(());
}

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
    let new_predicate = old_predicate.replace(
        "edit_prediction_conflict",
        "(edit_prediction && (showing_completions || in_leading_whitespace))",
    );

    if new_predicate != old_predicate {
        Some((context_predicate_range, new_predicate))
    } else {
        None
    }
}
