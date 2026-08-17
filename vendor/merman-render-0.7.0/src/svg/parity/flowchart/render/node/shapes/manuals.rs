//! Flowchart v2 manual input/file shapes.

use std::fmt::Write as _;

use crate::flowchart::flowchart_effective_text_style_for_node_classes;
use crate::svg::parity::{escape_xml_display, fmt, fmt_display};

use super::super::geom::path_from_points;
use super::super::helpers;
use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_manual_file(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &mut super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let metrics = helpers::compute_node_label_metrics(
        ctx,
        None,
        label.text,
        label.label_type,
        common.node_classes,
        common.node_styles,
    );
    let p = ctx.node_padding;
    let w = metrics.width + p;
    let h = (w + metrics.height).max(1.0);
    let pts = vec![
        (0.0, -h),
        (w + metrics.height, -h),
        ((w + metrics.height) / 2.0, 0.0),
    ];
    let path_data = path_from_points(&pts);
    if let Some((fill_d, stroke_d)) =
        super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
            roughjs_paths_for_svg_path(
                &path_data,
                common.fill_color,
                common.stroke_color,
                common.stroke_width,
                common.stroke_dasharray,
                common.hand_drawn_seed,
            )
        })
    {
        let _ = write!(
            out,
            r#"<g transform="translate({},{})" class="outer-path">"#,
            fmt_display(-h / 2.0),
            fmt_display(h / 2.0)
        );
        let _ = write!(
            out,
            r#"<path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/>"#,
            escape_xml_display(&fill_d),
            escape_xml_display(common.fill_color),
            escape_xml_display(common.style)
        );
        let _ = write!(
            out,
            r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/>"#,
            escape_xml_display(&stroke_d),
            escape_xml_display(common.stroke_color),
            fmt_display(common.stroke_width as f64),
            escape_xml_display(common.stroke_dasharray),
            escape_xml_display(common.style)
        );
        out.push_str("</g>");
    }

    let node_text_style = flowchart_effective_text_style_for_node_classes(
        &ctx.text_style,
        ctx.class_defs,
        common.node_classes,
        common.node_styles,
    );
    let bbox_y_offset = if ctx.node_html_labels {
        0.0
    } else {
        crate::text::svg_create_text_bbox_y_offset_px(&node_text_style)
    };
    label.dy = metrics.height / 2.0 - h / 2.0 + p / 2.0 + bbox_y_offset;
}

pub(in crate::svg::parity::flowchart::render::node) fn render_manual_input(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &mut super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let metrics = helpers::compute_node_label_metrics(
        ctx,
        None,
        label.text,
        label.label_type,
        common.node_classes,
        common.node_styles,
    );
    let p = ctx.node_padding;
    let w = (metrics.width + 2.0 * p).max(1.0);
    let h = (metrics.height + 2.0 * p).max(1.0);
    let x = -w / 2.0;
    let y = -h / 2.0;
    let points = vec![(x, y), (x, y + h), (x + w, y + h), (x + w, y - h / 2.0)];
    let path_data = path_from_points(&points);
    if let Some((fill_d, stroke_d)) =
        super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
            roughjs_paths_for_svg_path(
                &path_data,
                common.fill_color,
                common.stroke_color,
                common.stroke_width,
                common.stroke_dasharray,
                common.hand_drawn_seed,
            )
        })
    {
        let _ = write!(
            out,
            r#"<g class="basic label-container  outer-path" transform="translate(0,{})">"#,
            fmt(h / 4.0)
        );
        let _ = write!(
            out,
            r#"<path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/>"#,
            escape_xml_display(&fill_d),
            escape_xml_display(common.fill_color),
            escape_xml_display(common.style)
        );
        let _ = write!(
            out,
            r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/>"#,
            escape_xml_display(&stroke_d),
            escape_xml_display(common.stroke_color),
            fmt_display(common.stroke_width as f64),
            escape_xml_display(common.stroke_dasharray),
            escape_xml_display(common.style)
        );
        out.push_str("</g>");
    }

    let node_text_style = flowchart_effective_text_style_for_node_classes(
        &ctx.text_style,
        ctx.class_defs,
        common.node_classes,
        common.node_styles,
    );
    let bbox_y_offset = if ctx.node_html_labels {
        0.0
    } else {
        crate::text::svg_create_text_bbox_y_offset_px(&node_text_style)
    };
    label.dy = metrics.height / 2.0 - h / 4.0 + p - bbox_y_offset;
}
