use std::hash::Hash;

use crate::changes::{insert_deep_unchanged, ChangeKind, ChangeMap};
use crate::hash::DftHashSet;
use crate::lcs_diff;
use crate::syntax::{ContentId, Syntax};

const TINY_TREE_THRESHOLD: u32 = 10;
const MOSTLY_UNCHANGED_MIN_COMMON_CHILDREN: usize = 4;

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

fn count_common_unique(lhs: &Syntax, rhs: &Syntax) -> usize {
    let rhs_unique_ids = find_all_unique_content_ids(rhs);
    count_unique_subtrees(lhs, &rhs_unique_ids)
}

fn is_mostly_unchanged_list(lhs: &Syntax, rhs: &Syntax) -> bool {
    match (lhs, rhs) {
        (Syntax::List { .. }, Syntax::List { .. }) => {
            count_common_unique(lhs, rhs) >= MOSTLY_UNCHANGED_MIN_COMMON_CHILDREN
        }
        _ => false,
    }
}

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
        [Syntax::List {
            open_content: lhs_open,
            children: lhs_children,
            close_content: lhs_close,
            ..
        }],
        [Syntax::List {
            open_content: rhs_open,
            children: rhs_children,
            close_content: rhs_close,
            ..
        }],
    ) = (lhs_nodes, rhs_nodes)
    {
        if lhs_open == rhs_open && lhs_close == rhs_close {
            return Some((lhs_children.clone(), rhs_children.clone()));
        }
    }

    None
}

fn shrink_unchanged_delimiters<'a>(
    lhs_nodes: &[&'a Syntax<'a>],
    rhs_nodes: &[&'a Syntax<'a>],
    change_map: &mut ChangeMap<'a>,
) -> (bool, Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>) {
    if let (
        [Syntax::List {
            open_content: lhs_open,
            children: lhs_children,
            close_content: lhs_close,
            ..
        }],
        [Syntax::List {
            open_content: rhs_open,
            children: rhs_children,
            close_content: rhs_close,
            ..
        }],
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

fn shrink_unchanged_at_ends<'a>(
    lhs_nodes: &[&'a Syntax<'a>],
    rhs_nodes: &[&'a Syntax<'a>],
    change_map: &mut ChangeMap<'a>,
) -> (bool, Vec<&'a Syntax<'a>>, Vec<&'a Syntax<'a>>) {
    let mut lhs_nodes = lhs_nodes;
    let mut rhs_nodes = rhs_nodes;

    let mut changed = false;
    while let (Some(lhs_node), Some(rhs_node)) = (lhs_nodes.first(), rhs_nodes.first()) {
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
