use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;

use futures::channel::oneshot;
use scheduler::Instant;

use super::watchdog::Watchdog;
use super::{HangDetector, HangIncident, HangTrigger};
use crate::profiler::journal::ForegroundEvent;

/// Detection thresholds and polling cadence for
/// [`crate::App::start_hang_monitor`].
#[derive(Debug, Clone, Copy)]
pub struct HangMonitorConfig {
    /// Duration at which a single piece of foreground work counts as a hang.
    pub threshold: Duration,
    /// Total foreground spend within one interval that counts as a hang.
    pub frame_budget: Duration,
    /// How often the monitor thread drains the journal.
    pub interval: Duration,
}

/// Why [`crate::App::start_hang_monitor`] failed.
#[derive(Debug, thiserror::Error)]
pub enum HangMonitorError {
    /// The monitor was already started for this app.
    #[error("the hang monitor was already started")]
    AlreadyStarted,
    /// The monitor thread couldn't be spawned.
    #[error("failed to spawn the hang monitor thread: {0}")]
    Spawn(#[source] std::io::Error),
}

/// Why a [`HangMonitor`] polled its detector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HangMonitorPollReason {
    /// The monitor's regular interval elapsed.
    Interval,
    /// A poll was requested before the application quits. Consumers should
    /// deliver any batched results now.
    Flush,
}

/// The incidents a [`HangMonitor`] collected in one poll.
pub struct HangMonitorPoll {
    /// Incidents sealed since the previous poll, possibly none.
    pub incidents: Vec<HangIncident>,
    /// See [`HangDetector::first_present_at`].
    pub first_present_at: Option<Instant>,
    /// Why this poll happened.
    pub reason: HangMonitorPollReason,
}

enum Request {
    Flush { done: oneshot::Sender<()> },
}

/// Polls a [`HangDetector`] on a dedicated OS thread.
///
/// Detection runs off the foreground thread so incidents are still collected
/// and reported while GPUI's executors are stalled. The thread owns the
/// detector, the optional [`Watchdog`] that profiles each stall, and the
/// callback; other threads only send it requests. Dropping the monitor stops
/// the thread.
pub(crate) struct HangMonitor {
    /// `None` once dropping, so the thread sees the channel disconnect.
    requests: Option<mpsc::Sender<Request>>,
    thread: std::thread::Thread,
}

impl HangMonitor {
    /// Starts a thread that polls `detector` every `interval` and passes each
    /// poll's result, including empty ones, to `on_poll` on that thread.
    /// With a `watchdog`, the thread also samples the foreground between
    /// polls and attaches a [`super::StallProfile`] to each threshold
    /// incident.
    ///
    /// # Errors
    ///
    /// Returns an error when the thread can't be spawned.
    pub(crate) fn spawn<F>(
        detector: HangDetector,
        watchdog: Option<Watchdog>,
        interval: Duration,
        on_poll: F,
    ) -> std::io::Result<Self>
    where
        F: FnMut(HangMonitorPoll) + Send + 'static,
    {
        let (requests, receiver) = mpsc::channel();
        let thread = std::thread::Builder::new()
            .name("HangDetection".to_string())
            .spawn(move || run(detector, watchdog, interval, on_poll, receiver))?
            .thread()
            .clone();
        Ok(Self {
            requests: Some(requests),
            thread,
        })
    }

    /// Asks the monitor thread to poll now with [`HangMonitorPollReason::Flush`]
    /// without waiting for it. The returned receiver completes once the
    /// callback has returned. Returns `None` if the thread has exited.
    pub(crate) fn request_flush(&self) -> Option<oneshot::Receiver<()>> {
        let (done, finished) = oneshot::channel();
        self.requests.as_ref()?.send(Request::Flush { done }).ok()?;
        self.thread.unpark();
        Some(finished)
    }
}

impl Drop for HangMonitor {
    fn drop(&mut self) {
        self.requests = None;
        self.thread.unpark();
    }
}

fn run(
    mut detector: HangDetector,
    mut watchdog: Option<Watchdog>,
    interval: Duration,
    mut on_poll: impl FnMut(HangMonitorPoll),
    requests: mpsc::Receiver<Request>,
) {
    let mut next_poll = Instant::now() + interval;
    loop {
        let request = match requests.try_recv() {
            Ok(request) => Some(request),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => break,
        };
        let now = Instant::now();
        let (reason, done) = match request {
            Some(Request::Flush { done }) => (HangMonitorPollReason::Flush, Some(done)),
            None if now >= next_poll => (HangMonitorPollReason::Interval, None),
            None => {
                match watchdog.as_mut() {
                    Some(watchdog) => {
                        watchdog.wait(next_poll);
                        watchdog.sample(Instant::now());
                    }
                    None => std::thread::park_timeout(next_poll.saturating_duration_since(now)),
                }
                continue;
            }
        };
        if reason == HangMonitorPollReason::Interval {
            next_poll = now + interval;
        }

        let mut incidents = detector.poll();
        if let Some(watchdog) = watchdog.as_mut() {
            // Sampled after the drain, so every drained stall has ended by
            // the latest sample.
            watchdog.sample(Instant::now());
            for incident in &mut incidents {
                incident.stall_profile = profile_stall(watchdog, incident);
            }
        }
        on_poll(HangMonitorPoll {
            incidents,
            first_present_at: detector.first_present_at(),
            reason,
        });
        if let Some(done) = done {
            done.send(()).ok();
        }
    }
}

/// Profiles the incident's longest stall, when it crossed the threshold.
/// Budget incidents are made of work too short to sample.
fn profile_stall(watchdog: &Watchdog, incident: &HangIncident) -> Option<super::StallProfile> {
    if incident.trigger != HangTrigger::Threshold {
        return None;
    }
    let stall = incident.contributors.first()?;
    watchdog.profile(
        stall.start_time(),
        stall.end_time(),
        matches!(stall, ForegroundEvent::Present(_)),
    )
}
