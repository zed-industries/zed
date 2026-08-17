use crate::model::Bounds;

use super::super::path_bounds::{SvgPathBounds, svg_path_bounds_from_d};

pub(super) fn include_rect(
    bounds: &mut Option<Bounds>,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) {
    // Match Chromium's `getBBox()` behavior: ignore placeholder boxes that should not affect
    // the measured diagram bounds.
    let w = (max_x - min_x).abs();
    let h = (max_y - min_y).abs();
    if (w < 1e-9 && h < 1e-9) || (w <= 0.1 + 1e-9 && h <= 0.1 + 1e-9) {
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

pub(super) fn include_xywh(bounds: &mut Option<Bounds>, x: f64, y: f64, w: f64, h: f64) {
    include_rect(bounds, x, y, x + w, y + h);
}

pub(super) fn include_path_d(bounds: &mut Option<Bounds>, d: &str, dx: f64, dy: f64) {
    if let Some(pb) = svg_path_bounds_from_d(d) {
        include_path_bounds(bounds, &pb, dx, dy);
    }
}

pub(super) fn include_path_bounds(
    bounds: &mut Option<Bounds>,
    pb: &SvgPathBounds,
    dx: f64,
    dy: f64,
) {
    include_rect(
        bounds,
        pb.min_x + dx,
        pb.min_y + dy,
        pb.max_x + dx,
        pb.max_y + dy,
    );
}
