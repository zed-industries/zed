// Copyright 2023 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{Rect, Role};

use crate::node::Node;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterResult {
    Include,
    ExcludeNode,
    ExcludeSubtree,
}

fn common_filter_base(node: &Node) -> Option<FilterResult> {
    if node.is_focused() {
        return Some(FilterResult::Include);
    }

    if node.is_hidden() {
        return Some(FilterResult::ExcludeSubtree);
    }

    // Graft nodes are transparent containers pointing to a subtree
    if node.is_graft() {
        return Some(FilterResult::ExcludeNode);
    }

    let role = node.role();
    if role == Role::GenericContainer || role == Role::TextRun {
        return Some(FilterResult::ExcludeNode);
    }

    None
}

fn common_filter_without_parent_checks(node: &Node) -> FilterResult {
    common_filter_base(node).unwrap_or(FilterResult::Include)
}

fn is_first_sibling_in_parent_bbox<'a>(
    mut siblings: impl Iterator<Item = Node<'a>>,
    parent_bbox: Rect,
) -> bool {
    siblings.next().is_some_and(|sibling| {
        sibling
            .bounding_box()
            .is_some_and(|bbox| !bbox.intersect(parent_bbox).is_empty())
    })
}

pub fn common_filter(node: &Node) -> FilterResult {
    if let Some(result) = common_filter_base(node) {
        return result;
    }

    if let Some(parent) = node.parent() {
        if common_filter(&parent) == FilterResult::ExcludeSubtree {
            return FilterResult::ExcludeSubtree;
        }
    }

    if let Some(parent) = node.filtered_parent(&common_filter_without_parent_checks) {
        if parent.clips_children() {
            // If the parent clips its children, then exclude this subtree
            // if this child's bounding box isn't inside the parent's bounding
            // box, and if the previous or next filtered sibling isn't inside
            // the parent's bounding box either. The latter condition is meant
            // to allow off-screen items to be seen by consumers so they can be
            // scrolled into view.
            if let Some(bbox) = node.bounding_box() {
                if let Some(parent_bbox) = parent.bounding_box() {
                    if bbox.intersect(parent_bbox).is_empty()
                        && !(is_first_sibling_in_parent_bbox(
                            node.following_filtered_siblings(&common_filter_without_parent_checks),
                            parent_bbox,
                        ) || is_first_sibling_in_parent_bbox(
                            node.preceding_filtered_siblings(&common_filter_without_parent_checks),
                            parent_bbox,
                        ))
                    {
                        return FilterResult::ExcludeSubtree;
                    }
                }
            }
        }
    }

    FilterResult::Include
}

pub fn common_filter_with_root_exception(node: &Node) -> FilterResult {
    if node.is_root() {
        return FilterResult::Include;
    }
    common_filter(node)
}

#[cfg(test)]
mod tests {
    use accesskit::{Node, NodeId, Rect, Role, Tree, TreeId, TreeUpdate};
    use alloc::vec;

    use super::{
        common_filter, common_filter_with_root_exception,
        FilterResult::{self, *},
    };
    use crate::tests::nid;

    #[track_caller]
    fn assert_filter_result(expected: FilterResult, tree: &crate::Tree, id: NodeId) {
        assert_eq!(
            expected,
            common_filter(&tree.state().node_by_id(nid(id)).unwrap())
        );
    }

    #[test]
    fn normal() {
        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![NodeId(1)]);
                    node
                }),
                (NodeId(1), Node::new(Role::Button)),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(0),
        };
        let tree = crate::Tree::new(update, false);
        assert_filter_result(Include, &tree, NodeId(1));
    }

    #[test]
    fn hidden() {
        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![NodeId(1)]);
                    node
                }),
                (NodeId(1), {
                    let mut node = Node::new(Role::Button);
                    node.set_hidden();
                    node
                }),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(0),
        };
        let tree = crate::Tree::new(update, false);
        assert_filter_result(ExcludeSubtree, &tree, NodeId(1));
    }

    #[test]
    fn hidden_but_focused() {
        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![NodeId(1)]);
                    node
                }),
                (NodeId(1), {
                    let mut node = Node::new(Role::Button);
                    node.set_hidden();
                    node
                }),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(1),
        };
        let tree = crate::Tree::new(update, true);
        assert_filter_result(Include, &tree, NodeId(1));
    }

    #[test]
    fn generic_container() {
        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::GenericContainer);
                    node.set_children(vec![NodeId(1)]);
                    node
                }),
                (NodeId(1), Node::new(Role::Button)),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(0),
        };
        let tree = crate::Tree::new(update, false);
        assert_filter_result(ExcludeNode, &tree, NodeId(0));
        assert_eq!(
            Include,
            common_filter_with_root_exception(&tree.state().node_by_id(nid(NodeId(0))).unwrap())
        );
        assert_filter_result(Include, &tree, NodeId(1));
    }

    #[test]
    fn hidden_parent() {
        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::GenericContainer);
                    node.set_hidden();
                    node.set_children(vec![NodeId(1)]);
                    node
                }),
                (NodeId(1), Node::new(Role::Button)),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(0),
        };
        let tree = crate::Tree::new(update, false);
        assert_filter_result(ExcludeSubtree, &tree, NodeId(0));
        assert_filter_result(ExcludeSubtree, &tree, NodeId(1));
    }

    #[test]
    fn hidden_parent_but_focused() {
        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::GenericContainer);
                    node.set_hidden();
                    node.set_children(vec![NodeId(1)]);
                    node
                }),
                (NodeId(1), Node::new(Role::Button)),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(1),
        };
        let tree = crate::Tree::new(update, true);
        assert_filter_result(ExcludeSubtree, &tree, NodeId(0));
        assert_filter_result(Include, &tree, NodeId(1));
    }

    #[test]
    fn text_run() {
        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::TextInput);
                    node.set_children(vec![NodeId(1)]);
                    node
                }),
                (NodeId(1), Node::new(Role::TextRun)),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(0),
        };
        let tree = crate::Tree::new(update, false);
        assert_filter_result(ExcludeNode, &tree, NodeId(1));
    }

    fn clipped_children_test_tree() -> crate::Tree {
        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::ScrollView);
                    node.set_clips_children();
                    node.set_bounds(Rect::new(0.0, 0.0, 30.0, 30.0));
                    node.set_children(vec![
                        NodeId(1),
                        NodeId(2),
                        NodeId(3),
                        NodeId(4),
                        NodeId(5),
                        NodeId(6),
                        NodeId(7),
                        NodeId(8),
                        NodeId(9),
                        NodeId(10),
                        NodeId(11),
                    ]);
                    node
                }),
                (NodeId(1), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_bounds(Rect::new(0.0, -30.0, 30.0, -20.0));
                    node
                }),
                (NodeId(2), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_bounds(Rect::new(0.0, -20.0, 30.0, -10.0));
                    node
                }),
                (NodeId(3), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_bounds(Rect::new(0.0, -10.0, 30.0, 0.0));
                    node
                }),
                (NodeId(4), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_hidden();
                    node
                }),
                (NodeId(5), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_bounds(Rect::new(0.0, 0.0, 30.0, 10.0));
                    node
                }),
                (NodeId(6), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_bounds(Rect::new(0.0, 10.0, 30.0, 20.0));
                    node
                }),
                (NodeId(7), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_bounds(Rect::new(0.0, 20.0, 30.0, 30.0));
                    node
                }),
                (NodeId(8), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_hidden();
                    node
                }),
                (NodeId(9), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_bounds(Rect::new(0.0, 30.0, 30.0, 40.0));
                    node
                }),
                (NodeId(10), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_bounds(Rect::new(0.0, 40.0, 30.0, 50.0));
                    node
                }),
                (NodeId(11), {
                    let mut node = Node::new(Role::Unknown);
                    node.set_bounds(Rect::new(0.0, 50.0, 30.0, 60.0));
                    node
                }),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(0),
        };
        crate::Tree::new(update, false)
    }

    #[test]
    fn clipped_children_excluded_above() {
        let tree = clipped_children_test_tree();
        assert_filter_result(ExcludeSubtree, &tree, NodeId(1));
        assert_filter_result(ExcludeSubtree, &tree, NodeId(2));
    }

    #[test]
    fn clipped_children_included_above() {
        let tree = clipped_children_test_tree();
        assert_filter_result(Include, &tree, NodeId(3));
    }

    #[test]
    fn clipped_children_hidden() {
        let tree = clipped_children_test_tree();
        assert_filter_result(ExcludeSubtree, &tree, NodeId(4));
        assert_filter_result(ExcludeSubtree, &tree, NodeId(8));
    }

    #[test]
    fn clipped_children_visible() {
        let tree = clipped_children_test_tree();
        assert_filter_result(Include, &tree, NodeId(5));
        assert_filter_result(Include, &tree, NodeId(6));
        assert_filter_result(Include, &tree, NodeId(7));
    }

    #[test]
    fn clipped_children_included_below() {
        let tree = clipped_children_test_tree();
        assert_filter_result(Include, &tree, NodeId(9));
    }

    #[test]
    fn clipped_children_excluded_below() {
        let tree = clipped_children_test_tree();
        assert_filter_result(ExcludeSubtree, &tree, NodeId(10));
        assert_filter_result(ExcludeSubtree, &tree, NodeId(11));
    }

    #[test]
    fn graft_node() {
        use accesskit::Uuid;

        let subtree_id = TreeId(Uuid::from_u128(1));
        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![NodeId(1)]);
                    node
                }),
                (NodeId(1), {
                    let mut node = Node::new(Role::GenericContainer);
                    node.set_tree_id(subtree_id);
                    node
                }),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(0),
        };
        let tree = crate::Tree::new(update, false);
        assert_filter_result(ExcludeNode, &tree, NodeId(1));
    }
}
