use crate::co;
use crate::decl::*;

/// Type alias to
/// [`CCHOOKPROC`](https://learn.microsoft.com/en-us/windows/win32/api/commdlg/nc-commdlg-lpcchookproc)
/// callback function.
pub type CCHOOKPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: u32,
		wParam: usize,
		lParam: isize,
	) -> usize;

/// Type alias to
/// [`DLGPROC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-dlgproc)
/// callback function.
pub type DLGPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
	) -> isize;

/// Type alias to
/// [`EDITWORDBREAKPROC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-editwordbreakprocw)
/// callback function.
pub type EDITWORDBREAKPROC =
	extern "system" fn(
		lpch: *mut u16,
		ichCurrent: i32,
		cch: i32,
		code: i32,
	) -> i32;

/// Type alias to
/// [`HOOKPROC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-hookproc)
/// callback function.
pub type HOOKPROC =
	extern "system" fn(
		code: i32,
		wParam: usize,
		lParam: isize,
	) -> isize;

/// Type alias to
/// [`TIMERPROC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-timerproc)
/// callback function.
pub type TIMERPROC =
	extern "system" fn(
		hWnd: HWND,
		msg: co::WM,
		timerId: usize,
		nSeconds: u32,
	);

/// Type alias to
/// [`WNDPROC`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms633573(v=vs.85))
/// callback function.
pub type WNDPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
	) -> isize;
