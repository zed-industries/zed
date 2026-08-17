use super::super::*;
use super::BoundaryNode;

pub(in crate::svg::parity::flowchart) fn is_rounded_intersect_shift_shape(
    layout_shape: Option<&str>,
) -> bool {
    matches!(layout_shape, Some("roundedRect" | "rounded" | "event"))
}

pub(in crate::svg::parity::flowchart) fn is_polygon_layout_shape(
    layout_shape: Option<&str>,
) -> bool {
    matches!(
        layout_shape,
        Some(
            "hexagon"
                | "hex"
                | "prepare"
                | "odd"
                | "rect_left_inv_arrow"
                | "stadium"
                | "terminal"
                | "pill"
                | "subroutine"
                | "fr-rect"
                | "subproc"
                | "subprocess"
                | "framed-rectangle"
                | "lean_right"
                | "lean-r"
                | "lean-right"
                | "in-out"
                | "lean_left"
                | "lean-l"
                | "lean-left"
                | "out-in"
                | "trapezoid"
                | "trap-b"
                | "priority"
                | "trapezoid-bottom"
                | "inv_trapezoid"
                | "inv-trapezoid"
                | "trap-t"
                | "manual"
                | "trapezoid-top"
                | "curv-trap"
                | "display"
                | "curved-trapezoid"
                | "tri"
                | "extract"
                | "triangle"
                | "manual-file"
                | "flipped-triangle"
                | "flip-tri"
                | "manual-input"
                | "sloped-rectangle"
                | "sl-rect"
                | "hourglass"
                | "collate"
        )
    )
}

pub(in crate::svg::parity::flowchart) fn force_intersect_for_layout_shape(
    layout_shape: Option<&str>,
) -> bool {
    matches!(
        layout_shape,
        Some(
            "circle"
                | "diamond"
                | "diam"
                | "question"
                | "decision"
                | "roundedRect"
                | "rounded"
                | "cylinder"
                | "cyl"
                | "db"
                | "database"
                | "lin-cyl"
                | "disk"
                | "lined-cylinder"
                | "h-cyl"
                | "das"
                | "horizontal-cylinder"
                | "cross-circ"
                | "summary"
                | "crossed-circle"
                | "bow-rect"
                | "stored-data"
                | "bow-tie-rectangle"
                | "tag-rect"
                | "tagged-rectangle"
                | "tag-proc"
                | "tagged-process"
                | "doc"
                | "document"
                | "delay"
                | "half-rounded-rectangle"
                | "notch-pent"
                | "loop-limit"
                | "notched-pentagon"
                | "docs"
                | "documents"
                | "st-doc"
                | "stacked-document"
                | "tag-doc"
                | "tagged-document"
                | "bolt"
                | "com-link"
                | "lightning-bolt"
                | "f-circ"
                | "junction"
                | "filled-circle"
                | "win-pane"
                | "internal-storage"
                | "window-pane"
                | "stadium"
                | "terminal"
                | "pill",
        )
    ) || is_polygon_layout_shape(layout_shape)
}

fn intersect_rect(
    node: &BoundaryNode,
    point: &crate::model::LayoutPoint,
) -> crate::model::LayoutPoint {
    let x = node.x;
    let y = node.y;
    let dx = point.x - x;
    let dy = point.y - y;
    let mut w = node.width / 2.0;
    let mut h = node.height / 2.0;

    let (sx, sy) = if dy.abs() * w > dx.abs() * h {
        if dy < 0.0 {
            h = -h;
        }
        let sx = if dy == 0.0 { 0.0 } else { (h * dx) / dy };
        (sx, h)
    } else {
        if dx < 0.0 {
            w = -w;
        }
        let sy = if dx == 0.0 { 0.0 } else { (w * dy) / dx };
        (w, sy)
    };

    crate::model::LayoutPoint {
        x: x + sx,
        y: y + sy,
    }
}

fn intersect_circle(
    node: &BoundaryNode,
    point: &crate::model::LayoutPoint,
) -> crate::model::LayoutPoint {
    let dx = point.x - node.x;
    let dy = point.y - node.y;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist <= 1e-12 {
        return crate::model::LayoutPoint {
            x: node.x,
            y: node.y,
        };
    }
    let r = (node.width.min(node.height) / 2.0).max(0.0);
    crate::model::LayoutPoint {
        x: node.x + dx / dist * r,
        y: node.y + dy / dist * r,
    }
}

fn intersect_diamond(
    node: &BoundaryNode,
    point: &crate::model::LayoutPoint,
) -> crate::model::LayoutPoint {
    let vx = point.x - node.x;
    let vy = point.y - node.y;
    if !(vx.is_finite() && vy.is_finite()) {
        return crate::model::LayoutPoint {
            x: node.x,
            y: node.y,
        };
    }
    if vx.abs() <= 1e-12 && vy.abs() <= 1e-12 {
        return crate::model::LayoutPoint {
            x: node.x,
            y: node.y,
        };
    }
    let hw = (node.width / 2.0).max(1e-9);
    let hh = (node.height / 2.0).max(1e-9);
    let denom = vx.abs() / hw + vy.abs() / hh;
    if !(denom.is_finite() && denom > 0.0) {
        return crate::model::LayoutPoint {
            x: node.x,
            y: node.y,
        };
    }
    let t = 1.0 / denom;
    crate::model::LayoutPoint {
        x: node.x + vx * t,
        y: node.y + vy * t,
    }
}

fn intersect_cylinder(
    node: &BoundaryNode,
    point: &crate::model::LayoutPoint,
) -> crate::model::LayoutPoint {
    // Port of Mermaid `cylinder.ts` intersection logic (11.12.2):
    // - start from `intersect.rect(node, point)`,
    // - then adjust y when the intersection hits the curved top/bottom ellipses.
    let mut pos = intersect_rect(node, point);
    let x = pos.x - node.x;

    let w = node.width.max(1.0);
    let rx = w / 2.0;
    let ry = rx / (2.5 + w / 50.0);

    if rx != 0.0
        && (x.abs() < w / 2.0
            || ((x.abs() - w / 2.0).abs() < 1e-12
                && (pos.y - node.y).abs() > node.height / 2.0 - ry))
    {
        let mut y = ry * ry * (1.0 - (x * x) / (rx * rx));
        if y > 0.0 {
            y = y.sqrt();
        } else {
            y = 0.0;
        }
        y = ry - y;
        if point.y - node.y > 0.0 {
            y = -y;
        }
        pos.y += y;
    }

    pos
}

fn intersect_tilted_cylinder(
    node: &BoundaryNode,
    point: &crate::model::LayoutPoint,
) -> crate::model::LayoutPoint {
    // Port of Mermaid `tiltedCylinder.ts` intersection logic (11.12.2):
    // - start from `intersect.rect(node, point)`,
    // - then adjust x when the intersection hits the curved ends.
    let mut pos = intersect_rect(node, point);
    let y = pos.y - node.y;
    let top_or_bottom_center =
        (point.x - node.x).abs() < 1e-6 && (pos.x - node.x).abs() < 1e-6 && {
            let half_h = node.height / 2.0;
            (y.abs() - half_h).abs() < 1e-6
        };
    if top_or_bottom_center {
        return pos;
    }

    let ry = node.height / 2.0;
    let rx = if ry == 0.0 {
        0.0
    } else {
        ry / (2.5 + node.height / 50.0)
    };

    if ry != 0.0
        && (y.abs() < node.height / 2.0
            || (y.abs() == node.height / 2.0 && (pos.x - node.x).abs() > node.width / 2.0 - rx))
    {
        let mut x = rx * rx * (1.0 - (y * y) / (ry * ry));
        if x != 0.0 {
            x = x.abs().sqrt();
        }
        x = rx - x;
        if point.x - node.x > 0.0 {
            x = -x;
        }
        pos.x += x;
    }

    pos
}

fn intersect_line(
    p1: crate::model::LayoutPoint,
    p2: crate::model::LayoutPoint,
    q1: crate::model::LayoutPoint,
    q2: crate::model::LayoutPoint,
) -> Option<crate::model::LayoutPoint> {
    // Port of Mermaid `intersect-line.js` (11.12.2).
    //
    // This does segment intersection with a "denom/2" offset rounding that materially affects
    // flowchart endpoints and thus SVG `viewBox`/`max-width` parity.
    let a1 = p2.y - p1.y;
    let b1 = p1.x - p2.x;
    let c1 = p2.x * p1.y - p1.x * p2.y;

    let r3 = a1 * q1.x + b1 * q1.y + c1;
    let r4 = a1 * q2.x + b1 * q2.y + c1;

    fn same_sign(r1: f64, r2: f64) -> bool {
        r1 * r2 > 0.0
    }

    if r3 != 0.0 && r4 != 0.0 && same_sign(r3, r4) {
        return None;
    }

    let a2 = q2.y - q1.y;
    let b2 = q1.x - q2.x;
    let c2 = q2.x * q1.y - q1.x * q2.y;

    let r1 = a2 * p1.x + b2 * p1.y + c2;
    let r2 = a2 * p2.x + b2 * p2.y + c2;

    // Match Mermaid@11.12.2 `intersect-line.js`: the side test is an exact `!== 0` guard.
    // Keep this exact check so our segment intersection matches upstream for collinear and
    // endpoint cases (flowing into strict SVG `data-points` parity).
    if r1 != 0.0 && r2 != 0.0 && same_sign(r1, r2) {
        return None;
    }

    let denom = a1 * b2 - a2 * b1;
    if denom == 0.0 {
        return None;
    }

    let offset = (denom / 2.0).abs();

    let mut num = b1 * c2 - b2 * c1;
    let x = if num < 0.0 {
        (num - offset) / denom
    } else {
        (num + offset) / denom
    };

    num = a2 * c1 - a1 * c2;
    let y = if num < 0.0 {
        (num - offset) / denom
    } else {
        (num + offset) / denom
    };

    Some(crate::model::LayoutPoint { x, y })
}

fn intersect_line_mermaid_buggy_second_side(
    p1: crate::model::LayoutPoint,
    p2: crate::model::LayoutPoint,
    q1: crate::model::LayoutPoint,
    q2: crate::model::LayoutPoint,
) -> Option<crate::model::LayoutPoint> {
    let a1 = p2.y - p1.y;
    let b1 = p1.x - p2.x;
    let c1 = p2.x * p1.y - p1.x * p2.y;

    let r3 = a1 * q1.x + b1 * q1.y + c1;
    let r4 = a1 * q2.x + b1 * q2.y + c1;

    fn same_sign(r1: f64, r2: f64) -> bool {
        r1 * r2 > 0.0
    }

    if r3 != 0.0 && r4 != 0.0 && same_sign(r3, r4) {
        return None;
    }

    let a2 = q2.y - q1.y;
    let b2 = q1.x - q2.x;
    let c2 = q2.x * q1.y - q1.x * q2.y;

    let r1 = a2 * p1.x + b2 * p1.y + c2;
    let r2 = a2 * p2.x + b2 * p2.y + c2;

    let epsilon = 1e-6;
    if r1.abs() < epsilon && r2.abs() < epsilon && same_sign(r1, r2) {
        return None;
    }

    let denom = a1 * b2 - a2 * b1;
    if denom == 0.0 {
        return None;
    }

    let offset = (denom / 2.0).abs();

    let mut num = b1 * c2 - b2 * c1;
    let x = if num < 0.0 {
        (num - offset) / denom
    } else {
        (num + offset) / denom
    };

    num = a2 * c1 - a1 * c2;
    let y = if num < 0.0 {
        (num - offset) / denom
    } else {
        (num + offset) / denom
    };

    Some(crate::model::LayoutPoint { x, y })
}

fn intersect_polygon(
    node: &BoundaryNode,
    poly_points: &[crate::model::LayoutPoint],
    point: &crate::model::LayoutPoint,
) -> crate::model::LayoutPoint {
    // Port of Mermaid `intersect-polygon.js` (11.12.2).
    let x1 = node.x;
    let y1 = node.y;

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    for p in poly_points {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
    }

    let left = x1 - node.width / 2.0 - min_x;
    let top = y1 - node.height / 2.0 - min_y;

    let mut intersections: Vec<crate::model::LayoutPoint> = Vec::new();
    for i in 0..poly_points.len() {
        let p1 = &poly_points[i];
        let p2 = &poly_points[if i + 1 < poly_points.len() { i + 1 } else { 0 }];
        let q1 = crate::model::LayoutPoint {
            x: left + p1.x,
            y: top + p1.y,
        };
        let q2 = crate::model::LayoutPoint {
            x: left + p2.x,
            y: top + p2.y,
        };
        if let Some(inter) = intersect_line(
            crate::model::LayoutPoint { x: x1, y: y1 },
            point.clone(),
            q1,
            q2,
        ) {
            intersections.push(inter);
        }
    }

    if intersections.is_empty() {
        return crate::model::LayoutPoint { x: x1, y: y1 };
    }

    if intersections.len() > 1 {
        intersections.sort_by(|p, q| {
            let pdx = p.x - point.x;
            let pdy = p.y - point.y;
            let qdx = q.x - point.x;
            let qdy = q.y - point.y;
            let dist_p = (pdx * pdx + pdy * pdy).sqrt();
            let dist_q = (qdx * qdx + qdy * qdy).sqrt();
            dist_p
                .partial_cmp(&dist_q)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    intersections[0].clone()
}

fn intersect_polygon_hourglass(
    node: &BoundaryNode,
    point: &crate::model::LayoutPoint,
) -> crate::model::LayoutPoint {
    let poly_points = [
        crate::model::LayoutPoint { x: 0.0, y: 0.0 },
        crate::model::LayoutPoint {
            x: node.width.max(1.0),
            y: 0.0,
        },
        crate::model::LayoutPoint {
            x: 0.0,
            y: node.height.max(1.0),
        },
        crate::model::LayoutPoint {
            x: node.width.max(1.0),
            y: node.height.max(1.0),
        },
    ];

    let x1 = node.x;
    let y1 = node.y;

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    for p in &poly_points {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
    }

    let left = x1 - node.width / 2.0 - min_x;
    let top = y1 - node.height / 2.0 - min_y;

    let mut intersections: Vec<crate::model::LayoutPoint> = Vec::new();
    for i in 0..poly_points.len() {
        let p1 = &poly_points[i];
        let p2 = &poly_points[if i + 1 < poly_points.len() { i + 1 } else { 0 }];
        let q1 = crate::model::LayoutPoint {
            x: left + p1.x,
            y: top + p1.y,
        };
        let q2 = crate::model::LayoutPoint {
            x: left + p2.x,
            y: top + p2.y,
        };
        if let Some(inter) = intersect_line_mermaid_buggy_second_side(
            crate::model::LayoutPoint { x: x1, y: y1 },
            point.clone(),
            q1,
            q2,
        ) {
            intersections.push(inter);
        }
    }

    if intersections.is_empty() {
        return crate::model::LayoutPoint { x: x1, y: y1 };
    }

    if intersections.len() > 1 {
        intersections.sort_by(|p, q| {
            let pdx = p.x - point.x;
            let pdy = p.y - point.y;
            let qdx = q.x - point.x;
            let qdy = q.y - point.y;
            let dist_p = (pdx * pdx + pdy * pdy).sqrt();
            let dist_q = (qdx * qdx + qdy * qdy).sqrt();
            dist_p
                .partial_cmp(&dist_q)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    intersections[0].clone()
}

fn polygon_points_for_layout_shape(
    layout_shape: &str,
    node: &BoundaryNode,
) -> Option<Vec<crate::model::LayoutPoint>> {
    let w = node.width.max(1.0);
    let h = node.height.max(1.0);

    match layout_shape {
        // Mermaid "odd" nodes (`>... ]`) are rendered using `rect_left_inv_arrow`.
        //
        // Reference: Mermaid@11.12.2 `rectLeftInvArrow.ts`.
        //
        // Note: Flowchart layout dimensions model this as `node.width = w + h/4`, where `w`
        // corresponds to Mermaid's `w = max(bbox.width + padding, node.width)` prior to the
        // `updateNodeBounds(...)` bbox expansion.
        "odd" | "rect_left_inv_arrow" => {
            let base_w = (w - h / 4.0).max(1.0);
            let x = -base_w / 2.0;
            let y = -h / 2.0;
            let notch = y / 2.0; // negative
            Some(vec![
                crate::model::LayoutPoint { x: x + notch, y },
                crate::model::LayoutPoint { x, y: 0.0 },
                crate::model::LayoutPoint {
                    x: x + notch,
                    y: -y,
                },
                crate::model::LayoutPoint { x: -x, y: -y },
                crate::model::LayoutPoint { x: -x, y },
            ])
        }
        "subroutine" | "fr-rect" | "subproc" | "subprocess" | "framed-rectangle" => {
            // Port of Mermaid@11.12.2 `subroutine.ts` points used for polygon intersection.
            //
            // Mermaid's insertPolygonShape(...) uses `w = bbox.width + padding` but the
            // resulting bbox expands by `offset*2` (=16px) due to the outer frame.
            let inner_w = (w - 16.0).max(1.0);
            Some(vec![
                crate::model::LayoutPoint { x: 0.0, y: 0.0 },
                crate::model::LayoutPoint { x: inner_w, y: 0.0 },
                crate::model::LayoutPoint { x: inner_w, y: -h },
                crate::model::LayoutPoint { x: 0.0, y: -h },
                crate::model::LayoutPoint { x: 0.0, y: 0.0 },
                crate::model::LayoutPoint { x: -8.0, y: 0.0 },
                crate::model::LayoutPoint {
                    x: inner_w + 8.0,
                    y: 0.0,
                },
                crate::model::LayoutPoint {
                    x: inner_w + 8.0,
                    y: -h,
                },
                crate::model::LayoutPoint { x: -8.0, y: -h },
                crate::model::LayoutPoint { x: -8.0, y: 0.0 },
            ])
        }
        "hexagon" | "hex" | "prepare" => {
            let half_width = w / 2.0;
            let half_height = h / 2.0;
            let fixed_length = half_height / 2.0;
            let deduced_width = half_width - fixed_length;
            Some(vec![
                crate::model::LayoutPoint {
                    x: -deduced_width,
                    y: -half_height,
                },
                crate::model::LayoutPoint {
                    x: 0.0,
                    y: -half_height,
                },
                crate::model::LayoutPoint {
                    x: deduced_width,
                    y: -half_height,
                },
                crate::model::LayoutPoint {
                    x: half_width,
                    y: 0.0,
                },
                crate::model::LayoutPoint {
                    x: deduced_width,
                    y: half_height,
                },
                crate::model::LayoutPoint {
                    x: 0.0,
                    y: half_height,
                },
                crate::model::LayoutPoint {
                    x: -deduced_width,
                    y: half_height,
                },
                crate::model::LayoutPoint {
                    x: -half_width,
                    y: 0.0,
                },
            ])
        }
        "lean_right" | "lean-r" | "lean-right" | "in-out" => {
            let total_w = w;
            let w = (total_w - h).max(1.0);
            let dx = (3.0 * h) / 6.0;
            Some(vec![
                crate::model::LayoutPoint { x: -dx, y: 0.0 },
                crate::model::LayoutPoint { x: w, y: 0.0 },
                crate::model::LayoutPoint { x: w + dx, y: -h },
                crate::model::LayoutPoint { x: 0.0, y: -h },
            ])
        }
        "lean_left" | "lean-l" | "lean-left" | "out-in" => {
            let total_w = w;
            let w = (total_w - h).max(1.0);
            let dx = (3.0 * h) / 6.0;
            Some(vec![
                crate::model::LayoutPoint { x: 0.0, y: 0.0 },
                crate::model::LayoutPoint { x: w + dx, y: 0.0 },
                crate::model::LayoutPoint { x: w, y: -h },
                crate::model::LayoutPoint { x: -dx, y: -h },
            ])
        }
        "trapezoid" | "trap-b" | "priority" | "trapezoid-bottom" => {
            let total_w = w;
            let w = (total_w - h).max(1.0);
            let dx = (3.0 * h) / 6.0;
            Some(vec![
                crate::model::LayoutPoint { x: -dx, y: 0.0 },
                crate::model::LayoutPoint { x: w + dx, y: 0.0 },
                crate::model::LayoutPoint { x: w, y: -h },
                crate::model::LayoutPoint { x: 0.0, y: -h },
            ])
        }
        "inv_trapezoid" | "inv-trapezoid" | "trap-t" | "manual" | "trapezoid-top" => {
            let total_w = w;
            let w = (total_w - h).max(1.0);
            let dx = (3.0 * h) / 6.0;
            Some(vec![
                crate::model::LayoutPoint { x: 0.0, y: 0.0 },
                crate::model::LayoutPoint { x: w, y: 0.0 },
                crate::model::LayoutPoint { x: w + dx, y: -h },
                crate::model::LayoutPoint { x: -dx, y: -h },
            ])
        }
        "tri" | "extract" | "triangle" => Some(vec![
            crate::model::LayoutPoint { x: 0.0, y: 0.0 },
            crate::model::LayoutPoint { x: h, y: 0.0 },
            crate::model::LayoutPoint { x: h / 2.0, y: -h },
        ]),
        "manual-file" | "flipped-triangle" | "flip-tri" => Some(vec![
            crate::model::LayoutPoint { x: 0.0, y: -h },
            crate::model::LayoutPoint { x: h, y: -h },
            crate::model::LayoutPoint { x: h / 2.0, y: 0.0 },
        ]),
        "manual-input" | "sloped-rectangle" | "sl-rect" => {
            let base_h = (h / 1.5).max(1.0);
            let x = -w / 2.0;
            let y = -base_h / 2.0;
            Some(vec![
                crate::model::LayoutPoint { x, y },
                crate::model::LayoutPoint { x, y: y + base_h },
                crate::model::LayoutPoint {
                    x: x + w,
                    y: y + base_h,
                },
                crate::model::LayoutPoint {
                    x: x + w,
                    y: y - base_h / 2.0,
                },
            ])
        }
        "hourglass" | "collate" => Some(vec![
            crate::model::LayoutPoint { x: 0.0, y: 0.0 },
            crate::model::LayoutPoint { x: w, y: 0.0 },
            crate::model::LayoutPoint { x: 0.0, y: h },
            crate::model::LayoutPoint { x: w, y: h },
        ]),
        _ => None,
    }
}

fn compute_node_label_metrics_for_intersection(
    ctx: &FlowchartRenderCtx<'_>,
    node_id: &str,
) -> Option<crate::text::TextMetrics> {
    let flow_node = ctx.nodes_by_id.get(node_id)?;
    let label_text = flow_node.label.clone().unwrap_or_default();
    let label_type = flow_node
        .label_type
        .clone()
        .unwrap_or_else(|| "text".to_string());

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
    let mut metrics = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: ctx.measurer,
            raw_label: &label_text,
            label_type: &label_type,
            style: &node_text_style,
            max_width_px: Some(ctx.wrapping_width),
            wrap_mode: ctx.node_wrap_mode,
            config: ctx.config,
            math_renderer: ctx.math_renderer,
            preserve_string_whitespace_height: ctx.node_html_labels && ctx.edge_html_labels,
            whole_label_font_style: node_font_style.as_deref(),
        },
    );

    let span_css_height_parity = crate::flowchart::flowchart_node_has_span_css_height_parity(
        ctx.class_defs,
        &flow_node.classes,
    );
    if ctx.node_html_labels && ctx.edge_html_labels && span_css_height_parity {
        crate::text::flowchart_apply_mermaid_styled_node_height_parity(
            &mut metrics,
            &node_text_style,
        );
    }

    Some(metrics)
}

pub(in crate::svg::parity::flowchart) fn intersect_for_layout_shape(
    ctx: &FlowchartRenderCtx<'_>,
    node_id: &str,
    node: &BoundaryNode,
    layout_shape: Option<&str>,
    point: &crate::model::LayoutPoint,
) -> crate::model::LayoutPoint {
    fn generate_full_sine_wave_points(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        amplitude: f64,
        num_cycles: f64,
    ) -> Vec<crate::model::LayoutPoint> {
        let steps: usize = 50;
        let delta_x = x2 - x1;
        let delta_y = y2 - y1;
        let cycle_length = delta_x / num_cycles;
        let frequency = (2.0 * std::f64::consts::PI) / cycle_length;
        let mid_y = y1 + delta_y / 2.0;

        let mut points: Vec<crate::model::LayoutPoint> = Vec::with_capacity(steps + 1);
        for i in 0..=steps {
            let t = (i as f64) / (steps as f64);
            let x = x1 + t * delta_x;
            let y = mid_y + amplitude * (frequency * (x - x1)).sin();
            points.push(crate::model::LayoutPoint { x, y });
        }
        points
    }

    fn generate_circle_points(
        center_x: f64,
        center_y: f64,
        radius: f64,
        num_points: usize,
        start_deg: f64,
        end_deg: f64,
    ) -> Vec<crate::model::LayoutPoint> {
        let start = start_deg.to_radians();
        let end = end_deg.to_radians();
        let angle_range = end - start;
        let angle_step = if num_points > 1 {
            angle_range / (num_points as f64 - 1.0)
        } else {
            0.0
        };
        let mut out: Vec<crate::model::LayoutPoint> = Vec::with_capacity(num_points);
        for i in 0..num_points {
            let a = start + (i as f64) * angle_step;
            let x = center_x + radius * a.cos();
            let y = center_y + radius * a.sin();
            out.push(crate::model::LayoutPoint { x: -x, y: -y });
        }
        out
    }

    fn arc_points(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        rx: f64,
        ry: f64,
        clockwise: bool,
    ) -> Vec<crate::model::LayoutPoint> {
        let num_points: usize = 20;

        let mid_x = (x1 + x2) / 2.0;
        let mid_y = (y1 + y2) / 2.0;
        let angle = (y2 - y1).atan2(x2 - x1);

        let dx = (x2 - x1) / 2.0;
        let dy = (y2 - y1) / 2.0;
        let transformed_x = dx / rx;
        let transformed_y = dy / ry;
        let distance = (transformed_x * transformed_x + transformed_y * transformed_y).sqrt();
        if distance > 1.0 {
            return vec![
                crate::model::LayoutPoint { x: x1, y: y1 },
                crate::model::LayoutPoint { x: x2, y: y2 },
            ];
        }

        let scaled_center_distance = (1.0 - distance * distance).sqrt();
        let sign = if clockwise { -1.0 } else { 1.0 };
        let center_x = mid_x + scaled_center_distance * ry * angle.sin() * sign;
        let center_y = mid_y - scaled_center_distance * rx * angle.cos() * sign;

        let start_angle = ((y1 - center_y) / ry).atan2((x1 - center_x) / rx);
        let end_angle = ((y2 - center_y) / ry).atan2((x2 - center_x) / rx);

        let mut angle_range = end_angle - start_angle;
        if clockwise && angle_range < 0.0 {
            angle_range += 2.0 * std::f64::consts::PI;
        }
        if !clockwise && angle_range > 0.0 {
            angle_range -= 2.0 * std::f64::consts::PI;
        }

        let mut points: Vec<crate::model::LayoutPoint> = Vec::with_capacity(num_points);
        for i in 0..num_points {
            let t = i as f64 / (num_points - 1) as f64;
            let a = start_angle + t * angle_range;
            let x = center_x + rx * a.cos();
            let y = center_y + ry * a.sin();
            points.push(crate::model::LayoutPoint { x, y });
        }
        points
    }

    fn intersect_stadium(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        // Port of Mermaid@11.12.2 `stadium.ts` intersection behavior:
        // - `points` are generated from the theoretical render dimensions,
        // - `node.width/height` used by `intersect.polygon(...)` come from `updateNodeBounds(...)`.
        fn generate_circle_points(
            center_x: f64,
            center_y: f64,
            radius: f64,
            table: &[(f64, f64)],
        ) -> Vec<crate::model::LayoutPoint> {
            let mut pts = Vec::with_capacity(table.len());
            for &(cos, sin) in table {
                let x = center_x + radius * cos;
                let y = center_y + radius * sin;
                pts.push(crate::model::LayoutPoint { x: -x, y: -y });
            }
            pts
        }

        let Some(flow_node) = ctx.nodes_by_id.get(node_id) else {
            return intersect_rect(node, point);
        };

        let label_text = flow_node.label.clone().unwrap_or_default();
        let label_type = flow_node
            .label_type
            .clone()
            .unwrap_or_else(|| "text".to_string());

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
        let mut metrics = crate::flowchart::flowchart_label_metrics_for_layout(
            crate::flowchart::FlowchartLabelMetricsRequest {
                measurer: ctx.measurer,
                raw_label: &label_text,
                label_type: &label_type,
                style: &node_text_style,
                max_width_px: Some(ctx.wrapping_width),
                wrap_mode: ctx.node_wrap_mode,
                config: ctx.config,
                math_renderer: ctx.math_renderer,
                preserve_string_whitespace_height: ctx.node_html_labels && ctx.edge_html_labels,
                whole_label_font_style: node_font_style.as_deref(),
            },
        );

        let span_css_height_parity = crate::flowchart::flowchart_node_has_span_css_height_parity(
            ctx.class_defs,
            &flow_node.classes,
        );
        if ctx.node_html_labels && ctx.edge_html_labels && span_css_height_parity {
            crate::text::flowchart_apply_mermaid_styled_node_height_parity(
                &mut metrics,
                &node_text_style,
            );
        }

        let (render_w, render_h) = crate::flowchart::flowchart_node_render_dimensions(
            Some("stadium"),
            metrics,
            ctx.node_padding,
        );
        let mut w = render_w.max(1.0);
        let mut h = render_h.max(1.0);

        // The input bbox values that Mermaid uses to derive these dimensions come from DOM
        // APIs and behave like f32-rounded values in Chromium. Keep the sampled polygon points
        // on the same lattice so the downstream intersection rounding matches strict baselines.
        let w_f32 = w as f32;
        let h_f32 = h as f32;
        if w_f32.is_finite()
            && h_f32.is_finite()
            && w_f32.is_sign_positive()
            && h_f32.is_sign_positive()
        {
            w = w_f32 as f64;
            h = h_f32 as f64;
        }

        let radius = h / 2.0;

        let mut pts: Vec<crate::model::LayoutPoint> = Vec::with_capacity(2 + 50 + 1 + 50);
        pts.push(crate::model::LayoutPoint {
            x: -w / 2.0 + radius,
            y: -h / 2.0,
        });
        pts.push(crate::model::LayoutPoint {
            x: w / 2.0 - radius,
            y: -h / 2.0,
        });
        pts.extend(generate_circle_points(
            -w / 2.0 + radius,
            0.0,
            radius,
            &crate::trig_tables::STADIUM_ARC_90_270_COS_SIN,
        ));
        pts.push(crate::model::LayoutPoint {
            x: w / 2.0 - radius,
            y: h / 2.0,
        });
        pts.extend(generate_circle_points(
            w / 2.0 - radius,
            0.0,
            radius,
            &crate::trig_tables::STADIUM_ARC_270_450_COS_SIN,
        ));
        intersect_polygon(node, &pts, point)
    }

    fn intersect_hexagon(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        // Port of Mermaid@11.12.2 `hexagon.ts` intersection behavior:
        // - `points` are generated from the theoretical render dimensions,
        // - `node.width/height` used by `intersect.polygon(...)` come from `updateNodeBounds(...)`.
        let Some(flow_node) = ctx.nodes_by_id.get(node_id) else {
            return intersect_rect(node, point);
        };

        let label_text = flow_node.label.clone().unwrap_or_default();
        let label_type = flow_node
            .label_type
            .clone()
            .unwrap_or_else(|| "text".to_string());

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
        let mut metrics = crate::flowchart::flowchart_label_metrics_for_layout(
            crate::flowchart::FlowchartLabelMetricsRequest {
                measurer: ctx.measurer,
                raw_label: &label_text,
                label_type: &label_type,
                style: &node_text_style,
                max_width_px: Some(ctx.wrapping_width),
                wrap_mode: ctx.node_wrap_mode,
                config: ctx.config,
                math_renderer: ctx.math_renderer,
                preserve_string_whitespace_height: ctx.node_html_labels && ctx.edge_html_labels,
                whole_label_font_style: node_font_style.as_deref(),
            },
        );

        let span_css_height_parity = crate::flowchart::flowchart_node_has_span_css_height_parity(
            ctx.class_defs,
            &flow_node.classes,
        );
        if ctx.node_html_labels && ctx.edge_html_labels && span_css_height_parity {
            crate::text::flowchart_apply_mermaid_styled_node_height_parity(
                &mut metrics,
                &node_text_style,
            );
        }

        let (render_w, render_h) = crate::flowchart::flowchart_node_render_dimensions(
            Some("hexagon"),
            metrics,
            ctx.node_padding,
        );
        let w = render_w.max(1.0);
        let h = render_h.max(1.0);
        let half_width = w / 2.0;
        let half_height = h / 2.0;
        let fixed_length = half_height / 2.0;
        let deduced_width = half_width - fixed_length;

        let pts: Vec<crate::model::LayoutPoint> = vec![
            crate::model::LayoutPoint {
                x: -deduced_width,
                y: -half_height,
            },
            crate::model::LayoutPoint {
                x: 0.0,
                y: -half_height,
            },
            crate::model::LayoutPoint {
                x: deduced_width,
                y: -half_height,
            },
            crate::model::LayoutPoint {
                x: half_width,
                y: 0.0,
            },
            crate::model::LayoutPoint {
                x: deduced_width,
                y: half_height,
            },
            crate::model::LayoutPoint {
                x: 0.0,
                y: half_height,
            },
            crate::model::LayoutPoint {
                x: -deduced_width,
                y: half_height,
            },
            crate::model::LayoutPoint {
                x: -half_width,
                y: 0.0,
            },
        ];

        intersect_polygon(node, &pts, point)
    }

    fn intersect_curved_trapezoid(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        let Some(metrics) = compute_node_label_metrics_for_intersection(ctx, node_id) else {
            return intersect_rect(node, point);
        };

        let p = ctx.node_padding;
        let min_width = 20.0;
        let min_height = 5.0;
        let w = ((metrics.width + 2.0 * p) * 1.25).max(min_width);
        let h = (metrics.height + 2.0 * p).max(min_height);
        let radius = h / 2.0;
        let rw = w - radius;
        let tw = h / 4.0;

        let mut points: Vec<crate::model::LayoutPoint> = vec![
            crate::model::LayoutPoint { x: rw, y: 0.0 },
            crate::model::LayoutPoint { x: tw, y: 0.0 },
            crate::model::LayoutPoint { x: 0.0, y: h / 2.0 },
            crate::model::LayoutPoint { x: tw, y: h },
            crate::model::LayoutPoint { x: rw, y: h },
        ];
        points.extend(generate_circle_points(
            -rw,
            -h / 2.0,
            radius,
            50,
            270.0,
            90.0,
        ));

        intersect_polygon(node, &points, point)
    }

    fn intersect_bow_tie_rect(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        let Some(metrics) = compute_node_label_metrics_for_intersection(ctx, node_id) else {
            return intersect_rect(node, point);
        };

        let p = ctx.node_padding;
        let w = metrics.width + 2.0 * p;
        let h = metrics.height + p;
        let ry = h / 2.0;
        let rx = ry / (2.5 + h / 50.0);

        let mut points: Vec<crate::model::LayoutPoint> = Vec::new();
        points.push(crate::model::LayoutPoint {
            x: w / 2.0,
            y: -h / 2.0,
        });
        points.push(crate::model::LayoutPoint {
            x: -w / 2.0,
            y: -h / 2.0,
        });
        points.extend(arc_points(
            -w / 2.0,
            -h / 2.0,
            -w / 2.0,
            h / 2.0,
            rx,
            ry,
            false,
        ));
        points.push(crate::model::LayoutPoint {
            x: w / 2.0,
            y: h / 2.0,
        });
        points.extend(arc_points(
            w / 2.0,
            h / 2.0,
            w / 2.0,
            -h / 2.0,
            rx,
            ry,
            true,
        ));

        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for p in &points {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }

        let render_node = BoundaryNode {
            x: node.x,
            y: node.y,
            width: (max_x - min_x).max(0.0),
            height: (max_y - min_y).max(0.0),
        };

        intersect_polygon(&render_node, &points, point)
    }

    fn intersect_tagged_document(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        let Some(metrics) = compute_node_label_metrics_for_intersection(ctx, node_id) else {
            return intersect_rect(node, point);
        };

        let p = ctx.node_padding;
        let w = metrics.width + 2.0 * p;
        let h = metrics.height + 2.0 * p;
        let wave_amplitude = h / 8.0;
        let final_h = h + wave_amplitude;
        let ext = (w / 2.0) * 0.1;

        let mut points: Vec<crate::model::LayoutPoint> = Vec::new();
        points.push(crate::model::LayoutPoint {
            x: -w / 2.0 - ext,
            y: final_h / 2.0,
        });
        points.extend(generate_full_sine_wave_points(
            -w / 2.0 - ext,
            final_h / 2.0,
            w / 2.0 + ext,
            final_h / 2.0,
            wave_amplitude,
            0.8,
        ));
        points.push(crate::model::LayoutPoint {
            x: w / 2.0 + ext,
            y: -final_h / 2.0,
        });
        points.push(crate::model::LayoutPoint {
            x: -w / 2.0 - ext,
            y: -final_h / 2.0,
        });

        intersect_polygon(node, &points, point)
    }

    fn intersect_tagged_rect(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        let Some(metrics) = compute_node_label_metrics_for_intersection(ctx, node_id) else {
            return intersect_rect(node, point);
        };

        let p = ctx.node_padding;
        let w = metrics.width + 2.0 * p;
        let h = metrics.height + 2.0 * p;
        let x = -w / 2.0;
        let y = -h / 2.0;
        let tag_width = 0.2 * h;

        let points = vec![
            crate::model::LayoutPoint {
                x: x - tag_width / 2.0,
                y,
            },
            crate::model::LayoutPoint {
                x: x + w + tag_width / 2.0,
                y,
            },
            crate::model::LayoutPoint {
                x: x + w + tag_width / 2.0,
                y: y + h,
            },
            crate::model::LayoutPoint {
                x: x - tag_width / 2.0,
                y: y + h,
            },
        ];

        intersect_polygon(node, &points, point)
    }

    fn intersect_wave_document(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        let Some(metrics) = compute_node_label_metrics_for_intersection(ctx, node_id) else {
            return intersect_rect(node, point);
        };

        let p = ctx.node_padding;
        let w = (metrics.width + 2.0 * p).max(0.0);
        let h = (metrics.height + 2.0 * p).max(0.0);
        let wave_amplitude = h / 8.0;
        let final_h = h + wave_amplitude;
        let extra_w = ((14.0 - w).max(0.0)) / 2.0;

        let mut points: Vec<crate::model::LayoutPoint> = Vec::new();
        points.push(crate::model::LayoutPoint {
            x: -w / 2.0 - extra_w,
            y: final_h / 2.0,
        });
        points.extend(generate_full_sine_wave_points(
            -w / 2.0 - extra_w,
            final_h / 2.0,
            w / 2.0 + extra_w,
            final_h / 2.0,
            wave_amplitude,
            0.8,
        ));
        points.push(crate::model::LayoutPoint {
            x: w / 2.0 + extra_w,
            y: -final_h / 2.0,
        });
        points.push(crate::model::LayoutPoint {
            x: -w / 2.0 - extra_w,
            y: -final_h / 2.0,
        });

        intersect_polygon(node, &points, point)
    }

    fn intersect_delay(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        let Some(metrics) = compute_node_label_metrics_for_intersection(ctx, node_id) else {
            return intersect_rect(node, point);
        };

        let p = ctx.node_padding;
        let w = (metrics.width + 2.0 * p).max(15.0);
        let h = (metrics.height + 2.0 * p).max(10.0);
        let radius = h / 2.0;

        let mut points: Vec<crate::model::LayoutPoint> = Vec::new();
        points.push(crate::model::LayoutPoint {
            x: -w / 2.0,
            y: -h / 2.0,
        });
        points.push(crate::model::LayoutPoint {
            x: w / 2.0 - radius,
            y: -h / 2.0,
        });
        points.extend(generate_circle_points(
            -w / 2.0 + radius,
            0.0,
            radius,
            50,
            90.0,
            270.0,
        ));
        points.push(crate::model::LayoutPoint {
            x: w / 2.0 - radius,
            y: h / 2.0,
        });
        points.push(crate::model::LayoutPoint {
            x: -w / 2.0,
            y: h / 2.0,
        });

        intersect_polygon(node, &points, point)
    }

    fn intersect_notched_pentagon(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        let Some(metrics) = compute_node_label_metrics_for_intersection(ctx, node_id) else {
            return intersect_rect(node, point);
        };

        let p = ctx.node_padding;
        let w = (metrics.width + 2.0 * p).max(60.0);
        let h = (metrics.height + 2.0 * p).max(20.0);
        let points = vec![
            crate::model::LayoutPoint {
                x: (-w / 2.0) * 0.8,
                y: -h / 2.0,
            },
            crate::model::LayoutPoint {
                x: (w / 2.0) * 0.8,
                y: -h / 2.0,
            },
            crate::model::LayoutPoint {
                x: w / 2.0,
                y: (-h / 2.0) * 0.6,
            },
            crate::model::LayoutPoint {
                x: w / 2.0,
                y: h / 2.0,
            },
            crate::model::LayoutPoint {
                x: -w / 2.0,
                y: h / 2.0,
            },
            crate::model::LayoutPoint {
                x: -w / 2.0,
                y: (-h / 2.0) * 0.6,
            },
        ];

        intersect_polygon(node, &points, point)
    }

    fn intersect_multi_wave_edged_rect(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        let Some(metrics) = compute_node_label_metrics_for_intersection(ctx, node_id) else {
            return intersect_rect(node, point);
        };

        let p = ctx.node_padding;
        let w = metrics.width + 2.0 * p;
        let h = metrics.height + 2.0 * p;
        let wave_amplitude = h / 4.0;
        let final_h = h + wave_amplitude;
        let x = -w / 2.0;
        let y = -final_h / 2.0;
        let rect_offset = 5.0;

        let wave_points = generate_full_sine_wave_points(
            x - rect_offset,
            y + final_h + rect_offset,
            x + w - rect_offset,
            y + final_h + rect_offset,
            wave_amplitude,
            0.8,
        );
        let last_y = wave_points
            .last()
            .map(|p| p.y)
            .unwrap_or(y + final_h + rect_offset);

        let mut points: Vec<crate::model::LayoutPoint> = Vec::new();
        points.push(crate::model::LayoutPoint {
            x: x - rect_offset,
            y: y + rect_offset,
        });
        points.push(crate::model::LayoutPoint {
            x: x - rect_offset,
            y: y + final_h + rect_offset,
        });
        points.extend(wave_points);
        points.push(crate::model::LayoutPoint {
            x: x + w - rect_offset,
            y: last_y - rect_offset,
        });
        points.push(crate::model::LayoutPoint {
            x: x + w,
            y: last_y - rect_offset,
        });
        points.push(crate::model::LayoutPoint {
            x: x + w,
            y: last_y - 2.0 * rect_offset,
        });
        points.push(crate::model::LayoutPoint {
            x: x + w + rect_offset,
            y: last_y - 2.0 * rect_offset,
        });
        points.push(crate::model::LayoutPoint {
            x: x + w + rect_offset,
            y: y - rect_offset,
        });
        points.push(crate::model::LayoutPoint {
            x: x + rect_offset,
            y: y - rect_offset,
        });
        points.push(crate::model::LayoutPoint {
            x: x + rect_offset,
            y,
        });
        points.push(crate::model::LayoutPoint { x, y });
        points.push(crate::model::LayoutPoint {
            x,
            y: y + rect_offset,
        });

        intersect_polygon(node, &points, point)
    }

    fn intersect_lightning_bolt(
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        let width = 35.0;
        let height = 35.0;
        let gap = 7.0;
        let points = vec![
            crate::model::LayoutPoint { x: width, y: 0.0 },
            crate::model::LayoutPoint {
                x: 0.0,
                y: height + gap / 2.0,
            },
            crate::model::LayoutPoint {
                x: width - 2.0 * gap,
                y: height + gap / 2.0,
            },
            crate::model::LayoutPoint {
                x: 0.0,
                y: 2.0 * height,
            },
            crate::model::LayoutPoint {
                x: width,
                y: height - gap / 2.0,
            },
            crate::model::LayoutPoint {
                x: 2.0 * gap,
                y: height - gap / 2.0,
            },
        ];

        intersect_polygon(node, &points, point)
    }

    fn intersect_window_pane(
        ctx: &FlowchartRenderCtx<'_>,
        node_id: &str,
        node: &BoundaryNode,
        point: &crate::model::LayoutPoint,
    ) -> crate::model::LayoutPoint {
        let Some(metrics) = compute_node_label_metrics_for_intersection(ctx, node_id) else {
            return intersect_rect(node, point);
        };

        let p = ctx.node_padding;
        let w = metrics.width + 2.0 * p;
        let h = metrics.height + 2.0 * p;
        let rect_offset = 10.0;
        let x = -w / 2.0;
        let y = -h / 2.0;
        let points = vec![
            crate::model::LayoutPoint {
                x: x - rect_offset,
                y: y - rect_offset,
            },
            crate::model::LayoutPoint {
                x: x - rect_offset,
                y: y + h,
            },
            crate::model::LayoutPoint { x: x + w, y: y + h },
            crate::model::LayoutPoint {
                x: x + w,
                y: y - rect_offset,
            },
        ];

        intersect_polygon(node, &points, point)
    }

    match layout_shape {
        Some("circle") => intersect_circle(node, point),
        Some("f-circ" | "junction" | "filled-circle") => intersect_circle(node, point),
        Some("cross-circ" | "summary" | "crossed-circle") => intersect_circle(node, point),
        Some("cylinder" | "cyl" | "db" | "database") => intersect_cylinder(node, point),
        Some("lin-cyl" | "disk" | "lined-cylinder") => intersect_cylinder(node, point),
        Some("h-cyl" | "das" | "horizontal-cylinder") => intersect_tilted_cylinder(node, point),
        Some("hourglass" | "collate") => intersect_polygon_hourglass(node, point),
        Some("diamond" | "diam" | "question" | "decision") => intersect_diamond(node, point),
        Some("stadium" | "terminal" | "pill") => intersect_stadium(ctx, node_id, node, point),
        Some("hexagon" | "hex" | "prepare") => intersect_hexagon(ctx, node_id, node, point),
        Some("curv-trap" | "display" | "curved-trapezoid") => {
            intersect_curved_trapezoid(ctx, node_id, node, point)
        }
        Some("bow-rect" | "stored-data" | "bow-tie-rectangle") => {
            intersect_bow_tie_rect(ctx, node_id, node, point)
        }
        Some("tag-rect" | "tagged-rectangle" | "tag-proc" | "tagged-process") => {
            intersect_tagged_rect(ctx, node_id, node, point)
        }
        Some("doc" | "document") => intersect_wave_document(ctx, node_id, node, point),
        Some("delay" | "half-rounded-rectangle") => intersect_delay(ctx, node_id, node, point),
        Some("notch-pent" | "loop-limit" | "notched-pentagon") => {
            intersect_notched_pentagon(ctx, node_id, node, point)
        }
        Some("docs" | "documents" | "st-doc" | "stacked-document") => {
            intersect_multi_wave_edged_rect(ctx, node_id, node, point)
        }
        Some("tag-doc" | "tagged-document") => intersect_tagged_document(ctx, node_id, node, point),
        Some("bolt" | "com-link" | "lightning-bolt") => intersect_lightning_bolt(node, point),
        Some("win-pane" | "internal-storage" | "window-pane") => {
            intersect_window_pane(ctx, node_id, node, point)
        }
        Some(s) if is_polygon_layout_shape(Some(s)) => polygon_points_for_layout_shape(s, node)
            .map(|pts| intersect_polygon(node, &pts, point))
            .unwrap_or_else(|| intersect_rect(node, point)),
        _ => intersect_rect(node, point),
    }
}
