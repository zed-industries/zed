//! Deceleration for UIKit scroll gestures that do not supply native momentum.
//!
//! Recognition stays in UIKit. Only continuous scrolling supplies a release
//! velocity; discrete wheel events must never acquire a synthetic fling.

use gpui::{GestureTuning, Pixels, Point, ScrollDelta, ScrollEvent, ScrollFling, TouchPhase};
use std::time::Instant;

#[derive(Default)]
pub(crate) struct IndirectScroll {
    position: Option<Point<Pixels>>,
    momentum: Option<(ScrollFling, Instant, ScrollEvent)>,
}

impl IndirectScroll {
    pub(crate) fn handle_event(
        &mut self,
        event: &mut ScrollEvent,
        release_velocity: Option<Point<Pixels>>,
        now: Instant,
    ) {
        self.momentum = None;
        if event.touch_phase == TouchPhase::Started {
            self.position = Some(event.position);
        }
        // Keep the gesture and its fling on the original scroll container,
        // even if the pointer moves while UIKit is delivering the gesture.
        event.position = self.position.unwrap_or(event.position);
        if matches!(event.touch_phase, TouchPhase::Ended | TouchPhase::Cancelled) {
            self.position = None;
        }
        if event.touch_phase == TouchPhase::Ended
            && let Some(velocity) = release_velocity
        {
            let tuning = GestureTuning::default();
            let speed = f32::from(velocity.x).hypot(f32::from(velocity.y));
            if speed.is_finite() && speed >= tuning.min_fling_velocity {
                let fling = tuning.scroll_physics.fling(velocity);
                if !fling.duration().is_zero() {
                    self.momentum = Some((fling, now, event.clone()));
                }
            }
        }
    }

    pub(crate) fn tick(&mut self, now: Instant) -> Option<ScrollEvent> {
        let (fling, started_at, event) = self.momentum.as_mut()?;
        let elapsed = now.duration_since(*started_at);
        event.delta = ScrollDelta::Pixels(fling.advance(elapsed));
        event.touch_phase = if elapsed >= fling.duration() {
            TouchPhase::Ended
        } else {
            TouchPhase::Moved
        };
        let event = event.clone();
        if event.touch_phase == TouchPhase::Ended {
            self.momentum = None;
        }
        Some(event)
    }

    pub(crate) fn cancel(&mut self) {
        self.position = None;
        self.momentum = None;
    }
}

#[cfg(test)]
mod tests;
