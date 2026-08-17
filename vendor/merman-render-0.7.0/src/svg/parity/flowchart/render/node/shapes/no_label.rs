//! Flowchart v2 node shapes that do not emit a label group.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::util;

use super::super::geom::path_from_points;
use super::super::roughjs::{
    RoughRectSpec, roughjs_circle_path_d, roughjs_paths_for_rect, roughjs_paths_for_svg_path,
};

pub(in crate::svg::parity::flowchart::render::node) fn try_render_flowchart_v2_no_label(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) -> bool {
    match common.shape {
        // Flowchart v2 anchor: a tiny dot used as an invisible anchor node. Mermaid ignores
        // `node.label` and does not emit a label group.
        "anchor" => {
            let d =
                super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
                    roughjs_circle_path_d(2.0, common.hand_drawn_seed)
                })
                .unwrap_or_else(|| "M0,0".to_string());
            let _ = write!(
                out,
                r##"<g class="anchor" style=""><path d="{}" stroke="none" stroke-width="0" fill="black"/></g>"##,
                escape_attr(&d),
            );
            true
        }
        // Flowchart v2 "rendering-elements" aliases for state diagram start/end nodes.
        // Mermaid ignores `node.label` for these shapes and does not emit a label group.
        "sm-circ" | "small-circle" | "start" => {
            out.push_str(r#"<circle class="state-start" r="7" width="14" height="14"/>"#);
            true
        }
        "fr-circ" | "framed-circle" | "stop" => {
            let line_color = util::theme_color(ctx.config.as_value(), "lineColor", "#333333");
            let inner_fill =
                util::config_string(ctx.config.as_value(), &["themeVariables", "stateBorder"])
                    .unwrap_or_else(|| ctx.node_border_color.clone());

            let outer_d =
                super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
                    roughjs_circle_path_d(14.0, common.hand_drawn_seed)
                })
                .unwrap_or_else(|| "M0,0".to_string());
            let inner_d =
                super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
                    roughjs_circle_path_d(5.0, common.hand_drawn_seed)
                })
                .unwrap_or_else(|| "M0,0".to_string());

            let _ = write!(
                out,
                r##"<g class="outer-path"><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="2" fill="none" stroke-dasharray="{}" style="{}"/><g><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="2" fill="none" stroke-dasharray="{}" style="{}"/></g></g>"##,
                outer_d,
                escape_attr(common.fill_color),
                escape_attr(common.style),
                outer_d,
                escape_attr(&line_color),
                escape_attr(common.stroke_dasharray),
                escape_attr(common.style),
                inner_d,
                escape_attr(&inner_fill),
                escape_attr(common.style),
                inner_d,
                escape_attr(&inner_fill),
                escape_attr(common.stroke_dasharray),
                escape_attr(common.style),
            );
            true
        }
        // Flowchart v2 fork/join (no label; uses `lineColor` fill/stroke).
        "fork" | "join" => {
            // Mermaid inflates Dagre dimensions after `updateNodeBounds(...)` but does not
            // re-render the bar at the inflated size. Render the canonical shape dimensions.
            let (w, h) = if common.layout_node.width >= common.layout_node.height {
                (70.0, 10.0)
            } else {
                (10.0, 70.0)
            };
            let line_color = util::theme_color(ctx.config.as_value(), "lineColor", "#333333");
            let (fill_d, stroke_d) =
                super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
                    roughjs_paths_for_rect(RoughRectSpec {
                        x: -w / 2.0,
                        y: -h / 2.0,
                        w,
                        h,
                        fill: &line_color,
                        stroke: &line_color,
                        stroke_width: common.stroke_width,
                        seed: common.hand_drawn_seed,
                    })
                })
                .unwrap_or_else(|| ("M0,0".to_string(), "M0,0".to_string()));
            let _ = write!(
                out,
                r##"<g><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g>"##,
                fill_d,
                escape_attr(&line_color),
                escape_attr(common.style),
                stroke_d,
                escape_attr(&line_color),
                util::fmt_display(common.stroke_width as f64),
                escape_attr(common.stroke_dasharray),
                escape_attr(common.style),
            );
            true
        }
        // Flowchart v2 "rendering-elements" alias for state diagram choice pseudo-state.
        // Mermaid ignores `node.label` and does not emit a label group.
        "choice" => {
            // These path data strings match Mermaid's flowchart-v2 output (via the
            // rendering-elements/state pipeline) at 11.12.2.
            let fill_d = r#"M0 14 C3.0797827916219833 10.920217208378016, 6.159565583243967 7.840434416756033, 14 0 C9.445017992146312 -4.554982007853687, 4.890035984292625 -9.109964015707375, 0 -14 C-4.69590768981725 -9.30409231018275, -9.3918153796345 -4.608184620365501, -14 0 C-10.594632213003933 3.4053677869960666, -7.189264426007867 6.810735573992133, 0 14"#;
            let stroke_d = r#"M0 14 C2.800062938220799 11.1999370617792, 5.600125876441598 8.399874123558401, 14 0 M0 14 C3.0989264858886605 10.901073514111339, 6.197852971777321 7.802147028222679, 14 0 M14 0 C10.954967711679636 -3.045032288320363, 7.909935423359274 -6.090064576640726, 0 -14 M14 0 C10.242459432967006 -3.757540567032993, 6.484918865934014 -7.515081134065986, 0 -14 M0 -14 C-3.0194146709516647 -10.980585329048335, -6.038829341903329 -7.961170658096671, -14 0 M0 -14 C-5.262776161544025 -8.737223838455975, -10.52555232308805 -3.47444767691195, -14 0 M-14 0 C-9.98466955255717 4.01533044744283, -5.96933910511434 8.03066089488566, 0 14 M-14 0 C-8.907272248156367 5.092727751843632, -3.8145444963127364 10.185455503687264, 0 14"#;

            let _ = write!(
                out,
                r##"<g><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g>"##,
                fill_d,
                escape_attr(common.fill_color),
                escape_attr(common.style),
                stroke_d,
                escape_attr(common.stroke_color),
                util::fmt_display(common.stroke_width as f64),
                escape_attr(common.stroke_dasharray),
                escape_attr(common.style),
            );
            true
        }
        // Flowchart v2 lightning bolt (Communication link). Mermaid clears `node.label` and does
        // not emit a label group.
        "bolt" | "com-link" | "lightning-bolt" => {
            // Mermaid uses `width = max(35, node.width)` and `height = max(35, node.height)`,
            // then draws a 2*height tall bolt and translates it by `(-width/2, -height)`.
            let width = common.layout_node.width.max(35.0);
            let height = (common.layout_node.height / 2.0).max(35.0);
            let gap = 7.0;

            let points: Vec<(f64, f64)> = vec![
                (width, 0.0),
                (0.0, height + gap / 2.0),
                (width - 2.0 * gap, height + gap / 2.0),
                (0.0, 2.0 * height),
                (width, height - gap / 2.0),
                (2.0 * gap, height - gap / 2.0),
            ];
            let path_data = path_from_points(&points);
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
                r#"<g class="outer-path" transform="translate({},{})"><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g>"#,
                util::fmt(-width / 2.0),
                util::fmt(-height),
                escape_attr(&fill_d),
                escape_attr(common.fill_color),
                escape_attr(common.style),
                escape_attr(&stroke_d),
                escape_attr(common.stroke_color),
                util::fmt_display(common.stroke_width as f64),
                escape_attr(common.stroke_dasharray),
                escape_attr(common.style),
            );
            true
        }
        // Flowchart v2 filled circle (junction). Mermaid clears `node.label` and does not emit a
        // label group. Note that even in non-handDrawn mode Mermaid still uses RoughJS circle
        // paths (roughness=0), which have a slightly asymmetric bbox in Chromium.
        "f-circ" | "junction" | "filled-circle" => {
            let border =
                util::config_string(ctx.config.as_value(), &["themeVariables", "nodeBorder"])
                    .unwrap_or_else(|| ctx.node_border_color.clone());

            let effective_style: std::borrow::Cow<'_, str> = if common.style.trim().is_empty() {
                format!("fill: {border} !important;").into()
            } else {
                common.style.into()
            };

            let d =
                super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
                    roughjs_circle_path_d(14.0, common.hand_drawn_seed)
                })
                .unwrap_or_else(|| "M0,0".into());
            let _ = write!(
                out,
                r##"<g><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g>"##,
                escape_attr(&d),
                escape_attr(common.fill_color),
                escape_attr(effective_style.as_ref()),
                escape_attr(&d),
                escape_attr(common.stroke_color),
                util::fmt_display(common.stroke_width as f64),
                escape_attr(common.stroke_dasharray),
                escape_attr(effective_style.as_ref()),
            );
            true
        }
        // Flowchart v2 crossed circle (summary). Mermaid clears `node.label` and does not emit a
        // label group.
        "cross-circ" | "summary" | "crossed-circle" => {
            // Mermaid uses `radius = max(30, node.width)` before `updateNodeBounds(...)`. In
            // practice `node.width` is usually unset here, so radius=30.
            let radius = 30.0;

            let circle_d =
                super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
                    roughjs_circle_path_d(radius * 2.0, common.hand_drawn_seed)
                })
                .unwrap_or_else(|| "M0,0".into());

            // Port of Mermaid `createLine(r)` in `crossedCircle.ts`.
            let x_axis_45 = (std::f64::consts::PI / 4.0).cos();
            let y_axis_45 = (std::f64::consts::PI / 4.0).sin();
            let point_q1 = (radius * x_axis_45, radius * y_axis_45);
            let point_q2 = (-radius * x_axis_45, radius * y_axis_45);
            let point_q3 = (-radius * x_axis_45, -radius * y_axis_45);
            let point_q4 = (radius * x_axis_45, -radius * y_axis_45);
            let line_path = format!(
                "M {},{} L {},{} M {},{} L {},{}",
                point_q2.0,
                point_q2.1,
                point_q4.0,
                point_q4.1,
                point_q1.0,
                point_q1.1,
                point_q3.0,
                point_q3.1
            );
            let (line_fill_d, line_stroke_d) =
                super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
                    roughjs_paths_for_svg_path(
                        &line_path,
                        common.fill_color,
                        common.stroke_color,
                        common.stroke_width,
                        common.stroke_dasharray,
                        common.hand_drawn_seed,
                    )
                })
                .unwrap_or_else(|| ("".to_string(), "M0,0".to_string()));

            let _ = write!(
                out,
                r##"<g class="outer-path"><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/><g><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g></g>"##,
                escape_attr(&circle_d),
                escape_attr(common.fill_color),
                escape_attr(common.style),
                escape_attr(&circle_d),
                escape_attr(common.stroke_color),
                util::fmt_display(common.stroke_width as f64),
                escape_attr(common.stroke_dasharray),
                escape_attr(common.style),
                escape_attr(&line_fill_d),
                escape_attr(common.fill_color),
                escape_attr(common.style),
                escape_attr(&line_stroke_d),
                escape_attr(common.stroke_color),
                util::fmt_display(common.stroke_width as f64),
                escape_attr(common.stroke_dasharray),
                escape_attr(common.style),
            );
            true
        }
        _ => false,
    }
}
