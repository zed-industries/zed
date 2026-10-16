use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::edges;
use crate::store::FlowState;
use crate::types::*;

/// Diameter, in flow units, of the "+" button shown on a hovered edge. Also
/// how far from the edge's midpoint the cursor can be and still count as on
/// the button.
const EDGE_INSERT_BUTTON_SIZE: f32 = 18.0;

type NodeRendererFn = Box<dyn Fn(&FlowNode, &mut Window, &mut App) -> AnyElement>;

/// Convert a window-absolute mouse position into **flow-space units**.
///
/// The world is laid out once in flow units inside a `.scale(zoom)` container
/// whose window-space origin is `world_origin` (captured from the world
/// `canvas()`'s bounds each paint). So the inverse of the layout transform
/// `window = world_origin + flow * zoom` is
/// `flow = (window_position - world_origin) / zoom`. The origin already
/// includes the viewport pan, so applying `Viewport::screen_to_flow` here
/// would subtract that pan a second time.
///
/// Mouse events always deliver window-absolute positions regardless of the
/// enclosing element's scale, so every absolute-position comparison (node
/// picking, edge hit-testing, snapping, box-select) needs this.
fn window_to_flow(
    viewport: &Viewport,
    world_origin: Point<Pixels>,
    pos: Point<Pixels>,
) -> (f32, f32) {
    let local_x = pos.x.as_f32() - world_origin.x.as_f32();
    let local_y = pos.y.as_f32() - world_origin.y.as_f32();
    (local_x / viewport.zoom, local_y / viewport.zoom)
}

/// The top-level flow graph component.
///
/// Renders nodes as positioned divs and edges via a canvas paint layer.
pub struct FlowGraph {
    state: Entity<FlowState>,
    focus_handle: FocusHandle,
    node_renderers: HashMap<SharedString, NodeRendererFn>,
    /// Dedicated renderers for *container* nodes' header strips. When a type
    /// has one registered it wins over `node_renderers` for that container;
    /// without one, a container falls back to its ordinary `node_renderer`
    /// output (so pre-existing container renderers keep acting as headers).
    container_headers: HashMap<SharedString, NodeRendererFn>,
    default_renderer: Option<NodeRendererFn>,
    /// Called when a new connection is completed.
    on_connect: Option<Box<dyn Fn(&Connection, &mut FlowState)>>,
    /// Called when a connection drag ends over empty canvas (no snap
    /// target) instead of on another handle — the "drag from a handle,
    /// drop on empty space to create a new connected node" gesture. Given
    /// the in-progress draft (so the callback knows which node/handle/
    /// direction it started from) and the drop point already converted to
    /// flow-space coordinates (`Viewport::screen_to_flow`), with mutable
    /// access to `FlowState` to push the new node + edge directly.
    on_connection_drop: Option<std::rc::Rc<dyn Fn(&ConnectionDraft, FlowPoint, &mut FlowState)>>,
    /// Custom validation for connections. Return false to reject.
    is_valid_connection: Option<Box<dyn Fn(&Connection, &FlowState) -> bool>>,
    /// Whether to show the default node wrapper chrome (bg, border, shadow, padding).
    show_node_chrome: bool,
    /// Background color for the canvas (default: 0xf8f8f8).
    bg_color: u32,
    /// Dot grid color (default: 0xd4d4d4).
    grid_color: u32,
    /// Background pattern style.
    bg_pattern: BackgroundPattern,
    /// Node wrapper background color.
    node_bg_color: u32,
    /// Node wrapper border color.
    node_border_color: u32,
    /// Selection outline, connection-handle highlight, and snap-target
    /// color (default: 0x3b82f6, this crate's original hardcoded blue —
    /// unset consumers keep today's look).
    accent_color: u32,
    /// Ring color for nodes flagged `FlowNode::validation_error` (default:
    /// 0xef4444). Kept separate from `node_border_color`/`accent_color`
    /// because "this node is broken" has to stay legible on top of both the
    /// selection ring and live run status, and because hosts drive it from
    /// their theme's danger color.
    node_error_color: u32,
    /// Whether we've done the initial measurement pass.
    measured: bool,
    /// Fired on right-click for any node with `FlowNode.context_menu` set.
    /// `Rc` (not `Box`) so it can be cloned into the per-node mouse-down
    /// closure built fresh on every render. Deliberately generic (node id +
    /// window-absolute click point) — this crate has no `forge_ui`/UI-kit
    /// dependency and no opinion on what menu, if any, the consumer shows.
    on_node_context_menu: Option<std::rc::Rc<dyn Fn(NodeId, Point<Pixels>, &mut Window, &mut App)>>,
    /// Fired on right-click for any edge (hit-tested the same way the
    /// existing left-click edge-select path already does — see
    /// `edges::hit_test_edges`), since an edge has no element of its own to
    /// attach a per-edge handler to the way a node does (edges are painted,
    /// not laid out as divs).
    on_edge_context_menu: Option<std::rc::Rc<dyn Fn(EdgeId, Point<Pixels>, &mut Window, &mut App)>>,
    /// Fired when the "+" button on a hovered edge is clicked, with the
    /// edge and the click's window position. Setting it is what turns the
    /// button on: without a consumer to act on the click, hovering an edge
    /// shows nothing. This crate only reports the gesture; what gets
    /// inserted, and how the user picks it, is the consumer's business.
    on_edge_insert: Option<std::rc::Rc<dyn Fn(EdgeId, Point<Pixels>, &mut Window, &mut App)>>,
    /// Fired on right-click anywhere in the canvas. The callback receives
    /// both the window point and the corresponding canvas flow coordinate.
    on_canvas_context_menu: Option<
        std::rc::Rc<dyn Fn(Point<Pixels>, FlowPoint, Option<NodeId>, &mut Window, &mut App)>,
    >,
    /// This element's own on-screen origin within the window, captured each
    /// paint from the background/edges `canvas()`'s bounds. Mouse events
    /// deliver window-absolute positions, but `edges::hit_test_edges` (and
    /// everything else that positions things in "flow-canvas-local" space,
    /// like `Viewport::flow_to_screen`) compares against coordinates that
    /// don't include this element's own offset within a host window that
    /// embeds it below other chrome (a toolbar, tab bar, etc.) — subtracting
    /// this origin is what makes an edge click/right-click hit-test land on
    /// the same edge the user is actually looking at, instead of missing by
    /// exactly this element's vertical/horizontal offset. Node dragging and
    /// panning don't need this: they're delta-based (`later - earlier`), so
    /// a constant offset cancels out on its own — only *absolute* position
    /// comparisons like edge hit-testing are affected.
    canvas_origin: Rc<Cell<Point<Pixels>>>,
    /// This element's own on-screen box (window space), captured each paint
    /// from the background `canvas()`'s bounds. Unlike `canvas_origin` (the
    /// pan-offset *world* container's origin), this is the untransformed
    /// viewport rect, which is what a host needs to turn a window-absolute
    /// point into flow space — see [`FlowGraph::flow_point_at`] — and to
    /// locate the canvas' own center (the default drop position for
    /// "add an action here"). Left at its default until the first prepaint,
    /// so hosts should treat a zero-sized box as "not laid out yet".
    panel_bounds: Rc<Cell<Bounds<Pixels>>>,
}

impl FlowGraph {
    pub fn new(state: Entity<FlowState>, cx: &mut Context<Self>) -> Self {
        Self {
            state,
            focus_handle: cx.focus_handle(),
            node_renderers: HashMap::new(),
            container_headers: HashMap::new(),
            default_renderer: None,
            on_connect: None,
            on_connection_drop: None,
            is_valid_connection: None,
            show_node_chrome: true,
            bg_color: 0xf8f8f8,
            grid_color: 0xd4d4d4,
            bg_pattern: BackgroundPattern::Dots,
            node_bg_color: 0xffffff,
            node_border_color: 0xe2e2e2,
            accent_color: 0x3b82f6,
            node_error_color: 0xef4444,
            measured: false,

            on_node_context_menu: None,
            on_edge_context_menu: None,
            on_edge_insert: None,
            on_canvas_context_menu: None,
            canvas_origin: Rc::new(Cell::new(Point::default())),
            panel_bounds: Rc::new(Cell::new(Bounds::default())),
        }
    }

    /// This element's own on-screen box in window space, as of the last
    /// prepaint. Zero-sized until the first frame is laid out.
    ///
    /// Hosts need it to place things in the canvas without guessing: a
    /// "drop here" affordance that inserts at the cursor needs
    /// [`Self::flow_point_at`], and a "put it in the middle" fallback needs
    /// the box's center.
    pub fn panel_bounds(&self) -> Bounds<Pixels> {
        self.panel_bounds.get()
    }

    /// Convert a **window-absolute** point into flow space, the inverse of
    /// the layout transform the world container applies.
    ///
    /// This is deliberately not the private [`window_to_flow`] helper: that
    /// one takes the *world* origin, which already folds in the viewport pan,
    /// while a host only knows window-absolute coordinates (what mouse and
    /// drop events deliver). Subtracting this element's own origin first
    /// turns those into canvas-local screen coordinates, which is exactly
    /// what [`Viewport::screen_to_flow`] expects.
    ///
    /// `viewport` is the host's `FlowState::viewport` — it's passed in rather
    /// than read here so callers can hold the same snapshot they used for
    /// the rest of their hit-testing.
    pub fn flow_point_at(&self, pos: Point<Pixels>, viewport: crate::types::Viewport) -> FlowPoint {
        let origin = self.panel_bounds.get().origin;
        viewport.screen_to_flow((pos.x - origin.x).as_f32(), (pos.y - origin.y).as_f32())
    }

    /// Register a right-click handler for nodes with `context_menu` set.
    pub fn on_node_context_menu(
        mut self,
        callback: impl Fn(NodeId, Point<Pixels>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_node_context_menu = Some(std::rc::Rc::new(callback));
        self
    }

    /// Register a right-click handler for edges — fires when the click
    /// hit-tests onto an edge's path (same 5px threshold the left-click
    /// edge-select gesture uses).
    pub fn on_edge_context_menu(
        mut self,
        callback: impl Fn(EdgeId, Point<Pixels>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_edge_context_menu = Some(std::rc::Rc::new(callback));
        self
    }

    /// Show a "+" button on whichever edge the cursor is over, and call
    /// `callback` with that edge when it is clicked — the "insert a node
    /// here" gesture. See the field doc comment.
    pub fn on_edge_insert(
        mut self,
        callback: impl Fn(EdgeId, Point<Pixels>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_edge_insert = Some(std::rc::Rc::new(callback));
        self
    }

    /// Register a right-click handler for the canvas background and nodes.
    pub fn on_canvas_context_menu(
        mut self,
        callback: impl Fn(Point<Pixels>, FlowPoint, Option<NodeId>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_canvas_context_menu = Some(std::rc::Rc::new(callback));
        self
    }

    /// Register a renderer for a specific node type.
    pub fn node_renderer(
        mut self,
        node_type: impl Into<SharedString>,
        renderer: impl Fn(&FlowNode, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.node_renderers
            .insert(node_type.into(), Box::new(renderer));
        self
    }

    /// Set the default renderer for nodes without a specific type renderer.
    pub fn default_renderer(
        mut self,
        renderer: impl Fn(&FlowNode, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.default_renderer = Some(Box::new(renderer));
        self
    }

    /// Register a renderer for a *container* node's header strip. Takes
    /// precedence over `node_renderer` for container nodes; when absent, a
    /// container falls back to its `node_renderer` output acting as the
    /// header (preserving existing layouts). The rendered element is layered
    /// as a dedicated chrome strip across the container's top, sized to the
    /// container's `header_height` (`FlowNode::header_height`, default
    /// `store::CONTAINER_HEADER_HEIGHT`), and children lay out below it.
    pub fn container_header(
        mut self,
        node_type: impl Into<SharedString>,
        renderer: impl Fn(&FlowNode, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.container_headers
            .insert(node_type.into(), Box::new(renderer));
        self
    }

    /// Set the on_connect callback.
    pub fn on_connect(mut self, callback: impl Fn(&Connection, &mut FlowState) + 'static) -> Self {
        self.on_connect = Some(Box::new(callback));
        self
    }

    /// Set the on_connection_drop callback — see the field doc comment.
    pub fn on_connection_drop(
        mut self,
        callback: impl Fn(&ConnectionDraft, FlowPoint, &mut FlowState) + 'static,
    ) -> Self {
        self.on_connection_drop = Some(std::rc::Rc::new(callback));
        self
    }

    /// Hide the default node wrapper chrome (bg, border, shadow, padding).
    pub fn no_node_chrome(mut self) -> Self {
        self.show_node_chrome = false;
        self
    }

    /// Set the canvas background color.
    pub fn bg_color(mut self, color: u32) -> Self {
        self.bg_color = color;
        self
    }

    /// Set the dot grid color.
    pub fn grid_color(mut self, color: u32) -> Self {
        self.grid_color = color;
        self
    }

    /// Set the node wrapper background color.
    pub fn node_bg_color(mut self, color: u32) -> Self {
        self.node_bg_color = color;
        self
    }

    /// Set the node wrapper border color.
    pub fn node_border_color(mut self, color: u32) -> Self {
        self.node_border_color = color;
        self
    }

    /// Set the selection outline / connection-handle accent color.
    pub fn accent_color(mut self, color: u32) -> Self {
        self.accent_color = color;
        self
    }

    /// Live-resync every canvas color in one call, for consumers that
    /// observe the active theme and want a running canvas to track it (the
    /// builder setters above are one-shot: they take `mut self`, so they
    /// can't be re-applied to a living `Entity<FlowGraph>`).
    pub fn set_theme_colors(
        &mut self,
        bg_color: u32,
        grid_color: u32,
        node_bg_color: u32,
        node_border_color: u32,
        accent_color: u32,
    ) {
        self.bg_color = bg_color;
        self.grid_color = grid_color;
        self.node_bg_color = node_bg_color;
        self.node_border_color = node_border_color;
        self.accent_color = accent_color;
    }

    /// Live-resync the `FlowNode::validation_error` ring color — the builder
    /// setter takes `mut self` and so can't be re-applied to a living
    /// `Entity<FlowGraph>` the way [`Self::set_theme_colors`] can.
    pub fn set_node_error_color(&mut self, color: u32) {
        self.node_error_color = color;
    }

    /// Set the background pattern (Dots, Lines, Cross).
    pub fn bg_pattern(mut self, pattern: BackgroundPattern) -> Self {
        self.bg_pattern = pattern;
        self
    }

    /// Set custom connection validation.
    pub fn validate_connection(
        mut self,
        validator: impl Fn(&Connection, &FlowState) -> bool + 'static,
    ) -> Self {
        self.is_valid_connection = Some(Box::new(validator));
        self
    }

    /// Render a single node using the appropriate renderer.
    ///
    /// `abs_position` is the node's *resolved* flow-space position (walking
    /// any parent chain — see `FlowState::absolute_position`), already
    /// computed by the caller since that needs a `FlowState` borrow this
    /// method doesn't otherwise take. `footprint` is likewise
    /// `FlowState::node_footprint` — the container size for a node with
    /// children, or its measured/estimated leaf size.
    fn render_node(
        &self,
        node: &FlowNode,
        abs_position: FlowPoint,
        footprint: (f32, f32),
        is_container: bool,
        viewport: &Viewport,
        is_connecting: bool,
        snap_node_id: Option<&NodeId>,
        entity_id: EntityId,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        if node.hidden {
            return div().into_any_element();
        }

        // Node geometry is expressed in **flow-space units** and laid out
        // inside the world's `.scale(zoom)` container, so the framework
        // multiplies positions, sizes, text, and chrome by the zoom for us.
        // A child's offset from its container is already folded into
        // `abs_position` by `FlowState::absolute_position`, so every node is
        // a top-level absolutely-positioned child of the world using its
        // absolute flow coordinate — and it scales in lockstep with the box
        // that encloses it, at any zoom.
        let (screen_x, screen_y) = (abs_position.x, abs_position.y);
        let (footprint_w, footprint_h) = footprint;

        // Choose per-node content. A leaf gets its body from `node_renderer`.
        // A container's children live *below* a dedicated header strip whose
        // content comes from a `container_header` renderer when one is
        // registered for this type, otherwise falling back to the ordinary
        // `node_renderer` output (which used to render as a fixed 32px strip).
        // Splitting the two here (instead of computing one `content`) means a
        // container's body renderer isn't invoked at all when a dedicated
        // header exists — the header callbacks and the body callbacks stay
        // independent.
        let header_height = node
            .header_height
            .unwrap_or(crate::store::CONTAINER_HEADER_HEIGHT);

        let leaf_content: Option<AnyElement> = if is_container {
            None
        } else {
            Some(match node.node_type.as_ref() {
                Some(node_type) => match self.node_renderers.get(node_type) {
                    Some(renderer) => renderer(node, window, cx),
                    None => self.render_fallback_node(node, window, cx),
                },
                None => self.render_fallback_node(node, window, cx),
            })
        };

        let container_header_content: Option<AnyElement> = if is_container {
            Some(match node.node_type.as_ref() {
                Some(node_type) => match self
                    .container_headers
                    .get(node_type)
                    .or_else(|| self.node_renderers.get(node_type))
                {
                    Some(renderer) => renderer(node, window, cx),
                    None => self.render_fallback_node(node, window, cx),
                },
                None => self.render_fallback_node(node, window, cx),
            })
        } else {
            None
        };

        let node_id = node.id.clone();
        let state = self.state.clone();
        let canvas_origin = self.canvas_origin.clone();
        let selected = node.selected;
        let dragging = node.dragging;
        let show_chrome = self.show_node_chrome;
        let node_bg = self.node_bg_color;
        // Border precedence, most important first: a validation error outranks
        // both the selection ring and live run status, because it is the only
        // one that *blocks* the user (Run stays disabled while it stands).
        // Reading it here — once — keeps the two style passes below from
        // fighting over the same property in opposite orders.
        let has_validation_error = node.validation_error;
        let error_color = self.node_error_color;
        let node_border = if has_validation_error {
            error_color
        } else {
            node.accent_border.unwrap_or(self.node_border_color)
        };
        // A flagged node is always drawn `border_2`, whether or not it also has
        // an accent/run border underneath.
        let has_accent_border = node.accent_border.is_some() || has_validation_error;
        let element_id: ElementId = ElementId::Name(node.id.clone());

        // Dedicated header strip for containers: an absolutely-positioned
        // bar along the top, sized to `header_height` in flow units (which
        // the world's scale multiplies, keeping it matched to the `header_h`
        // offset applied by `FlowState::absolute_position` / `fit_container`).
        // Content is the `container_header` renderer output; chrome (tint +
        // bottom divider) only when `show_chrome` is on. Inset from the
        // container's rounded top corners so the bar's square corners stay
        // inside them.
        let container_header_strip: Option<AnyElement> = if is_container {
            container_header_content.map(|header_content| {
                div()
                    .absolute()
                    .top(px(1.0))
                    .left(px(8.0))
                    .right(px(8.0))
                    .h(px(header_height))
                    .flex()
                    .items_center()
                    .px_2()
                    .when(show_chrome, |el: Div| {
                        el.bg(gpui::rgba(0x00000008))
                            .border_b_1()
                            .border_color(gpui::rgb(node_border))
                    })
                    .child(header_content)
                    .into_any_element()
            })
        } else {
            None
        };

        // Build handle dot elements (skip if not connecting to reduce overhead)
        let handle_elements = if !node.handles.is_empty() {
            Self::render_handles(
                &node.handles,
                &node.id,
                &state,
                is_connecting,
                snap_node_id,
                node_bg,
                node_border,
                self.accent_color,
            )
        } else {
            Vec::new()
        };

        // Per-node measurement canvas: captures actual wrapper size. Whenever
        // a measurement actually changes (first paint of this node, or any
        // later change — e.g. a mindmap node added after the graph's own
        // one-shot initial-measure pass has already run), explicitly notify
        // the `FlowGraph` entity so edges get repainted against the fresh
        // size instead of staying pinned to a stale/default one indefinitely.
        let measure_state = self.state.clone();
        let measure_node_id = node.id.clone();
        let prev_w = node.measured_width;
        let prev_h = node.measured_height;
        let measure_canvas = canvas(
            |_bounds, _window, _cx| {},
            move |bounds, _: (), window, cx| {
                // This canvas lives inside the world's `.scale(zoom)`
                // container, so its layout bounds arrive already multiplied
                // by the zoom. Divide by the accumulated element scale to
                // record the node's size in *flow* units, which is what
                // `node_footprint`/`fit_container` work in — that keeps a
                // node's measured size independent of the current zoom, so
                // zooming neither refits containers nor retriggers this
                // callback.
                let s = window.element_scale();
                let w = px(bounds.size.width.as_f32() / s);
                let h = px(bounds.size.height.as_f32() / s);
                if prev_w != Some(w) || prev_h != Some(h) {
                    measure_state.update(cx, |state, _| {
                        if let Some(node) = state.get_node_mut(&measure_node_id) {
                            node.measured_width = Some(w);
                            node.measured_height = Some(h);
                        }
                        // A container's box is only as good as its children's
                        // real (measured) sizes — this node's just changed,
                        // so re-fit every container that encloses it. A
                        // resized container then re-measures itself and refits
                        // its own ancestors, so nested boxes settle in one
                        // pass up the chain. Uses the padding each container
                        // was originally fitted with (`container_padding`).
                        state.refit_ancestors(&measure_node_id);
                    });
                    cx.notify(entity_id);
                }
            },
        )
        .size_full()
        .absolute();

        // Assemble the wrapper's children: a leaf contributes its body, a
        // container contributes its header strip (its children render as
        // separate top-level nodes by the caller, above this box), followed
        // by the always-present measurement canvas and handle dots.
        let mut wrapper_children: Vec<AnyElement> = Vec::with_capacity(handle_elements.len() + 2);
        if let Some(strip) = container_header_strip {
            wrapper_children.push(strip);
        } else if let Some(body) = leaf_content {
            wrapper_children.push(body);
        }
        wrapper_children.push(measure_canvas.into_any_element());
        wrapper_children.extend(handle_elements);

        div()
            .id(element_id)
            .debug_selector({
                let node_id = node_id.clone();
                move || node_id.to_string()
            })
            .absolute()
            .left(px(screen_x))
            .top(px(screen_y))
            // A container (has children — e.g. a loop body) is forced to
            // its resolved footprint so there's room to lay children out
            // inside it, and gets a dashed border instead of the ordinary
            // solid one so it reads as "holds other nodes" rather than "is
            // one." A leaf node keeps intrinsic content sizing (no
            // width/height set here at all). The container's header strip is
            // layered as an absolutely-positioned child (see below), and
            // children sit below `header_height` in flow space — so no
            // padding-top is needed to reserve header space here.
            .when(is_container, |el| el.w(px(footprint_w)).h(px(footprint_h)))
            // Node box styling on the wrapper so handles align to visual edges
            .when(show_chrome, |el: Stateful<Div>| {
                // A container is only tinted, never filled. Edges, and the
                // line drawn while dragging a new connection, are painted on
                // one layer underneath every node, so an opaque container box
                // would hide every edge between the nodes inside it. The same
                // goes for a shadow, which is drawn across the whole box.
                el.when(!is_container, |el| el.bg(gpui::rgb(node_bg)).shadow_sm())
                    .when(is_container, |el| {
                        el.bg(gpui::rgba((node_bg << 8) | 0x33)).border_dashed()
                    })
                    .when(has_accent_border, |el| el.border_2())
                    .when(!has_accent_border, |el| el.border_1())
                    .border_color(gpui::rgb(node_border))
                    .rounded_lg()
                    .px_4()
                    .py_2()
            })
            .cursor(if dragging {
                CursorStyle::ClosedHand
            } else {
                CursorStyle::OpenHand
            })
            .when(selected && !has_validation_error, |el: Stateful<Div>| {
                el.border_2().border_color(gpui::rgb(self.accent_color))
            })
            .on_mouse_down(MouseButton::Left, {
                let node_id = node_id.clone();
                let state = state.clone();
                let canvas_origin = canvas_origin.clone();
                let viewport = *viewport;
                move |event, _window, cx| {
                    let multi = event.modifiers.platform;
                    let mouse_pos = event.position;
                    let mouse = window_to_flow(&viewport, canvas_origin.get(), mouse_pos);

                    state.update(cx, |state, _| {
                        // Don't start node drag if we're connecting
                        if state.connecting.is_some() {
                            return;
                        }

                        // If this click landed on a node nested inside a
                        // container (a loop/if/try body — which this handler
                        // fires for even when the child's own handler also
                        // does, because GPUI dispatches mouse-downs to every
                        // Normal-behavior hitbox under the cursor), rebind the
                        // effective target to that descendant so the
                        // container can't overwrite the child's selection.
                        let mut effective_id = node_id.clone();
                        if state.is_container(&node_id) {
                            if let Some(descendant) = state.descendant_at_flow(&node_id, mouse) {
                                effective_id = descendant;
                            }
                        }

                        // Handle selection
                        if !multi {
                            for n in &mut state.nodes {
                                n.selected = false;
                            }
                        }
                        // Z-index elevation
                        let max_z = state.nodes.iter().map(|n| n.z_index).max().unwrap_or(0);
                        if let Some(n) = state.get_node_mut(&effective_id) {
                            n.selected = !n.selected || !multi;
                            if n.selected {
                                n.z_index = max_z + 1;
                            }
                        }

                        // Start drag — collect all selected nodes
                        let mut node_origins = Vec::new();
                        if let Some(n) = state.get_node(&effective_id) {
                            if n.selected && n.draggable {
                                for n in &state.nodes {
                                    if n.selected && n.draggable {
                                        node_origins.push((n.id.clone(), n.position));
                                    }
                                }
                            }
                        }

                        if !node_origins.is_empty() {
                            state.push_undo();
                            state.drag_state = Some(DragState {
                                origin_mouse: (mouse_pos.x.as_f32(), mouse_pos.y.as_f32()),
                                node_origins,
                            });
                            for n in &mut state.nodes {
                                if n.selected && n.draggable {
                                    n.dragging = true;
                                }
                            }
                        }
                    });
                }
            })
            .when_some(
                node.context_menu
                    .then(|| self.on_node_context_menu.clone())
                    .flatten(),
                |el, callback| {
                    let node_id = node_id.clone();
                    let state = state.clone();
                    let canvas_origin = canvas_origin.clone();
                    let viewport = *viewport;
                    el.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                        let target = state.update(cx, |state, _| {
                            if state.is_container(&node_id) {
                                state
                                    .descendant_at_flow(
                                        &node_id,
                                        window_to_flow(
                                            &viewport,
                                            canvas_origin.get(),
                                            event.position,
                                        ),
                                    )
                                    .unwrap_or_else(|| node_id.clone())
                            } else {
                                node_id.clone()
                            }
                        });
                        callback(target, event.position, window, cx);
                    })
                },
            )
            .children(wrapper_children)
            .into_any_element()
    }

    /// Render handle dots for a node.
    fn render_handles(
        handles: &[HandleDef],
        node_id: &NodeId,
        state: &Entity<FlowState>,
        is_connecting: bool,
        snap_node_id: Option<&NodeId>,
        default_bg: u32,
        default_border: u32,
        accent: u32,
    ) -> Vec<AnyElement> {
        let handle_size = 10.0;
        let half = handle_size / 2.0;
        let is_snapped_node = snap_node_id == Some(node_id);

        handles
            .iter()
            .enumerate()
            .map(|(i, handle)| {
                let node_id = node_id.clone();
                let handle_id = handle.id.clone();
                let handle_type = handle.handle_type;
                let handle_position = handle.position;
                let state = state.clone();

                // Highlight: strongly if this is the snapped target, mildly if potential target
                let is_snap_target = is_snapped_node && handle_type == HandleType::Target;
                let is_potential_target = is_connecting && handle_type == HandleType::Target;

                let (bg_color, border_color, size_mult) = if is_snap_target {
                    // Actively snapped — full-strength accent pulse.
                    (gpui::rgb(accent), gpui::rgb(accent), 1.4)
                } else if is_potential_target {
                    // Valid potential target — the same accent, softened.
                    (gpui::rgba((accent << 8) | 0x80), gpui::rgb(accent), 1.0)
                } else {
                    (gpui::rgb(default_bg), gpui::rgb(default_border), 1.0)
                };
                let dot_size = handle_size * size_mult;

                let dot = div()
                    .w(px(dot_size))
                    .h(px(dot_size))
                    .rounded_full()
                    .bg(bg_color)
                    .border_1()
                    .border_color(border_color)
                    .cursor(CursorStyle::Crosshair)
                    .flex_shrink_0();

                let container = div()
                    .id(ElementId::Integer(i as u64))
                    .absolute()
                    .flex()
                    .items_center()
                    .justify_center()
                    // Handle mouse down → start connection
                    .on_mouse_down(MouseButton::Left, {
                        let state = state.clone();
                        let node_id = node_id.clone();
                        let handle_id = handle_id.clone();
                        move |event, _window, cx| {
                            let mouse_pos = event.position;
                            state.update(cx, |state, _| {
                                // Find handle center for the from_point
                                let from_point = state
                                    .find_handle_center(&node_id, &handle_id, handle_position)
                                    .unwrap_or((mouse_pos.x.as_f32(), mouse_pos.y.as_f32()));

                                state.connecting = Some(ConnectionDraft {
                                    from_node: node_id.clone(),
                                    from_handle: handle_id.clone(),
                                    from_type: handle_type,
                                    from_position: handle_position,
                                    from_point,
                                    to_point: (mouse_pos.x.as_f32(), mouse_pos.y.as_f32()),
                                    snap_target: None,
                                });
                                // Prevent node drag
                                state.drag_state = None;
                            });
                        }
                    })
                    // Handle mouse up → complete connection if valid
                    .on_mouse_up(MouseButton::Left, {
                        move |_event, _window, cx| {
                            state.update(cx, |state, _| {
                                if let Some(draft) = state.connecting.take() {
                                    // Build the connection
                                    let ConnectionDraft {
                                        from_node,
                                        from_handle,
                                        from_type,
                                        ..
                                    } = draft;
                                    let (source, target, source_handle, target_handle) =
                                        if from_type == HandleType::Source {
                                            (
                                                from_node,
                                                node_id.clone(),
                                                from_handle,
                                                handle_id.clone(),
                                            )
                                        } else {
                                            (
                                                node_id.clone(),
                                                from_node,
                                                handle_id.clone(),
                                                from_handle,
                                            )
                                        };

                                    let connection = Connection {
                                        source,
                                        target,
                                        source_handle,
                                        target_handle,
                                    };

                                    if state.is_valid_connection(&connection) {
                                        state.push_undo();
                                        let edge_id: SharedString =
                                            format!("e{}-{}", connection.source, connection.target)
                                                .into();
                                        state.add_edge_from_connection(&connection, edge_id);
                                    }
                                }
                            });
                        }
                    });

                let container = match handle.position {
                    HandlePosition::Left => container
                        .left(px(-half))
                        .top_0()
                        .bottom_0()
                        .w(px(handle_size)),
                    HandlePosition::Right => container
                        .right(px(-half))
                        .top_0()
                        .bottom_0()
                        .w(px(handle_size)),
                    HandlePosition::Top => container
                        .top(px(-half))
                        .left_0()
                        .right_0()
                        .h(px(handle_size)),
                    HandlePosition::Bottom => container
                        .bottom(px(-half))
                        .left_0()
                        .right_0()
                        .h(px(handle_size)),
                };

                container.child(dot).into_any_element()
            })
            .collect()
    }

    /// Render a node with no per-type renderer registered: the caller's
    /// `default_renderer` when one was set, otherwise the built-in
    /// type-less label. (`default_renderer` used to be stored and never
    /// consulted here, silently routing every unmapped node — e.g. a
    /// leaf whose type isn't in `node_renderers` — to the hardcoded
    /// `render_default_node` palette.)
    fn render_fallback_node(
        &self,
        node: &FlowNode,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        match &self.default_renderer {
            Some(renderer) => renderer(node, window, cx),
            None => self.render_default_node(node, window, cx),
        }
    }

    /// Default node rendering — just the label content (styling is on the wrapper).
    fn render_default_node(
        &self,
        node: &FlowNode,
        _window: &mut Window,
        _cx: &mut App,
    ) -> AnyElement {
        div()
            .min_w(px(80.0))
            .text_sm()
            .text_color(gpui::rgb(0x1a1a1a))
            .child(if node.label.is_empty() {
                node.id.to_string()
            } else {
                node.label.to_string()
            })
            .into_any_element()
    }

    /// Paint the background pattern across the graph viewport.
    ///
    /// The grid is anchored to the **world** origin rather than the viewport,
    /// so its phase follows the content as you pan.
    ///
    /// This canvas is outside the scaled world, so apply both the window
    /// element scale and graph zoom to pattern spacing and stroke sizes.
    /// `window.paint_quad` does not apply `Style::scale` itself. Dot size
    /// keeps the original `min(1.0)` clamp so zooming in doesn't turn the
    /// background into heavy blobs.
    fn paint_grid(
        bounds: &Bounds<Pixels>,
        grid_color: u32,
        pattern: BackgroundPattern,
        viewport: Viewport,
        window: &mut Window,
    ) {
        let color = gpui::rgb(grid_color);
        let element_scale = window.element_scale();
        let scale = element_scale * viewport.zoom;
        let spacing = 20.0 * scale;

        if spacing < 5.0 {
            return;
        }

        let bw = bounds.size.width.as_f32();
        let bh = bounds.size.height.as_f32();
        let ox = bounds.origin.x.as_f32();
        let oy = bounds.origin.y.as_f32();
        let start_x = (viewport.x * element_scale).rem_euclid(spacing);
        let start_y = (viewport.y * element_scale).rem_euclid(spacing);

        match pattern {
            BackgroundPattern::Dots => {
                let dot_size = px(1.5 * scale.min(1.0));
                let mut x = start_x;
                while x < bw {
                    let mut y = start_y;
                    while y < bh {
                        let dot_bounds = Bounds::new(
                            Point::new(px(ox + x) - dot_size / 2.0, px(oy + y) - dot_size / 2.0),
                            Size {
                                width: dot_size,
                                height: dot_size,
                            },
                        );
                        window.paint_quad(gpui::fill(dot_bounds, color));
                        y += spacing;
                    }
                    x += spacing;
                }
            }
            BackgroundPattern::Lines => {
                let line_w = px(0.5 * scale);
                let mut x = start_x;
                while x < bw {
                    let line = Bounds::new(
                        Point::new(px(ox + x), px(oy)),
                        Size {
                            width: line_w,
                            height: px(bh),
                        },
                    );
                    window.paint_quad(gpui::fill(line, color));
                    x += spacing;
                }
            }
            BackgroundPattern::Cross => {
                let line_w = px(0.5 * scale);
                // Vertical lines
                let mut x = start_x;
                while x < bw {
                    let line = Bounds::new(
                        Point::new(px(ox + x), px(oy)),
                        Size {
                            width: line_w,
                            height: px(bh),
                        },
                    );
                    window.paint_quad(gpui::fill(line, color));
                    x += spacing;
                }
                // Horizontal lines
                let mut y = start_y;
                while y < bh {
                    let line = Bounds::new(
                        Point::new(px(ox), px(oy + y)),
                        Size {
                            width: px(bw),
                            height: line_w,
                        },
                    );
                    window.paint_quad(gpui::fill(line, color));
                    y += spacing;
                }
            }
        }
    }

    /// Paint a selection box rectangle.
    ///
    /// `sel.start`/`sel.current` are **flow-space** points (the same space
    /// node bounds live in), so the whole rect is transformed to
    /// window-absolute with `origin + flow * element_scale` — border
    /// thickness included, so the marquee keeps its hairline weight as you
    /// zoom.
    fn paint_selection_box(
        sel: &SelectionBox,
        origin: (f32, f32),
        scale: f32,
        accent: u32,
        window: &mut Window,
    ) {
        let x = sel.start.0.min(sel.current.0);
        let y = sel.start.1.min(sel.current.1);
        let w = (sel.start.0 - sel.current.0).abs();
        let h = (sel.start.1 - sel.current.1).abs();

        if w < 0.5 || h < 0.5 {
            return;
        }

        // flow -> window-absolute
        let at = |fx: f32, fy: f32| (origin.0 + fx * scale, origin.1 + fy * scale);
        let (x, y) = at(x, y);
        let (w, h) = (w * scale, h * scale);
        let border = 1.0 * scale;

        let bounds = Bounds::new(
            Point::new(px(x), px(y)),
            Size {
                width: px(w),
                height: px(h),
            },
        );

        // Semi-transparent accent fill
        window.paint_quad(fill(bounds, gpui::rgba((accent << 8) | 0x18)));

        // Accent border
        let border_color: Background = gpui::rgb(accent).into();
        // Top
        let top = Bounds::new(
            Point::new(px(x), px(y)),
            Size {
                width: px(w),
                height: px(border),
            },
        );
        window.paint_quad(fill(top, border_color));
        // Bottom
        let bottom = Bounds::new(
            Point::new(px(x), px(y + h - border)),
            Size {
                width: px(w),
                height: px(border),
            },
        );
        window.paint_quad(fill(bottom, border_color));
        // Left
        let left = Bounds::new(
            Point::new(px(x), px(y)),
            Size {
                width: px(border),
                height: px(h),
            },
        );
        window.paint_quad(fill(left, border_color));
        // Right
        let right = Bounds::new(
            Point::new(px(x + w - border), px(y)),
            Size {
                width: px(border),
                height: px(h),
            },
        );
        window.paint_quad(fill(right, border_color));
    }

    /// Paint a draft connection line from handle to mouse cursor.
    ///
    /// Both `draft.from_point` (captured via `find_handle_center`) and
    /// `draft.to_point` (the live cursor) are **flow-space**, so the whole
    /// curve — stroke width included — is transformed to window-absolute with
    /// `origin + flow * element_scale`.
    fn paint_connection_draft(
        draft: &ConnectionDraft,
        origin: (f32, f32),
        scale: f32,
        accent: u32,
        window: &mut Window,
    ) {
        let color: Background = gpui::rgba((accent << 8) | 0x80).into();
        let at =
            |fx: f32, fy: f32| Point::new(px(origin.0 + fx * scale), px(origin.1 + fy * scale));
        let (sx, sy) = draft.from_point;
        let (tx, ty) = draft.to_point;

        let mut builder = PathBuilder::stroke(px(2.0 * scale));
        builder.move_to(at(sx, sy));

        // Simple bezier toward cursor
        let dx = (tx - sx).abs() * 0.5;
        let (cx1, cy1, cx2, cy2) = match draft.from_position {
            HandlePosition::Right => (sx + dx, sy, tx - dx, ty),
            HandlePosition::Left => (sx - dx, sy, tx + dx, ty),
            HandlePosition::Bottom => (sx, sy + dx, tx, ty - dx),
            HandlePosition::Top => (sx, sy - dx, tx, ty + dx),
        };

        builder.cubic_bezier_to(at(tx, ty), at(cx1, cy1), at(cx2, cy2));

        if let Ok(path) = builder.build() {
            window.paint_path(path, color);
        }
    }

    /// Paints a small "+" badge at the dangling end of a connection draft
    /// that isn't currently snapped to a handle — the visual affordance for
    /// "release here to create a new connected node" (see
    /// `on_connection_drop`). `pos` is **flow-space**, same space
    /// `draft.to_point` is in, and is transformed like the rest of the
    /// canvas layer.
    fn paint_drop_to_create_hint(
        pos: (f32, f32),
        origin: (f32, f32),
        scale: f32,
        accent: u32,
        window: &mut Window,
    ) {
        let size = 22.0 * scale;
        let half = size / 2.0;
        let cx = origin.0 + pos.0 * scale;
        let cy = origin.1 + pos.1 * scale;
        let bg: Background = gpui::rgba((accent << 8) | 0xe6).into();
        let bounds = Bounds::new(
            Point::new(px(cx - half), px(cy - half)),
            Size {
                width: px(size),
                height: px(size),
            },
        );
        window.paint_quad(fill(bounds, bg).corner_radii(px(half)));

        let line_color: Background = gpui::white().into();
        let pad = 6.0 * scale;
        let stroke = 2.0 * scale;
        let mut h = PathBuilder::stroke(px(stroke));
        h.move_to(Point::new(px(cx - half + pad), px(cy)));
        h.line_to(Point::new(px(cx + half - pad), px(cy)));
        if let Ok(path) = h.build() {
            window.paint_path(path, line_color);
        }
        let mut v = PathBuilder::stroke(px(stroke));
        v.move_to(Point::new(px(cx), px(cy - half + pad)));
        v.line_to(Point::new(px(cx), px(cy + half - pad)));
        if let Ok(path) = v.build() {
            window.paint_path(path, line_color);
        }
    }
}

impl Render for FlowGraph {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // On second render, edges will have correct node measurements
        // from the first render's measurement canvases.
        if !self.measured {
            self.measured = true;
            cx.notify(); // triggers one immediate re-render
        }

        let entity_id = cx.entity_id();
        let window_size = window.viewport_size();
        let win_w = window_size.width.as_f32();
        let win_h = window_size.height.as_f32();
        let cull_margin = 200.0;
        // Only worth painting the "drop here to create a node" hint if a
        // host actually registered `on_connection_drop` — otherwise a drop
        // on empty canvas just cancels, same as before this existed.
        let show_drop_to_create_hint = self.on_connection_drop.is_some();

        // Read state, extract what we need, then release the borrow
        let (
            viewport,
            is_panning,
            is_connecting,
            snap_node_id,
            connecting_draft,
            selection_box,
            visible_nodes,
            edge_label_elements,
        ) = {
            let state = self.state.read(cx);
            let viewport = state.viewport;
            let is_panning = state.pan_drag.is_some();
            let is_connecting = state.connecting.is_some();
            let snap_node_id = state
                .connecting
                .as_ref()
                .and_then(|d| d.snap_target.as_ref())
                .map(|t| t.node_id.clone());
            let connecting_draft = state.connecting.clone();
            let selection_box = state.selection_box;

            // Build visible node indices. Culling and sizing both use the
            // *resolved* absolute position/footprint (parent-chain-aware —
            // see `FlowState::absolute_position`/`node_footprint`), not the
            // raw (possibly parent-relative) `node.position` directly.
            //
            // Culling happens in **flow space**: the world container is
            // placed at `(viewport.x, viewport.y)` and covers `win_w/zoom ×
            // win_h/zoom` flow units, so the window rect `[0, win_w]` maps
            // back to the flow rect below. The margin is a screen-space
            // cushion, so it's divided by the zoom too.
            let cull_margin = cull_margin / viewport.zoom;
            let vis_x0 = -viewport.x / viewport.zoom - cull_margin;
            let vis_y0 = -viewport.y / viewport.zoom - cull_margin;
            let vis_x1 = (win_w - viewport.x) / viewport.zoom + cull_margin;
            let vis_y1 = (win_h - viewport.y) / viewport.zoom + cull_margin;
            let mut visible_indices: Vec<usize> = Vec::new();
            for (i, node) in state.nodes.iter().enumerate() {
                if node.hidden {
                    continue;
                }
                let abs_pos = state.absolute_position(&node.id).unwrap_or(node.position);
                let (nw, nh) = state.node_footprint(node);
                if abs_pos.x + nw < vis_x0
                    || abs_pos.x > vis_x1
                    || abs_pos.y + nh < vis_y0
                    || abs_pos.y > vis_y1
                {
                    continue;
                }
                visible_indices.push(i);
            }
            // Depth first, so a container always paints (and is listed as a
            // later sibling than) behind every one of its own descendants,
            // regardless of individual `z_index`; z_index only breaks ties
            // within the same depth, same as before nesting existed.
            visible_indices
                .sort_by_key(|&i| (state.depth_of(&state.nodes[i].id), state.nodes[i].z_index));

            let visible_nodes: Vec<(FlowNode, FlowPoint, (f32, f32), bool)> = visible_indices
                .iter()
                .map(|&i| {
                    let node = &state.nodes[i];
                    let abs_pos = state.absolute_position(&node.id).unwrap_or(node.position);
                    let footprint = state.node_footprint(node);
                    let is_container = state.is_container(&node.id);
                    (node.clone(), abs_pos, footprint, is_container)
                })
                .collect();

            // Edge labels
            let bg = self.bg_color;
            let mut edge_label_elements: Vec<AnyElement> = Vec::new();
            for edge in &state.edges {
                if let Some(ref label) = edge.label {
                    if let Some((lx, ly)) = edges::compute_edge_label_position(state, edge) {
                        let label_color = edge.color.unwrap_or(0xb1b1b7);
                        edge_label_elements.push(
                            div()
                                .absolute()
                                .left(px(lx))
                                .top(px(ly))
                                .px_2()
                                .py_0p5()
                                .bg(gpui::rgb(bg))
                                .rounded_sm()
                                .text_xs()
                                .text_color(gpui::rgb(label_color))
                                .child(label.to_string())
                                .into_any_element(),
                        );
                    }
                }
            }

            (
                viewport,
                is_panning,
                is_connecting,
                snap_node_id,
                connecting_draft,
                selection_box,
                visible_nodes,
                edge_label_elements,
            )
        }; // state borrow ends here

        // Render only visible nodes
        let mut node_elements: Vec<AnyElement> = Vec::with_capacity(visible_nodes.len());
        for (node, abs_pos, footprint, is_container) in &visible_nodes {
            node_elements.push(self.render_node(
                node,
                *abs_pos,
                *footprint,
                *is_container,
                &viewport,
                is_connecting,
                snap_node_id.as_ref(),
                entity_id,
                window,
                cx,
            ));
        }

        // The insert button for the hovered edge, laid out in flow units like
        // the edge labels it sits among. Pressing it must not reach the
        // canvas underneath, which would start a pan or select the edge.
        let insert_button: Option<AnyElement> = self.on_edge_insert.clone().and_then(|callback| {
            let state = self.state.read(cx);
            let edge_id = state.hovered_edge.clone()?;
            let edge = state.edges.iter().find(|edge| edge.id == edge_id)?;
            let (x, y) = edges::compute_edge_label_position(state, edge)?;
            let half = EDGE_INSERT_BUTTON_SIZE / 2.0;
            Some(
                div()
                    .id("flow-edge-insert")
                    .absolute()
                    .left(px(x - half))
                    .top(px(y - half))
                    .w(px(EDGE_INSERT_BUTTON_SIZE))
                    .h(px(EDGE_INSERT_BUTTON_SIZE))
                    .rounded_full()
                    .bg(gpui::rgb(self.accent_color))
                    .border_1()
                    .border_color(gpui::rgb(self.node_bg_color))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.0))
                    .line_height(px(EDGE_INSERT_BUTTON_SIZE - 2.0))
                    .text_color(gpui::rgb(self.node_bg_color))
                    .cursor(CursorStyle::PointingHand)
                    .child("+")
                    .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                        cx.stop_propagation();
                        callback(edge_id.clone(), event.position, window, cx);
                    })
                    .into_any_element(),
            )
        });

        let state_for_canvas = self.state.clone();
        let viewport_for_world = viewport;
        let bg_color = self.bg_color;
        let grid_color = self.grid_color;
        let bg_pattern = self.bg_pattern;
        let accent_color = self.accent_color;
        let state_for_scroll = self.state.clone();
        let state_for_mouse_down = self.state.clone();
        let state_for_mouse_move = self.state.clone();
        let state_for_mouse_up = self.state.clone();
        let state_for_canvas_context = self.state.clone();
        let on_connection_drop = self.on_connection_drop.clone();
        let state_for_key = self.state.clone();
        let state_for_pinch = self.state.clone();
        let on_canvas_context_menu = self.on_canvas_context_menu.clone();

        div()
            .id("flow-graph")
            .debug_selector(|| "flow-graph".to_string())
            .track_focus(&self.focus_handle)
            .size_full()
            .overflow_hidden()
            .relative()
            .bg(gpui::rgb(bg_color))
            .cursor(if is_panning {
                CursorStyle::ClosedHand
            } else if is_connecting {
                CursorStyle::Crosshair
            } else {
                CursorStyle::Arrow
            })
            // The world: laid out once in flow-space units inside a single
            // `.scale(zoom)` container. Node positions and sizes, text, node
            // chrome, handles, edge labels, and edge geometry are expressed
            // in flow units and zoomed by the framework. The background
            // pattern is painted separately across the fixed graph viewport
            // so panning the world cannot uncover part of the panel.
            //
            // The world is sized `viewport_size / zoom`; after scaling it
            // matches the viewport size and is clipped to the graph panel.
            // `left`/`top` carry the pan.
            .child(
                div()
                    .absolute()
                    .size_full()
                    .debug_selector(|| "flow-background".to_string())
                    .child({
                        let viewport_for_background = viewport_for_world;
                        let panel_bounds_for_layout = self.panel_bounds.clone();
                        canvas(
                            move |bounds, _window, _cx| {
                                // The background fills this element exactly
                                // (`.absolute().size_full()`), so its box is
                                // the graph panel's own on-screen rect — the
                                // untransformed viewport, with no pan or zoom
                                // folded in. Captured here (rather than
                                // reusing the world's `canvas_origin`, which
                                // is the panned world container) because
                                // `Self::flow_point_at` needs exactly this.
                                panel_bounds_for_layout.set(bounds);
                            },
                            move |bounds, _: (), window, _cx| {
                                Self::paint_grid(
                                    &bounds,
                                    grid_color,
                                    bg_pattern,
                                    viewport_for_background,
                                    window,
                                );
                            },
                        )
                        .size_full()
                    }),
            )
            .child(
                div()
                    .absolute()
                    .left(px(viewport_for_world.x))
                    .top(px(viewport_for_world.y))
                    .scale(viewport_for_world.zoom)
                    // Test-support only (a no-op in release builds): lets the
                    // zoom/resize tests read back the world's own box, which is
                    // how they assert its scaled geometry.
                    .debug_selector(|| "flow-world".to_string())
                    .w(px(win_w / viewport_for_world.zoom))
                    .h(px(win_h / viewport_for_world.zoom))
                    // Edge painting layer
                    .child({
                        let canvas_origin_for_layout = self.canvas_origin.clone();
                        canvas(
                            move |bounds, _window, _cx| {
                                canvas_origin_for_layout.set(bounds.origin);
                            },
                            move |bounds, _: (), window, cx| {
                                let origin = (bounds.origin.x.as_f32(), bounds.origin.y.as_f32());
                                let scale = window.element_scale();
                                let state = state_for_canvas.read(cx);
                                edges::paint_edges(state, origin, window);

                                // Paint draft connection line
                                if let Some(ref draft) = connecting_draft {
                                    Self::paint_connection_draft(
                                        draft,
                                        origin,
                                        scale,
                                        accent_color,
                                        window,
                                    );
                                    if show_drop_to_create_hint && draft.snap_target.is_none() {
                                        Self::paint_drop_to_create_hint(
                                            draft.to_point,
                                            origin,
                                            scale,
                                            accent_color,
                                            window,
                                        );
                                    }
                                }

                                // Paint selection box
                                if let Some(ref sel) = selection_box {
                                    Self::paint_selection_box(
                                        sel,
                                        origin,
                                        scale,
                                        accent_color,
                                        window,
                                    );
                                }
                            },
                        )
                        .absolute()
                        .size_full()
                    })
                    // Node layer
                    .children(node_elements)
                    // Edge labels
                    .children(edge_label_elements)
                    // The "+" on the hovered edge, above everything else in
                    // the world so a node near the midpoint can't cover it.
                    .children(insert_button),
            )
            // Mouse down on empty space → start panning, deselect, or edge selection
            .on_mouse_down(MouseButton::Left, {
                let canvas_origin_for_left_click = self.canvas_origin.clone();
                move |event, _window, cx| {
                    let mouse_pos = event.position;
                    state_for_mouse_down.update(cx, |state, _| {
                        // If a node drag or connection was already started, skip
                        if state.drag_state.is_some() || state.connecting.is_some() {
                            return;
                        }

                        // Try edge hit testing. `hit_test_edges` works in
                        // **flow-space**, so convert the click through
                        // `window_to_flow`. `mx`/`my` below stay
                        // window-absolute for the pan-drag code further down,
                        // which is delta-based and doesn't need this.
                        let origin = canvas_origin_for_left_click.get();
                        let flow_mouse = window_to_flow(&state.viewport, origin, mouse_pos);
                        let mx = mouse_pos.x.as_f32();
                        let my = mouse_pos.y.as_f32();
                        if let Some(edge_id) = edges::hit_test_edges(state, flow_mouse, 5.0) {
                            if !event.modifiers.platform {
                                for n in &mut state.nodes {
                                    n.selected = false;
                                }
                                for e in &mut state.edges {
                                    e.selected = false;
                                }
                            }
                            if let Some(edge) = state.edges.iter_mut().find(|e| e.id == edge_id) {
                                edge.selected = true;
                            }
                            return;
                        }

                        // Shift+drag → start box selection
                        if event.modifiers.shift {
                            if !event.modifiers.platform {
                                for n in &mut state.nodes {
                                    n.selected = false;
                                }
                                for e in &mut state.edges {
                                    e.selected = false;
                                }
                            }
                            // The marquee is tracked in flow-space, matching
                            // the node bounds it's tested against and the
                            // space `paint_selection_box` expects.
                            state.selection_box = Some(SelectionBox {
                                start: flow_mouse,
                                current: flow_mouse,
                            });
                            return;
                        }

                        // Deselect all nodes and edges
                        if !event.modifiers.platform {
                            for n in &mut state.nodes {
                                n.selected = false;
                            }
                            for e in &mut state.edges {
                                e.selected = false;
                            }
                        }

                        // Start pan drag
                        state.pan_drag = Some(PanDragState {
                            start_mouse: (mx, my),
                            start_viewport: (state.viewport.x, state.viewport.y),
                        });
                    });
                    cx.notify(entity_id);
                }
            })
            .when_some(on_canvas_context_menu, |el, callback| {
                let canvas_origin = self.canvas_origin.clone();
                let viewport = viewport_for_world;
                el.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                    let flow = window_to_flow(&viewport, canvas_origin.get(), event.position);
                    let node_id = state_for_canvas_context.read(cx).node_at_flow(flow);
                    callback(
                        event.position,
                        FlowPoint::new(flow.0, flow.1),
                        node_id,
                        window,
                        cx,
                    );
                })
            })
            // Right-click on an edge → `on_edge_context_menu`, same
            // `hit_test_edges` geometry the left-click select path above
            // uses. No-op (falls through to the OS/default context menu
            // behavior) when the click doesn't land on an edge, or no
            // callback is registered.
            .when_some(self.on_edge_context_menu.clone(), |el, callback| {
                let state_for_edge_ctx = self.state.clone();
                let canvas_origin_for_edge_ctx = self.canvas_origin.clone();
                el.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                    let flow = window_to_flow(
                        &state_for_edge_ctx.read(cx).viewport,
                        canvas_origin_for_edge_ctx.get(),
                        event.position,
                    );
                    let hit = edges::hit_test_edges(state_for_edge_ctx.read(cx), flow, 5.0);
                    if let Some(edge_id) = hit {
                        callback(edge_id, event.position, window, cx);
                    }
                })
            })
            // Global mouse move → handle dragging, panning, or connecting
            .on_mouse_move({
                let canvas_origin_for_mouse_move = self.canvas_origin.clone();
                let hover_edges = self.on_edge_insert.is_some();
                move |event, _window, cx| {
                    let mouse_pos = event.position;
                    let mx = mouse_pos.x.as_f32();
                    let my = mouse_pos.y.as_f32();

                    let mut changed = false;

                    state_for_mouse_move.update(cx, |state, _| {
                        // Flow-space cursor, for everything that compares
                        // against node/handle geometry. `mx`/`my` remain
                        // window-absolute for the delta-based pan and node
                        // drag paths below.
                        let flow_mouse = window_to_flow(
                            &state.viewport,
                            canvas_origin_for_mouse_move.get(),
                            mouse_pos,
                        );
                        // Box selection
                        if let Some(ref mut sel) = state.selection_box {
                            sel.current = flow_mouse;
                            // Select nodes whose flow bounds intersect the box
                            let (sx, sy, ex, ey) = (
                                sel.start.0.min(sel.current.0),
                                sel.start.1.min(sel.current.1),
                                sel.start.0.max(sel.current.0),
                                sel.start.1.max(sel.current.1),
                            );
                            // Resolved (absolute-position, footprint) per
                            // node computed first — `absolute_position`/
                            // `node_footprint` need an immutable `&state`,
                            // which can't overlap the `&mut state.nodes`
                            // loop below that writes `selected`.
                            let flow_boxes: Vec<(NodeId, (f32, f32), (f32, f32))> = state
                                .nodes
                                .iter()
                                .filter(|n| !n.hidden)
                                .map(|n| {
                                    let abs_pos =
                                        state.absolute_position(&n.id).unwrap_or(n.position);
                                    let footprint = state.node_footprint(n);
                                    (n.id.clone(), (abs_pos.x, abs_pos.y), footprint)
                                })
                                .collect();
                            for node in &mut state.nodes {
                                if node.hidden {
                                    continue;
                                }
                                let Some((_, (nx, ny), (nw, nh))) =
                                    flow_boxes.iter().find(|(id, _, _)| *id == node.id)
                                else {
                                    continue;
                                };
                                // AABB intersection
                                let intersects =
                                    *nx < ex && nx + nw > sx && *ny < ey && ny + nh > sy;
                                node.selected = intersects;
                            }
                            changed = true;
                        }
                        // Connection dragging with snap-to-handle
                        else if state.connecting.is_some() {
                            // Clone draft to avoid borrow conflict with find_snap_target
                            let mut draft = state.connecting.clone().unwrap();
                            let snap = state.find_snap_target(&draft, flow_mouse);
                            if let Some(ref target) = snap {
                                draft.to_point = target.point;
                            } else {
                                draft.to_point = flow_mouse;
                            }
                            draft.snap_target = snap;
                            state.connecting = Some(draft);
                            changed = true;
                        }
                        // Node dragging
                        else if let Some(ref drag) = state.drag_state {
                            let dx = (mx - drag.origin_mouse.0) / state.viewport.zoom;
                            let dy = (my - drag.origin_mouse.1) / state.viewport.zoom;

                            let origins = drag.node_origins.clone();
                            let snap = state.snap_to_grid;
                            let snap_grid = state.snap_grid;
                            for (node_id, origin) in &origins {
                                let mut new_x = origin.x + dx;
                                let mut new_y = origin.y + dy;

                                if snap {
                                    let (gx, gy) = snap_grid;
                                    new_x = (new_x / gx).round() * gx;
                                    new_y = (new_y / gy).round() * gy;
                                }

                                if let Some(node) = state.get_node_mut(node_id) {
                                    node.position = FlowPoint::new(new_x, new_y);
                                }
                            }
                            changed = true;
                        }
                        // Canvas panning
                        else if let Some(pan) = state.pan_drag {
                            state.viewport.x = pan.start_viewport.0 + (mx - pan.start_mouse.0);
                            state.viewport.y = pan.start_viewport.1 + (my - pan.start_mouse.1);
                            changed = true;
                        }
                        // Idle: track the edge under the cursor for the
                        // insert button. The button sits on the edge's
                        // midpoint and is wider than the line, so the edge
                        // stays hovered while the cursor is on the button
                        // even where that is off the line itself.
                        else if hover_edges {
                            let on_button = state.hovered_edge.clone().filter(|id| {
                                state
                                    .edges
                                    .iter()
                                    .find(|edge| edge.id == *id)
                                    .and_then(|edge| {
                                        edges::compute_edge_label_position(state, edge)
                                    })
                                    .is_some_and(|(x, y)| {
                                        let reach = EDGE_INSERT_BUTTON_SIZE;
                                        (flow_mouse.0 - x).abs() <= reach
                                            && (flow_mouse.1 - y).abs() <= reach
                                    })
                            });
                            let hovered = on_button
                                .or_else(|| edges::hit_test_edges(state, flow_mouse, 10.0));
                            if hovered != state.hovered_edge {
                                state.hovered_edge = hovered;
                                changed = true;
                            }
                        }
                    });

                    if changed {
                        cx.notify(entity_id);
                    }
                }
            })
            // Global mouse up → end dragging, panning, or cancel connection
            .on_mouse_up(MouseButton::Left, {
                let on_connection_drop = on_connection_drop.clone();
                move |_event, _window, cx| {
                    let mut changed = false;

                    state_for_mouse_up.update(cx, |state, _| {
                        if state.selection_box.is_some() {
                            state.selection_box = None;
                            changed = true;
                        }
                        if let Some(draft) = state.connecting.take() {
                            if let Some(snap) = draft.snap_target {
                                let SnapTarget {
                                    node_id: snap_node_id,
                                    handle_id: snap_handle_id,
                                    ..
                                } = snap;
                                // Complete the connection
                                let (source, target, source_handle, target_handle) =
                                    if draft.from_type == HandleType::Source {
                                        (
                                            draft.from_node.clone(),
                                            snap_node_id,
                                            draft.from_handle.clone(),
                                            snap_handle_id,
                                        )
                                    } else {
                                        (
                                            snap_node_id,
                                            draft.from_node.clone(),
                                            snap_handle_id,
                                            draft.from_handle.clone(),
                                        )
                                    };

                                let connection = Connection {
                                    source,
                                    target,
                                    source_handle,
                                    target_handle,
                                };

                                if state.is_valid_connection(&connection) {
                                    state.push_undo();
                                    let edge_id: SharedString =
                                        format!("e{}-{}", connection.source, connection.target)
                                            .into();
                                    state.add_edge_from_connection(&connection, edge_id);
                                }
                            } else if let Some(callback) = &on_connection_drop {
                                // Dropped on empty canvas — the "drag from a
                                // handle to create a new connected node"
                                // gesture (no snap target means the drop
                                // wasn't on another handle).
                                let flow_point = state
                                    .viewport
                                    .screen_to_flow(draft.to_point.0, draft.to_point.1);
                                callback(&draft, flow_point, state);
                            }
                            changed = true;
                        }
                        if state.drag_state.is_some() {
                            state.drag_state = None;
                            for n in &mut state.nodes {
                                n.dragging = false;
                            }
                            changed = true;
                        }
                        if state.pan_drag.is_some() {
                            state.pan_drag = None;
                            changed = true;
                        }
                    });

                    if changed {
                        cx.notify(entity_id);
                    }
                }
            })
            // Keyboard shortcuts (delete, undo/redo)
            .on_key_down({
                let focus = self.focus_handle.clone();
                move |event: &KeyDownEvent, window, cx| {
                    let key: &str = event.keystroke.key.as_ref();

                    // Only handle destructive keys (delete/backspace) when the
                    // flow graph itself is focused — not when a child element
                    // (like an Input inside a node) has focus.
                    let graph_focused = focus.is_focused(window);

                    // Undo: Cmd+Z (always allowed)
                    if key == "z"
                        && event.keystroke.modifiers.platform
                        && !event.keystroke.modifiers.shift
                    {
                        state_for_key.update(cx, |state, _| {
                            state.undo();
                        });
                        cx.notify(entity_id);
                        return;
                    }
                    // Redo: Cmd+Shift+Z
                    if key == "z"
                        && event.keystroke.modifiers.platform
                        && event.keystroke.modifiers.shift
                    {
                        state_for_key.update(cx, |state, _| {
                            state.redo();
                        });
                        cx.notify(entity_id);
                        return;
                    }

                    // Select all: Cmd+A
                    if key == "a" && event.keystroke.modifiers.platform && graph_focused {
                        state_for_key.update(cx, |state, _| {
                            state.select_all();
                        });
                        cx.notify(entity_id);
                        return;
                    }

                    // Tab / Shift+Tab: cycle the single-node selection —
                    // there was previously no way to reach a node at all
                    // without clicking it first, which also meant no way to
                    // reach the inspector for a node currently off-screen.
                    // Does not scroll the newly selected node into view
                    // (unlike `fit_view`, this handler doesn't have the
                    // container's real size to center against) — the
                    // selection can still land somewhere the viewport
                    // doesn't show.
                    if key == "tab" && graph_focused {
                        let forward = !event.keystroke.modifiers.shift;
                        state_for_key.update(cx, |state, _| {
                            state.select_next_node(forward);
                        });
                        cx.notify(entity_id);
                        return;
                    }

                    if (key == "backspace" || key == "delete") && graph_focused {
                        state_for_key.update(cx, |state, _| {
                            state.push_undo();
                            // Collect IDs of selected deletable nodes
                            let node_ids_to_remove: Vec<NodeId> = state
                                .nodes
                                .iter()
                                .filter(|n| n.selected && n.deletable)
                                .map(|n| n.id.clone())
                                .collect();

                            // Remove edges connected to deleted nodes + selected edges
                            state.edges.retain(|e| {
                                let connected_to_removed = node_ids_to_remove.contains(&e.source)
                                    || node_ids_to_remove.contains(&e.target);
                                let selected_and_deletable = e.selected && e.deletable;
                                !connected_to_removed && !selected_and_deletable
                            });

                            // Remove selected nodes
                            state.nodes.retain(|n| !node_ids_to_remove.contains(&n.id));
                            if !node_ids_to_remove.is_empty() {
                                state.rebuild_lookup();
                            }
                        });
                        cx.notify(entity_id);
                    }
                }
            })
            // Scroll to zoom / pan
            .on_scroll_wheel({
                move |event, _window, cx| {
                    let delta = event.delta.pixel_delta(px(20.0));
                    let mouse_pos = event.position;

                    if event.modifiers.platform || event.modifiers.control {
                        // Zoom towards mouse
                        let zoom_delta = -delta.y.as_f32() * 0.01;
                        state_for_scroll.update(cx, |state, _| {
                            let old_zoom = state.viewport.zoom;
                            let new_zoom =
                                (old_zoom + zoom_delta).clamp(state.min_zoom, state.max_zoom);
                            let mx = mouse_pos.x.as_f32();
                            let my = mouse_pos.y.as_f32();
                            state.viewport.x = mx - (mx - state.viewport.x) * (new_zoom / old_zoom);
                            state.viewport.y = my - (my - state.viewport.y) * (new_zoom / old_zoom);
                            state.viewport.zoom = new_zoom;
                            state.refit_all_containers();
                        });
                    } else {
                        // Pan
                        state_for_scroll.update(cx, |state, _| {
                            state.viewport.x += delta.x.as_f32();
                            state.viewport.y += delta.y.as_f32();
                        });
                    }

                    cx.notify(entity_id);
                }
            })
            // Pinch to zoom (macOS trackpad / Linux Wayland)
            .on_pinch({
                move |event, _window, cx| {
                    let mouse_pos = event.position;
                    let zoom_delta = event.delta;
                    state_for_pinch.update(cx, |state, _| {
                        let old_zoom = state.viewport.zoom;
                        let new_zoom =
                            (old_zoom * (1.0 + zoom_delta)).clamp(state.min_zoom, state.max_zoom);
                        let mx = mouse_pos.x.as_f32();
                        let my = mouse_pos.y.as_f32();
                        state.viewport.x = mx - (mx - state.viewport.x) * (new_zoom / old_zoom);
                        state.viewport.y = my - (my - state.viewport.y) * (new_zoom / old_zoom);
                        state.viewport.zoom = new_zoom;
                        state.refit_all_containers();
                    });
                    cx.notify(entity_id);
                }
            })
    }
}
