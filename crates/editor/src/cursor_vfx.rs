use std::time::{Duration, Instant};

use gpui::{Bounds, Hsla, Pixels, Point, Window, point, size};

/// Cursor visual effect mode for particle animations.
/// Provides enhanced visual feedback during cursor movement.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum CursorVfxMode {
    /// No particle effects
    #[default]
    None,
    /// Line of particles along movement path with spiral/curl effect
    Railgun,
    /// Dense particle stream in movement direction
    Torpedo,
    /// Scattered sparkle particles that drift and fall
    Pixiedust,
    /// Burst effect at cursor stop point
    Sonicboom,
    /// Concentric rings at cursor stop point
    Ripple,
    /// Geometric outline effect at cursor stop point
    Wireframe,
}

/// Default VFX settings
const DEFAULT_OPACITY: f32 = 200.0;
const DEFAULT_PARTICLE_LIFETIME: f32 = 0.5;
const DEFAULT_PARTICLE_DENSITY: f32 = 0.7;
const DEFAULT_PARTICLE_SPEED: f32 = 10.0;
const DEFAULT_PARTICLE_PHASE: f32 = 1.5;
const DEFAULT_PARTICLE_CURL: f32 = 1.0;
const DEFAULT_HIGHLIGHT_LIFETIME: f32 = 0.2;

/// Configuration for cursor VFX particle system.
#[derive(Debug, Clone)]
pub struct CursorVfxConfig {
    pub mode: CursorVfxMode,
    pub opacity: f32,
    pub particle_lifetime: Duration,
    pub highlight_lifetime: Duration,
    pub particle_density: f32,
    pub particle_speed: f32,
    pub particle_phase: f32,
    pub particle_curl: f32,
}

impl Default for CursorVfxConfig {
    fn default() -> Self {
        Self {
            mode: CursorVfxMode::None,
            opacity: DEFAULT_OPACITY,
            particle_lifetime: Duration::from_secs_f32(DEFAULT_PARTICLE_LIFETIME),
            highlight_lifetime: Duration::from_secs_f32(DEFAULT_HIGHLIGHT_LIFETIME),
            particle_density: DEFAULT_PARTICLE_DENSITY,
            particle_speed: DEFAULT_PARTICLE_SPEED,
            particle_phase: DEFAULT_PARTICLE_PHASE,
            particle_curl: DEFAULT_PARTICLE_CURL,
        }
    }
}

impl CursorVfxConfig {
    pub fn from_settings(
        mode: Option<CursorVfxMode>,
        opacity: Option<f32>,
        lifetime: Option<f32>,
        density: Option<f32>,
        speed: Option<f32>,
        phase: Option<f32>,
        curl: Option<f32>,
    ) -> Self {
        Self {
            mode: mode.unwrap_or(CursorVfxMode::None),
            opacity: opacity.unwrap_or(DEFAULT_OPACITY),
            particle_lifetime: Duration::from_secs_f32(
                lifetime.unwrap_or(DEFAULT_PARTICLE_LIFETIME),
            ),
            highlight_lifetime: Duration::from_secs_f32(DEFAULT_HIGHLIGHT_LIFETIME),
            particle_density: density.unwrap_or(DEFAULT_PARTICLE_DENSITY),
            particle_speed: speed.unwrap_or(DEFAULT_PARTICLE_SPEED),
            particle_phase: phase.unwrap_or(DEFAULT_PARTICLE_PHASE),
            particle_curl: curl.unwrap_or(DEFAULT_PARTICLE_CURL),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.mode != CursorVfxMode::None
    }
}

#[derive(Debug, Clone)]
struct Particle {
    position: Point<Pixels>,
    velocity: Point<Pixels>,
    birth: Instant,
    lifetime: Duration,
    base_opacity: f32,
    phase_offset: f32,
}

impl Particle {
    fn new(
        position: Point<Pixels>,
        velocity: Point<Pixels>,
        lifetime: Duration,
        opacity: f32,
        phase_offset: f32,
    ) -> Self {
        Self {
            position,
            velocity,
            birth: Instant::now(),
            lifetime,
            base_opacity: opacity,
            phase_offset,
        }
    }

    fn age(&self) -> f32 {
        self.birth.elapsed().as_secs_f32()
    }

    fn normalized_age(&self) -> f32 {
        (self.age() / self.lifetime.as_secs_f32()).min(1.0)
    }

    fn opacity(&self) -> f32 {
        let t = self.normalized_age();
        self.base_opacity * (1.0 - t)
    }

    fn is_alive(&self) -> bool {
        self.birth.elapsed() < self.lifetime
    }

    fn update(&mut self, dt: f32) {
        self.position.x = self.position.x + self.velocity.x * dt;
        self.position.y = self.position.y + self.velocity.y * dt;
    }
}

#[derive(Debug, Clone)]
struct HighlightEffect {
    center: Point<Pixels>,
    birth: Instant,
    lifetime: Duration,
    base_opacity: f32,
    mode: CursorVfxMode,
}

impl HighlightEffect {
    fn new(center: Point<Pixels>, lifetime: Duration, opacity: f32, mode: CursorVfxMode) -> Self {
        Self {
            center,
            birth: Instant::now(),
            lifetime,
            base_opacity: opacity,
            mode,
        }
    }

    fn normalized_age(&self) -> f32 {
        (self.birth.elapsed().as_secs_f32() / self.lifetime.as_secs_f32()).min(1.0)
    }

    fn is_alive(&self) -> bool {
        self.birth.elapsed() < self.lifetime
    }
}

#[derive(Debug)]
pub struct CursorVfxSystem {
    config: CursorVfxConfig,
    particles: Vec<Particle>,
    highlights: Vec<HighlightEffect>,
    last_pos: Option<Point<Pixels>>,
    was_moving: bool,
}

impl CursorVfxSystem {
    pub fn new(config: CursorVfxConfig) -> Self {
        Self {
            config,
            particles: Vec::with_capacity(200),
            highlights: Vec::with_capacity(10),
            last_pos: None,
            was_moving: false,
        }
    }

    pub fn set_config(&mut self, config: CursorVfxConfig) {
        self.config = config;
        if !self.config.is_enabled() {
            self.particles.clear();
            self.highlights.clear();
        }
    }

    pub fn config(&self) -> &CursorVfxConfig {
        &self.config
    }

    pub fn is_animating(&self) -> bool {
        !self.particles.is_empty() || !self.highlights.is_empty()
    }

    pub fn update(&mut self, cursor_pos: Point<Pixels>, dt: f32) {
        if !self.config.is_enabled() {
            return;
        }

        self.particles.retain(|p| p.is_alive());
        self.highlights.retain(|h| h.is_alive());

        for particle in &mut self.particles {
            particle.update(dt);

            if self.config.mode == CursorVfxMode::Railgun {
                let age = particle.age();
                let curl = self.config.particle_curl;
                let phase = particle.phase_offset + age * curl;
                let curl_factor = phase.sin() * 20.0;

                let vx = f32::from(particle.velocity.x);
                let vy = f32::from(particle.velocity.y);
                let speed = (vx * vx + vy * vy).sqrt();
                if speed > 0.1 {
                    let perp_x = -vy / speed;
                    let perp_y = vx / speed;
                    particle.position.x =
                        particle.position.x + Pixels::from(perp_x * curl_factor * dt);
                    particle.position.y =
                        particle.position.y + Pixels::from(perp_y * curl_factor * dt);
                }
            }
        }

        if let Some(last) = self.last_pos {
            let dx = f32::from(cursor_pos.x - last.x);
            let dy = f32::from(cursor_pos.y - last.y);
            let distance = (dx * dx + dy * dy).sqrt();

            if distance > 1.0 {
                self.was_moving = true;
                self.spawn_particles_along_path(last, cursor_pos, distance);
            } else if self.was_moving {
                self.was_moving = false;
                self.spawn_stop_effect(cursor_pos);
            }
        }

        self.last_pos = Some(cursor_pos);
    }

    fn spawn_particles_along_path(
        &mut self,
        from: Point<Pixels>,
        to: Point<Pixels>,
        distance: f32,
    ) {
        match self.config.mode {
            CursorVfxMode::None => {}
            CursorVfxMode::Railgun | CursorVfxMode::Torpedo | CursorVfxMode::Pixiedust => {
                self.spawn_trail_particles(from, to, distance);
            }
            CursorVfxMode::Sonicboom | CursorVfxMode::Ripple | CursorVfxMode::Wireframe => {}
        }
    }

    fn spawn_trail_particles(&mut self, from: Point<Pixels>, to: Point<Pixels>, distance: f32) {
        let count = ((distance * self.config.particle_density) as usize).clamp(1, 20);

        let dx = f32::from(to.x - from.x);
        let dy = f32::from(to.y - from.y);

        for i in 0..count {
            let t = i as f32 / count.max(1) as f32;

            let pos = point(from.x + Pixels::from(dx * t), from.y + Pixels::from(dy * t));

            let vel = match self.config.mode {
                CursorVfxMode::Railgun => {
                    let speed = self.config.particle_speed;
                    point(
                        Pixels::from(-dx * speed * 0.05),
                        Pixels::from(-dy * speed * 0.05),
                    )
                }
                CursorVfxMode::Torpedo => {
                    let speed = self.config.particle_speed;
                    point(
                        Pixels::from(-dx * speed * 0.1),
                        Pixels::from(-dy * speed * 0.1),
                    )
                }
                CursorVfxMode::Pixiedust => {
                    let rand_x = ((i * 7919) % 100) as f32 / 100.0 - 0.5;
                    let rand_y = ((i * 104729) % 100) as f32 / 100.0;
                    point(
                        Pixels::from(rand_x * self.config.particle_speed),
                        Pixels::from(rand_y * self.config.particle_speed * 0.5),
                    )
                }
                _ => point(Pixels::ZERO, Pixels::ZERO),
            };

            let phase_offset = match self.config.mode {
                CursorVfxMode::Railgun => t * self.config.particle_phase * std::f32::consts::TAU,
                _ => 0.0,
            };

            self.particles.push(Particle::new(
                pos,
                vel,
                self.config.particle_lifetime,
                self.config.opacity / 255.0,
                phase_offset,
            ));
        }
    }

    fn spawn_stop_effect(&mut self, pos: Point<Pixels>) {
        match self.config.mode {
            CursorVfxMode::Sonicboom | CursorVfxMode::Ripple | CursorVfxMode::Wireframe => {
                self.highlights.push(HighlightEffect::new(
                    pos,
                    self.config.highlight_lifetime,
                    self.config.opacity / 255.0,
                    self.config.mode,
                ));
            }
            _ => {}
        }
    }

    /// Alias for is_animating() - kept for backwards compatibility.
    pub fn is_active(&self) -> bool {
        self.is_animating()
    }

    pub fn paint(&self, origin: Point<Pixels>, window: &mut Window, color: Hsla) {
        if !self.config.is_enabled() {
            return;
        }

        for particle in &self.particles {
            let opacity = particle.opacity();
            if opacity > 0.01 {
                let c = Hsla {
                    a: color.a * opacity,
                    ..color
                };
                let pos = point(
                    particle.position.x + origin.x,
                    particle.position.y + origin.y,
                );
                let bounds = Bounds {
                    origin: pos,
                    size: size(Pixels::from(3.0), Pixels::from(3.0)),
                };
                window.paint_quad(gpui::fill(bounds, c));
            }
        }

        for highlight in &self.highlights {
            let t = highlight.normalized_age();
            let opacity = highlight.base_opacity * (1.0 - t);

            if opacity > 0.01 {
                let c = Hsla {
                    a: color.a * opacity,
                    ..color
                };

                match highlight.mode {
                    CursorVfxMode::Sonicboom => {
                        let radius = t * 30.0;
                        self.paint_burst(highlight.center, origin, radius, c, window);
                    }
                    CursorVfxMode::Ripple => {
                        let radius = t * 40.0;
                        self.paint_ripple(highlight.center, origin, radius, c, window);
                    }
                    CursorVfxMode::Wireframe => {
                        let size = 20.0 + t * 10.0;
                        self.paint_wireframe(highlight.center, origin, size, c, window);
                    }
                    _ => {}
                }
            }
        }
    }

    fn paint_burst(
        &self,
        center: Point<Pixels>,
        origin: Point<Pixels>,
        radius: f32,
        color: Hsla,
        window: &mut Window,
    ) {
        let pos = point(center.x + origin.x, center.y + origin.y);
        let num_rays = 8;
        for i in 0..num_rays {
            let angle = (i as f32 / num_rays as f32) * std::f32::consts::TAU;
            let end_x = pos.x + Pixels::from(angle.cos() * radius);
            let end_y = pos.y + Pixels::from(angle.sin() * radius);

            let bounds = Bounds {
                origin: point(
                    (pos.x + end_x) / 2.0 - Pixels::from(1.0),
                    (pos.y + end_y) / 2.0 - Pixels::from(1.0),
                ),
                size: size(Pixels::from(2.0), Pixels::from(2.0)),
            };
            window.paint_quad(gpui::fill(bounds, color));
        }
    }

    fn paint_ripple(
        &self,
        center: Point<Pixels>,
        origin: Point<Pixels>,
        radius: f32,
        color: Hsla,
        window: &mut Window,
    ) {
        let pos = point(center.x + origin.x, center.y + origin.y);
        let num_points = 16;
        for i in 0..num_points {
            let angle = (i as f32 / num_points as f32) * std::f32::consts::TAU;
            let point_x = pos.x + Pixels::from(angle.cos() * radius);
            let point_y = pos.y + Pixels::from(angle.sin() * radius);

            let bounds = Bounds {
                origin: point(point_x - Pixels::from(1.5), point_y - Pixels::from(1.5)),
                size: size(Pixels::from(3.0), Pixels::from(3.0)),
            };
            window.paint_quad(gpui::fill(bounds, color));
        }
    }

    fn paint_wireframe(
        &self,
        center: Point<Pixels>,
        origin: Point<Pixels>,
        size: f32,
        color: Hsla,
        window: &mut Window,
    ) {
        let pos = point(center.x + origin.x, center.y + origin.y);
        let half = size / 2.0;

        let corners = [
            point(pos.x - Pixels::from(half), pos.y - Pixels::from(half)),
            point(pos.x + Pixels::from(half), pos.y - Pixels::from(half)),
            point(pos.x + Pixels::from(half), pos.y + Pixels::from(half)),
            point(pos.x - Pixels::from(half), pos.y + Pixels::from(half)),
        ];

        for corner in corners {
            let bounds = Bounds {
                origin: point(corner.x - Pixels::from(2.0), corner.y - Pixels::from(2.0)),
                size: gpui::size(Pixels::from(4.0), Pixels::from(4.0)),
            };
            window.paint_quad(gpui::fill(bounds, color));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_vfx_does_nothing() {
        let config = CursorVfxConfig::default();
        let mut vfx = CursorVfxSystem::new(config);

        vfx.update(point(Pixels::from(100.0), Pixels::from(100.0)), 0.016);
        assert!(!vfx.is_active());
    }

    #[test]
    fn railgun_spawns_particles() {
        let mut config = CursorVfxConfig::default();
        config.mode = CursorVfxMode::Railgun;
        let mut vfx = CursorVfxSystem::new(config);

        vfx.update(point(Pixels::from(0.0), Pixels::from(0.0)), 0.016);
        vfx.update(point(Pixels::from(100.0), Pixels::from(0.0)), 0.016);

        assert!(vfx.is_active());
    }

    #[test]
    fn particles_expire() {
        let mut config = CursorVfxConfig::default();
        config.mode = CursorVfxMode::Pixiedust;
        config.particle_lifetime = Duration::from_millis(10);
        let mut vfx = CursorVfxSystem::new(config);

        vfx.update(point(Pixels::from(0.0), Pixels::from(0.0)), 0.016);
        vfx.update(point(Pixels::from(100.0), Pixels::from(0.0)), 0.016);
        assert!(vfx.is_active());

        std::thread::sleep(Duration::from_millis(20));
        vfx.update(point(Pixels::from(100.0), Pixels::from(0.0)), 0.016);
        assert!(!vfx.is_active());
    }

    #[test]
    fn sonicboom_spawns_on_stop() {
        let mut config = CursorVfxConfig::default();
        config.mode = CursorVfxMode::Sonicboom;
        let mut vfx = CursorVfxSystem::new(config);

        vfx.update(point(Pixels::from(0.0), Pixels::from(0.0)), 0.016);
        vfx.update(point(Pixels::from(100.0), Pixels::from(0.0)), 0.016);
        assert!(!vfx.is_active());

        vfx.update(point(Pixels::from(100.0), Pixels::from(0.0)), 0.016);
        assert!(vfx.is_active());
    }
}
