use std::{collections::VecDeque, time::Instant};

use gpui::SharedString;

use super::JobId;

pub struct GitJobDebugQueue {
    pending: VecDeque<PendingJob>,
    running: VecDeque<RunningJob>,
    completed: VecDeque<CompletedJob>,
}

const MAX_COMPLETED_JOBS: usize = 500;

#[derive(Clone, Debug)]
pub struct PendingJob {
    pub id: JobId,
    pub description: SharedString,
    pub key: Option<SharedString>,
    pub enqueued_at: Instant,
}

#[derive(Clone, Debug)]
pub struct RunningJob {
    pub id: JobId,
    pub description: SharedString,
    pub key: Option<SharedString>,
    pub enqueued_at: Instant,
    pub started_at: Instant,
}

#[derive(Clone, Debug)]
pub struct CompletedJob {
    pub id: JobId,
    pub description: SharedString,
    pub key: Option<SharedString>,
    pub enqueued_at: Instant,
    pub started_at: Option<Instant>,
    pub completed_at: Instant,
    pub status: CompletedJobStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompletedJobStatus {
    Finished,
    Skipped,
}

impl GitJobDebugQueue {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            running: VecDeque::new(),
            completed: VecDeque::new(),
        }
    }

    pub fn add(&mut self, id: JobId, description: &'static str, key: Option<SharedString>) {
        self.pending.push_back(PendingJob {
            id,
            description: description.into(),
            key,
            enqueued_at: Instant::now(),
        });
    }

    pub fn mark_running(&mut self, id: JobId) {
        let Some(index) = self.pending.iter().position(|job| job.id == id) else {
            return;
        };
        // Safe to unwrap: `index` was just found by `position()`, so it's in bounds.
        let pending = self.pending.remove(index).unwrap();

        self.running.push_back(RunningJob {
            id: pending.id,
            description: pending.description,
            key: pending.key,
            enqueued_at: pending.enqueued_at,
            started_at: Instant::now(),
        });
    }

    pub fn mark_complete(&mut self, id: JobId, status: CompletedJobStatus) {
        let (enqueued_at, started_at, description, key) =
            if let Some(index) = self.running.iter().position(|job| job.id == id) {
                let running = self.running.remove(index).unwrap();
                (
                    running.enqueued_at,
                    Some(running.started_at),
                    running.description,
                    running.key,
                )
            } else if let Some(index) = self.pending.iter().position(|job| job.id == id) {
                let pending = self.pending.remove(index).unwrap();
                (pending.enqueued_at, None, pending.description, pending.key)
            } else {
                return;
            };

        self.completed.push_back(CompletedJob {
            id,
            description,
            key,
            enqueued_at,
            started_at,
            completed_at: Instant::now(),
            status,
        });

        while self.completed.len() > MAX_COMPLETED_JOBS {
            self.completed.pop_front();
        }
    }

    pub fn to_debug_string(&self) -> String {
        let mut entries = Vec::new();

        let mut pending_count = 0u64;
        let mut running_count = 0u64;
        let mut finished_count = 0u64;
        let mut skipped_count = 0u64;

        for job in &self.pending {
            pending_count += 1;
            entries.push((job.enqueued_at, self.format_pending(job)));
        }
        for job in &self.running {
            running_count += 1;
            entries.push((job.enqueued_at, self.format_running(job)));
        }
        for job in &self.completed {
            match job.status {
                CompletedJobStatus::Finished => finished_count += 1,
                CompletedJobStatus::Skipped => skipped_count += 1,
            }
            entries.push((job.enqueued_at, self.format_completed(job)));
        }

        entries.sort_by_key(|(enqueued_at, _)| *enqueued_at);

        let json_entries: Vec<serde_json::Value> =
            entries.into_iter().map(|(_, json)| json).collect();

        let json = serde_json::json!({
            "summary": {
                "pending": pending_count,
                "running": running_count,
                "finished": finished_count,
                "skipped": skipped_count,
            },
            "entries": json_entries,
        });

        serde_json::to_string_pretty(&json).unwrap_or_default()
    }

    fn format_pending(&self, job: &PendingJob) -> serde_json::Value {
        serde_json::json!({
            "id": job.id,
            "description": job.description.as_ref(),
            "key": job.key.as_ref().map(|k| k.as_ref()),
            "status": "Pending",
            "enqueued": format!("{} ago", format_duration(job.enqueued_at.elapsed())),
        })
    }

    fn format_running(&self, job: &RunningJob) -> serde_json::Value {
        serde_json::json!({
            "id": job.id,
            "description": job.description.as_ref(),
            "key": job.key.as_ref().map(|k| k.as_ref()),
            "status": "Running",
            "enqueued": format!("{} ago", format_duration(job.enqueued_at.elapsed())),
            "wait_time": format_duration(job.started_at.duration_since(job.enqueued_at)),
            "run_time": format!("{} (still running)", format_duration(job.started_at.elapsed())),
        })
    }

    fn format_completed(&self, job: &CompletedJob) -> serde_json::Value {
        let status = match job.status {
            CompletedJobStatus::Finished => "Finished",
            CompletedJobStatus::Skipped => "Skipped",
        };

        let (wait_time, run_time) = if let Some(started) = job.started_at {
            let wait = format_duration(started.duration_since(job.enqueued_at));
            let run = format_duration(job.completed_at.duration_since(started));
            (wait, Some(run))
        } else {
            let wait = format!(
                "{} (skipped)",
                format_duration(job.completed_at.duration_since(job.enqueued_at))
            );
            (wait, None)
        };

        serde_json::json!({
            "id": job.id,
            "description": job.description.as_ref(),
            "key": job.key.as_ref().map(|k| k.as_ref()),
            "status": status,
            "enqueued": format!("{} ago", format_duration(job.enqueued_at.elapsed())),
            "wait_time": wait_time,
            "run_time": run_time,
        })
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs_f64();
    if secs < 0.001 {
        format!("{:.0}us", secs * 1_000_000.0)
    } else if secs < 1.0 {
        format!("{:.0}ms", secs * 1000.0)
    } else if secs < 60.0 {
        format!("{:.0}s", secs)
    } else if secs < 3600.0 {
        format!("{:.0}m", secs / 60.0)
    } else {
        format!("{:.0}h", secs / 3600.0)
    }
}
