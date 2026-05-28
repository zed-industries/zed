use std::fmt::Display;
use std::panic::Location;
use std::thread::{self, ThreadId};
use std::time::{Duration, Instant};

use collections::HashMap;
use gpui::{AppContext, TasksIncluded, profiler};
use itertools::Itertools;
use log::info;
use ui::App;

mod task_traces;

gpui::actions!(
    dev,
    [
        /// Causes a performance hang to test performance monitoring
        HangAction,
        /// Causes a performance hang to test performance monitoring
        HangBackground,
        /// Causes a performance hang to test performance monitoring
        HangForeground,
    ]
);

pub(crate) fn start(cx: &mut App) {
    let hang_time = if cfg!(debug_assertions) {
        if cfg!(windows) {
            // yes windows debug builds are horribly slow
            Duration::from_secs(30)
        } else {
            Duration::from_secs(5)
        }
    } else {
        // will be lowered over time or turned into a setting
        Duration::from_millis(100)
    };

    if cfg!(debug_assertions) {
        log::warn!("debug build, only reporting hangs longer then {hang_time:?}");
    }

    start_hang_detection(cx, hang_time);

    cx.on_action(move |_: &HangAction, _| {
        log::warn!(
            "Hanging the foreground for {hang_time:?} by blocking in an action. \
            Zed will be unresponsive for that time. This should trigger a report in the log",
        );
        thread::sleep(hang_time + Duration::from_micros(1));
        log::warn!("Hang ended");
    });
    cx.on_action(move |_: &HangBackground, cx| {
        cx.background_spawn(async move {
            log::warn!(
                "Hanging one background executor for {hang_time:?}. \
                This should trigger a report in the log",
            );
            thread::sleep(hang_time + Duration::from_micros(1));
            log::warn!("Hang ended");
        })
        .detach();
    });
    cx.on_action(move |_: &HangForeground, cx| {
        cx.spawn(async move |_| {
            log::warn!(
                "Hanging the foreground executor for {hang_time:?} seconds to test \
                performance monitoring! Zed will be unresponsive for that time. \
                This should trigger a report in the log"
            );
            thread::sleep(hang_time + Duration::from_micros(1));
            log::warn!("Hang ended");
        })
        .detach();
    });
}

fn start_hang_detection(cx: &App, report_longer_then: Duration) {
    let foreground_thread = thread::current().id();
    let action_resolver = cx.__action_resolver();

    // an OS thread to insulate detection and reporting from hangs on the fore
    // or background.
    thread::Builder::new()
        .name("HangDetection".to_string())
        .spawn(move || {
            // allow "bad" tasks during startup. Not because we should but since here
            // they are not observed by the user and to lower on clutter from the reporter
            thread::sleep(Duration::from_millis(200));
            let mut reporter = Reporter::new(Duration::from_secs(1));
            loop {
                thread::sleep(reporter.monitor_interval);
                let task_stats = profiler::get_all_stats(TasksIncluded::CompletedAndRunning);

                let mut reported_task_hangs = false;
                reported_task_hangs |= reporter.report_hanging_foreground(
                    &task_stats,
                    report_longer_then,
                    foreground_thread,
                );
                reported_task_hangs |= reporter.report_hanging_background(
                    &task_stats,
                    report_longer_then,
                    foreground_thread,
                );
                reporter.report_hanging_actions(&action_resolver, report_longer_then);

                if reported_task_hangs && let Some(path) = task_traces::save_any(foreground_thread)
                {
                    log::info!("Task trace has been saved to: {}", path.display());
                }
            }
        })
        .expect("App can always spawn threads");
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum PerfIssue {
    Foreground(&'static Location<'static>),
    Background(&'static Location<'static>),
    Action(&'static str),
}

struct Reporter {
    monitor_interval: Duration,
    forget_after: Duration,
    history: HashMap<PerfIssue, Instant>,
}

impl Reporter {
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
    fn new(monitor_interval: Duration) -> Self {
        Self {
            monitor_interval,
            forget_after: Duration::from_mins(5),
            history: HashMap::default(),
        }
    }
}

type ReportMade = bool;
impl Reporter {
    fn report_hanging_foreground(
        &mut self,
        task_stats: &[gpui::ThreadTaskStatistics],
        report_longer_then: Duration,
        foreground_thread: ThreadId,
    ) -> ReportMade {
        let foreground = task_stats
            .iter()
            .find(|t| t.thread_id == foreground_thread)
            .expect("main thread should be in all statistics");

        let hangs: Vec<_> = foreground
            .stats
            .longest_poll_times
            .into_iter()
            .filter(|task| task.poll_duration() > report_longer_then)
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
        report_longer_then: Duration,
        foreground_thread: ThreadId,
    ) -> ReportMade {
        let background = task_stats
            .iter()
            .filter(|t| t.thread_id != foreground_thread);

        let mut report_made = false;
        for worker in background {
            let hangs: Vec<_> = worker
                .stats
                .longest_poll_times
                .into_iter()
                .filter(|stat| stat.poll_duration() > report_longer_then)
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

    fn report_hanging_actions(
        &mut self,
        resolver: &gpui::ActionResolver,
        report_longer_then: Duration,
    ) {
        let hangs: Vec<_> = profiler::get_action_stats()
            .resolve(resolver)
            .0
            .into_iter()
            .filter(|action| action.runtime() > report_longer_then)
            .filter(|action| !self.hold_report(PerfIssue::Action(action.name)))
            .collect();

        self.update_reported(hangs.iter().map(|action| PerfIssue::Action(action.name)));
        if !hangs.is_empty() {
            info!("Action hang detected:\n{}", DisplayActions(hangs));
        }
    }
}

struct DisplayActions(Vec<gpui::profiler::ResolvedActionTiming>);
struct DisplayTasks<'a>(&'a [gpui::TaskTiming]);

impl Display for DisplayActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Actions(s) that ran too long\n")?;
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
                format!("{:?}", task.poll_duration()), // impl dbg does not support alignment
                task.location
            ))?;
            writeln!(f)?;
        }
        Ok(())
    }
}
