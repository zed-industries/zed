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

use crate::{syntax_graph::SyntaxRoute, syntax_tree::SyntaxHint};

/// Compute a syntax-aware diff between two `SyntaxTree`s.
///
/// When `lhs_range` and `rhs_range` are provided, the returned ranges are clipped to
/// those bounds and made relative (starting from 0). This is useful when diffing syntax
/// trees that may be larger than the region of interest (e.g., a function containing
/// multiple diff hunks).
pub fn diff_trees(
    lhs_tree: &SyntaxTree,
    rhs_tree: &SyntaxTree,
    lhs_range: Option<Range<usize>>,
    rhs_range: Option<Range<usize>>,
    options: &DiffOptions,
) -> Option<(Vec<Range<usize>>, Vec<Range<usize>>)> {
    let route =
        syntax_graph::shortest_path(lhs_tree, rhs_tree, options.max_syntax_diff_graph_size)?;

    let (lhs_ranges, rhs_ranges) =
        collect_ranges(&route, lhs_tree, rhs_tree, lhs_range, rhs_range, options);

    Some((merge_ranges(lhs_ranges), merge_ranges(rhs_ranges)))
}

fn collect_ranges(
    route: &SyntaxRoute<'_>,
    lhs_tree: &SyntaxTree,
    rhs_tree: &SyntaxTree,
    lhs_bounds: Option<Range<usize>>,
    rhs_bounds: Option<Range<usize>>,
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
                        if let Some((lhs_replace_ranges, rhs_replace_ranges)) = get_replace_ranges(
                            lhs_node,
                            rhs_node,
                            lhs_bounds.as_ref(),
                            rhs_bounds.as_ref(),
                            options,
                        ) {
                            lhs_ranges.extend(lhs_replace_ranges);
                            rhs_ranges.extend(rhs_replace_ranges);
                        }
                    } else {
                        lhs_ranges.extend(get_novel_ranges(lhs_node, lhs_bounds.as_ref()));
                        rhs_ranges.extend(get_novel_ranges(rhs_node, rhs_bounds.as_ref()));
                    }
                }
            }
            SyntaxEdge::NovelAtomLHS | SyntaxEdge::EnterNovelDelimiterLHS => {
                if let Some(lhs_node) = vertex.lhs.id().map(|id| lhs_tree.get(id)) {
                    lhs_ranges.extend(get_novel_ranges(lhs_node, lhs_bounds.as_ref()));
                }
            }
            SyntaxEdge::NovelAtomRHS | SyntaxEdge::EnterNovelDelimiterRHS => {
                if let Some(rhs_node) = vertex.rhs.id().map(|id| rhs_tree.get(id)) {
                    rhs_ranges.extend(get_novel_ranges(rhs_node, rhs_bounds.as_ref()));
                }
            }
            SyntaxEdge::EnterNovelDelimiterBoth => {
                if let Some(lhs_node) = vertex.lhs.id().map(|id| lhs_tree.get(id)) {
                    lhs_ranges.extend(get_novel_ranges(lhs_node, lhs_bounds.as_ref()));
                }
                if let Some(rhs_node) = vertex.rhs.id().map(|id| rhs_tree.get(id)) {
                    rhs_ranges.extend(get_novel_ranges(rhs_node, rhs_bounds.as_ref()));
                }
            }
            _ => {}
        }
    }

    (lhs_ranges, rhs_ranges)
}

fn get_novel_ranges(node: &SyntaxNode, bounds: Option<&Range<usize>>) -> ArrayVec<Range<usize>, 2> {
    let mut ranges = ArrayVec::new();

    if node.is_atom() {
        if let Some(r) = adjust_range_to_bounds(node.byte_range.clone(), bounds) {
            ranges.push(r);
        }
    } else {
        if let Some(r) = node
            .open_delimiter_range()
            .and_then(|r| adjust_range_to_bounds(r, bounds))
        {
            ranges.push(r);
        }

        if let Some(r) = node
            .close_delimiter_range()
            .and_then(|r| adjust_range_to_bounds(r, bounds))
        {
            ranges.push(r);
        }
    }

    ranges
}

fn get_replace_ranges(
    lhs_node: &SyntaxNode,
    rhs_node: &SyntaxNode,
    lhs_bounds: Option<&Range<usize>>,
    rhs_bounds: Option<&Range<usize>>,
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

        // Convert relative ranges to absolute byte positions, then adjust to bounds
        let lhs_offset = lhs_node.byte_range.start;
        let rhs_offset = rhs_node.byte_range.start;

        let lhs_ranges = lhs_word_ranges
            .into_iter()
            .map(|r| (r.start + lhs_offset)..(r.end + lhs_offset))
            .filter_map(|r| adjust_range_to_bounds(r, lhs_bounds))
            .collect();

        let rhs_ranges = rhs_word_ranges
            .into_iter()
            .map(|r| (r.start + rhs_offset)..(r.end + rhs_offset))
            .filter_map(|r| adjust_range_to_bounds(r, rhs_bounds))
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

fn adjust_range_to_bounds(
    range: Range<usize>,
    bounds: Option<&Range<usize>>,
) -> Option<Range<usize>> {
    let Some(bounds) = bounds else {
        return Some(range);
    };

    if range.end <= bounds.start || range.start >= bounds.end {
        return None;
    }

    let start = range.start.max(bounds.start) - bounds.start;
    let end = range.end.min(bounds.end) - bounds.start;
    Some(start..end)
}
