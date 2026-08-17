use super::ClassSvgRelation;
use super::bounds::{include_path_bounds, include_path_d, include_xywh};
use super::context::ClassRenderDetails;
use super::defs::class_marker_name;
use super::label::{
    class_html_div_style, render_class_html_label, write_class_svg_edge_text,
    write_class_svg_edge_text_markdown,
};
use crate::entities::decode_entities_minimal_cow;
use crate::generated::class_text_overrides_11_12_2 as class_text_overrides;
use crate::model::{Bounds, LayoutEdge, LayoutLabel, LayoutPoint};
use base64::Engine as _;
use std::fmt::Write as _;

use super::super::{escape_attr_display, escape_xml_into, fmt, json_stringify_points_into};
use rustc_hash::FxHashMap;

pub(super) struct ClassEdgeGroupsRenderState<'a> {
    pub out: &'a mut String,
    pub content_bounds: &'a mut Option<Bounds>,
    pub detail: &'a mut ClassRenderDetails,
}

pub(super) struct ClassEdgeGroupsRenderContext<'a> {
    pub edges: &'a [LayoutEdge],
    pub relations_by_id: &'a FxHashMap<&'a str, &'a ClassSvgRelation>,
    pub relation_index_by_id: &'a FxHashMap<&'a str, usize>,
    pub marker_url_prefix: &'a str,
    pub diagram_id: &'a str,
    pub content_tx: f64,
    pub content_ty: f64,
    pub bounds_dx: f64,
    pub bounds_dy: f64,
    pub edge_use_html_labels: bool,
    pub look: &'a str,
    pub timing_enabled: bool,
}

fn class_arrow_type_for_relation_end(ty: i32) -> Option<&'static str> {
    match ty {
        0 => Some("aggregation"),
        1 => Some("extension"),
        2 => Some("composition"),
        3 => Some("dependency"),
        4 => Some("lollipop"),
        _ => None,
    }
}

pub(super) fn class_line_with_marker_offset_points_into(
    input: &[LayoutPoint],
    relation: Option<&ClassSvgRelation>,
    out: &mut Vec<LayoutPoint>,
) {
    fn marker_offset_for(arrow_type: Option<&str>) -> Option<f64> {
        match arrow_type {
            Some("dependency") => Some(6.0),
            Some("lollipop") => Some(13.5),
            Some("aggregation" | "extension" | "composition") => Some(17.25),
            _ => None,
        }
    }

    fn calculate_delta_and_angle(a: &LayoutPoint, b: &LayoutPoint) -> (f64, f64, f64) {
        let delta_x = b.x - a.x;
        let delta_y = b.y - a.y;
        let angle = (delta_y / delta_x).atan();
        (angle, delta_x, delta_y)
    }

    out.clear();
    out.reserve(input.len());
    if input.len() < 2 {
        out.extend(input.iter().cloned());
        return;
    }

    let arrow_type_start =
        relation.and_then(|rel| class_arrow_type_for_relation_end(rel.relation.type1));
    let arrow_type_end =
        relation.and_then(|rel| class_arrow_type_for_relation_end(rel.relation.type2));
    let start = &input[0];
    let end = &input[input.len() - 1];
    let x_direction_is_left = start.x < end.x;
    let y_direction_is_down = start.y < end.y;
    let extra_room = 1.0;
    let start_marker_height = marker_offset_for(arrow_type_start);
    let end_marker_height = marker_offset_for(arrow_type_end);

    for (idx, point) in input.iter().enumerate() {
        let mut offset_x = 0.0;
        let mut offset_y = 0.0;

        if idx == 0 {
            if let Some(height) = start_marker_height {
                let (angle, delta_x, delta_y) = calculate_delta_and_angle(&input[0], &input[1]);
                offset_x = height * angle.cos() * if delta_x >= 0.0 { 1.0 } else { -1.0 };
                offset_y = height * angle.sin().abs() * if delta_y >= 0.0 { 1.0 } else { -1.0 };
            }
        } else if idx == input.len() - 1 {
            if let Some(height) = end_marker_height {
                let (angle, delta_x, delta_y) =
                    calculate_delta_and_angle(&input[input.len() - 1], &input[input.len() - 2]);
                offset_x = height * angle.cos() * if delta_x >= 0.0 { 1.0 } else { -1.0 };
                offset_y = height * angle.sin().abs() * if delta_y >= 0.0 { 1.0 } else { -1.0 };
            }
        }

        if let Some(height) = end_marker_height {
            let diff_x = (point.x - end.x).abs();
            let diff_y = (point.y - end.y).abs();
            if diff_x < height && diff_x > 0.0 && diff_y < height {
                let mut adjustment = height + extra_room - diff_x;
                adjustment *= if !x_direction_is_left { -1.0 } else { 1.0 };
                offset_x -= adjustment;
            }
        }
        if let Some(height) = start_marker_height {
            let diff_x = (point.x - start.x).abs();
            let diff_y = (point.y - start.y).abs();
            if diff_x < height && diff_x > 0.0 && diff_y < height {
                let mut adjustment = height + extra_room - diff_x;
                adjustment *= if !x_direction_is_left { -1.0 } else { 1.0 };
                offset_x += adjustment;
            }
        }

        if let Some(height) = end_marker_height {
            let diff_y = (point.y - end.y).abs();
            let diff_x = (point.x - end.x).abs();
            if diff_y < height && diff_y > 0.0 && diff_x < height {
                let mut adjustment = height + extra_room - diff_y;
                adjustment *= if !y_direction_is_down { -1.0 } else { 1.0 };
                offset_y -= adjustment;
            }
        }
        if let Some(height) = start_marker_height {
            let diff_y = (point.y - start.y).abs();
            let diff_x = (point.x - start.x).abs();
            if diff_y < height && diff_y > 0.0 && diff_x < height {
                let mut adjustment = height + extra_room - diff_y;
                adjustment *= if !y_direction_is_down { -1.0 } else { 1.0 };
                offset_y += adjustment;
            }
        }

        out.push(LayoutPoint {
            x: point.x + offset_x,
            y: point.y + offset_y,
        });
    }
}

fn class_js_round(v: f64, decimals: i32) -> f64 {
    if !v.is_finite() {
        return 0.0;
    }
    let factor = 10f64.powi(decimals);
    let rounded = (v * factor).round() / factor;
    if rounded == -0.0 { 0.0 } else { rounded }
}

fn class_calc_label_position(points: &[LayoutPoint]) -> Option<LayoutPoint> {
    if points.is_empty() {
        return None;
    }
    if points.len() == 1 {
        return Some(points[0].clone());
    }

    let mut total = 0.0;
    for window in points.windows(2) {
        total += (window[1].x - window[0].x).hypot(window[1].y - window[0].y);
    }
    if !total.is_finite() || total <= 0.0 {
        return Some(points[0].clone());
    }

    let mut remaining = total / 2.0;
    for window in points.windows(2) {
        let a = &window[0];
        let b = &window[1];
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
            return Some(LayoutPoint {
                x: class_js_round(b.x, 5),
                y: class_js_round(b.y, 5),
            });
        }
        return Some(LayoutPoint {
            x: class_js_round((1.0 - ratio) * a.x + ratio * b.x, 5),
            y: class_js_round((1.0 - ratio) * a.y + ratio * b.y, 5),
        });
    }

    Some(points[0].clone())
}

fn class_is_label_coordinate_in_path(point: &LayoutPoint, d_attr: &str) -> bool {
    let rounded_x = point.x.round() as i64;
    let rounded_y = point.y.round() as i64;
    let bytes = d_attr.as_bytes();
    let mut idx = 0usize;
    while idx < bytes.len() {
        let b = bytes[idx];
        let is_start = b.is_ascii_digit() || b == b'-' || b == b'.';
        if !is_start {
            idx += 1;
            continue;
        }

        let start = idx;
        idx += 1;
        while idx < bytes.len() {
            let b = bytes[idx];
            if b.is_ascii_digit() || b == b'.' {
                idx += 1;
                continue;
            }
            break;
        }

        if let Ok(v) = d_attr[start..idx].parse::<f64>() {
            let rounded = v.round() as i64;
            if rounded == rounded_x || rounded == rounded_y {
                return true;
            }
        }
    }

    false
}

pub(super) fn render_class_edge_groups(
    state: ClassEdgeGroupsRenderState<'_>,
    ctx: &ClassEdgeGroupsRenderContext<'_>,
) {
    let out = &mut *state.out;
    let content_bounds = &mut *state.content_bounds;
    let detail = &mut *state.detail;

    let mut edge_points_json_buf = String::new();
    let mut edge_points_json_ryu = ryu_js::Buffer::new();
    let mut edge_points_b64_buf = String::new();
    let mut edge_raw_points: Vec<LayoutPoint> = Vec::new();
    let mut edge_marker_points: Vec<LayoutPoint> = Vec::new();
    let mut edge_curve_points: Vec<LayoutPoint> = Vec::new();
    let mut edge_class_buf = String::with_capacity(64);
    let mut edge_dom_id_buf = String::with_capacity(64);

    let edge_paths_start = ctx.timing_enabled.then(web_time::Instant::now);
    let ordered_edges = class_edge_render_order(ctx.edges, ctx.relation_index_by_id);
    let mut edge_label_centers: FxHashMap<&str, LayoutPoint> =
        FxHashMap::with_capacity_and_hasher(ordered_edges.len(), Default::default());
    out.push_str(r#"<g class="edgePaths">"#);
    for e in ordered_edges.iter().copied() {
        if e.points.len() < 2 {
            continue;
        }

        class_edge_dom_id_into(&mut edge_dom_id_buf, e, ctx.relation_index_by_id);

        edge_raw_points.clear();
        edge_raw_points.reserve(e.points.len());
        for p in &e.points {
            edge_raw_points.push(LayoutPoint {
                x: p.x + ctx.content_tx,
                y: p.y + ctx.content_ty,
            });
        }

        let curve_start = ctx.timing_enabled.then(web_time::Instant::now);
        let relation = if e.id.starts_with("edgeNote") {
            None
        } else {
            ctx.relations_by_id.get(e.id.as_str()).copied()
        };
        class_line_with_marker_offset_points_into(
            &edge_raw_points,
            relation,
            &mut edge_marker_points,
        );
        let edge_curve_source = edge_marker_points.as_slice();
        let (d, d_pb) = if edge_curve_source.len() == 2 {
            edge_curve_points.clear();
            let a = &edge_curve_source[0];
            let b = &edge_curve_source[1];
            edge_curve_points.push(a.clone());
            edge_curve_points.push(LayoutPoint {
                x: (a.x + b.x) / 2.0,
                y: (a.y + b.y) / 2.0,
            });
            edge_curve_points.push(b.clone());
            super::super::curve::curve_basis_path_d_and_bounds(&edge_curve_points)
        } else {
            super::super::curve::curve_basis_path_d_and_bounds(edge_curve_source)
        };
        if let Some(lbl) = e.label.as_ref() {
            edge_label_centers.insert(
                e.id.as_str(),
                class_edge_label_center(&edge_raw_points, &d, lbl, ctx.content_tx, ctx.content_ty),
            );
        }
        if let Some(s) = curve_start {
            detail.edge_curve += s.elapsed();
        }
        let path_bounds_start = ctx.timing_enabled.then(web_time::Instant::now);
        if let Some(pb) = d_pb.as_ref() {
            include_path_bounds(content_bounds, pb, ctx.bounds_dx, ctx.bounds_dy);
        } else {
            include_path_d(content_bounds, &d, ctx.bounds_dx, ctx.bounds_dy);
        }
        if let Some(s) = path_bounds_start {
            detail.path_bounds += s.elapsed();
            detail.path_bounds_calls += 1;
        }

        let json_start = ctx.timing_enabled.then(web_time::Instant::now);
        edge_points_json_buf.clear();
        json_stringify_points_into(
            &mut edge_points_json_buf,
            &edge_raw_points,
            &mut edge_points_json_ryu,
        );
        if let Some(s) = json_start {
            detail.edge_points_json += s.elapsed();
        }

        let b64_start = ctx.timing_enabled.then(web_time::Instant::now);
        edge_points_b64_buf.clear();
        base64::engine::general_purpose::STANDARD
            .encode_string(edge_points_json_buf.as_bytes(), &mut edge_points_b64_buf);
        if let Some(s) = b64_start {
            detail.edge_points_b64 += s.elapsed();
        }

        edge_class_buf.clear();
        edge_class_buf.push_str("edge-thickness-normal ");
        if e.id.starts_with("edgeNote") {
            edge_class_buf.push_str(class_note_edge_pattern());
        } else if let Some(rel) = ctx.relations_by_id.get(e.id.as_str()) {
            edge_class_buf.push_str(class_edge_pattern(rel.relation.line_type));
        } else {
            edge_class_buf.push_str("edge-pattern-solid");
        }
        edge_class_buf.push_str(" relation");

        let _ = write!(
            out,
            r#"<path d="{}" id="{}-{}" class="{}" data-edge="true" data-et="edge" data-id="{}" data-points="{}" data-look="{}""#,
            escape_attr_display(&d),
            escape_attr_display(ctx.diagram_id),
            escape_attr_display(&edge_dom_id_buf),
            escape_attr_display(&edge_class_buf),
            escape_attr_display(&edge_dom_id_buf),
            escape_attr_display(&edge_points_b64_buf),
            escape_attr_display(ctx.look),
        );
        if !e.id.starts_with("edgeNote") {
            if let Some(rel) = ctx.relations_by_id.get(e.id.as_str()) {
                if let Some(name) = class_marker_name(rel.relation.type1, true) {
                    out.push_str(r#" marker-start="url(#"#);
                    out.push_str(ctx.marker_url_prefix);
                    out.push_str(name);
                    out.push_str(r#")""#);
                }
                if let Some(name) = class_marker_name(rel.relation.type2, false) {
                    out.push_str(r#" marker-end="url(#"#);
                    out.push_str(ctx.marker_url_prefix);
                    out.push_str(name);
                    out.push_str(r#")""#);
                }
            }
        }
        let _ = write!(out, r#" style="{}""#, class_edge_path_style(e.id.as_str()));
        out.push_str("/>");
    }
    out.push_str("</g>");
    if let Some(s) = edge_paths_start {
        detail.edge_paths += s.elapsed();
    }

    let edge_labels_start = ctx.timing_enabled.then(web_time::Instant::now);
    out.push_str(r#"<g class="edgeLabels">"#);
    // Mermaid's serialized SVG keeps all `edgeLabel` groups before `edgeTerminals`.
    for e in ordered_edges.iter().copied() {
        class_edge_dom_id_into(&mut edge_dom_id_buf, e, ctx.relation_index_by_id);
        let label_text = if e.id.starts_with("edgeNote") {
            ""
        } else {
            ctx.relations_by_id
                .get(e.id.as_str())
                .map(|r| r.title.as_str())
                .unwrap_or("")
        };

        let label_center = e.label.as_ref().map(|lbl| {
            edge_label_centers
                .get(e.id.as_str())
                .cloned()
                .unwrap_or(LayoutPoint {
                    x: lbl.x + ctx.content_tx,
                    y: lbl.y + ctx.content_ty,
                })
        });
        if !label_text.trim().is_empty() {
            if let (Some(lbl), Some(center)) = (e.label.as_ref(), label_center.as_ref()) {
                include_xywh(
                    content_bounds,
                    center.x - lbl.width / 2.0 + ctx.bounds_dx,
                    center.y - lbl.height / 2.0 + ctx.bounds_dy,
                    lbl.width.max(0.0),
                    lbl.height.max(0.0),
                );
            }
        }
        render_class_edge_label_group(
            out,
            edge_dom_id_buf.as_str(),
            label_text,
            e.label.as_ref(),
            label_center.as_ref().map(|center| center.x).unwrap_or(0.0),
            label_center.as_ref().map(|center| center.y).unwrap_or(0.0),
            ctx.edge_use_html_labels,
        );
    }
    for e in ordered_edges.iter().copied() {
        let Some(rel) = ctx.relations_by_id.get(e.id.as_str()).copied() else {
            continue;
        };
        let start_text = if rel.relation_title_1 == "none" {
            ""
        } else {
            rel.relation_title_1.as_str()
        };
        for lbl in [&e.start_label_left, &e.start_label_right] {
            if let Some(lbl) = lbl.as_ref() {
                let (terminal_w, terminal_h) = class_terminal_box_size(start_text);
                if terminal_w > 0.0 && terminal_h > 0.0 {
                    include_xywh(
                        content_bounds,
                        lbl.x + ctx.content_tx + ctx.bounds_dx,
                        lbl.y + ctx.content_ty + ctx.bounds_dy,
                        terminal_w,
                        terminal_h,
                    );
                    render_class_edge_terminal_group(
                        out,
                        lbl.x + ctx.content_tx,
                        lbl.y + ctx.content_ty,
                        start_text,
                        true,
                    );
                }
            }
        }
    }
    let mut ordered_end_edges = ordered_edges
        .iter()
        .copied()
        .enumerate()
        .collect::<Vec<_>>();
    // Mermaid inserts terminal labels asynchronously. End-only cardinalities regularly land in
    // front of two-sided edges once the DOM settles, so prefer edges without a start terminal
    // before preserving the original render order.
    ordered_end_edges.sort_by_key(|(idx, edge)| {
        (
            edge.start_label_left.is_some() || edge.start_label_right.is_some(),
            *idx,
        )
    });
    for (_, e) in ordered_end_edges {
        let Some(rel) = ctx.relations_by_id.get(e.id.as_str()).copied() else {
            continue;
        };
        let end_text = if rel.relation_title_2 == "none" {
            ""
        } else {
            rel.relation_title_2.as_str()
        };
        for lbl in [&e.end_label_left, &e.end_label_right] {
            if let Some(lbl) = lbl.as_ref() {
                let (terminal_w, terminal_h) = class_terminal_box_size(end_text);
                if terminal_w > 0.0 && terminal_h > 0.0 {
                    include_xywh(
                        content_bounds,
                        lbl.x + ctx.content_tx + ctx.bounds_dx,
                        lbl.y + ctx.content_ty + ctx.bounds_dy,
                        terminal_w,
                        terminal_h,
                    );
                    render_class_edge_terminal_group(
                        out,
                        lbl.x + ctx.content_tx,
                        lbl.y + ctx.content_ty,
                        end_text,
                        false,
                    );
                }
            }
        }
    }
    out.push_str("</g>");
    if let Some(s) = edge_labels_start {
        detail.edge_labels += s.elapsed();
    }
}

pub(super) fn class_edge_label_center(
    raw_points: &[LayoutPoint],
    d_attr: &str,
    label: &LayoutLabel,
    content_tx: f64,
    content_ty: f64,
) -> LayoutPoint {
    let mut center = LayoutPoint {
        x: label.x + content_tx,
        y: label.y + content_ty,
    };
    if let Some(mid) = raw_points.get(raw_points.len() / 2) {
        if !class_is_label_coordinate_in_path(mid, d_attr) {
            if let Some(pos) = class_calc_label_position(raw_points) {
                center = pos;
            }
        }
    }
    center
}

pub(super) fn render_class_edge_label_group(
    out: &mut String,
    dom_id: &str,
    label_text: &str,
    label: Option<&LayoutLabel>,
    center_x: f64,
    center_y: f64,
    use_html_labels: bool,
) {
    let decoded = decode_entities_minimal_cow(label_text);
    let trimmed = decoded.trim();
    if use_html_labels {
        let empty_div_style =
            class_html_div_style(0.0, class_text_overrides::class_html_label_max_width_px());
        if trimmed.is_empty() {
            let _ = write!(
                out,
                r#"<g class="edgeLabel"><g class="label" data-id="{}" transform="translate(0, 0)"><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}"><span class="edgeLabel"></span></div></foreignObject></g></g>"#,
                escape_attr_display(dom_id),
                escape_attr_display(empty_div_style.as_str())
            );
        } else if let Some(lbl) = label {
            let div_style = class_html_div_style(
                lbl.width.max(0.0),
                class_text_overrides::class_html_label_max_width_px(),
            );
            let _ = write!(
                out,
                r#"<g class="edgeLabel" transform="translate({}, {})"><g class="label" data-id="{}" transform="translate({}, {})"><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}">"#,
                fmt(center_x),
                fmt(center_y),
                escape_attr_display(dom_id),
                fmt(-lbl.width / 2.0),
                fmt(-lbl.height / 2.0),
                fmt(lbl.width.max(0.0)),
                fmt(lbl.height.max(0.0)),
                escape_attr_display(div_style.as_str()),
            );
            render_class_html_label(out, "edgeLabel", trimmed, true, None, None);
            out.push_str("</div></foreignObject></g></g>");
        } else {
            let _ = write!(
                out,
                r#"<g class="edgeLabel"><g class="label" data-id="{}" transform="translate(0, 0)"><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="{}"><span class="edgeLabel"></span></div></foreignObject></g></g>"#,
                escape_attr_display(dom_id),
                escape_attr_display(empty_div_style.as_str())
            );
        }
        return;
    }

    if trimmed.is_empty() {
        out.push_str(r#"<g><rect class="background" style="stroke: none"/></g>"#);
        let _ = write!(
            out,
            r#"<g class="edgeLabel"><g class="label" data-id="{}" transform="translate(0, 0)">"#,
            escape_attr_display(dom_id)
        );
        write_class_svg_edge_text(out, "", false);
        out.push_str("</g></g>");
    } else if let Some(lbl) = label {
        let _ = write!(
            out,
            r#"<g class="edgeLabel" transform="translate({}, {})"><g class="label" data-id="{}" transform="translate({}, {})"><g><rect class="background" style="" x="-2" y="-1" width="{}" height="{}"/>"#,
            fmt(center_x),
            fmt(center_y),
            escape_attr_display(dom_id),
            fmt(-lbl.width / 2.0),
            fmt(-lbl.height / 2.0),
            fmt(lbl.width.max(0.0)),
            fmt(lbl.height.max(0.0)),
        );
        write_class_svg_edge_text_markdown(out, trimmed, true);
        out.push_str("</g></g></g>");
    } else {
        out.push_str(r#"<g><rect class="background" style="stroke: none"/></g>"#);
        let _ = write!(
            out,
            r#"<g class="edgeLabel"><g class="label" data-id="{}" transform="translate(0, 0)">"#,
            escape_attr_display(dom_id)
        );
        write_class_svg_edge_text(out, trimmed, false);
        out.push_str("</g></g>");
    }
}

pub(super) fn class_terminal_box_size(text: &str) -> (f64, f64) {
    let decoded = decode_entities_minimal_cow(text);
    let trimmed = decoded.trim();
    if trimmed.is_empty() {
        return (0.0, 0.0);
    }
    (trimmed.chars().count() as f64 * 9.0, 12.0)
}

pub(super) fn render_class_edge_terminal_group(
    out: &mut String,
    x: f64,
    y: f64,
    text: &str,
    is_start_terminal: bool,
) {
    let decoded = decode_entities_minimal_cow(text);
    let trimmed = decoded.trim();
    if trimmed.is_empty() {
        return;
    }
    let (width, height) = class_terminal_box_size(trimmed);
    if is_start_terminal {
        let _ = write!(
            out,
            r#"<g class="edgeTerminals" transform="translate({}, {})"><g class="inner" transform="translate(0, 0)"><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="display: inline-block; padding-right: {}px; white-space: nowrap;"><span class="edgeLabel"><p>"#,
            fmt(x),
            fmt(y),
            fmt(width),
            fmt(height),
            class_text_overrides::class_html_span_padding_right_px(),
        );
        escape_xml_into(out, trimmed);
        out.push_str("</p></span></div></foreignObject></g></g>");
    } else {
        let _ = write!(
            out,
            r#"<g class="edgeTerminals" transform="translate({}, {})"><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="display: inline-block; padding-right: {}px; white-space: nowrap;"><span class="edgeLabel"><p>"#,
            fmt(x),
            fmt(y),
            fmt(width),
            fmt(height),
            class_text_overrides::class_html_span_padding_right_px(),
        );
        escape_xml_into(out, trimmed);
        out.push_str(r#"</p></span></div></foreignObject><g class="inner" transform="translate(0, 0)"/></g>"#);
    }
}

pub(super) fn class_edge_dom_id_into(
    out: &mut String,
    edge: &LayoutEdge,
    relation_index_by_id: &FxHashMap<&str, usize>,
) {
    out.clear();
    if edge.id.starts_with("edgeNote") {
        if let Some(note_idx) = edge
            .from
            .strip_prefix("note")
            .and_then(|rest| rest.parse::<usize>().ok())
        {
            let _ = write!(out, "edgeNote{note_idx}");
            return;
        }
        out.push_str(edge.id.as_str());
        return;
    }
    // Mermaid uses `getEdgeId` with prefix `id`.
    let idx = relation_index_by_id
        .get(edge.id.as_str())
        .copied()
        .unwrap_or(1);
    out.push_str("id_");
    out.push_str(edge.from.as_str());
    out.push('_');
    out.push_str(edge.to.as_str());
    out.push('_');
    let _ = write!(out, "{idx}");
}

pub(super) fn class_edge_pattern(line_type: i32) -> &'static str {
    // Mermaid class diagram `lineType` uses "dottedLine" for `..` which maps to the dashed pattern.
    if line_type == 1 {
        "edge-pattern-dashed"
    } else {
        "edge-pattern-solid"
    }
}

pub(super) fn class_note_edge_pattern() -> &'static str {
    "edge-pattern-dotted"
}

pub(super) fn class_edge_path_style(edge_id: &str) -> &'static str {
    if edge_id.starts_with("edgeNote") {
        "fill: none;;;fill: none"
    } else {
        ";;;"
    }
}

pub(super) fn class_edge_render_order<'a>(
    edges: &'a [LayoutEdge],
    relation_index_by_id: &FxHashMap<&str, usize>,
) -> Vec<&'a LayoutEdge> {
    let mut ordered = edges.iter().collect::<Vec<_>>();
    ordered.sort_by(|a, b| {
        let a_key = if a.id.starts_with("edgeNote") {
            (
                0_u8,
                a.id.trim_start_matches("edgeNote")
                    .parse::<usize>()
                    .unwrap_or(usize::MAX),
                a.id.as_str(),
            )
        } else {
            (
                1_u8,
                relation_index_by_id
                    .get(a.id.as_str())
                    .copied()
                    .unwrap_or(usize::MAX),
                a.id.as_str(),
            )
        };
        let b_key = if b.id.starts_with("edgeNote") {
            (
                0_u8,
                b.id.trim_start_matches("edgeNote")
                    .parse::<usize>()
                    .unwrap_or(usize::MAX),
                b.id.as_str(),
            )
        } else {
            (
                1_u8,
                relation_index_by_id
                    .get(b.id.as_str())
                    .copied()
                    .unwrap_or(usize::MAX),
                b.id.as_str(),
            )
        };
        a_key.cmp(&b_key)
    });
    ordered
}
