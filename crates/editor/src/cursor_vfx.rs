use std::time::{Duration, Instant};

use gpui::{Bounds, Hsla, Pixels, Point, Window, point, size};

use crate::editor_settings::CursorVfx;

/// Cursor visual effect mode.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum CursorVfxMode {
    /// No visual effects
    #[default]
    None,
    /// Burst effect at cursor stop point
    Sonicboom,
}

const DEFAULT_OPACITY: f32 = 200.0;
const DEFAULT_HIGHLIGHT_LIFETIME: f32 = 0.2;

#[derive(Debug, Clone)]
pub struct CursorVfxConfig {
    pub mode: CursorVfxMode,
    pub opacity: f32,
    pub highlight_lifetime: Duration,
}

impl Default for CursorVfxConfig {
    fn default() -> Self {
        Self {
            mode: CursorVfxMode::None,
            opacity: DEFAULT_OPACITY,
            highlight_lifetime: Duration::from_secs_f32(DEFAULT_HIGHLIGHT_LIFETIME),
        }
    }
}

impl CursorVfxConfig {
    pub fn is_enabled(&self) -> bool {
        self.mode != CursorVfxMode::None
    }

    pub fn from_runtime_settings(settings: &CursorVfx) -> Self {
        Self {
            mode: settings.mode,
            ..Default::default()
        }
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
        let lifetime_secs = self.lifetime.as_secs_f32();
        if lifetime_secs == 0.0 {
            return 1.0;
        }
        (self.birth.elapsed().as_secs_f32() / lifetime_secs).min(1.0)
    }

    fn is_alive(&self) -> bool {
        self.birth.elapsed() < self.lifetime
    }
}

#[derive(Debug)]
pub struct CursorVfxSystem {
    config: CursorVfxConfig,
    highlights: Vec<HighlightEffect>,
    last_pos: Option<Point<Pixels>>,
    was_moving: bool,
}

impl CursorVfxSystem {
    pub fn new(config: CursorVfxConfig) -> Self {
        Self {
            config,
            highlights: Vec::with_capacity(10),
            last_pos: None,
            was_moving: false,
        }
    }

    pub fn set_config(&mut self, config: CursorVfxConfig) {
        self.config = config;
        if !self.config.is_enabled() {
            self.highlights.clear();
        }
    }

    pub fn is_animating(&self) -> bool {
        !self.highlights.is_empty()
    }

    pub fn update(&mut self, cursor_pos: Point<Pixels>) {
        if !self.config.is_enabled() {
            return;
        }

        self.highlights.retain(|h| h.is_alive());

        if let Some(last) = self.last_pos {
            let dx = f32::from(cursor_pos.x - last.x);
            let dy = f32::from(cursor_pos.y - last.y);
            let distance = (dx * dx + dy * dy).sqrt();

            if distance > 1.0 {
                self.was_moving = true;
            } else if self.was_moving {
                self.was_moving = false;
                self.spawn_stop_effect(cursor_pos);
            }
        }

        self.last_pos = Some(cursor_pos);
    }

    fn spawn_stop_effect(&mut self, pos: Point<Pixels>) {
        if self.config.mode == CursorVfxMode::Sonicboom {
            self.highlights.push(HighlightEffect::new(
                pos,
                self.config.highlight_lifetime,
                self.config.opacity / 255.0,
                self.config.mode,
            ));
        }
    }

    pub fn paint(&self, origin: Point<Pixels>, window: &mut Window, color: Hsla) {
        if !self.config.is_enabled() {
            return;
        }

        for highlight in &self.highlights {
            let t = highlight.normalized_age();
            let opacity = highlight.base_opacity * (1.0 - t);

            if opacity > 0.01 && highlight.mode == CursorVfxMode::Sonicboom {
                let c = Hsla {
                    a: color.a * opacity,
                    ..color
                };
                let radius = t * 30.0;
                self.paint_burst(highlight.center, origin, radius, c, window);
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_vfx_does_nothing() {
        let config = CursorVfxConfig::default();
        let mut vfx = CursorVfxSystem::new(config);

        vfx.update(point(Pixels::from(100.0), Pixels::from(100.0)));
        assert!(!vfx.is_animating());
    }

    #[test]
    fn sonicboom_spawns_on_stop() {
        let mut config = CursorVfxConfig::default();
        config.mode = CursorVfxMode::Sonicboom;
        let mut vfx = CursorVfxSystem::new(config);

        vfx.update(point(Pixels::from(0.0), Pixels::from(0.0)));
        vfx.update(point(Pixels::from(100.0), Pixels::from(0.0)));
        assert!(!vfx.is_animating());

        vfx.update(point(Pixels::from(100.0), Pixels::from(0.0)));
        assert!(vfx.is_animating());
    }

    #[test]
    fn highlights_expire() {
        let mut config = CursorVfxConfig::default();
        config.mode = CursorVfxMode::Sonicboom;
        config.highlight_lifetime = Duration::from_millis(10);
        let mut vfx = CursorVfxSystem::new(config);

        vfx.update(point(Pixels::from(0.0), Pixels::from(0.0)));
        vfx.update(point(Pixels::from(100.0), Pixels::from(0.0)));
        vfx.update(point(Pixels::from(100.0), Pixels::from(0.0)));
        assert!(vfx.is_animating());

        std::thread::sleep(Duration::from_millis(20));
        vfx.update(point(Pixels::from(100.0), Pixels::from(0.0)));
        assert!(!vfx.is_animating());
    }
}
