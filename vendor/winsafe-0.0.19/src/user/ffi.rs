use crate::kernel::ffi_types::*;

extern_sys! { "comdlg32";
	ChooseColorW(PVOID) -> BOOL
	CommDlgExtendedError() -> u32
}

#[cfg(target_pointer_width = "32")]
extern_sys! { "user32";
	GetWindowLongW(HANDLE, i32) -> isize
	SetWindowLongW(HANDLE, i32, isize) -> isize
}

#[cfg(target_pointer_width = "64")]
extern_sys! { "user32";
	GetWindowLongPtrW(HANDLE, i32) -> isize
	SetWindowLongPtrW(HANDLE, i32, isize) -> isize
	InSendMessageEx() -> u32
}

extern_sys! { "user32";
	AdjustWindowRectEx(PVOID, u32, BOOL, u32) -> BOOL
	AdjustWindowRectExForDpi(PVOID, u32, BOOL, u32, u32) -> BOOL
	AllowSetForegroundWindow(u32) -> BOOL
	AnyPopup() -> BOOL
	AppendMenuW(HANDLE, u32, usize, PCSTR) -> BOOL
	ArrangeIconicWindows(HANDLE) -> u32
	AttachThreadInput(u32, u32, BOOL) -> BOOL
	BeginDeferWindowPos(i32) -> HANDLE
	BeginPaint(HANDLE, PVOID) -> HANDLE
	BlockInput(BOOL) -> BOOL
	BringWindowToTop(HANDLE) -> BOOL
	BroadcastSystemMessageW(u32, *mut u32, u32, usize, isize) -> i32
	CallNextHookEx(HANDLE, i32, usize, isize) -> isize
	ChangeDisplaySettingsExW(PCSTR, PVOID, PVOID, u32, PVOID) -> i32
	ChangeDisplaySettingsW(PVOID, u32) -> i32
	CheckMenuItem(HANDLE, u32, u32) -> i32
	CheckMenuRadioItem(HANDLE, u32, u32, u32, u32) -> BOOL
	ChildWindowFromPoint(HANDLE, i32, i32) -> HANDLE
	ClientToScreen(HANDLE, PVOID) -> BOOL
	ClipCursor(PCVOID) -> BOOL
	CloseClipboard() -> BOOL
	CloseDesktop(HANDLE) -> BOOL
	CloseWindow(HANDLE) -> BOOL
	CopyIcon(HANDLE) -> HANDLE
	CreateAcceleratorTableW(PVOID, i32) -> HANDLE
	CreateDesktopExW(PCSTR, PCSTR, PCVOID, u32, u32, PVOID, u32, PVOID) -> HANDLE
	CreateDesktopW(PCSTR, PCSTR, PCVOID, u32, u32, PVOID) -> HANDLE
	CreateDialogParamW(HANDLE, PCSTR, HANDLE, PFUNC, isize) -> HANDLE
	CreateMenu() -> HANDLE
	CreatePopupMenu() -> HANDLE
	CreateWindowExW(u32, PCSTR, PCSTR, u32, i32, i32, i32, i32, HANDLE, HANDLE, HANDLE, PVOID) -> HANDLE
	DeferWindowPos(HANDLE, HANDLE, HANDLE, i32, i32, i32, i32, u32) -> HANDLE
	DefWindowProcW(HANDLE, u32, usize, isize) -> isize
	DeleteMenu(HANDLE, u32, u32) -> BOOL
	DestroyAcceleratorTable(HANDLE) -> BOOL
	DestroyCursor(HANDLE) -> BOOL
	DestroyIcon(HANDLE) -> BOOL
	DestroyMenu(HANDLE) -> BOOL
	DestroyWindow(HANDLE) -> BOOL
	DialogBoxIndirectParamW(HANDLE, PCVOID, HANDLE, PFUNC, isize) -> isize
	DialogBoxParamW(HANDLE, PCSTR, HANDLE, PFUNC, isize) -> isize
	DispatchMessageW(PCVOID) -> isize
	DragDetect(HANDLE, i32, i32) -> BOOL
	DrawCaption(HANDLE, HANDLE, PCVOID, u32) -> BOOL
	DrawFocusRect(HANDLE, PCVOID) -> BOOL
	DrawMenuBar(HANDLE) -> BOOL
	DrawTextExW(HANDLE, PCSTR, i32, PVOID, u32, PCVOID) -> i32
	DrawTextW(HANDLE, PCSTR, i32, PVOID, u32) -> i32
	EmptyClipboard() -> BOOL
	EnableMenuItem(HANDLE, u32, u32) -> BOOL
	EnableScrollBar(HANDLE, u32, u32) -> BOOL
	EnableWindow(HANDLE, BOOL) -> BOOL
	EndDeferWindowPos(HANDLE) -> BOOL
	EndDialog(HANDLE, isize) -> BOOL
	EndMenu() -> BOOL
	EndPaint(HANDLE, PCVOID) -> BOOL
	EnumChildWindows(HANDLE, PFUNC, isize) -> BOOL
	EnumDisplayDevicesW(PCSTR, u32, PVOID, u32) -> BOOL
	EnumDisplayMonitors(HANDLE, PCVOID, PFUNC, isize) -> BOOL
	EnumDisplaySettingsExW(PCSTR, u32, PVOID, u32) -> BOOL
	EnumDisplaySettingsW(PCSTR, u32, PVOID) -> BOOL
	EnumThreadWindows(u32, PFUNC, isize) -> BOOL
	EnumWindows(PFUNC, isize) -> BOOL
	ExitWindowsEx(u32, u32) -> BOOL
	FindWindowExW(HANDLE, HANDLE, PCSTR, PCSTR) -> HANDLE
	FindWindowW(PCSTR, PCSTR) -> HANDLE
	GetActiveWindow() -> HANDLE
	GetAltTabInfoW(HANDLE, i32, PVOID, PSTR, u32) -> BOOL
	GetAncestor(HANDLE, u32) -> HANDLE
	GetAsyncKeyState(i32) -> i16
	GetCapture() -> HANDLE
	GetClassInfoExW(HANDLE, PCSTR, PVOID) -> BOOL
	GetClassLongPtrW(HANDLE, i32) -> usize
	GetClassNameW(HANDLE, PSTR, i32) -> i32
	GetClientRect(HANDLE, PVOID) -> BOOL
	GetClipboardData(u32) -> HANDLE
	GetClipboardSequenceNumber() -> u32
	GetClipCursor(PVOID) -> BOOL
	GetCursorPos(PVOID) -> BOOL
	GetDC(HANDLE) -> HANDLE
	GetDesktopWindow() -> HANDLE
	GetDialogBaseUnits() -> i32
	GetDlgCtrlID(HANDLE) -> i32
	GetDlgItem(HANDLE, i32) -> HANDLE
	GetDoubleClickTime() -> u32
	GetDpiForWindow(HANDLE) -> u32
	GetFocus() -> HANDLE
	GetForegroundWindow() -> HANDLE
	GetGUIThreadInfo(u32, PVOID) -> BOOL
	GetLastActivePopup(HANDLE) -> HANDLE
	GetMenu(HANDLE) -> HANDLE
	GetMenuBarInfo(HANDLE, i32, i32, PVOID) -> BOOL
	GetMenuCheckMarkDimensions() -> u32
	GetMenuDefaultItem(HANDLE, u32, u32) -> u32
	GetMenuInfo(HANDLE, PVOID) -> BOOL
	GetMenuItemCount(HANDLE) -> i32
	GetMenuItemID(HANDLE, i32) -> i32
	GetMenuItemInfoW(HANDLE, u32, BOOL, PVOID) -> BOOL
	GetMenuItemRect(HANDLE, HANDLE, u32, PVOID) -> BOOL
	GetMenuState(HANDLE, u32, u32) -> u32
	GetMenuStringW(HANDLE, u32, PSTR, i32, u32) -> i32
	GetMessagePos() -> u32
	GetMessageW(PVOID, HANDLE, u32, u32) -> BOOL
	GetMonitorInfoW(HANDLE, PVOID) -> BOOL
	GetNextDlgGroupItem(HANDLE, HANDLE, BOOL) -> HANDLE
	GetNextDlgTabItem(HANDLE, HANDLE, BOOL) -> HANDLE
	GetParent(HANDLE) -> HANDLE
	GetQueueStatus(u32) -> u32
	GetScrollInfo(HANDLE, i32, PVOID) -> BOOL
	GetScrollPos(HANDLE, i32) -> i32
	GetShellWindow() -> HANDLE
	GetSubMenu(HANDLE, i32) -> HANDLE
	GetSysColor(i32) -> u32
	GetSystemMenu(HANDLE, BOOL) -> HANDLE
	GetSystemMetrics(i32) -> i32
	GetSystemMetricsForDpi(i32, u32) -> i32
	GetThreadDesktop(u32) -> HANDLE
	GetTopWindow(HANDLE) -> HANDLE
	GetUpdateRect(HANDLE, PVOID, BOOL) -> BOOL
	GetUpdateRgn(HANDLE, HANDLE, BOOL) -> i32
	GetWindow(HANDLE, u32) -> HANDLE
	GetWindowDC(HANDLE) -> HANDLE
	GetWindowDisplayAffinity(HANDLE, PVOID) -> BOOL
	GetWindowInfo(HANDLE, PVOID) -> BOOL
	GetWindowModuleFileNameW(HANDLE, PSTR, u32) -> u32
	GetWindowPlacement(HANDLE, PVOID) -> BOOL
	GetWindowRect(HANDLE, PVOID) -> BOOL
	GetWindowRgn(HANDLE, HANDLE) -> i32
	GetWindowRgnBox(HANDLE, PVOID) -> i32
	GetWindowTextLengthW(HANDLE) -> i32
	GetWindowTextW(HANDLE, PSTR, i32) -> i32
	GetWindowThreadProcessId(HANDLE, *mut u32) -> u32
	HiliteMenuItem(HANDLE, HANDLE, u32, u32) -> BOOL
	InflateRect(PVOID, i32, i32) -> BOOL
	InSendMessage() -> BOOL
	InsertMenuItemW(HANDLE, u32, BOOL, PCVOID) -> BOOL
	IntersectRect(PVOID, PCVOID, PCVOID) -> BOOL
	InvalidateRect(HANDLE, PCVOID, BOOL) -> BOOL
	InvalidateRgn(HANDLE, HANDLE, BOOL) -> BOOL
	InvertRect(HANDLE, PCVOID) -> BOOL
	IsChild(HANDLE, HANDLE) -> BOOL
	IsDialogMessageW(HANDLE, PVOID) -> BOOL
	IsGUIThread(BOOL) -> BOOL
	IsIconic(HANDLE) -> BOOL
	IsMenu(HANDLE) -> BOOL
	IsRectEmpty(PVOID) -> BOOL
	IsWindow(HANDLE) -> BOOL
	IsWindowEnabled(HANDLE) -> BOOL
	IsWindowUnicode(HANDLE) -> BOOL
	IsWindowVisible(HANDLE) -> BOOL
	IsWow64Message() -> BOOL
	IsZoomed(HANDLE) -> BOOL
	KillTimer(HANDLE, usize) -> BOOL
	LoadAcceleratorsW(HANDLE, PCSTR) -> HANDLE
	LoadCursorW(HANDLE, PCSTR) -> HANDLE
	LoadIconW(HANDLE, PCSTR) -> HANDLE
	LoadMenuW(HANDLE, PCSTR) -> HANDLE
	LoadStringW(HANDLE, u32, PSTR, i32) -> i32
	LockSetForegroundWindow(u32) -> BOOL
	LockWindowUpdate(HANDLE) -> BOOL
	LogicalToPhysicalPoint(HANDLE, PVOID) -> BOOL
	MapDialogRect(HANDLE, PVOID) -> BOOL
	MapWindowPoints(HANDLE, HANDLE, PVOID, u32) -> i32
	MessageBoxW(HANDLE, PCSTR, PCSTR, u32) -> i32
	MonitorFromPoint(i32, i32, u32) -> HANDLE
	MonitorFromRect(PCVOID, u32) -> HANDLE
	MonitorFromWindow(HANDLE, u32) -> HANDLE
	MoveWindow(HANDLE, i32, i32, i32, i32, BOOL) -> BOOL
	OffsetRect(PVOID, i32, i32) -> BOOL
	OpenClipboard(HANDLE) -> BOOL
	OpenDesktopW(PCSTR, u32, BOOL, u32) -> HANDLE
	OpenInputDesktop(u32, BOOL, u32) -> HANDLE
	PaintDesktop(HANDLE) -> BOOL
	PeekMessageW(PVOID, HANDLE, u32, u32, u32) -> BOOL
	PostMessageW(HANDLE, u32, usize, isize) -> BOOL
	PostQuitMessage(i32)
	PostThreadMessageW(u32, u32, usize, isize) -> BOOL
	PtInRect(PCVOID, i32, i32) -> BOOL
	RealChildWindowFromPoint(HANDLE, i32, i32) -> HANDLE
	RealGetWindowClassW(HANDLE, PSTR, i32) -> u32
	RedrawWindow(HANDLE, PCVOID, HANDLE, u32) -> BOOL
	RegisterClassExW(PCVOID) -> u16
	RegisterWindowMessageW(PCSTR) -> u32
	ReleaseCapture() -> BOOL
	ReleaseDC(HANDLE, HANDLE) -> i32
	RemoveMenu(HANDLE, u32, u32) -> BOOL
	ScreenToClient(HANDLE, PVOID) -> BOOL
	ScrollWindowEx(HANDLE, i32, i32, PCVOID, PCVOID, HANDLE, PVOID, u32) -> i32
	SendInput(u32, PVOID, i32) -> u32
	SendMessageTimeoutW(HANDLE, u32, usize, isize, u32, u32, *mut isize) -> isize
	SendMessageW(HANDLE, u32, usize, isize) -> isize
	SetActiveWindow(HANDLE) -> HANDLE
	SetCapture(HANDLE) -> HANDLE
	SetCaretBlinkTime(u32) -> BOOL
	SetCaretPos(i32, i32) -> BOOL
	SetClipboardData(u32, HANDLE) -> HANDLE
	SetCursorPos(i32, i32) -> BOOL
	SetDoubleClickTime(u32) -> BOOL
	SetFocus(HANDLE) -> HANDLE
	SetForegroundWindow(HANDLE) -> BOOL
	SetLayeredWindowAttributes(HANDLE, u32, u8, u32) -> BOOL
	SetMenu(HANDLE, HANDLE) -> BOOL
	SetMenuDefaultItem(HANDLE, u32, u32) -> BOOL
	SetMenuInfo(HANDLE, PCVOID) -> BOOL
	SetMenuItemBitmaps(HANDLE, u32, u32, HANDLE, HANDLE) -> BOOL
	SetMenuItemInfoW(HANDLE, u32, BOOL, PCVOID) -> BOOL
	SetParent(HANDLE, HANDLE) -> HANDLE
	SetProcessDPIAware() -> BOOL
	SetScrollInfo(HANDLE, i32, PCVOID, BOOL) -> i32
	SetScrollPos(HANDLE, i32, i32, BOOL) -> i32
	SetScrollRange(HANDLE, i32, i32, i32, BOOL) -> BOOL
	SetSystemCursor(HANDLE, u32) -> BOOL
	SetThreadDesktop(HANDLE) -> BOOL
	SetTimer(HANDLE, usize, u32, PFUNC) -> usize
	SetUserObjectInformationW(HANDLE, i32, PVOID, u32) -> BOOL
	SetWindowDisplayAffinity(HANDLE, u32) -> BOOL
	SetWindowPlacement(HANDLE, PCVOID) -> BOOL
	SetWindowPos(HANDLE, HANDLE, i32, i32, i32, i32, u32) -> BOOL
	SetWindowRgn(HANDLE, HANDLE, BOOL) -> i32
	SetWindowsHookExW(i32, PFUNC, HANDLE, u32) -> HANDLE
	SetWindowTextW(HANDLE, PCSTR) -> BOOL
	ShowCaret(HANDLE) -> BOOL
	ShowCursor(BOOL) -> i32
	ShowOwnedPopups(HANDLE, BOOL) -> BOOL
	ShowWindow(HANDLE, i32) -> BOOL
	ShowWindowAsync(HANDLE, i32) -> BOOL
	SoundSentry() -> BOOL
	SubtractRect(PVOID, PCVOID, PCVOID) -> BOOL
	SwapMouseButton(BOOL) -> BOOL
	SwitchDesktop(HANDLE) -> BOOL
	SystemParametersInfoW(u32, u32, PVOID, u32) -> BOOL
	TileWindows(HANDLE, u32, PCVOID, u32, PCVOID) -> u16
	TrackMouseEvent(PVOID) -> BOOL
	TrackPopupMenu(HANDLE, u32, i32, i32, i32, HANDLE, PCVOID) -> BOOL
	TranslateAcceleratorW(HANDLE, HANDLE, PVOID) -> i32
	TranslateMessage(PCVOID) -> BOOL
	UnhookWindowsHookEx(HANDLE) -> BOOL
	UnionRect(PVOID, PCVOID, PCVOID) -> BOOL
	UnregisterClassW(PCSTR, HANDLE) -> BOOL
	UpdateWindow(HANDLE) -> BOOL
	ValidateRect(HANDLE, PCVOID) -> BOOL
	ValidateRgn(HANDLE, HANDLE) -> BOOL
	WaitMessage() -> BOOL
	WindowFromDC(HANDLE) -> HANDLE
	WindowFromPhysicalPoint(i32, i32) -> HANDLE
	WindowFromPoint(i32, i32) -> HANDLE
	WinHelpW(HANDLE, PCSTR, u32, usize) -> BOOL
}
