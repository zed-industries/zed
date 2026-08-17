use super::*;

pub(super) fn state_viewport_bounds_from_layout(layout: &StateDiagramV2Layout) -> Option<Bounds> {
    fn include_rect(bounds: &mut Option<Bounds>, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
        let w = (max_x - min_x).abs();
        let h = (max_y - min_y).abs();
        if w < 1e-9 && h < 1e-9 {
            return;
        }

        if let Some(cur) = bounds.as_mut() {
            cur.min_x = cur.min_x.min(min_x);
            cur.min_y = cur.min_y.min(min_y);
            cur.max_x = cur.max_x.max(max_x);
            cur.max_y = cur.max_y.max(max_y);
        } else {
            *bounds = Some(Bounds {
                min_x,
                min_y,
                max_x,
                max_y,
            });
        }
    }

    let mut bounds = layout.bounds.clone();

    for c in &layout.clusters {
        let left = c.x - c.width / 2.0;
        let top = c.y - c.height / 2.0;
        include_rect(
            &mut bounds,
            left,
            top,
            left + c.width.max(0.0),
            top + c.height.max(0.0),
        );

        let tl = &c.title_label;
        let left = tl.x - tl.width / 2.0;
        let top = tl.y - tl.height / 2.0;
        include_rect(
            &mut bounds,
            left,
            top,
            left + tl.width.max(0.0),
            top + tl.height.max(0.0),
        );
    }

    if bounds.is_none()
        && layout.nodes.is_empty()
        && layout.edges.is_empty()
        && layout.clusters.is_empty()
    {
        return Some(Bounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 0.0,
            max_y: 0.0,
        });
    }

    bounds
}
