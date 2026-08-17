//! On Windows, both Tracy and Rust use the `dbghelp.dll` symbol helper to resolve symbols for stack traces.
//! `dbghelp.dll` is single threaded and requires synchronization to call any of its functions.
//!
//! The Rust standard library includes the `backtrace-rs` crate for capturing and resolving backtraces.
//! When both the standard library and the `backtrace-rs` crate are used in the same program
//! they need to synchronize their access to `dbghelp.dll`.
//! They use a shared named Windows mutex for that, which we will use as well.
//!
//! Users of Tracy (like this crate) can define the `TRACY_DBGHELP_LOCK` variable for synchronizing access to `dbghelp.dll`.
//! We set `TRACY_DBGHELP_LOCK=RustBacktraceMutex` in the build script.
//! Tracy will call [`RustBacktraceMutexInit`], [`RustBacktraceMutexLock`], and [`RustBacktraceMutexUnlock`].
//! In those functions a handle to the shared named mutex is created, the mutex is locked, and unlocked respectively.
//!
//! There is also an issue with initialization between Tracy and `backtrace-rs`.
//! In particular, the `SymInitialize` function should only be called once per process
//! and will return an error on subsequent calls.
//! Both Tracy and `backtrace-rs` ignore errors of the `SymInitialize` function,
//! so calling it multiple times is not an issue.

use std::sync::atomic::{AtomicPtr, Ordering};

// Use the `windows_targets` crate and define all the things we need ourselves to avoid a dependency on `windows`
#[allow(clippy::upper_case_acronyms)]
type BOOL = i32;
#[allow(clippy::upper_case_acronyms)]
type HANDLE = *mut core::ffi::c_void;
#[allow(clippy::upper_case_acronyms)]
type PWSTR = *mut u16;
#[allow(clippy::upper_case_acronyms)]
type PCSTR = *const u8;
#[allow(clippy::upper_case_acronyms)]
type PCWSTR = *const u16;
type WIN32_ERROR = u32;
#[repr(C)]
struct SECURITY_ATTRIBUTES {
    nLength: u32,
    lpSecurityDescriptor: *mut core::ffi::c_void,
    bInheritHandle: BOOL,
}

const FALSE: BOOL = 0i32;
const TRUE: BOOL = 1i32;
const INFINITE: u32 = u32::MAX;
const WAIT_FAILED: u32 = 0xFFFFFFFF;

windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> u32);
windows_targets::link!("kernel32.dll" "system" fn CreateMutexA(lpmutexattributes: *const SECURITY_ATTRIBUTES, binitialowner: BOOL, lpname: PCSTR) -> HANDLE);
windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle: HANDLE, dwmilliseconds: u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn ReleaseMutex(hmutex: HANDLE) -> BOOL);
windows_targets::link!("kernel32.dll" "system" fn lstrlenW(lpstring : PCWSTR) -> i32);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcess() -> HANDLE);

windows_targets::link!("dbghelp.dll" "system" fn SymInitializeW(hprocess: HANDLE, usersearchpath: PCWSTR, finvadeprocess: BOOL) -> BOOL);
windows_targets::link!("dbghelp.dll" "system" fn SymGetSearchPathW(hprocess: HANDLE, searchpatha: PWSTR, searchpathlength: u32) -> BOOL);
windows_targets::link!("dbghelp.dll" "system" fn SymSetSearchPathW(hprocess: HANDLE, searchpatha: PCWSTR) -> BOOL);
windows_targets::link!("dbghelp.dll" "system" fn EnumerateLoadedModulesW64(hprocess: HANDLE, enumloadedmodulescallback: Option<unsafe extern "system" fn(modulename: PCWSTR, modulebase: u64, modulesize: u32, usercontext: *const core::ffi::c_void) -> BOOL>, usercontext: *const core::ffi::c_void) -> BOOL);

/// Handle to the shared named Windows mutex that synchronizes access to the `dbghelp.dll` symbol helper,
/// with the standard library and `backtrace-rs`.
/// Gets initialized by [`RustBacktraceMutexInit`],
/// and because there is no cleanup function, the handle is leaked.
static RUST_BACKTRACE_MUTEX: AtomicPtr<core::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

#[no_mangle]
extern "C" fn RustBacktraceMutexInit() {
    unsafe {
        // The name is the same one that the standard library and `backtrace-rs` use
        let name = format!("Local\\RustBacktraceMutex{:08X}\0", GetCurrentProcessId());
        // Creates a named mutex that is shared with the standard library and `backtrace-rs`
        // to synchronize access to `dbghelp.dll` functions, which are single threaded.
        let mutex = CreateMutexA(std::ptr::null(), FALSE, name.as_ptr());
        assert!(!mutex.is_null());

        // The old value is ignored because this function is only called once,
        // and normally the handle to the mutex is leaked anyway.
        RUST_BACKTRACE_MUTEX.store(mutex, Ordering::Release);
    }

    // We initialize `dbghelp.dll` symbol handler before Tracy does,
    // and add the directory of all loaded modules to the symbol search path.
    // This matches the behavior of the standard library and `backtrace-rs`,
    // and ensures symbols for backtraces don't break when using this crate.
    // Note that changing the symbol search path doesn't affect modules that were already loaded.
    init_dbghelp();
}

fn init_dbghelp() {
    unsafe {
        RustBacktraceMutexLock();

        SymInitializeW(GetCurrentProcess(), std::ptr::null(), FALSE);

        let mut paths = vec![0; 1024];
        if SymGetSearchPathW(
            GetCurrentProcess(),
            paths.as_mut_ptr(),
            paths.len().try_into().unwrap(),
        ) == TRUE
        {
            paths.truncate(lstrlenW(paths.as_ptr()).try_into().unwrap());
        } else {
            // As a fallback, use the current directory as a search path if `SymGetSearchPathW` fails,
            // which can happen when the buffer wasn't big enough
            paths = vec!['.' as u16];
        }

        // add the directory of all loaded modules to the symbol search path
        if EnumerateLoadedModulesW64(
            GetCurrentProcess(),
            Some(loaded_modules_callback),
            (&mut paths as *mut Vec<u16>).cast(),
        ) == TRUE
        {
            paths.push(0); // add null terminator
            SymSetSearchPathW(GetCurrentProcess(), paths.as_ptr());
        }

        RustBacktraceMutexUnlock();
    }
}

unsafe extern "system" fn loaded_modules_callback(
    module_name: PCWSTR,
    _module_base: u64,
    _module_size: u32,
    user_context: *const core::ffi::c_void,
) -> BOOL {
    let path = unsafe {
        std::slice::from_raw_parts(module_name, lstrlenW(module_name).try_into().unwrap())
    };
    let Some(last_separator) = path.iter().rposition(|&c| c == '/' as _ || c == '\\' as _) else {
        return TRUE;
    };
    let dir = &path[..last_separator];

    let paths = unsafe { &mut *user_context.cast::<Vec<u16>>().cast_mut() };
    if paths.split(|&c| c == ';' as _).all(|slice| slice != dir) {
        paths.push(';' as _);
        paths.extend(dir);
    }

    TRUE // continue enumeration
}

#[no_mangle]
extern "C" fn RustBacktraceMutexLock() {
    unsafe {
        let mutex = RUST_BACKTRACE_MUTEX.load(Ordering::Acquire);
        if !mutex.is_null() {
            assert_ne!(
                WaitForSingleObject(mutex, INFINITE),
                WAIT_FAILED,
                "{}",
                GetLastError()
            );
        }
    }
}

#[no_mangle]
extern "C" fn RustBacktraceMutexUnlock() {
    unsafe {
        let mutex = RUST_BACKTRACE_MUTEX.load(Ordering::Acquire);
        if !mutex.is_null() {
            assert_ne!(ReleaseMutex(mutex), 0, "{}", GetLastError());
        }
    }
}
