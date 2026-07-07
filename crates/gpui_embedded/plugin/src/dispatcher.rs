use crate::wit;
use gpui::{PlatformDispatcher, Priority, RunnableVariant};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

#[derive(Default)]
struct DispatcherState {
    runnables: VecDeque<RunnableVariant>,
    timers: Vec<(Instant, RunnableVariant)>,
}

/// A single-threaded scheduler that is pumped by the host through the `tick` export.
///
/// The guest never blocks; instead it queues work locally and asks the host for a wakeup via
/// the `request-tick` import. `run_until_idle` drains the queues and is driven from `tick`.
pub struct PluginDispatcher {
    state: Mutex<DispatcherState>,
    wakeups_suppressed: AtomicBool,
}

impl PluginDispatcher {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(DispatcherState::default()),
            wakeups_suppressed: AtomicBool::new(false),
        }
    }

    /// While suppressed, dispatches don't ask the host for a wakeup. `pump` drains everything
    /// queued during the current host call anyway, so a wakeup would only cause a no-op tick.
    pub fn set_wakeups_suppressed(&self, suppressed: bool) {
        self.wakeups_suppressed.store(suppressed, Ordering::Relaxed);
    }

    fn request_wakeup(&self, delay_ms: u32) {
        if !self.wakeups_suppressed.load(Ordering::Relaxed) {
            wit::request_tick(delay_ms);
        }
    }

    fn lock(&self) -> MutexGuard<'_, DispatcherState> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Run every due timer and then every queued runnable, repeating until nothing remains.
    /// Runnables and timers may enqueue further work, so this loops until a pass does nothing.
    pub fn run_until_idle(&self) {
        loop {
            let now = Instant::now();
            let (due_timers, runnable) = {
                let mut state = self.lock();
                let mut due = Vec::new();
                let mut pending = Vec::new();
                for (deadline, runnable) in std::mem::take(&mut state.timers) {
                    if deadline <= now {
                        due.push(runnable);
                    } else {
                        pending.push((deadline, runnable));
                    }
                }
                state.timers = pending;
                let runnable = state.runnables.pop_front();
                (due, runnable)
            };

            let mut ran_any = false;
            for timer in due_timers {
                timer.run();
                ran_any = true;
            }
            if let Some(runnable) = runnable {
                runnable.run();
                ran_any = true;
            }
            if !ran_any {
                break;
            }
        }
    }

    /// The delay until the earliest pending timer, so the caller can schedule the next wakeup.
    pub fn next_timer_delay(&self) -> Option<Duration> {
        let state = self.lock();
        let now = Instant::now();
        state
            .timers
            .iter()
            .map(|(deadline, _)| deadline.saturating_duration_since(now))
            .min()
    }
}

impl PlatformDispatcher for PluginDispatcher {
    fn is_main_thread(&self) -> bool {
        true
    }

    fn dispatch(&self, runnable: RunnableVariant, _priority: Priority) {
        self.lock().runnables.push_back(runnable);
        self.request_wakeup(0);
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        self.lock().runnables.push_back(runnable);
        self.request_wakeup(0);
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let deadline = Instant::now() + duration;
        self.lock().timers.push((deadline, runnable));
        let millis = duration.as_millis().min(u32::MAX as u128) as u32;
        self.request_wakeup(millis);
    }

    fn spawn_realtime(&self, function: Box<dyn FnOnce() + Send>) {
        function();
    }
}
