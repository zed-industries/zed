use std::time::{Duration, Instant};

use gpui::profiler::hang::{HangTrigger, SerializedHangIncident};

/// Cap on incidents per telemetry event. When more accrue between sends, the
/// ones with the largest stalls are kept and the incident counts still cover
/// them all.
const MAX_REPORTED_INCIDENTS: usize = 10;

// A long interval keeps hang telemetry a small fraction of event volume even
// for pathologically hang-prone sessions; the on-quit flush covers short ones.
const SEND_INTERVAL: Duration = Duration::from_mins(30);

pub struct Reporter {
    startup: Instant,
    last_send: Instant,
    pending: Vec<SerializedHangIncident>,
    threshold_incidents: u64,
    budget_incidents: u64,
    stalls_100to250: u64,
    stalls_250to1000: u64,
    stalls_over_1000: u64,
}

impl Reporter {
    pub fn new(startup: Instant) -> Self {
        Self {
            startup,
            last_send: Instant::now(),
            pending: Vec::new(),
            threshold_incidents: 0,
            budget_incidents: 0,
            stalls_100to250: 0,
            stalls_250to1000: 0,
            stalls_over_1000: 0,
        }
    }

    pub fn add(&mut self, incident: SerializedHangIncident) {
        match incident.trigger {
            HangTrigger::Threshold => self.threshold_incidents += 1,
            HangTrigger::Budget => self.budget_incidents += 1,
        }
        // Every incident counts here; `pending` keeps only the largest stalls.
        if incident.stall_ms >= 1000.0 {
            self.stalls_over_1000 += 1;
        } else if incident.stall_ms >= 250.0 {
            self.stalls_250to1000 += 1;
        } else if incident.stall_ms >= 100.0 {
            self.stalls_100to250 += 1;
        }
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

    pub fn send(&mut self) {
        let now = Instant::now();
        let report_window_seconds = now.duration_since(self.last_send).as_secs();
        self.last_send = now;
        if self.pending.is_empty() {
            return;
        }
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
            stalls_100to250 = std::mem::take(&mut self.stalls_100to250),
            stalls_250to1000 = std::mem::take(&mut self.stalls_250to1000),
            stalls_over_1000 = std::mem::take(&mut self.stalls_over_1000),
            uptime_seconds = now.duration_since(self.startup).as_secs(),
            report_window_seconds,
            measurement_version = gpui::profiler::hang::MEASUREMENT_VERSION
        );
    }
}
