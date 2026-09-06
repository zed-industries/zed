use crate::{
    AnyView, App, Bounds, EntityId, GlobalElementId, LayoutId, Pixels, ViewNode, ViewNodeCacheKey,
    ViewNodeRecording,
};
use collections::{FxHashMap, FxHashSet};
use slotmap::SlotMap;

slotmap::new_key_type! {
    /// Identifies one mounted view occurrence. Nodes are engine storage: they are not
    /// entities, cannot be observed or notified, and never appear in dependency sets.
    pub(crate) struct ViewNodeId;
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct ViewOccurrence {
    element: GlobalElementId,
    parent: Option<ViewNodeId>,
    index: usize,
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
    frame_stats: NodeStats,
    pub(crate) last_frame_stats: NodeStats,
    nodes: SlotMap<ViewNodeId, ViewNode>,
    /// Reverse of each node's `accessed_entities`: the nodes whose recorded output was
    /// computed from a read of the keyed entity. Ancestors are reached through `parent`.
    consumers: FxHashMap<EntityId, FxHashSet<ViewNodeId>>,
    /// Cleared dependency sets awaiting reuse as the accumulator for a rebuilding node.
    spare_dependency_sets: Vec<FxHashSet<EntityId>>,
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
            frame_stats: NodeStats::default(),
            last_frame_stats: NodeStats::default(),
            nodes: SlotMap::with_key(),
            consumers: FxHashMap::default(),
            spare_dependency_sets: Vec::new(),
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

    pub(crate) fn node(&self, node_id: ViewNodeId) -> &ViewNode {
        &self.nodes[node_id]
    }

    pub(crate) fn node_mut(&mut self, node_id: ViewNodeId) -> &mut ViewNode {
        &mut self.nodes[node_id]
    }

    pub(crate) fn take_recording(&mut self, node_id: ViewNodeId) -> Option<ViewNodeRecording> {
        self.nodes.get_mut(node_id)?.recording.take()
    }

    #[cfg(test)]
    pub(crate) fn recordings(&self) -> impl Iterator<Item = &ViewNodeRecording> {
        self.nodes
            .values()
            .filter_map(|node| node.recording.as_ref())
    }

    pub(crate) fn recording(&self, node_id: ViewNodeId) -> &ViewNodeRecording {
        self.nodes
            .get(node_id)
            .expect("recorded child is mounted")
            .recording
            .as_ref()
            .expect("recorded child has finished painting")
    }

    pub(crate) fn replay_scene(&self, node_id: ViewNodeId, scene: &mut crate::Scene) {
        self.recording(node_id).scene.replay(scene, self);
    }

    pub(crate) fn current_node(&self) -> Option<ViewNodeId> {
        self.traversal_stack.last().copied()
    }

    /// Takes an empty set to accumulate the entities a rebuilding node reads. Returned to
    /// the engine by `store_render`, which swaps it with the node's previous set.
    pub(crate) fn take_dependency_set(&mut self) -> FxHashSet<EntityId> {
        self.spare_dependency_sets.pop().unwrap_or_default()
    }

    pub(crate) fn recycle_dependency_set(&mut self, mut set: FxHashSet<EntityId>) {
        set.clear();
        self.spare_dependency_sets.push(set);
    }

    pub(crate) fn discard_dirty_layouts(&mut self) -> bool {
        if !self
            .nodes
            .keys()
            .all(|node_id| self.dirty_nodes.contains(&node_id))
        {
            return false;
        }
        for node in self.nodes.values_mut() {
            node.layout = None;
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
            self.dirty_nodes.extend(self.nodes.keys());
        }
    }

    /// Marks dirty every node whose recorded output was computed from a read of one of
    /// `sources`, and every ancestor of such a node. Reads only establish dirtiness; the
    /// sources' observers are not involved and no node is notified.
    pub(crate) fn invalidate_entities(&mut self, sources: &FxHashSet<EntityId>) {
        for source in sources {
            if !self.consumers.contains_key(source) {
                // Nothing recorded a read of this entity, so nothing says which output
                // depends on it. Rebuild everything rather than reuse stale output.
                self.dirty_nodes.extend(self.nodes.keys());
                return;
            }
        }
        for source in sources {
            self.invalidate_consumers(*source);
        }
    }

    /// Marks dirty the nodes that read `source` and their ancestors. Does nothing when no
    /// node has read `source`.
    pub(crate) fn invalidate_consumers(&mut self, source: EntityId) {
        let Some(consumers) = self.consumers.get(&source) else {
            return;
        };
        for consumer in consumers {
            let mut node_id = Some(*consumer);
            // A parent's output contains its children's. Stop at the first node that is
            // already dirty, since its ancestors were dirtied with it.
            while let Some(id) = node_id
                && self.dirty_nodes.insert(id)
            {
                node_id = self.nodes.get(id).and_then(|node| node.parent);
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

    fn next_occurrence(&self, element: GlobalElementId) -> ViewOccurrence {
        let parent = self.traversal_stack.last().copied();
        let siblings = parent
            .and_then(|parent| self.nodes.get(parent))
            .map(|parent| &parent.next_children)
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

    /// Mounts the view occurrence under the current traversal parent (creating its node on
    /// first sight), records it as a child for reconciliation, and makes it the current
    /// node until the matching `finish_prepaint`.
    pub(crate) fn begin_occurrence(
        &mut self,
        element: GlobalElementId,
        view_id: EntityId,
        view: Option<AnyView>,
        cache_key: &ViewNodeCacheKey,
    ) -> ViewNodeId {
        let occurrence = self.next_occurrence(element);
        let parent = occurrence.parent;
        let node_id = if let Some(node_id) = self.occurrences.get(&occurrence).copied() {
            node_id
        } else {
            let node_id = self.nodes.insert(ViewNode {
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
                previous_bounds: cache_key.bounds,
                accessed_entities: FxHashSet::default(),
                dependency_revisions: Vec::new(),
                recording: None,
            });
            self.occurrences.insert(occurrence, node_id);
            self.dirty_nodes.insert(node_id);
            node_id
        };

        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.nodes.get_mut(parent_id) {
                parent_node.next_children.push(node_id);
            }
        } else {
            self.next_roots.push(node_id);
        }
        self.traversal_stack.push(node_id);
        node_id
    }

    /// Returns the node's recording if its output can be reused for a frame whose ambient
    /// inputs are `cache_key`. Otherwise prepares the node to render again and returns `None`.
    pub(crate) fn reuse(
        &mut self,
        node_id: ViewNodeId,
        cache_key: &ViewNodeCacheKey,
        cx: &App,
    ) -> Option<ViewNodeRecording> {
        if self.can_reuse(node_id, cache_key, false, cx) {
            self.nodes[node_id].recording.take()
        } else {
            self.restart_render(node_id);
            None
        }
    }

    /// Like `reuse`, for the layout phase: bounds are not yet known, so they are excluded
    /// from the comparison, and reuse additionally requires a retained layout to graft.
    pub(crate) fn reuse_layout(
        &mut self,
        node_id: ViewNodeId,
        cache_key: &ViewNodeCacheKey,
        cx: &App,
    ) -> Option<(ViewNodeRecording, LayoutId)> {
        if self.can_reuse(node_id, cache_key, true, cx)
            && let Some(layout) = self.nodes[node_id].layout
            && let Some(recording) = self.nodes[node_id].recording.take()
        {
            Some((recording, layout))
        } else {
            self.restart_render(node_id);
            None
        }
    }

    fn can_reuse(
        &self,
        node_id: ViewNodeId,
        cache_key: &ViewNodeCacheKey,
        ignore_bounds: bool,
        cx: &App,
    ) -> bool {
        let node = &self.nodes[node_id];
        !self.full_refresh
            && !self.dirty_nodes.contains(&node_id)
            && !self.frame_bound_nodes.contains(&node_id)
            && node.recording.is_some()
            && node.cache_key.matches(cache_key, ignore_bounds)
            && node
                .dependency_revisions
                .iter()
                .all(|(source, revision)| cx.entities.revision(*source) == Some(*revision))
    }

    pub(crate) fn restart_render(&mut self, node_id: ViewNodeId) {
        self.frame_bound_nodes.remove(&node_id);
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.next_children.clear();
            node.accessed_local_state.clear();
        }
    }

    pub(crate) fn store_layout(&mut self, node_id: ViewNodeId, layout: LayoutId) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.layout = Some(layout);
        }
    }

    pub(crate) fn retained_layouts(&self) -> impl Iterator<Item = LayoutId> + '_ {
        self.nodes
            .iter()
            .filter(|(node_id, _)| !self.frame_bound_nodes.contains(node_id))
            .filter_map(|(_, node)| node.layout)
    }

    pub(crate) fn mark_frame_bound_layout(&mut self) {
        self.frame_bound_nodes
            .extend(self.traversal_stack.iter().copied());
    }

    pub(crate) fn enter_prepaint(&mut self, node_id: ViewNodeId) {
        self.traversal_stack.push(node_id);
    }

    pub(crate) fn finish_prepaint(&mut self, node_id: ViewNodeId, rendered: bool) {
        if rendered {
            self.reconcile_children(node_id);
        }
        self.pop_traversal(node_id);
    }

    pub(crate) fn store_render(
        &mut self,
        node_id: ViewNodeId,
        cache_key: ViewNodeCacheKey,
        recording: ViewNodeRecording,
        mut accessed_entities: FxHashSet<EntityId>,
        cx: &App,
    ) {
        let Some(node) = self.nodes.get_mut(node_id) else {
            return;
        };
        let old_bounds = node.previous_bounds;
        let new_bounds = cache_key.bounds;
        accessed_entities.insert(node.view_id);
        node.cache_key = cache_key;
        node.previous_bounds = new_bounds;
        node.dependency_revisions.clear();
        node.dependency_revisions
            .extend(accessed_entities.iter().filter_map(|source| {
                cx.entities
                    .revision(*source)
                    .map(|revision| (*source, revision))
            }));
        node.recording = Some(recording);
        node.local_state
            .retain(|key, _| node.accessed_local_state.contains(key));
        let previous_accesses = std::mem::replace(&mut node.accessed_entities, accessed_entities);
        Self::replace_dependencies(
            &mut self.consumers,
            node_id,
            &previous_accesses,
            &node.accessed_entities,
        );
        self.recycle_dependency_set(previous_accesses);

        self.dirty_nodes.remove(&node_id);
        self.frame_stats.rebuilt_scopes += 1;
        self.include_changed_bounds(old_bounds);
        self.include_changed_bounds(new_bounds);
    }

    pub(crate) fn store_graft(&mut self, node_id: ViewNodeId, recording: ViewNodeRecording) {
        self.frame_stats.reused_subtrees += 1;
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.recording = Some(recording);
        }
    }

    pub(crate) fn finish_frame(&mut self) -> Option<Bounds<Pixels>> {
        debug_assert!(self.traversal_stack.is_empty());
        std::mem::swap(&mut self.roots, &mut self.next_roots);
        let mut stale_roots = std::mem::take(&mut self.next_roots);
        for root_id in stale_roots.drain(..) {
            if !self.roots.contains(&root_id) {
                self.remove_subtree(root_id);
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

    fn reconcile_children(&mut self, node_id: ViewNodeId) {
        let Some(node) = self.nodes.get_mut(node_id) else {
            return;
        };
        let mut stale_children = std::mem::take(&mut node.children);
        let current_children = std::mem::take(&mut node.next_children);
        for child_id in &current_children {
            if let Some(child) = self.nodes.get_mut(*child_id) {
                child.parent = Some(node_id);
            }
        }
        for child_id in stale_children.drain(..) {
            if !current_children.contains(&child_id) {
                self.remove_subtree(child_id);
            }
        }
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.children = current_children;
            node.next_children = stale_children;
        }
    }

    fn remove_subtree(&mut self, node_id: ViewNodeId) {
        let Some(node) = self.nodes.remove(node_id) else {
            return;
        };
        for source in &node.accessed_entities {
            Self::remove_dependency(&mut self.consumers, node_id, *source);
        }
        self.recycle_dependency_set(node.accessed_entities);
        self.include_changed_bounds(node.previous_bounds);
        for child_id in node.children {
            self.remove_subtree(child_id);
        }
        self.occurrences.remove(&node.occurrence);
        self.frame_bound_nodes.remove(&node_id);
        self.dirty_nodes.remove(&node_id);
    }
}
