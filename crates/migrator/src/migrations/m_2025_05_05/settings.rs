use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::{MigrationPatterns, patterns::SETTINGS_ASSISTANT_PATTERN};

pub const SETTINGS_PATTERNS: MigrationPatterns = &[(SETTINGS_ASSISTANT_PATTERN, rename_assistant)];

fn rename_assistant(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let key_capture_ix = query.capture_index_for_name("key")?;
    let key_range = mat
        .nodes_for_capture_index(key_capture_ix)
        .next()?
        .byte_range();
    let key = contents.get(key_range.clone())?;
    if dbg!(key) != "assistant" {
        return None;
    }
    return Some((key_range, "agent".to_string()));
}
