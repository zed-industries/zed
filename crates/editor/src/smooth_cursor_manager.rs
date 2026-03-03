use collections::HashMap;
use gpui::{Point, Pixels};

use crate::smooth_cursor::SmoothCursor;

/// Omega (rad/s) for the critically-damped spring used to animate cursor motion.
/// At omega=30 the cursor reaches its target in ~100 ms with no overshoot.
const CURSOR_OMEGA: f32 = 30.0;

/// Cursors that jump further than this many pixels teleport instead of animating.
/// Covers go-to-definition, file open, and Ctrl+Home/End where animation
/// across the full viewport would look wrong.
const TELEPORT_DISTANCE: f32 = 600.0;

/// Per-editor table of in-flight cursor springs, keyed by the cursor's stable
/// pixel-origin (rounded to integer coords) encoded as a u64.  The key is
/// recomputed from the *target* position every frame so it tracks the logical
/// cursor even when the buffer scrolls under it.
///
/// Stored on `Editor` and driven frame-by-frame from `EditorElement::paint_cursors`.
#[derive(Debug, Default)]
pub struct SmoothCursorManager {
    /// Map from stable cursor key -> spring state.
    springs: HashMap<u64, SmoothCursor>,
}

impl SmoothCursorManager {
    pub fn new() -> Self {
        Self {
            springs: HashMap::default(),
        }
    }

    /// Encode a pixel position as a u64 key (32-bit x | 32-bit y).
    /// Positions are rounded to the nearest integer pixel so that sub-pixel
    /// jitter from scroll doesn't create phantom new cursors.
    fn key(pos: Point<Pixels>) -> u64 {
        let x = (pos.x.0.round() as i32) as u32;
        let y = (pos.y.0.round() as i32) as u32;
        ((x as u64) << 32) | (y as u64)
    }

    /// Given the set of *target* cursor positions for this frame, advance each
    /// spring by `dt` seconds and return the smoothed draw positions.
    ///
    /// Cursors that appear for the first time are initialised at their target
    /// so there is no animation on the very first frame (avoids a fly-in on
    /// every file open).
    ///
    /// Springs that have settled and are no longer in `targets` are pruned to
    /// keep the map small.
    pub fn tick(
        &mut self,
        targets: &[Point<Pixels>],
        dt: f32,
    ) -> Vec<Point<Pixels>> {
        // Build a set of keys for living cursors so we can prune stale ones.
        let live_keys: Vec<u64> = targets.iter().map(|&t| Self::key(t)).collect();

        // Remove springs whose target cursor is no longer present.
        self.springs.retain(|k, _| live_keys.contains(k));

        targets
            .iter()
            .map(|&target| {
                let key = Self::key(target);
                let spring = self.springs.entry(key).or_insert_with(|| {
                    SmoothCursor::new(target, CURSOR_OMEGA, TELEPORT_DISTANCE)
                });
                spring.set_target(target);
                spring.tick(dt);
                spring.current
            })
            .collect()
    }

    /// Returns true if any spring is still moving and the frame loop should
    /// keep ticking.
    pub fn is_animating(&self) -> bool {
        self.springs.values().any(|s| !s.is_settled())
    }
}
