use gpui::{Pixels, Point};

/// Critically damped spring for interpolating cursor draw position.
///
/// Uses the closed-form solution so behavior is stable without sub-stepping,
/// regardless of frame time variance. `dt` is clamped so a long stall (e.g.
/// returning to a tab after sleep) doesn't produce a visible position jump.
/// Jumps beyond `teleport_distance` snap immediately; this covers commands
/// like go-to-definition where animating across the viewport would be disorienting.
#[derive(Debug, Clone, PartialEq)]
pub struct SmoothCursor {
    pub current: Point<Pixels>,
    pub target: Point<Pixels>,
    velocity: Point<Pixels>,
    /// Natural angular frequency in rad/s. Controls how quickly the cursor
    /// tracks its target.
    omega: f32,
    /// Squared distance above which we teleport instead of animating.
    teleport_threshold_sq: f32,
}

impl SmoothCursor {
    pub fn new(initial: Point<Pixels>, omega: f32, teleport_distance: f32) -> Self {
        debug_assert!(omega.is_finite() && omega >= 0.0);
        debug_assert!(teleport_distance.is_finite() && teleport_distance >= 0.0);
        Self {
            current: initial,
            target: initial,
            velocity: Point::default(),
            omega,
            teleport_threshold_sq: teleport_distance * teleport_distance,
        }
    }

    pub fn set_target(&mut self, new_target: Point<Pixels>) {
        let dx = (self.current.x - new_target.x).0;
        let dy = (self.current.y - new_target.y).0;
        if dx * dx + dy * dy > self.teleport_threshold_sq {
            self.teleport_to(new_target);
            return;
        }
        self.target = new_target;
    }

    pub fn teleport_to(&mut self, pos: Point<Pixels>) {
        self.current = pos;
        self.target = pos;
        self.velocity = Point::default();
    }

    pub fn tick(&mut self, dt: f32) {
        if dt <= 0.0 {
            return;
        }
        // Cap dt so a long freeze doesn't appear as a sudden position snap.
        let dt = dt.min(0.05);
        let w = self.omega;
        if w <= 0.0 {
            self.current = self.target;
            self.velocity = Point::default();
            return;
        }

        // Closed-form critically-damped spring (zeta = 1).
        // Derived from the general solution to x'' + 2*w*x' + w^2*x = 0
        // with repeated root r = -w. Stays accurate at any dt without iteration.
        let e = (-w * dt).exp();

        let dx = (self.current.x - self.target.x).0;
        let vx = self.velocity.x.0;
        let cx = vx + dx * w;
        self.current.x = self.target.x + Pixels((dx + cx * dt) * e);
        self.velocity.x = Pixels((vx - cx * (w * dt)) * e);

        let dy = (self.current.y - self.target.y).0;
        let vy = self.velocity.y.0;
        let cy = vy + dy * w;
        self.current.y = self.target.y + Pixels((dy + cy * dt) * e);
        self.velocity.y = Pixels((vy - cy * (w * dt)) * e);
    }

    /// Both thresholds are sub-pixel, so the cursor renders identically to
    /// its target position before this returns true.
    pub fn is_settled(&self) -> bool {
        let dx = (self.current.x - self.target.x).0;
        let dy = (self.current.y - self.target.y).0;
        let vx = self.velocity.x.0;
        let vy = self.velocity.y.0;
        dx * dx + dy * dy < 1e-4 && vx * vx + vy * vy < 1e-4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pt(x: f32, y: f32) -> Point<Pixels> {
        Point { x: Pixels(x), y: Pixels(y) }
    }

    #[test]
    fn settles_at_target() {
        let mut sc = SmoothCursor::new(pt(0.0, 0.0), 30.0, 500.0);
        sc.set_target(pt(100.0, 50.0));
        for _ in 0..120 {
            sc.tick(1.0 / 60.0);
        }
        assert!(sc.is_settled(), "{sc:?}");
        assert!((sc.current.x.0 - 100.0).abs() < 0.01);
        assert!((sc.current.y.0 - 50.0).abs() < 0.01);
    }

    #[test]
    fn teleports_on_large_jump() {
        let mut sc = SmoothCursor::new(pt(0.0, 0.0), 30.0, 300.0);
        sc.set_target(pt(400.0, 0.0));
        assert_eq!(sc.current, pt(400.0, 0.0));
        assert!(sc.is_settled());
    }

    #[test]
    fn no_motion_when_at_target() {
        let mut sc = SmoothCursor::new(pt(50.0, 50.0), 30.0, 300.0);
        sc.tick(1.0 / 60.0);
        assert!(sc.is_settled());
    }

    #[test]
    fn zero_omega_snaps() {
        let mut sc = SmoothCursor::new(pt(0.0, 0.0), 0.0, 300.0);
        sc.set_target(pt(10.0, 10.0));
        sc.tick(1.0 / 60.0);
        assert_eq!(sc.current, pt(10.0, 10.0));
    }

    #[test]
    fn large_dt_does_not_nan() {
        let mut sc = SmoothCursor::new(pt(0.0, 0.0), 30.0, 999.0);
        sc.set_target(pt(100.0, 0.0));
        sc.tick(5.0);
        assert!(sc.current.x.0.is_finite());
        assert!(sc.current.y.0.is_finite());
    }
}
