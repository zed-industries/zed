// Copyright 2021 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

// Derived from Chromium's accessibility abstraction.
// Copyright 2018 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE.chromium file.

use core::iter::FusedIterator;

use accesskit::NodeId as LocalNodeId;

use crate::{
    filters::FilterResult,
    node::{Node, NodeId},
    tree::State as TreeState,
};

/// Iterator over child NodeIds, handling both normal nodes and graft nodes.
pub enum ChildIds<'a> {
    Normal {
        parent_id: NodeId,
        children: core::slice::Iter<'a, LocalNodeId>,
    },
    Graft(Option<NodeId>),
}

impl Iterator for ChildIds<'_> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Normal {
                parent_id,
                children,
            } => children
                .next()
                .map(|child| parent_id.with_same_tree(*child)),
            Self::Graft(id) => id.take(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for ChildIds<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Normal {
                parent_id,
                children,
            } => children
                .next_back()
                .map(|child| parent_id.with_same_tree(*child)),
            Self::Graft(id) => id.take(),
        }
    }
}

impl ExactSizeIterator for ChildIds<'_> {
    fn len(&self) -> usize {
        match self {
            Self::Normal { children, .. } => children.len(),
            Self::Graft(id) => usize::from(id.is_some()),
        }
    }
}

impl FusedIterator for ChildIds<'_> {}

/// An iterator that yields following siblings of a node.
///
/// This struct is created by the [`following_siblings`](Node::following_siblings) method on [`Node`].
pub struct FollowingSiblings<'a> {
    back_position: usize,
    done: bool,
    front_position: usize,
    parent: Option<Node<'a>>,
    node_id: NodeId,
}

impl<'a> FollowingSiblings<'a> {
    pub(crate) fn new(node: Node<'a>) -> Self {
        let parent_and_index = node.parent_and_index();
        let (back_position, front_position, done) =
            if let Some((ref parent, index)) = parent_and_index {
                // Graft nodes have only one child (the subtree root)
                if parent.is_graft() {
                    (0, 0, true)
                } else {
                    let back_position = parent.data().children().len() - 1;
                    let front_position = index + 1;
                    (
                        back_position,
                        front_position,
                        front_position > back_position,
                    )
                }
            } else {
                (0, 0, true)
            };
        Self {
            back_position,
            done,
            front_position,
            parent: parent_and_index.map(|(parent, _)| parent),
            node_id: node.id,
        }
    }
}

impl Iterator for FollowingSiblings<'_> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self.front_position == self.back_position;
            let child = self
                .parent
                .as_ref()?
                .data()
                .children()
                .get(self.front_position)?;
            self.front_position += 1;
            Some(self.node_id.with_same_tree(*child))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match self.done {
            true => 0,
            _ => self.back_position + 1 - self.front_position,
        };
        (len, Some(len))
    }
}

impl DoubleEndedIterator for FollowingSiblings<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self.back_position == self.front_position;
            let child = self
                .parent
                .as_ref()?
                .data()
                .children()
                .get(self.back_position)?;
            self.back_position -= 1;
            Some(self.node_id.with_same_tree(*child))
        }
    }
}

impl ExactSizeIterator for FollowingSiblings<'_> {}

impl FusedIterator for FollowingSiblings<'_> {}

/// An iterator that yields preceding siblings of a node.
///
/// This struct is created by the [`preceding_siblings`](Node::preceding_siblings) method on [`Node`].
pub struct PrecedingSiblings<'a> {
    back_position: usize,
    done: bool,
    front_position: usize,
    parent: Option<Node<'a>>,
    node_id: NodeId,
}

impl<'a> PrecedingSiblings<'a> {
    pub(crate) fn new(node: Node<'a>) -> Self {
        let parent_and_index = node.parent_and_index();
        let (back_position, front_position, done) =
            if let Some((ref parent, index)) = parent_and_index {
                // Graft nodes have only one child (the subtree root)
                if parent.is_graft() {
                    (0, 0, true)
                } else {
                    let front_position = index.saturating_sub(1);
                    (0, front_position, index == 0)
                }
            } else {
                (0, 0, true)
            };
        Self {
            back_position,
            done,
            front_position,
            parent: parent_and_index.map(|(parent, _)| parent),
            node_id: node.id,
        }
    }
}

impl Iterator for PrecedingSiblings<'_> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self.front_position == self.back_position;
            let child = self
                .parent
                .as_ref()?
                .data()
                .children()
                .get(self.front_position)?;
            if !self.done {
                self.front_position -= 1;
            }
            Some(self.node_id.with_same_tree(*child))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match self.done {
            true => 0,
            _ => self.front_position + 1 - self.back_position,
        };
        (len, Some(len))
    }
}

impl DoubleEndedIterator for PrecedingSiblings<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self.back_position == self.front_position;
            let child = self
                .parent
                .as_ref()?
                .data()
                .children()
                .get(self.back_position)?;
            self.back_position += 1;
            Some(self.node_id.with_same_tree(*child))
        }
    }
}

impl ExactSizeIterator for PrecedingSiblings<'_> {}

impl FusedIterator for PrecedingSiblings<'_> {}

fn next_filtered_sibling<'a>(
    node: Option<Node<'a>>,
    filter: &impl Fn(&Node) -> FilterResult,
) -> Option<Node<'a>> {
    let mut next = node;
    let mut consider_children = false;
    while let Some(current) = next {
        if let Some(Some(child)) = consider_children.then(|| current.children().next()) {
            let result = filter(&child);
            next = Some(child);
            if result == FilterResult::Include {
                return next;
            }
            consider_children = result == FilterResult::ExcludeNode;
        } else if let Some(sibling) = current.following_siblings().next() {
            let result = filter(&sibling);
            next = Some(sibling);
            if result == FilterResult::Include {
                return next;
            }
            if result == FilterResult::ExcludeNode {
                consider_children = true;
            }
        } else {
            let parent = current.parent();
            next = parent;
            if let Some(parent) = parent {
                if filter(&parent) != FilterResult::ExcludeNode {
                    return None;
                }
                consider_children = false;
            } else {
                return None;
            }
        }
    }
    None
}

fn previous_filtered_sibling<'a>(
    node: Option<Node<'a>>,
    filter: &impl Fn(&Node) -> FilterResult,
) -> Option<Node<'a>> {
    let mut previous = node;
    let mut consider_children = false;
    while let Some(current) = previous {
        if let Some(Some(child)) = consider_children.then(|| current.children().next_back()) {
            let result = filter(&child);
            previous = Some(child);
            if result == FilterResult::Include {
                return previous;
            }
            consider_children = result == FilterResult::ExcludeNode;
        } else if let Some(sibling) = current.preceding_siblings().next() {
            let result = filter(&sibling);
            previous = Some(sibling);
            if result == FilterResult::Include {
                return previous;
            }
            if result == FilterResult::ExcludeNode {
                consider_children = true;
            }
        } else {
            let parent = current.parent();
            previous = parent;
            if let Some(parent) = parent {
                if filter(&parent) != FilterResult::ExcludeNode {
                    return None;
                }
                consider_children = false;
            } else {
                return None;
            }
        }
    }
    None
}

/// An iterator that yields following siblings of a node according to the
/// specified filter.
///
/// This struct is created by the [`following_filtered_siblings`](Node::following_filtered_siblings) method on [`Node`].
pub struct FollowingFilteredSiblings<'a, Filter: Fn(&Node) -> FilterResult> {
    filter: Filter,
    back: Option<Node<'a>>,
    done: bool,
    front: Option<Node<'a>>,
}

impl<'a, Filter: Fn(&Node) -> FilterResult> FollowingFilteredSiblings<'a, Filter> {
    pub(crate) fn new(node: Node<'a>, filter: Filter) -> Self {
        let front = next_filtered_sibling(Some(node), &filter);
        let back = node
            .filtered_parent(&filter)
            .and_then(|parent| parent.last_filtered_child(&filter));
        Self {
            filter,
            back,
            done: back.is_none() || front.is_none(),
            front,
        }
    }
}

impl<'a, Filter: Fn(&Node) -> FilterResult> Iterator for FollowingFilteredSiblings<'a, Filter> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self
                .front
                .as_ref()
                .zip(self.back.as_ref())
                .map(|(f, b)| f.id() == b.id())
                .unwrap_or(true);
            let current = self.front;
            self.front = next_filtered_sibling(self.front, &self.filter);
            current
        }
    }
}

impl<Filter: Fn(&Node) -> FilterResult> DoubleEndedIterator
    for FollowingFilteredSiblings<'_, Filter>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self
                .front
                .as_ref()
                .zip(self.back.as_ref())
                .map(|(f, b)| f.id() == b.id())
                .unwrap_or(true);
            let current = self.back;
            self.back = previous_filtered_sibling(self.back, &self.filter);
            current
        }
    }
}

impl<Filter: Fn(&Node) -> FilterResult> FusedIterator for FollowingFilteredSiblings<'_, Filter> {}

/// An iterator that yields preceding siblings of a node according to the
/// specified filter.
///
/// This struct is created by the [`preceding_filtered_siblings`](Node::preceding_filtered_siblings) method on [`Node`].
pub struct PrecedingFilteredSiblings<'a, Filter: Fn(&Node) -> FilterResult> {
    filter: Filter,
    back: Option<Node<'a>>,
    done: bool,
    front: Option<Node<'a>>,
}

impl<'a, Filter: Fn(&Node) -> FilterResult> PrecedingFilteredSiblings<'a, Filter> {
    pub(crate) fn new(node: Node<'a>, filter: Filter) -> Self {
        let front = previous_filtered_sibling(Some(node), &filter);
        let back = node
            .filtered_parent(&filter)
            .and_then(|parent| parent.first_filtered_child(&filter));
        Self {
            filter,
            back,
            done: back.is_none() || front.is_none(),
            front,
        }
    }
}

impl<'a, Filter: Fn(&Node) -> FilterResult> Iterator for PrecedingFilteredSiblings<'a, Filter> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self
                .front
                .as_ref()
                .zip(self.back.as_ref())
                .map(|(f, b)| f.id() == b.id())
                .unwrap_or(true);
            let current = self.front;
            self.front = previous_filtered_sibling(self.front, &self.filter);
            current
        }
    }
}

impl<Filter: Fn(&Node) -> FilterResult> DoubleEndedIterator
    for PrecedingFilteredSiblings<'_, Filter>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self
                .front
                .as_ref()
                .zip(self.back.as_ref())
                .map(|(f, b)| f.id() == b.id())
                .unwrap_or(true);
            let current = self.back;
            self.back = next_filtered_sibling(self.back, &self.filter);
            current
        }
    }
}

impl<Filter: Fn(&Node) -> FilterResult> FusedIterator for PrecedingFilteredSiblings<'_, Filter> {}

/// An iterator that yields children of a node according to the specified
/// filter.
///
/// This struct is created by the [`filtered_children`](Node::filtered_children) method on [`Node`].
pub struct FilteredChildren<'a, Filter: Fn(&Node) -> FilterResult> {
    filter: Filter,
    back: Option<Node<'a>>,
    done: bool,
    front: Option<Node<'a>>,
}

impl<'a, Filter: Fn(&Node) -> FilterResult> FilteredChildren<'a, Filter> {
    pub(crate) fn new(node: Node<'a>, filter: Filter) -> Self {
        let front = node.first_filtered_child(&filter);
        let back = node.last_filtered_child(&filter);
        Self {
            filter,
            back,
            done: back.is_none() || front.is_none(),
            front,
        }
    }
}

impl<'a, Filter: Fn(&Node) -> FilterResult> Iterator for FilteredChildren<'a, Filter> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self
                .front
                .as_ref()
                .zip(self.back.as_ref())
                .map(|(f, b)| f.id() == b.id())
                .unwrap_or(true);
            let current = self.front;
            self.front = next_filtered_sibling(self.front, &self.filter);
            current
        }
    }
}

impl<Filter: Fn(&Node) -> FilterResult> DoubleEndedIterator for FilteredChildren<'_, Filter> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self
                .front
                .as_ref()
                .zip(self.back.as_ref())
                .map(|(f, b)| f.id() == b.id())
                .unwrap_or(true);
            let current = self.back;
            self.back = previous_filtered_sibling(self.back, &self.filter);
            current
        }
    }
}

impl<Filter: Fn(&Node) -> FilterResult> FusedIterator for FilteredChildren<'_, Filter> {}

pub(crate) enum LabelledBy<'a, Filter: Fn(&Node) -> FilterResult> {
    FromDescendants(FilteredChildren<'a, Filter>),
    Explicit {
        ids: core::slice::Iter<'a, LocalNodeId>,
        tree_state: &'a TreeState,
        node_id: NodeId,
    },
}

impl<'a, Filter: Fn(&Node) -> FilterResult> Iterator for LabelledBy<'a, Filter> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::FromDescendants(iter) => iter.next(),
            Self::Explicit {
                ids,
                tree_state,
                node_id,
            } => ids
                .next()
                .map(|id| tree_state.node_by_id(node_id.with_same_tree(*id)).unwrap()),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::FromDescendants(iter) => iter.size_hint(),
            Self::Explicit { ids, .. } => ids.size_hint(),
        }
    }
}

impl<Filter: Fn(&Node) -> FilterResult> DoubleEndedIterator for LabelledBy<'_, Filter> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::FromDescendants(iter) => iter.next_back(),
            Self::Explicit {
                ids,
                tree_state,
                node_id,
            } => ids
                .next_back()
                .map(|id| tree_state.node_by_id(node_id.with_same_tree(*id)).unwrap()),
        }
    }
}

impl<Filter: Fn(&Node) -> FilterResult> FusedIterator for LabelledBy<'_, Filter> {}

#[cfg(test)]
mod tests {
    use crate::{
        filters::common_filter,
        tests::*,
        tree::{ChangeHandler, TreeIndex},
        NodeId,
    };
    use accesskit::{Node, NodeId as LocalNodeId, Role, Tree, TreeId, TreeUpdate, Uuid};
    use alloc::{vec, vec::Vec};

    #[test]
    fn following_siblings() {
        let tree = test_tree();
        assert!(tree.state().root().following_siblings().next().is_none());
        assert_eq!(0, tree.state().root().following_siblings().len());
        assert_eq!(
            [
                PARAGRAPH_1_IGNORED_ID,
                PARAGRAPH_2_ID,
                PARAGRAPH_3_IGNORED_ID
            ],
            tree.state()
                .node_by_id(nid(PARAGRAPH_0_ID))
                .unwrap()
                .following_sibling_ids()
                .map(|id| id.to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert_eq!(
            3,
            tree.state()
                .node_by_id(nid(PARAGRAPH_0_ID))
                .unwrap()
                .following_siblings()
                .len()
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
            .unwrap()
            .following_siblings()
            .next()
            .is_none());
        assert_eq!(
            0,
            tree.state()
                .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
                .unwrap()
                .following_siblings()
                .len()
        );
    }

    #[test]
    fn following_siblings_reversed() {
        let tree = test_tree();
        assert!(tree
            .state()
            .root()
            .following_siblings()
            .next_back()
            .is_none());
        assert_eq!(
            [
                PARAGRAPH_3_IGNORED_ID,
                PARAGRAPH_2_ID,
                PARAGRAPH_1_IGNORED_ID
            ],
            tree.state()
                .node_by_id(nid(PARAGRAPH_0_ID))
                .unwrap()
                .following_sibling_ids()
                .rev()
                .map(|id| id.to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
            .unwrap()
            .following_siblings()
            .next_back()
            .is_none());
    }

    #[test]
    fn preceding_siblings() {
        let tree = test_tree();
        assert!(tree.state().root().preceding_siblings().next().is_none());
        assert_eq!(0, tree.state().root().preceding_siblings().len());
        assert_eq!(
            [PARAGRAPH_2_ID, PARAGRAPH_1_IGNORED_ID, PARAGRAPH_0_ID],
            tree.state()
                .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
                .unwrap()
                .preceding_sibling_ids()
                .map(|id| id.to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert_eq!(
            3,
            tree.state()
                .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
                .unwrap()
                .preceding_siblings()
                .len()
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .preceding_siblings()
            .next()
            .is_none());
        assert_eq!(
            0,
            tree.state()
                .node_by_id(nid(PARAGRAPH_0_ID))
                .unwrap()
                .preceding_siblings()
                .len()
        );
    }

    #[test]
    fn preceding_siblings_reversed() {
        let tree = test_tree();
        assert!(tree
            .state()
            .root()
            .preceding_siblings()
            .next_back()
            .is_none());
        assert_eq!(
            [PARAGRAPH_0_ID, PARAGRAPH_1_IGNORED_ID, PARAGRAPH_2_ID],
            tree.state()
                .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
                .unwrap()
                .preceding_sibling_ids()
                .rev()
                .map(|id| id.to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .preceding_siblings()
            .next_back()
            .is_none());
    }

    #[test]
    fn following_filtered_siblings() {
        let tree = test_tree();
        assert!(tree
            .state()
            .root()
            .following_filtered_siblings(test_tree_filter)
            .next()
            .is_none());
        assert_eq!(
            [LABEL_1_1_ID, PARAGRAPH_2_ID, LABEL_3_1_0_ID, BUTTON_3_2_ID],
            tree.state()
                .node_by_id(nid(PARAGRAPH_0_ID))
                .unwrap()
                .following_filtered_siblings(test_tree_filter)
                .map(|node| node.id().to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert_eq!(
            [BUTTON_3_2_ID],
            tree.state()
                .node_by_id(nid(LABEL_3_1_0_ID))
                .unwrap()
                .following_filtered_siblings(test_tree_filter)
                .map(|node| node.id().to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
            .unwrap()
            .following_filtered_siblings(test_tree_filter)
            .next()
            .is_none());
    }

    #[test]
    fn following_filtered_siblings_reversed() {
        let tree = test_tree();
        assert!(tree
            .state()
            .root()
            .following_filtered_siblings(test_tree_filter)
            .next_back()
            .is_none());
        assert_eq!(
            [BUTTON_3_2_ID, LABEL_3_1_0_ID, PARAGRAPH_2_ID, LABEL_1_1_ID],
            tree.state()
                .node_by_id(nid(PARAGRAPH_0_ID))
                .unwrap()
                .following_filtered_siblings(test_tree_filter)
                .rev()
                .map(|node| node.id().to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert_eq!(
            [BUTTON_3_2_ID,],
            tree.state()
                .node_by_id(nid(LABEL_3_1_0_ID))
                .unwrap()
                .following_filtered_siblings(test_tree_filter)
                .rev()
                .map(|node| node.id().to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
            .unwrap()
            .following_filtered_siblings(test_tree_filter)
            .next_back()
            .is_none());
    }

    #[test]
    fn preceding_filtered_siblings() {
        let tree = test_tree();
        assert!(tree
            .state()
            .root()
            .preceding_filtered_siblings(test_tree_filter)
            .next()
            .is_none());
        assert_eq!(
            [PARAGRAPH_2_ID, LABEL_1_1_ID, PARAGRAPH_0_ID],
            tree.state()
                .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
                .unwrap()
                .preceding_filtered_siblings(test_tree_filter)
                .map(|node| node.id().to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert_eq!(
            [PARAGRAPH_2_ID, LABEL_1_1_ID, PARAGRAPH_0_ID],
            tree.state()
                .node_by_id(nid(LABEL_3_1_0_ID))
                .unwrap()
                .preceding_filtered_siblings(test_tree_filter)
                .map(|node| node.id().to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .preceding_filtered_siblings(test_tree_filter)
            .next()
            .is_none());
    }

    #[test]
    fn preceding_filtered_siblings_reversed() {
        let tree = test_tree();
        assert!(tree
            .state()
            .root()
            .preceding_filtered_siblings(test_tree_filter)
            .next_back()
            .is_none());
        assert_eq!(
            [PARAGRAPH_0_ID, LABEL_1_1_ID, PARAGRAPH_2_ID],
            tree.state()
                .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
                .unwrap()
                .preceding_filtered_siblings(test_tree_filter)
                .rev()
                .map(|node| node.id().to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert_eq!(
            [PARAGRAPH_0_ID, LABEL_1_1_ID, PARAGRAPH_2_ID],
            tree.state()
                .node_by_id(nid(LABEL_3_1_0_ID))
                .unwrap()
                .preceding_filtered_siblings(test_tree_filter)
                .rev()
                .map(|node| node.id().to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .preceding_filtered_siblings(test_tree_filter)
            .next_back()
            .is_none());
    }

    #[test]
    fn filtered_children() {
        let tree = test_tree();
        assert_eq!(
            [
                PARAGRAPH_0_ID,
                LABEL_1_1_ID,
                PARAGRAPH_2_ID,
                LABEL_3_1_0_ID,
                BUTTON_3_2_ID
            ],
            tree.state()
                .root()
                .filtered_children(test_tree_filter)
                .map(|node| node.id().to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .filtered_children(test_tree_filter)
            .next()
            .is_none());
        assert!(tree
            .state()
            .node_by_id(nid(LABEL_0_0_IGNORED_ID))
            .unwrap()
            .filtered_children(test_tree_filter)
            .next()
            .is_none());
    }

    #[test]
    fn filtered_children_reversed() {
        let tree = test_tree();
        assert_eq!(
            [
                BUTTON_3_2_ID,
                LABEL_3_1_0_ID,
                PARAGRAPH_2_ID,
                LABEL_1_1_ID,
                PARAGRAPH_0_ID
            ],
            tree.state()
                .root()
                .filtered_children(test_tree_filter)
                .rev()
                .map(|node| node.id().to_components().0)
                .collect::<Vec<LocalNodeId>>()[..]
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .filtered_children(test_tree_filter)
            .next_back()
            .is_none());
        assert!(tree
            .state()
            .node_by_id(nid(LABEL_0_0_IGNORED_ID))
            .unwrap()
            .filtered_children(test_tree_filter)
            .next_back()
            .is_none());
    }

    #[test]
    fn graft_node_without_subtree_has_no_filtered_children() {
        let subtree_id = TreeId(Uuid::from_u128(1));

        let update = TreeUpdate {
            nodes: vec![
                (LocalNodeId(0), {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![LocalNodeId(1)]);
                    node
                }),
                (LocalNodeId(1), {
                    let mut node = Node::new(Role::GenericContainer);
                    node.set_tree_id(subtree_id);
                    node
                }),
            ],
            tree: Some(Tree::new(LocalNodeId(0))),
            tree_id: TreeId::ROOT,
            focus: LocalNodeId(0),
        };
        let tree = crate::Tree::new(update, false);

        let graft_node_id = NodeId::new(LocalNodeId(1), TreeIndex(0));
        let graft_node = tree.state().node_by_id(graft_node_id).unwrap();
        assert!(graft_node.filtered_children(common_filter).next().is_none());
    }

    #[test]
    fn filtered_children_crosses_subtree_boundary() {
        struct NoOpHandler;
        impl ChangeHandler for NoOpHandler {
            fn node_added(&mut self, _: &crate::Node) {}
            fn node_updated(&mut self, _: &crate::Node, _: &crate::Node) {}
            fn focus_moved(&mut self, _: Option<&crate::Node>, _: Option<&crate::Node>) {}
            fn node_removed(&mut self, _: &crate::Node) {}
        }

        let subtree_id = TreeId(Uuid::from_u128(1));

        let update = TreeUpdate {
            nodes: vec![
                (LocalNodeId(0), {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![LocalNodeId(1)]);
                    node
                }),
                (LocalNodeId(1), {
                    let mut node = Node::new(Role::GenericContainer);
                    node.set_tree_id(subtree_id);
                    node
                }),
            ],
            tree: Some(Tree::new(LocalNodeId(0))),
            tree_id: TreeId::ROOT,
            focus: LocalNodeId(0),
        };
        let mut tree = crate::Tree::new(update, false);

        let subtree_update = TreeUpdate {
            nodes: vec![
                (LocalNodeId(0), {
                    let mut node = Node::new(Role::Document);
                    node.set_children(vec![LocalNodeId(1)]);
                    node
                }),
                (LocalNodeId(1), Node::new(Role::Button)),
            ],
            tree: Some(Tree::new(LocalNodeId(0))),
            tree_id: subtree_id,
            focus: LocalNodeId(0),
        };
        tree.update_and_process_changes(subtree_update, &mut NoOpHandler);

        let root = tree.state().root();
        let filtered_children: Vec<_> = root.filtered_children(common_filter).collect();

        assert_eq!(1, filtered_children.len());
        let subtree_root_id = NodeId::new(LocalNodeId(0), TreeIndex(1));
        assert_eq!(subtree_root_id, filtered_children[0].id());

        let document = &filtered_children[0];
        let doc_children: Vec<_> = document.filtered_children(common_filter).collect();
        assert_eq!(1, doc_children.len());
        let button_id = NodeId::new(LocalNodeId(1), TreeIndex(1));
        assert_eq!(button_id, doc_children[0].id());
    }
}
