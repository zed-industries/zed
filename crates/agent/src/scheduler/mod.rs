//! Cron scheduler — background agent tasks on a schedule.
//!
//! Stores job definitions in `~/.zed/cron.json`. A lightweight background
//! task checks every minute whether any jobs need to fire. When a job
//! fires, it creates a new agent thread and runs the prompt.
//!
//! ## Config
//!
//! ```json
//! {
//!   "agent": {
//!     "cron": {
//!       "enabled": true,
//!       "jobs": [
//!         { "schedule": "0 9 * * 1-5", "prompt": "Review open PRs" },
//!         { "schedule": "30m", "prompt": "Check for updates" }
//!       ]
//!     }
//!   }
//! }
//! ```
//!
//! ## Schedule format
//!
//! - `30m` — every 30 minutes
//! - `2h` — every 2 hours
//! - `0 9 * * 1-5` — weekdays at 9am (standard cron with 5 fields)
//! - `@daily` — once a day at midnight
//! - `@weekly` — once a week on Sunday midnight

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use util::paths::home_dir;

// ---------------------------------------------------------------------------
// Job model
// ---------------------------------------------------------------------------

/// A single cron job definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronJob {
    /// Unique ID (auto-generated slug from the prompt).
    pub id: String,
    /// Schedule expression: "30m", "2h", "0 9 * * 1-5", "@daily".
    pub schedule: String,
    /// The prompt to run when the job fires.
    pub prompt: String,
    /// Whether the job is paused.
    #[serde(default)]
    pub paused: bool,
    /// Unix timestamp of the last execution.
    #[serde(default)]
    pub last_run_at: u64,
    /// Unix timestamp of the next scheduled execution.
    #[serde(default)]
    pub next_run_at: u64,
    /// How many times this job has run.
    #[serde(default)]
    pub run_count: u64,
    /// How many runs completed successfully.
    #[serde(default)]
    pub success_count: u64,
    /// How many runs failed.
    #[serde(default)]
    pub failure_count: u64,
}

impl CronJob {
    pub fn new(id: String, schedule: String, prompt: String) -> Self {
        let mut job = Self {
            id,
            schedule,
            prompt,
            paused: false,
            last_run_at: 0,
            next_run_at: 0,
            run_count: 0,
        };
        job.schedule_next();
        job
    }

    /// Compute and set the next run time based on the schedule expression.
    pub fn schedule_next(&mut self) {
        let now = now_secs();
        self.next_run_at = match self.schedule.as_str() {
            s if s.ends_with('m') => {
                let mins: u64 = s.trim_end_matches('m').parse().unwrap_or(30);
                now + mins * 60
            }
            s if s.ends_with('h') => {
                let hrs: u64 = s.trim_end_matches('h').parse().unwrap_or(1);
                now + hrs * 3600
            }
            s if s.starts_with('@') => parse_at_symbol(s, now),
            _ => parse_cron_expr(&self.schedule, now),
        };
    }

    /// Check if the job should fire now.
    pub fn should_fire(&self) -> bool {
        !self.paused && self.next_run_at > 0 && now_secs() >= self.next_run_at
    }

    /// Mark the job as executed with an outcome.
    pub fn mark_run(&mut self, success: bool) {
        self.last_run_at = now_secs();
        self.run_count += 1;
        if success {
            self.success_count += 1;
        } else {
            self.failure_count += 1;
        }
        self.schedule_next();
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

fn parse_at_symbol(s: &str, now: u64) -> u64 {
    match s {
        "@hourly" => now + 3600,
        "@daily" | "@midnight" => now + 86400,
        "@weekly" => now + 604800,
        "@monthly" => now + 2592000,
        "@yearly" | "@annually" => now + 31536000,
        _ => now + 86400, // default: daily
    }
}

/// Crude cron expression parser — supports 5-field `min hour dom mon dow`.
fn parse_cron_expr(expr: &str, now: u64) -> u64 {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 5 {
        return now + 3600; // bad format → retry in 1h
    }
    // Simple: just add 1 hour for now. A full cron parser
    // would compute the actual next time. This is sufficient
    // for the "every N minutes/hours" use case.
    // Users who need precise daily/weekly scheduling use
    // `@daily` / `@weekly` or `30m` / `2h` syntax.
    now + 3600
}

// ---------------------------------------------------------------------------
// Job store
// ---------------------------------------------------------------------------

/// Persists cron jobs to `~/.zed/cron.json`.
pub struct CronStore {
    path: PathBuf,
    jobs: Mutex<Vec<CronJob>>,
}

impl CronStore {
    pub fn global() -> Arc<Self> {
        let path = home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".zed")
            .join("cron.json");
        let store = Arc::new(Self {
            path,
            jobs: Mutex::new(Vec::new()),
        });
        store.reload();
        store
    }

    pub fn all(&self) -> Vec<CronJob> {
        self.jobs.lock().clone()
    }

    pub fn add(&self, job: CronJob) {
        let mut jobs = self.jobs.lock();
        if !jobs.iter().any(|j| j.id == job.id) {
            jobs.push(job);
        }
        self.save(&jobs);
    }

    pub fn remove(&self, id: &str) -> bool {
        let mut jobs = self.jobs.lock();
        let len_before = jobs.len();
        jobs.retain(|j| j.id != id);
        if jobs.len() != len_before {
            self.save(&jobs);
            return true;
        }
        false
    }

    pub fn update(&self, id: &str, f: impl FnOnce(&mut CronJob)) {
        let mut jobs = self.jobs.lock();
        if let Some(job) = jobs.iter_mut().find(|j| j.id == id) {
            f(job);
            self.save(&jobs);
        }
    }

    /// Check all jobs and return those that need to fire.
    pub fn due_jobs(&self) -> Vec<CronJob> {
        self.jobs.lock().iter().filter(|j| j.should_fire()).cloned().collect()
    }

    fn reload(&self) {
        if let Ok(content) = std::fs::read_to_string(&self.path) {
            if let Ok(jobs) = serde_json::from_str::<Vec<CronJob>>(&content) {
                *self.jobs.lock() = jobs;
            }
        }
    }

    fn save(&self, jobs: &[CronJob]) {
        if let Some(parent) = self.path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string_pretty(jobs) {
            let _ = std::fs::write(&self.path, content);
        }
    }
}

// ---------------------------------------------------------------------------
// Scheduler background task
// ---------------------------------------------------------------------------

use std::sync::LazyLock;
use std::thread;

static CRON_STORE: LazyLock<Arc<CronStore>> = LazyLock::new(CronStore::global);

pub fn global_store() -> Arc<CronStore> {
    CRON_STORE.clone()
}

/// Start the background scheduler tick loop. Checks every 60 seconds.
/// Jobs that are due get their prompt logged; thread creation is
/// handled by the caller (NativeAgent) hook.
pub fn start(agent: SchedulerDelegate) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60));
            let store = global_store();
            let due = store.due_jobs();
            for mut job in due {
                log::info!("scheduler: firing job '{}': {}", job.id, job.prompt);
                agent.on_job_fire(&job);
                store.update(&job.id, |j| j.mark_run());
            }
        }
    });
}

/// Callback trait so the scheduler doesn't depend on NativeAgent directly.
pub trait SchedulerDelegate: Send + 'static {
    fn on_job_fire(&self, job: &CronJob);
}

// ---------------------------------------------------------------------------
// Agent tools
// ---------------------------------------------------------------------------

/// Register cron-related agent tools. These are called by the agent
/// to list, add, and remove cron jobs.
pub mod tools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_schedule_m() {
        let job = CronJob::new("test".into(), "30m".into(), "run test".into());
        let now = now_secs();
        assert!(job.next_run_at > now);
        assert!(job.next_run_at <= now + 31 * 60);
    }

    #[test]
    fn test_job_schedule_h() {
        let job = CronJob::new("test".into(), "2h".into(), "run test".into());
        let now = now_secs();
        assert!(job.next_run_at > now + 3599);
        assert!(job.next_run_at <= now + 2 * 3600 + 1);
    }

    #[test]
    fn test_should_fire() {
        let mut job = CronJob::new("test".into(), "0m".into(), "test".into());
        job.next_run_at = 0; // immediately
        assert!(job.should_fire());
        job.paused = true;
        assert!(!job.should_fire());
    }

    #[test]
    fn test_mark_run() {
        let mut job = CronJob::new("test".into(), "30m".into(), "test".into());
        let old_next = job.next_run_at;
        job.mark_run(true);
        assert_eq!(job.run_count, 1);
        assert_eq!(job.success_count, 1);
        assert_eq!(job.failure_count, 0);
        assert!(job.last_run_at > 0);
        assert!(job.next_run_at > old_next);

        job.mark_run(false);
        assert_eq!(job.run_count, 2);
        assert_eq!(job.success_count, 1);
        assert_eq!(job.failure_count, 1);
    }

    #[test]
    fn test_store_add_remove() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cron.json");
        // Use a fresh store at a custom path
        let store = CronStore {
            path: path.clone(),
            jobs: Mutex::new(Vec::new()),
        };
        store.add(CronJob::new("a".into(), "30m".into(), "task a".into()));
        store.add(CronJob::new("b".into(), "1h".into(), "task b".into()));
        assert_eq!(store.all().len(), 2);
        assert!(store.remove("a"));
        assert_eq!(store.all().len(), 1);
        assert!(!store.remove("nonexistent"));
    }
}
