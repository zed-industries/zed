// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::cpu::*;
#[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
use crate::sys::process::*;
use crate::sys::utils::{get_sys_value, get_sys_value_by_name};

use crate::{Cpu, CpuRefreshKind, LoadAvg, MemoryRefreshKind, Pid, Process, ProcessesToUpdate, ProcessRefreshKind};

#[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ffi::CStr;
use std::mem;
use std::time::Duration;
#[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
use std::time::SystemTime;

use libc::{
    c_int, c_void, host_statistics64, mach_port_t, sysconf, sysctl, timeval, vm_statistics64,
    _SC_PAGESIZE,
};

#[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
declare_signals! {
    libc::c_int,
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
    // SIGPOLL doesn't exist on apple targets but since it's an equivalent of SIGIO on unix,
    // we simply use the SIGIO constant.
    Signal::Poll => libc::SIGIO,
    Signal::Sys => libc::SIGSYS,
    _ => None,
}
#[cfg(any(target_os = "ios", feature = "apple-sandbox"))]
declare_signals! {
    libc::c_int,
    _ => None,
}

#[doc = include_str!("../../../md_doc/supported_signals.md")]
pub const SUPPORTED_SIGNALS: &[crate::Signal] = supported_signals();
#[doc = include_str!("../../../md_doc/minimum_cpu_update_interval.md")]
pub const MINIMUM_CPU_UPDATE_INTERVAL: Duration = Duration::from_millis(200);

pub(crate) struct SystemInner {
    process_list: HashMap<Pid, Process>,
    mem_total: u64,
    mem_free: u64,
    mem_used: u64,
    mem_available: u64,
    swap_total: u64,
    swap_free: u64,
    page_size_b: u64,
    port: mach_port_t,
    #[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
    clock_info: Option<crate::sys::macos::system::SystemTimeInfo>,
    cpus: CpusWrapper,
}

#[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
pub(crate) struct Wrap<'a>(pub UnsafeCell<&'a mut HashMap<Pid, Process>>);

#[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
unsafe impl<'a> Send for Wrap<'a> {}
#[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
unsafe impl<'a> Sync for Wrap<'a> {}

fn boot_time() -> u64 {
    let mut boot_time = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut len = std::mem::size_of::<timeval>();
    let mut mib: [c_int; 2] = [libc::CTL_KERN, libc::KERN_BOOTTIME];

    unsafe {
        if sysctl(
            mib.as_mut_ptr(),
            mib.len() as _,
            &mut boot_time as *mut timeval as *mut _,
            &mut len,
            std::ptr::null_mut(),
            0,
        ) < 0
        {
            0
        } else {
            boot_time.tv_sec as _
        }
    }
}

#[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
fn get_now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|n| n.as_secs())
        .unwrap_or(0)
}

impl SystemInner {
    pub(crate) fn new() -> Self {
        unsafe {
            let port = libc::mach_host_self();

            Self {
                process_list: HashMap::with_capacity(200),
                mem_total: 0,
                mem_free: 0,
                mem_available: 0,
                mem_used: 0,
                swap_total: 0,
                swap_free: 0,
                page_size_b: sysconf(_SC_PAGESIZE) as _,
                port,
                #[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
                clock_info: crate::sys::macos::system::SystemTimeInfo::new(port),
                cpus: CpusWrapper::new(),
            }
        }
    }

    pub(crate) fn refresh_memory_specifics(&mut self, refresh_kind: MemoryRefreshKind) {
        let mut mib = [libc::CTL_VM as _, libc::VM_SWAPUSAGE as _];

        unsafe {
            if refresh_kind.swap() {
                // get system values
                // get swap info
                let mut xs: libc::xsw_usage = mem::zeroed::<libc::xsw_usage>();
                if get_sys_value(
                    mem::size_of::<libc::xsw_usage>(),
                    &mut xs as *mut _ as *mut c_void,
                    &mut mib,
                ) {
                    self.swap_total = xs.xsu_total;
                    self.swap_free = xs.xsu_avail;
                }
            }
            if refresh_kind.ram() {
                mib[0] = libc::CTL_HW as _;
                mib[1] = libc::HW_MEMSIZE as _;
                // get ram info
                if self.mem_total < 1 {
                    get_sys_value(
                        mem::size_of::<u64>(),
                        &mut self.mem_total as *mut u64 as *mut c_void,
                        &mut mib,
                    );
                }
                let mut count: u32 = libc::HOST_VM_INFO64_COUNT as _;
                let mut stat = mem::zeroed::<vm_statistics64>();
                if host_statistics64(
                    self.port,
                    libc::HOST_VM_INFO64,
                    &mut stat as *mut vm_statistics64 as *mut _,
                    &mut count,
                ) == libc::KERN_SUCCESS
                {
                    // From the apple documentation:
                    //
                    // /*
                    //  * NB: speculative pages are already accounted for in "free_count",
                    //  * so "speculative_count" is the number of "free" pages that are
                    //  * used to hold data that was read speculatively from disk but
                    //  * haven't actually been used by anyone so far.
                    //  */
                    self.mem_available = u64::from(stat.free_count)
                        .saturating_add(u64::from(stat.inactive_count))
                        .saturating_add(u64::from(stat.purgeable_count))
                        .saturating_sub(u64::from(stat.compressor_page_count))
                        .saturating_mul(self.page_size_b);
                    self.mem_used = u64::from(stat.active_count)
                        .saturating_add(u64::from(stat.wire_count))
                        .saturating_add(u64::from(stat.compressor_page_count))
                        .saturating_add(u64::from(stat.speculative_count))
                        .saturating_mul(self.page_size_b);
                    self.mem_free = u64::from(stat.free_count)
                        .saturating_sub(u64::from(stat.speculative_count))
                        .saturating_mul(self.page_size_b);
                }
            }
        }
    }

    pub(crate) fn cgroup_limits(&self) -> Option<crate::CGroupLimits> {
        None
    }

    pub(crate) fn refresh_cpu_specifics(&mut self, refresh_kind: CpuRefreshKind) {
        self.cpus.refresh(refresh_kind, self.port);
    }

    pub(crate) fn refresh_cpu_list(&mut self, refresh_kind: CpuRefreshKind) {
        self.cpus = CpusWrapper::new();
        self.cpus.refresh(refresh_kind, self.port);
    }

    #[cfg(any(target_os = "ios", feature = "apple-sandbox"))]
    pub(crate) fn refresh_processes_specifics(
        &mut self,
        processes_to_update: ProcessesToUpdate<'_>,
        _refresh_kind: ProcessRefreshKind,
    ) -> usize {
        0
    }

    #[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
    pub(crate) fn refresh_processes_specifics(
        &mut self,
        processes_to_update: ProcessesToUpdate<'_>,
        refresh_kind: ProcessRefreshKind,
    ) -> usize {
        use crate::utils::into_iter;
        use std::sync::atomic::{AtomicUsize, Ordering};

        unsafe {
            let count = libc::proc_listallpids(::std::ptr::null_mut(), 0);
            if count < 1 {
                return 0;
            }
        }
        if let Some(pids) = get_proc_list() {
            #[inline(always)]
            fn real_filter(e: Pid, filter: &[Pid]) -> bool {
                filter.contains(&e)
            }

            #[inline(always)]
            fn empty_filter(_e: Pid, _filter: &[Pid]) -> bool {
                true
            }

            #[allow(clippy::type_complexity)]
            let (filter, filter_callback, remove_processes): (
                &[Pid],
                &(dyn Fn(Pid, &[Pid]) -> bool + Sync + Send),
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
            let now = get_now();
            let port = self.port;
            let time_interval = self.clock_info.as_mut().map(|c| c.get_time_interval(port));
            let entries: Vec<Process> = {
                let wrap = &Wrap(UnsafeCell::new(&mut self.process_list));

                #[cfg(feature = "multithread")]
                use rayon::iter::ParallelIterator;

                into_iter(pids)
                    .flat_map(|pid| {
                        if !filter_callback(pid, filter) {
                            return None;
                        }
                        nb_updated.fetch_add(1, Ordering::Relaxed);
                        update_process(wrap, pid, time_interval, now, refresh_kind, false)
                            .unwrap_or_default()
                    })
                    .collect()
            };
            entries.into_iter().for_each(|entry| {
                self.process_list.insert(entry.pid(), entry);
            });
            if remove_processes {
                self.process_list
                    .retain(|_, proc_| std::mem::replace(&mut proc_.inner.updated, false));
            }
            nb_updated.into_inner()
        } else {
            0
        }
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
        self.cpus.global_cpu.percent()
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
        self.mem_available
    }

    pub(crate) fn used_memory(&self) -> u64 {
        self.mem_used
    }

    pub(crate) fn total_swap(&self) -> u64 {
        self.swap_total
    }

    pub(crate) fn free_swap(&self) -> u64 {
        self.swap_free
    }

    // TODO: need to be checked
    pub(crate) fn used_swap(&self) -> u64 {
        self.swap_total - self.swap_free
    }

    pub(crate) fn uptime() -> u64 {
        unsafe {
            let csec = libc::time(::std::ptr::null_mut());

            libc::difftime(csec, Self::boot_time() as _) as _
        }
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

    pub(crate) fn boot_time() -> u64 {
        boot_time()
    }

    pub(crate) fn name() -> Option<String> {
        get_system_info(libc::KERN_OSTYPE, Some("Darwin"))
    }

    pub(crate) fn long_os_version() -> Option<String> {
        #[cfg(target_os = "macos")]
        let friendly_name = match Self::os_version().unwrap_or_default() {
            f_n if f_n.starts_with("14.0") => "Sonoma",
            f_n if f_n.starts_with("10.16")
                | f_n.starts_with("11.0")
                | f_n.starts_with("11.1")
                | f_n.starts_with("11.2") =>
            {
                "Big Sur"
            }
            f_n if f_n.starts_with("10.15") => "Catalina",
            f_n if f_n.starts_with("10.14") => "Mojave",
            f_n if f_n.starts_with("10.13") => "High Sierra",
            f_n if f_n.starts_with("10.12") => "Sierra",
            f_n if f_n.starts_with("10.11") => "El Capitan",
            f_n if f_n.starts_with("10.10") => "Yosemite",
            f_n if f_n.starts_with("10.9") => "Mavericks",
            f_n if f_n.starts_with("10.8") => "Mountain Lion",
            f_n if f_n.starts_with("10.7") => "Lion",
            f_n if f_n.starts_with("10.6") => "Snow Leopard",
            f_n if f_n.starts_with("10.5") => "Leopard",
            f_n if f_n.starts_with("10.4") => "Tiger",
            f_n if f_n.starts_with("10.3") => "Panther",
            f_n if f_n.starts_with("10.2") => "Jaguar",
            f_n if f_n.starts_with("10.1") => "Puma",
            f_n if f_n.starts_with("10.0") => "Cheetah",
            _ => "",
        };

        #[cfg(target_os = "macos")]
        let long_name = Some(format!(
            "MacOS {} {}",
            Self::os_version().unwrap_or_default(),
            friendly_name
        ));

        #[cfg(target_os = "ios")]
        let long_name = Some(format!("iOS {}", Self::os_version().unwrap_or_default()));

        long_name
    }

    pub(crate) fn host_name() -> Option<String> {
        get_system_info(libc::KERN_HOSTNAME, None)
    }

    pub(crate) fn kernel_version() -> Option<String> {
        get_system_info(libc::KERN_OSRELEASE, None)
    }

    pub(crate) fn os_version() -> Option<String> {
        unsafe {
            // get the size for the buffer first
            let mut size = 0;
            if get_sys_value_by_name(b"kern.osproductversion\0", &mut size, std::ptr::null_mut())
                && size > 0
            {
                // now create a buffer with the size and get the real value
                let mut buf = vec![0_u8; size as _];

                if get_sys_value_by_name(
                    b"kern.osproductversion\0",
                    &mut size,
                    buf.as_mut_ptr() as *mut c_void,
                ) {
                    if let Some(pos) = buf.iter().position(|x| *x == 0) {
                        // Shrink buffer to terminate the null bytes
                        buf.resize(pos, 0);
                    }

                    String::from_utf8(buf).ok()
                } else {
                    // getting the system value failed
                    None
                }
            } else {
                // getting the system value failed, or did not return a buffer size
                None
            }
        }
    }

    pub(crate) fn distribution_id() -> String {
        std::env::consts::OS.to_owned()
    }

    pub(crate) fn cpu_arch() -> Option<String> {
        let mut arch_str: [u8; 32] = [0; 32];
        let mut mib = [libc::CTL_HW as _, libc::HW_MACHINE as _];

        unsafe {
            if get_sys_value(
                mem::size_of::<[u8; 32]>(),
                arch_str.as_mut_ptr() as *mut _,
                &mut mib,
            ) {
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

fn get_system_info(value: c_int, default: Option<&str>) -> Option<String> {
    let mut mib: [c_int; 2] = [libc::CTL_KERN, value];
    let mut size = 0;

    unsafe {
        // Call first to get size
        sysctl(
            mib.as_mut_ptr(),
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
            let mut buf = vec![0_u8; size as _];

            if sysctl(
                mib.as_mut_ptr(),
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
                if let Some(pos) = buf.iter().position(|x| *x == 0) {
                    // Shrink buffer to terminate the null bytes
                    buf.resize(pos, 0);
                }

                String::from_utf8(buf).ok()
            }
        }
    }
}
