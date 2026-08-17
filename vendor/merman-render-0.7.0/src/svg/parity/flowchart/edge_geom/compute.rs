//! Flowchart edge path geometry computation.

use super::*;

pub(super) fn flowchart_compute_edge_path_geom(
    request: FlowchartEdgePathGeomRequest<'_>,
    scratch: &mut FlowchartEdgeDataPointsScratch,
) -> Option<FlowchartEdgePathGeom> {
    let FlowchartEdgePathGeomRequest {
        ctx,
        edge,
        origin_x,
        origin_y,
        abs_top_transform,
        trace_enabled,
        viewbox_current_bounds,
    } = request;

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

        let mut total_distance = 0.0;
        let mut prev: Option<&crate::model::LayoutPoint> = None;
        for point in points {
            total_distance += mermaid_distance(point, prev);
            prev = Some(point);
        }

        mermaid_calculate_point(points, total_distance / 2.0)
    }

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
                let rounded = v.round() as i64;
                if rounded == rounded_x || rounded == rounded_y {
                    return true;
                }
            }
        }

        false
    }

    let le = ctx.layout_edges_by_id.get(edge.id.as_str())?;
    if le.points.len() < 2 {
        return None;
    }

    scratch.local_points.clear();
    scratch.local_points.reserve(le.points.len());
    for p in &le.points {
        scratch.local_points.push(crate::model::LayoutPoint {
            x: p.x + ctx.tx - origin_x,
            y: p.y + ctx.ty - origin_y,
        });
    }
    let local_points = scratch.local_points.as_slice();

    use super::{
        FlowchartEdgeTraceInput, TraceEndpointIntersection, boundary_for_cluster,
        boundary_for_node, curve_path_d_and_bounds, cut_path_at_intersect_into,
        dedup_consecutive_points_into, force_intersect_for_layout_shape,
        intersect_for_layout_shape, is_rounded_intersect_shift_shape,
        line_with_offset_for_edge_type, maybe_collapse_degenerate_subgraph_edge_route,
        maybe_fix_corners, maybe_normalize_selfedge_loop_points,
        maybe_remove_redundant_cluster_run_point, maybe_snap_data_point_to_f32,
        maybe_snap_shallow_basis_triplet_y_to_f32, maybe_truncate_data_point,
        normalize_cyclic_special_data_points, rounded_line_with_marker_offsets_for_edge_type,
        write_flowchart_edge_trace,
    };

    let is_cyclic_special = edge.id.contains("-cyclic-special-");
    dedup_consecutive_points_into(local_points, &mut scratch.tmp_points_a);
    let base_points: &mut Vec<crate::model::LayoutPoint> = &mut scratch.tmp_points_a;
    maybe_normalize_selfedge_loop_points(base_points);

    scratch.tmp_points_b.clear();
    scratch.tmp_points_b.extend_from_slice(base_points);
    let points_after_intersect: &mut Vec<crate::model::LayoutPoint> = &mut scratch.tmp_points_b;

    if base_points.len() >= 3 {
        let tail_shape = ctx
            .nodes_by_id
            .get(edge.from.as_str())
            .and_then(|n| n.layout_shape.as_deref());
        let head_shape = ctx
            .nodes_by_id
            .get(edge.to.as_str())
            .and_then(|n| n.layout_shape.as_deref());
        if let (Some(tail), Some(head)) = (
            boundary_for_node(
                ctx,
                edge.from.as_str(),
                origin_x,
                origin_y,
                is_cyclic_special,
            ),
            boundary_for_node(ctx, edge.to.as_str(), origin_x, origin_y, is_cyclic_special),
        ) {
            let interior = &base_points[1..base_points.len() - 1];
            if !interior.is_empty() {
                let mut start = base_points[0].clone();
                let mut end = base_points[base_points.len() - 1].clone();

                let eps = 1e-4;
                let start_is_center =
                    (start.x - tail.x).abs() < eps && (start.y - tail.y).abs() < eps;
                let end_is_center = (end.x - head.x).abs() < eps && (end.y - head.y).abs() < eps;

                if start_is_center || force_intersect_for_layout_shape(tail_shape) {
                    start = intersect_for_layout_shape(
                        ctx,
                        edge.from.as_str(),
                        &tail,
                        tail_shape,
                        &interior[0],
                    );
                    if is_rounded_intersect_shift_shape(tail_shape) {
                        start.x += 0.5;
                        start.y += 0.5;
                    }
                }

                if end_is_center || force_intersect_for_layout_shape(head_shape) {
                    end = intersect_for_layout_shape(
                        ctx,
                        edge.to.as_str(),
                        &head,
                        head_shape,
                        &interior[interior.len() - 1],
                    );
                    if is_rounded_intersect_shift_shape(head_shape) {
                        end.x += 0.5;
                        end.y += 0.5;
                    }
                }

                points_after_intersect.clear();
                points_after_intersect.reserve(interior.len() + 2);
                points_after_intersect.push(start);
                points_after_intersect.extend(interior.iter().cloned());
                points_after_intersect.push(end);
            }
        }
    }

    scratch.tmp_points_c.clear();
    if let Some(tc) = le.to_cluster.as_deref() {
        if let Some(boundary) = boundary_for_cluster(ctx, tc, origin_x, origin_y) {
            cut_path_at_intersect_into(base_points, &boundary, &mut scratch.tmp_points_c);
        } else {
            scratch
                .tmp_points_c
                .extend_from_slice(points_after_intersect);
        }
    } else {
        scratch
            .tmp_points_c
            .extend_from_slice(points_after_intersect);
    }
    if let Some(fc) = le.from_cluster.as_deref() {
        if let Some(boundary) = boundary_for_cluster(ctx, fc, origin_x, origin_y) {
            scratch.tmp_points_rev.clear();
            scratch
                .tmp_points_rev
                .extend_from_slice(&scratch.tmp_points_c);
            scratch.tmp_points_rev.reverse();

            cut_path_at_intersect_into(
                &scratch.tmp_points_rev,
                &boundary,
                &mut scratch.tmp_points_c,
            );
            scratch.tmp_points_c.reverse();
        }
    }
    let points_for_render: &mut Vec<crate::model::LayoutPoint> = &mut scratch.tmp_points_c;

    // Mermaid sets `data-points` as `btoa(JSON.stringify(points))` *before* any cluster clipping
    // (`cutPathAtIntersect`). Keep that exact ordering for strict DOM parity.
    let points_after_intersect_for_trace = trace_enabled.then(|| scratch.tmp_points_b.clone());
    let points_for_data_points: &mut Vec<crate::model::LayoutPoint> = &mut scratch.tmp_points_b;

    let mut trace_points_before_norm: Option<Vec<crate::model::LayoutPoint>> = None;
    let mut trace_points_after_norm: Option<Vec<crate::model::LayoutPoint>> = None;
    let mut trace_endpoint: Option<TraceEndpointIntersection> = None;
    if trace_enabled {
        trace_points_before_norm = Some(points_for_data_points.clone());
    }

    if is_cyclic_special {
        normalize_cyclic_special_data_points(
            ctx,
            edge,
            origin_x,
            origin_y,
            points_for_data_points,
            &mut trace_endpoint,
        );
        if trace_enabled {
            trace_points_after_norm = Some(points_for_data_points.clone());
        }
    }
    for p in points_for_data_points.iter_mut() {
        // Keep truncation scoped to y-coordinates: the observed upstream fixed-point artifacts
        // are for vertical intersections, while x-coordinates can legitimately land on thirds for
        // some polygon shapes (and truncating those breaks strict parity).
        p.x = maybe_snap_data_point_to_f32(p.x);
        p.y = maybe_snap_data_point_to_f32(maybe_truncate_data_point(p.y));
    }

    let interpolate = edge
        .interpolate
        .as_deref()
        .unwrap_or(ctx.default_edge_interpolate.as_str());
    let is_rounded = interpolate == "rounded";
    let is_basis = !matches!(
        interpolate,
        "linear"
            | "natural"
            | "bumpY"
            | "catmullRom"
            | "step"
            | "stepAfter"
            | "stepBefore"
            | "cardinal"
            | "monotoneX"
            | "monotoneY"
            | "rounded"
    );

    let label_text = edge.label.as_deref().unwrap_or_default();
    let label_type = edge.label_type.as_deref().unwrap_or("text");
    let label_text_plain = flowchart_label_plain_text(label_text, label_type, ctx.edge_html_labels);
    let has_label_text = !label_text_plain.trim().is_empty();
    let is_cluster_edge = le.to_cluster.is_some() || le.from_cluster.is_some();
    let points_for_label = has_label_text.then(|| points_for_render.clone());

    if is_basis && is_cluster_edge {
        maybe_remove_redundant_cluster_run_point(points_for_render);
    }

    if is_basis
        && is_cyclic_special
        && edge.id.contains("-cyclic-special-mid")
        && points_for_render.len() > 3
    {
        let a = points_for_render[0].clone();
        let mid = points_for_render[points_for_render.len() / 2].clone();
        let b = points_for_render[points_for_render.len() - 1].clone();
        points_for_render.clear();
        points_for_render.extend([a, mid, b]);
    }
    if points_for_render.len() == 1 {
        // Avoid emitting a degenerate `M x,y` path for clipped cluster-adjacent edges.
        points_for_render.clear();
        points_for_render.extend(scratch.local_points.iter().cloned());
    }

    // D3's `curveBasis` emits only a straight `M ... L ...` when there are exactly two points.
    // Mermaid's Dagre pipeline typically provides at least one intermediate point even for
    // straight-looking edges, resulting in `C` segments in the SVG `d`. To keep our output closer
    // to Mermaid's command sequence, re-insert a midpoint when our route collapses to two points
    // after normalization (but keep cluster-adjacent edges as-is: Mermaid uses straight segments
    // there).
    if is_basis
        && points_for_render.len() == 2
        && interpolate != "linear"
        && (!is_cluster_edge || is_cyclic_special)
    {
        let a = &points_for_render[0];
        let b = &points_for_render[1];
        points_for_render.insert(
            1,
            crate::model::LayoutPoint {
                x: (a.x + b.x) / 2.0,
                y: (a.y + b.y) / 2.0,
            },
        );
    }

    let mut line_data: Vec<crate::model::LayoutPoint> = points_for_render
        .iter()
        .filter(|p| !p.y.is_nan())
        .cloned()
        .collect();

    // Match Mermaid `fixCorners` in `rendering-elements/edges.js`: insert small offset points to
    // round orthogonal corners before feeding into D3's line generator. The `rounded` curve uses
    // its own rounded-corner generator and skips this pre-processing upstream.
    if !is_rounded {
        maybe_fix_corners(&mut line_data);
    }

    if is_basis {
        maybe_snap_shallow_basis_triplet_y_to_f32(&mut line_data, edge.edge_type.as_deref());
    }

    // Mermaid shortens edge paths so markers don't render on top of the line (see
    // `packages/mermaid/src/utils/lineWithOffset.ts`).

    let mut line_data = if is_rounded {
        rounded_line_with_marker_offsets_for_edge_type(&line_data, edge.edge_type.as_deref())
    } else {
        line_with_offset_for_edge_type(&line_data, edge.edge_type.as_deref())
    };
    maybe_collapse_degenerate_subgraph_edge_route(
        ctx,
        edge,
        points_for_data_points,
        &mut line_data,
    );

    let (d, raw_pb, skipped_bounds_for_viewbox) = curve_path_d_and_bounds(
        &line_data,
        interpolate,
        origin_x,
        abs_top_transform,
        viewbox_current_bounds,
    );
    let pb = svg_path_bounds_from_d(&d).or(raw_pb);

    let mut label_position = None;
    if let Some(points) = points_for_label.as_deref() {
        let mut points_has_changed = is_cluster_edge;
        if !points_has_changed && !points.is_empty() {
            let mid = &points[points.len() / 2];
            if !mermaid_is_label_coordinate_in_path(mid, &d) {
                points_has_changed = true;
            }
        }
        if points_has_changed {
            label_position = mermaid_calc_label_position(points);
        }
    }

    if trace_enabled {
        write_flowchart_edge_trace(FlowchartEdgeTraceInput {
            ctx,
            edge,
            layout_edge: le,
            origin_x,
            origin_y,
            base_points,
            points_after_intersect_for_trace: points_after_intersect_for_trace.as_deref(),
            points_for_render,
            points_for_data_points_before_norm: trace_points_before_norm.as_deref(),
            points_for_data_points_after_norm: trace_points_after_norm.as_deref(),
            points_for_data_points_final: points_for_data_points,
            endpoint_intersection: trace_endpoint,
        });
    }

    scratch.json.clear();
    json_stringify_points_into(
        &mut scratch.json,
        points_for_data_points.as_slice(),
        &mut scratch.ryu,
    );
    let mut data_points_b64 =
        String::with_capacity(base64::encoded_len(scratch.json.len(), true).unwrap_or_default());
    base64::engine::general_purpose::STANDARD
        .encode_string(scratch.json.as_bytes(), &mut data_points_b64);

    Some(FlowchartEdgePathGeom {
        d,
        pb,
        data_points_b64,
        label_position,
        bounds_skipped_for_viewbox: skipped_bounds_for_viewbox,
    })
}
