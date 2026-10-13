use std::collections::HashMap;

use gpui::{Bounds, Pixels, SharedString};

use crate::types::*;

/// Resolved handle position in screen coordinates, stored after layout.
#[derive(Debug, Clone)]
pub struct ResolvedHandle {
    pub node_id: NodeId,
    pub handle_id: Option<SharedString>,
    pub handle_type: HandleType,
    pub position: HandlePosition,
    pub bounds: Bounds<Pixels>,
}

/// Key for looking up a handle: (node_id, handle_id).
pub type HandleKey = (NodeId, Option<SharedString>);

/// Interior offset (from a container node's own top-left) where its
/// children's local coordinate space starts — leaves room for a header/
/// label strip. Used by `FlowState::absolute_position`/`fit_container`.
pub const CONTAINER_HEADER_HEIGHT: f32 = 32.0;
/// Interior left/right/bottom inset applied the same way.
pub const CONTAINER_PADDING: f32 = 12.0;
/// Fallback footprint for a container node with no explicit
/// `FlowNode::container_size` and nothing yet computed by `fit_container`.
pub const DEFAULT_CONTAINER_SIZE: (f32, f32) = (320.0, 220.0);
/// Padding used by an automatic (measure/zoom-driven) container re-fit when
/// the container was never fitted by an explicit `fit_container` call (so its
/// `FlowNode::container_padding` is `None`).
pub const DEFAULT_FIT_PADDING: f32 = CONTAINER_PADDING * 2.0;
/// Depth/ancestor walks bail out past this many hops rather than looping
/// forever if a cycle is ever accidentally introduced (e.g. by a buggy
/// `set_parent` caller) — graphs this crate targets are nowhere near this
/// deep, so hitting it always means a cycle, not a legitimate graph.
const MAX_NESTING_DEPTH: usize = 64;

/// A snapshot of the graph state for undo/redo.
#[derive(Clone)]
struct HistoryEntry {
    nodes: Vec<FlowNode>,
    edges: Vec<FlowEdge>,
}

/// The central state store for a flow graph. Wrapped in `Entity<FlowState>`.
pub struct FlowState {
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<FlowEdge>,
    /// External imports the graph depends on. Not part of undo/redo history —
    /// nothing in the Canvas edits this list interactively yet, it only
    /// round-trips through load/save.
    pub imports: Vec<FlowImport>,
    node_lookup: HashMap<NodeId, usize>,
    pub viewport: Viewport,
    pub handle_bounds: HashMap<HandleKey, ResolvedHandle>,
    // Interaction state
    pub drag_state: Option<DragState>,
    pub pan_drag: Option<PanDragState>,
    pub connecting: Option<ConnectionDraft>,
    pub selection_box: Option<SelectionBox>,
    /// The edge the cursor is on, when the graph offers inserting a node
    /// into an edge (`FlowGraph::on_edge_insert`). Drives the "+" button
    /// drawn at that edge's midpoint.
    pub hovered_edge: Option<EdgeId>,
    // Undo/redo
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
    // Config
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub snap_to_grid: bool,
    pub snap_grid: (f32, f32),
    pub connection_radius: f32,
}

impl FlowState {
    pub fn new(nodes: Vec<FlowNode>, edges: Vec<FlowEdge>) -> Self {
        let mut node_lookup = HashMap::new();
        for (i, node) in nodes.iter().enumerate() {
            node_lookup.insert(node.id.clone(), i);
        }

        Self {
            nodes,
            edges,
            imports: Vec::new(),
            node_lookup,
            viewport: Viewport::default(),
            handle_bounds: HashMap::new(),
            drag_state: None,
            pan_drag: None,
            connecting: None,
            selection_box: None,
            hovered_edge: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            min_zoom: 0.8,
            max_zoom: 2.5,
            snap_to_grid: false,
            snap_grid: (20.0, 20.0),
            connection_radius: 20.0,
        }
    }

    /// Replace the import list (e.g. right after `FlowState::new` when
    /// loading a file that had `gbk` imports).
    pub fn with_imports(mut self, imports: Vec<FlowImport>) -> Self {
        self.imports = imports;
        self
    }

    /// Rebuild the node lookup index.
    pub fn rebuild_lookup(&mut self) {
        self.node_lookup.clear();
        for (i, node) in self.nodes.iter().enumerate() {
            self.node_lookup.insert(node.id.clone(), i);
        }
    }

    /// Push the current state onto the undo stack. Call before destructive operations.
    pub fn push_undo(&mut self) {
        self.undo_stack.push(HistoryEntry {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        });
        // Clear redo stack on new action
        self.redo_stack.clear();
        // Limit history size
        if self.undo_stack.len() > 100 {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the last operation. Returns true if undo was performed.
    pub fn undo(&mut self) -> bool {
        if let Some(entry) = self.undo_stack.pop() {
            self.redo_stack.push(HistoryEntry {
                nodes: self.nodes.clone(),
                edges: self.edges.clone(),
            });
            self.nodes = entry.nodes;
            self.edges = entry.edges;
            self.rebuild_lookup();
            true
        } else {
            false
        }
    }

    /// Redo the last undone operation. Returns true if redo was performed.
    pub fn redo(&mut self) -> bool {
        if let Some(entry) = self.redo_stack.pop() {
            self.undo_stack.push(HistoryEntry {
                nodes: self.nodes.clone(),
                edges: self.edges.clone(),
            });
            self.nodes = entry.nodes;
            self.edges = entry.edges;
            self.rebuild_lookup();
            true
        } else {
            false
        }
    }

    /// Get a node by ID.
    pub fn get_node(&self, id: &NodeId) -> Option<&FlowNode> {
        self.node_lookup.get(id).map(|&i| &self.nodes[i])
    }

    /// Get a mutable node by ID.
    pub fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut FlowNode> {
        self.node_lookup.get(id).map(|&i| &mut self.nodes[i])
    }

    /// Set all nodes, rebuilding the index.
    pub fn set_nodes(&mut self, nodes: Vec<FlowNode>) {
        self.nodes = nodes;
        self.rebuild_lookup();
    }

    /// Set all edges.
    pub fn set_edges(&mut self, edges: Vec<FlowEdge>) {
        self.edges = edges;
    }

    /// Apply a batch of node changes.
    pub fn apply_node_changes(&mut self, changes: &[NodeChange]) {
        let mut needs_rebuild = false;

        for change in changes {
            match change {
                NodeChange::Position {
                    id,
                    position,
                    dragging,
                } => {
                    if let Some(node) = self.get_node_mut(id) {
                        node.position = *position;
                        node.dragging = *dragging;
                    }
                }
                NodeChange::Dimensions { id, width, height } => {
                    if let Some(node) = self.get_node_mut(id) {
                        node.measured_width = Some(*width);
                        node.measured_height = Some(*height);
                    }
                }
                NodeChange::Select { id, selected } => {
                    if let Some(node) = self.get_node_mut(id) {
                        node.selected = *selected;
                    }
                }
                NodeChange::Remove { id } => {
                    if let Some(&idx) = self.node_lookup.get(id) {
                        self.nodes.remove(idx);
                        needs_rebuild = true;
                    }
                }
            }
        }

        if needs_rebuild {
            self.rebuild_lookup();
        }
    }

    /// Apply a batch of edge changes.
    pub fn apply_edge_changes(&mut self, changes: &[EdgeChange]) {
        for change in changes {
            match change {
                EdgeChange::Select { id, selected } => {
                    if let Some(edge) = self.edges.iter_mut().find(|e| e.id == *id) {
                        edge.selected = *selected;
                    }
                }
                EdgeChange::Remove { id } => {
                    self.edges.retain(|e| e.id != *id);
                }
            }
        }
    }

    /// Set the viewport, clamping zoom to min/max.
    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = Viewport {
            x: viewport.x,
            y: viewport.y,
            zoom: viewport.zoom.clamp(self.min_zoom, self.max_zoom),
        };
    }

    /// Register a resolved handle position (called during layout).
    pub fn set_handle_bounds(&mut self, key: HandleKey, resolved: ResolvedHandle) {
        self.handle_bounds.insert(key, resolved);
    }

    /// Get the direct children of a container node (nodes whose `parent_id`
    /// points at `parent_id`). Empty for an ordinary leaf node.
    pub fn children_of(&self, parent_id: &NodeId) -> Vec<&FlowNode> {
        self.nodes
            .iter()
            .filter(|n| n.parent_id.as_ref() == Some(parent_id))
            .collect()
    }

    /// Whether `node_id` has at least one child — i.e. should render as a
    /// container box rather than an ordinary leaf node.
    pub fn is_container(&self, node_id: &NodeId) -> bool {
        self.nodes
            .iter()
            .any(|n| n.parent_id.as_ref() == Some(node_id))
    }

    /// How many ancestors `node_id` has (0 for a root/top-level node). Used
    /// to guarantee a container always paints behind its own descendants
    /// regardless of their individual `z_index`.
    pub fn depth_of(&self, node_id: &NodeId) -> usize {
        let mut depth = 0;
        let mut current = self.get_node(node_id).and_then(|n| n.parent_id.clone());
        while let Some(pid) = current {
            depth += 1;
            if depth >= MAX_NESTING_DEPTH {
                break;
            }
            current = self.get_node(&pid).and_then(|n| n.parent_id.clone());
        }
        depth
    }

    /// Whether `node_id` is `ancestor_id` itself or nested (at any depth)
    /// inside it — used to reject a reparent that would create a cycle.
    pub fn is_descendant(&self, ancestor_id: &NodeId, node_id: &NodeId) -> bool {
        let mut current = Some(node_id.clone());
        let mut hops = 0;
        while let Some(id) = current {
            if &id == ancestor_id {
                return true;
            }
            hops += 1;
            if hops >= MAX_NESTING_DEPTH {
                break;
            }
            current = self.get_node(&id).and_then(|n| n.parent_id.clone());
        }
        false
    }

    /// The deepest descendant of `ancestor_id` whose laid-out flow-space
    /// bounds contain `point` (flow-space units, as produced by subtracting
    /// the scaled world's origin and dividing by its zoom).
    ///
    /// GPUI dispatches a mouse-down to *every* Normal-behavior hitbox that
    /// contains the cursor (only `BlockMouse` occluders stop the hit-test),
    /// and container boxes render behind their children as siblings. A click
    /// on a node nested inside a loop/if/try body therefore reaches both the
    /// child and the enclosing container, and whichever handler runs last
    /// wins the selection — which is the container's, since it registers
    /// first. Containers call this to rebind the effective click target to
    /// the descendant actually under the cursor instead; `None` means the
    /// click landed on the container's own surface (header/empty interior)
    /// and should select the container itself.
    pub fn descendant_at_flow(&self, ancestor_id: &NodeId, point: (f32, f32)) -> Option<NodeId> {
        let mut deepest: Option<(usize, NodeId)> = None;
        for node in &self.nodes {
            if &node.id == ancestor_id || !self.is_descendant(ancestor_id, &node.id) {
                continue;
            }
            let Some(abs_pos) = self.absolute_position(&node.id) else {
                continue;
            };
            let (fw, fh) = self.node_footprint(node);
            if point.0 >= abs_pos.x
                && point.0 <= abs_pos.x + fw
                && point.1 >= abs_pos.y
                && point.1 <= abs_pos.y + fh
            {
                let depth = self.depth_of(&node.id);
                if deepest.as_ref().map_or(true, |(d, _)| depth > *d) {
                    deepest = Some((depth, node.id.clone()));
                }
            }
        }
        deepest.map(|(_, id)| id)
    }

    /// Returns the deepest node whose laid-out flow-space bounds contain
    /// `point` (flow-space units).
    pub fn node_at_flow(&self, point: (f32, f32)) -> Option<NodeId> {
        self.nodes
            .iter()
            .filter_map(|node| {
                let abs_pos = self.absolute_position(&node.id)?;
                let (width, height) = self.node_footprint(node);
                (point.0 >= abs_pos.x
                    && point.0 <= abs_pos.x + width
                    && point.1 >= abs_pos.y
                    && point.1 <= abs_pos.y + height)
                    .then_some((self.depth_of(&node.id), node.z_index, node.id.clone()))
            })
            .max_by_key(|(depth, z_index, _)| (*depth, *z_index))
            .map(|(_, _, id)| id)
    }

    /// A node's footprint (width, height) in **flow-space units** — its
    /// declared/auto-fit `container_size` if it has children, otherwise its
    /// measured (or estimated) leaf-node wrapper size.
    ///
    /// Both cases are in the same space. The world is laid out once in flow
    /// units and zoomed as a whole by `Style::scale` on the world container
    /// (see `graph.rs::render`), so a leaf's `measured_width`/`height` is
    /// recorded by the measurement canvas *divided by the accumulated element
    /// scale*, and `container_size` is computed by `fit_container` from
    /// children's flow-space local positions. Neither depends on
    /// `viewport.zoom`, so containers no longer need refitting when the user
    /// zooms.
    pub fn node_footprint(&self, node: &FlowNode) -> (f32, f32) {
        if self.is_container(&node.id) {
            node.container_size.unwrap_or(DEFAULT_CONTAINER_SIZE)
        } else {
            (
                node.measured_width.map(|p| p.as_f32()).unwrap_or(114.0),
                node.measured_height.map(|p| p.as_f32()).unwrap_or(54.0),
            )
        }
    }

    /// Resolve a node's position in top-level flow-space coordinates by
    /// walking its parent chain. For a root node this is just `position`
    /// unchanged; for a child, each ancestor level adds that ancestor's own
    /// absolute position plus the fixed interior offset
    /// (`CONTAINER_PADDING`/`CONTAINER_HEADER_HEIGHT`) its children's local
    /// space starts at. `None` if the node (or an ancestor referenced by a
    /// dangling `parent_id`) doesn't exist.
    pub fn absolute_position(&self, node_id: &NodeId) -> Option<FlowPoint> {
        self.absolute_position_at_depth(node_id, 0)
    }

    fn absolute_position_at_depth(&self, node_id: &NodeId, depth: usize) -> Option<FlowPoint> {
        if depth >= MAX_NESTING_DEPTH {
            return None;
        }
        let node = self.get_node(node_id)?;
        match &node.parent_id {
            Some(parent_id) => {
                let parent_abs = self.absolute_position_at_depth(parent_id, depth + 1)?;
                let parent = self.get_node(parent_id)?;
                // Each container reserves its own header strip height
                // (`header_height` when set, else the crate default), which
                // is where its children's local space starts.
                let header_h = parent.header_height.unwrap_or(CONTAINER_HEADER_HEIGHT);
                Some(FlowPoint::new(
                    parent_abs.x + CONTAINER_PADDING + node.position.x,
                    parent_abs.y + header_h + node.position.y,
                ))
            }
            None => Some(node.position),
        }
    }

    /// Auto-size a container's `container_size` to enclose all of its
    /// direct children (by their local positions/footprints) plus
    /// `padding`. Not automatic every frame — call it after adding,
    /// removing, or moving a child, e.g. right after `push_undo` on those
    /// operations. A no-op if the container currently has no children.
    ///
    /// Every quantity here is in **flow-space units**: child positions,
    /// child footprints (`node_footprint`), and the resulting
    /// `container_size`. Because the world is laid out once in flow units and
    /// zoomed as a whole, this function is completely independent of
    /// `viewport.zoom` — a box fitted at any zoom encloses its children at
    /// every zoom.
    ///
    /// Children's local space starts at the interior offset
    /// [`CONTAINER_PADDING`]/[`CONTAINER_HEADER_HEIGHT`] inside the box
    /// (see `absolute_position`), so those are folded into the extents too —
    /// otherwise the header strip would eat into `padding` on the bottom
    /// edge. `padding` is then breathing room on top of all of that.
    ///
    /// `graph.rs`'s measurement canvas re-fits the enclosing containers
    /// whenever a node's *measured* size changes, so boxes track real content
    /// rather than whatever estimate the graph was built with.
    pub fn fit_container(&mut self, container_id: &NodeId, padding: f32) {
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        let mut any = false;
        // Children's local space starts at this container's own header strip
        // height, so the box must leave that much room above their offsets.
        let header_h = self
            .get_node(container_id)
            .and_then(|c| c.header_height)
            .unwrap_or(CONTAINER_HEADER_HEIGHT);
        for node in &self.nodes {
            if node.parent_id.as_ref() != Some(container_id) {
                continue;
            }
            let (w, h) = self.node_footprint(node);
            max_x = max_x.max(CONTAINER_PADDING + node.position.x + w);
            max_y = max_y.max(header_h + node.position.y + h);
            any = true;
        }
        if !any {
            return;
        }
        if let Some(container) = self.get_node_mut(container_id) {
            container.container_size = Some((max_x + padding, max_y + padding));
            container.container_padding = Some(padding);
        }
    }

    /// Re-fit this node's enclosing container(s) after its own footprint
    /// changed (a leaf's `measured_*` size updated, or a nested container
    /// resized). Walks the parent chain innermost-first, re-fitting each
    /// ancestor with its original `FlowNode::container_padding` (falling
    /// back to [`DEFAULT_FIT_PADDING`] for containers that were never
    /// explicitly fitted). A no-op for a top-level node.
    pub fn refit_ancestors(&mut self, node_id: &NodeId) {
        let mut guard = 0;
        let mut current = self.get_node(node_id).and_then(|n| n.parent_id.clone());
        while let Some(parent_id) = current {
            if guard >= MAX_NESTING_DEPTH {
                break;
            }
            let padding = self
                .get_node(&parent_id)
                .and_then(|n| n.container_padding)
                .unwrap_or(DEFAULT_FIT_PADDING);
            self.fit_container(&parent_id, padding);
            current = self.get_node(&parent_id).and_then(|n| n.parent_id.clone());
            guard += 1;
        }
    }

    /// Re-fit every container node against its current children. Layout is
    /// zoom-independent, so this only needs calling after a structural change
    /// (adding, removing, or reparenting nodes) — never when the viewport
    /// zoom changes.
    pub fn refit_all_containers(&mut self) {
        let mut ids: Vec<NodeId> = self
            .nodes
            .iter()
            .filter(|n| self.is_container(&n.id))
            .map(|n| n.id.clone())
            .collect();
        // Innermost containers first, so an outer container's fit sees its
        // nested containers' *updated* sizes (`node_footprint` on a child
        // container reads `container_size`). Shallow depth sorts last.
        ids.sort_by_key(|id| std::cmp::Reverse(self.depth_of(id)));
        for id in ids {
            let padding = self
                .get_node(&id)
                .and_then(|n| n.container_padding)
                .unwrap_or(DEFAULT_FIT_PADDING);
            self.fit_container(&id, padding);
        }
    }

    /// Reparent a node, converting its `position` so its resolved
    /// *absolute* location is unchanged — moving a node into or out of a
    /// container shouldn't make it visually jump. `new_parent_id: None`
    /// makes it a root node again. Rejected silently (no-op) if
    /// `new_parent_id` would create a cycle (is the node itself, or one of
    /// its own descendants) or if either node can't be resolved.
    pub fn set_parent(&mut self, node_id: &NodeId, new_parent_id: Option<NodeId>) {
        if let Some(ref pid) = new_parent_id {
            if pid == node_id || self.is_descendant(node_id, pid) {
                return;
            }
        }
        let Some(abs) = self.absolute_position(node_id) else {
            return;
        };
        let new_local = match &new_parent_id {
            Some(pid) => {
                let Some(parent_abs) = self.absolute_position(pid) else {
                    return;
                };
                FlowPoint::new(
                    abs.x - parent_abs.x - CONTAINER_PADDING,
                    abs.y - parent_abs.y - CONTAINER_HEADER_HEIGHT,
                )
            }
            None => abs,
        };
        if let Some(node) = self.get_node_mut(node_id) {
            node.parent_id = new_parent_id;
            node.position = new_local;
        }
    }

    /// Find the flow-space center of a handle.
    ///
    /// Resolves the node's *absolute* flow-space position (walking any
    /// parent chain — see `absolute_position`) and offsets it by half the
    /// node's flow-space footprint (`node_footprint`), so a handle on a
    /// nested child lands in the right place. Callers convert to screen
    /// space themselves — edge painting multiplies by
    /// `Window::element_scale()` and adds the world origin, while hit-testing
    /// converts an incoming screen point back with
    /// `Viewport::screen_to_flow`.
    pub fn find_handle_center(
        &self,
        node_id: &NodeId,
        _handle_id: &Option<SharedString>,
        handle_position: HandlePosition,
    ) -> Option<(f32, f32)> {
        let node = self.get_node(node_id)?;
        let abs_pos = self.absolute_position(node_id)?;
        let (w, h) = self.node_footprint(node);

        let (cx, cy) = match handle_position {
            HandlePosition::Top => (abs_pos.x + w / 2.0, abs_pos.y),
            HandlePosition::Bottom => (abs_pos.x + w / 2.0, abs_pos.y + h),
            HandlePosition::Left => (abs_pos.x, abs_pos.y + h / 2.0),
            HandlePosition::Right => (abs_pos.x + w, abs_pos.y + h / 2.0),
        };
        Some((cx, cy))
    }

    /// Fit the viewport to show all nodes with padding.
    ///
    /// Only considers top-level (`parent_id.is_none()`) nodes — a child's
    /// extent is already covered by its container's own footprint
    /// (`node_footprint`, container-size-aware), so including it too would
    /// be redundant, not more accurate.
    pub fn fit_view(&mut self, padding: f32, container_width: f32, container_height: f32) {
        if self.nodes.is_empty() {
            return;
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for node in &self.nodes {
            if node.hidden || node.parent_id.is_some() {
                continue;
            }
            let (w, h) = self.node_footprint(node);
            min_x = min_x.min(node.position.x);
            min_y = min_y.min(node.position.y);
            max_x = max_x.max(node.position.x + w);
            max_y = max_y.max(node.position.y + h);
        }

        let content_width = max_x - min_x + padding * 2.0;
        let content_height = max_y - min_y + padding * 2.0;

        if content_width <= 0.0 || content_height <= 0.0 {
            return;
        }

        let zoom_x = container_width / content_width;
        let zoom_y = container_height / content_height;
        let ideal_zoom = zoom_x.min(zoom_y);

        // `min_zoom` exists to keep interactive zoom/pan usable (don't let a
        // scroll-wheel zoom out past the point controls become unusable) —
        // it isn't meant to cap how far a "fit everything" request can go.
        // A graph taller or wider than `min_zoom` allows for (a tall
        // top-to-bottom workflow, a wide schema) used to get its computed
        // zoom clamped back up here, silently breaking the fit: content
        // past the container's edge was cut off instead of shown. Callers
        // used to have to know to pre-raise `min_zoom` by hand before
        // calling this (see `gpui_flow`'s own kitchen-sink example); widen
        // it here instead so `fit_view` always actually fits, and later
        // manual zoom-out can reach the same level without snapping back.
        //
        // Only ever widens the floor, never narrows it — a graph that
        // already fit within the existing `min_zoom` computes an
        // `ideal_zoom` at or above it and this is a no-op, so nothing that
        // worked before changes.
        const ABSOLUTE_MIN_ZOOM: f32 = 0.05;
        if ideal_zoom < self.min_zoom {
            self.min_zoom = ideal_zoom.max(ABSOLUTE_MIN_ZOOM);
        }
        let zoom = ideal_zoom.clamp(self.min_zoom, self.max_zoom);

        self.viewport.zoom = zoom;
        self.viewport.x = (container_width - content_width * zoom) / 2.0 - (min_x - padding) * zoom;
        self.viewport.y =
            (container_height - content_height * zoom) / 2.0 - (min_y - padding) * zoom;
    }

    /// Zoom in by a step, centered on the container center.
    pub fn zoom_in(&mut self, container_width: f32, container_height: f32) {
        let factor = 1.2;
        let cx = container_width / 2.0;
        let cy = container_height / 2.0;
        let old_zoom = self.viewport.zoom;
        let new_zoom = (old_zoom * factor).clamp(self.min_zoom, self.max_zoom);
        self.viewport.x = cx - (cx - self.viewport.x) * (new_zoom / old_zoom);
        self.viewport.y = cy - (cy - self.viewport.y) * (new_zoom / old_zoom);
        self.viewport.zoom = new_zoom;
    }

    /// Zoom out by a step, centered on the container center.
    pub fn zoom_out(&mut self, container_width: f32, container_height: f32) {
        let factor = 1.0 / 1.2;
        let cx = container_width / 2.0;
        let cy = container_height / 2.0;
        let old_zoom = self.viewport.zoom;
        let new_zoom = (old_zoom * factor).clamp(self.min_zoom, self.max_zoom);
        self.viewport.x = cx - (cx - self.viewport.x) * (new_zoom / old_zoom);
        self.viewport.y = cy - (cy - self.viewport.y) * (new_zoom / old_zoom);
        self.viewport.zoom = new_zoom;
    }

    /// Center the viewport on a specific flow coordinate.
    pub fn set_center(
        &mut self,
        flow_x: f32,
        flow_y: f32,
        container_width: f32,
        container_height: f32,
    ) {
        self.viewport.x = container_width / 2.0 - flow_x * self.viewport.zoom;
        self.viewport.y = container_height / 2.0 - flow_y * self.viewport.zoom;
    }

    /// Select all nodes and edges.
    pub fn select_all(&mut self) {
        for node in &mut self.nodes {
            if node.selectable {
                node.selected = true;
            }
        }
        for edge in &mut self.edges {
            if edge.selectable {
                edge.selected = true;
            }
        }
    }

    /// Moves the single-node selection to the next (`forward: true`) or
    /// previous node in `self.nodes`' order — creation order, the same order
    /// `undo`/`redo` already treat as canonical, rather than a spatial
    /// left-to-right/top-to-bottom order that would need recomputing on
    /// every layout change. Wraps at both ends; selects the first eligible
    /// node when none was selected yet.
    ///
    /// Skips hidden and non-`selectable` nodes (the same guard `select_all`
    /// uses) and clears every other node's selection, matching a plain
    /// (non-multi) click. A container's children are included, not just
    /// top-level nodes — unlike [`Self::fit_view`], which only needs a
    /// container's own footprint, this is for reaching a specific node to
    /// inspect or edit, and a nested action is as valid a target as any
    /// other.
    pub fn select_next_node(&mut self, forward: bool) {
        let eligible: Vec<usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| !n.hidden && n.selectable)
            .map(|(i, _)| i)
            .collect();
        if eligible.is_empty() {
            return;
        }

        let current = self.nodes.iter().position(|n| n.selected);
        let next_pos = match current.and_then(|cur_ix| eligible.iter().position(|&i| i == cur_ix)) {
            Some(pos) if forward => (pos + 1) % eligible.len(),
            Some(pos) => (pos + eligible.len() - 1) % eligible.len(),
            None => 0,
        };
        let next_ix = eligible[next_pos];

        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.selected = i == next_ix;
        }
    }

    /// Get nodes that have edges pointing TO the given node.
    pub fn get_incomers(&self, node_id: &NodeId) -> Vec<&FlowNode> {
        self.edges
            .iter()
            .filter(|e| e.target == *node_id)
            .filter_map(|e| self.get_node(&e.source))
            .collect()
    }

    /// Get nodes that the given node has edges pointing TO.
    pub fn get_outgoers(&self, node_id: &NodeId) -> Vec<&FlowNode> {
        self.edges
            .iter()
            .filter(|e| e.source == *node_id)
            .filter_map(|e| self.get_node(&e.target))
            .collect()
    }

    /// Get all edges connected to a node (as source or target).
    pub fn get_connected_edges(&self, node_id: &NodeId) -> Vec<&FlowEdge> {
        self.edges
            .iter()
            .filter(|e| e.source == *node_id || e.target == *node_id)
            .collect()
    }

    /// Find the nearest valid handle to snap to during connection dragging.
    ///
    /// `mouse` is in **flow-space units**, matching `find_handle_center`'s
    /// return value (and therefore `SnapTarget::point`). `connection_radius`
    /// is a screen-space interaction threshold, so it is divided by the
    /// current zoom to keep the same on-screen snap distance at any zoom.
    pub fn find_snap_target(
        &self,
        draft: &ConnectionDraft,
        mouse: (f32, f32),
    ) -> Option<SnapTarget> {
        let radius = self.connection_radius / self.viewport.zoom;
        let mut best: Option<(f32, SnapTarget)> = None;

        for node in &self.nodes {
            if node.hidden {
                continue;
            }
            // Can't connect to the same node
            if node.id == draft.from_node {
                continue;
            }

            for handle in &node.handles {
                // Source→Target or Target→Source only
                if handle.handle_type == draft.from_type {
                    continue;
                }
                // Check per-handle connectable flag
                if !handle.is_connectable {
                    continue;
                }

                if let Some((hx, hy)) =
                    self.find_handle_center(&node.id, &handle.id, handle.position)
                {
                    let dx = mouse.0 - hx;
                    let dy = mouse.1 - hy;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist <= radius {
                        if best.is_none() || dist < best.as_ref().unwrap().0 {
                            best = Some((
                                dist,
                                SnapTarget {
                                    node_id: node.id.clone(),
                                    handle_id: handle.id.clone(),
                                    handle_type: handle.handle_type,
                                    position: handle.position,
                                    point: (hx, hy),
                                },
                            ));
                        }
                    }
                }
            }
        }

        best.map(|(_, target)| target)
    }

    /// Check if a connection is valid (default rules).
    pub fn is_valid_connection(&self, connection: &Connection) -> bool {
        // No self-connections
        if connection.source == connection.target {
            return false;
        }

        // No duplicate edges
        let duplicate = self.edges.iter().any(|e| {
            e.source == connection.source
                && e.target == connection.target
                && e.source_handle == connection.source_handle
                && e.target_handle == connection.target_handle
        });
        if duplicate {
            return false;
        }

        true
    }

    /// Add an edge from a completed connection.
    pub fn add_edge_from_connection(
        &mut self,
        connection: &Connection,
        edge_id: impl Into<SharedString>,
    ) {
        let mut edge = FlowEdge::new(
            edge_id,
            connection.source.clone(),
            connection.target.clone(),
        );
        if let Some(ref sh) = connection.source_handle {
            edge.source_handle = Some(sh.clone());
        }
        if let Some(ref th) = connection.target_handle {
            edge.target_handle = Some(th.clone());
        }
        self.edges.push(edge);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One child is enclosed by its container box iff the box's flow-space
    /// edge lands at/right of (and below) where the child's flow-space
    /// footprint ends. Mirrors `absolute_position` + `fit_container`, in case
    /// those ever drift apart.
    ///
    /// No zoom argument: layout is performed once in flow units and scaled by
    /// the world container, so enclosure is a purely flow-space question and
    /// holds at every zoom.
    fn box_encloses_node(state: &FlowState, container: &FlowNode, child: &FlowNode) -> bool {
        let (cw, ch) = container.container_size.unwrap_or(DEFAULT_CONTAINER_SIZE);
        let (w, h) = state.node_footprint(child);
        let header_h = container.header_height.unwrap_or(CONTAINER_HEADER_HEIGHT);
        let (cx, cy) = (
            CONTAINER_PADDING + child.position.x,
            header_h + child.position.y,
        );
        cw >= cx + w && ch >= cy + h
    }

    fn leaf_child(id: &str, x: f32, y: f32, w_flow: f32, h_flow: f32, parent: &str) -> FlowNode {
        let mut n = FlowNode::new(id, x, y);
        n.parent_id = Some(parent.into());
        n.measured_width = Some(Pixels::from(w_flow));
        n.measured_height = Some(Pixels::from(h_flow));
        n
    }

    #[test]
    fn fit_container_encloses_children() {
        // Container with two stacked leaves. Fitting happens entirely in flow
        // units, so the resulting box must enclose both leaves' flow-space
        // footprints.
        let mut state = FlowState::new(
            vec![
                FlowNode::new("c", 0.0, 0.0).container_size(320.0, 220.0),
                leaf_child("a", 20.0, 20.0, 172.0, 46.0, "c"),
                leaf_child("b", 20.0, 150.0, 172.0, 46.0, "c"),
            ],
            vec![],
        );
        state.fit_container(&"c".into(), 24.0);

        let (c, a, b) = {
            let nodes = &state.nodes;
            (
                nodes.iter().find(|n| n.id == "c").unwrap(),
                nodes.iter().find(|n| n.id == "a").unwrap(),
                nodes.iter().find(|n| n.id == "b").unwrap(),
            )
        };
        assert!(
            box_encloses_node(&state, c, a),
            "'a' (top leaf) not enclosed — box {}x{}, leaf at {}x{}",
            c.container_size.unwrap().0,
            c.container_size.unwrap().1,
            a.position.x,
            a.position.y,
        );
        assert!(
            box_encloses_node(&state, c, b),
            "'b' (bottom leaf) not enclosed — box {}x{}, leaf at {}x{}",
            c.container_size.unwrap().0,
            c.container_size.unwrap().1,
            b.position.x,
            b.position.y,
        );
    }

    #[test]
    fn fitted_box_is_independent_of_zoom() {
        // The world is laid out once in flow units and zoomed by the
        // framework, so a container's fitted size must be a pure function of
        // its children — zooming must not resize it. (This is the regression
        // guard for the old `/ zoom` in `fit_container`.)
        let build = || {
            FlowState::new(
                vec![
                    FlowNode::new("c", 0.0, 0.0).container_size(320.0, 220.0),
                    leaf_child("a", 20.0, 20.0, 172.0, 46.0, "c"),
                ],
                vec![],
            )
        };

        let mut at_1 = build();
        at_1.viewport.zoom = 1.0;
        at_1.fit_container(&"c".into(), 24.0);
        let size_at_1 = at_1.get_node(&"c".into()).unwrap().container_size.unwrap();

        let mut zoomed_out = build();
        zoomed_out.viewport.zoom = 0.35;
        zoomed_out.fit_container(&"c".into(), 24.0);
        let size_zoomed_out = zoomed_out
            .get_node(&"c".into())
            .unwrap()
            .container_size
            .unwrap();

        assert_eq!(
            size_at_1, size_zoomed_out,
            "the fitted box must not depend on the viewport zoom"
        );
        assert!(
            box_encloses_node(
                &zoomed_out,
                zoomed_out.get_node(&"c".into()).unwrap(),
                zoomed_out.get_node(&"a".into()).unwrap()
            ),
            "the box must still enclose its child at 0.35x"
        );
    }

    #[test]
    fn nested_container_encloses_child_container() {
        // ifVip-style: child containers fitted first, parent fitted against
        // their *fitted* sizes (not the 320x220 default).
        let mut state = FlowState::new(
            vec![
                FlowNode::new("ifVip", 0.0, 0.0),
                FlowNode::new("branchTrue", 20.0, 20.0).parent("ifVip"),
                leaf_child("a", 20.0, 20.0, 172.0, 46.0, "branchTrue"),
                leaf_child("b", 20.0, 280.0, 172.0, 46.0, "branchTrue"),
                FlowNode::new("branchFalse", 320.0, 20.0).parent("ifVip"),
                leaf_child("c", 20.0, 20.0, 172.0, 46.0, "branchFalse"),
            ],
            vec![],
        );
        state.fit_container(&"branchTrue".into(), 80.0);
        state.fit_container(&"branchFalse".into(), 80.0);
        state.fit_container(&"ifVip".into(), 24.0);

        let if_vip = state.get_node(&"ifVip".into()).unwrap();
        let branch_true = state.get_node(&"branchTrue".into()).unwrap();
        assert!(
            box_encloses_node(&state, if_vip, branch_true),
            "ifVip box {}x{} does not enclose branchTrue {}x{} at its fitted size",
            if_vip.container_size.unwrap().0,
            if_vip.container_size.unwrap().1,
            branch_true.container_size.unwrap().0,
            branch_true.container_size.unwrap().1,
        );
    }

    #[test]
    fn measuring_a_leaf_refits_its_container() {
        let mut state = FlowState::new(
            vec![
                FlowNode::new("c", 0.0, 0.0).container_size(320.0, 220.0),
                leaf_child("a", 20.0, 20.0, 172.0, 46.0, "c"),
            ],
            vec![],
        );
        state.fit_container(&"c".into(), 24.0);
        let before = state
            .get_node(&"c".into())
            .unwrap()
            .container_size
            .unwrap()
            .0;

        // Leaf measures wider than its bootstrap estimate (e.g. a long label).
        state.get_node_mut(&"a".into()).unwrap().measured_width = Some(Pixels::from(240.0));
        state.refit_ancestors(&"a".into());

        let after = state
            .get_node(&"c".into())
            .unwrap()
            .container_size
            .unwrap()
            .0;
        assert!(
            after > before,
            "container width should grow when a child measures wider: {before} -> {after}"
        );
    }

    #[test]
    fn custom_header_height_reserves_room_for_children() {
        // A container with a tall dedicated header (e.g. its own chrome +
        // title, 56 units) must push children down past it in
        // `absolute_position`, and `fit_container` must enclose them there.
        let mut state = FlowState::new(
            vec![
                FlowNode::new("c", 0.0, 0.0).header_height(56.0),
                leaf_child("a", 20.0, 20.0, 172.0, 46.0, "c"),
            ],
            vec![],
        );
        state.fit_container(&"c".into(), 24.0);

        {
            let (c, a) = (
                state.get_node(&"c".into()).unwrap(),
                state.get_node(&"a".into()).unwrap(),
            );
            assert!(
                box_encloses_node(&state, c, a),
                "box {}x{} must enclose child sitting below the 56-unit header",
                c.container_size.unwrap().0,
                c.container_size.unwrap().1,
            );
        }

        let abs = state.absolute_position(&"a".into()).unwrap();
        assert_eq!(
            abs.y, 76.0,
            "child's absolute y should sit below the 56-unit header (got {})",
            abs.y
        );
    }

    #[test]
    fn fit_view_widens_min_zoom_instead_of_clipping() {
        // A tall, narrow layout — three nodes stacked well past what the
        // default `min_zoom` (0.8) allows to fit into a normal-sized
        // container. Before this fix, `fit_view`'s clamp against the old,
        // narrower `min_zoom` would win, leaving the bottom node's footprint
        // past the visible viewport instead of actually fitting it.
        let mut state = FlowState::new(
            vec![
                FlowNode::new("a", 0.0, 0.0).size(200.0, 100.0),
                FlowNode::new("b", 0.0, 400.0).size(200.0, 100.0),
                FlowNode::new("c", 0.0, 800.0).size(200.0, 100.0),
            ],
            vec![],
        );
        assert_eq!(state.min_zoom, 0.8, "test assumes the documented default");

        state.fit_view(60.0, 900.0, 600.0);

        // Content spans y in [0, 900] plus 60 padding on each side = 1020,
        // so fitting it into a 600-tall container needs zoom ≈ 0.588 — below
        // the original 0.8 floor.
        assert!(
            state.viewport.zoom < 0.8,
            "fit_view should have zoomed out past the interactive floor to fit everything, got {}",
            state.viewport.zoom
        );
        assert!(
            (state.min_zoom - state.viewport.zoom).abs() < f32::EPSILON,
            "min_zoom should widen to match the zoom fit_view actually used, so a later \
             manual zoom-out can return to it (min_zoom={}, viewport.zoom={})",
            state.min_zoom,
            state.viewport.zoom
        );

        // Every node's footprint must land fully inside the container at the
        // zoom `fit_view` settled on — the actual "nothing gets clipped"
        // guarantee, checked directly rather than just inferred from zoom.
        let zoom = state.viewport.zoom;
        for id in ["a", "b", "c"] {
            let node = state.get_node(&id.into()).unwrap();
            let (w, h) = state.node_footprint(node);
            let screen_x = state.viewport.x + node.position.x * zoom;
            let screen_y = state.viewport.y + node.position.y * zoom;
            assert!(
                screen_x >= 0.0 && screen_x + w * zoom <= 900.0,
                "node {id} horizontally clipped: [{screen_x}, {}] outside [0, 900]",
                screen_x + w * zoom
            );
            assert!(
                screen_y >= 0.0 && screen_y + h * zoom <= 600.0,
                "node {id} vertically clipped: [{screen_y}, {}] outside [0, 600]",
                screen_y + h * zoom
            );
        }
    }

    #[test]
    fn fit_view_leaves_min_zoom_untouched_when_content_already_fits() {
        // A single small node fits comfortably within the default min/max
        // zoom range, so fit_view must not perturb `min_zoom` at all — the
        // widening only ever kicks in when it's actually needed.
        let mut state = FlowState::new(vec![FlowNode::new("a", 0.0, 0.0).size(100.0, 60.0)], vec![]);
        state.fit_view(60.0, 900.0, 600.0);

        assert_eq!(
            state.min_zoom, 0.8,
            "min_zoom should be unchanged when the ideal zoom is already within range"
        );
        assert!(
            state.viewport.zoom >= 0.8 && state.viewport.zoom <= 2.5,
            "zoom should land within the default range, got {}",
            state.viewport.zoom
        );
    }

    #[test]
    fn select_next_node_cycles_in_creation_order_and_wraps() {
        let mut state = FlowState::new(
            vec![
                FlowNode::new("a", 0.0, 0.0),
                FlowNode::new("b", 0.0, 0.0),
                FlowNode::new("c", 0.0, 0.0),
            ],
            vec![],
        );

        // Nothing selected yet: lands on the first node.
        state.select_next_node(true);
        assert_eq!(selected_id(&state), Some("a"));

        state.select_next_node(true);
        assert_eq!(selected_id(&state), Some("b"));
        state.select_next_node(true);
        assert_eq!(selected_id(&state), Some("c"));
        // Wraps forward past the last node.
        state.select_next_node(true);
        assert_eq!(selected_id(&state), Some("a"));

        // And backward past the first.
        state.select_next_node(false);
        assert_eq!(selected_id(&state), Some("c"));

        // Selecting the next node clears the previous one's flag — single
        // selection, matching a plain (non-multi) click.
        let selected_count = state.nodes.iter().filter(|n| n.selected).count();
        assert_eq!(selected_count, 1);
    }

    #[test]
    fn select_next_node_skips_hidden_and_unselectable() {
        let mut state = FlowState::new(
            vec![
                FlowNode::new("a", 0.0, 0.0),
                {
                    let mut hidden = FlowNode::new("b-hidden", 0.0, 0.0);
                    hidden.hidden = true;
                    hidden
                },
                {
                    let mut unselectable = FlowNode::new("c-unselectable", 0.0, 0.0);
                    unselectable.selectable = false;
                    unselectable
                },
                FlowNode::new("d", 0.0, 0.0),
            ],
            vec![],
        );

        state.select_next_node(true);
        assert_eq!(selected_id(&state), Some("a"));
        // Skips both "b-hidden" and "c-unselectable" in one step.
        state.select_next_node(true);
        assert_eq!(selected_id(&state), Some("d"));
        // Wraps straight back to "a", still skipping both.
        state.select_next_node(true);
        assert_eq!(selected_id(&state), Some("a"));
    }

    fn selected_id(state: &FlowState) -> Option<&str> {
        state.nodes.iter().find(|n| n.selected).map(|n| n.id.as_ref())
    }
}
