//! AST-aware diffing for syntax trees.

mod syntax_changes;
mod syntax_graph;
mod syntax_tree;

pub use syntax_changes::{SyntaxChange, SyntaxChanges};
pub use syntax_graph::{SyntaxEdge, SyntaxVertex};
pub use syntax_tree::{SyntaxId, SyntaxNode, SyntaxTree, SyntaxTreeCursor, build_tree};

use std::ops::Range;

use crate::syntax_graph::ExceededGraphLimit;

/// Default graph limit (10 million vertices).
pub const DEFAULT_GRAPH_LIMIT: usize = 10_000_000;

/// Result of a syntax diff operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxDiff {
    /// Byte ranges in the LHS that are novel (removed/changed).
    pub lhs_ranges: Vec<Range<usize>>,
    /// Byte ranges in the RHS that are novel (added/changed).
    pub rhs_ranges: Vec<Range<usize>>,
}

/// Compute a syntax-aware diff between two `SyntaxTree`s.
pub fn diff_trees(
    lhs_tree: &SyntaxTree,
    rhs_tree: &SyntaxTree,
) -> Result<SyntaxDiff, ExceededGraphLimit> {
    let mut lhs_change_map = SyntaxChanges::default();
    let mut rhs_change_map = SyntaxChanges::default();
    let route = syntax_graph::shortest_path(lhs_tree, rhs_tree, DEFAULT_GRAPH_LIMIT)?;

    populate_change_map(&route.0, &mut lhs_change_map, &mut rhs_change_map);

    let lhs_ranges = collect_novel_ranges(lhs_tree, &lhs_change_map);
    let rhs_ranges = collect_novel_ranges(rhs_tree, &rhs_change_map);

    Ok(SyntaxDiff {
        lhs_ranges,
        rhs_ranges,
    })
}

fn populate_change_map(
    route: &[(SyntaxVertex<'_>, SyntaxEdge)],
    lhs_map: &mut SyntaxChanges,
    rhs_map: &mut SyntaxChanges,
) {
    // Route is now (vertex_before, edge) pairs.
    // vertex_before.lhs and vertex_before.rhs point to the nodes being consumed by edge.

    for (vertex, edge) in route {
        match edge {
            SyntaxEdge::Replaced { levenshtein_pct } => {
                if let (Some(lhs_id), Some(rhs_id)) = (vertex.lhs.id(), vertex.rhs.id()) {
                    if *levenshtein_pct > 20 {
                        lhs_map.insert(lhs_id, SyntaxChange::Replaced(lhs_id, rhs_id));
                        rhs_map.insert(rhs_id, SyntaxChange::Replaced(lhs_id, rhs_id));
                    } else {
                        lhs_map.insert(lhs_id, SyntaxChange::Novel);
                        rhs_map.insert(rhs_id, SyntaxChange::Novel);
                    }
                }
            }
            SyntaxEdge::NovelAtomLHS | SyntaxEdge::EnterNovelDelimiterLHS => {
                if let Some(lhs_id) = vertex.lhs.id() {
                    lhs_map.insert(lhs_id, SyntaxChange::Novel);
                }
            }
            SyntaxEdge::NovelAtomRHS | SyntaxEdge::EnterNovelDelimiterRHS => {
                if let Some(rhs_id) = vertex.rhs.id() {
                    rhs_map.insert(rhs_id, SyntaxChange::Novel);
                }
            }
            _ => {}
        }
    }
}

fn collect_novel_ranges(tree: &SyntaxTree, change_map: &SyntaxChanges) -> Vec<Range<usize>> {
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
        .iter()
        .map(|range| range.start - tree.offset..range.end - tree.offset)
        .collect()
}
