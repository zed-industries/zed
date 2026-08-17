// Take a look at the license at the top of the repository in the LICENSE file.

use std::cell::UnsafeCell;
use std::collections::{HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs::{self, DirEntry, File, read_dir};
use std::io::Read;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;
use std::str::{self, FromStr};
use std::sync::atomic::{AtomicUsize, Ordering};

use libc::{c_ulong, gid_t, uid_t};

use crate::sys::system::SystemInfo;
use crate::sys::utils::{
    PathHandler, PathPush, get_all_data_from_file, get_all_utf8_data, realpath,
};
use crate::{
    DiskUsage, Gid, Pid, Process, ProcessRefreshKind, ProcessStatus, ProcessesToUpdate, Signal,
    ThreadKind, Uid,
};

use crate::sys::system::remaining_files;

#[doc(hidden)]
impl From<char> for ProcessStatus {
    fn from(status: char) -> ProcessStatus {
        match status {
            'R' => ProcessStatus::Run,
            'S' => ProcessStatus::Sleep,
            'I' => ProcessStatus::Idle,
            'D' => ProcessStatus::UninterruptibleDiskSleep,
            'Z' => ProcessStatus::Zombie,
            'T' => ProcessStatus::Stop,
            't' => ProcessStatus::Tracing,
            'X' | 'x' => ProcessStatus::Dead,
            'K' => ProcessStatus::Wakekill,
            'W' => ProcessStatus::Waking,
            'P' => ProcessStatus::Parked,
            x => ProcessStatus::Unknown(x as u32),
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
            ProcessStatus::Tracing => "Tracing",
            ProcessStatus::Dead => "Dead",
            ProcessStatus::Wakekill => "Wakekill",
            ProcessStatus::Waking => "Waking",
            ProcessStatus::Parked => "Parked",
            ProcessStatus::UninterruptibleDiskSleep => "UninterruptibleDiskSleep",
            _ => "Unknown",
        })
    }
}

#[allow(dead_code)]
#[repr(usize)]
enum ProcIndex {
    Pid = 0,
    State,
    ParentPid,
    GroupId,
    SessionId,
    Tty,
    ForegroundProcessGroupId,
    Flags,
    MinorFaults,
    ChildrenMinorFaults,
    MajorFaults,
    ChildrenMajorFaults,
    UserTime,
    SystemTime,
    ChildrenUserTime,
    ChildrenKernelTime,
    Priority,
    Nice,
    NumberOfThreads,
    IntervalTimerSigalarm,
    StartTime,
    VirtualSize,
    ResidentSetSize,
    // More exist but we only use the listed ones. For more, take a look at `man proc`.
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
    utime: u64,
    stime: u64,
    old_utime: u64,
    old_stime: u64,
    start_time_without_boot_time: u64,
    start_time: u64,
    start_time_raw: u64,
    run_time: u64,
    pub(crate) updated: bool,
    cpu_usage: f32,
    user_id: Option<Uid>,
    effective_user_id: Option<Uid>,
    group_id: Option<Gid>,
    effective_group_id: Option<Gid>,
    pub(crate) status: ProcessStatus,
    pub(crate) tasks: Option<HashSet<Pid>>,
    stat_file: Option<FileCounter>,
    old_read_bytes: u64,
    old_written_bytes: u64,
    read_bytes: u64,
    written_bytes: u64,
    thread_kind: Option<ThreadKind>,
    proc_path: PathBuf,
    accumulated_cpu_time: u64,
    exists: bool,
}

impl ProcessInner {
    pub(crate) fn new(pid: Pid, proc_path: PathBuf) -> Self {
        Self {
            name: OsString::new(),
            pid,
            parent: None,
            cmd: Vec::new(),
            environ: Vec::new(),
            exe: None,
            cwd: None,
            root: None,
            memory: 0,
            virtual_memory: 0,
            cpu_usage: 0.,
            utime: 0,
            stime: 0,
            old_utime: 0,
            old_stime: 0,
            updated: true,
            start_time_without_boot_time: 0,
            start_time: 0,
            start_time_raw: 0,
            run_time: 0,
            user_id: None,
            effective_user_id: None,
            group_id: None,
            effective_group_id: None,
            status: ProcessStatus::Unknown(0),
            tasks: None,
            stat_file: None,
            old_read_bytes: 0,
            old_written_bytes: 0,
            read_bytes: 0,
            written_bytes: 0,
            thread_kind: None,
            proc_path,
            accumulated_cpu_time: 0,
            exists: true,
        }
    }

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
        self.user_id.as_ref()
    }

    pub(crate) fn effective_user_id(&self) -> Option<&Uid> {
        self.effective_user_id.as_ref()
    }

    pub(crate) fn group_id(&self) -> Option<Gid> {
        self.group_id
    }

    pub(crate) fn effective_group_id(&self) -> Option<Gid> {
        self.effective_group_id
    }

    pub(crate) fn wait(&self) -> Option<ExitStatus> {
        // If anything fails when trying to retrieve the start time, better to return `None`.
        let (data, _) = _get_stat_data_and_file(&self.proc_path).ok()?;
        let parts = parse_stat_file(&data)?;

        if start_time_raw(&parts) != self.start_time_raw {
            sysinfo_debug!("Seems to not be the same process anymore");
            return None;
        }

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

    pub(crate) fn thread_kind(&self) -> Option<ThreadKind> {
        self.thread_kind
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
        let open_files_dir = self.proc_path.as_path().join("fd");
        match fs::read_dir(&open_files_dir) {
            Ok(entries) => Some(entries.count() as _),
            Err(_error) => {
                sysinfo_debug!(
                    "Failed to get open files in `{}`: {_error:?}",
                    open_files_dir.display(),
                );
                None
            }
        }
    }

    pub(crate) fn open_files_limit(&self) -> Option<usize> {
        let limits_files = self.proc_path.as_path().join("limits");
        match fs::read_to_string(&limits_files) {
            Ok(content) => {
                for line in content.lines() {
                    if let Some(line) = line.strip_prefix("Max open files ")
                        && let Some(nb) = line.split_whitespace().find(|p| !p.is_empty())
                    {
                        return usize::from_str(nb).ok();
                    }
                }
                None
            }
            Err(_error) => {
                sysinfo_debug!(
                    "Failed to get limits in `{}`: {_error:?}",
                    limits_files.display()
                );
                None
            }
        }
    }
}

pub(crate) fn compute_cpu_usage(p: &mut ProcessInner, total_time: f32, max_value: f32) {
    // First time updating the values without reference, wait for a second cycle to update cpu_usage
    if p.old_utime == 0 && p.old_stime == 0 {
        return;
    }

    // We use `max_value` to ensure that the process CPU usage will never get bigger than:
    // `"number of CPUs" * 100.`
    p.cpu_usage = (p
        .utime
        .saturating_sub(p.old_utime)
        .saturating_add(p.stime.saturating_sub(p.old_stime)) as f32
        / total_time
        * 100.)
        .min(max_value);
}

pub(crate) fn set_time(p: &mut ProcessInner, utime: u64, stime: u64) {
    p.old_utime = p.utime;
    p.old_stime = p.stime;
    p.utime = utime;
    p.stime = stime;
}

pub(crate) fn update_process_disk_activity(p: &mut ProcessInner, path: &mut PathHandler) {
    let data = match get_all_utf8_data(path.replace_and_join("io"), 16_384) {
        Ok(d) => d,
        Err(_) => return,
    };
    let mut done = 0;
    for line in data.split('\n') {
        let mut parts = line.split(": ");
        match parts.next() {
            Some("read_bytes") => {
                p.old_read_bytes = p.read_bytes;
                p.read_bytes = parts
                    .next()
                    .and_then(|x| x.parse::<u64>().ok())
                    .unwrap_or(p.old_read_bytes);
            }
            Some("write_bytes") => {
                p.old_written_bytes = p.written_bytes;
                p.written_bytes = parts
                    .next()
                    .and_then(|x| x.parse::<u64>().ok())
                    .unwrap_or(p.old_written_bytes);
            }
            _ => continue,
        }
        done += 1;
        if done > 1 {
            // No need to continue the reading.
            break;
        }
    }
}

struct Wrap<'a, T>(UnsafeCell<&'a mut T>);

impl<'a, T> Wrap<'a, T> {
    fn get(&self) -> &'a mut T {
        unsafe { *(self.0.get()) }
    }
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<T> Send for Wrap<'_, T> {}
unsafe impl<T> Sync for Wrap<'_, T> {}

#[inline(always)]
fn start_time_raw(parts: &Parts<'_>) -> u64 {
    u64::from_str(parts.str_parts[ProcIndex::StartTime as usize]).unwrap_or(0)
}

#[inline(always)]
fn compute_start_time_without_boot_time(parts: &Parts<'_>, info: &SystemInfo) -> (u64, u64) {
    let raw = start_time_raw(parts);
    // To be noted that the start time is invalid here, it still needs to be converted into
    // "real" time.
    (raw, raw / info.clock_cycle)
}

fn _get_stat_data_and_file(path: &Path) -> Result<(Vec<u8>, File), ()> {
    let mut file = File::open(path.join("stat")).map_err(|_| ())?;
    let data = get_all_data_from_file(&mut file, 1024).map_err(|_| ())?;
    Ok((data, file))
}

fn _get_stat_data(path: &Path, stat_file: &mut Option<FileCounter>) -> Result<Vec<u8>, ()> {
    let (data, file) = _get_stat_data_and_file(path)?;
    *stat_file = FileCounter::new(file);
    Ok(data)
}

#[inline(always)]
fn get_status(p: &mut ProcessInner, part: &str) {
    p.status = part
        .chars()
        .next()
        .map(ProcessStatus::from)
        .unwrap_or_else(|| ProcessStatus::Unknown(0));
}

fn refresh_user_group_ids(
    p: &mut ProcessInner,
    path: &mut PathHandler,
    refresh_kind: ProcessRefreshKind,
) {
    if !refresh_kind.user().needs_update(|| p.user_id.is_none()) {
        return;
    }

    if let Some(((user_id, effective_user_id), (group_id, effective_group_id))) =
        get_uid_and_gid(path.replace_and_join("status"))
    {
        p.user_id = Some(Uid(user_id));
        p.effective_user_id = Some(Uid(effective_user_id));
        p.group_id = Some(Gid(group_id));
        p.effective_group_id = Some(Gid(effective_group_id));
    }
}

#[allow(clippy::too_many_arguments)]
fn update_proc_info(
    p: &mut ProcessInner,
    parent_pid: Option<Pid>,
    refresh_kind: ProcessRefreshKind,
    proc_path: &mut PathHandler,
    str_parts: &[&str],
    uptime: u64,
    info: &SystemInfo,
) {
    update_parent_pid(p, parent_pid, str_parts);

    get_status(p, str_parts[ProcIndex::State as usize]);
    refresh_user_group_ids(p, proc_path, refresh_kind);

    if refresh_kind.exe().needs_update(|| p.exe.is_none()) {
        // Do not use cmd[0] because it is not the same thing.
        // See https://github.com/GuillaumeGomez/sysinfo/issues/697.
        p.exe = realpath(proc_path.replace_and_join("exe"));
        // If the target executable file was modified or removed, linux appends ` (deleted)` at
        // the end. We need to remove it.
        // See https://github.com/GuillaumeGomez/sysinfo/issues/1585.
        let deleted = b" (deleted)";
        if let Some(exe) = &mut p.exe
            && let Some(file_name) = exe.file_name()
            && file_name.as_encoded_bytes().ends_with(deleted)
        {
            let mut file_name = file_name.as_encoded_bytes().to_vec();
            file_name.truncate(file_name.len() - deleted.len());
            unsafe {
                exe.set_file_name(OsString::from_encoded_bytes_unchecked(file_name));
            }
        }
    }

    if refresh_kind.cmd().needs_update(|| p.cmd.is_empty()) {
        p.cmd = copy_from_file(proc_path.replace_and_join("cmdline"));
    }
    if refresh_kind.environ().needs_update(|| p.environ.is_empty()) {
        p.environ = copy_from_file(proc_path.replace_and_join("environ"));
    }
    if refresh_kind.cwd().needs_update(|| p.cwd.is_none()) {
        p.cwd = realpath(proc_path.replace_and_join("cwd"));
    }
    if refresh_kind.root().needs_update(|| p.root.is_none()) {
        p.root = realpath(proc_path.replace_and_join("root"));
    }

    update_time_and_memory(proc_path, p, str_parts, uptime, info, refresh_kind);
    if refresh_kind.disk_usage() {
        update_process_disk_activity(p, proc_path);
    }
    // Needs to be after `update_time_and_memory`.
    if refresh_kind.cpu() {
        // The external values for CPU times are in "ticks", which are
        // scaled by "HZ", which is pegged externally at 100 ticks/second.
        p.accumulated_cpu_time =
            p.utime.saturating_add(p.stime).saturating_mul(1_000) / info.clock_cycle;
    }
    p.updated = true;
}

fn update_parent_pid(p: &mut ProcessInner, parent_pid: Option<Pid>, str_parts: &[&str]) {
    p.parent = match parent_pid {
        Some(parent_pid) if parent_pid.0 != 0 => Some(parent_pid),
        _ => match Pid::from_str(str_parts[ProcIndex::ParentPid as usize]) {
            Ok(p) if p.0 != 0 => Some(p),
            _ => None,
        },
    };
}

fn retrieve_all_new_process_info(
    pid: Pid,
    parent_pid: Option<Pid>,
    parts: &Parts<'_>,
    path: &Path,
    info: &SystemInfo,
    refresh_kind: ProcessRefreshKind,
    uptime: u64,
) -> Process {
    let mut p = ProcessInner::new(pid, path.to_owned());
    let mut proc_path = PathHandler::new(path);
    let name = parts.short_exe;

    let (start_time_raw, start_time_without_boot_time) =
        compute_start_time_without_boot_time(parts, info);
    p.start_time_raw = start_time_raw;
    p.start_time_without_boot_time = start_time_without_boot_time;
    p.start_time = p
        .start_time_without_boot_time
        .saturating_add(info.boot_time);

    p.name = OsStr::from_bytes(name).to_os_string();
    if c_ulong::from_str(parts.str_parts[ProcIndex::Flags as usize])
        .map(|flags| flags & libc::PF_KTHREAD as c_ulong != 0)
        .unwrap_or(false)
    {
        p.thread_kind = Some(ThreadKind::Kernel);
    } else if parent_pid.is_some() {
        p.thread_kind = Some(ThreadKind::Userland);
    }

    update_proc_info(
        &mut p,
        parent_pid,
        refresh_kind,
        &mut proc_path,
        &parts.str_parts,
        uptime,
        info,
    );

    Process { inner: p }
}

fn update_existing_process(
    proc: &mut Process,
    parent_pid: Option<Pid>,
    uptime: u64,
    info: &SystemInfo,
    refresh_kind: ProcessRefreshKind,
    tasks: Option<HashSet<Pid>>,
) -> Result<Option<Process>, ()> {
    let entry = &mut proc.inner;
    let data = if let Some(mut f) = entry.stat_file.take() {
        match get_all_data_from_file(&mut f, 1024) {
            Ok(data) => {
                // Everything went fine, we put back the file descriptor.
                entry.stat_file = Some(f);
                data
            }
            Err(_) => {
                // It's possible that the file descriptor is no longer valid in case the
                // original process was terminated and another one took its place.
                _get_stat_data(&entry.proc_path, &mut entry.stat_file)?
            }
        }
    } else {
        _get_stat_data(&entry.proc_path, &mut entry.stat_file)?
    };
    entry.tasks = tasks;

    let parts = parse_stat_file(&data).ok_or(())?;
    let start_time_raw = start_time_raw(&parts);

    // It's possible that a new process took this same PID when the "original one" terminated.
    // If the start time differs, then it means it's not the same process anymore and that we
    // need to get all its information, hence why we check it here.
    if start_time_raw == entry.start_time_raw {
        let mut proc_path = PathHandler::new(&entry.proc_path);

        update_proc_info(
            entry,
            parent_pid,
            refresh_kind,
            &mut proc_path,
            &parts.str_parts,
            uptime,
            info,
        );

        refresh_user_group_ids(entry, &mut proc_path, refresh_kind);
        return Ok(None);
    }
    // If we're here, it means that the PID still exists but it's a different process.
    let p = retrieve_all_new_process_info(
        entry.pid,
        parent_pid,
        &parts,
        &entry.proc_path,
        info,
        refresh_kind,
        uptime,
    );
    *proc = p;
    // Since this PID is already in the HashMap, no need to add it again.
    Ok(None)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn _get_process_data(
    path: &Path,
    proc_list: &mut HashMap<Pid, Process>,
    pid: Pid,
    parent_pid: Option<Pid>,
    uptime: u64,
    info: &SystemInfo,
    refresh_kind: ProcessRefreshKind,
    tasks: Option<HashSet<Pid>>,
) -> Result<Option<Process>, ()> {
    if let Some(ref mut entry) = proc_list.get_mut(&pid) {
        return update_existing_process(entry, parent_pid, uptime, info, refresh_kind, tasks);
    }
    let mut stat_file = None;
    let data = _get_stat_data(path, &mut stat_file)?;
    let parts = parse_stat_file(&data).ok_or(())?;

    let mut new_process =
        retrieve_all_new_process_info(pid, parent_pid, &parts, path, info, refresh_kind, uptime);
    new_process.inner.stat_file = stat_file;
    new_process.inner.tasks = tasks;
    Ok(Some(new_process))
}

fn old_get_memory(entry: &mut ProcessInner, str_parts: &[&str], info: &SystemInfo) {
    // rss
    entry.memory = u64::from_str(str_parts[ProcIndex::ResidentSetSize as usize])
        .unwrap_or(0)
        .saturating_mul(info.page_size_b);
    // vsz correspond to the Virtual memory size in bytes.
    // see: https://man7.org/linux/man-pages/man5/proc.5.html
    entry.virtual_memory = u64::from_str(str_parts[ProcIndex::VirtualSize as usize]).unwrap_or(0);
}

fn slice_to_nb(s: &[u8]) -> u64 {
    let mut nb: u64 = 0;

    for c in s {
        nb = nb * 10 + (c - b'0') as u64;
    }
    nb
}

fn get_memory(path: &Path, entry: &mut ProcessInner, info: &SystemInfo) -> bool {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_e) => {
            sysinfo_debug!(
                "Using old memory information (failed to open {:?}: {_e:?})",
                path
            );
            return false;
        }
    };
    let mut buf = Vec::new();
    if let Err(_e) = file.read_to_end(&mut buf) {
        sysinfo_debug!(
            "Using old memory information (failed to read {:?}: {_e:?})",
            path
        );
        return false;
    }
    let mut parts = buf.split(|c| *c == b' ');
    entry.virtual_memory = parts
        .next()
        .map(slice_to_nb)
        .unwrap_or(0)
        .saturating_mul(info.page_size_b);
    entry.memory = parts
        .next()
        .map(slice_to_nb)
        .unwrap_or(0)
        .saturating_mul(info.page_size_b);
    true
}

#[allow(clippy::too_many_arguments)]
fn update_time_and_memory(
    path: &mut PathHandler,
    entry: &mut ProcessInner,
    str_parts: &[&str],
    uptime: u64,
    info: &SystemInfo,
    refresh_kind: ProcessRefreshKind,
) {
    {
        #[allow(clippy::collapsible_if)]
        if refresh_kind.memory() {
            // Keeping this nested level for readability reasons.
            if !get_memory(path.replace_and_join("statm"), entry, info) {
                old_get_memory(entry, str_parts, info);
            }
        }
        set_time(
            entry,
            u64::from_str(str_parts[ProcIndex::UserTime as usize]).unwrap_or(0),
            u64::from_str(str_parts[ProcIndex::SystemTime as usize]).unwrap_or(0),
        );
        entry.run_time = uptime.saturating_sub(entry.start_time_without_boot_time);
    }
}

struct ProcAndTasks {
    pid: Pid,
    parent_pid: Option<Pid>,
    path: PathBuf,
    tasks: Option<HashSet<Pid>>,
}

#[cfg(feature = "multithread")]
#[inline]
pub(crate) fn iter<T>(val: T) -> rayon::iter::IterBridge<T>
where
    T: rayon::iter::ParallelBridge,
{
    val.par_bridge()
}

#[cfg(not(feature = "multithread"))]
#[inline]
pub(crate) fn iter<T>(val: T) -> T
where
    T: Iterator,
{
    val
}

/// We're forced to read the whole `/proc` folder because if a process died and another took its
/// place, we need to get the task parent (if it's a task).
pub(crate) fn refresh_procs(
    proc_list: &mut HashMap<Pid, Process>,
    proc_path: &Path,
    uptime: u64,
    info: &SystemInfo,
    processes_to_update: ProcessesToUpdate<'_>,
    refresh_kind: ProcessRefreshKind,
) -> usize {
    #[cfg(feature = "multithread")]
    use rayon::iter::ParallelIterator;

    let nb_updated = AtomicUsize::new(0);

    // This code goes through processes (listed in `/proc`) and through tasks (listed in
    // `/proc/[PID]/task`). However, the stored tasks information is supposed to be already present
    // in the PIDs listed from `/proc` so there will be no duplicates between PIDs and tasks PID.
    //
    // If a task is not listed in `/proc`, then we don't retrieve its information.
    //
    // So in short: since we update the `HashMap` itself by adding/removing entries outside of the
    // parallel iterator, we can safely use it inside the parallel iterator and update its entries
    // concurrently.
    let procs = {
        let pid_iter: Box<dyn Iterator<Item = (PathBuf, Pid)> + Send> = match processes_to_update {
            ProcessesToUpdate::All => match read_dir(proc_path) {
                Ok(proc_entries) => Box::new(proc_entries.filter_map(filter_pid_entries)),
                Err(_err) => {
                    sysinfo_debug!("Failed to read folder {proc_path:?}: {_err:?}");
                    return 0;
                }
            },
            ProcessesToUpdate::Some(pids) => Box::new(
                pids.iter()
                    .map(|pid| (proc_path.join(pid.to_string()), *pid)),
            ),
        };

        let proc_list = Wrap(UnsafeCell::new(proc_list));

        iter(pid_iter)
            .flat_map(|(path, pid)| {
                get_proc_and_tasks(path, pid, refresh_kind, processes_to_update)
            })
            .filter_map(|e| {
                let proc_list = proc_list.get();
                let new_process = _get_process_data(
                    e.path.as_path(),
                    proc_list,
                    e.pid,
                    e.parent_pid,
                    uptime,
                    info,
                    refresh_kind,
                    e.tasks,
                )
                .ok()?;
                nb_updated.fetch_add(1, Ordering::Relaxed);
                new_process
            })
            .collect::<Vec<_>>()
    };
    for proc_ in procs {
        proc_list.insert(proc_.pid(), proc_);
    }
    nb_updated.into_inner()
}

fn filter_pid_entries(entry: Result<DirEntry, std::io::Error>) -> Option<(PathBuf, Pid)> {
    if let Ok(entry) = entry
        && let Ok(file_type) = entry.file_type()
        && file_type.is_dir()
        && let Some(name) = entry.file_name().to_str()
        && let Ok(pid) = usize::from_str(name)
    {
        Some((entry.path(), Pid::from(pid)))
    } else {
        None
    }
}

fn get_proc_and_tasks(
    path: PathBuf,
    pid: Pid,
    refresh_kind: ProcessRefreshKind,
    processes_to_update: ProcessesToUpdate<'_>,
) -> Vec<ProcAndTasks> {
    let mut parent_pid = None;
    let (mut procs, mut tasks) = if refresh_kind.tasks() {
        let procs = get_proc_tasks(&path, pid);
        let tasks = procs.iter().map(|ProcAndTasks { pid, .. }| *pid).collect();

        (procs, Some(tasks))
    } else {
        (Vec::new(), None)
    };

    if processes_to_update != ProcessesToUpdate::All {
        // If the process' tgid doesn't match its pid, it is a task
        if let Some(tgid) = get_tgid(&path.join("status"))
            && tgid != pid
        {
            parent_pid = Some(tgid);
            tasks = None;
        }

        // Don't add the tasks to the list of processes to update
        procs.clear();
    }

    procs.push(ProcAndTasks {
        pid,
        parent_pid,
        path,
        tasks,
    });

    procs
}

fn get_proc_tasks(path: &Path, parent_pid: Pid) -> Vec<ProcAndTasks> {
    let task_path = path.join("task");

    read_dir(task_path)
        .ok()
        .map(|task_entries| {
            task_entries
                .filter_map(filter_pid_entries)
                // Needed because tasks have their own PID listed in the "task" folder.
                .filter(|(_, pid)| *pid != parent_pid)
                .map(|(path, pid)| ProcAndTasks {
                    pid,
                    path,
                    parent_pid: Some(parent_pid),
                    tasks: None,
                })
                .collect()
        })
        .unwrap_or_default()
}

fn split_content(mut data: &[u8]) -> Vec<OsString> {
    let mut out = Vec::with_capacity(10);
    while let Some(pos) = data.iter().position(|c| *c == 0) {
        let s = &data[..pos].trim_ascii();
        if !s.is_empty() {
            out.push(OsStr::from_bytes(s).to_os_string());
        }
        data = &data[pos + 1..];
    }
    if !data.is_empty() {
        let s = data.trim_ascii();
        if !s.is_empty() {
            out.push(OsStr::from_bytes(s).to_os_string());
        }
    }
    out
}

fn copy_from_file(entry: &Path) -> Vec<OsString> {
    match File::open(entry) {
        Ok(mut f) => {
            let mut data = Vec::with_capacity(16_384);

            if let Err(_e) = f.read_to_end(&mut data) {
                sysinfo_debug!("Failed to read file in `copy_from_file`: {:?}", _e);
                Vec::new()
            } else {
                split_content(&data)
            }
        }
        Err(_e) => {
            sysinfo_debug!("Failed to open file in `copy_from_file`: {:?}", _e);
            Vec::new()
        }
    }
}

// Fetch tuples of real and effective UID and GID.
fn get_uid_and_gid(file_path: &Path) -> Option<((uid_t, uid_t), (gid_t, gid_t))> {
    let status_data = get_all_utf8_data(file_path, 16_385).ok()?;

    // We're only interested in the lines starting with Uid: and Gid:
    // here. From these lines, we're looking at the first and second entries to get
    // the real u/gid.

    let f = |h: &str, n: &str| -> (Option<uid_t>, Option<uid_t>) {
        if h.starts_with(n) {
            let mut ids = h.split_whitespace();
            let real = ids.nth(1).unwrap_or("0").parse().ok();
            let effective = ids.next().unwrap_or("0").parse().ok();

            (real, effective)
        } else {
            (None, None)
        }
    };
    let mut uid = None;
    let mut effective_uid = None;
    let mut gid = None;
    let mut effective_gid = None;
    for line in status_data.lines() {
        if let (Some(real), Some(effective)) = f(line, "Uid:") {
            debug_assert!(uid.is_none() && effective_uid.is_none());
            uid = Some(real);
            effective_uid = Some(effective);
        } else if let (Some(real), Some(effective)) = f(line, "Gid:") {
            debug_assert!(gid.is_none() && effective_gid.is_none());
            gid = Some(real);
            effective_gid = Some(effective);
        } else {
            continue;
        }
        if uid.is_some() && gid.is_some() {
            break;
        }
    }
    match (uid, effective_uid, gid, effective_gid) {
        (Some(uid), Some(effective_uid), Some(gid), Some(effective_gid)) => {
            Some(((uid, effective_uid), (gid, effective_gid)))
        }
        _ => None,
    }
}

fn get_tgid(file_path: &Path) -> Option<Pid> {
    const TGID_KEY: &str = "Tgid:";
    let status_data = get_all_utf8_data(file_path, 16_385).ok()?;
    let tgid_line = status_data
        .lines()
        .find(|line| line.starts_with(TGID_KEY))?;
    tgid_line[TGID_KEY.len()..].trim_start().parse().ok()
}

struct Parts<'a> {
    str_parts: Vec<&'a str>,
    short_exe: &'a [u8],
}

fn parse_stat_file(data: &[u8]) -> Option<Parts<'_>> {
    // The stat file is "interesting" to parse, because spaces cannot
    // be used as delimiters. The second field stores the command name
    // surrounded by parentheses. Unfortunately, whitespace and
    // parentheses are legal parts of the command, so parsing has to
    // proceed like this: The first field is delimited by the first
    // whitespace, the second field is everything until the last ')'
    // in the entire string. All other fields are delimited by
    // whitespace.

    let mut str_parts = Vec::with_capacity(51);
    let mut data_it = data.splitn(2, |&b| b == b' ');
    str_parts.push(str::from_utf8(data_it.next()?).ok()?);
    let mut data_it = data_it.next()?.rsplitn(2, |&b| b == b')');
    let data = str::from_utf8(data_it.next()?).ok()?;
    let short_exe = data_it.next()?;
    str_parts.extend(data.split_whitespace());
    Some(Parts {
        str_parts,
        short_exe: short_exe.strip_prefix(b"(").unwrap_or(short_exe),
    })
}

/// Type used to correctly handle the `REMAINING_FILES` global.
struct FileCounter(File);

impl FileCounter {
    fn new(f: File) -> Option<Self> {
        let any_remaining =
            remaining_files().fetch_update(Ordering::SeqCst, Ordering::SeqCst, |remaining| {
                if remaining > 0 {
                    Some(remaining - 1)
                } else {
                    // All file descriptors we were allowed are being used.
                    None
                }
            });

        any_remaining.ok().map(|_| Self(f))
    }
}

impl std::ops::Deref for FileCounter {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for FileCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for FileCounter {
    fn drop(&mut self) {
        remaining_files().fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::split_content;
    use std::ffi::OsString;

    // This test ensures that all the parts of the data are split.
    #[test]
    fn test_copy_file() {
        assert_eq!(split_content(b"hello\0"), vec![OsString::from("hello")]);
        assert_eq!(split_content(b"hello"), vec![OsString::from("hello")]);
        assert_eq!(
            split_content(b"hello\0b"),
            vec![OsString::from("hello"), "b".into()]
        );
        assert_eq!(
            split_content(b"hello\0\0\0\0b"),
            vec![OsString::from("hello"), "b".into()]
        );
    }
}
