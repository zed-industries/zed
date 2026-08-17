// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use alloc::vec::Vec;

use tiny_skia_path::Scalar;

use crate::{Color, GradientStop, Point, Shader, SpreadMode, Transform};

use super::gradient::{Gradient, DEGENERATE_THRESHOLD};
use crate::pipeline::RasterPipelineBuilder;

/// A linear gradient shader.
#[derive(Clone, PartialEq, Debug)]
pub struct LinearGradient {
    pub(crate) base: Gradient,
}

impl LinearGradient {
    /// Creates a new linear gradient shader.
    ///
    /// Returns `Shader::SolidColor` when:
    /// - `stops.len()` == 1
    /// - `start` and `end` are very close
    ///
    /// Returns `None` when:
    ///
    /// - `stops` is empty
    /// - `start` == `end`
    /// - `transform` is not invertible
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        start: Point,
        end: Point,
        stops: Vec<GradientStop>,
        mode: SpreadMode,
        transform: Transform,
    ) -> Option<Shader<'static>> {
        if stops.is_empty() {
            return None;
        }

        if stops.len() == 1 {
            return Some(Shader::SolidColor(stops[0].color));
        }

        let length = (end - start).length();
        if !length.is_finite() {
            return None;
        }

        if length.is_nearly_zero_within_tolerance(DEGENERATE_THRESHOLD) {
            // Degenerate gradient, the only tricky complication is when in clamp mode,
            // the limit of the gradient approaches two half planes of solid color
            // (first and last). However, they are divided by the line perpendicular
            // to the start and end point, which becomes undefined once start and end
            // are exactly the same, so just use the end color for a stable solution.

            // Except for special circumstances of clamped gradients,
            // every gradient shape (when degenerate) can be mapped to the same fallbacks.
            // The specific shape factories must account for special clamped conditions separately,
            // this will always return the last color for clamped gradients.
            match mode {
                SpreadMode::Pad => {
                    // Depending on how the gradient shape degenerates,
                    // there may be a more specialized fallback representation
                    // for the factories to use, but this is a reasonable default.
                    return Some(Shader::SolidColor(stops.last().unwrap().color));
                }
                SpreadMode::Reflect | SpreadMode::Repeat => {
                    // repeat and mirror are treated the same: the border colors are never visible,
                    // but approximate the final color as infinite repetitions of the colors, so
                    // it can be represented as the average color of the gradient.
                    return Some(Shader::SolidColor(average_gradient_color(&stops)));
                }
            }
        }

        transform.invert()?;

        let unit_ts = points_to_unit_ts(start, end)?;
        Some(Shader::LinearGradient(LinearGradient {
            base: Gradient::new(stops, mode, transform, unit_ts),
        }))
    }

    pub(crate) fn is_opaque(&self) -> bool {
        self.base.colors_are_opaque
    }

    pub(crate) fn push_stages(&self, p: &mut RasterPipelineBuilder) -> bool {
        self.base.push_stages(p, &|_| {}, &|_| {})
    }
}

fn points_to_unit_ts(start: Point, end: Point) -> Option<Transform> {
    let mut vec = end - start;
    let mag = vec.length();
    let inv = if mag != 0.0 { mag.invert() } else { 0.0 };

    vec.scale(inv);

    let mut ts = ts_from_sin_cos_at(-vec.y, vec.x, start.x, start.y);
    ts = ts.post_translate(-start.x, -start.y);
    ts = ts.post_scale(inv, inv);
    Some(ts)
}

fn average_gradient_color(points: &[GradientStop]) -> Color {
    use crate::wide::f32x4;

    fn load_color(c: Color) -> f32x4 {
        f32x4::from([c.red(), c.green(), c.blue(), c.alpha()])
    }

    fn store_color(c: f32x4) -> Color {
        let c: [f32; 4] = c.into();
        Color::from_rgba(c[0], c[1], c[2], c[3]).unwrap()
    }

    assert!(!points.is_empty());

    // The gradient is a piecewise linear interpolation between colors. For a given interval,
    // the integral between the two endpoints is 0.5 * (ci + cj) * (pj - pi), which provides that
    // intervals average color. The overall average color is thus the sum of each piece. The thing
    // to keep in mind is that the provided gradient definition may implicitly use p=0 and p=1.
    let mut blend = f32x4::splat(0.0);

    // Bake 1/(colorCount - 1) uniform stop difference into this scale factor
    let w_scale = f32x4::splat(0.5);

    for i in 0..points.len() - 1 {
        // Calculate the average color for the interval between pos(i) and pos(i+1)
        let c0 = load_color(points[i].color);
        let c1 = load_color(points[i + 1].color);
        // when pos == null, there are colorCount uniformly distributed stops, going from 0 to 1,
        // so pos[i + 1] - pos[i] = 1/(colorCount-1)
        let w = points[i + 1].position.get() - points[i].position.get();
        blend += w_scale * f32x4::splat(w) * (c1 + c0);
    }

    // Now account for any implicit intervals at the start or end of the stop definitions
    if points[0].position.get() > 0.0 {
        // The first color is fixed between p = 0 to pos[0], so 0.5 * (ci + cj) * (pj - pi)
        // becomes 0.5 * (c + c) * (pj - 0) = c * pj
        let c = load_color(points[0].color);
        blend += f32x4::splat(points[0].position.get()) * c;
    }

    let last_idx = points.len() - 1;
    if points[last_idx].position.get() < 1.0 {
        // The last color is fixed between pos[n-1] to p = 1, so 0.5 * (ci + cj) * (pj - pi)
        // becomes 0.5 * (c + c) * (1 - pi) = c * (1 - pi)
        let c = load_color(points[last_idx].color);
        blend += (f32x4::splat(1.0) - f32x4::splat(points[last_idx].position.get())) * c;
    }

    store_color(blend)
}

fn ts_from_sin_cos_at(sin: f32, cos: f32, px: f32, py: f32) -> Transform {
    let cos_inv = 1.0 - cos;
    Transform::from_row(
        cos,
        sin,
        -sin,
        cos,
        sdot(sin, py, cos_inv, px),
        sdot(-sin, px, cos_inv, py),
    )
}

fn sdot(a: f32, b: f32, c: f32, d: f32) -> f32 {
    a * b + c * d
}
