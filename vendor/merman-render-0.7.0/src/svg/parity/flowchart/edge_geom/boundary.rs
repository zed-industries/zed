use super::super::*;

#[derive(Debug, Clone, Copy)]
pub(in crate::svg::parity::flowchart) struct BoundaryNode {
    pub(in crate::svg::parity::flowchart) x: f64,
    pub(in crate::svg::parity::flowchart) y: f64,
    pub(in crate::svg::parity::flowchart) width: f64,
    pub(in crate::svg::parity::flowchart) height: f64,
}

pub(in crate::svg::parity::flowchart) fn boundary_for_node(
    ctx: &FlowchartRenderCtx<'_>,
    node_id: &str,
    origin_x: f64,
    origin_y: f64,
    normalize_cyclic_special: bool,
) -> Option<BoundaryNode> {
    let _ = normalize_cyclic_special;
    let n = ctx.layout_nodes_by_id.get(node_id)?;
    Some(BoundaryNode {
        x: n.x + ctx.tx - origin_x,
        y: n.y + ctx.ty - origin_y,
        width: n.width,
        height: n.height,
    })
}

pub(in crate::svg::parity::flowchart) fn boundary_for_cluster(
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

pub(in crate::svg::parity::flowchart) fn maybe_normalize_selfedge_loop_points(
    points: &mut [crate::model::LayoutPoint],
) {
    if points.len() != 7 {
        return;
    }
    let eps = 1e-6;
    let i = points[0].x;
    if (points[6].x - i).abs() > eps {
        return;
    }
    let top_y = points[1].y;
    let bottom_y = points[4].y;
    let a = points[3].y;
    let l = bottom_y - a;
    if !l.is_finite() || l.abs() < eps {
        return;
    }
    if (top_y - (a - l)).abs() > eps {
        return;
    }
    if (points[2].y - top_y).abs() > eps
        || (points[5].y - bottom_y).abs() > eps
        || (points[1].y - top_y).abs() > eps
        || (points[4].y - bottom_y).abs() > eps
    {
        return;
    }
    let mid_y = (top_y + bottom_y) / 2.0;
    if (mid_y - a).abs() > eps {
        return;
    }
    let dummy_x = points[3].x;
    let o = dummy_x - i;
    if !o.is_finite() {
        return;
    }
    let x1 = i + 2.0 * o / 3.0;
    let x2 = i + 5.0 * o / 6.0;
    if !(x1.is_finite() && x2.is_finite()) {
        return;
    }
    points[1].x = x1;
    points[2].x = x2;
    points[4].x = x2;
    points[5].x = x1;
    points[1].y = top_y;
    points[2].y = top_y;
    points[3].y = a;
    points[4].y = bottom_y;
    points[5].y = bottom_y;
}
