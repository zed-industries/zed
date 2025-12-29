//! Find nodes that are obviously unchanged, so we can run the main
//! diff on smaller inputs.

use std::hash::Hash;

use crate::diff::changes::{ChangeKind, ChangeMap, insert_deep_unchanged};
use crate::diff::lcs_diff;
use crate::hash::DftHashSet;
use crate::parse::syntax::{ContentId, Syntax};

const TINY_TREE_THRESHOLD: u32 = 10;
const MOSTLY_UNCHANGED_MIN_COMMON_CHILDREN: usize = 4;

/// Set [`ChangeKind`] on nodes that have exactly the same structure
/// on both sides, and return a vec of pairs that need proper diffing.
pub(crate) fn mark_unchanged<'a>(
    lhs_nodes: &[&'a Syntax<'a>],
    rhs_nodes: &[&'a Syntax<'a>],
    change_map: &mut ChangeMap<'a>,
) -> Vec<(Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> {
    let (_, lhs_nodes, rhs_nodes) = shrink_unchanged_at_ends(lhs_nodes, rhs_nodes, change_map);

    let mut nodes_to_diff = vec![];
    for (lhs_nodes, rhs_nodes) in split_mostly_unchanged_toplevel(&lhs_nodes, &rhs_nodes) {
        let (_, lhs_nodes, rhs_nodes) =
            shrink_unchanged_at_ends(&lhs_nodes, &rhs_nodes, change_map);
        nodes_to_diff.extend(split_unchanged(&lhs_nodes, &rhs_nodes, change_map));
    }

    nodes_to_diff
}

#[derive(Debug)]
enum ChangeState {
    UnchangedDelimiter,
    UnchangedNode,
    PossiblyChanged,
}

fn split_unchanged<'a>(
    lhs_nodes: &[&'a Syntax<'a>],
    rhs_nodes: &[&'a Syntax<'a>],
    change_map: &mut ChangeMap<'a>,
) -> Vec<(Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> {
    let size_threshold = if let Ok(env_threshold) = std::env::var("DFT_TINY_THRESHOLD") {
        env_threshold
            .parse::<u32>()
            .ok()
            .unwrap_or(TINY_TREE_THRESHOLD)
    } else {
        TINY_TREE_THRESHOLD
    };

    let mut res: Vec<(Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> = vec![];
    for (cs, lhs_section_nodes, rhs_section_nodes) in
        split_unchanged_toplevel(lhs_nodes, rhs_nodes, size_threshold)
    {
        match cs {
            ChangeState::UnchangedDelimiter => {
                assert_eq!(lhs_section_nodes.len(), rhs_section_nodes.len());
                for (lhs_section_node, rhs_section_node) in
                    lhs_section_nodes.iter().zip(rhs_section_nodes.iter())
                {
                    change_map.insert(lhs_section_node, ChangeKind::Unchanged(rhs_section_node));
                    change_map.insert(rhs_section_node, ChangeKind::Unchanged(lhs_section_node));
                }
            }
            ChangeState::UnchangedNode => {
                assert_eq!(lhs_section_nodes.len(), rhs_section_nodes.len());
                for (lhs_section_node, rhs_section_node) in
                    lhs_section_nodes.iter().zip(rhs_section_nodes.iter())
                {
                    insert_deep_unchanged(lhs_section_node, rhs_section_node, change_map);
                    insert_deep_unchanged(rhs_section_node, lhs_section_node, change_map);
                }
            }
            ChangeState::PossiblyChanged => {
                res.push((lhs_section_nodes, rhs_section_nodes));
            }
        }
    }

    res
}

fn split_unchanged_singleton_list<'a>(
    lhs_nodes: &[&'a Syntax<'a>],
    rhs_nodes: &[&'a Syntax<'a>],
    size_threshold: u32,
) -> Vec<(ChangeState, Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> {
    let mut res: Vec<(ChangeState, Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> = vec![];
    match as_singleton_list_children(lhs_nodes, rhs_nodes) {
        Some((lhs_children, rhs_children)) => {
            let mut split_children =
                split_unchanged_toplevel(&lhs_children, &rhs_children, size_threshold);
            if split_children.len() > 1 {
                res.push((
                    ChangeState::UnchangedDelimiter,
                    lhs_nodes.to_vec(),
                    rhs_nodes.to_vec(),
                ));
                // Managed to further split.
                res.append(&mut split_children);
            } else {
                // Did not split further. Keep the outer list, so we can use
                // its delimiter when doing the tree diff.
                res.push((
                    ChangeState::PossiblyChanged,
                    lhs_nodes.to_vec(),
                    rhs_nodes.to_vec(),
                ));
            }
        }
        None => {
            res.push((
                ChangeState::PossiblyChanged,
                lhs_nodes.to_vec(),
                rhs_nodes.to_vec(),
            ));
        }
    }

    res
}

fn find_unique_content_ids(node: &Syntax, unique_ids: &mut DftHashSet<ContentId>) {
    if node.content_is_unique() {
        unique_ids.insert(node.content_id());
    }
    if let Syntax::List { children, .. } = node {
        for child in children {
            find_unique_content_ids(child, unique_ids);
        }
    }
}

fn find_all_unique_content_ids(node: &Syntax) -> DftHashSet<u32> {
    let mut unique_ids = DftHashSet::default();
    find_unique_content_ids(node, &mut unique_ids);
    unique_ids
}

fn count_unique_subtrees(node: &Syntax, opposite_unique_ids: &DftHashSet<u32>) -> usize {
    if node.content_is_unique() && opposite_unique_ids.contains(&node.content_id()) {
        // Ignore children as soon as find a unique node, to avoid
        // overcounting.
        return 1;
    }

    if let Syntax::List { children, .. } = node {
        return children
            .iter()
            .map(|child| count_unique_subtrees(child, opposite_unique_ids))
            .sum();
    }

    0
}

/// Count the nodes in `lhs` that are unique to the LHS input, but are
/// also present in `rhs`.
///
/// Ignores children of unique nodes, so we don't overcount.
fn count_common_unique(lhs: &Syntax, rhs: &Syntax) -> usize {
    let rhs_unique_ids = find_all_unique_content_ids(rhs);
    count_unique_subtrees(lhs, &rhs_unique_ids)
}

/// Return true if both nodes are lists with same delimiters and have
/// the same start and end children.
fn is_mostly_unchanged_list(lhs: &Syntax, rhs: &Syntax) -> bool {
    match (lhs, rhs) {
        (Syntax::List { .. }, Syntax::List { .. }) => {
            count_common_unique(lhs, rhs) >= MOSTLY_UNCHANGED_MIN_COMMON_CHILDREN
        }
        _ => false,
    }
}

/// Split out top-level lists that are largely the same.
///
/// This is important in cases where we have two adjacent lists that
/// have a small number of changes.
///
/// ```
/// ; old
/// (1 2 3 4) (a b c d)
///
/// ; new
/// (1 2 novel 3 4) (a b novel-2 c d)
/// ```
///
/// By splitting out these mostly-unchanged lists, we can
/// substantially further shrink them.
fn split_mostly_unchanged_toplevel<'a>(
    lhs_nodes: &[&'a Syntax<'a>],
    rhs_nodes: &[&'a Syntax<'a>],
) -> Vec<(Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> {
    let mut lhs_nodes = lhs_nodes;
    let mut rhs_nodes = rhs_nodes;

    let mut leading: Vec<(Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> = vec![];
    while let (Some(lhs), Some(rhs)) = (lhs_nodes.first(), rhs_nodes.first()) {
        if is_mostly_unchanged_list(lhs, rhs) {
            leading.push((vec![lhs], vec![rhs]));

            lhs_nodes = &lhs_nodes[1..];
            rhs_nodes = &rhs_nodes[1..];
        } else {
            break;
        }
    }

    let mut trailing: Vec<(Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> = vec![];
    while let (Some(lhs), Some(rhs)) = (lhs_nodes.last(), rhs_nodes.last()) {
        if is_mostly_unchanged_list(lhs, rhs) {
            trailing.push((vec![lhs], vec![rhs]));

            lhs_nodes = &lhs_nodes[..lhs_nodes.len() - 1];
            rhs_nodes = &rhs_nodes[..rhs_nodes.len() - 1];
        } else {
            break;
        }
    }

    let mut res: Vec<(Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> = vec![];
    res.extend_from_slice(&leading[..]);

    if !lhs_nodes.is_empty() || !rhs_nodes.is_empty() {
        res.push((Vec::from(lhs_nodes), Vec::from(rhs_nodes)));
    }

    res.extend(trailing.into_iter().rev());

    res
}

/// Mark top-level nodes as unchanged if they have exactly the same
/// content on both sides.
fn split_unchanged_toplevel<'a>(
    lhs_nodes: &[&'a Syntax<'a>],
    rhs_nodes: &[&'a Syntax<'a>],
    size_threshold: u32,
) -> Vec<(ChangeState, Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> {
    let lhs_node_ids = lhs_nodes
        .iter()
        .map(|n| EqOnFirstItem(n.content_id(), *n))
        .collect::<Vec<_>>();
    let rhs_node_ids = rhs_nodes
        .iter()
        .map(|n| EqOnFirstItem(n.content_id(), *n))
        .collect::<Vec<_>>();

    let mut res: Vec<(ChangeState, Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> = vec![];
    let mut section_lhs_nodes = vec![];
    let mut section_rhs_nodes = vec![];

    for diff_res in lcs_diff::slice(&lhs_node_ids, &rhs_node_ids) {
        match diff_res {
            lcs_diff::DiffResult::Both(lhs, rhs) => {
                let lhs_node = lhs.1;
                let rhs_node = rhs.1;

                // If we've matched a node but it's very small, don't
                // mark it as unchanged. We're interested in marking
                // large nodes as unchanged for performance, but small
                // nodes are fine.
                //
                // If we don't do this, difftastic can find e.g. a
                // single unchanged comma in a huge expression, which
                // produces worse diffs.
                let tiny_node = match lhs_node {
                    Syntax::List {
                        num_descendants, ..
                    } => *num_descendants < size_threshold,
                    Syntax::Atom { .. } => true,
                };

                if tiny_node {
                    section_lhs_nodes.push(lhs_node);
                    section_rhs_nodes.push(rhs_node);
                } else {
                    if !section_lhs_nodes.is_empty() || !section_rhs_nodes.is_empty() {
                        res.extend(split_unchanged_singleton_list(
                            &section_lhs_nodes,
                            &section_rhs_nodes,
                            size_threshold,
                        ));
                        section_lhs_nodes = vec![];
                        section_rhs_nodes = vec![];
                    }

                    res.push((ChangeState::UnchangedNode, vec![lhs_node], vec![rhs_node]));
                }
            }
            lcs_diff::DiffResult::Left(lhs) => {
                section_lhs_nodes.push(lhs.1);
            }
            lcs_diff::DiffResult::Right(rhs) => {
                section_rhs_nodes.push(rhs.1);
            }
        }
    }

    if !section_lhs_nodes.is_empty() || !section_rhs_nodes.is_empty() {
        res.extend(split_unchanged_singleton_list(
            &section_lhs_nodes,
            &section_rhs_nodes,
            size_threshold,
        ));
    }

    res
}

#[derive(Debug, Clone)]
struct EqOnFirstItem<X, Y>(X, Y);

impl<X: Eq, Y> PartialEq for EqOnFirstItem<X, Y> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<X: Eq, Y> Eq for EqOnFirstItem<X, Y> {}

impl<X: Eq + PartialOrd, Y> PartialOrd for EqOnFirstItem<X, Y> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<X: Eq + Ord, Y> Ord for EqOnFirstItem<X, Y> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<X: Hash, Y> Hash for EqOnFirstItem<X, Y> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

fn as_singleton_list_children<'a>(
    lhs_nodes: &[&'a Syntax<'a>],
    rhs_nodes: &[&'a Syntax<'a>],
) -> Option<(Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>)> {
    if let (
        [
            Syntax::List {
                open_content: lhs_open,
                children: lhs_children,
                close_content: lhs_close,
                ..
            },
        ],
        [
            Syntax::List {
                open_content: rhs_open,
                children: rhs_children,
                close_content: rhs_close,
                ..
            },
        ],
    ) = (lhs_nodes, rhs_nodes)
    {
        if lhs_open == rhs_open && lhs_close == rhs_close {
            return Some((lhs_children.clone(), rhs_children.clone()));
        }
    }

    None
}

/// If we're comparing two lists that have the same delimiters, mark
/// the delimiters as unchanged and return the children.
fn shrink_unchanged_delimiters<'a>(
    lhs_nodes: &[&'a Syntax<'a>],
    rhs_nodes: &[&'a Syntax<'a>],
    change_map: &mut ChangeMap<'a>,
) -> (bool, Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>) {
    if let (
        [
            Syntax::List {
                open_content: lhs_open,
                children: lhs_children,
                close_content: lhs_close,
                ..
            },
        ],
        [
            Syntax::List {
                open_content: rhs_open,
                children: rhs_children,
                close_content: rhs_close,
                ..
            },
        ],
    ) = (lhs_nodes, rhs_nodes)
    {
        if lhs_open == rhs_open && lhs_close == rhs_close {
            let (changed_later, lhs_shrunk_nodes, rhs_shrunk_nodes) =
                shrink_unchanged_at_ends(lhs_children, rhs_children, change_map);
            if changed_later {
                change_map.insert(lhs_nodes[0], ChangeKind::Unchanged(rhs_nodes[0]));
                change_map.insert(rhs_nodes[0], ChangeKind::Unchanged(lhs_nodes[0]));

                return (true, lhs_shrunk_nodes, rhs_shrunk_nodes);
            }
        }
    }

    (false, Vec::from(lhs_nodes), Vec::from(rhs_nodes))
}

/// Skip syntax nodes at the beginning or end that are obviously
/// unchanged.
///
/// Set the [`ChangeKind`] on the definitely changed nodes, and return the
/// nodes that may contain changes.
fn shrink_unchanged_at_ends<'a>(
    lhs_nodes: &[&'a Syntax<'a>],
    rhs_nodes: &[&'a Syntax<'a>],
    change_map: &mut ChangeMap<'a>,
) -> (bool, Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>) {
    let mut lhs_nodes = lhs_nodes;
    let mut rhs_nodes = rhs_nodes;

    let mut changed = false;
    while let (Some(lhs_node), Some(rhs_node)) = (lhs_nodes.first(), rhs_nodes.first()) {
        // We don't consider TINY_TREE_THRESHOLD here because we are
        // only considering equal nodes at the beginning or end of the
        // file. There's no risk we split unrelated regions with a
        // trivial unchanged node in the middle.
        if lhs_node.content_id() == rhs_node.content_id() {
            insert_deep_unchanged(lhs_node, rhs_node, change_map);
            insert_deep_unchanged(rhs_node, lhs_node, change_map);

            changed = true;
            lhs_nodes = &lhs_nodes[1..];
            rhs_nodes = &rhs_nodes[1..];
        } else {
            break;
        }
    }

    while let (Some(lhs_node), Some(rhs_node)) = (lhs_nodes.last(), rhs_nodes.last()) {
        if lhs_node.content_id() == rhs_node.content_id() {
            insert_deep_unchanged(lhs_node, rhs_node, change_map);
            insert_deep_unchanged(rhs_node, lhs_node, change_map);

            changed = true;
            lhs_nodes = &lhs_nodes[..lhs_nodes.len() - 1];
            rhs_nodes = &rhs_nodes[..rhs_nodes.len() - 1];
        } else {
            break;
        }
    }

    if lhs_nodes.len() == 1 && rhs_nodes.len() == 1 {
        let (changed_later, lhs_nodes, rhs_nodes) =
            shrink_unchanged_delimiters(lhs_nodes, rhs_nodes, change_map);
        (changed || changed_later, lhs_nodes, rhs_nodes)
    } else {
        (changed, Vec::from(lhs_nodes), Vec::from(rhs_nodes))
    }
}

#[cfg(test)]
mod tests {
    use typed_arena::Arena;

    use super::*;
    use crate::{
        parse::guess_language,
        parse::tree_sitter_parser::{from_language, parse},
        syntax_tree::init_all_info,
    };

    #[test]
    fn test_shrink_unchanged_at_start() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(&arena, "unchanged A B", &config, false);
        let rhs_nodes = parse(&arena, "unchanged X", &config, false);
        init_all_info(&lhs_nodes, &rhs_nodes);

        let mut change_map = ChangeMap::default();
        let (_, lhs_after_skip, rhs_after_skip) =
            shrink_unchanged_at_ends(&lhs_nodes, &rhs_nodes, &mut change_map);

        assert_eq!(
            change_map.get(lhs_nodes[0]),
            Some(ChangeKind::Unchanged(rhs_nodes[0]))
        );
        assert_eq!(
            change_map.get(rhs_nodes[0]),
            Some(ChangeKind::Unchanged(lhs_nodes[0]))
        );

        assert_eq!(lhs_after_skip.len(), 2);
        assert_eq!(rhs_after_skip.len(), 1);
    }

    #[test]
    fn test_shrink_unchanged_at_end() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(&arena, "A B unchanged", &config, false);
        let rhs_nodes = parse(&arena, "X unchanged", &config, false);
        init_all_info(&lhs_nodes, &rhs_nodes);

        let mut change_map = ChangeMap::default();
        let (_, lhs_after_skip, rhs_after_skip) =
            shrink_unchanged_at_ends(&lhs_nodes, &rhs_nodes, &mut change_map);

        assert_eq!(
            change_map.get(lhs_nodes[2]),
            Some(ChangeKind::Unchanged(rhs_nodes[1]))
        );
        assert_eq!(
            change_map.get(rhs_nodes[1]),
            Some(ChangeKind::Unchanged(lhs_nodes[2]))
        );

        assert_eq!(lhs_after_skip.len(), 2);
        assert_eq!(rhs_after_skip.len(), 1);
    }

    #[test]
    fn test_shrink_unchanged_nested() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(
            &arena,
            "unchanged-before (more-unchanged (A))",
            &config,
            false,
        );
        let rhs_nodes = parse(
            &arena,
            "unchanged-before (more-unchanged (B))",
            &config,
            false,
        );
        init_all_info(&lhs_nodes, &rhs_nodes);

        let mut change_map = ChangeMap::default();
        let (_, lhs_after_skip, rhs_after_skip) =
            shrink_unchanged_at_ends(&lhs_nodes, &rhs_nodes, &mut change_map);

        // The only possibly changed nodes are inside the lists.
        assert_eq!(lhs_after_skip.len(), 1);
        assert!(matches!(lhs_after_skip[0], Syntax::List { .. }));

        assert_eq!(rhs_after_skip.len(), 1);
        assert!(matches!(rhs_after_skip[0], Syntax::List { .. }));

        // The inner items haven't had their change set yet.
        assert_eq!(change_map.get(lhs_after_skip[0]), None);
        assert_eq!(change_map.get(rhs_after_skip[0]), None);
    }

    #[test]
    fn test_split_unchanged_toplevel_at_start() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        // Make sure that the initial unchanged node exceeds TINY_TREE_THRESHOLD.
        let lhs_nodes = parse(
            &arena,
            "(unchanged (1 2 3 4 5 6 7 8 9 10)) A B",
            &config,
            false,
        );
        let rhs_nodes = parse(
            &arena,
            "(unchanged (1 2 3 4 5 6 7 8 9 10)) X",
            &config,
            false,
        );
        init_all_info(&lhs_nodes, &rhs_nodes);

        let mut change_map = ChangeMap::default();
        let res = split_unchanged(&lhs_nodes, &rhs_nodes, &mut change_map);

        assert_eq!(res.len(), 1);
        let (lhs_after_skip, rhs_after_skip) = &res[0];

        assert_eq!(
            change_map.get(lhs_nodes[0]),
            Some(ChangeKind::Unchanged(rhs_nodes[0]))
        );
        assert_eq!(
            change_map.get(rhs_nodes[0]),
            Some(ChangeKind::Unchanged(lhs_nodes[0]))
        );

        assert_eq!(lhs_after_skip.len(), 2);
        assert_eq!(rhs_after_skip.len(), 1);
    }

    #[test]
    fn test_split_unchanged_toplevel_at_end() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(
            &arena,
            "A B (unchanged (1 2 3 4 5 6 7 8 9 10))",
            &config,
            false,
        );
        let rhs_nodes = parse(
            &arena,
            "X (unchanged (1 2 3 4 5 6 7 8 9 10))",
            &config,
            false,
        );
        init_all_info(&lhs_nodes, &rhs_nodes);

        let mut change_map = ChangeMap::default();
        let res = split_unchanged(&lhs_nodes, &rhs_nodes, &mut change_map);

        assert_eq!(res.len(), 1);
        let (lhs_after_skip, rhs_after_skip) = &res[0];

        assert_eq!(
            change_map.get(lhs_nodes[2]),
            Some(ChangeKind::Unchanged(rhs_nodes[1]))
        );
        assert_eq!(
            change_map.get(rhs_nodes[1]),
            Some(ChangeKind::Unchanged(lhs_nodes[2]))
        );

        assert_eq!(lhs_after_skip.len(), 2);
        assert_eq!(rhs_after_skip.len(), 1);
    }

    #[test]
    fn test_split_preserves_outer_delimiters() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(&arena, "(A)", &config, false);
        let rhs_nodes = parse(&arena, "(B)", &config, false);
        init_all_info(&lhs_nodes, &rhs_nodes);

        let mut change_map = ChangeMap::default();
        let res = split_unchanged(&lhs_nodes, &rhs_nodes, &mut change_map);

        assert_eq!(res.len(), 1);
        let (lhs_after_skip, rhs_after_skip) = &res[0];

        // The only possibly changed nodes are inside the lists.
        assert_eq!(lhs_after_skip.len(), 1);
        assert!(matches!(lhs_after_skip[0], Syntax::List { .. }));

        assert_eq!(rhs_after_skip.len(), 1);
        assert!(matches!(rhs_after_skip[0], Syntax::List { .. }));

        // The outer list delimiters don't have their change set yet.
        assert_eq!(change_map.get(lhs_nodes[0]), None);
        assert_eq!(change_map.get(rhs_nodes[0]), None);
    }

    #[test]
    fn test_split_unchanged_middle() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(
            &arena,
            "novel-lhs (unchanged (1 2 3 4 5 6 7 8 9 10)) novel-lhs-2",
            &config,
            false,
        );
        let rhs_nodes = parse(
            &arena,
            "novel-rhs (unchanged (1 2 3 4 5 6 7 8 9 10)) novel-rhs-2",
            &config,
            false,
        );
        init_all_info(&lhs_nodes, &rhs_nodes);

        let mut change_map = ChangeMap::default();
        let res = split_unchanged(&lhs_nodes, &rhs_nodes, &mut change_map);
        assert_eq!(
            res,
            vec![
                (vec![lhs_nodes[0]], vec![rhs_nodes[0]]),
                (vec![lhs_nodes[2]], vec![rhs_nodes[2]])
            ]
        );

        assert_eq!(
            change_map.get(lhs_nodes[1]),
            Some(ChangeKind::Unchanged(rhs_nodes[1]))
        );
        assert_eq!(
            change_map.get(rhs_nodes[1]),
            Some(ChangeKind::Unchanged(lhs_nodes[1]))
        );
    }

    #[test]
    fn test_split_unchanged_multiple() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(
            &arena,
            "novel-lhs (unchanged-1 (1 2 3 4 5 6 7 8 9 10)) (unchanged-2 (1 2 3 4 5 6 7 8 9 10)) novel-lhs-2",
            &config,
            false,
        );
        let rhs_nodes = parse(
            &arena,
            "novel-rhs (unchanged-1 (1 2 3 4 5 6 7 8 9 10)) (unchanged-2 (1 2 3 4 5 6 7 8 9 10)) novel-rhs-2",
            &config,
            false,
        );
        init_all_info(&lhs_nodes, &rhs_nodes);

        let mut change_map = ChangeMap::default();
        let res = split_unchanged(&lhs_nodes, &rhs_nodes, &mut change_map);
        assert_eq!(
            res,
            vec![
                (vec![lhs_nodes[0]], vec![rhs_nodes[0]]),
                (vec![lhs_nodes[3]], vec![rhs_nodes[3]])
            ]
        );
    }

    #[test]
    fn test_split_unchanged_outer_delimiter() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(
            &arena,
            "(novel-lhs-before (1 2 3 4 5 6 7 8 9 10) novel-lhs-after)",
            &config,
            false,
        );
        let rhs_nodes = parse(
            &arena,
            "(novel-rhs-before (1 2 3 4 5 6 7 8 9 10) novel-rhs-after)",
            &config,
            false,
        );
        init_all_info(&lhs_nodes, &rhs_nodes);

        let mut change_map = ChangeMap::default();
        let res = split_unchanged(&lhs_nodes, &rhs_nodes, &mut change_map);
        assert_eq!(res.len(), 2);

        assert_eq!(
            change_map.get(lhs_nodes[0]),
            Some(ChangeKind::Unchanged(rhs_nodes[0]))
        );
    }

    #[test]
    fn test_split_mostly_unchanged_toplevel() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(
            &arena,
            "(1 2 3 4 5 6 7 8 9 10) (91 92 93 94 95 96 97 98 99 100)",
            &config,
            false,
        );
        let rhs_nodes = parse(
            &arena,
            "(1 2 3 4 5 novel-1 6 7 8 9 10) (91 92 93 94 95 novel-2 96 97 98 99 100)",
            &config,
            false,
        );
        init_all_info(&lhs_nodes, &rhs_nodes);

        let split = split_mostly_unchanged_toplevel(&lhs_nodes, &rhs_nodes);
        assert_eq!(split.len(), 2);
    }

    #[test]
    fn test_count_common_unique() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        // There are two subtrees that are unique on both sides and
        // shared between the two sides here:
        //
        // 1: shared-1
        // 2: (shared-2a shared-2b)
        let lhs_nodes = parse(
            &arena,
            "(shared-1 (shared-2a shared-2b) not-unique not-unique)",
            &config,
            false,
        );
        let rhs_nodes = parse(
            &arena,
            "(shared-1 (shared-2a shared-2b) not-unique)",
            &config,
            false,
        );
        init_all_info(&lhs_nodes, &rhs_nodes);

        assert_eq!(count_common_unique(lhs_nodes[0], rhs_nodes[0]), 2);
    }

    #[test]
    fn test_similar_with_common_grandchildren() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(&arena, "((novel-lhs 1 2 3 4 5)) x", &config, false);
        let rhs_nodes = parse(&arena, "((novel-rhs 1 2 3 4 5)) y", &config, false);
        init_all_info(&lhs_nodes, &rhs_nodes);

        assert_eq!(
            split_mostly_unchanged_toplevel(&lhs_nodes, &rhs_nodes).len(),
            2
        );
    }
    #[test]
    fn test_similar_ignore_delimiter() {
        let arena = Arena::new();
        let config = from_language(guess_language::Language::EmacsLisp);

        let lhs_nodes = parse(&arena, "(novel-lhs 1 2 3 4 5) x", &config, false);
        let rhs_nodes = parse(&arena, "[novel-rhs 1 2 3 4 5] y", &config, false);
        init_all_info(&lhs_nodes, &rhs_nodes);

        assert_eq!(
            split_mostly_unchanged_toplevel(&lhs_nodes, &rhs_nodes).len(),
            2
        );
    }
}
