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
