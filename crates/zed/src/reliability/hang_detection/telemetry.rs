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
    last_send: Instant,
    pending: Vec<SerializedHangIncident>,
    threshold_incidents: u64,
    budget_incidents: u64,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            last_send: Instant::now(),
            pending: Vec::new(),
            threshold_incidents: 0,
            budget_incidents: 0,
        }
    }

    pub fn add(&mut self, incident: SerializedHangIncident) {
        match incident.trigger {
            HangTrigger::Threshold => self.threshold_incidents += 1,
            HangTrigger::Budget => self.budget_incidents += 1,
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
        self.last_send = Instant::now();
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
            budget_incidents
        );
    }
}
