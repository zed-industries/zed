use gpui::{Pixels, SharedString};

/// Unique identifier for a node.
pub type NodeId = SharedString;

/// Unique identifier for an edge.
pub type EdgeId = SharedString;

/// A 2D point in flow (graph) coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FlowPoint {
    pub x: f32,
    pub y: f32,
}

impl FlowPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// The type of a handle — determines connection direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandleType {
    Source,
    Target,
}

/// Where on the node a handle is positioned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandlePosition {
    Top,
    Right,
    Bottom,
    Left,
}

/// Definition of a handle on a node.
#[derive(Debug, Clone)]
pub struct HandleDef {
    pub id: Option<SharedString>,
    pub handle_type: HandleType,
    pub position: HandlePosition,
    /// Whether this handle can accept/start connections. Default: true.
    pub is_connectable: bool,
}

impl HandleDef {
    pub fn source(position: HandlePosition) -> Self {
        Self {
            id: None,
            handle_type: HandleType::Source,
            position,
            is_connectable: true,
        }
    }

    pub fn target(position: HandlePosition) -> Self {
        Self {
            id: None,
            handle_type: HandleType::Target,
            position,
            is_connectable: true,
        }
    }

    pub fn connectable(mut self, connectable: bool) -> Self {
        self.is_connectable = connectable;
        self
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// Background grid pattern style.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BackgroundPattern {
    #[default]
    Dots,
    Lines,
    Cross,
}

/// The visual type and parameters of an edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeType {
    Bezier { curvature: f32 },
    SmoothStep { border_radius: f32, offset: f32 },
    Straight,
}

impl Default for EdgeType {
    fn default() -> Self {
        EdgeType::Bezier { curvature: 0.25 }
    }
}

/// A node in the flow graph.
#[derive(Debug, Clone)]
pub struct FlowNode {
    pub id: NodeId,
    pub position: FlowPoint,
    pub node_type: Option<SharedString>,
    pub label: SharedString,
    pub handles: Vec<HandleDef>,
    pub selected: bool,
    pub dragging: bool,
    pub draggable: bool,
    pub selectable: bool,
    pub connectable: bool,
    pub deletable: bool,
    pub hidden: bool,
    pub z_index: i32,
    pub measured_width: Option<Pixels>,
    pub measured_height: Option<Pixels>,
    /// Opt-in per node: when true, right-clicking this node fires
    /// `FlowGraph`'s `on_node_context_menu` callback (if one is set).
    /// Off by default — most consumers (e.g. this crate's own demo
    /// renderers) don't want a context menu. Intended for editors that let
    /// users attach arbitrary metadata to nodes (properties, a schema
    /// designer's column list, a task chain's per-step config, ...); this
    /// crate stays opinion-free about what the menu contains.
    pub context_menu: bool,
    /// Free-form key/value metadata, rendered and edited entirely by the
    /// consumer (this crate never reads or writes it itself). A natural
    /// fit for a node context menu's "Add Property" action.
    pub properties: Vec<(SharedString, SharedString)>,
    /// When set, this node is a *child* of the named node — a "container"
    /// (e.g. a loop body) can hold its own independent sub-graph of nodes.
    /// `position` for a child is interpreted as an offset *relative to its
    /// parent's own interior origin*, not an absolute flow-space
    /// coordinate — see `FlowState::absolute_position`. Left `None` for an
    /// ordinary top-level node (the overwhelming majority).
    pub parent_id: Option<NodeId>,
    /// Explicit footprint (width, height) for a node that has children —
    /// forces the container's own rendered box to this size (rather than
    /// intrinsic content sizing) so there's room to lay children out inside
    /// it. Ignored for a node with no children. `None` falls back to
    /// `store::DEFAULT_CONTAINER_SIZE`; call `FlowState::fit_container` to
    /// auto-size this from the current children instead of guessing.
    pub container_size: Option<(f32, f32)>,
    /// The last `padding` passed to `FlowState::fit_container` for this
    /// container. Remembered so the crate can re-fit the box later — when a
    /// child's `measured_*` size updates in `graph.rs`'s measurement canvas,
    /// or when `viewport.zoom` changes — using the consumer's original
    /// padding instead of a hard-coded default. `None` (never fitted) makes
    /// re-fits fall back to `store::DEFAULT_FIT_PADDING`.
    pub container_padding: Option<f32>,
    /// Height of this container's dedicated header strip, in flow-space
    /// units. Defaults to `store::CONTAINER_HEADER_HEIGHT` when `None`.
    /// Children's local space starts below it (see
    /// `FlowState::absolute_position`) and `FlowState::fit_container` folds
    /// it into the fitted box. Ignored for leaf nodes.
    pub header_height: Option<f32>,
    /// Overrides `FlowGraph::node_border_color` for just this node's own
    /// box, thicker (`border_2` instead of `border_1`) when set. This
    /// crate stays opinion-free about *why* — a consumer driving live
    /// status (a running/succeeded/failed node in a workflow executor, a
    /// validation error, ...) sets a raw color; this crate just draws it.
    /// `None` (the overwhelming majority of nodes) keeps the graph's
    /// ordinary uniform border.
    pub accent_border: Option<u32>,
    /// Marks this node as failing validation, drawn as a
    /// `FlowGraph::node_error_color` ring.
    ///
    /// Deliberately a *separate* flag from [`Self::accent_border`] rather than
    /// just another value in it: `accent_border` is a single slot that live run
    /// status also writes, and the two have different lifetimes. A node can be
    /// red because it just failed *and* red because it has a blocking
    /// validation error, and the "clear the run marker when the user edits
    /// this node" path must not wipe the validation ring on its way past.
    /// Keeping them apart means each subsystem owns its own field and neither
    /// has to know the other exists.
    ///
    /// Wins over `accent_border` *and* over the selection ring, since a node
    /// the user is looking at still needs its blocking error to be visible.
    pub validation_error: bool,
}

impl FlowNode {
    pub fn new(id: impl Into<SharedString>, x: f32, y: f32) -> Self {
        Self {
            id: id.into(),
            position: FlowPoint::new(x, y),
            node_type: None,
            label: SharedString::default(),
            handles: Vec::new(),
            selected: false,
            dragging: false,
            draggable: true,
            selectable: true,
            connectable: true,
            deletable: true,
            hidden: false,
            z_index: 0,
            measured_width: None,
            measured_height: None,
            context_menu: false,
            properties: Vec::new(),
            parent_id: None,
            container_size: None,
            container_padding: None,
            header_height: None,
            accent_border: None,
            validation_error: false,
        }
    }

    /// Make this node a child of `parent_id` — see the field doc comment on
    /// `parent_id` for how `position` is then interpreted.
    pub fn parent(mut self, parent_id: impl Into<SharedString>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// Force this container node's own rendered box to `(width, height)` —
    /// see the field doc comment on `container_size`.
    pub fn container_size(mut self, width: f32, height: f32) -> Self {
        self.container_size = Some((width, height));
        self
    }

    pub fn context_menu(mut self, enabled: bool) -> Self {
        self.context_menu = enabled;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn node_type(mut self, node_type: impl Into<SharedString>) -> Self {
        self.node_type = Some(node_type.into());
        self
    }

    pub fn handles(mut self, handles: Vec<HandleDef>) -> Self {
        self.handles = handles;
        self
    }

    /// Set initial estimated size (width, height in pixels).
    /// Used for edge positioning before the node is rendered and measured.
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.measured_width = Some(gpui::px(width));
        self.measured_height = Some(gpui::px(height));
        self
    }

    /// Set the height of this container's dedicated header strip (flow
    /// units). See the `header_height` field doc.
    pub fn header_height(mut self, height: f32) -> Self {
        self.header_height = Some(height);
        self
    }

    pub fn z_index(mut self, z: i32) -> Self {
        self.z_index = z;
        self
    }

    /// See the `accent_border` field doc.
    pub fn accent_border(mut self, color: impl Into<Option<u32>>) -> Self {
        self.accent_border = color.into();
        self
    }

    /// See the `validation_error` field doc.
    pub fn validation_error(mut self, validation_error: bool) -> Self {
        self.validation_error = validation_error;
        self
    }
}

/// An edge connecting two nodes.
#[derive(Debug, Clone)]
pub struct FlowEdge {
    pub id: EdgeId,
    pub source: NodeId,
    pub target: NodeId,
    pub source_handle: Option<SharedString>,
    pub target_handle: Option<SharedString>,
    pub edge_type: EdgeType,
    pub selected: bool,
    pub selectable: bool,
    pub deletable: bool,
    pub hidden: bool,
    pub animated: bool,
    /// Optional label displayed at the edge midpoint.
    pub label: Option<SharedString>,
    /// Custom edge color as 0xRRGGBB. If None, uses the default.
    pub color: Option<u32>,
    /// Custom stroke width. If None, uses default (2.0).
    pub stroke_width: Option<f32>,
}

impl FlowEdge {
    pub fn new(
        id: impl Into<SharedString>,
        source: impl Into<SharedString>,
        target: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            source: source.into(),
            target: target.into(),
            source_handle: None,
            target_handle: None,
            edge_type: EdgeType::default(),
            selected: false,
            selectable: true,
            deletable: true,
            hidden: false,
            animated: false,
            label: None,
            color: None,
            stroke_width: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = Some(width);
        self
    }

    pub fn source_handle(mut self, handle: impl Into<SharedString>) -> Self {
        self.source_handle = Some(handle.into());
        self
    }

    pub fn target_handle(mut self, handle: impl Into<SharedString>) -> Self {
        self.target_handle = Some(handle.into());
        self
    }

    pub fn edge_type(mut self, edge_type: EdgeType) -> Self {
        self.edge_type = edge_type;
        self
    }
}

/// One external import a flow graph depends on (ForgeFlow's `gbk <name>
/// vannaf "<path>"` — see `forge_gpui`'s `fdgn.rs`). Generic graph state
/// otherwise has no notion of a file format; this exists purely so a loaded
/// graph can carry its import list through edits and round-trip it back out
/// on save instead of silently dropping it.
#[derive(Debug, Clone, PartialEq)]
pub struct FlowImport {
    pub name: SharedString,
    pub path: SharedString,
}

impl FlowImport {
    pub fn new(name: impl Into<SharedString>, path: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }
}

/// Viewport state: pan offset and zoom level.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
}

impl Viewport {
    /// Convert a point from flow coordinates to screen coordinates.
    pub fn flow_to_screen(&self, point: FlowPoint) -> (f32, f32) {
        (point.x * self.zoom + self.x, point.y * self.zoom + self.y)
    }

    /// Convert a point from screen coordinates to flow coordinates.
    pub fn screen_to_flow(&self, screen_x: f32, screen_y: f32) -> FlowPoint {
        FlowPoint::new(
            (screen_x - self.x) / self.zoom,
            (screen_y - self.y) / self.zoom,
        )
    }
}

/// State tracked during a node drag operation.
#[derive(Debug, Clone)]
pub struct DragState {
    /// Screen coordinates where the drag started.
    pub origin_mouse: (f32, f32),
    /// Original flow-space positions of all dragged nodes.
    pub node_origins: Vec<(NodeId, FlowPoint)>,
}

/// State tracked during a canvas pan drag.
#[derive(Debug, Clone, Copy)]
pub struct PanDragState {
    /// Screen coordinates where the drag started.
    pub start_mouse: (f32, f32),
    /// Viewport position at drag start.
    pub start_viewport: (f32, f32),
}

/// State tracked during a box/lasso selection.
#[derive(Debug, Clone, Copy)]
pub struct SelectionBox {
    /// Screen coordinates where the selection started.
    pub start: (f32, f32),
    /// Current screen coordinates of the selection end.
    pub current: (f32, f32),
}

/// State tracked while dragging a connection from a handle.
#[derive(Debug, Clone)]
pub struct ConnectionDraft {
    pub from_node: NodeId,
    pub from_handle: Option<SharedString>,
    pub from_type: HandleType,
    pub from_position: HandlePosition,
    /// Screen coordinates of the source handle center.
    pub from_point: (f32, f32),
    /// Current mouse screen coordinates (the dangling end).
    pub to_point: (f32, f32),
    /// If the cursor is near a valid target handle, snap to it.
    pub snap_target: Option<SnapTarget>,
}

/// A handle that the connection draft is snapping to.
#[derive(Debug, Clone)]
pub struct SnapTarget {
    pub node_id: NodeId,
    pub handle_id: Option<SharedString>,
    pub handle_type: HandleType,
    pub position: HandlePosition,
    /// Screen coordinates of the snapped handle center.
    pub point: (f32, f32),
}

/// A completed connection between two handles.
#[derive(Debug, Clone)]
pub struct Connection {
    pub source: NodeId,
    pub target: NodeId,
    pub source_handle: Option<SharedString>,
    pub target_handle: Option<SharedString>,
}

/// Change events for nodes.
#[derive(Debug, Clone)]
pub enum NodeChange {
    Position {
        id: NodeId,
        position: FlowPoint,
        dragging: bool,
    },
    Dimensions {
        id: NodeId,
        width: Pixels,
        height: Pixels,
    },
    Select {
        id: NodeId,
        selected: bool,
    },
    Remove {
        id: NodeId,
    },
}

/// Change events for edges.
#[derive(Debug, Clone)]
pub enum EdgeChange {
    Select { id: EdgeId, selected: bool },
    Remove { id: EdgeId },
}
