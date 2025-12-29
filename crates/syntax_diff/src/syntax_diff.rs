//! AST-aware diffing for syntax trees.

mod changes;
mod syntax_graph;
mod syntax_tree;
mod unchanged;

pub use changes::{ChangeKind, ChangeMap};
pub use syntax_graph::{SyntaxEdge, SyntaxVertex};
pub use syntax_tree::{
    SyntaxAtomKind, SyntaxId, SyntaxNode, SyntaxTree, SyntaxTreeCursor, build_tree,
};

use std::ops::Range;

use crate::syntax_graph::ExceededGraphLimit;

/// Default graph limit (10 million vertices).
pub const DEFAULT_GRAPH_LIMIT: usize = 10_000_000;

/// Options for syntax diffing.
#[derive(Clone)]
pub struct DiffOptions {
    /// Maximum number of graph vertices before giving up.
    pub graph_limit: usize,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            graph_limit: DEFAULT_GRAPH_LIMIT,
        }
    }
}

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
    options: &DiffOptions,
) -> Result<SyntaxDiff, ExceededGraphLimit> {
    let mut change_map = ChangeMap::default();

    let regions = unchanged::mark_unchanged(lhs_tree, rhs_tree, &mut change_map);

    for region in regions {
        if region.lhs_ids.is_empty() && region.rhs_ids.is_empty() {
            continue;
        }

        let route = syntax_graph::shortest_path(lhs_tree, rhs_tree, options.graph_limit)?;

        populate_change_map(lhs_tree, rhs_tree, &route.0, &mut change_map);
    }

    let lhs_ranges = collect_novel_ranges(lhs_tree, &change_map);
    let rhs_ranges = collect_novel_ranges(rhs_tree, &change_map);

    Ok(SyntaxDiff {
        lhs_ranges,
        rhs_ranges,
    })
}

fn populate_change_map(
    lhs_tree: &SyntaxTree,
    rhs_tree: &SyntaxTree,
    route: &[(SyntaxVertex<'_>, SyntaxEdge)],
    map: &mut ChangeMap,
) {
    // Route is now (vertex_before, edge) pairs.
    // vertex_before.lhs and vertex_before.rhs point to the nodes being consumed by edge.

    for (vertex, edge) in route {
        match edge {
            SyntaxEdge::UnchangedNode { .. } => {
                // Both LHS and RHS nodes are unchanged
                if let (Some(lhs_id), Some(rhs_id)) = (vertex.lhs.id(), vertex.rhs.id()) {
                    changes::insert_deep_unchanged(lhs_tree, lhs_id, rhs_tree, rhs_id, map);
                    changes::insert_deep_unchanged(rhs_tree, rhs_id, lhs_tree, lhs_id, map);
                }
            }
            SyntaxEdge::EnterUnchangedDelimiter { .. } => {
                // The list nodes have matching delimiters
                if let (Some(lhs_id), Some(rhs_id)) = (vertex.lhs.id(), vertex.rhs.id()) {
                    map.insert(lhs_id, ChangeKind::Unchanged(rhs_id));
                    map.insert(rhs_id, ChangeKind::Unchanged(lhs_id));
                }
            }
            SyntaxEdge::ReplacedComment { levenshtein_pct }
            | SyntaxEdge::ReplacedString { levenshtein_pct } => {
                if let (Some(lhs_id), Some(rhs_id)) = (vertex.lhs.id(), vertex.rhs.id()) {
                    if *levenshtein_pct > 20 {
                        let kind = if matches!(edge, SyntaxEdge::ReplacedComment { .. }) {
                            ChangeKind::ReplacedComment(lhs_id, rhs_id)
                        } else {
                            ChangeKind::ReplacedString(lhs_id, rhs_id)
                        };
                        map.insert(lhs_id, kind);
                        map.insert(rhs_id, kind);
                    } else {
                        map.insert(lhs_id, ChangeKind::Novel);
                        map.insert(rhs_id, ChangeKind::Novel);
                    }
                }
            }
            SyntaxEdge::NovelAtomLHS => {
                if let Some(lhs_id) = vertex.lhs.id() {
                    map.insert(lhs_id, ChangeKind::Novel);
                }
            }
            SyntaxEdge::EnterNovelDelimiterLHS => {
                if let Some(lhs_id) = vertex.lhs.id() {
                    changes::insert_deep_novel(lhs_tree, lhs_id, map);
                }
            }
            SyntaxEdge::NovelAtomRHS => {
                if let Some(rhs_id) = vertex.rhs.id() {
                    map.insert(rhs_id, ChangeKind::Novel);
                }
            }
            SyntaxEdge::EnterNovelDelimiterRHS => {
                if let Some(rhs_id) = vertex.rhs.id() {
                    changes::insert_deep_novel(rhs_tree, rhs_id, map);
                }
            }
        }
    }
}

fn collect_novel_ranges(tree: &SyntaxTree, change_map: &ChangeMap) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();

    for id in tree.preorder() {
        match change_map.get(id) {
            Some(ChangeKind::Novel)
            | Some(ChangeKind::ReplacedComment(_, _))
            | Some(ChangeKind::ReplacedString(_, _)) => {
                let node = tree.get(id);
                ranges.push(node.byte_range());
            }
            Some(ChangeKind::Unchanged(_)) => {
                // Node is unchanged, but children might have changes
                // (for lists with unchanged delimiters)
            }
            None => {}
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
