use std::{
    ffi::{OsStr, OsString},
    os::windows::ffi::OsStrExt,
    path::{Path, PathBuf},
    sync::LazyLock,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result};
use windows::{
    Win32::{
        Foundation::{ERROR_MORE_DATA, HWND, LPARAM, WPARAM},
        System::RestartManager::{
            CCH_RM_SESSION_KEY, RmEndSession, RmGetList, RmRegisterResources, RmShutdown,
            RmStartSession,
        },
        UI::WindowsAndMessaging::PostMessageW,
    },
    core::{PCWSTR, PWSTR},
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
                // Deleting a directory can't be undone; if it still exists and can't be removed,
                // that's a cleanup issue for the next run, not a fatal one.
                match std::fs::remove_dir_all(&filename) {
                    Ok(()) => {}
                    Err(e) => log::warn!(
                        "Failed to remove leftover directory {}: {e:#}",
                        filename.display()
                    ),
                }
                Ok(())
            }),
        }
    }
}

#[cfg(not(test))]
pub(crate) static JOBS: LazyLock<[Job; 24]> = LazyLock::new(|| {
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
        Job::move_if_exists(p("bin\\zed.exe"), p("old\\bin\\zed.exe")),
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
        Job::move_if_exists(p("install\\bin\\zed.exe"), p("bin\\zed.exe")),
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

/// A Windows Restart Manager session, used to detect which processes hold handles to files.
struct RmSession {
    handle: u32,
}

impl RmSession {
    /// Starts a new session. Returns `None` if the session could not be started.
    fn new() -> Option<Self> {
        let mut handle: u32 = 0;
        let mut session_key = [0u16; CCH_RM_SESSION_KEY as usize + 1];
        let ok = unsafe {
            RmStartSession(&mut handle, None, PWSTR::from_raw(session_key.as_mut_ptr())).is_ok()
        };
        ok.then(|| Self { handle })
    }

    /// Registers a file so that `locked_process_count` reports the processes holding it.
    fn register_file(&self, path: &Path) -> bool {
        let wide_path: Vec<u16> = OsStr::new(path.as_os_str())
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let pcwstr = PCWSTR::from_raw(wide_path.as_ptr());
        unsafe { RmRegisterResources(self.handle, Some(std::slice::from_ref(&pcwstr)), None, None) }
            .is_ok()
    }

    /// The number of processes holding handles to the registered files, if the query succeeded.
    fn locked_process_count(&self) -> Option<u32> {
        let mut needed: u32 = 0;
        let mut count: u32 = 0;
        let mut reboot_reasons: u32 = 0;
        // In the two-call query pattern, ERROR_MORE_DATA is a success signal meaning
        // processes were found (with `needed` set to how many).
        let result =
            unsafe { RmGetList(self.handle, &mut needed, &mut count, None, &mut reboot_reasons) };
        if result.is_ok() || result == ERROR_MORE_DATA {
            Some(needed)
        } else {
            None
        }
    }

    /// Politely asks the processes holding handles to release them (e.g. Explorer's icon cache
    /// handles). Uncooperative processes keep their handles.
    fn request_release(&self) {
        // RmShutdown with flags=0 asks applications to release handles gracefully;
        // for Explorer this typically releases icon cache handles without closing it.
        let _ = unsafe { RmShutdown(self.handle, 0, None) };
    }
}

impl Drop for RmSession {
    fn drop(&mut self) {
        unsafe { let _ = RmEndSession(self.handle); }
    }
}

/// The files in the app directory that an update moves or overwrites, including the `old\`
/// destinations where leftovers from a previous interrupted update may still sit (possibly
/// locked by a process running from them).
fn files_involved(app_dir: &Path) -> Vec<PathBuf> {
    const RELATIVE_PATHS: &[&str] = &[
        "Zed.exe",
        "bin\\Zed.exe",
        "bin\\zed.exe",
        "bin\\zed",
        "OpenConsole.exe",
        "x64\\OpenConsole.exe",
        "arm64\\OpenConsole.exe",
        "conpty.dll",
        // Leftover destinations from a previous update
        "old\\Zed.exe",
        "old\\bin\\Zed.exe",
        "old\\bin\\zed.exe",
        "old\\bin\\zed",
        "old\\OpenConsole.exe",
        "old\\x64\\OpenConsole.exe",
        "old\\arm64\\OpenConsole.exe",
        "old\\conpty.dll",
    ];

    RELATIVE_PATHS
        .iter()
        .map(|path| app_dir.join(*path))
        .filter(|path| path.exists())
        .collect()
}

/// Uses Windows Restart Manager to find which of the files an update needs to move are
/// currently held by other processes (e.g. a terminal's OpenConsole.exe).
fn find_locked_files(app_dir: &Path) -> Vec<PathBuf> {
    files_involved(app_dir)
        .into_iter()
        .filter(|path| {
            let Some(session) = RmSession::new() else {
                return false;
            };
            if !session.register_file(path) {
                return false;
            }
            let locked = session.locked_process_count().is_some_and(|count| count > 0);
            if locked {
                log::info!("File {} is held by other process(es)", path.display());
            }
            locked
        })
        .collect()
}

/// Makes sure no other process is holding any of the files an update needs to move.
///
/// Politely asks processes to release their handles (e.g. Explorer's icon cache) and waits a
/// short while for them to do so. Returns an error describing the locked files if they are
/// still held, so that `perform_update` can defer the update instead of risking a
/// half-finished swap.
fn ensure_files_unlocked(app_dir: &Path) -> Result<()> {
    let mut locked = find_locked_files(app_dir);
    if locked.is_empty() {
        return Ok(());
    }

    log::info!(
        "{} file(s) are held by other processes, requesting release...",
        locked.len()
    );
    for path in &locked {
        if let Some(session) = RmSession::new()
            && session.register_file(path)
        {
            session.request_release();
        }
    }

    // Give the processes a moment to release their handles, re-checking periodically.
    for _ in 0..4 {
        std::thread::sleep(Duration::from_millis(500));
        locked = find_locked_files(app_dir);
        if locked.is_empty() {
            return Ok(());
        }
    }

    let files = locked
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    anyhow::bail!(
        "the following files are in use by other processes (e.g. an open terminal or Explorer): {files}"
    )
}

#[allow(clippy::disallowed_methods, reason = "doesn't run in the main binary")]
fn zed_launch_command(app_dir: &Path, launch_arguments: &[OsString]) -> std::process::Command {
    let mut command = std::process::Command::new(app_dir.join("Zed.exe"));
    command.args(launch_arguments);
    command
}

pub(crate) fn perform_update(
    app_dir: &Path,
    hwnd: Option<isize>,
    launch: bool,
    launch_arguments: &[OsString],
) -> Result<()> {
    let hwnd = hwnd.map(|ptr| HWND(ptr as _));

    // Before moving anything, make sure no other process is holding any of the files we need
    // to move (e.g. a terminal's OpenConsole.exe or Explorer's icon handles). If they are still
    // locked after politely asking for release, defer the update: the installation is left
    // untouched and the swap will be retried the next time Zed exits or restarts.
    if let Err(e) = ensure_files_unlocked(app_dir) {
        log::error!("Deferring Zed update: {e:#}");
        if launch {
            // Don't leave the user without an editor: start the currently installed version;
            // the swap will be retried when it exits.
            #[allow(clippy::disallowed_methods, reason = "doesn't run in the main binary")]
            let _child = zed_launch_command(app_dir, launch_arguments)
                .spawn()
                .context("Failed to launch Zed after deferring the update")?;
        }
        anyhow::bail!("Update deferred, it will be retried on the next exit or restart: {e}");
    }

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
                Err(err) => match err.downcast_ref::<std::io::Error>() {
                    Some(io_err) => match io_err.kind() {
                        std::io::ErrorKind::NotFound => {
                            log::error!("Operation failed with file not found, aborting: {}", err);
                            break 'outer;
                        }
                        _ => {
                            log::error!("Operation failed (retrying): {}", err);
                            std::thread::sleep(Duration::from_millis(50));
                        }
                    },
                    None => {
                        log::error!("Operation failed with unexpected error, aborting: {}", err);
                        break 'outer;
                    }
                },
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

        // Roll back every applied job. If one rollback fails (e.g. a directory that can't be
        // removed because a process is holding a file in it), keep rolling back the rest:
        // restoring the moved executables is what keeps Zed launchable.
        let mut failed_rollbacks = Vec::new();
        for i in (0..=last_successful_job).rev() {
            let job = &JOBS[i];
            if let Err(e) = (job.rollback)(app_dir) {
                log::error!("Rolling back job {i} failed: {e:#}");
                failed_rollbacks.push(i);
            }
        }

        if !failed_rollbacks.is_empty() {
            anyhow::bail!(
                "Autoupdate failed, and {} rollback(s) also failed (jobs {failed_rollbacks:?}) - the app might be left in an inconsistent state",
                failed_rollbacks.len()
            );
        }

        anyhow::bail!("Autoupdate failed, rollback successful");
    }

    if launch {
        #[allow(clippy::disallowed_methods, reason = "doesn't run in the main binary")]
        let _child = zed_launch_command(app_dir, launch_arguments)
            .spawn()
            .context("Failed to launch Zed after update")?;
    }
    log::info!("Update completed successfully");
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{
        ffi::{OsStr, OsString},
        os::windows::ffi::OsStrExt,
        path::Path,
        time::Duration,
    };

    use super::{ensure_files_unlocked, find_locked_files, perform_update, zed_launch_command};
    use windows::{
        core::PCWSTR,
        Win32::{
            Foundation::{CloseHandle, GENERIC_READ},
            Storage::FileSystem::{
                CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_NONE, OPEN_EXISTING,
            },
        },
    };

    #[test]
    fn test_zed_launch_command_preserves_arguments() {
        let arguments = vec![
            OsString::from("--user-data-dir"),
            OsString::from(r"C:\Zed Data"),
        ];
        let command = zed_launch_command(Path::new(r"C:\Program Files\Zed"), &arguments);

        assert_eq!(
            command.get_program(),
            Path::new(r"C:\Program Files\Zed\Zed.exe").as_os_str()
        );
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            arguments
                .iter()
                .map(OsString::as_os_str)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_perform_update() {
        let app_dir = tempfile::tempdir().unwrap();
        let app_dir = app_dir.path();
        assert!(perform_update(app_dir, None, false, &[]).is_ok());

        let app_dir = tempfile::tempdir().unwrap();
        let app_dir = app_dir.path();
        // Simulate a timeout
        unsafe { std::env::set_var("ZED_AUTO_UPDATE", "err1") };
        let ret = perform_update(app_dir, None, false, &[]);
        assert!(
            ret.is_err_and(|e| e.to_string().as_str() == "Autoupdate failed, nothing to rollback")
        );

        let app_dir = tempfile::tempdir().unwrap();
        let app_dir = app_dir.path();
        // Simulate a timeout
        unsafe { std::env::set_var("ZED_AUTO_UPDATE", "err2") };
        let ret = perform_update(app_dir, None, false, &[]);
        assert!(
            ret.is_err_and(|e| e.to_string().as_str() == "Autoupdate failed, rollback successful")
        );
        assert!(std::env::var("ZED_AUTO_UPDATE_RB").is_ok_and(|e| e == "rollback1"));
    }

    #[test]
    fn test_ensure_files_unlocked_succeeds_when_nothing_is_locked() {
        let app_dir = tempfile::tempdir().unwrap();
        std::fs::write(app_dir.path().join("Zed.exe"), b"").unwrap();
        std::fs::create_dir_all(app_dir.path().join("old\\x64")).unwrap();
        std::fs::write(
            app_dir.path().join("old\\x64\\OpenConsole.exe"),
            b"",
        )
        .unwrap();

        assert!(ensure_files_unlocked(app_dir.path()).is_ok());
    }

    #[test]
    fn test_find_locked_files_detects_exclusive_handle() {
        let app_dir = tempfile::tempdir().unwrap();
        let file = app_dir.path().join("Zed.exe");
        std::fs::write(&file, b"").unwrap();

        // Open the file with a share mode that blocks renaming or deleting it, like the
        // image of a running process.
        let wide_path: Vec<u16> = OsStr::new(file.as_os_str())
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let handle = unsafe {
            CreateFileW(
                PCWSTR::from_raw(wide_path.as_ptr()),
                GENERIC_READ.0,
                FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
        }
        .unwrap();

        // Restart Manager may take a moment to notice the new handle.
        let mut locked = Vec::new();
        for _ in 0..20 {
            locked = find_locked_files(app_dir.path());
            if !locked.is_empty() {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        assert!(
            locked.contains(&file),
            "expected {} to be reported as locked",
            file.display()
        );

        unsafe { let _ = CloseHandle(handle); };
    }

    /// Regression test for the incident where a leftover `old\x64\OpenConsole.exe` held by a
    /// running process made the update fail mid-swap and the rollback strand `Zed.exe` in
    /// `old\`, making the app disappear. The update must now be deferred *before* any file is
    /// moved, leaving the installation untouched.
    #[test]
    fn test_perform_update_defers_when_leftover_file_is_locked() {
        let app_dir = tempfile::tempdir().unwrap();
        let root = app_dir.path();

        // A leftover from a previous interrupted update, held with a share mode that blocks
        // rename/delete, like a process running from it.
        std::fs::create_dir_all(root.join("old\\x64")).unwrap();
        let leftover = root.join("old\\x64\\OpenConsole.exe");
        std::fs::write(&leftover, b"older").unwrap();

        let wide_path: Vec<u16> = OsStr::new(leftover.as_os_str())
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let handle = unsafe {
            CreateFileW(
                PCWSTR::from_raw(wide_path.as_ptr()),
                GENERIC_READ.0,
                FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
        }
        .unwrap();

        // The update must be deferred before any job runs.
        let err = perform_update(root, None, false, &[])
            .expect_err("update should be deferred");
        assert!(
            err.to_string().contains("Update deferred"),
            "unexpected error: {err}"
        );
        assert!(
            err.to_string().contains(leftover.file_name().unwrap().to_str().unwrap()),
            "error should name the locked file: {err}"
        );
        // No job ran: nothing was moved into `old\`.
        assert!(!root.join("test1").exists());

        // Once the file is released, the lock check passes again.
        unsafe { let _ = CloseHandle(handle); };
        for _ in 0..40 {
            if find_locked_files(root).is_empty() {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        assert!(ensure_files_unlocked(root).is_ok());
    }
}
