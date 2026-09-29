//! Turns GPUI foreground hang incidents into batched "Hang Incidents" telemetry.
//!
//! [`HangTelemetry`] consumes the polls of GPUI's hang monitor
//! ([`gpui::App::start_hang_monitor`]), serializes each incident, and batches
//! incidents into periodic telemetry events. Applications own delivery by
//! supplying a sink for the resulting [`FlexibleEvent`]s, so every GPUI app
//! that uses this crate reports hangs with the same thresholds and wire
//! schema.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use gpui::App;
use gpui::profiler::hang::{
    HangMonitorConfig, HangMonitorError, HangMonitorPoll, HangMonitorPollReason, HangTrigger,
    MEASUREMENT_VERSION, SerializedHangIncident,
};
use hdrhistogram::Histogram;
use serde_json::Value;
use telemetry_events::FlexibleEvent;

pub const EVENT_TYPE: &str = "Hang Incidents";

const MAX_SERIALIZED_CONTRIBUTORS: usize = 8;

/// Cap on incidents per telemetry event. When more accrue between sends, the
/// ones with the largest stalls are kept and the incident counts still cover
/// them all.
const MAX_REPORTED_INCIDENTS: usize = 10;

// A long interval keeps hang telemetry a small fraction of event volume even
// for pathologically hang-prone sessions; the on-quit flush covers short ones.
const SEND_INTERVAL: Duration = Duration::from_mins(30);

const MONITOR_INTERVAL: Duration = Duration::from_secs(1);

/// Duration at which a single piece of foreground work counts as a hang.
pub fn hang_threshold() -> Duration {
    if cfg!(debug_assertions) {
        if cfg!(windows) {
            // Windows debug builds are especially slow.
            Duration::from_secs(30)
        } else {
            Duration::from_secs(5)
        }
    } else {
        // Will be lowered over time or turned into a setting.
        Duration::from_millis(100)
    }
}

/// Total foreground spend within one interval that counts as a hang.
pub fn frame_budget() -> Duration {
    if cfg!(debug_assertions) {
        // Unoptimized builds routinely spend more than a release frame budget
        // on ordinary frames; keep dev builds from reporting constantly.
        Duration::from_millis(100)
    } else {
        // At least one dropped frame on any display. Generous while budget
        // incidents are plentiful; lower it as they get fixed.
        Duration::from_millis(24)
    }
}

/// The monitor configuration every app using this crate reports with.
pub fn monitor_config() -> HangMonitorConfig {
    HangMonitorConfig {
        threshold: hang_threshold(),
        frame_budget: frame_budget(),
        interval: MONITOR_INTERVAL,
    }
}

/// Serializes and batches hang incidents into "Hang Incidents" events.
///
/// Every 30 minutes, and when the app quits, the current batch is passed to
/// the event sink, including empty batches, whose `report_window_seconds`
/// still counts toward observed time.
pub struct HangTelemetry {
    startup: Instant,
    reporter: Reporter,
    send_event: Box<dyn Fn(FlexibleEvent) + Send>,
    observe_incidents: Option<Box<dyn FnMut(&[SerializedHangIncident]) + Send>>,
}

impl HangTelemetry {
    /// Creates telemetry that reports incident timestamps relative to
    /// `startup` and delivers events through `send_event`, which is called on
    /// the monitor thread and must not block.
    pub fn new(startup: Instant, send_event: impl Fn(FlexibleEvent) + Send + 'static) -> Self {
        Self {
            startup,
            reporter: Reporter::new(),
            send_event: Box::new(send_event),
            observe_incidents: None,
        }
    }

    /// Also passes each poll's serialized incidents to `observer` on the
    /// monitor thread, e.g. to attach recent hangs to feedback reports.
    pub fn with_incident_observer(
        mut self,
        observer: impl FnMut(&[SerializedHangIncident]) + Send + 'static,
    ) -> Self {
        self.observe_incidents = Some(Box::new(observer));
        self
    }

    /// Starts `cx`'s hang monitor with [`monitor_config`], reporting its
    /// polls through this telemetry.
    ///
    /// # Errors
    ///
    /// See [`App::start_hang_monitor`].
    pub fn start(mut self, cx: &mut App) -> Result<(), HangMonitorError> {
        cx.start_hang_monitor(monitor_config(), move |poll| self.handle_poll(poll))
    }

    fn handle_poll(&mut self, poll: HangMonitorPoll) {
        let flush = poll.reason == HangMonitorPollReason::Flush;
        let incidents = serialize_incidents(self.startup, poll);
        if !incidents.is_empty()
            && let Some(observe_incidents) = self.observe_incidents.as_mut()
        {
            observe_incidents(&incidents);
        }
        for incident in incidents {
            self.reporter.add(incident);
        }
        if flush || self.reporter.last_send.elapsed() > SEND_INTERVAL {
            (self.send_event)(self.reporter.take_event().into_flexible_event());
        }
    }
}

fn serialize_incidents(startup: Instant, poll: HangMonitorPoll) -> Vec<SerializedHangIncident> {
    poll.incidents
        .iter()
        .map(|incident| {
            SerializedHangIncident::convert(
                startup,
                incident,
                MAX_SERIALIZED_CONTRIBUTORS,
                poll.first_present_at,
            )
        })
        .collect()
}

struct Reporter {
    last_send: Instant,
    pending: Vec<SerializedHangIncident>,
    threshold_incidents: u64,
    budget_incidents: u64,
    /// Every incident's stall, in milliseconds; `pending` keeps only the largest.
    stalls: Histogram<u64>,
}

impl Reporter {
    fn new() -> Self {
        Self {
            last_send: Instant::now(),
            pending: Vec::new(),
            threshold_incidents: 0,
            budget_incidents: 0,
            stalls: Histogram::new(3).expect("3 significant figures is a valid histogram"),
        }
    }

    fn add(&mut self, incident: SerializedHangIncident) {
        match incident.trigger {
            HangTrigger::Threshold => self.threshold_incidents += 1,
            HangTrigger::Budget => self.budget_incidents += 1,
        }
        self.stalls.record(incident.stall_ms as u64).ok();
        self.pending.push(incident);
        if self.pending.len() > MAX_REPORTED_INCIDENTS {
            self.pending
                .sort_by(|first, second| second.stall_ms.total_cmp(&first.stall_ms));
            self.pending.truncate(MAX_REPORTED_INCIDENTS);
        }
    }

    fn take_event(&mut self) -> HangIncidentsEvent {
        let now = Instant::now();
        let report_window_seconds = now.duration_since(self.last_send).as_secs();
        self.last_send = now;
        let mut incidents = std::mem::take(&mut self.pending);
        incidents.sort_by(|first, second| second.stall_ms.total_cmp(&first.stall_ms));
        let threshold_incidents = std::mem::take(&mut self.threshold_incidents);
        let budget_incidents = std::mem::take(&mut self.budget_incidents);
        let event = HangIncidentsEvent {
            incidents,
            total_incidents: threshold_incidents + budget_incidents,
            threshold_incidents,
            budget_incidents,
            stall_p50_ms: self.stalls.value_at_quantile(0.5),
            stall_p95_ms: self.stalls.value_at_quantile(0.95),
            stall_max_ms: self.stalls.max(),
            report_window_seconds,
        };
        self.stalls.reset();
        event
    }
}

struct HangIncidentsEvent {
    incidents: Vec<SerializedHangIncident>,
    /// Predates the threshold/budget split; existing queries key on it.
    total_incidents: u64,
    threshold_incidents: u64,
    budget_incidents: u64,
    stall_p50_ms: u64,
    stall_p95_ms: u64,
    stall_max_ms: u64,
    report_window_seconds: u64,
}

impl HangIncidentsEvent {
    fn into_flexible_event(self) -> FlexibleEvent {
        FlexibleEvent {
            event_type: EVENT_TYPE.to_string(),
            event_properties: HashMap::from([
                ("incidents".to_string(), to_value(&self.incidents)),
                ("total_incidents".to_string(), self.total_incidents.into()),
                (
                    "threshold_incidents".to_string(),
                    self.threshold_incidents.into(),
                ),
                ("budget_incidents".to_string(), self.budget_incidents.into()),
                ("stall_p50_ms".to_string(), self.stall_p50_ms.into()),
                ("stall_p95_ms".to_string(), self.stall_p95_ms.into()),
                ("stall_max_ms".to_string(), self.stall_max_ms.into()),
                (
                    "report_window_seconds".to_string(),
                    self.report_window_seconds.into(),
                ),
                (
                    "measurement_version".to_string(),
                    MEASUREMENT_VERSION.into(),
                ),
            ]),
        }
    }
}

fn to_value(incidents: &[SerializedHangIncident]) -> Value {
    match serde_json::to_value(incidents) {
        Ok(value) => value,
        Err(error) => {
            log::error!("failed to serialize hang incidents: {error}");
            Value::Null
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reporter_keeps_largest_incidents_and_counts_all() {
        let mut reporter = Reporter::new();
        for stall_ms in 0..12 {
            let mut incident = serialized_incident(f64::from(stall_ms));
            if stall_ms % 2 == 0 {
                incident.trigger = HangTrigger::Budget;
            }
            reporter.add(incident);
        }

        let event = reporter.take_event();
        let stalls = event
            .incidents
            .iter()
            .map(|incident| incident.stall_ms)
            .collect::<Vec<_>>();

        assert_eq!(event.total_incidents, 12);
        assert_eq!(event.threshold_incidents, 6);
        assert_eq!(event.budget_incidents, 6);
        assert_eq!(event.stall_p50_ms, 5);
        assert_eq!(event.stall_p95_ms, 11);
        assert_eq!(event.stall_max_ms, 11);
        assert_eq!(
            stalls,
            vec![11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]
        );
    }

    #[test]
    fn reporter_resets_counts_and_histogram_between_events() {
        let mut reporter = Reporter::new();
        reporter.add(serialized_incident(100.0));
        reporter.take_event();

        let mut incident = serialized_incident(20.9);
        incident.trigger = HangTrigger::Budget;
        reporter.add(incident);
        let event = reporter.take_event();

        assert_eq!(event.total_incidents, 1);
        assert_eq!(event.threshold_incidents, 0);
        assert_eq!(event.budget_incidents, 1);
        assert_eq!(event.incidents[0].stall_ms, 20.9);
        assert_eq!(event.stall_p50_ms, 20);
        assert_eq!(event.stall_p95_ms, 20);
        assert_eq!(event.stall_max_ms, 20);

        let empty = reporter.take_event();
        assert!(empty.incidents.is_empty());
        assert_eq!(empty.total_incidents, 0);
        assert_eq!(empty.threshold_incidents, 0);
        assert_eq!(empty.budget_incidents, 0);
        assert_eq!(empty.stall_p50_ms, 0);
        assert_eq!(empty.stall_p95_ms, 0);
        assert_eq!(empty.stall_max_ms, 0);
    }

    #[test]
    fn reporter_reports_empty_observation_windows() {
        let mut reporter = Reporter::new();
        let last_send = Instant::now() - Duration::from_secs(60);
        reporter.last_send = last_send;
        let event = reporter.take_event().into_flexible_event();
        let report_window_seconds = reporter.last_send.duration_since(last_send).as_secs();

        assert!(report_window_seconds >= 60);
        assert_eq!(event.event_type, "Hang Incidents");
        assert_eq!(
            serde_json::to_value(event.event_properties).unwrap(),
            json!({
                "incidents": [],
                "total_incidents": 0,
                "threshold_incidents": 0,
                "budget_incidents": 0,
                "stall_p50_ms": 0,
                "stall_p95_ms": 0,
                "stall_max_ms": 0,
                "report_window_seconds": report_window_seconds,
                "measurement_version": 2
            })
        );
    }

    #[test]
    fn hang_incidents_event_uses_expected_wire_shape() {
        let event = HangIncidentsEvent {
            incidents: vec![serialized_incident(125.0)],
            total_incidents: 3,
            threshold_incidents: 2,
            budget_incidents: 1,
            stall_p50_ms: 25,
            stall_p95_ms: 125,
            stall_max_ms: 125,
            report_window_seconds: 1800,
        }
        .into_flexible_event();

        assert_eq!(event.event_type, "Hang Incidents");
        assert_eq!(
            serde_json::to_value(event.event_properties).unwrap(),
            json!({
                "incidents": [{
                    "measurement_version": 2,
                    "phase": "steady",
                    "trigger": "threshold",
                    "start_ms": 10.0,
                    "active_ms": 125.0,
                    "stall_ms": 125.0,
                    "dirty_to_present_ms": null,
                    "sealed_by": "idle",
                    "busy_fraction": 1.0,
                    "event_count": 1,
                    "small_poll_count": 0,
                    "small_poll_total_ms": 0.0,
                    "dropped_events": 0,
                    "journal_discontinuous": false,
                    "contributors": [],
                    "contributors_elided": 0
                }],
                "total_incidents": 3,
                "threshold_incidents": 2,
                "budget_incidents": 1,
                "stall_p50_ms": 25,
                "stall_p95_ms": 125,
                "stall_max_ms": 125,
                "report_window_seconds": 1800,
                "measurement_version": 2
            })
        );
    }

    fn serialized_incident(stall_ms: f64) -> SerializedHangIncident {
        SerializedHangIncident {
            measurement_version: MEASUREMENT_VERSION,
            phase: "steady",
            trigger: HangTrigger::Threshold,
            start_ms: 10.0,
            active_ms: stall_ms,
            stall_ms,
            dirty_to_present_ms: None,
            sealed_by: "idle",
            busy_fraction: 1.0,
            event_count: 1,
            small_poll_count: 0,
            small_poll_total_ms: 0.0,
            dropped_events: 0,
            journal_discontinuous: false,
            contributors: Vec::new(),
            contributors_elided: 0,
        }
    }
}
