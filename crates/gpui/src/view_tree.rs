use crate::{
    Bounds, EntityId, GlobalElementId, Pixels,
    window::{PaintIndex, PrepaintStateIndex},
};
use collections::FxHashMap;
use slotmap::SlotMap;
use smallvec::SmallVec;
use std::ops::Range;

slotmap::new_key_type! {
    /// Identifies one mount of a view in a window's [`ViewTree`].
    pub(crate) struct ViewNodeId;
}

/// One mount of an entity-backed view at one place in the element tree. The same
/// `Entity<V>` rendered in two places is two nodes.
pub(crate) struct ViewNode {
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "read by the tree's consumers, which land separately"
        )
    )]
    pub(crate) entity_id: EntityId,
    pub(crate) global_id: GlobalElementId,
    pub(crate) parent: Option<ViewNodeId>,
    pub(crate) children: SmallVec<[ViewNodeId; 4]>,
    pub(crate) bounds: Option<Bounds<Pixels>>,
    /// Where this node's prepaint output sits in the frame. `None` when the node was not
    /// prepainted itself this frame, because a cached ancestor reused its output wholesale.
    pub(crate) prepaint_range: Option<Range<PrepaintStateIndex>>,
    /// Where this node's paint output sits in the frame, with the same caveat as
    /// `prepaint_range`.
    pub(crate) paint_range: Option<Range<PaintIndex>>,
    visited_frame: u64,
}

/// The entity-backed views mounted in a window, as a tree.
///
/// GPUI still draws every frame from its flat per-frame arrays; this tree indexes them.
/// A node is mounted the first frame its view is laid out at a given [`GlobalElementId`]
/// and unmounted at the end of the first frame in which it is not visited. A view inside
/// a cached view that was reused stays mounted, since its output was reused with it.
#[derive(Default)]
pub(crate) struct ViewTree {
    nodes: SlotMap<ViewNodeId, ViewNode>,
    nodes_by_global_id: FxHashMap<GlobalElementId, ViewNodeId>,
    roots: Vec<ViewNodeId>,
    stack: Vec<ViewNodeId>,
    // Children lists are rebuilt from this at the end of the frame, so a node's children
    // are in the order they were first visited.
    visit_order: Vec<ViewNodeId>,
    frame: u64,
}

impl ViewTree {
    pub(crate) fn begin_frame(&mut self) {
        debug_assert!(self.stack.is_empty());
        self.frame += 1;
        self.visit_order.clear();
    }

    /// Unmounts every node that was not visited this frame and rebuilds the tree's
    /// structure from the nodes that were.
    pub(crate) fn finish_frame(&mut self) {
        debug_assert!(self.stack.is_empty());
        let frame = self.frame;
        let nodes_by_global_id = &mut self.nodes_by_global_id;
        self.nodes.retain(|_, node| {
            let visited = node.visited_frame == frame;
            if !visited {
                nodes_by_global_id.remove(&node.global_id);
            }
            visited
        });

        for node_id in &self.visit_order {
            if let Some(node) = self.nodes.get_mut(*node_id) {
                node.children.clear();
            }
        }
        self.roots.clear();
        for node_id in self.visit_order.iter().copied() {
            let parent = self.nodes.get(node_id).and_then(|node| node.parent);
            match parent.and_then(|parent| self.nodes.get_mut(parent)) {
                Some(parent) => parent.children.push(node_id),
                None => self.roots.push(node_id),
            }
        }
    }

    /// Returns the node for the view laid out at `global_id`, mounting it under the
    /// current node if this is the first frame it has been seen.
    pub(crate) fn visit(&mut self, global_id: &GlobalElementId, entity_id: EntityId) -> ViewNodeId {
        let frame = self.frame;
        let parent = self.stack.last().copied();
        if let Some(node_id) = self.nodes_by_global_id.get(global_id).copied()
            && let Some(node) = self.nodes.get_mut(node_id)
        {
            if node.visited_frame != frame {
                node.visited_frame = frame;
                node.parent = parent;
                node.bounds = None;
                node.prepaint_range = None;
                node.paint_range = None;
                self.visit_order.push(node_id);
            }
            return node_id;
        }

        let node_id = self.nodes.insert(ViewNode {
            entity_id,
            global_id: global_id.clone(),
            parent,
            children: SmallVec::new(),
            bounds: None,
            prepaint_range: None,
            paint_range: None,
            visited_frame: frame,
        });
        self.nodes_by_global_id.insert(global_id.clone(), node_id);
        self.visit_order.push(node_id);
        node_id
    }

    /// Keeps the descendants of `node_id` mounted, for a node whose output from the last
    /// frame was reused without drawing its descendants again.
    pub(crate) fn retain_descendants(&mut self, node_id: ViewNodeId) {
        let frame = self.frame;
        let mut pending: SmallVec<[ViewNodeId; 8]> = self
            .nodes
            .get(node_id)
            .map(|node| node.children.iter().rev().copied().collect())
            .unwrap_or_default();
        while let Some(child_id) = pending.pop() {
            let Some(child) = self.nodes.get_mut(child_id) else {
                continue;
            };
            if child.visited_frame == frame {
                continue;
            }
            child.visited_frame = frame;
            child.prepaint_range = None;
            child.paint_range = None;
            self.visit_order.push(child_id);
            pending.extend(child.children.iter().rev().copied());
        }
    }

    pub(crate) fn push(&mut self, node_id: ViewNodeId) {
        self.stack.push(node_id);
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn current(&self) -> Option<ViewNodeId> {
        self.stack.last().copied()
    }

    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "read by the tree's consumers, which land separately"
        )
    )]
    pub(crate) fn node(&self, node_id: ViewNodeId) -> Option<&ViewNode> {
        self.nodes.get(node_id)
    }

    pub(crate) fn node_mut(&mut self, node_id: ViewNodeId) -> Option<&mut ViewNode> {
        self.nodes.get_mut(node_id)
    }

    /// The nodes with no parent, in the order they were drawn: the window's root view,
    /// then prompts, drags and tooltips.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "read by the tree's consumers, which land separately"
        )
    )]
    pub(crate) fn roots(&self) -> &[ViewNodeId] {
        &self.roots
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Context, Entity, IntoElement, Render, StyleRefinement, TestAppContext, VisualTestContext,
        Window, deferred, div, prelude::*, px,
    };

    struct Leaf;

    impl Render for Leaf {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size(px(10.))
        }
    }

    struct Branch {
        child: Entity<Leaf>,
    }

    impl Render for Branch {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size(px(10.)).child(self.child.clone())
        }
    }

    #[derive(Default)]
    struct Root {
        children: Vec<Entity<Leaf>>,
        cached_branch: Option<Entity<Branch>>,
        deferred_child: Option<Entity<Leaf>>,
        branch: Option<Entity<Branch>>,
    }

    impl Render for Root {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .children(self.children.iter().cloned())
                .when_some(self.cached_branch.clone(), |this, branch| {
                    this.child(branch.cached(StyleRefinement::default().size(px(10.))))
                })
                .when_some(self.deferred_child.clone(), |this, child| {
                    this.child(deferred(child))
                })
                .when_some(self.branch.clone(), |this, branch| this.child(branch))
        }
    }

    fn draw(cx: &mut VisualTestContext) {
        cx.update(|window, cx| window.draw(cx).clear(cx));
    }

    fn nodes_of(entity: EntityId, cx: &mut VisualTestContext) -> Vec<ViewNodeId> {
        cx.update(|window, _| {
            window
                .view_tree
                .nodes
                .iter()
                .filter(|(_, node)| node.entity_id == entity)
                .map(|(node_id, _)| node_id)
                .collect()
        })
    }

    fn node_of(entity: EntityId, cx: &mut VisualTestContext) -> ViewNodeId {
        let nodes = nodes_of(entity, cx);
        assert_eq!(nodes.len(), 1, "expected one node for {entity:?}");
        nodes[0]
    }

    fn child_entities(node_id: ViewNodeId, cx: &mut VisualTestContext) -> Vec<EntityId> {
        cx.update(|window, _| {
            let tree = &window.view_tree;
            tree.node(node_id)
                .map(|node| {
                    node.children
                        .iter()
                        .filter_map(|child| tree.node(*child))
                        .map(|child| child.entity_id)
                        .collect()
                })
                .unwrap_or_default()
        })
    }

    #[gpui::test]
    fn mounts_views_as_a_tree_in_draw_order(cx: &mut TestAppContext) {
        let (root, cx) = cx.add_window_view(|_, cx| Root {
            children: vec![cx.new(|_| Leaf), cx.new(|_| Leaf)],
            ..Root::default()
        });
        draw(cx);

        let leaves = root.read_with(cx, |root, _| {
            root.children
                .iter()
                .map(|child| child.entity_id())
                .collect::<Vec<_>>()
        });
        let root_node = node_of(root.entity_id(), cx);
        assert_eq!(
            cx.update(|window, _| window.view_tree.roots().to_vec()),
            [root_node]
        );
        assert_eq!(child_entities(root_node, cx), leaves);
        assert_eq!(cx.update(|window, _| window.view_tree.len()), 3);
    }

    #[gpui::test]
    fn unmounts_views_that_stop_rendering_and_keeps_the_rest(cx: &mut TestAppContext) {
        let (root, cx) = cx.add_window_view(|_, cx| Root {
            children: vec![cx.new(|_| Leaf), cx.new(|_| Leaf)],
            ..Root::default()
        });
        draw(cx);
        let first = root.read_with(cx, |root, _| root.children[0].entity_id());
        let first_node = node_of(first, cx);

        root.update(cx, |root, cx| {
            root.children.pop();
            cx.notify();
        });
        draw(cx);

        assert_eq!(cx.update(|window, _| window.view_tree.len()), 2);
        assert_eq!(node_of(first, cx), first_node);
        assert_eq!(child_entities(node_of(root.entity_id(), cx), cx), [first]);
    }

    #[gpui::test]
    fn refresh_redraws_without_unmounting(cx: &mut TestAppContext) {
        let (root, cx) = cx.add_window_view(|_, cx| Root {
            children: vec![cx.new(|_| Leaf)],
            ..Root::default()
        });
        draw(cx);
        let leaf = root.read_with(cx, |root, _| root.children[0].entity_id());
        let leaf_node = node_of(leaf, cx);

        cx.update(|window, _| window.refresh());
        draw(cx);

        assert_eq!(node_of(leaf, cx), leaf_node);
    }

    #[gpui::test]
    fn reused_cached_view_keeps_its_descendants_mounted(cx: &mut TestAppContext) {
        let (root, cx) = cx.add_window_view(|_, cx| {
            let child = cx.new(|_| Leaf);
            Root {
                cached_branch: Some(cx.new(|_| Branch { child })),
                ..Root::default()
            }
        });
        draw(cx);
        let (branch, leaf) = root.read_with(cx, |root, cx| {
            let branch = root.cached_branch.clone().expect("cached branch");
            let leaf = branch.read(cx).child.entity_id();
            (branch.entity_id(), leaf)
        });
        let leaf_node = node_of(leaf, cx);

        for _ in 0..2 {
            root.update(cx, |_, cx| cx.notify());
            draw(cx);

            let branch_node = node_of(branch, cx);
            assert_eq!(node_of(leaf, cx), leaf_node);
            assert_eq!(child_entities(branch_node, cx), [leaf]);
            let (branch_prepainted, leaf_prepainted) = cx.update(|window, _| {
                let tree = &window.view_tree;
                (
                    tree.node(branch_node)
                        .and_then(|node| node.prepaint_range.clone()),
                    tree.node(leaf_node)
                        .and_then(|node| node.prepaint_range.clone()),
                )
            });
            assert!(
                branch_prepainted.is_some(),
                "the cached view itself is visited"
            );
            assert!(
                leaf_prepainted.is_none(),
                "its descendants were reused, not prepainted"
            );
        }
    }

    #[gpui::test]
    fn deferred_views_mount_under_the_view_that_deferred_them(cx: &mut TestAppContext) {
        let (root, cx) = cx.add_window_view(|_, cx| Root {
            deferred_child: Some(cx.new(|_| Leaf)),
            ..Root::default()
        });
        draw(cx);

        let deferred_child = root.read_with(cx, |root, _| {
            root.deferred_child.as_ref().map(|child| child.entity_id())
        });
        let root_node = node_of(root.entity_id(), cx);
        assert_eq!(
            cx.update(|window, _| window.view_tree.roots().to_vec()),
            [root_node]
        );
        assert_eq!(
            child_entities(root_node, cx),
            deferred_child.into_iter().collect::<Vec<_>>()
        );
    }

    #[gpui::test]
    fn one_entity_mounted_in_two_places_is_two_nodes(cx: &mut TestAppContext) {
        let (root, cx) = cx.add_window_view(|_, cx| {
            let leaf = cx.new(|_| Leaf);
            Root {
                children: vec![leaf.clone()],
                branch: Some(cx.new(|_| Branch { child: leaf })),
                ..Root::default()
            }
        });
        draw(cx);

        let leaf = root.read_with(cx, |root, _| root.children[0].entity_id());
        assert_eq!(nodes_of(leaf, cx).len(), 2);
    }
}
