//! Post-hoc detection of foreground hangs from the journal's event stream.
//!
//! A hang is any single piece of foreground work — a task poll, an action
//! handler, an input dispatch, or a window draw — that blocked the foreground
//! thread for at least a threshold duration. [`HangDetector`] drains the
//! journal, seals it into [`FrameSnapshot`]s, and reports each snapshot that
//! contains hangs as a [`HangIncident`], so a consumer sees every hang
//! together with everything else the foreground did in the same
//! between-frames interval.

use std::time::Duration;

use scheduler::Instant;

use super::journal::{
    ForegroundEvent, ForegroundEventCollector, FrameSnapshot, IntervalSealer,
};

/// Detects foreground hangs by polling the journal.
///
/// Detection is post-hoc: a hang is reported once the interval containing it
/// seals, which happens at the next draw or after the journal's seal timeout
/// of further foreground activity. Work that never yields back to the
/// foreground is not observed until it does.
pub struct HangDetector {
    collector: ForegroundEventCollector,
    sealer: IntervalSealer,
    threshold: Duration,
}

/// One sealed interval that contained at least one hang.
#[derive(Debug, Clone)]
pub struct HangIncident {
    /// The interval the hangs occurred in, including all non-hang foreground
    /// work recorded alongside them.
    pub snapshot: FrameSnapshot,
    /// The events that blocked the foreground for at least the detector's
    /// threshold, longest first.
    pub contributors: Vec<ForegroundEvent>,
}

impl HangDetector {
    /// Creates a detector reporting foreground work at or above `threshold`.
    /// Only events recorded from this point on are observed.
    pub fn new(threshold: Duration) -> Self {
        Self {
            collector: ForegroundEventCollector::new(),
            sealer: IntervalSealer::new(Instant::now()),
            threshold,
        }
    }

    /// Drains newly recorded events and returns the incidents sealed since
    /// the previous poll.
    pub fn poll(&mut self) -> Vec<HangIncident> {
        let drained = self.collector.collect_unseen();
        self.sealer.note_lost(drained.lost);
        self.sealer
            .push_events(drained.events)
            .into_iter()
            .filter_map(|snapshot| HangIncident::detect(snapshot, self.threshold))
            .collect()
    }
}

impl HangIncident {
    /// Returns an incident when the snapshot contains at least one event
    /// that blocked the foreground for `threshold` or longer.
    pub fn detect(snapshot: FrameSnapshot, threshold: Duration) -> Option<Self> {
        let mut contributors: Vec<ForegroundEvent> = snapshot
            .events
            .iter()
            .filter(|event| event.duration() >= threshold)
            .copied()
            .collect();
        if contributors.is_empty() {
            return None;
        }
        contributors.sort_by_key(|event| std::cmp::Reverse(event.duration()));
        Some(Self {
            snapshot,
            contributors,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::thread;
    use std::time::Duration;

    use rand::prelude::*;
    use scheduler::SpawnTime;

    use crate::{
        self as gpui, Context, FocusHandle, InteractiveElement, IntoElement, Modifiers,
        MouseButton, Render, Styled, TestAppContext, VisualTestContext, Window, div, point, px,
    };

    use super::super::journal::ForegroundEvent;
    use super::HangDetector;

    actions!(hang_test, [HangyAction]);

    // Well above legitimate per-event work in a test app (layout of one div,
    // empty polls), well below the injected hangs.
    const HANG_THRESHOLD: Duration = Duration::from_millis(10);

    #[derive(Clone, Default)]
    struct HangControls {
        render: Rc<Cell<Option<Duration>>>,
        input: Rc<Cell<Option<Duration>>>,
        action: Rc<Cell<Option<Duration>>>,
    }

    struct HangyView {
        controls: HangControls,
        focus_handle: FocusHandle,
    }

    impl Render for HangyView {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            if let Some(duration) = self.controls.render.take() {
                thread::sleep(duration);
            }
            let action_controls = self.controls.clone();
            let input_controls = self.controls.clone();
            div()
                .size_full()
                .track_focus(&self.focus_handle)
                .on_action(cx.listener(move |_, _: &HangyAction, _, _| {
                    if let Some(duration) = action_controls.action.take() {
                        thread::sleep(duration);
                    }
                }))
                .on_mouse_down(MouseButton::Left, move |_, _, _| {
                    if let Some(duration) = input_controls.input.take() {
                        thread::sleep(duration);
                    }
                })
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum HangKind {
        Render,
        Input,
        Action,
        Poll,
    }

    /// Renders a real element tree and injects randomly ordered, randomly
    /// sized hangs through the production dispatch paths: view rendering
    /// (window draw), platform input dispatch, action dispatch, and a
    /// foreground task poll. Asserts the detector reports every one of them.
    #[gpui::test(iterations = 5)]
    fn detects_randomly_placed_foreground_hangs(mut rng: StdRng, cx: &mut TestAppContext) {
        // Created first: the detector only observes events recorded after
        // this point, which isolates concurrently running tests' iterations.
        let mut detector = HangDetector::new(HANG_THRESHOLD);

        let controls = HangControls::default();
        let (view, cx) = cx.add_window_view(|_, cx| HangyView {
            controls: controls.clone(),
            focus_handle: cx.focus_handle(),
        });
        view.update_in(cx, |view, window, cx| {
            window.focus(&view.focus_handle, cx);
        });
        draw_window(cx);

        let mut injected: Vec<(HangKind, Duration)> = Vec::new();
        for _ in 0..rng.random_range(1..=4) {
            let duration = HANG_THRESHOLD + Duration::from_millis(rng.random_range(5..25));
            let kind = match rng.random_range(0..4) {
                0 => {
                    controls.render.set(Some(duration));
                    view.update_in(cx, |_, _, cx| cx.notify());
                    draw_window(cx);
                    HangKind::Render
                }
                1 => {
                    controls.input.set(Some(duration));
                    cx.simulate_mouse_down(point(px(5.), px(5.)), MouseButton::Left, Modifiers::none());
                    HangKind::Input
                }
                2 => {
                    controls.action.set(Some(duration));
                    cx.dispatch_action(HangyAction);
                    HangKind::Action
                }
                _ => {
                    simulate_blocked_foreground_poll(duration);
                    HangKind::Poll
                }
            };
            injected.push((kind, duration));

            // Innocent interleaved activity that must not confuse detection.
            if rng.random_bool(0.5) {
                draw_window(cx);
            }
            if rng.random_bool(0.5) {
                cx.simulate_mouse_up(point(px(5.), px(5.)), MouseButton::Left, Modifiers::none());
            }
        }

        // A final draw seals whatever interval is still open.
        draw_window(cx);

        let incidents = detector.poll();
        let contributors: Vec<ForegroundEvent> = incidents
            .iter()
            .flat_map(|incident| incident.contributors.iter().copied())
            .collect();

        for kind in [
            HangKind::Render,
            HangKind::Input,
            HangKind::Action,
            HangKind::Poll,
        ] {
            let expected: Vec<Duration> = injected
                .iter()
                .filter(|(injected_kind, _)| *injected_kind == kind)
                .map(|(_, duration)| *duration)
                .collect();
            let observed: Vec<Duration> = contributors
                .iter()
                .filter(|event| matches_kind(event, kind))
                .map(|event| event.duration())
                .collect();
            assert_all_matched(kind, expected, observed);
        }
    }

    /// Every injected hang must be covered by a distinct observed contributor
    /// at least as long as the injected sleep (sleeps never wake early).
    /// Extra observed contributors are allowed: other threads journaling
    /// concurrently is not a detection failure.
    fn assert_all_matched(kind: HangKind, mut expected: Vec<Duration>, mut observed: Vec<Duration>) {
        expected.sort_unstable_by(|a, b| b.cmp(a));
        observed.sort_unstable_by(|a, b| b.cmp(a));
        let mut observed = observed.into_iter();
        for expected_duration in expected {
            let matched = observed.find(|observed| *observed >= expected_duration);
            assert!(
                matched.is_some(),
                "injected {kind:?} hang of {expected_duration:?} was not detected"
            );
        }
    }

    fn matches_kind(event: &ForegroundEvent, kind: HangKind) -> bool {
        match (event, kind) {
            (ForegroundEvent::Draw(_), HangKind::Render) => true,
            (ForegroundEvent::Input(_), HangKind::Input) => true,
            (ForegroundEvent::Action(timing), HangKind::Action) => {
                timing.name.ends_with("HangyAction")
            }
            (ForegroundEvent::TaskPoll(timing), HangKind::Poll) => {
                timing.location.file() == file!()
            }
            _ => false,
        }
    }

    fn draw_window(cx: &mut VisualTestContext) {
        cx.update(|window, cx| window.draw(cx).clear(cx));
    }

    /// The deterministic test scheduler does not bracket runnables with the
    /// profiler hooks, so drive the same public hooks the platform
    /// dispatchers call around a poll that blocks the foreground.
    fn simulate_blocked_foreground_poll(duration: Duration) {
        let location = std::panic::Location::caller();
        crate::profiler::update_running_task(SpawnTime(scheduler::Instant::now()), location);
        thread::sleep(duration);
        crate::profiler::save_task_timing();
    }
}
