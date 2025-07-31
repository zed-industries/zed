use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::{MigrationPatterns, patterns::SETTINGS_DUPLICATED_AGENT_PATTERN};

pub const SETTINGS_PATTERNS: MigrationPatterns =
    &[(SETTINGS_DUPLICATED_AGENT_PATTERN, comment_duplicated_agent)];

fn comment_duplicated_agent(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let pair_ix = query.capture_index_for_name("pair1")?;
    let mut range = mat.nodes_for_capture_index(pair_ix).next()?.byte_range();

    // Include the comma into the commented region
    let rtext = &contents[range.end..];
    if let Some(comma_index) = rtext.find(',') {
        range.end += comma_index + 1;
    }

    let value = contents[range.clone()].to_string();
    let commented_value = format!("/* Duplicated key auto-commented: {value} */");
    Some((range, commented_value))
}
