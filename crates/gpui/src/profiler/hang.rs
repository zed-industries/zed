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
use serde::Serialize;

use super::SerializedLocation;
use super::journal::{ForegroundEvent, ForegroundEventCollector, FrameSnapshot, IntervalSealer};

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

/// A [`HangIncident`] in a telemetry-friendly form: timestamps in
/// microseconds since app startup, locations as plain data, contributor
/// count capped by the converter.
#[derive(Debug, Clone, Serialize)]
pub struct SerializedHangIncident {
    /// When the interval started, in microseconds since app startup.
    pub start: u64,
    /// Length of the interval in microseconds.
    pub duration_us: u64,
    /// Why the interval sealed: `"draw"` or `"timeout"`.
    pub reason: &'static str,
    /// Fraction of the interval the foreground spent working, `0.0..=1.0`.
    pub busy_fraction: f64,
    /// Total events recorded in the interval (before the contributor cap).
    pub event_count: usize,
    /// Count of task polls below the journal's floor.
    pub small_poll_count: u64,
    /// Total duration of task polls below the journal's floor, in microseconds.
    pub small_poll_total_us: u64,
    /// Events lost to caps or ring overwrites.
    pub dropped_events: u64,
    /// Contributors at or above the hang threshold, longest first, capped.
    pub contributors: Vec<SerializedHangContributor>,
    /// Contributors elided by the cap.
    pub contributors_elided: usize,
}

/// One hang contributor in serialized form.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SerializedHangContributor {
    /// A foreground task poll.
    TaskPoll {
        /// Where the task was spawned.
        location: SerializedLocation,
        /// When the poll started, in microseconds since app startup.
        start: u64,
        /// How long the poll blocked the foreground, in microseconds.
        duration_us: u64,
    },
    /// An action handler.
    Action {
        /// The action's name.
        name: &'static str,
        /// When the handler started, in microseconds since app startup.
        start: u64,
        /// How long the handler ran, in microseconds.
        duration_us: u64,
    },
    /// A platform input dispatch.
    Input {
        /// When the dispatch started, in microseconds since app startup.
        start: u64,
        /// How long the dispatch ran, in microseconds.
        duration_us: u64,
        /// Whether handling the input invalidated a window.
        caused_invalidation: bool,
    },
    /// A window draw.
    Draw {
        /// The window that was drawn.
        window_id: u64,
        /// When the draw started, in microseconds since app startup.
        start: u64,
        /// How long the draw took, in microseconds.
        duration_us: u64,
        /// Time from the frame's first invalidation to the end of its draw.
        dirty_to_draw_us: Option<u64>,
        /// Invalidations coalesced into the frame.
        invalidations: u64,
    },
    /// A frame presentation (zero duration; present only when a present
    /// itself is somehow a contributor).
    Present {
        /// The window whose frame was presented.
        window_id: u64,
        /// When the frame was presented, in microseconds since app startup.
        start: u64,
    },
}

impl SerializedHangIncident {
    /// Converts an incident, keeping at most `max_contributors` contributors.
    pub fn convert(startup: Instant, incident: &HangIncident, max_contributors: usize) -> Self {
        let since_startup =
            |instant: Instant| instant.saturating_duration_since(startup).as_micros() as u64;
        let snapshot = &incident.snapshot;
        Self {
            start: since_startup(snapshot.interval_start),
            duration_us: snapshot
                .interval_end
                .duration_since(snapshot.interval_start)
                .as_micros() as u64,
            reason: match snapshot.reason {
                super::journal::SealReason::Draw => "draw",
                super::journal::SealReason::Timeout => "timeout",
            },
            busy_fraction: snapshot.busy_fraction(),
            event_count: snapshot.events.len(),
            small_poll_count: snapshot.small_polls.count,
            small_poll_total_us: snapshot.small_polls.total.as_micros() as u64,
            dropped_events: snapshot.dropped_events,
            contributors: incident
                .contributors
                .iter()
                .take(max_contributors)
                .map(|event| SerializedHangContributor::convert(startup, event))
                .collect(),
            contributors_elided: incident.contributors.len().saturating_sub(max_contributors),
        }
    }
}

impl SerializedHangContributor {
    fn convert(startup: Instant, event: &ForegroundEvent) -> Self {
        let since_startup =
            |instant: Instant| instant.saturating_duration_since(startup).as_micros() as u64;
        let duration_us = event.duration().as_micros() as u64;
        match event {
            ForegroundEvent::TaskPoll(timing) => Self::TaskPoll {
                location: timing.location.into(),
                start: since_startup(timing.start),
                duration_us,
            },
            ForegroundEvent::Action(timing) => Self::Action {
                name: timing.name,
                start: since_startup(timing.start),
                duration_us,
            },
            ForegroundEvent::Input(timing) => Self::Input {
                start: since_startup(timing.start),
                duration_us,
                caused_invalidation: timing.caused_invalidation,
            },
            ForegroundEvent::Draw(timing) => Self::Draw {
                window_id: timing.window_id.as_u64(),
                start: since_startup(timing.draw_start),
                duration_us,
                dirty_to_draw_us: timing
                    .dirty_to_draw_duration()
                    .map(|duration| duration.as_micros() as u64),
                invalidations: timing.invalidations,
            },
            ForegroundEvent::Present(timing) => Self::Present {
                window_id: timing.window_id.as_u64(),
                start: since_startup(timing.presented_at),
            },
            ForegroundEvent::SmallPolls(flush) => {
                // The sealer folds these out of snapshot events; a contributor
                // can therefore never be one, but serialize defensively as an
                // unnamed poll spanning the flush rather than panicking.
                Self::TaskPoll {
                    location: SerializedLocation {
                        file: "<small poll summary>".into(),
                        line: 0,
                        column: 0,
                    },
                    start: since_startup(flush.since),
                    duration_us,
                }
            }
        }
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
