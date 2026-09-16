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
    ]
);

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

    let frame_budget = if cfg!(debug_assertions) {
        // Unoptimized builds routinely spend more than a release frame
        // budget on ordinary frames; keep dev builds from reporting
        // constantly.
        Duration::from_millis(100)
    } else {
        // At least one dropped frame on any display. Generous while budget
        // incidents are plentiful; lower it as they get fixed.
        Duration::from_millis(24)
    };

    if cfg!(debug_assertions) {
        log::warn!("debug build, only reporting hangs longer then {hang_time:?}");
    }

    start_hang_detection(hang_time, frame_budget, client, cx);

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

fn start_hang_detection(
    report_longer_then: Duration,
    frame_budget: Duration,
    client: Arc<Client>,
    cx: &App,
) {
    let foreground_thread = thread::current().id();
    let monitor_interval = Duration::from_secs(1);
    let telemetry = Arc::new(Mutex::new(telemetry::Reporter::new()));
    let incident_detector = Arc::new(spin::Mutex::new(HangDetector::new(
        cx.foreground_journal(),
        report_longer_then,
        frame_budget,
    )));
    let started = Instant::now();
    let startup = *STARTUP_TIME.get().unwrap_or(&started);
    let mut log = logging::Reporter::new(monitor_interval, report_longer_then, foreground_thread);

    cx.on_app_quit({
        let telemetry = Arc::clone(&telemetry);
        let incident_detector = Arc::clone(&incident_detector);
        move |_| {
            let mut incident_detector = incident_detector.lock();
            let incidents = incident_detector.poll();
            let first_present_at = incident_detector.first_present_at();
            drop(incident_detector);

            let mut telemetry = telemetry.lock();
            for incident in &incidents {
                telemetry.add(SerializedHangIncident::convert(
                    startup,
                    incident,
                    MAX_SERIALIZED_CONTRIBUTORS,
                    first_present_at,
                ));
            }
            telemetry.send();
            drop(telemetry);
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

                {
                    let mut incident_detector = incident_detector.lock();
                    let incidents = incident_detector.poll();
                    let first_present_at = incident_detector.first_present_at();
                    drop(incident_detector);

                    let mut telemetry = telemetry.lock();
                    for incident in &incidents {
                        let serialized_incident = SerializedHangIncident::convert(
                            startup,
                            incident,
                            MAX_SERIALIZED_CONTRIBUTORS,
                            first_present_at,
                        );
                        telemetry.add(serialized_incident);
                    }
                    telemetry.send_periodically();
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
