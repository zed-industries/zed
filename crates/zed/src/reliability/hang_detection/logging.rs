use std::fmt::Display;
use std::panic::Location;
use std::thread::ThreadId;
use std::time::{Duration, Instant};

use collections::HashMap;
use itertools::Itertools;
use log::info;

#[derive(Debug, Hash, Eq, PartialEq)]
enum PerfIssue {
    Foreground(&'static Location<'static>),
    Background(&'static Location<'static>),
    Action(&'static str),
}

pub struct Reporter {
    monitor_interval: Duration,
    forget_after: Duration,
    history: HashMap<PerfIssue, Instant>,
    report_longer_then: Duration,
    foreground_thread: ThreadId,
}

type ReportMade = bool;
impl Reporter {
    pub fn new(
        monitor_interval: Duration,
        report_longer_then: Duration,
        foreground_thread: ThreadId,
    ) -> Self {
        Self {
            monitor_interval,
            forget_after: Duration::from_mins(5),
            history: HashMap::default(),
            report_longer_then,
            foreground_thread,
        }
    }
    pub fn check_and_report(
        &mut self,
        task_stats: &[gpui::ThreadTaskStatistics],
        action_stats: &gpui::ActionStatistics,
    ) -> ReportMade {
        let mut reported_task_hangs = false;
        reported_task_hangs |= self.report_hanging_foreground(&task_stats);
        reported_task_hangs |= self.report_hanging_background(&task_stats);

        self.report_hanging_actions(action_stats);
        reported_task_hangs
    }

    fn hold_report(&self, issue: PerfIssue) -> bool {
        self.history
            .get(&issue)
            .map(Instant::elapsed)
            .is_some_and(|since_report| since_report > self.monitor_interval)
    }
    fn update_reported(&mut self, new: impl Iterator<Item = PerfIssue>) {
        let now = Instant::now();
        for issue in new {
            if self
                .history
                .get(&issue)
                .is_none_or(|s| s.elapsed() > self.forget_after)
            {
                self.history.insert(issue, now);
            }
        }
    }
}

impl Reporter {
    fn report_hanging_foreground(
        &mut self,
        task_stats: &[gpui::ThreadTaskStatistics],
    ) -> ReportMade {
        let foreground = self.foreground_thread;
        let Some(foreground) = task_stats.iter().find(|t| t.thread_id == foreground) else {
            return false; // during startup the foreground may not yet have statistics
        };

        let hangs: Vec<_> = foreground
            .stats
            .longest_poll_times
            .into_iter()
            .filter(|task| task.poll_duration() > self.report_longer_then)
            .filter(|task| !self.hold_report(PerfIssue::Foreground(task.location)))
            .collect();
        self.update_reported(
            hangs
                .iter()
                .map(|task| PerfIssue::Foreground(task.location)),
        );
        if !hangs.is_empty() {
            info!("New foreground hang detected:\n{}", DisplayTasks(&hangs));
        }

        !hangs.is_empty()
    }

    fn report_hanging_background(
        &mut self,
        task_stats: &[gpui::ThreadTaskStatistics],
    ) -> ReportMade {
        let foreground = self.foreground_thread;
        let background = task_stats.iter().filter(move |t| t.thread_id != foreground);

        let mut report_made = false;
        for worker in background {
            let hangs: Vec<_> = worker
                .stats
                .longest_poll_times
                .into_iter()
                .filter(|stat| stat.poll_duration() > self.report_longer_then)
                .filter(|task| !self.hold_report(PerfIssue::Background(task.location)))
                .collect();

            if hangs.is_empty() {
                continue;
            }

            self.update_reported(
                hangs
                    .iter()
                    .map(|task| PerfIssue::Background(task.location)),
            );

            info!(
                "Background hang detected on {}:\n{}",
                worker.thread_name.as_deref().unwrap_or_else(|| "Unknown"),
                DisplayTasks(&hangs)
            );
            report_made = true;
        }
        report_made
    }

    fn report_hanging_actions(&mut self, action_stats: &gpui::ActionStatistics) {
        let hangs: Vec<_> = action_stats
            .longest_runtimes(true)
            .filter(|action| action.runtime() > self.report_longer_then)
            .filter(|action| !self.hold_report(PerfIssue::Action(action.name)))
            .collect();

        self.update_reported(hangs.iter().map(|action| PerfIssue::Action(action.name)));
        if !hangs.is_empty() {
            info!("Action hang detected:\n{}", DisplayActions(hangs));
        }
    }
}

struct DisplayActions(Vec<gpui::profiler::ActionTiming>);
struct DisplayTasks<'a>(&'a [gpui::TaskTiming]);

impl Display for DisplayActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Actions(s) that ran too long\n")?;
        for action in self.0.iter().sorted_by_key(|action| action.runtime()).rev() {
            f.write_fmt(format_args!(
                "{:<20} - {}",
                format!("{:?}", action.runtime()), // impl debug does not support alignment
                action.name
            ))?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> Display for DisplayTasks<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Tasks(s) that ran too long\n")?;
        for task in self
            .0
            .iter()
            .sorted_by_key(|task| task.poll_duration())
            .rev()
        {
            f.write_fmt(format_args!(
                "{:<20} - {}",
                format!("{:?}", task.poll_duration()), // impl debug does not support alignment
                task.location
            ))?;
            writeln!(f)?;
        }
        Ok(())
    }
}
