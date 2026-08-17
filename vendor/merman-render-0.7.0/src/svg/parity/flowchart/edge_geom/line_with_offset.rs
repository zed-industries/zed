//! Mermaid `lineWithOffset` port.
//!
//! Mermaid shortens edge paths so markers don't render on top of the line (see
//! `packages/mermaid/src/utils/lineWithOffset.ts`).

fn marker_offset_for(arrow_type: Option<&str>) -> Option<f64> {
    match arrow_type {
        Some("arrow_point") => Some(4.0),
        Some("dependency") => Some(6.0),
        Some("lollipop") => Some(13.5),
        Some("aggregation" | "extension" | "composition") => Some(17.25),
        _ => None,
    }
}

pub(in crate::svg::parity::flowchart) fn line_with_offset_points(
    input: &[crate::model::LayoutPoint],
    arrow_type_start: Option<&str>,
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

    if input.len() < 2 {
        return input.to_vec();
    }

    let start = &input[0];
    let end = &input[input.len() - 1];

    let x_direction_is_left = start.x < end.x;
    let y_direction_is_down = start.y < end.y;
    let extra_room = 1.0;

    let start_marker_height = marker_offset_for(arrow_type_start);
    let end_marker_height = marker_offset_for(arrow_type_end);

    let mut out = Vec::with_capacity(input.len());
    for (i, p) in input.iter().enumerate() {
        let mut ox = 0.0;
        let mut oy = 0.0;

        if i == 0 {
            if let Some(h) = start_marker_height {
                let (angle, delta_x, delta_y) = calculate_delta_and_angle(&input[0], &input[1]);
                ox = h * angle.cos() * if delta_x >= 0.0 { 1.0 } else { -1.0 };
                oy = h * angle.sin().abs() * if delta_y >= 0.0 { 1.0 } else { -1.0 };
            }
        } else if i == input.len() - 1 {
            if let Some(h) = end_marker_height {
                let (angle, delta_x, delta_y) =
                    calculate_delta_and_angle(&input[input.len() - 1], &input[input.len() - 2]);
                ox = h * angle.cos() * if delta_x >= 0.0 { 1.0 } else { -1.0 };
                oy = h * angle.sin().abs() * if delta_y >= 0.0 { 1.0 } else { -1.0 };
            }
        }

        if let Some(h) = end_marker_height {
            let diff_x = (p.x - end.x).abs();
            let diff_y = (p.y - end.y).abs();
            if diff_x < h && diff_x > 0.0 && diff_y < h {
                let mut adjustment = h + extra_room - diff_x;
                adjustment *= if !x_direction_is_left { -1.0 } else { 1.0 };
                ox -= adjustment;
            }
        }
        if let Some(h) = start_marker_height {
            let diff_x = (p.x - start.x).abs();
            let diff_y = (p.y - start.y).abs();
            if diff_x < h && diff_x > 0.0 && diff_y < h {
                let mut adjustment = h + extra_room - diff_x;
                adjustment *= if !x_direction_is_left { -1.0 } else { 1.0 };
                ox += adjustment;
            }
        }

        if let Some(h) = end_marker_height {
            let diff_y = (p.y - end.y).abs();
            let diff_x = (p.x - end.x).abs();
            if diff_y < h && diff_y > 0.0 && diff_x < h {
                let mut adjustment = h + extra_room - diff_y;
                adjustment *= if !y_direction_is_down { -1.0 } else { 1.0 };
                oy -= adjustment;
            }
        }
        if let Some(h) = start_marker_height {
            let diff_y = (p.y - start.y).abs();
            let diff_x = (p.x - start.x).abs();
            if diff_y < h && diff_y > 0.0 && diff_x < h {
                let mut adjustment = h + extra_room - diff_y;
                adjustment *= if !y_direction_is_down { -1.0 } else { 1.0 };
                oy += adjustment;
            }
        }

        out.push(crate::model::LayoutPoint {
            x: p.x + ox,
            y: p.y + oy,
        });
    }

    out
}

pub(in crate::svg::parity::flowchart) fn rounded_line_with_marker_offsets_points(
    input: &[crate::model::LayoutPoint],
    arrow_type_start: Option<&str>,
    arrow_type_end: Option<&str>,
) -> Vec<crate::model::LayoutPoint> {
    let mut out = input.to_vec();
    if input.len() < 2 {
        return out;
    }

    if let Some(offset) = marker_offset_for(arrow_type_start) {
        let p1 = &input[0];
        let p2 = &input[1];
        let angle = (p2.y - p1.y).atan2(p2.x - p1.x);
        out[0].x = p1.x + offset * angle.cos();
        out[0].y = p1.y + offset * angle.sin();
    }

    let n = input.len();
    if let Some(offset) = marker_offset_for(arrow_type_end) {
        let p1 = &input[n - 1];
        let p2 = &input[n - 2];
        let angle = (p1.y - p2.y).atan2(p1.x - p2.x);
        out[n - 1].x = p1.x - offset * angle.cos();
        out[n - 1].y = p1.y - offset * angle.sin();
    }

    out
}

pub(in crate::svg::parity::flowchart) fn maybe_snap_shallow_basis_triplet_y_to_f32(
    points: &mut [crate::model::LayoutPoint],
    edge_type: Option<&str>,
) {
    let (_, arrow_type_end) = arrow_types_for_edge(edge_type);
    if arrow_type_end != Some("arrow_point") || points.len() != 3 {
        return;
    }

    let [p0, p1, p2] = points else {
        return;
    };

    if (p0.y - p1.y).abs() > 1e-9 {
        return;
    }

    if ((p2.y - p1.y).abs() - 0.5).abs() > 1e-6 {
        return;
    }

    fn snap_if_close_to_f32(v: f64) -> Option<f64> {
        if !v.is_finite() {
            return None;
        }
        let snapped = (v as f32) as f64;
        if !snapped.is_finite() || snapped + 1e-12 < v || (v - snapped).abs() > 1e-5 {
            return None;
        }
        Some(snapped)
    }

    let (Some(y0), Some(y1), Some(y2)) = (
        snap_if_close_to_f32(p0.y),
        snap_if_close_to_f32(p1.y),
        snap_if_close_to_f32(p2.y),
    ) else {
        return;
    };

    if (y0 - y1).abs() > 1e-9 || ((y2 - y1).abs() - 0.5).abs() > 1e-5 {
        return;
    }

    p0.y = y0;
    p1.y = y1;
    p2.y = y2;
}

pub(in crate::svg::parity::flowchart) fn arrow_types_for_edge(
    edge_type: Option<&str>,
) -> (Option<&'static str>, Option<&'static str>) {
    let arrow_type_start = match edge_type {
        Some("double_arrow_point") => Some("arrow_point"),
        Some("double_arrow_circle") => Some("arrow_circle"),
        Some("double_arrow_cross") => Some("arrow_cross"),
        _ => None,
    };
    let arrow_type_end = match edge_type {
        Some("arrow_open") => None,
        Some("arrow_cross") => Some("arrow_cross"),
        Some("arrow_circle") => Some("arrow_circle"),
        Some("double_arrow_point" | "arrow_point") => Some("arrow_point"),
        Some("double_arrow_circle") => Some("arrow_circle"),
        Some("double_arrow_cross") => Some("arrow_cross"),
        _ => Some("arrow_point"),
    };

    (arrow_type_start, arrow_type_end)
}

pub(in crate::svg::parity::flowchart) fn line_with_offset_for_edge_type(
    input: &[crate::model::LayoutPoint],
    edge_type: Option<&str>,
) -> Vec<crate::model::LayoutPoint> {
    let (arrow_type_start, arrow_type_end) = arrow_types_for_edge(edge_type);
    line_with_offset_points(input, arrow_type_start, arrow_type_end)
}

pub(in crate::svg::parity::flowchart) fn rounded_line_with_marker_offsets_for_edge_type(
    input: &[crate::model::LayoutPoint],
    edge_type: Option<&str>,
) -> Vec<crate::model::LayoutPoint> {
    let (arrow_type_start, arrow_type_end) = arrow_types_for_edge(edge_type);
    rounded_line_with_marker_offsets_points(input, arrow_type_start, arrow_type_end)
}
