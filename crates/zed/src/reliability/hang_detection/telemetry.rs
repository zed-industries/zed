use std::time::{Duration, Instant};

use gpui::profiler::hang::{HangTrigger, SerializedHangIncident};
use hdrhistogram::Histogram;

/// Cap on incidents per telemetry event. When more accrue between sends, the
/// ones with the largest stalls are kept and the incident counts still cover
/// them all.
const MAX_REPORTED_INCIDENTS: usize = 10;

// A long interval keeps hang telemetry a small fraction of event volume even
// for pathologically hang-prone sessions; the on-quit flush covers short ones.
const SEND_INTERVAL: Duration = Duration::from_mins(30);

pub struct Reporter {
    last_send: Instant,
    pending: Vec<SerializedHangIncident>,
    threshold_incidents: u64,
    budget_incidents: u64,
    // Every incident's stall, in milliseconds; `pending` keeps only the largest.
    stalls: Histogram<u64>,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            last_send: Instant::now(),
            pending: Vec::new(),
            threshold_incidents: 0,
            budget_incidents: 0,
            stalls: Histogram::new(3).expect("3 significant figures is a valid histogram"),
        }
    }

    pub fn add(&mut self, incident: SerializedHangIncident) {
        match incident.trigger {
            HangTrigger::Threshold => self.threshold_incidents += 1,
            HangTrigger::Budget => self.budget_incidents += 1,
        }
        self.stalls.record(incident.stall_ms as u64).ok();
        self.pending.push(incident);
        if self.pending.len() > MAX_REPORTED_INCIDENTS {
            self.pending
                .sort_by(|a, b| b.stall_ms.total_cmp(&a.stall_ms));
            self.pending.truncate(MAX_REPORTED_INCIDENTS);
        }
    }

    pub fn send_periodically(&mut self) {
        if self.last_send.elapsed() > SEND_INTERVAL {
            self.send();
        }
    }

    // Sends even without incidents so summing `report_window_seconds` gives the
    // observed time when computing incident rates.
    pub fn send(&mut self) {
        let now = Instant::now();
        let report_window_seconds = now.duration_since(self.last_send).as_secs();
        self.last_send = now;
        let mut incidents = std::mem::take(&mut self.pending);
        incidents.sort_by(|a, b| b.stall_ms.total_cmp(&a.stall_ms));
        let threshold_incidents = std::mem::take(&mut self.threshold_incidents);
        let budget_incidents = std::mem::take(&mut self.budget_incidents);
        // `total_incidents` predates the split; existing queries key on it.
        let total_incidents = threshold_incidents + budget_incidents;

        telemetry::event!(
            "Hang Incidents",
            incidents,
            total_incidents,
            threshold_incidents,
            budget_incidents,
            stall_p50_ms = self.stalls.value_at_quantile(0.5),
            stall_p95_ms = self.stalls.value_at_quantile(0.95),
            stall_max_ms = self.stalls.max(),
            report_window_seconds,
            measurement_version = gpui::profiler::hang::MEASUREMENT_VERSION
        );
        self.stalls.reset();
    }
}
