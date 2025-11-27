//! Animation Playground demo - bouncing balls with physics and particle effects.
//!
//! This demo showcases GPUI's animation and rendering capabilities on iOS:
//! - Tap to spawn bouncing balls with physics simulation
//! - Balls have trails and particle effects
//! - Demonstrates smooth 60fps rendering

use super::{BACKGROUND, TEXT, back_button, random_color};
use crate::{
    App, Bounds, Hsla, MouseButton, MouseDownEvent, MouseUpEvent, Pixels, Point, Window, canvas,
    div, fill, hsla, point, prelude::*, px, rgb, size,
};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

// Physics constants
const GRAVITY: f32 = 980.0; // pixels/sec^2
const BOUNCE_DAMPING: f32 = 0.7; // velocity retained after bounce
const FRICTION: f32 = 0.995; // velocity multiplier per frame
const MAX_BALLS: usize = 30;
const TRAIL_LENGTH: usize = 12;
const BALL_RADIUS: f32 = 20.0;
const PARTICLE_COUNT: usize = 12;
const PARTICLE_DURATION_MS: u64 = 600;

/// A bouncing ball with physics
#[derive(Clone)]
struct Ball {
    position: Point<f32>,
    velocity: Point<f32>,
    radius: f32,
    color: Hsla,
    trail: VecDeque<Point<f32>>,
}

impl Ball {
    fn new(id: usize, position: Point<f32>, velocity: Point<f32>) -> Self {
        let color_rgb = random_color(id);
        Self {
            position,
            velocity,
            radius: BALL_RADIUS,
            color: rgb(color_rgb).into(),
            trail: VecDeque::with_capacity(TRAIL_LENGTH),
        }
    }

    fn update(&mut self, dt: f32, bounds: &Bounds<f32>) {
        // Store trail position
        if self.trail.len() >= TRAIL_LENGTH {
            self.trail.pop_front();
        }
        self.trail.push_back(self.position);

        // Apply gravity
        self.velocity.y += GRAVITY * dt;

        // Apply friction
        self.velocity.x *= FRICTION;
        self.velocity.y *= FRICTION;

        // Update position
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;

        // Bounce off walls
        let min_x = bounds.origin.x + self.radius;
        let max_x = bounds.origin.x + bounds.size.width - self.radius;
        let min_y = bounds.origin.y + self.radius;
        let max_y = bounds.origin.y + bounds.size.height - self.radius;

        if self.position.x < min_x {
            self.position.x = min_x;
            self.velocity.x = -self.velocity.x * BOUNCE_DAMPING;
        } else if self.position.x > max_x {
            self.position.x = max_x;
            self.velocity.x = -self.velocity.x * BOUNCE_DAMPING;
        }

        if self.position.y < min_y {
            self.position.y = min_y;
            self.velocity.y = -self.velocity.y * BOUNCE_DAMPING;
        } else if self.position.y > max_y {
            self.position.y = max_y;
            self.velocity.y = -self.velocity.y * BOUNCE_DAMPING;
        }
    }
}

/// A particle for burst effects
#[derive(Clone)]
struct Particle {
    position: Point<f32>,
    velocity: Point<f32>,
    start_time: Instant,
    duration: Duration,
    color: Hsla,
    size: f32,
}

impl Particle {
    fn new(position: Point<f32>, angle: f32, speed: f32, color: Hsla) -> Self {
        let velocity = point(angle.cos() * speed, angle.sin() * speed);
        Self {
            position,
            velocity,
            start_time: Instant::now(),
            duration: Duration::from_millis(PARTICLE_DURATION_MS),
            color,
            size: 8.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        // Slow down
        self.velocity.x *= 0.98;
        self.velocity.y *= 0.98;
    }

    fn progress(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        (elapsed / self.duration.as_secs_f32()).min(1.0)
    }

    fn is_alive(&self) -> bool {
        self.start_time.elapsed() < self.duration
    }
}

/// Animation Playground demo view
pub struct AnimationPlayground {
    balls: Vec<Ball>,
    particles: Vec<Particle>,
    last_frame_time: Instant,
    pub next_ball_id: usize,
    pub touch_start: Option<(Point<f32>, Instant)>,
    pub current_touch: Option<Point<f32>>,
    bounds: Bounds<f32>,
    #[allow(dead_code)]
    on_back: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl AnimationPlayground {
    pub fn new() -> Self {
        Self {
            balls: Vec::new(),
            particles: Vec::new(),
            last_frame_time: Instant::now(),
            next_ball_id: 0,
            touch_start: None,
            current_touch: None,
            bounds: Bounds {
                origin: point(0.0, 0.0),
                size: size(400.0, 800.0),
            },
            on_back: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_on_back<F>(mut self, on_back: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_back = Some(Box::new(on_back));
        self
    }

    pub fn spawn_ball(&mut self, position: Point<f32>, velocity: Point<f32>) {
        if self.balls.len() >= MAX_BALLS {
            self.balls.remove(0);
        }
        let ball = Ball::new(self.next_ball_id, position, velocity);
        self.next_ball_id += 1;
        self.balls.push(ball);
    }

    pub fn spawn_particles(&mut self, position: Point<f32>, color: Hsla) {
        let angle_step = std::f32::consts::PI * 2.0 / PARTICLE_COUNT as f32;
        for i in 0..PARTICLE_COUNT {
            let angle = angle_step * i as f32;
            let speed = 200.0 + (i as f32 * 20.0);
            self.particles
                .push(Particle::new(position, angle, speed, color));
        }
    }

    fn update_physics(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        // Cap dt to prevent huge jumps
        let dt = dt.min(0.05);

        // Update balls
        for ball in &mut self.balls {
            ball.update(dt, &self.bounds);
        }

        // Update particles
        for particle in &mut self.particles {
            particle.update(dt);
        }

        // Remove dead particles
        self.particles.retain(|p| p.is_alive());
    }

    fn handle_touch_down(&mut self, event: &MouseDownEvent, cx: &mut crate::Context<Self>) {
        let pos = point(event.position.x.0, event.position.y.0);
        self.touch_start = Some((pos, Instant::now()));
        self.current_touch = Some(pos);
        cx.notify();
    }

    fn handle_touch_up(&mut self, event: &MouseUpEvent, cx: &mut crate::Context<Self>) {
        let position = point(event.position.x.0, event.position.y.0);

        if let Some((start_pos, start_time)) = self.touch_start.take() {
            let elapsed = start_time.elapsed();
            let dx = position.x - start_pos.x;
            let dy = position.y - start_pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // If short tap with little movement, spawn particles
            if elapsed < Duration::from_millis(200) && distance < 20.0 {
                let color_rgb = random_color(self.next_ball_id);
                self.spawn_particles(position, rgb(color_rgb).into());
                self.next_ball_id += 1;
            } else {
                // Calculate velocity from drag
                let dt = elapsed.as_secs_f32().max(0.01);
                let velocity = point(dx / dt * 0.5, dy / dt * 0.5);
                self.spawn_ball(start_pos, velocity);
            }
        }
        self.current_touch = None;
        cx.notify();
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

        // Update physics each frame
        self.update_physics();

        // Clone data for rendering
        let balls = self.balls.clone();
        let particles = self.particles.clone();
        let touch_start = self.touch_start.map(|(p, _)| p);
        let current_touch = self.current_touch;

        div()
            .size_full()
            .bg(rgb(BACKGROUND))
            .child(
                canvas(
                    move |bounds, _window, _cx| {
                        // Convert bounds to f32 for physics
                        let bounds_f32 = Bounds {
                            origin: point(bounds.origin.x.0, bounds.origin.y.0),
                            size: size(bounds.size.width.0, bounds.size.height.0),
                        };
                        (
                            bounds_f32,
                            balls.clone(),
                            particles.clone(),
                            touch_start,
                            current_touch,
                        )
                    },
                    move |_bounds,
                          (_bounds_f32, balls, particles, touch_start, current_touch),
                          window,
                          _cx| {
                        // Draw trails
                        for ball in &balls {
                            for (i, &trail_pos) in ball.trail.iter().enumerate() {
                                let alpha = (i as f32 / TRAIL_LENGTH as f32) * 0.4;
                                let trail_size =
                                    ball.radius * (0.3 + 0.7 * i as f32 / TRAIL_LENGTH as f32);
                                let color = hsla(ball.color.h, ball.color.s, ball.color.l, alpha);
                                paint_circle(
                                    window,
                                    point(px(trail_pos.x), px(trail_pos.y)),
                                    px(trail_size),
                                    color,
                                );
                            }
                        }

                        // Draw particles
                        for particle in &particles {
                            let progress = particle.progress();
                            let alpha = 1.0 - progress;
                            let size = particle.size * (1.0 - progress * 0.5);
                            let color =
                                hsla(particle.color.h, particle.color.s, particle.color.l, alpha);
                            paint_circle(
                                window,
                                point(px(particle.position.x), px(particle.position.y)),
                                px(size),
                                color,
                            );
                        }

                        // Draw balls
                        for ball in &balls {
                            paint_circle(
                                window,
                                point(px(ball.position.x), px(ball.position.y)),
                                px(ball.radius),
                                ball.color,
                            );
                            // Inner highlight
                            let highlight = hsla(ball.color.h, ball.color.s * 0.5, 0.9, 0.5);
                            paint_circle(
                                window,
                                point(
                                    px(ball.position.x - ball.radius * 0.3),
                                    px(ball.position.y - ball.radius * 0.3),
                                ),
                                px(ball.radius * 0.3),
                                highlight,
                            );
                        }

                        // Draw drag line if dragging
                        if let (Some(start), Some(current)) = (touch_start, current_touch) {
                            // Draw a line from start to current (using dots)
                            let dx = current.x - start.x;
                            let dy = current.y - start.y;
                            let dist = (dx * dx + dy * dy).sqrt();
                            let steps = (dist / 10.0) as i32;
                            for i in 0..=steps {
                                let t = i as f32 / steps.max(1) as f32;
                                let x = start.x + dx * t;
                                let y = start.y + dy * t;
                                paint_circle(
                                    window,
                                    point(px(x), px(y)),
                                    px(3.0),
                                    hsla(0.0, 0.0, 1.0, 0.5),
                                );
                            }
                        }
                    },
                )
                .size_full(),
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
                            .bg(hsla(0.0, 0.0, 0.1, 0.8))
                            .rounded_lg()
                            .text_color(rgb(TEXT))
                            .text_sm()
                            .child("Tap to burst, drag to throw balls"),
                    ),
            )
            // Back button
            .child(back_button(on_back))
    }

    /// Update bounds from the canvas callback
    pub fn set_bounds(&mut self, bounds: Bounds<f32>) {
        self.bounds = bounds;
    }
}

impl crate::Render for AnimationPlayground {
    fn render(&mut self, window: &mut Window, cx: &mut crate::Context<Self>) -> impl IntoElement {
        // Request continuous animation frame
        window.request_animation_frame();

        // Update physics each frame
        self.update_physics();

        // Clone data for rendering
        let balls = self.balls.clone();
        let particles = self.particles.clone();
        let touch_start = self.touch_start.map(|(p, _)| p);
        let current_touch = self.current_touch;

        div()
            .size_full()
            .bg(rgb(BACKGROUND))
            // Touch handling layer
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event, _window, cx| {
                    this.handle_touch_down(event, cx);
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, event, _window, cx| {
                    this.handle_touch_up(event, cx);
                }),
            )
            .child(
                canvas(
                    move |bounds, _window, _cx| {
                        let bounds_f32 = Bounds {
                            origin: point(bounds.origin.x.0, bounds.origin.y.0),
                            size: size(bounds.size.width.0, bounds.size.height.0),
                        };
                        (
                            bounds_f32,
                            balls.clone(),
                            particles.clone(),
                            touch_start,
                            current_touch,
                        )
                    },
                    move |_bounds,
                          (_bounds_f32, balls, particles, touch_start, current_touch),
                          window,
                          _cx| {
                        // Draw trails
                        for ball in &balls {
                            for (i, &trail_pos) in ball.trail.iter().enumerate() {
                                let alpha = (i as f32 / TRAIL_LENGTH as f32) * 0.4;
                                let trail_size =
                                    ball.radius * (0.3 + 0.7 * i as f32 / TRAIL_LENGTH as f32);
                                let color = hsla(ball.color.h, ball.color.s, ball.color.l, alpha);
                                paint_circle(
                                    window,
                                    point(px(trail_pos.x), px(trail_pos.y)),
                                    px(trail_size),
                                    color,
                                );
                            }
                        }

                        // Draw particles
                        for particle in &particles {
                            let progress = particle.progress();
                            let alpha = 1.0 - progress;
                            let psize = particle.size * (1.0 - progress * 0.5);
                            let color =
                                hsla(particle.color.h, particle.color.s, particle.color.l, alpha);
                            paint_circle(
                                window,
                                point(px(particle.position.x), px(particle.position.y)),
                                px(psize),
                                color,
                            );
                        }

                        // Draw balls
                        for ball in &balls {
                            paint_circle(
                                window,
                                point(px(ball.position.x), px(ball.position.y)),
                                px(ball.radius),
                                ball.color,
                            );
                            let highlight = hsla(ball.color.h, ball.color.s * 0.5, 0.9, 0.5);
                            paint_circle(
                                window,
                                point(
                                    px(ball.position.x - ball.radius * 0.3),
                                    px(ball.position.y - ball.radius * 0.3),
                                ),
                                px(ball.radius * 0.3),
                                highlight,
                            );
                        }

                        // Draw drag line if dragging
                        if let (Some(start), Some(current)) = (touch_start, current_touch) {
                            let dx = current.x - start.x;
                            let dy = current.y - start.y;
                            let dist = (dx * dx + dy * dy).sqrt();
                            let steps = (dist / 10.0) as i32;
                            for i in 0..=steps {
                                let t = i as f32 / steps.max(1) as f32;
                                let x = start.x + dx * t;
                                let y = start.y + dy * t;
                                paint_circle(
                                    window,
                                    point(px(x), px(y)),
                                    px(3.0),
                                    hsla(0.0, 0.0, 1.0, 0.5),
                                );
                            }
                        }
                    },
                )
                .size_full(),
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
                            .bg(hsla(0.0, 0.0, 0.1, 0.8))
                            .rounded_lg()
                            .text_color(rgb(TEXT))
                            .text_sm()
                            .child("Tap to burst, drag to throw balls"),
                    ),
            )
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
