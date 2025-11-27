//! Prefer contiguous novel nodes on the same line.
//!
//! A slider takes the following form:
//!
//! Old:
//!
//! ```text
//! A B
//! C D
//! ```
//!
//! New:
//!
//! ```text
//! A B
//! A B
//! C D
//! ```
//!
//! It would be correct, but ugly, to show the following diff:
//!
//! ```text
//! A +B+
//! +A+ B
//! C D
//! ```
//!
//! This module fixes these cases. It identifies situations where we
//! can change which item is marked as novel (e.g. either `B` in the
//! example above) whilst still showing a valid, minimal diff.
//!
//! A similar problem exists with line-oriented diffs, see
//! [diff-slider-tools](https://github.com/mhagger/diff-slider-tools)
//! for a thorough discussion.

use crate::{
    changes::{insert_deep_novel, insert_deep_unchanged, ChangeKind::*, ChangeMap},
    syntax::Syntax::{self, *},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderPreference {
    PreferOuter,
    #[default]
    PreferInner,
}

pub(crate) fn fix_all_sliders<'a>(
    preference: SliderPreference,
    nodes: &[&'a Syntax<'a>],
    change_map: &mut ChangeMap<'a>,
) {
    // TODO: fix sliders that require more than two steps.
    fix_all_sliders_one_step(nodes, change_map);
    fix_all_sliders_one_step(nodes, change_map);

    fix_all_nested_sliders(preference, nodes, change_map);
}

fn fix_all_sliders_one_step<'a>(nodes: &[&'a Syntax<'a>], change_map: &mut ChangeMap<'a>) {
    for node in nodes {
        if let List { children, .. } = node {
            fix_all_sliders_one_step(children, change_map);
        }
    }
    fix_sliders(nodes, change_map);
}

fn fix_all_nested_sliders<'a>(
    preference: SliderPreference,
    nodes: &[&'a Syntax<'a>],
    change_map: &mut ChangeMap<'a>,
) {
    let prefer_outer = preference == SliderPreference::PreferOuter;
    for node in nodes {
        if prefer_outer {
            fix_nested_slider_prefer_outer(node, change_map);
        } else {
            fix_nested_slider_prefer_inner(node, change_map);
        }
    }
}

fn fix_nested_slider_prefer_outer<'a>(node: &'a Syntax<'a>, change_map: &mut ChangeMap<'a>) {
    if let List { children, .. } = node {
        match change_map
            .get(node)
            .expect("Changes should be set before slider correction")
        {
            Unchanged(_) => {
                let mut candidates = vec![];
                unchanged_descendants_for_outer_slider(children, &mut candidates, change_map);

                if let [candidate] = candidates[..] {
                    if matches!(candidate, List { .. })
                        && matches!(change_map.get(candidate), Some(Novel))
                    {
                        push_unchanged_to_descendant(node, candidate, change_map);
                    }
                }
            }
            ReplacedComment(_, _) | ReplacedString(_, _) | Novel => {}
        }

        for child in children {
            fix_nested_slider_prefer_outer(child, change_map);
        }
    }
}

fn fix_nested_slider_prefer_inner<'a>(node: &'a Syntax<'a>, change_map: &mut ChangeMap<'a>) {
    if let List { children, .. } = node {
        match change_map
            .get(node)
            .expect("Changes should be set before slider correction")
        {
            Unchanged(_) => {}
            ReplacedComment(_, _) | ReplacedString(_, _) => {}
            Novel => {
                let mut found_unchanged = vec![];
                unchanged_descendants(children, &mut found_unchanged, change_map);

                if let [List { .. }] = found_unchanged[..] {
                    push_unchanged_to_ancestor(node, found_unchanged[0], change_map);
                }
            }
        }

        for child in children {
            fix_nested_slider_prefer_inner(child, change_map);
        }
    }
}

fn unchanged_descendants<'a>(
    nodes: &[&'a Syntax<'a>],
    found: &mut Vec<&'a Syntax<'a>>,
    change_map: &ChangeMap<'a>,
) {
    if found.len() > 1 {
        return;
    }

    for node in nodes {
        match change_map.get(node).expect("Node changes should be set") {
            Unchanged(_) => {
                found.push(node);
            }
            Novel | ReplacedComment(_, _) | ReplacedString(_, _) => {
                if let List { children, .. } = node {
                    unchanged_descendants(children, found, change_map);
                }
            }
        }
    }
}

fn unchanged_descendants_for_outer_slider<'a>(
    nodes: &[&'a Syntax<'a>],
    found: &mut Vec<&'a Syntax<'a>>,
    change_map: &ChangeMap<'a>,
) {
    if found.len() > 1 {
        return;
    }

    for node in nodes {
        let is_unchanged = matches!(change_map.get(node), Some(Unchanged(_)));

        match node {
            Atom { .. } => {
                if is_unchanged {
                    // Sliding requires a single list, so an unchanged atom means we can't slide.
                    found.push(node);
                    break;
                }
            }
            List { children, .. } => {
                if is_unchanged {
                    // Can't slide unchanged delimiters.
                    found.push(node);
                    break;
                } else {
                    let has_unchanged_children = children
                        .iter()
                        .any(|node| matches!(change_map.get(node), Some(Unchanged(_))));
                    if has_unchanged_children {
                        found.push(node);
                    } else {
                        unchanged_descendants_for_outer_slider(children, found, change_map);
                    }
                }
            }
        }
    }
}

fn push_unchanged_to_descendant<'a>(
    root: &'a Syntax<'a>,
    inner: &'a Syntax<'a>,
    change_map: &mut ChangeMap<'a>,
) {
    let root_change = change_map
        .get(root)
        .expect("Changes should be set before slider correction");

    let delimiters_match = match (root, inner) {
        (
            List {
                open_content: root_open,
                close_content: root_close,
                ..
            },
            List {
                open_content: inner_open,
                close_content: inner_close,
                ..
            },
        ) => root_open == inner_open && root_close == inner_close,
        _ => false,
    };

    if delimiters_match {
        change_map.insert(root, Novel);
        change_map.insert(inner, root_change);
    }
}

fn push_unchanged_to_ancestor<'a>(
    root: &'a Syntax<'a>,
    inner: &'a Syntax<'a>,
    change_map: &mut ChangeMap<'a>,
) {
    let inner_change = change_map.get(inner).expect("Node changes should be set");

    let delimiters_match = match (root, inner) {
        (
            List {
                open_content: root_open,
                close_content: root_close,
                ..
            },
            List {
                open_content: inner_open,
                close_content: inner_close,
                ..
            },
        ) => root_open == inner_open && root_close == inner_close,
        _ => false,
    };

    if delimiters_match {
        change_map.insert(root, inner_change);
        change_map.insert(inner, Novel);
    }
}

fn fix_sliders<'a>(nodes: &[&'a Syntax<'a>], change_map: &mut ChangeMap<'a>) {
    for (region_start, region_end) in novel_regions_after_unchanged(nodes, change_map) {
        slide_to_prev_node(nodes, change_map, region_start, region_end);
    }
    for (region_start, region_end) in novel_regions_before_unchanged(nodes, change_map) {
        slide_to_next_node(nodes, change_map, region_start, region_end);
    }
}

fn novel_regions_after_unchanged<'a>(
    nodes: &[&'a Syntax<'a>],
    change_map: &ChangeMap<'a>,
) -> Vec<(usize, usize)> {
    let mut regions: Vec<Vec<usize>> = vec![];
    let mut region: Option<Vec<usize>> = None;

    for (i, node) in nodes.iter().enumerate() {
        let change = change_map.get(node).expect("Node changes should be set");

        match change {
            Unchanged(_) => {
                if let Some(region) = region {
                    regions.push(region);
                }
                region = Some(vec![]);
            }
            Novel => {
                if let Some(mut r) = region {
                    r.push(i);
                    region = Some(r);
                }
            }
            ReplacedComment(_, _) | ReplacedString(_, _) => {
                if let Some(region) = region {
                    regions.push(region);
                }
                region = None;
            }
        }
    }

    if let Some(region) = region {
        regions.push(region);
    }

    regions
        .into_iter()
        .filter(|r| !r.is_empty())
        .map(|r| {
            (
                *r.first().expect("Region should be non-empty after filter"),
                *r.last().expect("Region should be non-empty after filter"),
            )
        })
        .collect()
}

fn novel_regions_before_unchanged<'a>(
    nodes: &[&'a Syntax<'a>],
    change_map: &ChangeMap<'a>,
) -> Vec<(usize, usize)> {
    let mut regions: Vec<Vec<usize>> = vec![];
    let mut region: Option<Vec<usize>> = None;

    for (i, node) in nodes.iter().enumerate() {
        let change = change_map.get(node).expect("Node changes should be set");

        match change {
            Unchanged(_) => {
                if let Some(region) = region {
                    regions.push(region);
                }
                region = None;
            }
            Novel => {
                let mut r = region.unwrap_or_default();
                r.push(i);
                region = Some(r);
            }
            ReplacedComment(_, _) | ReplacedString(_, _) => {
                region = None;
            }
        }
    }

    if let Some(region) = region {
        regions.push(region);
    }

    regions
        .into_iter()
        .filter(|r| !r.is_empty())
        .map(|r| {
            (
                *r.first().expect("Region should be non-empty after filter"),
                *r.last().expect("Region should be non-empty after filter"),
            )
        })
        .collect()
}

fn is_novel_deep<'a>(node: &Syntax<'a>, change_map: &ChangeMap<'a>) -> bool {
    match node {
        List { children, .. } => {
            if !matches!(change_map.get(node), Some(Novel)) {
                return false;
            }
            for child in children {
                if !is_novel_deep(child, change_map) {
                    return false;
                }
            }

            true
        }
        Atom { .. } => matches!(change_map.get(node), Some(Novel)),
    }
}

fn slide_to_prev_node<'a>(
    nodes: &[&'a Syntax<'a>],
    change_map: &mut ChangeMap<'a>,
    start_idx: usize,
    end_idx: usize,
) {
    if start_idx == 0 {
        return;
    }
    if start_idx == end_idx {
        return;
    }

    let start_node = nodes[start_idx];
    let last_node = nodes[end_idx];
    let before_start_node = nodes[start_idx - 1];
    let before_last_node = nodes[end_idx - 1];

    if before_start_node.content_id() != last_node.content_id() {
        return;
    }

    let distance_to_before_start = distance_between(before_start_node, start_node);
    let distance_to_last = distance_between(before_last_node, last_node);

    if distance_to_before_start <= distance_to_last {
        let opposite = match change_map
            .get(before_start_node)
            .expect("Node changes should be set")
        {
            Unchanged(n) => {
                if before_start_node.content_id() != n.content_id() {
                    return;
                }
                n
            }
            _ => {
                return;
            }
        };

        for node in &nodes[start_idx..=end_idx] {
            if !is_novel_deep(node, change_map) {
                return;
            }
        }

        insert_deep_novel(before_start_node, change_map);
        insert_deep_unchanged(last_node, opposite, change_map);
        insert_deep_unchanged(opposite, last_node, change_map);
    }
}

fn slide_to_next_node<'a>(
    nodes: &[&'a Syntax<'a>],
    change_map: &mut ChangeMap<'a>,
    start_idx: usize,
    end_idx: usize,
) {
    if end_idx == nodes.len() - 1 {
        return;
    }
    if start_idx == end_idx {
        return;
    }

    let start_node = nodes[start_idx];
    let last_node = nodes[end_idx];
    let after_start_node = nodes[start_idx + 1];
    let after_last_node = nodes[end_idx + 1];

    if after_last_node.content_id() != start_node.content_id() {
        return;
    }

    let distance_to_start = distance_between(start_node, after_start_node);
    let distance_to_after_last = distance_between(last_node, after_last_node);

    if distance_to_after_last < distance_to_start {
        let opposite = match change_map
            .get(after_last_node)
            .expect("Node changes should be set")
        {
            Unchanged(n) => {
                if after_last_node.content_id() != n.content_id() {
                    return;
                }
                n
            }
            _ => {
                return;
            }
        };
        for node in &nodes[start_idx..=end_idx] {
            if !is_novel_deep(node, change_map) {
                return;
            }
        }

        insert_deep_unchanged(start_node, opposite, change_map);
        insert_deep_unchanged(opposite, start_node, change_map);
        insert_deep_novel(after_last_node, change_map);
    }
}

fn distance_between(prev: &Syntax, next: &Syntax) -> (usize, usize) {
    let prev_end = prev.byte_range().end;
    let next_start = next.byte_range().start;

    if next_start > prev_end {
        (next_start - prev_end, 0)
    } else {
        (0, 0)
    }
}
