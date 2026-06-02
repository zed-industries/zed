use std::{
    hint::cold_path,
    time::{Duration, Instant},
};

use itertools::Itertools;

use crate::action::Action;

#[doc(hidden)]
#[derive(Clone)]
pub struct ActionStatistics {
    runtime_to_beat: Duration,

    longest_runtimes: heapless::Vec<ActionTiming, 5>,
    running: Option<(&'static str, Instant)>,
}

impl std::fmt::Debug for ActionStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionStatistics")
            .field("runtime_to_beat", &self.runtime_to_beat)
            .field("longest_runtimes", &self.longest_runtimes)
            .field(
                "running",
                &self.running.map(|(id, started)| (id, started.elapsed())),
            )
            .finish()
    }
}

impl std::fmt::Display for ActionStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Actions that blocked the longest\n")?;
        for action in self
            .longest_runtimes(true)
            .sorted_by_key(|action| action.runtime())
            .rev()
        {
            f.write_fmt(format_args!(
                "{:<20} - {}",
                format!("{:?}", action.runtime()), // impl dbg does not support alignment
                action.name
            ))?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for ActionStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionStatistics {
    const fn new() -> Self {
        Self {
            // This keeps more calls on the fast path by only tracking
            // problematic polls
            runtime_to_beat: Duration::from_micros(100),
            longest_runtimes: heapless::Vec::new(),
            running: None,
        }
    }

    pub fn take(&mut self) -> Self {
        let taken = std::mem::take(self);
        self.running = taken.running;
        taken
    }

    pub fn is_empty(&self) -> bool {
        self.longest_runtimes.is_empty()
    }

    pub fn update_running_action(&mut self, action: &'static str, started: Instant) {
        self.running = Some((action, started));
    }

    pub fn save_action_timing(&mut self) {
        let now = Instant::now();
        let (action, started) = self
            .running
            .take()
            .expect("only called after `update_running_action`");

        let runtime = now.duration_since(started);
        if runtime >= self.runtime_to_beat {
            cold_path(); // most actions are not the worst, optimize for that

            if self.longest_runtimes.is_full()
                && let Some(to_replace) = self
                    .longest_runtimes
                    .iter_mut()
                    .min_by_key(|action| runtime >= action.runtime())
            {
                *to_replace = ActionTiming {
                    name: action,
                    start: started,
                    end: now,
                };
            } else {
                self.longest_runtimes
                    .push(ActionTiming {
                        name: action,
                        start: started,
                        end: now,
                    })
                    .expect("just checked it is not full");
            };

            self.runtime_to_beat = self
                .longest_runtimes
                .iter()
                .map(|action| action.runtime())
                .min()
                .expect("never empty");
        }
    }

    pub fn longest_runtimes(&self, include_running: bool) -> impl Iterator<Item = ActionTiming> {
        self.longest_runtimes.iter().copied().chain(
            self.running
                .into_iter()
                .filter(move |_| include_running)
                .map(|(name, start)| ActionTiming {
                    name,
                    start,
                    end: Instant::now(),
                }),
        )
    }
}

#[doc(hidden)]
/// UNSTABLE only for use in the profiler and zed-reliability
#[derive(Copy, Clone)]
pub struct ActionTiming {
    pub name: &'static str,
    pub start: Instant,
    pub end: Instant,
}

impl core::fmt::Debug for ActionTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionTiming")
            .field("name", &self.name)
            .field("runtime", &self.runtime())
            .finish()
    }
}

impl ActionTiming {
    pub fn duration(&self) -> Duration {
        self.end.saturating_duration_since(self.start)
    }
}

impl ActionTiming {
    #[doc(hidden)]
    pub fn runtime(&self) -> Duration {
        self.end - self.start
    }
}

// The profiler is careful to never block when the lock is held, therefore a
// spinlock is optimal.
static ACTION_STATISTICS: spin::Mutex<ActionStatistics> =
    const { spin::Mutex::new(ActionStatistics::new()) };

#[doc(hidden)]
pub(crate) fn update_running_action(action: &(dyn Action + 'static), cx: &mut crate::App) {
    let now = Instant::now();
    let action = action.type_id();
    let action = cx.actions.try_resolve_action(&action).unwrap_or("un-named");
    ACTION_STATISTICS.lock().update_running_action(action, now);
}

#[doc(hidden)]
pub(crate) fn save_action_timing() {
    ACTION_STATISTICS.lock().save_action_timing();
}

#[doc(hidden)]
pub fn take_action_stats() -> ActionStatistics {
    ACTION_STATISTICS.lock().take()
}
