//! Flowchart edge label renderer.

use super::super::*;
use super::root::flowchart_wrap_svg_text_lines;

// Mermaid's `createText(...)` defaults its `width` argument to 200. Flowchart edge labels call
// `createText(...)` without overriding that width, so keep edge label wrapping/max-width fixed at
// 200px (independent of `flowchart.wrappingWidth`).
const FLOWCHART_EDGE_LABEL_WRAP_WIDTH: f64 = 200.0;

pub(in crate::svg::parity) fn render_flowchart_edge_label(
    out: &mut String,
    ctx: &FlowchartRenderCtx<'_>,
    edge: &crate::flowchart::FlowEdge,
    origin_x: f64,
    origin_y: f64,
    edge_cache: Option<&FxHashMap<&str, FlowchartEdgePathCacheEntry>>,
) {
    let label_text = edge.label.as_deref().unwrap_or_default();
    let label_type = edge.label_type.as_deref().unwrap_or("text");
    let label_text_plain = flowchart_label_plain_text(label_text, label_type, ctx.edge_html_labels);
    let compiled_label_styles = flowchart_compile_styles(
        ctx.class_defs,
        &edge.classes,
        &ctx.default_edge_style,
        &edge.style,
    );
    let span_style_attr = OptionalStyleXmlAttr(compiled_label_styles.label_style.as_str());
    let div_style_prefix = crate::svg::parity::flowchart::style::flowchart_label_div_style_prefix(
        &compiled_label_styles,
        false,
    );

    fn js_round(v: f64, decimals: i32) -> f64 {
        if !v.is_finite() {
            return 0.0;
        }
        let factor = 10f64.powi(decimals);
        let x = v * factor;
        let r = (x + 0.5).floor() / factor;
        if r == -0.0 { 0.0 } else { r }
    }

    fn calc_label_position(
        points: &[crate::model::LayoutPoint],
    ) -> Option<crate::model::LayoutPoint> {
        // Mermaid `utils.calcLabelPosition(points)`:
        // - computes polyline total length
        // - traverses half distance along segments
        // - rounds interpolated coordinates to 5 decimals using JS `Math.round`.
        if points.is_empty() {
            return None;
        }
        if points.len() == 1 {
            return Some(points[0].clone());
        }

        let mut total = 0.0;
        for w in points.windows(2) {
            total += (w[1].x - w[0].x).hypot(w[1].y - w[0].y);
        }
        if !total.is_finite() || total <= 0.0 {
            return Some(points[0].clone());
        }

        let mut remaining = total / 2.0;
        for w in points.windows(2) {
            let a = &w[0];
            let b = &w[1];
            let seg = (b.x - a.x).hypot(b.y - a.y);
            if !seg.is_finite() || seg <= 0.0 {
                return Some(a.clone());
            }
            if seg < remaining {
                remaining -= seg;
                continue;
            }
            let ratio = remaining / seg;
            if ratio <= 0.0 {
                return Some(a.clone());
            }
            if ratio >= 1.0 {
                return Some(crate::model::LayoutPoint {
                    x: js_round(b.x, 5),
                    y: js_round(b.y, 5),
                });
            }
            return Some(crate::model::LayoutPoint {
                x: js_round((1.0 - ratio) * a.x + ratio * b.x, 5),
                y: js_round((1.0 - ratio) * a.y + ratio * b.y, 5),
            });
        }

        Some(points[0].clone())
    }

    fn fallback_midpoint(
        le: &crate::model::LayoutEdge,
        ctx: &FlowchartRenderCtx<'_>,
        origin_x: f64,
        origin_y: f64,
    ) -> (f64, f64) {
        let Some(p) = le.points.get(le.points.len() / 2) else {
            return (ctx.tx - origin_x, ctx.ty - origin_y);
        };
        (p.x + ctx.tx - origin_x, p.y + ctx.ty - origin_y)
    }

    if !ctx.edge_html_labels {
        if let Some(le) = ctx.layout_edges_by_id.get(edge.id.as_str()) {
            if let Some(lbl) = le.label.as_ref() {
                let x = lbl.x + ctx.tx - origin_x;
                let y = lbl.y + ctx.ty - origin_y;

                if label_text_plain.trim().is_empty() {
                    if !label_text.trim().is_empty() {
                        let _ = write!(
                            out,
                            r#"<g class="edgeLabel" transform="translate({},{})"><g class="label" data-id="{}" transform="translate(-2,-2)"><g><rect class="background" style="" x="-2" y="-2" width="4" height="4"/>"#,
                            fmt_display(x),
                            fmt_display(y),
                            escape_xml_display(&edge.id),
                        );
                        write_flowchart_svg_text_centered(out, "", true);
                        out.push_str("</g></g></g>");
                        return;
                    }
                } else {
                    let w = lbl.width.max(0.0);
                    let h = lbl.height.max(0.0);
                    let (dx, dy) = if w > 0.0 && h > 0.0 {
                        (-w / 2.0, -h / 2.0)
                    } else {
                        (0.0, 0.0)
                    };
                    let background_y =
                        crate::text::flowchart_svg_edge_label_background_y_px(&ctx.text_style);
                    let _ = write!(
                        out,
                        r#"<g class="edgeLabel" transform="translate({},{})"><g class="label" data-id="{}" transform="translate({},{})"><g><rect class="background" style="" x="-2" y="{}" width="{}" height="{}"/>"#,
                        fmt_display(x),
                        fmt_display(y),
                        escape_xml_display(&edge.id),
                        fmt_display(dx),
                        fmt_display(dy),
                        fmt_display(background_y),
                        fmt_display(w),
                        fmt_display(h)
                    );
                    let wrapped = flowchart_wrap_svg_text_lines(
                        ctx.measurer,
                        &label_text_plain,
                        &ctx.text_style,
                        Some(FLOWCHART_EDGE_LABEL_WRAP_WIDTH),
                        true,
                    )
                    .join("\n");
                    if label_type == "markdown" {
                        write_flowchart_svg_text_markdown_wrapped_centered(
                            out,
                            label_text,
                            true,
                            ctx.measurer,
                            &ctx.text_style,
                            Some(FLOWCHART_EDGE_LABEL_WRAP_WIDTH),
                        );
                    } else {
                        write_flowchart_svg_text_centered(out, &wrapped, true);
                    }
                    out.push_str("</g></g></g>");
                    return;
                }
            }

            if !label_text_plain.trim().is_empty() {
                let (x, y) = fallback_midpoint(le, ctx, origin_x, origin_y);
                let metrics = ctx.measurer.measure_wrapped(
                    &label_text_plain,
                    &ctx.text_style,
                    Some(FLOWCHART_EDGE_LABEL_WRAP_WIDTH),
                    crate::text::WrapMode::SvgLike,
                );
                let w = (metrics.width + 4.0).max(1.0);
                let h = (metrics.height + 4.0).max(1.0);
                let background_y =
                    crate::text::flowchart_svg_edge_label_background_y_px(&ctx.text_style);
                let _ = write!(
                    out,
                    r#"<g class="edgeLabel" transform="translate({},{})"><g class="label" data-id="{}" transform="translate({},{})"><g><rect class="background" style="" x="-2" y="{}" width="{}" height="{}"/>"#,
                    fmt_display(x),
                    fmt_display(y),
                    escape_xml_display(&edge.id),
                    fmt_display(-w / 2.0),
                    fmt_display(-h / 2.0),
                    fmt_display(background_y),
                    fmt_display(w),
                    fmt_display(h)
                );
                let wrapped = flowchart_wrap_svg_text_lines(
                    ctx.measurer,
                    &label_text_plain,
                    &ctx.text_style,
                    Some(FLOWCHART_EDGE_LABEL_WRAP_WIDTH),
                    true,
                )
                .join("\n");
                if label_type == "markdown" {
                    write_flowchart_svg_text_markdown_wrapped_centered(
                        out,
                        label_text,
                        true,
                        ctx.measurer,
                        &ctx.text_style,
                        Some(FLOWCHART_EDGE_LABEL_WRAP_WIDTH),
                    );
                } else {
                    write_flowchart_svg_text_centered(out, &wrapped, true);
                }
                out.push_str("</g></g></g>");
                return;
            }
        }

        let _ = write!(
            out,
            r#"<g class="edgeLabel"><g class="label" data-id="{}" transform="translate(0,0)">"#,
            escape_xml_display(&edge.id)
        );
        write_flowchart_svg_text_centered(out, "", false);
        out.push_str("</g></g>");
        return;
    }

    let label_html = if label_text.trim().is_empty() {
        String::new()
    } else {
        flowchart_edge_label_html(label_text, label_type, ctx.config, ctx.math_renderer)
    };

    if let Some(le) = ctx.layout_edges_by_id.get(edge.id.as_str()) {
        if let Some(lbl) = le.label.as_ref() {
            let mut x = lbl.x + ctx.tx - origin_x;
            let mut y = lbl.y + ctx.ty - origin_y;

            let cached_geom = edge_cache
                .and_then(|m| m.get(edge.id.as_str()))
                .filter(|entry| {
                    (entry.origin_x - origin_x).abs() <= 1e-9
                        && (entry.origin_y - origin_y).abs() <= 1e-9
                })
                .map(|entry| &entry.geom);
            if let Some(pos) = cached_geom.and_then(|geom| geom.label_position.as_ref()) {
                x = pos.x;
                y = pos.y;
            } else if le.to_cluster.is_some() || le.from_cluster.is_some() {
                // Fallback for callers that do not provide edge path cache: approximate Mermaid's
                // `updatedPath` cluster behavior from the clipped polyline.
                fn dedup_consecutive_points(
                    input: &[crate::model::LayoutPoint],
                ) -> Vec<crate::model::LayoutPoint> {
                    if input.len() <= 1 {
                        return input.to_vec();
                    }
                    const EPS: f64 = 1e-9;
                    let mut out: Vec<crate::model::LayoutPoint> = Vec::with_capacity(input.len());
                    for p in input {
                        if out.last().is_some_and(|prev| {
                            (prev.x - p.x).abs() <= EPS && (prev.y - p.y).abs() <= EPS
                        }) {
                            continue;
                        }
                        out.push(p.clone());
                    }
                    out
                }

                #[derive(Debug, Clone, Copy)]
                struct BoundaryNode {
                    x: f64,
                    y: f64,
                    width: f64,
                    height: f64,
                }

                fn outside_node(node: &BoundaryNode, point: &crate::model::LayoutPoint) -> bool {
                    let dx = (point.x - node.x).abs();
                    let dy = (point.y - node.y).abs();
                    let w = node.width / 2.0;
                    let h = node.height / 2.0;
                    dx >= w || dy >= h
                }

                fn rect_intersection(
                    node: &BoundaryNode,
                    outside_point: &crate::model::LayoutPoint,
                    inside_point: &crate::model::LayoutPoint,
                ) -> crate::model::LayoutPoint {
                    let x = node.x;
                    let y = node.y;

                    let w = node.width / 2.0;
                    let h = node.height / 2.0;

                    let q_abs = (outside_point.y - inside_point.y).abs();
                    let r_abs = (outside_point.x - inside_point.x).abs();

                    if (y - outside_point.y).abs() * w > (x - outside_point.x).abs() * h {
                        let q = if inside_point.y < outside_point.y {
                            outside_point.y - h - y
                        } else {
                            y - h - outside_point.y
                        };
                        let r = if q_abs == 0.0 {
                            0.0
                        } else {
                            (r_abs * q) / q_abs
                        };
                        let mut res = crate::model::LayoutPoint {
                            x: if inside_point.x < outside_point.x {
                                inside_point.x + r
                            } else {
                                inside_point.x - r_abs + r
                            },
                            y: if inside_point.y < outside_point.y {
                                inside_point.y + q_abs - q
                            } else {
                                inside_point.y - q_abs + q
                            },
                        };

                        if r.abs() <= 1e-9 {
                            res.x = outside_point.x;
                            res.y = outside_point.y;
                        }
                        if r_abs == 0.0 {
                            res.x = outside_point.x;
                        }
                        if q_abs == 0.0 {
                            res.y = outside_point.y;
                        }
                        return res;
                    }

                    let r = if inside_point.x < outside_point.x {
                        outside_point.x - w - x
                    } else {
                        x - w - outside_point.x
                    };
                    let q = if r_abs == 0.0 {
                        0.0
                    } else {
                        (q_abs * r) / r_abs
                    };
                    let mut ix = if inside_point.x < outside_point.x {
                        inside_point.x + r_abs - r
                    } else {
                        inside_point.x - r_abs + r
                    };
                    let mut iy = if inside_point.y < outside_point.y {
                        inside_point.y + q
                    } else {
                        inside_point.y - q
                    };

                    if r.abs() <= 1e-9 {
                        ix = outside_point.x;
                        iy = outside_point.y;
                    }
                    if r_abs == 0.0 {
                        ix = outside_point.x;
                    }
                    if q_abs == 0.0 {
                        iy = outside_point.y;
                    }

                    crate::model::LayoutPoint { x: ix, y: iy }
                }

                fn cut_path_at_intersect(
                    input: &[crate::model::LayoutPoint],
                    boundary: &BoundaryNode,
                ) -> Vec<crate::model::LayoutPoint> {
                    if input.is_empty() {
                        return Vec::new();
                    }
                    let mut out: Vec<crate::model::LayoutPoint> = Vec::new();
                    let mut last_point_outside = input[0].clone();
                    let mut is_inside = false;
                    const EPS: f64 = 1e-9;

                    for point in input {
                        if !outside_node(boundary, point) && !is_inside {
                            let inter = rect_intersection(boundary, &last_point_outside, point);
                            if !out.iter().any(|p| {
                                (p.x - inter.x).abs() <= EPS && (p.y - inter.y).abs() <= EPS
                            }) {
                                out.push(inter);
                            }
                            is_inside = true;
                        } else {
                            last_point_outside = point.clone();
                            if !is_inside {
                                out.push(point.clone());
                            }
                        }
                    }
                    out
                }

                fn boundary_for_cluster(
                    ctx: &FlowchartRenderCtx<'_>,
                    cluster_id: &str,
                    origin_x: f64,
                    origin_y: f64,
                ) -> Option<BoundaryNode> {
                    let n = ctx.layout_clusters_by_id.get(cluster_id)?;
                    Some(BoundaryNode {
                        x: n.x + ctx.tx - origin_x,
                        y: n.y + ctx.ty - origin_y,
                        width: n.width,
                        height: n.height,
                    })
                }

                let mut points: Vec<crate::model::LayoutPoint> = le
                    .points
                    .iter()
                    .map(|p| crate::model::LayoutPoint {
                        x: p.x + ctx.tx - origin_x,
                        y: p.y + ctx.ty - origin_y,
                    })
                    .collect();
                points = dedup_consecutive_points(&points);

                if let Some(tc) = le.to_cluster.as_deref() {
                    if let Some(boundary) = boundary_for_cluster(ctx, tc, origin_x, origin_y) {
                        points = cut_path_at_intersect(&points, &boundary);
                    }
                }
                if let Some(fc) = le.from_cluster.as_deref() {
                    if let Some(boundary) = boundary_for_cluster(ctx, fc, origin_x, origin_y) {
                        points.reverse();
                        points = cut_path_at_intersect(&points, &boundary);
                        points.reverse();
                    }
                }

                if let Some(pos) = calc_label_position(&points) {
                    x = pos.x;
                    y = pos.y;
                }
            }

            let w = lbl.width.max(0.0);
            let h = lbl.height.max(0.0);
            let wrapped_style = if w >= FLOWCHART_EDGE_LABEL_WRAP_WIDTH - 0.01 {
                format!(
                    "display: table; white-space: break-spaces; line-height: 1.5; max-width: {mw}px; text-align: center; width: {mw}px;",
                    mw = fmt_display(FLOWCHART_EDGE_LABEL_WRAP_WIDTH)
                )
            } else {
                "display: table-cell; white-space: nowrap; line-height: 1.5; max-width: 200px; text-align: center;".to_string()
            };
            let div_style = if div_style_prefix.is_empty() {
                wrapped_style
            } else {
                format!("{div_style_prefix}{wrapped_style}")
            };
            let _ = write!(
                out,
                r#"<g class="edgeLabel" transform="translate({},{})"><g class="label" data-id="{}" transform="translate({},{})"><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}"><span class="edgeLabel"{}>{}</span></div></foreignObject></g></g>"#,
                fmt_display(x),
                fmt_display(y),
                escape_xml_display(&edge.id),
                fmt_display(-w / 2.0),
                fmt_display(-h / 2.0),
                fmt_display(w),
                fmt_display(h),
                escape_xml_display(&div_style),
                span_style_attr,
                label_html
            );
            return;
        }

        if !label_text_plain.trim().is_empty() {
            let (x, y) = fallback_midpoint(le, ctx, origin_x, origin_y);
            let has_inline_style_tags = if label_type == "markdown" {
                false
            } else {
                let lower = label_text.to_ascii_lowercase();
                crate::text::flowchart_html_has_inline_style_tags(&lower)
            };

            let metrics = if label_type == "markdown" {
                crate::text::measure_markdown_with_flowchart_bold_deltas(
                    ctx.measurer,
                    label_text,
                    &ctx.text_style,
                    Some(FLOWCHART_EDGE_LABEL_WRAP_WIDTH),
                    ctx.edge_wrap_mode,
                )
            } else if has_inline_style_tags {
                crate::text::measure_html_with_flowchart_bold_deltas(
                    ctx.measurer,
                    label_text,
                    &ctx.text_style,
                    Some(FLOWCHART_EDGE_LABEL_WRAP_WIDTH),
                    ctx.edge_wrap_mode,
                )
            } else {
                ctx.measurer.measure_wrapped(
                    &label_text_plain,
                    &ctx.text_style,
                    Some(FLOWCHART_EDGE_LABEL_WRAP_WIDTH),
                    ctx.edge_wrap_mode,
                )
            };
            let w = metrics.width.max(1.0);
            let h = metrics.height.max(1.0);
            let wrapped_style = if w >= FLOWCHART_EDGE_LABEL_WRAP_WIDTH - 0.01 {
                format!(
                    "display: table; white-space: break-spaces; line-height: 1.5; max-width: {mw}px; text-align: center; width: {mw}px;",
                    mw = fmt_display(FLOWCHART_EDGE_LABEL_WRAP_WIDTH)
                )
            } else {
                "display: table-cell; white-space: nowrap; line-height: 1.5; max-width: 200px; text-align: center;".to_string()
            };
            let div_style = if div_style_prefix.is_empty() {
                wrapped_style
            } else {
                format!("{div_style_prefix}{wrapped_style}")
            };
            let _ = write!(
                out,
                r#"<g class="edgeLabel" transform="translate({},{})"><g class="label" data-id="{}" transform="translate({},{})"><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}"><span class="edgeLabel"{}>{}</span></div></foreignObject></g></g>"#,
                fmt_display(x),
                fmt_display(y),
                escape_xml_display(&edge.id),
                fmt_display(-w / 2.0),
                fmt_display(-h / 2.0),
                fmt_display(w.max(0.0)),
                fmt_display(h.max(0.0)),
                escape_xml_display(&div_style),
                span_style_attr,
                label_html
            );
            return;
        }
    }

    let _ = write!(
        out,
        r#"<g class="edgeLabel"><g class="label" data-id="{}" transform="translate(0,0)"><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}display: table-cell; white-space: nowrap; line-height: 1.5; max-width: 200px; text-align: center;"><span class="edgeLabel"{}></span></div></foreignObject></g></g>"#,
        escape_xml_display(&edge.id),
        escape_xml_display(&div_style_prefix),
        span_style_attr
    );
}
