// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::{OsStr, OsString};
use std::mem::{self, MaybeUninit};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

use libc::{c_int, c_void, kill};

use crate::{DiskUsage, Gid, Pid, Process, ProcessRefreshKind, ProcessStatus, Signal, Uid};

use crate::sys::process::ThreadStatus;
use crate::sys::system::Wrap;
use crate::unix::utils::cstr_to_rust_with_size;

pub(crate) struct ProcessInner {
    pub(crate) name: OsString,
    pub(crate) cmd: Vec<OsString>,
    pub(crate) exe: Option<PathBuf>,
    pid: Pid,
    parent: Option<Pid>,
    pub(crate) environ: Vec<OsString>,
    cwd: Option<PathBuf>,
    pub(crate) root: Option<PathBuf>,
    pub(crate) memory: u64,
    pub(crate) virtual_memory: u64,
    old_utime: u64,
    old_stime: u64,
    start_time: u64,
    run_time: u64,
    pub(crate) updated: bool,
    cpu_usage: f32,
    user_id: Option<Uid>,
    effective_user_id: Option<Uid>,
    group_id: Option<Gid>,
    effective_group_id: Option<Gid>,
    pub(crate) process_status: ProcessStatus,
    /// Status of process (running, stopped, waiting, etc). `None` means `sysinfo` doesn't have
    /// enough rights to get this information.
    ///
    /// This is very likely this one that you want instead of `process_status`.
    pub(crate) status: Option<ThreadStatus>,
    pub(crate) old_read_bytes: u64,
    pub(crate) old_written_bytes: u64,
    pub(crate) read_bytes: u64,
    pub(crate) written_bytes: u64,
}

impl ProcessInner {
    pub(crate) fn new_empty(pid: Pid) -> Self {
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
            old_utime: 0,
            old_stime: 0,
            updated: true,
            start_time: 0,
            run_time: 0,
            user_id: None,
            effective_user_id: None,
            group_id: None,
            effective_group_id: None,
            process_status: ProcessStatus::Unknown(0),
            status: None,
            old_read_bytes: 0,
            old_written_bytes: 0,
            read_bytes: 0,
            written_bytes: 0,
        }
    }

    pub(crate) fn new(pid: Pid, parent: Option<Pid>, start_time: u64, run_time: u64) -> Self {
        Self {
            name: OsString::new(),
            pid,
            parent,
            cmd: Vec::new(),
            environ: Vec::new(),
            exe: None,
            cwd: None,
            root: None,
            memory: 0,
            virtual_memory: 0,
            cpu_usage: 0.,
            old_utime: 0,
            old_stime: 0,
            updated: true,
            start_time,
            run_time,
            user_id: None,
            effective_user_id: None,
            group_id: None,
            effective_group_id: None,
            process_status: ProcessStatus::Unknown(0),
            status: None,
            old_read_bytes: 0,
            old_written_bytes: 0,
            read_bytes: 0,
            written_bytes: 0,
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
        // If the status is `Run`, then it's very likely wrong so we instead
        // return a `ProcessStatus` converted from the `ThreadStatus`.
        if self.process_status == ProcessStatus::Run {
            if let Some(thread_status) = self.status {
                return ProcessStatus::from(thread_status);
            }
        }
        self.process_status
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
            read_bytes: self.read_bytes.saturating_sub(self.old_read_bytes),
            total_read_bytes: self.read_bytes,
            written_bytes: self.written_bytes.saturating_sub(self.old_written_bytes),
            total_written_bytes: self.written_bytes,
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
}

#[allow(deprecated)] // Because of libc::mach_absolute_time.
pub(crate) fn compute_cpu_usage(
    p: &mut ProcessInner,
    task_info: libc::proc_taskinfo,
    system_time: u64,
    user_time: u64,
    time_interval: Option<f64>,
) {
    if let Some(time_interval) = time_interval {
        let total_existing_time = p.old_stime.saturating_add(p.old_utime);
        let mut updated_cpu_usage = false;
        if time_interval > 0.000001 && total_existing_time > 0 {
            let total_current_time = task_info
                .pti_total_system
                .saturating_add(task_info.pti_total_user);

            let total_time_diff = total_current_time.saturating_sub(total_existing_time);
            if total_time_diff > 0 {
                p.cpu_usage = (total_time_diff as f64 / time_interval * 100.) as f32;
                updated_cpu_usage = true;
            }
        }
        if !updated_cpu_usage {
            p.cpu_usage = 0.;
        }
        p.old_stime = task_info.pti_total_system;
        p.old_utime = task_info.pti_total_user;
    } else {
        unsafe {
            // This is the "backup way" of CPU computation.
            let time = libc::mach_absolute_time();
            let task_time = user_time
                .saturating_add(system_time)
                .saturating_add(task_info.pti_total_user)
                .saturating_add(task_info.pti_total_system);

            let system_time_delta = if task_time < p.old_utime {
                task_time
            } else {
                task_time.saturating_sub(p.old_utime)
            };
            let time_delta = if time < p.old_stime {
                time
            } else {
                time.saturating_sub(p.old_stime)
            };
            p.old_utime = task_time;
            p.old_stime = time;
            p.cpu_usage = if time_delta == 0 {
                0f32
            } else {
                (system_time_delta as f64 * 100f64 / time_delta as f64) as f32
            };
        }
    }
}

unsafe fn get_task_info(pid: Pid) -> libc::proc_taskinfo {
    let mut task_info = mem::zeroed::<libc::proc_taskinfo>();
    // If it doesn't work, we just don't have memory information for this process
    // so it's "fine".
    libc::proc_pidinfo(
        pid.0,
        libc::PROC_PIDTASKINFO,
        0,
        &mut task_info as *mut libc::proc_taskinfo as *mut c_void,
        mem::size_of::<libc::proc_taskinfo>() as _,
    );
    task_info
}

#[inline]
fn check_if_pid_is_alive(pid: Pid, check_if_alive: bool) -> bool {
    // In case we are iterating all pids we got from `proc_listallpids`, then
    // there is no point checking if the process is alive since it was returned
    // from this function.
    if !check_if_alive {
        return true;
    }
    unsafe {
        if kill(pid.0, 0) == 0 {
            return true;
        }
        // `kill` failed but it might not be because the process is dead.
        let errno = crate::unix::libc_errno();
        // If errno is equal to ESCHR, it means the process is dead.
        !errno.is_null() && *errno != libc::ESRCH
    }
}

unsafe fn get_bsd_info(pid: Pid) -> Option<libc::proc_bsdinfo> {
    let mut info = mem::zeroed::<libc::proc_bsdinfo>();

    if libc::proc_pidinfo(
        pid.0,
        libc::PROC_PIDTBSDINFO,
        0,
        &mut info as *mut _ as *mut _,
        mem::size_of::<libc::proc_bsdinfo>() as _,
    ) != mem::size_of::<libc::proc_bsdinfo>() as _
    {
        None
    } else {
        Some(info)
    }
}

fn get_parent(info: &libc::proc_bsdinfo) -> Option<Pid> {
    match info.pbi_ppid as i32 {
        0 => None,
        p => Some(Pid(p)),
    }
}

unsafe fn create_new_process(
    pid: Pid,
    now: u64,
    refresh_kind: ProcessRefreshKind,
    info: Option<libc::proc_bsdinfo>,
) -> Result<Option<Process>, ()> {
    let info = match info {
        Some(info) => info,
        None => {
            let mut p = ProcessInner::new_empty(pid);
            if get_exe_and_name_backup(&mut p, refresh_kind) {
                get_cwd_root(&mut p, refresh_kind);
                return Ok(Some(Process { inner: p }));
            }
            // If we can't even have the name, no point in keeping it.
            return Err(());
        }
    };

    let parent = get_parent(&info);

    let start_time = info.pbi_start_tvsec;
    let run_time = now.saturating_sub(start_time);

    let mut p = ProcessInner::new(pid, parent, start_time, run_time);
    if !get_process_infos(&mut p, refresh_kind) && !get_exe_and_name_backup(&mut p, refresh_kind) {
        // If we can't even have the name, no point in keeping it.
        return Err(());
    }
    get_cwd_root(&mut p, refresh_kind);

    if refresh_kind.memory() {
        let task_info = get_task_info(pid);
        p.memory = task_info.pti_resident_size;
        p.virtual_memory = task_info.pti_virtual_size;
    }

    p.user_id = Some(Uid(info.pbi_ruid));
    p.effective_user_id = Some(Uid(info.pbi_uid));
    p.group_id = Some(Gid(info.pbi_rgid));
    p.effective_group_id = Some(Gid(info.pbi_gid));
    p.process_status = ProcessStatus::from(info.pbi_status);
    if refresh_kind.disk_usage() {
        update_proc_disk_activity(&mut p);
    }
    Ok(Some(Process { inner: p }))
}

/// Less efficient way to retrieve `exe` and `name`.
unsafe fn get_exe_and_name_backup(
    process: &mut ProcessInner,
    refresh_kind: ProcessRefreshKind,
) -> bool {
    let exe_needs_update = refresh_kind.exe().needs_update(|| process.exe.is_none());
    if !process.name.is_empty() && !exe_needs_update {
        return false;
    }
    let mut buffer: Vec<u8> = Vec::with_capacity(libc::PROC_PIDPATHINFO_MAXSIZE as _);
    match libc::proc_pidpath(
        process.pid.0,
        buffer.as_mut_ptr() as *mut _,
        libc::PROC_PIDPATHINFO_MAXSIZE as _,
    ) {
        x if x > 0 => {
            buffer.set_len(x as _);
            let tmp = OsString::from_vec(buffer);
            let exe = PathBuf::from(tmp);
            if process.name.is_empty() {
                exe.file_name()
                    .unwrap_or_default()
                    .clone_into(&mut process.name);
            }
            if exe_needs_update {
                process.exe = Some(exe);
            }
            true
        }
        _ => false,
    }
}

unsafe fn convert_node_path_info(node: &libc::vnode_info_path) -> Option<PathBuf> {
    if node.vip_vi.vi_stat.vst_dev == 0 {
        return None;
    }
    cstr_to_rust_with_size(
        node.vip_path.as_ptr() as _,
        Some(node.vip_path.len() * node.vip_path[0].len()),
    )
    .map(PathBuf::from)
}

unsafe fn get_cwd_root(process: &mut ProcessInner, refresh_kind: ProcessRefreshKind) {
    let cwd_needs_update = refresh_kind.cwd().needs_update(|| process.cwd.is_none());
    let root_needs_update = refresh_kind.root().needs_update(|| process.root.is_none());
    if !cwd_needs_update && !root_needs_update {
        return;
    }
    let mut vnodepathinfo = mem::zeroed::<libc::proc_vnodepathinfo>();
    let result = libc::proc_pidinfo(
        process.pid.0,
        libc::PROC_PIDVNODEPATHINFO,
        0,
        &mut vnodepathinfo as *mut _ as *mut _,
        mem::size_of::<libc::proc_vnodepathinfo>() as _,
    );
    if result < 1 {
        sysinfo_debug!("Failed to retrieve cwd and root for {}", process.pid.0);
        return;
    }
    if cwd_needs_update {
        process.cwd = convert_node_path_info(&vnodepathinfo.pvi_cdir);
    }
    if root_needs_update {
        process.root = convert_node_path_info(&vnodepathinfo.pvi_rdir);
    }
}

unsafe fn get_process_infos(process: &mut ProcessInner, refresh_kind: ProcessRefreshKind) -> bool {
    /*
     * /---------------\ 0x00000000
     * | ::::::::::::: |
     * |---------------| <-- Beginning of data returned by sysctl() is here.
     * | argc          |
     * |---------------|
     * | exec_path     |
     * |---------------|
     * | 0             |
     * |---------------|
     * | arg[0]        |
     * |---------------|
     * | 0             |
     * |---------------|
     * | arg[n]        |
     * |---------------|
     * | 0             |
     * |---------------|
     * | env[0]        |
     * |---------------|
     * | 0             |
     * |---------------|
     * | env[n]        |
     * |---------------|
     * | ::::::::::::: |
     * |---------------| <-- Top of stack.
     * :               :
     * :               :
     * \---------------/ 0xffffffff
     */
    let mut mib: [libc::c_int; 3] = [libc::CTL_KERN, libc::KERN_PROCARGS2, process.pid.0 as _];
    let mut arg_max = 0;
    // First we retrieve the size we will need for our data (in `arg_max`).
    if libc::sysctl(
        mib.as_mut_ptr(),
        mib.len() as _,
        std::ptr::null_mut(),
        &mut arg_max,
        std::ptr::null_mut(),
        0,
    ) == -1
    {
        sysinfo_debug!(
            "couldn't get arguments and environment size for PID {}",
            process.pid.0
        );
        return false; // not enough rights I assume?
    }

    let mut proc_args: Vec<u8> = Vec::with_capacity(arg_max as _);
    if libc::sysctl(
        mib.as_mut_ptr(),
        mib.len() as _,
        proc_args.as_mut_slice().as_mut_ptr() as *mut _,
        &mut arg_max,
        std::ptr::null_mut(),
        0,
    ) == -1
    {
        sysinfo_debug!(
            "couldn't get arguments and environment for PID {}",
            process.pid.0
        );
        return false; // What changed since the previous call? Dark magic!
    }

    proc_args.set_len(arg_max);

    if proc_args.is_empty() {
        return false;
    }
    // We copy the number of arguments (`argc`) to `n_args`.
    let mut n_args: c_int = 0;
    libc::memcpy(
        &mut n_args as *mut _ as *mut _,
        proc_args.as_slice().as_ptr() as *const _,
        mem::size_of::<c_int>(),
    );

    // We skip `argc`.
    let proc_args = &proc_args[mem::size_of::<c_int>()..];

    let (exe, proc_args) = get_exe(proc_args);
    if process.name.is_empty() {
        exe.file_name()
            .unwrap_or_default()
            .clone_into(&mut process.name);
    }

    if refresh_kind.exe().needs_update(|| process.exe.is_none()) {
        process.exe = Some(exe.to_owned());
    }

    let environ_needs_update = refresh_kind
        .environ()
        .needs_update(|| process.environ.is_empty());
    let cmd_needs_update = refresh_kind.cmd().needs_update(|| process.cmd.is_empty());
    if !environ_needs_update && !cmd_needs_update {
        // Nothing else to be done!
        return true;
    }
    let proc_args = get_arguments(&mut process.cmd, proc_args, n_args, cmd_needs_update);
    if environ_needs_update {
        get_environ(&mut process.environ, proc_args);
    }
    true
}

fn get_exe(data: &[u8]) -> (&Path, &[u8]) {
    let pos = data.iter().position(|c| *c == 0).unwrap_or(data.len());
    let (exe, proc_args) = data.split_at(pos);
    (Path::new(OsStr::from_bytes(exe)), proc_args)
}

fn get_arguments<'a>(
    cmd: &mut Vec<OsString>,
    mut data: &'a [u8],
    mut n_args: c_int,
    refresh_cmd: bool,
) -> &'a [u8] {
    if refresh_cmd {
        cmd.clear();
    }

    if n_args < 1 {
        return data;
    }
    while data.first() == Some(&0) {
        data = &data[1..];
    }

    while n_args > 0 && !data.is_empty() {
        let pos = data.iter().position(|c| *c == 0).unwrap_or(data.len());
        let arg = &data[..pos];
        if !arg.is_empty() && refresh_cmd {
            cmd.push(OsStr::from_bytes(arg).to_os_string());
        }
        data = &data[pos..];
        while data.first() == Some(&0) {
            data = &data[1..];
        }
        n_args -= 1;
    }
    data
}

fn get_environ(environ: &mut Vec<OsString>, mut data: &[u8]) {
    environ.clear();

    while data.first() == Some(&0) {
        data = &data[1..];
    }

    while !data.is_empty() {
        let pos = data.iter().position(|c| *c == 0).unwrap_or(data.len());
        let arg = &data[..pos];
        if arg.is_empty() {
            return;
        }
        environ.push(OsStr::from_bytes(arg).to_os_string());
        data = &data[pos..];
        while data.first() == Some(&0) {
            data = &data[1..];
        }
    }
}

pub(crate) fn update_process(
    wrap: &Wrap,
    pid: Pid,
    time_interval: Option<f64>,
    now: u64,
    refresh_kind: ProcessRefreshKind,
    check_if_alive: bool,
) -> Result<Option<Process>, ()> {
    unsafe {
        if let Some(ref mut p) = (*wrap.0.get()).get_mut(&pid) {
            let p = &mut p.inner;

            if let Some(info) = get_bsd_info(pid) {
                if info.pbi_start_tvsec != p.start_time {
                    // We don't it to be removed, just replaced.
                    p.updated = true;
                    // The owner of this PID changed.
                    return create_new_process(pid, now, refresh_kind, Some(info));
                }
                let parent = get_parent(&info);
                // Update the parent if it changed.
                if p.parent != parent {
                    p.parent = parent;
                }
            }

            if !get_process_infos(p, refresh_kind) {
                get_exe_and_name_backup(p, refresh_kind);
            }
            get_cwd_root(p, refresh_kind);

            if refresh_kind.disk_usage() {
                update_proc_disk_activity(p);
            }

            let mut thread_info = mem::zeroed::<libc::proc_threadinfo>();
            let (user_time, system_time, thread_status) = if libc::proc_pidinfo(
                pid.0,
                libc::PROC_PIDTHREADINFO,
                0,
                &mut thread_info as *mut libc::proc_threadinfo as *mut c_void,
                mem::size_of::<libc::proc_threadinfo>() as _,
            ) != 0
            {
                (
                    thread_info.pth_user_time,
                    thread_info.pth_system_time,
                    Some(ThreadStatus::from(thread_info.pth_run_state)),
                )
            } else {
                // It very likely means that the process is dead...
                if check_if_pid_is_alive(pid, check_if_alive) {
                    (0, 0, Some(ThreadStatus::Running))
                } else {
                    return Err(());
                }
            };
            p.status = thread_status;
            p.run_time = now.saturating_sub(p.start_time);

            if refresh_kind.cpu() || refresh_kind.memory() {
                let task_info = get_task_info(pid);

                if refresh_kind.cpu() {
                    compute_cpu_usage(p, task_info, system_time, user_time, time_interval);
                }
                if refresh_kind.memory() {
                    p.memory = task_info.pti_resident_size;
                    p.virtual_memory = task_info.pti_virtual_size;
                }
            }
            p.updated = true;
            Ok(None)
        } else {
            create_new_process(pid, now, refresh_kind, get_bsd_info(pid))
        }
    }
}

fn update_proc_disk_activity(p: &mut ProcessInner) {
    p.old_read_bytes = p.read_bytes;
    p.old_written_bytes = p.written_bytes;

    let mut pidrusage = MaybeUninit::<libc::rusage_info_v2>::uninit();

    unsafe {
        let retval = libc::proc_pid_rusage(
            p.pid().0 as _,
            libc::RUSAGE_INFO_V2,
            pidrusage.as_mut_ptr() as _,
        );

        if retval < 0 {
            sysinfo_debug!("proc_pid_rusage failed: {:?}", retval);
        } else {
            let pidrusage = pidrusage.assume_init();
            p.read_bytes = pidrusage.ri_diskio_bytesread;
            p.written_bytes = pidrusage.ri_diskio_byteswritten;
        }
    }
}

#[allow(clippy::uninit_vec)]
pub(crate) fn get_proc_list() -> Option<Vec<Pid>> {
    unsafe {
        let count = libc::proc_listallpids(::std::ptr::null_mut(), 0);
        if count < 1 {
            return None;
        }
        let mut pids: Vec<Pid> = Vec::with_capacity(count as usize);
        pids.set_len(count as usize);
        let count = count * mem::size_of::<Pid>() as i32;
        let x = libc::proc_listallpids(pids.as_mut_ptr() as *mut c_void, count);

        if x < 1 || x as usize >= pids.len() {
            None
        } else {
            pids.set_len(x as usize);
            Some(pids)
        }
    }
}
