use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::{MigrationPatterns, patterns::SETTINGS_DUPLICATED_AGENT_PATTERN};

pub const SETTINGS_PATTERNS: MigrationPatterns =
    &[(SETTINGS_DUPLICATED_AGENT_PATTERN, comment_duplicated_agent)];

fn comment_duplicated_agent(
    _contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let pair_ix = query.capture_index_for_name("pair1")?;
    let pair_range = mat.nodes_for_capture_index(pair_ix).next()?.byte_range();

    let value = _contents[pair_range.clone()].to_string();
    let commented_value = format!("/* Duplicated key auto-commented: {value} */");
    Some((pair_range, commented_value))
}
