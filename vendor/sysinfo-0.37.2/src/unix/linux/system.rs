// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::cpu::{CpusWrapper, get_physical_core_count};
use crate::sys::process::{compute_cpu_usage, refresh_procs};
use crate::sys::utils::{get_all_utf8_data, to_u64};
use crate::{
    Cpu, CpuRefreshKind, LoadAvg, MemoryRefreshKind, Pid, Process, ProcessRefreshKind,
    ProcessesToUpdate,
};

use libc::{self, _SC_CLK_TCK, _SC_HOST_NAME_MAX, _SC_PAGESIZE, c_char, sysconf};

use std::cmp::min;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::mem::MaybeUninit;
use std::path::Path;
use std::str::FromStr;
use std::sync::{OnceLock, atomic::AtomicIsize};
use std::time::Duration;

unsafe fn getrlimit() -> Option<libc::rlimit> {
    let mut limits = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };

    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limits) } != 0 {
        None
    } else {
        Some(limits)
    }
}

pub(crate) fn get_max_nb_fds() -> usize {
    unsafe {
        let mut limits = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        if libc::getrlimit(libc::RLIMIT_NOFILE, &mut limits) != 0 {
            // Most Linux system now defaults to 1024.
            1024 / 2
        } else {
            limits.rlim_max as usize / 2
        }
    }
}

// This whole thing is to prevent having too many files open at once. It could be problematic
// for processes using a lot of files and using sysinfo at the same time.
pub(crate) fn remaining_files() -> &'static AtomicIsize {
    static REMAINING_FILES: OnceLock<AtomicIsize> = OnceLock::new();
    REMAINING_FILES.get_or_init(|| unsafe {
        let Some(mut limits) = getrlimit() else {
            // Most Linux system now defaults to 1024.
            return AtomicIsize::new(1024 / 2);
        };
        // We save the value in case the update fails.
        let current = limits.rlim_cur;

        // The set the soft limit to the hard one.
        limits.rlim_cur = limits.rlim_max;
        // In this part, we leave minimum 50% of the available file descriptors to the process
        // using sysinfo.
        AtomicIsize::new(if libc::setrlimit(libc::RLIMIT_NOFILE, &limits) == 0 {
            limits.rlim_cur / 2
        } else {
            current / 2
        } as _)
    })
}

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
    Signal::Poll => libc::SIGPOLL,
    Signal::Power => libc::SIGPWR,
    Signal::Sys => libc::SIGSYS,
}

#[doc = include_str!("../../../md_doc/supported_signals.md")]
pub const SUPPORTED_SIGNALS: &[crate::Signal] = supported_signals();
#[doc = include_str!("../../../md_doc/minimum_cpu_update_interval.md")]
pub const MINIMUM_CPU_UPDATE_INTERVAL: Duration = Duration::from_millis(200);

fn boot_time() -> u64 {
    if let Ok(buf) = File::open("/proc/stat").and_then(|mut f| {
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        Ok(buf)
    }) {
        let line = buf.split(|c| *c == b'\n').find(|l| l.starts_with(b"btime"));

        if let Some(line) = line {
            return line
                .split(|x| *x == b' ')
                .filter(|s| !s.is_empty())
                .nth(1)
                .map(to_u64)
                .unwrap_or(0);
        }
    }
    // Either we didn't find "btime" or "/proc/stat" wasn't available for some reason...
    unsafe {
        let mut up: libc::timespec = std::mem::zeroed();
        if libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut up) == 0 {
            up.tv_sec as u64
        } else {
            sysinfo_debug!("clock_gettime failed: boot time cannot be retrieve...");
            0
        }
    }
}

pub(crate) struct SystemInfo {
    pub(crate) page_size_b: u64,
    pub(crate) clock_cycle: u64,
    pub(crate) boot_time: u64,
}

impl SystemInfo {
    fn new() -> Self {
        unsafe {
            Self {
                page_size_b: sysconf(_SC_PAGESIZE) as _,
                clock_cycle: sysconf(_SC_CLK_TCK) as _,
                boot_time: boot_time(),
            }
        }
    }
}

pub(crate) struct SystemInner {
    process_list: HashMap<Pid, Process>,
    mem_total: u64,
    mem_free: u64,
    mem_available: u64,
    mem_buffers: u64,
    mem_page_cache: u64,
    mem_shmem: u64,
    mem_slab_reclaimable: u64,
    swap_total: u64,
    swap_free: u64,
    info: SystemInfo,
    cpus: CpusWrapper,
}

impl SystemInner {
    /// It is sometime possible that a CPU usage computation is bigger than
    /// `"number of CPUs" * 100`.
    ///
    /// To prevent that, we compute ahead of time this maximum value and ensure that processes'
    /// CPU usage don't go over it.
    fn get_max_process_cpu_usage(&self) -> f32 {
        self.cpus.len() as f32 * 100.
    }

    fn update_procs_cpu(&mut self, refresh_kind: ProcessRefreshKind) {
        if !refresh_kind.cpu() {
            return;
        }
        self.cpus
            .refresh_if_needed(true, CpuRefreshKind::nothing().with_cpu_usage());

        if self.cpus.is_empty() {
            sysinfo_debug!("cannot compute processes CPU usage: no CPU found...");
            return;
        }
        let (new, old) = self.cpus.get_global_raw_times();
        let total_time = if old > new { 1 } else { new - old };
        let total_time = total_time as f32 / self.cpus.len() as f32;
        let max_value = self.get_max_process_cpu_usage();

        for proc_ in self.process_list.values_mut() {
            compute_cpu_usage(&mut proc_.inner, total_time, max_value);
        }
    }

    fn refresh_cpus(&mut self, only_update_global_cpu: bool, refresh_kind: CpuRefreshKind) {
        self.cpus.refresh(only_update_global_cpu, refresh_kind);
    }
}

impl SystemInner {
    pub(crate) fn new() -> Self {
        Self {
            process_list: HashMap::new(),
            mem_total: 0,
            mem_free: 0,
            mem_available: 0,
            mem_buffers: 0,
            mem_page_cache: 0,
            mem_shmem: 0,
            mem_slab_reclaimable: 0,
            swap_total: 0,
            swap_free: 0,
            cpus: CpusWrapper::new(),
            info: SystemInfo::new(),
        }
    }

    pub(crate) fn refresh_memory_specifics(&mut self, refresh_kind: MemoryRefreshKind) {
        if !refresh_kind.ram() && !refresh_kind.swap() {
            return;
        }
        let mut mem_available_found = false;
        read_table("/proc/meminfo", ':', |key, value_kib| {
            let field = match key {
                "MemTotal" => &mut self.mem_total,
                "MemFree" => &mut self.mem_free,
                "MemAvailable" => {
                    mem_available_found = true;
                    &mut self.mem_available
                }
                "Buffers" => &mut self.mem_buffers,
                "Cached" => &mut self.mem_page_cache,
                "Shmem" => &mut self.mem_shmem,
                "SReclaimable" => &mut self.mem_slab_reclaimable,
                "SwapTotal" => &mut self.swap_total,
                "SwapFree" => &mut self.swap_free,
                _ => return,
            };
            // /proc/meminfo reports KiB, though it says "kB". Convert it.
            *field = value_kib.saturating_mul(1_024);
        });

        // Linux < 3.14 may not have MemAvailable in /proc/meminfo
        // So it should fallback to the old way of estimating available memory
        // https://github.com/KittyKatt/screenFetch/issues/386#issuecomment-249312716
        if !mem_available_found {
            self.mem_available = self
                .mem_free
                .saturating_add(self.mem_buffers)
                .saturating_add(self.mem_page_cache)
                .saturating_add(self.mem_slab_reclaimable)
                .saturating_sub(self.mem_shmem);
        }
    }

    pub(crate) fn cgroup_limits(&self) -> Option<crate::CGroupLimits> {
        crate::CGroupLimits::new(self)
    }

    pub(crate) fn refresh_cpu_specifics(&mut self, refresh_kind: CpuRefreshKind) {
        self.refresh_cpus(false, refresh_kind);
    }

    pub(crate) fn refresh_processes_specifics(
        &mut self,
        processes_to_update: ProcessesToUpdate<'_>,
        refresh_kind: ProcessRefreshKind,
    ) -> usize {
        let uptime = Self::uptime();
        let nb_updated = refresh_procs(
            &mut self.process_list,
            Path::new("/proc"),
            uptime,
            &self.info,
            processes_to_update,
            refresh_kind,
        );
        self.update_procs_cpu(refresh_kind);
        nb_updated
    }

    // COMMON PART
    //
    // Need to be moved into a "common" file to avoid duplication.

    pub(crate) fn processes(&self) -> &HashMap<Pid, Process> {
        &self.process_list
    }

    pub(crate) fn processes_mut(&mut self) -> &mut HashMap<Pid, Process> {
        &mut self.process_list
    }

    pub(crate) fn process(&self, pid: Pid) -> Option<&Process> {
        self.process_list.get(&pid)
    }

    pub(crate) fn global_cpu_usage(&self) -> f32 {
        self.cpus.global_cpu.usage()
    }

    pub(crate) fn cpus(&self) -> &[Cpu] {
        &self.cpus.cpus
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
        self.mem_total - self.mem_available
    }

    pub(crate) fn total_swap(&self) -> u64 {
        self.swap_total
    }

    pub(crate) fn free_swap(&self) -> u64 {
        self.swap_free
    }

    // need to be checked
    pub(crate) fn used_swap(&self) -> u64 {
        self.swap_total - self.swap_free
    }

    pub(crate) fn uptime() -> u64 {
        if cfg!(not(target_os = "android"))
            && let Ok(content) = get_all_utf8_data("/proc/uptime", 50)
            && let Some(uptime) = content.split('.').next().and_then(|t| t.parse().ok())
        {
            return uptime;
        }
        Self::uptime_with_sysinfo()
    }

    fn uptime_with_sysinfo() -> u64 {
        unsafe {
            let mut s = MaybeUninit::<libc::sysinfo>::uninit();
            if libc::sysinfo(s.as_mut_ptr()) != 0 {
                return 0;
            }
            let s = s.assume_init();
            if s.uptime < 1 { 0 } else { s.uptime as u64 }
        }
    }

    pub(crate) fn boot_time() -> u64 {
        boot_time()
    }

    pub(crate) fn load_average() -> LoadAvg {
        let mut s = String::new();
        if File::open("/proc/loadavg")
            .and_then(|mut f| f.read_to_string(&mut s))
            .is_err()
        {
            return LoadAvg::default();
        }
        let loads = s
            .trim()
            .split(' ')
            .take(3)
            .map(|val| val.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        LoadAvg {
            one: loads[0],
            five: loads[1],
            fifteen: loads[2],
        }
    }

    #[cfg(not(target_os = "android"))]
    pub(crate) fn name() -> Option<String> {
        get_system_info_linux(
            InfoType::Name,
            Path::new("/etc/os-release"),
            Path::new("/etc/lsb-release"),
        )
    }

    #[cfg(target_os = "android")]
    pub(crate) fn name() -> Option<String> {
        get_system_info_android(InfoType::Name)
    }

    #[cfg(not(target_os = "android"))]
    pub(crate) fn long_os_version() -> Option<String> {
        let mut long_name = "Linux".to_owned();

        let distro_name = Self::name();
        let distro_version = Self::os_version();
        if let Some(distro_version) = &distro_version {
            // "Linux (Ubuntu 24.04)"
            long_name.push_str(" (");
            long_name.push_str(distro_name.as_deref().unwrap_or("unknown"));
            long_name.push(' ');
            long_name.push_str(distro_version);
            long_name.push(')');
        } else if let Some(distro_name) = &distro_name {
            // "Linux (Ubuntu)"
            long_name.push_str(" (");
            long_name.push_str(distro_name);
            long_name.push(')');
        }

        Some(long_name)
    }

    #[cfg(target_os = "android")]
    pub(crate) fn long_os_version() -> Option<String> {
        let mut long_name = "Android".to_owned();

        if let Some(os_version) = Self::os_version() {
            long_name.push(' ');
            long_name.push_str(&os_version);
        }

        // Android's name() is extracted from the system property "ro.product.model"
        // which is documented as "The end-user-visible name for the end product."
        // So this produces a long_os_version like "Android 15 on Pixel 9 Pro".
        if let Some(product_name) = Self::name() {
            long_name.push_str(" on ");
            long_name.push_str(&product_name);
        }

        Some(long_name)
    }

    pub(crate) fn host_name() -> Option<String> {
        unsafe {
            let hostname_max = sysconf(_SC_HOST_NAME_MAX);
            let mut buffer = vec![0_u8; hostname_max as usize];
            if libc::gethostname(buffer.as_mut_ptr() as *mut c_char, buffer.len()) == 0 {
                if let Some(pos) = buffer.iter().position(|x| *x == 0) {
                    // Shrink buffer to terminate the null bytes
                    buffer.resize(pos, 0);
                }
                String::from_utf8(buffer).ok()
            } else {
                sysinfo_debug!("gethostname failed: hostname cannot be retrieved...");
                None
            }
        }
    }

    pub(crate) fn kernel_version() -> Option<String> {
        let mut raw = MaybeUninit::<libc::utsname>::zeroed();

        unsafe {
            if libc::uname(raw.as_mut_ptr()) == 0 {
                let info = raw.assume_init();

                let release = info
                    .release
                    .iter()
                    .filter(|c| **c != 0)
                    .map(|c| *c as u8 as char)
                    .collect::<String>();

                Some(release)
            } else {
                None
            }
        }
    }

    #[cfg(not(target_os = "android"))]
    pub(crate) fn os_version() -> Option<String> {
        get_system_info_linux(
            InfoType::OsVersion,
            Path::new("/etc/os-release"),
            Path::new("/etc/lsb-release"),
        )
    }

    #[cfg(target_os = "android")]
    pub(crate) fn os_version() -> Option<String> {
        get_system_info_android(InfoType::OsVersion)
    }

    #[cfg(not(target_os = "android"))]
    pub(crate) fn distribution_id() -> String {
        get_system_info_linux(
            InfoType::DistributionID,
            Path::new("/etc/os-release"),
            Path::new(""),
        )
        .unwrap_or_else(|| std::env::consts::OS.to_owned())
    }

    #[cfg(target_os = "android")]
    pub(crate) fn distribution_id() -> String {
        // Currently get_system_info_android doesn't support InfoType::DistributionID and always
        // returns None. This call is done anyway for consistency with non-Android implementation
        // and to suppress dead-code warning for DistributionID on Android.
        get_system_info_android(InfoType::DistributionID)
            .unwrap_or_else(|| std::env::consts::OS.to_owned())
    }

    #[cfg(not(target_os = "android"))]
    pub(crate) fn distribution_id_like() -> Vec<String> {
        system_info_as_list(get_system_info_linux(
            InfoType::DistributionIDLike,
            Path::new("/etc/os-release"),
            Path::new(""),
        ))
    }

    #[cfg(target_os = "android")]
    pub(crate) fn distribution_id_like() -> Vec<String> {
        // Currently get_system_info_android doesn't support InfoType::DistributionIDLike and always
        // returns None. This call is done anyway for consistency with non-Android implementation
        // and to suppress dead-code warning for DistributionIDLike on Android.
        system_info_as_list(get_system_info_android(InfoType::DistributionIDLike))
    }

    #[cfg(not(target_os = "android"))]
    pub(crate) fn kernel_name() -> Option<&'static str> {
        Some("Linux")
    }

    #[cfg(target_os = "android")]
    pub(crate) fn kernel_name() -> Option<&'static str> {
        Some("Android kernel")
    }

    pub(crate) fn cpu_arch() -> Option<String> {
        let mut raw = MaybeUninit::<libc::utsname>::uninit();

        unsafe {
            if libc::uname(raw.as_mut_ptr()) != 0 {
                return None;
            }
            let info = raw.assume_init();
            // Converting `&[i8]` to `&[u8]`.
            let machine: &[u8] =
                std::slice::from_raw_parts(info.machine.as_ptr() as *const _, info.machine.len());

            CStr::from_bytes_until_nul(machine)
                .ok()
                .and_then(|res| match res.to_str() {
                    Ok(arch) => Some(arch.to_string()),
                    Err(_) => None,
                })
        }
    }

    pub(crate) fn physical_core_count() -> Option<usize> {
        get_physical_core_count()
    }

    pub(crate) fn refresh_cpu_list(&mut self, refresh_kind: CpuRefreshKind) {
        self.cpus = CpusWrapper::new();
        self.refresh_cpu_specifics(refresh_kind);
    }

    pub(crate) fn open_files_limit() -> Option<usize> {
        unsafe {
            match getrlimit() {
                Some(limits) => Some(limits.rlim_cur as _),
                None => {
                    sysinfo_debug!("getrlimit failed");
                    None
                }
            }
        }
    }
}

fn read_u64(filename: &str) -> Option<u64> {
    let result = get_all_utf8_data(filename, 16_635)
        .ok()
        .and_then(|d| u64::from_str(d.trim()).ok());

    if result.is_none() {
        sysinfo_debug!("Failed to read u64 in filename {}", filename);
    }

    result
}

fn read_table<F>(filename: &str, colsep: char, mut f: F)
where
    F: FnMut(&str, u64),
{
    if let Ok(content) = get_all_utf8_data(filename, 16_635) {
        content
            .split('\n')
            .flat_map(|line| {
                let mut split = line.split(colsep);
                let key = split.next()?;
                let value = split.next()?;
                let value0 = value.trim_start().split(' ').next()?;
                let value0_u64 = u64::from_str(value0).ok()?;
                Some((key, value0_u64))
            })
            .for_each(|(k, v)| f(k, v));
    }
}

fn read_table_key(filename: &str, target_key: &str, colsep: char) -> Option<u64> {
    if let Ok(content) = get_all_utf8_data(filename, 16_635) {
        return content.split('\n').find_map(|line| {
            let mut split = line.split(colsep);
            let key = split.next()?;
            if key != target_key {
                return None;
            }

            let value = split.next()?;
            let value0 = value.trim_start().split(' ').next()?;
            u64::from_str(value0).ok()
        });
    }

    None
}

impl crate::CGroupLimits {
    fn new(sys: &SystemInner) -> Option<Self> {
        assert!(
            sys.mem_total != 0,
            "You need to call System::refresh_memory before trying to get cgroup limits!",
        );
        if let (Some(mem_cur), Some(mem_max), Some(mem_rss)) = (
            // cgroups v2
            read_u64("/sys/fs/cgroup/memory.current"),
            // memory.max contains `max` when no limit is set.
            read_u64("/sys/fs/cgroup/memory.max").or(Some(u64::MAX)),
            read_table_key("/sys/fs/cgroup/memory.stat", "anon", ' '),
        ) {
            let mut limits = Self {
                total_memory: sys.mem_total,
                free_memory: sys.mem_free,
                free_swap: sys.swap_free,
                rss: mem_rss,
            };

            limits.total_memory = min(mem_max, sys.mem_total);
            limits.free_memory = limits.total_memory.saturating_sub(mem_cur);

            if let Some(swap_cur) = read_u64("/sys/fs/cgroup/memory.swap.current") {
                limits.free_swap = sys.swap_total.saturating_sub(swap_cur);
            }

            Some(limits)
        } else if let (Some(mem_cur), Some(mem_max), Some(mem_rss)) = (
            // cgroups v1
            read_u64("/sys/fs/cgroup/memory/memory.usage_in_bytes"),
            read_u64("/sys/fs/cgroup/memory/memory.limit_in_bytes"),
            read_table_key("/sys/fs/cgroup/memory/memory.stat", "total_rss", ' '),
        ) {
            let mut limits = Self {
                total_memory: sys.mem_total,
                free_memory: sys.mem_free,
                free_swap: sys.swap_free,
                rss: mem_rss,
            };

            limits.total_memory = min(mem_max, sys.mem_total);
            limits.free_memory = limits.total_memory.saturating_sub(mem_cur);

            Some(limits)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq)]
enum InfoType {
    /// The end-user friendly name of:
    /// - Android: The device model
    /// - Linux: The distributions name
    Name,
    OsVersion,
    /// Machine-parseable ID of a distribution, see
    /// https://www.freedesktop.org/software/systemd/man/os-release.html#ID=
    DistributionID,
    /// Machine-parseable ID_LIKE of related distributions, see
    /// <https://www.freedesktop.org/software/systemd/man/latest/os-release.html#ID_LIKE=>
    DistributionIDLike,
}

#[cfg(not(target_os = "android"))]
fn get_system_info_linux(info: InfoType, path: &Path, fallback_path: &Path) -> Option<String> {
    if let Ok(buf) = File::open(path).and_then(|mut f| {
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        Ok(buf)
    }) {
        let info_str = match info {
            InfoType::Name => "NAME=",
            InfoType::OsVersion => "VERSION_ID=",
            InfoType::DistributionID => "ID=",
            InfoType::DistributionIDLike => "ID_LIKE=",
        };

        for line in buf.lines() {
            if let Some(stripped) = line.strip_prefix(info_str) {
                return Some(stripped.replace('"', ""));
            }
        }
    }

    // Fallback to `/etc/lsb-release` file for systems where VERSION_ID is not included.
    // VERSION_ID is not required in the `/etc/os-release` file
    // per https://www.linux.org/docs/man5/os-release.html
    // If this fails for some reason, fallback to None
    let buf = File::open(fallback_path)
        .and_then(|mut f| {
            let mut buf = String::new();
            f.read_to_string(&mut buf)?;
            Ok(buf)
        })
        .ok()?;

    let info_str = match info {
        InfoType::OsVersion => "DISTRIB_RELEASE=",
        InfoType::Name => "DISTRIB_ID=",
        InfoType::DistributionID => {
            // lsb-release is inconsistent with os-release and unsupported.
            return None;
        }
        InfoType::DistributionIDLike => {
            // lsb-release doesn't support ID_LIKE.
            return None;
        }
    };
    for line in buf.lines() {
        if let Some(stripped) = line.strip_prefix(info_str) {
            return Some(stripped.replace('"', ""));
        }
    }
    None
}

/// Returns a system info value as a list of strings.
/// Absence of a value is treated as an empty list.
fn system_info_as_list(sysinfo: Option<String>) -> Vec<String> {
    match sysinfo {
        Some(value) => value.split_ascii_whitespace().map(String::from).collect(),
        // For list fields absence of a field is equivalent to an empty list.
        None => Vec::new(),
    }
}

#[cfg(target_os = "android")]
fn get_system_info_android(info: InfoType) -> Option<String> {
    // https://android.googlesource.com/platform/frameworks/base/+/refs/heads/master/core/java/android/os/Build.java#58
    let name: &'static [u8] = match info {
        InfoType::Name => b"ro.product.model\0",
        InfoType::OsVersion => b"ro.build.version.release\0",
        InfoType::DistributionID => {
            // Not supported.
            return None;
        }
        InfoType::DistributionIDLike => {
            // Not supported.
            return None;
        }
    };

    let mut value_buffer = vec![0u8; libc::PROP_VALUE_MAX as usize];
    unsafe {
        let len = libc::__system_property_get(
            name.as_ptr() as *const c_char,
            value_buffer.as_mut_ptr() as *mut c_char,
        );

        if len != 0 {
            if let Some(pos) = value_buffer.iter().position(|c| *c == 0) {
                value_buffer.resize(pos, 0);
            }
            String::from_utf8(value_buffer).ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::InfoType;
    #[cfg(target_os = "android")]
    use super::get_system_info_android;
    #[cfg(not(target_os = "android"))]
    use super::get_system_info_linux;
    use super::read_table;
    use super::read_table_key;
    use super::system_info_as_list;
    use std::collections::HashMap;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_table() {
        // Create a temporary file with test content
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "KEY1:100 kB").unwrap();
        writeln!(file, "KEY2:200 kB").unwrap();
        writeln!(file, "KEY3:300 kB").unwrap();
        writeln!(file, "KEY4:invalid").unwrap();

        let file_path = file.path().to_str().unwrap();

        // Test reading the table
        let mut result = HashMap::new();
        read_table(file_path, ':', |key, value| {
            result.insert(key.to_string(), value);
        });

        assert_eq!(result.get("KEY1"), Some(&100));
        assert_eq!(result.get("KEY2"), Some(&200));
        assert_eq!(result.get("KEY3"), Some(&300));
        assert_eq!(result.get("KEY4"), None);

        // Test with different separator and units
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "KEY1 400 MB").unwrap();
        writeln!(file, "KEY2 500 GB").unwrap();
        writeln!(file, "KEY3 600").unwrap();

        let file_path = file.path().to_str().unwrap();

        let mut result = HashMap::new();
        read_table(file_path, ' ', |key, value| {
            result.insert(key.to_string(), value);
        });

        assert_eq!(result.get("KEY1"), Some(&400));
        assert_eq!(result.get("KEY2"), Some(&500));
        assert_eq!(result.get("KEY3"), Some(&600));

        // Test with empty file
        let file = NamedTempFile::new().unwrap();
        let file_path = file.path().to_str().unwrap();

        let mut result = HashMap::new();
        read_table(file_path, ':', |key, value| {
            result.insert(key.to_string(), value);
        });

        assert!(result.is_empty());

        // Test with non-existent file
        let mut result = HashMap::new();
        read_table("/nonexistent/file", ':', |key, value| {
            result.insert(key.to_string(), value);
        });

        assert!(result.is_empty());
    }

    #[test]
    fn test_read_table_key() {
        // Create a temporary file with test content
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "KEY1:100 kB").unwrap();
        writeln!(file, "KEY2:200 kB").unwrap();
        writeln!(file, "KEY3:300 kB").unwrap();

        let file_path = file.path().to_str().unwrap();

        // Test existing keys
        assert_eq!(read_table_key(file_path, "KEY1", ':'), Some(100));
        assert_eq!(read_table_key(file_path, "KEY2", ':'), Some(200));
        assert_eq!(read_table_key(file_path, "KEY3", ':'), Some(300));

        // Test non-existent key
        assert_eq!(read_table_key(file_path, "KEY4", ':'), None);

        // Test with different separator
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "KEY1 400 kB").unwrap();
        writeln!(file, "KEY2 500 kB").unwrap();

        let file_path = file.path().to_str().unwrap();

        assert_eq!(read_table_key(file_path, "KEY1", ' '), Some(400));
        assert_eq!(read_table_key(file_path, "KEY2", ' '), Some(500));

        // Test with invalid file
        assert_eq!(read_table_key("/nonexistent/file", "KEY1", ':'), None);
    }

    #[test]
    #[cfg(target_os = "android")]
    fn lsb_release_fallback_android() {
        assert!(get_system_info_android(InfoType::OsVersion).is_some());
        assert!(get_system_info_android(InfoType::Name).is_some());
        assert!(get_system_info_android(InfoType::DistributionID).is_none());
        assert!(get_system_info_android(InfoType::DistributionIDLike).is_none());
    }

    #[test]
    #[cfg(not(target_os = "android"))]
    fn lsb_release_fallback_not_android() {
        use std::path::Path;

        let dir = tempfile::tempdir().expect("failed to create temporary directory");
        let tmp1 = dir.path().join("tmp1");
        let tmp2 = dir.path().join("tmp2");

        // /etc/os-release
        std::fs::write(
            &tmp1,
            r#"NAME="Ubuntu"
VERSION="20.10 (Groovy Gorilla)"
ID=ubuntu
ID_LIKE=debian
PRETTY_NAME="Ubuntu 20.10"
VERSION_ID="20.10"
VERSION_CODENAME=groovy
UBUNTU_CODENAME=groovy
"#,
        )
        .expect("Failed to create tmp1");

        // /etc/lsb-release
        std::fs::write(
            &tmp2,
            r#"DISTRIB_ID=Ubuntu
DISTRIB_RELEASE=20.10
DISTRIB_CODENAME=groovy
DISTRIB_DESCRIPTION="Ubuntu 20.10"
"#,
        )
        .expect("Failed to create tmp2");

        // Check for the "normal" path: "/etc/os-release"
        assert_eq!(
            get_system_info_linux(InfoType::OsVersion, &tmp1, Path::new("")),
            Some("20.10".to_owned())
        );
        assert_eq!(
            get_system_info_linux(InfoType::Name, &tmp1, Path::new("")),
            Some("Ubuntu".to_owned())
        );
        assert_eq!(
            get_system_info_linux(InfoType::DistributionID, &tmp1, Path::new("")),
            Some("ubuntu".to_owned())
        );
        assert_eq!(
            get_system_info_linux(InfoType::DistributionIDLike, &tmp1, Path::new("")),
            Some("debian".to_owned())
        );

        // Check for the "fallback" path: "/etc/lsb-release"
        assert_eq!(
            get_system_info_linux(InfoType::OsVersion, Path::new(""), &tmp2),
            Some("20.10".to_owned())
        );
        assert_eq!(
            get_system_info_linux(InfoType::Name, Path::new(""), &tmp2),
            Some("Ubuntu".to_owned())
        );
        assert_eq!(
            get_system_info_linux(InfoType::DistributionID, Path::new(""), &tmp2),
            None
        );
        assert_eq!(
            get_system_info_linux(InfoType::DistributionIDLike, Path::new(""), &tmp2),
            None
        );
    }

    #[test]
    fn test_system_info_as_list() {
        // No value.
        assert_eq!(system_info_as_list(None), Vec::<String>::new());
        // Empty value.
        assert_eq!(
            system_info_as_list(Some("".to_string())),
            Vec::<String>::new(),
        );
        // Whitespaces only.
        assert_eq!(
            system_info_as_list(Some(" ".to_string())),
            Vec::<String>::new(),
        );
        // Single value.
        assert_eq!(
            system_info_as_list(Some("debian".to_string())),
            vec!["debian".to_string()],
        );
        // Multiple values.
        assert_eq!(
            system_info_as_list(Some("rhel fedora".to_string())),
            vec!["rhel".to_string(), "fedora".to_string()],
        );
        // Multiple spaces.
        assert_eq!(
            system_info_as_list(Some("rhel        fedora".to_string())),
            vec!["rhel".to_string(), "fedora".to_string()],
        );
    }
}
