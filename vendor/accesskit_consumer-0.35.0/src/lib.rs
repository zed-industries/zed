// Copyright 2021 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

#![no_std]

extern crate alloc;

pub(crate) mod tree;
pub use tree::{ChangeHandler as TreeChangeHandler, State as TreeState, Tree};

pub(crate) mod node;
pub use node::{Node, NodeId};

pub(crate) mod filters;
pub use filters::{common_filter, common_filter_with_root_exception, FilterResult};

pub(crate) mod iterators;

pub(crate) mod text;
pub use text::{
    Position as TextPosition, Range as TextRange, RangePropertyValue as TextRangePropertyValue,
    WeakRange as WeakTextRange,
};

#[cfg(test)]
mod tests {
    use accesskit::{
        Affine, Node, NodeId as LocalNodeId, Rect, Role, Tree, TreeId, TreeUpdate, Vec2,
    };
    use alloc::vec;

    use crate::node::NodeId;
    use crate::tree::TreeIndex;
    use crate::FilterResult;

    pub fn nid(id: LocalNodeId) -> NodeId {
        NodeId::new(id, TreeIndex(0))
    }

    pub const ROOT_ID: LocalNodeId = LocalNodeId(0);
    pub const PARAGRAPH_0_ID: LocalNodeId = LocalNodeId(1);
    pub const LABEL_0_0_IGNORED_ID: LocalNodeId = LocalNodeId(2);
    pub const PARAGRAPH_1_IGNORED_ID: LocalNodeId = LocalNodeId(3);
    pub const BUTTON_1_0_HIDDEN_ID: LocalNodeId = LocalNodeId(4);
    pub const CONTAINER_1_0_0_HIDDEN_ID: LocalNodeId = LocalNodeId(5);
    pub const LABEL_1_1_ID: LocalNodeId = LocalNodeId(6);
    pub const BUTTON_1_2_HIDDEN_ID: LocalNodeId = LocalNodeId(7);
    pub const CONTAINER_1_2_0_HIDDEN_ID: LocalNodeId = LocalNodeId(8);
    pub const PARAGRAPH_2_ID: LocalNodeId = LocalNodeId(9);
    pub const LABEL_2_0_ID: LocalNodeId = LocalNodeId(10);
    pub const PARAGRAPH_3_IGNORED_ID: LocalNodeId = LocalNodeId(11);
    pub const EMPTY_CONTAINER_3_0_IGNORED_ID: LocalNodeId = LocalNodeId(12);
    pub const LINK_3_1_IGNORED_ID: LocalNodeId = LocalNodeId(13);
    pub const LABEL_3_1_0_ID: LocalNodeId = LocalNodeId(14);
    pub const BUTTON_3_2_ID: LocalNodeId = LocalNodeId(15);
    pub const EMPTY_CONTAINER_3_3_IGNORED_ID: LocalNodeId = LocalNodeId(16);

    pub fn test_tree() -> crate::tree::Tree {
        let root = {
            let mut node = Node::new(Role::RootWebArea);
            node.set_children(vec![
                PARAGRAPH_0_ID,
                PARAGRAPH_1_IGNORED_ID,
                PARAGRAPH_2_ID,
                PARAGRAPH_3_IGNORED_ID,
            ]);
            node
        };
        let paragraph_0 = {
            let mut node = Node::new(Role::Paragraph);
            node.set_children(vec![LABEL_0_0_IGNORED_ID]);
            node
        };
        let label_0_0_ignored = {
            let mut node = Node::new(Role::Label);
            node.set_value("label_0_0_ignored");
            node
        };
        let paragraph_1_ignored = {
            let mut node = Node::new(Role::Paragraph);
            node.set_transform(Affine::translate(Vec2::new(10.0, 40.0)));
            node.set_bounds(Rect {
                x0: 0.0,
                y0: 0.0,
                x1: 800.0,
                y1: 40.0,
            });
            node.set_children(vec![
                BUTTON_1_0_HIDDEN_ID,
                LABEL_1_1_ID,
                BUTTON_1_2_HIDDEN_ID,
            ]);
            node
        };
        let button_1_0_hidden = {
            let mut node = Node::new(Role::Button);
            node.set_label("button_1_0_hidden");
            node.set_hidden();
            node.set_children(vec![CONTAINER_1_0_0_HIDDEN_ID]);
            node
        };
        let container_1_0_0_hidden = {
            let mut node = Node::new(Role::GenericContainer);
            node.set_hidden();
            node
        };
        let label_1_1 = {
            let mut node = Node::new(Role::Label);
            node.set_bounds(Rect {
                x0: 10.0,
                y0: 10.0,
                x1: 90.0,
                y1: 30.0,
            });
            node.set_value("label_1_1");
            node
        };
        let button_1_2_hidden = {
            let mut node = Node::new(Role::Button);
            node.set_label("button_1_2_hidden");
            node.set_hidden();
            node.set_children(vec![CONTAINER_1_2_0_HIDDEN_ID]);
            node
        };
        let container_1_2_0_hidden = {
            let mut node = Node::new(Role::GenericContainer);
            node.set_hidden();
            node
        };
        let paragraph_2 = {
            let mut node = Node::new(Role::Paragraph);
            node.set_children(vec![LABEL_2_0_ID]);
            node
        };
        let label_2_0 = {
            let mut node = Node::new(Role::Label);
            node.set_label("label_2_0");
            node
        };
        let paragraph_3_ignored = {
            let mut node = Node::new(Role::Paragraph);
            node.set_children(vec![
                EMPTY_CONTAINER_3_0_IGNORED_ID,
                LINK_3_1_IGNORED_ID,
                BUTTON_3_2_ID,
                EMPTY_CONTAINER_3_3_IGNORED_ID,
            ]);
            node
        };
        let empty_container_3_0_ignored = Node::new(Role::GenericContainer);
        let link_3_1_ignored = {
            let mut node = Node::new(Role::Link);
            node.set_children(vec![LABEL_3_1_0_ID]);
            node
        };
        let label_3_1_0 = {
            let mut node = Node::new(Role::Label);
            node.set_value("label_3_1_0");
            node
        };
        let button_3_2 = {
            let mut node = Node::new(Role::Button);
            node.set_label("button_3_2");
            node
        };
        let empty_container_3_3_ignored = Node::new(Role::GenericContainer);
        let initial_update = TreeUpdate {
            nodes: vec![
                (ROOT_ID, root),
                (PARAGRAPH_0_ID, paragraph_0),
                (LABEL_0_0_IGNORED_ID, label_0_0_ignored),
                (PARAGRAPH_1_IGNORED_ID, paragraph_1_ignored),
                (BUTTON_1_0_HIDDEN_ID, button_1_0_hidden),
                (CONTAINER_1_0_0_HIDDEN_ID, container_1_0_0_hidden),
                (LABEL_1_1_ID, label_1_1),
                (BUTTON_1_2_HIDDEN_ID, button_1_2_hidden),
                (CONTAINER_1_2_0_HIDDEN_ID, container_1_2_0_hidden),
                (PARAGRAPH_2_ID, paragraph_2),
                (LABEL_2_0_ID, label_2_0),
                (PARAGRAPH_3_IGNORED_ID, paragraph_3_ignored),
                (EMPTY_CONTAINER_3_0_IGNORED_ID, empty_container_3_0_ignored),
                (LINK_3_1_IGNORED_ID, link_3_1_ignored),
                (LABEL_3_1_0_ID, label_3_1_0),
                (BUTTON_3_2_ID, button_3_2),
                (EMPTY_CONTAINER_3_3_IGNORED_ID, empty_container_3_3_ignored),
            ],
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: ROOT_ID,
        };
        crate::tree::Tree::new(initial_update, false)
    }

    pub fn test_tree_filter(node: &crate::Node) -> FilterResult {
        let id = node.id();
        if node.is_hidden() {
            FilterResult::ExcludeSubtree
        } else if id == nid(LABEL_0_0_IGNORED_ID)
            || id == nid(PARAGRAPH_1_IGNORED_ID)
            || id == nid(PARAGRAPH_3_IGNORED_ID)
            || id == nid(EMPTY_CONTAINER_3_0_IGNORED_ID)
            || id == nid(LINK_3_1_IGNORED_ID)
            || id == nid(EMPTY_CONTAINER_3_3_IGNORED_ID)
        {
            FilterResult::ExcludeNode
        } else {
            FilterResult::Include
        }
    }
}
