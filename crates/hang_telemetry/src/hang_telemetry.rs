//! Turns GPUI foreground hang incidents into batched "Hang Incidents" telemetry.
//!
//! [`HangTelemetry`] polls a [`HangDetector`] over the app's foreground journal,
//! serializes each incident, and batches them into periodic telemetry events.
//! Applications own delivery: they send the returned [`FlexibleEvent`] through
//! their own telemetry pipeline, so every GPUI app that uses this crate reports
//! hangs with the same thresholds and wire schema.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use gpui::profiler::hang::{
    HangDetector, HangTrigger, MEASUREMENT_VERSION, SerializedHangIncident,
};
use gpui::profiler::journal::ForegroundJournal;
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

/// Detects foreground hangs and batches them into telemetry events.
///
/// Polling and sending are driven by the caller, typically from a dedicated
/// OS thread so reporting keeps working while GPUI's executors are stalled.
pub struct HangTelemetry {
    detector: HangDetector,
    reporter: Reporter,
    startup: Instant,
}

impl HangTelemetry {
    /// Starts observing `journal` with [`hang_threshold`] and [`frame_budget`].
    ///
    /// Incident timestamps are reported relative to `startup`.
    pub fn new(journal: ForegroundJournal, startup: Instant) -> Self {
        Self {
            detector: HangDetector::new(journal, hang_threshold(), frame_budget()),
            reporter: Reporter::new(),
            startup,
        }
    }

    /// Adds incidents sealed since the previous call to the pending batch.
    ///
    /// Returns the newly collected incidents so callers can also surface them
    /// elsewhere, such as in feedback reports.
    pub fn collect(&mut self) -> Vec<SerializedHangIncident> {
        let incidents = self.detector.poll();
        let first_present_at = self.detector.first_present_at();
        let serialized_incidents = incidents
            .iter()
            .map(|incident| {
                SerializedHangIncident::convert(
                    self.startup,
                    incident,
                    MAX_SERIALIZED_CONTRIBUTORS,
                    first_present_at,
                )
            })
            .collect::<Vec<_>>();
        for incident in &serialized_incidents {
            self.reporter.add(incident.clone());
        }
        serialized_incidents
    }

    /// Takes the pending batch as an event once the send interval has elapsed.
    pub fn take_event_if_due(&mut self) -> Option<FlexibleEvent> {
        (self.reporter.last_send.elapsed() > SEND_INTERVAL).then(|| self.take_event())
    }

    /// Takes the pending batch as an event, even when it has no incidents.
    ///
    /// Empty events still matter: summing `report_window_seconds` gives the
    /// observed time when computing incident rates.
    pub fn take_event(&mut self) -> FlexibleEvent {
        self.reporter.take_event().into_flexible_event()
    }
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
