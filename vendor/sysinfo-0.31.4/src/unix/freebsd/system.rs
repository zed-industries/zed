// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    Cpu, CpuRefreshKind, LoadAvg, MemoryRefreshKind, Pid, Process, ProcessesToUpdate, ProcessInner, ProcessRefreshKind,
};

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, SystemTime};

use crate::sys::cpu::{physical_core_count, CpusWrapper};
use crate::sys::process::get_exe;
use crate::sys::utils::{
    self, boot_time, c_buf_to_os_string, c_buf_to_utf8_string, from_cstr_array, get_sys_value,
    get_sys_value_by_name, init_mib,
};

use libc::c_int;

declare_signals! {
    c_int,
    Signal::Hangup => libc::SIGHUP,
    Signal::Interrupt => libc::SIGINT,
    Signal::Quit => libc::SIGQUIT,
    Signal::Illegal => libc::SIGILL,
    Signal::Trap => libc::SIGTRAP,
    Signal::Abort => libc::SIGABRT,
    Signal::IOT => libc::SIGIOT,
    Signal::Bus => libc::SIGBUS,
    Signal::FloatingPointException => libc::SIGFPE,
    Signal::Kill => libc::SIGKILL,
    Signal::User1 => libc::SIGUSR1,
    Signal::Segv => libc::SIGSEGV,
    Signal::User2 => libc::SIGUSR2,
    Signal::Pipe => libc::SIGPIPE,
    Signal::Alarm => libc::SIGALRM,
    Signal::Term => libc::SIGTERM,
    Signal::Child => libc::SIGCHLD,
    Signal::Continue => libc::SIGCONT,
    Signal::Stop => libc::SIGSTOP,
    Signal::TSTP => libc::SIGTSTP,
    Signal::TTIN => libc::SIGTTIN,
    Signal::TTOU => libc::SIGTTOU,
    Signal::Urgent => libc::SIGURG,
    Signal::XCPU => libc::SIGXCPU,
    Signal::XFSZ => libc::SIGXFSZ,
    Signal::VirtualAlarm => libc::SIGVTALRM,
    Signal::Profiling => libc::SIGPROF,
    Signal::Winch => libc::SIGWINCH,
    Signal::IO => libc::SIGIO,
    Signal::Sys => libc::SIGSYS,
    _ => None,
}

#[doc = include_str!("../../../md_doc/supported_signals.md")]
pub const SUPPORTED_SIGNALS: &[crate::Signal] = supported_signals();
#[doc = include_str!("../../../md_doc/minimum_cpu_update_interval.md")]
pub const MINIMUM_CPU_UPDATE_INTERVAL: Duration = Duration::from_millis(100);

pub(crate) struct SystemInner {
    process_list: HashMap<Pid, Process>,
    mem_total: u64,
    mem_free: u64,
    mem_used: u64,
    swap_total: u64,
    swap_used: u64,
    system_info: SystemInfo,
    cpus: CpusWrapper,
}

impl SystemInner {
    pub(crate) fn new() -> Self {
        Self {
            process_list: HashMap::with_capacity(200),
            mem_total: 0,
            mem_free: 0,
            mem_used: 0,
            swap_total: 0,
            swap_used: 0,
            system_info: SystemInfo::new(),
            cpus: CpusWrapper::new(),
        }
    }

    pub(crate) fn refresh_memory_specifics(&mut self, refresh_kind: MemoryRefreshKind) {
        if refresh_kind.ram() {
            if self.mem_total == 0 {
                self.mem_total = self.system_info.get_total_memory();
            }
            self.mem_used = self.system_info.get_used_memory();
            self.mem_free = self.system_info.get_free_memory();
        }
        if refresh_kind.swap() {
            let (swap_used, swap_total) = self.system_info.get_swap_info();
            self.swap_total = swap_total;
            self.swap_used = swap_used;
        }
    }

    pub(crate) fn cgroup_limits(&self) -> Option<crate::CGroupLimits> {
        None
    }

    pub(crate) fn refresh_cpu_specifics(&mut self, refresh_kind: CpuRefreshKind) {
        self.cpus.refresh(refresh_kind)
    }

    pub(crate) fn refresh_cpu_list(&mut self, refresh_kind: CpuRefreshKind) {
        self.cpus = CpusWrapper::new();
        self.cpus.refresh(refresh_kind);
    }

    pub(crate) fn refresh_processes_specifics(
        &mut self,
        processes_to_update: ProcessesToUpdate<'_>,
        refresh_kind: ProcessRefreshKind,
    ) -> usize {
        unsafe { self.refresh_procs(processes_to_update, refresh_kind) }
    }

    // COMMON PART
    //
    // Need to be moved into a "common" file to avoid duplication.

    pub(crate) fn processes(&self) -> &HashMap<Pid, Process> {
        &self.process_list
    }

    pub(crate) fn process(&self, pid: Pid) -> Option<&Process> {
        self.process_list.get(&pid)
    }

    pub(crate) fn global_cpu_usage(&self) -> f32 {
        self.cpus.global_cpu_usage
    }

    pub(crate) fn cpus(&self) -> &[Cpu] {
        &self.cpus.cpus
    }

    pub(crate) fn physical_core_count(&self) -> Option<usize> {
        physical_core_count()
    }

    pub(crate) fn total_memory(&self) -> u64 {
        self.mem_total
    }

    pub(crate) fn free_memory(&self) -> u64 {
        self.mem_free
    }

    pub(crate) fn available_memory(&self) -> u64 {
        self.mem_free
    }

    pub(crate) fn used_memory(&self) -> u64 {
        self.mem_used
    }

    pub(crate) fn total_swap(&self) -> u64 {
        self.swap_total
    }

    pub(crate) fn free_swap(&self) -> u64 {
        self.swap_total - self.swap_used
    }

    // TODO: need to be checked
    pub(crate) fn used_swap(&self) -> u64 {
        self.swap_used
    }

    pub(crate) fn uptime() -> u64 {
        unsafe {
            let csec = libc::time(std::ptr::null_mut());

            libc::difftime(csec, Self::boot_time() as _) as u64
        }
    }

    pub(crate) fn boot_time() -> u64 {
        boot_time()
    }

    pub(crate) fn load_average() -> LoadAvg {
        let mut loads = vec![0f64; 3];
        unsafe {
            libc::getloadavg(loads.as_mut_ptr(), 3);
            LoadAvg {
                one: loads[0],
                five: loads[1],
                fifteen: loads[2],
            }
        }
    }

    pub(crate) fn name() -> Option<String> {
        let mut os_type: [c_int; 2] = [0; 2];
        unsafe {
            init_mib(b"kern.ostype\0", &mut os_type);
            get_system_info(&os_type, Some("FreeBSD"))
        }
    }

    pub(crate) fn os_version() -> Option<String> {
        let mut os_release: [c_int; 2] = [0; 2];
        unsafe {
            init_mib(b"kern.osrelease\0", &mut os_release);
            // It returns something like "13.0-RELEASE". We want to keep everything until the "-".
            get_system_info(&os_release, None)
                .and_then(|s| s.split('-').next().map(|s| s.to_owned()))
        }
    }

    pub(crate) fn long_os_version() -> Option<String> {
        let mut os_release: [c_int; 2] = [0; 2];
        unsafe {
            init_mib(b"kern.osrelease\0", &mut os_release);
            get_system_info(&os_release, None)
        }
    }

    pub(crate) fn host_name() -> Option<String> {
        let mut hostname: [c_int; 2] = [0; 2];
        unsafe {
            init_mib(b"kern.hostname\0", &mut hostname);
            get_system_info(&hostname, None)
        }
    }

    pub(crate) fn kernel_version() -> Option<String> {
        let mut kern_version: [c_int; 2] = [0; 2];
        unsafe {
            init_mib(b"kern.version\0", &mut kern_version);
            get_system_info(&kern_version, None)
        }
    }

    pub(crate) fn distribution_id() -> String {
        std::env::consts::OS.to_owned()
    }

    pub(crate) fn cpu_arch() -> Option<String> {
        let mut arch_str: [u8; 32] = [0; 32];
        let mib = [libc::CTL_HW as _, libc::HW_MACHINE as _];

        unsafe {
            if get_sys_value(&mib, &mut arch_str) {
                CStr::from_bytes_until_nul(&arch_str)
                    .ok()
                    .and_then(|res| match res.to_str() {
                        Ok(arch) => Some(arch.to_string()),
                        Err(_) => None,
                    })
            } else {
                None
            }
        }
    }
}

impl SystemInner {
    unsafe fn refresh_procs(
        &mut self,
        processes_to_update: ProcessesToUpdate<'_>,
        refresh_kind: ProcessRefreshKind,
    ) -> usize {
        let mut count = 0;
        let kvm_procs = libc::kvm_getprocs(
            self.system_info.kd.as_ptr(),
            libc::KERN_PROC_PROC,
            0,
            &mut count,
        );
        if count < 1 {
            sysinfo_debug!("kvm_getprocs returned nothing...");
            return 0;
        }

        #[inline(always)]
        fn real_filter(e: &libc::kinfo_proc, filter: &[Pid]) -> bool {
            filter.contains(&Pid(e.ki_pid))
        }

        #[inline(always)]
        fn empty_filter(_e: &libc::kinfo_proc, _filter: &[Pid]) -> bool {
            true
        }

        #[allow(clippy::type_complexity)]
        let (filter, filter_callback, remove_processes): (
            &[Pid],
            &(dyn Fn(&libc::kinfo_proc, &[Pid]) -> bool + Sync + Send),
            bool,
        ) = match processes_to_update {
            ProcessesToUpdate::All => (&[], &empty_filter, true),
            ProcessesToUpdate::Some(pids) => {
                if pids.is_empty() {
                    return 0;
                }
                (pids, &real_filter, false)
            }
        };

        let nb_updated = AtomicUsize::new(0);

        let new_processes = {
            #[cfg(feature = "multithread")]
            use rayon::iter::{ParallelIterator, ParallelIterator as IterTrait};
            #[cfg(not(feature = "multithread"))]
            use std::iter::Iterator as IterTrait;

            let kvm_procs: &mut [utils::KInfoProc] =
                std::slice::from_raw_parts_mut(kvm_procs as _, count as _);

            let fscale = self.system_info.fscale;
            let page_size = self.system_info.page_size as isize;
            let now = get_now();
            let proc_list = utils::WrapMap(UnsafeCell::new(&mut self.process_list));

            IterTrait::filter_map(crate::utils::into_iter(kvm_procs), |kproc| {
                if !filter_callback(kproc, filter) {
                    return None;
                }
                let ret = super::process::get_process_data(
                    kproc,
                    &proc_list,
                    page_size,
                    fscale,
                    now,
                    refresh_kind,
                )
                .ok()?;
                nb_updated.fetch_add(1, Ordering::Relaxed);
                ret
            })
            .collect::<Vec<_>>()
        };

        if remove_processes {
            // We remove all processes that don't exist anymore.
            self.process_list
                .retain(|_, v| std::mem::replace(&mut v.inner.updated, false));
        }

        for process in new_processes {
            self.process_list.insert(process.inner.pid, process);
        }
        let kvm_procs: &mut [utils::KInfoProc] =
            std::slice::from_raw_parts_mut(kvm_procs as _, count as _);

        for kproc in kvm_procs {
            if let Some(process) = self.process_list.get_mut(&Pid(kproc.ki_pid)) {
                add_missing_proc_info(&mut self.system_info, kproc, process, refresh_kind);
            }
        }
        nb_updated.into_inner()
    }
}

unsafe fn add_missing_proc_info(
    system_info: &mut SystemInfo,
    kproc: &libc::kinfo_proc,
    proc_: &mut Process,
    refresh_kind: ProcessRefreshKind,
) {
    {
        let kd = system_info.kd.as_ptr();
        let proc_inner = &mut proc_.inner;
        let cmd_needs_update = refresh_kind
            .cmd()
            .needs_update(|| proc_inner.cmd.is_empty());
        if proc_inner.name.is_empty() || cmd_needs_update {
            let cmd = from_cstr_array(libc::kvm_getargv(kd, kproc, 0) as _);

            if !cmd.is_empty() {
                // First, we try to retrieve the name from the command line.
                let p = Path::new(&cmd[0]);
                if let Some(name) = p.file_name() {
                    name.clone_into(&mut proc_inner.name);
                }

                if cmd_needs_update {
                    proc_inner.cmd = cmd;
                }
            }
        }
        get_exe(&mut proc_inner.exe, proc_inner.pid, refresh_kind);
        system_info.get_proc_missing_info(kproc, proc_inner, refresh_kind);
        if proc_inner.name.is_empty() {
            // The name can be cut short because the `ki_comm` field size is limited,
            // which is why we prefer to get the name from the command line as much as
            // possible.
            proc_inner.name = c_buf_to_os_string(&kproc.ki_comm);
        }
        if refresh_kind
            .environ()
            .needs_update(|| proc_inner.environ.is_empty())
        {
            proc_inner.environ = from_cstr_array(libc::kvm_getenvv(kd, kproc, 0) as _);
        }
    }
}

#[derive(Debug)]
struct Zfs {
    enabled: bool,
    mib_arcstats_size: [c_int; 5],
}

impl Zfs {
    fn new() -> Self {
        let mut zfs = Self {
            enabled: false,
            mib_arcstats_size: Default::default(),
        };
        unsafe {
            init_mib(
                b"kstat.zfs.misc.arcstats.size\0",
                &mut zfs.mib_arcstats_size,
            );
            let mut arc_size: u64 = 0;
            if get_sys_value(&zfs.mib_arcstats_size, &mut arc_size) {
                zfs.enabled = arc_size != 0;
            }
        }
        zfs
    }

    fn arc_size(&self) -> Option<u64> {
        if self.enabled {
            let mut arc_size: u64 = 0;
            unsafe {
                get_sys_value(&self.mib_arcstats_size, &mut arc_size);
                Some(arc_size)
            }
        } else {
            None
        }
    }
}

/// This struct is used to get system information more easily.
#[derive(Debug)]
struct SystemInfo {
    hw_physical_memory: [c_int; 2],
    page_size: c_int,
    virtual_page_count: [c_int; 4],
    virtual_wire_count: [c_int; 4],
    virtual_active_count: [c_int; 4],
    virtual_cache_count: [c_int; 4],
    virtual_inactive_count: [c_int; 4],
    virtual_free_count: [c_int; 4],
    buf_space: [c_int; 2],
    kd: NonNull<libc::kvm_t>,
    /// From FreeBSD manual: "The kernel fixed-point scale factor". It's used when computing
    /// processes' CPU usage.
    fscale: f32,
    procstat: *mut libc::procstat,
    zfs: Zfs,
}

// This is needed because `kd: *mut libc::kvm_t` isn't thread-safe.
unsafe impl Send for SystemInfo {}
unsafe impl Sync for SystemInfo {}

impl SystemInfo {
    fn new() -> Self {
        unsafe {
            let mut errbuf =
                MaybeUninit::<[libc::c_char; libc::_POSIX2_LINE_MAX as usize]>::uninit();
            let kd = NonNull::new(libc::kvm_openfiles(
                std::ptr::null(),
                b"/dev/null\0".as_ptr() as *const _,
                std::ptr::null(),
                0,
                errbuf.as_mut_ptr() as *mut _,
            ))
            .expect("kvm_openfiles failed");

            let mut si = SystemInfo {
                hw_physical_memory: Default::default(),
                page_size: 0,
                virtual_page_count: Default::default(),
                virtual_wire_count: Default::default(),
                virtual_active_count: Default::default(),
                virtual_cache_count: Default::default(),
                virtual_inactive_count: Default::default(),
                virtual_free_count: Default::default(),
                buf_space: Default::default(),
                kd,
                fscale: 0.,
                procstat: std::ptr::null_mut(),
                zfs: Zfs::new(),
            };
            let mut fscale: c_int = 0;
            if !get_sys_value_by_name(b"kern.fscale\0", &mut fscale) {
                // Default value used in htop.
                fscale = 2048;
            }
            si.fscale = fscale as f32;

            if !get_sys_value_by_name(b"vm.stats.vm.v_page_size\0", &mut si.page_size) {
                panic!("cannot get page size...");
            }

            init_mib(b"hw.physmem\0", &mut si.hw_physical_memory);
            init_mib(b"vm.stats.vm.v_page_count\0", &mut si.virtual_page_count);
            init_mib(b"vm.stats.vm.v_wire_count\0", &mut si.virtual_wire_count);
            init_mib(
                b"vm.stats.vm.v_active_count\0",
                &mut si.virtual_active_count,
            );
            init_mib(b"vm.stats.vm.v_cache_count\0", &mut si.virtual_cache_count);
            init_mib(
                b"vm.stats.vm.v_inactive_count\0",
                &mut si.virtual_inactive_count,
            );
            init_mib(b"vm.stats.vm.v_free_count\0", &mut si.virtual_free_count);
            init_mib(b"vfs.bufspace\0", &mut si.buf_space);

            si
        }
    }

    /// Returns (used, total).
    fn get_swap_info(&self) -> (u64, u64) {
        // Magic number used in htop. Cannot find how they got it when reading `kvm_getswapinfo`
        // source code so here we go...
        const LEN: usize = 16;
        let mut swap = MaybeUninit::<[libc::kvm_swap; LEN]>::uninit();
        unsafe {
            let nswap =
                libc::kvm_getswapinfo(self.kd.as_ptr(), swap.as_mut_ptr() as *mut _, LEN as _, 0)
                    as usize;
            if nswap < 1 {
                return (0, 0);
            }
            let swap =
                std::slice::from_raw_parts(swap.as_ptr() as *mut libc::kvm_swap, nswap.min(LEN));
            let (used, total) = swap.iter().fold((0, 0), |(used, total): (u64, u64), swap| {
                (
                    used.saturating_add(swap.ksw_used as _),
                    total.saturating_add(swap.ksw_total as _),
                )
            });
            (
                used.saturating_mul(self.page_size as _),
                total.saturating_mul(self.page_size as _),
            )
        }
    }

    fn get_total_memory(&self) -> u64 {
        let mut nb_pages: u64 = 0;
        unsafe {
            if get_sys_value(&self.virtual_page_count, &mut nb_pages) {
                return nb_pages.saturating_mul(self.page_size as _);
            }

            // This is a fallback. It includes all the available memory, not just the one available
            // for the users.
            let mut total_memory: u64 = 0;
            get_sys_value(&self.hw_physical_memory, &mut total_memory);
            total_memory
        }
    }

    fn get_used_memory(&self) -> u64 {
        let mut mem_active: u64 = 0;
        let mut mem_wire: u64 = 0;

        unsafe {
            get_sys_value(&self.virtual_active_count, &mut mem_active);
            get_sys_value(&self.virtual_wire_count, &mut mem_wire);

            let mut mem_wire = mem_wire.saturating_mul(self.page_size as _);
            // We need to subtract "ZFS ARC" from the "wired memory" because it should belongs to cache
            // but the kernel reports it as "wired memory" instead...
            if let Some(arc_size) = self.zfs.arc_size() {
                mem_wire = mem_wire.saturating_sub(arc_size);
            }
            mem_active
                .saturating_mul(self.page_size as _)
                .saturating_add(mem_wire)
        }
    }

    fn get_free_memory(&self) -> u64 {
        let mut buffers_mem: u64 = 0;
        let mut inactive_mem: u64 = 0;
        let mut cached_mem: u64 = 0;
        let mut free_mem: u64 = 0;

        unsafe {
            get_sys_value(&self.buf_space, &mut buffers_mem);
            get_sys_value(&self.virtual_inactive_count, &mut inactive_mem);
            get_sys_value(&self.virtual_cache_count, &mut cached_mem);
            get_sys_value(&self.virtual_free_count, &mut free_mem);
            // For whatever reason, buffers_mem is already the right value...
            buffers_mem
                .saturating_add(inactive_mem.saturating_mul(self.page_size as _))
                .saturating_add(cached_mem.saturating_mul(self.page_size as _))
                .saturating_add(free_mem.saturating_mul(self.page_size as _))
        }
    }

    #[allow(clippy::collapsible_if)] // I keep as is for readability reasons.
    unsafe fn get_proc_missing_info(
        &mut self,
        kproc: &libc::kinfo_proc,
        proc_: &mut ProcessInner,
        refresh_kind: ProcessRefreshKind,
    ) {
        let mut done = 0;
        let cwd_needs_update = refresh_kind.cwd().needs_update(|| proc_.cwd().is_none());
        let root_needs_update = refresh_kind.root().needs_update(|| proc_.root().is_none());
        if cwd_needs_update {
            done += 1;
        }
        if root_needs_update {
            done += 1;
        }
        if done == 0 {
            return;
        }
        if self.procstat.is_null() {
            self.procstat = libc::procstat_open_sysctl();
            if self.procstat.is_null() {
                sysinfo_debug!("procstat_open_sysctl failed");
                return;
            }
        }
        let head = libc::procstat_getfiles(self.procstat, kproc as *const _ as usize as *mut _, 0);
        if head.is_null() {
            return;
        }
        let mut entry = (*head).stqh_first;
        while !entry.is_null() && done > 0 {
            {
                let tmp = &*entry;
                if tmp.fs_uflags & libc::PS_FST_UFLAG_CDIR != 0 {
                    if cwd_needs_update && !tmp.fs_path.is_null() {
                        if let Ok(p) = CStr::from_ptr(tmp.fs_path).to_str() {
                            proc_.cwd = Some(PathBuf::from(p));
                            done -= 1;
                        }
                    }
                } else if tmp.fs_uflags & libc::PS_FST_UFLAG_RDIR != 0 {
                    if root_needs_update && !tmp.fs_path.is_null() {
                        if let Ok(p) = CStr::from_ptr(tmp.fs_path).to_str() {
                            proc_.root = Some(PathBuf::from(p));
                            done -= 1;
                        }
                    }
                }
            }
            entry = (*entry).next.stqe_next;
        }
        libc::procstat_freefiles(self.procstat, head);
    }
}

impl Drop for SystemInfo {
    fn drop(&mut self) {
        unsafe {
            libc::kvm_close(self.kd.as_ptr());
            if !self.procstat.is_null() {
                libc::procstat_close(self.procstat);
            }
        }
    }
}

fn get_system_info(mib: &[c_int], default: Option<&str>) -> Option<String> {
    let mut size = 0;

    unsafe {
        // Call first to get size
        libc::sysctl(
            mib.as_ptr(),
            mib.len() as _,
            std::ptr::null_mut(),
            &mut size,
            std::ptr::null_mut(),
            0,
        );

        // exit early if we did not update the size
        if size == 0 {
            default.map(|s| s.to_owned())
        } else {
            // set the buffer to the correct size
            let mut buf: Vec<libc::c_char> = vec![0; size as _];

            if libc::sysctl(
                mib.as_ptr(),
                mib.len() as _,
                buf.as_mut_ptr() as _,
                &mut size,
                std::ptr::null_mut(),
                0,
            ) == -1
            {
                // If command fails return default
                default.map(|s| s.to_owned())
            } else {
                c_buf_to_utf8_string(&buf)
            }
        }
    }
}

fn get_now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|n| n.as_secs())
        .unwrap_or(0)
}
