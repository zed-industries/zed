// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DiskUsage, Gid, Pid, Process, ProcessRefreshKind, ProcessStatus, Signal, Uid};

use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::{Path, PathBuf};

use libc::kill;

use super::utils::{get_sys_value_str, WrapMap};

#[doc(hidden)]
impl From<libc::c_char> for ProcessStatus {
    fn from(status: libc::c_char) -> ProcessStatus {
        match status {
            libc::SIDL => ProcessStatus::Idle,
            libc::SRUN => ProcessStatus::Run,
            libc::SSLEEP => ProcessStatus::Sleep,
            libc::SSTOP => ProcessStatus::Stop,
            libc::SZOMB => ProcessStatus::Zombie,
            libc::SWAIT => ProcessStatus::Dead,
            libc::SLOCK => ProcessStatus::LockBlocked,
            x => ProcessStatus::Unknown(x as _),
        }
    }
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            ProcessStatus::Idle => "Idle",
            ProcessStatus::Run => "Runnable",
            ProcessStatus::Sleep => "Sleeping",
            ProcessStatus::Stop => "Stopped",
            ProcessStatus::Zombie => "Zombie",
            ProcessStatus::Dead => "Dead",
            ProcessStatus::LockBlocked => "LockBlocked",
            _ => "Unknown",
        })
    }
}

pub(crate) struct ProcessInner {
    pub(crate) name: OsString,
    pub(crate) cmd: Vec<OsString>,
    pub(crate) exe: Option<PathBuf>,
    pub(crate) pid: Pid,
    parent: Option<Pid>,
    pub(crate) environ: Vec<OsString>,
    pub(crate) cwd: Option<PathBuf>,
    pub(crate) root: Option<PathBuf>,
    pub(crate) memory: u64,
    pub(crate) virtual_memory: u64,
    pub(crate) updated: bool,
    cpu_usage: f32,
    start_time: u64,
    run_time: u64,
    pub(crate) status: ProcessStatus,
    user_id: Uid,
    effective_user_id: Uid,
    group_id: Gid,
    effective_group_id: Gid,
    read_bytes: u64,
    old_read_bytes: u64,
    written_bytes: u64,
    old_written_bytes: u64,
}

impl ProcessInner {
    pub(crate) fn kill_with(&self, signal: Signal) -> Option<bool> {
        let c_signal = crate::sys::system::convert_signal(signal)?;
        unsafe { Some(libc::kill(self.pid.0, c_signal) == 0) }
    }

    pub(crate) fn name(&self) -> &OsStr {
        &self.name
    }

    pub(crate) fn cmd(&self) -> &[OsString] {
        &self.cmd
    }

    pub(crate) fn exe(&self) -> Option<&Path> {
        self.exe.as_deref()
    }

    pub(crate) fn pid(&self) -> Pid {
        self.pid
    }

    pub(crate) fn environ(&self) -> &[OsString] {
        &self.environ
    }

    pub(crate) fn cwd(&self) -> Option<&Path> {
        self.cwd.as_deref()
    }

    pub(crate) fn root(&self) -> Option<&Path> {
        self.root.as_deref()
    }

    pub(crate) fn memory(&self) -> u64 {
        self.memory
    }

    pub(crate) fn virtual_memory(&self) -> u64 {
        self.virtual_memory
    }

    pub(crate) fn parent(&self) -> Option<Pid> {
        self.parent
    }

    pub(crate) fn status(&self) -> ProcessStatus {
        self.status
    }

    pub(crate) fn start_time(&self) -> u64 {
        self.start_time
    }

    pub(crate) fn run_time(&self) -> u64 {
        self.run_time
    }

    pub(crate) fn cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    pub(crate) fn disk_usage(&self) -> DiskUsage {
        DiskUsage {
            written_bytes: self.written_bytes.saturating_sub(self.old_written_bytes),
            total_written_bytes: self.written_bytes,
            read_bytes: self.read_bytes.saturating_sub(self.old_read_bytes),
            total_read_bytes: self.read_bytes,
        }
    }

    pub(crate) fn user_id(&self) -> Option<&Uid> {
        Some(&self.user_id)
    }

    pub(crate) fn effective_user_id(&self) -> Option<&Uid> {
        Some(&self.effective_user_id)
    }

    pub(crate) fn group_id(&self) -> Option<Gid> {
        Some(self.group_id)
    }

    pub(crate) fn effective_group_id(&self) -> Option<Gid> {
        Some(self.effective_group_id)
    }

    pub(crate) fn wait(&self) {
        let mut status = 0;
        // attempt waiting
        unsafe {
            if retry_eintr!(libc::waitpid(self.pid.0, &mut status, 0)) < 0 {
                // attempt failed (non-child process) so loop until process ends
                let duration = std::time::Duration::from_millis(10);
                while kill(self.pid.0, 0) == 0 {
                    std::thread::sleep(duration);
                }
            }
        }
    }

    pub(crate) fn session_id(&self) -> Option<Pid> {
        unsafe {
            let session_id = libc::getsid(self.pid.0);
            if session_id < 0 {
                None
            } else {
                Some(Pid(session_id))
            }
        }
    }
}

pub(crate) unsafe fn get_process_data(
    kproc: &libc::kinfo_proc,
    wrap: &WrapMap,
    page_size: isize,
    fscale: f32,
    now: u64,
    refresh_kind: ProcessRefreshKind,
) -> Result<Option<Process>, ()> {
    if kproc.ki_pid != 1 && (kproc.ki_flag as libc::c_int & libc::P_SYSTEM) != 0 {
        // We filter out the kernel threads.
        return Err(());
    }

    // We now get the values needed for both new and existing process.
    let cpu_usage = if refresh_kind.cpu() {
        (100 * kproc.ki_pctcpu) as f32 / fscale
    } else {
        0.
    };
    // Processes can be reparented apparently?
    let parent = if kproc.ki_ppid != 0 {
        Some(Pid(kproc.ki_ppid))
    } else {
        None
    };
    let status = ProcessStatus::from(kproc.ki_stat);

    // from FreeBSD source /src/usr.bin/top/machine.c
    let (virtual_memory, memory) = if refresh_kind.memory() {
        (
            kproc.ki_size as _,
            (kproc.ki_rssize as u64).saturating_mul(page_size as _),
        )
    } else {
        (0, 0)
    };

    // FIXME: This is to get the "real" run time (in micro-seconds).
    // let run_time = (kproc.ki_runtime + 5_000) / 10_000;

    let start_time = kproc.ki_start.tv_sec as u64;

    if let Some(proc_) = (*wrap.0.get()).get_mut(&Pid(kproc.ki_pid)) {
        let proc_ = &mut proc_.inner;
        proc_.updated = true;
        // If the `start_time` we just got is different from the one stored, it means it's not the
        // same process.
        if proc_.start_time == start_time {
            proc_.cpu_usage = cpu_usage;
            proc_.parent = parent;
            proc_.status = status;
            if refresh_kind.memory() {
                proc_.virtual_memory = virtual_memory;
                proc_.memory = memory;
            }
            proc_.run_time = now.saturating_sub(proc_.start_time);

            if refresh_kind.disk_usage() {
                proc_.old_read_bytes = proc_.read_bytes;
                proc_.read_bytes = kproc.ki_rusage.ru_inblock as _;
                proc_.old_written_bytes = proc_.written_bytes;
                proc_.written_bytes = kproc.ki_rusage.ru_oublock as _;
            }

            return Ok(None);
        }
    }

    // This is a new process, we need to get more information!

    // For some reason, it can return completely invalid path like `p\u{5}`. So we need to use
    // procstat to get around this problem.
    // let cwd = get_sys_value_str(
    //     &[
    //         libc::CTL_KERN,
    //         libc::KERN_PROC,
    //         libc::KERN_PROC_CWD,
    //         kproc.ki_pid,
    //     ],
    //     &mut buffer,
    // )
    // .map(|s| s.into())
    // .unwrap_or_else(PathBuf::new);

    Ok(Some(Process {
        inner: ProcessInner {
            pid: Pid(kproc.ki_pid),
            parent,
            user_id: Uid(kproc.ki_ruid),
            effective_user_id: Uid(kproc.ki_uid),
            group_id: Gid(kproc.ki_rgid),
            effective_group_id: Gid(kproc.ki_svgid),
            start_time,
            run_time: now.saturating_sub(start_time),
            cpu_usage,
            virtual_memory,
            memory,
            // procstat_getfiles
            cwd: None,
            exe: None,
            // kvm_getargv isn't thread-safe so we get it in the main thread.
            name: OsString::new(),
            // kvm_getargv isn't thread-safe so we get it in the main thread.
            cmd: Vec::new(),
            // kvm_getargv isn't thread-safe so we get it in the main thread.
            root: None,
            // kvm_getenvv isn't thread-safe so we get it in the main thread.
            environ: Vec::new(),
            status,
            read_bytes: kproc.ki_rusage.ru_inblock as _,
            old_read_bytes: 0,
            written_bytes: kproc.ki_rusage.ru_oublock as _,
            old_written_bytes: 0,
            updated: false,
        },
    }))
}

pub(crate) unsafe fn get_exe(
    exe: &mut Option<PathBuf>,
    pid: crate::Pid,
    refresh_kind: ProcessRefreshKind,
) {
    if refresh_kind.exe().needs_update(|| exe.is_none()) {
        let mut buffer = [0; libc::PATH_MAX as usize + 1];

        *exe = get_sys_value_str(
            &[
                libc::CTL_KERN,
                libc::KERN_PROC,
                libc::KERN_PROC_PATHNAME,
                pid.0,
            ],
            &mut buffer,
        )
        .map(PathBuf::from);
    }
}
