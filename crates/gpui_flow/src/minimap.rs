use std::cell::Cell;
use std::rc::Rc;

use gpui::*;

use crate::store::FlowState;

const MINIMAP_WIDTH: f32 = 200.0;
const MINIMAP_HEIGHT: f32 = 140.0;
const MINIMAP_PADDING: f32 = 10.0;

/// A minimap component that shows a bird's-eye view of the flow graph.
///
/// Renders a scaled-down view of all nodes and edges, with a rectangle
/// indicating the current viewport. Click or drag on the minimap to pan.
pub struct Minimap {
    state: Entity<FlowState>,
    /// Container bounds captured during rendering (for viewport calculations).
    container_bounds: Option<(f32, f32)>,
    /// The minimap div's own screen bounds, captured each paint by the
    /// canvas element below and read back by the mouse handlers. Mouse
    /// events report `event.position` in *window* coordinates, not
    /// relative to this div, so panning math needs this to convert —
    /// without it, a click was treated as if it were already relative to
    /// the minimap's top-left, producing a wildly wrong flow-space target
    /// and panning the viewport far off-screen (nodes appeared to
    /// "disappear" because they were still there, just no longer visible).
    bounds: Rc<Cell<Bounds<Pixels>>>,
    background: Hsla,
    border: Hsla,
}

impl Minimap {
    pub fn new(state: Entity<FlowState>) -> Self {
        Self {
            state,
            container_bounds: None,
            bounds: Rc::new(Cell::new(Bounds::default())),
            background: gpui::rgba(0x1a1a1acc).into(),
            border: gpui::rgba(0xffffff33).into(),
        }
    }

    /// Set the container bounds (the main flow graph's size).
    pub fn container_bounds(mut self, width: f32, height: f32) -> Self {
        self.container_bounds = Some((width, height));
        self
    }

    pub fn theme_colors(&mut self, background: Hsla, border: Hsla) {
        self.background = background;
        self.border = border;
    }
}

impl Render for Minimap {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state_for_canvas = self.state.clone();
        let state_for_mouse = self.state.clone();
        let entity_id = cx.entity_id();
        let container = self.container_bounds.unwrap_or((900.0, 600.0));
        let bounds_for_layout = self.bounds.clone();
        let bounds_for_down = self.bounds.clone();
        let bounds_for_move = self.bounds.clone();
        let background = self.background;
        let border = self.border;

        div()
            .id("flow-minimap")
            .debug_selector(|| "flow-minimap".to_string())
            .w(px(MINIMAP_WIDTH))
            .h(px(MINIMAP_HEIGHT))
            .bg(background)
            .rounded_md()
            .border_1()
            .border_color(border)
            .overflow_hidden()
            .child(
                canvas(
                    move |bounds, _window, _cx| {
                        bounds_for_layout.set(bounds);
                    },
                    move |bounds, _: (), window, cx| {
                        let state = state_for_canvas.read(cx);
                        paint_minimap(&bounds, state, container, window);
                    },
                )
                .size_full(),
            )
            .on_mouse_down(MouseButton::Left, {
                let state = state_for_mouse.clone();
                move |event, _window, cx| {
                    let origin = bounds_for_down.get().origin;
                    let mx = event.position.x.as_f32() - origin.x.as_f32();
                    let my = event.position.y.as_f32() - origin.y.as_f32();
                    pan_to_minimap_point(&state, mx, my, container, cx);
                    cx.notify(entity_id);
                }
            })
            .on_mouse_move({
                let state = state_for_mouse;
                move |event, _window, cx| {
                    if event.pressed_button == Some(MouseButton::Left) {
                        let origin = bounds_for_move.get().origin;
                        let mx = event.position.x.as_f32() - origin.x.as_f32();
                        let my = event.position.y.as_f32() - origin.y.as_f32();
                        pan_to_minimap_point(&state, mx, my, container, cx);
                        cx.notify(entity_id);
                    }
                }
            })
    }
}

/// Pan the viewport so the center of the visible area aligns with the clicked minimap point.
///
/// `mx`/`my` are **minimap-local** pixels (the caller subtracts this canvas's
/// own window origin). The minimap's mapping is a pure affine
/// `flow -> minimap` transform built from `compute_graph_bounds`, so undoing
/// it is exact — the minimap deliberately has no `viewport.zoom` term of its
/// own; the viewport indicator is drawn by mapping the *visible flow rect*
/// through the same transform.
fn pan_to_minimap_point(
    state: &Entity<FlowState>,
    mx: f32,
    my: f32,
    container: (f32, f32),
    cx: &mut App,
) {
    state.update(cx, |state, _| {
        let (graph_bounds, _) = compute_graph_bounds(state);
        if graph_bounds.2 <= 0.0 || graph_bounds.3 <= 0.0 {
            return;
        }

        let inner_w = MINIMAP_WIDTH - MINIMAP_PADDING * 2.0;
        let inner_h = MINIMAP_HEIGHT - MINIMAP_PADDING * 2.0;
        let scale_x = inner_w / graph_bounds.2;
        let scale_y = inner_h / graph_bounds.3;
        let scale = scale_x.min(scale_y);

        let offset_x = (inner_w - graph_bounds.2 * scale) / 2.0 + MINIMAP_PADDING;
        let offset_y = (inner_h - graph_bounds.3 * scale) / 2.0 + MINIMAP_PADDING;

        // Invert the flow -> minimap map to get the clicked flow point.
        let flow_x = (mx - offset_x) / scale + graph_bounds.0;
        let flow_y = (my - offset_y) / scale + graph_bounds.1;

        // Center viewport on this flow point
        state.viewport.x = container.0 / 2.0 - flow_x * state.viewport.zoom;
        state.viewport.y = container.1 / 2.0 - flow_y * state.viewport.zoom;
    });
}

/// Paint the minimap contents.
///
/// Everything here works in **flow-space units** and maps to minimap pixels
/// through the single affine transform derived from `compute_graph_bounds`.
/// `FlowState::node_footprint` returns flow units for both leaves and
/// containers, so node rects, edge lines, the graph-bounds span, and the
/// viewport indicator all share one space — previously leaf footprints were
/// screen pixels while container footprints were flow units, which made
/// `scale` meaningless as a ratio and left the viewport indicator visibly out
/// of register with the nodes it was meant to frame.
fn paint_minimap(
    bounds: &Bounds<Pixels>,
    state: &FlowState,
    container: (f32, f32),
    window: &mut Window,
) {
    let (graph_bounds, has_nodes) = compute_graph_bounds(state);
    if !has_nodes {
        return;
    }

    let bx = bounds.origin.x.as_f32();
    let by = bounds.origin.y.as_f32();

    // Scale graph to fit minimap with padding
    let inner_w = MINIMAP_WIDTH - MINIMAP_PADDING * 2.0;
    let inner_h = MINIMAP_HEIGHT - MINIMAP_PADDING * 2.0;
    let scale_x = inner_w / graph_bounds.2;
    let scale_y = inner_h / graph_bounds.3;
    let scale = scale_x.min(scale_y);

    let offset_x = bx + (inner_w - graph_bounds.2 * scale) / 2.0 + MINIMAP_PADDING;
    let offset_y = by + (inner_h - graph_bounds.3 * scale) / 2.0 + MINIMAP_PADDING;

    // Paint nodes as small rectangles. Uses each node's *resolved* absolute
    // position (walking any parent chain — see
    // `FlowState::absolute_position`), so a child renders nested inside its
    // container's rectangle here too, same as on the main canvas.
    let node_color = gpui::rgba(0xffffff88);
    for node in &state.nodes {
        if node.hidden {
            continue;
        }
        let abs_pos = state.absolute_position(&node.id).unwrap_or(node.position);
        let (w, h) = state.node_footprint(node);
        let nx = offset_x + (abs_pos.x - graph_bounds.0) * scale;
        let ny = offset_y + (abs_pos.y - graph_bounds.1) * scale;
        let nw = w * scale;
        let nh = h * scale;

        let node_bounds = Bounds::new(
            Point::new(px(nx), px(ny)),
            Size {
                width: px(nw),
                height: px(nh),
            },
        );

        let color = if node.selected {
            gpui::rgba(0x3b82f6aa)
        } else {
            node_color
        };
        window.paint_quad(fill(node_bounds, color));
    }

    // Paint edges as thin lines
    let edge_color: Background = gpui::rgba(0xffffff44).into();
    for edge in &state.edges {
        if edge.hidden {
            continue;
        }
        let source = state.get_node(&edge.source);
        let target = state.get_node(&edge.target);
        if let (Some(src), Some(tgt)) = (source, target) {
            let (sw, sh) = state.node_footprint(src);
            let (tw, th) = state.node_footprint(tgt);
            let src_abs = state.absolute_position(&src.id).unwrap_or(src.position);
            let tgt_abs = state.absolute_position(&tgt.id).unwrap_or(tgt.position);

            let sx = offset_x + (src_abs.x + sw / 2.0 - graph_bounds.0) * scale;
            let sy = offset_y + (src_abs.y + sh / 2.0 - graph_bounds.1) * scale;
            let tx = offset_x + (tgt_abs.x + tw / 2.0 - graph_bounds.0) * scale;
            let ty = offset_y + (tgt_abs.y + th / 2.0 - graph_bounds.1) * scale;

            let mut builder = PathBuilder::stroke(px(1.0));
            builder.move_to(Point::new(px(sx), px(sy)));
            builder.line_to(Point::new(px(tx), px(ty)));
            if let Ok(path) = builder.build() {
                window.paint_path(path, edge_color);
            }
        }
    }

    // Paint viewport indicator
    let viewport = &state.viewport;
    // Convert viewport screen bounds to flow coordinates
    let vp_left = -viewport.x / viewport.zoom;
    let vp_top = -viewport.y / viewport.zoom;
    let vp_width = container.0 / viewport.zoom;
    let vp_height = container.1 / viewport.zoom;

    let vx = offset_x + (vp_left - graph_bounds.0) * scale;
    let vy = offset_y + (vp_top - graph_bounds.1) * scale;
    let vw = vp_width * scale;
    let vh = vp_height * scale;

    let vp_bounds = Bounds::new(
        Point::new(px(vx), px(vy)),
        Size {
            width: px(vw),
            height: px(vh),
        },
    );
    window.paint_quad(fill(vp_bounds, gpui::rgba(0x3b82f620)));

    // Viewport border
    let border_color: Background = gpui::rgba(0x3b82f6aa).into();
    let top = Bounds::new(
        Point::new(px(vx), px(vy)),
        Size {
            width: px(vw),
            height: px(1.0),
        },
    );
    window.paint_quad(fill(top, border_color));
    let bottom = Bounds::new(
        Point::new(px(vx), px(vy + vh)),
        Size {
            width: px(vw),
            height: px(1.0),
        },
    );
    window.paint_quad(fill(bottom, border_color));
    let left = Bounds::new(
        Point::new(px(vx), px(vy)),
        Size {
            width: px(1.0),
            height: px(vh),
        },
    );
    window.paint_quad(fill(left, border_color));
    let right = Bounds::new(
        Point::new(px(vx + vw), px(vy)),
        Size {
            width: px(1.0),
            height: px(vh),
        },
    );
    window.paint_quad(fill(right, border_color));
}

/// Compute the bounding box of all *top-level* nodes in flow coordinates —
/// a child's extent is already covered by its container's own footprint
/// (`FlowState::node_footprint`, container-size-aware), so including it too
/// would double-count rather than improve accuracy, same reasoning as
/// `FlowState::fit_view`. Returns ((min_x, min_y, width, height), has_nodes).
fn compute_graph_bounds(state: &FlowState) -> ((f32, f32, f32, f32), bool) {
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;
    let mut count = 0;

    for node in &state.nodes {
        if node.hidden || node.parent_id.is_some() {
            continue;
        }
        let (w, h) = state.node_footprint(node);
        min_x = min_x.min(node.position.x);
        min_y = min_y.min(node.position.y);
        max_x = max_x.max(node.position.x + w);
        max_y = max_y.max(node.position.y + h);
        count += 1;
    }

    if count == 0 {
        return ((0.0, 0.0, 0.0, 0.0), false);
    }

    // Add some padding
    let padding = 50.0;
    min_x -= padding;
    min_y -= padding;
    max_x += padding;
    max_y += padding;

    ((min_x, min_y, max_x - min_x, max_y - min_y), true)
}
