//! Find nodes that are obviously unchanged, so we can run the main
//! diff on smaller inputs.

use crate::changes::{ChangeKind, ChangeMap, insert_deep_unchanged};
use crate::syntax_tree::{SyntaxId, SyntaxTree};

/// A region of nodes that may have changes and needs diffing.
pub struct UnchangedRegion {
    pub lhs_ids: Vec<SyntaxId>,
    pub rhs_ids: Vec<SyntaxId>,
}

/// Mark obviously unchanged nodes and return regions that need diffing.
///
/// This is a pre-processing step before running the full graph-based diff.
/// By identifying large unchanged subtrees upfront, we can significantly
/// reduce the size of the diff graph.
pub fn mark_unchanged<'a>(
    lhs_tree: &'a SyntaxTree,
    rhs_tree: &'a SyntaxTree,
    change_map: &mut ChangeMap,
) -> Vec<UnchangedRegion> {
    let lhs_root = match lhs_tree.root() {
        Some(id) => id,
        None => return vec![],
    };
    let rhs_root = match rhs_tree.root() {
        Some(id) => id,
        None => return vec![],
    };

    let lhs_children: Vec<_> = lhs_tree.children(lhs_root).collect();
    let rhs_children: Vec<_> = rhs_tree.children(rhs_root).collect();

    let (lhs_nodes, rhs_nodes) =
        shrink_unchanged_at_ends(lhs_tree, &lhs_children, rhs_tree, &rhs_children, change_map);

    if lhs_nodes.is_empty() && rhs_nodes.is_empty() {
        return vec![];
    }

    vec![UnchangedRegion {
        lhs_ids: lhs_nodes,
        rhs_ids: rhs_nodes,
    }]
}

/// Skip nodes at the beginning and end that are identical.
fn shrink_unchanged_at_ends(
    lhs_tree: &SyntaxTree,
    lhs_nodes: &[SyntaxId],
    rhs_tree: &SyntaxTree,
    rhs_nodes: &[SyntaxId],
    change_map: &mut ChangeMap,
) -> (Vec<SyntaxId>, Vec<SyntaxId>) {
    let mut lhs_nodes = lhs_nodes;
    let mut rhs_nodes = rhs_nodes;

    while let (Some(&lhs_id), Some(&rhs_id)) = (lhs_nodes.first(), rhs_nodes.first()) {
        let lhs_node = lhs_tree.get(lhs_id);
        let rhs_node = rhs_tree.get(rhs_id);

        if lhs_node.structural_hash() == rhs_node.structural_hash() {
            insert_deep_unchanged(lhs_tree, lhs_id, rhs_tree, rhs_id, change_map);
            insert_deep_unchanged(rhs_tree, rhs_id, lhs_tree, lhs_id, change_map);

            lhs_nodes = &lhs_nodes[1..];
            rhs_nodes = &rhs_nodes[1..];
        } else {
            break;
        }
    }

    while let (Some(&lhs_id), Some(&rhs_id)) = (lhs_nodes.last(), rhs_nodes.last()) {
        let lhs_node = lhs_tree.get(lhs_id);
        let rhs_node = rhs_tree.get(rhs_id);

        if lhs_node.structural_hash() == rhs_node.structural_hash() {
            insert_deep_unchanged(lhs_tree, lhs_id, rhs_tree, rhs_id, change_map);
            insert_deep_unchanged(rhs_tree, rhs_id, lhs_tree, lhs_id, change_map);

            lhs_nodes = &lhs_nodes[..lhs_nodes.len() - 1];
            rhs_nodes = &rhs_nodes[..rhs_nodes.len() - 1];
        } else {
            break;
        }
    }

    if lhs_nodes.len() == 1 && rhs_nodes.len() == 1 {
        shrink_unchanged_delimiters(lhs_tree, lhs_nodes, rhs_tree, rhs_nodes, change_map)
    } else {
        (lhs_nodes.to_vec(), rhs_nodes.to_vec())
    }
}

/// If both sides are a single list with matching delimiters, mark the
/// delimiters as unchanged and recurse into children.
fn shrink_unchanged_delimiters(
    lhs_tree: &SyntaxTree,
    lhs_nodes: &[SyntaxId],
    rhs_tree: &SyntaxTree,
    rhs_nodes: &[SyntaxId],
    change_map: &mut ChangeMap,
) -> (Vec<SyntaxId>, Vec<SyntaxId>) {
    let (&lhs_id, &rhs_id) = match (lhs_nodes.first(), rhs_nodes.first()) {
        (Some(l), Some(r)) => (l, r),
        _ => return (lhs_nodes.to_vec(), rhs_nodes.to_vec()),
    };

    let lhs_node = lhs_tree.get(lhs_id);
    let rhs_node = rhs_tree.get(rhs_id);

    if !lhs_node.is_list() || !rhs_node.is_list() {
        return (lhs_nodes.to_vec(), rhs_nodes.to_vec());
    }

    if lhs_node.open_delimiter() != rhs_node.open_delimiter()
        || lhs_node.close_delimiter() != rhs_node.close_delimiter()
    {
        return (lhs_nodes.to_vec(), rhs_nodes.to_vec());
    }

    let lhs_children: Vec<_> = lhs_tree.children(lhs_id).collect();
    let rhs_children: Vec<_> = rhs_tree.children(rhs_id).collect();

    let (shrunk_lhs, shrunk_rhs) =
        shrink_unchanged_at_ends(lhs_tree, &lhs_children, rhs_tree, &rhs_children, change_map);

    if shrunk_lhs.len() < lhs_children.len() || shrunk_rhs.len() < rhs_children.len() {
        change_map.insert(lhs_id, ChangeKind::Unchanged(rhs_id));
        change_map.insert(rhs_id, ChangeKind::Unchanged(lhs_id));
        (shrunk_lhs, shrunk_rhs)
    } else {
        (lhs_nodes.to_vec(), rhs_nodes.to_vec())
    }
}
