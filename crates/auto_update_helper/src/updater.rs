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
        Foundation::{CloseHandle, ERROR_MORE_DATA, HANDLE, HWND, LPARAM, WPARAM},
        System::{
            RestartManager::{
                CCH_RM_SESSION_KEY, RM_PROCESS_INFO, RmEndSession, RmGetList, RmRegisterResources,
                RmStartSession,
            },
            Threading::{
                OpenProcess, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION,
                PROCESS_TERMINATE, QueryFullProcessImageNameW, TerminateProcess,
            },
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
            rollback: Box::new(move |app_dir| remove_created_dir(app_dir, name)),
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
            rollback: Box::new(move |app_dir| remove_created_dir(app_dir, name)),
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

                // A directory rollback that already ran may have removed the destination's parent
                // (e.g. x64\ when restoring a terminal host), so recreate it.
                if let Some(parent) = old_file.parent() {
                    std::fs::create_dir_all(parent)
                        .context(format!("Failed to create directory {}", parent.display()))?;
                }

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

                    // A directory rollback that already ran may have removed the destination's
                    // parent (e.g. x64\ when restoring a terminal host), so recreate it.
                    if let Some(parent) = old_file.parent() {
                        std::fs::create_dir_all(parent)
                            .context(format!("Failed to create directory {}", parent.display()))?;
                    }

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
            rollback: Box::new(|_| {
                // This job never fails, so its rollback is unreachable; and deleting updates\
                // would remove versions.txt, breaking the retry of a deferred update.
                Ok(())
            }),
        }
    }
}

/// Removes a directory created by a `mkdir` job without touching pre-existing contents: a
/// non-recursive removal can't delete leftovers from an earlier interrupted update, and a
/// non-empty directory is not a rollback failure (the next run's cleanup will retry).
fn remove_created_dir(app_dir: &Path, name: &'static Path) -> Result<()> {
    let dir = app_dir.join(name);
    match std::fs::remove_dir(&dir) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => log::warn!("Failed to remove directory {}: {e:#}", dir.display()),
    }
    Ok(())
}

/// The jobs that swap the staged installation over the current one. Shared between production
/// and tests so the tests always exercise the real job list.
fn swap_jobs() -> Vec<Job> {
    fn p(value: &str) -> &Path {
        Path::new(value)
    }
    vec![
        // Move old files
        // Not deleting because installing new files can fail
        Job::mkdir(p("old")),
        Job::move_file(p("Zed.exe"), p("old\\Zed.exe")),
        Job::mkdir(p("old\\bin")),
        // bin\Zed.exe and bin\zed.exe are the same file (NTFS is case-insensitive).
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
    ]
}

/// The jobs that clean up after a successful swap.
fn cleanup_jobs() -> Vec<Job> {
    fn p(value: &str) -> &Path {
        Path::new(value)
    }
    vec![
        // Cleanup installer and updates folder
        Job::rmdir_nofail(p("updates")),
        Job::rmdir_nofail(p("install")),
        // Cleanup old installation
        Job::rmdir_nofail(p("old")),
    ]
}

#[cfg(not(test))]
pub(crate) static JOBS: LazyLock<Vec<Job>> = LazyLock::new(|| {
    let mut jobs = swap_jobs();
    jobs.extend(cleanup_jobs());
    jobs
});

/// A job that fails when ZED_AUTO_UPDATE equals `value`; its rollback sets ZED_AUTO_UPDATE_RB.
#[cfg(test)]
fn fail_on(value: &'static str) -> Job {
    Job {
        apply: Box::new(move |_| {
            std::thread::sleep(Duration::from_millis(1000));
            if let Ok(config) = std::env::var("ZED_AUTO_UPDATE") {
                if config == value {
                    Err(std::io::Error::other("Simulated error")).context("Anyhow!")
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
        }),
        rollback: Box::new(move |_| {
            unsafe { std::env::set_var("ZED_AUTO_UPDATE_RB", "rollback1") };
            Ok(())
        }),
    }
}

#[cfg(test)]
pub(crate) static JOBS: LazyLock<Vec<Job>> = LazyLock::new(|| {
    // The production job list (so tests exercise the real swap and its rollback order), with a
    // failure-injecting job at the front (abort before any job ran) and after the swap
    // (failure mid-swap, before cleanup).
    let mut jobs = vec![fail_on("err1")];
    jobs.extend(swap_jobs());
    jobs.push(fail_on("err2"));
    jobs.extend(cleanup_jobs());
    jobs
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

    /// Registers a file so that `locked_holders` reports the processes holding it.
    fn register_file(&self, path: &Path) -> bool {
        let wide_path: Vec<u16> = OsStr::new(path.as_os_str())
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let pcwstr = PCWSTR::from_raw(wide_path.as_ptr());
        unsafe { RmRegisterResources(self.handle, Some(std::slice::from_ref(&pcwstr)), None, None) }
            .is_ok()
    }

    /// The processes holding handles to the registered file(s), if the query succeeded.
    fn locked_holders(&self) -> Option<Vec<LockHolder>> {
        let mut needed: u32 = 0;
        let mut count: u32 = 0;
        let mut reboot_reasons: u32 = 0;

        // First call without a buffer: ERROR_MORE_DATA means holders were found, with `needed`
        // set to how many.
        let result = unsafe {
            RmGetList(
                self.handle,
                &mut needed,
                &mut count,
                None,
                &mut reboot_reasons,
            )
        };
        if !(result.is_ok() || result == ERROR_MORE_DATA) {
            return None;
        }
        if needed == 0 {
            return Some(Vec::new());
        }

        // Grow the buffer and retry while the OS reports that not all entries fit.
        let mut infos = vec![RM_PROCESS_INFO::default(); needed as usize];
        loop {
            let mut written = infos.len() as u32;
            let result = unsafe {
                RmGetList(
                    self.handle,
                    &mut needed,
                    &mut written,
                    Some(infos.as_mut_ptr()),
                    &mut reboot_reasons,
                )
            };
            if result.is_ok() {
                return Some(
                    infos
                        .iter()
                        .take(written as usize)
                        .map(|info| LockHolder {
                            pid: info.Process.dwProcessId,
                            name: wide_string(&info.strAppName),
                        })
                        .collect(),
                );
            }
            if result != ERROR_MORE_DATA || needed <= written {
                return None;
            }
            infos.resize(needed as usize, RM_PROCESS_INFO::default());
        }
    }
}

/// Converts a null-terminated (or full) wide string buffer to a String.
fn wide_string(buffer: &[u16]) -> String {
    let len = buffer.iter().position(|c| *c == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..len])
}

/// A process holding a handle to one of the files an update needs to move.
#[derive(Debug)]
struct LockHolder {
    pid: u32,
    name: String,
}

impl Drop for RmSession {
    fn drop(&mut self) {
        if unsafe { RmEndSession(self.handle) }.is_err() {
            log::warn!("Failed to end Restart Manager session {}", self.handle);
        }
    }
}

/// The `old\` files an update renames onto or deletes, including the destinations where
/// leftovers from a previous interrupted update may still sit. A process running from such a
/// file keeps it locked against replacement, which is what actually blocks the swap; holders of
/// other files don't prevent moving them.
fn old_installation_files(old_dir: &Path) -> Vec<PathBuf> {
    const FILE_NAMES: &[&str] = &[
        "Zed.exe",
        "bin\\Zed.exe",
        "bin\\zed",
        "OpenConsole.exe",
        "x64\\OpenConsole.exe",
        "arm64\\OpenConsole.exe",
        "conpty.dll",
    ];

    FILE_NAMES
        .iter()
        .map(|name| old_dir.join(name))
        .filter(|path| path.exists())
        .collect()
}

/// The processes holding handles to `old\` files, as (file, pid, app name) triples.
fn find_lock_holders(app_dir: &Path) -> Vec<(PathBuf, u32, String)> {
    let old_dir = app_dir.join("old");
    let mut holders = Vec::new();
    for path in old_installation_files(&old_dir) {
        let Some(session) = RmSession::new() else {
            continue;
        };
        if !session.register_file(&path) {
            continue;
        }
        if let Some(holders_of_file) = session.locked_holders() {
            for holder in holders_of_file {
                log::info!(
                    "File {} is held by process {} ({})",
                    path.display(),
                    holder.pid,
                    holder.name
                );
                holders.push((path.clone(), holder.pid, holder.name));
            }
        }
    }
    holders
}

/// Closes a handle, logging (rather than failing on) the rare close error.
fn close_handle(handle: HANDLE) {
    if unsafe { CloseHandle(handle) }.is_err() {
        log::warn!("Failed to close process handle");
    }
}

/// The full image path of a process, if it can be resolved.
fn image_path_of(handle: HANDLE) -> Option<String> {
    let mut buffer = [0u16; 1024];
    let mut size = buffer.len() as u32;
    let ok = unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        )
    }
    .is_ok();
    ok.then(|| wide_string(&buffer[..size as usize]))
}

/// True if `image` is a path inside `dir` (case-insensitive, as Windows paths are).
fn is_inside(image: &str, dir: &Path) -> bool {
    let image = image.to_lowercase();
    let dir = dir.to_string_lossy().to_lowercase();
    image.starts_with(&format!("{}\\", dir.trim_end_matches('\\')))
}

/// True if `pid` can be opened and its image path is inside `dir`.
fn is_running_from(pid: u32, dir: &Path) -> bool {
    let Ok(handle) = (unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }) else {
        return false;
    };
    let running_from = image_path_of(handle).is_some_and(|image| is_inside(&image, dir));
    close_handle(handle);
    running_from
}

/// Terminates `pid` if its image is inside `dir`. The image check and the termination use the
/// same process handle so a reused PID can't redirect the kill to an unrelated process.
fn terminate_process_running_from(pid: u32, dir: &Path) {
    let Ok(handle) = (unsafe {
        OpenProcess(
            PROCESS_TERMINATE | PROCESS_QUERY_LIMITED_INFORMATION,
            false,
            pid,
        )
    }) else {
        log::warn!("Failed to open process {pid} for termination");
        return;
    };
    match image_path_of(handle) {
        Some(image) if is_inside(&image, dir) => {
            log::info!(
                "Terminating leftover process {pid} ({image}) running from {}",
                dir.display()
            );
            if unsafe { TerminateProcess(handle, 0) }.is_err() {
                log::warn!("Failed to terminate process {pid} ({image})");
            }
        }
        Some(image) => {
            log::info!(
                "Leaving process {pid} ({image}) alone: it is not running from {}",
                dir.display()
            );
        }
        None => log::warn!("Failed to resolve the image path of process {pid}; leaving it alone"),
    }
    close_handle(handle);
}

/// Terminates leftover processes running from `old\` and defers the update if one still holds a
/// file after that.
///
/// A process running from such a file keeps its image locked against replacement, which is what
/// actually blocks the swap; by the time an update runs it should already be gone, so it is
/// terminated. Holders of other files (the app shutting down, Explorer's icon cache, antivirus)
/// don't prevent moving them, so they are left alone. If a leftover survives termination, the
/// update is deferred: the installation is left untouched and the swap will be retried when Zed
/// exits.
///
/// Note that a `bin\zed.exe` CLI that was waiting (e.g. as a git editor) across an update is
/// itself running from `old\bin\` by the next update, and is terminated then; that is rare and
/// acceptable.
fn ensure_files_unlocked(app_dir: &Path) -> Result<()> {
    let old_dir = app_dir.join("old");
    let holders = find_lock_holders(app_dir);
    if holders.is_empty() {
        return Ok(());
    }

    // Terminate the leftover processes running from `old\`; any other holder is left alone.
    let mut pids: Vec<u32> = holders.iter().map(|(_, pid, _)| *pid).collect();
    pids.sort_unstable();
    pids.dedup();
    for pid in pids {
        terminate_process_running_from(pid, &old_dir);
    }

    // Give the terminated processes a moment to release their handles; only leftovers still
    // running from `old\` can block the swap.
    let mut remaining = Vec::new();
    for _ in 0..4 {
        std::thread::sleep(Duration::from_millis(500));
        remaining = find_lock_holders(app_dir)
            .into_iter()
            .filter(|(_, pid, _)| is_running_from(*pid, &old_dir))
            .collect();
        if remaining.is_empty() {
            return Ok(());
        }
    }

    let mut files: Vec<String> = remaining
        .iter()
        .map(|(path, _, _)| path.display().to_string())
        .collect();
    files.sort();
    files.dedup();
    anyhow::bail!(
        "the following files are still held by processes running from old\\: {}",
        files.join(", ")
    )
}

/// An update that was deferred because files are still locked; the swap will be retried when
/// Zed exits next. A distinct error type so callers can tell a deferral apart from a real
/// failure and not bother the user about it.
#[derive(Debug)]
pub(crate) struct UpdateDeferred(pub(crate) anyhow::Error);

impl std::fmt::Display for UpdateDeferred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The inner error is not repeated here; source() already surfaces it in chains.
        f.write_str("Update deferred, it will be retried when Zed exits")
    }
}

impl std::error::Error for UpdateDeferred {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
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

    // The app only runs us when an update has been staged (the installer writes the new files to
    // install\ and records the version in updates\versions.txt). Refuse to touch anything if there
    // is nothing to install: running the swap anyway would move the current installation into old\
    // with nothing to replace it.
    let versions_file = app_dir.join("updates\\versions.txt");
    if !versions_file.exists() {
        anyhow::bail!(
            "No pending update found ({} is missing)",
            versions_file.display()
        );
    }

    // Before moving anything, terminate leftover processes running from old\ and make sure none
    // of them still holds a file the swap renames onto; if one survives, defer the update: the
    // installation is left untouched and the swap will be retried when Zed exits.
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
        return Err(anyhow::Error::new(UpdateDeferred(e)));
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
                "Autoupdate failed, and {} rollback(s) also failed (jobs {failed_rollbacks:?}), so the app might be left in an inconsistent state",
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
        time::{Duration, Instant},
    };

    use super::{
        ensure_files_unlocked, find_lock_holders, is_inside, perform_update, zed_launch_command,
    };

    // `perform_update` tests read and set process-wide environment variables (the test jobs
    // simulate failures via ZED_AUTO_UPDATE), so they must not run concurrently.
    static PERFORM_UPDATE_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Makes `app_dir` look like an installed Zed with a staged update, so the production job
    /// list can run against it.
    fn stage_test_update(app_dir: &Path) {
        // Current installation.
        std::fs::write(app_dir.join("Zed.exe"), b"current-zed").unwrap();
        std::fs::create_dir_all(app_dir.join("bin")).unwrap();
        std::fs::write(app_dir.join("bin\\Zed.exe"), b"current-bin-zed").unwrap();
        std::fs::write(app_dir.join("bin\\zed"), b"current-bin-zed-script").unwrap();
        std::fs::write(app_dir.join("conpty.dll"), b"current-conpty").unwrap();
        std::fs::create_dir_all(app_dir.join("x64")).unwrap();
        std::fs::write(app_dir.join("x64\\OpenConsole.exe"), b"current-x64").unwrap();
        std::fs::create_dir_all(app_dir.join("arm64")).unwrap();
        std::fs::write(app_dir.join("arm64\\OpenConsole.exe"), b"current-arm64").unwrap();

        // Staged update.
        std::fs::create_dir_all(app_dir.join("updates")).unwrap();
        std::fs::write(app_dir.join("updates\\versions.txt"), "test").unwrap();
        std::fs::create_dir_all(app_dir.join("install\\bin")).unwrap();
        std::fs::write(app_dir.join("install\\Zed.exe"), b"new-zed").unwrap();
        std::fs::write(app_dir.join("install\\bin\\Zed.exe"), b"new-bin-zed").unwrap();
        std::fs::write(app_dir.join("install\\bin\\zed"), b"new-bin-zed-script").unwrap();
        std::fs::write(app_dir.join("install\\conpty.dll"), b"new-conpty").unwrap();
        std::fs::create_dir_all(app_dir.join("install\\x64")).unwrap();
        std::fs::write(app_dir.join("install\\x64\\OpenConsole.exe"), b"new-x64").unwrap();
        std::fs::create_dir_all(app_dir.join("install\\arm64")).unwrap();
        std::fs::write(
            app_dir.join("install\\arm64\\OpenConsole.exe"),
            b"new-arm64",
        )
        .unwrap();
    }

    /// A spawned process that is killed if the test ends before it exits, so a failed
    /// assertion can't leave it running.
    struct StrayProcess {
        child: std::process::Child,
        exited: bool,
    }

    impl StrayProcess {
        /// Waits up to `timeout` for the process to exit, returning whether it did.
        fn wait_exited(&mut self, timeout: Duration) -> bool {
            let start = Instant::now();
            while start.elapsed() < timeout {
                match self.child.try_wait() {
                    Ok(Some(_)) => {
                        self.exited = true;
                        return true;
                    }
                    Ok(None) => {}
                    Err(e) => {
                        log::warn!("Failed to wait for stray test process: {e}");
                        return false;
                    }
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            false
        }
    }

    impl Drop for StrayProcess {
        fn drop(&mut self) {
            if self.exited {
                return;
            }
            if let Err(e) = self.child.kill() {
                log::warn!("Failed to kill stray test process: {e}");
            }
        }
    }

    use windows::{
        Win32::{
            Foundation::{CloseHandle, GENERIC_READ},
            Storage::FileSystem::{
                CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE,
                OPEN_EXISTING,
            },
        },
        core::PCWSTR,
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
        let _guard = PERFORM_UPDATE_MUTEX.lock().unwrap();

        // A full swap succeeds and leaves the new installation in place.
        let app_dir = tempfile::tempdir().unwrap();
        stage_test_update(app_dir.path());
        assert!(perform_update(app_dir.path(), None, false, &[]).is_ok());
        assert_eq!(
            std::fs::read(app_dir.path().join("Zed.exe")).unwrap(),
            b"new-zed"
        );
        assert_eq!(
            std::fs::read(app_dir.path().join("x64\\OpenConsole.exe")).unwrap(),
            b"new-x64"
        );
        assert!(!app_dir.path().join("old").exists());

        // A failure before any job ran leaves everything untouched.
        let app_dir = tempfile::tempdir().unwrap();
        stage_test_update(app_dir.path());
        unsafe { std::env::set_var("ZED_AUTO_UPDATE", "err1") };
        let ret = perform_update(app_dir.path(), None, false, &[]);
        assert!(
            ret.is_err_and(|e| e.to_string().as_str() == "Autoupdate failed, nothing to rollback")
        );
        assert_eq!(
            std::fs::read(app_dir.path().join("Zed.exe")).unwrap(),
            b"current-zed"
        );
        assert!(!app_dir.path().join("old").exists());

        // A failure mid-swap rolls back every job and restores the current installation,
        // including files whose destination directory was removed by an earlier rollback.
        let app_dir = tempfile::tempdir().unwrap();
        stage_test_update(app_dir.path());
        unsafe { std::env::set_var("ZED_AUTO_UPDATE", "err2") };
        let ret = perform_update(app_dir.path(), None, false, &[]);
        assert!(
            ret.is_err_and(|e| e.to_string().as_str() == "Autoupdate failed, rollback successful")
        );
        assert!(std::env::var("ZED_AUTO_UPDATE_RB").is_ok_and(|e| e == "rollback1"));
        // The install\x64 move rollback emptied x64\, then the mkdir_if_exists(x64) rollback
        // removed it; the old\x64\OpenConsole.exe move rollback must recreate it.
        assert_eq!(
            std::fs::read(app_dir.path().join("x64\\OpenConsole.exe")).unwrap(),
            b"current-x64"
        );
        assert_eq!(
            std::fs::read(app_dir.path().join("Zed.exe")).unwrap(),
            b"current-zed"
        );
        assert!(!app_dir.path().join("old").exists());

        // The test jobs read these process-wide variables; clear them so other tests are not
        // affected.
        unsafe {
            std::env::remove_var("ZED_AUTO_UPDATE");
            std::env::remove_var("ZED_AUTO_UPDATE_RB");
        }
    }

    #[test]
    fn test_perform_update_without_staged_update_fails_cleanly() {
        let app_dir = tempfile::tempdir().unwrap();
        let root = app_dir.path();
        std::fs::write(root.join("Zed.exe"), b"").unwrap();

        // Running the helper without a staged update must refuse to touch anything instead of
        // moving the current installation into old\ with nothing to replace it.
        let err = perform_update(root, None, false, &[])
            .expect_err("should refuse to run without a staged update");
        assert!(err.to_string().contains("No pending update"));
        assert!(
            root.join("Zed.exe").exists(),
            "installation must be untouched"
        );
        assert!(!root.join("old").exists(), "no job may have run");
    }

    #[test]
    fn test_ensure_files_unlocked_succeeds_when_nothing_is_locked() {
        let app_dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(app_dir.path().join("old\\x64")).unwrap();
        std::fs::write(app_dir.path().join("old\\x64\\OpenConsole.exe"), b"").unwrap();

        assert!(ensure_files_unlocked(app_dir.path()).is_ok());
    }

    #[test]
    fn test_find_lock_holders_reports_holder() {
        let app_dir = tempfile::tempdir().unwrap();
        let file = app_dir.path().join("old\\Zed.exe");
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(&file, b"").unwrap();

        // Open the file so Restart Manager reports this process as holding it.
        let wide_path: Vec<u16> = OsStr::new(file.as_os_str())
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let handle = unsafe {
            CreateFileW(
                PCWSTR::from_raw(wide_path.as_ptr()),
                GENERIC_READ.0,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
        }
        .unwrap();

        // Restart Manager may take a moment to notice the new handle.
        let mut holders = Vec::new();
        for _ in 0..20 {
            holders = find_lock_holders(app_dir.path());
            if !holders.is_empty() {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        assert!(
            holders
                .iter()
                .any(|(path, pid, _)| *path == file && *pid == std::process::id()),
            "expected {} to be reported as held by our own process",
            file.display()
        );

        if unsafe { CloseHandle(handle) }.is_err() {
            log::warn!("Failed to close test file handle");
        }
    }

    #[test]
    fn test_is_inside_classification() {
        let old_dir = Path::new(r"C:\Users\miku\AppData\Local\Programs\Zed\old");

        // Processes running from the old installation are leftovers.
        assert!(is_inside(
            r"C:\Users\miku\AppData\Local\Programs\Zed\old\x64\OpenConsole.exe",
            old_dir
        ));
        assert!(is_inside(
            r"C:\Users\miku\AppData\Local\Programs\Zed\old\bin\zed.exe",
            old_dir
        ));

        // Matching is case-insensitive.
        assert!(is_inside(
            r"c:\users\MIKU\appdata\local\programs\zed\OLD\X64\OPENCONSOLE.EXE",
            old_dir
        ));

        // A different directory that merely starts with the same name is not inside.
        assert!(!is_inside(
            r"C:\Users\miku\AppData\Local\Programs\Zed\old2\x64\OpenConsole.exe",
            old_dir
        ));

        // The current installation is not a leftover.
        assert!(!is_inside(
            r"C:\Users\miku\AppData\Local\Programs\Zed\Zed.exe",
            old_dir
        ));
        assert!(!is_inside(
            r"C:\Users\miku\AppData\Local\Programs\Zed\bin\zed.exe",
            old_dir
        ));
    }

    /// Regression test for the incident where a leftover `old\x64\OpenConsole.exe` held by a
    /// process running from it made the update fail mid-swap and the rollback strand `Zed.exe`
    /// in `old\`, making the app disappear. The stray process must now be terminated and the
    /// swap renamed onto the leftover's path, instead of failing.
    #[allow(
        clippy::disallowed_methods,
        reason = "test spawns a stray process to terminate"
    )]
    #[test]
    fn test_perform_update_terminates_stray_zed_process() {
        let _guard = PERFORM_UPDATE_MUTEX.lock().unwrap();
        let app_dir = tempfile::tempdir().unwrap();
        let root = app_dir.path();
        stage_test_update(root);

        // A leftover from a previous interrupted update, with a "stray process" running from it:
        // a copy of powershell.exe living in old\x64, like a zombie terminal host would. Its
        // image lock keeps the file from being replaced, which is what actually blocks the swap.
        let leftover = root.join("old\\x64\\OpenConsole.exe");
        std::fs::create_dir_all(leftover.parent().unwrap()).unwrap();
        let system_powershell = Path::new(&std::env::var("WINDIR").unwrap())
            .join(r"System32\WindowsPowerShell\v1.0\powershell.exe");
        std::fs::copy(&system_powershell, &leftover).unwrap();

        let mut stray = StrayProcess {
            child: std::process::Command::new(&leftover)
                .args([
                    "-NoProfile",
                    "-NonInteractive",
                    "-Command",
                    "Start-Sleep -Seconds 120",
                ])
                .spawn()
                .unwrap(),
            exited: false,
        };

        // Wait until the stray's image lock is visible to Restart Manager.
        let mut locked = false;
        for _ in 0..50 {
            if find_lock_holders(root)
                .iter()
                .any(|(path, _, _)| path == &leftover)
            {
                locked = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(200));
        }
        assert!(locked, "stray process should be holding the leftover file");

        // The update must terminate the stray and complete.
        perform_update(root, None, false, &[])
            .expect("update should complete after terminating the stray process");

        assert!(
            stray.wait_exited(Duration::from_secs(5)),
            "stray process should have been terminated"
        );

        // The swap renamed onto the leftover's path and the new file took its place.
        assert_eq!(
            std::fs::read(root.join("x64\\OpenConsole.exe")).unwrap(),
            b"new-x64"
        );
        assert!(!root.join("old").exists());
    }
}
