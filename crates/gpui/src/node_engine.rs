use crate::{
    AnyView, App, AppContext, Bounds, Entity, EntityId, GlobalElementId, LayoutId, Pixels,
    ViewNode, ViewNodeCacheKey, ViewNodeRecording,
};
use collections::{FxHashMap, FxHashSet};
pub(crate) type ViewNodeId = EntityId;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct ViewOccurrence {
    element: GlobalElementId,
    parent: Option<ViewNodeId>,
    index: usize,
}

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

/// Work performed by the node engine in its last completed frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct NodeStats {
    /// Input that forced every scope to rebuild, when present.
    pub full_refresh_reason: Option<&'static str>,
    /// Scopes with measurement captures that cannot survive the frame arena.
    pub frame_bound_scopes: usize,
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
    frame_stats: NodeStats,
    pub(crate) last_frame_stats: NodeStats,
    nodes: FxHashMap<ViewNodeId, Entity<ViewNode>>,
    consumers: FxHashMap<EntityId, FxHashSet<ViewNodeId>>,
    occurrences: FxHashMap<ViewOccurrence, ViewNodeId>,
    dirty_nodes: FxHashSet<ViewNodeId>,
    frame_bound_nodes: FxHashSet<ViewNodeId>,
    traversal_stack: Vec<ViewNodeId>,
    roots: Vec<ViewNodeId>,
    next_roots: Vec<ViewNodeId>,
    full_refresh: bool,
    #[cfg(test)]
    eager: bool,
    changed_bounds: Option<Bounds<Pixels>>,
}

impl NodeEngine {
    #[cfg(test)]
    pub(crate) fn new_eager() -> Self {
        Self {
            eager: true,
            ..Self::new()
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            invalidation_queue: Vec::new(),
            frame_stats: NodeStats::default(),
            last_frame_stats: NodeStats::default(),
            nodes: FxHashMap::default(),
            consumers: FxHashMap::default(),
            occurrences: FxHashMap::default(),
            dirty_nodes: FxHashSet::default(),
            frame_bound_nodes: FxHashSet::default(),
            traversal_stack: Vec::new(),
            roots: Vec::new(),
            next_roots: Vec::new(),
            full_refresh: true,
            #[cfg(test)]
            eager: false,
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

    #[cfg(test)]
    pub(crate) fn recordings<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl Iterator<Item = &'a ViewNodeRecording> {
        self.nodes
            .values()
            .filter_map(move |node| node.read(cx).recording.as_ref())
    }

    pub(crate) fn recording<'a>(&self, node_id: ViewNodeId, cx: &'a App) -> &'a ViewNodeRecording {
        self.nodes
            .get(&node_id)
            .expect("recorded child is mounted")
            .read(cx)
            .recording
            .as_ref()
            .expect("recorded child has finished painting")
    }

    pub(crate) fn replay_scene(&self, node_id: ViewNodeId, scene: &mut crate::Scene, cx: &App) {
        let node = self
            .nodes
            .get(&node_id)
            .expect("node scene child must be mounted")
            .read(cx);
        let recording = node
            .recording
            .as_ref()
            .expect("node scene child must have finished painting");
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

    pub(crate) fn begin_frame(&mut self, full_refresh_reason: Option<&'static str>) {
        debug_assert!(self.traversal_stack.is_empty());
        #[cfg(test)]
        let full_refresh_reason = if self.eager {
            Some("eager reference")
        } else {
            full_refresh_reason
        };
        self.full_refresh = full_refresh_reason.is_some();
        self.frame_stats = NodeStats {
            full_refresh_reason,
            ..NodeStats::default()
        };
        self.changed_bounds = None;
        self.next_roots.clear();
        if self.full_refresh {
            self.dirty_nodes.extend(self.nodes.keys().copied());
        }
    }

    pub(crate) fn invalidate_entities(&mut self, sources: &FxHashSet<EntityId>) {
        for source in sources {
            if !self.consumers.contains_key(source) {
                // Nothing recorded a read of this entity, so nothing says which output
                // depends on it. Rebuild everything rather than reuse stale output.
                self.dirty_nodes.extend(self.nodes.keys().copied());
                return;
            }
        }
        for source in sources {
            self.invalidate_consumers(*source);
        }
    }

    pub(crate) fn invalidate_consumers(&mut self, source: EntityId) {
        let pending = &mut self.invalidation_queue;
        pending.clear();
        pending.extend(self.consumers.get(&source).into_iter().flatten().copied());
        while let Some(node_id) = pending.pop() {
            if self.dirty_nodes.insert(node_id) {
                pending.extend(self.consumers.get(&node_id).into_iter().flatten().copied());
            }
        }
    }

    fn replace_dependencies(
        consumers: &mut FxHashMap<EntityId, FxHashSet<ViewNodeId>>,
        node_id: ViewNodeId,
        previous: &FxHashSet<EntityId>,
        current: &FxHashSet<EntityId>,
    ) {
        if previous == current {
            return;
        }
        for source in previous.difference(current) {
            Self::remove_dependency(consumers, node_id, *source);
        }
        for source in current.difference(previous) {
            consumers.entry(*source).or_default().insert(node_id);
        }
    }

    fn remove_dependency(
        consumers: &mut FxHashMap<EntityId, FxHashSet<ViewNodeId>>,
        node_id: ViewNodeId,
        source: EntityId,
    ) {
        if let Some(nodes) = consumers.get_mut(&source) {
            nodes.remove(&node_id);
            if nodes.is_empty() {
                consumers.remove(&source);
            }
        }
    }

    fn next_occurrence(&self, element: GlobalElementId, cx: &App) -> ViewOccurrence {
        let parent = self.traversal_stack.last().copied();
        let siblings = parent
            .and_then(|parent| self.nodes.get(&parent))
            .map(|parent| &parent.read(cx).next_children)
            .unwrap_or(&self.next_roots);
        let mut occurrence = ViewOccurrence {
            element,
            parent,
            index: 0,
        };
        // Element IDs can repeat when one view is mounted twice in the same scope.
        while self
            .occurrences
            .get(&occurrence)
            .is_some_and(|node| siblings.contains(node))
        {
            occurrence.index += 1;
        }
        occurrence
    }

    pub(crate) fn begin_occurrence(
        &mut self,
        occurrence: GlobalElementId,
        view_id: EntityId,
        view: Option<AnyView>,
        cache_key: ViewNodeCacheKey,
        cx: &mut App,
    ) -> NodeRenderDecision {
        let occurrence = self.next_occurrence(occurrence, cx);
        self.begin_resolved_occurrence(occurrence, view_id, view, cache_key, cx)
    }

    fn begin_resolved_occurrence(
        &mut self,
        occurrence: ViewOccurrence,
        view_id: EntityId,
        view: Option<AnyView>,
        cache_key: ViewNodeCacheKey,
        cx: &mut App,
    ) -> NodeRenderDecision {
        let parent = occurrence.parent;
        let node_id = if let Some(node_id) = self.occurrences.get(&occurrence).copied() {
            node_id
        } else {
            let previous_bounds = cache_key.bounds;
            let node = ViewNode {
                local_state: FxHashMap::default(),
                accessed_local_state: FxHashSet::default(),
                layout: None,
                occurrence: occurrence.clone(),
                parent,
                children: Vec::new(),
                next_children: Vec::new(),
                view_id,
                _view: view,
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
                    node.next_children.push(node_id);
                });
            }
        } else {
            self.next_roots.push(node_id);
        }

        let graft = self.nodes.get(&node_id).and_then(|node| {
            let previous = node.read(cx);
            if !self.full_refresh
                && !self.dirty_nodes.contains(&node_id)
                && !self.frame_bound_nodes.contains(&node_id)
                && previous.cache_key == cache_key
                && previous
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
        let occurrence = self.next_occurrence(occurrence, cx);
        let previous = self
            .occurrences
            .get(&occurrence)
            .and_then(|node_id| self.nodes.get(node_id))
            .map(|node| (node.read(cx).cache_key.bounds, node.read(cx).layout));
        if let Some((bounds, _)) = previous {
            cache_key.bounds = bounds;
        }
        let decision =
            self.begin_resolved_occurrence(occurrence, view.entity_id(), Some(view), cache_key, cx);
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
        mut accessed_entities: FxHashSet<EntityId>,
        cx: &mut App,
    ) {
        let Some(node) = self.nodes.get(&node_id) else {
            return;
        };
        let node = node.read(cx);
        let old_bounds = node.previous_bounds;
        let new_bounds = cache_key.bounds;
        accessed_entities.insert(node.view_id);
        // A parent's output contains its children's, so a dirty child dirties its ancestors
        // through the same graph as any other dependency.
        accessed_entities.extend(node.children.iter().copied());
        accessed_entities.remove(&node_id);

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
                node.local_state
                    .retain(|key, _| node.accessed_local_state.contains(key));
                previous_accesses
            });
            Self::replace_dependencies(
                &mut self.consumers,
                node_id,
                &previous_accesses,
                &node.read(cx).accessed_entities,
            );
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
        cx: &mut App,
    ) {
        self.frame_stats.reused_subtrees += 1;
        if let Some(node) = self.nodes.get(&node_id) {
            node.update(cx, |node, _| {
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
        self.frame_stats.frame_bound_scopes = self.frame_bound_nodes.len();
        self.last_frame_stats = self.frame_stats;
        self.changed_bounds.take()
    }

    #[cfg(test)]
    pub(crate) fn clear(&mut self) {
        self.nodes.clear();
        self.consumers.clear();
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
        let (bounds, children, occurrence, accessed_entities) = node.update(cx, |node, _| {
            node.recording = None;
            node.local_state.clear();
            (
                node.previous_bounds,
                std::mem::take(&mut node.children),
                node.occurrence.clone(),
                std::mem::take(&mut node.accessed_entities),
            )
        });
        for source in &accessed_entities {
            Self::remove_dependency(&mut self.consumers, node_id, *source);
        }
        cx.entities.recycle_access_scope(accessed_entities);
        self.include_changed_bounds(bounds);
        for child_id in children {
            self.remove_subtree(child_id, cx);
        }
        self.occurrences.remove(&occurrence);
        self.frame_bound_nodes.remove(&node_id);
        self.dirty_nodes.remove(&node_id);
    }
}
