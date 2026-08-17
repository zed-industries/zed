//! Flowchart v2 shapes that emit a label container and continue into label rendering.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::{OptionalStyleAttr, escape_attr};
use crate::svg::parity::util;

use super::super::geom::path_from_points;
use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_hourglass_collate(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let w = common.layout_node.width.max(30.0);
    let h = common.layout_node.height.max(30.0);
    let pts: Vec<(f64, f64)> = vec![(0.0, 0.0), (w, 0.0), (0.0, h), (w, h)];
    let path_data = path_from_points(&pts);
    let (fill_d, stroke_d) =
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
        .unwrap_or_else(|| ("M0,0".to_string(), "M0,0".to_string()));
    let _ = write!(
        out,
        r##"<g class="basic label-container outer-path" transform="translate({},{})"><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g>"##,
        util::fmt(-w / 2.0),
        util::fmt(-h / 2.0),
        escape_attr(&fill_d),
        escape_attr(common.fill_color),
        escape_attr(common.style),
        escape_attr(&stroke_d),
        escape_attr(common.stroke_color),
        util::fmt(common.stroke_width as f64),
        escape_attr(common.stroke_dasharray),
        escape_attr(common.style),
    );
}

pub(in crate::svg::parity::flowchart::render::node) fn render_notched_rectangle(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
) {
    let w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let notch = 12.0;
    let pts: Vec<(f64, f64)> = vec![
        (notch, -h),
        (w, -h),
        (w, 0.0),
        (0.0, 0.0),
        (0.0, -h + notch),
        (notch, -h),
    ];
    let mut points_attr = String::new();
    for (idx, (px, py)) in pts.iter().copied().enumerate() {
        if idx > 0 {
            points_attr.push(' ');
        }
        let _ = write!(&mut points_attr, "{},{}", util::fmt(px), util::fmt(py));
    }
    let _ = write!(
        out,
        r#"<polygon points="{}" class="label-container" transform="translate({},{})"{} />"#,
        points_attr,
        util::fmt(-w / 2.0),
        util::fmt(h / 2.0),
        OptionalStyleAttr(common.style),
    );
}
