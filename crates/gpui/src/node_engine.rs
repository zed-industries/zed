use crate::{
    Bounds, EntityId, GlobalElementId, LayoutId, Pixels, ViewNode, ViewNodeCacheKey,
    view_node::{MetadataPhase, NodeOutput, OutputItem, OutputSlot, ViewNodeScene},
};
use collections::{FxHashMap, FxHashSet};
use slotmap::SlotMap;
use std::{any::TypeId, ops::ControlFlow};

/// A point in a scope's output that `NodeEngine::rollback` returns to.
#[derive(Clone, Copy)]
pub(crate) struct OutputCheckpoint {
    items: usize,
    dispatch_pushes: u32,
}

/// Which frame's root a query walks: the one drawn last, which events are dispatched
/// against, or the one being drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FrameOutput {
    Rendered,
    Next,
}

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
    /// Nodes mounted so far this frame, so a repeated element id gets the next occurrence.
    mounted_this_frame: FxHashSet<ViewNodeId>,
    dirty_nodes: FxHashSet<ViewNodeId>,
    frame_bound_nodes: FxHashSet<ViewNodeId>,
    /// The nodes being drawn, innermost last, each with the phase it is in.
    traversal_stack: Vec<(ViewNodeId, MetadataPhase)>,
    /// Output drawn outside every node, and the splices to the root nodes: the root of a
    /// frame, which walking in order reproduces. Unlike nodes, the root is rebuilt from
    /// scratch every frame, so the frame being drawn and the frame events are dispatched
    /// against each have their own, swapped when the window swaps its frames.
    next_output: NodeOutput,
    rendered_output: NodeOutput,
    /// The phase output drawn outside every node belongs to.
    frame_phase: MetadataPhase,
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
            mounted_this_frame: FxHashSet::default(),
            dirty_nodes: FxHashSet::default(),
            frame_bound_nodes: FxHashSet::default(),
            traversal_stack: Vec::new(),
            next_output: NodeOutput::default(),
            rendered_output: NodeOutput::default(),
            frame_phase: MetadataPhase::Prepaint,
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

    /// Takes the node's recorded scene so painting can record into it again.
    pub(crate) fn take_scene(&mut self, node_id: ViewNodeId) -> ViewNodeScene {
        self.nodes
            .get_mut(node_id)
            .map(|node| std::mem::take(&mut node.output.phase_mut(MetadataPhase::Paint).scene))
            .unwrap_or_default()
    }

    pub(crate) fn store_scene(&mut self, node_id: ViewNodeId, scene: ViewNodeScene) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.output.phase_mut(MetadataPhase::Paint).scene = scene;
        }
    }

    /// Replays the node's recorded scene, and its children's where they were painted.
    pub(crate) fn replay_scene(&self, node_id: ViewNodeId, scene: &mut crate::Scene) {
        // A child only appears in its parent's scene after painting, and is removed only
        // when the parent repaints, so it is always present here.
        if let Some(node) = self.nodes.get(node_id) {
            node.output
                .phase(MetadataPhase::Paint)
                .scene
                .replay(scene, self);
        }
    }

    pub(crate) fn current_node(&self) -> Option<ViewNodeId> {
        self.traversal_stack.last().map(|(node_id, _)| *node_id)
    }

    /// Sets the phase for output drawn outside every node.
    pub(crate) fn set_frame_phase(&mut self, phase: MetadataPhase) {
        self.frame_phase = phase;
    }

    /// Makes the frame just drawn the one queries run against, and starts a new one.
    pub(crate) fn swap_frame_outputs(&mut self) {
        std::mem::swap(&mut self.next_output, &mut self.rendered_output);
        self.next_output.reset();
        // Root element states not accessed by the frame just drawn were not carried over.
        self.next_output.element_states.clear();
    }

    /// Takes the state kept for `key` in the scope being drawn, recording the access so the
    /// state survives the redraw. Outside every node, the state may still be in the
    /// rendered frame's root.
    pub(crate) fn take_element_state(
        &mut self,
        key: &(GlobalElementId, TypeId),
    ) -> Option<crate::window::ElementStateBox> {
        match self.current_node() {
            Some(node_id) => {
                let output = &mut self.nodes[node_id].output;
                output.accessed_element_states.insert(key.clone());
                output.element_states.remove(key)
            }
            None => {
                self.next_output.accessed_element_states.insert(key.clone());
                self.next_output
                    .element_states
                    .remove(key)
                    .or_else(|| self.rendered_output.element_states.remove(key))
            }
        }
    }

    pub(crate) fn put_element_state(
        &mut self,
        key: (GlobalElementId, TypeId),
        state: crate::window::ElementStateBox,
    ) {
        let (_, _, output) = self.current_output();
        output.element_states.insert(key, state);
    }

    fn output(&self, root: FrameOutput, owner: Option<ViewNodeId>) -> Option<&NodeOutput> {
        match owner {
            Some(node_id) => self.nodes.get(node_id).map(|node| &node.output),
            None => Some(match root {
                FrameOutput::Rendered => &self.rendered_output,
                FrameOutput::Next => &self.next_output,
            }),
        }
    }

    /// Output slots are only leased against the rendered frame.
    fn output_mut(&mut self, owner: Option<ViewNodeId>) -> Option<&mut NodeOutput> {
        match owner {
            Some(node_id) => self.nodes.get_mut(node_id).map(|node| &mut node.output),
            None => Some(&mut self.rendered_output),
        }
    }

    /// The output being drawn into right now: the innermost node's, in its phase, or the
    /// frame's when drawing outside every node (deferred draws, roots that are not views).
    fn current_output(&mut self) -> (Option<ViewNodeId>, MetadataPhase, &mut NodeOutput) {
        match self.traversal_stack.last().copied() {
            Some((node_id, phase)) => (Some(node_id), phase, &mut self.nodes[node_id].output),
            None => (None, self.frame_phase, &mut self.next_output),
        }
    }

    /// Appends `item` to the output being drawn and returns its position.
    pub(crate) fn push(&mut self, item: OutputItem) -> OutputSlot {
        let (owner, phase, output) = self.current_output();
        let generation = output.generation;
        let items = &mut output.phase_mut(phase).items;
        items.push(item);
        OutputSlot {
            owner,
            phase,
            index: items.len() - 1,
            generation,
        }
    }

    /// Records a dispatch node pushed for the element being drawn; see
    /// [`OutputItem::DispatchPush`].
    pub(crate) fn push_dispatch_node(&mut self, live: crate::DispatchNodeId) {
        let (_, phase, output) = self.current_output();
        let output = output.phase_mut(phase);
        let index = output.dispatch_pushes;
        output.dispatch_pushes += 1;
        self.push(OutputItem::DispatchPush(live, index));
    }

    /// A point in the output being drawn that `rollback` can return to, discarding
    /// everything drawn after it.
    pub(crate) fn checkpoint(&mut self) -> OutputCheckpoint {
        let (_, phase, output) = self.current_output();
        let output = output.phase(phase);
        OutputCheckpoint {
            items: output.items.len(),
            dispatch_pushes: output.dispatch_pushes,
        }
    }

    pub(crate) fn rollback(&mut self, checkpoint: OutputCheckpoint) {
        let (_, phase, output) = self.current_output();
        let output = output.phase_mut(phase);
        output.items.truncate(checkpoint.items);
        output.dispatch_pushes = checkpoint.dispatch_pushes;
    }

    /// Visits every item in a frame in the order it was drawn, descending into child
    /// nodes where they were entered. Stops when `visit` breaks.
    pub(crate) fn walk<'a>(
        &'a self,
        root: FrameOutput,
        mut visit: impl FnMut(OutputSlot, &'a OutputItem) -> ControlFlow<()>,
    ) {
        for phase in [
            MetadataPhase::Layout,
            MetadataPhase::Prepaint,
            MetadataPhase::Paint,
        ] {
            if self.walk_output(root, None, phase, &mut visit).is_break() {
                return;
            }
        }
    }

    /// [`Self::walk`] in reverse drawing order.
    pub(crate) fn walk_rev<'a>(
        &'a self,
        root: FrameOutput,
        mut visit: impl FnMut(OutputSlot, &'a OutputItem) -> ControlFlow<()>,
    ) {
        for phase in [
            MetadataPhase::Paint,
            MetadataPhase::Prepaint,
            MetadataPhase::Layout,
        ] {
            if self
                .walk_output_rev(root, None, phase, &mut visit)
                .is_break()
            {
                return;
            }
        }
    }

    fn walk_output<'a>(
        &'a self,
        root: FrameOutput,
        owner: Option<ViewNodeId>,
        phase: MetadataPhase,
        visit: &mut impl FnMut(OutputSlot, &'a OutputItem) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        // A child that was removed since its parent last drew is skipped.
        let Some(output) = self.output(root, owner) else {
            return ControlFlow::Continue(());
        };
        for (index, item) in output.phase(phase).items.iter().enumerate() {
            match item {
                OutputItem::Child(child, child_phase) => {
                    self.walk_output(root, Some(*child), *child_phase, visit)?
                }
                item => visit(
                    OutputSlot {
                        owner,
                        phase,
                        index,
                        generation: output.generation,
                    },
                    item,
                )?,
            }
        }
        ControlFlow::Continue(())
    }

    fn walk_output_rev<'a>(
        &'a self,
        root: FrameOutput,
        owner: Option<ViewNodeId>,
        phase: MetadataPhase,
        visit: &mut impl FnMut(OutputSlot, &'a OutputItem) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let Some(output) = self.output(root, owner) else {
            return ControlFlow::Continue(());
        };
        for (index, item) in output.phase(phase).items.iter().enumerate().rev() {
            match item {
                OutputItem::Child(child, child_phase) => {
                    self.walk_output_rev(root, Some(*child), *child_phase, visit)?
                }
                item => visit(
                    OutputSlot {
                        owner,
                        phase,
                        index,
                        generation: output.generation,
                    },
                    item,
                )?,
            }
        }
        ControlFlow::Continue(())
    }

    /// Visits the items of one node's phase in drawing order, descending into the children
    /// it entered. Used to replay a reused node's prepaint into the frame's dispatch tree.
    pub(crate) fn walk_node<'a>(
        &'a self,
        node_id: ViewNodeId,
        phase: MetadataPhase,
        mut visit: impl FnMut(OutputSlot, &'a OutputItem) -> ControlFlow<()>,
    ) {
        // Visiting every item; the walk only breaks when `visit` does.
        let _ = self.walk_output(FrameOutput::Next, Some(node_id), phase, &mut visit);
    }

    /// Copies the dispatch nodes a node pushed while prepainting out of the frame's dispatch
    /// tree, now that painting has added their listeners and contexts. Existing slots are
    /// cloned into so their listener buffers are reused.
    pub(crate) fn snapshot_dispatch_nodes(
        &mut self,
        node_id: ViewNodeId,
        dispatch_tree: &crate::key_dispatch::DispatchTree,
    ) {
        let Some(node) = self.nodes.get_mut(node_id) else {
            return;
        };
        let output = node.output.phase_mut(MetadataPhase::Prepaint);
        for item in &output.items {
            if let OutputItem::DispatchPush(live, index) = item {
                let recorded = dispatch_tree.node(*live);
                match output.dispatch_nodes.get_mut(*index as usize) {
                    Some(slot) => slot.clone_from(recorded),
                    None => output.dispatch_nodes.push(recorded.clone()),
                }
            }
        }
        output
            .dispatch_nodes
            .truncate(output.dispatch_pushes as usize);
    }

    /// The recorded copy of a dispatch node a reused view pushed.
    pub(crate) fn recorded_dispatch_node(
        &self,
        slot: OutputSlot,
        index: u32,
    ) -> Option<&crate::key_dispatch::DispatchNode> {
        self.output(FrameOutput::Next, slot.owner)?
            .phase(slot.phase)
            .dispatch_nodes
            .get(index as usize)
    }

    /// Takes the callback at `slot` out of its output for a call, via `take` on the matching
    /// variant. Returns `None` while it is already leased or once its owner has redrawn;
    /// return it with `restore`.
    pub(crate) fn lease<T>(
        &mut self,
        slot: OutputSlot,
        take: impl FnOnce(&mut OutputItem) -> Option<T>,
    ) -> Option<T> {
        let output = self.output_mut(slot.owner)?;
        if output.generation != slot.generation {
            return None;
        }
        take(output.phase_mut(slot.phase).items.get_mut(slot.index)?)
    }

    pub(crate) fn restore<T>(
        &mut self,
        slot: OutputSlot,
        value: T,
        put: impl FnOnce(&mut OutputItem, T),
    ) {
        // The owner may have redrawn during the call, in which case the value is stale.
        if let Some(output) = self.output_mut(slot.owner)
            && output.generation == slot.generation
            && let Some(item) = output.phase_mut(slot.phase).items.get_mut(slot.index)
        {
            put(item, value);
        }
    }

    /// Records that `child` is entering `phase` inside the output being drawn, so the
    /// child's output of that phase is walked at this point.
    fn splice(&mut self, child: ViewNodeId, phase: MetadataPhase) {
        self.push(OutputItem::Child(child, phase));
        self.traversal_stack.push((child, phase));
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
        self.mounted_this_frame.clear();
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
        let mut occurrence = ViewOccurrence {
            element,
            parent: self.current_node(),
            index: 0,
        };
        // Element IDs can repeat when one view is mounted twice in the same scope.
        while self
            .occurrences
            .get(&occurrence)
            .is_some_and(|node| self.mounted_this_frame.contains(node))
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
        cache_key: &ViewNodeCacheKey,
    ) -> ViewNodeId {
        let occurrence = self.next_occurrence(element);
        let parent = occurrence.parent;
        let node_id = if let Some(node_id) = self.occurrences.get(&occurrence).copied() {
            node_id
        } else {
            let node_id = self.nodes.insert(ViewNode {
                output: NodeOutput::default(),
                layout: None,
                occurrence: occurrence.clone(),
                parent,
                children: Vec::new(),
                next_children: Vec::new(),
                view_id: None,
                owned_entity: None,
                cache_key: cache_key.clone(),
                previous_bounds: cache_key.bounds,
                accessed_entities: FxHashSet::default(),
                painted: false,
            });
            self.occurrences.insert(occurrence, node_id);
            self.dirty_nodes.insert(node_id);
            node_id
        };
        self.mounted_this_frame.insert(node_id);

        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.nodes.get_mut(parent_id) {
                parent_node.next_children.push(node_id);
            }
        } else {
            self.next_roots.push(node_id);
        }
        self.splice(node_id, MetadataPhase::Layout);
        node_id
    }

    /// Sets the entity whose notification re-renders the node.
    pub(crate) fn set_view_id(&mut self, node_id: ViewNodeId, view_id: EntityId) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.view_id = Some(view_id);
        }
    }

    /// The entity the node created for its view on a previous mount, taken so the view can
    /// reuse or replace it; return it with `store_owned_entity`.
    pub(crate) fn take_owned_entity(&mut self, node_id: ViewNodeId) -> Option<crate::AnyEntity> {
        self.nodes.get_mut(node_id)?.owned_entity.take()
    }

    pub(crate) fn store_owned_entity(
        &mut self,
        node_id: ViewNodeId,
        entity: Option<crate::AnyEntity>,
    ) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.owned_entity = entity;
        }
    }

    /// The position of the next view of type `type_name` to render inline in the scope being
    /// drawn, counting from zero.
    pub(crate) fn next_inline_occurrence(&mut self, type_name: &'static str) -> u64 {
        let (_, _, output) = self.current_output();
        let occurrence = output.inline_views.entry(type_name).or_default();
        let index = *occurrence;
        *occurrence += 1;
        index
    }

    /// Undoes `begin_occurrence` for a view that turned out to have no entity to back a
    /// node, after its phase has been finished.
    pub(crate) fn abandon_occurrence(&mut self, node_id: ViewNodeId) {
        let (_, phase, output) = self.current_output();
        let items = &mut output.phase_mut(phase).items;
        if matches!(items.last(), Some(OutputItem::Child(child, _)) if *child == node_id) {
            items.pop();
        }
        match self.nodes.get(node_id).and_then(|node| node.parent) {
            Some(parent) => {
                if let Some(parent) = self.nodes.get_mut(parent) {
                    parent.next_children.retain(|child| *child != node_id);
                }
            }
            None => self.next_roots.retain(|root| *root != node_id),
        }
        self.remove_subtree(node_id);
    }

    /// Returns the node's retained layout if its output can be reused for a frame whose
    /// ambient inputs are `cache_key`. Called during layout, so bounds are not yet known and
    /// are excluded here; prepaint compares them once they are. When `None`, the caller
    /// restarts the node's render.
    pub(crate) fn reuse_layout(
        &mut self,
        node_id: ViewNodeId,
        cache_key: &ViewNodeCacheKey,
    ) -> Option<LayoutId> {
        let node = &self.nodes[node_id];
        if !self.full_refresh
            && node.painted
            && !self.dirty_nodes.contains(&node_id)
            && !self.frame_bound_nodes.contains(&node_id)
            && node.cache_key.matches(cache_key, true)
        {
            node.layout
        } else {
            None
        }
    }

    pub(crate) fn restart_render(&mut self, node_id: ViewNodeId) {
        self.frame_bound_nodes.remove(&node_id);
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.next_children.clear();
            node.output.reset();
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

    /// Prevents the current node and its ancestors from reusing this frame's output. Used when
    /// a scope produced something a recording cannot hold: a measurement closure that may
    /// capture frame-arena elements, or a deferred draw whose element lives in the arena.
    pub(crate) fn mark_frame_bound(&mut self) {
        self.frame_bound_nodes
            .extend(self.traversal_stack.iter().map(|(node_id, _)| *node_id));
    }

    pub(crate) fn enter_prepaint(&mut self, node_id: ViewNodeId) {
        self.splice(node_id, MetadataPhase::Prepaint);
    }

    pub(crate) fn enter_paint(&mut self, node_id: ViewNodeId) {
        self.splice(node_id, MetadataPhase::Paint);
    }

    /// Adds text looked up on the node's behalf outside its traversal, such as while
    /// measuring its layout.
    pub(crate) fn append_text(&mut self, node_id: ViewNodeId, text: crate::text_system::TextUse) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.output
                .phase_mut(MetadataPhase::Layout)
                .text
                .append(text);
        }
    }

    /// Leaves the node's current phase. When the phase `rendered` (ran the view's elements
    /// rather than reusing its output), the text it looked up replaces the node's, and a
    /// rendered prepaint reconciles the children it mounted.
    pub(crate) fn finish_phase(
        &mut self,
        node_id: ViewNodeId,
        rendered: bool,
        text: crate::text_system::TextUse,
    ) {
        if rendered && let Some((_, phase)) = self.traversal_stack.last().copied() {
            if phase == MetadataPhase::Prepaint {
                self.reconcile_children(node_id);
            }
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.output.phase_mut(phase).text = text;
            }
        }
        self.pop_traversal(node_id);
    }

    pub(crate) fn store_render(
        &mut self,
        node_id: ViewNodeId,
        cache_key: ViewNodeCacheKey,
        mut accessed_entities: FxHashSet<EntityId>,
    ) {
        let Some(node) = self.nodes.get_mut(node_id) else {
            return;
        };
        let old_bounds = node.previous_bounds;
        let new_bounds = cache_key.bounds;
        accessed_entities.extend(node.view_id);
        node.cache_key = cache_key;
        node.previous_bounds = new_bounds;
        node.painted = true;
        node.output.retain_accessed_element_states();
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

    pub(crate) fn store_graft(&mut self) {
        self.frame_stats.reused_subtrees += 1;
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
        self.mounted_this_frame.clear();
        self.dirty_nodes.clear();
        self.frame_bound_nodes.clear();
        self.traversal_stack.clear();
        self.next_output.reset();
        self.rendered_output.reset();
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
        debug_assert_eq!(popped.map(|(node_id, _)| node_id), Some(node_id));
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
        self.mounted_this_frame.remove(&node_id);
    }
}
