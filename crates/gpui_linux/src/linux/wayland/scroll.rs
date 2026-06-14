use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use super::window::WaylandWindowStatePtr;
use gpui::{
    Modifiers, Pixels, PlatformInput, Point, ScrollDelta, ScrollWheelEvent, TouchPhase, point, px,
};

const KINETIC_SCROLL_HISTORY_WINDOW: Duration = Duration::from_millis(150);
const KINETIC_SCROLL_FRICTION: f32 = 4.0;
const KINETIC_SCROLL_STOP_VELOCITY: f32 = 5.0;
const KINETIC_SCROLL_MAX_VELOCITY: f32 = 6000.0;

pub(crate) struct KineticScrollController {
    history: KineticScrollHistory,
    id: u64,
    scroll: Option<KineticScroll>,
    finger_active: bool,
    finger_start_pending: bool,
    finger_stop_pending: bool,
}

struct KineticScrollHistory {
    entries: VecDeque<(Instant, Point<Pixels>)>,
    displacement: Point<Pixels>,
}

struct KineticScroll {
    id: u64,
    window: WaylandWindowStatePtr,
    position: Point<Pixels>,
    modifiers: Modifiers,
    velocity: Point<Pixels>,
    last_time: Instant,
}

impl KineticScrollController {
    pub(crate) fn new() -> Self {
        Self {
            history: KineticScrollHistory::new(),
            id: 0,
            scroll: None,
            finger_active: false,
            finger_start_pending: false,
            finger_stop_pending: false,
        }
    }

    pub(crate) fn start_finger_scroll(&mut self) {
        self.id += 1;
        self.scroll = None;
        self.finger_stop_pending = false;
        if !self.finger_active {
            self.finger_active = true;
            self.finger_start_pending = true;
            self.history.clear();
        }
    }

    pub(crate) fn stop_finger_scroll(&mut self) -> bool {
        if self.finger_active {
            self.finger_stop_pending = true;
            true
        } else {
            false
        }
    }

    pub(crate) fn touch_phase(&mut self) -> TouchPhase {
        if self.finger_start_pending {
            self.finger_start_pending = false;
            TouchPhase::Started
        } else {
            TouchPhase::Moved
        }
    }

    pub(crate) fn record_delta(&mut self, time: Instant, delta: Point<Pixels>) {
        self.history.push(time, delta);
    }

    pub(crate) fn has_pending_stop(&self) -> bool {
        self.finger_stop_pending
    }

    pub(crate) fn finish_pending_stop(
        &mut self,
        window: WaylandWindowStatePtr,
        position: Point<Pixels>,
        modifiers: Modifiers,
    ) -> Option<(WaylandWindowStatePtr, PlatformInput)> {
        self.finger_stop_pending = false;
        self.finger_active = false;
        self.finger_start_pending = false;
        let velocity = self.history.velocity(Instant::now());
        self.history.clear();
        self.start(window, position, modifiers, velocity)
    }

    pub(crate) fn tick(
        &mut self,
        window: &WaylandWindowStatePtr,
    ) -> Option<(WaylandWindowStatePtr, PlatformInput)> {
        let kinetic_scroll = self.scroll.as_mut()?;
        if kinetic_scroll.id != self.id || !kinetic_scroll.window.ptr_eq(window) {
            return None;
        }

        let now = Instant::now();
        let elapsed = now
            .duration_since(kinetic_scroll.last_time)
            .as_secs_f32()
            .min(0.05);
        kinetic_scroll.last_time = now;

        let delta = point(
            px(f32::from(kinetic_scroll.velocity.x) * elapsed),
            px(f32::from(kinetic_scroll.velocity.y) * elapsed),
        );
        let velocity_multiplier = (-KINETIC_SCROLL_FRICTION * elapsed).exp();
        kinetic_scroll.velocity.x = px(f32::from(kinetic_scroll.velocity.x) * velocity_multiplier);
        kinetic_scroll.velocity.y = px(f32::from(kinetic_scroll.velocity.y) * velocity_multiplier);

        let finished = is_kinetic_scroll_stopped(kinetic_scroll.velocity);

        let input = PlatformInput::ScrollWheel(ScrollWheelEvent {
            position: kinetic_scroll.position,
            delta: ScrollDelta::Pixels(delta),
            modifiers: kinetic_scroll.modifiers,
            touch_phase: if finished {
                TouchPhase::Ended
            } else {
                TouchPhase::Moved
            },
        });

        let window = kinetic_scroll.window.clone();
        if finished {
            self.finger_active = false;
            self.finger_start_pending = false;
            self.finger_stop_pending = false;
            self.scroll = None;
            self.history.clear();
        }

        Some((window, input))
    }

    pub(crate) fn cancel(&mut self) -> Option<(WaylandWindowStatePtr, PlatformInput)> {
        let kinetic_scroll = self.scroll.take()?;

        self.id += 1;
        self.finger_active = false;
        self.finger_start_pending = false;
        self.finger_stop_pending = false;
        self.history.clear();

        Some((
            kinetic_scroll.window,
            PlatformInput::ScrollWheel(ScrollWheelEvent {
                position: kinetic_scroll.position,
                delta: ScrollDelta::Pixels(point(px(0.0), px(0.0))),
                modifiers: kinetic_scroll.modifiers,
                touch_phase: TouchPhase::Ended,
            }),
        ))
    }

    fn start(
        &mut self,
        window: WaylandWindowStatePtr,
        position: Point<Pixels>,
        modifiers: Modifiers,
        velocity: Point<Pixels>,
    ) -> Option<(WaylandWindowStatePtr, PlatformInput)> {
        self.id += 1;
        if is_kinetic_scroll_stopped(velocity) {
            return Some((
                window,
                PlatformInput::ScrollWheel(ScrollWheelEvent {
                    position,
                    delta: ScrollDelta::Pixels(point(px(0.0), px(0.0))),
                    modifiers,
                    touch_phase: TouchPhase::Ended,
                }),
            ));
        }

        let id = self.id;
        self.scroll = Some(KineticScroll {
            id,
            window,
            position,
            modifiers,
            velocity,
            last_time: Instant::now(),
        });
        None
    }
}

impl KineticScrollHistory {
    fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            displacement: point(px(0.0), px(0.0)),
        }
    }

    fn push(&mut self, time: Instant, delta: Point<Pixels>) {
        self.entries.push_back((time, delta));
        self.displacement += delta;
        let cutoff = time - KINETIC_SCROLL_HISTORY_WINDOW;
        while self
            .entries
            .front()
            .is_some_and(|(entry_time, _)| *entry_time < cutoff)
        {
            let delta = self
                .entries
                .pop_front()
                .map(|(_instant, delta)| delta)
                .unwrap_or(point(px(0.0), px(0.0)));
            self.displacement -= delta;
        }
    }

    fn velocity(&self, now: Instant) -> Point<Pixels> {
        let Some((first_time, _)) = self.entries.front() else {
            return point(px(0.0), px(0.0));
        };

        let duration = now.duration_since(*first_time).as_secs_f32();
        if duration == 0.0 {
            return point(px(0.0), px(0.0));
        }

        let delta = self.displacement;

        point(
            px((f32::from(delta.x) / duration)
                .clamp(-KINETIC_SCROLL_MAX_VELOCITY, KINETIC_SCROLL_MAX_VELOCITY)),
            px((f32::from(delta.y) / duration)
                .clamp(-KINETIC_SCROLL_MAX_VELOCITY, KINETIC_SCROLL_MAX_VELOCITY)),
        )
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.displacement = point(px(0.0), px(0.0));
    }
}

fn is_kinetic_scroll_stopped(velocity: Point<Pixels>) -> bool {
    f32::from(velocity.x).abs() < KINETIC_SCROLL_STOP_VELOCITY
        && f32::from(velocity.y).abs() < KINETIC_SCROLL_STOP_VELOCITY
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_kinetic_scroll_stopped() {
        assert!(is_kinetic_scroll_stopped(point(px(0.0), px(0.0))));
        assert!(is_kinetic_scroll_stopped(point(px(4.9), px(4.9))));
        assert!(is_kinetic_scroll_stopped(point(px(-4.9), px(4.9))));
        assert!(!is_kinetic_scroll_stopped(point(px(5.0), px(0.0))));
        assert!(!is_kinetic_scroll_stopped(point(px(0.0), px(5.0))));
        assert!(!is_kinetic_scroll_stopped(point(px(100.0), px(100.0))));
    }

    #[test]
    fn test_history_velocity_empty() {
        let history = KineticScrollHistory::new();
        let velocity = history.velocity(Instant::now());
        assert_eq!(f32::from(velocity.x), 0.0);
        assert_eq!(f32::from(velocity.y), 0.0);
    }

    #[test]
    fn test_history_velocity_single_entry() {
        let mut history = KineticScrollHistory::new();
        let now = Instant::now();
        history.push(
            now - Duration::from_millis(100),
            point(px(100.0), px(200.0)),
        );
        let velocity = history.velocity(now);
        assert!((f32::from(velocity.x) - 1000.0).abs() < 1.0);
        assert!((f32::from(velocity.y) - 2000.0).abs() < 1.0);
    }

    #[test]
    fn test_history_velocity_multiple_entries() {
        let mut history = KineticScrollHistory::new();
        let now = Instant::now();
        history.push(now - Duration::from_millis(100), point(px(50.0), px(0.0)));
        history.push(now - Duration::from_millis(50), point(px(50.0), px(100.0)));
        let velocity = history.velocity(now);
        assert!((f32::from(velocity.x) - 1000.0).abs() < 1.0);
        assert!((f32::from(velocity.y) - 1000.0).abs() < 1.0);
    }

    #[test]
    fn test_history_velocity_zero_duration() {
        let mut history = KineticScrollHistory::new();
        let now = Instant::now();
        history.push(now, point(px(100.0), px(200.0)));
        let velocity = history.velocity(now);
        assert_eq!(f32::from(velocity.x), 0.0);
        assert_eq!(f32::from(velocity.y), 0.0);
    }

    #[test]
    fn test_history_velocity_clamped() {
        let mut history = KineticScrollHistory::new();
        let now = Instant::now();
        history.push(
            now - Duration::from_millis(1),
            point(px(60000.0), px(-60000.0)),
        );
        let velocity = history.velocity(now);
        assert_eq!(f32::from(velocity.x), KINETIC_SCROLL_MAX_VELOCITY);
        assert_eq!(f32::from(velocity.y), -KINETIC_SCROLL_MAX_VELOCITY);
    }

    #[test]
    fn test_history_prunes_old_entries() {
        let mut history = KineticScrollHistory::new();
        let now = Instant::now();
        history.push(
            now - Duration::from_millis(300),
            point(px(9999.0), px(9999.0)),
        );
        history.push(
            now - Duration::from_millis(100),
            point(px(100.0), px(100.0)),
        );
        let velocity = history.velocity(now);
        assert!((f32::from(velocity.x) - 1000.0).abs() < 1.0);
        assert!((f32::from(velocity.y) - 1000.0).abs() < 1.0);
        assert_eq!(history.entries.len(), 1);
    }

    #[test]
    fn test_history_clear() {
        let mut history = KineticScrollHistory::new();
        let now = Instant::now();
        history.push(now, point(px(100.0), px(100.0)));
        assert_eq!(history.entries.len(), 1);
        history.clear();
        assert_eq!(history.entries.len(), 0);
    }

    #[test]
    fn test_touch_phase_sequence() {
        let mut scroller = KineticScrollController::new();
        assert!(matches!(scroller.touch_phase(), TouchPhase::Moved));

        scroller.start_finger_scroll();
        assert!(matches!(scroller.touch_phase(), TouchPhase::Started));
        assert!(matches!(scroller.touch_phase(), TouchPhase::Moved));
        assert!(matches!(scroller.touch_phase(), TouchPhase::Moved));
    }

    #[test]
    fn test_start_finger_scroll_id_increments() {
        let mut scroller = KineticScrollController::new();
        let id_before = scroller.id;
        scroller.start_finger_scroll();
        assert_eq!(scroller.id, id_before + 1);
    }

    #[test]
    fn test_start_finger_scroll_resets_stop_pending() {
        let mut scroller = KineticScrollController::new();
        scroller.start_finger_scroll();
        assert!(scroller.stop_finger_scroll());
        assert!(scroller.has_pending_stop());
        scroller.start_finger_scroll();
        assert!(!scroller.has_pending_stop());
    }

    #[test]
    fn test_start_finger_scroll_does_not_reset_active() {
        let mut scroller = KineticScrollController::new();
        scroller.start_finger_scroll();
        assert!(scroller.finger_active);
        assert!(scroller.finger_start_pending);

        scroller.touch_phase();
        assert!(!scroller.finger_start_pending);

        scroller.start_finger_scroll();
        assert!(scroller.finger_active);
        assert!(!scroller.finger_start_pending);
    }

    #[test]
    fn test_stop_finger_scroll_when_inactive() {
        let mut scroller = KineticScrollController::new();
        assert!(!scroller.stop_finger_scroll());
        assert!(!scroller.has_pending_stop());
    }

    #[test]
    fn test_stop_finger_scroll_when_active() {
        let mut scroller = KineticScrollController::new();
        scroller.start_finger_scroll();
        assert!(scroller.stop_finger_scroll());
        assert!(scroller.has_pending_stop());
    }

    #[test]
    fn test_record_delta_and_has_pending_stop() {
        let mut scroller = KineticScrollController::new();
        scroller.start_finger_scroll();
        scroller.record_delta(Instant::now(), point(px(10.0), px(20.0)));
        assert_eq!(scroller.history.entries.len(), 1);
        assert!(!scroller.has_pending_stop());

        scroller.stop_finger_scroll();
        assert!(scroller.has_pending_stop());
    }
}
