use crate::decl::*;
use crate::kernel::ffi_types::*;

pub(in crate::user) extern "system" fn func_enum_thread_wnd<F>(
	hwnd: HWND,
	lparam: isize,
) -> BOOL
	where F: FnMut(HWND) -> bool,
{
	let func = unsafe { &mut *(lparam as *mut F) };
	func(hwnd) as _
}

pub(in crate::user) extern "system" fn func_enum_windows<F>(
	hwnd: HWND,
	lparam: isize,
) -> BOOL
	where F: FnMut(HWND) -> bool,
{
	let func = unsafe { &mut *(lparam as *mut F) };
	func(hwnd) as _
}

pub(in crate::user) extern "system" fn hdc_enum_display_monitors<F>(
	hmon: HMONITOR,
	hdc: HDC,
	rc: *const RECT,
	lparam: isize,
) -> BOOL
	where F: FnMut(HMONITOR, HDC, &RECT) -> bool,
{
	let func = unsafe { &mut *(lparam as *mut F) };
	func(hmon, hdc, unsafe { &*rc }) as _
}

pub(in crate::user) extern "system" fn hwnd_enum_child_windows<F>(
	hwnd: HWND,
	lparam: isize,
) -> BOOL
	where F: FnMut(HWND) -> bool,
{
	let func = unsafe { &mut *(lparam as *mut F) };
	func(hwnd) as _
}
