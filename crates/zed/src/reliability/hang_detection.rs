use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use ::reliability::HangReporting;
use client::Client;
use gpui::{AppContext, TasksIncluded, profiler};
use ui::App;

use crate::STARTUP_TIME;

mod logging;
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

pub(crate) fn start(client: Arc<Client>, cx: &mut App) {
    let hang_time = ::reliability::hang_threshold();

    if cfg!(debug_assertions) {
        log::warn!("debug build, only reporting hangs longer then {hang_time:?}");
    }

    start_hang_detection(hang_time, client, cx);

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

fn start_hang_detection(report_longer_then: Duration, client: Arc<Client>, cx: &App) {
    let foreground_thread = thread::current().id();
    let monitor_interval = Duration::from_secs(1);
    let started = Instant::now();
    let startup = *STARTUP_TIME.get().unwrap_or(&started);
    let hang_reporting = HangReporting::start(
        cx.foreground_journal(),
        startup,
        telemetry::send_event,
        None,
    );
    match hang_reporting {
        Ok(hang_reporting) => {
            cx.on_app_quit(move |_| {
                hang_reporting.flush();
                client.telemetry().flush_events()
            })
            .detach();
        }
        Err(error) => log::error!("failed to start hang reporting: {error}"),
    }

    let mut log = logging::Reporter::new(monitor_interval, report_longer_then, foreground_thread);
    // An OS thread keeps the legacy hang logs and task traces working while
    // the foreground or background executors are hung.
    thread::Builder::new()
        .name("HangLogging".to_string())
        .spawn(move || {
            // allow "bad" tasks during startup. Not because we should but since here
            // they are not observed by the user and to lower on clutter from the reporter
            thread::sleep(Duration::from_millis(200));
            loop {
                thread::sleep(monitor_interval);
                let task_stats = profiler::take_all_stats(TasksIncluded::CompletedAndRunning);
                let action_stats = profiler::take_action_stats();

                let should_write_trace = log.check_and_report(&task_stats, &action_stats);
                if should_write_trace {
                    if let Some(path) = task_traces::save_any(foreground_thread) {
                        log::info!("Task trace has been saved to: {}", path.display());
                    }
                }
            }
        })
        .expect("App can always spawn threads");
}
