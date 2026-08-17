#[inline]
pub unsafe fn BroadcastSystemMessageA(flags: u32, lpinfo: Option<*mut u32>, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> i32 {
    windows_core::link!("user32.dll" "system" fn BroadcastSystemMessageA(flags : u32, lpinfo : *mut u32, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> i32);
    unsafe { BroadcastSystemMessageA(flags, lpinfo.unwrap_or(core::mem::zeroed()) as _, msg, wparam, lparam) }
}
#[inline]
pub unsafe fn BroadcastSystemMessageExA(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: Option<*mut BROADCAST_SYSTEM_MESSAGE_INFO>, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pbsminfo: Option<*mut BSMINFO>) -> i32 {
    windows_core::link!("user32.dll" "system" fn BroadcastSystemMessageExA(flags : BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo : *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, pbsminfo : *mut BSMINFO) -> i32);
    unsafe { BroadcastSystemMessageExA(flags, lpinfo.unwrap_or(core::mem::zeroed()) as _, msg, wparam, lparam, pbsminfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BroadcastSystemMessageExW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: Option<*mut BROADCAST_SYSTEM_MESSAGE_INFO>, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pbsminfo: Option<*mut BSMINFO>) -> i32 {
    windows_core::link!("user32.dll" "system" fn BroadcastSystemMessageExW(flags : BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo : *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, pbsminfo : *mut BSMINFO) -> i32);
    unsafe { BroadcastSystemMessageExW(flags, lpinfo.unwrap_or(core::mem::zeroed()) as _, msg, wparam, lparam, pbsminfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BroadcastSystemMessageW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: Option<*mut BROADCAST_SYSTEM_MESSAGE_INFO>, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> i32 {
    windows_core::link!("user32.dll" "system" fn BroadcastSystemMessageW(flags : BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo : *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM) -> i32);
    unsafe { BroadcastSystemMessageW(flags, lpinfo.unwrap_or(core::mem::zeroed()) as _, msg, wparam, lparam) }
}
#[inline]
pub unsafe fn CloseDesktop(hdesktop: HDESK) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn CloseDesktop(hdesktop : HDESK) -> windows_core::BOOL);
    unsafe { CloseDesktop(hdesktop).ok() }
}
#[inline]
pub unsafe fn CloseWindowStation(hwinsta: HWINSTA) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn CloseWindowStation(hwinsta : HWINSTA) -> windows_core::BOOL);
    unsafe { CloseWindowStation(hwinsta).ok() }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopA<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEA>, dwflags: DESKTOP_CONTROL_FLAGS, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDesktopA(lpszdesktop : windows_core::PCSTR, lpszdevice : windows_core::PCSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEA, dwflags : DESKTOP_CONTROL_FLAGS, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES) -> HDESK);
    let result__ = unsafe { CreateDesktopA(lpszdesktop.param().abi(), lpszdevice.param().abi(), pdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopExA<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEA>, dwflags: DESKTOP_CONTROL_FLAGS, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, ulheapsize: u32, pvoid: Option<*const core::ffi::c_void>) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDesktopExA(lpszdesktop : windows_core::PCSTR, lpszdevice : windows_core::PCSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEA, dwflags : DESKTOP_CONTROL_FLAGS, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES, ulheapsize : u32, pvoid : *const core::ffi::c_void) -> HDESK);
    let result__ = unsafe { CreateDesktopExA(lpszdesktop.param().abi(), lpszdevice.param().abi(), pdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _, ulheapsize, pvoid.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopExW<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEW>, dwflags: DESKTOP_CONTROL_FLAGS, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, ulheapsize: u32, pvoid: Option<*const core::ffi::c_void>) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDesktopExW(lpszdesktop : windows_core::PCWSTR, lpszdevice : windows_core::PCWSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEW, dwflags : DESKTOP_CONTROL_FLAGS, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES, ulheapsize : u32, pvoid : *const core::ffi::c_void) -> HDESK);
    let result__ = unsafe { CreateDesktopExW(lpszdesktop.param().abi(), lpszdevice.param().abi(), pdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _, ulheapsize, pvoid.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopW<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEW>, dwflags: DESKTOP_CONTROL_FLAGS, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDesktopW(lpszdesktop : windows_core::PCWSTR, lpszdevice : windows_core::PCWSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEW, dwflags : DESKTOP_CONTROL_FLAGS, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES) -> HDESK);
    let result__ = unsafe { CreateDesktopW(lpszdesktop.param().abi(), lpszdevice.param().abi(), pdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateWindowStationA<P0>(lpwinsta: P0, dwflags: u32, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<HWINSTA>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateWindowStationA(lpwinsta : windows_core::PCSTR, dwflags : u32, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES) -> HWINSTA);
    let result__ = unsafe { CreateWindowStationA(lpwinsta.param().abi(), dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateWindowStationW<P0>(lpwinsta: P0, dwflags: u32, dwdesiredaccess: u32, lpsa: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<HWINSTA>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateWindowStationW(lpwinsta : windows_core::PCWSTR, dwflags : u32, dwdesiredaccess : u32, lpsa : *const super::super::Security:: SECURITY_ATTRIBUTES) -> HWINSTA);
    let result__ = unsafe { CreateWindowStationW(lpwinsta.param().abi(), dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn EnumDesktopWindows(hdesktop: Option<HDESK>, lpfn: super::super::UI::WindowsAndMessaging::WNDENUMPROC, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn EnumDesktopWindows(hdesktop : HDESK, lpfn : super::super::UI::WindowsAndMessaging:: WNDENUMPROC, lparam : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumDesktopWindows(hdesktop.unwrap_or(core::mem::zeroed()) as _, lpfn, lparam).ok() }
}
#[inline]
pub unsafe fn EnumDesktopsA(hwinsta: Option<HWINSTA>, lpenumfunc: DESKTOPENUMPROCA, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn EnumDesktopsA(hwinsta : HWINSTA, lpenumfunc : DESKTOPENUMPROCA, lparam : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumDesktopsA(hwinsta.unwrap_or(core::mem::zeroed()) as _, lpenumfunc, lparam).ok() }
}
#[inline]
pub unsafe fn EnumDesktopsW(hwinsta: Option<HWINSTA>, lpenumfunc: DESKTOPENUMPROCW, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn EnumDesktopsW(hwinsta : HWINSTA, lpenumfunc : DESKTOPENUMPROCW, lparam : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumDesktopsW(hwinsta.unwrap_or(core::mem::zeroed()) as _, lpenumfunc, lparam).ok() }
}
#[inline]
pub unsafe fn EnumWindowStationsA(lpenumfunc: WINSTAENUMPROCA, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn EnumWindowStationsA(lpenumfunc : WINSTAENUMPROCA, lparam : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumWindowStationsA(lpenumfunc, lparam).ok() }
}
#[inline]
pub unsafe fn EnumWindowStationsW(lpenumfunc: WINSTAENUMPROCW, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn EnumWindowStationsW(lpenumfunc : WINSTAENUMPROCW, lparam : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumWindowStationsW(lpenumfunc, lparam).ok() }
}
#[inline]
pub unsafe fn GetProcessWindowStation() -> windows_core::Result<HWINSTA> {
    windows_core::link!("user32.dll" "system" fn GetProcessWindowStation() -> HWINSTA);
    let result__ = unsafe { GetProcessWindowStation() };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetThreadDesktop(dwthreadid: u32) -> windows_core::Result<HDESK> {
    windows_core::link!("user32.dll" "system" fn GetThreadDesktop(dwthreadid : u32) -> HDESK);
    let result__ = unsafe { GetThreadDesktop(dwthreadid) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetUserObjectInformationA(hobj: super::super::Foundation::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: Option<*mut core::ffi::c_void>, nlength: u32, lpnlengthneeded: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn GetUserObjectInformationA(hobj : super::super::Foundation:: HANDLE, nindex : USER_OBJECT_INFORMATION_INDEX, pvinfo : *mut core::ffi::c_void, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetUserObjectInformationA(hobj, nindex, pvinfo.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GetUserObjectInformationW(hobj: super::super::Foundation::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: Option<*mut core::ffi::c_void>, nlength: u32, lpnlengthneeded: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn GetUserObjectInformationW(hobj : super::super::Foundation:: HANDLE, nindex : USER_OBJECT_INFORMATION_INDEX, pvinfo : *mut core::ffi::c_void, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetUserObjectInformationW(hobj, nindex, pvinfo.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn OpenDesktopA<P0>(lpszdesktop: P0, dwflags: DESKTOP_CONTROL_FLAGS, finherit: bool, dwdesiredaccess: u32) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn OpenDesktopA(lpszdesktop : windows_core::PCSTR, dwflags : DESKTOP_CONTROL_FLAGS, finherit : windows_core::BOOL, dwdesiredaccess : u32) -> HDESK);
    let result__ = unsafe { OpenDesktopA(lpszdesktop.param().abi(), dwflags, finherit.into(), dwdesiredaccess) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn OpenDesktopW<P0>(lpszdesktop: P0, dwflags: DESKTOP_CONTROL_FLAGS, finherit: bool, dwdesiredaccess: u32) -> windows_core::Result<HDESK>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn OpenDesktopW(lpszdesktop : windows_core::PCWSTR, dwflags : DESKTOP_CONTROL_FLAGS, finherit : windows_core::BOOL, dwdesiredaccess : u32) -> HDESK);
    let result__ = unsafe { OpenDesktopW(lpszdesktop.param().abi(), dwflags, finherit.into(), dwdesiredaccess) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn OpenInputDesktop(dwflags: DESKTOP_CONTROL_FLAGS, finherit: bool, dwdesiredaccess: DESKTOP_ACCESS_FLAGS) -> windows_core::Result<HDESK> {
    windows_core::link!("user32.dll" "system" fn OpenInputDesktop(dwflags : DESKTOP_CONTROL_FLAGS, finherit : windows_core::BOOL, dwdesiredaccess : DESKTOP_ACCESS_FLAGS) -> HDESK);
    let result__ = unsafe { OpenInputDesktop(dwflags, finherit.into(), dwdesiredaccess) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn OpenWindowStationA<P0>(lpszwinsta: P0, finherit: bool, dwdesiredaccess: u32) -> windows_core::Result<HWINSTA>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn OpenWindowStationA(lpszwinsta : windows_core::PCSTR, finherit : windows_core::BOOL, dwdesiredaccess : u32) -> HWINSTA);
    let result__ = unsafe { OpenWindowStationA(lpszwinsta.param().abi(), finherit.into(), dwdesiredaccess) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn OpenWindowStationW<P0>(lpszwinsta: P0, finherit: bool, dwdesiredaccess: u32) -> windows_core::Result<HWINSTA>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn OpenWindowStationW(lpszwinsta : windows_core::PCWSTR, finherit : windows_core::BOOL, dwdesiredaccess : u32) -> HWINSTA);
    let result__ = unsafe { OpenWindowStationW(lpszwinsta.param().abi(), finherit.into(), dwdesiredaccess) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetProcessWindowStation(hwinsta: HWINSTA) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SetProcessWindowStation(hwinsta : HWINSTA) -> windows_core::BOOL);
    unsafe { SetProcessWindowStation(hwinsta).ok() }
}
#[inline]
pub unsafe fn SetThreadDesktop(hdesktop: HDESK) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SetThreadDesktop(hdesktop : HDESK) -> windows_core::BOOL);
    unsafe { SetThreadDesktop(hdesktop).ok() }
}
#[inline]
pub unsafe fn SetUserObjectInformationA(hobj: super::super::Foundation::HANDLE, nindex: i32, pvinfo: *const core::ffi::c_void, nlength: u32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SetUserObjectInformationA(hobj : super::super::Foundation:: HANDLE, nindex : i32, pvinfo : *const core::ffi::c_void, nlength : u32) -> windows_core::BOOL);
    unsafe { SetUserObjectInformationA(hobj, nindex, pvinfo, nlength).ok() }
}
#[inline]
pub unsafe fn SetUserObjectInformationW(hobj: super::super::Foundation::HANDLE, nindex: i32, pvinfo: *const core::ffi::c_void, nlength: u32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SetUserObjectInformationW(hobj : super::super::Foundation:: HANDLE, nindex : i32, pvinfo : *const core::ffi::c_void, nlength : u32) -> windows_core::BOOL);
    unsafe { SetUserObjectInformationW(hobj, nindex, pvinfo, nlength).ok() }
}
#[inline]
pub unsafe fn SwitchDesktop(hdesktop: HDESK) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SwitchDesktop(hdesktop : HDESK) -> windows_core::BOOL);
    unsafe { SwitchDesktop(hdesktop).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BROADCAST_SYSTEM_MESSAGE_FLAGS(pub u32);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BROADCAST_SYSTEM_MESSAGE_INFO(pub u32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BSMINFO {
    pub cbSize: u32,
    pub hdesk: HDESK,
    pub hwnd: super::super::Foundation::HWND,
    pub luid: super::super::Foundation::LUID,
}
pub const BSM_ALLCOMPONENTS: BROADCAST_SYSTEM_MESSAGE_INFO = BROADCAST_SYSTEM_MESSAGE_INFO(0u32);
pub const BSM_ALLDESKTOPS: BROADCAST_SYSTEM_MESSAGE_INFO = BROADCAST_SYSTEM_MESSAGE_INFO(16u32);
pub const BSM_APPLICATIONS: BROADCAST_SYSTEM_MESSAGE_INFO = BROADCAST_SYSTEM_MESSAGE_INFO(8u32);
pub type DESKTOPENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: super::super::Foundation::LPARAM) -> windows_core::BOOL>;
pub type DESKTOPENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::LPARAM) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DESKTOP_ACCESS_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DESKTOP_CONTROL_FLAGS(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDESK(pub *mut core::ffi::c_void);
impl HDESK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HDESK {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("user32.dll" "system" fn CloseDesktop(hdesktop : *mut core::ffi::c_void) -> i32);
            unsafe {
                CloseDesktop(self.0);
            }
        }
    }
}
impl Default for HDESK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWINSTA(pub *mut core::ffi::c_void);
impl HWINSTA {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HWINSTA {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("user32.dll" "system" fn CloseWindowStation(hwinsta : *mut core::ffi::c_void) -> i32);
            unsafe {
                CloseWindowStation(self.0);
            }
        }
    }
}
impl Default for HWINSTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UOI_FLAGS: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(1i32);
pub const UOI_HEAPSIZE: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(5i32);
pub const UOI_IO: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(6i32);
pub const UOI_NAME: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(2i32);
pub const UOI_TYPE: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(3i32);
pub const UOI_USER_SID: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USEROBJECTFLAGS {
    pub fInherit: windows_core::BOOL,
    pub fReserved: windows_core::BOOL,
    pub dwFlags: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_OBJECT_INFORMATION_INDEX(pub i32);
pub type WINSTAENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: super::super::Foundation::LPARAM) -> windows_core::BOOL>;
pub type WINSTAENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::LPARAM) -> windows_core::BOOL>;
