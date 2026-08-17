// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Disks, Networks, Pid, Process, ProcessesToUpdate, System};
use libc::{self, c_char, c_float, c_uint, c_void, size_t};
use std::borrow::BorrowMut;
use std::ffi::CString;

/// on windows, libc has not include pid_t.
#[cfg(target_os = "windows")]
pub type PID = usize;

/// other platforms, use libc::pid_t
#[cfg(not(target_os = "windows"))]
pub type PID = libc::pid_t;

/// Equivalent of [`System`][crate::System] struct.
pub type CSystem = *mut c_void;
/// Equivalent of [`Process`][crate::Process] struct.
pub type CProcess = *const c_void;
/// C string returned from `CString::into_raw`.
pub type RString = *const c_char;
/// Callback used by [`processes`][crate::System#method.processes].
pub type ProcessLoop = extern "C" fn(pid: PID, process: CProcess, data: *mut c_void) -> bool;
/// Callback used by [`tasks`][crate::Process#method.tasks].
pub type ProcessPidLoop = extern "C" fn(pid: PID, data: *mut c_void) -> bool;
/// Equivalent of [`Networks`][crate::Networks] struct.
pub type CNetworks = *mut c_void;
/// Equivalent of [`Disks`][crate::Disks] struct.
pub type CDisks = *mut c_void;

/// Equivalent of [`System::new()`][crate::System#method.new].
#[no_mangle]
pub extern "C" fn sysinfo_init() -> CSystem {
    let system = Box::new(System::new());
    Box::into_raw(system) as CSystem
}

/// Equivalent of `System::drop()`. Important in C to cleanup memory.
#[no_mangle]
pub extern "C" fn sysinfo_destroy(system: CSystem) {
    assert!(!system.is_null());
    unsafe {
        drop(Box::from_raw(system as *mut System));
    }
}

/// Equivalent of [`System::refresh_memory()`][crate::System#method.refresh_memory].
#[no_mangle]
pub extern "C" fn sysinfo_refresh_memory(system: CSystem) {
    assert!(!system.is_null());
    unsafe {
        let mut system: Box<System> = Box::from_raw(system as *mut System);
        {
            let system: &mut System = system.borrow_mut();
            system.refresh_memory();
        }
        Box::into_raw(system);
    }
}

/// Equivalent of [`System::refresh_cpu_usage()`][crate::System#method.refresh_cpu_usage].
#[no_mangle]
pub extern "C" fn sysinfo_refresh_cpu(system: CSystem) {
    assert!(!system.is_null());
    unsafe {
        let mut system: Box<System> = Box::from_raw(system as *mut System);
        {
            let system: &mut System = system.borrow_mut();
            system.refresh_cpu_usage();
        }
        Box::into_raw(system);
    }
}

/// Equivalent of [`System::refresh_all()`][crate::System#method.refresh_all].
#[no_mangle]
pub extern "C" fn sysinfo_refresh_all(system: CSystem) {
    assert!(!system.is_null());
    unsafe {
        let mut system: Box<System> = Box::from_raw(system as *mut System);
        {
            let system: &mut System = system.borrow_mut();
            system.refresh_all();
        }
        Box::into_raw(system);
    }
}

/// Equivalent of [`System::refresh_processes(ProcessesToUpdate::All)`].
///
/// [`System::refresh_processes(ProcessesToUpdate::All)`]: crate::System#method.refresh_processes
#[no_mangle]
pub extern "C" fn sysinfo_refresh_processes(system: CSystem) {
    assert!(!system.is_null());
    unsafe {
        let mut system: Box<System> = Box::from_raw(system as *mut System);
        {
            let system: &mut System = system.borrow_mut();
            system.refresh_processes(ProcessesToUpdate::All);
        }
        Box::into_raw(system);
    }
}

/// Equivalent of [`System::refresh_processes(ProcessesToUpdate::Some(pid))`].
///
/// [`System::refresh_processes(ProcessesToUpdate::Some(pid))`]: crate::System#method.refresh_processes
#[no_mangle]
pub extern "C" fn sysinfo_refresh_process(system: CSystem, pid: PID) {
    assert!(!system.is_null());
    unsafe {
        let mut system: Box<System> = Box::from_raw(system as *mut System);
        {
            let system: &mut System = system.borrow_mut();
            system.refresh_processes(ProcessesToUpdate::Some(&[Pid::from_u32(pid as _)]));
        }
        Box::into_raw(system);
    }
}

/// Equivalent of [`Disks::new()`][crate::Disks#method.new].
#[no_mangle]
pub extern "C" fn sysinfo_disks_init() -> CDisks {
    let disks = Box::new(Disks::new());
    Box::into_raw(disks) as CDisks
}

/// Equivalent of `Disks::drop()`. Important in C to cleanup memory.
#[no_mangle]
pub extern "C" fn sysinfo_disks_destroy(disks: CDisks) {
    assert!(!disks.is_null());
    unsafe {
        drop(Box::from_raw(disks as *mut Disks));
    }
}

/// Equivalent of [`Disks::refresh()`][crate::Disks#method.refresh].
#[no_mangle]
pub extern "C" fn sysinfo_disks_refresh(disks: CDisks) {
    assert!(!disks.is_null());
    unsafe {
        let mut disks: Box<Disks> = Box::from_raw(disks as *mut Disks);
        {
            let disks: &mut Disks = disks.borrow_mut();
            disks.refresh();
        }
        Box::into_raw(disks);
    }
}

/// Equivalent of [`Disks::refresh_list()`][crate::Disks#method.refresh_list].
#[no_mangle]
pub extern "C" fn sysinfo_disks_refresh_list(disks: CDisks) {
    assert!(!disks.is_null());
    unsafe {
        let mut disks: Box<Disks> = Box::from_raw(disks as *mut Disks);
        {
            let disks: &mut Disks = disks.borrow_mut();
            disks.refresh_list();
        }
        Box::into_raw(disks);
    }
}

/// Equivalent of [`System::total_memory()`][crate::System#method.total_memory].
#[no_mangle]
pub extern "C" fn sysinfo_total_memory(system: CSystem) -> size_t {
    assert!(!system.is_null());
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        let ret = system.total_memory() as size_t;
        Box::into_raw(system);
        ret
    }
}

/// Equivalent of [`System::free_memory()`][crate::System#method.free_memory].
#[no_mangle]
pub extern "C" fn sysinfo_free_memory(system: CSystem) -> size_t {
    assert!(!system.is_null());
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        let ret = system.free_memory() as size_t;
        Box::into_raw(system);
        ret
    }
}

/// Equivalent of [`System::used_memory()`][crate::System#method.used_memory].
#[no_mangle]
pub extern "C" fn sysinfo_used_memory(system: CSystem) -> size_t {
    assert!(!system.is_null());
    let system: Box<System> = unsafe { Box::from_raw(system as *mut System) };
    let ret = system.used_memory() as size_t;
    Box::into_raw(system);
    ret
}

/// Equivalent of [`System::total_swap()`][crate::System#method.total_swap].
#[no_mangle]
pub extern "C" fn sysinfo_total_swap(system: CSystem) -> size_t {
    assert!(!system.is_null());
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        let ret = system.total_swap() as size_t;
        Box::into_raw(system);
        ret
    }
}

/// Equivalent of [`System::free_swap()`][crate::System#method.free_swap].
#[no_mangle]
pub extern "C" fn sysinfo_free_swap(system: CSystem) -> size_t {
    assert!(!system.is_null());
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        let ret = system.free_swap() as size_t;
        Box::into_raw(system);
        ret
    }
}

/// Equivalent of [`System::used_swap()`][crate::System#method.used_swap].
#[no_mangle]
pub extern "C" fn sysinfo_used_swap(system: CSystem) -> size_t {
    assert!(!system.is_null());
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        let ret = system.used_swap() as size_t;
        Box::into_raw(system);
        ret
    }
}

/// Equivalent of [`Networks::new()`][crate::Networks#method.new].
#[no_mangle]
pub extern "C" fn sysinfo_networks_init() -> CNetworks {
    let networks = Box::new(Networks::new());
    Box::into_raw(networks) as CNetworks
}

/// Equivalent of `Networks::drop()`. Important in C to cleanup memory.
#[no_mangle]
pub extern "C" fn sysinfo_networks_destroy(networks: CNetworks) {
    assert!(!networks.is_null());
    unsafe {
        drop(Box::from_raw(networks as *mut Networks));
    }
}

/// Equivalent of [`Networks::refresh_list()`][crate::Networks#method.refresh_list].
#[no_mangle]
pub extern "C" fn sysinfo_networks_refresh_list(networks: CNetworks) {
    assert!(!networks.is_null());
    unsafe {
        let mut networks: Box<Networks> = Box::from_raw(networks as *mut Networks);
        {
            let networks: &mut Networks = networks.borrow_mut();
            networks.refresh_list();
        }
        Box::into_raw(networks);
    }
}

/// Equivalent of [`Networks::refresh()`][crate::Networks#method.refresh].
#[no_mangle]
pub extern "C" fn sysinfo_networks_refresh(networks: CNetworks) {
    assert!(!networks.is_null());
    unsafe {
        let mut networks: Box<Networks> = Box::from_raw(networks as *mut Networks);
        {
            let networks: &mut Networks = networks.borrow_mut();
            networks.refresh();
        }
        Box::into_raw(networks);
    }
}

/// Equivalent of
/// `system::networks().iter().fold(0, |acc, (_, data)| acc + data.received() as size_t)`.
#[no_mangle]
pub extern "C" fn sysinfo_networks_received(networks: CNetworks) -> size_t {
    assert!(!networks.is_null());
    unsafe {
        let networks: Box<Networks> = Box::from_raw(networks as *mut Networks);
        let ret = networks.iter().fold(0, |acc: size_t, (_, data)| {
            acc.saturating_add(data.received() as size_t)
        });
        Box::into_raw(networks);
        ret
    }
}

/// Equivalent of
/// `system::networks().iter().fold(0, |acc, (_, data)| acc + data.transmitted() as size_t)`.
#[no_mangle]
pub extern "C" fn sysinfo_networks_transmitted(networks: CNetworks) -> size_t {
    assert!(!networks.is_null());
    unsafe {
        let networks: Box<Networks> = Box::from_raw(networks as *mut Networks);
        let ret = networks.iter().fold(0, |acc: size_t, (_, data)| {
            acc.saturating_add(data.transmitted() as size_t)
        });
        Box::into_raw(networks);
        ret
    }
}

/// Equivalent of [`System::cpus_usage()`][crate::System#method.cpus_usage].
///
/// * `length` will contain the number of CPU usage added into `procs`.
/// * `procs` will be allocated if it's null and will contain of CPU usage.
#[no_mangle]
pub extern "C" fn sysinfo_cpus_usage(
    system: CSystem,
    length: *mut c_uint,
    procs: *mut *mut c_float,
) {
    assert!(!system.is_null());
    if procs.is_null() || length.is_null() {
        return;
    }
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        {
            let cpus = system.cpus();
            if (*procs).is_null() {
                (*procs) =
                    libc::malloc(::std::mem::size_of::<c_float>() * cpus.len()) as *mut c_float;
            }
            for (pos, cpu) in cpus.iter().skip(1).enumerate() {
                (*(*procs).offset(pos as isize)) = cpu.cpu_usage();
            }
            *length = cpus.len() as c_uint - 1;
        }
        Box::into_raw(system);
    }
}

/// Equivalent of [`System::processes()`][crate::System#method.processes]. Returns an
/// array ended by a null pointer. Must be freed.
///
/// # ⚠️ WARNING ⚠️
///
/// While having this method returned processes, you should *never* call any refresh method!
#[no_mangle]
pub extern "C" fn sysinfo_processes(
    system: CSystem,
    fn_pointer: Option<ProcessLoop>,
    data: *mut c_void,
) -> size_t {
    assert!(!system.is_null());
    if let Some(fn_pointer) = fn_pointer {
        unsafe {
            let system: Box<System> = Box::from_raw(system as *mut System);
            let len = {
                let entries = system.processes();
                for (pid, process) in entries {
                    if !fn_pointer(pid.0 as _, process as *const Process as CProcess, data) {
                        break;
                    }
                }
                entries.len() as size_t
            };
            Box::into_raw(system);
            len
        }
    } else {
        0
    }
}

/// Equivalent of [`System::process()`][crate::System#method.process].
///
/// # ⚠️ WARNING ⚠️
///
/// While having this method returned process, you should *never* call any
/// refresh method!
#[no_mangle]
pub extern "C" fn sysinfo_process_by_pid(system: CSystem, pid: PID) -> CProcess {
    assert!(!system.is_null());
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        let ret = if let Some(process) = system.process(Pid(pid as _)) {
            process as *const Process as CProcess
        } else {
            std::ptr::null()
        };
        Box::into_raw(system);
        ret
    }
}

/// Equivalent of iterating over [`Process::tasks()`][crate::Process#method.tasks].
///
/// # ⚠️ WARNING ⚠️
///
/// While having this method processes, you should *never* call any refresh method!
#[no_mangle]
pub extern "C" fn sysinfo_process_tasks(
    process: CProcess,
    fn_pointer: Option<ProcessPidLoop>,
    data: *mut c_void,
) -> size_t {
    assert!(!process.is_null());
    if let Some(fn_pointer) = fn_pointer {
        unsafe {
            let process = process as *const Process;
            if let Some(tasks) = (*process).tasks() {
                for pid in tasks {
                    if !fn_pointer(pid.0 as _, data) {
                        break;
                    }
                }
                tasks.len() as size_t
            } else {
                0
            }
        }
    } else {
        0
    }
}

/// Equivalent of [`Process::pid()`][crate::Process#method.pid].
#[no_mangle]
pub extern "C" fn sysinfo_process_pid(process: CProcess) -> PID {
    assert!(!process.is_null());
    let process = process as *const Process;
    unsafe { (*process).pid().0 as _ }
}

/// Equivalent of [`Process::parent()`][crate::Process#method.parent].
///
/// In case there is no known parent, it returns `0`.
#[no_mangle]
pub extern "C" fn sysinfo_process_parent_pid(process: CProcess) -> PID {
    assert!(!process.is_null());
    let process = process as *const Process;
    unsafe { (*process).parent().unwrap_or(Pid(0)).0 as _ }
}

/// Equivalent of [`Process::cpu_usage()`][crate::Process#method.cpu_usage].
#[no_mangle]
pub extern "C" fn sysinfo_process_cpu_usage(process: CProcess) -> c_float {
    assert!(!process.is_null());
    let process = process as *const Process;
    unsafe { (*process).cpu_usage() }
}

/// Equivalent of [`Process::memory()`][crate::Process#method.memory].
#[no_mangle]
pub extern "C" fn sysinfo_process_memory(process: CProcess) -> size_t {
    assert!(!process.is_null());
    let process = process as *const Process;
    unsafe { (*process).memory() as usize }
}

/// Equivalent of [`Process::virtual_memory()`][crate::Process#method.virtual_memory].
#[no_mangle]
pub extern "C" fn sysinfo_process_virtual_memory(process: CProcess) -> size_t {
    assert!(!process.is_null());
    let process = process as *const Process;
    unsafe { (*process).virtual_memory() as usize }
}

/// Equivalent of [`Process::exe()`][crate::Process#method.exe].
#[no_mangle]
pub extern "C" fn sysinfo_process_executable_path(process: CProcess) -> RString {
    assert!(!process.is_null());
    let process = process as *const Process;
    unsafe {
        if let Some(p) = (*process).exe().and_then(|exe| exe.to_str()) {
            if let Ok(c) = CString::new(p) {
                return c.into_raw() as _;
            }
        }
        std::ptr::null()
    }
}

/// Equivalent of [`Process::root()`][crate::Process#method.root].
#[no_mangle]
pub extern "C" fn sysinfo_process_root_directory(process: CProcess) -> RString {
    assert!(!process.is_null());
    let process = process as *const Process;
    unsafe {
        if let Some(p) = (*process).root().and_then(|root| root.to_str()) {
            if let Ok(c) = CString::new(p) {
                return c.into_raw() as _;
            }
        }
        std::ptr::null()
    }
}

/// Equivalent of [`Process::cwd()`][crate::Process#method.cwd].
#[no_mangle]
pub extern "C" fn sysinfo_process_current_directory(process: CProcess) -> RString {
    assert!(!process.is_null());
    let process = process as *const Process;
    unsafe {
        if let Some(p) = (*process).cwd().and_then(|cwd| cwd.to_str()) {
            if let Ok(c) = CString::new(p) {
                return c.into_raw() as _;
            }
        }
        std::ptr::null()
    }
}

/// Frees a C string created with `CString::into_raw()`.
#[no_mangle]
pub extern "C" fn sysinfo_rstring_free(s: RString) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s as usize as *mut _);
        }
    }
}

/// Equivalent of [`cpu::vendor_id()`].
#[no_mangle]
pub extern "C" fn sysinfo_cpu_vendor_id(system: CSystem) -> RString {
    assert!(!system.is_null());
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        let c_string = if let Some(c) = system
            .cpus()
            .first()
            .and_then(|cpu| CString::new(cpu.vendor_id()).ok())
        {
            c.into_raw() as RString
        } else {
            std::ptr::null()
        };
        Box::into_raw(system);
        c_string
    }
}

/// Equivalent of [`cpu::brand()`].
#[no_mangle]
pub extern "C" fn sysinfo_cpu_brand(system: CSystem) -> RString {
    assert!(!system.is_null());
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        let c_string = if let Some(c) = system
            .cpus()
            .first()
            .and_then(|cpu| CString::new(cpu.brand()).ok())
        {
            c.into_raw() as RString
        } else {
            std::ptr::null()
        };
        Box::into_raw(system);
        c_string
    }
}

/// Equivalent of [`system::physical_core_count()`].
#[no_mangle]
pub extern "C" fn sysinfo_cpu_physical_cores(system: CSystem) -> u32 {
    assert!(!system.is_null());
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        let count = system.physical_core_count().unwrap_or(0);
        Box::into_raw(system);
        count as u32
    }
}

/// Equivalent of [`cpu::frequency()`].
#[no_mangle]
pub extern "C" fn sysinfo_cpu_frequency(system: CSystem) -> u64 {
    assert!(!system.is_null());
    unsafe {
        let system: Box<System> = Box::from_raw(system as *mut System);
        let freq = system
            .cpus()
            .first()
            .map(|cpu| cpu.frequency())
            .unwrap_or(0);
        Box::into_raw(system);
        freq
    }
}

/// Equivalent of [`System::name()`][crate::System#method.name].
#[no_mangle]
pub extern "C" fn sysinfo_system_name() -> RString {
    if let Some(c) = System::name().and_then(|p| CString::new(p).ok()) {
        c.into_raw() as _
    } else {
        std::ptr::null()
    }
}

/// Equivalent of [`System::version()`][crate::System#method.version].
#[no_mangle]
pub extern "C" fn sysinfo_system_version() -> RString {
    if let Some(c) = System::os_version().and_then(|c| CString::new(c).ok()) {
        c.into_raw() as _
    } else {
        std::ptr::null()
    }
}

/// Equivalent of [`System::kernel_version()`][crate::System#method.kernel_version].
#[no_mangle]
pub extern "C" fn sysinfo_system_kernel_version() -> RString {
    if let Some(c) = System::kernel_version().and_then(|c| CString::new(c).ok()) {
        c.into_raw() as _
    } else {
        std::ptr::null()
    }
}

/// Equivalent of [`System::host_name()`][crate::System#method.host_name].
#[no_mangle]
pub extern "C" fn sysinfo_system_host_name() -> RString {
    if let Some(c) = System::host_name().and_then(|c| CString::new(c).ok()) {
        c.into_raw() as _
    } else {
        std::ptr::null()
    }
}

/// Equivalent of [`System::long_os_version()`][crate::System#method.long_os_version].
#[no_mangle]
pub extern "C" fn sysinfo_system_long_version() -> RString {
    if let Some(c) = System::long_os_version().and_then(|c| CString::new(c).ok()) {
        c.into_raw() as _
    } else {
        std::ptr::null()
    }
}
