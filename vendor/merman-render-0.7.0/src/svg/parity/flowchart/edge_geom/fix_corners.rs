//! Mermaid `fixCorners` port.
//!
//! Mermaid's flowchart edge rendering inserts small offset points around orthogonal corners
//! before feeding the polyline into D3's line generator. This helps avoid sharp right-angle
//! corners when using curved interpolators.

pub(in crate::svg::parity::flowchart) fn maybe_fix_corners(
    points: &mut Vec<crate::model::LayoutPoint>,
) {
    if points.is_empty() {
        return;
    }

    const CORNER_DIST: f64 = 5.0;
    let mut corner_positions: Vec<usize> = Vec::new();
    for i in 1..points.len().saturating_sub(1) {
        let prev = &points[i - 1];
        let curr = &points[i];
        let next = &points[i + 1];

        let is_corner_xy = prev.x == curr.x
            && curr.y == next.y
            && (curr.x - next.x).abs() > CORNER_DIST
            && (curr.y - prev.y).abs() > CORNER_DIST;
        let is_corner_yx = prev.y == curr.y
            && curr.x == next.x
            && (curr.x - prev.x).abs() > CORNER_DIST
            && (curr.y - next.y).abs() > CORNER_DIST;

        if is_corner_xy || is_corner_yx {
            corner_positions.push(i);
        }
    }

    if corner_positions.is_empty() {
        return;
    }

    fn find_adjacent_point(
        point_a: &crate::model::LayoutPoint,
        point_b: &crate::model::LayoutPoint,
        distance: f64,
    ) -> crate::model::LayoutPoint {
        let x_diff = point_b.x - point_a.x;
        let y_diff = point_b.y - point_a.y;
        let len = (x_diff * x_diff + y_diff * y_diff).sqrt();
        if len == 0.0 {
            return point_b.clone();
        }
        let ratio = distance / len;
        crate::model::LayoutPoint {
            x: point_b.x - ratio * x_diff,
            y: point_b.y - ratio * y_diff,
        }
    }

    let a = (2.0_f64).sqrt() * 2.0;
    let mut out: Vec<crate::model::LayoutPoint> = Vec::new();
    for i in 0..points.len() {
        if !corner_positions.contains(&i) {
            out.push(points[i].clone());
            continue;
        }

        let prev = &points[i - 1];
        let next = &points[i + 1];
        let corner = &points[i];
        let new_prev = find_adjacent_point(prev, corner, CORNER_DIST);
        let new_next = find_adjacent_point(next, corner, CORNER_DIST);
        let x_diff = new_next.x - new_prev.x;
        let y_diff = new_next.y - new_prev.y;

        out.push(new_prev.clone());

        let mut new_corner = corner.clone();
        if (next.x - prev.x).abs() > 10.0 && (next.y - prev.y).abs() >= 10.0 {
            let r = CORNER_DIST;
            if corner.x == new_prev.x {
                new_corner = crate::model::LayoutPoint {
                    x: if x_diff < 0.0 {
                        new_prev.x - r + a
                    } else {
                        new_prev.x + r - a
                    },
                    y: if y_diff < 0.0 {
                        new_prev.y - a
                    } else {
                        new_prev.y + a
                    },
                };
            } else {
                new_corner = crate::model::LayoutPoint {
                    x: if x_diff < 0.0 {
                        new_prev.x - a
                    } else {
                        new_prev.x + a
                    },
                    y: if y_diff < 0.0 {
                        new_prev.y - r + a
                    } else {
                        new_prev.y + r - a
                    },
                };
            }
        }

        out.push(new_corner);
        out.push(new_next);
    }

    *points = out;
}
