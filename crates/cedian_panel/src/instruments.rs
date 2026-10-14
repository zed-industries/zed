//! The benchmark's instruments in the app (ADR-0058 decision 3), each off
//! until its env var names a file: `CEDIAN_TIMING` gets the CLI's spawn and
//! turn rows (`cedian_shell::timing`), `CEDIAN_FRAMES` one row of draw
//! durations per OMP run, from GPUI's frame profiler.

use cedian_shell::timing::{self, Row};
use gpui::Window;
use gpui::profiler::FrameDurationSnapshot;
use serde_json::json;
use std::io::Write as _;
use std::time::Instant;

pub const FRAMES_ENV: &str = "CEDIAN_FRAMES";

#[derive(Default)]
pub(crate) struct Instruments {
    spawn: Option<Instant>,
    turn: Option<TurnClock>,
    stream: Option<FrameDurationSnapshot>,
}

/// One turn of ours: its row is written once OMP's prompt call has
/// returned and the panel has handled `Settled`, in either order.
struct TurnClock {
    started: Instant,
    span: Option<(Instant, Instant)>,
    ended: Option<Instant>,
}

impl Instruments {
    pub(crate) fn spawning(&mut self) {
        self.spawn = Some(Instant::now());
    }

    pub(crate) fn ready(&mut self) {
        if let Some(started) = self.spawn.take() {
            timing::record(Row::Spawn(started.elapsed()));
        }
    }

    /// The person sent a prompt at `started`; a turn left unfinished before
    /// is dropped.
    pub(crate) fn sent(&mut self, started: Instant) {
        self.turn = Some(TurnClock {
            started,
            span: None,
            ended: None,
        });
    }

    /// OMP got the prompt at `sent` and its `prompt_result` came back at
    /// `returned`.
    pub(crate) fn prompt_returned(&mut self, sent: Instant, returned: Instant) {
        if let Some(turn) = &mut self.turn {
            turn.span = Some((sent, returned));
        }
        self.flush();
    }

    /// The panel's own work on a settled turn is done.
    pub(crate) fn settled(&mut self) {
        if let Some(turn) = &mut self.turn {
            turn.ended.get_or_insert_with(Instant::now);
        }
        self.flush();
    }

    /// The prompt never ran: no row.
    pub(crate) fn abandoned(&mut self) {
        self.turn = None;
    }

    fn flush(&mut self) {
        if let Some(TurnClock {
            started,
            span: Some((sent, returned)),
            ended: Some(ended),
        }) = self.turn
        {
            self.turn = None;
            timing::record(Row::Turn {
                started,
                sent,
                returned,
                ended,
            });
        }
    }

    /// OMP started a run: its reply streams from here.
    pub(crate) fn stream_started(&mut self, window: &Window) {
        if std::env::var_os(FRAMES_ENV).is_some() {
            self.stream = Some(window.frame_duration_snapshot());
        }
    }

    /// The run settled: one row of the frames drawn while it streamed.
    pub(crate) fn stream_ended(&mut self, window: &Window) {
        let (Some(start), Some(path)) = (self.stream.take(), std::env::var_os(FRAMES_ENV)) else {
            return;
        };
        let row = frames_row(&start, &window.frame_duration_snapshot());
        let written = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .and_then(|mut f| writeln!(f, "{row}"));
        if let Err(e) = written {
            log::error!("cedian: {FRAMES_ENV}={}: {e}", path.display());
        }
    }
}

/// The draws between two snapshots of one window: count and p50, p95,
/// p99 and max in milliseconds.
pub fn frames_row(start: &FrameDurationSnapshot, end: &FrameDurationSnapshot) -> serde_json::Value {
    let mut draws = end.draw_duration_histogram.clone();
    // Should not happen (GPUI's histogram never resets), but if it does the
    // row is every draw since the window opened, and says so.
    let window_total = draws.subtract(&start.draw_duration_histogram).is_err();
    if window_total {
        draws = end.draw_duration_histogram.clone();
    }
    let ms = |ns: u64| (ns as f64 / 1e4).round() / 100.0;
    json!({
        "event": "frames",
        "frames": draws.len(),
        "draw_p50_ms": ms(draws.value_at_quantile(0.50)),
        "draw_p95_ms": ms(draws.value_at_quantile(0.95)),
        "draw_p99_ms": ms(draws.value_at_quantile(0.99)),
        "draw_max_ms": ms(draws.max()),
        "window_total": window_total,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use hdrhistogram::Histogram;

    fn snapshot(draws_ms: &[u64]) -> FrameDurationSnapshot {
        let mut draws = Histogram::<u64>::new(3).unwrap();
        for ms in draws_ms {
            draws.record(ms * 1_000_000).unwrap();
        }
        FrameDurationSnapshot {
            dirty_to_present_histogram: Histogram::new(3).unwrap(),
            draw_duration_histogram: draws,
            present_interval_histogram: Histogram::new(3).unwrap(),
            by_refresh_interval: Vec::new(),
        }
    }

    #[test]
    fn a_row_is_the_draws_between_its_snapshots() {
        let row = frames_row(&snapshot(&[100]), &snapshot(&[100, 4, 8]));
        assert_eq!(row["frames"], 2, "{row}");
        assert_eq!(row["window_total"], false, "{row}");
    }

    #[test]
    fn a_row_that_falls_back_to_the_whole_window_says_so() {
        let row = frames_row(&snapshot(&[100, 7]), &snapshot(&[4]));
        assert_eq!(row["window_total"], true, "{row}");
        assert_eq!(row["frames"], 1, "{row}");
    }
}
