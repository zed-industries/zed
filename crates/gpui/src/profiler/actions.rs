use std::{
    any::TypeId,
    hint::cold_path,
    time::{Duration, Instant},
};

use itertools::Itertools;

use crate::{ActionRegistry, action::Action};

#[doc(hidden)]
#[derive(Clone)]
pub struct ActionStatistics {
    runtime_to_beat: Duration,
    longest_runtimes: heapless::Vec<ActionTiming, 5>,
    running: Option<(TypeId, Instant)>,
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

impl std::fmt::Display for ResolvedActionStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Actions that blocked the longest\n")?;
        for action in self.0.iter().sorted_by_key(|action| action.runtime()).rev() {
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

impl ActionStatistics {
    const fn new() -> Self {
        Self {
            runtime_to_beat: Duration::ZERO,
            longest_runtimes: heapless::Vec::new(),
            running: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.longest_runtimes.is_empty()
    }

    pub fn update_running_action(&mut self, action: TypeId, started: Instant) {
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
                    id: action,
                    start: started,
                    end: now,
                };
            } else {
                self.longest_runtimes
                    .push(ActionTiming {
                        id: action,
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

    pub fn resolve(self, resolver: &crate::ActionResolver) -> ResolvedActionStatistics {
        ResolvedActionStatistics(
            self.longest_runtimes
                .into_iter()
                .flat_map(|timing| timing.try_resolve(&resolver.0))
                .collect(),
        )
    }

    pub fn longest_runtimes(&self) -> impl Iterator<Item = ActionTiming> {
        self.longest_runtimes
            .iter()
            .copied()
            .chain(self.running.into_iter().map(|(id, start)| ActionTiming {
                id,
                start,
                end: Instant::now(),
            }))
    }
}

#[doc(hidden)]
/// Resolved variant of [`ActionTiming`] where the actions are resolved (use
/// names instead of type ids)
#[derive(Debug, Clone)]
pub struct ResolvedActionStatistics(pub Vec<ResolvedActionTiming>);
impl ResolvedActionStatistics {
    #[doc(hidden)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    #[doc(hidden)]
    pub fn empty() -> Self {
        Self(Vec::new())
    }
}

#[doc(hidden)]
/// UNSTABLE only for use in the profiler and zed-reliability
#[derive(Copy, Clone)]
pub struct ActionTiming {
    pub id: TypeId,
    pub start: Instant,
    pub end: Instant,
}

impl core::fmt::Debug for ActionTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionTiming")
            .field("id", &self.id)
            .field("runtime", &self.runtime())
            .finish()
    }
}

impl ActionTiming {
    pub fn runtime(&self) -> Duration {
        self.end - self.start
    }
    fn try_resolve(self, actions: &ActionRegistry) -> Option<ResolvedActionTiming> {
        match actions.try_resolve_action(&self.id) {
            Some(action_name) => Some(ResolvedActionTiming {
                name: action_name,
                start: self.start,
                end: self.end,
            }),
            None => {
                cold_path();
                log::error!("Profiler could not resolve action name");
                None
            }
        }
    }
}

#[doc(hidden)]
/// Resolved variant of [`ActionTiming`] with Type_Id replaced with the Action's
/// name instead.
#[derive(Debug, Clone)]
pub struct ResolvedActionTiming {
    pub name: &'static str,
    pub start: Instant,
    pub end: Instant,
}

impl ResolvedActionTiming {
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
pub(crate) fn update_running_action(action: &(dyn Action + 'static)) {
    let now = Instant::now();
    let action = action.type_id();
    ACTION_STATISTICS.lock().update_running_action(action, now);
}

#[doc(hidden)]
pub(crate) fn save_action_timing() {
    ACTION_STATISTICS.lock().save_action_timing();
}

#[doc(hidden)]
pub fn get_action_stats() -> ActionStatistics {
    ACTION_STATISTICS.lock().clone()
}
