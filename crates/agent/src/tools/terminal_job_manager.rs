use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, AsyncApp, Global};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use crate::thread::TerminalHandle;

/// Job status for background terminal execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TerminalJobStatus {
    Running,
    Completed,
    Failed,
    Canceled,
}

/// Record of a background terminal job
#[derive(Debug, Clone, Serialize)]
pub struct TerminalJobRecord {
    pub job_id: String,
    pub command: String,
    pub working_dir: String,
    pub started_at: SystemTime,
    pub finished_at: Option<SystemTime>,
    pub status: TerminalJobStatus,
    pub exit_code: Option<i32>,
    pub terminal_id: acp::TerminalId,
    pub output: String,
    pub last_read_position: usize,
}

impl TerminalJobRecord {
    /// Get duration of the job (elapsed or total if finished)
    pub fn duration(&self) -> Option<Duration> {
        let end_time = self.finished_at.unwrap_or_else(SystemTime::now);
        end_time.duration_since(self.started_at).ok()
    }

    /// Get duration as formatted string
    pub fn duration_string(&self) -> String {
        self.duration()
            .map(|d| {
                if self.finished_at.is_some() {
                    format!("{:.2}s", d.as_secs_f64())
                } else {
                    format!("{:.2}s (running)", d.as_secs_f64())
                }
            })
            .unwrap_or_else(|| "unknown".to_string())
    }
}

/// Global terminal job registry
#[derive(Clone)]
pub struct TerminalJobManager {
    jobs: Arc<Mutex<HashMap<String, TerminalJobRecord>>>,
    job_counter: Arc<Mutex<u64>>,
    terminal_handles: Arc<Mutex<HashMap<String, Rc<dyn TerminalHandle>>>>,
}

impl Global for TerminalJobManager {}

impl TerminalJobManager {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
            job_counter: Arc::new(Mutex::new(1)),
            terminal_handles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Initialize the global job manager
    pub fn init_global(cx: &mut App) {
        cx.set_global(Self::new());
    }

    /// Get the global job manager instance
    pub fn global(cx: &App) -> Self {
        cx.global::<Self>().clone()
    }

    /// Generate a new unique job ID
    pub fn new_job_id(&self) -> String {
        let mut counter = self.job_counter.lock().unwrap();
        let id = *counter;
        *counter += 1;
        format!("terminal-job-{}", id)
    }

    /// Register a new job with its terminal handle
    pub fn register_job(
        &self,
        job_id: String,
        command: String,
        working_dir: String,
        terminal_id: acp::TerminalId,
        terminal_handle: Rc<dyn TerminalHandle>,
    ) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.insert(
            job_id.clone(),
            TerminalJobRecord {
                job_id: job_id.clone(),
                command,
                working_dir,
                started_at: SystemTime::now(),
                finished_at: None,
                status: TerminalJobStatus::Running,
                exit_code: None,
                terminal_id,
                output: String::new(),
                last_read_position: 0,
            },
        );

        // Store the terminal handle
        let mut handles = self.terminal_handles.lock().unwrap();
        handles.insert(job_id, terminal_handle);
    }

    /// Update read position for incremental output tracking
    pub fn update_read_position(&self, job_id: &str, position: usize) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.last_read_position = position;
        }
    }

    /// Complete a job with exit status and final output
    pub fn complete_job(&self, job_id: &str, exit_code: Option<i32>, output: String) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.finished_at = Some(SystemTime::now());
            job.exit_code = exit_code;
            job.output = output;
            job.status = if exit_code == Some(0) {
                TerminalJobStatus::Completed
            } else {
                TerminalJobStatus::Failed
            };

            // Remove the terminal handle since the job is done
            let mut handles = self.terminal_handles.lock().unwrap();
            handles.remove(job_id);
        }
    }

    /// Update job output (for incremental updates while running)
    pub fn update_output(&self, job_id: &str, output: String) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.output = output;
        }
    }

    /// Get incremental output (only new since last read)
    pub fn get_incremental_output(&self, job_id: &str) -> Option<(String, bool)> {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            let new_output = job.output[job.last_read_position..].to_string();
            job.last_read_position = job.output.len();
            let is_running = matches!(job.status, TerminalJobStatus::Running);
            Some((new_output, is_running))
        } else {
            None
        }
    }

    /// Get full output
    pub fn get_full_output(&self, job_id: &str) -> Option<String> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(job_id).map(|job| job.output.clone())
    }

    /// Cancel a job by killing its terminal process
    pub fn cancel_job(&self, job_id: &str, cx: &AsyncApp) -> Result<()> {
        // First check if job exists and is running
        {
            let jobs = self.jobs.lock().unwrap();
            if let Some(job) = jobs.get(job_id) {
                if !matches!(job.status, TerminalJobStatus::Running) {
                    anyhow::bail!("Job is not running");
                }
            } else {
                anyhow::bail!("Job not found");
            }
        }

        // Get and kill the terminal
        {
            let handles = self.terminal_handles.lock().unwrap();
            if let Some(terminal) = handles.get(job_id) {
                terminal.kill(cx)?;
            }
        }

        // Mark as canceled
        {
            let mut jobs = self.jobs.lock().unwrap();
            if let Some(job) = jobs.get_mut(job_id) {
                job.status = TerminalJobStatus::Canceled;
                job.finished_at = Some(SystemTime::now());
            }
        }

        // Remove the terminal handle
        {
            let mut handles = self.terminal_handles.lock().unwrap();
            handles.remove(job_id);
        }

        Ok(())
    }

    /// Get a job by ID
    pub fn get_job(&self, job_id: &str) -> Option<TerminalJobRecord> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(job_id).cloned()
    }

    /// Reset read position to get all output again
    pub fn reset_read_position(&self, job_id: &str) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.last_read_position = 0;
        }
    }

    /// List all jobs
    pub fn list_jobs(&self) -> Vec<TerminalJobRecord> {
        let jobs = self.jobs.lock().unwrap();
        let mut job_list: Vec<TerminalJobRecord> = jobs.values().cloned().collect();
        job_list.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        job_list
    }

    /// List jobs with filtering options
    pub fn list_jobs_filtered(
        &self,
        status_filter: Option<&[TerminalJobStatus]>,
        limit: Option<usize>,
    ) -> Vec<TerminalJobRecord> {
        let jobs = self.jobs.lock().unwrap();
        let mut job_list: Vec<TerminalJobRecord> =
            jobs.values()
                .filter(|job| {
                    // Filter by status
                    if let Some(statuses) = status_filter {
                        if !statuses.iter().any(|s| {
                            std::mem::discriminant(s) == std::mem::discriminant(&job.status)
                        }) {
                            return false;
                        }
                    }
                    true
                })
                .cloned()
                .collect();

        job_list.sort_by(|a, b| b.started_at.cmp(&a.started_at));

        if let Some(limit) = limit {
            job_list.truncate(limit);
        }

        job_list
    }

    /// Delete a job from history
    pub fn delete_job(&self, job_id: &str) -> Result<()> {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.remove(job_id)
            .ok_or_else(|| anyhow::anyhow!("Job not found"))?;
        Ok(())
    }

    /// Get count of running jobs
    pub fn running_count(&self) -> usize {
        let jobs = self.jobs.lock().unwrap();
        jobs.values()
            .filter(|job| matches!(job.status, TerminalJobStatus::Running))
            .count()
    }

    /// Clean up old completed jobs (older than specified duration)
    pub fn cleanup_old_jobs(&self, older_than: Duration) {
        let mut jobs = self.jobs.lock().unwrap();
        let now = SystemTime::now();
        jobs.retain(|_, job| {
            if matches!(job.status, TerminalJobStatus::Running) {
                return true;
            }
            if let Some(finished_at) = job.finished_at {
                if let Ok(elapsed) = now.duration_since(finished_at) {
                    return elapsed < older_than;
                }
            }
            true
        });
    }
}

impl Default for TerminalJobManager {
    fn default() -> Self {
        Self::new()
    }
}
