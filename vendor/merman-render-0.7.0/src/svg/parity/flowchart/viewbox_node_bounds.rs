//! Flowchart node rendered-bounds preparation for final viewBox calculation.

use super::render::node::geom::{generate_circle_points, generate_full_sine_wave_points};
use super::*;

fn union_svg_path_bounds(paths: &[&str]) -> Option<crate::svg::parity::path_bounds::SvgPathBounds> {
    let mut bounds: Option<crate::svg::parity::path_bounds::SvgPathBounds> = None;
    for d in paths {
        let Some(pb) = crate::svg::parity::path_bounds::svg_path_bounds_from_d(d) else {
            continue;
        };
        bounds = Some(match bounds {
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
    bounds
}

fn rough_svg_path_bounds(
    path_data: &str,
) -> Option<crate::svg::parity::path_bounds::SvgPathBounds> {
    let (fill_d, stroke_d) =
        crate::svg::parity::flowchart::render::node::roughjs::roughjs_paths_for_svg_path(
            path_data, "#000", "#000", 1.3, "0 0", 0,
        )?;
    union_svg_path_bounds(&[fill_d.as_str(), stroke_d.as_str()])
}

fn rough_stroke_svg_path_bounds(
    path_data: &str,
) -> Option<crate::svg::parity::path_bounds::SvgPathBounds> {
    let stroke_d =
        crate::svg::parity::flowchart::render::node::roughjs::roughjs_stroke_path_for_svg_path(
            path_data, "#000", 1.3, "0 0", 0,
        )?;
    crate::svg::parity::path_bounds::svg_path_bounds_from_d(&stroke_d)
}

fn measure_flowchart_layout_node_label(
    ctx: &FlowchartRenderCtx<'_>,
    n: &LayoutNode,
) -> Option<crate::text::TextMetrics> {
    let flow_node = ctx.nodes_by_id.get(n.id.as_str())?;
    let label = flow_node.label.as_deref().unwrap_or("");
    let label_type = flow_node
        .label_type
        .as_deref()
        .unwrap_or(if ctx.node_html_labels { "html" } else { "text" });
    let label_base_style = if ctx.node_wrap_mode == crate::text::WrapMode::HtmlLike {
        &ctx.html_label_text_style
    } else {
        &ctx.text_style
    };
    let node_text_style = crate::flowchart::flowchart_effective_text_style_for_node_classes(
        label_base_style,
        ctx.class_defs,
        &flow_node.classes,
        &flow_node.styles,
    );
    let node_font_style = crate::flowchart::flowchart_effective_font_style_for_node_classes(
        ctx.class_defs,
        &flow_node.classes,
        &flow_node.styles,
    );
    Some(crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: ctx.measurer,
            raw_label: label,
            label_type,
            style: &node_text_style,
            max_width_px: Some(ctx.wrapping_width),
            wrap_mode: ctx.node_wrap_mode,
            config: ctx.config,
            math_renderer: ctx.math_renderer,
            preserve_string_whitespace_height: ctx.node_html_labels && ctx.edge_html_labels,
            whole_label_font_style: node_font_style.as_deref(),
        },
    ))
}

fn layout_node_metrics_or_zero(
    ctx: &FlowchartRenderCtx<'_>,
    n: &LayoutNode,
) -> crate::text::TextMetrics {
    if let (Some(width), Some(height)) = (n.label_width, n.label_height) {
        return crate::text::TextMetrics {
            width,
            height,
            line_count: 0,
        };
    }
    measure_flowchart_layout_node_label(ctx, n).unwrap_or(crate::text::TextMetrics {
        width: 0.0,
        height: 0.0,
        line_count: 0,
    })
}

fn layout_node_label_size_or_zero(ctx: &FlowchartRenderCtx<'_>, n: &LayoutNode) -> (f64, f64) {
    let metrics = layout_node_metrics_or_zero(ctx, n);
    (metrics.width, metrics.height)
}

fn layout_node_label_width_if_known(ctx: &FlowchartRenderCtx<'_>, n: &LayoutNode) -> Option<f64> {
    n.label_width
        .or_else(|| measure_flowchart_layout_node_label(ctx, n).map(|metrics| metrics.width))
}

pub(in crate::svg::parity::flowchart) fn include_flowchart_node_rendered_bounds<'data, F>(
    ctx: &FlowchartRenderCtx<'data>,
    nodes: &[LayoutNode],
    subgraph_title_y_shift: f64,
    effective_parent_for_id: &F,
    include_rect: &mut impl FnMut(f64, f64, f64, f64),
) where
    F: Fn(&str) -> Option<&'data str>,
{
    let node_padding = ctx.node_padding;

    let y_offset_for_root = |root: Option<&str>| -> f64 {
        if root.is_some() && subgraph_title_y_shift.abs() >= 1e-9 {
            -subgraph_title_y_shift
        } else {
            0.0
        }
    };

    for n in nodes {
        let is_empty_subgraph_node = ctx
            .subgraphs_by_id
            .get(n.id.as_str())
            .is_some_and(|sg| sg.nodes.is_empty());
        let root = if n.is_cluster && ctx.recursive_clusters.contains(n.id.as_str()) {
            Some(n.id.as_str())
        } else {
            effective_parent_for_id(&n.id)
        };
        let y_off = y_offset_for_root(root);
        if n.is_cluster || ctx.node_dom_index.contains_key(n.id.as_str()) || is_empty_subgraph_node
        {
            let mut left_hw = n.width / 2.0;
            let mut right_hw = left_hw;
            let mut top_hh = n.height / 2.0;
            let mut bottom_hh = top_hh;
            if !n.is_cluster {
                if let Some(shape) = ctx
                    .nodes_by_id
                    .get(n.id.as_str())
                    .and_then(|node| node.layout_shape.as_deref())
                {
                    // Mermaid's flowchart-v2 rhombus node renderer offsets the polygon by
                    // `(-width/2 + 0.5, height/2)` so the diamond outline stays on the same
                    // pixel lattice as other nodes. This makes the DOM bbox slightly asymmetric
                    // around the node center and affects the root `getBBox()` width.
                    if shape == "diamond" || shape == "diam" || shape == "rhombus" {
                        left_hw = (left_hw - 0.5).max(0.0);
                        right_hw += 0.5;
                    }

                    // Mermaid `stateEnd.ts` renders the framed-circle using a RoughJS ellipse
                    // path with a slightly asymmetric bbox in Chromium.
                    if matches!(shape, "fr-circ" | "framed-circle" | "stop") {
                        left_hw = 7.0;
                        right_hw = (n.width - 7.0).max(0.0);
                    }

                    // Mermaid `filledCircle.ts` uses a RoughJS circle path (roughness=0) whose
                    // bbox is slightly asymmetric.
                    if matches!(shape, "f-circ") {
                        left_hw = 7.0;
                        right_hw = (n.width - 7.0).max(0.0);
                    }

                    // Mermaid `crossedCircle.ts` uses a RoughJS circle path with radius=30; its
                    // bbox is slightly asymmetric in Chromium.
                    if matches!(shape, "cross-circ" | "summary" | "crossed-circle") {
                        left_hw = 30.0;
                        right_hw = (n.width - 30.0).max(0.0);
                        top_hh = 30.0;
                        bottom_hh = 30.0;
                    }

                    // Mermaid `curvedTrapezoid.ts` draws its rough path from the
                    // "theoretical" text+padding width, but Dagre uses the
                    // `updateNodeBounds(...)` bbox which can be slightly narrower.
                    if matches!(shape, "curv-trap" | "display" | "curved-trapezoid") {
                        if let Some(label_w) = layout_node_label_width_if_known(ctx, n) {
                            let pre_w = ((label_w + 2.0 * node_padding) * 1.25).max(20.0);
                            left_hw = pre_w / 2.0;
                            right_hw = (n.width - left_hw).max(0.0);
                        }
                    }

                    // Mermaid `waveEdgedRectangle.ts` (document) stores Dagre dimensions from
                    // `updateNodeBounds(...)`, but the final root viewport comes from the rendered
                    // RoughJS path bbox. Rebuild that bbox directly.
                    if matches!(shape, "doc" | "document") {
                        let (label_w, label_h) = layout_node_label_size_or_zero(ctx, n);
                        let w = (label_w + 2.0 * node_padding).max(0.0);
                        let h = (label_h + 2.0 * node_padding).max(0.0);
                        let wave_amplitude = h / 8.0;
                        let final_h = h + wave_amplitude;
                        let extra_w = ((14.0 - w).max(0.0)) / 2.0;
                        let mut points: Vec<(f64, f64)> = Vec::new();
                        points.push((-w / 2.0 - extra_w, final_h / 2.0));
                        points.extend(generate_full_sine_wave_points(
                            -w / 2.0 - extra_w,
                            final_h / 2.0,
                            w / 2.0 + extra_w,
                            final_h / 2.0,
                            wave_amplitude,
                            0.8,
                        ));
                        points.push((w / 2.0 + extra_w, -final_h / 2.0));
                        points.push((-w / 2.0 - extra_w, -final_h / 2.0));

                        let path_data =
                            crate::svg::parity::roughjs_common::closed_path_d_from_points(&points);
                        if let Some(pb) = rough_svg_path_bounds(&path_data) {
                            let y_shift = -wave_amplitude / 2.0;
                            left_hw = (-pb.min_x).max(0.0);
                            right_hw = pb.max_x.max(0.0);
                            top_hh = (-(pb.min_y + y_shift)).max(0.0);
                            bottom_hh = (pb.max_y + y_shift).max(0.0);
                        }
                    }

                    // Mermaid `linedWaveEdgedRect.ts` follows the same split as the other wave
                    // document shapes: Dagre uses the post-`updateNodeBounds(...)` dimensions,
                    // while the rendered root bbox comes from the original label-box path.
                    if matches!(shape, "lin-doc" | "lined-document") {
                        let (label_w, label_h) = layout_node_label_size_or_zero(ctx, n);
                        let w = (label_w + 2.0 * node_padding).max(0.0);
                        let h = (label_h + 2.0 * node_padding).max(0.0);
                        let wave_amplitude = h / 8.0;
                        let final_h = h + wave_amplitude;
                        let extra = (w / 2.0) * 0.1;
                        let mut points: Vec<(f64, f64)> = Vec::new();
                        points.push((-w / 2.0 - extra, -final_h / 2.0));
                        points.push((-w / 2.0 - extra, final_h / 2.0));
                        points.extend(generate_full_sine_wave_points(
                            -w / 2.0 - extra,
                            final_h / 2.0,
                            w / 2.0 + extra,
                            final_h / 2.0,
                            wave_amplitude,
                            0.8,
                        ));
                        points.push((w / 2.0 + extra, -final_h / 2.0));
                        points.push((-w / 2.0 - extra, -final_h / 2.0));
                        points.push((-w / 2.0, -final_h / 2.0));
                        points.push((-w / 2.0, (final_h / 2.0) * 1.1));
                        points.push((-w / 2.0, -final_h / 2.0));

                        let path_data =
                            crate::svg::parity::roughjs_common::closed_path_d_from_points(&points);
                        if let Some(pb) = rough_svg_path_bounds(&path_data) {
                            let y_shift = -wave_amplitude / 2.0;
                            left_hw = (-pb.min_x).max(0.0);
                            right_hw = pb.max_x.max(0.0);
                            top_hh = (-(pb.min_y + y_shift)).max(0.0);
                            bottom_hh = (pb.max_y + y_shift).max(0.0);
                        }
                    }

                    // Mermaid `taggedWaveEdgedRectangle.ts` (tagged-document) renders from the
                    // base label box, then `updateNodeBounds(...)` stores a slightly shorter outer
                    // bbox. The rendered wave is also vertically asymmetric.
                    if matches!(shape, "tag-doc" | "tagged-document") {
                        let (label_w, label_h) = layout_node_label_size_or_zero(ctx, n);
                        let w = (label_w + 2.0 * node_padding).max(0.0);
                        let h = (label_h + 2.0 * node_padding).max(0.0);
                        let wave_amplitude = h / 8.0;
                        let final_h = h + wave_amplitude;
                        let extra = (w / 2.0) * 0.1;
                        let tag_width = 0.2 * w;
                        let tag_height = 0.2 * h;

                        let mut wave_points: Vec<(f64, f64)> = Vec::new();
                        wave_points.push((-w / 2.0 - extra, final_h / 2.0));
                        wave_points.extend(generate_full_sine_wave_points(
                            -w / 2.0 - extra,
                            final_h / 2.0,
                            w / 2.0 + extra,
                            final_h / 2.0,
                            wave_amplitude,
                            0.8,
                        ));
                        wave_points.push((w / 2.0 + extra, -final_h / 2.0));
                        wave_points.push((-w / 2.0 - extra, -final_h / 2.0));

                        let x = -w / 2.0 + extra;
                        let y = -final_h / 2.0 - tag_height * 0.4;
                        let mut tag_points: Vec<(f64, f64)> = Vec::new();
                        tag_points.push((x + w - tag_width, (y + h) * 1.3));
                        tag_points.push((x + w, y + h - tag_height));
                        tag_points.push((x + w, (y + h) * 0.9));
                        tag_points.extend(generate_full_sine_wave_points(
                            x + w,
                            (y + h) * 1.25,
                            x + w - tag_width,
                            (y + h) * 1.3,
                            -h * 0.02,
                            0.5,
                        ));

                        let wave_path_data =
                            crate::svg::parity::roughjs_common::closed_path_d_from_points(
                                &wave_points,
                            );
                        let tag_path_data =
                            crate::svg::parity::roughjs_common::closed_path_d_from_points(
                                &tag_points,
                            );
                        let mut bounds: Option<crate::svg::parity::path_bounds::SvgPathBounds> =
                            None;
                        for pb in [
                            rough_svg_path_bounds(&wave_path_data),
                            rough_svg_path_bounds(&tag_path_data),
                        ]
                        .into_iter()
                        .flatten()
                        {
                            bounds = Some(match bounds {
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
                        if let Some(pb) = bounds {
                            let y_shift = -wave_amplitude / 2.0;
                            left_hw = (-pb.min_x).max(0.0);
                            right_hw = pb.max_x.max(0.0);
                            top_hh = (-(pb.min_y + y_shift)).max(0.0);
                            bottom_hh = (pb.max_y + y_shift).max(0.0);
                        }
                    }

                    // Mermaid computes the root viewport from the rendered DOM bbox. Curly
                    // brace/comment shapes emit narrow RoughJS stroke paths plus an invisible
                    // path; using the inflated Dagre `node.width / 2` keeps a phantom edge.
                    if matches!(
                        shape,
                        "comment" | "brace" | "brace-l" | "brace-r" | "braces"
                    ) {
                        let metrics = layout_node_metrics_or_zero(ctx, n);
                        let geometry = crate::svg::parity::flowchart::render::node::shapes::curly_brace_comment_geometry(
                            shape,
                            metrics.width,
                            metrics.height,
                            node_padding,
                        );
                        let mut bounds: Option<crate::svg::parity::path_bounds::SvgPathBounds> =
                            None;
                        for path in geometry.paths {
                            if let Some(mut pb) = rough_stroke_svg_path_bounds(&path.d) {
                                pb.min_x += geometry.group_tx;
                                pb.max_x += geometry.group_tx;
                                bounds = Some(match bounds {
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
                        }
                        if let Some(pb) = bounds {
                            left_hw = (-pb.min_x).max(0.0);
                            right_hw = pb.max_x.max(0.0);
                            top_hh = (-pb.min_y).max(0.0);
                            bottom_hh = pb.max_y.max(0.0);
                        }
                    }

                    // Mermaid `forkJoin.ts` inflates Dagre dimensions but the rendered bar
                    // remains `70x10` (or `10x70` for LR).
                    if matches!(shape, "fork" | "join") {
                        if n.width >= n.height {
                            left_hw = 35.0;
                            right_hw = 35.0;
                            top_hh = 5.0;
                            bottom_hh = 5.0;
                        } else {
                            left_hw = 5.0;
                            right_hw = 5.0;
                            top_hh = 35.0;
                            bottom_hh = 35.0;
                        }
                    }

                    // Mermaid `multiWaveEdgedRectangle.ts` emits a bottom sine wave and then
                    // translates the whole group upward by `waveAmplitude / 2`.
                    if matches!(shape, "docs" | "documents" | "st-doc" | "stacked-document") {
                        let (label_w, label_h) = layout_node_label_size_or_zero(ctx, n);
                        let w = label_w + 2.0 * node_padding;
                        let h = label_h + 3.0 * node_padding;
                        let wave_amplitude = h / 8.0;
                        let final_h = h + wave_amplitude / 2.0;
                        let rect_offset = 10.0;
                        let y = -final_h / 2.0;
                        let baseline_y = y + final_h + rect_offset;

                        let mut max_wave_y = baseline_y;
                        let delta_x = w;
                        let cycle_length = if delta_x.abs() < 1e-9 {
                            delta_x
                        } else {
                            delta_x / 0.8
                        };
                        let frequency = if cycle_length.abs() < 1e-9 {
                            0.0
                        } else {
                            (2.0 * std::f64::consts::PI) / cycle_length
                        };
                        for i in 0..=50 {
                            let t = i as f64 / 50.0;
                            let x = t * delta_x;
                            let wave_y = baseline_y + wave_amplitude * (frequency * x).sin();
                            max_wave_y = max_wave_y.max(wave_y);
                        }

                        let top_y = y - rect_offset - wave_amplitude / 2.0;
                        let bottom_y = max_wave_y - wave_amplitude / 2.0;
                        top_hh = -top_y;
                        bottom_hh = bottom_y;
                        if left_hw == right_hw {
                            left_hw = w / 2.0 + rect_offset;
                            right_hw = left_hw;
                        }
                    }

                    if matches!(shape, "delay" | "half-rounded-rectangle") {
                        let label_w = n.label_width.unwrap_or(0.0);
                        let label_h = n.label_height.unwrap_or(0.0);
                        let w = (label_w + 2.0 * node_padding).max(15.0);
                        let h = (label_h + 2.0 * node_padding).max(10.0);
                        let radius = h / 2.0;
                        let mut points: Vec<(f64, f64)> = Vec::new();
                        points.push((-w / 2.0, -h / 2.0));
                        points.push((w / 2.0 - radius, -h / 2.0));
                        points.extend(generate_circle_points(
                            -w / 2.0 + radius,
                            0.0,
                            radius,
                            50,
                            90.0,
                            270.0,
                        ));
                        points.push((w / 2.0 - radius, h / 2.0));
                        points.push((-w / 2.0, h / 2.0));

                        let path_data =
                            crate::svg::parity::roughjs_common::closed_path_d_from_points(&points);
                        if let Some(pb) = rough_svg_path_bounds(&path_data) {
                            left_hw = (-pb.min_x).max(0.0);
                            right_hw = pb.max_x.max(0.0);
                            top_hh = (-pb.min_y).max(0.0);
                            bottom_hh = pb.max_y.max(0.0);
                        }
                    }

                    if matches!(shape, "notch-pent" | "loop-limit" | "notched-pentagon") {
                        let label_w = n.label_width.unwrap_or(0.0);
                        let label_h = n.label_height.unwrap_or(0.0);
                        let w = (label_w + 2.0 * node_padding).max(60.0);
                        let h = (label_h + 2.0 * node_padding).max(20.0);
                        let points = vec![
                            ((-w / 2.0) * 0.8, -h / 2.0),
                            ((w / 2.0) * 0.8, -h / 2.0),
                            (w / 2.0, (-h / 2.0) * 0.6),
                            (w / 2.0, h / 2.0),
                            (-w / 2.0, h / 2.0),
                            (-w / 2.0, (-h / 2.0) * 0.6),
                        ];
                        let path_data =
                            crate::svg::parity::roughjs_common::closed_path_d_from_points(&points);
                        if let Some(pb) = rough_svg_path_bounds(&path_data) {
                            left_hw = (-pb.min_x).max(0.0);
                            right_hw = pb.max_x.max(0.0);
                            top_hh = (-pb.min_y).max(0.0);
                            bottom_hh = pb.max_y.max(0.0);
                        }
                    }
                }
            }
            include_rect(
                n.x - left_hw,
                n.y + y_off - top_hh,
                n.x + right_hw,
                n.y + y_off + bottom_hh,
            );
        } else {
            include_rect(n.x, n.y + y_off, n.x + n.width, n.y + y_off + n.height);
        }
    }
}
