//! Flowchart node renderer.

use super::super::*;

pub(in crate::svg::parity::flowchart) mod geom;
mod helpers;
mod label;
pub(in crate::svg::parity) mod roughjs;
pub(in crate::svg::parity::flowchart) mod shapes;

pub(in crate::svg::parity::flowchart::render) struct FlowchartNodeRenderCommon<'a> {
    pub shape: &'a str,
    pub look: &'a str,
    pub layout_node: &'a crate::model::LayoutNode,
    pub node_classes: &'a [String],
    pub node_styles: &'a [String],
    pub node_icon: Option<&'a str>,
    pub node_img: Option<&'a str>,
    pub node_pos: Option<&'a str>,
    pub node_constraint: Option<&'a str>,
    pub node_asset_width: Option<f64>,
    pub node_asset_height: Option<f64>,
    pub style: &'a str,
    pub fill_color: &'a str,
    pub stroke_color: &'a str,
    pub stroke_width: f32,
    pub stroke_dasharray: &'a str,
    pub hand_drawn_seed: u64,
    pub wrapped_in_a: bool,
    pub timing_enabled: bool,
}

impl FlowchartNodeRenderCommon<'_> {
    pub(super) fn look_is_neo(&self) -> bool {
        self.look == "neo"
    }

    pub(super) fn look_is_hand_drawn(&self) -> bool {
        self.look == "handDrawn"
    }
}

pub(in crate::svg::parity::flowchart::render) struct FlowchartNodeLabelState<'a> {
    pub text: &'a str,
    pub label_type: &'a str,
    pub dx: f64,
    pub dy: f64,
}

pub(in crate::svg::parity::flowchart) fn render_flowchart_node(
    out: &mut String,
    ctx: &FlowchartRenderCtx<'_>,
    node_id: &str,
    origin_x: f64,
    origin_y: f64,
    timing_enabled: bool,
    details: &mut FlowchartRenderDetails,
) {
    let Some(layout_node) = ctx.layout_nodes_by_id.get(node_id) else {
        return;
    };

    let x = layout_node.x + ctx.tx - origin_x;
    let y = layout_node.y + ctx.ty - origin_y;

    if helpers::try_render_self_loop_label_placeholder(out, node_id, x, y) {
        return;
    }

    let Some(resolved) = helpers::resolve_node_render_info(ctx, node_id) else {
        return;
    };

    let tooltip = ctx.tooltips.get(node_id).map(|s| s.as_str()).unwrap_or("");
    let tooltip_enabled = !tooltip.trim().is_empty();

    let dom_idx = resolved.dom_idx;
    let class_attr_base = resolved.class_attr_base;
    let wrapped_in_a = resolved.wrapped_in_a;
    let href = resolved.href;
    let shape: &str = resolved.shape;
    let node_icon = resolved.node_icon;
    let node_img = resolved.node_img;
    let node_pos = resolved.node_pos;
    let node_constraint = resolved.node_constraint;
    let node_asset_width = resolved.node_asset_width;
    let node_asset_height = resolved.node_asset_height;
    let node_styles = resolved.node_styles;
    let node_classes = resolved.node_classes;

    let empty_classes: &[String] = &[];
    let node_classes_for_wrapper = match shape {
        // Mermaid flowchart-v2 start/stop nodes do not carry classDef classes on the wrapper.
        // Styling is applied via inline styles on the shape paths (stop) or ignored (start).
        "sm-circ" | "small-circle" | "start" | "fr-circ" | "framed-circle" | "stop" => {
            empty_classes
        }
        _ => node_classes,
    };
    let look = flowchart_config_look(ctx.config);

    helpers::open_node_wrapper(
        out,
        helpers::NodeWrapperAttrs {
            diagram_id: ctx.diagram_id,
            node_id,
            dom_idx,
            class_attr_base,
            node_classes: node_classes_for_wrapper,
            wrapped_in_a,
            href,
            x,
            y,
            tooltip_enabled,
            tooltip,
            look,
        },
    );

    let style_start = timing_enabled.then(web_time::Instant::now);
    let mut compiled_styles =
        flowchart_compile_node_styles(ctx.class_defs, node_classes, node_styles, &[]);
    if let Some(s) = style_start {
        details.node_style_compile += s.elapsed();
    }
    let style = std::mem::take(&mut compiled_styles.node_style);
    let fill_color = compiled_styles
        .fill
        .as_deref()
        .unwrap_or(ctx.node_fill_color.as_str());
    let stroke_color = compiled_styles
        .stroke
        .as_deref()
        .unwrap_or(ctx.node_border_color.as_str());
    let stroke_width: f32 = compiled_styles
        .stroke_width
        .as_deref()
        .and_then(|v| v.trim_end_matches("px").trim().parse::<f32>().ok())
        .unwrap_or(1.3);
    let stroke_dasharray = compiled_styles
        .stroke_dasharray
        .as_deref()
        .unwrap_or("0 0")
        .trim();

    let hand_drawn_seed = ctx
        .config
        .as_value()
        .get("handDrawnSeed")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let common = FlowchartNodeRenderCommon {
        shape,
        look,
        layout_node,
        node_classes,
        node_styles,
        node_icon,
        node_img,
        node_pos,
        node_constraint,
        node_asset_width,
        node_asset_height,
        style: &style,
        fill_color,
        stroke_color,
        stroke_width,
        stroke_dasharray,
        hand_drawn_seed,
        wrapped_in_a,
        timing_enabled,
    };
    let mut label = FlowchartNodeLabelState {
        text: if resolved.label_text_is_node_id {
            node_id
        } else {
            resolved.label_text
        },
        label_type: resolved.label_type,
        dx: 0.0,
        dy: 0.0,
    };

    if shapes::try_render_flowchart_v2_no_label(out, ctx, &common, details) {
        out.push_str("</g>");
        if common.wrapped_in_a {
            out.push_str("</a>");
        }
        return;
    }

    if shapes::render_flowchart_v2_shape(out, ctx, &common, &mut label, details) {
        return;
    }

    label::render_flowchart_node_label(out, ctx, &common, &label, &compiled_styles, details);
}
