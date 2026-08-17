// Take a look at the license at the top of the repository in the LICENSE file.

use std::cell::UnsafeCell;
use std::collections::{HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs::{self, DirEntry, File};
use std::io::Read;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::str::{self, FromStr};
use std::sync::atomic::{AtomicUsize, Ordering};

use libc::{c_ulong, gid_t, kill, uid_t};

use crate::sys::system::SystemInfo;
use crate::sys::utils::{
    get_all_data_from_file, get_all_utf8_data, realpath, PathHandler, PathPush,
};
use crate::{
    DiskUsage, Gid, Pid, Process, ProcessesToUpdate, ProcessRefreshKind, ProcessStatus, Signal, ThreadKind, Uid,
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
        }
    }

    pub(crate) fn kill_with(&self, signal: Signal) -> Option<bool> {
        let c_signal = crate::sys::system::convert_signal(signal)?;
        unsafe { Some(kill(self.pid.0, c_signal) == 0) }
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

    pub(crate) fn thread_kind(&self) -> Option<ThreadKind> {
        self.thread_kind
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

pub(crate) fn unset_updated(p: &mut ProcessInner) {
    p.updated = false;
}

pub(crate) fn set_time(p: &mut ProcessInner, utime: u64, stime: u64) {
    p.old_utime = p.utime;
    p.old_stime = p.stime;
    p.utime = utime;
    p.stime = stime;
    p.updated = true;
}

pub(crate) fn update_process_disk_activity(p: &mut ProcessInner, path: &mut PathHandler) {
    let data = match get_all_utf8_data(path.join("io"), 16_384) {
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
unsafe impl<'a, T> Send for Wrap<'a, T> {}
unsafe impl<'a, T> Sync for Wrap<'a, T> {}

#[inline(always)]
fn compute_start_time_without_boot_time(parts: &Parts<'_>, info: &SystemInfo) -> u64 {
    // To be noted that the start time is invalid here, it still needs to be converted into
    // "real" time.
    u64::from_str(parts.str_parts[ProcIndex::StartTime as usize]).unwrap_or(0) / info.clock_cycle
}

fn _get_stat_data(path: &Path, stat_file: &mut Option<FileCounter>) -> Result<Vec<u8>, ()> {
    let mut file = File::open(path.join("stat")).map_err(|_| ())?;
    let data = get_all_data_from_file(&mut file, 1024).map_err(|_| ())?;
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
        get_uid_and_gid(path.join("status"))
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
        p.exe = realpath(proc_path.join("exe"));
    }

    if refresh_kind.cmd().needs_update(|| p.cmd.is_empty()) {
        p.cmd = copy_from_file(proc_path.join("cmdline"));
    }
    if refresh_kind.environ().needs_update(|| p.environ.is_empty()) {
        p.environ = copy_from_file(proc_path.join("environ"));
    }
    if refresh_kind.cwd().needs_update(|| p.cwd.is_none()) {
        p.cwd = realpath(proc_path.join("cwd"));
    }
    if refresh_kind.root().needs_update(|| p.root.is_none()) {
        p.root = realpath(proc_path.join("root"));
    }

    update_time_and_memory(proc_path, p, str_parts, uptime, info, refresh_kind);
    if refresh_kind.disk_usage() {
        update_process_disk_activity(p, proc_path);
    }
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

    p.start_time_without_boot_time = compute_start_time_without_boot_time(parts, info);
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

pub(crate) fn _get_process_data(
    path: &Path,
    proc_list: &mut HashMap<Pid, Process>,
    pid: Pid,
    parent_pid: Option<Pid>,
    uptime: u64,
    info: &SystemInfo,
    refresh_kind: ProcessRefreshKind,
) -> Result<(Option<Process>, Pid), ()> {
    let data;
    let parts = if let Some(ref mut entry) = proc_list.get_mut(&pid) {
        let entry = &mut entry.inner;
        data = if let Some(mut f) = entry.stat_file.take() {
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
            _get_stat_data(path, &mut entry.stat_file)?
        };
        let parts = parse_stat_file(&data).ok_or(())?;
        let start_time_without_boot_time = compute_start_time_without_boot_time(&parts, info);

        // It's possible that a new process took this same PID when the "original one" terminated.
        // If the start time differs, then it means it's not the same process anymore and that we
        // need to get all its information, hence why we check it here.
        if start_time_without_boot_time == entry.start_time_without_boot_time {
            let mut proc_path = PathHandler::new(path);

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
            return Ok((None, pid));
        }
        parts
    } else {
        let mut stat_file = None;
        let data = _get_stat_data(path, &mut stat_file)?;
        let parts = parse_stat_file(&data).ok_or(())?;

        let mut p = retrieve_all_new_process_info(
            pid,
            parent_pid,
            &parts,
            path,
            info,
            refresh_kind,
            uptime,
        );
        p.inner.stat_file = stat_file;
        return Ok((Some(p), pid));
    };

    // If we're here, it means that the PID still exists but it's a different process.
    let p =
        retrieve_all_new_process_info(pid, parent_pid, &parts, path, info, refresh_kind, uptime);
    match proc_list.get_mut(&pid) {
        Some(ref mut entry) => **entry = p,
        // If it ever enters this case, it means that the process was removed from the HashMap
        // in-between with the usage of dark magic.
        None => unreachable!(),
    }
    // Since this PID is already in the HashMap, no need to add it again.
    Ok((None, pid))
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
            if !get_memory(path.join("statm"), entry, info) {
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

fn get_all_pid_entries(
    parent: Option<&OsStr>,
    parent_pid: Option<Pid>,
    entry: DirEntry,
    data: &mut Vec<ProcAndTasks>,
) -> Option<Pid> {
    let Ok(file_type) = entry.file_type() else {
        return None;
    };
    if !file_type.is_dir() {
        return None;
    }

    let entry = entry.path();
    let name = entry.file_name();

    if name == parent {
        // Needed because tasks have their own PID listed in the "task" folder.
        return None;
    }
    let name = name?;
    let pid = Pid::from(usize::from_str(name.to_str()?).ok()?);

    let tasks_dir = Path::join(&entry, "task");

    let tasks = if let Ok(entries) = fs::read_dir(tasks_dir) {
        let mut tasks = HashSet::new();
        for task in entries
            .into_iter()
            .filter_map(|entry| get_all_pid_entries(Some(name), Some(pid), entry.ok()?, data))
        {
            tasks.insert(task);
        }
        Some(tasks)
    } else {
        None
    };

    data.push(ProcAndTasks {
        pid,
        parent_pid,
        path: entry,
        tasks,
    });
    Some(pid)
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

pub(crate) fn refresh_procs(
    proc_list: &mut HashMap<Pid, Process>,
    path: &Path,
    uptime: u64,
    info: &SystemInfo,
    processes_to_update: ProcessesToUpdate<'_>,
    refresh_kind: ProcessRefreshKind,
) -> usize {
    #[cfg(feature = "multithread")]
    use rayon::iter::ParallelIterator;

    #[inline(always)]
    fn real_filter(e: &ProcAndTasks, filter: &[Pid]) -> bool {
        filter.contains(&e.pid)
    }

    #[inline(always)]
    fn empty_filter(_e: &ProcAndTasks, _filter: &[Pid]) -> bool {
        true
    }

    #[allow(clippy::type_complexity)]
    let (filter, filter_callback): (
        &[Pid],
        &(dyn Fn(&ProcAndTasks, &[Pid]) -> bool + Sync + Send),
    ) = match processes_to_update {
        ProcessesToUpdate::All => (&[], &empty_filter),
        ProcessesToUpdate::Some(pids) => {
            if pids.is_empty() {
                return 0;
            }
            (pids, &real_filter)
        }
    };

    let nb_updated = AtomicUsize::new(0);

    // FIXME: To prevent retrieving a task more than once (it can be listed in `/proc/[PID]/task`
    // subfolder and directly in `/proc` at the same time), might be interesting to use a `HashSet`.
    let procs = {
        let d = match fs::read_dir(path) {
            Ok(d) => d,
            Err(_err) => {
                sysinfo_debug!("Failed to read folder {path:?}: {_err:?}");
                return 0
            },
        };
        let proc_list = Wrap(UnsafeCell::new(proc_list));

        iter(d)
            .map(|entry| {
                let Ok(entry) = entry else { return Vec::new() };
                let mut entries = Vec::new();
                get_all_pid_entries(None, None, entry, &mut entries);
                entries
            })
            .flatten()
            .filter(|e| filter_callback(e, filter))
            .filter_map(|e| {
                let (mut p, _) = _get_process_data(
                    e.path.as_path(),
                    proc_list.get(),
                    e.pid,
                    e.parent_pid,
                    uptime,
                    info,
                    refresh_kind,
                )
                .ok()?;
                nb_updated.fetch_add(1, Ordering::Relaxed);
                if let Some(ref mut p) = p {
                    p.inner.tasks = e.tasks;
                }
                p
            })
            .collect::<Vec<_>>()
    };
    for proc_ in procs {
        proc_list.insert(proc_.pid(), proc_);
    }
    nb_updated.into_inner()
}

// FIXME: To be removed once MSRV for this crate is 1.80 nd use the `trim_ascii()` method instead.
fn trim_ascii(mut bytes: &[u8]) -> &[u8] {
    // Code from Rust code library.
    while let [rest @ .., last] = bytes {
        if last.is_ascii_whitespace() {
            bytes = rest;
        } else {
            break;
        }
    }
    while let [first, rest @ ..] = bytes {
        if first.is_ascii_whitespace() {
            bytes = rest;
        } else {
            break;
        }
    }
    bytes
}

fn copy_from_file(entry: &Path) -> Vec<OsString> {
    match File::open(entry) {
        Ok(mut f) => {
            let mut data = Vec::with_capacity(16_384);

            if let Err(_e) = f.read_to_end(&mut data) {
                sysinfo_debug!("Failed to read file in `copy_from_file`: {:?}", _e);
                Vec::new()
            } else {
                let mut out = Vec::with_capacity(10);
                let mut data = data.as_slice();
                while let Some(pos) = data.iter().position(|c| *c == 0) {
                    let s = trim_ascii(&data[..pos]);
                    if !s.is_empty() {
                        out.push(OsStr::from_bytes(s).to_os_string());
                    }
                    data = &data[pos + 1..];
                }
                out
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
