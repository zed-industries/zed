use super::*;
use crate::generated::state_text_overrides_11_12_2 as state_text_overrides;

#[derive(Debug, Clone, Copy)]
struct StateEdgeBoundaryNode {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

fn state_edge_dedup_consecutive_points(
    input: &[crate::model::LayoutPoint],
) -> Vec<crate::model::LayoutPoint> {
    if input.len() <= 1 {
        return input.to_vec();
    }
    const EPS: f64 = 1e-9;
    let mut out: Vec<crate::model::LayoutPoint> = Vec::with_capacity(input.len());
    for p in input {
        if out
            .last()
            .is_some_and(|prev| (prev.x - p.x).abs() <= EPS && (prev.y - p.y).abs() <= EPS)
        {
            continue;
        }
        out.push(p.clone());
    }
    out
}

fn state_edge_outside_node(
    node: &StateEdgeBoundaryNode,
    point: &crate::model::LayoutPoint,
) -> bool {
    let dx = (point.x - node.x).abs();
    let dy = (point.y - node.y).abs();
    let w = node.width / 2.0;
    let h = node.height / 2.0;
    dx >= w || dy >= h
}

fn state_edge_rect_intersection(
    node: &StateEdgeBoundaryNode,
    inside_point: &crate::model::LayoutPoint,
    outside_point: &crate::model::LayoutPoint,
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

fn state_edge_cut_path_at_intersect(
    input: &[crate::model::LayoutPoint],
    boundary: &StateEdgeBoundaryNode,
) -> Vec<crate::model::LayoutPoint> {
    if input.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<crate::model::LayoutPoint> = Vec::new();
    let mut last_point_outside = input[0].clone();
    let mut is_inside = false;
    const EPS: f64 = 1e-9;

    for point in input {
        if !state_edge_outside_node(boundary, point) && !is_inside {
            // Mermaid's dagre-wrapper cuts an edge as it *enters* a cluster boundary.
            // `state_edge_rect_intersection` expects the point *inside* the rectangle first.
            let inter = state_edge_rect_intersection(boundary, point, &last_point_outside);
            if !out
                .iter()
                .any(|p| (p.x - inter.x).abs() <= EPS && (p.y - inter.y).abs() <= EPS)
            {
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

fn state_edge_boundary_for_cluster(
    ctx: &StateRenderCtx<'_>,
    cluster_id: &str,
    ox: f64,
    oy: f64,
) -> Option<StateEdgeBoundaryNode> {
    let mut resolved = cluster_id;
    if !ctx.layout_clusters_by_id.contains_key(resolved) {
        // Mermaid's state diagram edges sometimes annotate cluster endpoints as `state-<id>-<n>`
        // while the cluster itself is keyed by `<id>`.
        if let Some(rest) = resolved.strip_prefix("state-") {
            if let Some((base, suffix)) = rest.rsplit_once('-') {
                if !base.is_empty()
                    && !suffix.is_empty()
                    && suffix.bytes().all(|b| b.is_ascii_digit())
                {
                    resolved = base;
                }
            }
        }
    }

    let n = ctx.layout_clusters_by_id.get(resolved).copied()?;
    Some(StateEdgeBoundaryNode {
        x: n.x - ox,
        y: n.y - oy,
        width: n.width,
        height: n.height,
    })
}

fn state_marker_offset_for(arrow_type_end: Option<&str>) -> Option<f64> {
    match arrow_type_end {
        Some("arrow_barb_neo") => Some(5.5),
        _ => None,
    }
}

fn state_line_with_end_marker_offset_points(
    input: &[crate::model::LayoutPoint],
    arrow_type_end: Option<&str>,
) -> Vec<crate::model::LayoutPoint> {
    fn calculate_delta_and_angle(
        a: &crate::model::LayoutPoint,
        b: &crate::model::LayoutPoint,
    ) -> (f64, f64, f64) {
        let delta_x = b.x - a.x;
        let delta_y = b.y - a.y;
        let angle = (delta_y / delta_x).atan();
        (angle, delta_x, delta_y)
    }

    let Some(end_marker_height) = state_marker_offset_for(arrow_type_end) else {
        return input.to_vec();
    };
    if input.len() < 2 {
        return input.to_vec();
    }

    let start = &input[0];
    let end = &input[input.len() - 1];
    let x_direction_is_left = start.x < end.x;
    let y_direction_is_down = start.y < end.y;
    let extra_room = 1.0;

    let mut out = Vec::with_capacity(input.len());
    for (idx, point) in input.iter().enumerate() {
        let mut offset_x = 0.0;
        let mut offset_y = 0.0;

        if idx == input.len() - 1 {
            let (angle, delta_x, delta_y) =
                calculate_delta_and_angle(&input[input.len() - 1], &input[input.len() - 2]);
            offset_x = end_marker_height * angle.cos() * if delta_x >= 0.0 { 1.0 } else { -1.0 };
            offset_y =
                end_marker_height * angle.sin().abs() * if delta_y >= 0.0 { 1.0 } else { -1.0 };
        }

        let diff_x = (point.x - end.x).abs();
        let diff_y = (point.y - end.y).abs();
        if diff_x < end_marker_height && diff_x > 0.0 && diff_y < end_marker_height {
            let mut adjustment = end_marker_height + extra_room - diff_x;
            adjustment *= if !x_direction_is_left { -1.0 } else { 1.0 };
            offset_x -= adjustment;
        }
        if diff_y < end_marker_height && diff_y > 0.0 && diff_x < end_marker_height {
            let mut adjustment = end_marker_height + extra_room - diff_y;
            adjustment *= if !y_direction_is_down { -1.0 } else { 1.0 };
            offset_y -= adjustment;
        }

        out.push(crate::model::LayoutPoint {
            x: point.x + offset_x,
            y: point.y + offset_y,
        });
    }

    out
}

fn state_edge_prepare_points(
    ctx: &StateRenderCtx<'_>,
    le: &crate::model::LayoutEdge,
    edge_id: &str,
    arrow_type_end: Option<&str>,
    origin_x: f64,
    origin_y: f64,
) -> (
    Vec<crate::model::LayoutPoint>,
    Vec<crate::model::LayoutPoint>,
) {
    let mut local_points: Vec<crate::model::LayoutPoint> = Vec::new();
    for p in &le.points {
        local_points.push(crate::model::LayoutPoint {
            x: p.x - origin_x,
            y: p.y - origin_y,
        });
    }

    let is_cyclic_special = edge_id.contains("-cyclic-special-");
    let mut points_for_curve = if is_cyclic_special {
        state_edge_dedup_consecutive_points(&local_points)
    } else {
        local_points.clone()
    };

    // Match Mermaid `dagre-wrapper/edges.js insertEdge`: cut the path at cluster boundaries when the
    // edge connects to a cluster.
    if let Some(tc) = le.to_cluster.as_deref() {
        if let Some(boundary) = state_edge_boundary_for_cluster(ctx, tc, origin_x, origin_y) {
            points_for_curve = state_edge_cut_path_at_intersect(&points_for_curve, &boundary);
        }
    }
    if let Some(fc) = le.from_cluster.as_deref() {
        if let Some(boundary) = state_edge_boundary_for_cluster(ctx, fc, origin_x, origin_y) {
            let mut rev = points_for_curve;
            rev.reverse();
            rev = state_edge_cut_path_at_intersect(&rev, &boundary);
            rev.reverse();
            points_for_curve = rev;
        }
    }

    if is_cyclic_special {
        if edge_id.contains("-cyclic-special-mid") && points_for_curve.len() > 3 {
            points_for_curve = vec![
                points_for_curve[0].clone(),
                points_for_curve[points_for_curve.len() / 2].clone(),
                points_for_curve[points_for_curve.len() - 1].clone(),
            ];
        }
        if points_for_curve.len() == 4 {
            // Mermaid's cyclic-special helper edges frequently collapse the 4-point basis
            // case into the 3-point command sequence (`C` count = 2).
            points_for_curve.remove(1);
        }
        if edge_id.ends_with("-cyclic-special-2") && points_for_curve.len() == 6 {
            // Some cyclic-special-2 helper edges are routed with 6 points but Mermaid's path
            // command sequence matches the 5-point `curveBasis` case (`C` count = 4).
            points_for_curve.remove(1);
        }
    }
    points_for_curve = state_line_with_end_marker_offset_points(&points_for_curve, arrow_type_end);

    (local_points, points_for_curve)
}

fn state_edge_encode_path(
    ctx: &StateRenderCtx<'_>,
    le: &crate::model::LayoutEdge,
    edge_id: &str,
    arrow_type_end: Option<&str>,
    origin_x: f64,
    origin_y: f64,
) -> (String, String) {
    let (local_points, points_for_curve) =
        state_edge_prepare_points(ctx, le, edge_id, arrow_type_end, origin_x, origin_y);

    let data_points = base64::engine::general_purpose::STANDARD
        .encode(serde_json::to_vec(&local_points).unwrap_or_default());
    let d = curve_basis_path_d(&points_for_curve);
    (d, data_points)
}

pub(super) fn render_state_edge_path(
    out: &mut String,
    ctx: &StateRenderCtx<'_>,
    edge: &StateSvgEdge,
    origin_x: f64,
    origin_y: f64,
) {
    let mut classes = "edge-thickness-normal edge-pattern-solid".to_string();
    for c in edge.classes.split_whitespace() {
        if c.trim().is_empty() {
            continue;
        }
        classes.push(' ');
        classes.push_str(c.trim());
    }

    let marker_end = match edge.arrow_type_end.trim() {
        "arrow_barb" | "arrow_barb_neo" => {
            Some(format!("url(#{}_stateDiagram-barbEnd)", ctx.diagram_id))
        }
        _ => None,
    };

    if edge.start == edge.end {
        let start = edge.start.as_str();
        let id1 = format!("{start}-cyclic-special-1");
        let idm = format!("{start}-cyclic-special-mid");
        let id2 = format!("{start}-cyclic-special-2");

        let segments = [(&id1, None), (&idm, None), (&id2, marker_end.as_ref())];
        for (sid, marker) in segments {
            let Some(le) = ctx.layout_edges_by_id.get(sid.as_str()).copied() else {
                continue;
            };
            if le.points.len() < 2 {
                continue;
            }
            let (d, data_points) = state_edge_encode_path(
                ctx,
                le,
                sid,
                marker.map(|_| edge.arrow_type_end.as_str()),
                origin_x,
                origin_y,
            );
            let _ = write!(
                out,
                r#"<path d="{}" id="{}" class="{}" style="fill:none;;;fill:none" data-edge="true" data-et="edge" data-id="{}" data-points="{}""#,
                d,
                escape_xml_display(sid),
                escape_xml_display(&classes),
                escape_xml_display(sid),
                data_points
            );
            if let Some(m) = marker {
                let _ = write!(out, r#" marker-end="{}""#, escape_xml_display(m));
            }
            out.push_str("/>");
        }
        return;
    }

    let Some(le) = ctx.layout_edges_by_id.get(edge.id.as_str()).copied() else {
        return;
    };
    if le.points.len() < 2 {
        return;
    }

    let (d, data_points) = state_edge_encode_path(
        ctx,
        le,
        edge.id.as_str(),
        Some(edge.arrow_type_end.as_str()),
        origin_x,
        origin_y,
    );

    let _ = write!(
        out,
        r#"<path d="{}" id="{}" class="{}" style="fill:none;;;fill:none" data-edge="true" data-et="edge" data-id="{}" data-points="{}""#,
        d,
        escape_xml_display(&edge.id),
        escape_xml_display(&classes),
        escape_xml_display(&edge.id),
        data_points
    );
    if let Some(m) = marker_end {
        let _ = write!(out, r#" marker-end="{}""#, escape_xml_display(&m));
    }
    out.push_str("/>");
}

pub(super) fn render_state_edge_label(
    out: &mut String,
    ctx: &StateRenderCtx<'_>,
    edge: &StateSvgEdge,
    origin_x: f64,
    origin_y: f64,
) {
    fn edge_label_div_style(label_w: f64) -> String {
        // Mermaid uses `createText(..., { width: 200 })` for state edge labels and flips the XHTML
        // `<div>` container to wrapping mode when the label reaches the max width.
        let max_width = state_text_overrides::state_edge_label_max_width_px();
        if label_w >= max_width - 1e-3 {
            format!(
                "display: table; white-space: break-spaces; line-height: 1.5; max-width: {}px; text-align: center; width: {}px;",
                fmt_display(max_width),
                fmt_display(max_width),
            )
        } else {
            format!(
                "display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: center;",
                fmt_display(max_width),
            )
        }
    }

    fn mermaid_round_number(num: f64, precision: i32) -> f64 {
        let factor = 10_f64.powi(precision);
        (num * factor).round() / factor
    }

    fn mermaid_distance(
        point: &crate::model::LayoutPoint,
        prev: Option<&crate::model::LayoutPoint>,
    ) -> f64 {
        let Some(prev) = prev else {
            return 0.0;
        };
        ((point.x - prev.x).powi(2) + (point.y - prev.y).powi(2)).sqrt()
    }

    fn mermaid_calculate_point(
        points: &[crate::model::LayoutPoint],
        distance_to_traverse: f64,
    ) -> Option<crate::model::LayoutPoint> {
        let mut prev: Option<&crate::model::LayoutPoint> = None;
        let mut remaining = distance_to_traverse;
        for point in points {
            if let Some(prev_point) = prev {
                let vector_distance = mermaid_distance(point, Some(prev_point));
                if vector_distance == 0.0 {
                    return Some(prev_point.clone());
                }
                if vector_distance < remaining {
                    remaining -= vector_distance;
                } else {
                    let distance_ratio = remaining / vector_distance;
                    if distance_ratio <= 0.0 {
                        return Some(prev_point.clone());
                    }
                    if distance_ratio >= 1.0 {
                        return Some(point.clone());
                    }
                    if distance_ratio > 0.0 && distance_ratio < 1.0 {
                        return Some(crate::model::LayoutPoint {
                            x: mermaid_round_number(
                                (1.0 - distance_ratio) * prev_point.x + distance_ratio * point.x,
                                5,
                            ),
                            y: mermaid_round_number(
                                (1.0 - distance_ratio) * prev_point.y + distance_ratio * point.y,
                                5,
                            ),
                        });
                    }
                }
            }
            prev = Some(point);
        }
        None
    }

    fn mermaid_calc_label_position(
        points: &[crate::model::LayoutPoint],
    ) -> Option<crate::model::LayoutPoint> {
        if points.is_empty() {
            return None;
        }
        if points.len() == 1 {
            return Some(points[0].clone());
        }

        let mut total_distance: f64 = 0.0;
        let mut prev: Option<&crate::model::LayoutPoint> = None;
        for point in points {
            total_distance += mermaid_distance(point, prev);
            prev = Some(point);
        }

        let remaining_distance = total_distance / 2.0;
        mermaid_calculate_point(points, remaining_distance)
    }

    let empty_edge_label_style = edge_label_div_style(0.0);
    let label_text = edge.label.trim();
    if edge.start == edge.end {
        let start = edge.start.as_str();
        let id1 = format!("{start}-cyclic-special-1");
        let idm = format!("{start}-cyclic-special-mid");
        let id2 = format!("{start}-cyclic-special-2");

        // Mermaid emits self-loop label containers in the order:
        // `*-cyclic-special-1`, `*-cyclic-special-mid` (visible label), `*-cyclic-special-2`.
        let _ = write!(
            out,
            r#"<g class="edgeLabel"><g class="label" data-id="{}" transform="translate(0, 0)"><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}"><span class="edgeLabel"></span></div></foreignObject></g></g>"#,
            escape_attr(&id1),
            empty_edge_label_style.as_str()
        );

        // Mermaid ties the visible self-loop label to the `*-mid` segment.
        if !label_text.is_empty() {
            if let Some(le) = ctx.layout_edges_by_id.get(idm.as_str()).copied() {
                if let Some(lbl) = le.label.as_ref() {
                    let cx = lbl.x - origin_x;
                    let cy = lbl.y - origin_y;
                    let w = lbl.width.max(0.0);
                    let h = lbl.height.max(0.0);
                    let _ = write!(
                        out,
                        r#"<g class="edgeLabel" transform="translate({}, {})"><g class="label" data-id="{}" transform="translate({}, {})"><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}"><span class="edgeLabel">{}</span></div></foreignObject></g></g>"#,
                        fmt_display(cx),
                        fmt_display(cy),
                        escape_xml_display(&idm),
                        fmt_display(-w / 2.0),
                        fmt_display(-h / 2.0),
                        fmt_display(w),
                        fmt_display(h),
                        edge_label_div_style(w),
                        state_edge_label_html(label_text)
                    );
                }
            }
        } else {
            let _ = write!(
                out,
                r#"<g class="edgeLabel"><g class="label" data-id="{}" transform="translate(0, 0)"><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}"><span class="edgeLabel"></span></div></foreignObject></g></g>"#,
                escape_xml_display(&idm),
                empty_edge_label_style.as_str()
            );
        }

        let _ = write!(
            out,
            r#"<g class="edgeLabel"><g class="label" data-id="{}" transform="translate(0, 0)"><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}"><span class="edgeLabel"></span></div></foreignObject></g></g>"#,
            escape_attr(&id2),
            empty_edge_label_style.as_str()
        );
        return;
    }

    if label_text.is_empty() {
        let _ = write!(
            out,
            r#"<g class="edgeLabel"><g class="label" data-id="{}" transform="translate(0, 0)"><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}"><span class="edgeLabel"></span></div></foreignObject></g></g>"#,
            escape_xml_display(&edge.id),
            empty_edge_label_style.as_str()
        );
        return;
    }

    let Some(le) = ctx.layout_edges_by_id.get(edge.id.as_str()).copied() else {
        return;
    };
    let Some(lbl) = le.label.as_ref() else {
        return;
    };

    let mut cx = lbl.x - origin_x;
    let mut cy = lbl.y - origin_y;

    // Mermaid `rendering-elements/edges.js insertEdge` sets `paths.updatedPath` when:
    // - cluster cutting happened (`toCluster` / `fromCluster`)
    // - or the mid point would not be present in the rendered `d` string (curveBasis does not
    //   pass through all control points; labels anchored on those points drift).
    //
    // `positionEdgeLabel` then recomputes the label center from `utils.calcLabelPosition(...)`
    // *only when* `updatedPath` exists. Otherwise it keeps Dagre's `edge.x/y` unchanged.
    let (_local_points, points_for_curve) = state_edge_prepare_points(
        ctx,
        le,
        edge.id.as_str(),
        Some(edge.arrow_type_end.as_str()),
        origin_x,
        origin_y,
    );

    fn mermaid_is_label_coordinate_in_path(
        point: &crate::model::LayoutPoint,
        d_attr: &str,
    ) -> bool {
        let rounded_x = point.x.round() as i64;
        let rounded_y = point.y.round() as i64;

        let bytes = d_attr.as_bytes();
        let mut i = 0usize;
        while i < bytes.len() {
            let b = bytes[i];
            let is_start = b.is_ascii_digit() || b == b'-' || b == b'.';
            if !is_start {
                i += 1;
                continue;
            }

            let start = i;
            i += 1;
            while i < bytes.len() {
                let b = bytes[i];
                if b.is_ascii_digit() || b == b'.' {
                    i += 1;
                    continue;
                }
                break;
            }

            let token = &d_attr[start..i];
            if let Ok(v) = token.parse::<f64>() {
                let r = v.round() as i64;
                if r == rounded_x || r == rounded_y {
                    return true;
                }
            }
        }

        false
    }

    let mut points_has_changed = le.to_cluster.is_some() || le.from_cluster.is_some();
    if !points_has_changed && !points_for_curve.is_empty() {
        let d_attr = curve_basis_path_d(&points_for_curve);
        let mid = &points_for_curve[points_for_curve.len() / 2];
        if !mermaid_is_label_coordinate_in_path(mid, &d_attr) {
            points_has_changed = true;
        }
    }

    if points_has_changed {
        if let Some(pos) = mermaid_calc_label_position(&points_for_curve) {
            cx = pos.x;
            cy = pos.y;
        }
    }
    let w = lbl.width.max(0.0);
    let h = lbl.height.max(0.0);

    let _ = write!(
        out,
        r#"<g class="edgeLabel" transform="translate({}, {})"><g class="label" data-id="{}" transform="translate({}, {})"><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}"><span class="edgeLabel">{}</span></div></foreignObject></g></g>"#,
        fmt_display(cx),
        fmt_display(cy),
        escape_xml_display(&edge.id),
        fmt_display(-w / 2.0),
        fmt_display(-h / 2.0),
        fmt_display(w),
        fmt_display(h),
        edge_label_div_style(w),
        state_edge_label_html(label_text)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_line_with_end_marker_offset_shortens_neo_barb_terminal_point() {
        let input = vec![
            crate::model::LayoutPoint { x: 0.0, y: 0.0 },
            crate::model::LayoutPoint { x: 10.0, y: 0.0 },
        ];

        let output = state_line_with_end_marker_offset_points(&input, Some("arrow_barb_neo"));

        assert_eq!(output.len(), 2);
        assert!((output[0].x - 0.0).abs() <= 1e-9);
        assert!((output[0].y - 0.0).abs() <= 1e-9);
        assert!((output[1].x - 4.5).abs() <= 1e-9);
        assert!((output[1].y - 0.0).abs() <= 1e-9);
    }
}
