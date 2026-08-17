// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use alloc::vec::Vec;

use tiny_skia_path::{NormalizedF32, Scalar};

use crate::{Color, SpreadMode, Transform};

use crate::pipeline::RasterPipelineBuilder;
use crate::pipeline::{self, EvenlySpaced2StopGradientCtx, GradientColor, GradientCtx};

// The default SCALAR_NEARLY_ZERO threshold of .0024 is too big and causes regressions for svg
// gradients defined in the wild.
pub const DEGENERATE_THRESHOLD: f32 = 1.0 / (1 << 15) as f32;

/// A gradient point.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GradientStop {
    pub(crate) position: NormalizedF32,
    pub(crate) color: Color,
}

impl GradientStop {
    /// Creates a new gradient point.
    ///
    /// `position` will be clamped to a 0..=1 range.
    pub fn new(position: f32, color: Color) -> Self {
        GradientStop {
            position: NormalizedF32::new_clamped(position),
            color,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Gradient {
    stops: Vec<GradientStop>,
    tile_mode: SpreadMode,
    pub(crate) transform: Transform,
    points_to_unit: Transform,
    pub(crate) colors_are_opaque: bool,
    has_uniform_stops: bool,
}

impl Gradient {
    pub fn new(
        mut stops: Vec<GradientStop>,
        tile_mode: SpreadMode,
        transform: Transform,
        points_to_unit: Transform,
    ) -> Self {
        debug_assert!(stops.len() > 1);

        // Note: we let the caller skip the first and/or last position.
        // i.e. pos[0] = 0.3, pos[1] = 0.7
        // In these cases, we insert dummy entries to ensure that the final data
        // will be bracketed by [0, 1].
        // i.e. our_pos[0] = 0, our_pos[1] = 0.3, our_pos[2] = 0.7, our_pos[3] = 1
        let dummy_first = stops[0].position.get() != 0.0;
        let dummy_last = stops[stops.len() - 1].position.get() != 1.0;

        // Now copy over the colors, adding the dummies as needed.
        if dummy_first {
            stops.insert(0, GradientStop::new(0.0, stops[0].color));
        }

        if dummy_last {
            stops.push(GradientStop::new(1.0, stops[stops.len() - 1].color));
        }

        let colors_are_opaque = stops.iter().all(|p| p.color.is_opaque());

        // Pin the last value to 1.0, and make sure positions are monotonic.
        let start_index = if dummy_first { 0 } else { 1 };
        let mut prev = 0.0;
        let mut has_uniform_stops = true;
        let uniform_step = stops[start_index].position.get() - prev;
        for i in start_index..stops.len() {
            let curr = if i + 1 == stops.len() {
                // The last one must be zero.
                1.0
            } else {
                stops[i].position.get().bound(prev, 1.0)
            };

            has_uniform_stops &= uniform_step.is_nearly_equal(curr - prev);
            stops[i].position = NormalizedF32::new_clamped(curr);
            prev = curr;
        }

        Gradient {
            stops,
            tile_mode,
            transform,
            points_to_unit,
            colors_are_opaque,
            has_uniform_stops,
        }
    }

    pub fn push_stages(
        &self,
        p: &mut RasterPipelineBuilder,
        push_stages_pre: &dyn Fn(&mut RasterPipelineBuilder),
        push_stages_post: &dyn Fn(&mut RasterPipelineBuilder),
    ) -> bool {
        p.push(pipeline::Stage::SeedShader);

        let ts = match self.transform.invert() {
            Some(v) => v,
            None => {
                log::warn!("failed to invert a gradient transform. Nothing will be rendered");
                return false;
            }
        };
        let ts = ts.post_concat(self.points_to_unit);
        p.push_transform(ts);

        push_stages_pre(p);

        match self.tile_mode {
            SpreadMode::Reflect => {
                p.push(pipeline::Stage::ReflectX1);
            }
            SpreadMode::Repeat => {
                p.push(pipeline::Stage::RepeatX1);
            }
            SpreadMode::Pad => {
                if self.has_uniform_stops {
                    // We clamp only when the stops are evenly spaced.
                    // If not, there may be hard stops, and clamping ruins hard stops at 0 and/or 1.
                    // In that case, we must make sure we're using the general "gradient" stage,
                    // which is the only stage that will correctly handle unclamped t.
                    p.push(pipeline::Stage::PadX1);
                }
            }
        }

        // The two-stop case with stops at 0 and 1.
        if self.stops.len() == 2 {
            debug_assert!(self.has_uniform_stops);

            let c0 = self.stops[0].color;
            let c1 = self.stops[1].color;

            p.ctx.evenly_spaced_2_stop_gradient = EvenlySpaced2StopGradientCtx {
                factor: GradientColor::new(
                    c1.red() - c0.red(),
                    c1.green() - c0.green(),
                    c1.blue() - c0.blue(),
                    c1.alpha() - c0.alpha(),
                ),
                bias: GradientColor::from(c0),
            };

            p.push(pipeline::Stage::EvenlySpaced2StopGradient);
        } else {
            // Unlike Skia, we do not support the `evenly_spaced_gradient` stage.
            // In our case, there is no performance difference.

            let mut ctx = GradientCtx::default();

            // Note: In order to handle clamps in search, the search assumes
            // a stop conceptually placed at -inf.
            // Therefore, the max number of stops is `self.points.len()+1`.
            //
            // We also need at least 16 values for lowp pipeline.
            ctx.factors.reserve((self.stops.len() + 1).max(16));
            ctx.biases.reserve((self.stops.len() + 1).max(16));

            ctx.t_values.reserve(self.stops.len() + 1);

            // Remove the dummy stops inserted by Gradient::new
            // because they are naturally handled by the search method.
            let (first_stop, last_stop) = if self.stops.len() > 2 {
                let first = if self.stops[0].color != self.stops[1].color {
                    0
                } else {
                    1
                };

                let len = self.stops.len();
                let last = if self.stops[len - 2].color != self.stops[len - 1].color {
                    len - 1
                } else {
                    len - 2
                };
                (first, last)
            } else {
                (0, 1)
            };

            let mut t_l = self.stops[first_stop].position.get();
            let mut c_l = GradientColor::from(self.stops[first_stop].color);
            ctx.push_const_color(c_l);
            ctx.t_values.push(NormalizedF32::ZERO);
            // N.B. lastStop is the index of the last stop, not one after.
            for i in first_stop..last_stop {
                let t_r = self.stops[i + 1].position.get();
                let c_r = GradientColor::from(self.stops[i + 1].color);
                debug_assert!(t_l <= t_r);
                if t_l < t_r {
                    // For each stop we calculate a bias B and a scale factor F, such that
                    // for any t between stops n and n+1, the color we want is B[n] + F[n]*t.
                    let f = GradientColor::new(
                        (c_r.r - c_l.r) / (t_r - t_l),
                        (c_r.g - c_l.g) / (t_r - t_l),
                        (c_r.b - c_l.b) / (t_r - t_l),
                        (c_r.a - c_l.a) / (t_r - t_l),
                    );
                    ctx.factors.push(f);

                    ctx.biases.push(GradientColor::new(
                        c_l.r - f.r * t_l,
                        c_l.g - f.g * t_l,
                        c_l.b - f.b * t_l,
                        c_l.a - f.a * t_l,
                    ));

                    ctx.t_values.push(NormalizedF32::new_clamped(t_l));
                }

                t_l = t_r;
                c_l = c_r;
            }

            ctx.push_const_color(c_l);
            ctx.t_values.push(NormalizedF32::new_clamped(t_l));

            ctx.len = ctx.factors.len();

            // All lists must have the same length.
            debug_assert_eq!(ctx.factors.len(), ctx.t_values.len());
            debug_assert_eq!(ctx.biases.len(), ctx.t_values.len());

            // Will with zeros until we have enough data to fit into F32x16.
            while ctx.factors.len() < 16 {
                ctx.factors.push(GradientColor::default());
                ctx.biases.push(GradientColor::default());
            }

            p.push(pipeline::Stage::Gradient);
            p.ctx.gradient = ctx;
        }

        if !self.colors_are_opaque {
            p.push(pipeline::Stage::Premultiply);
        }

        push_stages_post(p);

        true
    }

    pub fn apply_opacity(&mut self, opacity: f32) {
        for stop in &mut self.stops {
            stop.color.apply_opacity(opacity);
        }

        self.colors_are_opaque = self.stops.iter().all(|p| p.color.is_opaque());
    }
}
