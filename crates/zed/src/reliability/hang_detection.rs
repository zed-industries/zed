use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use client::Client;
use gpui::profiler::hang::{HangDetector, SerializedHangIncident};
use gpui::{AppContext, TasksIncluded, profiler};
use parking_lot::Mutex;
use ui::App;

use crate::STARTUP_TIME;

mod logging;
mod task_traces;
mod telemetry;

gpui::actions!(
    dev,
    [
        /// Causes a performance hang to test performance monitoring
        HangAction,
        /// Causes a performance hang to test performance monitoring
        HangBackground,
        /// Causes a performance hang to test performance monitoring
        HangForeground,
        /// Blocks the foreground for 50ms inside an action handler to test
        /// hang incident reporting
        HangBriefly,
    ]
);

// TODO(demo): tune before shipping incident telemetry. 50ms so `dev: hang
// briefly` is caught; debug builds will also report legitimately slow work.
const INCIDENT_THRESHOLD: Duration = Duration::from_millis(50);
const MAX_SERIALIZED_CONTRIBUTORS: usize = 8;

pub(crate) fn start(client: Arc<Client>, cx: &mut App) {
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
    cx.on_action(move |_: &HangBriefly, _| {
        log::warn!(
            "Blocking the foreground for 50ms in an action handler. \
            This should be reported as a hang incident"
        );
        thread::sleep(Duration::from_millis(50) + Duration::from_micros(1));
        log::warn!("Hang ended");
    });
}

fn start_hang_detection(report_longer_then: Duration, client: Arc<Client>, cx: &App) {
    let foreground_thread = thread::current().id();
    let monitor_interval = Duration::from_secs(1);
    let telemetry = Arc::new(Mutex::new(telemetry::Reporter::new(foreground_thread)));
    let mut log = logging::Reporter::new(monitor_interval, report_longer_then, foreground_thread);

    let telemetry2 = Arc::clone(&telemetry);
    cx.on_app_quit({
        move |_| {
            telemetry2.lock().send();
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
            let mut incident_detector = HangDetector::new(INCIDENT_THRESHOLD);
            let started = Instant::now();
            let startup = *STARTUP_TIME.get().unwrap_or(&started);
            loop {
                thread::sleep(monitor_interval);
                // TODO(yara) the telemetry should not include still running tasks while the
                // reports being logged should.
                let task_stats = profiler::take_all_stats(TasksIncluded::CompletedAndRunning);
                let action_stats = profiler::take_action_stats();

                {
                    let mut telemetry = telemetry.lock();
                    telemetry.update(&task_stats, &action_stats);
                    telemetry.send_periodically();
                }

                // TODO(demo): dbg!-only while the payload shape is under
                // review; becomes a telemetry event once it looks right.
                #[allow(clippy::dbg_macro)]
                for incident in incident_detector.poll() {
                    let incident = SerializedHangIncident::convert(
                        startup,
                        &incident,
                        MAX_SERIALIZED_CONTRIBUTORS,
                    );
                    dbg!(incident);
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
