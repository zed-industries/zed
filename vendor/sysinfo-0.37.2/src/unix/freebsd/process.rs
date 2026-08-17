// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DiskUsage, Gid, Pid, Process, ProcessRefreshKind, ProcessStatus, Signal, Uid};

use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;

use super::utils::{WrapMap, get_sys_value_str, get_sysctl_raw};

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
    accumulated_cpu_time: u64,
    exists: bool,
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

    pub(crate) fn accumulated_cpu_time(&self) -> u64 {
        self.accumulated_cpu_time
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

    pub(crate) fn wait(&self) -> Option<ExitStatus> {
        crate::unix::utils::wait_process(self.pid)
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

    pub(crate) fn switch_updated(&mut self) -> bool {
        std::mem::replace(&mut self.updated, false)
    }

    pub(crate) fn set_nonexistent(&mut self) {
        self.exists = false;
    }

    pub(crate) fn exists(&self) -> bool {
        self.exists
    }

    pub(crate) fn open_files(&self) -> Option<usize> {
        let mib = &[
            libc::CTL_KERN,
            libc::KERN_PROC,
            libc::KERN_PROC_FILEDESC,
            self.pid.0 as _,
        ];
        let mut len = 0;
        unsafe {
            if get_sysctl_raw(mib, std::ptr::null_mut(), &mut len).is_none() {
                sysinfo_debug!("Failed to query `open_files` info");
                return None;
            }
            let Some(data) = AllocatedPtr::<()>::new(len) else {
                sysinfo_debug!("Failed to allocate memory to get `open_files` info");
                return None;
            };
            // No clue why, it's done this way in `freebsd/lib/libutil/kinfo_getfile.c` so I suppose
            // they have a good reason...
            len = len * 4 / 3;
            if get_sysctl_raw(mib, data.0, &mut len).is_none() {
                sysinfo_debug!("Couldn't retrieve `open_files` data");
                return None;
            }
            let mut current = data.0;
            let end = current.byte_add(len);
            let mut count = 0;
            while current < end {
                let t = current as *mut libc::kinfo_file;
                if t.is_null() || (*t).kf_structsize == 0 {
                    break;
                }
                current = current.byte_add((*t).kf_structsize as _);
                count += 1;
            }
            Some(count)
        }
    }

    pub(crate) fn open_files_limit(&self) -> Option<usize> {
        crate::System::open_files_limit()
    }
}

struct AllocatedPtr<T>(*mut T);

impl<T> AllocatedPtr<T> {
    fn new(size: libc::size_t) -> Option<Self> {
        unsafe {
            let ptr = libc::malloc(size);
            if ptr.is_null() {
                None
            } else {
                Some(Self(ptr as _))
            }
        }
    }
}

impl<T> Drop for AllocatedPtr<T> {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                libc::free(self.0 as _);
            }
        }
    }
}

#[inline]
fn get_accumulated_cpu_time(kproc: &libc::kinfo_proc) -> u64 {
    // from FreeBSD source /bin/ps/print.c
    kproc.ki_runtime / 1_000
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
        Some((100 * kproc.ki_pctcpu) as f32 / fscale)
    } else {
        None
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

    if let Some(proc_) = unsafe { (*wrap.0.get()).get_mut(&Pid(kproc.ki_pid)) } {
        let proc_ = &mut proc_.inner;
        proc_.updated = true;
        // If the `start_time` we just got is different from the one stored, it means it's not the
        // same process.
        if proc_.start_time == start_time {
            if let Some(cpu_usage) = cpu_usage {
                proc_.cpu_usage = cpu_usage;
            }
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
            if refresh_kind.cpu() {
                proc_.accumulated_cpu_time = get_accumulated_cpu_time(kproc);
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
            cpu_usage: cpu_usage.unwrap_or(0.),
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
            accumulated_cpu_time: if refresh_kind.cpu() {
                get_accumulated_cpu_time(kproc)
            } else {
                0
            },
            updated: true,
            exists: true,
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

        unsafe {
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
}
