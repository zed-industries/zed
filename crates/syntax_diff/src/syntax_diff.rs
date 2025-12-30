//! AST-aware diffing for syntax trees.

mod syntax_graph;
mod syntax_tree;

use collections::FxHashMap;
pub use syntax_graph::{SyntaxEdge, SyntaxPath, SyntaxVertex};
pub use syntax_tree::{SyntaxId, SyntaxNode, SyntaxTree, SyntaxTreeCursor, build_tree};

use std::ops::Range;

use crate::syntax_graph::ExceededGraphLimit;

/// Default graph limit (10 million vertices).
pub const DEFAULT_GRAPH_LIMIT: usize = 10_000_000;

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
    let mut lhs_change_map = FxHashMap::default();
    let mut rhs_change_map = FxHashMap::default();
    let route = syntax_graph::shortest_path(lhs_tree, rhs_tree, DEFAULT_GRAPH_LIMIT)?;

    // Route entries have vertices[0] = from (source), vertices[1] = to (destination).
    // The source vertex's lhs/rhs point to the nodes being consumed by the edge.
    for path in route.0 {
        let Some(edge) = path.edge else { continue };
        let Some(vertex) = path.vertices[0].as_ref() else {
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

    let lhs_ranges = collect_novel_ranges(lhs_tree, &lhs_change_map);
    let rhs_ranges = collect_novel_ranges(rhs_tree, &rhs_change_map);

    Ok(SyntaxDiff {
        lhs_ranges,
        rhs_ranges,
    })
}

fn collect_novel_ranges(
    tree: &SyntaxTree,
    change_map: &FxHashMap<SyntaxId, SyntaxChange>,
) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();

    for (id, change) in change_map.iter() {
        match change {
            SyntaxChange::Novel => {
                let node = tree.get(*id);

                if node.is_atom() {
                    ranges.push(node.byte_range.clone());
                } else {
                    let open = node.open_delimiter_range();
                    let close = node.close_delimiter_range();

                    if !open.is_empty() {
                        ranges.push(open);
                    }

                    if !close.is_empty() {
                        ranges.push(close);
                    }
                }
            }
            SyntaxChange::Replaced(_, _) => {
                ranges.push(tree.get(*id).byte_range.clone());
            }
            _ => {}
        }
    }

    if ranges.is_empty() {
        return ranges;
    }

    // Merge overlapping/adjacent ranges
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
