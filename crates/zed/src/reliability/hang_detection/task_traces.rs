use std::path::PathBuf;
use std::thread::ThreadId;

use anyhow::Context;
use gpui::{SerializedThreadTaskTimings, TasksIncluded, profiler};
use util::ResultExt;

use crate::STARTUP_TIME;

pub fn save_any(main_thread_id: ThreadId) -> Option<PathBuf> {
    cleanup_old_hang_traces();
    let thread_timings = gpui::profiler::get_all_timings(TasksIncluded::CompletedAndRunning);

    let thread_timings = thread_timings
        .into_iter()
        .map(|mut timings| {
            if timings.thread_id == main_thread_id {
                timings.thread_name = Some("main".to_string());
            }

            SerializedThreadTaskTimings::convert(*STARTUP_TIME.get().unwrap(), timings)
        })
        .collect::<Vec<_>>();

    let Some(timings) = serde_json::to_string(&thread_timings)
        .context("hang timings serialization")
        .log_err()
    else {
        return None;
    };

    if profiler::trace_enabled() {
        None
    } else {
        cleanup_old_hang_traces();
        let trace_path = paths::hang_traces_dir().join(&format!(
            "hang-{}.miniprof.json",
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
        ));
        std::fs::write(&trace_path, timings)
            .context("hang trace file writing")
            .log_err();
        Some(trace_path)
    }
}

pub fn cleanup_old_hang_traces() {
    if let Ok(entries) = std::fs::read_dir(paths::hang_traces_dir()) {
        let mut files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|ext| ext == "json" || ext == "miniprof")
            })
            .collect();

        const MAX_HANG_TRACES: usize = 3;
        if files.len() > MAX_HANG_TRACES {
            files.sort_by_key(|entry| entry.file_name());
            for entry in files.iter().take(files.len() - MAX_HANG_TRACES) {
                std::fs::remove_file(entry.path()).log_err();
            }
        }
    }
}
