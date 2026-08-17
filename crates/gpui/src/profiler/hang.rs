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
    first_frame_at: Option<Instant>,
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
            first_frame_at: None,
        }
    }

    /// When the first window draw observed by this detector finished, marking
    /// the end of the pre-first-frame startup phase. `None` until a draw has
    /// been observed.
    pub fn first_frame_at(&self) -> Option<Instant> {
        self.first_frame_at
    }

    /// Drains newly recorded events and returns the incidents sealed since
    /// the previous poll.
    pub fn poll(&mut self) -> Vec<HangIncident> {
        let drained = self.collector.collect_unseen();
        if self.first_frame_at.is_none() {
            self.first_frame_at = drained.events.iter().find_map(|event| match event {
                ForegroundEvent::Draw(timing) => Some(timing.draw_end),
                _ => None,
            });
        }
        self.sealer.note_lost(drained.lost);
        self.sealer
            .push_events(drained.events)
            .into_iter()
            .filter_map(|snapshot| HangIncident::detect(snapshot, self.threshold))
            .collect()
    }
}

/// A [`HangIncident`] in a telemetry-friendly form: timestamps and durations
/// in fractional milliseconds since app startup (microsecond precision),
/// locations as plain data, contributor count capped by the converter.
#[derive(Debug, Clone, Serialize)]
pub struct SerializedHangIncident {
    /// `"startup"` when the active window began before the first observed
    /// window frame finished drawing (see [`HangDetector::first_frame_at`]),
    /// otherwise `"steady"`. Separates pre-first-paint hangs from hangs of a
    /// visible app without suppressing either.
    pub phase: &'static str,
    /// When the incident's active window started, in milliseconds since app
    /// startup: the sealing frame's first invalidation, or the earliest
    /// contributor's start when nothing was pending a repaint. Foreground
    /// idle time between the previous frame and the cause is excluded.
    pub start_ms: f64,
    /// Length of the active window in milliseconds: from the cause to the
    /// seal. Exceeds `stall_ms` when several stalls piled up on one frame.
    pub active_ms: f64,
    /// The longest single block of foreground work, in milliseconds: the
    /// best estimate of the freeze a user perceived. Always the duration of
    /// the first contributor.
    pub stall_ms: f64,
    /// For draw-sealed incidents, how long the frame that closed the
    /// incident had been dirty before reaching the screen, in milliseconds:
    /// how long a needed repaint kept the user waiting.
    pub dirty_to_draw_ms: Option<f64>,
    /// What closed the incident: `"draw"` (a frame was produced) or
    /// `"timeout"` (nothing drew for the seal timeout). This labels the
    /// boundary, not the hang's cause — the cause is the first contributor.
    pub sealed_by: &'static str,
    /// Fraction of the active window the foreground spent working,
    /// `0.0..=1.0`. Low values with a high `dirty_to_draw_ms` indicate
    /// throttling or scheduling delay rather than application work.
    pub busy_fraction: f64,
    /// Total events recorded in the interval (before the contributor cap).
    pub event_count: usize,
    /// Count of task polls below the journal's floor.
    pub small_poll_count: u64,
    /// Total duration of task polls below the journal's floor, in milliseconds.
    pub small_poll_total_ms: f64,
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
        /// When the poll started, in milliseconds since app startup.
        start_ms: f64,
        /// How long the poll blocked the foreground, in milliseconds.
        duration_ms: f64,
    },
    /// An action handler.
    Action {
        /// The action's name.
        name: &'static str,
        /// When the handler started, in milliseconds since app startup.
        start_ms: f64,
        /// How long the handler ran, in milliseconds.
        duration_ms: f64,
    },
    /// A platform input dispatch.
    Input {
        /// When the dispatch started, in milliseconds since app startup.
        start_ms: f64,
        /// How long the dispatch ran, in milliseconds.
        duration_ms: f64,
        /// Whether handling the input invalidated a window.
        caused_invalidation: bool,
    },
    /// A window draw.
    Draw {
        /// The window that was drawn.
        window_id: u64,
        /// When the draw started, in milliseconds since app startup.
        start_ms: f64,
        /// How long the draw took, in milliseconds.
        duration_ms: f64,
        /// Time from the frame's first invalidation to the end of its draw,
        /// in milliseconds.
        dirty_to_draw_ms: Option<f64>,
        /// Invalidations coalesced into the frame.
        invalidations: u64,
    },
    /// A frame presentation (zero duration; present only when a present
    /// itself is somehow a contributor).
    Present {
        /// The window whose frame was presented.
        window_id: u64,
        /// When the frame was presented, in milliseconds since app startup.
        start_ms: f64,
    },
}

/// Milliseconds with microsecond precision: keeps `dbg!`/JSON output short
/// (`115.954` rather than `115.95400000000001` or `115954`).
fn as_millis(duration: Duration) -> f64 {
    duration.as_micros() as f64 / 1000.0
}

impl SerializedHangIncident {
    /// Converts an incident, keeping at most `max_contributors` contributors.
    /// `first_frame_at` is the end of the first observed window draw
    /// (typically [`HangDetector::first_frame_at`]); incidents whose active
    /// window begins before it (or before any frame exists) are tagged with
    /// the `"startup"` phase.
    pub fn convert(
        startup: Instant,
        incident: &HangIncident,
        max_contributors: usize,
        first_frame_at: Option<Instant>,
    ) -> Self {
        let since_startup =
            |instant: Instant| as_millis(instant.saturating_duration_since(startup));
        let snapshot = &incident.snapshot;
        let (active_start, active_end) = incident.active_window();
        let active = active_end.duration_since(active_start);
        let busy_fraction = if active.is_zero() {
            1.0
        } else {
            snapshot
                .occupancy_within(active_start, active_end)
                .div_duration_f64(active)
                .min(1.0)
        };
        Self {
            phase: match first_frame_at {
                Some(first_frame_at) if active_start >= first_frame_at => "steady",
                _ => "startup",
            },
            start_ms: since_startup(active_start),
            active_ms: as_millis(active),
            stall_ms: incident
                .contributors
                .first()
                .map(|event| as_millis(event.duration()))
                .unwrap_or(0.0),
            dirty_to_draw_ms: match snapshot.events.last() {
                Some(ForegroundEvent::Draw(timing)) => {
                    timing.dirty_to_draw_duration().map(as_millis)
                }
                _ => None,
            },
            sealed_by: match snapshot.reason {
                super::journal::SealReason::Draw => "draw",
                super::journal::SealReason::Timeout => "timeout",
            },
            busy_fraction: (busy_fraction * 1000.0).round() / 1000.0,
            event_count: snapshot.events.len(),
            small_poll_count: snapshot.small_polls.count,
            small_poll_total_ms: as_millis(snapshot.small_polls.total),
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
            |instant: Instant| as_millis(instant.saturating_duration_since(startup));
        let duration_ms = as_millis(event.duration());
        match event {
            ForegroundEvent::TaskPoll(timing) => Self::TaskPoll {
                location: timing.location.into(),
                start_ms: since_startup(timing.start),
                duration_ms,
            },
            ForegroundEvent::Action(timing) => Self::Action {
                name: timing.name,
                start_ms: since_startup(timing.start),
                duration_ms,
            },
            ForegroundEvent::Input(timing) => Self::Input {
                start_ms: since_startup(timing.start),
                duration_ms,
                caused_invalidation: timing.caused_invalidation,
            },
            ForegroundEvent::Draw(timing) => Self::Draw {
                window_id: timing.window_id.as_u64(),
                start_ms: since_startup(timing.draw_start),
                duration_ms,
                dirty_to_draw_ms: timing.dirty_to_draw_duration().map(as_millis),
                invalidations: timing.invalidations,
            },
            ForegroundEvent::Present(timing) => Self::Present {
                window_id: timing.window_id.as_u64(),
                start_ms: since_startup(timing.presented_at),
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
                    start_ms: since_startup(flush.since),
                    duration_ms,
                }
            }
        }
    }
}

impl HangIncident {
    /// The incident's reporting window: from its earliest cause — the
    /// sealing frame's first invalidation, or the earliest contributor's
    /// start when no repaint was pending — to the seal. This trims
    /// foreground-idle time between the previous frame and the cause, and
    /// may begin before the underlying snapshot when a contributor was
    /// already running at the previous seal.
    pub fn active_window(&self) -> (Instant, Instant) {
        let snapshot = &self.snapshot;
        let dirty_at = match snapshot.events.last() {
            Some(ForegroundEvent::Draw(timing)) => timing.dirty_at,
            _ => None,
        };
        let earliest_contributor = self
            .contributors
            .iter()
            .map(|event| event.start_time())
            .min();
        let start = [dirty_at, earliest_contributor]
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(snapshot.interval_start);
        (start, snapshot.interval_end)
    }

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
                    cx.simulate_mouse_down(
                        point(px(5.), px(5.)),
                        MouseButton::Left,
                        Modifiers::none(),
                    );
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
    fn assert_all_matched(
        kind: HangKind,
        mut expected: Vec<Duration>,
        mut observed: Vec<Duration>,
    ) {
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
