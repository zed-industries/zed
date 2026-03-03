use collections::HashMap;
use gpui::{Point, Pixels};
use std::time::Instant;

use crate::smooth_cursor::SmoothCursor;

/// Natural frequency (rad/s) for the critically-damped spring.
/// omega=30 gives ~100 ms settle time with zero overshoot.
const CURSOR_OMEGA: f32 = 30.0;

/// Cursors that jump further than this many pixels snap immediately.
/// Prevents disorienting fly-ins on go-to-definition, Ctrl+Home/End, etc.
const TELEPORT_DISTANCE: f32 = 600.0;

/// Per-editor table of in-flight cursor springs.
///
/// Keyed by the cursor's target pixel position encoded as a u64
/// (rounded to integer coords so sub-pixel scroll jitter doesn't
/// create phantom entries).  The key is recomputed from the *target*
/// every frame so it naturally follows the logical cursor as the
/// buffer scrolls beneath it.
///
/// Lives in `element.rs`'s module-level static, keyed by editor entity id.
/// Timing is self-contained: each manager records the `Instant` of its
/// last tick so callers don't need to supply a clock.
#[derive(Debug)]
pub struct SmoothCursorManager {
    springs: HashMap<u64, SmoothCursor>,
    last_tick: Option<Instant>,
}

impl Default for SmoothCursorManager {
    fn default() -> Self {
        Self {
            springs: HashMap::default(),
            last_tick: None,
        }
    }
}

impl SmoothCursorManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Encode a pixel position as a u64 key (32-bit x packed into high
    /// bits, 32-bit y in low bits).  Rounded to the nearest integer pixel
    /// so sub-pixel jitter from scrolling doesn't spawn phantom cursors.
    fn key(pos: Point<Pixels>) -> u64 {
        let x = (pos.x.0.round() as i32) as u32;
        let y = (pos.y.0.round() as i32) as u32;
        ((x as u64) << 32) | (y as u64)
    }

    /// Advance every spring by the wall-clock time elapsed since the last
    /// call and return the smoothed draw positions.
    ///
    /// First-time cursors are initialised *at* their target, so there is
    /// no fly-in animation when a file is first opened.
    /// Springs for cursors that have disappeared are pruned automatically.
    pub fn tick(&mut self, targets: &[Point<Pixels>]) -> Vec<Point<Pixels>> {
        let now = Instant::now();
        let dt = match self.last_tick {
            Some(prev) => {
                let elapsed = now.duration_since(prev).as_secs_f32();
                // Cap at 50 ms so a long stall doesn't look like a snap.
                elapsed.min(0.05)
            }
            // First frame: dt=0 means all springs initialise at target.
            None => 0.0,
        };
        self.last_tick = Some(now);

        // Prune springs for cursors that no longer exist.
        let live_keys: Vec<u64> = targets.iter().map(|&t| Self::key(t)).collect();
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

    /// `true` while any spring has not yet settled.  Used to decide
    /// whether to call `window.request_animation_frame()`.
    pub fn is_animating(&self) -> bool {
        self.springs.values().any(|s| !s.is_settled())
    }
}
