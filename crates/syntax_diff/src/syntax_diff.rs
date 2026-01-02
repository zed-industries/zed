//! AST-aware diffing for syntax trees.

mod syntax_graph;
mod syntax_tree;

use collections::FxHashMap;
use language::DiffOptions;
pub use syntax_graph::{SyntaxEdge, SyntaxPath, SyntaxVertex};
pub use syntax_tree::{SyntaxId, SyntaxNode, SyntaxTree, SyntaxTreeCursor, build_tree};

use std::ops::Range;

use crate::{syntax_graph::ExceededGraphLimit, syntax_tree::SyntaxHint};

/// Default graph limit (1 million vertices).
///
/// Difftastic uses a higher value: https://github.com/Wilfred/difftastic/blob/cba6cc5d5a0b47b36fdb028a87af03c89d1908b4/src/options.rs#L25
pub const DEFAULT_GRAPH_LIMIT: usize = 1_000_000;

/// The kind of change for a syntax node.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SyntaxChange {
    /// This node is unchanged. The associated ID is the corresponding
    /// node in the opposite tree.
    Unchanged(SyntaxId),
    /// This node was replaced with another node.
    Replaced(SyntaxId, SyntaxId),
    /// This node is novel (added or removed).
    Novel,
}

/// Result of a syntax diff operation.
///
/// Ranges are absolute byte positions in the original source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxDiff {
    /// Absolute byte ranges in the LHS that are novel (removed/changed).
    pub lhs_ranges: Vec<Range<usize>>,
    /// Absolute byte ranges in the RHS that are novel (added/changed).
    pub rhs_ranges: Vec<Range<usize>>,
}

impl SyntaxDiff {
    /// Adjusts absolute byte ranges to be relative to the given offsets.
    ///
    /// This filters out ranges that are entirely before their respective offset
    /// (e.g., syntax nodes outside the hunk's range) and shifts the remaining
    /// ranges so they start from 0 relative to the offset.
    pub fn relative_to(self, lhs_offset: usize, rhs_offset: usize) -> Self {
        Self {
            lhs_ranges: adjust_ranges(self.lhs_ranges, lhs_offset),
            rhs_ranges: adjust_ranges(self.rhs_ranges, rhs_offset),
        }
    }

    /// Clips ranges to the given bounds, removing ranges outside and trimming
    /// ranges that partially overlap.
    ///
    /// This is necessary because `descendant_for_byte_range` may return an AST node
    /// larger than the hunk (e.g., an entire function when multiple hunks exist within it).
    /// The diff produces ranges for the full node, so we clip them to the hunk's boundaries.
    pub fn bound_to(self, lhs_bounds: Range<usize>, rhs_bounds: Range<usize>) -> Self {
        Self {
            lhs_ranges: clip_ranges_to_bounds(self.lhs_ranges, lhs_bounds),
            rhs_ranges: clip_ranges_to_bounds(self.rhs_ranges, rhs_bounds),
        }
    }
}

/// Compute a syntax-aware diff between two `SyntaxTree`s.
pub fn diff_trees(
    lhs_tree: &SyntaxTree,
    rhs_tree: &SyntaxTree,
) -> Result<SyntaxDiff, ExceededGraphLimit> {
    let route = syntax_graph::shortest_path(lhs_tree, rhs_tree, DEFAULT_GRAPH_LIMIT)?;

    let mut lhs_change_map = FxHashMap::with_capacity_and_hasher(route.0.len(), Default::default());
    let mut rhs_change_map = FxHashMap::with_capacity_and_hasher(route.0.len(), Default::default());

    for path in route.0 {
        let Some(edge) = path.edge else { continue };
        let Some(vertex) = path.from.as_ref() else {
            continue;
        };

        match edge {
            SyntaxEdge::Replaced { levenshtein_pct } => {
                if let (Some(lhs_id), Some(rhs_id)) = (vertex.lhs.id(), vertex.rhs.id()) {
                    if levenshtein_pct > 20 {
                        lhs_change_map.insert(lhs_id, SyntaxChange::Replaced(lhs_id, rhs_id));
                        rhs_change_map.insert(rhs_id, SyntaxChange::Replaced(lhs_id, rhs_id));
                    } else {
                        lhs_change_map.insert(lhs_id, SyntaxChange::Novel);
                        rhs_change_map.insert(rhs_id, SyntaxChange::Novel);
                    }
                }
            }
            SyntaxEdge::NovelAtomLHS | SyntaxEdge::EnterNovelDelimiterLHS => {
                if let Some(lhs_id) = vertex.lhs.id() {
                    lhs_change_map.insert(lhs_id, SyntaxChange::Novel);
                }
            }
            SyntaxEdge::NovelAtomRHS | SyntaxEdge::EnterNovelDelimiterRHS => {
                if let Some(rhs_id) = vertex.rhs.id() {
                    rhs_change_map.insert(rhs_id, SyntaxChange::Novel);
                }
            }
            _ => {}
        }
    }

    let mut lhs_ranges = collect_novel_ranges(lhs_tree, &lhs_change_map);
    let mut rhs_ranges = collect_novel_ranges(rhs_tree, &rhs_change_map);

    let (lhs_replace_ranges, rhs_replace_ranges) =
        collect_replace_ranges(lhs_tree, rhs_tree, &lhs_change_map);
    lhs_ranges.extend(lhs_replace_ranges);
    rhs_ranges.extend(rhs_replace_ranges);

    Ok(SyntaxDiff {
        lhs_ranges: merge_ranges(lhs_ranges),
        rhs_ranges: merge_ranges(rhs_ranges),
    })
}

fn collect_novel_ranges(
    tree: &SyntaxTree,
    change_map: &FxHashMap<SyntaxId, SyntaxChange>,
) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();

    for (id, change) in change_map.iter() {
        if let SyntaxChange::Novel = change {
            let node = tree.get(*id);

            if node.is_atom() {
                ranges.push(node.byte_range.clone());
            } else {
                if let Some(open_delimiter_range) = node.open_delimiter_range() {
                    ranges.push(open_delimiter_range);
                }

                if let Some(close_delimiter_range) = node.close_delimiter_range() {
                    ranges.push(close_delimiter_range);
                }
            }
        }
    }

    ranges
}

fn collect_replace_ranges(
    lhs_tree: &SyntaxTree,
    rhs_tree: &SyntaxTree,
    lhs_change_map: &FxHashMap<SyntaxId, SyntaxChange>,
) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    let mut lhs_ranges = Vec::new();
    let mut rhs_ranges = Vec::new();

    for (lhs_id, change) in lhs_change_map.iter() {
        if let SyntaxChange::Replaced(_, rhs_id) = change {
            let lhs_node = lhs_tree.get(*lhs_id);
            let rhs_node = rhs_tree.get(*rhs_id);

            if let (
                Some(SyntaxHint::Comment(lhs_comment)),
                Some(SyntaxHint::Comment(rhs_comment)),
            ) = (lhs_node.hint.as_ref(), rhs_node.hint.as_ref())
            {
                let (lhs_word_ranges, rhs_word_ranges) =
                    language::word_diff_ranges(lhs_comment, rhs_comment, DiffOptions::default());

                // Convert relative ranges to absolute byte positions
                let lhs_offset = lhs_node.byte_range.start;
                let rhs_offset = rhs_node.byte_range.start;

                lhs_ranges.extend(
                    lhs_word_ranges
                        .into_iter()
                        .map(|r| (r.start + lhs_offset)..(r.end + lhs_offset)),
                );
                rhs_ranges.extend(
                    rhs_word_ranges
                        .into_iter()
                        .map(|r| (r.start + rhs_offset)..(r.end + rhs_offset)),
                );
            }
        }
    }

    (lhs_ranges, rhs_ranges)
}

fn merge_ranges(mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| r.start);
    let mut merged = vec![ranges[0].clone()];

    for range in ranges.into_iter().skip(1) {
        let last = merged.last_mut().expect("merged is non-empty");
        if range.start <= last.end {
            last.end = last.end.max(range.end);
        } else {
            merged.push(range);
        }
    }

    merged
}

fn adjust_ranges(ranges: Vec<Range<usize>>, offset: usize) -> Vec<Range<usize>> {
    ranges
        .into_iter()
        .filter_map(|range| {
            if range.end <= offset {
                return None;
            }
            let start = range.start.saturating_sub(offset);
            let end = range.end - offset;
            Some(start..end)
        })
        .collect()
}

fn clip_ranges_to_bounds(ranges: Vec<Range<usize>>, bounds: Range<usize>) -> Vec<Range<usize>> {
    ranges
        .into_iter()
        .filter_map(|range| {
            if range.end <= bounds.start || range.start >= bounds.end {
                return None;
            }

            Some(range.start.max(bounds.start)..range.end.min(bounds.end))
        })
        .collect()
}
