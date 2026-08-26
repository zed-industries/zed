use collections::{HashMap, HashSet};
use gpui::{App, FrameDurationSnapshot, Global, InputLatencySnapshot, Window, WindowId, actions};
use hdrhistogram::Histogram;
use std::time::Instant;

actions!(
    dev,
    [
        /// Opens a buffer showing the input-to-frame latency histogram for the current window.
        DumpInputLatencyHistogram,
    ]
);

/// The data needed to format an input-latency report, captured synchronously
/// by [`snapshot_input_latency_report`]. Formatting via
/// [`format_input_latency_report`] is pure computation and can happen on a
/// background thread.
pub struct InputLatencyReportData {
    snapshot: InputLatencySnapshot,
    previous_snapshot: Option<InputLatencySnapshot>,
    previous_timestamp: Option<chrono::DateTime<chrono::Local>>,
    timestamp: chrono::DateTime<chrono::Local>,
    /// The collab username of the user whose machine produced the data.
    /// Set when the report will be placed in a buffer visible to other
    /// collaborators, so it's clear whose latency was measured.
    reported_by: Option<String>,
}

/// Captures the input-to-frame latency histogram for the given window, along
/// with the previous report's baseline (tracked via a global on the `App`) for
/// delta computation. Advances the baseline to this snapshot.
pub fn snapshot_input_latency_report(
    window: &Window,
    reported_by: Option<String>,
    cx: &mut App,
) -> InputLatencyReportData {
    let snapshot = window.input_latency_snapshot();
    let timestamp = chrono::Local::now();
    let state = cx.default_global::<ReporterState>();

    let data = InputLatencyReportData {
        snapshot: snapshot.clone(),
        previous_snapshot: state.previous_snapshot.take(),
        previous_timestamp: state.previous_timestamp.take(),
        timestamp,
        reported_by,
    };

    state.previous_snapshot = Some(snapshot);
    state.previous_timestamp = Some(timestamp);

    data
}

/// Generates a formatted text report from a previously captured
/// [`InputLatencyReportData`]. If the capture included a previous snapshot,
/// includes a delta section showing changes since that report.
pub fn format_input_latency_report(data: &InputLatencyReportData) -> String {
    format_report(data)
}

#[derive(Default)]
struct ReporterState {
    previous_snapshot: Option<InputLatencySnapshot>,
    previous_timestamp: Option<chrono::DateTime<chrono::Local>>,
}

impl Global for ReporterState {}

/// Per-window state used for telemetry delta computation. Kept separate from
/// `ReporterState` so the user-facing dump and the background telemetry flush
/// maintain independent baselines.
#[derive(Default)]
struct TelemetryReporterState {
    /// Keyed by window id. Each entry holds the cumulative snapshot at the time
    /// of the last telemetry flush, plus the wall-clock time of that flush.
    previous: HashMap<WindowId, (Instant, InputLatencySnapshot)>,
}

impl Global for TelemetryReporterState {}

/// Nanosecond boundaries for the time-range buckets used in telemetry.
/// These match the display distribution in format_report so the two stay in sync.
const MS4_NS: u64 = 4_000_000;
const MS8_NS: u64 = 8_000_000;
const MS16_NS: u64 = 16_000_000;
const MS33_NS: u64 = 33_000_000;
const MS100_NS: u64 = 100_000_000;

/// Minimum number of frames that must be present in the delta window for the
/// telemetry report to be sent. Avoids sending noise for windows that are
/// mostly idle.
const MIN_FRAMES_TO_REPORT: u64 = 5_000;

/// Computes and sends a `input_latency_report` telemetry event for the given
/// window if enough frames have been recorded since the last report.
///
/// Call this periodically (e.g. every five minutes) from a spawned task. A
/// separate baseline snapshot is kept per window so user-facing histogram dumps
/// and telemetry never share state.
pub fn report_input_latency_telemetry(window: &Window, cx: &mut App) {
    let current = window.input_latency_snapshot();
    let window_id = window.window_handle().window_id();

    let open_window_ids = open_window_ids(cx);
    let state = cx.default_global::<TelemetryReporterState>();
    state
        .previous
        .retain(|window_id, _| open_window_ids.contains(window_id));
    let now = Instant::now();

    let (delta_latency, delta_coalesce, report_window_seconds) =
        if let Some((prev_instant, prev_snapshot)) = state.previous.get(&window_id) {
            let mut delta_latency = current.latency_histogram.clone();
            delta_latency
                .subtract(&prev_snapshot.latency_histogram)
                .ok();
            let mut delta_coalesce = current.events_per_frame_histogram.clone();
            delta_coalesce
                .subtract(&prev_snapshot.events_per_frame_histogram)
                .ok();
            let elapsed = now.duration_since(*prev_instant).as_secs();
            (delta_latency, delta_coalesce, elapsed)
        } else {
            // First report for this window: the full cumulative histogram is the
            // delta from the empty starting state. We don't know how long the
            // window has been open, so record 0 to signal that this is the
            // initial accumulation period rather than a fixed-width window.
            (
                current.latency_histogram.clone(),
                current.events_per_frame_histogram.clone(),
                0u64,
            )
        };

    let total_frames = delta_latency.len();
    if total_frames < MIN_FRAMES_TO_REPORT {
        return;
    }

    state.previous.insert(window_id, (now, current));

    let frames_sub4 = count_frames_in_range(&delta_latency, 0, MS4_NS);
    let frames_4to8 = count_frames_in_range(&delta_latency, MS4_NS, MS8_NS);
    let frames_8to16 = count_frames_in_range(&delta_latency, MS8_NS, MS16_NS);
    let frames_16to33 = count_frames_in_range(&delta_latency, MS16_NS, MS33_NS);
    let frames_33to100 = count_frames_in_range(&delta_latency, MS33_NS, MS100_NS);
    // frames > 100 ms are implicitly total_frames - (sub4 + 4to8 + 8to16 + 16to33 + 33to100)

    let frames_with_1_event = count_frames_in_range(&delta_coalesce, 1, 2);
    let frames_with_2_events = count_frames_in_range(&delta_coalesce, 2, 3);
    let frames_with_3_events = count_frames_in_range(&delta_coalesce, 3, 4);
    // frames with 4+ events are implicitly total_frames - (1 + 2 + 3)

    telemetry::event!(
        "Latency Report",
        frames_sub4 = frames_sub4,
        frames_4to8 = frames_4to8,
        frames_8to16 = frames_8to16,
        frames_16to33 = frames_16to33,
        frames_33to100 = frames_33to100,
        total_frames = total_frames,
        frames_with_1_event = frames_with_1_event,
        frames_with_2_events = frames_with_2_events,
        frames_with_3_events = frames_with_3_events,
        report_window_seconds = report_window_seconds,
    );
}

/// Per-window baselines for frame-duration telemetry, keyed by window id. Kept
/// separate from the input-latency baselines so the two reports never share
/// state.
#[derive(Default)]
struct FrameDurationTelemetryState {
    previous: HashMap<WindowId, (Instant, FrameDurationSnapshot)>,
}

impl Global for FrameDurationTelemetryState {}

/// Nanosecond boundaries for the present-interval buckets used in telemetry:
/// roughly the 120Hz, 60Hz, and 30Hz frame budgets, with headroom for jitter.
const MS9_NS: u64 = 9_000_000;
const MS18_NS: u64 = 18_000_000;
const MS36_NS: u64 = 36_000_000;

/// Minimum number of draws that must be present in the delta window for the
/// frame-duration report to be sent. Avoids sending noise for windows that are
/// mostly idle.
const MIN_DRAWS_TO_REPORT: u64 = 1_000;

/// Computes and sends a `Frame Duration Report` telemetry event for the given
/// window if enough frames were drawn since the last report.
///
/// The report contains bucketed draw durations (how long `Window::draw` took),
/// bucketed present intervals (the achieved frame-to-frame cadence while the
/// window was animating), and the average dirty-to-present duration.
///
/// Call this periodically from a spawned task.
pub fn report_frame_duration_telemetry(window: &Window, cx: &mut App) {
    let current = window.frame_duration_snapshot();
    let window_handle = window.window_handle();
    let window_id = window_handle.window_id();

    let open_window_ids = open_window_ids(cx);
    let state = cx.default_global::<FrameDurationTelemetryState>();
    state
        .previous
        .retain(|window_id, _| open_window_ids.contains(window_id));
    let now = Instant::now();

    let (delta_draws, delta_intervals, delta_dirty_to_present, report_window_seconds) =
        if let Some((prev_instant, prev_snapshot)) = state.previous.get(&window_id) {
            let mut delta_draws = current.draw_duration_histogram.clone();
            delta_draws
                .subtract(&prev_snapshot.draw_duration_histogram)
                .ok();
            let mut delta_intervals = current.present_interval_histogram.clone();
            delta_intervals
                .subtract(&prev_snapshot.present_interval_histogram)
                .ok();
            let mut delta_dirty_to_present = current.dirty_to_present_histogram.clone();
            if delta_dirty_to_present
                .subtract(&prev_snapshot.dirty_to_present_histogram)
                .is_err()
            {
                delta_dirty_to_present = current.dirty_to_present_histogram.clone();
            }
            let elapsed = now.duration_since(*prev_instant).as_secs();
            (
                delta_draws,
                delta_intervals,
                delta_dirty_to_present,
                elapsed,
            )
        } else {
            // First report for this window: the full cumulative histograms are
            // the delta from the empty starting state. We don't know how long
            // the window has been open, so record 0 to signal that this is the
            // initial accumulation period rather than a fixed-width window.
            (
                current.draw_duration_histogram.clone(),
                current.present_interval_histogram.clone(),
                current.dirty_to_present_histogram.clone(),
                0u64,
            )
        };

    let total_draws = delta_draws.len();
    if total_draws < MIN_DRAWS_TO_REPORT {
        return;
    }

    state.previous.insert(window_id, (now, current));

    let draws_sub4 = count_frames_in_range(&delta_draws, 0, MS4_NS);
    let draws_4to8 = count_frames_in_range(&delta_draws, MS4_NS, MS8_NS);
    let draws_8to16 = count_frames_in_range(&delta_draws, MS8_NS, MS16_NS);
    let draws_16to33 = count_frames_in_range(&delta_draws, MS16_NS, MS33_NS);
    // draws > 33ms are implicitly total_draws - (sub4 + 4to8 + 8to16 + 16to33)

    let total_intervals = delta_intervals.len();
    let intervals_sub9 = count_frames_in_range(&delta_intervals, 0, MS9_NS);
    let intervals_9to18 = count_frames_in_range(&delta_intervals, MS9_NS, MS18_NS);
    let intervals_18to36 = count_frames_in_range(&delta_intervals, MS18_NS, MS36_NS);
    let intervals_36to100 = count_frames_in_range(&delta_intervals, MS36_NS, MS100_NS);
    // intervals > 100ms are implicitly total_intervals - (the buckets above)
    let average_dirty_to_present_ms = delta_dirty_to_present.mean() / 1_000_000.0;

    telemetry::event!(
        "Frame Duration Report",
        draws_sub4 = draws_sub4,
        draws_4to8 = draws_4to8,
        draws_8to16 = draws_8to16,
        draws_16to33 = draws_16to33,
        total_draws = total_draws,
        intervals_sub9 = intervals_sub9,
        intervals_9to18 = intervals_9to18,
        intervals_18to36 = intervals_18to36,
        intervals_36to100 = intervals_36to100,
        total_intervals = total_intervals,
        average_dirty_to_present_ms = average_dirty_to_present_ms,
        root_entity_type_name = window_handle.root_entity_type_name(),
        report_window_seconds = report_window_seconds,
    );
}

fn open_window_ids(cx: &App) -> HashSet<WindowId> {
    cx.windows()
        .into_iter()
        .map(|window| window.window_id())
        .collect()
}

fn count_frames_in_range(histogram: &Histogram<u64>, low_ns: u64, high_ns: u64) -> u64 {
    histogram
        .iter_recorded()
        .filter(|v| v.value_iterated_to() >= low_ns && v.value_iterated_to() < high_ns)
        .map(|v| v.count_at_value())
        .sum()
}

fn format_report(data: &InputLatencyReportData) -> String {
    let snapshot = &data.snapshot;
    let histogram = &snapshot.latency_histogram;
    let total = histogram.len();

    if total == 0 {
        return "No input latency samples recorded yet.\n\nTry typing or clicking in a buffer first.".to_string();
    }

    let percentiles: &[(&str, f64)] = &[
        ("min  ", 0.0),
        ("p50  ", 0.50),
        ("p75  ", 0.75),
        ("p90  ", 0.90),
        ("p95  ", 0.95),
        ("p99  ", 0.99),
        ("p99.9", 0.999),
        ("max  ", 1.0),
    ];

    let now = data.timestamp;

    let mut report = String::new();
    report.push_str("Input Latency Histogram\n");
    report.push_str("=======================\n");

    let timestamp = now.format("%Y-%m-%d %H:%M:%S %Z");
    report.push_str(&format!("Timestamp: {timestamp}\n"));
    if let Some(reported_by) = &data.reported_by {
        report.push_str(&format!("Reported from {reported_by}'s machine\n"));
    }
    report.push_str(&format!("Samples: {total}\n"));
    if snapshot.mid_draw_events_dropped > 0 {
        report.push_str(&format!(
            "Mid-draw events excluded: {}\n",
            snapshot.mid_draw_events_dropped
        ));
    }

    write_latency_percentiles(&mut report, "Percentiles", histogram, percentiles);
    write_latency_distribution(&mut report, "Distribution", histogram);

    let coalesce = &snapshot.events_per_frame_histogram;
    let coalesce_total = coalesce.len();
    if coalesce_total > 0 {
        report.push('\n');
        report.push_str("Events coalesced per frame:\n");
        for (label, quantile) in percentiles {
            let value = if *quantile == 0.0 {
                coalesce.min()
            } else if *quantile == 1.0 {
                coalesce.max()
            } else {
                coalesce.value_at_quantile(*quantile)
            };
            report.push_str(&format!("  {label}: {value:>6} events\n"));
        }

        report.push('\n');
        report.push_str("Distribution:\n");
        let bar_width = 30usize;
        let max_count = coalesce.max();
        for n in 1..=max_count {
            let count = coalesce
                .iter_recorded()
                .filter(|value| value.value_iterated_to() == n)
                .map(|value| value.count_at_value())
                .sum::<u64>();
            if count == 0 {
                continue;
            }
            let fraction = count as f64 / coalesce_total as f64;
            let bar_len = (fraction * bar_width as f64) as usize;
            let bar = "\u{2588}".repeat(bar_len);
            report.push_str(&format!(
                "  {n:>6} events: {count:>6} ({:>5.1}%) {bar}\n",
                fraction * 100.0,
            ));
        }
    }

    // Delta section: compare against the previous report's snapshot.
    if let (Some(prev_snapshot), Some(prev_timestamp)) =
        (&data.previous_snapshot, &data.previous_timestamp)
    {
        let prev_latency = &prev_snapshot.latency_histogram;
        let prev_total = prev_latency.len();
        let delta_total = total - prev_total;

        report.push('\n');
        report.push_str("Delta Since Last Report\n");
        report.push_str("-----------------------\n");
        let prev_ts = prev_timestamp.format("%Y-%m-%d %H:%M:%S %Z");
        let elapsed_secs = (now - *prev_timestamp).num_seconds().max(0);
        report.push_str(&format!(
            "Previous report: {prev_ts} ({elapsed_secs}s ago)\n"
        ));
        report.push_str(&format!("New samples: {delta_total}\n"));

        if delta_total > 0 {
            let mut delta_histogram = histogram.clone();
            delta_histogram.subtract(prev_latency).ok();

            write_latency_percentiles(
                &mut report,
                "Percentiles (new samples only)",
                &delta_histogram,
                percentiles,
            );
            write_latency_distribution(
                &mut report,
                "Distribution (new samples only)",
                &delta_histogram,
            );
        }
    }

    report
}

fn write_latency_percentiles(
    report: &mut String,
    heading: &str,
    histogram: &Histogram<u64>,
    percentiles: &[(&str, f64)],
) {
    let ns_to_ms = |ns: u64| ns as f64 / 1_000_000.0;

    report.push('\n');
    report.push_str(heading);
    report.push_str(":\n");
    for (label, quantile) in percentiles {
        let value_ns = if *quantile == 0.0 {
            histogram.min()
        } else if *quantile == 1.0 {
            histogram.max()
        } else {
            histogram.value_at_quantile(*quantile)
        };
        let hz = if value_ns > 0 {
            1_000_000_000.0 / value_ns as f64
        } else {
            f64::INFINITY
        };
        report.push_str(&format!(
            "  {label}: {:>8.2}ms  ({:>7.1} Hz)\n",
            ns_to_ms(value_ns),
            hz
        ));
    }
}

fn write_latency_distribution(report: &mut String, heading: &str, histogram: &Histogram<u64>) {
    const BUCKETS: &[(u64, u64, &str, &str)] = &[
        (0, 4_000_000, "0\u{2013}4ms", "(excellent)"),
        (4_000_000, 8_000_000, "4\u{2013}8ms", "(120fps)"),
        (8_000_000, 16_000_000, "8\u{2013}16ms", "(60fps)"),
        (16_000_000, 33_000_000, "16\u{2013}33ms", "(30fps)"),
        (33_000_000, 100_000_000, "33\u{2013}100ms", ""),
        (100_000_000, u64::MAX, "100ms+", "(sluggish)"),
    ];
    let bar_width = 30usize;
    let total = histogram.len() as f64;

    report.push('\n');
    report.push_str(heading);
    report.push_str(":\n");
    for (low, high, range, note) in BUCKETS {
        let count: u64 = histogram
            .iter_recorded()
            .filter(|value| value.value_iterated_to() >= *low && value.value_iterated_to() < *high)
            .map(|value| value.count_at_value())
            .sum();
        let fraction = if total > 0.0 {
            count as f64 / total
        } else {
            0.0
        };
        let bar_len = (fraction * bar_width as f64) as usize;
        let bar = "\u{2588}".repeat(bar_len);
        report.push_str(&format!(
            "  {range:>8}  {note:<11}: {count:>6} ({:>5.1}%) {bar}\n",
            fraction * 100.0,
        ));
    }
}
