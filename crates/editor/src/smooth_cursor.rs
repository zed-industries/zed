use std::time::Instant;

use gpui::{Bounds, Hsla, PathBuilder, Pixels, Window, point, px, size};
use smallvec::SmallVec;

use crate::{CursorShape, editor_settings::SmoothCursorSettings};

const SMOOTH_CURSOR_SETTLE_DISTANCE_PX: f32 = 0.35;

/// Corner-based cursor trail animation inspired by Kitty's `cursor_trail.c`.
///
/// The editor keeps the logical cursor exact, while this state only controls
/// the visual interpolation of the local painted cursor.
#[derive(Clone, Debug)]
pub(crate) struct SmoothCursorAnimationState {
    target_bounds: Bounds<Pixels>,
    pub(crate) corners: [gpui::Point<Pixels>; 4],
    pub(crate) last_frame: Instant,
    /// The scroll x-position (in pixels) when the corners were last set.
    /// Used to adjust corners when the scroll position changes between frames.
    pub(crate) scroll_x: gpui::Pixels,
    /// The scroll y-position (in rows) when the corners were last set.
    /// Used to adjust corners when the scroll position changes between frames.
    pub(crate) scroll_y: f64,
}

pub(crate) struct SmoothCursorFrame {
    pub(crate) trail: Option<SmoothCursorTrail>,
    pub(crate) animating: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct SmoothCursorTrail {
    points: [gpui::Point<Pixels>; 4],
    color: Hsla,
}

impl SmoothCursorAnimationState {
    pub(crate) fn new(
        bounds: Bounds<Pixels>,
        now: Instant,
        scroll_x: gpui::Pixels,
        scroll_y: f64,
    ) -> Self {
        Self {
            target_bounds: bounds,
            corners: bounds_corners(bounds),
            last_frame: now,
            scroll_x,
            scroll_y,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn snap_to(
        &mut self,
        bounds: Bounds<Pixels>,
        now: Instant,
        scroll_x: gpui::Pixels,
        scroll_y: f64,
    ) {
        self.target_bounds = bounds;
        self.corners = bounds_corners(bounds);
        self.last_frame = now;
        self.scroll_x = scroll_x;
        self.scroll_y = scroll_y;
    }

    pub(crate) fn retarget(&mut self, bounds: Bounds<Pixels>) {
        // We intentionally do not impose a distance snap here. Long jumps such
        // as `gg` should leave a continuous trail, which is the main part of
        // the Kitty-inspired effect.
        self.target_bounds = bounds;
    }

    pub(crate) fn corners_origin(&self) -> gpui::Point<Pixels> {
        self.corners[0]
    }

    pub(crate) fn inherit_from(&mut self, other: &SmoothCursorAnimationState) {
        self.corners = other.corners;
        self.last_frame = other.last_frame;
        self.scroll_x = other.scroll_x;
        self.scroll_y = other.scroll_y;
    }

    pub(crate) fn step(
        &mut self,
        now: Instant,
        color: Hsla,
        settings: &SmoothCursorSettings,
        line_height: Pixels,
    ) -> SmoothCursorFrame {
        let dt = now
            .duration_since(self.last_frame)
            .as_secs_f32()
            .clamp(1.0 / 240.0, 1.0 / 24.0);
        self.last_frame = now;

        let target_corners = bounds_corners(self.target_bounds);
        let center_x: f32 = self.target_bounds.center().x.into();
        let center_y: f32 = self.target_bounds.center().y.into();
        let width: f32 = self.target_bounds.size.width.into();
        let height: f32 = self.target_bounds.size.height.into();
        let half_diagonal = ((width.powi(2) + height.powi(2)).sqrt() / 2.0).max(0.001);

        let mut dx = [0.0f32; 4];
        let mut dy = [0.0f32; 4];
        let mut dot = [0.0f32; 4];
        let mut animating = false;

        for i in 0..4 {
            let target_x: f32 = target_corners[i].x.into();
            let target_y: f32 = target_corners[i].y.into();
            let current_x: f32 = self.corners[i].x.into();
            let current_y: f32 = self.corners[i].y.into();
            dx[i] = target_x - current_x;
            dy[i] = target_y - current_y;
            let dist = (dx[i] * dx[i] + dy[i] * dy[i]).sqrt();

            if dist > SMOOTH_CURSOR_SETTLE_DISTANCE_PX {
                animating = true;
                let corner_to_center_x = target_x - center_x;
                let corner_to_center_y = target_y - center_y;
                dot[i] = (dx[i] * corner_to_center_x + dy[i] * corner_to_center_y)
                    / (half_diagonal * dist.max(0.001));
            } else {
                self.corners[i] = target_corners[i];
                dx[i] = 0.0;
                dy[i] = 0.0;
                dot[i] = 0.0;
            }
        }

        if animating {
            let min_dot = dot.iter().copied().fold(f32::MAX, f32::min);
            let max_dot = dot.iter().copied().fold(f32::MIN, f32::max);
            let mut decay_fast = settings.leading_smooth_time.as_secs_f32().clamp(0.01, 2.0);
            let mut decay_slow = settings.smooth_time.as_secs_f32().clamp(0.04, 2.0);

            // For large jumps (>= 20 lines), increase the decay times so the
            // trail remains visible even when smooth scroll is racing the
            // cursor to the destination.
            let lh: f32 = line_height.into();
            if lh > 0.0 {
                let max_dy = dy.iter().copied().map(f32::abs).fold(0.0f32, f32::max);
                let line_distance = max_dy / lh;
                if line_distance >= 20.0
                    && settings.large_jump_multiplier > 0.0
                    && settings.large_jump_multiplier != 1.0
                {
                    decay_fast *= settings.large_jump_multiplier;
                    decay_slow *= settings.large_jump_multiplier;
                }
            }

            for i in 0..4 {
                if dx[i] == 0.0 && dy[i] == 0.0 {
                    continue;
                }

                let decay = if (max_dot - min_dot).abs() < 1e-5 {
                    decay_slow
                } else {
                    decay_slow
                        + (decay_fast - decay_slow) * (dot[i] - min_dot) / (max_dot - min_dot)
                };

                let step = 1.0 - (-10.0 * dt / decay).exp2();
                self.corners[i].x += px(dx[i] * step);
                self.corners[i].y += px(dy[i] * step);
            }
        }

        let trail = if settings.trail && settings.trail_opacity > 0.0 && animating {
            let max_dist = (0..4)
                .map(|i| {
                    let target_x: f32 = target_corners[i].x.into();
                    let target_y: f32 = target_corners[i].y.into();
                    let current_x: f32 = self.corners[i].x.into();
                    let current_y: f32 = self.corners[i].y.into();
                    let dx = target_x - current_x;
                    let dy = target_y - current_y;
                    (dx * dx + dy * dy).sqrt()
                })
                .fold(0.0f32, f32::max);

            if max_dist > settings.trail_min_distance {
                let visibility_ratio = (max_dist / (height * 0.5)).clamp(0.0, 1.0);
                let trail_alpha = settings.trail_opacity.clamp(0.0, 1.0) * visibility_ratio;

                Some(SmoothCursorTrail {
                    points: self.corners,
                    color: color.opacity(trail_alpha),
                })
            } else {
                None
            }
        } else {
            None
        };

        SmoothCursorFrame { trail, animating }
    }
}

impl SmoothCursorTrail {
    pub(crate) fn paint(&self, origin: gpui::Point<Pixels>, window: &mut Window) {
        let points = self
            .points
            .iter()
            .map(|point| *point + origin)
            .collect::<SmallVec<[gpui::Point<Pixels>; 4]>>();
        let mut path_builder = PathBuilder::fill();
        path_builder.add_polygon(&points, true);
        if let Ok(path) = path_builder.build() {
            window.paint_path(path, self.color);
        }
    }
}

pub(crate) fn cursor_bounds(
    origin: gpui::Point<Pixels>,
    block_width: Pixels,
    line_height: Pixels,
    shape: CursorShape,
) -> Bounds<Pixels> {
    match shape {
        CursorShape::Bar => Bounds {
            origin,
            size: size(px(2.0), line_height),
        },
        CursorShape::Block | CursorShape::Hollow => Bounds {
            origin,
            size: size(block_width, line_height),
        },
        CursorShape::Underline => Bounds {
            origin: origin + gpui::Point::new(Pixels::ZERO, line_height - px(2.0)),
            size: size(block_width, px(2.0)),
        },
    }
}

fn bounds_corners(bounds: Bounds<Pixels>) -> [gpui::Point<Pixels>; 4] {
    [
        point(bounds.left(), bounds.top()),
        point(bounds.right(), bounds.top()),
        point(bounds.right(), bounds.bottom()),
        point(bounds.left(), bounds.bottom()),
    ]
}
