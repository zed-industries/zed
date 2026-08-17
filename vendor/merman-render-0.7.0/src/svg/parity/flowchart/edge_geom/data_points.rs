//! `data-points` normalization helpers.
//!
//! Mermaid encodes flowchart edge points as Base64(JSON.stringify(points)). For strict SVG XML
//! parity we keep that behavior, including a few extremely conservative floating-point
//! normalizations that are observed in upstream baselines.

pub(in crate::svg::parity::flowchart) fn maybe_truncate_data_point(v: f64) -> f64 {
    if !v.is_finite() {
        return 0.0;
    }

    let scale = 262_144.0; // 2^18
    let scaled = v * scale;
    let floor = scaled.floor();
    let frac = scaled - floor;

    // Keep this extremely conservative: legitimate Dagre self-loop points frequently land near
    // 1/3 multiples at this scale (e.g. `...45833333333334`), and upstream Mermaid does not
    // truncate those. Only truncate when we're effectively on the boundary.
    let eps = 1e-12;
    let one_third = 1.0 / 3.0;
    let two_thirds = 2.0 / 3.0;
    let should_truncate = (frac - one_third).abs() < eps || (frac - two_thirds).abs() < eps;
    if !should_truncate {
        return v;
    }

    let out = floor / scale;
    if out == -0.0 { 0.0 } else { out }
}

pub(in crate::svg::parity::flowchart) fn maybe_snap_data_point_to_f32(v: f64) -> f64 {
    if !v.is_finite() {
        return 0.0;
    }

    // Upstream Mermaid (V8) frequently ends up with coordinates that are effectively f32-rounded
    // due to DOM/layout measurement pipelines. When our headless math lands extremely close to
    // those f32 values, snap to that lattice so `data-points` Base64(JSON.stringify(...)) matches
    // bit-for-bit.
    fn next_up(v: f64) -> f64 {
        if !v.is_finite() {
            return v;
        }
        if v == 0.0 {
            return f64::from_bits(1);
        }
        let bits = v.to_bits();
        if v > 0.0 {
            f64::from_bits(bits + 1)
        } else {
            f64::from_bits(bits - 1)
        }
    }

    fn next_down(v: f64) -> f64 {
        if !v.is_finite() {
            return v;
        }
        if v == 0.0 {
            return -f64::from_bits(1);
        }
        let bits = v.to_bits();
        if v > 0.0 {
            f64::from_bits(bits - 1)
        } else {
            f64::from_bits(bits + 1)
        }
    }

    let snapped = (v as f32) as f64;
    if !snapped.is_finite() {
        return v;
    }

    // Common case: we're nowhere near the f32 lattice. Avoid the heavier bit-level checks.
    let diff = (v - snapped).abs();
    if diff > 1e-12 {
        return if v == -0.0 { 0.0 } else { v };
    }

    // Preserve exact 1-ULP offsets around the snapped value. Upstream Mermaid frequently
    // produces values like `761.5937500000001` (next_up of `761.59375`) and
    // `145.49999999999997` (next_down of `145.5`) due to floating-point rounding, and snapping
    // those back to the f32 lattice would reduce strict parity.
    let v_bits = v.to_bits();
    let snapped_bits = snapped.to_bits();
    if v_bits == snapped_bits
        || v_bits == next_up(snapped).to_bits()
        || v_bits == next_down(snapped).to_bits()
    {
        return if v == -0.0 { 0.0 } else { v };
    }

    // Keep the snapping extremely tight: upstream `data-points` frequently include tiny non-f32
    // artifacts (several f64 ulps away from the f32-rounded value), and snapping too aggressively
    // erases those strict-parity baselines.
    if diff < 1e-14 {
        if snapped == -0.0 { 0.0 } else { snapped }
    } else {
        v
    }
}
