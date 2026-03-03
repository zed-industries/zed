use gpui::{Pixels, Point};

/// Critically-damped harmonic oscillator for smooth cursor draw-origin interpolation.
///
/// Uses the closed-form solution so there are no iterative sub-steps and the result
/// is unconditionally stable for any positive `omega` and any `dt` up to 50 ms.
///
/// # Spring tuning
/// `omega` is the natural angular frequency in rad/s.
///
/// * 20 to 25  -> silky, slightly laggy (notebook feel)
/// * 30 to 40  -> snappy, similar to the macOS text cursor
/// * 50+       -> almost instant; mainly useful for testing
///
/// # Teleport threshold
/// Jumps larger than `teleport_distance` pixels snap the cursor instantly.
/// This covers Ctrl+G and jump-to-definition without visible smear across the screen.
#[derive(Debug, Clone, PartialEq)]
pub struct SmoothCursor {
    pub current: Point<Pixels>,
    pub target: Point<Pixels>,
    velocity: Point<Pixels>,
    omega: f32,
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

    /// Move the target. Teleports if the distance exceeds the configured threshold.
    #[inline]
    pub fn set_target(&mut self, new_target: Point<Pixels>) {
        let dx = (self.current.x - new_target.x).0;
        let dy = (self.current.y - new_target.y).0;
        if dx * dx + dy * dy > self.teleport_threshold_sq {
            self.teleport_to(new_target);
            return;
        }
        self.target = new_target;
    }

    /// Snap the current position to `pos` and zero out velocity.
    #[inline]
    pub fn teleport_to(&mut self, pos: Point<Pixels>) {
        self.current = pos;
        self.target = pos;
        self.velocity = Point::default();
    }

    /// Advance the spring simulation by `dt` seconds.
    ///
    /// Safe to call with `dt == 0`. Clamps at 50 ms to absorb stalls from tab switches.
    pub fn tick(&mut self, dt: f32) {
        if dt <= 0.0 {
            return;
        }
        let dt = dt.min(0.05);
        let w = self.omega;
        if w <= 0.0 {
            // Zero stiffness means instant snap.
            self.current = self.target;
            self.velocity = Point::default();
            return;
        }

        // Closed-form critically-damped spring (zeta = 1):
        //   x(t) = x_target + (x0 + (v0 + w*x0)*t) * e^(-w*t)
        //   v(t) =            (v0 - (v0 + w*x0)*w*t) * e^(-w*t)
        // where x0 = current - target, v0 = velocity.
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

    /// Returns true once position and velocity are both below the perceptible threshold.
    ///
    /// Both thresholds are 1e-4 px^2, which is well below a single sub-pixel.
    /// The cursor renders identically to its target before this returns true.
    #[inline]
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
        Point {
            x: Pixels(x),
            y: Pixels(y),
        }
    }

    #[test]
    fn settles_at_target() {
        let mut sc = SmoothCursor::new(pt(0.0, 0.0), 30.0, 500.0);
        sc.set_target(pt(100.0, 50.0));
        for _ in 0..120 {
            sc.tick(1.0 / 60.0);
        }
        assert!(sc.is_settled(), "did not settle: {sc:?}");
        assert!((sc.current.x.0 - 100.0).abs() < 0.01);
        assert!((sc.current.y.0 - 50.0).abs() < 0.01);
    }

    #[test]
    fn teleports_on_large_jump() {
        let mut sc = SmoothCursor::new(pt(0.0, 0.0), 30.0, 300.0);
        sc.set_target(pt(400.0, 0.0)); // 400 px > 300 px threshold
        assert_eq!(sc.current, pt(400.0, 0.0));
        assert!(sc.is_settled());
    }

    #[test]
    fn no_motion_when_already_at_target() {
        let mut sc = SmoothCursor::new(pt(50.0, 50.0), 30.0, 300.0);
        sc.tick(1.0 / 60.0);
        assert!(sc.is_settled());
    }

    #[test]
    fn zero_omega_snaps_immediately() {
        let mut sc = SmoothCursor::new(pt(0.0, 0.0), 0.0, 300.0);
        sc.set_target(pt(10.0, 10.0));
        sc.tick(1.0 / 60.0);
        assert_eq!(sc.current, pt(10.0, 10.0));
    }

    #[test]
    fn large_dt_does_not_produce_nan() {
        let mut sc = SmoothCursor::new(pt(0.0, 0.0), 30.0, 999.0);
        sc.set_target(pt(100.0, 0.0));
        // Simulate a multi-second stall; should not explode.
        sc.tick(5.0);
        assert!(sc.current.x.0.is_finite());
        assert!(sc.current.y.0.is_finite());
    }
}
