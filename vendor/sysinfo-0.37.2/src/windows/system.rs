// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    Cpu, CpuRefreshKind, LoadAvg, MemoryRefreshKind, Pid, ProcessRefreshKind, ProcessesToUpdate,
};

use crate::sys::cpu::*;
use crate::{Process, ProcessInner};

use std::collections::HashMap;
use std::ffi::OsStr;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use std::time::{Duration, SystemTime};

use windows::Win32::Foundation::{self, HANDLE, STILL_ACTIVE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::ProcessStatus::{K32GetPerformanceInfo, PERFORMANCE_INFORMATION};
use windows::Win32::System::Registry::{
    HKEY, HKEY_LOCAL_MACHINE, KEY_READ, REG_NONE, RegCloseKey, RegOpenKeyExW, RegQueryValueExW,
};
use windows::Win32::System::SystemInformation::{self, GetSystemInfo};
use windows::Win32::System::SystemInformation::{
    ComputerNamePhysicalDnsHostname, GetComputerNameExW, GetTickCount64, GlobalMemoryStatusEx,
    MEMORYSTATUSEX, SYSTEM_INFO,
};
use windows::Win32::System::Threading::GetExitCodeProcess;
use windows::core::{Owned, PCWSTR, PWSTR};

declare_signals! {
    (),
    Signal::Kill => (),
    _ => None,
}

#[doc = include_str!("../../md_doc/supported_signals.md")]
pub const SUPPORTED_SIGNALS: &[crate::Signal] = supported_signals();
#[doc = include_str!("../../md_doc/minimum_cpu_update_interval.md")]
pub const MINIMUM_CPU_UPDATE_INTERVAL: Duration = Duration::from_millis(200);

const WINDOWS_ELEVEN_BUILD_NUMBER: u32 = 22000;

impl SystemInner {
    fn is_windows_eleven() -> bool {
        WINDOWS_ELEVEN_BUILD_NUMBER
            <= Self::kernel_version()
                .unwrap_or_default()
                .parse()
                .unwrap_or(0)
    }
}

/// Calculates system boot time in seconds with improved precision.
/// Uses nanoseconds throughout to avoid rounding errors in uptime calculation,
/// converting to seconds only at the end for stable results. Result is capped
/// within u64 limits to handle edge cases.
unsafe fn boot_time() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let system_time_ns = n.as_nanos();
            // milliseconds to nanoseconds
            let tick_count_ns = unsafe { GetTickCount64() } as u128 * 1_000_000;
            // nanoseconds to seconds
            let boot_time_sec = system_time_ns.saturating_sub(tick_count_ns) / 1_000_000_000;
            boot_time_sec.try_into().unwrap_or(u64::MAX)
        }
        Err(_e) => {
            sysinfo_debug!("Failed to compute boot time: {:?}", _e);
            0
        }
    }
}

pub(crate) struct SystemInner {
    process_list: HashMap<Pid, Process>,
    mem_total: u64,
    mem_available: u64,
    swap_total: u64,
    swap_used: u64,
    cpus: CpusWrapper,
    query: Option<Query>,
}

impl SystemInner {
    pub(crate) fn new() -> Self {
        Self {
            process_list: HashMap::with_capacity(500),
            mem_total: 0,
            mem_available: 0,
            swap_total: 0,
            swap_used: 0,
            cpus: CpusWrapper::new(),
            query: None,
        }
    }

    fn initialize_cpu_counters(&mut self, refresh_kind: CpuRefreshKind) {
        if let Some(ref mut query) = self.query {
            add_english_counter(
                r"\Processor(_Total)\% Idle Time".to_string(),
                query,
                &mut self.cpus.global.key_used,
                "tot_0".to_owned(),
            );
            for (pos, proc_) in self.cpus.iter_mut(refresh_kind).enumerate() {
                add_english_counter(
                    format!(r"\Processor({pos})\% Idle Time"),
                    query,
                    get_key_used(proc_),
                    format!("{pos}_0"),
                );
            }
        }
    }

    pub(crate) fn refresh_cpu_specifics(&mut self, refresh_kind: CpuRefreshKind) {
        if self.query.is_none() {
            self.query = Query::new(false);
            self.initialize_cpu_counters(refresh_kind);
        } else if self.cpus.global.key_used.is_none() {
            self.query = Query::new(true);
            self.initialize_cpu_counters(refresh_kind);
        }
        if let Some(ref mut query) = self.query {
            query.refresh();
            let mut total_idle_time = None;
            if let Some(ref key_used) = self.cpus.global.key_used {
                total_idle_time = Some(
                    query
                        .get(&key_used.unique_id)
                        .expect("global_key_idle disappeared"),
                );
            }
            if let Some(total_idle_time) = total_idle_time {
                self.cpus.global.set_cpu_usage(100.0 - total_idle_time);
            }
            for cpu in self.cpus.iter_mut(refresh_kind) {
                let mut idle_time = None;
                if let Some(ref key_used) = *get_key_used(cpu) {
                    idle_time = Some(
                        query
                            .get(&key_used.unique_id)
                            .expect("key_used disappeared"),
                    );
                }
                if let Some(idle_time) = idle_time {
                    cpu.inner.set_cpu_usage(100.0 - idle_time);
                }
            }
            if refresh_kind.frequency() {
                self.cpus.get_frequencies();
            }
        }
    }

    pub(crate) fn refresh_cpu_list(&mut self, refresh_kind: CpuRefreshKind) {
        self.cpus = CpusWrapper::new();
        self.refresh_cpu_specifics(refresh_kind);
    }

    pub(crate) fn refresh_memory_specifics(&mut self, refresh_kind: MemoryRefreshKind) {
        unsafe {
            if refresh_kind.ram() {
                let mut mem_info: MEMORYSTATUSEX = zeroed();
                mem_info.dwLength = size_of::<MEMORYSTATUSEX>() as _;
                let _err = GlobalMemoryStatusEx(&mut mem_info);
                self.mem_total = mem_info.ullTotalPhys as _;
                self.mem_available = mem_info.ullAvailPhys as _;
            }
            if refresh_kind.swap() {
                let mut perf_info: PERFORMANCE_INFORMATION = zeroed();
                if K32GetPerformanceInfo(&mut perf_info, size_of::<PERFORMANCE_INFORMATION>() as _)
                    .as_bool()
                {
                    let page_size = perf_info.PageSize as u64;
                    let physical_total = perf_info.PhysicalTotal as u64;
                    let commit_limit = perf_info.CommitLimit as u64;
                    let commit_total = perf_info.CommitTotal as u64;
                    self.swap_total =
                        page_size.saturating_mul(commit_limit.saturating_sub(physical_total));
                    self.swap_used =
                        page_size.saturating_mul(commit_total.saturating_sub(physical_total));
                }
            }
        }
    }

    pub(crate) fn cgroup_limits(&self) -> Option<crate::CGroupLimits> {
        None
    }

    #[allow(clippy::cast_ptr_alignment)]
    pub(crate) fn refresh_processes_specifics(
        &mut self,
        processes_to_update: ProcessesToUpdate<'_>,
        refresh_kind: ProcessRefreshKind,
    ) -> usize {
        #[inline(always)]
        fn real_filter(e: Pid, filter: &[Pid]) -> bool {
            filter.contains(&e)
        }

        #[inline(always)]
        fn empty_filter(_e: Pid, _filter: &[Pid]) -> bool {
            true
        }

        #[allow(clippy::type_complexity)]
        let (filter_array, filter_callback): (
            &[Pid],
            &(dyn Fn(Pid, &[Pid]) -> bool + Sync + Send),
        ) = match processes_to_update {
            ProcessesToUpdate::All => (&[], &empty_filter),
            ProcessesToUpdate::Some(pids) => {
                if pids.is_empty() {
                    return 0;
                }
                (pids, &real_filter)
            }
        };

        let now = get_now();

        let nb_cpus = if refresh_kind.cpu() {
            self.cpus.len() as u64
        } else {
            0
        };

        // Use the amazing and cool CreateToolhelp32Snapshot function.
        // Take a snapshot of all running processes. Match the result to an error
        let snapshot = match unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) } {
            Ok(handle) => handle,
            Err(_err) => {
                sysinfo_debug!(
                    "Error capturing process snapshot: CreateToolhelp32Snapshot returned {}",
                    _err
                );
                return 0;
            }
        };

        // This owns the above handle and makes sure that close will be called when dropped.
        let snapshot = unsafe { Owned::new(snapshot) };

        // https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-processentry32w
        // Microsoft documentation states that for PROCESSENTRY32W, before calling Process32FirstW,
        // the 'dwSize' field MUST be set to the size of the PROCESSENTRY32W. Otherwise, Process32FirstW fails.
        let mut process_entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        let mut num_procs = 0; // keep track of the number of updated processes
        let process_list = &mut self.process_list;

        // process the first process
        unsafe {
            if let Err(_error) = Process32FirstW(*snapshot, &mut process_entry) {
                sysinfo_debug!("Process32FirstW has failed: {_error:?}");
                return 0;
            }
        }

        // Iterate over processes in the snapshot.
        // Use Process32NextW to process the next PROCESSENTRY32W in the snapshot
        loop {
            let proc_id = Pid::from_u32(process_entry.th32ProcessID);

            if filter_callback(proc_id, filter_array) {
                // exists already
                if let Some(p) = process_list.get_mut(&proc_id) {
                    // Update with the most recent information
                    let p = &mut p.inner;
                    p.update(refresh_kind, nb_cpus, now, false);

                    // Update parent process
                    let parent = if process_entry.th32ParentProcessID == 0 {
                        None
                    } else {
                        Some(Pid::from_u32(process_entry.th32ParentProcessID))
                    };

                    p.parent = parent;
                } else {
                    // Make a new 'ProcessInner' using the Windows PROCESSENTRY32W struct.
                    let mut p = ProcessInner::from_process_entry(&process_entry, now);
                    p.update(refresh_kind, nb_cpus, now, false);
                    process_list.insert(proc_id, Process { inner: p });
                }

                num_procs += 1;
            }

            // nothing else to process
            if unsafe { Process32NextW(*snapshot, &mut process_entry).is_err() } {
                break;
            }
        }

        num_procs
    }

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
        self.cpus.global_cpu_usage()
    }

    pub(crate) fn cpus(&self) -> &[Cpu] {
        self.cpus.cpus()
    }

    pub(crate) fn total_memory(&self) -> u64 {
        self.mem_total
    }

    pub(crate) fn free_memory(&self) -> u64 {
        // MEMORYSTATUSEX doesn't report free memory
        self.mem_available
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
        self.swap_total - self.swap_used
    }

    pub(crate) fn used_swap(&self) -> u64 {
        self.swap_used
    }

    pub(crate) fn uptime() -> u64 {
        unsafe { GetTickCount64() / 1_000 }
    }

    pub(crate) fn boot_time() -> u64 {
        unsafe { boot_time() }
    }

    pub(crate) fn load_average() -> LoadAvg {
        get_load_average()
    }

    pub(crate) fn name() -> Option<String> {
        Some("Windows".to_owned())
    }

    pub(crate) fn long_os_version() -> Option<String> {
        if Self::is_windows_eleven() {
            return get_reg_string_value(
                HKEY_LOCAL_MACHINE,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
                "ProductName",
            )
            .map(|product_name| product_name.replace("Windows 10 ", "Windows 11 "));
        }
        get_reg_string_value(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
            "ProductName",
        )
    }

    pub(crate) fn host_name() -> Option<String> {
        get_dns_hostname()
    }

    pub(crate) fn kernel_version() -> Option<String> {
        get_reg_string_value(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
            "CurrentBuildNumber",
        )
    }

    pub(crate) fn os_version() -> Option<String> {
        let build_number = get_reg_string_value(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
            "CurrentBuildNumber",
        )
        .unwrap_or_default();
        let major = if Self::is_windows_eleven() {
            11u32
        } else {
            u32::from_le_bytes(
                get_reg_value_u32(
                    HKEY_LOCAL_MACHINE,
                    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
                    "CurrentMajorVersionNumber",
                )
                .unwrap_or_default(),
            )
        };
        Some(format!("{major} ({build_number})"))
    }

    pub(crate) fn distribution_id() -> String {
        std::env::consts::OS.to_owned()
    }

    pub(crate) fn distribution_id_like() -> Vec<String> {
        Vec::new()
    }

    pub(crate) fn kernel_name() -> Option<&'static str> {
        Some("Windows")
    }

    pub(crate) fn cpu_arch() -> Option<String> {
        unsafe {
            // https://docs.microsoft.com/fr-fr/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info
            let mut info = SYSTEM_INFO::default();
            GetSystemInfo(&mut info);
            match info.Anonymous.Anonymous.wProcessorArchitecture {
                SystemInformation::PROCESSOR_ARCHITECTURE_ALPHA => Some("alpha".to_string()),
                SystemInformation::PROCESSOR_ARCHITECTURE_ALPHA64 => Some("alpha64".to_string()),
                SystemInformation::PROCESSOR_ARCHITECTURE_AMD64 => Some("x86_64".to_string()),
                SystemInformation::PROCESSOR_ARCHITECTURE_ARM => Some("arm".to_string()),
                SystemInformation::PROCESSOR_ARCHITECTURE_ARM32_ON_WIN64 => Some("arm".to_string()),
                SystemInformation::PROCESSOR_ARCHITECTURE_ARM64 => Some("arm64".to_string()),
                SystemInformation::PROCESSOR_ARCHITECTURE_IA32_ON_ARM64
                | SystemInformation::PROCESSOR_ARCHITECTURE_IA32_ON_WIN64 => {
                    Some("ia32".to_string())
                }
                SystemInformation::PROCESSOR_ARCHITECTURE_IA64 => Some("ia64".to_string()),
                SystemInformation::PROCESSOR_ARCHITECTURE_INTEL => Some("x86".to_string()),
                SystemInformation::PROCESSOR_ARCHITECTURE_MIPS => Some("mips".to_string()),
                SystemInformation::PROCESSOR_ARCHITECTURE_PPC => Some("powerpc".to_string()),
                _ => None,
            }
        }
    }

    pub(crate) fn physical_core_count() -> Option<usize> {
        get_physical_core_count()
    }

    pub(crate) fn open_files_limit() -> Option<usize> {
        // Apparently when using C run-time libraries, it's limited by _NHANDLE_.
        // It's a define:
        //
        // ```
        // #define IOINFO_L2E          6
        // #define IOINFO_ARRAY_ELTS   (1 << IOINFO_L2E)
        // #define IOINFO_ARRAYS       128
        // #define _NHANDLE_ (IOINFO_ARRAYS * IOINFO_ARRAY_ELTS)
        // ```
        //
        // So 128 * (1 << 6) = 8192
        Some(8192)
    }
}

pub(crate) fn is_proc_running(handle: HANDLE) -> bool {
    let mut exit_code = 0;
    unsafe { GetExitCodeProcess(handle, &mut exit_code) }.is_ok()
        && exit_code == STILL_ACTIVE.0 as u32
}

fn get_dns_hostname() -> Option<String> {
    let mut buffer_size = 0;
    // Running this first to get the buffer size since the DNS name can be longer than MAX_COMPUTERNAME_LENGTH
    // setting the `lpBuffer` to null will return the buffer size
    // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw
    unsafe {
        let _err = GetComputerNameExW(ComputerNamePhysicalDnsHostname, None, &mut buffer_size);

        // Setting the buffer with the new length
        let mut buffer = vec![0_u16; buffer_size as usize];

        // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ne-sysinfoapi-computer_name_format
        if GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            Some(PWSTR::from_raw(buffer.as_mut_ptr())),
            &mut buffer_size,
        )
        .is_ok()
        {
            if let Some(pos) = buffer.iter().position(|c| *c == 0) {
                buffer.resize(pos, 0);
            }

            return String::from_utf16(&buffer).ok();
        }
    }

    sysinfo_debug!("Failed to get computer hostname");
    None
}

fn add_english_counter(
    s: String,
    query: &mut super::cpu::Query,
    keys: &mut Option<KeyHandler>,
    counter_name: String,
) {
    let mut full = s.encode_utf16().collect::<Vec<_>>();
    full.push(0);
    if query.add_english_counter(&counter_name, full) {
        *keys = Some(KeyHandler::new(counter_name));
    }
}

fn get_now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|n| n.as_secs())
        .unwrap_or(0)
}

fn utf16_str<S: AsRef<OsStr> + ?Sized>(text: &S) -> Vec<u16> {
    OsStr::new(text)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>()
}

struct RegKey(HKEY);

impl RegKey {
    unsafe fn open(hkey: HKEY, path: &[u16]) -> Option<Self> {
        let mut new_hkey = Default::default();
        if unsafe {
            RegOpenKeyExW(
                hkey,
                PCWSTR::from_raw(path.as_ptr()),
                Some(0),
                KEY_READ,
                &mut new_hkey,
            )
        }
        .is_err()
        {
            return None;
        }
        Some(Self(new_hkey))
    }

    unsafe fn get_value(
        &self,
        field_name: &[u16],
        buf: &mut [u8],
        buf_len: &mut u32,
    ) -> windows::core::Result<()> {
        let mut buf_type = REG_NONE;

        unsafe {
            RegQueryValueExW(
                self.0,
                PCWSTR::from_raw(field_name.as_ptr()),
                None,
                Some(&mut buf_type),
                Some(buf.as_mut_ptr()),
                Some(buf_len),
            )
        }
        .ok()
    }
}

impl Drop for RegKey {
    fn drop(&mut self) {
        let _err = unsafe { RegCloseKey(self.0) };
    }
}

pub(crate) fn get_reg_string_value(hkey: HKEY, path: &str, field_name: &str) -> Option<String> {
    let c_path = utf16_str(path);
    let c_field_name = utf16_str(field_name);

    unsafe {
        let new_key = RegKey::open(hkey, &c_path)?;
        let mut buf_len: u32 = 2048;
        let mut buf: Vec<u8> = Vec::with_capacity(buf_len as usize);

        loop {
            match new_key.get_value(&c_field_name, &mut buf, &mut buf_len) {
                Ok(()) => break,
                Err(err) if err.code() == Foundation::ERROR_MORE_DATA.to_hresult() => {
                    // Needs to be updated for `Vec::reserve` to actually add additional capacity.
                    buf.set_len(buf.capacity());
                    buf.reserve(buf_len as _);
                }
                _ => return None,
            }
        }

        buf.set_len(buf_len as _);

        let words = std::slice::from_raw_parts(buf.as_ptr() as *const u16, buf.len() / 2);
        let mut s = String::from_utf16_lossy(words);
        while s.ends_with('\u{0}') {
            s.pop();
        }
        Some(s)
    }
}

pub(crate) fn get_reg_value_u32(hkey: HKEY, path: &str, field_name: &str) -> Option<[u8; 4]> {
    let c_path = utf16_str(path);
    let c_field_name = utf16_str(field_name);

    unsafe {
        let new_key = RegKey::open(hkey, &c_path)?;
        let mut buf_len: u32 = 4;
        let mut buf = [0u8; 4];

        new_key
            .get_value(&c_field_name, &mut buf, &mut buf_len)
            .map(|_| buf)
            .ok()
    }
}
