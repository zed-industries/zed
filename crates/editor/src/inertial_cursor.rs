// crates/editor/src/inertial_cursor.rs
//
// Inertial smooth cursor animation for Zed.
// This module provides animation state management for cursor position interpolation.

use std::time::{Duration, Instant};

use gpui::{Pixels, Point, point};
use settings::SmoothCaretMode;

/// Maximum animation step to prevent large jumps (~8.3ms at 120Hz)
/// This ensures smooth animation even when frames are skipped.
pub const MAX_ANIMATION_DT: f32 = 1.0 / 120.0;

/// Distance threshold (in pixels) above which cursor snaps immediately.
/// Prevents excessive stretching during very large jumps (e.g., search, goto).
const LARGE_JUMP_SNAP_THRESHOLD: f32 = 1000.0;

/// Minimum animation time for corner animation.
/// Allows near-instant snap for leading corners with high trail_size.
const MIN_CORNER_ANIMATION_TIME: f32 = 0.001;

/// Convergence threshold for spring animation.
/// Only check position for convergence, not velocity.
const SPRING_CONVERGENCE_THRESHOLD: f32 = 0.01;

// ============================================================================
// Cursor Animation Ticker (Frame Pacing)
// ============================================================================

/// Centralized frame pacing for cursor animations.
/// This handles the catchup mechanism to ensure smooth animation
/// even when frame intervals are uneven.
///
/// Usage:
/// 1. Call `tick(now)` at the start of each frame to get the smoothed dt
/// 2. Pass that dt to cursor's `update_physics(dt)`
/// 3. Call `stop()` when animation finishes
#[derive(Debug, Clone)]
pub struct CursorAnimationTicker {
    /// When the current animation period started.
    animation_start: Instant,
    /// How much animation time has been simulated (virtual time).
    animation_time: Duration,
    /// Expected frame duration based on display refresh rate (~8.3ms for 120Hz).
    expected_frame_duration: Duration,
    /// Whether we're currently in an animation period.
    is_animating: bool,
}

impl Default for CursorAnimationTicker {
    fn default() -> Self {
        Self::new()
    }
}

impl CursorAnimationTicker {
    /// Default expected frame duration (120Hz).
    const DEFAULT_FRAME_DURATION: Duration = Duration::from_micros(8333);

    pub fn new() -> Self {
        Self {
            animation_start: Instant::now(),
            animation_time: Duration::ZERO,
            expected_frame_duration: Self::DEFAULT_FRAME_DURATION,
            is_animating: false,
        }
    }

    /// Set the expected frame duration from refresh rate in Hz.
    pub fn set_refresh_rate(&mut self, hz: f32) {
        if hz > 0.0 {
            self.expected_frame_duration = Duration::from_secs_f32(1.0 / hz);
        }
    }

    /// The dt is smoothed using a catchup mechanism:
    /// - If behind by 1+ frames, catch up immediately
    /// - If behind by less, spread catchup over 10 frames
    pub fn tick(&mut self, now: Instant) -> Duration {
        if !self.is_animating {
            self.animation_start = now;
            self.animation_time = Duration::ZERO;
            self.is_animating = true;
        }

        // Calculate target animation time (wall clock since animation start)
        let target_time = now.saturating_duration_since(self.animation_start);
        let mut delta = target_time.saturating_sub(self.animation_time);

        // Protection against huge jumps (e.g., switching tabs, laptop sleep)
        if delta > Duration::from_millis(500) {
            self.animation_start = now;
            self.animation_time = Duration::ZERO;
            delta = self.expected_frame_duration;
        }

        // Smooth catchup mechanism for consistent frame pacing.
        // If behind by 1+ frames, catch up immediately.
        // If less, spread catchup over 10 frames for smoothness.
        let catchup = if delta >= self.expected_frame_duration {
            delta
        } else {
            Duration::from_secs_f64(delta.as_secs_f64() / 10.0)
        };

        let dt = self.expected_frame_duration + catchup;
        self.animation_time += dt;

        dt
    }

    pub fn stop(&mut self) {
        self.is_animating = false;
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }
}

// ============================================================================
// Critically Damped Spring Animation
// ============================================================================

/// Single-axis critically damped spring animation.
/// This is the core building block for smooth cursor movement.
/// Based on GDC 2021 Math In Game Development Summit.
#[derive(Debug, Clone, Copy)]
pub struct SpringAnimation {
    /// Current offset from target (converges to 0)
    pub position: f32,
    /// Current velocity
    pub velocity: f32,
}

impl Default for SpringAnimation {
    fn default() -> Self {
        Self {
            position: 0.0,
            velocity: 0.0,
        }
    }
}

impl SpringAnimation {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the spring animation. Returns true if still animating.
    /// Uses critically damped spring formula (zeta = 1.0).
    /// Checks convergence only at the end, only checking position (not velocity).
    pub fn update(&mut self, dt: f32, animation_length: f32) -> bool {
        // If animation_length <= dt, reset immediately
        if animation_length <= dt {
            self.reset();
            return false;
        }

        if self.position == 0.0 {
            return false;
        }

        // omega = 4.0 / animation_length gives ~98% convergence in animation_length time
        let omega = 4.0 / animation_length;

        // Analytical solution for critically damped oscillator:
        // x(t) = (a + b*t) * e^(-omega*t)
        // where a = initial_offset, b = initial_velocity + a*omega
        let a = self.position;
        let b = a * omega + self.velocity;
        let c = (-omega * dt).exp();

        self.position = (a + b * dt) * c;
        self.velocity = c * (-a * omega - b * dt * omega + b);

        if self.position.abs() < SPRING_CONVERGENCE_THRESHOLD {
            self.reset();
            false
        } else {
            true
        }
    }

    pub fn reset(&mut self) {
        self.position = 0.0;
        self.velocity = 0.0;
    }

    pub fn is_complete(&self) -> bool {
        self.position.abs() < SPRING_CONVERGENCE_THRESHOLD
    }
}

/// Configuration for inertial cursor animation.
/// Derived from SmoothCaretMode presets or custom settings.
/// Uses QuadCorner animation (4 independent corners).
#[derive(Debug, Clone, Copy)]
pub struct InertialCursorConfig {
    pub enabled: bool,
    /// Animation duration for large jumps (in seconds).
    pub animation_time: Duration,
    /// Animation duration for small moves/typing (in seconds).
    pub short_animation_time: Duration,
    /// Trail effect size (0.0-1.0):
    /// - 1.0 = Leading edge snaps instantly (minimum latency, maximum responsiveness)
    /// - 0.5 = Balanced animation
    /// - 0.0 = Full smooth animation (maximum smoothness, more perceived latency)
    pub trail_size: f32,
    /// Whether to animate cursor during insert mode (typing).
    /// When false, cursor snaps instantly for short horizontal movements.
    pub animate_in_insert_mode: bool,
}

impl InertialCursorConfig {
    /// Create configuration from a SmoothCaretMode preset.
    /// All presets use QuadCorner animation with critically damped springs.
    pub fn from_mode(mode: SmoothCaretMode) -> Self {
        match mode {
            SmoothCaretMode::Off => Self {
                enabled: false,
                animation_time: Duration::from_millis(0),
                short_animation_time: Duration::from_millis(0),
                trail_size: 0.0,
                animate_in_insert_mode: true,
            },
            SmoothCaretMode::On => Self {
                enabled: true,
                animation_time: Duration::from_millis(150),
                short_animation_time: Duration::from_millis(25),
                trail_size: 1.0,
                animate_in_insert_mode: true,
            },
        }
    }
}

impl Default for InertialCursorConfig {
    fn default() -> Self {
        Self::from_mode(SmoothCaretMode::On)
    }
}

/// 2D vector for internal animation calculations.
/// Uses f32 for arithmetic, converts to/from gpui::Point<Pixels> at boundaries.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline]
    pub fn from_point(p: Point<Pixels>) -> Self {
        Self {
            x: f32::from(p.x),
            y: f32::from(p.y),
        }
    }

    #[inline]
    pub fn to_point(self) -> Point<Pixels> {
        point(Pixels::from(self.x), Pixels::from(self.y))
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    #[inline]
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    #[inline]
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn div(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

// ============================================================================
// Four-Corner Animation
// ============================================================================

/// Single corner of the cursor quad with independent X/Y spring animation.
/// Each corner animates at different speeds based on movement direction.
/// Uses independent X/Y springs for more natural diagonal movement.
///
/// Architecture: `corner_destination` is recalculated EVERY frame with
/// current `cursor_dimensions` and `center`. This ensures smooth animation even
/// when cell size changes during animation.
#[derive(Debug, Clone)]
pub struct AnimatedCorner {
    /// Independent X-axis spring animation
    animation_x: SpringAnimation,
    /// Independent Y-axis spring animation
    animation_y: SpringAnimation,
    /// Current position (computed from destination - spring offset)
    current_position: Vec2,
    /// Relative offset from cursor center (-0.5 to 0.5)
    relative_position: Vec2,
    /// Previous destination position for detecting changes (exact comparison)
    previous_destination: Vec2,
    /// Animation length for this corner (in seconds).
    /// Set once per jump, stays constant throughout the animation.
    /// animation_length is absolute, not a multiplier.
    animation_length: f32,
    /// Whether this is a short movement (typing) - used to reset velocity
    is_short_movement: bool,
}

impl AnimatedCorner {
    pub fn new(relative_x: f32, relative_y: f32) -> Self {
        Self {
            animation_x: SpringAnimation::new(),
            animation_y: SpringAnimation::new(),
            current_position: Vec2::ZERO,
            relative_position: Vec2::new(relative_x, relative_y),
            previous_destination: Vec2::new(-1000.0, -1000.0), // Impossible initial value for change detection
            animation_length: 0.1, // Default 100ms, will be set properly on first jump
            is_short_movement: false,
        }
    }

    /// Calculate direction alignment for ranking.
    /// Returns dot product of corner offset from center and movement direction.
    pub fn calculate_alignment(&self, top_left: Vec2, cell_width: f32, cell_height: f32) -> f32 {
        let corner_dest = Vec2::new(
            top_left.x + self.relative_position.x * cell_width,
            top_left.y + self.relative_position.y * cell_height,
        );
        let move_delta = corner_dest - self.current_position;
        let distance = move_delta.length();

        if distance > 0.001 {
            let direction = Vec2::new(move_delta.x / distance, move_delta.y / distance);
            // Use offset from center (0.5, 0.5) for corner direction
            // This gives: (-0.5,-0.5), (0.5,-0.5), (0.5,0.5), (-0.5,0.5)
            let offset_from_center = Vec2::new(
                self.relative_position.x - 0.5,
                self.relative_position.y - 0.5,
            );
            let rel_len = offset_from_center.length();
            if rel_len > 0.001 {
                let rel_norm = Vec2::new(
                    offset_from_center.x / rel_len,
                    offset_from_center.y / rel_len,
                );
                rel_norm.x * direction.x + rel_norm.y * direction.y
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Set animation length for this corner.
    /// Called once per position change. animation_length stays constant
    /// for all frames of this animation.
    /// `is_short` indicates a short movement (typing) where velocity should be reset.
    pub fn jump(&mut self, animation_length: f32, is_short: bool) {
        self.animation_length = animation_length.max(MIN_CORNER_ANIMATION_TIME);
        self.is_short_movement = is_short;
    }

    /// Update corner animation with current parameters.
    /// CRITICAL: cursor_dimensions and center are passed EVERY frame.
    /// This ensures correct animation even when cell size changes.
    ///
    /// Returns true if still animating.
    pub fn update(
        &mut self,
        cursor_dimensions: (f32, f32), // (cell_width, cell_height)
        center: Vec2,
        dt: f32,
        immediate_movement: bool,
    ) -> bool {
        // Calculate corner destination with CURRENT dimensions
        let corner_destination = Vec2::new(
            center.x + self.relative_position.x * cursor_dimensions.0,
            center.y + self.relative_position.y * cursor_dimensions.1,
        );

        // Exact comparison to detect position change
        if corner_destination != self.previous_destination {
            // Initialize spring with delta from current position to new destination
            let delta = corner_destination - self.current_position;
            self.animation_x.position = delta.x;
            self.animation_y.position = delta.y;

            // For short movements (typing), reset velocity to prevent accumulation
            // and micro-jitter during rapid keystrokes. For navigation, preserve
            // velocity for smooth chaining.
            if self.is_short_movement {
                self.animation_x.velocity = 0.0;
                self.animation_y.velocity = 0.0;
            }

            self.previous_destination = corner_destination;
        }

        if immediate_movement {
            self.current_position = corner_destination;
            self.animation_x.reset();
            self.animation_y.reset();
            return false;
        }

        // Update springs with stored animation_length (constant for this animation)
        let mut animating = self.animation_x.update(dt, self.animation_length);
        animating |= self.animation_y.update(dt, self.animation_length);

        self.current_position.x = corner_destination.x - self.animation_x.position;
        self.current_position.y = corner_destination.y - self.animation_y.position;

        animating
    }

    pub fn position(&self) -> Vec2 {
        self.current_position
    }

    pub fn is_complete(&self) -> bool {
        self.animation_x.is_complete() && self.animation_y.is_complete()
    }

    pub fn init_position(&mut self, center: Vec2, cell_width: f32, cell_height: f32) {
        let pos = Vec2::new(
            center.x + self.relative_position.x * cell_width,
            center.y + self.relative_position.y * cell_height,
        );
        self.current_position = pos;
        self.previous_destination = pos;
        self.animation_x.reset();
        self.animation_y.reset();
    }
}

/// Four-corner cursor for parallelogram deformation.
/// Each corner animates independently based on movement direction.
/// Architecture: corner positions recalculated every frame with
/// current cell dimensions.
#[derive(Debug, Clone)]
pub struct QuadCursor {
    /// The 4 corners of the cursor quad (each with independent X/Y springs)
    corners: [AnimatedCorner; 4],
    /// Target position (center of cursor)
    logical_center: Vec2,
    /// Animation configuration
    config: InertialCursorConfig,
    /// Last frame's delta time (for VFX)
    last_frame_dt: f32,
    /// Whether cursor jumped this frame (triggers animation_length calculation)
    jumped: bool,
    /// Current cell dimensions (set immediately, no animation)
    cell_width: f32,
    cell_height: f32,
}

impl QuadCursor {
    pub fn new(
        config: InertialCursorConfig,
        initial_pos: Point<Pixels>,
        cell_width: f32,
        cell_height: f32,
    ) -> Self {
        let top_left = Vec2::from_point(initial_pos);
        // Corner offsets as fractions of cell size (top-left based: 0,0 to 1,1)
        // This matches how Zed passes cursor position (top-left of cell)
        // Order: Top-left, Top-right, Bottom-right, Bottom-left
        let mut corners = [
            AnimatedCorner::new(0.0, 0.0),
            AnimatedCorner::new(1.0, 0.0),
            AnimatedCorner::new(1.0, 1.0),
            AnimatedCorner::new(0.0, 1.0),
        ];

        for corner in &mut corners {
            corner.init_position(top_left, cell_width, cell_height);
        }

        Self {
            corners,
            logical_center: top_left,
            config,
            last_frame_dt: 1.0 / 120.0,
            jumped: false,
            cell_width,
            cell_height,
        }
    }

    pub fn set_config(&mut self, config: InertialCursorConfig) {
        self.config = config;
        if !config.enabled {
            self.snap_to_logical();
        }
    }

    /// Set cell dimensions (immediate, no animation).
    /// Corner positions are recalculated every frame with current dimensions,
    /// so cell size changes are handled smoothly by the corner springs.
    pub fn set_cell_size(&mut self, width: f32, height: f32) {
        self.cell_width = width;
        self.cell_height = height;
    }

    /// Set the logical (target) center position.
    /// Only sets animation_length via jump(). Actual animation happens in update_physics().
    pub fn set_logical_pos(&mut self, pos: Point<Pixels>) {
        let old_center = self.logical_center;
        self.logical_center = Vec2::from_point(pos);

        if !self.config.enabled {
            self.snap_to_logical();
            return;
        }

        // Detect movement distance
        let move_vec = self.logical_center - old_center;
        let move_distance = move_vec.length();

        // Skip if position hasn't changed
        if move_distance < 0.001 {
            return;
        }

        // Snap immediately for very large jumps to prevent excessive stretching
        if move_distance > LARGE_JUMP_SNAP_THRESHOLD {
            self.snap_to_logical();
            return;
        }

        self.jumped = true;

        // Short jump detection: horizontal movement <= 2 chars AND vertical ~= 0
        let is_short_animation = if self.cell_width > 0.001 && self.cell_height > 0.001 {
            let chars_x = (move_vec.x / self.cell_width).abs();
            let chars_y = (move_vec.y / self.cell_height).abs();
            // Use 0.1 threshold (10% of cell height) for sub-pixel tolerance
            chars_x <= 2.001 && chars_y < 0.1
        } else {
            false
        };

        // Skip animation for short movements if animate_in_insert_mode is disabled
        if !self.config.animate_in_insert_mode && is_short_animation {
            self.snap_to_logical();
            self.jumped = false;
            return;
        }

        // Calculate animation_length for each corner
        if is_short_animation {
            // Short jump: uniform animation_length for ALL corners (straight line)
            let anim_length = self.config.short_animation_time.as_secs_f32().max(0.001);
            for corner in &mut self.corners {
                corner.jump(anim_length, true);
            }
        } else {
            // Long jump: rank-based trailing effect
            // 1. Calculate alignment for each corner
            let alignments: [(usize, f32); 4] = [
                (
                    0,
                    self.corners[0].calculate_alignment(
                        self.logical_center,
                        self.cell_width,
                        self.cell_height,
                    ),
                ),
                (
                    1,
                    self.corners[1].calculate_alignment(
                        self.logical_center,
                        self.cell_width,
                        self.cell_height,
                    ),
                ),
                (
                    2,
                    self.corners[2].calculate_alignment(
                        self.logical_center,
                        self.cell_width,
                        self.cell_height,
                    ),
                ),
                (
                    3,
                    self.corners[3].calculate_alignment(
                        self.logical_center,
                        self.cell_width,
                        self.cell_height,
                    ),
                ),
            ];

            // 2. Sort by alignment to assign ranks (0 = most trailing, 3 = most leading)
            let mut sorted: [(usize, f32); 4] = alignments;
            sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            // 3. Build rank lookup: ranks[corner_index] = rank (0-3)
            let mut ranks = [0usize; 4];
            for (rank, (idx, _)) in sorted.iter().enumerate() {
                ranks[*idx] = rank;
            }

            // 4. Calculate animation_length based on rank (triangular trail)
            let base_time = self.config.animation_time.as_secs_f32().max(0.001);
            let trail_size = self.config.trail_size;
            let leading_time = base_time * (1.0 - trail_size).clamp(0.0, 1.0);
            let trailing_time = base_time;
            let middle_time = (leading_time + trailing_time) / 2.0;

            for (i, corner) in self.corners.iter_mut().enumerate() {
                // Rank 2-3 both get leading speed
                let anim_length = match ranks[i] {
                    0 => trailing_time,    // Most trailing - slowest
                    1 => middle_time,      // Middle corner - medium
                    2..=3 => leading_time, // Leading corners - fastest
                    _ => base_time,
                };
                corner.jump(anim_length, false);
            }
        }
    }

    pub fn logical_pos(&self) -> Point<Pixels> {
        self.logical_center.to_point()
    }

    pub fn corner_positions(&self) -> [Point<Pixels>; 4] {
        [
            self.corners[0].position().to_point(),
            self.corners[1].position().to_point(),
            self.corners[2].position().to_point(),
            self.corners[3].position().to_point(),
        ]
    }

    pub fn visual_pos(&self) -> Point<Pixels> {
        self.corners[0].position().to_point()
    }

    pub fn last_frame_dt(&self) -> f32 {
        self.last_frame_dt
    }

    pub fn snap_to_logical(&mut self) {
        for corner in &mut self.corners {
            corner.init_position(self.logical_center, self.cell_width, self.cell_height);
        }
    }

    pub fn is_animating(&self) -> bool {
        if !self.config.enabled {
            return false;
        }
        // Include jumped flag: animation starts when set_logical_pos() is called,
        // even before update_physics() processes it
        self.jumped || self.corners.iter().any(|c| !c.is_complete())
    }

    /// Update physics with a given delta time.
    /// This is the core animation logic called by tick_cursor_animations().
    /// Returns `true` if visual position changed.
    ///
    /// CRITICAL architecture:
    /// - corner.update() receives CURRENT (cell_width, cell_height) and center
    /// - corner_destination is recalculated INSIDE update() with fresh values
    /// - This ensures smooth animation even when cell size changes
    pub fn update_physics(&mut self, dt: f32) -> bool {
        let mut changed = false;

        // Pass current dimensions to each corner's update()
        // Each corner recalculates its destination with fresh values
        let cursor_dimensions = (self.cell_width, self.cell_height);
        let center = self.logical_center;
        let immediate_movement = false; // Only true for animate_in_insert_mode=false

        for corner in &mut self.corners {
            if corner.update(cursor_dimensions, center, dt, immediate_movement) {
                changed = true;
            }
        }

        self.jumped = false;

        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_cursor_disabled_snaps_immediately() {
        let mut cursor = QuadCursor::new(
            InertialCursorConfig::from_mode(SmoothCaretMode::Off),
            point(Pixels::from(0.0), Pixels::from(0.0)),
            10.0,
            20.0,
        );

        cursor.set_logical_pos(point(Pixels::from(100.0), Pixels::from(0.0)));
        let changed = cursor.update_physics(0.016); // 16ms delta time

        let visual = cursor.visual_pos();
        let logical = cursor.logical_pos();
        assert_eq!(f32::from(visual.x), f32::from(logical.x));
        assert!(!changed, "Disabled cursor shouldn't animate");
    }

    #[test]
    fn quad_cursor_converges_to_target() {
        let mut cursor = QuadCursor::new(
            InertialCursorConfig::from_mode(SmoothCaretMode::On),
            point(Pixels::from(0.0), Pixels::from(0.0)),
            10.0,
            20.0,
        );

        cursor.set_logical_pos(point(Pixels::from(100.0), Pixels::from(0.0)));

        // Simulate ~2 seconds of animation at 60fps
        for _ in 0..120 {
            cursor.update_physics(0.016); // 16ms delta time
        }

        let visual = cursor.visual_pos();
        let x = f32::from(visual.x);
        assert!(
            (x - 100.0).abs() < 1.0,
            "Expected cursor to be close to target, got x = {}",
            x
        );
    }

    #[test]
    fn quad_cursor_is_animating_reflects_state() {
        let mut cursor = QuadCursor::new(
            InertialCursorConfig::from_mode(SmoothCaretMode::On),
            point(Pixels::from(0.0), Pixels::from(0.0)),
            10.0,
            20.0,
        );

        // Initially at rest
        assert!(!cursor.is_animating());

        // Move to new position
        cursor.set_logical_pos(point(Pixels::from(100.0), Pixels::from(0.0)));
        assert!(cursor.is_animating());

        // After convergence
        for _ in 0..200 {
            cursor.update_physics(0.016); // 16ms delta time
        }
        assert!(!cursor.is_animating());
    }
}
