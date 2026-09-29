use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;

use scheduler::Instant;

use super::{HangDetector, HangIncident};

/// How long [`crate::App::shutdown`] waits for the final flush. Quit handlers
/// get [`crate::SHUTDOWN_TIMEOUT`] in total, and the app still has to deliver
/// what the flush produced.
pub(crate) const HANG_MONITOR_FLUSH_TIMEOUT: Duration = Duration::from_millis(100);

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
    Flush { done: mpsc::Sender<()> },
}

/// Polls a [`HangDetector`] on a dedicated OS thread.
///
/// Detection runs off the foreground thread so incidents are still collected
/// and reported while GPUI's executors are stalled. The thread owns the
/// detector and the callback; other threads only send it requests. Dropping
/// the monitor stops the thread.
pub(crate) struct HangMonitor {
    requests: mpsc::Sender<Request>,
}

impl HangMonitor {
    /// Starts a thread that polls `detector` every `interval` and passes each
    /// poll's result, including empty ones, to `on_poll` on that thread.
    ///
    /// # Errors
    ///
    /// Returns an error when the thread can't be spawned.
    pub(crate) fn spawn<F>(
        mut detector: HangDetector,
        interval: Duration,
        mut on_poll: F,
    ) -> std::io::Result<Self>
    where
        F: FnMut(HangMonitorPoll) + Send + 'static,
    {
        let (requests, receiver) = mpsc::channel();
        std::thread::Builder::new()
            .name("HangDetection".to_string())
            .spawn(move || {
                loop {
                    let (reason, done) = match receiver.recv_timeout(interval) {
                        Ok(Request::Flush { done }) => (HangMonitorPollReason::Flush, Some(done)),
                        Err(RecvTimeoutError::Timeout) => (HangMonitorPollReason::Interval, None),
                        Err(RecvTimeoutError::Disconnected) => break,
                    };
                    let incidents = detector.poll();
                    on_poll(HangMonitorPoll {
                        incidents,
                        first_present_at: detector.first_present_at(),
                        reason,
                    });
                    if let Some(done) = done {
                        done.send(()).ok();
                    }
                }
            })?;
        Ok(Self { requests })
    }

    /// Asks the monitor thread to poll now with [`HangMonitorPollReason::Flush`]
    /// and waits up to `timeout` for the callback to finish. Returns whether
    /// it finished in time.
    pub(crate) fn flush(&self, timeout: Duration) -> bool {
        let (done, finished) = mpsc::channel();
        if self.requests.send(Request::Flush { done }).is_err() {
            return false;
        }
        finished.recv_timeout(timeout).is_ok()
    }
}
