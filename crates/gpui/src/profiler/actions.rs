#[cfg(feature = "profiler")]
use std::sync::LazyLock;
use std::time::{Duration, Instant};

use itertools::Itertools;
use smallvec::SmallVec;

#[cfg(feature = "profiler")]
use crate::App;
use crate::DispatchPhase;
#[cfg(feature = "profiler")]
use crate::action::Action;

#[derive(Clone, Copy, Debug)]
struct RunningAction {
    name: &'static str,
    phase: DispatchPhase,
    source_location: &'static std::panic::Location<'static>,
    started_at: Instant,
}

#[doc(hidden)]
#[derive(Clone)]
pub struct ActionStatistics {
    runtime_to_beat: Duration,

    longest_runtimes: heapless::Vec<ActionTiming, 5>,
    running: SmallVec<[RunningAction; 4]>,
}

impl std::fmt::Debug for ActionStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionStatistics")
            .field("runtime_to_beat", &self.runtime_to_beat)
            .field("longest_runtimes", &self.longest_runtimes)
            .field(
                "running",
                &self
                    .running
                    .iter()
                    .map(|action| {
                        (
                            action.name,
                            action.phase,
                            action.source_location,
                            action.started_at.elapsed(),
                        )
                    })
                    .collect::<Vec<_>>(),
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
    fn new() -> Self {
        Self {
            // This keeps more calls on the fast path by only tracking
            // problematic polls
            runtime_to_beat: Duration::from_micros(100),
            longest_runtimes: heapless::Vec::new(),
            running: SmallVec::new(),
        }
    }

    pub fn take(&mut self) -> Self {
        let taken = std::mem::take(self);
        self.running = taken.running.clone();
        taken
    }

    pub fn is_empty(&self) -> bool {
        self.longest_runtimes.is_empty()
    }

    pub fn begin_action_handler(
        &mut self,
        name: &'static str,
        phase: DispatchPhase,
        source_location: &'static std::panic::Location<'static>,
    ) {
        self.running.push(RunningAction {
            name,
            phase,
            source_location,
            started_at: Instant::now(),
        });
    }

    pub fn end_action_handler(&mut self) {
        let Some(action) = self.running.pop() else {
            std::hint::cold_path();
            debug_assert!(false, "an action handler must be running before it ends");
            return;
        };

        let timing = ActionTiming {
            name: action.name,
            phase: action.phase,
            source_location: action.source_location,
            start: action.started_at,
            end: Instant::now(),
        };
        let runtime = timing.runtime();
        if runtime >= self.runtime_to_beat {
            std::hint::cold_path(); // most actions are not the worst, optimize for that

            if self.longest_runtimes.is_full()
                && let Some(to_replace) = self
                    .longest_runtimes
                    .iter_mut()
                    .min_by_key(|action| runtime >= action.runtime())
            {
                *to_replace = timing;
            } else {
                self.longest_runtimes
                    .push(timing)
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
        let now = Instant::now();
        self.longest_runtimes.iter().copied().chain(
            self.running
                .iter()
                .filter(move |_| include_running)
                .map(move |action| ActionTiming {
                    name: action.name,
                    phase: action.phase,
                    source_location: action.source_location,
                    start: action.started_at,
                    end: now,
                }),
        )
    }
}

#[doc(hidden)]
/// UNSTABLE only for use in the profiler and zed-reliability
#[derive(Copy, Clone)]
pub struct ActionTiming {
    pub name: &'static str,
    pub phase: DispatchPhase,
    pub source_location: &'static std::panic::Location<'static>,
    pub start: Instant,
    pub end: Instant,
}

impl core::fmt::Debug for ActionTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionTiming")
            .field("name", &self.name)
            .field("phase", &self.phase)
            .field("source_location", &self.source_location)
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
#[cfg(feature = "profiler")]
static ACTION_STATISTICS: LazyLock<spin::Mutex<ActionStatistics>> =
    LazyLock::new(|| spin::Mutex::new(ActionStatistics::new()));

#[doc(hidden)]
#[cfg(feature = "profiler")]
pub(crate) fn begin_action_handler(
    action: &(dyn Action + 'static),
    phase: DispatchPhase,
    source_location: &'static std::panic::Location<'static>,
    cx: &mut App,
) {
    let action = action.type_id();
    let name = cx.actions.try_resolve_action(&action).unwrap_or("un-named");
    ACTION_STATISTICS
        .lock()
        .begin_action_handler(name, phase, source_location);
}

#[doc(hidden)]
#[cfg(feature = "profiler")]
pub(crate) fn end_action_handler() {
    ACTION_STATISTICS.lock().end_action_handler();
}

#[doc(hidden)]
#[cfg(feature = "profiler")]
pub fn take_action_stats() -> ActionStatistics {
    ACTION_STATISTICS.lock().take()
}

#[doc(hidden)]
#[cfg(not(feature = "profiler"))]
pub fn take_action_stats() -> ActionStatistics {
    ActionStatistics::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restores_outer_action_after_nested_handler_finishes() {
        let mut statistics = ActionStatistics::new();
        statistics.runtime_to_beat = Duration::from_secs(60);
        let source_location = std::panic::Location::caller();

        statistics.begin_action_handler("outer_action", DispatchPhase::Bubble, source_location);
        statistics.begin_action_handler("inner_action", DispatchPhase::Capture, source_location);
        statistics.end_action_handler();

        let running = statistics.longest_runtimes(true).collect::<Vec<_>>();
        assert_eq!(running.len(), 1);
        assert_eq!(running[0].name, "outer_action");
        assert_eq!(running[0].phase, DispatchPhase::Bubble);
        assert_eq!(running[0].source_location, source_location);

        statistics.end_action_handler();
        assert_eq!(statistics.longest_runtimes(true).count(), 0);
    }

    #[test]
    fn records_action_handler_registration_location() {
        let mut statistics = ActionStatistics::new();
        statistics.runtime_to_beat = Duration::ZERO;
        let source_location = std::panic::Location::caller();

        statistics.begin_action_handler("test_action", DispatchPhase::Capture, source_location);
        statistics.end_action_handler();

        let timing = statistics
            .longest_runtimes(false)
            .next()
            .expect("action timing should be recorded");
        assert_eq!(timing.name, "test_action");
        assert_eq!(timing.phase, DispatchPhase::Capture);
        assert_eq!(timing.source_location, source_location);
    }
}
