use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use client::Client;
use gpui::{AppContext, TasksIncluded, profiler};
use hang_telemetry::HangTelemetry;
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
    let hang_time = hang_telemetry::hang_threshold();

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
    let hang_telemetry = Arc::new(spin::Mutex::new(HangTelemetry::new(
        cx.foreground_journal(),
        startup,
    )));
    let mut log = logging::Reporter::new(monitor_interval, report_longer_then, foreground_thread);

    cx.on_app_quit({
        let hang_telemetry = Arc::clone(&hang_telemetry);
        move |_| {
            let mut hang_telemetry = hang_telemetry.lock();
            hang_telemetry.collect();
            let event = hang_telemetry.take_event();
            drop(hang_telemetry);
            telemetry::send_event(event);
            client.telemetry().flush_events()
        }
    })
    .detach();

    // an OS thread to insulate detection and reporting from hangs on the fore
    // or background.
    thread::Builder::new()
        .name("HangDetection".to_string())
        .spawn(move || {
            // allow "bad" tasks during startup. Not because we should but since here
            // they are not observed by the user and to lower on clutter from the reporter
            thread::sleep(Duration::from_millis(200));
            loop {
                thread::sleep(monitor_interval);
                let task_stats = profiler::take_all_stats(TasksIncluded::CompletedAndRunning);
                let action_stats = profiler::take_action_stats();

                let event = {
                    let mut hang_telemetry = hang_telemetry.lock();
                    hang_telemetry.collect();
                    hang_telemetry.take_event_if_due()
                };
                if let Some(event) = event {
                    telemetry::send_event(event);
                }

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
