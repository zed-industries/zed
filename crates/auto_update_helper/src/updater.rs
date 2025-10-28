use std::{
    path::Path,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result};
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::PostMessageW,
};

use crate::windows_impl::WM_JOB_UPDATED;

type Job = fn(&Path) -> Result<()>;

#[cfg(not(test))]
pub(crate) const JOBS: &[Job] = &[
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
    |app_dir| {
        let zed_wsl = app_dir.join("bin\\zed");
        log::info!("Removing old file: {}", zed_wsl.display());
        std::fs::remove_file(&zed_wsl)
            .context(format!("Failed to remove old file {}", zed_wsl.display()))
    },
    // TODO: remove after a few weeks once everyone is on the new version and this file never exists
    |app_dir| {
        let open_console = app_dir.join("OpenConsole.exe");
        if open_console.exists() {
            log::info!("Removing old file: {}", open_console.display());
            std::fs::remove_file(&open_console).context(format!(
                "Failed to remove old file {}",
                open_console.display()
            ))?
        }
        Ok(())
    },
    |app_dir| {
        let archs = ["x64", "arm64"];
        for arch in archs {
            let open_console = app_dir.join(format!("{arch}\\OpenConsole.exe"));
            if open_console.exists() {
                log::info!("Removing old file: {}", open_console.display());
                std::fs::remove_file(&open_console).context(format!(
                    "Failed to remove old file {}",
                    open_console.display()
                ))?
            }
        }
        Ok(())
    },
    |app_dir| {
        let conpty = app_dir.join("conpty.dll");
        log::info!("Removing old file: {}", conpty.display());
        std::fs::remove_file(&conpty)
            .context(format!("Failed to remove old file {}", conpty.display()))
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
    |app_dir| {
        let zed_wsl_source = app_dir.join("install\\bin\\zed");
        let zed_wsl_dest = app_dir.join("bin\\zed");
        log::info!(
            "Copying new file {} to {}",
            zed_wsl_source.display(),
            zed_wsl_dest.display()
        );
        std::fs::copy(&zed_wsl_source, &zed_wsl_dest)
            .map(|_| ())
            .context(format!(
                "Failed to copy new file {} to {}",
                zed_wsl_source.display(),
                zed_wsl_dest.display()
            ))
    },
    |app_dir| {
        let archs = ["x64", "arm64"];
        for arch in archs {
            let open_console_source = app_dir.join(format!("install\\{arch}\\OpenConsole.exe"));
            let open_console_dest = app_dir.join(format!("{arch}\\OpenConsole.exe"));
            if open_console_source.exists() {
                log::info!(
                    "Copying new file {} to {}",
                    open_console_source.display(),
                    open_console_dest.display()
                );
                let parent = open_console_dest.parent().context(format!(
                    "Failed to get parent directory of {}",
                    open_console_dest.display()
                ))?;
                std::fs::create_dir_all(parent)
                    .context(format!("Failed to create directory {}", parent.display()))?;
                std::fs::copy(&open_console_source, &open_console_dest)
                    .map(|_| ())
                    .context(format!(
                        "Failed to copy new file {} to {}",
                        open_console_source.display(),
                        open_console_dest.display()
                    ))?
            }
        }
        Ok(())
    },
    |app_dir| {
        let conpty_source = app_dir.join("install\\conpty.dll");
        let conpty_dest = app_dir.join("conpty.dll");
        log::info!(
            "Copying new file {} to {}",
            conpty_source.display(),
            conpty_dest.display()
        );
        std::fs::copy(&conpty_source, &conpty_dest)
            .map(|_| ())
            .context(format!(
                "Failed to copy new file {} to {}",
                conpty_source.display(),
                conpty_dest.display()
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
pub(crate) const JOBS: &[Job] = &[
    |_| {
        std::thread::sleep(Duration::from_millis(1000));
        if let Ok(config) = std::env::var("ZED_AUTO_UPDATE") {
            match config.as_str() {
                "err" => Err(std::io::Error::other("Simulated error")).context("Anyhow!"),
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
                "err" => Err(std::io::Error::other("Simulated error")).context("Anyhow!"),
                _ => panic!("Unknown ZED_AUTO_UPDATE value: {}", config),
            }
        } else {
            Ok(())
        }
    },
];

pub(crate) fn perform_update(app_dir: &Path, hwnd: Option<isize>, launch: bool) -> Result<()> {
    let hwnd = hwnd.map(|ptr| HWND(ptr as _));

    for job in JOBS.iter() {
        let start = Instant::now();
        loop {
            anyhow::ensure!(start.elapsed().as_secs() <= 2, "Timed out");
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

                    log::error!("Operation failed: {} ({:?})", err, io_err.kind());
                    std::thread::sleep(Duration::from_millis(50));
                }
            }
        }
    }
    if launch {
        #[allow(clippy::disallowed_methods, reason = "doesn't run in the main binary")]
        let _ = std::process::Command::new(app_dir.join("Zed.exe")).spawn();
    }
    log::info!("Update completed successfully");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::perform_update;

    #[test]
    fn test_perform_update() {
        let app_dir = std::path::Path::new("C:/");
        assert!(perform_update(app_dir, None, false).is_ok());

        // Simulate a timeout
        unsafe { std::env::set_var("ZED_AUTO_UPDATE", "err") };
        let ret = perform_update(app_dir, None, false);
        assert!(ret.is_err_and(|e| e.to_string().as_str() == "Timed out"));
    }
}
