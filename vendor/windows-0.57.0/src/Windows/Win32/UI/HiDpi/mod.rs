#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn AdjustWindowRectExForDpi<P0>(lprect: *mut super::super::Foundation::RECT, dwstyle: super::WindowsAndMessaging::WINDOW_STYLE, bmenu: P0, dwexstyle: super::WindowsAndMessaging::WINDOW_EX_STYLE, dpi: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn AdjustWindowRectExForDpi(lprect : *mut super::super::Foundation:: RECT, dwstyle : super::WindowsAndMessaging:: WINDOW_STYLE, bmenu : super::super::Foundation:: BOOL, dwexstyle : super::WindowsAndMessaging:: WINDOW_EX_STYLE, dpi : u32) -> super::super::Foundation:: BOOL);
    AdjustWindowRectExForDpi(lprect, dwstyle, bmenu.param().abi(), dwexstyle, dpi).ok()
}
#[inline]
pub unsafe fn AreDpiAwarenessContextsEqual<P0, P1>(dpicontexta: P0, dpicontextb: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<DPI_AWARENESS_CONTEXT>,
    P1: windows_core::Param<DPI_AWARENESS_CONTEXT>,
{
    windows_targets::link!("user32.dll" "system" fn AreDpiAwarenessContextsEqual(dpicontexta : DPI_AWARENESS_CONTEXT, dpicontextb : DPI_AWARENESS_CONTEXT) -> super::super::Foundation:: BOOL);
    AreDpiAwarenessContextsEqual(dpicontexta.param().abi(), dpicontextb.param().abi())
}
#[inline]
pub unsafe fn EnableNonClientDpiScaling<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn EnableNonClientDpiScaling(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    EnableNonClientDpiScaling(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn GetAwarenessFromDpiAwarenessContext<P0>(value: P0) -> DPI_AWARENESS
where
    P0: windows_core::Param<DPI_AWARENESS_CONTEXT>,
{
    windows_targets::link!("user32.dll" "system" fn GetAwarenessFromDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS);
    GetAwarenessFromDpiAwarenessContext(value.param().abi())
}
#[inline]
pub unsafe fn GetDialogControlDpiChangeBehavior<P0>(hwnd: P0) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetDialogControlDpiChangeBehavior(hwnd : super::super::Foundation:: HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS);
    GetDialogControlDpiChangeBehavior(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetDialogDpiChangeBehavior<P0>(hdlg: P0) -> DIALOG_DPI_CHANGE_BEHAVIORS
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetDialogDpiChangeBehavior(hdlg : super::super::Foundation:: HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS);
    GetDialogDpiChangeBehavior(hdlg.param().abi())
}
#[inline]
pub unsafe fn GetDpiAwarenessContextForProcess<P0>(hprocess: P0) -> DPI_AWARENESS_CONTEXT
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn GetDpiAwarenessContextForProcess(hprocess : super::super::Foundation:: HANDLE) -> DPI_AWARENESS_CONTEXT);
    GetDpiAwarenessContextForProcess(hprocess.param().abi())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetDpiForMonitor<P0>(hmonitor: P0, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HMONITOR>,
{
    windows_targets::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn GetDpiForMonitor(hmonitor : super::super::Graphics::Gdi:: HMONITOR, dpitype : MONITOR_DPI_TYPE, dpix : *mut u32, dpiy : *mut u32) -> windows_core::HRESULT);
    GetDpiForMonitor(hmonitor.param().abi(), dpitype, dpix, dpiy).ok()
}
#[inline]
pub unsafe fn GetDpiForSystem() -> u32 {
    windows_targets::link!("user32.dll" "system" fn GetDpiForSystem() -> u32);
    GetDpiForSystem()
}
#[inline]
pub unsafe fn GetDpiForWindow<P0>(hwnd: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetDpiForWindow(hwnd : super::super::Foundation:: HWND) -> u32);
    GetDpiForWindow(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetDpiFromDpiAwarenessContext<P0>(value: P0) -> u32
where
    P0: windows_core::Param<DPI_AWARENESS_CONTEXT>,
{
    windows_targets::link!("user32.dll" "system" fn GetDpiFromDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> u32);
    GetDpiFromDpiAwarenessContext(value.param().abi())
}
#[inline]
pub unsafe fn GetProcessDpiAwareness<P0>(hprocess: P0) -> windows_core::Result<PROCESS_DPI_AWARENESS>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn GetProcessDpiAwareness(hprocess : super::super::Foundation:: HANDLE, value : *mut PROCESS_DPI_AWARENESS) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetProcessDpiAwareness(hprocess.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetSystemDpiForProcess<P0>(hprocess: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn GetSystemDpiForProcess(hprocess : super::super::Foundation:: HANDLE) -> u32);
    GetSystemDpiForProcess(hprocess.param().abi())
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetSystemMetricsForDpi(nindex: super::WindowsAndMessaging::SYSTEM_METRICS_INDEX, dpi: u32) -> i32 {
    windows_targets::link!("user32.dll" "system" fn GetSystemMetricsForDpi(nindex : super::WindowsAndMessaging:: SYSTEM_METRICS_INDEX, dpi : u32) -> i32);
    GetSystemMetricsForDpi(nindex, dpi)
}
#[inline]
pub unsafe fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT {
    windows_targets::link!("user32.dll" "system" fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT);
    GetThreadDpiAwarenessContext()
}
#[inline]
pub unsafe fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR {
    windows_targets::link!("user32.dll" "system" fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR);
    GetThreadDpiHostingBehavior()
}
#[inline]
pub unsafe fn GetWindowDpiAwarenessContext<P0>(hwnd: P0) -> DPI_AWARENESS_CONTEXT
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowDpiAwarenessContext(hwnd : super::super::Foundation:: HWND) -> DPI_AWARENESS_CONTEXT);
    GetWindowDpiAwarenessContext(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetWindowDpiHostingBehavior<P0>(hwnd: P0) -> DPI_HOSTING_BEHAVIOR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn GetWindowDpiHostingBehavior(hwnd : super::super::Foundation:: HWND) -> DPI_HOSTING_BEHAVIOR);
    GetWindowDpiHostingBehavior(hwnd.param().abi())
}
#[inline]
pub unsafe fn IsValidDpiAwarenessContext<P0>(value: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<DPI_AWARENESS_CONTEXT>,
{
    windows_targets::link!("user32.dll" "system" fn IsValidDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> super::super::Foundation:: BOOL);
    IsValidDpiAwarenessContext(value.param().abi())
}
#[inline]
pub unsafe fn LogicalToPhysicalPointForPerMonitorDPI<P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn LogicalToPhysicalPointForPerMonitorDPI(hwnd : super::super::Foundation:: HWND, lppoint : *mut super::super::Foundation:: POINT) -> super::super::Foundation:: BOOL);
    LogicalToPhysicalPointForPerMonitorDPI(hwnd.param().abi(), lppoint)
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn OpenThemeDataForDpi<P0, P1>(hwnd: P0, pszclasslist: P1, dpi: u32) -> super::Controls::HTHEME
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("uxtheme.dll" "system" fn OpenThemeDataForDpi(hwnd : super::super::Foundation:: HWND, pszclasslist : windows_core::PCWSTR, dpi : u32) -> super::Controls:: HTHEME);
    OpenThemeDataForDpi(hwnd.param().abi(), pszclasslist.param().abi(), dpi)
}
#[inline]
pub unsafe fn PhysicalToLogicalPointForPerMonitorDPI<P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn PhysicalToLogicalPointForPerMonitorDPI(hwnd : super::super::Foundation:: HWND, lppoint : *mut super::super::Foundation:: POINT) -> super::super::Foundation:: BOOL);
    PhysicalToLogicalPointForPerMonitorDPI(hwnd.param().abi(), lppoint)
}
#[inline]
pub unsafe fn SetDialogControlDpiChangeBehavior<P0>(hwnd: P0, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetDialogControlDpiChangeBehavior(hwnd : super::super::Foundation:: HWND, mask : DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values : DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation:: BOOL);
    SetDialogControlDpiChangeBehavior(hwnd.param().abi(), mask, values).ok()
}
#[inline]
pub unsafe fn SetDialogDpiChangeBehavior<P0>(hdlg: P0, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("user32.dll" "system" fn SetDialogDpiChangeBehavior(hdlg : super::super::Foundation:: HWND, mask : DIALOG_DPI_CHANGE_BEHAVIORS, values : DIALOG_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation:: BOOL);
    SetDialogDpiChangeBehavior(hdlg.param().abi(), mask, values).ok()
}
#[inline]
pub unsafe fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-shcore-scaling-l1-1-1.dll" "system" fn SetProcessDpiAwareness(value : PROCESS_DPI_AWARENESS) -> windows_core::HRESULT);
    SetProcessDpiAwareness(value).ok()
}
#[inline]
pub unsafe fn SetProcessDpiAwarenessContext<P0>(value: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<DPI_AWARENESS_CONTEXT>,
{
    windows_targets::link!("user32.dll" "system" fn SetProcessDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> super::super::Foundation:: BOOL);
    SetProcessDpiAwarenessContext(value.param().abi()).ok()
}
#[inline]
pub unsafe fn SetThreadDpiAwarenessContext<P0>(dpicontext: P0) -> DPI_AWARENESS_CONTEXT
where
    P0: windows_core::Param<DPI_AWARENESS_CONTEXT>,
{
    windows_targets::link!("user32.dll" "system" fn SetThreadDpiAwarenessContext(dpicontext : DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS_CONTEXT);
    SetThreadDpiAwarenessContext(dpicontext.param().abi())
}
#[inline]
pub unsafe fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR {
    windows_targets::link!("user32.dll" "system" fn SetThreadDpiHostingBehavior(value : DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR);
    SetThreadDpiHostingBehavior(value)
}
#[inline]
pub unsafe fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: Option<*mut core::ffi::c_void>, fwinini: u32, dpi: u32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn SystemParametersInfoForDpi(uiaction : u32, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : u32, dpi : u32) -> super::super::Foundation:: BOOL);
    SystemParametersInfoForDpi(uiaction, uiparam, core::mem::transmute(pvparam.unwrap_or(std::ptr::null_mut())), fwinini, dpi).ok()
}
pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(0i32);
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(1i32);
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(2i32);
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(0i32);
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(1i32);
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(4i32);
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(2i32);
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-3i32 as _);
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-4i32 as _);
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-2i32 as _);
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-1i32 as _);
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-5i32 as _);
pub const DPI_AWARENESS_INVALID: DPI_AWARENESS = DPI_AWARENESS(-1i32);
pub const DPI_AWARENESS_PER_MONITOR_AWARE: DPI_AWARENESS = DPI_AWARENESS(2i32);
pub const DPI_AWARENESS_SYSTEM_AWARE: DPI_AWARENESS = DPI_AWARENESS(1i32);
pub const DPI_AWARENESS_UNAWARE: DPI_AWARENESS = DPI_AWARENESS(0i32);
pub const DPI_HOSTING_BEHAVIOR_DEFAULT: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(0i32);
pub const DPI_HOSTING_BEHAVIOR_INVALID: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(-1i32);
pub const DPI_HOSTING_BEHAVIOR_MIXED: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(1i32);
pub const MDT_ANGULAR_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(1i32);
pub const MDT_DEFAULT: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(0i32);
pub const MDT_EFFECTIVE_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(0i32);
pub const MDT_RAW_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(2i32);
pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(0i32);
pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(2i32);
pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(pub i32);
impl windows_core::TypeKind for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS").field(&self.0).finish()
    }
}
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
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIALOG_DPI_CHANGE_BEHAVIORS(pub i32);
impl windows_core::TypeKind for DIALOG_DPI_CHANGE_BEHAVIORS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIALOG_DPI_CHANGE_BEHAVIORS").field(&self.0).finish()
    }
}
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
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DPI_AWARENESS(pub i32);
impl windows_core::TypeKind for DPI_AWARENESS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DPI_AWARENESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DPI_AWARENESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DPI_HOSTING_BEHAVIOR(pub i32);
impl windows_core::TypeKind for DPI_HOSTING_BEHAVIOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DPI_HOSTING_BEHAVIOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DPI_HOSTING_BEHAVIOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MONITOR_DPI_TYPE(pub i32);
impl windows_core::TypeKind for MONITOR_DPI_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MONITOR_DPI_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MONITOR_DPI_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROCESS_DPI_AWARENESS(pub i32);
impl windows_core::TypeKind for PROCESS_DPI_AWARENESS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROCESS_DPI_AWARENESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROCESS_DPI_AWARENESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DPI_AWARENESS_CONTEXT(pub isize);
impl DPI_AWARENESS_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for DPI_AWARENESS_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DPI_AWARENESS_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
