//! Node geometry helpers (point generation and path assembly).
//!
//! These are ports of Mermaid rendering-element helpers and are used by multiple node shapes.

pub(super) use crate::svg::parity::roughjs_common::{
    closed_path_d_from_points as path_from_points, mermaid_arc_points as arc_points,
};

pub(in crate::svg::parity::flowchart) fn generate_circle_points(
    center_x: f64,
    center_y: f64,
    radius: f64,
    num_points: usize,
    start_angle_deg: f64,
    end_angle_deg: f64,
) -> Vec<(f64, f64)> {
    // Ported from Mermaid `generateCirclePoints(...)` in
    // `packages/mermaid/src/rendering-util/rendering-elements/shapes/util.ts`.
    //
    // Note: Mermaid pushes negated coordinates (`{ x: -x, y: -y }`).
    let start = start_angle_deg.to_radians();
    let end = end_angle_deg.to_radians();
    let angle_range = end - start;
    let step = angle_range / (num_points.saturating_sub(1).max(1) as f64);
    let mut pts: Vec<(f64, f64)> = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let angle = start + (i as f64) * step;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        pts.push((-x, -y));
    }
    pts
}

pub(in crate::svg::parity::flowchart) fn generate_full_sine_wave_points(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    amplitude: f64,
    num_cycles: f64,
) -> Vec<(f64, f64)> {
    // Ported from Mermaid `generateFullSineWavePoints` (50 segments).
    let steps: usize = 50;
    let delta_x = x2 - x1;
    let delta_y = y2 - y1;
    let cycle_length = delta_x / num_cycles;
    let frequency = (2.0 * std::f64::consts::PI) / cycle_length;
    let mid_y = y1 + delta_y / 2.0;

    let mut points: Vec<(f64, f64)> = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let t = (i as f64) / (steps as f64);
        let x = x1 + t * delta_x;
        let y = mid_y + amplitude * (frequency * (x - x1)).sin();
        points.push((x, y));
    }
    points
}
