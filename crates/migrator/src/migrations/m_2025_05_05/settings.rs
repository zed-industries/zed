use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::{
    MigrationPatterns, patterns::SETTINGS_ASSISTANT_PATTERN,
    patterns::SETTINGS_EDIT_PREDICTIONS_ASSISTANT_PATTERN,
};

pub const SETTINGS_PATTERNS: MigrationPatterns = &[
    (SETTINGS_ASSISTANT_PATTERN, rename_assistant),
    (
        SETTINGS_EDIT_PREDICTIONS_ASSISTANT_PATTERN,
        rename_edit_prediction_assistant,
    ),
];

fn rename_assistant(
    _contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let key_capture_ix = query.capture_index_for_name("key")?;
    let key_range = mat
        .nodes_for_capture_index(key_capture_ix)
        .next()?
        .byte_range();
    Some((key_range, "agent".to_string()))
}

fn rename_edit_prediction_assistant(
    _contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let key_capture_ix = query.capture_index_for_name("enabled_in_assistant")?;
    let key_range = mat
        .nodes_for_capture_index(key_capture_ix)
        .next()?
        .byte_range();
    Some((key_range, "enabled_in_text_threads".to_string()))
}
