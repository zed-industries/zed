//! Experiment-only: logs every window draw and what invalidated it.
//!
//! When `GPUI_DRAW_LOG` names a file, each draw appends one line:
//! `<NEXTEST_TEST_NAME>\t<window id>\t<cause>=<count>,...`, where the causes
//! are the entity types that notified, and refreshes, since the previous draw
//! on this thread.

use collections::FxHashMap;
use parking_lot::Mutex;
use std::{cell::RefCell, fs::File, io::Write as _, sync::OnceLock};

thread_local! {
    static CAUSES: RefCell<Vec<&'static str>> = const { RefCell::new(Vec::new()) };
}

fn log_file() -> Option<&'static Mutex<File>> {
    static FILE: OnceLock<Option<Mutex<File>>> = OnceLock::new();
    FILE.get_or_init(|| {
        let path = std::env::var_os("GPUI_DRAW_LOG")?;
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .ok()?;
        Some(Mutex::new(file))
    })
    .as_ref()
}

pub(crate) fn record_cause(cause: &'static str) {
    if log_file().is_some() {
        CAUSES.with(|causes| causes.borrow_mut().push(cause));
    }
}

pub(crate) fn record_draw(window_id: u64) {
    let Some(file) = log_file() else {
        return;
    };
    let causes = CAUSES.with(|causes| std::mem::take(&mut *causes.borrow_mut()));
    let mut counts = FxHashMap::<&'static str, usize>::default();
    for cause in causes {
        *counts.entry(cause).or_default() += 1;
    }
    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|left, right| right.1.cmp(&left.1));
    let causes = counts
        .iter()
        .map(|(cause, count)| format!("{cause}={count}"))
        .collect::<Vec<_>>()
        .join(",");
    let test = std::env::var("NEXTEST_TEST_NAME").unwrap_or_default();
    let mut line = format!("{test}\t{window_id}\t{causes}\n");
    line.retain(|character| character != '\r');
    file.lock().write_all(line.as_bytes()).ok();
}
