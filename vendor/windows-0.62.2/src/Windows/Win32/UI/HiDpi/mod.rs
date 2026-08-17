#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn AdjustWindowRectExForDpi(lprect: *mut super::super::Foundation::RECT, dwstyle: super::WindowsAndMessaging::WINDOW_STYLE, bmenu: bool, dwexstyle: super::WindowsAndMessaging::WINDOW_EX_STYLE, dpi: u32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn AdjustWindowRectExForDpi(lprect : *mut super::super::Foundation:: RECT, dwstyle : super::WindowsAndMessaging:: WINDOW_STYLE, bmenu : windows_core::BOOL, dwexstyle : super::WindowsAndMessaging:: WINDOW_EX_STYLE, dpi : u32) -> windows_core::BOOL);
    unsafe { AdjustWindowRectExForDpi(lprect as _, dwstyle, bmenu.into(), dwexstyle, dpi).ok() }
}
#[inline]
pub unsafe fn AreDpiAwarenessContextsEqual(dpicontexta: DPI_AWARENESS_CONTEXT, dpicontextb: DPI_AWARENESS_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AreDpiAwarenessContextsEqual(dpicontexta : DPI_AWARENESS_CONTEXT, dpicontextb : DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
    unsafe { AreDpiAwarenessContextsEqual(dpicontexta, dpicontextb) }
}
#[inline]
pub unsafe fn EnableNonClientDpiScaling(hwnd: super::super::Foundation::HWND) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn EnableNonClientDpiScaling(hwnd : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { EnableNonClientDpiScaling(hwnd).ok() }
}
#[inline]
pub unsafe fn GetAwarenessFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS {
    windows_core::link!("user32.dll" "system" fn GetAwarenessFromDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS);
    unsafe { GetAwarenessFromDpiAwarenessContext(value) }
}
#[inline]
pub unsafe fn GetDialogControlDpiChangeBehavior(hwnd: super::super::Foundation::HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    windows_core::link!("user32.dll" "system" fn GetDialogControlDpiChangeBehavior(hwnd : super::super::Foundation:: HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS);
    unsafe { GetDialogControlDpiChangeBehavior(hwnd) }
}
#[inline]
pub unsafe fn GetDialogDpiChangeBehavior(hdlg: super::super::Foundation::HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS {
    windows_core::link!("user32.dll" "system" fn GetDialogDpiChangeBehavior(hdlg : super::super::Foundation:: HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS);
    unsafe { GetDialogDpiChangeBehavior(hdlg) }
}
#[inline]
pub unsafe fn GetDpiAwarenessContextForProcess(hprocess: super::super::Foundation::HANDLE) -> DPI_AWARENESS_CONTEXT {
    windows_core::link!("user32.dll" "system" fn GetDpiAwarenessContextForProcess(hprocess : super::super::Foundation:: HANDLE) -> DPI_AWARENESS_CONTEXT);
    unsafe { GetDpiAwarenessContextForProcess(hprocess) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetDpiForMonitor(hmonitor: super::super::Graphics::Gdi::HMONITOR, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn GetDpiForMonitor(hmonitor : super::super::Graphics::Gdi:: HMONITOR, dpitype : MONITOR_DPI_TYPE, dpix : *mut u32, dpiy : *mut u32) -> windows_core::HRESULT);
    unsafe { GetDpiForMonitor(hmonitor, dpitype, dpix as _, dpiy as _).ok() }
}
#[inline]
pub unsafe fn GetDpiForSystem() -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDpiForSystem() -> u32);
    unsafe { GetDpiForSystem() }
}
#[inline]
pub unsafe fn GetDpiForWindow(hwnd: super::super::Foundation::HWND) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDpiForWindow(hwnd : super::super::Foundation:: HWND) -> u32);
    unsafe { GetDpiForWindow(hwnd) }
}
#[inline]
pub unsafe fn GetDpiFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDpiFromDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> u32);
    unsafe { GetDpiFromDpiAwarenessContext(value) }
}
#[inline]
pub unsafe fn GetProcessDpiAwareness(hprocess: Option<super::super::Foundation::HANDLE>) -> windows_core::Result<PROCESS_DPI_AWARENESS> {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn GetProcessDpiAwareness(hprocess : super::super::Foundation:: HANDLE, value : *mut PROCESS_DPI_AWARENESS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetProcessDpiAwareness(hprocess.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetSystemDpiForProcess(hprocess: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetSystemDpiForProcess(hprocess : super::super::Foundation:: HANDLE) -> u32);
    unsafe { GetSystemDpiForProcess(hprocess) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetSystemMetricsForDpi(nindex: super::WindowsAndMessaging::SYSTEM_METRICS_INDEX, dpi: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetSystemMetricsForDpi(nindex : super::WindowsAndMessaging:: SYSTEM_METRICS_INDEX, dpi : u32) -> i32);
    unsafe { GetSystemMetricsForDpi(nindex, dpi) }
}
#[inline]
pub unsafe fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT {
    windows_core::link!("user32.dll" "system" fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT);
    unsafe { GetThreadDpiAwarenessContext() }
}
#[inline]
pub unsafe fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR {
    windows_core::link!("user32.dll" "system" fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR);
    unsafe { GetThreadDpiHostingBehavior() }
}
#[inline]
pub unsafe fn GetWindowDpiAwarenessContext(hwnd: super::super::Foundation::HWND) -> DPI_AWARENESS_CONTEXT {
    windows_core::link!("user32.dll" "system" fn GetWindowDpiAwarenessContext(hwnd : super::super::Foundation:: HWND) -> DPI_AWARENESS_CONTEXT);
    unsafe { GetWindowDpiAwarenessContext(hwnd) }
}
#[inline]
pub unsafe fn GetWindowDpiHostingBehavior(hwnd: super::super::Foundation::HWND) -> DPI_HOSTING_BEHAVIOR {
    windows_core::link!("user32.dll" "system" fn GetWindowDpiHostingBehavior(hwnd : super::super::Foundation:: HWND) -> DPI_HOSTING_BEHAVIOR);
    unsafe { GetWindowDpiHostingBehavior(hwnd) }
}
#[inline]
pub unsafe fn IsValidDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsValidDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
    unsafe { IsValidDpiAwarenessContext(value) }
}
#[inline]
pub unsafe fn LogicalToPhysicalPointForPerMonitorDPI(hwnd: Option<super::super::Foundation::HWND>, lppoint: *mut super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn LogicalToPhysicalPointForPerMonitorDPI(hwnd : super::super::Foundation:: HWND, lppoint : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { LogicalToPhysicalPointForPerMonitorDPI(hwnd.unwrap_or(core::mem::zeroed()) as _, lppoint as _) }
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn OpenThemeDataForDpi<P1>(hwnd: Option<super::super::Foundation::HWND>, pszclasslist: P1, dpi: u32) -> super::Controls::HTHEME
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("uxtheme.dll" "system" fn OpenThemeDataForDpi(hwnd : super::super::Foundation:: HWND, pszclasslist : windows_core::PCWSTR, dpi : u32) -> super::Controls:: HTHEME);
    unsafe { OpenThemeDataForDpi(hwnd.unwrap_or(core::mem::zeroed()) as _, pszclasslist.param().abi(), dpi) }
}
#[inline]
pub unsafe fn PhysicalToLogicalPointForPerMonitorDPI(hwnd: Option<super::super::Foundation::HWND>, lppoint: *mut super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PhysicalToLogicalPointForPerMonitorDPI(hwnd : super::super::Foundation:: HWND, lppoint : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { PhysicalToLogicalPointForPerMonitorDPI(hwnd.unwrap_or(core::mem::zeroed()) as _, lppoint as _) }
}
#[inline]
pub unsafe fn SetDialogControlDpiChangeBehavior(hwnd: super::super::Foundation::HWND, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SetDialogControlDpiChangeBehavior(hwnd : super::super::Foundation:: HWND, mask : DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values : DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> windows_core::BOOL);
    unsafe { SetDialogControlDpiChangeBehavior(hwnd, mask, values).ok() }
}
#[inline]
pub unsafe fn SetDialogDpiChangeBehavior(hdlg: super::super::Foundation::HWND, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SetDialogDpiChangeBehavior(hdlg : super::super::Foundation:: HWND, mask : DIALOG_DPI_CHANGE_BEHAVIORS, values : DIALOG_DPI_CHANGE_BEHAVIORS) -> windows_core::BOOL);
    unsafe { SetDialogDpiChangeBehavior(hdlg, mask, values).ok() }
}
#[inline]
pub unsafe fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn SetProcessDpiAwareness(value : PROCESS_DPI_AWARENESS) -> windows_core::HRESULT);
    unsafe { SetProcessDpiAwareness(value).ok() }
}
#[inline]
pub unsafe fn SetProcessDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SetProcessDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
    unsafe { SetProcessDpiAwarenessContext(value).ok() }
}
#[inline]
pub unsafe fn SetThreadDpiAwarenessContext(dpicontext: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS_CONTEXT {
    windows_core::link!("user32.dll" "system" fn SetThreadDpiAwarenessContext(dpicontext : DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS_CONTEXT);
    unsafe { SetThreadDpiAwarenessContext(dpicontext) }
}
#[inline]
pub unsafe fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR {
    windows_core::link!("user32.dll" "system" fn SetThreadDpiHostingBehavior(value : DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR);
    unsafe { SetThreadDpiHostingBehavior(value) }
}
#[inline]
pub unsafe fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: Option<*mut core::ffi::c_void>, fwinini: u32, dpi: u32) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn SystemParametersInfoForDpi(uiaction : u32, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : u32, dpi : u32) -> windows_core::BOOL);
    unsafe { SystemParametersInfoForDpi(uiaction, uiparam, pvparam.unwrap_or(core::mem::zeroed()) as _, fwinini, dpi).ok() }
}
pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(0i32);
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(1i32);
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(2i32);
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(0i32);
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(1i32);
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(4i32);
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(pub i32);
impl DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIALOG_DPI_CHANGE_BEHAVIORS(pub i32);
impl DIALOG_DPI_CHANGE_BEHAVIORS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DPI_AWARENESS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DPI_AWARENESS_CONTEXT(pub *mut core::ffi::c_void);
impl DPI_AWARENESS_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for DPI_AWARENESS_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-3i32 as _);
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-4i32 as _);
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-2i32 as _);
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-1i32 as _);
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-5i32 as _);
pub const DPI_AWARENESS_INVALID: DPI_AWARENESS = DPI_AWARENESS(-1i32);
pub const DPI_AWARENESS_PER_MONITOR_AWARE: DPI_AWARENESS = DPI_AWARENESS(2i32);
pub const DPI_AWARENESS_SYSTEM_AWARE: DPI_AWARENESS = DPI_AWARENESS(1i32);
pub const DPI_AWARENESS_UNAWARE: DPI_AWARENESS = DPI_AWARENESS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DPI_HOSTING_BEHAVIOR(pub i32);
pub const DPI_HOSTING_BEHAVIOR_DEFAULT: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(0i32);
pub const DPI_HOSTING_BEHAVIOR_INVALID: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(-1i32);
pub const DPI_HOSTING_BEHAVIOR_MIXED: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(1i32);
pub const MDT_ANGULAR_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(1i32);
pub const MDT_DEFAULT: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(0i32);
pub const MDT_EFFECTIVE_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(0i32);
pub const MDT_RAW_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MONITOR_DPI_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_DPI_AWARENESS(pub i32);
pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(0i32);
pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(2i32);
pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(1i32);
