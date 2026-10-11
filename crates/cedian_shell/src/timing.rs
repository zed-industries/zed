//! Per-turn timing for the speed benchmark (ADR-0037, ADR-0058): with
//! `CEDIAN_TIMING` set to a file path, each spawn and each turn appends one
//! JSON line, the same from the CLI and the app. cedian overhead = spawn +
//! context + post; `omp_ms` is OMP's own time (prompt sent → `prompt_result`).

use serde_json::{Value, json};
use std::io::Write as _;
use std::path::Path;
use std::time::{Duration, Instant};

pub const TIMING_ENV: &str = "CEDIAN_TIMING";

/// One timing row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Row {
    /// OMP started and its session opened.
    Spawn(Duration),
    /// One turn, from four moments in order: the person sent it, OMP got
    /// the prompt, OMP's `prompt_result` came back, cedian's own work on
    /// the turn ended. A later moment that reads earlier counts as equal;
    /// an end before the return is flagged `settled_before_return_ms`.
    Turn {
        started: Instant,
        sent: Instant,
        returned: Instant,
        ended: Instant,
    },
}

impl Row {
    /// The JSON line. A turn's parts are cut from one clock, so
    /// `context_ms + omp_ms + post_ms == total_ms` always.
    pub fn to_json(&self) -> Value {
        match *self {
            Row::Spawn(d) => json!({"event": "spawn", "ms": ms(d)}),
            Row::Turn {
                started,
                sent,
                returned,
                ended,
            } => {
                let at = |t: Instant| ms(t.saturating_duration_since(started));
                let sent = at(sent);
                let returned = at(returned).max(sent);
                let total = at(ended).max(returned);
                let mut row = json!({
                    "event": "turn", "context_ms": sent, "omp_ms": returned - sent,
                    "post_ms": total - returned, "total_ms": total,
                });
                // cedian settled before OMP's prompt call returned: post_ms
                // reads 0, and the row says by how much the order was off.
                let early = returned.saturating_sub(at(ended));
                if early > 0 {
                    row["settled_before_return_ms"] = early.into();
                }
                row
            }
        }
    }
}

/// Append `row` to the file `CEDIAN_TIMING` names; nothing when unset.
pub fn record(row: Row) {
    if let Some(path) = std::env::var_os(TIMING_ENV) {
        record_to(Path::new(&path), row);
    }
}

fn record_to(path: &Path, row: Row) {
    let written = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut f| writeln!(f, "{}", row.to_json()));
    if let Err(e) = written {
        eprintln!("cedian: {TIMING_ENV}={}: {e}", path.display());
    }
}

pub fn ms(d: Duration) -> u64 {
    u64::try_from(d.as_millis()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_turns_parts_add_up_to_its_total() {
        let t0 = Instant::now();
        let at = |us: u64| t0 + Duration::from_micros(us);
        let row = Row::Turn {
            started: t0,
            sent: at(1_900),
            returned: at(5_800),
            ended: at(7_700),
        }
        .to_json();
        let part = |k: &str| row[k].as_u64().unwrap();
        assert_eq!(
            (
                part("context_ms"),
                part("omp_ms"),
                part("post_ms"),
                part("total_ms")
            ),
            (1, 4, 2, 7),
            "{row}"
        );
        assert!(row.get("settled_before_return_ms").is_none(), "{row}");
    }

    #[test]
    fn moments_out_of_order_count_as_equal() {
        let t0 = Instant::now();
        let at = |ms: u64| t0 + Duration::from_millis(ms);
        let row = Row::Turn {
            started: t0,
            sent: at(5),
            returned: at(9),
            ended: at(8),
        }
        .to_json();
        assert_eq!(row["post_ms"], 0, "{row}");
        assert_eq!(row["total_ms"], 9, "{row}");
        assert_eq!(row["settled_before_return_ms"], 1, "{row}");
    }

    #[test]
    fn rows_append_one_line_each() {
        let path = std::env::temp_dir().join(format!("cedian-timing-{}", std::process::id()));
        let _ = std::fs::remove_file(&path);
        record_to(&path, Row::Spawn(Duration::from_millis(12)));
        let t = Instant::now();
        record_to(
            &path,
            Row::Turn {
                started: t,
                sent: t,
                returned: t,
                ended: t,
            },
        );
        let lines: Vec<Value> = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(lines[0], json!({"event": "spawn", "ms": 12}));
        assert_eq!(lines[1]["event"], "turn");
        assert_eq!(lines.len(), 2);
        std::fs::remove_file(&path).unwrap();
    }
}
