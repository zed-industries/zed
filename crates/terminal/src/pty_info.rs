use alacritty_terminal::tty::Pty;
#[cfg(target_os = "windows")]
use std::num::NonZeroU32;
#[cfg(unix)]
use std::os::fd::AsRawFd;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use windows::Win32::{Foundation::HANDLE, System::Threading::GetProcessId};

use sysinfo::{Pid, Process, ProcessRefreshKind, RefreshKind, System, UpdateKind};

pub struct ProcessIdGetter {
    handle: i32,
    fallback_pid: u32,
}

#[cfg(unix)]
impl ProcessIdGetter {
    fn new(pty: &Pty) -> ProcessIdGetter {
        ProcessIdGetter {
            handle: pty.file().as_raw_fd(),
            fallback_pid: pty.child().id(),
        }
    }

    fn pid(&self) -> Option<Pid> {
        let pid = unsafe { libc::tcgetpgrp(self.handle) };
        if pid < 0 {
            return Some(Pid::from_u32(self.fallback_pid));
        }
        Some(Pid::from_u32(pid as u32))
    }

    pub fn fallback_pid(&self) -> u32 {
        self.fallback_pid
    }
}

#[cfg(windows)]
impl ProcessIdGetter {
    fn new(pty: &Pty) -> ProcessIdGetter {
        let child = pty.child_watcher();
        let handle = child.raw_handle();
        let fallback_pid = child.pid().unwrap_or_else(|| unsafe {
            NonZeroU32::new_unchecked(GetProcessId(HANDLE(handle as _)))
        });

        ProcessIdGetter {
            handle: handle as i32,
            fallback_pid: u32::from(fallback_pid),
        }
    }

    fn pid(&self) -> Option<Pid> {
        let pid = unsafe { GetProcessId(HANDLE(self.handle as _)) };
        // the GetProcessId may fail and returns zero, which will lead to a stack overflow issue
        if pid == 0 {
            // in the builder process, there is a small chance, almost negligible,
            // that this value could be zero, which means child_watcher returns None,
            // GetProcessId returns 0.
            if self.fallback_pid == 0 {
                return None;
            }
            return Some(Pid::from_u32(self.fallback_pid));
        }
        Some(Pid::from_u32(pid))
    }

    pub fn fallback_pid(&self) -> u32 {
        self.fallback_pid
    }
}

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub name: String,
    pub cwd: PathBuf,
    pub argv: Vec<String>,
}

/// Fetches Zed-relevant Pseudo-Terminal (PTY) process information
pub struct PtyProcessInfo {
    system: System,
    refresh_kind: ProcessRefreshKind,
    pid_getter: ProcessIdGetter,
    pub current: Option<ProcessInfo>,
}

impl PtyProcessInfo {
    pub fn new(pty: &Pty) -> PtyProcessInfo {
        let process_refresh_kind = ProcessRefreshKind::new()
            .with_cmd(UpdateKind::Always)
            .with_cwd(UpdateKind::Always)
            .with_exe(UpdateKind::Always);
        let refresh_kind = RefreshKind::new().with_processes(process_refresh_kind);
        let system = System::new_with_specifics(refresh_kind);

        PtyProcessInfo {
            system,
            refresh_kind: process_refresh_kind,
            pid_getter: ProcessIdGetter::new(pty),
            current: None,
        }
    }

    pub fn pid_getter(&self) -> &ProcessIdGetter {
        &self.pid_getter
    }

    fn refresh(&mut self) -> Option<&Process> {
        let pid = self.pid_getter.pid()?;
        if self.system.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::Some(&[pid]),
            self.refresh_kind,
        ) == 1
        {
            self.system.process(pid)
        } else {
            None
        }
    }

    fn load(&mut self) -> Option<ProcessInfo> {
        let process = self.refresh()?;
        let cwd = process.cwd().map_or(PathBuf::new(), |p| p.to_owned());

        let info = ProcessInfo {
            name: process.name().to_str()?.to_owned(),
            cwd,
            argv: process
                .cmd()
                .iter()
                .filter_map(|s| s.to_str().map(ToOwned::to_owned))
                .collect(),
        };
        self.current = Some(info.clone());
        Some(info)
    }

    /// Updates the cached process info, returns whether the Zed-relevant info has changed
    pub fn has_changed(&mut self) -> bool {
        let current = self.load();
        let has_changed = match (self.current.as_ref(), current.as_ref()) {
            (None, None) => false,
            (Some(prev), Some(now)) => prev.cwd != now.cwd || prev.name != now.name,
            _ => true,
        };
        if has_changed {
            self.current = current;
        }
        has_changed
    }

    pub fn pid(&self) -> Option<Pid> {
        self.pid_getter.pid()
    }

    pub fn has_active_child_processes_readonly(&self) -> bool {
        let main_pid = self.pid_getter().fallback_pid();

        // Create a temporary system instance to inspect all processes
        let process_refresh_kind = ProcessRefreshKind::new()
            .with_cmd(UpdateKind::Always)
            .with_exe(UpdateKind::Always);
        let refresh_kind = RefreshKind::new().with_processes(process_refresh_kind);
        let mut temp_system = System::new_with_specifics(refresh_kind);

        temp_system
            .refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, process_refresh_kind);

        let mut shell_pids = Vec::new();

        for process in temp_system.processes().values() {
            if let Some(parent_pid) = process.parent() {
                if parent_pid == Pid::from_u32(main_pid) {
                    if let Some(name) = process.name().to_str() {
                        if is_shell_process(name) {
                            shell_pids.push(process.pid());
                        } else {
                            return true;
                        }
                    }
                }
            }
        }

        for shell_pid in &shell_pids {
            for process in temp_system.processes().values() {
                if let Some(parent_pid) = process.parent() {
                    if parent_pid == *shell_pid {
                        return true;
                    }
                }
            }
        }

        false
    }
}

/// Helper function to determine if a process name is a shell process
fn is_shell_process(process_name: &str) -> bool {
    matches!(
        process_name,
        "bash"
            | "zsh"
            | "fish"
            | "sh"
            | "csh"
            | "tcsh"
            | "ksh"
            | "dash"
            | "powershell"
            | "pwsh"
            | "cmd"
    )
}
