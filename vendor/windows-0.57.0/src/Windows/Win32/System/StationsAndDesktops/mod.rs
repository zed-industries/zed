#[inline]
pub unsafe fn BroadcastSystemMessageA<P0, P1>(flags: u32, lpinfo: Option<*mut u32>, msg: u32, wparam: P0, lparam: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::WPARAM>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn BroadcastSystemMessageA(flags : u32, lpinfo : *mut u32, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> i32);
    BroadcastSystemMessageA(flags, core::mem::transmute(lpinfo.unwrap_or(std::ptr::null_mut())), msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn BroadcastSystemMessageExA<P0, P1>(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: Option<*mut BROADCAST_SYSTEM_MESSAGE_INFO>, msg: u32, wparam: P0, lparam: P1, pbsminfo: Option<*mut BSMINFO>) -> i32
where
    P0: windows_core::Param<super::super::Foundation::WPARAM>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn BroadcastSystemMessageExA(flags : BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo : *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, pbsminfo : *mut BSMINFO) -> i32);
    BroadcastSystemMessageExA(flags, core::mem::transmute(lpinfo.unwrap_or(std::ptr::null_mut())), msg, wparam.param().abi(), lparam.param().abi(), core::mem::transmute(pbsminfo.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn BroadcastSystemMessageExW<P0, P1>(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: Option<*mut BROADCAST_SYSTEM_MESSAGE_INFO>, msg: u32, wparam: P0, lparam: P1, pbsminfo: Option<*mut BSMINFO>) -> i32
where
    P0: windows_core::Param<super::super::Foundation::WPARAM>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn BroadcastSystemMessageExW(flags : BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo : *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, pbsminfo : *mut BSMINFO) -> i32);
    BroadcastSystemMessageExW(flags, core::mem::transmute(lpinfo.unwrap_or(std::ptr::null_mut())), msg, wparam.param().abi(), lparam.param().abi(), core::mem::transmute(pbsminfo.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn BroadcastSystemMessageW<P0, P1>(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: Option<*mut BROADCAST_SYSTEM_MESSAGE_INFO>, msg: u32, wparam: P0, lparam: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::WPARAM>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn BroadcastSystemMessageW(flags : BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo : *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> i32);
    BroadcastSystemMessageW(flags, core::mem::transmute(lpinfo.unwrap_or(std::ptr::null_mut())), msg, wparam.param().abi(), lparam.param().abi())
}
#[inline]
pub unsafe fn CloseDesktop<P0>(hdesktop: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HDESK>,
{
    windows_targets::link!("user32.dll" "system" fn CloseDesktop(hdesktop : HDESK) -> super::super::Foundation:: BOOL);
    CloseDesktop(hdesktop.param().abi()).ok()
}
#[inline]
pub unsafe fn CloseWindowStation<P0>(hwinsta: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HWINSTA>,
{
    windows_targets::link!("user32.dll" "system" fn CloseWindowStation(hwinsta : HWINSTA) -> super::super::Foundation:: BOOL);
    CloseWindowStation(hwinsta.param().abi()).ok()
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopA<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEA>, dwflags: DESKTOP_CONTROL_FLAGS, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CreateDesktopA(lpszdesktop : windows_core::PCSTR, lpszdevice : windows_core::PCSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEA, dwflags : DESKTOP_CONTROL_FLAGS, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES) -> HDESK);
    let result__ = CreateDesktopA(lpszdesktop.param().abi(), lpszdevice.param().abi(), core::mem::transmute(pdevmode.unwrap_or(std::ptr::null())), dwflags, dwdesiredaccess, core::mem::transmute(lpsa.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopExA<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEA>, dwflags: DESKTOP_CONTROL_FLAGS, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, ulheapsize: u32, pvoid: Option<*const core::ffi::c_void>) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CreateDesktopExA(lpszdesktop : windows_core::PCSTR, lpszdevice : windows_core::PCSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEA, dwflags : DESKTOP_CONTROL_FLAGS, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES, ulheapsize : u32, pvoid : *const core::ffi::c_void) -> HDESK);
    let result__ = CreateDesktopExA(lpszdesktop.param().abi(), lpszdevice.param().abi(), core::mem::transmute(pdevmode.unwrap_or(std::ptr::null())), dwflags, dwdesiredaccess, core::mem::transmute(lpsa.unwrap_or(std::ptr::null())), ulheapsize, core::mem::transmute(pvoid.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopExW<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEW>, dwflags: DESKTOP_CONTROL_FLAGS, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, ulheapsize: u32, pvoid: Option<*const core::ffi::c_void>) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CreateDesktopExW(lpszdesktop : windows_core::PCWSTR, lpszdevice : windows_core::PCWSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEW, dwflags : DESKTOP_CONTROL_FLAGS, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES, ulheapsize : u32, pvoid : *const core::ffi::c_void) -> HDESK);
    let result__ = CreateDesktopExW(lpszdesktop.param().abi(), lpszdevice.param().abi(), core::mem::transmute(pdevmode.unwrap_or(std::ptr::null())), dwflags, dwdesiredaccess, core::mem::transmute(lpsa.unwrap_or(std::ptr::null())), ulheapsize, core::mem::transmute(pvoid.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopW<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEW>, dwflags: DESKTOP_CONTROL_FLAGS, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CreateDesktopW(lpszdesktop : windows_core::PCWSTR, lpszdevice : windows_core::PCWSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEW, dwflags : DESKTOP_CONTROL_FLAGS, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES) -> HDESK);
    let result__ = CreateDesktopW(lpszdesktop.param().abi(), lpszdevice.param().abi(), core::mem::transmute(pdevmode.unwrap_or(std::ptr::null())), dwflags, dwdesiredaccess, core::mem::transmute(lpsa.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateWindowStationA<P0>(lpwinsta: P0, dwflags: u32, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<HWINSTA>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CreateWindowStationA(lpwinsta : windows_core::PCSTR, dwflags : u32, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES) -> HWINSTA);
    let result__ = CreateWindowStationA(lpwinsta.param().abi(), dwflags, dwdesiredaccess, core::mem::transmute(lpsa.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateWindowStationW<P0>(lpwinsta: P0, dwflags: u32, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<HWINSTA>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("user32.dll" "system" fn CreateWindowStationW(lpwinsta : windows_core::PCWSTR, dwflags : u32, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES) -> HWINSTA);
    let result__ = CreateWindowStationW(lpwinsta.param().abi(), dwflags, dwdesiredaccess, core::mem::transmute(lpsa.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn EnumDesktopWindows<P0, P1>(hdesktop: P0, lpfn: super::super::UI::WindowsAndMessaging::WNDENUMPROC, lparam: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HDESK>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn EnumDesktopWindows(hdesktop : HDESK, lpfn : super::super::UI::WindowsAndMessaging:: WNDENUMPROC, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    EnumDesktopWindows(hdesktop.param().abi(), lpfn, lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn EnumDesktopsA<P0, P1>(hwinsta: P0, lpenumfunc: DESKTOPENUMPROCA, lparam: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HWINSTA>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn EnumDesktopsA(hwinsta : HWINSTA, lpenumfunc : DESKTOPENUMPROCA, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    EnumDesktopsA(hwinsta.param().abi(), lpenumfunc, lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn EnumDesktopsW<P0, P1>(hwinsta: P0, lpenumfunc: DESKTOPENUMPROCW, lparam: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HWINSTA>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn EnumDesktopsW(hwinsta : HWINSTA, lpenumfunc : DESKTOPENUMPROCW, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    EnumDesktopsW(hwinsta.param().abi(), lpenumfunc, lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn EnumWindowStationsA<P0>(lpenumfunc: WINSTAENUMPROCA, lparam: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn EnumWindowStationsA(lpenumfunc : WINSTAENUMPROCA, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    EnumWindowStationsA(lpenumfunc, lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn EnumWindowStationsW<P0>(lpenumfunc: WINSTAENUMPROCW, lparam: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("user32.dll" "system" fn EnumWindowStationsW(lpenumfunc : WINSTAENUMPROCW, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    EnumWindowStationsW(lpenumfunc, lparam.param().abi()).ok()
}
#[inline]
pub unsafe fn GetProcessWindowStation() -> windows_core::Result<HWINSTA> {
    windows_targets::link!("user32.dll" "system" fn GetProcessWindowStation() -> HWINSTA);
    let result__ = GetProcessWindowStation();
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetThreadDesktop(dwthreadid: u32) -> windows_core::Result<HDESK> {
    windows_targets::link!("user32.dll" "system" fn GetThreadDesktop(dwthreadid : u32) -> HDESK);
    let result__ = GetThreadDesktop(dwthreadid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetUserObjectInformationA<P0>(hobj: P0, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: Option<*mut core::ffi::c_void>, nlength: u32, lpnlengthneeded: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn GetUserObjectInformationA(hobj : super::super::Foundation:: HANDLE, nindex : USER_OBJECT_INFORMATION_INDEX, pvinfo : *mut core::ffi::c_void, nlength : u32, lpnlengthneeded : *mut u32) -> super::super::Foundation:: BOOL);
    GetUserObjectInformationA(hobj.param().abi(), nindex, core::mem::transmute(pvinfo.unwrap_or(std::ptr::null_mut())), nlength, core::mem::transmute(lpnlengthneeded.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetUserObjectInformationW<P0>(hobj: P0, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: Option<*mut core::ffi::c_void>, nlength: u32, lpnlengthneeded: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn GetUserObjectInformationW(hobj : super::super::Foundation:: HANDLE, nindex : USER_OBJECT_INFORMATION_INDEX, pvinfo : *mut core::ffi::c_void, nlength : u32, lpnlengthneeded : *mut u32) -> super::super::Foundation:: BOOL);
    GetUserObjectInformationW(hobj.param().abi(), nindex, core::mem::transmute(pvinfo.unwrap_or(std::ptr::null_mut())), nlength, core::mem::transmute(lpnlengthneeded.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn OpenDesktopA<P0, P1>(lpszdesktop: P0, dwflags: DESKTOP_CONTROL_FLAGS, finherit: P1, dwdesiredaccess: u32) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn OpenDesktopA(lpszdesktop : windows_core::PCSTR, dwflags : DESKTOP_CONTROL_FLAGS, finherit : super::super::Foundation:: BOOL, dwdesiredaccess : u32) -> HDESK);
    let result__ = OpenDesktopA(lpszdesktop.param().abi(), dwflags, finherit.param().abi(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenDesktopW<P0, P1>(lpszdesktop: P0, dwflags: DESKTOP_CONTROL_FLAGS, finherit: P1, dwdesiredaccess: u32) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn OpenDesktopW(lpszdesktop : windows_core::PCWSTR, dwflags : DESKTOP_CONTROL_FLAGS, finherit : super::super::Foundation:: BOOL, dwdesiredaccess : u32) -> HDESK);
    let result__ = OpenDesktopW(lpszdesktop.param().abi(), dwflags, finherit.param().abi(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenInputDesktop<P0>(dwflags: DESKTOP_CONTROL_FLAGS, finherit: P0, dwdesiredaccess: DESKTOP_ACCESS_FLAGS) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn OpenInputDesktop(dwflags : DESKTOP_CONTROL_FLAGS, finherit : super::super::Foundation:: BOOL, dwdesiredaccess : DESKTOP_ACCESS_FLAGS) -> HDESK);
    let result__ = OpenInputDesktop(dwflags, finherit.param().abi(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenWindowStationA<P0, P1>(lpszwinsta: P0, finherit: P1, dwdesiredaccess: u32) -> windows_core::Result<HWINSTA>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn OpenWindowStationA(lpszwinsta : windows_core::PCSTR, finherit : super::super::Foundation:: BOOL, dwdesiredaccess : u32) -> HWINSTA);
    let result__ = OpenWindowStationA(lpszwinsta.param().abi(), finherit.param().abi(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenWindowStationW<P0, P1>(lpszwinsta: P0, finherit: P1, dwdesiredaccess: u32) -> windows_core::Result<HWINSTA>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn OpenWindowStationW(lpszwinsta : windows_core::PCWSTR, finherit : super::super::Foundation:: BOOL, dwdesiredaccess : u32) -> HWINSTA);
    let result__ = OpenWindowStationW(lpszwinsta.param().abi(), finherit.param().abi(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn SetProcessWindowStation<P0>(hwinsta: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HWINSTA>,
{
    windows_targets::link!("user32.dll" "system" fn SetProcessWindowStation(hwinsta : HWINSTA) -> super::super::Foundation:: BOOL);
    SetProcessWindowStation(hwinsta.param().abi()).ok()
}
#[inline]
pub unsafe fn SetThreadDesktop<P0>(hdesktop: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HDESK>,
{
    windows_targets::link!("user32.dll" "system" fn SetThreadDesktop(hdesktop : HDESK) -> super::super::Foundation:: BOOL);
    SetThreadDesktop(hdesktop.param().abi()).ok()
}
#[inline]
pub unsafe fn SetUserObjectInformationA<P0>(hobj: P0, nindex: i32, pvinfo: *const core::ffi::c_void, nlength: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn SetUserObjectInformationA(hobj : super::super::Foundation:: HANDLE, nindex : i32, pvinfo : *const core::ffi::c_void, nlength : u32) -> super::super::Foundation:: BOOL);
    SetUserObjectInformationA(hobj.param().abi(), nindex, pvinfo, nlength).ok()
}
#[inline]
pub unsafe fn SetUserObjectInformationW<P0>(hobj: P0, nindex: i32, pvinfo: *const core::ffi::c_void, nlength: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn SetUserObjectInformationW(hobj : super::super::Foundation:: HANDLE, nindex : i32, pvinfo : *const core::ffi::c_void, nlength : u32) -> super::super::Foundation:: BOOL);
    SetUserObjectInformationW(hobj.param().abi(), nindex, pvinfo, nlength).ok()
}
#[inline]
pub unsafe fn SwitchDesktop<P0>(hdesktop: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HDESK>,
{
    windows_targets::link!("user32.dll" "system" fn SwitchDesktop(hdesktop : HDESK) -> super::super::Foundation:: BOOL);
    SwitchDesktop(hdesktop.param().abi()).ok()
}
pub const BSF_ALLOWSFW: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(128u32);
pub const BSF_FLUSHDISK: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(4u32);
pub const BSF_FORCEIFHUNG: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(32u32);
pub const BSF_IGNORECURRENTTASK: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(2u32);
pub const BSF_LUID: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(1024u32);
pub const BSF_NOHANG: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(8u32);
pub const BSF_NOTIMEOUTIFNOTHUNG: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(64u32);
pub const BSF_POSTMESSAGE: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(16u32);
pub const BSF_QUERY: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(1u32);
pub const BSF_RETURNHDESK: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(512u32);
pub const BSF_SENDNOTIFYMESSAGE: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(256u32);
pub const BSM_ALLCOMPONENTS: BROADCAST_SYSTEM_MESSAGE_INFO = BROADCAST_SYSTEM_MESSAGE_INFO(0u32);
pub const BSM_ALLDESKTOPS: BROADCAST_SYSTEM_MESSAGE_INFO = BROADCAST_SYSTEM_MESSAGE_INFO(16u32);
pub const BSM_APPLICATIONS: BROADCAST_SYSTEM_MESSAGE_INFO = BROADCAST_SYSTEM_MESSAGE_INFO(8u32);
pub const DESKTOP_CREATEMENU: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(4u32);
pub const DESKTOP_CREATEWINDOW: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(2u32);
pub const DESKTOP_DELETE: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(65536u32);
pub const DESKTOP_ENUMERATE: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(64u32);
pub const DESKTOP_HOOKCONTROL: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(8u32);
pub const DESKTOP_JOURNALPLAYBACK: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(32u32);
pub const DESKTOP_JOURNALRECORD: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(16u32);
pub const DESKTOP_READOBJECTS: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(1u32);
pub const DESKTOP_READ_CONTROL: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(131072u32);
pub const DESKTOP_SWITCHDESKTOP: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(256u32);
pub const DESKTOP_SYNCHRONIZE: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(1048576u32);
pub const DESKTOP_WRITEOBJECTS: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(128u32);
pub const DESKTOP_WRITE_DAC: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(262144u32);
pub const DESKTOP_WRITE_OWNER: DESKTOP_ACCESS_FLAGS = DESKTOP_ACCESS_FLAGS(524288u32);
pub const DF_ALLOWOTHERACCOUNTHOOK: DESKTOP_CONTROL_FLAGS = DESKTOP_CONTROL_FLAGS(1u32);
pub const UOI_FLAGS: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(1i32);
pub const UOI_HEAPSIZE: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(5i32);
pub const UOI_IO: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(6i32);
pub const UOI_NAME: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(2i32);
pub const UOI_TYPE: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(3i32);
pub const UOI_USER_SID: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(4i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BROADCAST_SYSTEM_MESSAGE_FLAGS(pub u32);
impl windows_core::TypeKind for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BROADCAST_SYSTEM_MESSAGE_FLAGS").field(&self.0).finish()
    }
}
impl BROADCAST_SYSTEM_MESSAGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BROADCAST_SYSTEM_MESSAGE_INFO(pub u32);
impl windows_core::TypeKind for BROADCAST_SYSTEM_MESSAGE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BROADCAST_SYSTEM_MESSAGE_INFO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BROADCAST_SYSTEM_MESSAGE_INFO").field(&self.0).finish()
    }
}
impl BROADCAST_SYSTEM_MESSAGE_INFO {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for BROADCAST_SYSTEM_MESSAGE_INFO {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for BROADCAST_SYSTEM_MESSAGE_INFO {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for BROADCAST_SYSTEM_MESSAGE_INFO {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for BROADCAST_SYSTEM_MESSAGE_INFO {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for BROADCAST_SYSTEM_MESSAGE_INFO {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DESKTOP_ACCESS_FLAGS(pub u32);
impl windows_core::TypeKind for DESKTOP_ACCESS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DESKTOP_ACCESS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DESKTOP_ACCESS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DESKTOP_CONTROL_FLAGS(pub u32);
impl windows_core::TypeKind for DESKTOP_CONTROL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DESKTOP_CONTROL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DESKTOP_CONTROL_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USER_OBJECT_INFORMATION_INDEX(pub i32);
impl windows_core::TypeKind for USER_OBJECT_INFORMATION_INDEX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USER_OBJECT_INFORMATION_INDEX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USER_OBJECT_INFORMATION_INDEX").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BSMINFO {
    pub cbSize: u32,
    pub hdesk: HDESK,
    pub hwnd: super::super::Foundation::HWND,
    pub luid: super::super::Foundation::LUID,
}
impl windows_core::TypeKind for BSMINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for BSMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDESK(pub isize);
impl HDESK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HDESK {
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = CloseDesktop(*self);
        }
    }
}
impl Default for HDESK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDESK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWINSTA(pub isize);
impl HWINSTA {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HWINSTA {
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = CloseWindowStation(*self);
        }
    }
}
impl Default for HWINSTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HWINSTA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USEROBJECTFLAGS {
    pub fInherit: super::super::Foundation::BOOL,
    pub fReserved: super::super::Foundation::BOOL,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for USEROBJECTFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for USEROBJECTFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DESKTOPENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
pub type DESKTOPENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
pub type WINSTAENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
pub type WINSTAENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
