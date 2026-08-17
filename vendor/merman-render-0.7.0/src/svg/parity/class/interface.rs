use crate::entities::decode_entities_minimal_cow;
use crate::generated::class_text_overrides_11_12_2 as class_text_overrides;
use crate::model::{Bounds, LayoutNode};
use crate::text::{TextMeasurer, TextStyle, WrapMode};
use std::fmt::Write as _;

use super::super::{escape_attr_display, escape_xml_into, fmt};
use super::ClassSvgInterface;
use super::bounds::include_xywh;
use super::node::ClassNodeRenderPosition;

pub(super) struct ClassInterfaceRenderContext<'a> {
    pub diagram_id: &'a str,
    pub measurer: &'a dyn TextMeasurer,
    pub text_style: &'a TextStyle,
    pub line_height: f64,
    pub look: &'a str,
}

pub(super) struct ClassInterfaceRenderState<'a> {
    pub out: &'a mut String,
    pub content_bounds: &'a mut Option<Bounds>,
}

pub(super) fn render_class_interface_node(
    state: ClassInterfaceRenderState<'_>,
    iface: &ClassSvgInterface,
    layout_node: &LayoutNode,
    position: ClassNodeRenderPosition,
    ctx: &ClassInterfaceRenderContext<'_>,
) {
    let out = &mut *state.out;
    let content_bounds = &mut *state.content_bounds;

    let label_text = decode_entities_minimal_cow(iface.label.trim());
    let (fo_w_raw, fo_h_raw) = match (layout_node.label_width, layout_node.label_height) {
        (Some(w), Some(h)) => (w, h),
        _ => {
            let metrics =
                ctx.measurer
                    .measure_wrapped(&label_text, ctx.text_style, None, WrapMode::HtmlLike);
            (metrics.width, metrics.height)
        }
    };
    let fo_w = fo_w_raw.max(1.0);
    let fo_h = fo_h_raw.max(ctx.line_height).max(1.0);

    let w = fo_w;
    let h = fo_h;
    let left = -w / 2.0;
    let top = -h / 2.0;

    include_xywh(
        content_bounds,
        position.node_bounds_tx + left,
        position.node_bounds_ty + top,
        w,
        h,
    );
    include_xywh(
        content_bounds,
        position.node_bounds_tx + left,
        position.node_bounds_ty + top,
        fo_w,
        fo_h,
    );

    let _ = write!(
        out,
        r#"<g class="node undefined" id="{}-{}" data-look="{}" transform="translate({}, {})"><rect class="basic label-container" style="opacity:0; !important" x="{}" y="{}" width="{}" height="{}"/><g class="label" style="" transform="translate({}, {})"><rect/><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: center;"><span class="nodeLabel"><p>"#,
        escape_attr_display(ctx.diagram_id),
        escape_attr_display(&iface.id),
        escape_attr_display(ctx.look),
        fmt(position.node_tx),
        fmt(position.node_ty),
        fmt(left),
        fmt(top),
        fmt(w),
        fmt(h),
        fmt(left),
        fmt(top),
        fmt(fo_w),
        fmt(fo_h),
        class_text_overrides::class_html_label_max_width_px(),
    );
    for (idx, line) in label_text.split('\n').enumerate() {
        if idx > 0 {
            out.push_str("<br />");
        }
        escape_xml_into(out, line);
    }
    out.push_str("</p></span></div></foreignObject></g></g>");
}
