use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::{MigrationPatterns, patterns::SETTINGS_ASSISTANT_PATTERN};

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(SETTINGS_ASSISTANT_PATTERN, rename_assistant)];

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
