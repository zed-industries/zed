use gpui::{App, Global, InputLatencySnapshot, Window, actions};
use hdrhistogram::Histogram;

actions!(
    dev,
    [
        /// Opens a buffer showing the input-to-frame latency histogram for the current window.
        DumpInputLatencyHistogram,
    ]
);

/// Generates a formatted text report of the input-to-frame latency histogram
/// for the given window. If a previous report was generated (tracked via a
/// global on the `App`), includes a delta section showing changes since that
/// report.
pub fn format_input_latency_report(window: &Window, cx: &mut App) -> String {
    let snapshot = window.input_latency_snapshot();
    let state = cx.default_global::<ReporterState>();
    let report = format_report(&snapshot, state);

    state.previous_snapshot = Some(snapshot);
    state.previous_timestamp = Some(chrono::Local::now());

    report
}

#[derive(Default)]
struct ReporterState {
    previous_snapshot: Option<InputLatencySnapshot>,
    previous_timestamp: Option<chrono::DateTime<chrono::Local>>,
}

impl Global for ReporterState {}

fn format_report(snapshot: &InputLatencySnapshot, previous: &ReporterState) -> String {
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

    let now = chrono::Local::now();

    let mut report = String::new();
    report.push_str("Input Latency Histogram\n");
    report.push_str("=======================\n");

    let timestamp = now.format("%Y-%m-%d %H:%M:%S %Z");
    report.push_str(&format!("Timestamp: {timestamp}\n"));
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
        (&previous.previous_snapshot, &previous.previous_timestamp)
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
