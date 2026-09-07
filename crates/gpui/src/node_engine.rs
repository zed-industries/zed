use crate::{
    Bounds, EntityId, GlobalElementId, LayoutId, Pixels, ViewNode, ViewNodeCacheKey,
    view_node::{DispatchOp, MetadataPhase, NodeOutput, OutputItem, OutputSlot, ViewNodeScene},
};
use collections::{FxHashMap, FxHashSet};
use slotmap::SlotMap;
use smallvec::SmallVec;
use std::{any::TypeId, ops::ControlFlow};

/// A point in a scope's output that `NodeEngine::rollback` returns to. `None` when taken
/// outside every node, where nothing is recorded.
#[derive(Clone, Copy)]
pub(crate) struct OutputCheckpoint(Option<(ViewNodeId, MetadataPhase, usize, usize, u32)>);

/// Which frame's roots a query walks: the frame drawn last, which events are dispatched
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

/// Where a view was mounted: its element path, under which node, and which repeat of
/// that path within the node. Hashing uses the path's running hash, which the window
/// maintains as ids are pushed, so finding a node again costs no walk of the path; the
/// path itself is compared only on a hash match, to rule out a collision.
#[derive(Clone)]
pub(crate) struct ViewOccurrence {
    element: GlobalElementId,
    path_hash: u64,
    parent: Option<ViewNodeId>,
    index: usize,
}

impl PartialEq for ViewOccurrence {
    fn eq(&self, other: &Self) -> bool {
        self.path_hash == other.path_hash
            && self.parent == other.parent
            && self.index == other.index
            && (std::sync::Arc::ptr_eq(&self.element.0, &other.element.0)
                || self.element.0 == other.element.0)
    }
}

impl Eq for ViewOccurrence {}

impl std::hash::Hash for ViewOccurrence {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path_hash.hash(state);
        self.parent.hash(state);
        self.index.hash(state);
    }
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
    /// An estimate of the heap the engine holds between frames for its nodes' recordings,
    /// dependency sets and bookkeeping, from container capacities. Excludes what boxed
    /// listeners, element states and shaped text point to, and the retained Taffy tree.
    pub retained_bytes: usize,
}

/// The entities one node's render read. Nodes read a handful, so a small vector with a
/// linear scan is cheaper than a hash set; it is sorted and deduplicated when stored.
pub(crate) type DependencySet = SmallVec<[EntityId; 8]>;

/// Adds `entity_id` to a set being recorded. Reads of one entity tend to repeat back to
/// back, so the last entry is checked before the rest.
pub(crate) fn record_dependency(set: &mut DependencySet, entity_id: EntityId) {
    if set.last() != Some(&entity_id) && !set.contains(&entity_id) {
        set.push(entity_id);
    }
}

pub(crate) struct NodeEngine {
    frame_stats: NodeStats,
    pub(crate) last_frame_stats: NodeStats,
    nodes: SlotMap<ViewNodeId, ViewNode>,
    /// Reverse of each node's `accessed_entities`: the nodes whose recorded output was
    /// computed from a read of the keyed entity. Ancestors are reached through `parent`.
    consumers: FxHashMap<EntityId, FxHashSet<ViewNodeId>>,
    /// Cleared dependency sets awaiting reuse as the accumulator for a rebuilding node.
    spare_dependency_sets: Vec<DependencySet>,
    occurrences: FxHashMap<ViewOccurrence, ViewNodeId>,
    /// How many live nodes are `dirty`, so "is every node dirty" is a comparison.
    dirty_count: usize,
    /// How many live nodes are `frame_bound`.
    frame_bound_count: usize,
    /// Layout roots of nodes removed this frame, dropped from the layout tree at its end.
    retired_layouts: Vec<LayoutId>,
    /// The nodes being drawn, innermost last, each with the phase it is in.
    traversal_stack: Vec<(ViewNodeId, MetadataPhase)>,
    /// Scratch for `invalidate_consumers`, which cannot walk `consumers` while setting flags.
    invalidation_scratch: Vec<ViewNodeId>,
    /// Scratch for `snapshot_dispatch_nodes`: whether each open dispatch push is being
    /// dropped, so its pop is dropped with it.
    elided_dispatch_pushes: Vec<bool>,
    /// A frame is its roots, in drawing order: the window's root view, then the roots
    /// attached by `defer_draw` in priority order, then the prompt, drag overlay or
    /// tooltip. Walking them in order reproduces the frame. `roots` is the frame drawn
    /// last, which events are dispatched against; `next_roots` is the frame being drawn.
    roots: Vec<ViewNodeId>,
    next_roots: Vec<ViewNodeId>,
    full_refresh: bool,
    /// Counts from one, so a phase that has never drawn (`text_frame` zero) is not
    /// mistaken for one drawn in the frame before the first.
    frame: u64,
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
            dirty_count: 0,
            frame_bound_count: 0,
            retired_layouts: Vec::new(),
            traversal_stack: Vec::new(),
            elided_dispatch_pushes: Vec::new(),
            invalidation_scratch: Vec::new(),
            roots: Vec::new(),
            next_roots: Vec::new(),
            full_refresh: true,
            frame: 1,
            #[cfg(test)]
            eager: false,
            changed_bounds: None,
        }
    }

    pub(crate) fn node(&self, node_id: ViewNodeId) -> &ViewNode {
        &self.nodes[node_id]
    }

    /// See [`NodeStats::retained_bytes`]. Walks every node, so it is computed on demand.
    pub(crate) fn retained_bytes(&self) -> usize {
        let nodes: usize = self
            .nodes
            .values()
            .map(|node| {
                size_of::<ViewNode>()
                    + node.output.retained_bytes()
                    + node.accessed_entities.capacity() * size_of::<EntityId>()
                    + (node.children.capacity() + node.next_children.capacity())
                        * size_of::<ViewNodeId>()
            })
            .sum();
        let consumers: usize = self
            .consumers
            .values()
            .map(|set| set.capacity() * size_of::<ViewNodeId>())
            .sum::<usize>()
            + self.consumers.capacity() * size_of::<(EntityId, FxHashSet<ViewNodeId>)>();
        nodes
            + consumers
            + self
                .spare_dependency_sets
                .iter()
                .map(|set| set.capacity() * size_of::<EntityId>())
                .sum::<usize>()
            + self.occurrences.capacity() * size_of::<(ViewOccurrence, ViewNodeId)>()
    }

    fn set_dirty(&mut self, node_id: ViewNodeId) -> bool {
        match self.nodes.get_mut(node_id) {
            Some(node) if !node.dirty => {
                node.dirty = true;
                self.dirty_count += 1;
                true
            }
            _ => false,
        }
    }

    fn clear_dirty(node: &mut ViewNode, dirty_count: &mut usize) {
        if node.dirty {
            node.dirty = false;
            *dirty_count -= 1;
        }
    }

    fn set_frame_bound(&mut self, node_id: ViewNodeId) {
        if let Some(node) = self.nodes.get_mut(node_id)
            && !node.frame_bound
        {
            node.frame_bound = true;
            self.frame_bound_count += 1;
        }
    }

    fn clear_frame_bound(node: &mut ViewNode, frame_bound_count: &mut usize) {
        if node.frame_bound {
            node.frame_bound = false;
            *frame_bound_count -= 1;
        }
    }

    fn mark_all_dirty(&mut self) {
        for node in self.nodes.values_mut() {
            node.dirty = true;
        }
        self.dirty_count = self.nodes.len();
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

    /// Takes the state kept for `key` in the node being drawn, recording the access so the
    /// state survives the redraw.
    pub(crate) fn take_element_state(
        &mut self,
        key: &(GlobalElementId, TypeId),
    ) -> Option<crate::window::ElementStateBox> {
        let Some(node_id) = self.current_node() else {
            debug_assert!(false, "element state is only kept inside a node");
            return None;
        };
        let output = &mut self.nodes[node_id].output;
        output.accessed_element_states.insert(key.clone());
        output.element_states.remove(key)
    }

    pub(crate) fn put_element_state(
        &mut self,
        key: (GlobalElementId, TypeId),
        state: crate::window::ElementStateBox,
    ) {
        if let Some((_, _, output)) = self.current_output() {
            output.element_states.insert(key, state);
        }
    }

    fn output(&self, owner: ViewNodeId) -> Option<&NodeOutput> {
        self.nodes.get(owner).map(|node| &node.output)
    }

    fn output_mut(&mut self, owner: ViewNodeId) -> Option<&mut NodeOutput> {
        self.nodes.get_mut(owner).map(|node| &mut node.output)
    }

    /// The output being drawn into right now: the innermost node's, in its phase. `None`
    /// outside every node, where nothing is recorded: the only output drawn there is the
    /// dispatch node an element pushes around a root view, which is rebuilt every frame.
    fn current_output(&mut self) -> Option<(ViewNodeId, MetadataPhase, &mut NodeOutput)> {
        let (node_id, phase) = self.traversal_stack.last().copied()?;
        Some((node_id, phase, &mut self.nodes[node_id].output))
    }

    /// Appends `item` to the output being drawn.
    pub(crate) fn push(&mut self, item: OutputItem) {
        match self.current_output() {
            Some((_, phase, output)) => output.phase_mut(phase).items.push(item),
            None => debug_assert!(false, "output outside every node is lost"),
        }
    }

    /// Records a root the scope being drawn attached with `defer_draw`.
    pub(crate) fn push_root(&mut self, node: ViewNodeId, priority: usize) {
        if let Some((_, phase, output)) = self.current_output() {
            output
                .phase_mut(phase)
                .dispatch
                .push(DispatchOp::Root(node, priority));
        }
    }

    /// Records a dispatch node pushed for the element being drawn; see [`DispatchOp`].
    /// Outside every node nothing replays it, so nothing is recorded.
    pub(crate) fn push_dispatch_node(&mut self, live: crate::DispatchNodeId) {
        if let Some((_, phase, output)) = self.current_output() {
            let output = output.phase_mut(phase);
            output.dispatch_pushes += 1;
            output.dispatch.push(DispatchOp::PushLive(live));
        }
    }

    pub(crate) fn pop_dispatch_node(&mut self) {
        if let Some((_, phase, output)) = self.current_output() {
            output.phase_mut(phase).dispatch.push(DispatchOp::Pop);
        }
    }

    /// A point in the output being drawn that `rollback` can return to, discarding
    /// everything drawn after it.
    pub(crate) fn checkpoint(&mut self) -> OutputCheckpoint {
        OutputCheckpoint(self.current_output().map(|(node_id, phase, output)| {
            let output = output.phase(phase);
            (
                node_id,
                phase,
                output.items.len(),
                output.dispatch.len(),
                output.dispatch_pushes,
            )
        }))
    }

    pub(crate) fn rollback(&mut self, checkpoint: OutputCheckpoint) {
        if let Some((node_id, phase, items, dispatch, dispatch_pushes)) = checkpoint.0
            && let Some(node) = self.nodes.get_mut(node_id)
        {
            let output = node.output.phase_mut(phase);
            output.items.truncate(items);
            output.dispatch.truncate(dispatch);
            output.dispatch_pushes = dispatch_pushes;
        }
    }

    fn frame_roots(&self, frame: FrameOutput) -> &[ViewNodeId] {
        match frame {
            FrameOutput::Rendered => &self.roots,
            FrameOutput::Next => &self.next_roots,
        }
    }

    /// Visits every item in a frame in the order it was drawn: each phase across the roots
    /// in order, descending into child nodes where they were entered. Stops when `visit`
    /// breaks.
    pub(crate) fn walk<'a>(
        &'a self,
        frame: FrameOutput,
        mut visit: impl FnMut(OutputSlot, &'a OutputItem) -> ControlFlow<()>,
    ) {
        for phase in [
            MetadataPhase::Layout,
            MetadataPhase::Prepaint,
            MetadataPhase::Paint,
        ] {
            for root in self.frame_roots(frame) {
                if self.walk_output(*root, phase, &mut visit).is_break() {
                    return;
                }
            }
        }
    }

    /// [`Self::walk`] in reverse drawing order.
    pub(crate) fn walk_rev<'a>(
        &'a self,
        frame: FrameOutput,
        mut visit: impl FnMut(OutputSlot, &'a OutputItem) -> ControlFlow<()>,
    ) {
        for phase in [
            MetadataPhase::Paint,
            MetadataPhase::Prepaint,
            MetadataPhase::Layout,
        ] {
            for root in self.frame_roots(frame).iter().rev() {
                if self.walk_output_rev(*root, phase, &mut visit).is_break() {
                    return;
                }
            }
        }
    }

    fn walk_output<'a>(
        &'a self,
        owner: ViewNodeId,
        phase: MetadataPhase,
        visit: &mut impl FnMut(OutputSlot, &'a OutputItem) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        // A child that was removed since its parent last drew is skipped.
        let Some(output) = self.output(owner) else {
            return ControlFlow::Continue(());
        };
        for (index, item) in output.phase(phase).items.iter().enumerate() {
            match item {
                OutputItem::Child(child, child_phase) => {
                    self.walk_output(*child, *child_phase, visit)?
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
        owner: ViewNodeId,
        phase: MetadataPhase,
        visit: &mut impl FnMut(OutputSlot, &'a OutputItem) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let Some(output) = self.output(owner) else {
            return ControlFlow::Continue(());
        };
        for (index, item) in output.phase(phase).items.iter().enumerate().rev() {
            match item {
                OutputItem::Child(child, child_phase) => {
                    self.walk_output_rev(*child, *child_phase, visit)?
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

    /// Replays the dispatch tree a reused node and its descendants built while prepainting:
    /// `visit` receives each op with the recorded dispatch nodes of the scope that produced
    /// it, in drawing order, descending into children where they were drawn.
    pub(crate) fn walk_dispatch(
        &self,
        node_id: ViewNodeId,
        visit: &mut impl FnMut(&DispatchOp, &[crate::key_dispatch::DispatchNode]),
    ) {
        // A child that was removed since its parent last drew is skipped.
        let Some(output) = self.output(node_id) else {
            return;
        };
        let output = output.phase(MetadataPhase::Prepaint);
        for op in &output.dispatch {
            match op {
                DispatchOp::Child(child) => self.walk_dispatch(*child, visit),
                op => visit(op, &output.dispatch_nodes),
            }
        }
    }

    /// Copies the dispatch nodes a node pushed while prepainting out of the frame's dispatch
    /// tree, now that painting has added their listeners and contexts. Existing slots are
    /// cloned into so their listener buffers are reused. Most elements' nodes turn out
    /// empty; their push and pop are dropped from the recording, since a replay without
    /// them dispatches identically and does less.
    pub(crate) fn snapshot_dispatch_nodes(
        &mut self,
        node_id: ViewNodeId,
        dispatch_tree: &crate::key_dispatch::DispatchTree,
    ) {
        let Some(node) = self.nodes.get_mut(node_id) else {
            return;
        };
        let output = node.output.phase_mut(MetadataPhase::Prepaint);
        let ops = &mut output.dispatch;
        let elided = &mut self.elided_dispatch_pushes;
        elided.clear();
        let mut kept_pushes = 0u32;
        let mut write = 0;
        for read in 0..ops.len() {
            let keep = match ops[read] {
                DispatchOp::PushLive(live) => {
                    let recorded = dispatch_tree.node(live);
                    let keep = !recorded.is_empty();
                    elided.push(!keep);
                    if keep {
                        match output.dispatch_nodes.get_mut(kept_pushes as usize) {
                            Some(slot) => slot.clone_from(recorded),
                            None => output.dispatch_nodes.push(recorded.clone()),
                        }
                        ops[read] = DispatchOp::Push(kept_pushes);
                        kept_pushes += 1;
                    }
                    keep
                }
                DispatchOp::Push(_) => {
                    debug_assert!(false, "a scope's dispatch nodes are snapshotted once");
                    true
                }
                DispatchOp::Pop => !elided.pop().unwrap_or(false),
                DispatchOp::Child(_) | DispatchOp::Root(..) => true,
            };
            if keep {
                ops[write] = ops[read];
                write += 1;
            }
        }
        ops.truncate(write);
        output.dispatch_pushes = kept_pushes;
        output.dispatch_nodes.truncate(kept_pushes as usize);
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

    /// Enters `phase` of `node`. Inside another node, records where the node's output of
    /// that phase belongs in the enclosing output; at the top level, the node is a root of
    /// the frame, registered in drawing order when its prepaint is entered.
    fn splice(&mut self, node: ViewNodeId, phase: MetadataPhase) {
        if self.traversal_stack.is_empty() {
            if phase == MetadataPhase::Prepaint && !self.next_roots.contains(&node) {
                self.next_roots.push(node);
            }
        } else {
            self.push(OutputItem::Child(node, phase));
            if phase == MetadataPhase::Prepaint
                && let Some((_, parent_phase, output)) = self.current_output()
            {
                output
                    .phase_mut(parent_phase)
                    .dispatch
                    .push(DispatchOp::Child(node));
            }
        }
        self.traversal_stack.push((node, phase));
    }

    /// Takes an empty set to accumulate the entities a rebuilding node reads. Returned to
    /// the engine by `store_render`, which swaps it with the node's previous set.
    pub(crate) fn take_dependency_set(&mut self) -> DependencySet {
        self.spare_dependency_sets.pop().unwrap_or_default()
    }

    pub(crate) fn recycle_dependency_set(&mut self, mut set: DependencySet) {
        set.clear();
        self.spare_dependency_sets.push(set);
    }

    pub(crate) fn discard_dirty_layouts(&mut self) -> bool {
        if self.dirty_count < self.nodes.len() {
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
        self.frame += 1;
        self.frame_stats = NodeStats {
            full_refresh_reason,
            ..NodeStats::default()
        };
        self.changed_bounds = None;
        // `next_roots` is not cleared: a root drawn between frames (a test's `draw`) is
        // part of the frame that follows it, ahead of the window root.
        if self.full_refresh {
            self.mark_all_dirty();
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
                self.mark_all_dirty();
                return;
            }
        }
        for source in sources {
            self.invalidate_consumers(*source);
        }
    }

    /// Every entity some live node's recorded output was computed from.
    pub(crate) fn dependency_sources(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.consumers.keys().copied()
    }

    /// Marks dirty the nodes that read `source` and their ancestors. Does nothing when no
    /// node has read `source`.
    pub(crate) fn invalidate_consumers(&mut self, source: EntityId) {
        let Some(consumers) = self.consumers.get(&source) else {
            return;
        };
        // A parent's output contains its children's. Stop at the first node that is
        // already dirty, since its ancestors were dirtied with it.
        let mut pending = std::mem::take(&mut self.invalidation_scratch);
        pending.extend(consumers.iter().copied());
        while let Some(consumer) = pending.pop() {
            let mut node_id = Some(consumer);
            while let Some(id) = node_id
                && self.set_dirty(id)
            {
                node_id = self.nodes.get(id).and_then(|node| node.parent);
            }
        }
        self.invalidation_scratch = pending;
    }

    /// Both sets are sorted and deduplicated, so the difference is one merge.
    fn replace_dependencies(
        consumers: &mut FxHashMap<EntityId, FxHashSet<ViewNodeId>>,
        node_id: ViewNodeId,
        previous: &DependencySet,
        current: &DependencySet,
    ) {
        if previous == current {
            return;
        }
        let (mut old, mut new) = (previous.iter().peekable(), current.iter().peekable());
        loop {
            match (old.peek(), new.peek()) {
                (Some(source), None) => {
                    Self::remove_dependency(consumers, node_id, **source);
                    old.next();
                }
                (None, Some(source)) => {
                    consumers.entry(**source).or_default().insert(node_id);
                    new.next();
                }
                (Some(removed), Some(added)) => match removed.cmp(added) {
                    std::cmp::Ordering::Less => {
                        Self::remove_dependency(consumers, node_id, **removed);
                        old.next();
                    }
                    std::cmp::Ordering::Greater => {
                        consumers.entry(**added).or_default().insert(node_id);
                        new.next();
                    }
                    std::cmp::Ordering::Equal => {
                        old.next();
                        new.next();
                    }
                },
                (None, None) => break,
            }
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

    /// The occurrence the view at `element` mounts as, with its node if it has one.
    fn next_occurrence(
        &self,
        element: GlobalElementId,
        path_hash: u64,
    ) -> (ViewOccurrence, Option<ViewNodeId>) {
        let mut occurrence = ViewOccurrence {
            element,
            path_hash,
            parent: self.current_node(),
            index: 0,
        };
        // Element IDs can repeat when one view is mounted twice in the same scope.
        loop {
            let node_id = self.occurrences.get(&occurrence).copied();
            match node_id {
                Some(node_id) if self.nodes[node_id].mounted_frame == self.frame => {
                    occurrence.index += 1;
                }
                _ => return (occurrence, node_id),
            }
        }
    }

    /// Mounts the view occurrence under the current traversal parent (creating its node on
    /// first sight), records it as a child for reconciliation, and makes it the current
    /// node until the matching `finish_prepaint`.
    pub(crate) fn begin_occurrence(
        &mut self,
        element: GlobalElementId,
        path_hash: u64,
        cache_key: &ViewNodeCacheKey,
    ) -> ViewNodeId {
        let (occurrence, node_id) = self.next_occurrence(element, path_hash);
        let parent = occurrence.parent;
        let node_id = if let Some(node_id) = node_id {
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
                accessed_entities: DependencySet::new(),
                painted: false,
                dirty: true,
                frame_bound: false,
                mounted_frame: 0,
            });
            self.dirty_count += 1;
            self.occurrences.insert(occurrence, node_id);
            node_id
        };
        self.nodes[node_id].mounted_frame = self.frame;

        if let Some(parent_id) = parent
            && let Some(parent_node) = self.nodes.get_mut(parent_id)
        {
            parent_node.next_children.push(node_id);
        }
        self.splice(node_id, MetadataPhase::Layout);
        node_id
    }

    /// Mounts a root that is not a view: the element of a `defer_draw`, or a test's `draw`.
    /// It is keyed by the element-id scope it was mounted from, under the node being drawn
    /// (the owner, for a deferred draw), so it is found again when that scope draws again.
    /// Its `parent` is the owner, so whatever dirties the root dirties the owner, which
    /// attaches it again; it is not one of the owner's children, since it is drawn from the
    /// frame's root list and lives as long as some drawn output attaches it. The caller
    /// records the attachment with [`DispatchOp::Root`] where that applies.
    pub(crate) fn mount_root(
        &mut self,
        element: GlobalElementId,
        path_hash: u64,
        cache_key: &ViewNodeCacheKey,
    ) -> ViewNodeId {
        let (occurrence, node_id) = self.next_occurrence(element, path_hash);
        let node_id = if let Some(node_id) = node_id {
            node_id
        } else {
            let node_id = self.nodes.insert(ViewNode {
                output: NodeOutput::default(),
                layout: None,
                occurrence: occurrence.clone(),
                parent: occurrence.parent,
                children: Vec::new(),
                next_children: Vec::new(),
                view_id: None,
                owned_entity: None,
                cache_key: cache_key.clone(),
                previous_bounds: cache_key.bounds,
                accessed_entities: DependencySet::new(),
                painted: false,
                dirty: true,
                frame_bound: false,
                mounted_frame: 0,
            });
            self.dirty_count += 1;
            self.occurrences.insert(occurrence, node_id);
            node_id
        };
        self.nodes[node_id].mounted_frame = self.frame;
        node_id
    }

    /// Drops a root whose attachment was rolled back before it was drawn.
    pub(crate) fn abandon_root(&mut self, node_id: ViewNodeId) {
        self.remove_subtree(node_id);
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
        let Some((_, _, output)) = self.current_output() else {
            return 0;
        };
        let occurrence = output.inline_views.entry(type_name).or_default();
        let index = *occurrence;
        *occurrence += 1;
        index
    }

    /// Undoes `begin_occurrence` for a view that turned out to have no entity to back a
    /// node, after its phase has been finished.
    pub(crate) fn abandon_occurrence(&mut self, node_id: ViewNodeId) {
        if let Some((_, phase, output)) = self.current_output() {
            let output = output.phase_mut(phase);
            if matches!(output.items.last(), Some(OutputItem::Child(child, _)) if *child == node_id)
            {
                output.items.pop();
            }
            if matches!(output.dispatch.last(), Some(DispatchOp::Child(child)) if *child == node_id)
            {
                output.dispatch.pop();
            }
        }
        if let Some(parent) = self.nodes.get(node_id).and_then(|node| node.parent)
            && let Some(parent) = self.nodes.get_mut(parent)
        {
            parent.next_children.retain(|child| *child != node_id);
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
            && !node.dirty
            && !node.frame_bound
            && node.cache_key.matches(cache_key, true)
        {
            node.layout
        } else {
            None
        }
    }

    /// Takes the text each phase of the node looked up, ahead of a redraw that records anew,
    /// with whether it was looked up this frame or the one before. The line cache keeps two
    /// frames of layouts, so such text is still in it and needs no seeding.
    pub(crate) fn take_text(
        &mut self,
        node_id: ViewNodeId,
    ) -> impl Iterator<Item = (crate::text_system::TextUse, bool)> + '_ {
        let frame = self.frame;
        self.nodes
            .get_mut(node_id)
            .into_iter()
            .flat_map(|node| node.output.phases_mut())
            .map(move |phase| {
                (
                    std::mem::take(&mut phase.text),
                    phase.text_frame + 1 >= frame,
                )
            })
    }

    pub(crate) fn restart_render(&mut self, node_id: ViewNodeId) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            Self::clear_frame_bound(node, &mut self.frame_bound_count);
            node.next_children.clear();
            node.output.reset();
        }
    }

    /// Records the node's new layout root and returns the previous one, which the caller
    /// drops from the layout tree: its live children have been re-attached under the new
    /// root by the render that produced it.
    pub(crate) fn store_layout(
        &mut self,
        node_id: ViewNodeId,
        layout: LayoutId,
    ) -> Option<LayoutId> {
        let node = self.nodes.get_mut(node_id)?;
        let previous = node.layout.replace(layout);
        previous.filter(|previous| *previous != layout)
    }

    /// Layout roots that stop being retained when this frame ends: those of removed nodes
    /// (collected as they were removed) and of frame-bound nodes, whose measurement closures
    /// may capture the frame arena and so must not outlive it.
    pub(crate) fn take_retired_layouts(&mut self) -> Vec<LayoutId> {
        let mut retired = std::mem::take(&mut self.retired_layouts);
        if self.frame_bound_count > 0 {
            for node in self.nodes.values_mut() {
                if node.frame_bound
                    && let Some(layout) = node.layout.take()
                {
                    retired.push(layout);
                }
            }
        }
        retired
    }

    /// Prevents the current node and its ancestors from reusing this frame's output. Used when
    /// a scope produced something a recording cannot hold: a measurement closure that may
    /// capture frame-arena elements. Ancestors are reached through `parent` as well as the
    /// traversal stack: a root attached with `defer_draw` is drawn with only itself on the
    /// stack, and its measurement closures live in its owner's retained layout.
    pub(crate) fn mark_frame_bound(&mut self) {
        for index in 0..self.traversal_stack.len() {
            let (node_id, _) = self.traversal_stack[index];
            self.set_frame_bound(node_id);
        }
        let mut node_id = self.current_node();
        while let Some(id) = node_id {
            self.set_frame_bound(id);
            node_id = self.nodes.get(id).and_then(|node| node.parent);
        }
    }

    /// Enters the layout phase of a root mounted with `mount_root`, so the nodes its element
    /// mounts are its children. Views enter layout through `begin_occurrence`.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn enter_layout(&mut self, node_id: ViewNodeId) {
        self.splice(node_id, MetadataPhase::Layout);
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
            let phase = node.output.phase_mut(MetadataPhase::Layout);
            phase.text.append(text);
            phase.text_frame = self.frame;
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
                let output = node.output.phase_mut(phase);
                output.text = text;
                output.text_frame = self.frame;
            }
        }
        self.pop_traversal(node_id);
    }

    pub(crate) fn store_render(
        &mut self,
        node_id: ViewNodeId,
        cache_key: ViewNodeCacheKey,
        mut accessed_entities: DependencySet,
    ) {
        let Some(node) = self.nodes.get_mut(node_id) else {
            return;
        };
        let old_bounds = node.previous_bounds;
        let new_bounds = cache_key.bounds;
        accessed_entities.extend(node.view_id);
        accessed_entities.sort_unstable();
        accessed_entities.dedup();
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

        if let Some(node) = self.nodes.get_mut(node_id) {
            Self::clear_dirty(node, &mut self.dirty_count);
        }
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
        self.frame_stats.frame_bound_scopes = self.frame_bound_count;
        self.last_frame_stats = self.frame_stats;
        self.changed_bounds.take()
    }

    #[cfg(test)]
    pub(crate) fn clear(&mut self) {
        self.nodes.clear();
        self.consumers.clear();
        self.occurrences.clear();
        self.dirty_count = 0;
        self.frame_bound_count = 0;
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
        // Children usually come back in the same order; a wide node with reordered children
        // would otherwise pay a quadratic scan here.
        if stale_children != current_children {
            let current: FxHashSet<ViewNodeId> = current_children.iter().copied().collect();
            for child_id in stale_children.drain(..) {
                if !current.contains(&child_id) {
                    self.remove_subtree(child_id);
                }
            }
        }
        stale_children.clear();
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.children = current_children;
            node.next_children = stale_children;
        }
    }

    fn remove_subtree(&mut self, node_id: ViewNodeId) {
        let Some(mut node) = self.nodes.remove(node_id) else {
            return;
        };
        Self::clear_dirty(&mut node, &mut self.dirty_count);
        Self::clear_frame_bound(&mut node, &mut self.frame_bound_count);
        self.retired_layouts.extend(node.layout);
        for source in &node.accessed_entities {
            Self::remove_dependency(&mut self.consumers, node_id, *source);
        }
        self.recycle_dependency_set(node.accessed_entities);
        self.include_changed_bounds(node.previous_bounds);
        for child_id in node.children {
            self.remove_subtree(child_id);
        }
        self.occurrences.remove(&node.occurrence);
    }
}

/// The engine's oracle over a fixture that exercises every kind of frame state the
/// engine retains: nested views mounted and unmounted, a `uniform_list`, wrapped text,
/// focus, hover, scrolling, a deferred popover, and window resizes. A seeded sequence of
/// updates drives it, and after each update the incrementally drawn frame must equal a
/// full refresh from the same state. `test_workspace_rendering_stress` in `editor` is the
/// same oracle over a real workspace.
#[cfg(test)]
mod oracle_tests {
    use crate::{
        Context, Entity, FocusHandle, Modifiers, Render, ScrollHandle, ScrollStrategy,
        SharedString, TestAppContext, UniformListScrollHandle, VisualTestContext, Window, anchored,
        deferred, div, point, prelude::*, px, rgb, size, uniform_list,
    };
    use rand::prelude::*;

    struct Row {
        index: usize,
        color: u32,
    }

    impl Render for Row {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .h(px(20.))
                .w_full()
                .bg(rgb(self.color))
                .child(SharedString::from(format!("row {}", self.index)))
        }
    }

    /// Owns the popover, so a change elsewhere in the fixture leaves it clean and its
    /// attached root is replayed rather than drawn again.
    struct Panel {
        focus_handles: Vec<FocusHandle>,
        popover_open: bool,
        popover_revision: usize,
        popover_row: Option<Entity<Row>>,
    }

    impl Render for Panel {
        fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .h(px(30.))
                .children(self.focus_handles.iter().enumerate().map(|(ix, handle)| {
                    div()
                        .id(("focus", ix))
                        .track_focus(handle)
                        .size(px(30.))
                        .bg(rgb(0x8888ff))
                        .when(handle.is_focused(window), |this| this.bg(rgb(0xff8800)))
                }))
                .children((0..3usize).map(|ix| {
                    div()
                        .id(("hover", ix))
                        .size(px(30.))
                        .bg(rgb(0x88ff88))
                        .hover(|style| style.bg(rgb(0xff0000)))
                }))
                .when(self.popover_open, |this| {
                    this.child(deferred(
                        anchored().position(point(px(20.), px(20.))).child(
                            div()
                                .w(px(120.))
                                .p(px(4.))
                                .bg(rgb(0xffffaa))
                                .child(SharedString::from(format!(
                                    "popover {}",
                                    self.popover_revision
                                )))
                                .children(self.popover_row.clone()),
                        ),
                    ))
                })
        }
    }

    struct Fixture {
        panel: Entity<Panel>,
        rows: Vec<Entity<Row>>,
        list_scroll: UniformListScrollHandle,
        text_scroll: ScrollHandle,
        text_revision: usize,
    }

    impl Render for Fixture {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let rows = self.rows.clone();
            let paragraph = format!(
                "revision {} of a paragraph that wraps across several lines — λ→ — {}",
                self.text_revision,
                "words ".repeat(self.text_revision % 5 + 8)
            );
            div()
                .size_full()
                .flex()
                .flex_col()
                .bg(rgb(0xf0f0f0))
                .child(self.panel.clone())
                .child(
                    uniform_list("rows", rows.len(), move |range, _, _| {
                        rows[range].iter().cloned().collect()
                    })
                    .track_scroll(&self.list_scroll)
                    .h(px(120.))
                    .w_full(),
                )
                .child(
                    div()
                        .id("text")
                        .overflow_y_scroll()
                        .track_scroll(&self.text_scroll)
                        .h(px(60.))
                        .w(px(160.))
                        .child(SharedString::from(paragraph)),
                )
        }
    }

    #[gpui::test(iterations = 5)]
    fn node_engine_oracle(cx: &mut TestAppContext, mut rng: StdRng) {
        let mut next_row = 0;
        let mut new_row = |cx: &mut crate::App, rng: &mut StdRng| {
            let row = cx.new(|_| Row {
                index: next_row,
                color: rng.random::<u32>() & 0xffffff,
            });
            next_row += 1;
            row
        };
        let window = cx.open_window(size(px(300.), px(300.)), |_, cx| {
            let rows: Vec<_> = (0..40).map(|_| new_row(cx, &mut rng)).collect();
            let popover_row = rows.first().cloned();
            Fixture {
                panel: cx.new(|cx| Panel {
                    focus_handles: (0..3).map(|_| cx.focus_handle()).collect(),
                    popover_open: false,
                    popover_revision: 0,
                    popover_row,
                }),
                rows,
                list_scroll: UniformListScrollHandle::new(),
                text_scroll: ScrollHandle::new(),
                text_revision: 0,
            }
        });
        cx.run_until_parked();
        let mut visual = VisualTestContext::from_window(window.into(), cx);
        let mut reused_subtrees = 0;

        for step in 0..40 {
            let action = rng.random_range(0..9);
            match action {
                0 => {
                    let rows = window
                        .read_with(cx, |fixture, _| fixture.rows.clone())
                        .expect("window open");
                    if let Some(row) = rows.choose(&mut rng) {
                        row.update(cx, |row, cx| {
                            row.color ^= 0x00ff00;
                            cx.notify();
                        });
                    }
                }
                1 => window
                    .update(cx, |fixture, _, cx| {
                        fixture.panel.update(cx, |panel, cx| {
                            panel.popover_open = !panel.popover_open;
                            panel.popover_revision += 1;
                            cx.notify();
                        })
                    })
                    .expect("window open"),
                2 => {
                    let item = rng.random_range(0..40);
                    window
                        .update(cx, |fixture, _, cx| {
                            fixture
                                .list_scroll
                                .scroll_to_item(item, ScrollStrategy::Top);
                            cx.notify();
                        })
                        .expect("window open");
                }
                3 => {
                    let position = point(
                        px(rng.random_range(0..300) as f32),
                        px(rng.random_range(0..300) as f32),
                    );
                    visual.simulate_mouse_move(position, None, Modifiers::default());
                }
                4 => {
                    let ix = rng.random_range(0..3);
                    window
                        .update(cx, |fixture, window, cx| {
                            let handle = fixture.panel.read(cx).focus_handles[ix].clone();
                            window.focus(&handle, cx);
                        })
                        .expect("window open");
                }
                5 => {
                    let width = [300., 260., 340.][rng.random_range(0..3)];
                    visual.simulate_resize(size(px(width), px(300.)));
                }
                6 => window
                    .update(cx, |fixture, _, cx| {
                        fixture.text_revision += 1;
                        cx.notify();
                    })
                    .expect("window open"),
                7 => {
                    let add = rng.random_bool(0.5);
                    let row = add.then(|| cx.update(|cx| new_row(cx, &mut rng)));
                    window
                        .update(cx, |fixture, _, cx| {
                            match row {
                                Some(row) => {
                                    let at = rng.random_range(0..=fixture.rows.len());
                                    fixture.rows.insert(at, row);
                                }
                                None if fixture.rows.len() > 1 => {
                                    let at = rng.random_range(0..fixture.rows.len());
                                    fixture.rows.remove(at);
                                }
                                None => {}
                            }
                            cx.notify();
                        })
                        .expect("window open");
                }
                _ => {
                    let offset = rng.random_range(0..80) as f32;
                    window
                        .update(cx, |fixture, _, cx| {
                            fixture.text_scroll.set_offset(point(px(0.), px(-offset)));
                            cx.notify();
                        })
                        .expect("window open");
                }
            }
            cx.run_until_parked();
            let stats = visual.assert_incremental_matches_full_refresh(format_args!(
                "step {step} (action {action})"
            ));
            reused_subtrees += stats.reused_subtrees;
        }
        assert!(reused_subtrees > 0, "the fixture must exercise node reuse");
    }

    /// What the engine holds between frames must not grow while the same tree is redrawn:
    /// a node redrawn in place reuses its buffers, and reused nodes allocate nothing.
    #[gpui::test]
    fn node_engine_retained_memory_is_flat_across_reuse(cx: &mut TestAppContext) {
        let mut rng = StdRng::seed_from_u64(0);
        let window = cx.open_window(size(px(300.), px(300.)), |_, cx| {
            let rows: Vec<_> = (0..40)
                .map(|index| {
                    cx.new(|_| Row {
                        index,
                        color: rng.random::<u32>() & 0xffffff,
                    })
                })
                .collect();
            let popover_row = rows.first().cloned();
            Fixture {
                panel: cx.new(|cx| Panel {
                    focus_handles: (0..3).map(|_| cx.focus_handle()).collect(),
                    popover_open: true,
                    popover_revision: 0,
                    popover_row,
                }),
                rows,
                list_scroll: UniformListScrollHandle::new(),
                text_scroll: ScrollHandle::new(),
                text_revision: 0,
            }
        });
        cx.run_until_parked();
        let mut redraw_one_row = |step: usize, cx: &mut TestAppContext| {
            let rows = window
                .read_with(cx, |fixture, _| fixture.rows.clone())
                .expect("window open");
            rows[step % rows.len()].update(cx, |row, cx| {
                row.color ^= 0x0000ff;
                cx.notify();
            });
            cx.run_until_parked();
            window
                .update(cx, |_, window, _| window.node_stats())
                .expect("window open")
        };
        for step in 0..100 {
            redraw_one_row(step, cx);
        }
        let settled = redraw_one_row(100, cx);
        assert!(settled.reused_subtrees > 0);
        // Which row is dirty moves one small container between two capacities.
        let tolerance = 256;
        for step in 101..1000 {
            let stats = redraw_one_row(step, cx);
            assert_eq!(stats.live_nodes, settled.live_nodes, "step {step}");
            assert_eq!(stats.layout_nodes, settled.layout_nodes, "step {step}");
            assert!(
                stats.retained_bytes <= settled.retained_bytes + tolerance,
                "step {step}: retained {} bytes, settled at {}",
                stats.retained_bytes,
                settled.retained_bytes
            );
        }
    }

    /// A list whose items are plain elements rather than views. Each visible item, and the
    /// item measured for the row height, is laid out as its own root every frame the list
    /// draws, so nothing in the list's retained layout tree reaches those trees.
    struct ListOfDivs {
        revision: usize,
        /// Stays clean, so redrawing the list is an incremental frame rather than a full
        /// refresh that clears the layout tree.
        sibling: Entity<Row>,
    }

    impl Render for ListOfDivs {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let revision = self.revision;
            div()
                .size_full()
                .flex()
                .flex_col()
                .child(self.sibling.clone())
                .child(
                    uniform_list("rows", 40, move |range, _, _| {
                        range
                            .map(|index| {
                                div()
                                    .h(px(20.))
                                    .w_full()
                                    .child(SharedString::from(format!("row {index} r{revision}")))
                            })
                            .collect()
                    })
                    .h(px(200.))
                    .w_full(),
                )
        }
    }

    /// Layout trees laid out as roots inside a node (list items, editor blocks) are dropped
    /// with the frame, or the layout tree grows by one such tree per item per frame.
    #[gpui::test]
    fn layout_trees_measured_inside_a_node_do_not_accumulate(cx: &mut TestAppContext) {
        let window = cx.open_window(size(px(300.), px(300.)), |_, cx| ListOfDivs {
            revision: 0,
            sibling: cx.new(|_| Row {
                index: 0,
                color: 0x336699,
            }),
        });
        cx.run_until_parked();
        let mut redraw = |cx: &mut TestAppContext| {
            window
                .update(cx, |list, _, cx| {
                    list.revision += 1;
                    cx.notify();
                })
                .expect("window open");
            cx.run_until_parked();
            window
                .update(cx, |_, window, _| window.node_stats())
                .expect("window open")
        };
        let settled = redraw(cx);
        assert!(settled.reused_subtrees > 0, "the sibling must be reused");
        for step in 0..50 {
            let stats = redraw(cx);
            assert_eq!(stats.layout_nodes, settled.layout_nodes, "step {step}");
        }
    }
}
