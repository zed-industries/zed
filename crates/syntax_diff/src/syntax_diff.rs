//! AST-aware diffing for syntax trees.

#[cfg(test)]
mod syntax_diff_tests;
mod syntax_graph;
mod syntax_tree;

use arrayvec::ArrayVec;
use language::DiffOptions;
pub use syntax_graph::{SyntaxEdge, SyntaxPath, SyntaxVertex};
pub use syntax_tree::{SyntaxId, SyntaxNode, SyntaxTree, SyntaxTreeCursor, build_tree};

use std::ops::Range;

use crate::{
    syntax_graph::{ExceededGraphLimit, SyntaxRoute},
    syntax_tree::SyntaxHint,
};

/// Result of a syntax diff operation.
///
/// Ranges are absolute byte positions in the original source text.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
    options: &DiffOptions,
) -> Result<SyntaxDiff, ExceededGraphLimit> {
    let route =
        syntax_graph::shortest_path(lhs_tree, rhs_tree, options.max_syntax_diff_graph_size)?;

    let (lhs_ranges, rhs_ranges) = collect_ranges(&route, lhs_tree, rhs_tree, options);

    Ok(SyntaxDiff {
        lhs_ranges: merge_ranges(lhs_ranges),
        rhs_ranges: merge_ranges(rhs_ranges),
    })
}

fn collect_ranges(
    route: &SyntaxRoute<'_>,
    lhs_tree: &SyntaxTree,
    rhs_tree: &SyntaxTree,
    options: &DiffOptions,
) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    let mut lhs_ranges = Vec::default();
    let mut rhs_ranges = Vec::default();

    for path in &route.0 {
        let Some(edge) = path.edge else { continue };
        let Some(vertex) = path.from.as_ref() else {
            continue;
        };

        match edge {
            SyntaxEdge::Replaced { levenshtein_pct } => {
                if let (Some(lhs_node), Some(rhs_node)) = (
                    vertex.lhs.id().map(|id| lhs_tree.get(id)),
                    vertex.rhs.id().map(|id| rhs_tree.get(id)),
                ) {
                    if levenshtein_pct > 20 {
                        if let Some((lhs_replace_ranges, rhs_replace_ranges)) =
                            get_replace_ranges(lhs_node, rhs_node, options)
                        {
                            lhs_ranges.extend(lhs_replace_ranges);
                            rhs_ranges.extend(rhs_replace_ranges);
                        }
                    } else {
                        lhs_ranges.extend(get_novel_ranges(lhs_node));
                        rhs_ranges.extend(get_novel_ranges(rhs_node));
                    }
                }
            }
            SyntaxEdge::NovelAtomLHS | SyntaxEdge::EnterNovelDelimiterLHS => {
                if let Some(lhs_node) = vertex.lhs.id().map(|id| lhs_tree.get(id)) {
                    lhs_ranges.extend(get_novel_ranges(lhs_node));
                }
            }
            SyntaxEdge::NovelAtomRHS | SyntaxEdge::EnterNovelDelimiterRHS => {
                if let Some(rhs_node) = vertex.rhs.id().map(|id| rhs_tree.get(id)) {
                    rhs_ranges.extend(get_novel_ranges(rhs_node));
                }
            }
            _ => {}
        }
    }

    (lhs_ranges, rhs_ranges)
}

fn get_novel_ranges(node: &SyntaxNode) -> ArrayVec<Range<usize>, 2> {
    let mut ranges = ArrayVec::new();

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

    ranges
}

fn get_replace_ranges(
    lhs_node: &SyntaxNode,
    rhs_node: &SyntaxNode,
    options: &DiffOptions,
) -> Option<(Vec<Range<usize>>, Vec<Range<usize>>)> {
    if let (Some(SyntaxHint::Comment(lhs_comment)), Some(SyntaxHint::Comment(rhs_comment))) =
        (lhs_node.hint.as_ref(), rhs_node.hint.as_ref())
    {
        let (lhs_word_ranges, rhs_word_ranges) = language::word_diff_ranges(
            lhs_comment,
            rhs_comment,
            DiffOptions {
                language_scope: options.language_scope.clone(),
                ..*options
            },
        );

        // Convert relative ranges to absolute byte positions
        let lhs_offset = lhs_node.byte_range.start;
        let rhs_offset = rhs_node.byte_range.start;

        let lhs_ranges = lhs_word_ranges
            .into_iter()
            .map(|r| (r.start + lhs_offset)..(r.end + lhs_offset))
            .collect();

        let rhs_ranges = rhs_word_ranges
            .into_iter()
            .map(|r| (r.start + rhs_offset)..(r.end + rhs_offset))
            .collect();

        Some((lhs_ranges, rhs_ranges))
    } else {
        None
    }
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
