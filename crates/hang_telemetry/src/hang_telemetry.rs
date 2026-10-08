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
    HangIncident, HangMonitorConfig, HangMonitorError, HangMonitorPoll, HangMonitorPollReason,
    HangTrigger, MEASUREMENT_VERSION, SerializedHangIncident,
};
use gpui::profiler::journal::ForegroundEvent;
use serde::Serialize;
use serde_json::Value;
use telemetry_events::FlexibleEvent;

pub const EVENT_TYPE: &str = "Hang Incidents";

const MAX_SERIALIZED_CONTRIBUTORS: usize = 8;

/// Cap on incidents per telemetry event. When more accrue between sends, the
/// ones with the largest stalls are kept and the incident counts still cover
/// them all.
const MAX_REPORTED_INCIDENTS: usize = 10;

/// Cap on the contributors summed across every incident in an event, kept by
/// total duration, so frequent causes show up even when no single incident
/// they're in is among the largest.
const MAX_REPORTED_CONTRIBUTOR_TOTALS: usize = 20;

/// Cap on the distinct contributors tracked between sends, bounding memory
/// for sessions with many distinct slow tasks.
const MAX_TRACKED_CONTRIBUTORS: usize = 512;

// A long interval keeps hang telemetry a small fraction of event volume even
// for pathologically hang-prone sessions; the on-quit flush covers short ones.
const SEND_INTERVAL: Duration = Duration::from_mins(30);

const MONITOR_INTERVAL: Duration = Duration::from_secs(1);

/// Upper bounds, in milliseconds, of the stall buckets each event reports.
/// Each bucket counts stalls up to and including its bound; a final bucket
/// counts longer ones.
const STALL_BUCKETS_MS: [u64; 8] = [50, 100, 250, 500, 1000, 2000, 5000, 10000];

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

/// Total foreground spend within one interval that counts as a hang, for
/// frames on displays whose refresh interval is unknown. Elsewhere a frame
/// counts as a hang when it misses several refreshes.
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
        let active_time = poll.active_time;
        for incident in &poll.incidents {
            self.reporter.add_contributors(incident);
        }
        let incidents = serialize_incidents(self.startup, poll);
        if !incidents.is_empty()
            && let Some(observe_incidents) = self.observe_incidents.as_mut()
        {
            observe_incidents(&incidents);
        }
        for incident in incidents {
            self.reporter.add(incident);
        }
        self.reporter.active_time += active_time;
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
    /// Every incident's stall, bucketed by [`STALL_BUCKETS_MS`]; `pending`
    /// keeps only the largest incidents.
    stall_buckets: [u64; 9],
    stall_max_ms: u64,
    /// See [`gpui::profiler::hang::HangDetector::take_active_time`].
    active_time: Duration,
    active_threshold_incidents: u64,
    active_budget_incidents: u64,
    contributor_totals: HashMap<String, ContributorTotal>,
    contributors_untracked: u64,
}

/// One contributor's share of every incident since the last send.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
struct ContributorTotal {
    /// See [`contributor_name`].
    name: String,
    /// Incidents it contributed to.
    incidents: u64,
    /// Its total duration across those incidents, in milliseconds. Nested
    /// work, such as a draw inside an input dispatch, counts toward both.
    total_ms: f64,
}

/// What a contributor was, without timing: the action's name, where the task
/// was spawned, or the kind of input or frame work.
fn contributor_name(event: &ForegroundEvent) -> String {
    match event {
        ForegroundEvent::TaskPoll(timing) => {
            format!("task:{}:{}", timing.location.file(), timing.location.line())
        }
        ForegroundEvent::Action(timing) => format!("action:{}", timing.name),
        ForegroundEvent::Input(timing) => format!("input:{}", timing.kind),
        ForegroundEvent::Draw(_) => "draw".to_string(),
        ForegroundEvent::Present(_) => "present".to_string(),
        ForegroundEvent::SmallPolls(_) => "small_polls".to_string(),
    }
}

impl Reporter {
    fn new() -> Self {
        Self {
            last_send: Instant::now(),
            pending: Vec::new(),
            threshold_incidents: 0,
            budget_incidents: 0,
            stall_buckets: [0; 9],
            stall_max_ms: 0,
            active_time: Duration::ZERO,
            active_threshold_incidents: 0,
            active_budget_incidents: 0,
            contributor_totals: HashMap::new(),
            contributors_untracked: 0,
        }
    }

    fn add_contributors(&mut self, incident: &HangIncident) {
        let mut counted = Vec::new();
        for event in &incident.contributors {
            let name = contributor_name(event);
            if !self.contributor_totals.contains_key(&name)
                && self.contributor_totals.len() >= MAX_TRACKED_CONTRIBUTORS
            {
                self.contributors_untracked += 1;
                continue;
            }
            let total = self
                .contributor_totals
                .entry(name.clone())
                .or_insert_with(|| ContributorTotal {
                    name: name.clone(),
                    ..ContributorTotal::default()
                });
            total.total_ms += event.duration().as_secs_f64() * 1000.0;
            if !counted.contains(&name) {
                total.incidents += 1;
                counted.push(name);
            }
        }
    }

    fn add(&mut self, incident: SerializedHangIncident) {
        match incident.trigger {
            HangTrigger::Threshold => {
                self.threshold_incidents += 1;
                self.active_threshold_incidents += u64::from(incident.during_active_use);
            }
            HangTrigger::Budget => {
                self.budget_incidents += 1;
                self.active_budget_incidents += u64::from(incident.during_active_use);
            }
        }
        let stall_ms = incident.stall_ms.max(0.0).ceil() as u64;
        let bucket = STALL_BUCKETS_MS
            .iter()
            .position(|bound| stall_ms <= *bound)
            .unwrap_or(STALL_BUCKETS_MS.len());
        self.stall_buckets[bucket] += 1;
        self.stall_max_ms = self.stall_max_ms.max(stall_ms);
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
        let mut contributor_totals = std::mem::take(&mut self.contributor_totals)
            .into_values()
            .collect::<Vec<_>>();
        contributor_totals.sort_by(|first, second| second.total_ms.total_cmp(&first.total_ms));
        let contributor_totals_elided = contributor_totals
            .len()
            .saturating_sub(MAX_REPORTED_CONTRIBUTOR_TOTALS)
            as u64
            + std::mem::take(&mut self.contributors_untracked);
        contributor_totals.truncate(MAX_REPORTED_CONTRIBUTOR_TOTALS);
        for total in &mut contributor_totals {
            total.total_ms = (total.total_ms * 1000.0).round() / 1000.0;
        }
        let event = HangIncidentsEvent {
            incidents,
            total_incidents: threshold_incidents + budget_incidents,
            threshold_incidents,
            budget_incidents,
            stall_buckets: std::mem::take(&mut self.stall_buckets),
            stall_max_ms: std::mem::take(&mut self.stall_max_ms),
            report_window_seconds,
            active_seconds: std::mem::take(&mut self.active_time).as_secs(),
            active_threshold_incidents: std::mem::take(&mut self.active_threshold_incidents),
            active_budget_incidents: std::mem::take(&mut self.active_budget_incidents),
            contributor_totals,
            contributor_totals_elided,
        };
        event
    }
}

struct HangIncidentsEvent {
    incidents: Vec<SerializedHangIncident>,
    /// Predates the threshold/budget split; existing queries key on it.
    total_incidents: u64,
    threshold_incidents: u64,
    budget_incidents: u64,
    stall_buckets: [u64; 9],
    stall_max_ms: u64,
    report_window_seconds: u64,
    /// Of `report_window_seconds`, how long the user was actively using the
    /// app (within a minute of a key press, click, or scroll), for rates such
    /// as hangs per active hour.
    active_seconds: u64,
    /// Of `threshold_incidents`, those during active use.
    active_threshold_incidents: u64,
    /// Of `budget_incidents`, those during active use.
    active_budget_incidents: u64,
    /// Contributors summed across every incident, not only the reported ones,
    /// longest total first.
    contributor_totals: Vec<ContributorTotal>,
    /// Contributors left out of `contributor_totals` by its cap, or untracked.
    contributor_totals_elided: u64,
}

impl HangIncidentsEvent {
    fn into_flexible_event(self) -> FlexibleEvent {
        let mut event_properties = HashMap::from([
            ("incidents".to_string(), to_value(&self.incidents)),
            ("total_incidents".to_string(), self.total_incidents.into()),
            (
                "threshold_incidents".to_string(),
                self.threshold_incidents.into(),
            ),
            ("budget_incidents".to_string(), self.budget_incidents.into()),
            ("stall_max_ms".to_string(), self.stall_max_ms.into()),
            (
                "report_window_seconds".to_string(),
                self.report_window_seconds.into(),
            ),
            ("active_seconds".to_string(), self.active_seconds.into()),
            (
                "active_threshold_incidents".to_string(),
                self.active_threshold_incidents.into(),
            ),
            (
                "active_budget_incidents".to_string(),
                self.active_budget_incidents.into(),
            ),
            (
                "contributor_totals".to_string(),
                serde_json::to_value(&self.contributor_totals).unwrap_or_else(|error| {
                    log::error!("failed to serialize contributor totals: {error}");
                    Value::Null
                }),
            ),
            (
                "contributor_totals_elided".to_string(),
                self.contributor_totals_elided.into(),
            ),
            (
                "measurement_version".to_string(),
                MEASUREMENT_VERSION.into(),
            ),
        ]);
        for (index, count) in self.stall_buckets.iter().enumerate() {
            let name = match STALL_BUCKETS_MS.get(index) {
                Some(bound) => format!("stall_ms_le_{bound}"),
                None => format!(
                    "stall_ms_gt_{}",
                    STALL_BUCKETS_MS.last().copied().unwrap_or_default()
                ),
            };
            event_properties.insert(name, (*count).into());
        }
        FlexibleEvent {
            event_type: EVENT_TYPE.to_string(),
            event_properties,
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
    fn reporter_sums_contributors_across_all_incidents() {
        use gpui::profiler::{ActionTiming, journal::InputTiming};

        let start = Instant::now();
        let at = |ms: u64| start + Duration::from_millis(ms);
        let action = |name: &'static str, start_ms: u64, end_ms: u64| {
            ForegroundEvent::Action(ActionTiming {
                name,
                start: at(start_ms),
                end: at(end_ms),
            })
        };
        let mut reporter = Reporter::new();
        for index in 0..30u64 {
            let mut contributors = vec![
                action("editor::Paste", index * 100, index * 100 + 40),
                ForegroundEvent::Input(InputTiming {
                    kind: "key_down",
                    start: at(index * 100),
                    end: at(index * 100 + 41),
                    caused_invalidation: true,
                }),
            ];
            if index == 0 {
                // A name repeated within one incident counts as one incident.
                contributors.push(action("editor::Paste", 50, 60));
            }
            reporter.add_contributors(&incident_with_contributors(contributors));
        }
        let event = reporter.take_event();

        assert_eq!(
            event.contributor_totals,
            [
                ContributorTotal {
                    name: "input:key_down".to_string(),
                    incidents: 30,
                    total_ms: 1230.0,
                },
                ContributorTotal {
                    name: "action:editor::Paste".to_string(),
                    incidents: 30,
                    total_ms: 1210.0,
                },
            ]
        );
        assert_eq!(event.contributor_totals_elided, 0);
        assert!(reporter.take_event().contributor_totals.is_empty());
    }

    #[test]
    fn reporter_counts_incidents_during_active_use() {
        let mut reporter = Reporter::new();
        for (trigger, during_active_use) in [
            (HangTrigger::Threshold, true),
            (HangTrigger::Threshold, false),
            (HangTrigger::Budget, true),
        ] {
            let mut incident = serialized_incident(120.0);
            incident.trigger = trigger;
            incident.during_active_use = during_active_use;
            reporter.add(incident);
        }
        let event = reporter.take_event();
        assert_eq!(event.threshold_incidents, 2);
        assert_eq!(event.active_threshold_incidents, 1);
        assert_eq!(event.budget_incidents, 1);
        assert_eq!(event.active_budget_incidents, 1);
    }

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
        assert_eq!(event.stall_buckets, [12, 0, 0, 0, 0, 0, 0, 0, 0]);
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
        assert_eq!(event.stall_buckets, [1, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(event.stall_max_ms, 21);

        let empty = reporter.take_event();
        assert!(empty.incidents.is_empty());
        assert_eq!(empty.total_incidents, 0);
        assert_eq!(empty.threshold_incidents, 0);
        assert_eq!(empty.budget_incidents, 0);
        assert_eq!(empty.stall_buckets, [0; 9]);
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
                "stall_max_ms": 0,
                "stall_ms_le_50": 0,
                "stall_ms_le_100": 0,
                "stall_ms_le_250": 0,
                "stall_ms_le_500": 0,
                "stall_ms_le_1000": 0,
                "stall_ms_le_2000": 0,
                "stall_ms_le_5000": 0,
                "stall_ms_le_10000": 0,
                "stall_ms_gt_10000": 0,
                "report_window_seconds": report_window_seconds,
                "active_seconds": 0,
                "active_threshold_incidents": 0,
                "active_budget_incidents": 0,
                "contributor_totals": [],
                "contributor_totals_elided": 0,
                "measurement_version": 3
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
            stall_buckets: [1, 0, 2, 0, 0, 0, 0, 0, 0],
            stall_max_ms: 125,
            report_window_seconds: 1800,
            active_seconds: 900,
            active_threshold_incidents: 1,
            active_budget_incidents: 1,
            contributor_totals: vec![ContributorTotal {
                name: "action:editor::Paste".to_string(),
                incidents: 2,
                total_ms: 250.5,
            }],
            contributor_totals_elided: 4,
        }
        .into_flexible_event();

        assert_eq!(event.event_type, "Hang Incidents");
        assert_eq!(
            serde_json::to_value(event.event_properties).unwrap(),
            json!({
                "incidents": [{
                    "measurement_version": 3,
                    "phase": "steady",
                    "trigger": "threshold",
                    "during_active_use": false,
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
                "stall_max_ms": 125,
                "stall_ms_le_50": 1,
                "stall_ms_le_100": 0,
                "stall_ms_le_250": 2,
                "stall_ms_le_500": 0,
                "stall_ms_le_1000": 0,
                "stall_ms_le_2000": 0,
                "stall_ms_le_5000": 0,
                "stall_ms_le_10000": 0,
                "stall_ms_gt_10000": 0,
                "report_window_seconds": 1800,
                "active_seconds": 900,
                "active_threshold_incidents": 1,
                "active_budget_incidents": 1,
                "contributor_totals": [{
                    "name": "action:editor::Paste",
                    "incidents": 2,
                    "total_ms": 250.5
                }],
                "contributor_totals_elided": 4,
                "measurement_version": 3
            })
        );
    }

    fn incident_with_contributors(contributors: Vec<ForegroundEvent>) -> HangIncident {
        let start = Instant::now();
        HangIncident {
            snapshot: gpui::profiler::journal::FrameSnapshot {
                interval_start: start,
                boundary: gpui::profiler::journal::IntervalBoundary::Idle { ended_at: start },
                events: contributors.clone(),
                small_polls: Vec::new(),
                dropped_events: 0,
                journal_discontinuous: false,
            },
            trigger: HangTrigger::Threshold,
            contributors,
            during_active_use: false,
        }
    }

    fn serialized_incident(stall_ms: f64) -> SerializedHangIncident {
        SerializedHangIncident {
            measurement_version: MEASUREMENT_VERSION,
            phase: "steady",
            trigger: HangTrigger::Threshold,
            during_active_use: false,
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
