use core::f64;

const TAU: f64 = f64::consts::PI * 2.0;

// Calculate an angle between two unit vectors
//
// Since we measure angle between radii of circular arcs,
// we can use simplified math (without length normalization)
//
fn unit_vector_angle(ux: f64, uy: f64, vx: f64, vy: f64) -> f64 {
    let sign = if (ux * vy) - (uy * vx) < 0.0 {
        -1.0
    } else {
        1.0
    };
    let mut dot = ux * vx + uy * vy;

    // Add this to work with arbitrary vectors:
    // dot /= Math.sqrt(ux * ux + uy * uy) * Math.sqrt(vx * vx + vy * vy);

    // rounding errors, e.g. -1.0000000000000002 can screw up this
    if dot > 1.0 {
        dot = 1.0;
    }
    if dot < -1.0 {
        dot = -1.0;
    }

    sign * dot.acos()
}

// Convert from endpoint to center parameterization,
// see http://www.w3.org/TR/SVG11/implnote.html#ArcImplementationNotes
//
// Return [cx, cy, theta1, delta_theta]
//
fn get_arc_center(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    fa: bool,
    fs: bool,
    rx: f64,
    ry: f64,
    sin_phi: f64,
    cos_phi: f64,
) -> [f64; 4] {
    // Step 1.
    //
    // Moving an ellipse so origin will be the middlepoint between our two
    // points. After that, rotate it to line up ellipse axes with coordinate
    // axes.
    //
    let x1p = cos_phi * (x1 - x2) / 2.0 + sin_phi * (y1 - y2) / 2.0;
    let y1p = -sin_phi * (x1 - x2) / 2.0 + cos_phi * (y1 - y2) / 2.0;

    let rx_sq = rx * rx;
    let ry_sq = ry * ry;
    let x1p_sq = x1p * x1p;
    let y1p_sq = y1p * y1p;

    // Step 2.
    //
    // Compute coordinates of the centre of this ellipse (cx', cy')
    // in the new coordinate system.
    //
    let mut radicant = (rx_sq * ry_sq) - (rx_sq * y1p_sq) - (ry_sq * x1p_sq);

    if radicant < 0.0 {
        // due to rounding errors it might be e.g. -1.3877787807814457e-17
        radicant = 0.0;
    }

    radicant /= (rx_sq * y1p_sq) + (ry_sq * x1p_sq);
    radicant = radicant.sqrt() * if fa == fs { -1.0 } else { 1.0 };

    let cxp = radicant * rx / ry * y1p;
    let cyp = radicant * -ry / rx * x1p;

    // Step 3.
    //
    // Transform back to get centre coordinates (cx, cy) in the original
    // coordinate system.
    //
    let cx = cos_phi * cxp - sin_phi * cyp + (x1 + x2) / 2.0;
    let cy = sin_phi * cxp + cos_phi * cyp + (y1 + y2) / 2.0;

    // Step 4.
    //
    // Compute angles (theta1, delta_theta).
    //
    let v1x = (x1p - cxp) / rx;
    let v1y = (y1p - cyp) / ry;
    let v2x = (-x1p - cxp) / rx;
    let v2y = (-y1p - cyp) / ry;

    let theta1 = unit_vector_angle(1.0, 0.0, v1x, v1y);
    let mut delta_theta = unit_vector_angle(v1x, v1y, v2x, v2y);

    if !fs && delta_theta > 0.0 {
        delta_theta -= TAU;
    }
    if fs && delta_theta < 0.0 {
        delta_theta += TAU;
    }

    [cx, cy, theta1, delta_theta]
}

//
// Approximate one unit arc segment with bézier curves,
// see http://math.stackexchange.com/questions/873224
//
fn approximate_unit_arc(theta1: f64, delta_theta: f64) -> [f64; 8] {
    let alpha = 4.0 / 3.0 * (delta_theta / 4.0).tan();

    let x1 = theta1.cos();
    let y1 = theta1.sin();
    let x2 = (theta1 + delta_theta).cos();
    let y2 = (theta1 + delta_theta).sin();

    return [
        x1,
        y1,
        x1 - y1 * alpha,
        y1 + x1 * alpha,
        x2 + y2 * alpha,
        y2 - x2 * alpha,
        x2,
        y2,
    ];
}

pub fn a2c(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    fa: bool,
    fs: bool,
    rx: f64,
    ry: f64,
    phi: f64,
) -> Vec<[f64; 8]> {
    let sin_phi = (phi * TAU / 360.0).sin();
    let cos_phi = (phi * TAU / 360.0).cos();

    // Make sure radii are valid
    //
    let x1p = cos_phi * (x1 - x2) / 2.0 + sin_phi * (y1 - y2) / 2.0;
    let y1p = -sin_phi * (x1 - x2) / 2.0 + cos_phi * (y1 - y2) / 2.0;

    if x1p == 0.0 && y1p == 0.0 {
        // we're asked to draw line to itself
        return vec![];
    }

    if rx == 0.0 || ry == 0.0 {
        // one of the radii is zero
        return vec![];
    }

    // Compensate out-of-range radii
    //
    let mut rx = rx.abs();
    let mut ry = ry.abs();

    let lambda = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
    if lambda > 1.0 {
        rx *= lambda.sqrt();
        ry *= lambda.sqrt();
    }

    // Get center parameters (cx, cy, theta1, delta_theta)
    //
    let cc = get_arc_center(x1, y1, x2, y2, fa, fs, rx, ry, sin_phi, cos_phi);

    let mut result = vec![];
    let mut theta1 = cc[2];
    let mut delta_theta = cc[3];

    // Split an arc to multiple segments, so each segment
    // will be less than τ/4 (= 90°)
    //
    let segments = *[(delta_theta.abs() / (TAU / 4.0)).ceil(), 1.0]
        .iter()
        .max_by(|a, b| a.total_cmp(b))
        .expect("can not find max") as u32;
    delta_theta /= segments as f64;

    for _ in 0..segments {
        result.push(approximate_unit_arc(theta1, delta_theta));
        theta1 += delta_theta;
    }

    // We have a bezier approximation of a unit circle,
    // now need to transform back to the original ellipse
    //
    return result
        .iter_mut()
        .map(|curve| {
            for i in (0..curve.len()).step_by(2) {
                let mut x = curve[i + 0];
                let mut y = curve[i + 1];

                // scale
                x *= rx;
                y *= ry;

                // rotate
                let xp = cos_phi * x - sin_phi * y;
                let yp = sin_phi * x + cos_phi * y;

                // translate
                curve[i + 0] = xp + cc[0];
                curve[i + 1] = yp + cc[1];
            }

            return *curve;
        })
        .collect();
}
