//! Flowchart v2 curly brace / comment shapes.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::{fmt, fmt_display};

use super::super::geom::path_from_points;
use super::super::roughjs::roughjs_stroke_path_for_svg_path;

pub(in crate::svg::parity::flowchart) struct CurlyBraceCommentGeometry {
    pub(in crate::svg::parity::flowchart) group_tx: f64,
    pub(in crate::svg::parity::flowchart) label_dx: f64,
    pub(in crate::svg::parity::flowchart) paths: Vec<CurlyBraceCommentPath>,
}

pub(in crate::svg::parity::flowchart) struct CurlyBraceCommentPath {
    pub(in crate::svg::parity::flowchart) d: String,
    pub(in crate::svg::parity::flowchart) visible: bool,
}

fn circle_points(
    center_x: f64,
    center_y: f64,
    radius: f64,
    num_points: usize,
    start_deg: f64,
    end_deg: f64,
    negate: bool,
) -> Vec<(f64, f64)> {
    let start = start_deg.to_radians();
    let end = end_deg.to_radians();
    let angle_range = end - start;
    let angle_step = if num_points > 1 {
        angle_range / (num_points as f64 - 1.0)
    } else {
        0.0
    };
    let mut out: Vec<(f64, f64)> = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let a = start + (i as f64) * angle_step;
        let x = center_x + radius * a.cos();
        let y = center_y + radius * a.sin();
        if negate {
            out.push((-x, -y));
        } else {
            out.push((x, y));
        }
    }
    out
}

pub(in crate::svg::parity::flowchart) fn curly_brace_comment_geometry(
    shape: &str,
    label_w: f64,
    label_h: f64,
    padding: f64,
) -> CurlyBraceCommentGeometry {
    let w = (label_w + padding).max(1.0);
    let h = (label_h + padding).max(1.0);
    let radius = (h * 0.1).max(5.0);

    let (group_tx, label_dx) = match shape {
        "comment" | "brace" | "brace-l" => (radius, -radius / 2.0),
        "brace-r" => (-radius, 0.0),
        "braces" => (radius - radius / 4.0, 0.0),
        _ => (0.0, 0.0),
    };

    let paths = if shape == "braces" {
        // Mermaid `curlyBraces.ts`: two visible brace paths + one invisible rect path.
        let left_points: Vec<(f64, f64)> = [
            circle_points(w / 2.0, -h / 2.0, radius, 30, -90.0, 0.0, true),
            vec![(-w / 2.0 - radius, radius)],
            circle_points(
                w / 2.0 + radius * 2.0,
                -radius,
                radius,
                20,
                -180.0,
                -270.0,
                true,
            ),
            circle_points(
                w / 2.0 + radius * 2.0,
                radius,
                radius,
                20,
                -90.0,
                -180.0,
                true,
            ),
            vec![(-w / 2.0 - radius, -h / 2.0)],
            circle_points(w / 2.0, h / 2.0, radius, 20, 0.0, 90.0, true),
        ]
        .into_iter()
        .flatten()
        .collect();
        let right_points: Vec<(f64, f64)> = [
            circle_points(
                -w / 2.0 + radius + radius / 2.0,
                -h / 2.0,
                radius,
                20,
                -90.0,
                -180.0,
                true,
            ),
            vec![(w / 2.0 - radius / 2.0, radius)],
            circle_points(
                -w / 2.0 - radius / 2.0,
                -radius,
                radius,
                20,
                0.0,
                90.0,
                true,
            ),
            circle_points(
                -w / 2.0 - radius / 2.0,
                radius,
                radius,
                20,
                -90.0,
                0.0,
                true,
            ),
            vec![(w / 2.0 - radius / 2.0, -radius)],
            circle_points(
                -w / 2.0 + radius + radius / 2.0,
                h / 2.0,
                radius,
                30,
                -180.0,
                -270.0,
                true,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        let rect_points: Vec<(f64, f64)> = [
            vec![(w / 2.0, -h / 2.0 - radius), (-w / 2.0, -h / 2.0 - radius)],
            circle_points(w / 2.0, -h / 2.0, radius, 20, -90.0, 0.0, true),
            vec![(-w / 2.0 - radius, -radius)],
            circle_points(
                w / 2.0 + radius * 2.0,
                -radius,
                radius,
                20,
                -180.0,
                -270.0,
                true,
            ),
            circle_points(
                w / 2.0 + radius * 2.0,
                radius,
                radius,
                20,
                -90.0,
                -180.0,
                true,
            ),
            vec![(-w / 2.0 - radius, h / 2.0)],
            circle_points(w / 2.0, h / 2.0, radius, 20, 0.0, 90.0, true),
            vec![
                (-w / 2.0, h / 2.0 + radius),
                (w / 2.0 - radius - radius / 2.0, h / 2.0 + radius),
            ],
            circle_points(
                -w / 2.0 + radius + radius / 2.0,
                -h / 2.0,
                radius,
                20,
                -90.0,
                -180.0,
                true,
            ),
            vec![(w / 2.0 - radius / 2.0, radius)],
            circle_points(
                -w / 2.0 - radius / 2.0,
                -radius,
                radius,
                20,
                0.0,
                90.0,
                true,
            ),
            circle_points(
                -w / 2.0 - radius / 2.0,
                radius,
                radius,
                20,
                -90.0,
                0.0,
                true,
            ),
            vec![(w / 2.0 - radius / 2.0, -radius)],
            circle_points(
                -w / 2.0 + radius + radius / 2.0,
                h / 2.0,
                radius,
                30,
                -180.0,
                -270.0,
                true,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();

        let left_path = path_from_points(&left_points)
            .trim_end_matches('Z')
            .to_string();
        let right_path = path_from_points(&right_points)
            .trim_end_matches('Z')
            .to_string();
        let rect_path = path_from_points(&rect_points);
        vec![
            CurlyBraceCommentPath {
                d: left_path,
                visible: true,
            },
            CurlyBraceCommentPath {
                d: right_path,
                visible: true,
            },
            CurlyBraceCommentPath {
                d: rect_path,
                visible: false,
            },
        ]
    } else {
        // Mermaid `curlyBraceLeft.ts` / `curlyBraceRight.ts`.
        let (points, rect_points) = if shape == "brace-r" {
            let points: Vec<(f64, f64)> = [
                circle_points(w / 2.0, -h / 2.0, radius, 20, -90.0, 0.0, false),
                vec![(w / 2.0 + radius, -radius)],
                circle_points(
                    w / 2.0 + radius * 2.0,
                    -radius,
                    radius,
                    20,
                    -180.0,
                    -270.0,
                    false,
                ),
                circle_points(
                    w / 2.0 + radius * 2.0,
                    radius,
                    radius,
                    20,
                    -90.0,
                    -180.0,
                    false,
                ),
                vec![(w / 2.0 + radius, h / 2.0)],
                circle_points(w / 2.0, h / 2.0, radius, 20, 0.0, 90.0, false),
            ]
            .into_iter()
            .flatten()
            .collect();
            let rect_points: Vec<(f64, f64)> = [
                vec![(-w / 2.0, -h / 2.0 - radius), (w / 2.0, -h / 2.0 - radius)],
                circle_points(w / 2.0, -h / 2.0, radius, 20, -90.0, 0.0, false),
                vec![(w / 2.0 + radius, -radius)],
                circle_points(
                    w / 2.0 + radius * 2.0,
                    -radius,
                    radius,
                    20,
                    -180.0,
                    -270.0,
                    false,
                ),
                circle_points(
                    w / 2.0 + radius * 2.0,
                    radius,
                    radius,
                    20,
                    -90.0,
                    -180.0,
                    false,
                ),
                vec![(w / 2.0 + radius, h / 2.0)],
                circle_points(w / 2.0, h / 2.0, radius, 20, 0.0, 90.0, false),
                vec![(w / 2.0, h / 2.0 + radius), (-w / 2.0, h / 2.0 + radius)],
            ]
            .into_iter()
            .flatten()
            .collect();
            (points, rect_points)
        } else {
            let points: Vec<(f64, f64)> = [
                circle_points(w / 2.0, -h / 2.0, radius, 30, -90.0, 0.0, true),
                vec![(-w / 2.0 - radius, radius)],
                circle_points(
                    w / 2.0 + radius * 2.0,
                    -radius,
                    radius,
                    20,
                    -180.0,
                    -270.0,
                    true,
                ),
                circle_points(
                    w / 2.0 + radius * 2.0,
                    radius,
                    radius,
                    20,
                    -90.0,
                    -180.0,
                    true,
                ),
                vec![(-w / 2.0 - radius, -h / 2.0)],
                circle_points(w / 2.0, h / 2.0, radius, 20, 0.0, 90.0, true),
            ]
            .into_iter()
            .flatten()
            .collect();
            let rect_points: Vec<(f64, f64)> = [
                vec![(w / 2.0, -h / 2.0 - radius), (-w / 2.0, -h / 2.0 - radius)],
                circle_points(w / 2.0, -h / 2.0, radius, 20, -90.0, 0.0, true),
                vec![(-w / 2.0 - radius, -radius)],
                circle_points(w / 2.0 + w * 0.1, -radius, radius, 20, -180.0, -270.0, true),
                circle_points(w / 2.0 + w * 0.1, radius, radius, 20, -90.0, -180.0, true),
                vec![(-w / 2.0 - radius, h / 2.0)],
                circle_points(w / 2.0, h / 2.0, radius, 20, 0.0, 90.0, true),
                vec![(-w / 2.0, h / 2.0 + radius), (w / 2.0, h / 2.0 + radius)],
            ]
            .into_iter()
            .flatten()
            .collect();
            (points, rect_points)
        };

        let brace_path = path_from_points(&points).trim_end_matches('Z').to_string();
        let rect_path = path_from_points(&rect_points);
        vec![
            CurlyBraceCommentPath {
                d: brace_path,
                visible: true,
            },
            CurlyBraceCommentPath {
                d: rect_path,
                visible: false,
            },
        ]
    };

    CurlyBraceCommentGeometry {
        group_tx,
        label_dx,
        paths,
    }
}

pub(in crate::svg::parity::flowchart::render::node) fn render_curly_brace_comment(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &mut super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let metrics = super::super::helpers::compute_node_label_metrics(
        ctx,
        Some(common.layout_node),
        label.text,
        label.label_type,
        common.node_classes,
        common.node_styles,
    );
    let geometry = curly_brace_comment_geometry(
        common.shape,
        metrics.width,
        metrics.height,
        ctx.node_padding,
    );
    label.dx = geometry.label_dx;

    let mut stroke_d = |d: &str| {
        super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
            roughjs_stroke_path_for_svg_path(
                d,
                common.stroke_color,
                common.stroke_width,
                common.stroke_dasharray,
                common.hand_drawn_seed,
            )
        })
        .unwrap_or_else(|| "M0,0".to_string())
    };

    let _ = write!(
        out,
        r##"<g class="text" transform="translate({},0)">"##,
        fmt(geometry.group_tx),
    );
    for path in geometry.paths {
        let d = stroke_d(&path.d);
        if path.visible {
            out.push_str("<g>");
        } else {
            out.push_str(r#"<g stroke-opacity="0">"#);
        }
        let _ = write!(
            out,
            r##"<path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/>"##,
            escape_attr(&d),
            escape_attr(common.stroke_color),
            fmt_display(common.stroke_width as f64),
            escape_attr(common.stroke_dasharray),
            escape_attr(common.style),
        );
        out.push_str("</g>");
    }
    out.push_str("</g>");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn union_translated_path_bounds(
        geometry: &CurlyBraceCommentGeometry,
    ) -> crate::svg::parity::path_bounds::SvgPathBounds {
        let mut out: Option<crate::svg::parity::path_bounds::SvgPathBounds> = None;
        for path in &geometry.paths {
            let mut pb = crate::svg::parity::path_bounds::svg_path_bounds_from_d(&path.d)
                .expect("path bounds");
            pb.min_x += geometry.group_tx;
            pb.max_x += geometry.group_tx;
            out = Some(match out {
                Some(mut acc) => {
                    acc.min_x = acc.min_x.min(pb.min_x);
                    acc.min_y = acc.min_y.min(pb.min_y);
                    acc.max_x = acc.max_x.max(pb.max_x);
                    acc.max_y = acc.max_y.max(pb.max_y);
                    acc
                }
                None => pb,
            });
        }
        out.expect("non-empty geometry")
    }

    #[test]
    fn brace_r_geometry_uses_label_box_not_updated_layout_box() {
        let label_w = 198.320_312_5;
        let label_h = 54.2;
        let padding = 15.0;
        let geometry = curly_brace_comment_geometry("brace-r", label_w, label_h, padding);

        assert_eq!(geometry.paths.len(), 2);
        assert!(geometry.paths[0].visible);
        assert!(!geometry.paths[1].visible);
        assert!((geometry.group_tx + 6.92).abs() < 1e-9);

        let bounds = union_translated_path_bounds(&geometry);
        let expected_width = label_w + padding + 2.0 * 6.92;
        assert!(((bounds.max_x - bounds.min_x) - expected_width).abs() < 1e-9);
    }
}
