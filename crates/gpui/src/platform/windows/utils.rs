use std::iter::once;

use collections::VecDeque;
use windows::Win32::{
    Foundation::{BOOL, HMODULE, HWND, LPARAM, RECT},
    Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
    System::LibraryLoader::{
        GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
        GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
    },
    UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWL_USERDATA},
};

use crate::{log_windows_error_with_message, MonitorHandle};

pub fn encode_wide(input: &str) -> Vec<u16> {
    input.encode_utf16().chain(once(0)).collect::<Vec<u16>>()
}

pub fn get_module_handle() -> HMODULE {
    unsafe {
        let mut h_module = std::mem::zeroed();
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            windows::core::w!("ZedModule"),
            &mut h_module,
        )
        .unwrap(); // should never fail

        return h_module;
    }
}

pub fn available_monitors() -> VecDeque<MonitorHandle> {
    let mut monitors: VecDeque<MonitorHandle> = VecDeque::new();
    unsafe {
        EnumDisplayMonitors(
            HDC::default(),
            None,
            Some(monitor_enum_proc),
            LPARAM(&mut monitors as *mut _ as _),
        );
    }
    monitors
}

/// Get the low-order word, the first arg can be signed or unsigned number,
/// the second arg must be i16 or u16
///
/// # examples
/// ```rust
/// let num: u32 = 0x00008001;
/// assert_eq!(loword!(num, u16), 32769);
/// assert_eq!(loword!(num, i16), -32767);
/// ```
#[macro_export]
macro_rules! loword {
    ($num:expr, $t:ty) => {
        ($num & 0xFFFF) as $t
    };
}

/// Get the high-order word, the first arg can be signed or unsigned number,
/// the second arg must be i16 or u16
///
/// # examples
/// ```rust
/// let num: u32 = 0x80010000;
/// assert_eq!(hiword!(num, u16), 32769);
/// assert_eq!(hiword!(num, i16), -32767);
/// ```
#[macro_export]
macro_rules! hiword {
    ($num:expr, $t:ty) => {
        (($num >> 16) & 0xFFFF) as $t
    };
}

#[inline]
pub unsafe fn set_windowdata<T>(handle: HWND, data: T) {
    let raw = Box::into_raw(Box::new(data));
    let ret = SetWindowLongPtrW(handle, GWL_USERDATA, raw as _);
    if ret == 0 {
        log_windows_error_with_message!(None);
        let _ = SetWindowLongPtrW(handle, GWL_USERDATA, raw as _);
    }
}

#[inline]
pub unsafe fn get_windowdata(handle: HWND) -> isize {
    GetWindowLongPtrW(handle, GWL_USERDATA)
}

pub fn log_windows_error(_e: &windows::core::Error) {
    log_windows_error_with_message!(None);
}

/// Log windows errors.
///
/// # examples
/// ```rust
/// log_windows_error_with_message!("Error");
/// log_windows_error_with_message!(None);
/// ```
#[macro_export]
macro_rules! log_windows_error_with_message {
    ($s: literal) => {{
        let caller = std::panic::Location::caller();
        log::error!(
            concat!($s, " at {}:{}: {}"),
            caller.file(),
            caller.line(),
            std::io::Error::last_os_error()
        );
    }};
    (None) => {{
        let caller = std::panic::Location::caller();
        log::error!(
            "Windows error at {}:{}: {}",
            caller.file(),
            caller.line(),
            std::io::Error::last_os_error()
        );
    }};
}

unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _place: *mut RECT,
    data: LPARAM,
) -> BOOL {
    let monitors = data.0 as *mut VecDeque<MonitorHandle>;
    unsafe { (*monitors).push_back(MonitorHandle::new(hmonitor)) };
    true.into() // continue enumeration
}
