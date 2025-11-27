//! Shader Showcase demo - dynamic gradients and visual effects.
//!
//! This demo showcases GPUI's GPU rendering capabilities on iOS:
//! - Dynamic gradient backgrounds that respond to touch
//! - Floating orbs with parallax movement
//! - Ripple effects on tap
//! - Color cycling animations

use super::back_button;
use crate::{
    App, Bounds, ColorSpace, Hsla, Pixels, Point, Window, canvas, div, fill, hsla,
    linear_color_stop, linear_gradient, point, prelude::*, px, size,
};
use std::time::{Duration, Instant};

// Number of orbs in the demo (for reference, orbs are created manually)
#[allow(dead_code)]
const ORB_COUNT: usize = 8;
const RIPPLE_DURATION_MS: u64 = 1000;
const MAX_RIPPLES: usize = 5;

/// A floating orb with parallax effect
#[derive(Clone)]
struct Orb {
    base_position: Point<f32>,
    parallax_factor: f32,
    radius: f32,
    color: Hsla,
    pulse_phase: f32,
}

impl Orb {
    fn new(x: f32, y: f32, radius: f32, parallax: f32, hue: f32) -> Self {
        Self {
            base_position: point(x, y),
            parallax_factor: parallax,
            radius,
            color: hsla(hue, 0.7, 0.6, 0.4),
            pulse_phase: hue * std::f32::consts::PI * 2.0,
        }
    }

    fn current_position(&self, touch_offset: Point<f32>, time: f32) -> Point<f32> {
        let pulse = (time * 2.0 + self.pulse_phase).sin() * 5.0;
        point(
            self.base_position.x + touch_offset.x * self.parallax_factor,
            self.base_position.y + touch_offset.y * self.parallax_factor + pulse,
        )
    }

    fn current_radius(&self, time: f32) -> f32 {
        let pulse = (time * 1.5 + self.pulse_phase).sin() * 0.1 + 1.0;
        self.radius * pulse
    }
}

/// An expanding ripple effect
#[derive(Clone)]
struct Ripple {
    center: Point<f32>,
    start_time: Instant,
    duration: Duration,
    color: Hsla,
}

impl Ripple {
    fn new(center: Point<f32>, hue: f32) -> Self {
        Self {
            center,
            start_time: Instant::now(),
            duration: Duration::from_millis(RIPPLE_DURATION_MS),
            color: hsla(hue, 0.8, 0.6, 1.0),
        }
    }

    fn progress(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        (elapsed / self.duration.as_secs_f32()).min(1.0)
    }

    fn is_alive(&self) -> bool {
        self.start_time.elapsed() < self.duration
    }

    fn current_radius(&self, max_radius: f32) -> f32 {
        let progress = self.progress();
        // Ease out for natural expansion
        let eased = 1.0 - (1.0 - progress).powi(3);
        max_radius * eased
    }

    fn current_alpha(&self) -> f32 {
        let progress = self.progress();
        (1.0 - progress).powi(2)
    }
}

/// Shader Showcase demo view
pub struct ShaderShowcase {
    start_time: Instant,
    pub touch_position: Option<Point<f32>>,
    pub screen_center: Point<f32>,
    orbs: Vec<Orb>,
    ripples: Vec<Ripple>,
}

impl ShaderShowcase {
    pub fn new() -> Self {
        // Create orbs at various positions
        let orbs = vec![
            Orb::new(100.0, 200.0, 80.0, 0.1, 0.0),
            Orb::new(300.0, 150.0, 60.0, 0.15, 0.1),
            Orb::new(200.0, 400.0, 100.0, 0.05, 0.2),
            Orb::new(350.0, 500.0, 50.0, 0.2, 0.3),
            Orb::new(50.0, 600.0, 70.0, 0.12, 0.4),
            Orb::new(280.0, 700.0, 90.0, 0.08, 0.5),
            Orb::new(150.0, 300.0, 40.0, 0.25, 0.6),
            Orb::new(320.0, 350.0, 55.0, 0.18, 0.7),
        ];

        Self {
            start_time: Instant::now(),
            touch_position: None,
            screen_center: point(200.0, 400.0),
            orbs,
            ripples: Vec::new(),
        }
    }

    fn time(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    fn gradient_angle(&self) -> f32 {
        if let Some(touch) = self.touch_position {
            let dx = touch.x - self.screen_center.x;
            let dy = touch.y - self.screen_center.y;
            (dy.atan2(dx).to_degrees() + 90.0) % 360.0
        } else {
            // Default: slowly rotating gradient
            (self.time() * 20.0) % 360.0
        }
    }

    fn touch_offset(&self) -> Point<f32> {
        if let Some(touch) = self.touch_position {
            point(
                touch.x - self.screen_center.x,
                touch.y - self.screen_center.y,
            )
        } else {
            point(0.0, 0.0)
        }
    }

    fn cycling_hue(&self, base: f32) -> f32 {
        (base + self.time() * 0.05) % 1.0
    }

    pub fn spawn_ripple(&mut self, position: Point<f32>) {
        if self.ripples.len() >= MAX_RIPPLES {
            self.ripples.remove(0);
        }
        let hue = self.cycling_hue(self.ripples.len() as f32 * 0.15);
        self.ripples.push(Ripple::new(position, hue));
    }

    fn handle_touch_down(&mut self, position: Point<f32>) {
        self.touch_position = Some(position);
        self.spawn_ripple(position);
    }

    fn handle_touch_move(&mut self, position: Point<f32>) {
        self.touch_position = Some(position);
    }

    fn handle_touch_up(&mut self) {
        self.touch_position = None;
    }

    pub fn render_with_back_button<F>(
        &mut self,
        window: &mut Window,
        on_back: F,
    ) -> impl IntoElement
    where
        F: Fn(&(), &mut Window, &mut App) + 'static,
    {
        // Request continuous animation frame
        window.request_animation_frame();

        // Remove dead ripples
        self.ripples.retain(|r| r.is_alive());

        // Get current state for rendering
        let time = self.time();
        let gradient_angle = self.gradient_angle();
        let touch_offset = self.touch_offset();
        let hue1 = self.cycling_hue(0.6);
        let hue2 = self.cycling_hue(0.9);
        let orbs = self.orbs.clone();
        let ripples = self.ripples.clone();

        div()
            .size_full()
            // Dynamic gradient background
            .bg(linear_gradient(
                gradient_angle,
                linear_color_stop(hsla(hue1, 0.8, 0.15, 1.0), 0.0),
                linear_color_stop(hsla(hue2, 0.7, 0.25, 1.0), 1.0),
            )
            .color_space(ColorSpace::Oklab))
            // Canvas for custom effects
            .child(
                canvas(
                    move |bounds, _window, _cx| {
                        let bounds_f32 = Bounds {
                            origin: point(bounds.origin.x.0, bounds.origin.y.0),
                            size: size(bounds.size.width.0, bounds.size.height.0),
                        };
                        let max_ripple_radius =
                            (bounds_f32.size.width.max(bounds_f32.size.height)) * 0.7;
                        (
                            bounds_f32,
                            max_ripple_radius,
                            time,
                            touch_offset,
                            orbs.clone(),
                            ripples.clone(),
                        )
                    },
                    move |_bounds,
                          (_bounds_f32, max_ripple_radius, time, touch_offset, orbs, ripples),
                          window,
                          _cx| {
                        // Draw orbs (back to front by parallax factor)
                        let mut sorted_orbs: Vec<_> = orbs.iter().collect();
                        sorted_orbs.sort_by(|a, b| {
                            a.parallax_factor.partial_cmp(&b.parallax_factor).unwrap()
                        });

                        for orb in sorted_orbs {
                            let pos = orb.current_position(touch_offset, time);
                            let radius = orb.current_radius(time);

                            // Draw glow layers (outer to inner)
                            for i in (0..4).rev() {
                                let glow_radius = radius * (1.0 + i as f32 * 0.5);
                                let glow_alpha = orb.color.a * (0.1 / (i as f32 + 1.0));
                                let glow_color =
                                    hsla(orb.color.h, orb.color.s, orb.color.l, glow_alpha);
                                paint_circle(
                                    window,
                                    point(px(pos.x), px(pos.y)),
                                    px(glow_radius),
                                    glow_color,
                                );
                            }

                            // Draw core
                            paint_circle(
                                window,
                                point(px(pos.x), px(pos.y)),
                                px(radius),
                                orb.color,
                            );

                            // Draw highlight
                            let highlight = hsla(orb.color.h, orb.color.s * 0.5, 0.9, 0.3);
                            paint_circle(
                                window,
                                point(px(pos.x - radius * 0.3), px(pos.y - radius * 0.3)),
                                px(radius * 0.4),
                                highlight,
                            );
                        }

                        // Draw ripples
                        for ripple in ripples.iter() {
                            let radius = ripple.current_radius(max_ripple_radius);
                            let alpha = ripple.current_alpha() * 0.5;
                            let color = hsla(ripple.color.h, ripple.color.s, ripple.color.l, alpha);

                            // Draw expanding ring
                            paint_ring(
                                window,
                                point(px(ripple.center.x), px(ripple.center.y)),
                                px(radius),
                                px(3.0),
                                color,
                            );
                        }
                    },
                )
                .size_full(),
            )
            // Title overlay
            .child(
                div()
                    .absolute()
                    .top(px(100.0))
                    .left_0()
                    .right_0()
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .text_2xl()
                            .text_color(hsla(0.0, 0.0, 1.0, 0.8))
                            .child("Shader Showcase"),
                    ),
            )
            // Instructions
            .child(
                div()
                    .absolute()
                    .bottom(px(100.0))
                    .left_0()
                    .right_0()
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .bg(hsla(0.0, 0.0, 0.0, 0.3))
                            .rounded_lg()
                            .text_color(hsla(0.0, 0.0, 1.0, 0.9))
                            .text_sm()
                            .child("Touch to control gradients & create ripples"),
                    ),
            )
            // Back button
            .child(back_button(on_back))
    }

    /// Update screen center based on bounds
    pub fn set_screen_center(&mut self, center: Point<f32>) {
        self.screen_center = center;
    }
}

/// Paint a filled circle
fn paint_circle(window: &mut Window, center: Point<Pixels>, radius: Pixels, color: Hsla) {
    let bounds = Bounds {
        origin: point(center.x - radius, center.y - radius),
        size: size(radius * 2.0, radius * 2.0),
    };
    window.paint_quad(fill(bounds, color).corner_radii(radius));
}

/// Paint a ring (circle outline)
fn paint_ring(
    window: &mut Window,
    center: Point<Pixels>,
    radius: Pixels,
    thickness: Pixels,
    color: Hsla,
) {
    // Draw as two circles: outer filled, inner transparent
    // Since we don't have stroke, we'll draw a series of circles
    let steps = 36;
    let angle_step = std::f32::consts::PI * 2.0 / steps as f32;

    for i in 0..steps {
        let angle = angle_step * i as f32;
        let x = center.x + px(radius.0 * angle.cos());
        let y = center.y + px(radius.0 * angle.sin());
        paint_circle(window, point(x, y), thickness, color);
    }
}
