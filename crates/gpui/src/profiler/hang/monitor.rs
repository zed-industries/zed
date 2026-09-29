use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;

use scheduler::Instant;

use super::{HangDetector, HangIncident};

/// Why a [`HangMonitor`] polled its detector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HangMonitorPollReason {
    /// The monitor's regular interval elapsed.
    Interval,
    /// [`HangMonitor::flush`] requested a poll, e.g. because the application
    /// is quitting. Consumers should deliver any batched results now.
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
pub struct HangMonitor {
    requests: mpsc::Sender<Request>,
}

impl HangMonitor {
    /// Starts a thread that polls `detector` every `interval` and passes each
    /// poll's result, including empty ones, to `on_poll` on that thread.
    ///
    /// # Errors
    ///
    /// Returns an error when the thread can't be spawned.
    pub fn spawn<F>(
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
    pub fn flush(&self, timeout: Duration) -> bool {
        let (done, finished) = mpsc::channel();
        if self.requests.send(Request::Flush { done }).is_err() {
            return false;
        }
        finished.recv_timeout(timeout).is_ok()
    }
}
