// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::system::is_proc_running;
use crate::sys::utils::HandleWrapper;
use crate::windows::Sid;
use crate::{DiskUsage, Gid, Pid, ProcessRefreshKind, ProcessStatus, Signal, Uid};

use std::ffi::{OsStr, OsString};
use std::fmt;
#[cfg(feature = "debug")]
use std::io;
use std::mem::{size_of, zeroed, MaybeUninit};
use std::os::windows::ffi::OsStringExt;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process;
use std::ptr::null_mut;
use std::str;
use std::sync::{Arc, OnceLock};

use libc::c_void;
use ntapi::ntexapi::SYSTEM_PROCESS_INFORMATION;
use ntapi::ntrtl::RTL_USER_PROCESS_PARAMETERS;
use ntapi::ntwow64::{PEB32, RTL_USER_PROCESS_PARAMETERS32};
use windows::core::PCWSTR;
use windows::Wdk::System::SystemServices::RtlGetVersion;
use windows::Wdk::System::Threading::{
    NtQueryInformationProcess, ProcessBasicInformation, ProcessCommandLineInformation,
    ProcessWow64Information, PROCESSINFOCLASS,
};
use windows::Win32::Foundation::{
    LocalFree, ERROR_INSUFFICIENT_BUFFER, FILETIME, HANDLE, HINSTANCE, HLOCAL, MAX_PATH,
    STATUS_BUFFER_OVERFLOW, STATUS_BUFFER_TOO_SMALL, STATUS_INFO_LENGTH_MISMATCH, UNICODE_STRING,
};
use windows::Win32::Security::{GetTokenInformation, TokenUser, TOKEN_QUERY, TOKEN_USER};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapFree, VirtualQueryEx, HEAP_ZERO_MEMORY, MEMORY_BASIC_INFORMATION,
};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::RemoteDesktop::ProcessIdToSessionId;
use windows::Win32::System::SystemInformation::OSVERSIONINFOEXW;
use windows::Win32::System::Threading::{
    GetProcessIoCounters, GetProcessTimes, GetSystemTimes, OpenProcess, OpenProcessToken,
    CREATE_NO_WINDOW, IO_COUNTERS, PEB, PROCESS_BASIC_INFORMATION, PROCESS_QUERY_INFORMATION,
    PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_READ,
};
use windows::Win32::UI::Shell::CommandLineToArgvW;

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            ProcessStatus::Run => "Runnable",
            _ => "Unknown",
        })
    }
}

fn get_process_handler(pid: Pid) -> Option<HandleWrapper> {
    if pid.0 == 0 {
        return None;
    }
    let options = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ;

    HandleWrapper::new(unsafe { OpenProcess(options, false, pid.0 as u32).unwrap_or_default() })
        .or_else(|| {
            sysinfo_debug!(
                "OpenProcess failed, error: {:?}",
                io::Error::last_os_error()
            );
            HandleWrapper::new(unsafe {
                OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid.0 as u32)
                    .unwrap_or_default()
            })
        })
        .or_else(|| {
            sysinfo_debug!(
                "OpenProcess limited failed, error: {:?}",
                io::Error::last_os_error()
            );
            None
        })
}

unsafe fn get_process_user_id(process: &mut ProcessInner, refresh_kind: ProcessRefreshKind) {
    struct HeapWrap<T>(*mut T);

    impl<T> HeapWrap<T> {
        unsafe fn new(size: u32) -> Option<Self> {
            let ptr = HeapAlloc(GetProcessHeap().ok()?, HEAP_ZERO_MEMORY, size as _) as *mut T;
            if ptr.is_null() {
                sysinfo_debug!("HeapAlloc failed");
                None
            } else {
                Some(Self(ptr))
            }
        }
    }

    impl<T> Drop for HeapWrap<T> {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    if let Ok(heap) = GetProcessHeap() {
                        let _err = HeapFree(heap, Default::default(), Some(self.0.cast()));
                    }
                }
            }
        }
    }

    let handle = match &process.handle {
        // We get back the pointer so we don't need to clone the wrapping `Arc`.
        Some(handle) => ***handle,
        None => return,
    };

    if !refresh_kind
        .user()
        .needs_update(|| process.user_id.is_none())
    {
        return;
    }

    let mut token = Default::default();

    if OpenProcessToken(handle, TOKEN_QUERY, &mut token).is_err() {
        sysinfo_debug!("OpenProcessToken failed");
        return;
    }

    let token = match HandleWrapper::new(token) {
        Some(token) => token,
        None => return,
    };

    let mut size = 0;

    if let Err(err) = GetTokenInformation(*token, TokenUser, None, 0, &mut size) {
        if err.code() != ERROR_INSUFFICIENT_BUFFER.to_hresult() {
            sysinfo_debug!("GetTokenInformation failed, error: {:?}", err);
            return;
        }
    }

    let ptu: HeapWrap<TOKEN_USER> = match HeapWrap::new(size) {
        Some(ptu) => ptu,
        None => return,
    };

    if let Err(_err) = GetTokenInformation(*token, TokenUser, Some(ptu.0.cast()), size, &mut size) {
        sysinfo_debug!(
            "GetTokenInformation failed (returned {_err:?}), error: {:?}",
            io::Error::last_os_error()
        );
        return;
    }

    // We force this check to prevent overwritting a `Some` with a `None`.
    if let Some(uid) = Sid::from_psid((*ptu.0).User.Sid).map(Uid) {
        process.user_id = Some(uid);
    }
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for HandleWrapper {}
unsafe impl Sync for HandleWrapper {}

pub(crate) struct ProcessInner {
    name: OsString,
    cmd: Vec<OsString>,
    exe: Option<PathBuf>,
    pid: Pid,
    user_id: Option<Uid>,
    environ: Vec<OsString>,
    cwd: Option<PathBuf>,
    root: Option<PathBuf>,
    pub(crate) memory: u64,
    pub(crate) virtual_memory: u64,
    pub(crate) parent: Option<Pid>,
    status: ProcessStatus,
    handle: Option<Arc<HandleWrapper>>,
    cpu_calc_values: CPUsageCalculationValues,
    start_time: u64,
    pub(crate) run_time: u64,
    cpu_usage: f32,
    pub(crate) updated: bool,
    old_read_bytes: u64,
    old_written_bytes: u64,
    read_bytes: u64,
    written_bytes: u64,
}

struct CPUsageCalculationValues {
    old_process_sys_cpu: u64,
    old_process_user_cpu: u64,
    old_system_sys_cpu: u64,
    old_system_user_cpu: u64,
}

impl CPUsageCalculationValues {
    fn new() -> Self {
        CPUsageCalculationValues {
            old_process_sys_cpu: 0,
            old_process_user_cpu: 0,
            old_system_sys_cpu: 0,
            old_system_user_cpu: 0,
        }
    }
}

fn windows_8_1_or_newer() -> &'static bool {
    static WINDOWS_8_1_OR_NEWER: OnceLock<bool> = OnceLock::new();
    WINDOWS_8_1_OR_NEWER.get_or_init(|| unsafe {
        let mut version_info: OSVERSIONINFOEXW = MaybeUninit::zeroed().assume_init();

        version_info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOEXW>() as u32;
        if RtlGetVersion((&mut version_info as *mut OSVERSIONINFOEXW).cast()).is_err() {
            return true;
        }

        // Windows 8.1 is 6.3
        version_info.dwMajorVersion > 6
            || version_info.dwMajorVersion == 6 && version_info.dwMinorVersion >= 3
    })
}

#[cfg(feature = "debug")]
unsafe fn display_ntstatus_error(ntstatus: windows::core::HRESULT) {
    let code = ntstatus.0;
    let message = ntstatus.message();
    sysinfo_debug!(
        "Couldn't get process infos: NtQuerySystemInformation returned {}: {}",
        code,
        message
    );
}

unsafe fn get_exe(process_handler: &HandleWrapper) -> Option<PathBuf> {
    let mut exe_buf = [0u16; MAX_PATH as usize + 1];
    GetModuleFileNameExW(
        **process_handler,
        HINSTANCE::default(),
        exe_buf.as_mut_slice(),
    );

    Some(PathBuf::from(null_terminated_wchar_to_string(&exe_buf)))
}

impl ProcessInner {
    pub(crate) fn new(pid: Pid, parent: Option<Pid>, now: u64, name: OsString) -> Self {
        let (handle, start_time, run_time) = if let Some(handle) = get_process_handler(pid) {
            let (start_time, run_time) = get_start_and_run_time(*handle, now);
            (Some(Arc::new(handle)), start_time, run_time)
        } else {
            (None, 0, 0)
        };
        Self {
            handle,
            name,
            pid,
            parent,
            user_id: None,
            cmd: Vec::new(),
            environ: Vec::new(),
            exe: None,
            cwd: None,
            root: None,
            status: ProcessStatus::Run,
            memory: 0,
            virtual_memory: 0,
            cpu_usage: 0.,
            cpu_calc_values: CPUsageCalculationValues::new(),
            start_time,
            run_time,
            updated: true,
            old_read_bytes: 0,
            old_written_bytes: 0,
            read_bytes: 0,
            written_bytes: 0,
        }
    }

    pub(crate) fn update(
        &mut self,
        refresh_kind: crate::ProcessRefreshKind,
        nb_cpus: u64,
        now: u64,
        refresh_parent: bool,
        pi: &SYSTEM_PROCESS_INFORMATION,
    ) {
        if refresh_kind.cpu() {
            compute_cpu_usage(self, nb_cpus);
        }
        if refresh_kind.disk_usage() {
            update_disk_usage(self);
        }
        if refresh_kind.memory() {
            self.memory = pi.WorkingSetSize as _;
            self.virtual_memory = pi.VirtualSize as _;
        }
        unsafe {
            get_process_user_id(self, refresh_kind);
            get_process_params(self, refresh_kind, refresh_parent);
        }
        if refresh_kind.exe().needs_update(|| self.exe.is_none()) {
            unsafe {
                self.exe = match self.handle.as_ref() {
                    Some(handle) => get_exe(handle),
                    None => get_executable_path(self.pid),
                };
            }
        }
        self.run_time = now.saturating_sub(self.start_time());
        self.updated = true;
    }

    pub(crate) fn get_handle(&self) -> Option<HANDLE> {
        self.handle.as_ref().map(|h| ***h)
    }

    pub(crate) fn get_start_time(&self) -> Option<u64> {
        self.handle.as_ref().map(|handle| get_start_time(***handle))
    }

    pub(crate) fn kill_with(&self, signal: Signal) -> Option<bool> {
        crate::sys::system::convert_signal(signal)?;
        let mut kill = process::Command::new("taskkill.exe");
        kill.arg("/PID").arg(self.pid.to_string()).arg("/F");
        kill.creation_flags(CREATE_NO_WINDOW.0);
        match kill.output() {
            Ok(o) => Some(o.status.success()),
            Err(_) => Some(false),
        }
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
        None
    }

    pub(crate) fn group_id(&self) -> Option<Gid> {
        None
    }

    pub(crate) fn effective_group_id(&self) -> Option<Gid> {
        None
    }

    pub(crate) fn wait(&self) {
        if let Some(handle) = self.get_handle() {
            while is_proc_running(handle) {
                if get_start_time(handle) != self.start_time() {
                    // PID owner changed so the previous process was finished!
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        } else {
            // In this case, we can't do anything so we just return.
            sysinfo_debug!("can't wait on this process so returning");
        }
    }

    pub(crate) fn session_id(&self) -> Option<Pid> {
        unsafe {
            let mut out = 0;
            if ProcessIdToSessionId(self.pid.as_u32(), &mut out).is_ok() {
                return Some(Pid(out as _));
            }
            sysinfo_debug!(
                "ProcessIdToSessionId failed, error: {:?}",
                io::Error::last_os_error()
            );
            None
        }
    }
}

#[inline]
unsafe fn get_process_times(handle: HANDLE) -> u64 {
    let mut fstart: FILETIME = zeroed();
    let mut x = zeroed();

    let _err = GetProcessTimes(
        handle,
        &mut fstart as *mut FILETIME,
        &mut x as *mut FILETIME,
        &mut x as *mut FILETIME,
        &mut x as *mut FILETIME,
    );
    filetime_to_u64(fstart)
}

// On Windows, the root folder is always the current drive. So we get it from its `cwd`.
fn update_root(refresh_kind: ProcessRefreshKind, cwd: &Path, root: &mut Option<PathBuf>) {
    if !refresh_kind.root().needs_update(|| root.is_none()) {
        return;
    }
    if cwd.has_root() {
        let mut ancestors = cwd.ancestors().peekable();
        while let Some(path) = ancestors.next() {
            if ancestors.peek().is_none() {
                *root = Some(path.into());
                return;
            }
        }
    } else {
        *root = None;
    }
}

#[inline]
fn compute_start(process_times: u64) -> u64 {
    // 11_644_473_600 is the number of seconds between the Windows epoch (1601-01-01) and
    // the Linux epoch (1970-01-01).
    process_times / 10_000_000 - 11_644_473_600
}

fn get_start_and_run_time(handle: HANDLE, now: u64) -> (u64, u64) {
    unsafe {
        let process_times = get_process_times(handle);
        let start = compute_start(process_times);
        let run_time = check_sub(now, start);
        (start, run_time)
    }
}

#[inline]
pub(crate) fn get_start_time(handle: HANDLE) -> u64 {
    unsafe {
        let process_times = get_process_times(handle);
        compute_start(process_times)
    }
}

unsafe fn ph_query_process_variable_size(
    process_handle: HANDLE,
    process_information_class: PROCESSINFOCLASS,
) -> Option<Vec<u16>> {
    let mut return_length = MaybeUninit::<u32>::uninit();

    if let Err(err) = NtQueryInformationProcess(
        process_handle,
        process_information_class as _,
        null_mut(),
        0,
        return_length.as_mut_ptr() as *mut _,
    )
    .ok()
    {
        if ![
            STATUS_BUFFER_OVERFLOW.into(),
            STATUS_BUFFER_TOO_SMALL.into(),
            STATUS_INFO_LENGTH_MISMATCH.into(),
        ]
        .contains(&err.code())
        {
            return None;
        }
    }

    let mut return_length = return_length.assume_init();
    let buf_len = (return_length as usize) / 2;
    let mut buffer: Vec<u16> = Vec::with_capacity(buf_len + 1);
    if NtQueryInformationProcess(
        process_handle,
        process_information_class as _,
        buffer.as_mut_ptr() as *mut _,
        return_length,
        &mut return_length as *mut _,
    )
    .is_err()
    {
        return None;
    }
    buffer.set_len(buf_len);
    buffer.push(0);
    Some(buffer)
}

unsafe fn get_cmdline_from_buffer(buffer: PCWSTR) -> Vec<OsString> {
    // Get argc and argv from the command line
    let mut argc = MaybeUninit::<i32>::uninit();
    let argv_p = CommandLineToArgvW(buffer, argc.as_mut_ptr());
    if argv_p.is_null() {
        return Vec::new();
    }
    let argc = argc.assume_init();
    let argv = std::slice::from_raw_parts(argv_p, argc as usize);

    let mut res = Vec::new();
    for arg in argv {
        res.push(OsString::from_wide(arg.as_wide()));
    }

    let _err = LocalFree(HLOCAL(argv_p as _));

    res
}

unsafe fn get_region_size(handle: HANDLE, ptr: *const c_void) -> Result<usize, &'static str> {
    let mut meminfo = MaybeUninit::<MEMORY_BASIC_INFORMATION>::uninit();
    if VirtualQueryEx(
        handle,
        Some(ptr),
        meminfo.as_mut_ptr().cast(),
        size_of::<MEMORY_BASIC_INFORMATION>(),
    ) == 0
    {
        return Err("Unable to read process memory information");
    }
    let meminfo = meminfo.assume_init();
    Ok((meminfo.RegionSize as isize - ptr.offset_from(meminfo.BaseAddress)) as usize)
}

unsafe fn get_process_data(
    handle: HANDLE,
    ptr: *const c_void,
    size: usize,
) -> Result<Vec<u16>, &'static str> {
    let mut buffer: Vec<u16> = Vec::with_capacity(size / 2 + 1);
    let mut bytes_read = 0;

    if ReadProcessMemory(
        handle,
        ptr,
        buffer.as_mut_ptr().cast(),
        size,
        Some(&mut bytes_read),
    )
    .is_err()
    {
        return Err("Unable to read process data");
    }

    // Documentation states that the function fails if not all data is accessible.
    if bytes_read != size {
        return Err("ReadProcessMemory returned unexpected number of bytes read");
    }

    buffer.set_len(size / 2);
    buffer.push(0);

    Ok(buffer)
}

trait RtlUserProcessParameters {
    fn get_cmdline(&self, handle: HANDLE) -> Result<Vec<u16>, &'static str>;
    fn get_cwd(&self, handle: HANDLE) -> Result<Vec<u16>, &'static str>;
    fn get_environ(&self, handle: HANDLE) -> Result<Vec<u16>, &'static str>;
}

macro_rules! impl_RtlUserProcessParameters {
    ($t:ty) => {
        impl RtlUserProcessParameters for $t {
            fn get_cmdline(&self, handle: HANDLE) -> Result<Vec<u16>, &'static str> {
                let ptr = self.CommandLine.Buffer;
                let size = self.CommandLine.Length;
                unsafe { get_process_data(handle, ptr as _, size as _) }
            }
            fn get_cwd(&self, handle: HANDLE) -> Result<Vec<u16>, &'static str> {
                let ptr = self.CurrentDirectory.DosPath.Buffer;
                let size = self.CurrentDirectory.DosPath.Length;
                unsafe { get_process_data(handle, ptr as _, size as _) }
            }
            fn get_environ(&self, handle: HANDLE) -> Result<Vec<u16>, &'static str> {
                let ptr = self.Environment;
                unsafe {
                    let size = get_region_size(handle, ptr as _)?;
                    get_process_data(handle, ptr as _, size as _)
                }
            }
        }
    };
}

impl_RtlUserProcessParameters!(RTL_USER_PROCESS_PARAMETERS32);
impl_RtlUserProcessParameters!(RTL_USER_PROCESS_PARAMETERS);

fn has_anything_to_update(process: &ProcessInner, refresh_kind: ProcessRefreshKind) -> bool {
    refresh_kind.cmd().needs_update(|| process.cmd.is_empty())
        || refresh_kind
            .environ()
            .needs_update(|| process.environ.is_empty())
        || refresh_kind.cwd().needs_update(|| process.cwd.is_none())
        || refresh_kind.root().needs_update(|| process.root.is_none())
}

unsafe fn get_process_params(
    process: &mut ProcessInner,
    refresh_kind: ProcessRefreshKind,
    refresh_parent: bool,
) {
    let has_anything_to_update = has_anything_to_update(process, refresh_kind);
    if !refresh_parent && !has_anything_to_update {
        return;
    }

    let handle = match process.handle.as_ref().map(|handle| handle.0) {
        Some(h) => h,
        None => return,
    };

    // First check if target process is running in wow64 compatibility emulator
    let mut pwow32info = MaybeUninit::<*const c_void>::uninit();
    if NtQueryInformationProcess(
        handle,
        ProcessWow64Information,
        pwow32info.as_mut_ptr().cast(),
        size_of::<*const c_void>() as u32,
        null_mut(),
    )
    .is_err()
    {
        sysinfo_debug!("Unable to check WOW64 information about the process");
        return;
    }
    let pwow32info = pwow32info.assume_init();

    // Get parent and PEB64 from PROCESS_BASIC_INFORMATION
    // PEB64 will only be used if the target is 64 bit process
    let pinfo = if refresh_parent || pwow32info.is_null() {
        let mut pbasicinfo = MaybeUninit::<PROCESS_BASIC_INFORMATION>::uninit();
        if NtQueryInformationProcess(
            handle,
            ProcessBasicInformation,
            pbasicinfo.as_mut_ptr().cast(),
            size_of::<PROCESS_BASIC_INFORMATION>() as u32,
            null_mut(),
        )
        .is_err()
        {
            sysinfo_debug!("Unable to get basic process information");
            return;
        }
        let pinfo = pbasicinfo.assume_init();

        let ppid: usize = pinfo.InheritedFromUniqueProcessId as _;
        let parent = if ppid != 0 {
            Some(Pid(pinfo.InheritedFromUniqueProcessId as _))
        } else {
            None
        };
        process.parent = parent;

        if !has_anything_to_update {
            return;
        }

        Some(pinfo)
    } else {
        None
    };

    if pwow32info.is_null() {
        // target is a 64 bit process

        let Some(pinfo) = pinfo else { return };

        let mut peb = MaybeUninit::<PEB>::uninit();
        if ReadProcessMemory(
            handle,
            pinfo.PebBaseAddress.cast(),
            peb.as_mut_ptr().cast(),
            size_of::<PEB>(),
            None,
        )
        .is_err()
        {
            sysinfo_debug!("Unable to read process PEB");
            return;
        }

        let peb = peb.assume_init();

        let mut proc_params = MaybeUninit::<RTL_USER_PROCESS_PARAMETERS>::uninit();
        if ReadProcessMemory(
            handle,
            peb.ProcessParameters.cast(),
            proc_params.as_mut_ptr().cast(),
            size_of::<RTL_USER_PROCESS_PARAMETERS>(),
            None,
        )
        .is_err()
        {
            sysinfo_debug!("Unable to read process parameters");
            return;
        }

        let proc_params = proc_params.assume_init();
        get_cmd_line(&proc_params, handle, refresh_kind, &mut process.cmd);
        get_proc_env(&proc_params, handle, refresh_kind, &mut process.environ);
        get_cwd_and_root(
            &proc_params,
            handle,
            refresh_kind,
            &mut process.cwd,
            &mut process.root,
        );
    }
    // target is a 32 bit process in wow64 mode

    if !has_anything_to_update {
        return;
    }

    let mut peb32 = MaybeUninit::<PEB32>::uninit();
    if ReadProcessMemory(
        handle,
        pwow32info,
        peb32.as_mut_ptr().cast(),
        size_of::<PEB32>(),
        None,
    )
    .is_err()
    {
        sysinfo_debug!("Unable to read PEB32");
        return;
    }
    let peb32 = peb32.assume_init();

    let mut proc_params = MaybeUninit::<RTL_USER_PROCESS_PARAMETERS32>::uninit();
    if ReadProcessMemory(
        handle,
        peb32.ProcessParameters as *mut _,
        proc_params.as_mut_ptr().cast(),
        size_of::<RTL_USER_PROCESS_PARAMETERS32>(),
        None,
    )
    .is_err()
    {
        sysinfo_debug!("Unable to read 32 bit process parameters");
        return;
    }
    let proc_params = proc_params.assume_init();
    get_cmd_line(&proc_params, handle, refresh_kind, &mut process.cmd);
    get_proc_env(&proc_params, handle, refresh_kind, &mut process.environ);
    get_cwd_and_root(
        &proc_params,
        handle,
        refresh_kind,
        &mut process.cwd,
        &mut process.root,
    );
}

fn get_cwd_and_root<T: RtlUserProcessParameters>(
    params: &T,
    handle: HANDLE,
    refresh_kind: ProcessRefreshKind,
    cwd: &mut Option<PathBuf>,
    root: &mut Option<PathBuf>,
) {
    let cwd_needs_update = refresh_kind.cwd().needs_update(|| cwd.is_none());
    let root_needs_update = refresh_kind.root().needs_update(|| root.is_none());
    if !cwd_needs_update && !root_needs_update {
        return;
    }
    match params.get_cwd(handle) {
        Ok(buffer) => unsafe {
            let tmp_cwd = PathBuf::from(null_terminated_wchar_to_string(buffer.as_slice()));
            // Should always be called after we refreshed `cwd`.
            update_root(refresh_kind, &tmp_cwd, root);
            if cwd_needs_update {
                *cwd = Some(tmp_cwd);
            }
        },
        Err(_e) => {
            sysinfo_debug!("get_cwd_and_root failed to get data: {:?}", _e);
            *cwd = None;
        }
    }
}

unsafe fn null_terminated_wchar_to_string(slice: &[u16]) -> OsString {
    match slice.iter().position(|&x| x == 0) {
        Some(pos) => OsString::from_wide(&slice[..pos]),
        None => OsString::from_wide(slice),
    }
}

fn get_cmd_line_old<T: RtlUserProcessParameters>(params: &T, handle: HANDLE) -> Vec<OsString> {
    match params.get_cmdline(handle) {
        Ok(buffer) => unsafe { get_cmdline_from_buffer(PCWSTR::from_raw(buffer.as_ptr())) },
        Err(_e) => {
            sysinfo_debug!("get_cmd_line_old failed to get data: {}", _e);
            Vec::new()
        }
    }
}

#[allow(clippy::cast_ptr_alignment)]
fn get_cmd_line_new(handle: HANDLE) -> Vec<OsString> {
    unsafe {
        if let Some(buffer) = ph_query_process_variable_size(handle, ProcessCommandLineInformation)
        {
            let buffer = (*(buffer.as_ptr() as *const UNICODE_STRING)).Buffer;

            get_cmdline_from_buffer(PCWSTR::from_raw(buffer.as_ptr()))
        } else {
            vec![]
        }
    }
}

fn get_cmd_line<T: RtlUserProcessParameters>(
    params: &T,
    handle: HANDLE,
    refresh_kind: ProcessRefreshKind,
    cmd_line: &mut Vec<OsString>,
) {
    if !refresh_kind.cmd().needs_update(|| cmd_line.is_empty()) {
        return;
    }
    if *windows_8_1_or_newer() {
        *cmd_line = get_cmd_line_new(handle);
    } else {
        *cmd_line = get_cmd_line_old(params, handle);
    }
}

fn get_proc_env<T: RtlUserProcessParameters>(
    params: &T,
    handle: HANDLE,
    refresh_kind: ProcessRefreshKind,
    environ: &mut Vec<OsString>,
) {
    if !refresh_kind.environ().needs_update(|| environ.is_empty()) {
        return;
    }
    match params.get_environ(handle) {
        Ok(buffer) => {
            let equals = "=".encode_utf16().next().unwrap();
            let raw_env = buffer;
            environ.clear();
            let mut begin = 0;
            while let Some(offset) = raw_env[begin..].iter().position(|&c| c == 0) {
                let end = begin + offset;
                if raw_env[begin..end].iter().any(|&c| c == equals) {
                    environ.push(OsString::from_wide(&raw_env[begin..end]));
                    begin = end + 1;
                } else {
                    break;
                }
            }
        }
        Err(_e) => {
            sysinfo_debug!("get_proc_env failed to get data: {}", _e);
            *environ = Vec::new();
        }
    }
}

pub(crate) fn get_executable_path(_pid: Pid) -> Option<PathBuf> {
    /*let where_req = format!("ProcessId={}", pid);

    if let Some(ret) = run_wmi(&["process", "where", &where_req, "get", "ExecutablePath"]) {
        for line in ret.lines() {
            if line.is_empty() || line == "ExecutablePath" {
                continue
            }
            return line.to_owned();
        }
    }*/
    None
}

#[inline]
fn check_sub(a: u64, b: u64) -> u64 {
    if a < b {
        a
    } else {
        a - b
    }
}

/// Before changing this function, you must consider the following:
/// <https://github.com/GuillaumeGomez/sysinfo/issues/459>
pub(crate) fn compute_cpu_usage(p: &mut ProcessInner, nb_cpus: u64) {
    unsafe {
        let mut ftime: FILETIME = zeroed();
        let mut fsys: FILETIME = zeroed();
        let mut fuser: FILETIME = zeroed();
        let mut fglobal_idle_time: FILETIME = zeroed();
        let mut fglobal_kernel_time: FILETIME = zeroed(); // notice that it includes idle time
        let mut fglobal_user_time: FILETIME = zeroed();

        if let Some(handle) = p.get_handle() {
            let _err = GetProcessTimes(handle, &mut ftime, &mut ftime, &mut fsys, &mut fuser);
        }
        // FIXME: should these values be stored in one place to make use of
        // `MINIMUM_CPU_UPDATE_INTERVAL`?
        let _err = GetSystemTimes(
            Some(&mut fglobal_idle_time),
            Some(&mut fglobal_kernel_time),
            Some(&mut fglobal_user_time),
        );

        let sys = filetime_to_u64(fsys);
        let user = filetime_to_u64(fuser);
        let global_kernel_time = filetime_to_u64(fglobal_kernel_time);
        let global_user_time = filetime_to_u64(fglobal_user_time);

        let delta_global_kernel_time =
            check_sub(global_kernel_time, p.cpu_calc_values.old_system_sys_cpu);
        let delta_global_user_time =
            check_sub(global_user_time, p.cpu_calc_values.old_system_user_cpu);
        let delta_user_time = check_sub(user, p.cpu_calc_values.old_process_user_cpu);
        let delta_sys_time = check_sub(sys, p.cpu_calc_values.old_process_sys_cpu);

        p.cpu_calc_values.old_process_user_cpu = user;
        p.cpu_calc_values.old_process_sys_cpu = sys;
        p.cpu_calc_values.old_system_user_cpu = global_user_time;
        p.cpu_calc_values.old_system_sys_cpu = global_kernel_time;

        let denominator = delta_global_user_time.saturating_add(delta_global_kernel_time) as f32;

        if denominator < 0.00001 {
            p.cpu_usage = 0.;
            return;
        }

        p.cpu_usage = 100.0
            * (delta_user_time.saturating_add(delta_sys_time) as f32 / denominator)
            * nb_cpus as f32;
    }
}

pub(crate) fn update_disk_usage(p: &mut ProcessInner) {
    let mut counters = MaybeUninit::<IO_COUNTERS>::uninit();

    if let Some(handle) = p.get_handle() {
        unsafe {
            if GetProcessIoCounters(handle, counters.as_mut_ptr()).is_err() {
                sysinfo_debug!("GetProcessIoCounters call failed on process {}", p.pid());
            } else {
                let counters = counters.assume_init();
                p.old_read_bytes = p.read_bytes;
                p.old_written_bytes = p.written_bytes;
                p.read_bytes = counters.ReadTransferCount;
                p.written_bytes = counters.WriteTransferCount;
            }
        }
    }
}

#[inline(always)]
const fn filetime_to_u64(ft: FILETIME) -> u64 {
    (ft.dwHighDateTime as u64) << 32 | (ft.dwLowDateTime as u64)
}
