use std::{
    path::Path,
    sync::LazyLock,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result};
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::PostMessageW,
};

use crate::windows_impl::WM_JOB_UPDATED;

pub(crate) struct Job {
    pub apply: Box<dyn Fn(&Path) -> Result<()> + Send + Sync>,
    pub rollback: Box<dyn Fn(&Path) -> Result<()> + Send + Sync>,
}

impl Job {
    pub fn mkdir(name: &'static Path) -> Self {
        Job {
            apply: Box::new(move |app_dir| {
                let dir = app_dir.join(name);
                std::fs::create_dir_all(&dir)
                    .context(format!("Failed to create directory {}", dir.display()))
            }),
            rollback: Box::new(move |app_dir| {
                let dir = app_dir.join(name);
                std::fs::remove_dir_all(&dir)
                    .context(format!("Failed to remove directory {}", dir.display()))
            }),
        }
    }

    pub fn mkdir_if_exists(name: &'static Path, check: &'static Path) -> Self {
        Job {
            apply: Box::new(move |app_dir| {
                let dir = app_dir.join(name);
                let check = app_dir.join(check);

                if check.exists() {
                    std::fs::create_dir_all(&dir)
                        .context(format!("Failed to create directory {}", dir.display()))?
                }
                Ok(())
            }),
            rollback: Box::new(move |app_dir| {
                let dir = app_dir.join(name);

                if dir.exists() {
                    std::fs::remove_dir_all(&dir)
                        .context(format!("Failed to remove directory {}", dir.display()))?
                }

                Ok(())
            }),
        }
    }

    pub fn move_file(filename: &'static Path, new_filename: &'static Path) -> Self {
        Job {
            apply: Box::new(move |app_dir| {
                let old_file = app_dir.join(filename);
                let new_file = app_dir.join(new_filename);
                log::info!(
                    "Moving file: {}->{}",
                    old_file.display(),
                    new_file.display()
                );

                std::fs::rename(&old_file, new_file)
                    .context(format!("Failed to move file {}", old_file.display()))
            }),
            rollback: Box::new(move |app_dir| {
                let old_file = app_dir.join(filename);
                let new_file = app_dir.join(new_filename);
                log::info!(
                    "Rolling back file move: {}->{}",
                    old_file.display(),
                    new_file.display()
                );

                std::fs::rename(&new_file, &old_file).context(format!(
                    "Failed to rollback file move {}->{}",
                    new_file.display(),
                    old_file.display()
                ))
            }),
        }
    }

    pub fn move_if_exists(filename: &'static Path, new_filename: &'static Path) -> Self {
        Job {
            apply: Box::new(move |app_dir| {
                let old_file = app_dir.join(filename);
                let new_file = app_dir.join(new_filename);

                if old_file.exists() {
                    log::info!(
                        "Moving file: {}->{}",
                        old_file.display(),
                        new_file.display()
                    );

                    std::fs::rename(&old_file, new_file)
                        .context(format!("Failed to move file {}", old_file.display()))?;
                }

                Ok(())
            }),
            rollback: Box::new(move |app_dir| {
                let old_file = app_dir.join(filename);
                let new_file = app_dir.join(new_filename);

                if new_file.exists() {
                    log::info!(
                        "Rolling back file move: {}->{}",
                        old_file.display(),
                        new_file.display()
                    );

                    std::fs::rename(&new_file, &old_file).context(format!(
                        "Failed to rollback file move {}->{}",
                        new_file.display(),
                        old_file.display()
                    ))?
                }

                Ok(())
            }),
        }
    }

    pub fn rmdir_nofail(filename: &'static Path) -> Self {
        Job {
            apply: Box::new(move |app_dir| {
                let filename = app_dir.join(filename);
                log::info!("Removing file: {}", filename.display());
                if let Err(e) = std::fs::remove_dir_all(&filename) {
                    log::warn!("Failed to remove directory: {}", e);
                }

                Ok(())
            }),
            rollback: Box::new(move |app_dir| {
                let filename = app_dir.join(filename);
                anyhow::bail!(
                    "Delete operations cannot be rolled back, file: {}",
                    filename.display()
                )
            }),
        }
    }
}

#[cfg(not(test))]
pub(crate) static JOBS: LazyLock<[Job; 22]> = LazyLock::new(|| {
    fn p(value: &str) -> &Path {
        Path::new(value)
    }
    [
        // Move old files
        // Not deleting because installing new files can fail
        Job::mkdir(p("old")),
        Job::move_file(p("Zed.exe"), p("old\\Zed.exe")),
        Job::mkdir(p("old\\bin")),
        Job::move_file(p("bin\\Zed.exe"), p("old\\bin\\Zed.exe")),
        Job::move_file(p("bin\\zed"), p("old\\bin\\zed")),
        //
        // TODO: remove after a few weeks once everyone is on the new version and this file never exists
        Job::move_if_exists(p("OpenConsole.exe"), p("old\\OpenConsole.exe")),
        Job::mkdir(p("old\\x64")),
        Job::mkdir(p("old\\arm64")),
        Job::move_if_exists(p("x64\\OpenConsole.exe"), p("old\\x64\\OpenConsole.exe")),
        Job::move_if_exists(
            p("arm64\\OpenConsole.exe"),
            p("old\\arm64\\OpenConsole.exe"),
        ),
        //
        Job::move_file(p("conpty.dll"), p("old\\conpty.dll")),
        // Copy new files
        Job::move_file(p("install\\Zed.exe"), p("Zed.exe")),
        Job::move_file(p("install\\bin\\Zed.exe"), p("bin\\Zed.exe")),
        Job::move_file(p("install\\bin\\zed"), p("bin\\zed")),
        //
        Job::mkdir_if_exists(p("x64"), p("install\\x64")),
        Job::mkdir_if_exists(p("arm64"), p("install\\arm64")),
        Job::move_if_exists(
            p("install\\x64\\OpenConsole.exe"),
            p("x64\\OpenConsole.exe"),
        ),
        Job::move_if_exists(
            p("install\\arm64\\OpenConsole.exe"),
            p("arm64\\OpenConsole.exe"),
        ),
        //
        Job::move_file(p("install\\conpty.dll"), p("conpty.dll")),
        // Cleanup installer and updates folder
        Job::rmdir_nofail(p("updates")),
        Job::rmdir_nofail(p("install")),
        // Cleanup old installation
        Job::rmdir_nofail(p("old")),
    ]
});

#[cfg(test)]
pub(crate) static JOBS: LazyLock<[Job; 9]> = LazyLock::new(|| {
    fn p(value: &str) -> &Path {
        Path::new(value)
    }
    [
        Job {
            apply: Box::new(|_| {
                std::thread::sleep(Duration::from_millis(1000));
                if let Ok(config) = std::env::var("ZED_AUTO_UPDATE") {
                    match config.as_str() {
                        "err1" => Err(std::io::Error::other("Simulated error")).context("Anyhow!"),
                        "err2" => Ok(()),
                        _ => panic!("Unknown ZED_AUTO_UPDATE value: {}", config),
                    }
                } else {
                    Ok(())
                }
            }),
            rollback: Box::new(|_| {
                unsafe { std::env::set_var("ZED_AUTO_UPDATE_RB", "rollback1") };
                Ok(())
            }),
        },
        Job::mkdir(p("test1")),
        Job::mkdir_if_exists(p("test_exists"), p("test1")),
        Job::mkdir_if_exists(p("test_missing"), p("dont")),
        Job {
            apply: Box::new(|folder| {
                std::fs::write(folder.join("test1/test"), "test")?;
                Ok(())
            }),
            rollback: Box::new(|folder| {
                std::fs::remove_file(folder.join("test1/test"))?;
                Ok(())
            }),
        },
        Job::move_file(p("test1/test"), p("test1/moved")),
        Job::move_if_exists(p("test1/test"), p("test1/noop")),
        Job {
            apply: Box::new(|_| {
                std::thread::sleep(Duration::from_millis(1000));
                if let Ok(config) = std::env::var("ZED_AUTO_UPDATE") {
                    match config.as_str() {
                        "err1" => Ok(()),
                        "err2" => Err(std::io::Error::other("Simulated error")).context("Anyhow!"),
                        _ => panic!("Unknown ZED_AUTO_UPDATE value: {}", config),
                    }
                } else {
                    Ok(())
                }
            }),
            rollback: Box::new(|_| Ok(())),
        },
        Job::rmdir_nofail(p("test1/nofolder")),
    ]
});

pub(crate) fn perform_update(app_dir: &Path, hwnd: Option<isize>, launch: bool) -> Result<()> {
    let hwnd = hwnd.map(|ptr| HWND(ptr as _));

    let mut last_successful_job = None;
    'outer: for (i, job) in JOBS.iter().enumerate() {
        let start = Instant::now();
        loop {
            if start.elapsed().as_secs() > 2 {
                log::error!("Timed out, rolling back");
                break 'outer;
            }
            match (job.apply)(app_dir) {
                Ok(_) => {
                    last_successful_job = Some(i);
                    unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
                    break;
                }
                Err(err) => {
                    // Check if it's a "not found" error
                    let io_err = err.downcast_ref::<std::io::Error>().unwrap();
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        log::warn!("File or folder not found.");
                        last_successful_job = Some(i);
                        unsafe { PostMessageW(hwnd, WM_JOB_UPDATED, WPARAM(0), LPARAM(0))? };
                        break;
                    }

                    log::error!("Operation failed: {} ({:?})", err, io_err.kind());
                    std::thread::sleep(Duration::from_millis(50));
                }
            }
        }
    }

    if last_successful_job
        .map(|job| job != JOBS.len() - 1)
        .unwrap_or(true)
    {
        let Some(last_successful_job) = last_successful_job else {
            anyhow::bail!("Autoupdate failed, nothing to rollback");
        };

        for job in (0..=last_successful_job).rev() {
            let job = &JOBS[job];
            if let Err(e) = (job.rollback)(app_dir) {
                anyhow::bail!(
                    "Job rollback failed, the app might be left in an inconsistent state: ({:?})",
                    e
                );
            }
        }

        anyhow::bail!("Autoupdate failed, rollback successful");
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
        let app_dir = tempfile::tempdir().unwrap();
        let app_dir = app_dir.path();
        assert!(perform_update(app_dir, None, false).is_ok());

        let app_dir = tempfile::tempdir().unwrap();
        let app_dir = app_dir.path();
        // Simulate a timeout
        unsafe { std::env::set_var("ZED_AUTO_UPDATE", "err1") };
        let ret = perform_update(app_dir, None, false);
        assert!(
            ret.is_err_and(|e| e.to_string().as_str() == "Autoupdate failed, nothing to rollback")
        );

        let app_dir = tempfile::tempdir().unwrap();
        let app_dir = app_dir.path();
        // Simulate a timeout
        unsafe { std::env::set_var("ZED_AUTO_UPDATE", "err2") };
        let ret = perform_update(app_dir, None, false);
        assert!(
            ret.is_err_and(|e| e.to_string().as_str() == "Autoupdate failed, rollback successful")
        );
        assert!(std::env::var("ZED_AUTO_UPDATE_RB").is_ok_and(|e| e == "rollback1"));
    }
}
