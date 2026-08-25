// Cursor animation physics adapted from vscode-neovide-cursor:
// https://github.com/LengineerC/vscode-neovide-cursor
//
// Copyright (c) 2025 LengineerC
// Licensed under the MIT License:
// https://github.com/LengineerC/vscode-neovide-cursor/blob/main/LICENSE

use collections::HashMap;
use gpui::{Bounds, Pixels, Point, point, px};
use std::time::{Duration, Instant};

const MAX_FRAME_DURATION: Duration = Duration::from_millis(33);
const SPRING_RESET_EPSILON: f32 = 0.001;
const SPRING_ACTIVE_EPSILON: f32 = 0.01;
const CORNER_ACTIVE_DISTANCE: f32 = 0.5;
const GEOMETRY_EPSILON: f32 = 0.01;
const SHORT_MOVE_THRESHOLD: f32 = 8.0;
const LEADING_SNAP_THRESHOLD: f32 = 0.5;
const SNAP_ANIMATION_LENGTH_SECONDS: f32 = 0.02;
const ANIMATION_RESET_THRESHOLD_SECONDS: f32 = 0.075;
const MAX_TRAIL_DISTANCE_FACTOR: f32 = 100.0;
const RANK_TRAIL_FACTORS: [f32; 4] = [1.0, 0.9, 0.5, 0.3];
const ANIMATION_LENGTH_SECONDS: f32 = 0.125;
const SHORT_ANIMATION_LENGTH_SECONDS: f32 = 0.05;
const TRAIL_SIZE: f32 = 1.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LogicalCursorPosition {
    pub row: u32,
    pub column: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CursorViewport {
    content_origin: Point<Pixels>,
    text_bounds: Bounds<Pixels>,
    scroll_position: Point<f64>,
    scroll_pixel_position: Point<f64>,
    line_height: Pixels,
    em_advance: Pixels,
}

impl CursorViewport {
    pub(crate) fn new(
        content_origin: Point<Pixels>,
        text_bounds: Bounds<Pixels>,
        scroll_position: Point<f64>,
        scroll_pixel_position: Point<f64>,
        line_height: Pixels,
        em_advance: Pixels,
    ) -> Self {
        Self {
            content_origin,
            text_bounds,
            scroll_position,
            scroll_pixel_position,
            line_height,
            em_advance,
        }
    }

    fn is_finite(self) -> bool {
        f32::from(self.content_origin.x).is_finite()
            && f32::from(self.content_origin.y).is_finite()
            && f32::from(self.text_bounds.origin.x).is_finite()
            && f32::from(self.text_bounds.origin.y).is_finite()
            && f32::from(self.text_bounds.size.width).is_finite()
            && f32::from(self.text_bounds.size.height).is_finite()
            && self.scroll_position.x.is_finite()
            && self.scroll_position.y.is_finite()
            && self.scroll_pixel_position.x.is_finite()
            && self.scroll_pixel_position.y.is_finite()
            && f32::from(self.line_height).is_finite()
            && f32::from(self.em_advance).is_finite()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct AnimationPoint {
    x: f32,
    y: f32,
}

impl AnimationPoint {
    fn length(self) -> f32 {
        self.x.hypot(self.y)
    }

    fn normalized(self) -> Self {
        let length = self.length();
        if length == 0.0 || !length.is_finite() {
            return Self::default();
        }

        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl std::ops::Add for AnimationPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for AnimationPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CursorGeometry {
    origin: AnimationPoint,
    width: f32,
    height: f32,
}

impl CursorGeometry {
    fn from_bounds(bounds: Bounds<Pixels>) -> Self {
        Self {
            origin: AnimationPoint {
                x: bounds.origin.x.into(),
                y: bounds.origin.y.into(),
            },
            width: bounds.size.width.into(),
            height: bounds.size.height.into(),
        }
    }

    fn center(self) -> AnimationPoint {
        AnimationPoint {
            x: self.origin.x + self.width / 2.0,
            y: self.origin.y + self.height / 2.0,
        }
    }

    fn is_finite(self) -> bool {
        self.origin.is_finite()
            && self.width.is_finite()
            && self.height.is_finite()
            && self.width > 0.0
            && self.height > 0.0
    }

    fn has_same_size(self, other: Self) -> bool {
        nearly_equal(self.width, other.width) && nearly_equal(self.height, other.height)
    }

    fn has_same_origin(self, other: Self) -> bool {
        nearly_equal(self.origin.x, other.origin.x) && nearly_equal(self.origin.y, other.origin.y)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct DampedSpringAnimation {
    position: f32,
    velocity: f32,
}

impl DampedSpringAnimation {
    fn update(&mut self, elapsed_seconds: f32, animation_length: f32) -> bool {
        if !elapsed_seconds.is_finite()
            || elapsed_seconds < 0.0
            || !animation_length.is_finite()
            || animation_length <= elapsed_seconds
            || self.position.abs() < SPRING_RESET_EPSILON
        {
            self.reset();
            return false;
        }

        if elapsed_seconds == 0.0 {
            return self.position.abs() >= SPRING_ACTIVE_EPSILON;
        }

        let angular_frequency = 4.0 / animation_length;
        let initial_position = self.position;
        let combined_velocity = self.position * angular_frequency + self.velocity;
        let decay = (-angular_frequency * elapsed_seconds).exp();
        self.position = (initial_position + combined_velocity * elapsed_seconds) * decay;
        self.velocity = decay
            * (-initial_position * angular_frequency
                - combined_velocity * elapsed_seconds * angular_frequency
                + combined_velocity);

        if !self.position.is_finite()
            || !self.velocity.is_finite()
            || self.position.abs() < SPRING_RESET_EPSILON
        {
            self.reset();
            return false;
        }

        self.position.abs() >= SPRING_ACTIVE_EPSILON
    }

    fn reset(&mut self) {
        self.position = 0.0;
        self.velocity = 0.0;
    }
}

#[derive(Clone, Copy, Debug)]
struct Corner {
    relative_position: AnimationPoint,
    current_position: AnimationPoint,
    target_position: AnimationPoint,
    horizontal_animation: DampedSpringAnimation,
    vertical_animation: DampedSpringAnimation,
    animation_length: f32,
}

impl Corner {
    fn new(relative_position: AnimationPoint) -> Self {
        Self {
            relative_position,
            current_position: AnimationPoint::default(),
            target_position: AnimationPoint::default(),
            horizontal_animation: DampedSpringAnimation::default(),
            vertical_animation: DampedSpringAnimation::default(),
            animation_length: 0.0,
        }
    }

    fn destination(self, geometry: CursorGeometry) -> AnimationPoint {
        let center = geometry.center();
        AnimationPoint {
            x: center.x + self.relative_position.x * geometry.width,
            y: center.y + self.relative_position.y * geometry.height,
        }
    }

    fn direction_alignment(self, geometry: CursorGeometry) -> f32 {
        let travel_direction = (self.destination(geometry) - self.current_position).normalized();
        let corner_direction = self.relative_position.normalized();
        travel_direction.x * corner_direction.x + travel_direction.y * corner_direction.y
    }

    fn snap(&mut self, geometry: CursorGeometry) {
        let destination = self.destination(geometry);
        self.current_position = destination;
        self.target_position = destination;
        self.horizontal_animation.reset();
        self.vertical_animation.reset();
    }

    fn retarget(&mut self, geometry: CursorGeometry, rank: usize) {
        let destination = self.destination(geometry);
        let horizontal_jump =
            (destination.x - self.target_position.x) / geometry.width.max(f32::EPSILON);
        let vertical_jump =
            (destination.y - self.target_position.y) / geometry.height.max(f32::EPSILON);
        let normalized_jump = AnimationPoint {
            x: horizontal_jump,
            y: vertical_jump,
        }
        .normalized();
        let corner_direction = self.relative_position.normalized();
        let leading_alignment =
            normalized_jump.x * corner_direction.x + normalized_jump.y * corner_direction.y;
        let is_short_jump = horizontal_jump.abs() <= SHORT_MOVE_THRESHOLD
            && vertical_jump.abs() <= SPRING_RESET_EPSILON;

        let base_animation_length = if is_short_jump {
            ANIMATION_LENGTH_SECONDS.min(SHORT_ANIMATION_LENGTH_SECONDS)
        } else {
            ANIMATION_LENGTH_SECONDS
        };
        let reference_animation_length = if leading_alignment > LEADING_SNAP_THRESHOLD {
            SNAP_ANIMATION_LENGTH_SECONDS
        } else {
            base_animation_length * RANK_TRAIL_FACTORS[rank.min(3)]
        };
        self.animation_length = base_animation_length
            + (reference_animation_length - base_animation_length) * TRAIL_SIZE;

        if self.animation_length > ANIMATION_RESET_THRESHOLD_SECONDS {
            self.horizontal_animation.reset();
            self.vertical_animation.reset();
        }

        self.target_position = destination;
        self.horizontal_animation.position = destination.x - self.current_position.x;
        self.vertical_animation.position = destination.y - self.current_position.y;
    }

    fn update(&mut self, elapsed_seconds: f32, max_trail_distance: f32) -> bool {
        self.horizontal_animation
            .update(elapsed_seconds, self.animation_length);
        self.vertical_animation
            .update(elapsed_seconds, self.animation_length);
        self.horizontal_animation.position = self
            .horizontal_animation
            .position
            .clamp(-max_trail_distance, max_trail_distance);
        self.vertical_animation.position = self
            .vertical_animation
            .position
            .clamp(-max_trail_distance, max_trail_distance);
        self.current_position = AnimationPoint {
            x: self.target_position.x - self.horizontal_animation.position,
            y: self.target_position.y - self.vertical_animation.position,
        };
        self.horizontal_animation.position.abs() > CORNER_ACTIVE_DISTANCE
            || self.vertical_animation.position.abs() > CORNER_ACTIVE_DISTANCE
    }
}

#[derive(Clone)]
pub(crate) struct CursorAnimationState {
    corners: [Corner; 4],
    target_geometry: Option<CursorGeometry>,
    last_logical_position: Option<LogicalCursorPosition>,
    last_viewport: Option<CursorViewport>,
    last_frame_at: Option<Instant>,
    active: bool,
}

#[derive(Default)]
pub(crate) struct CursorAnimationStates {
    states: HashMap<usize, CursorAnimationState>,
    newest_selection_id: Option<usize>,
    newest_state: Option<CursorAnimationState>,
}

impl CursorAnimationStates {
    pub(crate) fn reconcile_newest_selection(&mut self, newest_selection_id: Option<usize>) {
        // Mouse selection begins with a fresh pending selection id. Seed that id from an
        // independent snapshot of the previous newest cursor's geometry and velocity. The
        // snapshot survives selection replacement and visibility filtering, so click animation
        // does not depend on whether a render-time SelectionLayout happens to compare equal to
        // the pending selection.
        if let Some(newest_selection_id) = newest_selection_id
            && self.newest_selection_id != Some(newest_selection_id)
            && !self.states.contains_key(&newest_selection_id)
        {
            let inherited_state = self
                .newest_selection_id
                .and_then(|selection_id| self.states.get(&selection_id).cloned())
                .or_else(|| self.newest_state.clone());
            if let Some(state) = inherited_state {
                self.states.insert(newest_selection_id, state);
            }
        }

        self.newest_selection_id = newest_selection_id;
    }

    pub(crate) fn capture_newest_state(&mut self) {
        if let Some(newest_selection_id) = self.newest_selection_id
            && let Some(state) = self.states.get(&newest_selection_id)
        {
            self.newest_state = Some(state.clone());
        }
    }

    pub(crate) fn update(
        &mut self,
        selection_id: usize,
        logical_position: LogicalCursorPosition,
        target_bounds: Bounds<Pixels>,
        viewport: CursorViewport,
        now: Instant,
    ) -> Option<[Point<Pixels>; 4]> {
        self.states.entry(selection_id).or_default().update(
            logical_position,
            target_bounds,
            viewport,
            now,
        )
    }

    pub(crate) fn remove(&mut self, selection_id: usize) {
        self.states.remove(&selection_id);
    }

    pub(crate) fn retain(&mut self, mut keep: impl FnMut(usize) -> bool) {
        self.states.retain(|selection_id, _| keep(*selection_id));
    }

    pub(crate) fn clear(&mut self) {
        self.states.clear();
        self.newest_selection_id = None;
        self.newest_state = None;
    }
}

impl Default for CursorAnimationState {
    fn default() -> Self {
        const RELATIVE_POSITIONS: [AnimationPoint; 4] = [
            AnimationPoint { x: -0.5, y: -0.5 },
            AnimationPoint { x: 0.5, y: -0.5 },
            AnimationPoint { x: 0.5, y: 0.5 },
            AnimationPoint { x: -0.5, y: 0.5 },
        ];

        Self {
            corners: RELATIVE_POSITIONS.map(Corner::new),
            target_geometry: None,
            last_logical_position: None,
            last_viewport: None,
            last_frame_at: None,
            active: false,
        }
    }
}

impl CursorAnimationState {
    pub(crate) fn update(
        &mut self,
        logical_position: LogicalCursorPosition,
        target_bounds: Bounds<Pixels>,
        viewport: CursorViewport,
        now: Instant,
    ) -> Option<[Point<Pixels>; 4]> {
        let target_geometry = CursorGeometry::from_bounds(target_bounds);
        if !target_geometry.is_finite() || !viewport.is_finite() {
            self.reset();
            return None;
        }

        let Some(previous_geometry) = self.target_geometry else {
            self.snap(logical_position, target_geometry, viewport, now);
            return None;
        };

        let logical_position_changed = self.last_logical_position != Some(logical_position);
        let target_origin_changed = !previous_geometry.has_same_origin(target_geometry);
        let viewport_changed = self.last_viewport != Some(viewport);
        let geometry_size_changed = !previous_geometry.has_same_size(target_geometry);

        if viewport_changed
            || geometry_size_changed
            || (!logical_position_changed && target_origin_changed)
        {
            self.snap(logical_position, target_geometry, viewport, now);
            return None;
        }

        self.last_logical_position = Some(logical_position);
        self.last_viewport = Some(viewport);
        self.target_geometry = Some(target_geometry);

        if target_origin_changed {
            let elapsed = if self.active {
                self.elapsed_since_last_frame(now)
            } else {
                Duration::ZERO
            };
            self.retarget(target_geometry);
            self.advance(elapsed);
            self.last_frame_at = Some(now);
        } else if self.active {
            let elapsed = self.elapsed_since_last_frame(now);
            self.advance(elapsed);
            self.last_frame_at = Some(now);
        }

        self.active.then(|| {
            self.corners
                .map(|corner| point(px(corner.current_position.x), px(corner.current_position.y)))
        })
    }

    pub(crate) fn reset(&mut self) {
        self.target_geometry = None;
        self.last_logical_position = None;
        self.last_viewport = None;
        self.last_frame_at = None;
        self.active = false;
        for corner in &mut self.corners {
            corner.horizontal_animation.reset();
            corner.vertical_animation.reset();
        }
    }

    fn snap(
        &mut self,
        logical_position: LogicalCursorPosition,
        geometry: CursorGeometry,
        viewport: CursorViewport,
        now: Instant,
    ) {
        for corner in &mut self.corners {
            corner.snap(geometry);
        }
        self.target_geometry = Some(geometry);
        self.last_logical_position = Some(logical_position);
        self.last_viewport = Some(viewport);
        self.last_frame_at = Some(now);
        self.active = false;
    }

    fn retarget(&mut self, geometry: CursorGeometry) {
        let mut aligned_corners: [(usize, f32); 4] =
            std::array::from_fn(|index| (index, self.corners[index].direction_alignment(geometry)));
        aligned_corners.sort_by(|left, right| left.1.total_cmp(&right.1));
        let mut ranks = [0; 4];
        for (rank, (index, _)) in aligned_corners.into_iter().enumerate() {
            ranks[index] = rank;
        }

        for (index, corner) in self.corners.iter_mut().enumerate() {
            corner.retarget(geometry, ranks[index]);
        }
        self.active = self.corners.iter().any(|corner| {
            corner.horizontal_animation.position != 0.0 || corner.vertical_animation.position != 0.0
        });
    }

    fn elapsed_since_last_frame(&self, now: Instant) -> Duration {
        self.last_frame_at
            .and_then(|last_frame_at| now.checked_duration_since(last_frame_at))
            .unwrap_or_default()
            .min(MAX_FRAME_DURATION)
    }

    fn advance(&mut self, elapsed: Duration) {
        let elapsed_seconds = elapsed.as_secs_f32();
        let max_trail_distance = self
            .target_geometry
            .map(|geometry| geometry.width.max(geometry.height) * MAX_TRAIL_DISTANCE_FACTOR)
            .unwrap_or_default();
        self.active = self.corners.iter_mut().fold(false, |active, corner| {
            corner.update(elapsed_seconds, max_trail_distance) || active
        });

        if !self.active
            && let Some(target_geometry) = self.target_geometry
        {
            for corner in &mut self.corners {
                corner.snap(target_geometry);
            }
        }
    }
}

fn nearly_equal(left: f32, right: f32) -> bool {
    (left - right).abs() <= GEOMETRY_EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::size;

    fn bounds(x: f32, y: f32) -> Bounds<Pixels> {
        Bounds {
            origin: point(px(x), px(y)),
            size: size(px(2.0), px(20.0)),
        }
    }

    fn viewport(scroll_y: f64) -> CursorViewport {
        CursorViewport::new(
            point(px(0.0), px(0.0)),
            Bounds {
                origin: point(px(0.0), px(0.0)),
                size: size(px(800.0), px(600.0)),
            },
            point(0.0, scroll_y),
            point(0.0, scroll_y * 20.0),
            px(20.0),
            px(8.0),
        )
    }

    fn logical_position(row: u32, column: u32) -> LogicalCursorPosition {
        LogicalCursorPosition { row, column }
    }

    #[test]
    fn spring_converges() {
        let mut spring = DampedSpringAnimation {
            position: 100.0,
            velocity: 0.0,
        };

        for _ in 0..60 {
            spring.update(1.0 / 120.0, 0.1);
        }

        assert_eq!(spring.position, 0.0);
        assert_eq!(spring.velocity, 0.0);
    }

    #[test]
    fn first_position_snaps_and_reset_forgets_it() {
        let now = Instant::now();
        let mut state = CursorAnimationState::default();
        assert!(
            state
                .update(
                    logical_position(1, 1),
                    bounds(10.0, 20.0),
                    viewport(0.0),
                    now,
                )
                .is_none()
        );
        assert!(!state.active);
        assert_eq!(state.corners[0].current_position.x, 10.0);

        state.reset();
        assert!(state.target_geometry.is_none());
        assert!(state.last_logical_position.is_none());
    }

    #[test]
    fn mid_flight_retarget_starts_from_current_geometry() {
        let now = Instant::now();
        let mut state = CursorAnimationState::default();
        state.update(logical_position(0, 0), bounds(0.0, 0.0), viewport(0.0), now);
        state.update(
            logical_position(0, 10),
            bounds(100.0, 0.0),
            viewport(0.0),
            now,
        );
        state.update(
            logical_position(0, 10),
            bounds(100.0, 0.0),
            viewport(0.0),
            now + Duration::from_millis(16),
        );
        let position_before_retarget = state.corners.map(|corner| corner.current_position);

        state.update(
            logical_position(0, 20),
            bounds(200.0, 0.0),
            viewport(0.0),
            now + Duration::from_millis(16),
        );
        assert_eq!(
            state.corners.map(|corner| corner.current_position),
            position_before_retarget
        );

        for frame in 2..30 {
            state.update(
                logical_position(0, 20),
                bounds(200.0, 0.0),
                viewport(0.0),
                now + Duration::from_millis(frame * 16),
            );
        }
        assert!(!state.active);
        assert_eq!(state.corners[0].current_position.x, 200.0);
    }

    #[test]
    fn short_horizontal_movement_uses_short_duration() {
        let now = Instant::now();
        let mut state = CursorAnimationState::default();
        state.update(logical_position(0, 0), bounds(0.0, 0.0), viewport(0.0), now);
        state.update(logical_position(0, 1), bounds(4.0, 0.0), viewport(0.0), now);

        assert!((state.corners[0].animation_length - 0.05).abs() < f32::EPSILON);
        assert!((state.corners[3].animation_length - 0.045).abs() < f32::EPSILON);
        assert!((state.corners[1].animation_length - 0.02).abs() < f32::EPSILON);
        assert!((state.corners[2].animation_length - 0.02).abs() < f32::EPSILON);
    }

    #[test]
    fn short_mid_flight_retarget_preserves_momentum() {
        let now = Instant::now();
        let mut state = CursorAnimationState::default();
        state.update(logical_position(0, 0), bounds(0.0, 0.0), viewport(0.0), now);
        state.update(logical_position(0, 1), bounds(4.0, 0.0), viewport(0.0), now);
        state.update(
            logical_position(0, 1),
            bounds(4.0, 0.0),
            viewport(0.0),
            now + Duration::from_millis(16),
        );
        let velocity_before_retarget = state.corners[0].horizontal_animation.velocity;
        assert_ne!(velocity_before_retarget, 0.0);

        state.update(
            logical_position(0, 2),
            bounds(8.0, 0.0),
            viewport(0.0),
            now + Duration::from_millis(16),
        );

        assert_eq!(
            state.corners[0].horizontal_animation.velocity,
            velocity_before_retarget
        );
    }

    #[test]
    fn movement_after_idle_starts_from_previous_geometry() {
        let now = Instant::now();
        let mut state = CursorAnimationState::default();
        state.update(logical_position(0, 0), bounds(0.0, 0.0), viewport(0.0), now);

        let corners = state
            .update(
                logical_position(0, 1),
                bounds(4.0, 0.0),
                viewport(0.0),
                now + Duration::from_secs(5),
            )
            .unwrap();
        assert_eq!(corners[0].x, px(0.0));

        let corners = state
            .update(
                logical_position(0, 1),
                bounds(4.0, 0.0),
                viewport(0.0),
                now + Duration::from_secs(5) + Duration::from_millis(16),
            )
            .unwrap();
        assert!(corners[0].x > px(0.0));
        assert!(corners[0].x < px(4.0));
    }

    #[test]
    fn long_movement_ranks_leading_corners_ahead_of_trailing_corners() {
        let now = Instant::now();
        let mut state = CursorAnimationState::default();
        state.update(logical_position(0, 0), bounds(0.0, 0.0), viewport(0.0), now);
        state.update(
            logical_position(0, 20),
            bounds(100.0, 0.0),
            viewport(0.0),
            now,
        );

        assert!(state.corners[1].animation_length < state.corners[0].animation_length);
        assert!(state.corners[2].animation_length < state.corners[3].animation_length);
    }

    #[test]
    fn diagonal_bar_jump_keeps_dimensionally_leading_edge_together() {
        let now = Instant::now();
        let mut state = CursorAnimationState::default();
        state.update(logical_position(0, 0), bounds(0.0, 0.0), viewport(0.0), now);
        state.update(
            logical_position(10, 25),
            bounds(200.0, 200.0),
            viewport(0.0),
            now,
        );

        // Match the reference implementation: hard-snap alignment uses a jump vector
        // normalized independently by cursor width and height. For a 2x20 bar moving 200px on
        // both axes, horizontal travel dominates in cursor dimensions, so both right corners
        // form one leading edge instead of only the diagonal-most corner racing ahead.
        assert!(
            (state.corners[1].animation_length - SNAP_ANIMATION_LENGTH_SECONDS).abs()
                < f32::EPSILON
        );
        assert!(
            (state.corners[2].animation_length - SNAP_ANIMATION_LENGTH_SECONDS).abs()
                < f32::EPSILON
        );
        assert!(state.corners[0].animation_length > SNAP_ANIMATION_LENGTH_SECONDS);
        assert!(state.corners[3].animation_length > SNAP_ANIMATION_LENGTH_SECONDS);

        state.update(
            logical_position(10, 25),
            bounds(200.0, 200.0),
            viewport(0.0),
            now + Duration::from_millis(16),
        );
        let leading_edge_height =
            state.corners[2].current_position.y - state.corners[1].current_position.y;
        assert!((leading_edge_height - 20.0).abs() < GEOMETRY_EPSILON);
    }

    #[test]
    fn viewport_movement_snaps_even_during_animation() {
        let now = Instant::now();
        let mut state = CursorAnimationState::default();
        state.update(logical_position(0, 0), bounds(0.0, 0.0), viewport(0.0), now);
        state.update(
            logical_position(0, 10),
            bounds(100.0, 0.0),
            viewport(0.0),
            now,
        );
        assert!(state.active);

        assert!(
            state
                .update(
                    logical_position(0, 10),
                    bounds(100.0, -20.0),
                    viewport(1.0),
                    now + Duration::from_millis(16),
                )
                .is_none()
        );
        assert!(!state.active);
        assert_eq!(state.corners[0].current_position.y, -20.0);
    }

    #[test]
    fn invalid_or_large_elapsed_time_never_produces_non_finite_values() {
        let mut spring = DampedSpringAnimation {
            position: 100.0,
            velocity: 10.0,
        };
        spring.update(f32::NAN, 0.1);
        assert!(spring.position.is_finite());
        assert!(spring.velocity.is_finite());

        spring.position = f32::INFINITY;
        spring.velocity = f32::NEG_INFINITY;
        spring.update(1.0, 0.1);
        assert!(spring.position.is_finite());
        assert!(spring.velocity.is_finite());
    }

    #[test]
    fn multiple_cursor_states_animate_independently_and_cleanup() {
        let now = Instant::now();
        let mut states = CursorAnimationStates::default();

        for (selection_id, x) in [(1, 0.0), (2, 40.0)] {
            assert!(
                states
                    .update(
                        selection_id,
                        logical_position(0, selection_id as u32),
                        bounds(x, 0.0),
                        viewport(0.0),
                        now,
                    )
                    .is_none()
            );
            assert!(
                states
                    .update(
                        selection_id,
                        logical_position(0, selection_id as u32 + 1),
                        bounds(x + 4.0, 0.0),
                        viewport(0.0),
                        now,
                    )
                    .is_some()
            );
        }

        assert_eq!(states.states.len(), 2);
        let second_cursor_before = states
            .states
            .get(&2)
            .unwrap()
            .corners
            .map(|corner| corner.current_position);

        states.update(
            1,
            logical_position(0, 3),
            bounds(8.0, 0.0),
            viewport(0.0),
            now + Duration::from_millis(16),
        );

        assert_eq!(
            states
                .states
                .get(&2)
                .unwrap()
                .corners
                .map(|corner| corner.current_position),
            second_cursor_before
        );

        states.retain(|selection_id| selection_id == 1);
        assert_eq!(states.states.len(), 1);
        assert!(states.states.contains_key(&1));
        states.clear();
        assert!(states.states.is_empty());
    }

    #[test]
    fn replacing_newest_selection_id_preserves_click_animation() {
        let now = Instant::now();
        let mut states = CursorAnimationStates::default();

        states.reconcile_newest_selection(Some(1));
        assert!(
            states
                .update(
                    1,
                    logical_position(0, 0),
                    bounds(0.0, 0.0),
                    viewport(0.0),
                    now,
                )
                .is_none()
        );

        states.capture_newest_state();
        states.reconcile_newest_selection(Some(2));
        assert!(states.states.contains_key(&1));
        assert!(states.states.contains_key(&2));
        states.retain(|selection_id| selection_id == 2);
        assert!(!states.states.contains_key(&1));
        assert!(
            states
                .update(
                    2,
                    logical_position(4, 8),
                    bounds(80.0, 80.0),
                    viewport(0.0),
                    now,
                )
                .is_some()
        );
    }

    #[test]
    fn adding_cursor_inherits_previous_newest_geometry_without_removing_it() {
        let now = Instant::now();
        let mut states = CursorAnimationStates::default();

        states.reconcile_newest_selection(Some(1));
        states.update(
            1,
            logical_position(0, 0),
            bounds(0.0, 0.0),
            viewport(0.0),
            now,
        );

        states.capture_newest_state();
        states.reconcile_newest_selection(Some(2));
        assert!(states.states.contains_key(&1));
        assert!(states.states.contains_key(&2));
        assert!(
            states
                .update(
                    2,
                    logical_position(4, 8),
                    bounds(80.0, 80.0),
                    viewport(0.0),
                    now,
                )
                .is_some()
        );
        assert_eq!(
            states.states.get(&1).unwrap().last_logical_position,
            Some(logical_position(0, 0))
        );
    }

    #[test]
    fn newest_geometry_snapshot_survives_state_visibility_cleanup() {
        let now = Instant::now();
        let mut states = CursorAnimationStates::default();

        states.reconcile_newest_selection(Some(1));
        states.update(
            1,
            logical_position(0, 0),
            bounds(0.0, 0.0),
            viewport(0.0),
            now,
        );
        states.capture_newest_state();
        states.retain(|_| false);
        assert!(states.states.is_empty());

        states.reconcile_newest_selection(Some(2));
        assert!(states.states.contains_key(&2));
        assert!(
            states
                .update(
                    2,
                    logical_position(4, 8),
                    bounds(80.0, 80.0),
                    viewport(0.0),
                    now,
                )
                .is_some()
        );
    }
}
