use crate::{
    AnyView, App, AppContext, Bounds, Entity, EntityId, GlobalElementId, LayoutId, Pixels,
    ViewNode, ViewNodeCacheKey, ViewNodeRecording,
};
use collections::{FxHashMap, FxHashSet};
#[cfg(test)]
use std::cell::Cell;

#[cfg(test)]
thread_local! {
    static FORCE_NODE_ENGINE: Cell<bool> = const { Cell::new(false) };
}

pub(crate) type ViewNodeId = EntityId;

pub(crate) enum NodeRenderDecision {
    Graft {
        node_id: ViewNodeId,
        recording: ViewNodeRecording,
        accessed_entities: FxHashSet<EntityId>,
    },
    Render {
        node_id: ViewNodeId,
    },
}

/// Work performed by the experimental retained engine in its last completed frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct RetainedNodeStats {
    /// Scopes whose output was rebuilt.
    pub rebuilt_scopes: usize,
    /// Subtrees whose recorded output was reused without visiting their descendants.
    pub reused_subtrees: usize,
    /// Mounted node entities after reconciliation.
    pub live_nodes: usize,
    /// Taffy nodes retained after the frame.
    pub layout_nodes: usize,
}

pub(crate) struct NodeEngine {
    invalidation_queue: Vec<ViewNodeId>,
    scene_children: Vec<(std::ops::Range<usize>, ViewNodeId)>,
    frame_stats: RetainedNodeStats,
    pub(crate) last_frame_stats: RetainedNodeStats,
    nodes: FxHashMap<ViewNodeId, Entity<ViewNode>>,
    occurrences: FxHashMap<GlobalElementId, ViewNodeId>,
    dirty_nodes: FxHashSet<ViewNodeId>,
    frame_bound_nodes: FxHashSet<ViewNodeId>,
    traversal_stack: Vec<ViewNodeId>,
    roots: Vec<ViewNodeId>,
    next_roots: Vec<ViewNodeId>,
    full_refresh: bool,
    changed_bounds: Option<Bounds<Pixels>>,
}

pub(crate) enum DrawEngine {
    Legacy,
    Node(NodeEngine),
}

impl DrawEngine {
    pub(crate) fn from_environment() -> Self {
        #[cfg(test)]
        let forced_for_test = FORCE_NODE_ENGINE.get();
        #[cfg(not(test))]
        let forced_for_test = false;

        if forced_for_test
            || std::env::var("GPUI_EXPERIMENTAL_NODE_ENGINE").is_ok_and(|value| value == "1")
        {
            Self::Node(NodeEngine::new())
        } else {
            Self::Legacy
        }
    }

    #[cfg(test)]
    pub(crate) fn force_node_engine_for_test() -> NodeEngineTestGuard {
        let previous = FORCE_NODE_ENGINE.replace(true);
        NodeEngineTestGuard { previous }
    }
}

#[cfg(test)]
pub(crate) struct NodeEngineTestGuard {
    previous: bool,
}

#[cfg(test)]
impl Drop for NodeEngineTestGuard {
    fn drop(&mut self) {
        FORCE_NODE_ENGINE.set(self.previous);
    }
}

impl NodeEngine {
    pub(crate) fn new() -> Self {
        Self {
            invalidation_queue: Vec::new(),
            scene_children: Vec::new(),
            frame_stats: RetainedNodeStats::default(),
            last_frame_stats: RetainedNodeStats::default(),
            nodes: FxHashMap::default(),
            occurrences: FxHashMap::default(),
            dirty_nodes: FxHashSet::default(),
            frame_bound_nodes: FxHashSet::default(),
            traversal_stack: Vec::new(),
            roots: Vec::new(),
            next_roots: Vec::new(),
            full_refresh: true,
            changed_bounds: None,
        }
    }

    pub(crate) fn take_recording(
        &mut self,
        node_id: ViewNodeId,
        cx: &mut App,
    ) -> Option<ViewNodeRecording> {
        self.nodes
            .get(&node_id)?
            .update(cx, |node, _| node.recording.take())
    }

    pub(crate) fn child_scenes(
        &mut self,
        node_id: ViewNodeId,
        cx: &App,
    ) -> &mut [(std::ops::Range<usize>, ViewNodeId)] {
        self.scene_children.clear();
        self.scene_children.extend(
            self.nodes
                .get(&node_id)
                .into_iter()
                .flat_map(|node| node.read(cx).children.iter())
                .filter_map(|child| self.nodes.get(child))
                .filter_map(|child| {
                    let child_id = child.entity_id();
                    let child = child.read(cx);
                    child.recording.as_ref().map(|_| {
                        (
                            child.paint_range.start.scene_index..child.paint_range.end.scene_index,
                            child_id,
                        )
                    })
                }),
        );
        &mut self.scene_children
    }

    pub(crate) fn replay_scene(&self, node_id: ViewNodeId, scene: &mut crate::Scene, cx: &App) {
        let node = self
            .nodes
            .get(&node_id)
            .expect("retained scene child must be mounted")
            .read(cx);
        let recording = node
            .recording
            .as_ref()
            .expect("retained scene child must have finished painting");
        recording.scene.replay(scene, self, cx);
    }

    pub(crate) fn current_node(&self) -> Option<Entity<ViewNode>> {
        self.traversal_stack
            .last()
            .and_then(|node_id| self.nodes.get(node_id))
            .cloned()
    }

    pub(crate) fn discard_dirty_layouts(&mut self, cx: &mut App) -> bool {
        if !self
            .nodes
            .keys()
            .all(|node_id| self.dirty_nodes.contains(node_id))
        {
            return false;
        }
        for node in self.nodes.values() {
            node.update(cx, |node, _| node.layout = None);
        }
        true
    }

    pub(crate) fn begin_frame(&mut self, full_refresh: bool) {
        debug_assert!(self.traversal_stack.is_empty());
        self.full_refresh = full_refresh;
        self.frame_stats = RetainedNodeStats::default();
        self.changed_bounds = None;
        self.next_roots.clear();
        if full_refresh {
            self.dirty_nodes.extend(self.nodes.keys().copied());
        }
    }

    pub(crate) fn invalidate_entities(&mut self, entities: &FxHashSet<EntityId>, cx: &App) {
        let pending = &mut self.invalidation_queue;
        pending.clear();
        if !entities.is_empty()
            && !entities
                .iter()
                .any(|entity| self.nodes.contains_key(entity))
        {
            self.dirty_nodes.extend(self.nodes.keys().copied());
        }
        for entity_id in entities {
            if self.nodes.contains_key(entity_id) {
                pending.push(*entity_id);
            }
        }
        while let Some(node_id) = pending.pop() {
            if self.dirty_nodes.insert(node_id)
                && let Some(node) = self.nodes.get(&node_id)
                && let Some(parent) = node.read(cx).parent
            {
                pending.push(parent);
            }
        }
    }

    pub(crate) fn begin_occurrence(
        &mut self,
        occurrence: GlobalElementId,
        view: AnyView,
        cache_key: ViewNodeCacheKey,
        cx: &mut App,
    ) -> NodeRenderDecision {
        let parent = self.traversal_stack.last().copied();
        let node_id = if let Some(node_id) = self.occurrences.get(&occurrence).copied() {
            node_id
        } else {
            let previous_bounds = cache_key.bounds;
            let node = ViewNode {
                paint_range: Default::default(),
                local_state: FxHashMap::default(),
                accessed_local_state: FxHashSet::default(),
                layout: None,
                occurrence: occurrence.clone(),
                parent,
                children: Vec::new(),
                next_children: Vec::new(),
                view,
                cache_key: cache_key.clone(),
                previous_bounds,
                accessed_entities: FxHashSet::default(),
                dependency_revisions: Vec::new(),
                recording: None,
            };
            let node = cx.new(|_| node);
            let node_id = node.entity_id();
            self.nodes.insert(node_id, node);
            self.occurrences.insert(occurrence, node_id);
            self.dirty_nodes.insert(node_id);
            node_id
        };

        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.nodes.get(&parent_id) {
                parent_node.update(cx, |node, _| {
                    if !node.next_children.contains(&node_id) {
                        node.next_children.push(node_id);
                    }
                });
            }
        } else if !self.next_roots.contains(&node_id) {
            self.next_roots.push(node_id);
        }

        let graft = self.nodes.get(&node_id).and_then(|node| {
            let retained = node.read(cx);
            if !self.full_refresh
                && !self.dirty_nodes.contains(&node_id)
                && !self.frame_bound_nodes.contains(&node_id)
                && retained.cache_key == cache_key
                && retained
                    .dependency_revisions
                    .iter()
                    .all(|(source, revision)| cx.entities.revision(*source) == Some(*revision))
            {
                node.update(cx, |node, cx| {
                    node.recording.take().map(|recording| {
                        let mut accessed_entities = cx.entities.take_access_scope();
                        accessed_entities.extend(node.accessed_entities.iter().copied());
                        (recording, accessed_entities)
                    })
                })
            } else {
                None
            }
        });

        self.traversal_stack.push(node_id);
        if let Some((recording, accessed_entities)) = graft {
            NodeRenderDecision::Graft {
                node_id,
                recording,
                accessed_entities,
            }
        } else {
            self.restart_render(node_id, cx);
            NodeRenderDecision::Render { node_id }
        }
    }

    pub(crate) fn restart_render(&mut self, node_id: ViewNodeId, cx: &mut App) {
        self.frame_bound_nodes.remove(&node_id);
        if let Some(node) = self.nodes.get(&node_id) {
            node.update(cx, |node, _| {
                node.next_children.clear();
                node.accessed_local_state.clear();
            });
        }
    }

    pub(crate) fn begin_layout(
        &mut self,
        occurrence: GlobalElementId,
        view: AnyView,
        mut cache_key: ViewNodeCacheKey,
        cx: &mut App,
    ) -> (NodeRenderDecision, Option<LayoutId>) {
        let previous = self
            .occurrences
            .get(&occurrence)
            .and_then(|node_id| self.nodes.get(node_id))
            .map(|node| (node.read(cx).cache_key.bounds, node.read(cx).layout));
        if let Some((bounds, _)) = previous {
            cache_key.bounds = bounds;
        }
        let decision = self.begin_occurrence(occurrence, view, cache_key, cx);
        (decision, previous.and_then(|(_, layout)| layout))
    }

    pub(crate) fn store_layout(&mut self, node_id: ViewNodeId, layout: LayoutId, cx: &mut App) {
        if let Some(node) = self.nodes.get(&node_id) {
            node.update(cx, |node, _| node.layout = Some(layout));
        }
    }

    pub(crate) fn cache_key(&self, node_id: ViewNodeId, cx: &App) -> Option<ViewNodeCacheKey> {
        self.nodes
            .get(&node_id)
            .map(|node| node.read(cx).cache_key.clone())
    }

    pub(crate) fn retained_layouts<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl Iterator<Item = LayoutId> + 'a {
        self.nodes
            .iter()
            .filter(|(node_id, _)| !self.frame_bound_nodes.contains(node_id))
            .filter_map(|(_, node)| node.read(cx).layout)
    }

    pub(crate) fn mark_frame_bound_layout(&mut self) {
        self.frame_bound_nodes
            .extend(self.traversal_stack.iter().copied());
    }

    pub(crate) fn enter_prepaint(&mut self, node_id: ViewNodeId) {
        self.traversal_stack.push(node_id);
    }

    pub(crate) fn finish_prepaint(&mut self, node_id: ViewNodeId, rendered: bool, cx: &mut App) {
        if rendered {
            self.reconcile_children(node_id, cx);
        }
        self.pop_traversal(node_id);
    }

    pub(crate) fn store_render(
        &mut self,
        node_id: ViewNodeId,
        cache_key: ViewNodeCacheKey,
        recording: ViewNodeRecording,
        paint_range: std::ops::Range<crate::PaintIndex>,
        mut accessed_entities: FxHashSet<EntityId>,
        cx: &mut App,
    ) {
        let Some(node) = self.nodes.get(&node_id) else {
            return;
        };
        let node = node.read(cx);
        let old_bounds = node.previous_bounds;
        let new_bounds = cache_key.bounds;
        let own_entity_id = node.view.entity_id();
        accessed_entities.insert(own_entity_id);
        accessed_entities.insert(node_id);

        cx.replace_render_dependencies(node_id, &accessed_entities);

        if let Some(node) = self.nodes.get(&node_id) {
            let previous_accesses = node.update(cx, |node, cx| {
                node.cache_key = cache_key;
                node.previous_bounds = new_bounds;
                let previous_accesses =
                    std::mem::replace(&mut node.accessed_entities, accessed_entities);
                node.dependency_revisions.clear();
                node.dependency_revisions.extend(
                    node.accessed_entities
                        .iter()
                        .filter(|source| !self.nodes.contains_key(source))
                        .filter_map(|source| {
                            cx.entities
                                .revision(*source)
                                .map(|revision| (*source, revision))
                        }),
                );
                node.recording = Some(recording);
                node.paint_range = paint_range;
                node.local_state
                    .retain(|key, _| node.accessed_local_state.contains(key));
                previous_accesses
            });
            cx.entities.recycle_access_scope(previous_accesses);
        }

        self.dirty_nodes.remove(&node_id);
        self.frame_stats.rebuilt_scopes += 1;
        self.include_changed_bounds(old_bounds);
        self.include_changed_bounds(new_bounds);
    }

    pub(crate) fn store_graft(
        &mut self,
        node_id: ViewNodeId,
        recording: ViewNodeRecording,
        paint_range: std::ops::Range<crate::PaintIndex>,
        cx: &mut App,
    ) {
        self.frame_stats.reused_subtrees += 1;
        if let Some(node) = self.nodes.get(&node_id) {
            node.update(cx, |node, _| {
                node.paint_range = paint_range;
                node.recording = Some(recording);
            });
        }
    }

    pub(crate) fn finish_frame(&mut self, cx: &mut App) -> Option<Bounds<Pixels>> {
        debug_assert!(self.traversal_stack.is_empty());
        std::mem::swap(&mut self.roots, &mut self.next_roots);
        let mut stale_roots = std::mem::take(&mut self.next_roots);
        for root_id in stale_roots.drain(..) {
            if !self.roots.contains(&root_id) {
                self.remove_subtree(root_id, cx);
            }
        }
        self.next_roots = stale_roots;
        self.full_refresh = false;
        self.frame_stats.live_nodes = self.nodes.len();
        self.last_frame_stats = self.frame_stats;
        self.changed_bounds.take()
    }

    #[cfg(test)]
    pub(crate) fn clear(&mut self, cx: &mut App) {
        for node_id in self.nodes.keys() {
            cx.remove_render_dependencies(*node_id);
        }
        self.nodes.clear();
        self.scene_children.clear();
        self.invalidation_queue.clear();
        self.occurrences.clear();
        self.dirty_nodes.clear();
        self.frame_bound_nodes.clear();
        self.traversal_stack.clear();
        self.roots.clear();
        self.next_roots.clear();
        self.full_refresh = true;
        self.changed_bounds = None;
    }

    fn include_changed_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.changed_bounds = Some(
            self.changed_bounds
                .map(|damage| damage.union(&bounds))
                .unwrap_or(bounds),
        );
    }

    fn pop_traversal(&mut self, node_id: ViewNodeId) {
        let popped = self.traversal_stack.pop();
        debug_assert_eq!(popped, Some(node_id));
    }

    fn reconcile_children(&mut self, node_id: ViewNodeId, cx: &mut App) {
        let Some(node) = self.nodes.get(&node_id) else {
            return;
        };
        let (mut stale_children, current_children) = node.update(cx, |node, _| {
            (
                std::mem::take(&mut node.children),
                std::mem::take(&mut node.next_children),
            )
        });
        for child_id in &current_children {
            if let Some(child) = self.nodes.get(child_id) {
                child.update(cx, |child, _| child.parent = Some(node_id));
            }
        }
        for child_id in stale_children.drain(..) {
            if !current_children.contains(&child_id) {
                self.remove_subtree(child_id, cx);
            }
        }
        if let Some(node) = self.nodes.get(&node_id) {
            node.update(cx, |node, _| {
                node.children = current_children;
                node.next_children = stale_children;
            });
        }
    }

    fn remove_subtree(&mut self, node_id: ViewNodeId, cx: &mut App) {
        let Some(node) = self.nodes.remove(&node_id) else {
            return;
        };
        let (bounds, children, occurrence) = node.update(cx, |node, _| {
            node.recording = None;
            node.local_state.clear();
            {
                node.accessed_entities.clear();
                (
                    node.previous_bounds,
                    std::mem::take(&mut node.children),
                    node.occurrence.clone(),
                )
            }
        });
        self.include_changed_bounds(bounds);
        for child_id in children {
            self.remove_subtree(child_id, cx);
        }
        self.occurrences.remove(&occurrence);
        self.frame_bound_nodes.remove(&node_id);
        self.dirty_nodes.remove(&node_id);
        cx.remove_render_dependencies(node_id);
    }
}
