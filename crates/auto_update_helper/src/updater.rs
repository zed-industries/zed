use std::path::{Path, PathBuf};

use anyhow::Result;
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::PostMessageW,
};

use crate::windows_impl::WM_JOB_UPDATED;

#[derive(Debug, PartialEq, Eq)]
enum UpdateStatus {
    RemoveOld(RemoveJob),
    CopyNew(CopyJob),
    Cleanup(CleanupJob),
    Done,
}

pub(crate) fn perform_update(app_dir: &Path, hwnd: isize) -> Result<()> {
    let hwnd = HWND(hwnd as _);
    let (remove_job, copy_job, cleanup_job) = collect_jobs(app_dir);

    let start = std::time::Instant::now();
    let mut status = UpdateStatus::RemoveOld(remove_job);
    while start.elapsed().as_secs() < 10 {
        match status {
            UpdateStatus::RemoveOld(old_files) => {
                log::info!("Removing old files: {:?}", old_files);
                if let Some(left_old_files) = old_files.run(hwnd)? {
                    status = UpdateStatus::RemoveOld(left_old_files);
                } else {
                    status = UpdateStatus::CopyNew(copy_job.clone());
                }
            }
            UpdateStatus::CopyNew(new_files) => {
                log::info!("Copying new files: {:?}", new_files);
                if let Some(left_new_files) = new_files.run(hwnd)? {
                    status = UpdateStatus::CopyNew(left_new_files);
                } else {
                    status = UpdateStatus::Cleanup(cleanup_job.clone());
                }
            }
            UpdateStatus::Cleanup(cleanup) => {
                log::info!("Cleaning up: {:?}", cleanup);
                if let Some(left_cleanup) = cleanup.run(hwnd)? {
                    status = UpdateStatus::Cleanup(left_cleanup);
                } else {
                    status = UpdateStatus::Done;
                }
            }
            UpdateStatus::Done => {
                let _ = std::process::Command::new(app_dir.join("Zed.exe")).spawn();
                break;
            }
        }
    }
    if status != UpdateStatus::Done {
        Err(anyhow::anyhow!("Failed to update Zed, timeout"))
    } else {
        log::info!("Zed updated successfully");
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RemoveJob(Vec<PathBuf>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct CopyJob(Vec<CopyDetails>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct CopyDetails {
    from: PathBuf,
    to: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CleanupJob(Vec<PathBuf>);

fn collect_jobs(appdir: &Path) -> (RemoveJob, CopyJob, CleanupJob) {
    let updates_dir = appdir.join("updates");
    let install_dir = appdir.join("install");
    (
        RemoveJob(vec![appdir.join("Zed.exe"), appdir.join("bin\\zed.exe")]),
        CopyJob(vec![
            CopyDetails {
                from: install_dir.join("Zed.exe"),
                to: appdir.join("Zed.exe"),
            },
            CopyDetails {
                from: install_dir.join("bin\\zed.exe"),
                to: appdir.join("bin\\zed.exe"),
            },
        ]),
        CleanupJob(vec![updates_dir, install_dir]),
    )
}

impl RemoveJob {
    #[cfg(not(debug_assertions))]
    fn run(self, hwnd: HWND) -> Result<Option<Self>> {
        let mut jobs = Vec::with_capacity(self.0.len());
        for old_file in self.0.into_iter() {
            if !old_file.exists() {
                log::warn!("Old file not found: {}", old_file.display());
                unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            } else if let Err(error) = std::fs::remove_file(&old_file) {
                log::error!(
                    "Failed to remove old file {}: {:?}",
                    old_file.display(),
                    error
                );
                jobs.push(old_file);
            } else {
                log::info!("Removed old file: {}", old_file.display());
                unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            }
        }
        if jobs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Self(jobs)))
        }
    }

    #[cfg(debug_assertions)]
    fn run(self, hwnd: HWND) -> Result<Option<Self>> {
        for old_file in self.0.into_iter() {
            log::info!("Removed old file: {}", old_file.display());
            std::thread::sleep(std::time::Duration::from_secs(1));
            unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
        }
        Ok(None)
    }
}

impl CopyJob {
    #[cfg(not(debug_assertions))]
    fn run(self, hwnd: HWND) -> Result<Option<Self>> {
        let mut jobs = Vec::with_capacity(self.0.len());
        for details in self.0.into_iter() {
            if !details.from.exists() {
                log::warn!("New file not found: {}", details.from.display());
                unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            } else if let Err(error) = std::fs::copy(&details.from, &details.to) {
                log::error!(
                    "Failed to copy new file {} to {}: {:?}",
                    details.from.display(),
                    details.to.display(),
                    error
                );
                jobs.push(details);
            } else {
                log::info!(
                    "Copied new file {} to {}",
                    details.from.display(),
                    details.to.display()
                );
                unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            }
        }
        if jobs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Self(jobs)))
        }
    }

    #[cfg(debug_assertions)]
    fn run(self, hwnd: HWND) -> Result<Option<Self>> {
        for details in self.0.into_iter() {
            log::info!(
                "Copied new file {} to {}",
                details.from.display(),
                details.to.display()
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
            unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
        }
        Ok(None)
    }
}

impl CleanupJob {
    #[cfg(not(debug_assertions))]
    fn run(self, hwnd: HWND) -> Result<Option<Self>> {
        let mut jobs = Vec::with_capacity(self.0.len());
        for cleanup in self.0.into_iter() {
            if !cleanup.exists() {
                log::warn!("Directory not found: {}", cleanup.display());
                unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            } else if let Err(error) = std::fs::remove_dir_all(&cleanup) {
                log::error!(
                    "Failed to remove directory {}: {:?}",
                    cleanup.display(),
                    error
                );
                jobs.push(cleanup);
            } else {
                log::info!("Removed directory: {}", cleanup.display());
                unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            }
        }
        if jobs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Self(jobs)))
        }
    }

    #[cfg(debug_assertions)]
    fn run(self, hwnd: HWND) -> Result<Option<Self>> {
        for cleanup in self.0.into_iter() {
            log::info!("Removed directory: {}", cleanup.display());
            std::thread::sleep(std::time::Duration::from_secs(1));
            unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
        }
        Ok(None)
    }
}
