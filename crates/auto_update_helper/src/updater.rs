// #![cfg_attr(test, allow(unused_macros))]
// #![cfg_attr(test, allow(dead_code))]

use std::{
    path::Path,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::PostMessageW,
};

use crate::windows_impl::WM_JOB_UPDATED;

// The number here equals the number of calls to `retry_loop` in `perform_update`.
// So if you add or remove a call to `retry_loop`, make sure to update this number too.
pub(crate) const JOBS_COUNT: usize = 6;

pub(crate) fn perform_update(app_dir: &Path, hwnd: Option<isize>) -> Result<()> {
    let hwnd = hwnd.map(|ptr| HWND(ptr as _));

    // Delete old files
    retry_loop(hwnd, || {
        let zed_executable = app_dir.join("Zed.exe");
        log::info!("Removing old file: {}", zed_executable.display());
        std::fs::remove_file(&zed_executable).context(format!(
            "Failed to remove old file {}",
            zed_executable.display()
        ))
    })?;
    retry_loop(hwnd, || {
        let zed_cli = app_dir.join("bin\\zed.exe");
        log::info!("Removing old file: {}", zed_cli.display());
        std::fs::remove_file(&zed_cli)
            .context(format!("Failed to remove old file {}", zed_cli.display()))
    })?;

    // Copy new files
    retry_loop(hwnd, || {
        let zed_executable_source = app_dir.join("install\\Zed.exe");
        let zed_executable_dest = app_dir.join("Zed.exe");
        log::info!(
            "Copying new file {} to {}",
            zed_executable_source.display(),
            zed_executable_dest.display()
        );
        std::fs::copy(&zed_executable_source, &zed_executable_dest).context(format!(
            "Failed to copy new file {} to {}",
            zed_executable_source.display(),
            zed_executable_dest.display()
        ))
    })?;
    retry_loop(hwnd, || {
        let zed_cli_source = app_dir.join("install\\bin\\zed.exe");
        let zed_cli_dest = app_dir.join("bin\\zed.exe");
        log::info!(
            "Copying new file {} to {}",
            zed_cli_source.display(),
            zed_cli_dest.display()
        );
        std::fs::copy(&zed_cli_source, &zed_cli_dest).context(format!(
            "Failed to copy new file {} to {}",
            zed_cli_source.display(),
            zed_cli_dest.display()
        ))
    })?;

    // Post cleanup jobs
    retry_loop(hwnd, || {
        let updates_folder = app_dir.join("updates");
        log::info!("Cleaning up: {}", updates_folder.display());
        std::fs::remove_dir_all(&updates_folder).context(format!(
            "Failed to remove updates folder {}",
            updates_folder.display()
        ))
    })?;
    retry_loop(hwnd, || {
        let installer_folder = app_dir.join("install");
        log::info!("Cleaning up: {}", installer_folder.display());
        std::fs::remove_dir_all(&installer_folder).context(format!(
            "Failed to remove installer folder {}",
            installer_folder.display()
        ))
    })?;

    Ok(())
}

#[cfg(not(test))]
fn retry_loop<R>(hwnd: Option<HWND>, f: impl Fn() -> Result<R>) -> Result<()> {
    let start = Instant::now();
    while start.elapsed().as_secs() <= 2 {
        match f() {
            Ok(_) => {
                unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
                return Ok(());
            }
            Err(anyhow_err) => {
                let io_err = anyhow_err.downcast_ref::<std::io::Error>().unwrap();
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    log::warn!("File or folder not found.");
                    unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
                    return Ok(());
                } else {
                    println!("Operation failed: {}", anyhow_err);
                    log::error!("Operation failed: {}", anyhow_err);
                    // wait for a bit before retrying
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
    Err(anyhow::anyhow!("Timed out"))
}

#[cfg(test)]
fn retry_loop<R>(hwnd: Option<HWND>, _: impl Fn() -> Result<R>) -> Result<()> {
    let start = Instant::now();
    while start.elapsed().as_secs() <= 2 {
        let result: anyhow::Result<()> = if let Ok(config) = std::env::var("ZED_AUTO_UPDATE") {
            match config.as_str() {
                "err" => {
                    std::thread::sleep(Duration::from_millis(500));
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Simulated error",
                    ))
                    .context("Anyhow!")
                }
                _ => panic!("Unknown ZED_AUTO_UPDATE value: {}", config),
            }
        } else {
            Ok(())
        };
        match result {
            Ok(_) => {
                unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
                return Ok(());
            }
            Err(anyhow_err) => {
                let io_err = anyhow_err.downcast_ref::<std::io::Error>().unwrap();
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    log::warn!("File or folder not found.");
                    unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
                    return Ok(());
                } else {
                    log::error!("Operation failed: {}", anyhow_err);
                    // wait for a bit before retrying
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
    Err(anyhow::anyhow!("Timed out"))
}

#[cfg(test)]
mod test {
    use super::perform_update;

    #[test]
    fn test_perform_update() {
        let app_dir = std::path::Path::new("C:/");
        assert!(perform_update(app_dir, None).is_ok());

        // Simulate a timeout
        unsafe { std::env::set_var("ZED_AUTO_UPDATE", "err") };
        let ret = perform_update(app_dir, None);
        assert!(ret.is_err_and(|e| e.to_string().as_str() == "Timed out"));
    }
}
