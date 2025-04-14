use std::{
    os::windows::process::CommandExt,
    path::Path,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    System::Threading::CREATE_NEW_PROCESS_GROUP,
    UI::WindowsAndMessaging::PostMessageW,
};

use crate::windows_impl::WM_JOB_UPDATED;

type Job = fn(&Path) -> Result<()>;

#[cfg(not(test))]
pub(crate) const JOBS: [Job; 6] = [
    // Delete old files
    |app_dir| {
        let zed_executable = app_dir.join("Zed.exe");
        log::info!("Removing old file: {}", zed_executable.display());
        std::fs::remove_file(&zed_executable).context(format!(
            "Failed to remove old file {}",
            zed_executable.display()
        ))
    },
    |app_dir| {
        let zed_cli = app_dir.join("bin\\zed.exe");
        log::info!("Removing old file: {}", zed_cli.display());
        std::fs::remove_file(&zed_cli)
            .context(format!("Failed to remove old file {}", zed_cli.display()))
    },
    // Copy new files
    |app_dir| {
        let zed_executable_source = app_dir.join("install\\Zed.exe");
        let zed_executable_dest = app_dir.join("Zed.exe");
        log::info!(
            "Copying new file {} to {}",
            zed_executable_source.display(),
            zed_executable_dest.display()
        );
        std::fs::copy(&zed_executable_source, &zed_executable_dest)
            .map(|_| ())
            .context(format!(
                "Failed to copy new file {} to {}",
                zed_executable_source.display(),
                zed_executable_dest.display()
            ))
    },
    |app_dir| {
        let zed_cli_source = app_dir.join("install\\bin\\zed.exe");
        let zed_cli_dest = app_dir.join("bin\\zed.exe");
        log::info!(
            "Copying new file {} to {}",
            zed_cli_source.display(),
            zed_cli_dest.display()
        );
        std::fs::copy(&zed_cli_source, &zed_cli_dest)
            .map(|_| ())
            .context(format!(
                "Failed to copy new file {} to {}",
                zed_cli_source.display(),
                zed_cli_dest.display()
            ))
    },
    // Clean up installer folder and updates folder
    |app_dir| {
        let updates_folder = app_dir.join("updates");
        log::info!("Cleaning up: {}", updates_folder.display());
        std::fs::remove_dir_all(&updates_folder).context(format!(
            "Failed to remove updates folder {}",
            updates_folder.display()
        ))
    },
    |app_dir| {
        let installer_folder = app_dir.join("install");
        log::info!("Cleaning up: {}", installer_folder.display());
        std::fs::remove_dir_all(&installer_folder).context(format!(
            "Failed to remove installer folder {}",
            installer_folder.display()
        ))
    },
];

#[cfg(test)]
pub(crate) const JOBS: [Job; 2] = [
    |_| {
        std::thread::sleep(Duration::from_millis(1000));
        if let Ok(config) = std::env::var("ZED_AUTO_UPDATE") {
            match config.as_str() {
                "err" => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Simulated error",
                ))
                .context("Anyhow!"),
                _ => panic!("Unknown ZED_AUTO_UPDATE value: {}", config),
            }
        } else {
            Ok(())
        }
    },
    |_| {
        std::thread::sleep(Duration::from_millis(1000));
        if let Ok(config) = std::env::var("ZED_AUTO_UPDATE") {
            match config.as_str() {
                "err" => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Simulated error",
                ))
                .context("Anyhow!"),
                _ => panic!("Unknown ZED_AUTO_UPDATE value: {}", config),
            }
        } else {
            Ok(())
        }
    },
];

pub(crate) fn perform_update(app_dir: &Path, hwnd: Option<isize>) -> Result<()> {
    let hwnd = hwnd.map(|ptr| HWND(ptr as _));

    for job in JOBS.iter() {
        let start = Instant::now();
        loop {
            if start.elapsed().as_secs() > 2 {
                return Err(anyhow::anyhow!("Timed out"));
            }
            match (*job)(app_dir) {
                Ok(_) => {
                    unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
                    break;
                }
                Err(err) => {
                    // Check if it's a "not found" error
                    let io_err = err.downcast_ref::<std::io::Error>().unwrap();
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        log::warn!("File or folder not found.");
                        unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
                        break;
                    }

                    log::error!("Operation failed: {}", err);
                    std::thread::sleep(Duration::from_millis(50));
                }
            }
        }
    }
    let _ = std::process::Command::new(app_dir.join("Zed.exe"))
        .creation_flags(CREATE_NEW_PROCESS_GROUP.0)
        .spawn();
    log::info!("Update completed successfully");
    Ok(())
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
