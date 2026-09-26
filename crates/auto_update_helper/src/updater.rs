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
        Foundation::{CloseHandle, ERROR_MORE_DATA, HWND, LPARAM, WPARAM},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW,
                TH32CS_SNAPPROCESS,
            },
            RestartManager::{
                CCH_RM_SESSION_KEY, RmEndSession, RmGetList, RmRegisterResources, RmStartSession,
                RM_PROCESS_INFO,
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
        // In the two-call query pattern, ERROR_MORE_DATA is a success signal meaning
        // processes were found (with `needed` set to how many).
        let result =
            unsafe { RmGetList(self.handle, &mut needed, &mut count, None, &mut reboot_reasons) };
        if !(result.is_ok() || result == ERROR_MORE_DATA) {
            return None;
        }
        if needed == 0 {
            return Some(Vec::new());
        }

        // Allocate for the largest known layout so any of them fits, and declare the buffer's
        // entry capacity in both count parameters: different rstrtmgr.dll versions read it from a
        // different one (passing zero makes the call fail with ERROR_MORE_DATA).
        let max_entry = RM_PROCESS_INFO_LAYOUTS.iter().map(|layout| layout.0).max().unwrap();
        let mut buffer = vec![0u8; needed as usize * max_entry];
        let mut cap1 = needed;
        let mut cap2 = needed;
        let result = unsafe {
            RmGetList(
                self.handle,
                &mut cap1,
                &mut cap2,
                Some(buffer.as_mut_ptr() as *mut RM_PROCESS_INFO),
                &mut reboot_reasons,
            )
        };
        // The number of filled entries comes back in pnProcInfo (the capacity parameter).
        let count = cap2;
        if !(result.is_ok() || (result == ERROR_MORE_DATA && count > 0)) {
            return None;
        }

        // Work out which layout the OS wrote. With several entries, validate the PIDs parsed at
        // each candidate stride against the live process table: a wrong stride yields implausible
        // PIDs. With a single entry the PID is at offset 0 in every layout, so any works.
        let (entry_size, pid_offset, name_offset) = if count >= 2 {
            let live = live_process_pids();
            RM_PROCESS_INFO_LAYOUTS
                .iter()
                .copied()
                .find(|&(candidate, pid_candidate, _)| {
                    (0..count as usize).all(|i| {
                        let pid = u32::from_le_bytes(
                            buffer[i * candidate + pid_candidate..i * candidate + pid_candidate + 4]
                                .try_into()
                                .unwrap(),
                        );
                        live.as_ref().is_some_and(|pids| pids.contains(&pid))
                    })
                })
                .unwrap_or(RM_PROCESS_INFO_LAYOUTS[0])
        } else {
            RM_PROCESS_INFO_LAYOUTS[0]
        };
        Some(parse_rm_holders(
            &buffer,
            count,
            entry_size,
            pid_offset,
            name_offset,
        ))
    }
}

/// Candidate `RM_PROCESS_INFO` layouts written by rstrtmgr.dll: (entry size, PID offset,
/// app-name offset). The struct changed between Windows versions (the process field became
/// {DWORD, FILETIME} and the app name shrank from MAX_PATH to 256 wide chars), so we detect which
/// one was used at runtime.
const RM_PROCESS_INFO_LAYOUTS: &[(usize, usize, usize)] = &[
    // Current Windows: Process = {DWORD, FILETIME} (12 bytes), strAppName[256].
    (12 + 2 * 256 + 2 * 64 + 4 * 4, 0, 12),
    // Classic layout: Process = {DWORD, HINSTANCE} (16 bytes), strAppName[MAX_PATH = 260].
    (16 + 2 * 260 + 2 * 64 + 4 * 4, 0, 16),
];

/// Parses PIDs and app names out of a `RmGetList` buffer laid out per `RM_PROCESS_INFO_LAYOUTS`.
fn parse_rm_holders(
    buffer: &[u8],
    count: u32,
    entry_size: usize,
    pid_offset: usize,
    name_offset: usize,
) -> Vec<LockHolder> {
    (0..count as usize)
        .map(|i| {
            let base = i * entry_size;
            let pid = u32::from_le_bytes(buffer[base + pid_offset..base + pid_offset + 4].try_into().unwrap());
            let name_bytes = &buffer[base + name_offset..base + entry_size];
            let wide: Vec<u16> = name_bytes
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            let name_len = wide.iter().position(|c| *c == 0).unwrap_or(wide.len());
            LockHolder {
                pid,
                name: String::from_utf16_lossy(&wide[..name_len]),
            }
        })
        .collect()
}

/// A process holding a handle to one of the files an update needs to move.
#[derive(Debug)]
struct LockHolder {
    pid: u32,
    name: String,
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

/// Uses Windows Restart Manager to find which of the files an update needs to move are held by
/// other processes, and which processes hold them.
fn find_lock_holders(app_dir: &Path) -> Vec<(PathBuf, u32, String)> {
    let mut holders = Vec::new();
    for path in files_involved(app_dir) {
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

/// The full image path of a process, if it can be resolved.
fn process_image_path(pid: u32) -> Option<String> {
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }.ok()?;
    let mut buffer = [0u16; 1024];
    let mut size = buffer.len() as u32;
    let image = unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        )
    }
    .is_ok()
    .then(|| {
        String::from_utf16_lossy(&buffer[..size as usize])
            .trim_end_matches('\0')
            .to_string()
    });
    unsafe { let _ = CloseHandle(handle); };
    image
}

/// True for processes that belong to this Zed installation: anything running from the install
/// directory (Zed.exe, bin\*, x64\*, arm64\*, old\*, tools\*) or from a leftover installer copy.
fn is_zed_related_image(image: &str, app_dir: &Path) -> bool {
    let image_lower = image.to_lowercase();
    let app_root = app_dir.to_string_lossy().to_lowercase();
    let app_root = app_root.trim_end_matches('\\');
    if image_lower.starts_with(&format!("{app_root}\\")) {
        return true;
    }
    // The downloaded installer runs from a zed-auto-update temp directory (see
    // auto_update::install_release_windows), and Inno renames itself to *.tmp while running.
    let file_name = image_lower.rsplit('\\').next().unwrap_or(image_lower.as_str());
    image_lower.contains("\\zed-auto-update")
        || (file_name.starts_with("zed-") && file_name.ends_with(".tmp"))
}

/// Terminates a process, returning whether the termination request succeeded.
fn terminate_process(pid: u32) -> bool {
    let Ok(handle) = (unsafe { OpenProcess(PROCESS_TERMINATE, false, pid) }) else {
        return false;
    };
    let terminated = unsafe { TerminateProcess(handle, 0) }.is_ok();
    unsafe { let _ = CloseHandle(handle); };
    terminated
}

/// The PIDs of all currently running processes, if a snapshot could be taken.
fn live_process_pids() -> Option<std::collections::HashSet<u32>> {
    let Ok(snapshot) = (unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }) else {
        return None;
    };
    let mut entry: PROCESSENTRY32W = unsafe { std::mem::zeroed() };
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
    let mut pids = std::collections::HashSet::new();
    if unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
        loop {
            pids.insert(entry.th32ProcessID);
            if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
                break;
            }
        }
    }
    unsafe { let _ = CloseHandle(snapshot); };
    Some(pids)
}

/// The PID of the process that spawned us (the Zed instance shutting down), if it can be found.
fn own_parent_pid() -> Option<u32> {
    let Ok(snapshot) = (unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }) else {
        return None;
    };
    let mut entry: PROCESSENTRY32W = unsafe { std::mem::zeroed() };
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
    let own_pid = std::process::id();
    let mut parent_pid = None;
    if unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
        loop {
            if entry.th32ProcessID == own_pid {
                parent_pid = Some(entry.th32ParentProcessID);
                break;
            }
            if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
                break;
            }
        }
    }
    unsafe { let _ = CloseHandle(snapshot); };
    parent_pid
}

/// Whether a holder is one of Zed's own stray processes (running from the install directory or a
/// leftover installer copy): by the time an update runs it should already be gone, and it keeps
/// the old binaries locked. Excludes ourselves, which never holds blocking handles.
fn is_zed_stray_holder(pid: u32, own_pid: u32, app_dir: &Path) -> bool {
    pid != own_pid
        && process_image_path(pid).is_some_and(|image| is_zed_related_image(&image, app_dir))
}

/// Makes sure no process holds any of the files an update needs to move in a way that would block
/// the swap.
///
/// Zed's own stray processes (a zombie Zed.exe, an orphaned terminal host running from `old\`, a
/// leftover installer subprocess) keep the old binaries locked; by the time an update runs they
/// should already be gone, so they are terminated. Other holders (e.g. Explorer's icon cache
/// handles) don't block moving files, so the swap proceeds with them present. If a stray process
/// survives termination, the update is deferred: the installation is left untouched and the swap
/// is retried the next time Zed exits or restarts.
fn ensure_files_unlocked(app_dir: &Path) -> Result<()> {
    let mut holders = find_lock_holders(app_dir);
    if holders.is_empty() {
        return Ok(());
    }

    let own_pid = std::process::id();
    let parent_pid = own_parent_pid();

    // Terminate Zed's own stray processes that are holding files.
    for (_, pid, _) in &holders {
        if *pid == own_pid || Some(*pid) == parent_pid {
            continue;
        }
        if let Some(image) = process_image_path(*pid)
            && is_zed_related_image(&image, app_dir)
        {
            log::info!(
                "Terminating stray Zed process {} ({}) that holds a locked file",
                pid,
                image
            );
            terminate_process(*pid);
        }
    }

    // Wait for terminated processes to release their handles; only strays still alive can block
    // the swap. The process that spawned us (the app shutting down) counts as blocking: we never
    // kill it, but if it still holds its image the swap would fail, so defer and retry on the
    // next exit.
    for _ in 0..4 {
        std::thread::sleep(Duration::from_millis(500));
        holders = find_lock_holders(app_dir);
        if !holders
            .iter()
            .any(|(_, pid, _)| is_zed_stray_holder(*pid, own_pid, app_dir))
        {
            return Ok(());
        }
    }

    let mut files: Vec<String> = holders
        .iter()
        .filter(|(_, pid, _)| is_zed_stray_holder(*pid, own_pid, app_dir))
        .map(|(path, _, _)| path.display().to_string())
        .collect();
    files.sort();
    files.dedup();
    anyhow::bail!(
        "the following files are still held by other processes: {}",
        files.join(", ")
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

    // Before moving anything, make sure no process holds the files we need to move in a way that
    // would block the swap. Stray Zed processes are terminated; if one survives, defer the update:
    // the installation is left untouched and the swap will be retried the next time Zed exits or
    // restarts.
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

    use super::{
        ensure_files_unlocked, find_lock_holders, is_zed_related_image, perform_update,
        zed_launch_command,
    };

    // `perform_update` tests read and set process-wide environment variables (the test jobs
    // simulate failures via ZED_AUTO_UPDATE), so they must not run concurrently.
    static PERFORM_UPDATE_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Creates the flag file the installer writes when it stages an update, so that
    /// `perform_update` sees a pending update.
    fn stage_test_update(app_dir: &Path) {
        std::fs::create_dir_all(app_dir.join("updates")).unwrap();
        std::fs::write(app_dir.join("updates\\versions.txt"), "test").unwrap();
    }
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
        let _guard = PERFORM_UPDATE_MUTEX.lock().unwrap();
        let app_dir = tempfile::tempdir().unwrap();
        let app_dir = app_dir.path();
        stage_test_update(app_dir);
        assert!(perform_update(app_dir, None, false, &[]).is_ok());

        let app_dir = tempfile::tempdir().unwrap();
        let app_dir = app_dir.path();
        stage_test_update(app_dir);
        // Simulate a timeout
        unsafe { std::env::set_var("ZED_AUTO_UPDATE", "err1") };
        let ret = perform_update(app_dir, None, false, &[]);
        assert!(
            ret.is_err_and(|e| e.to_string().as_str() == "Autoupdate failed, nothing to rollback")
        );

        let app_dir = tempfile::tempdir().unwrap();
        let app_dir = app_dir.path();
        stage_test_update(app_dir);
        // Simulate a timeout
        unsafe { std::env::set_var("ZED_AUTO_UPDATE", "err2") };
        let ret = perform_update(app_dir, None, false, &[]);
        assert!(
            ret.is_err_and(|e| e.to_string().as_str() == "Autoupdate failed, rollback successful")
        );
        assert!(std::env::var("ZED_AUTO_UPDATE_RB").is_ok_and(|e| e == "rollback1"));

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
        assert!(root.join("Zed.exe").exists(), "installation must be untouched");
        assert!(!root.join("old").exists(), "no job may have run");
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
    fn test_find_lock_holders_reports_holder() {
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

        unsafe { let _ = CloseHandle(handle); };
    }

    #[test]
    fn test_is_zed_related_image_classification() {
        let app_dir = Path::new(r"C:\Users\miku\AppData\Local\Programs\Zed");

        // Processes running from the installation directory are ours.
        assert!(is_zed_related_image(
            r"C:\Users\miku\AppData\Local\Programs\Zed\Zed.exe",
            app_dir
        ));
        assert!(is_zed_related_image(
            r"C:\Users\miku\AppData\Local\Programs\Zed\bin\zed.exe",
            app_dir
        ));
        assert!(is_zed_related_image(
            r"C:\Users\miku\AppData\Local\Programs\Zed\old\x64\OpenConsole.exe",
            app_dir
        ));

        // Matching is case-insensitive.
        assert!(is_zed_related_image(
            r"c:\users\MIKU\appdata\local\programs\zed\ZED.EXE",
            app_dir
        ));

        // A different directory that merely starts with the same name is not ours.
        assert!(!is_zed_related_image(
            r"C:\Users\miku\AppData\Local\Programs\Zed2\Zed.exe",
            app_dir
        ));

        // Leftover installer copies are ours too.
        assert!(is_zed_related_image(
            r"C:\Users\miku\AppData\Local\Temp\zed-auto-update1a2b3c\Zed-x86_64.exe",
            app_dir
        ));
        assert!(is_zed_related_image(r"D:\downloads\Zed-x86_64.tmp", app_dir));

        // Everything else is not.
        assert!(!is_zed_related_image(
            r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",
            app_dir
        ));
        assert!(!is_zed_related_image(r"C:\Windows\explorer.exe", app_dir));
    }

    /// Regression test for the incident where a leftover `old\x64\OpenConsole.exe` held by a
    /// process running from it made the update fail mid-swap and the rollback strand `Zed.exe`
    /// in `old\`, making the app disappear. The stray process must now be terminated and the
    /// update complete, instead of failing.
    #[allow(clippy::disallowed_methods, reason = "test spawns a stray process to terminate")]
    #[test]
    fn test_perform_update_terminates_stray_zed_process() {
        let _guard = PERFORM_UPDATE_MUTEX.lock().unwrap();
        let app_dir = tempfile::tempdir().unwrap();
        let root = app_dir.path();
        stage_test_update(root);

        // A leftover from a previous interrupted update.
        std::fs::create_dir_all(root.join("old\\x64")).unwrap();
        let leftover = root.join("old\\x64\\OpenConsole.exe");
        std::fs::write(&leftover, b"older").unwrap();

        // A "stray Zed process": a copy of powershell.exe living in the install directory,
        // holding the leftover with a share mode that blocks rename/delete, like a running
        // image would.
        std::fs::create_dir_all(root.join("bin")).unwrap();
        let stray_image = root.join("bin\\zed.exe");
        let system_powershell = Path::new(&std::env::var("WINDIR").unwrap())
            .join(r"System32\WindowsPowerShell\v1.0\powershell.exe");
        std::fs::copy(&system_powershell, &stray_image).unwrap();

        let hold_script = format!(
            "$f=[System.IO.File]::Open('{}',[System.IO.FileMode]::Open,[System.IO.FileAccess]::Read,[System.IO.FileShare]::None); Start-Sleep -Seconds 120",
            leftover.to_string_lossy()
        );
        let mut child = std::process::Command::new(&stray_image)
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                &hold_script,
            ])
            .spawn()
            .unwrap();

        // Wait until the stray's lock is visible to Restart Manager.
        let mut locked = false;
        for _ in 0..50 {
            if find_lock_holders(root).iter().any(|(path, _, _)| path == &leftover) {
                locked = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(200));
        }
        assert!(locked, "stray process should be holding the leftover file");

        // The update must terminate the stray and complete.
        match perform_update(root, None, false, &[]) {
            Ok(()) => {}
            Err(e) => panic!("update failed: {e}"),
        }

        let mut exited = false;
        for _ in 0..50 {
            if child.try_wait().is_ok_and(|status| status.is_some()) {
                exited = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        assert!(exited, "stray process should have been terminated");
    }
}
