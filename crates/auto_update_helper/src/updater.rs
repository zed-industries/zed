use std::path::{Path, PathBuf};

use anyhow::Result;
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::PostMessageW,
};

use crate::WM_JOB_UPDATED;

#[derive(Debug, PartialEq, Eq)]
enum UpdateStatus {
    RemoveOld(Vec<PathBuf>),
    CopyNew(Vec<(PathBuf, PathBuf)>),
    DeleteInstall,
    DeleteUpdates,
    Done,
}

#[cfg(not(debug_assertions))]
pub(crate) fn perform_update(app_dir: &Path, hwnd: isize) -> Result<()> {
    let install_dir = app_dir.join("install");
    let update_dir = app_dir.join("updates");
    let hwnd = HWND(hwnd as _);

    let start = std::time::Instant::now();
    let mut status =
        UpdateStatus::RemoveOld(vec![app_dir.join("Zed.exe"), app_dir.join("bin\\zed.exe")]);
    while start.elapsed().as_secs() < 10 {
        match status {
            UpdateStatus::RemoveOld(old_files) => {
                log::info!("Removing old files: {:?}", old_files);
                let mut sccess = Vec::with_capacity(old_files.len());
                for old_file in old_files.iter() {
                    if old_file.exists() {
                        let result = std::fs::remove_file(&old_file);
                        if let Err(error) = result {
                            log::error!(
                                "Failed to remove old file {}: {:?}",
                                old_file.display(),
                                error
                            );
                        } else {
                            sccess.push(old_file);
                            unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0)) }?;
                        }
                    } else {
                        log::warn!("Old file not found: {}", old_file.display());
                        sccess.push(old_file);
                        unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0)) }?;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                let left_old_files = old_files
                    .iter()
                    .filter(|old_file| !sccess.contains(old_file))
                    .map(|old_file| old_file.clone())
                    .collect::<Vec<_>>();
                if left_old_files.is_empty() {
                    status = UpdateStatus::CopyNew(vec![
                        (install_dir.join("Zed.exe"), app_dir.join("Zed.exe")),
                        (
                            install_dir.join("bin\\zed.exe"),
                            app_dir.join("bin\\zed.exe"),
                        ),
                    ]);
                } else {
                    status = UpdateStatus::RemoveOld(left_old_files);
                }
            }
            UpdateStatus::CopyNew(new_files) => {
                log::info!("Copying new files: {:?}", new_files);
                let mut sccess = Vec::with_capacity(new_files.len());
                for (new_file, old_file) in new_files.iter() {
                    if new_file.exists() {
                        let result = std::fs::copy(&new_file, &old_file);
                        if let Err(error) = result {
                            log::error!(
                                "Failed to copy new file {} to {}: {:?}",
                                new_file.display(),
                                old_file.display(),
                                error
                            );
                        } else {
                            sccess.push((new_file, old_file));
                            unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0)) }?;
                        }
                    } else {
                        log::warn!("New file not found: {}", new_file.display());
                        sccess.push((new_file, old_file));
                        unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0)) }?;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                let left_new_files = new_files
                    .iter()
                    .filter(|(new_file, _)| !sccess.iter().any(|(n, _)| *n == new_file))
                    .map(|(new_file, old_file)| (new_file.clone(), old_file.clone()))
                    .collect::<Vec<_>>();

                if left_new_files.is_empty() {
                    status = UpdateStatus::DeleteInstall;
                } else {
                    status = UpdateStatus::CopyNew(left_new_files);
                }
            }
            UpdateStatus::DeleteInstall => {
                log::info!("Deleting install directory: {}", install_dir.display());
                let result = std::fs::remove_dir_all(&install_dir);
                if let Err(error) = result {
                    log::error!("Failed to remove install directory: {:?}", error);
                    continue;
                }
                unsafe {
                    PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            UpdateStatus::DeleteUpdates => {
                log::info!("Deleting updates directory: {}", update_dir.display());
                let result = std::fs::remove_dir_all(&update_dir);
                if let Err(error) = result {
                    log::error!("Failed to remove updates directory: {:?}", error);
                    continue;
                }
                unsafe {
                    PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
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

#[cfg(debug_assertions)]
pub(crate) fn perform_update(_: &Path, hwnd: isize) -> Result<()> {
    let hwnd = HWND(hwnd as _);

    let start = std::time::Instant::now();
    let mut status = UpdateStatus::RemoveOld(vec![]);
    while start.elapsed().as_secs() < 10 {
        match status {
            UpdateStatus::RemoveOld(_) => {
                for _ in 0..2 {
                    unsafe {
                        PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                status = UpdateStatus::CopyNew(vec![]);
            }
            UpdateStatus::CopyNew(_) => {
                for _ in 0..2 {
                    unsafe {
                        PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                status = UpdateStatus::DeleteInstall;
            }
            UpdateStatus::DeleteInstall => {
                unsafe {
                    PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                }
                status = UpdateStatus::DeleteUpdates;
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            UpdateStatus::DeleteUpdates => {
                unsafe {
                    PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))?;
                }
                status = UpdateStatus::Done;
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            UpdateStatus::Done => {
                break;
            }
        }
    }
    if status != UpdateStatus::Done {
        Err(anyhow::anyhow!("Failed to update Zed, timeout"))
    } else {
        Ok(())
    }
}
