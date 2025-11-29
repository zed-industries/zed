//! Smooth cursor animation for the Zed editor.
//!
//! This module provides smooth cursor animation similar to VS Code's
//! `editor.cursorSmoothCaretAnimation` feature.
//!
//! Implementation inspired by VS Code PR #51197:
//! https://github.com/microsoft/vscode/pull/51197
//!
//! Related issue: https://github.com/zed-industries/zed/issues/4688

use gpui::{point, Point, Pixels};
use std::time::{Duration, Instant};

/// Represents the animated state of a cursor.
#[derive(Clone, Debug)]
pub struct CursorAnimationState {
    /// The position the cursor is animating from.
    start_position: Point<Pixels>,
    /// The target position the cursor is animating to.
    target_position: Point<Pixels>,
    /// When the animation started.
    animation_start: Instant,
    /// Total duration of the animation.
    animation_duration: Duration,
    /// Whether animation is currently in progress.
    is_animating: bool,
}

impl Default for CursorAnimationState {
    fn default() -> Self {
        Self {
            start_position: point(Pixels::ZERO, Pixels::ZERO),
            target_position: point(Pixels::ZERO, Pixels::ZERO),
            animation_start: Instant::now(),
            animation_duration: Duration::from_millis(80),
            is_animating: false,
        }
    }
}

impl CursorAnimationState {
    /// Creates a new cursor animation state with the specified duration.
    pub fn new(duration_ms: u64) -> Self {
        Self {
            animation_duration: Duration::from_millis(duration_ms),
            ..Default::default()
        }
    }

    /// Updates the animation duration.
    pub fn set_duration(&mut self, duration_ms: u64) {
        self.animation_duration = Duration::from_millis(duration_ms);
    }

    /// Sets a new target position, starting the animation from the current
    /// interpolated position.
    pub fn animate_to(&mut self, new_target: Point<Pixels>) {
        // If target hasn't changed, no animation needed
        if self.target_position == new_target {
            return;
        }

        // Start from current interpolated position
        self.start_position = self.current_position();
        self.target_position = new_target;
        self.animation_start = Instant::now();
        self.is_animating = true;
    }

    /// Immediately sets the cursor position without animation.
    /// Use this for initial positioning or when animation is disabled.
    pub fn set_position_immediately(&mut self, position: Point<Pixels>) {
        self.start_position = position;
        self.target_position = position;
        self.is_animating = false;
    }

    /// Returns the current interpolated cursor position.
    pub fn current_position(&self) -> Point<Pixels> {
        if !self.is_animating {
            return self.target_position;
        }

        let elapsed = self.animation_start.elapsed();

        if elapsed >= self.animation_duration {
            return self.target_position;
        }

        let progress = elapsed.as_secs_f32() / self.animation_duration.as_secs_f32();
        let eased_progress = Self::ease_out_quint(progress);

        point(
            self.lerp(self.start_position.x, self.target_position.x, eased_progress),
            self.lerp(self.start_position.y, self.target_position.y, eased_progress),
        )
    }

    /// Returns true if the animation is still in progress.
    pub fn is_animating(&self) -> bool {
        if !self.is_animating {
            return false;
        }
        self.animation_start.elapsed() < self.animation_duration
    }

    /// Returns the target position (final destination).
    pub fn target_position(&self) -> Point<Pixels> {
        self.target_position
    }

    /// Linear interpolation between two pixel values.
    fn lerp(&self, start: Pixels, end: Pixels, t: f32) -> Pixels {
        Pixels(start.0 + (end.0 - start.0) * t)
    }

    /// Ease-out quint easing function for smooth deceleration.
    /// This provides a natural-feeling cursor movement that starts fast
    /// and gradually slows down as it approaches the target.
    ///
    /// Formula: 1 - (1 - t)^5
    fn ease_out_quint(t: f32) -> f32 {
        1.0 - (1.0 - t).powi(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate_positioning() {
        let mut state = CursorAnimationState::new(80);
        let pos = point(Pixels(100.0), Pixels(200.0));
        state.set_position_immediately(pos);

        assert_eq!(state.current_position(), pos);
        assert!(!state.is_animating());
    }

    #[test]
    fn test_animation_start() {
        let mut state = CursorAnimationState::new(80);
        state.set_position_immediately(point(Pixels(0.0), Pixels(0.0)));

        let target = point(Pixels(100.0), Pixels(100.0));
        state.animate_to(target);

        assert!(state.is_animating());
        assert_eq!(state.target_position(), target);
    }

    #[test]
    fn test_no_animation_for_same_target() {
        let mut state = CursorAnimationState::new(80);
        let pos = point(Pixels(100.0), Pixels(100.0));
        state.set_position_immediately(pos);

        // Animating to the same position should not start animation
        state.animate_to(pos);
        assert!(!state.is_animating());
    }

    #[test]
    fn test_ease_out_quint() {
        // At t=0, should be 0
        assert!((CursorAnimationState::ease_out_quint(0.0) - 0.0).abs() < 0.001);
        // At t=1, should be 1
        assert!((CursorAnimationState::ease_out_quint(1.0) - 1.0).abs() < 0.001);
        // At t=0.5, should be > 0.5 (ease-out accelerates early)
        assert!(CursorAnimationState::ease_out_quint(0.5) > 0.5);
    }
}
