use crate::{
    Bounds, ElementId, EntityId, GlobalElementId, LayoutId, Pixels, ViewNode, ViewNodeCacheKey,
    view_node::{
        DispatchLink, DispatchOp, DispatchParent, MetadataPhase, NodeOutput, OutputItem,
        OutputSlot, RecordedDispatchNode, ViewNodeScene,
    },
};
use collections::{FxHashMap, FxHashSet};
use slotmap::SlotMap;
use smallvec::SmallVec;
use std::{ops::ControlFlow, ops::Range};

/// A point in the frame being drawn that `ViewTree::rollback` returns to: the output of the
/// scope being drawn (`None` outside every node, where nothing is recorded), with the
/// children it has mounted and the inline components it has counted, and how many nodes
/// the frame has mounted and renders it has noted so far.
pub(crate) struct OutputCheckpoint {
    output: Option<OutputPosition>,
    mounted: usize,
    rendered_phases: usize,
    prepainted_layouts: usize,
}

struct OutputPosition {
    node_id: ViewNodeId,
    phase: MetadataPhase,
    items: usize,
    dispatch: usize,
    next_children: usize,
    /// Restored so a retry's inline components take the occurrences, and so the state,
    /// they had before the attempt.
    inline_views: Option<crate::view_node::InlineViewCounts>,
}

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
///
/// Every path under one parent node starts with the parent's own path, so only the part
/// below it is compared: `parent_depth` is the length of the parent's path. Comparing
/// the whole path would walk every id from the window's root for every view, every
/// frame, since each frame builds its paths anew.
#[derive(Clone)]
pub(crate) struct ViewOccurrence {
    element: GlobalElementId,
    path_hash: u64,
    parent: Option<ViewNodeId>,
    parent_depth: usize,
    index: usize,
}

impl ViewOccurrence {
    fn below_parent(&self) -> &[ElementId] {
        let path = &*self.element.0;
        &path[self.parent_depth.min(path.len())..]
    }
}

impl PartialEq for ViewOccurrence {
    fn eq(&self, other: &Self) -> bool {
        let equal = self.path_hash == other.path_hash
            && self.parent == other.parent
            && self.index == other.index
            && self.element.0.len() == other.element.0.len()
            && self.below_parent() == other.below_parent();
        debug_assert!(
            !equal || self.element.0 == other.element.0,
            "occurrences under one parent share the parent's path"
        );
        equal
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

/// Work performed by the view tree in its last completed frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct ViewTreeStats {
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

/// The entities one node's render read. Nodes read a handful, so a small vector with a
/// linear scan is cheaper than a hash set; it is sorted and deduplicated when stored.
pub(crate) type DependencySet = SmallVec<[EntityId; 4]>;

/// Where the reads of a node rendering this frame are kept between its phases; see
/// [`ViewTree::begin_rendered_reads`].
#[derive(Clone, Copy)]
pub(crate) struct RenderedReads(usize);

/// A node rendering this frame: the last phase it rendered, what it has read so far, and,
/// once it has prepainted, the cache key its render will be stored under.
struct RenderingNode {
    node_id: ViewNodeId,
    phase: MetadataPhase,
    reads: DependencySet,
    cache_key: Option<ViewNodeCacheKey>,
    /// Whether a prepaint reconciled the node's children, which it keeps when a rollback
    /// returns the entry to its layout.
    reconciled: bool,
}

/// Adds `entity_id` to a set being recorded. Reads of one entity tend to repeat back to
/// back, so the last entry is checked before the rest.
pub(crate) fn record_dependency(set: &mut DependencySet, entity_id: EntityId) {
    if set.last() != Some(&entity_id) && !set.contains(&entity_id) {
        set.push(entity_id);
    }
}

pub(crate) struct ViewTree {
    frame_stats: ViewTreeStats,
    pub(crate) last_frame_stats: ViewTreeStats,
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
    /// For each entry of `traversal_stack`, the length of the element-id path the node was
    /// entered at, which is where its children's paths continue from.
    traversal_depths: Vec<usize>,
    /// Scratch for `invalidate_consumers`, which cannot walk `consumers` while setting flags.
    invalidation_scratch: Vec<ViewNodeId>,
    /// Emptied scene records, so a replayed node records into buffers with capacity.
    spare_scenes: Vec<ViewNodeScene>,
    /// Scratch for `snapshot_dispatch_nodes`: where each live dispatch node in the scope's
    /// range resolves to, so parents of later nodes resolve in one step.
    dispatch_resolution: Vec<DispatchParent>,
    /// Scratch for `snapshot_dispatch_nodes`: the live dispatch ranges of the scope's
    /// children, which it skips.
    child_dispatch_ranges: Vec<Range<usize>>,
    /// For each node whose prepaint was grafted this frame, the frame's dispatch node its
    /// first recorded node was reproduced as; the rest follow it consecutively. Grafting
    /// the node's paint fills those nodes with what paint gave them; once the frame is
    /// drawn, `retarget_grafted_dispatch` moves the node's records onto them.
    grafted_dispatch: FxHashMap<ViewNodeId, usize>,
    /// Nodes whose paint was grafted by the replay in progress, awaiting that fill.
    painted_grafts: Vec<ViewNodeId>,
    /// The nodes rendering this frame, each with the last phase it rendered and what it has
    /// read so far, taken when it paints; see `begin_rendered_reads`.
    rendered_phases: Vec<RenderingNode>,
    /// The nodes mounted this frame, in order, each with whether it was created by the
    /// mount, so `rollback` can undo mounts.
    mounted_this_frame: Vec<(ViewNodeId, bool)>,
    /// The `rendered_phases` entries whose layout was followed by a prepaint this frame,
    /// so `rollback` can return an entry from before the checkpoint to its layout.
    prepainted_layouts: Vec<usize>,
    /// A frame is its roots, in drawing order: the window's root view, then the roots
    /// attached by `defer_draw` in priority order, then the prompt, drag overlay or
    /// tooltip. Walking them in order reproduces the frame. `roots` is the frame drawn
    /// last, which events are dispatched against; `next_roots` is the frame being drawn.
    /// They are in prepaint order, which prepaints nested deferred draws in a round after
    /// their owners'.
    roots: Vec<ViewNodeId>,
    next_roots: Vec<ViewNodeId>,
    /// The same roots in paint order, which paints every deferred draw in one priority
    /// order; walks visit paint output in this order.
    paint_roots: Vec<ViewNodeId>,
    next_paint_roots: Vec<ViewNodeId>,
    full_refresh: bool,
    /// Counts from one, so a phase that has never drawn (`text_frame` zero) is not
    /// mistaken for one drawn in the frame before the first.
    frame: u64,
    #[cfg(test)]
    eager: bool,
    changed_bounds: Option<Bounds<Pixels>>,
}

impl ViewTree {
    #[cfg(test)]
    pub(crate) fn new_eager() -> Self {
        Self {
            eager: true,
            ..Self::new()
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            frame_stats: ViewTreeStats::default(),
            last_frame_stats: ViewTreeStats::default(),
            nodes: SlotMap::with_key(),
            consumers: FxHashMap::default(),
            spare_dependency_sets: Vec::new(),
            occurrences: FxHashMap::default(),
            dirty_count: 0,
            frame_bound_count: 0,
            retired_layouts: Vec::new(),
            traversal_stack: Vec::new(),
            traversal_depths: Vec::new(),
            dispatch_resolution: Vec::new(),
            child_dispatch_ranges: Vec::new(),
            grafted_dispatch: FxHashMap::default(),
            painted_grafts: Vec::new(),
            rendered_phases: Vec::new(),
            mounted_this_frame: Vec::new(),
            prepainted_layouts: Vec::new(),
            spare_scenes: Vec::new(),
            invalidation_scratch: Vec::new(),
            roots: Vec::new(),
            next_roots: Vec::new(),
            paint_roots: Vec::new(),
            next_paint_roots: Vec::new(),
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
            .map(|node| std::mem::take(&mut node.output.scene))
            .unwrap_or_default()
    }

    pub(crate) fn store_scene(&mut self, node_id: ViewNodeId, scene: ViewNodeScene) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.output.scene = scene;
            node.painted_frame = self.frame;
        }
    }

    /// Paints the node's primitives from `rendered`, the frame they were last drawn in,
    /// into `scene`, and its children's where they were painted, recording the node anew
    /// so its record addresses `scene`.
    pub(crate) fn replay_scene(
        &mut self,
        node_id: ViewNodeId,
        rendered: &crate::Scene,
        scene: &mut crate::Scene,
    ) {
        // A child only appears in its parent's scene after painting, and is removed only
        // when the parent repaints, so it is always present here.
        let Some(node) = self.nodes.get_mut(node_id) else {
            return;
        };
        let previous = std::mem::take(&mut node.output.scene);
        let fresh = self.spare_scenes.pop().unwrap_or_default();
        scene.begin_node_scene(fresh);
        previous.replay(rendered, scene, self);
        let recorded = scene.finish_node_scene(node_id);
        self.store_scene(node_id, recorded);
        self.spare_scenes.push(previous);
        self.painted_grafts.push(node_id);
    }

    /// Gives the dispatch nodes of the nodes whose paint was just grafted what their paint
    /// gave them in `source`, the frame they were recorded in: key contexts and listeners.
    /// Their prepaint graft reproduced the nodes themselves.
    pub(crate) fn fill_painted_grafts(
        &mut self,
        source: &crate::key_dispatch::DispatchTree,
        tree: &mut crate::key_dispatch::DispatchTree,
    ) {
        let mut painted = std::mem::take(&mut self.painted_grafts);
        for node_id in painted.drain(..) {
            let (Some(start), Some(node)) =
                (self.grafted_dispatch.get(&node_id), self.nodes.get(node_id))
            else {
                continue;
            };
            for (offset, recorded) in node.output.dispatch_nodes.iter().enumerate() {
                tree.fill_recorded(
                    crate::DispatchNodeId::from_index(start + offset),
                    source,
                    recorded.source,
                );
            }
        }
        self.painted_grafts = painted;
    }

    pub(crate) fn current_node(&self) -> Option<ViewNodeId> {
        self.traversal_stack.last().map(|(node_id, _)| *node_id)
    }

    pub(crate) fn current_phase(&self) -> Option<MetadataPhase> {
        self.traversal_stack.last().map(|(_, phase)| *phase)
    }

    /// Takes the state kept for `key` in the node being drawn; `put_element_state` stores it
    /// back stamped with this redraw, which is what keeps it past the redraw.
    pub(crate) fn take_element_state(
        &mut self,
        key: &crate::view_node::ElementStateKey,
    ) -> Option<crate::window::ElementStateBox> {
        let Some(node_id) = self.current_node() else {
            debug_assert!(false, "element state is only kept inside a node");
            return None;
        };
        let (_, state) = self.nodes[node_id].output.element_states.remove(key)?;
        Some(state)
    }

    pub(crate) fn put_element_state(
        &mut self,
        key: crate::view_node::ElementStateKey,
        state: crate::window::ElementStateBox,
    ) {
        if let Some((_, _, output)) = self.current_output() {
            let generation = output.generation;
            output.element_states.insert(key, (generation, state));
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

    /// Records a root the scope being drawn attached with `defer_draw`, under the live
    /// dispatch node active there.
    pub(crate) fn push_root(
        &mut self,
        node: ViewNodeId,
        priority: usize,
        under: Option<crate::DispatchNodeId>,
    ) {
        if let Some((_, _, output)) = self.current_output() {
            output
                .dispatch
                .push(DispatchOp::Root(node, priority, DispatchLink::live(under)));
        }
    }

    /// A point in the output being drawn that `rollback` can return to, discarding
    /// everything drawn after it.
    pub(crate) fn checkpoint(&mut self) -> OutputCheckpoint {
        let output = self
            .traversal_stack
            .last()
            .copied()
            .and_then(|(node_id, phase)| {
                let node = self.nodes.get(node_id)?;
                Some(OutputPosition {
                    node_id,
                    phase,
                    items: node.output.phase(phase).items.len(),
                    dispatch: node.output.dispatch.len(),
                    next_children: node.next_children.len(),
                    inline_views: node.output.inline_views().cloned(),
                })
            });
        OutputCheckpoint {
            output,
            mounted: self.mounted_this_frame.len(),
            rendered_phases: self.rendered_phases.len(),
            prepainted_layouts: self.prepainted_layouts.len(),
        }
    }

    /// Discards what was drawn since `checkpoint`, as if it had not been drawn: the scope's
    /// output and the children it mounted, the renders noted for the end of the frame, and
    /// the mounts themselves. A node created since the checkpoint is removed; one that
    /// existed is no longer mounted this frame, so a retry finds it again rather than
    /// mounting a second node for the same view, and is dirty, since its render may have
    /// replaced its record with a partial one.
    pub(crate) fn rollback(&mut self, checkpoint: OutputCheckpoint) {
        if let Some(position) = checkpoint.output
            && let Some(node) = self.nodes.get_mut(position.node_id)
        {
            node.output
                .phase_mut(position.phase)
                .items
                .truncate(position.items);
            node.output.dispatch.truncate(position.dispatch);
            if let Some(inline_views) = position.inline_views {
                *node.output.inline_views_mut() = inline_views;
            } else if node.output.inline_views().is_some() {
                node.output.inline_views_mut().clear();
            }
            node.next_children.truncate(position.next_children);
        }
        // A node laid out before the checkpoint and prepainted after it keeps its entry,
        // which goes back to its layout: the prepaint's dispatch nodes and cache key are
        // gone. Its reads keep what the discarded prepaint added, which can only make it
        // invalidate more often.
        for index in self
            .prepainted_layouts
            .split_off(checkpoint.prepainted_layouts)
        {
            if index < checkpoint.rendered_phases
                && let Some(rendering) = self.rendered_phases.get_mut(index)
            {
                rendering.phase = MetadataPhase::Layout;
                rendering.cache_key = None;
                let node_id = rendering.node_id;
                self.set_dirty(node_id);
            }
        }
        let discarded = self.rendered_phases.split_off(checkpoint.rendered_phases);
        for rendering in discarded {
            self.recycle_dependency_set(rendering.reads);
        }
        let rolled_back = self.mounted_this_frame.split_off(checkpoint.mounted);
        for (node_id, created) in rolled_back {
            if created {
                self.remove_subtree(node_id);
            } else if let Some(node) = self.nodes.get_mut(node_id) {
                node.mounted_frame = 0;
                node.next_children.clear();
                self.set_dirty(node_id);
            }
        }
    }

    fn frame_roots(&self, frame: FrameOutput, phase: MetadataPhase) -> &[ViewNodeId] {
        match (frame, phase) {
            (FrameOutput::Rendered, MetadataPhase::Paint) => &self.paint_roots,
            (FrameOutput::Rendered, _) => &self.roots,
            (FrameOutput::Next, MetadataPhase::Paint) => &self.next_paint_roots,
            (FrameOutput::Next, _) => &self.next_roots,
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
            for root in self.frame_roots(frame, phase) {
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
            for root in self.frame_roots(frame, phase).iter().rev() {
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

    /// Rebuilds, in `tree`, the dispatch nodes a reused node and its descendants pushed
    /// while prepainting, copied from `source`, the frame they were recorded in, hanging
    /// the node's top-level ones from `attachment`. Roots the
    /// subtree attached are reported to `attach_root` with the dispatch node they hang from.
    /// Returns whether one of the rebuilt nodes is `focus`.
    ///
    /// Only what prepaint gave the nodes is rebuilt here; `fill_painted_grafts` adds what
    /// paint gave them for the nodes whose paint is replayed too.
    pub(crate) fn replay_dispatch(
        &mut self,
        node_id: ViewNodeId,
        attachment: Option<crate::DispatchNodeId>,
        source: &crate::key_dispatch::DispatchTree,
        tree: &mut crate::key_dispatch::DispatchTree,
        focus: Option<crate::FocusId>,
        attach_root: &mut impl FnMut(ViewNodeId, usize, crate::DispatchNodeId),
    ) -> bool {
        let mut grafted = std::mem::take(&mut self.grafted_dispatch);
        let contains_focus = self.replay_dispatch_into(
            node_id,
            attachment,
            source,
            tree,
            focus,
            attach_root,
            &mut grafted,
        );
        self.grafted_dispatch = grafted;
        contains_focus
    }

    fn replay_dispatch_into(
        &self,
        node_id: ViewNodeId,
        attachment: Option<crate::DispatchNodeId>,
        source: &crate::key_dispatch::DispatchTree,
        tree: &mut crate::key_dispatch::DispatchTree,
        focus: Option<crate::FocusId>,
        attach_root: &mut impl FnMut(ViewNodeId, usize, crate::DispatchNodeId),
        grafted: &mut FxHashMap<ViewNodeId, usize>,
    ) -> bool {
        // A child that was removed since its parent last drew is skipped.
        let Some(output) = self.output(node_id) else {
            return false;
        };
        let mut contains_focus = false;
        // Most scopes keep a handful of nodes: a view's, a focusable's, a key context's.
        let mut rebuilt: SmallVec<[crate::DispatchNodeId; 8]> = SmallVec::new();
        for recorded in &output.dispatch_nodes {
            let parent = match recorded.parent {
                DispatchParent::Recorded(index) => rebuilt.get(index as usize).copied(),
                DispatchParent::Attachment => attachment,
            };
            let copy = tree.push_recorded_under(parent, source, recorded.source);
            rebuilt.push(copy);
            contains_focus |= focus.is_some() && tree.node(copy).focus_id == focus;
        }
        if let Some(first) = rebuilt.first() {
            grafted.insert(node_id, first.index());
        }
        let mut registered = 0;
        for op in &output.dispatch {
            let (link, child) = match *op {
                DispatchOp::Child(child, link) => (link, Some(child)),
                DispatchOp::Root(_, _, link) => (link, None),
            };
            let preceding = (link.preceding as usize).clamp(registered, rebuilt.len());
            for copy in &rebuilt[registered..preceding] {
                tree.register_recorded(*copy);
            }
            registered = preceding;
            let under = match link.resolved {
                DispatchParent::Recorded(index) => rebuilt.get(index as usize).copied(),
                DispatchParent::Attachment => attachment,
            };
            match (*op, child) {
                (_, Some(child)) => {
                    contains_focus |= self.replay_dispatch_into(
                        child,
                        under,
                        source,
                        tree,
                        focus,
                        attach_root,
                        grafted,
                    );
                }
                (DispatchOp::Root(root, priority, _), None) => {
                    if let Some(under) = under {
                        attach_root(root, priority, under);
                    }
                }
                _ => {}
            }
        }
        for copy in &rebuilt[registered..] {
            tree.register_recorded(*copy);
        }
        contains_focus
    }

    /// Records where the live dispatch nodes a node is about to push will start.
    pub(crate) fn begin_dispatch_range(&mut self, node_id: ViewNodeId, start: usize) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.output.dispatch_range = start as u32..start as u32;
        }
    }

    pub(crate) fn end_dispatch_range(&mut self, node_id: ViewNodeId, end: usize) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            let range = &mut node.output.dispatch_range;
            range.end = (end as u32).max(range.start);
        }
    }

    /// Records the dispatch nodes a node pushed while prepainting, by their index in the
    /// frame's tree. Its pushes are the range recorded by `begin_dispatch_range`/`end_dispatch_range`,
    /// minus its children's ranges, which the children copy themselves. Empty nodes — most
    /// elements' — are left out, and whatever hung from one is resolved to its nearest kept
    /// ancestor or, above the range, to the scope's attachment point.
    ///
    /// Called after the node paints, which adds key contexts and listeners, or, for a node
    /// that prepainted without painting, once the frame is drawn: the record must then hold
    /// what a fresh frame would have held, with every link resolved.
    pub(crate) fn snapshot_dispatch_nodes(
        &mut self,
        node_id: ViewNodeId,
        dispatch_tree: &crate::key_dispatch::DispatchTree,
    ) {
        let Some(node) = self.nodes.get(node_id) else {
            return;
        };
        let range =
            node.output.dispatch_range.start as usize..node.output.dispatch_range.end as usize;
        let mut resolution = std::mem::take(&mut self.dispatch_resolution);
        resolution.clear();
        resolution.resize(range.len(), DispatchParent::Attachment);
        let resolve =
            |resolution: &[DispatchParent], live: Option<crate::DispatchNodeId>| match live {
                Some(live) if range.contains(&live.index()) => {
                    resolution[live.index() - range.start]
                }
                _ => DispatchParent::Attachment,
            };

        // The scopes whose prepaint ran inside this one — the `Child` ops, in drawing order —
        // pushed nested ranges that they record themselves. They are not always this
        // node's `children`: a deferred root draws views that belong to its owner.
        let mut child_ranges = std::mem::take(&mut self.child_dispatch_ranges);
        child_ranges.clear();
        child_ranges.extend(node.output.dispatch.iter().filter_map(|op| match op {
            DispatchOp::Child(child, _) => self.nodes.get(*child).map(|child| {
                let range = &child.output.dispatch_range;
                range.start as usize..range.end as usize
            }),
            DispatchOp::Root(..) => None,
        }));
        // Pushes are sequential, so the scope's nodes pushed before a child are those below
        // where the child's range starts. A root registers nothing in replay, so it takes
        // the previous child's position.
        let mut position = range.start;
        let attachment_positions: SmallVec<[usize; 8]> = node
            .output
            .dispatch
            .iter()
            .map(|op| {
                if let DispatchOp::Child(child, _) = op
                    && let Some(child) = self.nodes.get(*child)
                {
                    position = child.output.dispatch_range.start as usize;
                }
                position
            })
            .collect();
        let mut next_child_ranges = child_ranges.iter().peekable();
        let mut kept = 0u32;
        let mut live = range.start;
        let mut skip_until = None;
        let output = &mut self.nodes[node_id].output;
        output.dispatch_nodes.clear();
        while live < range.end {
            if skip_until.is_none()
                && let Some(next) = next_child_ranges.peek()
                && next.start <= live
            {
                skip_until = Some(next.end);
                next_child_ranges.next();
            }
            if let Some(end) = skip_until {
                if live < end {
                    live += 1;
                    continue;
                }
                skip_until = None;
                continue;
            }
            let source = crate::DispatchNodeId::from_index(live);
            let recorded = dispatch_tree.node(source);
            let parent = resolve(&resolution, recorded.parent());
            if recorded.is_empty() {
                resolution[live - range.start] = parent;
            } else {
                resolution[live - range.start] = DispatchParent::Recorded(kept);
                kept += 1;
                output
                    .dispatch_nodes
                    .push(RecordedDispatchNode { parent, source });
            }
            live += 1;
        }
        for (op, position) in output.dispatch.iter_mut().zip(attachment_positions) {
            let (DispatchOp::Child(_, link) | DispatchOp::Root(_, _, link)) = op;
            link.resolved = resolve(&resolution, link.live);
            link.preceding = output
                .dispatch_nodes
                .partition_point(|recorded| recorded.source.index() < position)
                as u32;
        }
        self.dispatch_resolution = resolution;
        self.child_dispatch_ranges = child_ranges;
    }

    /// Moves the records of the nodes grafted this frame onto the copies the graft made,
    /// so the next frame, which copies from this one, can graft them again. A node whose
    /// paint was not grafted (hidden) then records the structure-only nodes it has in
    /// this frame, as a fresh frame would have given it. Called once the frame is drawn:
    /// until then, grafting a node's paint still reads the frame its records address.
    pub(crate) fn retarget_grafted_dispatch(&mut self) {
        for (node_id, start) in self.grafted_dispatch.drain() {
            let Some(node) = self.nodes.get_mut(node_id) else {
                continue;
            };
            for (offset, recorded) in node.output.dispatch_nodes.iter_mut().enumerate() {
                recorded.source = crate::DispatchNodeId::from_index(start + offset);
            }
        }
    }

    /// Forgets the grafts whose copies a rolled-back prepaint removed from the frame's
    /// dispatch tree, which now ends at `dispatch_len`, so they are not retargeted onto
    /// nodes pushed after it.
    pub(crate) fn discard_grafts_from(&mut self, dispatch_len: usize) {
        self.grafted_dispatch
            .retain(|_, start| *start < dispatch_len);
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
    /// the frame, registered in drawing order when its prepaint is entered, and in paint
    /// order when its paint is.
    fn splice(
        &mut self,
        node: ViewNodeId,
        phase: MetadataPhase,
        under: Option<crate::DispatchNodeId>,
        depth: usize,
    ) {
        if self.traversal_stack.is_empty() {
            let roots = match phase {
                MetadataPhase::Layout => None,
                MetadataPhase::Prepaint => Some(&mut self.next_roots),
                MetadataPhase::Paint => Some(&mut self.next_paint_roots),
            };
            if let Some(roots) = roots
                && !roots.contains(&node)
            {
                roots.push(node);
            }
        } else {
            self.push(OutputItem::Child(node, phase));
            if phase == MetadataPhase::Prepaint
                && let Some((_, _, output)) = self.current_output()
            {
                output
                    .dispatch
                    .push(DispatchOp::Child(node, DispatchLink::live(under)));
            }
        }
        self.traversal_stack.push((node, phase));
        self.traversal_depths.push(depth);
    }

    /// Takes an empty set to accumulate the entities a rebuilding node reads. Returned to
    /// the engine by `store_render`, which swaps it with the node's previous set.
    pub(crate) fn take_dependency_set(&mut self) -> DependencySet {
        self.spare_dependency_sets.pop().unwrap_or_default()
    }

    /// Keeps a cleared set for reuse when it has a heap buffer worth keeping; an inline one
    /// costs nothing to make.
    pub(crate) fn recycle_dependency_set(&mut self, mut set: DependencySet) {
        if set.spilled() && self.spare_dependency_sets.len() < 64 {
            set.clear();
            self.spare_dependency_sets.push(set);
        }
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
        self.grafted_dispatch.clear();
        self.discard_frame_records();
        self.frame_stats = ViewTreeStats {
            full_refresh_reason,
            ..ViewTreeStats::default()
        };
        self.changed_bounds = None;
        // `next_roots` is not cleared: a root drawn between frames (a test's `draw`) is
        // part of the frame that follows it, ahead of the window root.
        if self.full_refresh {
            self.mark_all_dirty();
        }
    }

    /// Accounts for a frame that replaced the rendered one without drawing (a test that
    /// skips drawing). Nothing can be replayed out of an empty frame, so the frame counter
    /// advances as if a frame had been drawn: no node's record is from the previous frame.
    pub(crate) fn skip_frame(&mut self) {
        self.frame += 1;
        self.discard_frame_records();
    }

    /// Drops what an earlier frame noted for its own end, if that end never came (a test's
    /// `draw` between frames).
    fn discard_frame_records(&mut self) {
        self.mounted_this_frame.clear();
        self.prepainted_layouts.clear();
        let rendered = std::mem::take(&mut self.rendered_phases);
        for rendering in rendered {
            self.recycle_dependency_set(rendering.reads);
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
            parent_depth: self.traversal_depths.last().copied().unwrap_or(0),
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
        let created = node_id.is_none();
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
                accessed_entities: DependencySet::new(),
                painted_frame: 0,
                dirty: true,
                frame_bound: false,
                mounted_frame: 0,
            });
            self.dirty_count += 1;
            self.occurrences.insert(occurrence, node_id);
            node_id
        };
        self.nodes[node_id].mounted_frame = self.frame;
        self.mounted_this_frame.push((node_id, created));

        if let Some(parent_id) = parent
            && let Some(parent_node) = self.nodes.get_mut(parent_id)
        {
            parent_node.next_children.push(node_id);
        }
        let depth = self.nodes[node_id].occurrence.element.0.len();
        self.splice(node_id, MetadataPhase::Layout, None, depth);
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
                accessed_entities: DependencySet::new(),
                painted_frame: 0,
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
        self.nodes
            .get_mut(node_id)?
            .owned_entity
            .take()
            .map(|entity| *entity)
    }

    pub(crate) fn store_owned_entity(
        &mut self,
        node_id: ViewNodeId,
        entity: Option<crate::AnyEntity>,
    ) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.owned_entity = entity.map(Box::new);
        }
    }

    /// The position of the next view of type `type_name` to render inline at the element
    /// path hashed as `path_hash` in the scope being drawn, counting from zero.
    pub(crate) fn next_inline_occurrence(
        &mut self,
        path_hash: u64,
        type_name: &'static str,
    ) -> u64 {
        let Some((_, _, output)) = self.current_output() else {
            return 0;
        };
        let occurrence = output
            .inline_views_mut()
            .entry((path_hash, type_name))
            .or_default();
        let index = *occurrence;
        *occurrence += 1;
        index
    }

    /// Undoes `begin_occurrence` for a view that turned out to have no entity to back a
    /// node, after its phase has been finished.
    pub(crate) fn abandon_occurrence(&mut self, node_id: ViewNodeId) {
        if let Some((_, phase, output)) = self.current_output() {
            let items = &mut output.phase_mut(phase).items;
            if matches!(items.last(), Some(OutputItem::Child(child, _)) if *child == node_id) {
                items.pop();
            }
            if matches!(output.dispatch.last(), Some(DispatchOp::Child(child, _)) if *child == node_id)
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
            && node.painted_frame + 1 == self.frame
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

    /// Records the node's new layout root and returns the previous one for the caller to
    /// drop from the layout tree, unless a child it mounted still owns it: its live
    /// children have been re-attached under the new root by the render that produced it.
    /// Ancestors that shared the previous root share the new one.
    pub(crate) fn store_layout(
        &mut self,
        node_id: ViewNodeId,
        layout: LayoutId,
    ) -> Option<LayoutId> {
        let previous = self.nodes.get_mut(node_id)?.layout.replace(layout)?;
        if previous == layout {
            return None;
        }
        // A view that renders another view directly shares that child's root, so ancestors
        // sharing the old root take the new one with it...
        let mut ancestor = self.nodes[node_id].parent;
        while let Some(ancestor_id) = ancestor
            && let Some(node) = self.nodes.get_mut(ancestor_id)
            && node.layout == Some(previous)
        {
            node.layout = Some(layout);
            ancestor = node.parent;
        }
        // ...and when a reused child is now wrapped in an element of this view's, the old
        // root is still the child's.
        let shared = self.nodes[node_id].next_children.iter().any(|child| {
            self.nodes
                .get(*child)
                .is_some_and(|child| child.layout == Some(previous))
        });
        (!shared).then_some(previous)
    }

    /// Layout roots that stop being retained when this frame ends: those of removed nodes
    /// (collected as they were removed) and of frame-bound nodes, whose measurement closures
    /// may capture the frame arena and so must not outlive it.
    pub(crate) fn take_retired_layouts(&mut self) -> Vec<LayoutId> {
        let mut retired = std::mem::take(&mut self.retired_layouts);
        if self.frame_bound_count > 0 {
            let frame_bound: Vec<ViewNodeId> = self
                .nodes
                .iter()
                .filter(|(_, node)| node.frame_bound && node.layout.is_some())
                .map(|(node_id, _)| node_id)
                .collect();
            for node_id in frame_bound {
                let Some(layout) = self.nodes[node_id].layout.take() else {
                    continue;
                };
                // A node that rendered a child view directly shares the child's root,
                // which stays retained while the child is not frame-bound itself.
                let shared = self.nodes[node_id].children.iter().any(|child| {
                    self.nodes
                        .get(*child)
                        .is_some_and(|child| !child.frame_bound && child.layout == Some(layout))
                });
                if !shared {
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
    /// `depth` is the length of the element-id path in scope, as for the other phases.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn enter_layout(&mut self, node_id: ViewNodeId, depth: usize) {
        self.splice(node_id, MetadataPhase::Layout, None, depth);
    }

    /// Enters the node's prepaint; `under` is the live dispatch node it will hang from and
    /// `depth` the length of the element-id path in scope.
    pub(crate) fn enter_prepaint(
        &mut self,
        node_id: ViewNodeId,
        under: Option<crate::DispatchNodeId>,
        depth: usize,
    ) {
        self.splice(node_id, MetadataPhase::Prepaint, under, depth);
    }

    pub(crate) fn enter_paint(&mut self, node_id: ViewNodeId, depth: usize) {
        self.splice(node_id, MetadataPhase::Paint, None, depth);
    }

    /// Adds text looked up on the node's behalf outside its traversal, such as while
    /// measuring its layout. A reused node's layout can be measured again every frame
    /// without the node rendering, so the first such text in a frame replaces what the
    /// node held rather than adding to it. A measurement that looked nothing up (it
    /// answered from its own cache) leaves the held text alone.
    pub(crate) fn append_text(&mut self, node_id: ViewNodeId, text: crate::text_system::TextUse) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            let phase = node.output.phase_mut(MetadataPhase::Layout);
            if phase.text_frame == self.frame {
                phase.text.append(text);
            } else if !text.is_empty() {
                phase.text = text;
                phase.text_frame = self.frame;
            }
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

    /// Keeps what a rendering node read in its first rendered phase until the node paints,
    /// returning the handle its later phases reach the set through. The set is kept here
    /// rather than carried by the element, since an element may lay out or prepaint a child
    /// view without painting it (`visibility: hidden`, `display: none`, a measured sample):
    /// `finish_unpainted_renders` then commits it for the nodes that did not paint.
    pub(crate) fn begin_rendered_reads(
        &mut self,
        node_id: ViewNodeId,
        phase: MetadataPhase,
        reads: DependencySet,
    ) -> RenderedReads {
        let index = self.rendered_phases.len();
        self.rendered_phases.push(RenderingNode {
            node_id,
            phase,
            reads,
            cache_key: None,
            reconciled: false,
        });
        RenderedReads(index)
    }

    /// Takes the reads kept for a node, to extend them in its prepaint and return them with
    /// `finish_rendered_prepaint`.
    pub(crate) fn take_rendered_reads(&mut self, reads: RenderedReads) -> DependencySet {
        self.rendered_phases
            .get_mut(reads.0)
            .map(|rendering| std::mem::take(&mut rendering.reads))
            .unwrap_or_default()
    }

    /// Returns the reads taken with `take_rendered_reads` as of the end of the node's
    /// prepaint, with the cache key its render is stored under once it paints.
    pub(crate) fn finish_rendered_prepaint(
        &mut self,
        reads: RenderedReads,
        set: DependencySet,
        cache_key: ViewNodeCacheKey,
    ) {
        if let Some(rendering) = self.rendered_phases.get_mut(reads.0) {
            if rendering.phase == MetadataPhase::Layout {
                self.prepainted_layouts.push(reads.0);
            }
            rendering.phase = MetadataPhase::Prepaint;
            rendering.reconciled = true;
            rendering.reads = set;
            rendering.cache_key = Some(cache_key);
        }
    }

    /// Takes what a node's layout and prepaint recorded for its paint to extend and store.
    pub(crate) fn take_rendered_prepaint(
        &mut self,
        reads: RenderedReads,
    ) -> Option<(DependencySet, ViewNodeCacheKey)> {
        let rendering = self.rendered_phases.get_mut(reads.0)?;
        let cache_key = rendering.cache_key.take()?;
        Some((std::mem::take(&mut rendering.reads), cache_key))
    }

    /// Records, for the nodes that rendered this frame without painting, what they read and
    /// the dispatch nodes they pushed, as painting does for the rest, and retires the
    /// children and element states their render dropped. Without it a notify of
    /// something such a node read would be unknown to the tree, and every node would be
    /// rebuilt to be safe. Called once drawing is done, while the frame's dispatch tree is
    /// still the one the nodes pushed into.
    ///
    /// A committed node rendered from current state, so it is no longer dirty. That matters
    /// beyond bookkeeping: invalidation stops climbing at the first dirty node, taking its
    /// ancestors to be dirty already. A node that never paints keeps no record from a
    /// painted frame, so clearing the flag does not make it reusable.
    pub(crate) fn finish_unpainted_renders(&mut self, tree: &crate::key_dispatch::DispatchTree) {
        let mut rendered = std::mem::take(&mut self.rendered_phases);
        for RenderingNode {
            node_id,
            phase,
            reads: accessed,
            reconciled,
            ..
        } in rendered.drain(..)
        {
            let painted = self
                .nodes
                .get(node_id)
                .is_none_or(|node| node.painted_frame == self.frame);
            if painted {
                self.recycle_dependency_set(accessed);
                continue;
            }
            self.commit_dependencies(node_id, accessed);
            if phase == MetadataPhase::Prepaint {
                self.snapshot_dispatch_nodes(node_id, tree);
            }
            // Prepaint reconciles a node's children; one only laid out (under
            // `display: none`) does it here, before its next render discards them.
            if !reconciled {
                self.reconcile_children(node_id);
            }
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.output.retain_accessed_element_states();
            }
        }
        self.rendered_phases = rendered;
    }

    fn commit_dependencies(&mut self, node_id: ViewNodeId, mut current: DependencySet) {
        let Some(node) = self.nodes.get_mut(node_id) else {
            self.recycle_dependency_set(current);
            return;
        };
        current.extend(node.view_id);
        current.sort_unstable();
        current.dedup();
        let previous = std::mem::replace(&mut node.accessed_entities, current);
        Self::replace_dependencies(
            &mut self.consumers,
            node_id,
            &previous,
            &node.accessed_entities,
        );
        Self::clear_dirty(node, &mut self.dirty_count);
        self.recycle_dependency_set(previous);
    }

    pub(crate) fn store_render(
        &mut self,
        node_id: ViewNodeId,
        cache_key: ViewNodeCacheKey,
        accessed_entities: DependencySet,
    ) {
        let Some(node) = self.nodes.get_mut(node_id) else {
            return;
        };
        let old_bounds = node.cache_key.bounds;
        let new_bounds = cache_key.bounds;
        node.cache_key = cache_key;
        node.output.retain_accessed_element_states();
        self.inherit_group_reads(node_id);
        self.commit_dependencies(node_id, accessed_entities);
        self.frame_stats.rebuilt_scopes += 1;
        self.include_changed_bounds(old_bounds);
        self.include_changed_bounds(new_bounds);
    }

    /// Records that the scope being painted resolved group `name` to `found`, unless the
    /// scope pushed that group itself, so the scope is not reused once the group resolves
    /// to another hitbox.
    pub(crate) fn record_group_read(
        &mut self,
        name: &crate::SharedString,
        found: Option<(crate::HitboxId, Option<ViewNodeId>)>,
    ) {
        let Some((node_id, MetadataPhase::Paint)) = self.traversal_stack.last().copied() else {
            return;
        };
        let owner = found.and_then(|(_, owner)| owner);
        if owner == Some(node_id) {
            return;
        }
        let read = crate::view_node::GroupRead {
            name: name.clone(),
            hitbox: found.map(|(hitbox, _)| hitbox),
            owner,
        };
        let reads = self.nodes[node_id].output.group_reads_mut();
        if !reads.contains(&read) {
            reads.push(read);
        }
    }

    /// Whether every group the node's subtree resolved outside itself still resolves to
    /// the same hitbox in `groups`, the groups enclosing it as it is prepainted.
    pub(crate) fn group_reads_unchanged(
        &self,
        node_id: ViewNodeId,
        groups: &crate::elements::GroupHitboxes,
    ) -> bool {
        self.nodes.get(node_id).is_some_and(|node| {
            node.output
                .group_reads()
                .iter()
                .all(|read| groups.get(&read.name).map(|(hitbox, _)| hitbox) == read.hitbox)
        })
    }

    /// Adds to the node's group reads those of the scopes painted inside it, except those
    /// of groups the node pushed, so checking the node covers what it paints.
    fn inherit_group_reads(&mut self, node_id: ViewNodeId) {
        let Some(node) = self.nodes.get(node_id) else {
            return;
        };
        let own = node.output.group_reads();
        let mut inherited = Vec::new();
        // The scopes painted inside this one, which are not always its `children`: a view
        // drawn by a deferred root paints outside its owner, under other groups.
        let painted = node
            .output
            .phase(MetadataPhase::Paint)
            .items
            .iter()
            .filter_map(|item| match item {
                OutputItem::Child(child, MetadataPhase::Paint) => Some(*child),
                _ => None,
            });
        for child in painted {
            let Some(child) = self.nodes.get(child) else {
                continue;
            };
            let reads = child.output.group_reads();
            for read in reads {
                if read.owner != Some(node_id) && !own.contains(read) && !inherited.contains(read) {
                    inherited.push(read.clone());
                }
            }
        }
        if !inherited.is_empty() {
            self.nodes[node_id]
                .output
                .group_reads_mut()
                .extend(inherited);
        }
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
        std::mem::swap(&mut self.paint_roots, &mut self.next_paint_roots);
        self.next_paint_roots.clear();
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
        self.traversal_depths.clear();
        self.roots.clear();
        self.next_roots.clear();
        self.paint_roots.clear();
        self.next_paint_roots.clear();
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
        self.traversal_depths.pop();
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
        self.include_changed_bounds(node.cache_key.bounds);
        // Children mounted this frame are in `next_children` until reconciliation.
        for child_id in node.children.into_iter().chain(node.next_children) {
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
                .id("row")
                .key_context("Row")
                .on_key_down(|_, _, _| {})
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
                .id("panel")
                .key_context("Panel")
                .on_modifiers_changed(|_, _, _| {})
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
    fn view_tree_oracle(cx: &mut TestAppContext, mut rng: StdRng) {
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
    /// nodes and layout roots are reused, not re-created.
    #[gpui::test]
    fn view_tree_is_flat_across_reuse(cx: &mut TestAppContext) {
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
                .update(cx, |_, window, _| window.view_tree_stats())
                .expect("window open")
        };
        for step in 0..100 {
            redraw_one_row(step, cx);
        }
        let settled = redraw_one_row(100, cx);
        assert!(settled.reused_subtrees > 0);
        for step in 101..1000 {
            let stats = redraw_one_row(step, cx);
            assert_eq!(stats.live_nodes, settled.live_nodes, "step {step}");
            assert_eq!(stats.layout_nodes, settled.layout_nodes, "step {step}");
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
                .update(cx, |_, window, _| window.view_tree_stats())
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
