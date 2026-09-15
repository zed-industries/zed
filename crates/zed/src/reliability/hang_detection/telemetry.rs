use std::time::{Duration, Instant};

use gpui::profiler::{
    hang::{HangTrigger, MEASUREMENT_VERSION, SerializedHangIncident},
    journal::LifecycleCounts,
};

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
    lifecycle_counts: LifecycleCounts,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            last_send: Instant::now(),
            pending: Vec::new(),
            threshold_incidents: 0,
            budget_incidents: 0,
            lifecycle_counts: LifecycleCounts::default(),
        }
    }

    pub fn add_lifecycle_counts(&mut self, counts: LifecycleCounts) {
        self.lifecycle_counts.interrupted_spans += counts.interrupted_spans;
        self.lifecycle_counts.sleep_transitions += counts.sleep_transitions;
        self.lifecycle_counts.wake_transitions += counts.wake_transitions;
        self.lifecycle_counts.excluded_frame_samples += counts.excluded_frame_samples;
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
        if self.pending.is_empty()
            && self.lifecycle_counts.interrupted_spans == 0
            && self.lifecycle_counts.sleep_transitions == 0
            && self.lifecycle_counts.wake_transitions == 0
            && self.lifecycle_counts.excluded_frame_samples == 0
        {
            return;
        }
        let mut incidents = std::mem::take(&mut self.pending);
        incidents.sort_by(|a, b| b.stall_ms.total_cmp(&a.stall_ms));
        let threshold_incidents = std::mem::take(&mut self.threshold_incidents);
        let budget_incidents = std::mem::take(&mut self.budget_incidents);
        let lifecycle_counts = std::mem::take(&mut self.lifecycle_counts);
        // `total_incidents` predates the split; existing queries key on it.
        let total_incidents = threshold_incidents + budget_incidents;

        telemetry::event!(
            "Hang Incidents",
            incidents,
            total_incidents,
            threshold_incidents,
            budget_incidents,
            measurement_version = MEASUREMENT_VERSION,
            lifecycle_coverage = "delivered_platform_callbacks",
            interrupted_spans = lifecycle_counts.interrupted_spans,
            sleep_transitions = lifecycle_counts.sleep_transitions,
            wake_transitions = lifecycle_counts.wake_transitions,
            excluded_frame_samples = lifecycle_counts.excluded_frame_samples
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_counts_accumulate_independently_of_hang_examples() {
        let mut reporter = Reporter::new();
        reporter.add_lifecycle_counts(LifecycleCounts {
            interrupted_spans: 2,
            sleep_transitions: 1,
            wake_transitions: 1,
            excluded_frame_samples: 3,
        });
        reporter.add_lifecycle_counts(LifecycleCounts {
            interrupted_spans: 4,
            sleep_transitions: 1,
            wake_transitions: 1,
            excluded_frame_samples: 5,
        });
        assert!(reporter.pending.is_empty());
        assert_eq!(reporter.lifecycle_counts.interrupted_spans, 6);
        assert_eq!(reporter.lifecycle_counts.sleep_transitions, 2);
        assert_eq!(reporter.lifecycle_counts.wake_transitions, 2);
        assert_eq!(reporter.lifecycle_counts.excluded_frame_samples, 8);
        assert_eq!(reporter.threshold_incidents, 0);
        assert_eq!(reporter.budget_incidents, 0);
    }

    #[test]
    fn reports_lifecycle_only_windows_and_resets_counts() {
        for counts in [
            LifecycleCounts {
                interrupted_spans: 1,
                ..LifecycleCounts::default()
            },
            LifecycleCounts {
                sleep_transitions: 1,
                ..LifecycleCounts::default()
            },
            LifecycleCounts {
                wake_transitions: 1,
                ..LifecycleCounts::default()
            },
            LifecycleCounts {
                excluded_frame_samples: 1,
                ..LifecycleCounts::default()
            },
        ] {
            let mut reporter = Reporter::new();
            reporter.add_lifecycle_counts(counts);
            reporter.send();
            assert!(reporter.pending.is_empty());
            assert_eq!(reporter.lifecycle_counts.interrupted_spans, 0);
            assert_eq!(reporter.lifecycle_counts.sleep_transitions, 0);
            assert_eq!(reporter.lifecycle_counts.wake_transitions, 0);
            assert_eq!(reporter.lifecycle_counts.excluded_frame_samples, 0);
        }
    }
}
