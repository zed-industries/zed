// #![cfg_attr(test, allow(unused_macros))]
// #![cfg_attr(test, allow(dead_code))]

use std::{
    path::Path,
    time::{Duration, Instant},
};

use anyhow::Result;
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::PostMessageW,
};

use crate::windows_impl::WM_JOB_UPDATED;

// The number here equals the number of calls to `retry_loop` in `perform_update`.
// So if you add or remove a call to `retry_loop`, make sure to update this number too.
pub(crate) const JOBS_COUNT: usize = 6;

macro_rules! log_err {
    ($e:expr, $s:literal) => {
        $e.inspect_err(|e| {
            log::error!("{}: {}", $s, e);
        })
    };
}

pub(crate) fn perform_update(app_dir: &Path, hwnd: Option<isize>) -> Result<()> {
    let hwnd = hwnd.map(|ptr| HWND(ptr as _));

    // Delete old files
    retry_loop(hwnd, || {
        let path = app_dir.join("Zed.exe");
        if path.exists() {
            log::info!("Removing old file: {}", path.display());
            log_err!(std::fs::remove_file(path), "Failed to remove old file")
        } else {
            log::warn!("Old file not found: {}", path.display());
            Ok(())
        }
    })?;
    retry_loop(hwnd, || {
        let path = app_dir.join("bin\\zed.exe");
        if path.exists() {
            log::info!("Removing old file: {}", path.display());
            log_err!(std::fs::remove_file(path), "Failed to remove old file")
        } else {
            log::warn!("Old file not found: {}", path.display());
            Ok(())
        }
    })?;

    // Copy new files
    retry_loop(hwnd, || {
        let from_path = app_dir.join("install\\Zed.exe");
        let to_path = app_dir.join("Zed.exe");
        if from_path.exists() {
            log::info!(
                "Copying new file {} to {}",
                from_path.display(),
                to_path.display()
            );
            log_err!(std::fs::copy(from_path, to_path), "Failed to copy new file")
        } else {
            log::warn!("New file not found: {}", from_path.display());
            Ok(0)
        }
    })?;
    retry_loop(hwnd, || {
        let from_path = app_dir.join("install\\bin\\zed.exe");
        let to_path = app_dir.join("bin\\zed.exe");
        if from_path.exists() {
            log::info!(
                "Copying new file {} to {}",
                from_path.display(),
                to_path.display()
            );
            log_err!(std::fs::copy(from_path, to_path), "Failed to copy new file")
        } else {
            log::warn!("New file not found: {}", from_path.display());
            Ok(0)
        }
    })?;

    // Post cleanup jobs
    retry_loop(hwnd, || {
        let path = app_dir.join("updates");
        if path.exists() {
            log::info!("Cleaning up: {}", path.display());
            log_err!(std::fs::remove_dir_all(path), "Failed to remove directory")
        } else {
            log::warn!("Directory not found: {}", path.display());
            Ok(())
        }
    })?;
    retry_loop(hwnd, || {
        let path = app_dir.join("install");
        if path.exists() {
            log::info!("Cleaning up: {}", path.display());
            log_err!(std::fs::remove_dir_all(path), "Failed to remove directory")
        } else {
            log::warn!("Directory not found: {}", path.display());
            Ok(())
        }
    })?;

    Ok(())
}

#[cfg(not(test))]
fn retry_loop<R>(hwnd: Option<HWND>, f: impl Fn() -> std::io::Result<R>) -> Result<()> {
    let start = Instant::now();
    while start.elapsed().as_secs() <= 1 {
        if f().is_ok() {
            unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    Err(anyhow::anyhow!("Timed out"))
}

#[cfg(test)]
fn retry_loop<R>(hwnd: Option<HWND>, _: impl Fn() -> std::io::Result<R>) -> Result<()> {
    let start = Instant::now();
    while start.elapsed().as_secs() <= 1 {
        let result = if let Ok(config) = std::env::var("ZED_AUTO_UPDATE") {
            match config.as_str() {
                "inf" => {
                    std::thread::sleep(Duration::from_millis(500));
                    Err(anyhow::anyhow!("Simulated timeout"))
                }
                "err" => return Err(anyhow::anyhow!("Test error")),
                _ => panic!("Unknown ZED_AUTO_UPDATE value: {}", config),
            }
        } else {
            Ok(())
        };
        if result.is_ok() {
            unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    Err(anyhow::anyhow!("Update timed out"))
}

#[cfg(test)]
mod test {
    use super::perform_update;

    #[test]
    fn test_perform_update() {
        let app_dir = std::path::Path::new("C:/");
        assert!(perform_update(app_dir, None).is_ok());

        // Simulate a timeout
        std::env::set_var("ZED_AUTO_UPDATE", "inf");
        let ret = perform_update(app_dir, None);
        assert!(ret.is_err_and(|e| e.to_string().as_str() == "Update timed out"));

        // Simulate a test error
        std::env::set_var("ZED_AUTO_UPDATE", "err");
        let ret = perform_update(app_dir, None);
        assert!(ret.is_err_and(|e| e.to_string().as_str() == "Test error"));
    }
}
