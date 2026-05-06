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

pub(crate) struct KineticScroller {
    history: KineticScrollHistory,
    id: u64,
    scroll: Option<KineticScroll>,
    finger_active: bool,
    finger_start_pending: bool,
    finger_stop_pending: bool,
}

struct KineticScrollHistory {
    entries: VecDeque<(Instant, Point<Pixels>)>,
}

struct KineticScroll {
    id: u64,
    window: WaylandWindowStatePtr,
    position: Point<Pixels>,
    modifiers: Modifiers,
    velocity: Point<Pixels>,
    last_time: Instant,
}

impl KineticScroller {
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
        }
    }

    fn push(&mut self, time: Instant, delta: Point<Pixels>) {
        self.entries.push_back((time, delta));
        let cutoff = time - KINETIC_SCROLL_HISTORY_WINDOW;
        while self
            .entries
            .front()
            .is_some_and(|(entry_time, _)| *entry_time < cutoff)
        {
            self.entries.pop_front();
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

        let mut delta = point(px(0.0), px(0.0));
        for (_, entry_delta) in &self.entries {
            delta.x += entry_delta.x;
            delta.y += entry_delta.y;
        }

        point(
            px((f32::from(delta.x) / duration)
                .clamp(-KINETIC_SCROLL_MAX_VELOCITY, KINETIC_SCROLL_MAX_VELOCITY)),
            px((f32::from(delta.y) / duration)
                .clamp(-KINETIC_SCROLL_MAX_VELOCITY, KINETIC_SCROLL_MAX_VELOCITY)),
        )
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}

fn is_kinetic_scroll_stopped(velocity: Point<Pixels>) -> bool {
    f32::from(velocity.x).abs() < KINETIC_SCROLL_STOP_VELOCITY
        && f32::from(velocity.y).abs() < KINETIC_SCROLL_STOP_VELOCITY
}
