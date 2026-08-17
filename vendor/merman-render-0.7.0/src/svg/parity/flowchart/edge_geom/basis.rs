//! Basis curve route simplifications.
//!
//! Mermaid uses Dagre routes with D3 `curveBasis`; some edge/cluster cases need a simplified
//! route to keep the emitted command sequence aligned with upstream.

pub(in crate::svg::parity::flowchart) fn maybe_remove_redundant_cluster_run_point(
    points: &mut Vec<crate::model::LayoutPoint>,
) {
    if points.len() != 8 {
        return;
    }

    const EPS: f64 = 1e-9;
    let len = points.len();
    let mut best_run: Option<(usize, usize)> = None;

    // Find the longest axis-aligned run (same x or same y) of consecutive points.
    for axis in 0..2 {
        let mut i = 0usize;
        while i + 1 < len {
            let base = if axis == 0 { points[i].x } else { points[i].y };
            if ((if axis == 0 {
                points[i + 1].x
            } else {
                points[i + 1].y
            }) - base)
                .abs()
                > EPS
            {
                i += 1;
                continue;
            }

            let start = i;
            while i + 1 < len {
                let v = if axis == 0 {
                    points[i + 1].x
                } else {
                    points[i + 1].y
                };
                if (v - base).abs() > EPS {
                    break;
                }
                i += 1;
            }
            let end = i;
            if end + 1 - start >= 6 {
                best_run = match best_run {
                    Some((bs, be)) if (be + 1 - bs) >= (end + 1 - start) => Some((bs, be)),
                    _ => Some((start, end)),
                };
            }
            i += 1;
        }
    }

    if let Some((start, end)) = best_run {
        let idx = end.saturating_sub(1);
        if idx > start && idx > 0 && idx + 1 < len {
            points.remove(idx);
        }
    }
}
