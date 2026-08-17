use super::BoundaryNode;

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

pub(in crate::svg::parity::flowchart) fn cut_path_at_intersect_into(
    input: &[crate::model::LayoutPoint],
    boundary: &BoundaryNode,
    out: &mut Vec<crate::model::LayoutPoint>,
) {
    out.clear();
    if input.is_empty() {
        return;
    }
    out.reserve(input.len() + 1);

    let mut last_point_outside = input[0].clone();
    let mut is_inside = false;
    const EPS: f64 = 1e-9;

    for point in input {
        if !outside_node(boundary, point) && !is_inside {
            let inter = rect_intersection(boundary, &last_point_outside, point);
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
                out.push(crate::model::LayoutPoint {
                    x: point.x,
                    y: point.y,
                });
            }
        }
    }
}

pub(in crate::svg::parity::flowchart) fn dedup_consecutive_points_into(
    input: &[crate::model::LayoutPoint],
    out: &mut Vec<crate::model::LayoutPoint>,
) {
    out.clear();
    if input.is_empty() {
        return;
    }
    const EPS: f64 = 1e-9;
    out.reserve(input.len());
    for p in input {
        if out
            .last()
            .is_some_and(|prev| (prev.x - p.x).abs() <= EPS && (prev.y - p.y).abs() <= EPS)
        {
            continue;
        }
        out.push(crate::model::LayoutPoint { x: p.x, y: p.y });
    }
}
