#[cfg(feature = "Win32_Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
#[inline]
pub unsafe fn AbortPrinter(hprinter: PRINTER_HANDLE) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn AbortPrinter(hprinter : PRINTER_HANDLE) -> windows_core::BOOL);
    unsafe { AbortPrinter(core::mem::transmute(hprinter)) }
}
#[inline]
pub unsafe fn AddFormA(hprinter: PRINTER_HANDLE, level: u32, pform: *const u8) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn AddFormA(hprinter : PRINTER_HANDLE, level : u32, pform : *const u8) -> windows_core::BOOL);
    unsafe { AddFormA(core::mem::transmute(hprinter), level, pform) }
}
#[inline]
pub unsafe fn AddFormW(hprinter: PRINTER_HANDLE, level: u32, pform: *const u8) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn AddFormW(hprinter : PRINTER_HANDLE, level : u32, pform : *const u8) -> windows_core::BOOL);
    unsafe { AddFormW(core::mem::transmute(hprinter), level, pform) }
}
#[inline]
pub unsafe fn AddJobA(hprinter: PRINTER_HANDLE, level: u32, pdata: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn AddJobA(hprinter : PRINTER_HANDLE, level : u32, pdata : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { AddJobA(core::mem::transmute(hprinter), level, core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn AddJobW(hprinter: PRINTER_HANDLE, level: u32, pdata: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn AddJobW(hprinter : PRINTER_HANDLE, level : u32, pdata : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { AddJobW(core::mem::transmute(hprinter), level, core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn AddMonitorA<P0>(pname: P0, level: u32, pmonitors: Option<*const u8>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddMonitorA(pname : windows_core::PCSTR, level : u32, pmonitors : *const u8) -> windows_core::BOOL);
    unsafe { AddMonitorA(pname.param().abi(), level, pmonitors.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AddMonitorW<P0>(pname: P0, level: u32, pmonitors: Option<*const u8>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddMonitorW(pname : windows_core::PCWSTR, level : u32, pmonitors : *const u8) -> windows_core::BOOL);
    unsafe { AddMonitorW(pname.param().abi(), level, pmonitors.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AddPortA<P0, P2>(pname: P0, hwnd: super::super::Foundation::HWND, pmonitorname: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPortA(pname : windows_core::PCSTR, hwnd : super::super::Foundation:: HWND, pmonitorname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { AddPortA(pname.param().abi(), hwnd, pmonitorname.param().abi()).ok() }
}
#[inline]
pub unsafe fn AddPortW<P0, P2>(pname: P0, hwnd: super::super::Foundation::HWND, pmonitorname: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPortW(pname : windows_core::PCWSTR, hwnd : super::super::Foundation:: HWND, pmonitorname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AddPortW(pname.param().abi(), hwnd, pmonitorname.param().abi()).ok() }
}
#[inline]
pub unsafe fn AddPrintDeviceObject(hprinter: PRINTER_HANDLE) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("spoolss.dll" "system" fn AddPrintDeviceObject(hprinter : PRINTER_HANDLE, phdeviceobject : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        AddPrintDeviceObject(core::mem::transmute(hprinter), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn AddPrintProcessorA<P0, P1, P2, P3>(pname: P0, penvironment: P1, ppathname: P2, pprintprocessorname: P3) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrintProcessorA(pname : windows_core::PCSTR, penvironment : windows_core::PCSTR, ppathname : windows_core::PCSTR, pprintprocessorname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { AddPrintProcessorA(pname.param().abi(), penvironment.param().abi(), ppathname.param().abi(), pprintprocessorname.param().abi()) }
}
#[inline]
pub unsafe fn AddPrintProcessorW<P0, P1, P2, P3>(pname: P0, penvironment: P1, ppathname: P2, pprintprocessorname: P3) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrintProcessorW(pname : windows_core::PCWSTR, penvironment : windows_core::PCWSTR, ppathname : windows_core::PCWSTR, pprintprocessorname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AddPrintProcessorW(pname.param().abi(), penvironment.param().abi(), ppathname.param().abi(), pprintprocessorname.param().abi()) }
}
#[inline]
pub unsafe fn AddPrintProvidorA<P0>(pname: P0, level: u32, pprovidorinfo: *const u8) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrintProvidorA(pname : windows_core::PCSTR, level : u32, pprovidorinfo : *const u8) -> windows_core::BOOL);
    unsafe { AddPrintProvidorA(pname.param().abi(), level, pprovidorinfo) }
}
#[inline]
pub unsafe fn AddPrintProvidorW<P0>(pname: P0, level: u32, pprovidorinfo: *const u8) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrintProvidorW(pname : windows_core::PCWSTR, level : u32, pprovidorinfo : *const u8) -> windows_core::BOOL);
    unsafe { AddPrintProvidorW(pname.param().abi(), level, pprovidorinfo) }
}
#[inline]
pub unsafe fn AddPrinterA<P0>(pname: P0, level: u32, pprinter: *const u8) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrinterA(pname : windows_core::PCSTR, level : u32, pprinter : *const u8) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { AddPrinterA(pname.param().abi(), level, pprinter) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn AddPrinterConnection2A<P1>(hwnd: Option<super::super::Foundation::HWND>, pszname: P1, dwlevel: u32, pconnectioninfo: *const core::ffi::c_void) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrinterConnection2A(hwnd : super::super::Foundation:: HWND, pszname : windows_core::PCSTR, dwlevel : u32, pconnectioninfo : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { AddPrinterConnection2A(hwnd.unwrap_or(core::mem::zeroed()) as _, pszname.param().abi(), dwlevel, pconnectioninfo) }
}
#[inline]
pub unsafe fn AddPrinterConnection2W<P1>(hwnd: Option<super::super::Foundation::HWND>, pszname: P1, dwlevel: u32, pconnectioninfo: *const core::ffi::c_void) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrinterConnection2W(hwnd : super::super::Foundation:: HWND, pszname : windows_core::PCWSTR, dwlevel : u32, pconnectioninfo : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { AddPrinterConnection2W(hwnd.unwrap_or(core::mem::zeroed()) as _, pszname.param().abi(), dwlevel, pconnectioninfo) }
}
#[inline]
pub unsafe fn AddPrinterConnectionA<P0>(pname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrinterConnectionA(pname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { AddPrinterConnectionA(pname.param().abi()) }
}
#[inline]
pub unsafe fn AddPrinterConnectionW<P0>(pname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrinterConnectionW(pname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AddPrinterConnectionW(pname.param().abi()) }
}
#[inline]
pub unsafe fn AddPrinterDriverA<P0>(pname: P0, level: u32, pdriverinfo: *const u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrinterDriverA(pname : windows_core::PCSTR, level : u32, pdriverinfo : *const u8) -> windows_core::BOOL);
    unsafe { AddPrinterDriverA(pname.param().abi(), level, pdriverinfo).ok() }
}
#[inline]
pub unsafe fn AddPrinterDriverExA<P0>(pname: P0, level: u32, lpbdriverinfo: *const u8, dwfilecopyflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrinterDriverExA(pname : windows_core::PCSTR, level : u32, lpbdriverinfo : *const u8, dwfilecopyflags : u32) -> windows_core::BOOL);
    unsafe { AddPrinterDriverExA(pname.param().abi(), level, lpbdriverinfo, dwfilecopyflags) }
}
#[inline]
pub unsafe fn AddPrinterDriverExW<P0>(pname: P0, level: u32, lpbdriverinfo: *const u8, dwfilecopyflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrinterDriverExW(pname : windows_core::PCWSTR, level : u32, lpbdriverinfo : *const u8, dwfilecopyflags : u32) -> windows_core::BOOL);
    unsafe { AddPrinterDriverExW(pname.param().abi(), level, lpbdriverinfo, dwfilecopyflags) }
}
#[inline]
pub unsafe fn AddPrinterDriverW<P0>(pname: P0, level: u32, pdriverinfo: *const u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrinterDriverW(pname : windows_core::PCWSTR, level : u32, pdriverinfo : *const u8) -> windows_core::BOOL);
    unsafe { AddPrinterDriverW(pname.param().abi(), level, pdriverinfo).ok() }
}
#[inline]
pub unsafe fn AddPrinterW<P0>(pname: P0, level: u32, pprinter: *const u8) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AddPrinterW(pname : windows_core::PCWSTR, level : u32, pprinter : *const u8) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { AddPrinterW(pname.param().abi(), level, pprinter) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn AdvancedDocumentPropertiesA<P2>(hwnd: super::super::Foundation::HWND, hprinter: PRINTER_HANDLE, pdevicename: P2, pdevmodeoutput: Option<*mut super::Gdi::DEVMODEA>, pdevmodeinput: Option<*const super::Gdi::DEVMODEA>) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AdvancedDocumentPropertiesA(hwnd : super::super::Foundation:: HWND, hprinter : PRINTER_HANDLE, pdevicename : windows_core::PCSTR, pdevmodeoutput : *mut super::Gdi:: DEVMODEA, pdevmodeinput : *const super::Gdi:: DEVMODEA) -> i32);
    unsafe { AdvancedDocumentPropertiesA(hwnd, core::mem::transmute(hprinter), pdevicename.param().abi(), pdevmodeoutput.unwrap_or(core::mem::zeroed()) as _, pdevmodeinput.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn AdvancedDocumentPropertiesW<P2>(hwnd: super::super::Foundation::HWND, hprinter: PRINTER_HANDLE, pdevicename: P2, pdevmodeoutput: Option<*mut super::Gdi::DEVMODEW>, pdevmodeinput: Option<*const super::Gdi::DEVMODEW>) -> i32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn AdvancedDocumentPropertiesW(hwnd : super::super::Foundation:: HWND, hprinter : PRINTER_HANDLE, pdevicename : windows_core::PCWSTR, pdevmodeoutput : *mut super::Gdi:: DEVMODEW, pdevmodeinput : *const super::Gdi:: DEVMODEW) -> i32);
    unsafe { AdvancedDocumentPropertiesW(hwnd, core::mem::transmute(hprinter), pdevicename.param().abi(), pdevmodeoutput.unwrap_or(core::mem::zeroed()) as _, pdevmodeinput.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AppendPrinterNotifyInfoData(pinfodest: *const PRINTER_NOTIFY_INFO, pdatasrc: Option<*const PRINTER_NOTIFY_INFO_DATA>, fdwflags: u32) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn AppendPrinterNotifyInfoData(pinfodest : *const PRINTER_NOTIFY_INFO, pdatasrc : *const PRINTER_NOTIFY_INFO_DATA, fdwflags : u32) -> windows_core::BOOL);
    unsafe { AppendPrinterNotifyInfoData(pinfodest, pdatasrc.unwrap_or(core::mem::zeroed()) as _, fdwflags) }
}
#[inline]
pub unsafe fn CallRouterFindFirstPrinterChangeNotification(hprinterrpc: super::super::Foundation::HANDLE, fdwfilterflags: u32, fdwoptions: u32, hnotify: super::super::Foundation::HANDLE, pprinternotifyoptions: *const PRINTER_NOTIFY_OPTIONS) -> u32 {
    windows_core::link!("spoolss.dll" "system" fn CallRouterFindFirstPrinterChangeNotification(hprinterrpc : super::super::Foundation:: HANDLE, fdwfilterflags : u32, fdwoptions : u32, hnotify : super::super::Foundation:: HANDLE, pprinternotifyoptions : *const PRINTER_NOTIFY_OPTIONS) -> u32);
    unsafe { CallRouterFindFirstPrinterChangeNotification(hprinterrpc, fdwfilterflags, fdwoptions, hnotify, pprinternotifyoptions) }
}
#[inline]
pub unsafe fn ClosePrinter(hprinter: PRINTER_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("winspool.drv" "system" fn ClosePrinter(hprinter : PRINTER_HANDLE) -> windows_core::BOOL);
    unsafe { ClosePrinter(core::mem::transmute(hprinter)).ok() }
}
#[inline]
pub unsafe fn CloseSpoolFileHandle(hprinter: PRINTER_HANDLE, hspoolfile: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn CloseSpoolFileHandle(hprinter : PRINTER_HANDLE, hspoolfile : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { CloseSpoolFileHandle(core::mem::transmute(hprinter), hspoolfile) }
}
#[inline]
pub unsafe fn CommitSpoolData(hprinter: PRINTER_HANDLE, hspoolfile: super::super::Foundation::HANDLE, cbcommit: u32) -> super::super::Foundation::HANDLE {
    windows_core::link!("winspool.drv" "system" fn CommitSpoolData(hprinter : PRINTER_HANDLE, hspoolfile : super::super::Foundation:: HANDLE, cbcommit : u32) -> super::super::Foundation:: HANDLE);
    unsafe { CommitSpoolData(core::mem::transmute(hprinter), hspoolfile, cbcommit) }
}
#[inline]
pub unsafe fn CommonPropertySheetUIA(hwndowner: super::super::Foundation::HWND, pfnpropsheetui: PFNPROPSHEETUI, lparam: super::super::Foundation::LPARAM, presult: Option<*mut u32>) -> i32 {
    windows_core::link!("compstui.dll" "system" fn CommonPropertySheetUIA(hwndowner : super::super::Foundation:: HWND, pfnpropsheetui : PFNPROPSHEETUI, lparam : super::super::Foundation:: LPARAM, presult : *mut u32) -> i32);
    unsafe { CommonPropertySheetUIA(hwndowner, pfnpropsheetui, lparam, presult.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CommonPropertySheetUIW(hwndowner: super::super::Foundation::HWND, pfnpropsheetui: PFNPROPSHEETUI, lparam: super::super::Foundation::LPARAM, presult: Option<*mut u32>) -> i32 {
    windows_core::link!("compstui.dll" "system" fn CommonPropertySheetUIW(hwndowner : super::super::Foundation:: HWND, pfnpropsheetui : PFNPROPSHEETUI, lparam : super::super::Foundation:: LPARAM, presult : *mut u32) -> i32);
    unsafe { CommonPropertySheetUIW(hwndowner, pfnpropsheetui, lparam, presult.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ConfigurePortA<P0, P2>(pname: P0, hwnd: super::super::Foundation::HWND, pportname: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn ConfigurePortA(pname : windows_core::PCSTR, hwnd : super::super::Foundation:: HWND, pportname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ConfigurePortA(pname.param().abi(), hwnd, pportname.param().abi()) }
}
#[inline]
pub unsafe fn ConfigurePortW<P0, P2>(pname: P0, hwnd: super::super::Foundation::HWND, pportname: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn ConfigurePortW(pname : windows_core::PCWSTR, hwnd : super::super::Foundation:: HWND, pportname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ConfigurePortW(pname.param().abi(), hwnd, pportname.param().abi()) }
}
#[inline]
pub unsafe fn ConnectToPrinterDlg(hwnd: super::super::Foundation::HWND, flags: u32) -> super::super::Foundation::HANDLE {
    windows_core::link!("winspool.drv" "system" fn ConnectToPrinterDlg(hwnd : super::super::Foundation:: HWND, flags : u32) -> super::super::Foundation:: HANDLE);
    unsafe { ConnectToPrinterDlg(hwnd, flags) }
}
#[inline]
pub unsafe fn CorePrinterDriverInstalledA<P0, P1>(pszserver: P0, pszenvironment: P1, coredriverguid: windows_core::GUID, ftdriverdate: super::super::Foundation::FILETIME, dwldriverversion: u64) -> windows_core::Result<windows_core::BOOL>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn CorePrinterDriverInstalledA(pszserver : windows_core::PCSTR, pszenvironment : windows_core::PCSTR, coredriverguid : windows_core::GUID, ftdriverdate : super::super::Foundation:: FILETIME, dwldriverversion : u64, pbdriverinstalled : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CorePrinterDriverInstalledA(pszserver.param().abi(), pszenvironment.param().abi(), core::mem::transmute(coredriverguid), core::mem::transmute(ftdriverdate), dwldriverversion, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CorePrinterDriverInstalledW<P0, P1>(pszserver: P0, pszenvironment: P1, coredriverguid: windows_core::GUID, ftdriverdate: super::super::Foundation::FILETIME, dwldriverversion: u64) -> windows_core::Result<windows_core::BOOL>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn CorePrinterDriverInstalledW(pszserver : windows_core::PCWSTR, pszenvironment : windows_core::PCWSTR, coredriverguid : windows_core::GUID, ftdriverdate : super::super::Foundation:: FILETIME, dwldriverversion : u64, pbdriverinstalled : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CorePrinterDriverInstalledW(pszserver.param().abi(), pszenvironment.param().abi(), core::mem::transmute(coredriverguid), core::mem::transmute(ftdriverdate), dwldriverversion, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CreatePrintAsyncNotifyChannel<P0, P4>(pszname: P0, pnotificationtype: *const windows_core::GUID, euserfilter: PrintAsyncNotifyUserFilter, econversationstyle: PrintAsyncNotifyConversationStyle, pcallback: P4) -> windows_core::Result<IPrintAsyncNotifyChannel>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<IPrintAsyncNotifyCallback>,
{
    windows_core::link!("winspool.drv" "system" fn CreatePrintAsyncNotifyChannel(pszname : windows_core::PCWSTR, pnotificationtype : *const windows_core::GUID, euserfilter : PrintAsyncNotifyUserFilter, econversationstyle : PrintAsyncNotifyConversationStyle, pcallback : * mut core::ffi::c_void, ppiasynchnotification : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreatePrintAsyncNotifyChannel(pszname.param().abi(), pnotificationtype, euserfilter, econversationstyle, pcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreatePrinterIC(hprinter: PRINTER_HANDLE, pdevmode: Option<*const super::Gdi::DEVMODEW>) -> super::super::Foundation::HANDLE {
    windows_core::link!("winspool.drv" "system" fn CreatePrinterIC(hprinter : PRINTER_HANDLE, pdevmode : *const super::Gdi:: DEVMODEW) -> super::super::Foundation:: HANDLE);
    unsafe { CreatePrinterIC(core::mem::transmute(hprinter), pdevmode.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeleteFormA<P1>(hprinter: PRINTER_HANDLE, pformname: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeleteFormA(hprinter : PRINTER_HANDLE, pformname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeleteFormA(core::mem::transmute(hprinter), pformname.param().abi()) }
}
#[inline]
pub unsafe fn DeleteFormW<P1>(hprinter: PRINTER_HANDLE, pformname: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeleteFormW(hprinter : PRINTER_HANDLE, pformname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeleteFormW(core::mem::transmute(hprinter), pformname.param().abi()) }
}
#[inline]
pub unsafe fn DeleteJobNamedProperty<P2>(hprinter: PRINTER_HANDLE, jobid: u32, pszname: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeleteJobNamedProperty(hprinter : PRINTER_HANDLE, jobid : u32, pszname : windows_core::PCWSTR) -> u32);
    unsafe { DeleteJobNamedProperty(core::mem::transmute(hprinter), jobid, pszname.param().abi()) }
}
#[inline]
pub unsafe fn DeleteMonitorA<P0, P1, P2>(pname: P0, penvironment: P1, pmonitorname: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeleteMonitorA(pname : windows_core::PCSTR, penvironment : windows_core::PCSTR, pmonitorname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeleteMonitorA(pname.param().abi(), penvironment.param().abi(), pmonitorname.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeleteMonitorW<P0, P1, P2>(pname: P0, penvironment: P1, pmonitorname: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeleteMonitorW(pname : windows_core::PCWSTR, penvironment : windows_core::PCWSTR, pmonitorname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeleteMonitorW(pname.param().abi(), penvironment.param().abi(), pmonitorname.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeletePortA<P0, P2>(pname: P0, hwnd: super::super::Foundation::HWND, pportname: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePortA(pname : windows_core::PCSTR, hwnd : super::super::Foundation:: HWND, pportname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeletePortA(pname.param().abi(), hwnd, pportname.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeletePortW<P0, P2>(pname: P0, hwnd: super::super::Foundation::HWND, pportname: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePortW(pname : windows_core::PCWSTR, hwnd : super::super::Foundation:: HWND, pportname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeletePortW(pname.param().abi(), hwnd, pportname.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeletePrintProcessorA<P0, P1, P2>(pname: P0, penvironment: P1, pprintprocessorname: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrintProcessorA(pname : windows_core::PCSTR, penvironment : windows_core::PCSTR, pprintprocessorname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeletePrintProcessorA(pname.param().abi(), penvironment.param().abi(), pprintprocessorname.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrintProcessorW<P0, P1, P2>(pname: P0, penvironment: P1, pprintprocessorname: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrintProcessorW(pname : windows_core::PCWSTR, penvironment : windows_core::PCWSTR, pprintprocessorname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeletePrintProcessorW(pname.param().abi(), penvironment.param().abi(), pprintprocessorname.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrintProvidorA<P0, P1, P2>(pname: P0, penvironment: P1, pprintprovidorname: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrintProvidorA(pname : windows_core::PCSTR, penvironment : windows_core::PCSTR, pprintprovidorname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeletePrintProvidorA(pname.param().abi(), penvironment.param().abi(), pprintprovidorname.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrintProvidorW<P0, P1, P2>(pname: P0, penvironment: P1, pprintprovidorname: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrintProvidorW(pname : windows_core::PCWSTR, penvironment : windows_core::PCWSTR, pprintprovidorname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeletePrintProvidorW(pname.param().abi(), penvironment.param().abi(), pprintprovidorname.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrinter(hprinter: PRINTER_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("winspool.drv" "system" fn DeletePrinter(hprinter : PRINTER_HANDLE) -> windows_core::BOOL);
    unsafe { DeletePrinter(core::mem::transmute(hprinter)).ok() }
}
#[inline]
pub unsafe fn DeletePrinterConnectionA<P0>(pname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterConnectionA(pname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeletePrinterConnectionA(pname.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrinterConnectionW<P0>(pname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterConnectionW(pname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeletePrinterConnectionW(pname.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrinterDataA<P1>(hprinter: PRINTER_HANDLE, pvaluename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterDataA(hprinter : PRINTER_HANDLE, pvaluename : windows_core::PCSTR) -> u32);
    unsafe { DeletePrinterDataA(core::mem::transmute(hprinter), pvaluename.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrinterDataExA<P1, P2>(hprinter: PRINTER_HANDLE, pkeyname: P1, pvaluename: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterDataExA(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCSTR, pvaluename : windows_core::PCSTR) -> u32);
    unsafe { DeletePrinterDataExA(core::mem::transmute(hprinter), pkeyname.param().abi(), pvaluename.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrinterDataExW<P1, P2>(hprinter: PRINTER_HANDLE, pkeyname: P1, pvaluename: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterDataExW(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCWSTR, pvaluename : windows_core::PCWSTR) -> u32);
    unsafe { DeletePrinterDataExW(core::mem::transmute(hprinter), pkeyname.param().abi(), pvaluename.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrinterDataW<P1>(hprinter: PRINTER_HANDLE, pvaluename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterDataW(hprinter : PRINTER_HANDLE, pvaluename : windows_core::PCWSTR) -> u32);
    unsafe { DeletePrinterDataW(core::mem::transmute(hprinter), pvaluename.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrinterDriverA<P0, P1, P2>(pname: P0, penvironment: P1, pdrivername: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterDriverA(pname : windows_core::PCSTR, penvironment : windows_core::PCSTR, pdrivername : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeletePrinterDriverA(pname.param().abi(), penvironment.param().abi(), pdrivername.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrinterDriverExA<P0, P1, P2>(pname: P0, penvironment: P1, pdrivername: P2, dwdeleteflag: u32, dwversionflag: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterDriverExA(pname : windows_core::PCSTR, penvironment : windows_core::PCSTR, pdrivername : windows_core::PCSTR, dwdeleteflag : u32, dwversionflag : u32) -> windows_core::BOOL);
    unsafe { DeletePrinterDriverExA(pname.param().abi(), penvironment.param().abi(), pdrivername.param().abi(), dwdeleteflag, dwversionflag) }
}
#[inline]
pub unsafe fn DeletePrinterDriverExW<P0, P1, P2>(pname: P0, penvironment: P1, pdrivername: P2, dwdeleteflag: u32, dwversionflag: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterDriverExW(pname : windows_core::PCWSTR, penvironment : windows_core::PCWSTR, pdrivername : windows_core::PCWSTR, dwdeleteflag : u32, dwversionflag : u32) -> windows_core::BOOL);
    unsafe { DeletePrinterDriverExW(pname.param().abi(), penvironment.param().abi(), pdrivername.param().abi(), dwdeleteflag, dwversionflag) }
}
#[inline]
pub unsafe fn DeletePrinterDriverPackageA<P0, P1, P2>(pszserver: P0, pszinfpath: P1, pszenvironment: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterDriverPackageA(pszserver : windows_core::PCSTR, pszinfpath : windows_core::PCSTR, pszenvironment : windows_core::PCSTR) -> windows_core::HRESULT);
    unsafe { DeletePrinterDriverPackageA(pszserver.param().abi(), pszinfpath.param().abi(), pszenvironment.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeletePrinterDriverPackageW<P0, P1, P2>(pszserver: P0, pszinfpath: P1, pszenvironment: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterDriverPackageW(pszserver : windows_core::PCWSTR, pszinfpath : windows_core::PCWSTR, pszenvironment : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DeletePrinterDriverPackageW(pszserver.param().abi(), pszinfpath.param().abi(), pszenvironment.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeletePrinterDriverW<P0, P1, P2>(pname: P0, penvironment: P1, pdrivername: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterDriverW(pname : windows_core::PCWSTR, penvironment : windows_core::PCWSTR, pdrivername : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeletePrinterDriverW(pname.param().abi(), penvironment.param().abi(), pdrivername.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrinterIC(hprinteric: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn DeletePrinterIC(hprinteric : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { DeletePrinterIC(hprinteric) }
}
#[inline]
pub unsafe fn DeletePrinterKeyA<P1>(hprinter: PRINTER_HANDLE, pkeyname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterKeyA(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCSTR) -> u32);
    unsafe { DeletePrinterKeyA(core::mem::transmute(hprinter), pkeyname.param().abi()) }
}
#[inline]
pub unsafe fn DeletePrinterKeyW<P1>(hprinter: PRINTER_HANDLE, pkeyname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeletePrinterKeyW(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCWSTR) -> u32);
    unsafe { DeletePrinterKeyW(core::mem::transmute(hprinter), pkeyname.param().abi()) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DevQueryPrint(hprinter: PRINTER_HANDLE, pdevmode: *const super::Gdi::DEVMODEA, presid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn DevQueryPrint(hprinter : PRINTER_HANDLE, pdevmode : *const super::Gdi:: DEVMODEA, presid : *mut u32) -> windows_core::BOOL);
    unsafe { DevQueryPrint(core::mem::transmute(hprinter), pdevmode, presid as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DevQueryPrintEx(pdqpinfo: *mut DEVQUERYPRINT_INFO) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn DevQueryPrintEx(pdqpinfo : *mut DEVQUERYPRINT_INFO) -> windows_core::BOOL);
    unsafe { DevQueryPrintEx(pdqpinfo as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DocumentPropertiesA<P2>(hwnd: Option<super::super::Foundation::HWND>, hprinter: PRINTER_HANDLE, pdevicename: P2, pdevmodeoutput: Option<*mut super::Gdi::DEVMODEA>, pdevmodeinput: Option<*const super::Gdi::DEVMODEA>, fmode: u32) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DocumentPropertiesA(hwnd : super::super::Foundation:: HWND, hprinter : PRINTER_HANDLE, pdevicename : windows_core::PCSTR, pdevmodeoutput : *mut super::Gdi:: DEVMODEA, pdevmodeinput : *const super::Gdi:: DEVMODEA, fmode : u32) -> i32);
    unsafe { DocumentPropertiesA(hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(hprinter), pdevicename.param().abi(), pdevmodeoutput.unwrap_or(core::mem::zeroed()) as _, pdevmodeinput.unwrap_or(core::mem::zeroed()) as _, fmode) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DocumentPropertiesW<P2>(hwnd: Option<super::super::Foundation::HWND>, hprinter: PRINTER_HANDLE, pdevicename: P2, pdevmodeoutput: Option<*mut super::Gdi::DEVMODEW>, pdevmodeinput: Option<*const super::Gdi::DEVMODEW>, fmode: u32) -> i32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DocumentPropertiesW(hwnd : super::super::Foundation:: HWND, hprinter : PRINTER_HANDLE, pdevicename : windows_core::PCWSTR, pdevmodeoutput : *mut super::Gdi:: DEVMODEW, pdevmodeinput : *const super::Gdi:: DEVMODEW, fmode : u32) -> i32);
    unsafe { DocumentPropertiesW(hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(hprinter), pdevicename.param().abi(), pdevmodeoutput.unwrap_or(core::mem::zeroed()) as _, pdevmodeinput.unwrap_or(core::mem::zeroed()) as _, fmode) }
}
#[inline]
pub unsafe fn EndDocPrinter(hprinter: PRINTER_HANDLE) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn EndDocPrinter(hprinter : PRINTER_HANDLE) -> windows_core::BOOL);
    unsafe { EndDocPrinter(core::mem::transmute(hprinter)) }
}
#[inline]
pub unsafe fn EndPagePrinter(hprinter: PRINTER_HANDLE) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn EndPagePrinter(hprinter : PRINTER_HANDLE) -> windows_core::BOOL);
    unsafe { EndPagePrinter(core::mem::transmute(hprinter)) }
}
#[inline]
pub unsafe fn EnumFormsA(hprinter: PRINTER_HANDLE, level: u32, pform: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn EnumFormsA(hprinter : PRINTER_HANDLE, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumFormsA(core::mem::transmute(hprinter), level, core::mem::transmute(pform.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pform.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _) }
}
#[inline]
pub unsafe fn EnumFormsW(hprinter: PRINTER_HANDLE, level: u32, pform: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn EnumFormsW(hprinter : PRINTER_HANDLE, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumFormsW(core::mem::transmute(hprinter), level, core::mem::transmute(pform.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pform.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _) }
}
#[inline]
pub unsafe fn EnumJobNamedProperties(hprinter: PRINTER_HANDLE, jobid: u32, pcproperties: *mut u32, ppproperties: *mut *mut PrintNamedProperty) -> u32 {
    windows_core::link!("winspool.drv" "system" fn EnumJobNamedProperties(hprinter : PRINTER_HANDLE, jobid : u32, pcproperties : *mut u32, ppproperties : *mut *mut PrintNamedProperty) -> u32);
    unsafe { EnumJobNamedProperties(core::mem::transmute(hprinter), jobid, pcproperties as _, ppproperties as _) }
}
#[inline]
pub unsafe fn EnumJobsA(hprinter: PRINTER_HANDLE, firstjob: u32, nojobs: u32, level: u32, pjob: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winspool.drv" "system" fn EnumJobsA(hprinter : PRINTER_HANDLE, firstjob : u32, nojobs : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumJobsA(core::mem::transmute(hprinter), firstjob, nojobs, level, core::mem::transmute(pjob.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pjob.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _).ok() }
}
#[inline]
pub unsafe fn EnumJobsW(hprinter: PRINTER_HANDLE, firstjob: u32, nojobs: u32, level: u32, pjob: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winspool.drv" "system" fn EnumJobsW(hprinter : PRINTER_HANDLE, firstjob : u32, nojobs : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumJobsW(core::mem::transmute(hprinter), firstjob, nojobs, level, core::mem::transmute(pjob.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pjob.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _).ok() }
}
#[inline]
pub unsafe fn EnumMonitorsA<P0>(pname: P0, level: u32, pmonitor: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumMonitorsA(pname : windows_core::PCSTR, level : u32, pmonitor : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumMonitorsA(pname.param().abi(), level, core::mem::transmute(pmonitor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pmonitor.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _) }
}
#[inline]
pub unsafe fn EnumMonitorsW<P0>(pname: P0, level: u32, pmonitor: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumMonitorsW(pname : windows_core::PCWSTR, level : u32, pmonitor : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumMonitorsW(pname.param().abi(), level, core::mem::transmute(pmonitor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pmonitor.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _) }
}
#[inline]
pub unsafe fn EnumPortsA<P0>(pname: P0, level: u32, pport: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPortsA(pname : windows_core::PCSTR, level : u32, pport : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumPortsA(pname.param().abi(), level, core::mem::transmute(pport.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pport.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _) }
}
#[inline]
pub unsafe fn EnumPortsW<P0>(pname: P0, level: u32, pport: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPortsW(pname : windows_core::PCWSTR, level : u32, pport : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumPortsW(pname.param().abi(), level, core::mem::transmute(pport.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pport.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _) }
}
#[inline]
pub unsafe fn EnumPrintProcessorDatatypesA<P0, P1>(pname: P0, pprintprocessorname: P1, level: u32, pdatatypes: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrintProcessorDatatypesA(pname : windows_core::PCSTR, pprintprocessorname : windows_core::PCSTR, level : u32, pdatatypes : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumPrintProcessorDatatypesA(pname.param().abi(), pprintprocessorname.param().abi(), level, core::mem::transmute(pdatatypes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdatatypes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _) }
}
#[inline]
pub unsafe fn EnumPrintProcessorDatatypesW<P0, P1>(pname: P0, pprintprocessorname: P1, level: u32, pdatatypes: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrintProcessorDatatypesW(pname : windows_core::PCWSTR, pprintprocessorname : windows_core::PCWSTR, level : u32, pdatatypes : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumPrintProcessorDatatypesW(pname.param().abi(), pprintprocessorname.param().abi(), level, core::mem::transmute(pdatatypes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdatatypes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _) }
}
#[inline]
pub unsafe fn EnumPrintProcessorsA<P0, P1>(pname: P0, penvironment: P1, level: u32, pprintprocessorinfo: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrintProcessorsA(pname : windows_core::PCSTR, penvironment : windows_core::PCSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumPrintProcessorsA(pname.param().abi(), penvironment.param().abi(), level, core::mem::transmute(pprintprocessorinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pprintprocessorinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _) }
}
#[inline]
pub unsafe fn EnumPrintProcessorsW<P0, P1>(pname: P0, penvironment: P1, level: u32, pprintprocessorinfo: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrintProcessorsW(pname : windows_core::PCWSTR, penvironment : windows_core::PCWSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumPrintProcessorsW(pname.param().abi(), penvironment.param().abi(), level, core::mem::transmute(pprintprocessorinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pprintprocessorinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _) }
}
#[inline]
pub unsafe fn EnumPrinterDataA(hprinter: PRINTER_HANDLE, dwindex: u32, pvaluename: &mut [u8], pcbvaluename: *mut u32, ptype: Option<*mut u32>, pdata: Option<&mut [u8]>, pcbdata: Option<*mut u32>) -> u32 {
    windows_core::link!("winspool.drv" "system" fn EnumPrinterDataA(hprinter : PRINTER_HANDLE, dwindex : u32, pvaluename : windows_core::PSTR, cbvaluename : u32, pcbvaluename : *mut u32, ptype : *mut u32, pdata : *mut u8, cbdata : u32, pcbdata : *mut u32) -> u32);
    unsafe { EnumPrinterDataA(core::mem::transmute(hprinter), dwindex, core::mem::transmute(pvaluename.as_ptr()), pvaluename.len().try_into().unwrap(), pcbvaluename as _, ptype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EnumPrinterDataExA<P1>(hprinter: PRINTER_HANDLE, pkeyname: P1, penumvalues: Option<&mut [u8]>, pcbenumvalues: *mut u32, pnenumvalues: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrinterDataExA(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCSTR, penumvalues : *mut u8, cbenumvalues : u32, pcbenumvalues : *mut u32, pnenumvalues : *mut u32) -> u32);
    unsafe { EnumPrinterDataExA(core::mem::transmute(hprinter), pkeyname.param().abi(), core::mem::transmute(penumvalues.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), penumvalues.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbenumvalues as _, pnenumvalues as _) }
}
#[inline]
pub unsafe fn EnumPrinterDataExW<P1>(hprinter: PRINTER_HANDLE, pkeyname: P1, penumvalues: Option<&mut [u8]>, pcbenumvalues: *mut u32, pnenumvalues: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrinterDataExW(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCWSTR, penumvalues : *mut u8, cbenumvalues : u32, pcbenumvalues : *mut u32, pnenumvalues : *mut u32) -> u32);
    unsafe { EnumPrinterDataExW(core::mem::transmute(hprinter), pkeyname.param().abi(), core::mem::transmute(penumvalues.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), penumvalues.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbenumvalues as _, pnenumvalues as _) }
}
#[inline]
pub unsafe fn EnumPrinterDataW(hprinter: PRINTER_HANDLE, dwindex: u32, pvaluename: windows_core::PWSTR, cbvaluename: u32, pcbvaluename: *mut u32, ptype: Option<*mut u32>, pdata: Option<&mut [u8]>, pcbdata: Option<*mut u32>) -> u32 {
    windows_core::link!("winspool.drv" "system" fn EnumPrinterDataW(hprinter : PRINTER_HANDLE, dwindex : u32, pvaluename : windows_core::PWSTR, cbvaluename : u32, pcbvaluename : *mut u32, ptype : *mut u32, pdata : *mut u8, cbdata : u32, pcbdata : *mut u32) -> u32);
    unsafe { EnumPrinterDataW(core::mem::transmute(hprinter), dwindex, core::mem::transmute(pvaluename), cbvaluename, pcbvaluename as _, ptype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EnumPrinterDriversA<P0, P1>(pname: P0, penvironment: P1, level: u32, pdriverinfo: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrinterDriversA(pname : windows_core::PCSTR, penvironment : windows_core::PCSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumPrinterDriversA(pname.param().abi(), penvironment.param().abi(), level, core::mem::transmute(pdriverinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _).ok() }
}
#[inline]
pub unsafe fn EnumPrinterDriversW<P0, P1>(pname: P0, penvironment: P1, level: u32, pdriverinfo: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrinterDriversW(pname : windows_core::PCWSTR, penvironment : windows_core::PCWSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumPrinterDriversW(pname.param().abi(), penvironment.param().abi(), level, core::mem::transmute(pdriverinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _).ok() }
}
#[inline]
pub unsafe fn EnumPrinterKeyA<P1>(hprinter: PRINTER_HANDLE, pkeyname: P1, psubkey: Option<&mut [u8]>, pcbsubkey: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrinterKeyA(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCSTR, psubkey : windows_core::PSTR, cbsubkey : u32, pcbsubkey : *mut u32) -> u32);
    unsafe { EnumPrinterKeyA(core::mem::transmute(hprinter), pkeyname.param().abi(), core::mem::transmute(psubkey.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psubkey.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbsubkey as _) }
}
#[inline]
pub unsafe fn EnumPrinterKeyW<P1>(hprinter: PRINTER_HANDLE, pkeyname: P1, psubkey: Option<windows_core::PWSTR>, cbsubkey: u32, pcbsubkey: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrinterKeyW(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCWSTR, psubkey : windows_core::PWSTR, cbsubkey : u32, pcbsubkey : *mut u32) -> u32);
    unsafe { EnumPrinterKeyW(core::mem::transmute(hprinter), pkeyname.param().abi(), psubkey.unwrap_or(core::mem::zeroed()) as _, cbsubkey, pcbsubkey as _) }
}
#[inline]
pub unsafe fn EnumPrintersA<P1>(flags: u32, name: P1, level: u32, pprinterenum: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrintersA(flags : u32, name : windows_core::PCSTR, level : u32, pprinterenum : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumPrintersA(flags, name.param().abi(), level, core::mem::transmute(pprinterenum.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pprinterenum.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _).ok() }
}
#[inline]
pub unsafe fn EnumPrintersW<P1>(flags: u32, name: P1, level: u32, pprinterenum: Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn EnumPrintersW(flags : u32, name : windows_core::PCWSTR, level : u32, pprinterenum : *mut u8, cbbuf : u32, pcbneeded : *mut u32, pcreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumPrintersW(flags, name.param().abi(), level, core::mem::transmute(pprinterenum.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pprinterenum.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _, pcreturned as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ExtDeviceMode<P3, P4, P6>(hwnd: Option<super::super::Foundation::HWND>, hinst: Option<super::super::Foundation::HANDLE>, pdevmodeoutput: Option<*mut super::Gdi::DEVMODEA>, pdevicename: P3, pport: P4, pdevmodeinput: Option<*const super::Gdi::DEVMODEA>, pprofile: P6, fmode: u32) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn ExtDeviceMode(hwnd : super::super::Foundation:: HWND, hinst : super::super::Foundation:: HANDLE, pdevmodeoutput : *mut super::Gdi:: DEVMODEA, pdevicename : windows_core::PCSTR, pport : windows_core::PCSTR, pdevmodeinput : *const super::Gdi:: DEVMODEA, pprofile : windows_core::PCSTR, fmode : u32) -> i32);
    unsafe { ExtDeviceMode(hwnd.unwrap_or(core::mem::zeroed()) as _, hinst.unwrap_or(core::mem::zeroed()) as _, pdevmodeoutput.unwrap_or(core::mem::zeroed()) as _, pdevicename.param().abi(), pport.param().abi(), pdevmodeinput.unwrap_or(core::mem::zeroed()) as _, pprofile.param().abi(), fmode) }
}
#[inline]
pub unsafe fn FindClosePrinterChangeNotification(hchange: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn FindClosePrinterChangeNotification(hchange : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { FindClosePrinterChangeNotification(hchange) }
}
#[inline]
pub unsafe fn FindFirstPrinterChangeNotification(hprinter: PRINTER_HANDLE, fdwfilter: u32, fdwoptions: u32, pprinternotifyoptions: Option<*const core::ffi::c_void>) -> super::super::Foundation::HANDLE {
    windows_core::link!("winspool.drv" "system" fn FindFirstPrinterChangeNotification(hprinter : PRINTER_HANDLE, fdwfilter : u32, fdwoptions : u32, pprinternotifyoptions : *const core::ffi::c_void) -> super::super::Foundation:: HANDLE);
    unsafe { FindFirstPrinterChangeNotification(core::mem::transmute(hprinter), fdwfilter, fdwoptions, pprinternotifyoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FindNextPrinterChangeNotification(hchange: super::super::Foundation::HANDLE, pdwchange: Option<*mut u32>, pvreserved: Option<*const core::ffi::c_void>, ppprinternotifyinfo: Option<*mut *mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn FindNextPrinterChangeNotification(hchange : super::super::Foundation:: HANDLE, pdwchange : *mut u32, pvreserved : *const core::ffi::c_void, ppprinternotifyinfo : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FindNextPrinterChangeNotification(hchange, pdwchange.unwrap_or(core::mem::zeroed()) as _, pvreserved.unwrap_or(core::mem::zeroed()) as _, ppprinternotifyinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FlushPrinter(hprinter: PRINTER_HANDLE, pbuf: Option<*const core::ffi::c_void>, cbbuf: u32, pcwritten: *mut u32, csleep: u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn FlushPrinter(hprinter : PRINTER_HANDLE, pbuf : *const core::ffi::c_void, cbbuf : u32, pcwritten : *mut u32, csleep : u32) -> windows_core::BOOL);
    unsafe { FlushPrinter(core::mem::transmute(hprinter), pbuf.unwrap_or(core::mem::zeroed()) as _, cbbuf, pcwritten as _, csleep) }
}
#[inline]
pub unsafe fn FreePrintNamedPropertyArray(ppproperties: Option<&mut [*mut PrintNamedProperty]>) {
    windows_core::link!("winspool.drv" "system" fn FreePrintNamedPropertyArray(cproperties : u32, ppproperties : *mut *mut PrintNamedProperty));
    unsafe { FreePrintNamedPropertyArray(ppproperties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppproperties.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn FreePrintPropertyValue(pvalue: *mut PrintPropertyValue) {
    windows_core::link!("winspool.drv" "system" fn FreePrintPropertyValue(pvalue : *mut PrintPropertyValue));
    unsafe { FreePrintPropertyValue(pvalue as _) }
}
#[inline]
pub unsafe fn FreePrinterNotifyInfo(pprinternotifyinfo: *const PRINTER_NOTIFY_INFO) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn FreePrinterNotifyInfo(pprinternotifyinfo : *const PRINTER_NOTIFY_INFO) -> windows_core::BOOL);
    unsafe { FreePrinterNotifyInfo(pprinternotifyinfo) }
}
#[inline]
pub unsafe fn GdiDeleteSpoolFileHandle(spoolfilehandle: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("gdi32.dll" "system" fn GdiDeleteSpoolFileHandle(spoolfilehandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { GdiDeleteSpoolFileHandle(spoolfilehandle) }
}
#[inline]
pub unsafe fn GdiEndDocEMF(spoolfilehandle: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("gdi32.dll" "system" fn GdiEndDocEMF(spoolfilehandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { GdiEndDocEMF(spoolfilehandle) }
}
#[inline]
pub unsafe fn GdiEndPageEMF(spoolfilehandle: super::super::Foundation::HANDLE, dwoptimization: u32) -> windows_core::BOOL {
    windows_core::link!("gdi32.dll" "system" fn GdiEndPageEMF(spoolfilehandle : super::super::Foundation:: HANDLE, dwoptimization : u32) -> windows_core::BOOL);
    unsafe { GdiEndPageEMF(spoolfilehandle, dwoptimization) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdiGetDC(spoolfilehandle: super::super::Foundation::HANDLE) -> super::Gdi::HDC {
    windows_core::link!("gdi32.dll" "system" fn GdiGetDC(spoolfilehandle : super::super::Foundation:: HANDLE) -> super::Gdi:: HDC);
    unsafe { GdiGetDC(spoolfilehandle) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdiGetDevmodeForPage(spoolfilehandle: super::super::Foundation::HANDLE, dwpagenumber: u32, pcurrdm: *mut *mut super::Gdi::DEVMODEW, plastdm: *mut *mut super::Gdi::DEVMODEW) -> windows_core::BOOL {
    windows_core::link!("gdi32.dll" "system" fn GdiGetDevmodeForPage(spoolfilehandle : super::super::Foundation:: HANDLE, dwpagenumber : u32, pcurrdm : *mut *mut super::Gdi:: DEVMODEW, plastdm : *mut *mut super::Gdi:: DEVMODEW) -> windows_core::BOOL);
    unsafe { GdiGetDevmodeForPage(spoolfilehandle, dwpagenumber, pcurrdm as _, plastdm as _) }
}
#[inline]
pub unsafe fn GdiGetPageCount(spoolfilehandle: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("gdi32.dll" "system" fn GdiGetPageCount(spoolfilehandle : super::super::Foundation:: HANDLE) -> u32);
    unsafe { GdiGetPageCount(spoolfilehandle) }
}
#[inline]
pub unsafe fn GdiGetPageHandle(spoolfilehandle: super::super::Foundation::HANDLE, page: u32, pdwpagetype: *mut u32) -> super::super::Foundation::HANDLE {
    windows_core::link!("gdi32.dll" "system" fn GdiGetPageHandle(spoolfilehandle : super::super::Foundation:: HANDLE, page : u32, pdwpagetype : *mut u32) -> super::super::Foundation:: HANDLE);
    unsafe { GdiGetPageHandle(spoolfilehandle, page, pdwpagetype as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdiGetSpoolFileHandle<P0, P2>(pwszprintername: P0, pdevmode: *mut super::Gdi::DEVMODEW, pwszdocname: P2) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdi32.dll" "system" fn GdiGetSpoolFileHandle(pwszprintername : windows_core::PCWSTR, pdevmode : *mut super::Gdi:: DEVMODEW, pwszdocname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    unsafe { GdiGetSpoolFileHandle(pwszprintername.param().abi(), pdevmode as _, pwszdocname.param().abi()) }
}
#[inline]
pub unsafe fn GdiPlayPageEMF(spoolfilehandle: super::super::Foundation::HANDLE, hemf: super::super::Foundation::HANDLE, prectdocument: *mut super::super::Foundation::RECT, prectborder: *mut super::super::Foundation::RECT, prectclip: *mut super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_core::link!("gdi32.dll" "system" fn GdiPlayPageEMF(spoolfilehandle : super::super::Foundation:: HANDLE, hemf : super::super::Foundation:: HANDLE, prectdocument : *mut super::super::Foundation:: RECT, prectborder : *mut super::super::Foundation:: RECT, prectclip : *mut super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { GdiPlayPageEMF(spoolfilehandle, hemf, prectdocument as _, prectborder as _, prectclip as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdiResetDCEMF(spoolfilehandle: super::super::Foundation::HANDLE, pcurrdm: *mut super::Gdi::DEVMODEW) -> windows_core::BOOL {
    windows_core::link!("gdi32.dll" "system" fn GdiResetDCEMF(spoolfilehandle : super::super::Foundation:: HANDLE, pcurrdm : *mut super::Gdi:: DEVMODEW) -> windows_core::BOOL);
    unsafe { GdiResetDCEMF(spoolfilehandle, pcurrdm as _) }
}
#[cfg(feature = "Win32_Storage_Xps")]
#[inline]
pub unsafe fn GdiStartDocEMF(spoolfilehandle: super::super::Foundation::HANDLE, pdocinfo: *mut super::super::Storage::Xps::DOCINFOW) -> windows_core::BOOL {
    windows_core::link!("gdi32.dll" "system" fn GdiStartDocEMF(spoolfilehandle : super::super::Foundation:: HANDLE, pdocinfo : *mut super::super::Storage::Xps:: DOCINFOW) -> windows_core::BOOL);
    unsafe { GdiStartDocEMF(spoolfilehandle, pdocinfo as _) }
}
#[inline]
pub unsafe fn GdiStartPageEMF(spoolfilehandle: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("gdi32.dll" "system" fn GdiStartPageEMF(spoolfilehandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { GdiStartPageEMF(spoolfilehandle) }
}
#[inline]
pub unsafe fn GenerateCopyFilePaths<P0, P1>(pszprintername: P0, pszdirectory: P1, psplclientinfo: *const u8, dwlevel: u32, pszsourcedir: windows_core::PWSTR, pcchsourcedirsize: *mut u32, psztargetdir: windows_core::PWSTR, pcchtargetdirsize: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn GenerateCopyFilePaths(pszprintername : windows_core::PCWSTR, pszdirectory : windows_core::PCWSTR, psplclientinfo : *const u8, dwlevel : u32, pszsourcedir : windows_core::PWSTR, pcchsourcedirsize : *mut u32, psztargetdir : windows_core::PWSTR, pcchtargetdirsize : *mut u32, dwflags : u32) -> u32);
    unsafe { GenerateCopyFilePaths(pszprintername.param().abi(), pszdirectory.param().abi(), psplclientinfo, dwlevel, core::mem::transmute(pszsourcedir), pcchsourcedirsize as _, core::mem::transmute(psztargetdir), pcchtargetdirsize as _, dwflags) }
}
#[inline]
pub unsafe fn GetCPSUIUserData(hdlg: super::super::Foundation::HWND) -> usize {
    windows_core::link!("compstui.dll" "system" fn GetCPSUIUserData(hdlg : super::super::Foundation:: HWND) -> usize);
    unsafe { GetCPSUIUserData(hdlg) }
}
#[inline]
pub unsafe fn GetCorePrinterDriversA<P0, P1, P2>(pszserver: P0, pszenvironment: P1, pszzcoredriverdependencies: P2, pcoreprinterdrivers: &mut [CORE_PRINTER_DRIVERA]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetCorePrinterDriversA(pszserver : windows_core::PCSTR, pszenvironment : windows_core::PCSTR, pszzcoredriverdependencies : windows_core::PCSTR, ccoreprinterdrivers : u32, pcoreprinterdrivers : *mut CORE_PRINTER_DRIVERA) -> windows_core::HRESULT);
    unsafe { GetCorePrinterDriversA(pszserver.param().abi(), pszenvironment.param().abi(), pszzcoredriverdependencies.param().abi(), pcoreprinterdrivers.len().try_into().unwrap(), core::mem::transmute(pcoreprinterdrivers.as_ptr())).ok() }
}
#[inline]
pub unsafe fn GetCorePrinterDriversW<P0, P1, P2>(pszserver: P0, pszenvironment: P1, pszzcoredriverdependencies: P2, pcoreprinterdrivers: &mut [CORE_PRINTER_DRIVERW]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetCorePrinterDriversW(pszserver : windows_core::PCWSTR, pszenvironment : windows_core::PCWSTR, pszzcoredriverdependencies : windows_core::PCWSTR, ccoreprinterdrivers : u32, pcoreprinterdrivers : *mut CORE_PRINTER_DRIVERW) -> windows_core::HRESULT);
    unsafe { GetCorePrinterDriversW(pszserver.param().abi(), pszenvironment.param().abi(), pszzcoredriverdependencies.param().abi(), pcoreprinterdrivers.len().try_into().unwrap(), core::mem::transmute(pcoreprinterdrivers.as_ptr())).ok() }
}
#[inline]
pub unsafe fn GetDefaultPrinterA(pszbuffer: Option<windows_core::PSTR>, pcchbuffer: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn GetDefaultPrinterA(pszbuffer : windows_core::PSTR, pcchbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetDefaultPrinterA(pszbuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuffer as _) }
}
#[inline]
pub unsafe fn GetDefaultPrinterW(pszbuffer: Option<windows_core::PWSTR>, pcchbuffer: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn GetDefaultPrinterW(pszbuffer : windows_core::PWSTR, pcchbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetDefaultPrinterW(pszbuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuffer as _) }
}
#[inline]
pub unsafe fn GetFormA<P1>(hprinter: PRINTER_HANDLE, pformname: P1, level: u32, pform: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetFormA(hprinter : PRINTER_HANDLE, pformname : windows_core::PCSTR, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetFormA(core::mem::transmute(hprinter), pformname.param().abi(), level, core::mem::transmute(pform.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pform.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetFormW<P1>(hprinter: PRINTER_HANDLE, pformname: P1, level: u32, pform: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetFormW(hprinter : PRINTER_HANDLE, pformname : windows_core::PCWSTR, level : u32, pform : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetFormW(core::mem::transmute(hprinter), pformname.param().abi(), level, core::mem::transmute(pform.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pform.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetJobA(hprinter: PRINTER_HANDLE, jobid: u32, level: u32, pjob: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn GetJobA(hprinter : PRINTER_HANDLE, jobid : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetJobA(core::mem::transmute(hprinter), jobid, level, core::mem::transmute(pjob.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pjob.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetJobAttributes<P0>(pprintername: P0, pdevmode: *const super::Gdi::DEVMODEW, pattributeinfo: *mut ATTRIBUTE_INFO_3) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("spoolss.dll" "system" fn GetJobAttributes(pprintername : windows_core::PCWSTR, pdevmode : *const super::Gdi:: DEVMODEW, pattributeinfo : *mut ATTRIBUTE_INFO_3) -> windows_core::BOOL);
    unsafe { GetJobAttributes(pprintername.param().abi(), pdevmode, pattributeinfo as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetJobAttributesEx<P0>(pprintername: P0, pdevmode: *const super::Gdi::DEVMODEW, dwlevel: u32, pattributeinfo: &mut [u8], dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("spoolss.dll" "system" fn GetJobAttributesEx(pprintername : windows_core::PCWSTR, pdevmode : *const super::Gdi:: DEVMODEW, dwlevel : u32, pattributeinfo : *mut u8, nsize : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { GetJobAttributesEx(pprintername.param().abi(), pdevmode, dwlevel, core::mem::transmute(pattributeinfo.as_ptr()), pattributeinfo.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn GetJobNamedPropertyValue<P2>(hprinter: PRINTER_HANDLE, jobid: u32, pszname: P2, pvalue: *mut PrintPropertyValue) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetJobNamedPropertyValue(hprinter : PRINTER_HANDLE, jobid : u32, pszname : windows_core::PCWSTR, pvalue : *mut PrintPropertyValue) -> u32);
    unsafe { GetJobNamedPropertyValue(core::mem::transmute(hprinter), jobid, pszname.param().abi(), pvalue as _) }
}
#[inline]
pub unsafe fn GetJobW(hprinter: PRINTER_HANDLE, jobid: u32, level: u32, pjob: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn GetJobW(hprinter : PRINTER_HANDLE, jobid : u32, level : u32, pjob : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetJobW(core::mem::transmute(hprinter), jobid, level, core::mem::transmute(pjob.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pjob.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrintExecutionData(pdata: *mut PRINT_EXECUTION_DATA) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn GetPrintExecutionData(pdata : *mut PRINT_EXECUTION_DATA) -> windows_core::BOOL);
    unsafe { GetPrintExecutionData(pdata as _) }
}
#[inline]
pub unsafe fn GetPrintOutputInfo<P1>(hwnd: super::super::Foundation::HWND, pszprinter: P1, phfile: *mut super::super::Foundation::HANDLE, ppszoutputfile: *mut windows_core::PWSTR) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrintOutputInfo(hwnd : super::super::Foundation:: HWND, pszprinter : windows_core::PCWSTR, phfile : *mut super::super::Foundation:: HANDLE, ppszoutputfile : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { GetPrintOutputInfo(hwnd, pszprinter.param().abi(), phfile as _, ppszoutputfile as _).ok() }
}
#[inline]
pub unsafe fn GetPrintProcessorDirectoryA<P0, P1>(pname: P0, penvironment: P1, level: u32, pprintprocessorinfo: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrintProcessorDirectoryA(pname : windows_core::PCSTR, penvironment : windows_core::PCSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrintProcessorDirectoryA(pname.param().abi(), penvironment.param().abi(), level, core::mem::transmute(pprintprocessorinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pprintprocessorinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrintProcessorDirectoryW<P0, P1>(pname: P0, penvironment: P1, level: u32, pprintprocessorinfo: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrintProcessorDirectoryW(pname : windows_core::PCWSTR, penvironment : windows_core::PCWSTR, level : u32, pprintprocessorinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrintProcessorDirectoryW(pname.param().abi(), penvironment.param().abi(), level, core::mem::transmute(pprintprocessorinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pprintprocessorinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterA(hprinter: PRINTER_HANDLE, level: u32, pprinter: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winspool.drv" "system" fn GetPrinterA(hprinter : PRINTER_HANDLE, level : u32, pprinter : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrinterA(core::mem::transmute(hprinter), level, core::mem::transmute(pprinter.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pprinter.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _).ok() }
}
#[inline]
pub unsafe fn GetPrinterDataA<P1>(hprinter: PRINTER_HANDLE, pvaluename: P1, ptype: Option<*mut u32>, pdata: Option<&mut [u8]>, pcbneeded: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDataA(hprinter : PRINTER_HANDLE, pvaluename : windows_core::PCSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
    unsafe { GetPrinterDataA(core::mem::transmute(hprinter), pvaluename.param().abi(), ptype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterDataExA<P1, P2>(hprinter: PRINTER_HANDLE, pkeyname: P1, pvaluename: P2, ptype: Option<*mut u32>, pdata: Option<&mut [u8]>, pcbneeded: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDataExA(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCSTR, pvaluename : windows_core::PCSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
    unsafe { GetPrinterDataExA(core::mem::transmute(hprinter), pkeyname.param().abi(), pvaluename.param().abi(), ptype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterDataExW<P1, P2>(hprinter: PRINTER_HANDLE, pkeyname: P1, pvaluename: P2, ptype: Option<*mut u32>, pdata: Option<&mut [u8]>, pcbneeded: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDataExW(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCWSTR, pvaluename : windows_core::PCWSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
    unsafe { GetPrinterDataExW(core::mem::transmute(hprinter), pkeyname.param().abi(), pvaluename.param().abi(), ptype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterDataW<P1>(hprinter: PRINTER_HANDLE, pvaluename: P1, ptype: Option<*mut u32>, pdata: Option<&mut [u8]>, pcbneeded: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDataW(hprinter : PRINTER_HANDLE, pvaluename : windows_core::PCWSTR, ptype : *mut u32, pdata : *mut u8, nsize : u32, pcbneeded : *mut u32) -> u32);
    unsafe { GetPrinterDataW(core::mem::transmute(hprinter), pvaluename.param().abi(), ptype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterDriver2A<P2>(hwnd: Option<super::super::Foundation::HWND>, hprinter: PRINTER_HANDLE, penvironment: P2, level: u32, pdriverinfo: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDriver2A(hwnd : super::super::Foundation:: HWND, hprinter : PRINTER_HANDLE, penvironment : windows_core::PCSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrinterDriver2A(hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(hprinter), penvironment.param().abi(), level, core::mem::transmute(pdriverinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterDriver2W<P2>(hwnd: Option<super::super::Foundation::HWND>, hprinter: PRINTER_HANDLE, penvironment: P2, level: u32, pdriverinfo: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDriver2W(hwnd : super::super::Foundation:: HWND, hprinter : PRINTER_HANDLE, penvironment : windows_core::PCWSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrinterDriver2W(hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(hprinter), penvironment.param().abi(), level, core::mem::transmute(pdriverinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterDriverA<P1>(hprinter: PRINTER_HANDLE, penvironment: P1, level: u32, pdriverinfo: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDriverA(hprinter : PRINTER_HANDLE, penvironment : windows_core::PCSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrinterDriverA(core::mem::transmute(hprinter), penvironment.param().abi(), level, core::mem::transmute(pdriverinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterDriverDirectoryA<P0, P1>(pname: P0, penvironment: P1, level: u32, pdriverdirectory: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDriverDirectoryA(pname : windows_core::PCSTR, penvironment : windows_core::PCSTR, level : u32, pdriverdirectory : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrinterDriverDirectoryA(pname.param().abi(), penvironment.param().abi(), level, core::mem::transmute(pdriverdirectory.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdriverdirectory.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterDriverDirectoryW<P0, P1>(pname: P0, penvironment: P1, level: u32, pdriverdirectory: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDriverDirectoryW(pname : windows_core::PCWSTR, penvironment : windows_core::PCWSTR, level : u32, pdriverdirectory : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrinterDriverDirectoryW(pname.param().abi(), penvironment.param().abi(), level, core::mem::transmute(pdriverdirectory.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdriverdirectory.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterDriverPackagePathA<P0, P1, P2, P3>(pszserver: P0, pszenvironment: P1, pszlanguage: P2, pszpackageid: P3, pszdriverpackagecab: Option<&mut [u8]>, pcchrequiredsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDriverPackagePathA(pszserver : windows_core::PCSTR, pszenvironment : windows_core::PCSTR, pszlanguage : windows_core::PCSTR, pszpackageid : windows_core::PCSTR, pszdriverpackagecab : windows_core::PSTR, cchdriverpackagecab : u32, pcchrequiredsize : *mut u32) -> windows_core::HRESULT);
    unsafe { GetPrinterDriverPackagePathA(pszserver.param().abi(), pszenvironment.param().abi(), pszlanguage.param().abi(), pszpackageid.param().abi(), core::mem::transmute(pszdriverpackagecab.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszdriverpackagecab.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcchrequiredsize as _).ok() }
}
#[inline]
pub unsafe fn GetPrinterDriverPackagePathW<P0, P1, P2, P3>(pszserver: P0, pszenvironment: P1, pszlanguage: P2, pszpackageid: P3, pszdriverpackagecab: Option<&mut [u16]>, pcchrequiredsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDriverPackagePathW(pszserver : windows_core::PCWSTR, pszenvironment : windows_core::PCWSTR, pszlanguage : windows_core::PCWSTR, pszpackageid : windows_core::PCWSTR, pszdriverpackagecab : windows_core::PWSTR, cchdriverpackagecab : u32, pcchrequiredsize : *mut u32) -> windows_core::HRESULT);
    unsafe { GetPrinterDriverPackagePathW(pszserver.param().abi(), pszenvironment.param().abi(), pszlanguage.param().abi(), pszpackageid.param().abi(), core::mem::transmute(pszdriverpackagecab.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszdriverpackagecab.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcchrequiredsize as _).ok() }
}
#[inline]
pub unsafe fn GetPrinterDriverW<P1>(hprinter: PRINTER_HANDLE, penvironment: P1, level: u32, pdriverinfo: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn GetPrinterDriverW(hprinter : PRINTER_HANDLE, penvironment : windows_core::PCWSTR, level : u32, pdriverinfo : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrinterDriverW(core::mem::transmute(hprinter), penvironment.param().abi(), level, core::mem::transmute(pdriverinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _) }
}
#[inline]
pub unsafe fn GetPrinterW(hprinter: PRINTER_HANDLE, level: u32, pprinter: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("winspool.drv" "system" fn GetPrinterW(hprinter : PRINTER_HANDLE, level : u32, pprinter : *mut u8, cbbuf : u32, pcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrinterW(core::mem::transmute(hprinter), level, core::mem::transmute(pprinter.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pprinter.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _).ok() }
}
#[inline]
pub unsafe fn GetSpoolFileHandle(hprinter: PRINTER_HANDLE) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("winspool.drv" "system" fn GetSpoolFileHandle(hprinter : PRINTER_HANDLE) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { GetSpoolFileHandle(core::mem::transmute(hprinter)) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn ImpersonatePrinterClient(htoken: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn ImpersonatePrinterClient(htoken : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { ImpersonatePrinterClient(htoken) }
}
#[inline]
pub unsafe fn InstallPrinterDriverFromPackageA<P0, P1, P2, P3>(pszserver: P0, pszinfpath: P1, pszdrivername: P2, pszenvironment: P3, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn InstallPrinterDriverFromPackageA(pszserver : windows_core::PCSTR, pszinfpath : windows_core::PCSTR, pszdrivername : windows_core::PCSTR, pszenvironment : windows_core::PCSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { InstallPrinterDriverFromPackageA(pszserver.param().abi(), pszinfpath.param().abi(), pszdrivername.param().abi(), pszenvironment.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn InstallPrinterDriverFromPackageW<P0, P1, P2, P3>(pszserver: P0, pszinfpath: P1, pszdrivername: P2, pszenvironment: P3, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn InstallPrinterDriverFromPackageW(pszserver : windows_core::PCWSTR, pszinfpath : windows_core::PCWSTR, pszdrivername : windows_core::PCWSTR, pszenvironment : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { InstallPrinterDriverFromPackageW(pszserver.param().abi(), pszinfpath.param().abi(), pszdrivername.param().abi(), pszenvironment.param().abi(), dwflags).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn IsValidDevmodeA(pdevmode: Option<*const super::Gdi::DEVMODEA>, devmodesize: usize) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn IsValidDevmodeA(pdevmode : *const super::Gdi:: DEVMODEA, devmodesize : usize) -> windows_core::BOOL);
    unsafe { IsValidDevmodeA(pdevmode.unwrap_or(core::mem::zeroed()) as _, devmodesize) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn IsValidDevmodeW(pdevmode: Option<*const super::Gdi::DEVMODEW>, devmodesize: usize) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn IsValidDevmodeW(pdevmode : *const super::Gdi:: DEVMODEW, devmodesize : usize) -> windows_core::BOOL);
    unsafe { IsValidDevmodeW(pdevmode.unwrap_or(core::mem::zeroed()) as _, devmodesize) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OpenPrinter2A<P0>(pprintername: P0, phprinter: *mut PRINTER_HANDLE, pdefault: Option<*const PRINTER_DEFAULTSA>, poptions: Option<*const PRINTER_OPTIONSA>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn OpenPrinter2A(pprintername : windows_core::PCSTR, phprinter : *mut PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSA, poptions : *const PRINTER_OPTIONSA) -> windows_core::BOOL);
    unsafe { OpenPrinter2A(pprintername.param().abi(), phprinter as _, pdefault.unwrap_or(core::mem::zeroed()) as _, poptions.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OpenPrinter2W<P0>(pprintername: P0, phprinter: *mut PRINTER_HANDLE, pdefault: Option<*const PRINTER_DEFAULTSW>, poptions: Option<*const PRINTER_OPTIONSW>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn OpenPrinter2W(pprintername : windows_core::PCWSTR, phprinter : *mut PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSW, poptions : *const PRINTER_OPTIONSW) -> windows_core::BOOL);
    unsafe { OpenPrinter2W(pprintername.param().abi(), phprinter as _, pdefault.unwrap_or(core::mem::zeroed()) as _, poptions.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OpenPrinterA<P0>(pprintername: P0, phprinter: *mut PRINTER_HANDLE, pdefault: Option<*const PRINTER_DEFAULTSA>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn OpenPrinterA(pprintername : windows_core::PCSTR, phprinter : *mut PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSA) -> windows_core::BOOL);
    unsafe { OpenPrinterA(pprintername.param().abi(), phprinter as _, pdefault.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OpenPrinterW<P0>(pprintername: P0, phprinter: *mut PRINTER_HANDLE, pdefault: Option<*const PRINTER_DEFAULTSW>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn OpenPrinterW(pprintername : windows_core::PCWSTR, phprinter : *mut PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSW) -> windows_core::BOOL);
    unsafe { OpenPrinterW(pprintername.param().abi(), phprinter as _, pdefault.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn PartialReplyPrinterChangeNotification(hprinter: PRINTER_HANDLE, pdatasrc: Option<*const PRINTER_NOTIFY_INFO_DATA>) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn PartialReplyPrinterChangeNotification(hprinter : PRINTER_HANDLE, pdatasrc : *const PRINTER_NOTIFY_INFO_DATA) -> windows_core::BOOL);
    unsafe { PartialReplyPrinterChangeNotification(core::mem::transmute(hprinter), pdatasrc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PlayGdiScriptOnPrinterIC(hprinteric: super::super::Foundation::HANDLE, pin: &[u8], pout: &mut [u8], ul: u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn PlayGdiScriptOnPrinterIC(hprinteric : super::super::Foundation:: HANDLE, pin : *const u8, cin : u32, pout : *mut u8, cout : u32, ul : u32) -> windows_core::BOOL);
    unsafe { PlayGdiScriptOnPrinterIC(hprinteric, core::mem::transmute(pin.as_ptr()), pin.len().try_into().unwrap(), core::mem::transmute(pout.as_ptr()), pout.len().try_into().unwrap(), ul) }
}
#[inline]
pub unsafe fn PrinterMessageBoxA<P3, P4>(hprinter: PRINTER_HANDLE, error: u32, hwnd: super::super::Foundation::HWND, ptext: P3, pcaption: P4, dwtype: u32) -> u32
where
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn PrinterMessageBoxA(hprinter : PRINTER_HANDLE, error : u32, hwnd : super::super::Foundation:: HWND, ptext : windows_core::PCSTR, pcaption : windows_core::PCSTR, dwtype : u32) -> u32);
    unsafe { PrinterMessageBoxA(core::mem::transmute(hprinter), error, hwnd, ptext.param().abi(), pcaption.param().abi(), dwtype) }
}
#[inline]
pub unsafe fn PrinterMessageBoxW<P3, P4>(hprinter: PRINTER_HANDLE, error: u32, hwnd: super::super::Foundation::HWND, ptext: P3, pcaption: P4, dwtype: u32) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn PrinterMessageBoxW(hprinter : PRINTER_HANDLE, error : u32, hwnd : super::super::Foundation:: HWND, ptext : windows_core::PCWSTR, pcaption : windows_core::PCWSTR, dwtype : u32) -> u32);
    unsafe { PrinterMessageBoxW(core::mem::transmute(hprinter), error, hwnd, ptext.param().abi(), pcaption.param().abi(), dwtype) }
}
#[inline]
pub unsafe fn PrinterProperties(hwnd: super::super::Foundation::HWND, hprinter: PRINTER_HANDLE) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn PrinterProperties(hwnd : super::super::Foundation:: HWND, hprinter : PRINTER_HANDLE) -> windows_core::BOOL);
    unsafe { PrinterProperties(hwnd, core::mem::transmute(hprinter)) }
}
#[inline]
pub unsafe fn ProvidorFindClosePrinterChangeNotification(hprinter: PRINTER_HANDLE) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn ProvidorFindClosePrinterChangeNotification(hprinter : PRINTER_HANDLE) -> windows_core::BOOL);
    unsafe { ProvidorFindClosePrinterChangeNotification(core::mem::transmute(hprinter)) }
}
#[inline]
pub unsafe fn ProvidorFindFirstPrinterChangeNotification(hprinter: PRINTER_HANDLE, fdwflags: u32, fdwoptions: u32, hnotify: super::super::Foundation::HANDLE, pprinternotifyoptions: Option<*const core::ffi::c_void>, pvreserved1: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn ProvidorFindFirstPrinterChangeNotification(hprinter : PRINTER_HANDLE, fdwflags : u32, fdwoptions : u32, hnotify : super::super::Foundation:: HANDLE, pprinternotifyoptions : *const core::ffi::c_void, pvreserved1 : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ProvidorFindFirstPrinterChangeNotification(core::mem::transmute(hprinter), fdwflags, fdwoptions, hnotify, pprinternotifyoptions.unwrap_or(core::mem::zeroed()) as _, pvreserved1.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ReadPrinter(hprinter: PRINTER_HANDLE, pbuf: *mut core::ffi::c_void, cbbuf: u32, pnobytesread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn ReadPrinter(hprinter : PRINTER_HANDLE, pbuf : *mut core::ffi::c_void, cbbuf : u32, pnobytesread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadPrinter(core::mem::transmute(hprinter), pbuf as _, cbbuf, pnobytesread as _) }
}
#[inline]
pub unsafe fn RegisterForPrintAsyncNotifications<P0, P4>(pszname: P0, pnotificationtype: *const windows_core::GUID, euserfilter: PrintAsyncNotifyUserFilter, econversationstyle: PrintAsyncNotifyConversationStyle, pcallback: P4) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<IPrintAsyncNotifyCallback>,
{
    windows_core::link!("winspool.drv" "system" fn RegisterForPrintAsyncNotifications(pszname : windows_core::PCWSTR, pnotificationtype : *const windows_core::GUID, euserfilter : PrintAsyncNotifyUserFilter, econversationstyle : PrintAsyncNotifyConversationStyle, pcallback : * mut core::ffi::c_void, phnotify : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RegisterForPrintAsyncNotifications(pszname.param().abi(), pnotificationtype, euserfilter, econversationstyle, pcallback.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RemovePrintDeviceObject(hdeviceobject: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("spoolss.dll" "system" fn RemovePrintDeviceObject(hdeviceobject : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { RemovePrintDeviceObject(hdeviceobject).ok() }
}
#[inline]
pub unsafe fn ReplyPrinterChangeNotification(hprinter: PRINTER_HANDLE, fdwchangeflags: u32, pdwresult: Option<*mut u32>, pprinternotifyinfo: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn ReplyPrinterChangeNotification(hprinter : PRINTER_HANDLE, fdwchangeflags : u32, pdwresult : *mut u32, pprinternotifyinfo : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ReplyPrinterChangeNotification(core::mem::transmute(hprinter), fdwchangeflags, pdwresult.unwrap_or(core::mem::zeroed()) as _, pprinternotifyinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ReplyPrinterChangeNotificationEx(hnotify: super::super::Foundation::HANDLE, dwcolor: u32, fdwflags: u32, pdwresult: *mut u32, pprinternotifyinfo: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn ReplyPrinterChangeNotificationEx(hnotify : super::super::Foundation:: HANDLE, dwcolor : u32, fdwflags : u32, pdwresult : *mut u32, pprinternotifyinfo : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ReplyPrinterChangeNotificationEx(hnotify, dwcolor, fdwflags, pdwresult as _, pprinternotifyinfo) }
}
#[inline]
pub unsafe fn ReportJobProcessingProgress(printerhandle: super::super::Foundation::HANDLE, jobid: u32, joboperation: EPrintXPSJobOperation, jobprogress: EPrintXPSJobProgress) -> windows_core::Result<()> {
    windows_core::link!("winspool.drv" "system" fn ReportJobProcessingProgress(printerhandle : super::super::Foundation:: HANDLE, jobid : u32, joboperation : EPrintXPSJobOperation, jobprogress : EPrintXPSJobProgress) -> windows_core::HRESULT);
    unsafe { ReportJobProcessingProgress(printerhandle, jobid, joboperation, jobprogress).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ResetPrinterA(hprinter: PRINTER_HANDLE, pdefault: Option<*const PRINTER_DEFAULTSA>) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn ResetPrinterA(hprinter : PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSA) -> windows_core::BOOL);
    unsafe { ResetPrinterA(core::mem::transmute(hprinter), pdefault.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ResetPrinterW(hprinter: PRINTER_HANDLE, pdefault: Option<*const PRINTER_DEFAULTSW>) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn ResetPrinterW(hprinter : PRINTER_HANDLE, pdefault : *const PRINTER_DEFAULTSW) -> windows_core::BOOL);
    unsafe { ResetPrinterW(core::mem::transmute(hprinter), pdefault.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RevertToPrinterSelf() -> super::super::Foundation::HANDLE {
    windows_core::link!("spoolss.dll" "system" fn RevertToPrinterSelf() -> super::super::Foundation:: HANDLE);
    unsafe { RevertToPrinterSelf() }
}
#[inline]
pub unsafe fn RouterAllocBidiMem(numbytes: usize) -> *mut core::ffi::c_void {
    windows_core::link!("spoolss.dll" "system" fn RouterAllocBidiMem(numbytes : usize) -> *mut core::ffi::c_void);
    unsafe { RouterAllocBidiMem(numbytes) }
}
#[inline]
pub unsafe fn RouterAllocBidiResponseContainer(count: u32) -> *mut BIDI_RESPONSE_CONTAINER {
    windows_core::link!("spoolss.dll" "system" fn RouterAllocBidiResponseContainer(count : u32) -> *mut BIDI_RESPONSE_CONTAINER);
    unsafe { RouterAllocBidiResponseContainer(count) }
}
#[inline]
pub unsafe fn RouterAllocPrinterNotifyInfo(cprinternotifyinfodata: u32) -> *mut PRINTER_NOTIFY_INFO {
    windows_core::link!("spoolss.dll" "system" fn RouterAllocPrinterNotifyInfo(cprinternotifyinfodata : u32) -> *mut PRINTER_NOTIFY_INFO);
    unsafe { RouterAllocPrinterNotifyInfo(cprinternotifyinfodata) }
}
#[inline]
pub unsafe fn RouterFreeBidiMem(pmempointer: *const core::ffi::c_void) {
    windows_core::link!("spoolss.dll" "system" fn RouterFreeBidiMem(pmempointer : *const core::ffi::c_void));
    unsafe { RouterFreeBidiMem(pmempointer) }
}
#[inline]
pub unsafe fn RouterFreeBidiResponseContainer(pdata: *const BIDI_RESPONSE_CONTAINER) -> u32 {
    windows_core::link!("winspool.drv" "system" fn RouterFreeBidiResponseContainer(pdata : *const BIDI_RESPONSE_CONTAINER) -> u32);
    unsafe { RouterFreeBidiResponseContainer(pdata) }
}
#[inline]
pub unsafe fn RouterFreePrinterNotifyInfo(pinfo: Option<*const PRINTER_NOTIFY_INFO>) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn RouterFreePrinterNotifyInfo(pinfo : *const PRINTER_NOTIFY_INFO) -> windows_core::BOOL);
    unsafe { RouterFreePrinterNotifyInfo(pinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ScheduleJob(hprinter: PRINTER_HANDLE, jobid: u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn ScheduleJob(hprinter : PRINTER_HANDLE, jobid : u32) -> windows_core::BOOL);
    unsafe { ScheduleJob(core::mem::transmute(hprinter), jobid) }
}
#[inline]
pub unsafe fn SetCPSUIUserData(hdlg: super::super::Foundation::HWND, cpsuiuserdata: usize) -> windows_core::BOOL {
    windows_core::link!("compstui.dll" "system" fn SetCPSUIUserData(hdlg : super::super::Foundation:: HWND, cpsuiuserdata : usize) -> windows_core::BOOL);
    unsafe { SetCPSUIUserData(hdlg, cpsuiuserdata) }
}
#[inline]
pub unsafe fn SetDefaultPrinterA<P0>(pszprinter: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn SetDefaultPrinterA(pszprinter : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetDefaultPrinterA(pszprinter.param().abi()) }
}
#[inline]
pub unsafe fn SetDefaultPrinterW<P0>(pszprinter: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn SetDefaultPrinterW(pszprinter : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetDefaultPrinterW(pszprinter.param().abi()) }
}
#[inline]
pub unsafe fn SetFormA<P1>(hprinter: PRINTER_HANDLE, pformname: P1, level: u32, pform: *const u8) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn SetFormA(hprinter : PRINTER_HANDLE, pformname : windows_core::PCSTR, level : u32, pform : *const u8) -> windows_core::BOOL);
    unsafe { SetFormA(core::mem::transmute(hprinter), pformname.param().abi(), level, pform) }
}
#[inline]
pub unsafe fn SetFormW<P1>(hprinter: PRINTER_HANDLE, pformname: P1, level: u32, pform: *const u8) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn SetFormW(hprinter : PRINTER_HANDLE, pformname : windows_core::PCWSTR, level : u32, pform : *const u8) -> windows_core::BOOL);
    unsafe { SetFormW(core::mem::transmute(hprinter), pformname.param().abi(), level, pform) }
}
#[inline]
pub unsafe fn SetJobA(hprinter: PRINTER_HANDLE, jobid: u32, level: u32, pjob: Option<*const u8>, command: u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn SetJobA(hprinter : PRINTER_HANDLE, jobid : u32, level : u32, pjob : *const u8, command : u32) -> windows_core::BOOL);
    unsafe { SetJobA(core::mem::transmute(hprinter), jobid, level, pjob.unwrap_or(core::mem::zeroed()) as _, command) }
}
#[inline]
pub unsafe fn SetJobNamedProperty(hprinter: PRINTER_HANDLE, jobid: u32, pproperty: *const PrintNamedProperty) -> u32 {
    windows_core::link!("winspool.drv" "system" fn SetJobNamedProperty(hprinter : PRINTER_HANDLE, jobid : u32, pproperty : *const PrintNamedProperty) -> u32);
    unsafe { SetJobNamedProperty(core::mem::transmute(hprinter), jobid, pproperty) }
}
#[inline]
pub unsafe fn SetJobW(hprinter: PRINTER_HANDLE, jobid: u32, level: u32, pjob: Option<*const u8>, command: u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn SetJobW(hprinter : PRINTER_HANDLE, jobid : u32, level : u32, pjob : *const u8, command : u32) -> windows_core::BOOL);
    unsafe { SetJobW(core::mem::transmute(hprinter), jobid, level, pjob.unwrap_or(core::mem::zeroed()) as _, command) }
}
#[inline]
pub unsafe fn SetPortA<P0, P1>(pname: P0, pportname: P1, dwlevel: u32, pportinfo: *const u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn SetPortA(pname : windows_core::PCSTR, pportname : windows_core::PCSTR, dwlevel : u32, pportinfo : *const u8) -> windows_core::BOOL);
    unsafe { SetPortA(pname.param().abi(), pportname.param().abi(), dwlevel, pportinfo).ok() }
}
#[inline]
pub unsafe fn SetPortW<P0, P1>(pname: P0, pportname: P1, dwlevel: u32, pportinfo: *const u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn SetPortW(pname : windows_core::PCWSTR, pportname : windows_core::PCWSTR, dwlevel : u32, pportinfo : *const u8) -> windows_core::BOOL);
    unsafe { SetPortW(pname.param().abi(), pportname.param().abi(), dwlevel, pportinfo).ok() }
}
#[inline]
pub unsafe fn SetPrinterA(hprinter: PRINTER_HANDLE, level: u32, pprinter: Option<*const u8>, command: u32) -> windows_core::Result<()> {
    windows_core::link!("winspool.drv" "system" fn SetPrinterA(hprinter : PRINTER_HANDLE, level : u32, pprinter : *const u8, command : u32) -> windows_core::BOOL);
    unsafe { SetPrinterA(core::mem::transmute(hprinter), level, pprinter.unwrap_or(core::mem::zeroed()) as _, command).ok() }
}
#[inline]
pub unsafe fn SetPrinterDataA<P1>(hprinter: PRINTER_HANDLE, pvaluename: P1, r#type: u32, pdata: &[u8]) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn SetPrinterDataA(hprinter : PRINTER_HANDLE, pvaluename : windows_core::PCSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
    unsafe { SetPrinterDataA(core::mem::transmute(hprinter), pvaluename.param().abi(), r#type, core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SetPrinterDataExA<P1, P2>(hprinter: PRINTER_HANDLE, pkeyname: P1, pvaluename: P2, r#type: u32, pdata: &[u8]) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn SetPrinterDataExA(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCSTR, pvaluename : windows_core::PCSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
    unsafe { SetPrinterDataExA(core::mem::transmute(hprinter), pkeyname.param().abi(), pvaluename.param().abi(), r#type, core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SetPrinterDataExW<P1, P2>(hprinter: PRINTER_HANDLE, pkeyname: P1, pvaluename: P2, r#type: u32, pdata: &[u8]) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn SetPrinterDataExW(hprinter : PRINTER_HANDLE, pkeyname : windows_core::PCWSTR, pvaluename : windows_core::PCWSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
    unsafe { SetPrinterDataExW(core::mem::transmute(hprinter), pkeyname.param().abi(), pvaluename.param().abi(), r#type, core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SetPrinterDataW<P1>(hprinter: PRINTER_HANDLE, pvaluename: P1, r#type: u32, pdata: &[u8]) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn SetPrinterDataW(hprinter : PRINTER_HANDLE, pvaluename : windows_core::PCWSTR, r#type : u32, pdata : *const u8, cbdata : u32) -> u32);
    unsafe { SetPrinterDataW(core::mem::transmute(hprinter), pvaluename.param().abi(), r#type, core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SetPrinterW(hprinter: PRINTER_HANDLE, level: u32, pprinter: Option<*const u8>, command: u32) -> windows_core::Result<()> {
    windows_core::link!("winspool.drv" "system" fn SetPrinterW(hprinter : PRINTER_HANDLE, level : u32, pprinter : *const u8, command : u32) -> windows_core::BOOL);
    unsafe { SetPrinterW(core::mem::transmute(hprinter), level, pprinter.unwrap_or(core::mem::zeroed()) as _, command).ok() }
}
#[inline]
pub unsafe fn SplIsSessionZero(hprinter: PRINTER_HANDLE, jobid: u32, pissessionzero: *mut windows_core::BOOL) -> u32 {
    windows_core::link!("spoolss.dll" "system" fn SplIsSessionZero(hprinter : PRINTER_HANDLE, jobid : u32, pissessionzero : *mut windows_core::BOOL) -> u32);
    unsafe { SplIsSessionZero(core::mem::transmute(hprinter), jobid, pissessionzero as _) }
}
#[inline]
pub unsafe fn SplPromptUIInUsersSession(hprinter: PRINTER_HANDLE, jobid: u32, puiparams: *const SHOWUIPARAMS, presponse: *mut u32) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn SplPromptUIInUsersSession(hprinter : PRINTER_HANDLE, jobid : u32, puiparams : *const SHOWUIPARAMS, presponse : *mut u32) -> windows_core::BOOL);
    unsafe { SplPromptUIInUsersSession(core::mem::transmute(hprinter), jobid, puiparams, presponse as _) }
}
#[inline]
pub unsafe fn SpoolerCopyFileEvent<P0, P1>(pszprintername: P0, pszkey: P1, dwcopyfileevent: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn SpoolerCopyFileEvent(pszprintername : windows_core::PCWSTR, pszkey : windows_core::PCWSTR, dwcopyfileevent : u32) -> windows_core::BOOL);
    unsafe { SpoolerCopyFileEvent(pszprintername.param().abi(), pszkey.param().abi(), dwcopyfileevent) }
}
#[inline]
pub unsafe fn SpoolerFindClosePrinterChangeNotification(hprinter: PRINTER_HANDLE) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn SpoolerFindClosePrinterChangeNotification(hprinter : PRINTER_HANDLE) -> windows_core::BOOL);
    unsafe { SpoolerFindClosePrinterChangeNotification(core::mem::transmute(hprinter)) }
}
#[inline]
pub unsafe fn SpoolerFindFirstPrinterChangeNotification(hprinter: PRINTER_HANDLE, fdwfilterflags: u32, fdwoptions: u32, pprinternotifyoptions: *const core::ffi::c_void, pvreserved: Option<*const core::ffi::c_void>, pnotificationconfig: *const core::ffi::c_void, phnotify: Option<*mut super::super::Foundation::HANDLE>, phevent: Option<*mut super::super::Foundation::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn SpoolerFindFirstPrinterChangeNotification(hprinter : PRINTER_HANDLE, fdwfilterflags : u32, fdwoptions : u32, pprinternotifyoptions : *const core::ffi::c_void, pvreserved : *const core::ffi::c_void, pnotificationconfig : *const core::ffi::c_void, phnotify : *mut super::super::Foundation:: HANDLE, phevent : *mut super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { SpoolerFindFirstPrinterChangeNotification(core::mem::transmute(hprinter), fdwfilterflags, fdwoptions, pprinternotifyoptions, pvreserved.unwrap_or(core::mem::zeroed()) as _, pnotificationconfig, phnotify.unwrap_or(core::mem::zeroed()) as _, phevent.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SpoolerFindNextPrinterChangeNotification(hprinter: PRINTER_HANDLE, pfdwchange: *mut u32, pprinternotifyoptions: Option<*const core::ffi::c_void>, ppprinternotifyinfo: Option<*mut *mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn SpoolerFindNextPrinterChangeNotification(hprinter : PRINTER_HANDLE, pfdwchange : *mut u32, pprinternotifyoptions : *const core::ffi::c_void, ppprinternotifyinfo : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SpoolerFindNextPrinterChangeNotification(core::mem::transmute(hprinter), pfdwchange as _, pprinternotifyoptions.unwrap_or(core::mem::zeroed()) as _, ppprinternotifyinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SpoolerFreePrinterNotifyInfo(pinfo: *const PRINTER_NOTIFY_INFO) {
    windows_core::link!("spoolss.dll" "system" fn SpoolerFreePrinterNotifyInfo(pinfo : *const PRINTER_NOTIFY_INFO));
    unsafe { SpoolerFreePrinterNotifyInfo(pinfo) }
}
#[inline]
pub unsafe fn SpoolerRefreshPrinterChangeNotification(hprinter: PRINTER_HANDLE, dwcolor: u32, poptions: *const PRINTER_NOTIFY_OPTIONS, ppinfo: Option<*mut *mut PRINTER_NOTIFY_INFO>) -> windows_core::BOOL {
    windows_core::link!("spoolss.dll" "system" fn SpoolerRefreshPrinterChangeNotification(hprinter : PRINTER_HANDLE, dwcolor : u32, poptions : *const PRINTER_NOTIFY_OPTIONS, ppinfo : *mut *mut PRINTER_NOTIFY_INFO) -> windows_core::BOOL);
    unsafe { SpoolerRefreshPrinterChangeNotification(core::mem::transmute(hprinter), dwcolor, poptions, ppinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn StartDocPrinterA(hprinter: PRINTER_HANDLE, level: u32, pdocinfo: *const DOC_INFO_1A) -> u32 {
    windows_core::link!("winspool.drv" "system" fn StartDocPrinterA(hprinter : PRINTER_HANDLE, level : u32, pdocinfo : *const DOC_INFO_1A) -> u32);
    unsafe { StartDocPrinterA(core::mem::transmute(hprinter), level, pdocinfo) }
}
#[inline]
pub unsafe fn StartDocPrinterW(hprinter: PRINTER_HANDLE, level: u32, pdocinfo: *const DOC_INFO_1W) -> u32 {
    windows_core::link!("winspool.drv" "system" fn StartDocPrinterW(hprinter : PRINTER_HANDLE, level : u32, pdocinfo : *const DOC_INFO_1W) -> u32);
    unsafe { StartDocPrinterW(core::mem::transmute(hprinter), level, pdocinfo) }
}
#[inline]
pub unsafe fn StartPagePrinter(hprinter: PRINTER_HANDLE) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn StartPagePrinter(hprinter : PRINTER_HANDLE) -> windows_core::BOOL);
    unsafe { StartPagePrinter(core::mem::transmute(hprinter)) }
}
#[inline]
pub unsafe fn UnRegisterForPrintAsyncNotifications(param0: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("winspool.drv" "system" fn UnRegisterForPrintAsyncNotifications(param0 : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { UnRegisterForPrintAsyncNotifications(param0).ok() }
}
#[inline]
pub unsafe fn UpdatePrintDeviceObject(hprinter: PRINTER_HANDLE, hdeviceobject: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("spoolss.dll" "system" fn UpdatePrintDeviceObject(hprinter : PRINTER_HANDLE, hdeviceobject : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { UpdatePrintDeviceObject(core::mem::transmute(hprinter), hdeviceobject).ok() }
}
#[inline]
pub unsafe fn UploadPrinterDriverPackageA<P0, P1, P2>(pszserver: P0, pszinfpath: P1, pszenvironment: P2, dwflags: u32, hwnd: super::super::Foundation::HWND, pszdestinfpath: windows_core::PSTR, pcchdestinfpath: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn UploadPrinterDriverPackageA(pszserver : windows_core::PCSTR, pszinfpath : windows_core::PCSTR, pszenvironment : windows_core::PCSTR, dwflags : u32, hwnd : super::super::Foundation:: HWND, pszdestinfpath : windows_core::PSTR, pcchdestinfpath : *mut u32) -> windows_core::HRESULT);
    unsafe { UploadPrinterDriverPackageA(pszserver.param().abi(), pszinfpath.param().abi(), pszenvironment.param().abi(), dwflags, hwnd, core::mem::transmute(pszdestinfpath), pcchdestinfpath as _).ok() }
}
#[inline]
pub unsafe fn UploadPrinterDriverPackageW<P0, P1, P2>(pszserver: P0, pszinfpath: P1, pszenvironment: P2, dwflags: u32, hwnd: super::super::Foundation::HWND, pszdestinfpath: windows_core::PWSTR, pcchdestinfpath: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn UploadPrinterDriverPackageW(pszserver : windows_core::PCWSTR, pszinfpath : windows_core::PCWSTR, pszenvironment : windows_core::PCWSTR, dwflags : u32, hwnd : super::super::Foundation:: HWND, pszdestinfpath : windows_core::PWSTR, pcchdestinfpath : *mut u32) -> windows_core::HRESULT);
    unsafe { UploadPrinterDriverPackageW(pszserver.param().abi(), pszinfpath.param().abi(), pszenvironment.param().abi(), dwflags, hwnd, core::mem::transmute(pszdestinfpath), pcchdestinfpath as _).ok() }
}
#[inline]
pub unsafe fn WaitForPrinterChange(hprinter: PRINTER_HANDLE, flags: u32) -> u32 {
    windows_core::link!("winspool.drv" "system" fn WaitForPrinterChange(hprinter : PRINTER_HANDLE, flags : u32) -> u32);
    unsafe { WaitForPrinterChange(core::mem::transmute(hprinter), flags) }
}
#[inline]
pub unsafe fn WritePrinter(hprinter: PRINTER_HANDLE, pbuf: *const core::ffi::c_void, cbbuf: u32, pcwritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winspool.drv" "system" fn WritePrinter(hprinter : PRINTER_HANDLE, pbuf : *const core::ffi::c_void, cbbuf : u32, pcwritten : *mut u32) -> windows_core::BOOL);
    unsafe { WritePrinter(core::mem::transmute(hprinter), pbuf, cbbuf, pcwritten as _) }
}
#[inline]
pub unsafe fn XcvDataW<P1>(hxcv: super::super::Foundation::HANDLE, pszdataname: P1, pinputdata: Option<&[u8]>, poutputdata: Option<&mut [u8]>, pcboutputneeded: *mut u32, pdwstatus: Option<*mut u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn XcvDataW(hxcv : super::super::Foundation:: HANDLE, pszdataname : windows_core::PCWSTR, pinputdata : *const u8, cbinputdata : u32, poutputdata : *mut u8, cboutputdata : u32, pcboutputneeded : *mut u32, pdwstatus : *mut u32) -> windows_core::BOOL);
    unsafe { XcvDataW(hxcv, pszdataname.param().abi(), core::mem::transmute(pinputdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pinputdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(poutputdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), poutputdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcboutputneeded as _, pdwstatus.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADDJOB_INFO_1A {
    pub Path: windows_core::PSTR,
    pub JobId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADDJOB_INFO_1W {
    pub Path: windows_core::PWSTR,
    pub JobId: u32,
}
pub const ALREADY_REGISTERED: PrintAsyncNotifyError = PrintAsyncNotifyError(15i32);
pub const ALREADY_UNREGISTERED: PrintAsyncNotifyError = PrintAsyncNotifyError(14i32);
pub const APD_COPY_ALL_FILES: u32 = 4u32;
pub const APD_COPY_FROM_DIRECTORY: u32 = 16u32;
pub const APD_COPY_NEW_FILES: u32 = 8u32;
pub const APD_STRICT_DOWNGRADE: u32 = 2u32;
pub const APD_STRICT_UPGRADE: u32 = 1u32;
pub const APPLYCPSUI_NO_NEWDEF: u32 = 1u32;
pub const APPLYCPSUI_OK_CANCEL_BUTTON: u32 = 2u32;
pub const ASYNC_CALL_ALREADY_PARKED: PrintAsyncNotifyError = PrintAsyncNotifyError(12i32);
pub const ASYNC_CALL_IN_PROGRESS: PrintAsyncNotifyError = PrintAsyncNotifyError(17i32);
pub const ASYNC_NOTIFICATION_FAILURE: PrintAsyncNotifyError = PrintAsyncNotifyError(6i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ATTRIBUTE_INFO_1 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ATTRIBUTE_INFO_2 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
    pub dwColorOptimization: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ATTRIBUTE_INFO_3 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
    pub dwColorOptimization: u32,
    pub dmPrintQuality: i16,
    pub dmYResolution: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ATTRIBUTE_INFO_4 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
    pub dwColorOptimization: u32,
    pub dmPrintQuality: i16,
    pub dmYResolution: i16,
    pub dwDuplexFlags: u32,
    pub dwNupDirection: u32,
    pub dwBookletFlags: u32,
    pub dwScalingPercentX: u32,
    pub dwScalingPercentY: u32,
}
pub const BIDI_ACCESS_ADMINISTRATOR: u32 = 1u32;
pub const BIDI_ACCESS_USER: u32 = 2u32;
pub const BIDI_ACTION_ENUM_SCHEMA: windows_core::PCWSTR = windows_core::w!("EnumSchema");
pub const BIDI_ACTION_GET: windows_core::PCWSTR = windows_core::w!("Get");
pub const BIDI_ACTION_GET_ALL: windows_core::PCWSTR = windows_core::w!("GetAll");
pub const BIDI_ACTION_GET_WITH_ARGUMENT: windows_core::PCWSTR = windows_core::w!("GetWithArgument");
pub const BIDI_ACTION_SET: windows_core::PCWSTR = windows_core::w!("Set");
pub const BIDI_BLOB: BIDI_TYPE = BIDI_TYPE(7i32);
pub const BIDI_BOOL: BIDI_TYPE = BIDI_TYPE(3i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BIDI_DATA {
    pub dwBidiType: u32,
    pub u: BIDI_DATA_0,
}
impl Default for BIDI_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BIDI_DATA_0 {
    pub bData: windows_core::BOOL,
    pub iData: i32,
    pub sData: windows_core::PWSTR,
    pub fData: f32,
    pub biData: BINARY_CONTAINER,
}
impl Default for BIDI_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BIDI_ENUM: BIDI_TYPE = BIDI_TYPE(6i32);
pub const BIDI_FLOAT: BIDI_TYPE = BIDI_TYPE(2i32);
pub const BIDI_INT: BIDI_TYPE = BIDI_TYPE(1i32);
pub const BIDI_NULL: BIDI_TYPE = BIDI_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BIDI_REQUEST_CONTAINER {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [BIDI_REQUEST_DATA; 1],
}
impl Default for BIDI_REQUEST_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BIDI_REQUEST_DATA {
    pub dwReqNumber: u32,
    pub pSchema: windows_core::PWSTR,
    pub data: BIDI_DATA,
}
impl Default for BIDI_REQUEST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BIDI_RESPONSE_CONTAINER {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [BIDI_RESPONSE_DATA; 1],
}
impl Default for BIDI_RESPONSE_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BIDI_RESPONSE_DATA {
    pub dwResult: u32,
    pub dwReqNumber: u32,
    pub pSchema: windows_core::PWSTR,
    pub data: BIDI_DATA,
}
impl Default for BIDI_RESPONSE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BIDI_STRING: BIDI_TYPE = BIDI_TYPE(4i32);
pub const BIDI_TEXT: BIDI_TYPE = BIDI_TYPE(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BIDI_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BINARY_CONTAINER {
    pub cbBuf: u32,
    pub pData: *mut u8,
}
impl Default for BINARY_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BOOKLET_EDGE_LEFT: u32 = 0u32;
pub const BOOKLET_EDGE_RIGHT: u32 = 1u32;
pub const BOOKLET_PRINT: u32 = 2u32;
pub const BORDER_PRINT: u32 = 0u32;
pub const BidiRequest: windows_core::GUID = windows_core::GUID::from_u128(0xb9162a23_45f9_47cc_80f5_fe0fe9b9e1a2);
pub const BidiRequestContainer: windows_core::GUID = windows_core::GUID::from_u128(0xfc5b8a24_db05_4a01_8388_22edf6c2bbba);
pub const BidiSpl: windows_core::GUID = windows_core::GUID::from_u128(0x2a614240_a4c5_4c33_bd87_1bc709331639);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BranchOfficeJobData {
    pub eEventType: EBranchOfficeJobEventType,
    pub JobId: u32,
    pub JobInfo: BranchOfficeJobData_0,
}
impl Default for BranchOfficeJobData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BranchOfficeJobData_0 {
    pub LogJobPrinted: BranchOfficeJobDataPrinted,
    pub LogJobRendered: BranchOfficeJobDataRendered,
    pub LogJobError: BranchOfficeJobDataError,
    pub LogPipelineFailed: BranchOfficeJobDataPipelineFailed,
    pub LogOfflineFileFull: BranchOfficeLogOfflineFileFull,
}
impl Default for BranchOfficeJobData_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BranchOfficeJobDataContainer {
    pub cJobDataEntries: u32,
    pub JobData: [BranchOfficeJobData; 1],
}
impl Default for BranchOfficeJobDataContainer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BranchOfficeJobDataError {
    pub LastError: u32,
    pub pDocumentName: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub pPrinterName: windows_core::PWSTR,
    pub pDataType: windows_core::PWSTR,
    pub TotalSize: i64,
    pub PrintedSize: i64,
    pub TotalPages: u32,
    pub PrintedPages: u32,
    pub pMachineName: windows_core::PWSTR,
    pub pJobError: windows_core::PWSTR,
    pub pErrorDescription: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BranchOfficeJobDataPipelineFailed {
    pub pDocumentName: windows_core::PWSTR,
    pub pPrinterName: windows_core::PWSTR,
    pub pExtraErrorInfo: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BranchOfficeJobDataPrinted {
    pub Status: u32,
    pub pDocumentName: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub pMachineName: windows_core::PWSTR,
    pub pPrinterName: windows_core::PWSTR,
    pub pPortName: windows_core::PWSTR,
    pub Size: i64,
    pub TotalPages: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BranchOfficeJobDataRendered {
    pub Size: i64,
    pub ICMMethod: u32,
    pub Color: i16,
    pub PrintQuality: i16,
    pub YResolution: i16,
    pub Copies: i16,
    pub TTOption: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BranchOfficeLogOfflineFileFull {
    pub pMachineName: windows_core::PWSTR,
}
pub const CC_BIG5: i32 = -10i32;
pub const CC_CP437: i32 = -1i32;
pub const CC_CP850: i32 = -2i32;
pub const CC_CP863: i32 = -3i32;
pub const CC_DEFAULT: u32 = 0u32;
pub const CC_GB2312: i32 = -16i32;
pub const CC_ISC: i32 = -11i32;
pub const CC_JIS: i32 = -12i32;
pub const CC_JIS_ANK: i32 = -13i32;
pub const CC_NOPRECNV: u32 = 65535u32;
pub const CC_NS86: i32 = -14i32;
pub const CC_SJIS: i32 = -17i32;
pub const CC_TCA: i32 = -15i32;
pub const CC_WANSUNG: i32 = -18i32;
pub const CDM_CONVERT: u32 = 1u32;
pub const CDM_CONVERT351: u32 = 2u32;
pub const CDM_DRIVER_DEFAULT: u32 = 4u32;
pub const CHANNEL_ACQUIRED: PrintAsyncNotifyError = PrintAsyncNotifyError(16i32);
pub const CHANNEL_ALREADY_CLOSED: PrintAsyncNotifyError = PrintAsyncNotifyError(8i32);
pub const CHANNEL_ALREADY_OPENED: PrintAsyncNotifyError = PrintAsyncNotifyError(9i32);
pub const CHANNEL_CLOSED_BY_ANOTHER_LISTENER: PrintAsyncNotifyError = PrintAsyncNotifyError(2i32);
pub const CHANNEL_CLOSED_BY_SAME_LISTENER: PrintAsyncNotifyError = PrintAsyncNotifyError(3i32);
pub const CHANNEL_CLOSED_BY_SERVER: PrintAsyncNotifyError = PrintAsyncNotifyError(1i32);
pub const CHANNEL_NOT_OPENED: PrintAsyncNotifyError = PrintAsyncNotifyError(11i32);
pub const CHANNEL_RELEASED_BY_LISTENER: PrintAsyncNotifyError = PrintAsyncNotifyError(4i32);
pub const CHANNEL_WAITING_FOR_CLIENT_NOTIFICATION: PrintAsyncNotifyError = PrintAsyncNotifyError(10i32);
pub const CHKBOXS_FALSE_PDATA: u32 = 3u32;
pub const CHKBOXS_FALSE_TRUE: u32 = 0u32;
pub const CHKBOXS_NONE_PDATA: u32 = 6u32;
pub const CHKBOXS_NO_PDATA: u32 = 4u32;
pub const CHKBOXS_NO_YES: u32 = 1u32;
pub const CHKBOXS_OFF_ON: u32 = 2u32;
pub const CHKBOXS_OFF_PDATA: u32 = 5u32;
pub const CLSID_OEMPTPROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0x91723892_45d2_48e2_9ec9_562379daf992);
pub const CLSID_OEMRENDER: windows_core::GUID = windows_core::GUID::from_u128(0x6d6abf26_9f38_11d1_882a_00c04fb961ec);
pub const CLSID_OEMUI: windows_core::GUID = windows_core::GUID::from_u128(0xabce80d7_9f46_11d1_882a_00c04fb961ec);
pub const CLSID_OEMUIMXDC: windows_core::GUID = windows_core::GUID::from_u128(0x4e144300_5b43_4288_932a_5e4dd6d82bed);
pub const CLSID_PTPROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0x46ac151b_8490_4531_96cc_55bf2bf19e11);
pub const CLSID_XPSRASTERIZER_FACTORY: windows_core::GUID = windows_core::GUID::from_u128(0x503e79bf_1d09_4764_9d72_1eb0c65967c6);
pub const COLOR_OPTIMIZATION: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug)]
pub struct COMPROPSHEETUI {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::super::Foundation::HINSTANCE,
    pub pCallerName: *mut i8,
    pub UserData: usize,
    pub pHelpFile: *mut i8,
    pub pfnCallBack: _CPSUICALLBACK,
    pub pOptItem: *mut OPTITEM,
    pub pDlgPage: *mut DLGPAGE,
    pub cOptItem: u16,
    pub cDlgPage: u16,
    pub IconID: usize,
    pub pOptItemName: *mut i8,
    pub CallerVersion: u16,
    pub OptItemVersion: u16,
    pub dwReserved: [usize; 4],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for COMPROPSHEETUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONFIG_INFO_DATA_1 {
    pub Reserved: [u8; 128],
    pub dwVersion: u32,
}
impl Default for CONFIG_INFO_DATA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COPYFILE_EVENT_ADD_PRINTER_CONNECTION: u32 = 3u32;
pub const COPYFILE_EVENT_DELETE_PRINTER: u32 = 2u32;
pub const COPYFILE_EVENT_DELETE_PRINTER_CONNECTION: u32 = 4u32;
pub const COPYFILE_EVENT_FILES_CHANGED: u32 = 5u32;
pub const COPYFILE_EVENT_SET_PRINTER_DATAEX: u32 = 1u32;
pub const COPYFILE_FLAG_CLIENT_SPOOLER: u32 = 1u32;
pub const COPYFILE_FLAG_SERVER_SPOOLER: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CORE_PRINTER_DRIVERA {
    pub CoreDriverGUID: windows_core::GUID,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub szPackageID: [i8; 260],
}
impl Default for CORE_PRINTER_DRIVERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CORE_PRINTER_DRIVERW {
    pub CoreDriverGUID: windows_core::GUID,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub szPackageID: [u16; 260],
}
impl Default for CORE_PRINTER_DRIVERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPSFUNC_ADD_HPROPSHEETPAGE: u32 = 0u32;
pub const CPSFUNC_ADD_PCOMPROPSHEETUI: u32 = 3u32;
pub const CPSFUNC_ADD_PCOMPROPSHEETUIA: u32 = 2u32;
pub const CPSFUNC_ADD_PCOMPROPSHEETUIW: u32 = 3u32;
pub const CPSFUNC_ADD_PFNPROPSHEETUI: u32 = 5u32;
pub const CPSFUNC_ADD_PFNPROPSHEETUIA: u32 = 4u32;
pub const CPSFUNC_ADD_PFNPROPSHEETUIW: u32 = 5u32;
pub const CPSFUNC_ADD_PROPSHEETPAGE: u32 = 1u32;
pub const CPSFUNC_ADD_PROPSHEETPAGEA: u32 = 15u32;
pub const CPSFUNC_ADD_PROPSHEETPAGEW: u32 = 1u32;
pub const CPSFUNC_DELETE_HCOMPROPSHEET: u32 = 6u32;
pub const CPSFUNC_DO_APPLY_CPSUI: u32 = 25u32;
pub const CPSFUNC_GET_HPSUIPAGES: u32 = 10u32;
pub const CPSFUNC_GET_PAGECOUNT: u32 = 8u32;
pub const CPSFUNC_GET_PFNPROPSHEETUI_ICON: u32 = 14u32;
pub const CPSFUNC_IGNORE_CPSUI_PSN_APPLY: u32 = 24u32;
pub const CPSFUNC_INSERT_PSUIPAGE: u32 = 17u32;
pub const CPSFUNC_INSERT_PSUIPAGEA: u32 = 16u32;
pub const CPSFUNC_INSERT_PSUIPAGEW: u32 = 17u32;
pub const CPSFUNC_LOAD_CPSUI_ICON: u32 = 13u32;
pub const CPSFUNC_LOAD_CPSUI_STRING: u32 = 12u32;
pub const CPSFUNC_LOAD_CPSUI_STRINGA: u32 = 11u32;
pub const CPSFUNC_LOAD_CPSUI_STRINGW: u32 = 12u32;
pub const CPSFUNC_QUERY_DATABLOCK: u32 = 22u32;
pub const CPSFUNC_SET_DATABLOCK: u32 = 21u32;
pub const CPSFUNC_SET_DMPUB_HIDEBITS: u32 = 23u32;
pub const CPSFUNC_SET_FUSION_CONTEXT: u32 = 26u32;
pub const CPSFUNC_SET_HSTARTPAGE: u32 = 7u32;
pub const CPSFUNC_SET_PSUIPAGE_ICON: u32 = 20u32;
pub const CPSFUNC_SET_PSUIPAGE_TITLE: u32 = 19u32;
pub const CPSFUNC_SET_PSUIPAGE_TITLEA: u32 = 18u32;
pub const CPSFUNC_SET_PSUIPAGE_TITLEW: u32 = 19u32;
pub const CPSFUNC_SET_RESULT: u32 = 9u32;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct CPSUICBPARAM {
    pub cbSize: u16,
    pub Reason: u16,
    pub hDlg: super::super::Foundation::HWND,
    pub pOptItem: *mut OPTITEM,
    pub cOptItem: u16,
    pub Flags: u16,
    pub pCurItem: *mut OPTITEM,
    pub Anonymous: CPSUICBPARAM_0,
    pub UserData: usize,
    pub Result: usize,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for CPSUICBPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union CPSUICBPARAM_0 {
    pub OldSel: i32,
    pub pOldSel: *mut i8,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for CPSUICBPARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPSUICB_ACTION_ITEMS_APPLIED: u32 = 4u32;
pub const CPSUICB_ACTION_NONE: u32 = 0u32;
pub const CPSUICB_ACTION_NO_APPLY_EXIT: u32 = 3u32;
pub const CPSUICB_ACTION_OPTIF_CHANGED: u32 = 1u32;
pub const CPSUICB_ACTION_REINIT_ITEMS: u32 = 2u32;
pub const CPSUICB_REASON_ABOUT: u32 = 9u32;
pub const CPSUICB_REASON_APPLYNOW: u32 = 6u32;
pub const CPSUICB_REASON_DLGPROC: u32 = 3u32;
pub const CPSUICB_REASON_ECB_CHANGED: u32 = 2u32;
pub const CPSUICB_REASON_EXTPUSH: u32 = 5u32;
pub const CPSUICB_REASON_ITEMS_REVERTED: u32 = 8u32;
pub const CPSUICB_REASON_KILLACTIVE: u32 = 11u32;
pub const CPSUICB_REASON_OPTITEM_SETFOCUS: u32 = 7u32;
pub const CPSUICB_REASON_PUSHBUTTON: u32 = 1u32;
pub const CPSUICB_REASON_SEL_CHANGED: u32 = 0u32;
pub const CPSUICB_REASON_SETACTIVE: u32 = 10u32;
pub const CPSUICB_REASON_UNDO_CHANGES: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPSUIDATABLOCK {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl Default for CPSUIDATABLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPSUIF_ABOUT_CALLBACK: u32 = 4u32;
pub const CPSUIF_ICONID_AS_HICON: u32 = 2u32;
pub const CPSUIF_UPDATE_PERMISSION: u32 = 1u32;
pub const CPSUI_CANCEL: u32 = 0u32;
pub const CPSUI_OK: u32 = 1u32;
pub const CPSUI_REBOOTSYSTEM: u32 = 3u32;
pub const CPSUI_RESTARTWINDOWS: u32 = 2u32;
pub const CUSTOMPARAM_HEIGHT: u32 = 1u32;
pub const CUSTOMPARAM_HEIGHTOFFSET: u32 = 3u32;
pub const CUSTOMPARAM_MAX: u32 = 5u32;
pub const CUSTOMPARAM_ORIENTATION: u32 = 4u32;
pub const CUSTOMPARAM_WIDTH: u32 = 0u32;
pub const CUSTOMPARAM_WIDTHOFFSET: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CUSTOMSIZEPARAM {
    pub dwOrder: i32,
    pub lMinVal: i32,
    pub lMaxVal: i32,
}
pub const Compression_Fast: EXpsCompressionOptions = EXpsCompressionOptions(3i32);
pub const Compression_Normal: EXpsCompressionOptions = EXpsCompressionOptions(1i32);
pub const Compression_NotCompressed: EXpsCompressionOptions = EXpsCompressionOptions(0i32);
pub const Compression_Small: EXpsCompressionOptions = EXpsCompressionOptions(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DATATYPES_INFO_1A {
    pub pName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DATATYPES_INFO_1W {
    pub pName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DATA_HEADER {
    pub dwSignature: u32,
    pub wSize: u16,
    pub wDataID: u16,
    pub dwDataSize: u32,
    pub dwReserved: u32,
}
pub const DEF_PRIORITY: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DELETE_PORT_DATA_1 {
    pub psztPortName: [u16; 64],
    pub Reserved: [u8; 98],
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl Default for DELETE_PORT_DATA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICEPROPERTYHEADER {
    pub cbSize: u16,
    pub Flags: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pszPrinterName: *mut i8,
}
impl Default for DEVICEPROPERTYHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVQUERYPRINT_INFO {
    pub cbSize: u16,
    pub Level: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pszErrorStr: windows_core::PWSTR,
    pub cchErrorStr: u32,
    pub cchNeeded: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DEVQUERYPRINT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DF_BKSP_OK: u32 = 64u32;
pub const DF_NOITALIC: u32 = 1u32;
pub const DF_NOUNDER: u32 = 2u32;
pub const DF_NO_BOLD: u32 = 8u32;
pub const DF_NO_DOUBLE_UNDERLINE: u32 = 16u32;
pub const DF_NO_STRIKETHRU: u32 = 32u32;
pub const DF_TYPE_CAPSL: u32 = 3u32;
pub const DF_TYPE_HPINTELLIFONT: u32 = 0u32;
pub const DF_TYPE_OEM1: u32 = 4u32;
pub const DF_TYPE_OEM2: u32 = 5u32;
pub const DF_TYPE_PST1: u32 = 2u32;
pub const DF_TYPE_TRUETYPE: u32 = 1u32;
pub const DF_XM_CR: u32 = 4u32;
pub const DISPID_PRINTEREXTENSION_CONTEXT: u32 = 11800u32;
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION: u32 = 12100u32;
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION_COUNT: u32 = 12101u32;
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION_GETAT: u32 = 12102u32;
pub const DISPID_PRINTEREXTENSION_CONTEXT_DRIVERPROPERTIES: u32 = 11803u32;
pub const DISPID_PRINTEREXTENSION_CONTEXT_PRINTERQUEUE: u32 = 11801u32;
pub const DISPID_PRINTEREXTENSION_CONTEXT_PRINTSCHEMATICKET: u32 = 11802u32;
pub const DISPID_PRINTEREXTENSION_CONTEXT_USERPROPERTIES: u32 = 11804u32;
pub const DISPID_PRINTEREXTENSION_EVENT: u32 = 12200u32;
pub const DISPID_PRINTEREXTENSION_EVENTARGS: u32 = 12000u32;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_BIDINOTIFICATION: u32 = 12001u32;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_DETAILEDREASONID: u32 = 12005u32;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_REASONID: u32 = 12002u32;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_REQUEST: u32 = 12003u32;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_SOURCEAPPLICATION: u32 = 12004u32;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_WINDOWMODAL: u32 = 12006u32;
pub const DISPID_PRINTEREXTENSION_EVENTARGS_WINDOWPARENT: u32 = 12007u32;
pub const DISPID_PRINTEREXTENSION_EVENT_ONDRIVEREVENT: u32 = 12201u32;
pub const DISPID_PRINTEREXTENSION_EVENT_ONPRINTERQUEUESENUMERATED: u32 = 12202u32;
pub const DISPID_PRINTEREXTENSION_REQUEST: u32 = 11900u32;
pub const DISPID_PRINTEREXTENSION_REQUEST_CANCEL: u32 = 11901u32;
pub const DISPID_PRINTEREXTENSION_REQUEST_COMPLETE: u32 = 11902u32;
pub const DISPID_PRINTERPROPERTYBAG: u32 = 11400u32;
pub const DISPID_PRINTERPROPERTYBAG_GETBOOL: u32 = 11401u32;
pub const DISPID_PRINTERPROPERTYBAG_GETBYTES: u32 = 11407u32;
pub const DISPID_PRINTERPROPERTYBAG_GETINT32: u32 = 11403u32;
pub const DISPID_PRINTERPROPERTYBAG_GETREADSTREAM: u32 = 11409u32;
pub const DISPID_PRINTERPROPERTYBAG_GETSTRING: u32 = 11405u32;
pub const DISPID_PRINTERPROPERTYBAG_GETWRITESTREAM: u32 = 11410u32;
pub const DISPID_PRINTERPROPERTYBAG_SETBOOL: u32 = 11402u32;
pub const DISPID_PRINTERPROPERTYBAG_SETBYTES: u32 = 11408u32;
pub const DISPID_PRINTERPROPERTYBAG_SETINT32: u32 = 11404u32;
pub const DISPID_PRINTERPROPERTYBAG_SETSTRING: u32 = 11406u32;
pub const DISPID_PRINTERQUEUE: u32 = 11600u32;
pub const DISPID_PRINTERQUEUEEVENT: u32 = 11700u32;
pub const DISPID_PRINTERQUEUEEVENT_ONBIDIRESPONSERECEIVED: u32 = 11701u32;
pub const DISPID_PRINTERQUEUEVIEW: u32 = 12700u32;
pub const DISPID_PRINTERQUEUEVIEW_EVENT: u32 = 12800u32;
pub const DISPID_PRINTERQUEUEVIEW_EVENT_ONCHANGED: u32 = 12801u32;
pub const DISPID_PRINTERQUEUEVIEW_SETVIEWRANGE: u32 = 12701u32;
pub const DISPID_PRINTERQUEUE_GETPRINTERQUEUEVIEW: u32 = 11606u32;
pub const DISPID_PRINTERQUEUE_GETPROPERTIES: u32 = 11604u32;
pub const DISPID_PRINTERQUEUE_HANDLE: u32 = 11601u32;
pub const DISPID_PRINTERQUEUE_NAME: u32 = 11602u32;
pub const DISPID_PRINTERQUEUE_SENDBIDIQUERY: u32 = 11603u32;
pub const DISPID_PRINTERQUEUE_SENDBIDISETREQUESTASYNC: u32 = 11605u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG: u32 = 11500u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETBOOL: u32 = 11501u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETBYTES: u32 = 11507u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETINT32: u32 = 11503u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETREADSTREAM: u32 = 11509u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETSTREAMASXML: u32 = 11411u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETSTRING: u32 = 11505u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETWRITESTREAM: u32 = 11510u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETBOOL: u32 = 11502u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETBYTES: u32 = 11508u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETINT32: u32 = 11504u32;
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETSTRING: u32 = 11506u32;
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM: u32 = 11200u32;
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM_READ: u32 = 11201u32;
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM_WRITE: u32 = 11202u32;
pub const DISPID_PRINTERSCRIPTABLESTREAM: u32 = 11300u32;
pub const DISPID_PRINTERSCRIPTABLESTREAM_COMMIT: u32 = 11301u32;
pub const DISPID_PRINTERSCRIPTABLESTREAM_SEEK: u32 = 11302u32;
pub const DISPID_PRINTERSCRIPTABLESTREAM_SETSIZE: u32 = 11303u32;
pub const DISPID_PRINTERSCRIPTCONTEXT: u32 = 12300u32;
pub const DISPID_PRINTERSCRIPTCONTEXT_DRIVERPROPERTIES: u32 = 12301u32;
pub const DISPID_PRINTERSCRIPTCONTEXT_QUEUEPROPERTIES: u32 = 12302u32;
pub const DISPID_PRINTERSCRIPTCONTEXT_USERPROPERTIES: u32 = 12303u32;
pub const DISPID_PRINTJOBCOLLECTION: u32 = 12600u32;
pub const DISPID_PRINTJOBCOLLECTION_COUNT: u32 = 12601u32;
pub const DISPID_PRINTJOBCOLLECTION_GETAT: u32 = 12602u32;
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION: u32 = 10900u32;
pub const DISPID_PRINTSCHEMA_ASYNCOPERATIONEVENT: u32 = 11100u32;
pub const DISPID_PRINTSCHEMA_ASYNCOPERATIONEVENT_COMPLETED: u32 = 11101u32;
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION_CANCEL: u32 = 10902u32;
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION_START: u32 = 10901u32;
pub const DISPID_PRINTSCHEMA_CAPABILITIES: u32 = 10800u32;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETFEATURE: u32 = 10802u32;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETFEATURE_KEYNAME: u32 = 10801u32;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETOPTIONS: u32 = 10807u32;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETPARAMETERDEFINITION: u32 = 10808u32;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETSELECTEDOPTION: u32 = 10806u32;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_JOBCOPIESMAXVALUE: u32 = 10805u32;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_JOBCOPIESMINVALUE: u32 = 10804u32;
pub const DISPID_PRINTSCHEMA_CAPABILITIES_PAGEIMAGEABLESIZE: u32 = 10803u32;
pub const DISPID_PRINTSCHEMA_DISPLAYABLEELEMENT: u32 = 10100u32;
pub const DISPID_PRINTSCHEMA_DISPLAYABLEELEMENT_DISPLAYNAME: u32 = 10101u32;
pub const DISPID_PRINTSCHEMA_ELEMENT: u32 = 10000u32;
pub const DISPID_PRINTSCHEMA_ELEMENT_NAME: u32 = 10002u32;
pub const DISPID_PRINTSCHEMA_ELEMENT_NAMESPACEURI: u32 = 10003u32;
pub const DISPID_PRINTSCHEMA_ELEMENT_XMLNODE: u32 = 10001u32;
pub const DISPID_PRINTSCHEMA_FEATURE: u32 = 10600u32;
pub const DISPID_PRINTSCHEMA_FEATURE_DISPLAYUI: u32 = 10604u32;
pub const DISPID_PRINTSCHEMA_FEATURE_GETOPTION: u32 = 10603u32;
pub const DISPID_PRINTSCHEMA_FEATURE_SELECTEDOPTION: u32 = 10601u32;
pub const DISPID_PRINTSCHEMA_FEATURE_SELECTIONTYPE: u32 = 10602u32;
pub const DISPID_PRINTSCHEMA_NUPOPTION: u32 = 10400u32;
pub const DISPID_PRINTSCHEMA_NUPOPTION_PAGESPERSHEET: u32 = 10401u32;
pub const DISPID_PRINTSCHEMA_OPTION: u32 = 10200u32;
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION: u32 = 10500u32;
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION_COUNT: u32 = 10501u32;
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION_GETAT: u32 = 10502u32;
pub const DISPID_PRINTSCHEMA_OPTION_CONSTRAINED: u32 = 10202u32;
pub const DISPID_PRINTSCHEMA_OPTION_GETPROPERTYVALUE: u32 = 10203u32;
pub const DISPID_PRINTSCHEMA_OPTION_SELECTED: u32 = 10201u32;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE: u32 = 10700u32;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_EXTENT_HEIGHT: u32 = 10706u32;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_EXTENT_WIDTH: u32 = 10705u32;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_IMAGEABLE_HEIGHT: u32 = 10702u32;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_IMAGEABLE_WIDTH: u32 = 10701u32;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_ORIGIN_HEIGHT: u32 = 10704u32;
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_ORIGIN_WIDTH: u32 = 10703u32;
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION: u32 = 10300u32;
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION_HEIGHT: u32 = 10302u32;
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION_WIDTH: u32 = 10301u32;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION: u32 = 12500u32;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_DATATYPE: u32 = 12503u32;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_RANGEMAX: u32 = 12505u32;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_RANGEMIN: u32 = 12504u32;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_UNITTYPE: u32 = 12502u32;
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_USERINPUTREQUIRED: u32 = 12501u32;
pub const DISPID_PRINTSCHEMA_PARAMETERINITIALIZER: u32 = 12400u32;
pub const DISPID_PRINTSCHEMA_PARAMETERINITIALIZER_VALUE: u32 = 12401u32;
pub const DISPID_PRINTSCHEMA_TICKET: u32 = 11000u32;
pub const DISPID_PRINTSCHEMA_TICKET_COMMITASYNC: u32 = 11004u32;
pub const DISPID_PRINTSCHEMA_TICKET_GETCAPABILITIES: u32 = 11006u32;
pub const DISPID_PRINTSCHEMA_TICKET_GETFEATURE: u32 = 11002u32;
pub const DISPID_PRINTSCHEMA_TICKET_GETFEATURE_KEYNAME: u32 = 11001u32;
pub const DISPID_PRINTSCHEMA_TICKET_GETPARAMETERINITIALIZER: u32 = 11008u32;
pub const DISPID_PRINTSCHEMA_TICKET_JOBCOPIESALLDOCUMENTS: u32 = 11007u32;
pub const DISPID_PRINTSCHEMA_TICKET_NOTIFYXMLCHANGED: u32 = 11005u32;
pub const DISPID_PRINTSCHEMA_TICKET_VALIDATEASYNC: u32 = 11003u32;
pub const DI_CHANNEL: u32 = 1u32;
pub const DI_MEMORYMAP_WRITE: u32 = 1u32;
pub const DI_READ_SPOOL_JOB: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct DLGPAGE {
    pub cbSize: u16,
    pub Flags: u16,
    pub DlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub pTabName: *mut i8,
    pub IconID: usize,
    pub Anonymous: DLGPAGE_0,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for DLGPAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union DLGPAGE_0 {
    pub DlgTemplateID: u16,
    pub hDlgTemplate: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for DLGPAGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMPUB_BOOKLET_EDGE: u32 = 21u32;
pub const DMPUB_COLOR: u32 = 6u32;
pub const DMPUB_COPIES_COLLATE: u32 = 3u32;
pub const DMPUB_DEFSOURCE: u32 = 4u32;
pub const DMPUB_DITHERTYPE: u32 = 13u32;
pub const DMPUB_DUPLEX: u32 = 7u32;
pub const DMPUB_FIRST: u32 = 1u32;
pub const DMPUB_FORMNAME: u32 = 9u32;
pub const DMPUB_ICMINTENT: u32 = 11u32;
pub const DMPUB_ICMMETHOD: u32 = 10u32;
pub const DMPUB_LAST: u32 = 21u32;
pub const DMPUB_MANUAL_DUPLEX: u32 = 19u32;
pub const DMPUB_MEDIATYPE: u32 = 12u32;
pub const DMPUB_NONE: u32 = 0u32;
pub const DMPUB_NUP: u32 = 16u32;
pub const DMPUB_NUP_DIRECTION: u32 = 18u32;
pub const DMPUB_OEM_GRAPHIC_ITEM: u32 = 98u32;
pub const DMPUB_OEM_PAPER_ITEM: u32 = 97u32;
pub const DMPUB_OEM_ROOT_ITEM: u32 = 99u32;
pub const DMPUB_ORIENTATION: u32 = 1u32;
pub const DMPUB_OUTPUTBIN: u32 = 14u32;
pub const DMPUB_PAGEORDER: u32 = 17u32;
pub const DMPUB_PRINTQUALITY: u32 = 5u32;
pub const DMPUB_QUALITY: u32 = 15u32;
pub const DMPUB_SCALE: u32 = 2u32;
pub const DMPUB_STAPLE: u32 = 20u32;
pub const DMPUB_TTOPTION: u32 = 8u32;
pub const DMPUB_USER: u32 = 100u32;
pub const DM_ADVANCED: u32 = 16u32;
pub const DM_INVALIDATE_DRIVER_CACHE: u32 = 536870912u32;
pub const DM_NOPERMISSION: u32 = 32u32;
pub const DM_PROMPT_NON_MODAL: u32 = 1073741824u32;
pub const DM_RESERVED: u32 = 2147483648u32;
pub const DM_USER_DEFAULT: u32 = 64u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOCEVENT_CREATEDCPRE {
    pub pszDriver: windows_core::PWSTR,
    pub pszDevice: windows_core::PWSTR,
    pub pdm: *mut super::Gdi::DEVMODEW,
    pub bIC: windows_core::BOOL,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DOCEVENT_CREATEDCPRE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOCEVENT_ESCAPE {
    pub iEscape: i32,
    pub cjInput: i32,
    pub pvInData: *mut core::ffi::c_void,
}
impl Default for DOCEVENT_ESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOCEVENT_FILTER {
    pub cbSize: u32,
    pub cElementsAllocated: u32,
    pub cElementsNeeded: u32,
    pub cElementsReturned: u32,
    pub aDocEventCall: [u32; 1],
}
impl Default for DOCEVENT_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOCUMENTEVENT_ABORTDOC: u32 = 9u32;
pub const DOCUMENTEVENT_CREATEDCPOST: u32 = 2u32;
pub const DOCUMENTEVENT_CREATEDCPRE: u32 = 1u32;
pub const DOCUMENTEVENT_DELETEDC: u32 = 10u32;
pub const DOCUMENTEVENT_ENDDOC: u32 = 8u32;
pub const DOCUMENTEVENT_ENDDOCPOST: u32 = 12u32;
pub const DOCUMENTEVENT_ENDDOCPRE: u32 = 8u32;
pub const DOCUMENTEVENT_ENDPAGE: u32 = 7u32;
pub const DOCUMENTEVENT_ESCAPE: u32 = 11u32;
pub const DOCUMENTEVENT_FAILURE: i32 = -1i32;
pub const DOCUMENTEVENT_FIRST: u32 = 1u32;
pub const DOCUMENTEVENT_LAST: u32 = 15u32;
pub const DOCUMENTEVENT_QUERYFILTER: u32 = 14u32;
pub const DOCUMENTEVENT_RESETDCPOST: u32 = 4u32;
pub const DOCUMENTEVENT_RESETDCPRE: u32 = 3u32;
pub const DOCUMENTEVENT_SPOOLED: u32 = 65536u32;
pub const DOCUMENTEVENT_STARTDOC: u32 = 5u32;
pub const DOCUMENTEVENT_STARTDOCPOST: u32 = 13u32;
pub const DOCUMENTEVENT_STARTDOCPRE: u32 = 5u32;
pub const DOCUMENTEVENT_STARTPAGE: u32 = 6u32;
pub const DOCUMENTEVENT_SUCCESS: u32 = 1u32;
pub const DOCUMENTEVENT_UNSUPPORTED: u32 = 0u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPOST: u32 = 5u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRE: u32 = 2u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPOST: u32 = 11u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPRE: u32 = 8u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPOST: u32 = 13u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRE: u32 = 1u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPOST: u32 = 12u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPRE: u32 = 7u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEEPRE: u32 = 3u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPOST: u32 = 4u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPOST: u32 = 10u32;
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPRE: u32 = 9u32;
pub const DOCUMENTEVENT_XPS_CANCELJOB: u32 = 6u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOCUMENTPROPERTYHEADER {
    pub cbSize: u16,
    pub Reserved: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pszPrinterName: *mut i8,
    pub pdmIn: *mut super::Gdi::DEVMODEA,
    pub pdmOut: *mut super::Gdi::DEVMODEA,
    pub cbOut: u32,
    pub fMode: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DOCUMENTPROPERTYHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOC_INFO_1A {
    pub pDocName: windows_core::PSTR,
    pub pOutputFile: windows_core::PSTR,
    pub pDatatype: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOC_INFO_1W {
    pub pDocName: windows_core::PWSTR,
    pub pOutputFile: windows_core::PWSTR,
    pub pDatatype: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOC_INFO_2A {
    pub pDocName: windows_core::PSTR,
    pub pOutputFile: windows_core::PSTR,
    pub pDatatype: windows_core::PSTR,
    pub dwMode: u32,
    pub JobId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOC_INFO_2W {
    pub pDocName: windows_core::PWSTR,
    pub pOutputFile: windows_core::PWSTR,
    pub pDatatype: windows_core::PWSTR,
    pub dwMode: u32,
    pub JobId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOC_INFO_3A {
    pub pDocName: windows_core::PSTR,
    pub pOutputFile: windows_core::PSTR,
    pub pDatatype: windows_core::PSTR,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOC_INFO_3W {
    pub pDocName: windows_core::PWSTR,
    pub pOutputFile: windows_core::PWSTR,
    pub pDatatype: windows_core::PWSTR,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOC_INFO_INTERNAL {
    pub pDocName: *mut i8,
    pub pOutputFile: *mut i8,
    pub pDatatype: *mut i8,
    pub bLowILJob: windows_core::BOOL,
    pub hTokenLowIL: super::super::Foundation::HANDLE,
}
impl Default for DOC_INFO_INTERNAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOC_INFO_INTERNAL_LEVEL: u32 = 100u32;
pub const DPD_DELETE_ALL_FILES: u32 = 4u32;
pub const DPD_DELETE_SPECIFIC_VERSION: u32 = 2u32;
pub const DPD_DELETE_UNUSED_FILES: u32 = 1u32;
pub const DPF_ICONID_AS_HICON: u32 = 1u32;
pub const DPF_USE_HDLGTEMPLATE: u32 = 2u32;
pub const DPS_NOPERMISSION: u32 = 1u32;
pub const DP_STD_DOCPROPPAGE1: u32 = 65533u32;
pub const DP_STD_DOCPROPPAGE2: u32 = 65534u32;
pub const DP_STD_RESERVED_START: u32 = 65520u32;
pub const DP_STD_TREEVIEWPAGE: u32 = 65535u32;
pub const DRIVER_EVENT_DELETE: u32 = 2u32;
pub const DRIVER_EVENT_INITIALIZE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_1A {
    pub pName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_1W {
    pub pName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_2A {
    pub cVersion: u32,
    pub pName: windows_core::PSTR,
    pub pEnvironment: windows_core::PSTR,
    pub pDriverPath: windows_core::PSTR,
    pub pDataFile: windows_core::PSTR,
    pub pConfigFile: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_2W {
    pub cVersion: u32,
    pub pName: windows_core::PWSTR,
    pub pEnvironment: windows_core::PWSTR,
    pub pDriverPath: windows_core::PWSTR,
    pub pDataFile: windows_core::PWSTR,
    pub pConfigFile: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_3A {
    pub cVersion: u32,
    pub pName: windows_core::PSTR,
    pub pEnvironment: windows_core::PSTR,
    pub pDriverPath: windows_core::PSTR,
    pub pDataFile: windows_core::PSTR,
    pub pConfigFile: windows_core::PSTR,
    pub pHelpFile: windows_core::PSTR,
    pub pDependentFiles: windows_core::PSTR,
    pub pMonitorName: windows_core::PSTR,
    pub pDefaultDataType: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_3W {
    pub cVersion: u32,
    pub pName: windows_core::PWSTR,
    pub pEnvironment: windows_core::PWSTR,
    pub pDriverPath: windows_core::PWSTR,
    pub pDataFile: windows_core::PWSTR,
    pub pConfigFile: windows_core::PWSTR,
    pub pHelpFile: windows_core::PWSTR,
    pub pDependentFiles: windows_core::PWSTR,
    pub pMonitorName: windows_core::PWSTR,
    pub pDefaultDataType: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_4A {
    pub cVersion: u32,
    pub pName: windows_core::PSTR,
    pub pEnvironment: windows_core::PSTR,
    pub pDriverPath: windows_core::PSTR,
    pub pDataFile: windows_core::PSTR,
    pub pConfigFile: windows_core::PSTR,
    pub pHelpFile: windows_core::PSTR,
    pub pDependentFiles: windows_core::PSTR,
    pub pMonitorName: windows_core::PSTR,
    pub pDefaultDataType: windows_core::PSTR,
    pub pszzPreviousNames: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_4W {
    pub cVersion: u32,
    pub pName: windows_core::PWSTR,
    pub pEnvironment: windows_core::PWSTR,
    pub pDriverPath: windows_core::PWSTR,
    pub pDataFile: windows_core::PWSTR,
    pub pConfigFile: windows_core::PWSTR,
    pub pHelpFile: windows_core::PWSTR,
    pub pDependentFiles: windows_core::PWSTR,
    pub pMonitorName: windows_core::PWSTR,
    pub pDefaultDataType: windows_core::PWSTR,
    pub pszzPreviousNames: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_5A {
    pub cVersion: u32,
    pub pName: windows_core::PSTR,
    pub pEnvironment: windows_core::PSTR,
    pub pDriverPath: windows_core::PSTR,
    pub pDataFile: windows_core::PSTR,
    pub pConfigFile: windows_core::PSTR,
    pub dwDriverAttributes: u32,
    pub dwConfigVersion: u32,
    pub dwDriverVersion: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_5W {
    pub cVersion: u32,
    pub pName: windows_core::PWSTR,
    pub pEnvironment: windows_core::PWSTR,
    pub pDriverPath: windows_core::PWSTR,
    pub pDataFile: windows_core::PWSTR,
    pub pConfigFile: windows_core::PWSTR,
    pub dwDriverAttributes: u32,
    pub dwConfigVersion: u32,
    pub dwDriverVersion: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_6A {
    pub cVersion: u32,
    pub pName: windows_core::PSTR,
    pub pEnvironment: windows_core::PSTR,
    pub pDriverPath: windows_core::PSTR,
    pub pDataFile: windows_core::PSTR,
    pub pConfigFile: windows_core::PSTR,
    pub pHelpFile: windows_core::PSTR,
    pub pDependentFiles: windows_core::PSTR,
    pub pMonitorName: windows_core::PSTR,
    pub pDefaultDataType: windows_core::PSTR,
    pub pszzPreviousNames: windows_core::PSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: windows_core::PSTR,
    pub pszOEMUrl: windows_core::PSTR,
    pub pszHardwareID: windows_core::PSTR,
    pub pszProvider: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_6W {
    pub cVersion: u32,
    pub pName: windows_core::PWSTR,
    pub pEnvironment: windows_core::PWSTR,
    pub pDriverPath: windows_core::PWSTR,
    pub pDataFile: windows_core::PWSTR,
    pub pConfigFile: windows_core::PWSTR,
    pub pHelpFile: windows_core::PWSTR,
    pub pDependentFiles: windows_core::PWSTR,
    pub pMonitorName: windows_core::PWSTR,
    pub pDefaultDataType: windows_core::PWSTR,
    pub pszzPreviousNames: windows_core::PWSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: windows_core::PWSTR,
    pub pszOEMUrl: windows_core::PWSTR,
    pub pszHardwareID: windows_core::PWSTR,
    pub pszProvider: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_8A {
    pub cVersion: u32,
    pub pName: windows_core::PSTR,
    pub pEnvironment: windows_core::PSTR,
    pub pDriverPath: windows_core::PSTR,
    pub pDataFile: windows_core::PSTR,
    pub pConfigFile: windows_core::PSTR,
    pub pHelpFile: windows_core::PSTR,
    pub pDependentFiles: windows_core::PSTR,
    pub pMonitorName: windows_core::PSTR,
    pub pDefaultDataType: windows_core::PSTR,
    pub pszzPreviousNames: windows_core::PSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: windows_core::PSTR,
    pub pszOEMUrl: windows_core::PSTR,
    pub pszHardwareID: windows_core::PSTR,
    pub pszProvider: windows_core::PSTR,
    pub pszPrintProcessor: windows_core::PSTR,
    pub pszVendorSetup: windows_core::PSTR,
    pub pszzColorProfiles: windows_core::PSTR,
    pub pszInfPath: windows_core::PSTR,
    pub dwPrinterDriverAttributes: u32,
    pub pszzCoreDriverDependencies: windows_core::PSTR,
    pub ftMinInboxDriverVerDate: super::super::Foundation::FILETIME,
    pub dwlMinInboxDriverVerVersion: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_8W {
    pub cVersion: u32,
    pub pName: windows_core::PWSTR,
    pub pEnvironment: windows_core::PWSTR,
    pub pDriverPath: windows_core::PWSTR,
    pub pDataFile: windows_core::PWSTR,
    pub pConfigFile: windows_core::PWSTR,
    pub pHelpFile: windows_core::PWSTR,
    pub pDependentFiles: windows_core::PWSTR,
    pub pMonitorName: windows_core::PWSTR,
    pub pDefaultDataType: windows_core::PWSTR,
    pub pszzPreviousNames: windows_core::PWSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: windows_core::PWSTR,
    pub pszOEMUrl: windows_core::PWSTR,
    pub pszHardwareID: windows_core::PWSTR,
    pub pszProvider: windows_core::PWSTR,
    pub pszPrintProcessor: windows_core::PWSTR,
    pub pszVendorSetup: windows_core::PWSTR,
    pub pszzColorProfiles: windows_core::PWSTR,
    pub pszInfPath: windows_core::PWSTR,
    pub dwPrinterDriverAttributes: u32,
    pub pszzCoreDriverDependencies: windows_core::PWSTR,
    pub ftMinInboxDriverVerDate: super::super::Foundation::FILETIME,
    pub dwlMinInboxDriverVerVersion: u64,
}
pub const DRIVER_KERNELMODE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVER_UPGRADE_INFO_1 {
    pub pPrinterName: *mut i8,
    pub pOldDriverDirectory: *mut i8,
}
impl Default for DRIVER_UPGRADE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVER_UPGRADE_INFO_2 {
    pub pPrinterName: *mut i8,
    pub pOldDriverDirectory: *mut i8,
    pub cVersion: u32,
    pub pName: *mut i8,
    pub pEnvironment: *mut i8,
    pub pDriverPath: *mut i8,
    pub pDataFile: *mut i8,
    pub pConfigFile: *mut i8,
    pub pHelpFile: *mut i8,
    pub pDependentFiles: *mut i8,
    pub pMonitorName: *mut i8,
    pub pDefaultDataType: *mut i8,
    pub pszzPreviousNames: *mut i8,
}
impl Default for DRIVER_UPGRADE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRIVER_USERMODE: u32 = 2u32;
pub const DSPRINT_PENDING: u32 = 2147483648u32;
pub const DSPRINT_PUBLISH: u32 = 1u32;
pub const DSPRINT_REPUBLISH: u32 = 8u32;
pub const DSPRINT_UNPUBLISH: u32 = 4u32;
pub const DSPRINT_UPDATE: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EATTRIBUTE_DATATYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EBranchOfficeJobEventType(pub i32);
pub const ECBF_CHECKNAME_AT_FRONT: u32 = 1u32;
pub const ECBF_CHECKNAME_ONLY: u32 = 128u32;
pub const ECBF_CHECKNAME_ONLY_ENABLED: u32 = 2u32;
pub const ECBF_ICONID_AS_HICON: u32 = 4u32;
pub const ECBF_MASK: u32 = 255u32;
pub const ECBF_OVERLAY_ECBICON_IF_CHECKED: u32 = 16u32;
pub const ECBF_OVERLAY_NO_ICON: u32 = 64u32;
pub const ECBF_OVERLAY_STOP_ICON: u32 = 32u32;
pub const ECBF_OVERLAY_WARNING_ICON: u32 = 8u32;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type EMFPLAYPROC = Option<unsafe extern "system" fn(param0: super::Gdi::HDC, param1: i32, param2: super::super::Foundation::HANDLE) -> i32>;
pub const EMF_PP_COLOR_OPTIMIZATION: u32 = 1u32;
pub const EPF_ICONID_AS_HICON: u32 = 8u32;
pub const EPF_INCL_SETUP_TITLE: u32 = 2u32;
pub const EPF_MASK: u32 = 255u32;
pub const EPF_NO_DOT_DOT_DOT: u32 = 4u32;
pub const EPF_OVERLAY_NO_ICON: u32 = 64u32;
pub const EPF_OVERLAY_STOP_ICON: u32 = 32u32;
pub const EPF_OVERLAY_WARNING_ICON: u32 = 16u32;
pub const EPF_PUSH_TYPE_DLGPROC: u32 = 1u32;
pub const EPF_USE_HDLGTEMPLATE: u32 = 128u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EPrintPropertyType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EPrintXPSJobOperation(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EPrintXPSJobProgress(pub i32);
pub const ERROR_BIDI_DEVICE_CONFIG_UNCHANGED: u32 = 13014u32;
pub const ERROR_BIDI_DEVICE_OFFLINE: u32 = 13004u32;
pub const ERROR_BIDI_ERROR_BASE: u32 = 13000u32;
pub const ERROR_BIDI_GET_ARGUMENT_NOT_SUPPORTED: u32 = 13012u32;
pub const ERROR_BIDI_GET_MISSING_ARGUMENT: u32 = 13013u32;
pub const ERROR_BIDI_GET_REQUIRES_ARGUMENT: u32 = 13011u32;
pub const ERROR_BIDI_NO_BIDI_SCHEMA_EXTENSIONS: u32 = 13016u32;
pub const ERROR_BIDI_NO_LOCALIZED_RESOURCES: u32 = 13015u32;
pub const ERROR_BIDI_SCHEMA_NOT_SUPPORTED: u32 = 13005u32;
pub const ERROR_BIDI_SCHEMA_READ_ONLY: u32 = 13002u32;
pub const ERROR_BIDI_SCHEMA_WRITE_ONLY: u32 = 13010u32;
pub const ERROR_BIDI_SERVER_OFFLINE: u32 = 13003u32;
pub const ERROR_BIDI_SET_DIFFERENT_TYPE: u32 = 13006u32;
pub const ERROR_BIDI_SET_INVALID_SCHEMAPATH: u32 = 13008u32;
pub const ERROR_BIDI_SET_MULTIPLE_SCHEMAPATH: u32 = 13007u32;
pub const ERROR_BIDI_SET_UNKNOWN_FAILURE: u32 = 13009u32;
pub const ERROR_BIDI_STATUS_OK: u32 = 0u32;
pub const ERROR_BIDI_STATUS_WARNING: u32 = 13001u32;
pub const ERROR_BIDI_UNSUPPORTED_CLIENT_LANGUAGE: u32 = 13017u32;
pub const ERROR_BIDI_UNSUPPORTED_RESOURCE_FORMAT: u32 = 13018u32;
pub const ERR_CPSUI_ALLOCMEM_FAILED: i32 = -2i32;
pub const ERR_CPSUI_CREATEPROPPAGE_FAILED: i32 = -10i32;
pub const ERR_CPSUI_CREATE_IMAGELIST_FAILED: i32 = -33i32;
pub const ERR_CPSUI_CREATE_TRACKBAR_FAILED: i32 = -31i32;
pub const ERR_CPSUI_CREATE_UDARROW_FAILED: i32 = -32i32;
pub const ERR_CPSUI_DMCOPIES_USE_EXTPUSH: i32 = -43i32;
pub const ERR_CPSUI_FUNCTION_NOT_IMPLEMENTED: i32 = -9999i32;
pub const ERR_CPSUI_GETLASTERROR: i32 = -1i32;
pub const ERR_CPSUI_INTERNAL_ERROR: i32 = -10000i32;
pub const ERR_CPSUI_INVALID_DLGPAGEIDX: i32 = -16i32;
pub const ERR_CPSUI_INVALID_DLGPAGE_CBSIZE: i32 = -14i32;
pub const ERR_CPSUI_INVALID_DMPUBID: i32 = -29i32;
pub const ERR_CPSUI_INVALID_DMPUB_TVOT: i32 = -30i32;
pub const ERR_CPSUI_INVALID_ECB_CBSIZE: i32 = -26i32;
pub const ERR_CPSUI_INVALID_EDITBOX_BUF_SIZE: i32 = -25i32;
pub const ERR_CPSUI_INVALID_EDITBOX_PSEL: i32 = -24i32;
pub const ERR_CPSUI_INVALID_EXTPUSH_CBSIZE: i32 = -39i32;
pub const ERR_CPSUI_INVALID_LBCB_TYPE: i32 = -35i32;
pub const ERR_CPSUI_INVALID_LPARAM: i32 = -4i32;
pub const ERR_CPSUI_INVALID_OPTITEM_CBSIZE: i32 = -19i32;
pub const ERR_CPSUI_INVALID_OPTPARAM_CBSIZE: i32 = -23i32;
pub const ERR_CPSUI_INVALID_OPTTYPE_CBSIZE: i32 = -20i32;
pub const ERR_CPSUI_INVALID_OPTTYPE_COUNT: i32 = -21i32;
pub const ERR_CPSUI_INVALID_PDATA: i32 = -3i32;
pub const ERR_CPSUI_INVALID_PDLGPAGE: i32 = -13i32;
pub const ERR_CPSUI_INVALID_PUSHBUTTON_TYPE: i32 = -38i32;
pub const ERR_CPSUI_INVALID_TVOT_TYPE: i32 = -34i32;
pub const ERR_CPSUI_MORE_THAN_ONE_STDPAGE: i32 = -12i32;
pub const ERR_CPSUI_MORE_THAN_ONE_TVPAGE: i32 = -11i32;
pub const ERR_CPSUI_NO_EXTPUSH_DLGTEMPLATEID: i32 = -41i32;
pub const ERR_CPSUI_NO_PROPSHEETPAGE: i32 = -8i32;
pub const ERR_CPSUI_NULL_CALLERNAME: i32 = -6i32;
pub const ERR_CPSUI_NULL_ECB_PCHECKEDNAME: i32 = -28i32;
pub const ERR_CPSUI_NULL_ECB_PTITLE: i32 = -27i32;
pub const ERR_CPSUI_NULL_EXTPUSH_CALLBACK: i32 = -42i32;
pub const ERR_CPSUI_NULL_EXTPUSH_DLGPROC: i32 = -40i32;
pub const ERR_CPSUI_NULL_HINST: i32 = -5i32;
pub const ERR_CPSUI_NULL_OPTITEMNAME: i32 = -7i32;
pub const ERR_CPSUI_NULL_POPTITEM: i32 = -18i32;
pub const ERR_CPSUI_NULL_POPTPARAM: i32 = -22i32;
pub const ERR_CPSUI_SUBITEM_DIFF_DLGPAGEIDX: i32 = -17i32;
pub const ERR_CPSUI_SUBITEM_DIFF_OPTIF_HIDE: i32 = -36i32;
pub const ERR_CPSUI_TOO_MANY_DLGPAGES: i32 = -15i32;
pub const ERR_CPSUI_TOO_MANY_PROPSHEETPAGES: i32 = -9i32;
pub const ERR_CPSUI_ZERO_OPTITEM: i32 = -44i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTCHKBOX {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub pSeparator: *mut i8,
    pub pCheckedName: *mut i8,
    pub IconID: usize,
    pub wReserved: [u16; 4],
    pub dwReserved: [usize; 2],
}
impl Default for EXTCHKBOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct EXTPUSH {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub Anonymous1: EXTPUSH_0,
    pub IconID: usize,
    pub Anonymous2: EXTPUSH_1,
    pub dwReserved: [usize; 3],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for EXTPUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union EXTPUSH_0 {
    pub DlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub pfnCallBack: super::super::Foundation::FARPROC,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for EXTPUSH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union EXTPUSH_1 {
    pub DlgTemplateID: u16,
    pub hDlgTemplate: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for EXTPUSH_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXTTEXTMETRIC {
    pub emSize: i16,
    pub emPointSize: i16,
    pub emOrientation: i16,
    pub emMasterHeight: i16,
    pub emMinScale: i16,
    pub emMaxScale: i16,
    pub emMasterUnits: i16,
    pub emCapHeight: i16,
    pub emXHeight: i16,
    pub emLowerCaseAscent: i16,
    pub emLowerCaseDescent: i16,
    pub emSlant: i16,
    pub emSuperScript: i16,
    pub emSubScript: i16,
    pub emSuperScriptSize: i16,
    pub emSubScriptSize: i16,
    pub emUnderlineOffset: i16,
    pub emUnderlineWidth: i16,
    pub emDoubleUpperUnderlineOffset: i16,
    pub emDoubleLowerUnderlineOffset: i16,
    pub emDoubleUpperUnderlineWidth: i16,
    pub emDoubleLowerUnderlineWidth: i16,
    pub emStrikeOutOffset: i16,
    pub emStrikeOutWidth: i16,
    pub emKernPairs: u16,
    pub emKernTracks: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXpsCompressionOptions(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXpsFontOptions(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXpsFontRestriction(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXpsJobConsumption(pub i32);
pub const E_VERSION_NOT_SUPPORTED: u32 = 2147745793u32;
pub const FG_CANCHANGE: u32 = 128u32;
pub const FILL_WITH_DEFAULTS: u32 = 1u32;
pub const FMTID_PrinterPropertyBag: windows_core::GUID = windows_core::GUID::from_u128(0x75f9adca_097d_45c3_a6e4_bab29e276f3e);
pub const FNT_INFO_CURRENTFONTID: u32 = 10u32;
pub const FNT_INFO_FONTBOLD: u32 = 6u32;
pub const FNT_INFO_FONTHEIGHT: u32 = 4u32;
pub const FNT_INFO_FONTITALIC: u32 = 7u32;
pub const FNT_INFO_FONTMAXWIDTH: u32 = 13u32;
pub const FNT_INFO_FONTSTRIKETHRU: u32 = 9u32;
pub const FNT_INFO_FONTUNDERLINE: u32 = 8u32;
pub const FNT_INFO_FONTWIDTH: u32 = 5u32;
pub const FNT_INFO_GRAYPERCENTAGE: u32 = 1u32;
pub const FNT_INFO_MAX: u32 = 14u32;
pub const FNT_INFO_NEXTFONTID: u32 = 2u32;
pub const FNT_INFO_NEXTGLYPH: u32 = 3u32;
pub const FNT_INFO_PRINTDIRINCCDEGREES: u32 = 0u32;
pub const FNT_INFO_TEXTXRES: u32 = 12u32;
pub const FNT_INFO_TEXTYRES: u32 = 11u32;
pub const FONT_DIR_SORTED: u32 = 1u32;
pub const FONT_FL_DEVICEFONT: u32 = 16u32;
pub const FONT_FL_GLYPHSET_GTT: u32 = 32u32;
pub const FONT_FL_GLYPHSET_RLE: u32 = 64u32;
pub const FONT_FL_IFI: u32 = 2u32;
pub const FONT_FL_PERMANENT_SF: u32 = 8u32;
pub const FONT_FL_RESERVED: u32 = 32768u32;
pub const FONT_FL_SOFTFONT: u32 = 4u32;
pub const FONT_FL_UFM: u32 = 1u32;
pub const FORM_BUILTIN: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FORM_INFO_1A {
    pub Flags: u32,
    pub pName: windows_core::PSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FORM_INFO_1W {
    pub Flags: u32,
    pub pName: windows_core::PWSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FORM_INFO_2A {
    pub Flags: u32,
    pub pName: windows_core::PCSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
    pub pKeyword: windows_core::PCSTR,
    pub StringType: u32,
    pub pMuiDll: windows_core::PCSTR,
    pub dwResourceId: u32,
    pub pDisplayName: windows_core::PCSTR,
    pub wLangId: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FORM_INFO_2W {
    pub Flags: u32,
    pub pName: windows_core::PCWSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
    pub pKeyword: windows_core::PCSTR,
    pub StringType: u32,
    pub pMuiDll: windows_core::PCWSTR,
    pub dwResourceId: u32,
    pub pDisplayName: windows_core::PCWSTR,
    pub wLangId: u16,
}
pub const FORM_PRINTER: u32 = 2u32;
pub const FORM_USER: u32 = 0u32;
pub const FinalPageCount: PageCountType = PageCountType(0i32);
pub const Font_Normal: EXpsFontOptions = EXpsFontOptions(0i32);
pub const Font_Obfusticate: EXpsFontOptions = EXpsFontOptions(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLYPHRUN {
    pub wcLow: u16,
    pub wGlyphCount: u16,
}
pub const GPD_OEMCUSTOMDATA: u32 = 1u32;
pub const GUID_DEVINTERFACE_IPPUSB_PRINT: windows_core::GUID = windows_core::GUID::from_u128(0xf2f40381_f46d_4e51_bce7_62de6cf2d098);
pub const GUID_DEVINTERFACE_USBPRINT: windows_core::GUID = windows_core::GUID::from_u128(0x28d78fad_5a12_11d1_ae5b_0000f803a8c2);
windows_core::imp::define_interface!(IAsyncGetSendNotificationCookie, IAsyncGetSendNotificationCookie_Vtbl, 0);
impl core::ops::Deref for IAsyncGetSendNotificationCookie {
    type Target = IPrintAsyncCookie;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAsyncGetSendNotificationCookie, windows_core::IUnknown, IPrintAsyncCookie);
impl IAsyncGetSendNotificationCookie {
    pub unsafe fn FinishAsyncCallWithData<P0>(&self, param0: P0, param1: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintAsyncNotifyDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).FinishAsyncCallWithData)(windows_core::Interface::as_raw(self), param0.param().abi(), param1.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncGetSendNotificationCookie_Vtbl {
    pub base__: IPrintAsyncCookie_Vtbl,
    pub FinishAsyncCallWithData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAsyncGetSendNotificationCookie_Impl: IPrintAsyncCookie_Impl {
    fn FinishAsyncCallWithData(&self, param0: windows_core::Ref<IPrintAsyncNotifyDataObject>, param1: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IAsyncGetSendNotificationCookie_Vtbl {
    pub const fn new<Identity: IAsyncGetSendNotificationCookie_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: IAsyncGetSendNotificationCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncGetSendNotificationCookie_Impl::FinishAsyncCallWithData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self { base__: IPrintAsyncCookie_Vtbl::new::<Identity, OFFSET>(), FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncGetSendNotificationCookie as windows_core::Interface>::IID || iid == &<IPrintAsyncCookie as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAsyncGetSendNotificationCookie {}
windows_core::imp::define_interface!(IAsyncGetSrvReferralCookie, IAsyncGetSrvReferralCookie_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IAsyncGetSrvReferralCookie, windows_core::IUnknown);
impl IAsyncGetSrvReferralCookie {
    pub unsafe fn FinishAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FinishAsyncCall)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn CancelAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelAsyncCall)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn FinishAsyncCallWithData<P0>(&self, param0: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FinishAsyncCallWithData)(windows_core::Interface::as_raw(self), param0.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncGetSrvReferralCookie_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FinishAsyncCall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub CancelAsyncCall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub FinishAsyncCallWithData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IAsyncGetSrvReferralCookie_Impl: windows_core::IUnknownImpl {
    fn FinishAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()>;
    fn CancelAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()>;
    fn FinishAsyncCallWithData(&self, param0: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IAsyncGetSrvReferralCookie_Vtbl {
    pub const fn new<Identity: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FinishAsyncCall<Identity: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncGetSrvReferralCookie_Impl::FinishAsyncCall(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncGetSrvReferralCookie_Impl::CancelAsyncCall(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncGetSrvReferralCookie_Impl::FinishAsyncCallWithData(this, core::mem::transmute(&param0)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FinishAsyncCall: FinishAsyncCall::<Identity, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, OFFSET>,
            FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncGetSrvReferralCookie as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAsyncGetSrvReferralCookie {}
windows_core::imp::define_interface!(IBidiAsyncNotifyChannel, IBidiAsyncNotifyChannel_Vtbl, 0x532818f7_921b_4fb2_bff8_2f4fd52ebebf);
impl core::ops::Deref for IBidiAsyncNotifyChannel {
    type Target = IPrintAsyncNotifyChannel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBidiAsyncNotifyChannel, windows_core::IUnknown, IPrintAsyncNotifyChannel);
impl IBidiAsyncNotifyChannel {
    pub unsafe fn CreateNotificationChannel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateNotificationChannel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetPrintName(&self, param0: *const Option<IPrintAsyncNotifyDataObject>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrintName)(windows_core::Interface::as_raw(self), core::mem::transmute(param0)).ok() }
    }
    pub unsafe fn GetChannelNotificationType(&self, param0: *const Option<IPrintAsyncNotifyDataObject>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetChannelNotificationType)(windows_core::Interface::as_raw(self), core::mem::transmute(param0)).ok() }
    }
    pub unsafe fn AsyncGetNotificationSendResponse<P0, P1>(&self, param0: P0, param1: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintAsyncNotifyDataObject>,
        P1: windows_core::Param<IAsyncGetSendNotificationCookie>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncGetNotificationSendResponse)(windows_core::Interface::as_raw(self), param0.param().abi(), param1.param().abi()).ok() }
    }
    pub unsafe fn AsyncCloseChannel<P0, P1>(&self, param0: P0, param1: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintAsyncNotifyDataObject>,
        P1: windows_core::Param<IPrintAsyncCookie>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncCloseChannel)(windows_core::Interface::as_raw(self), param0.param().abi(), param1.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBidiAsyncNotifyChannel_Vtbl {
    pub base__: IPrintAsyncNotifyChannel_Vtbl,
    pub CreateNotificationChannel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrintName: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChannelNotificationType: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AsyncGetNotificationSendResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AsyncCloseChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBidiAsyncNotifyChannel_Impl: IPrintAsyncNotifyChannel_Impl {
    fn CreateNotificationChannel(&self) -> windows_core::Result<()>;
    fn GetPrintName(&self, param0: *const Option<IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
    fn GetChannelNotificationType(&self, param0: *const Option<IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
    fn AsyncGetNotificationSendResponse(&self, param0: windows_core::Ref<IPrintAsyncNotifyDataObject>, param1: windows_core::Ref<IAsyncGetSendNotificationCookie>) -> windows_core::Result<()>;
    fn AsyncCloseChannel(&self, param0: windows_core::Ref<IPrintAsyncNotifyDataObject>, param1: windows_core::Ref<IPrintAsyncCookie>) -> windows_core::Result<()>;
}
impl IBidiAsyncNotifyChannel_Vtbl {
    pub const fn new<Identity: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateNotificationChannel<Identity: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiAsyncNotifyChannel_Impl::CreateNotificationChannel(this).into()
            }
        }
        unsafe extern "system" fn GetPrintName<Identity: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiAsyncNotifyChannel_Impl::GetPrintName(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetChannelNotificationType<Identity: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiAsyncNotifyChannel_Impl::GetChannelNotificationType(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn AsyncGetNotificationSendResponse<Identity: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiAsyncNotifyChannel_Impl::AsyncGetNotificationSendResponse(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn AsyncCloseChannel<Identity: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiAsyncNotifyChannel_Impl::AsyncCloseChannel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self {
            base__: IPrintAsyncNotifyChannel_Vtbl::new::<Identity, OFFSET>(),
            CreateNotificationChannel: CreateNotificationChannel::<Identity, OFFSET>,
            GetPrintName: GetPrintName::<Identity, OFFSET>,
            GetChannelNotificationType: GetChannelNotificationType::<Identity, OFFSET>,
            AsyncGetNotificationSendResponse: AsyncGetNotificationSendResponse::<Identity, OFFSET>,
            AsyncCloseChannel: AsyncCloseChannel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBidiAsyncNotifyChannel as windows_core::Interface>::IID || iid == &<IPrintAsyncNotifyChannel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBidiAsyncNotifyChannel {}
windows_core::imp::define_interface!(IBidiRequest, IBidiRequest_Vtbl, 0x8f348bd7_4b47_4755_8a9d_0f422df3dc89);
windows_core::imp::interface_hierarchy!(IBidiRequest, windows_core::IUnknown);
impl IBidiRequest {
    pub unsafe fn SetSchema<P0>(&self, pszschema: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSchema)(windows_core::Interface::as_raw(self), pszschema.param().abi()).ok() }
    }
    pub unsafe fn SetInputData(&self, dwtype: u32, pdata: *const u8, usize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInputData)(windows_core::Interface::as_raw(self), dwtype, pdata, usize).ok() }
    }
    pub unsafe fn GetResult(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOutputData(&self, dwindex: u32, ppszschema: *mut windows_core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOutputData)(windows_core::Interface::as_raw(self), dwindex, ppszschema as _, pdwtype as _, ppdata as _, usize as _).ok() }
    }
    pub unsafe fn GetEnumCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBidiRequest_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSchema: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetInputData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32) -> windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetOutputData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR, *mut u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetEnumCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IBidiRequest_Impl: windows_core::IUnknownImpl {
    fn SetSchema(&self, pszschema: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetInputData(&self, dwtype: u32, pdata: *const u8, usize: u32) -> windows_core::Result<()>;
    fn GetResult(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetOutputData(&self, dwindex: u32, ppszschema: *mut windows_core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> windows_core::Result<()>;
    fn GetEnumCount(&self) -> windows_core::Result<u32>;
}
impl IBidiRequest_Vtbl {
    pub const fn new<Identity: IBidiRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSchema<Identity: IBidiRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszschema: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiRequest_Impl::SetSchema(this, core::mem::transmute(&pszschema)).into()
            }
        }
        unsafe extern "system" fn SetInputData<Identity: IBidiRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtype: u32, pdata: *const u8, usize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiRequest_Impl::SetInputData(this, core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&usize)).into()
            }
        }
        unsafe extern "system" fn GetResult<Identity: IBidiRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBidiRequest_Impl::GetResult(this) {
                    Ok(ok__) => {
                        phr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputData<Identity: IBidiRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppszschema: *mut windows_core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiRequest_Impl::GetOutputData(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&ppszschema), core::mem::transmute_copy(&pdwtype), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&usize)).into()
            }
        }
        unsafe extern "system" fn GetEnumCount<Identity: IBidiRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtotal: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBidiRequest_Impl::GetEnumCount(this) {
                    Ok(ok__) => {
                        pdwtotal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSchema: SetSchema::<Identity, OFFSET>,
            SetInputData: SetInputData::<Identity, OFFSET>,
            GetResult: GetResult::<Identity, OFFSET>,
            GetOutputData: GetOutputData::<Identity, OFFSET>,
            GetEnumCount: GetEnumCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBidiRequest as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBidiRequest {}
windows_core::imp::define_interface!(IBidiRequestContainer, IBidiRequestContainer_Vtbl, 0xd752f6c0_94a8_4275_a77d_8f1d1a1121ae);
windows_core::imp::interface_hierarchy!(IBidiRequestContainer, windows_core::IUnknown);
impl IBidiRequestContainer {
    pub unsafe fn AddRequest<P0>(&self, prequest: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBidiRequest>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRequest)(windows_core::Interface::as_raw(self), prequest.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumObject(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRequestCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBidiRequestContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEnumObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEnumObject: usize,
    pub GetRequestCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBidiRequestContainer_Impl: windows_core::IUnknownImpl {
    fn AddRequest(&self, prequest: windows_core::Ref<IBidiRequest>) -> windows_core::Result<()>;
    fn GetEnumObject(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn GetRequestCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IBidiRequestContainer_Vtbl {
    pub const fn new<Identity: IBidiRequestContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddRequest<Identity: IBidiRequestContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiRequestContainer_Impl::AddRequest(this, core::mem::transmute_copy(&prequest)).into()
            }
        }
        unsafe extern "system" fn GetEnumObject<Identity: IBidiRequestContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBidiRequestContainer_Impl::GetEnumObject(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRequestCount<Identity: IBidiRequestContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBidiRequestContainer_Impl::GetRequestCount(this) {
                    Ok(ok__) => {
                        pucount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRequest: AddRequest::<Identity, OFFSET>,
            GetEnumObject: GetEnumObject::<Identity, OFFSET>,
            GetRequestCount: GetRequestCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBidiRequestContainer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBidiRequestContainer {}
windows_core::imp::define_interface!(IBidiSpl, IBidiSpl_Vtbl, 0xd580dc0e_de39_4649_baa8_bf0b85a03a97);
windows_core::imp::interface_hierarchy!(IBidiSpl, windows_core::IUnknown);
impl IBidiSpl {
    pub unsafe fn BindDevice<P0>(&self, pszdevicename: P0, dwaccess: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindDevice)(windows_core::Interface::as_raw(self), pszdevicename.param().abi(), dwaccess).ok() }
    }
    pub unsafe fn UnbindDevice(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnbindDevice)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SendRecv<P0, P1>(&self, pszaction: P0, prequest: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IBidiRequest>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendRecv)(windows_core::Interface::as_raw(self), pszaction.param().abi(), prequest.param().abi()).ok() }
    }
    pub unsafe fn MultiSendRecv<P0, P1>(&self, pszaction: P0, prequestcontainer: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IBidiRequestContainer>,
    {
        unsafe { (windows_core::Interface::vtable(self).MultiSendRecv)(windows_core::Interface::as_raw(self), pszaction.param().abi(), prequestcontainer.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBidiSpl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BindDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub UnbindDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendRecv: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MultiSendRecv: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBidiSpl_Impl: windows_core::IUnknownImpl {
    fn BindDevice(&self, pszdevicename: &windows_core::PCWSTR, dwaccess: u32) -> windows_core::Result<()>;
    fn UnbindDevice(&self) -> windows_core::Result<()>;
    fn SendRecv(&self, pszaction: &windows_core::PCWSTR, prequest: windows_core::Ref<IBidiRequest>) -> windows_core::Result<()>;
    fn MultiSendRecv(&self, pszaction: &windows_core::PCWSTR, prequestcontainer: windows_core::Ref<IBidiRequestContainer>) -> windows_core::Result<()>;
}
impl IBidiSpl_Vtbl {
    pub const fn new<Identity: IBidiSpl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BindDevice<Identity: IBidiSpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdevicename: windows_core::PCWSTR, dwaccess: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiSpl_Impl::BindDevice(this, core::mem::transmute(&pszdevicename), core::mem::transmute_copy(&dwaccess)).into()
            }
        }
        unsafe extern "system" fn UnbindDevice<Identity: IBidiSpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiSpl_Impl::UnbindDevice(this).into()
            }
        }
        unsafe extern "system" fn SendRecv<Identity: IBidiSpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaction: windows_core::PCWSTR, prequest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiSpl_Impl::SendRecv(this, core::mem::transmute(&pszaction), core::mem::transmute_copy(&prequest)).into()
            }
        }
        unsafe extern "system" fn MultiSendRecv<Identity: IBidiSpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaction: windows_core::PCWSTR, prequestcontainer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiSpl_Impl::MultiSendRecv(this, core::mem::transmute(&pszaction), core::mem::transmute_copy(&prequestcontainer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindDevice: BindDevice::<Identity, OFFSET>,
            UnbindDevice: UnbindDevice::<Identity, OFFSET>,
            SendRecv: SendRecv::<Identity, OFFSET>,
            MultiSendRecv: MultiSendRecv::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBidiSpl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBidiSpl {}
windows_core::imp::define_interface!(IBidiSpl2, IBidiSpl2_Vtbl, 0x0e8f51b8_8273_4906_8e7b_be453ffd2e2b);
windows_core::imp::interface_hierarchy!(IBidiSpl2, windows_core::IUnknown);
impl IBidiSpl2 {
    pub unsafe fn BindDevice<P0>(&self, pszdevicename: P0, dwaccess: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindDevice)(windows_core::Interface::as_raw(self), pszdevicename.param().abi(), dwaccess).ok() }
    }
    pub unsafe fn UnbindDevice(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnbindDevice)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SendRecvXMLString(&self, bstrrequest: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendRecvXMLString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrequest), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SendRecvXMLStream<P0>(&self, psrequest: P0) -> windows_core::Result<super::super::System::Com::IStream>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendRecvXMLStream)(windows_core::Interface::as_raw(self), psrequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBidiSpl2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BindDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub UnbindDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendRecvXMLString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SendRecvXMLStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SendRecvXMLStream: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBidiSpl2_Impl: windows_core::IUnknownImpl {
    fn BindDevice(&self, pszdevicename: &windows_core::PCWSTR, dwaccess: u32) -> windows_core::Result<()>;
    fn UnbindDevice(&self) -> windows_core::Result<()>;
    fn SendRecvXMLString(&self, bstrrequest: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SendRecvXMLStream(&self, psrequest: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IBidiSpl2_Vtbl {
    pub const fn new<Identity: IBidiSpl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BindDevice<Identity: IBidiSpl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdevicename: windows_core::PCWSTR, dwaccess: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiSpl2_Impl::BindDevice(this, core::mem::transmute(&pszdevicename), core::mem::transmute_copy(&dwaccess)).into()
            }
        }
        unsafe extern "system" fn UnbindDevice<Identity: IBidiSpl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBidiSpl2_Impl::UnbindDevice(this).into()
            }
        }
        unsafe extern "system" fn SendRecvXMLString<Identity: IBidiSpl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrequest: *mut core::ffi::c_void, pbstrresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBidiSpl2_Impl::SendRecvXMLString(this, core::mem::transmute(&bstrrequest)) {
                    Ok(ok__) => {
                        pbstrresponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendRecvXMLStream<Identity: IBidiSpl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrequest: *mut core::ffi::c_void, ppsresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBidiSpl2_Impl::SendRecvXMLStream(this, core::mem::transmute_copy(&psrequest)) {
                    Ok(ok__) => {
                        ppsresponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindDevice: BindDevice::<Identity, OFFSET>,
            UnbindDevice: UnbindDevice::<Identity, OFFSET>,
            SendRecvXMLString: SendRecvXMLString::<Identity, OFFSET>,
            SendRecvXMLStream: SendRecvXMLStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBidiSpl2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBidiSpl2 {}
pub const IDI_CPSUI_ADVANCE: u32 = 64058u32;
pub const IDI_CPSUI_AUTOSEL: u32 = 64025u32;
pub const IDI_CPSUI_COLLATE: u32 = 64030u32;
pub const IDI_CPSUI_COLOR: u32 = 64040u32;
pub const IDI_CPSUI_COPY: u32 = 64046u32;
pub const IDI_CPSUI_DEVICE: u32 = 64060u32;
pub const IDI_CPSUI_DEVICE2: u32 = 64061u32;
pub const IDI_CPSUI_DEVICE_FEATURE: u32 = 64080u32;
pub const IDI_CPSUI_DITHER_COARSE: u32 = 64042u32;
pub const IDI_CPSUI_DITHER_FINE: u32 = 64043u32;
pub const IDI_CPSUI_DITHER_LINEART: u32 = 64044u32;
pub const IDI_CPSUI_DITHER_NONE: u32 = 64041u32;
pub const IDI_CPSUI_DOCUMENT: u32 = 64059u32;
pub const IDI_CPSUI_DUPLEX_HORZ: u32 = 64032u32;
pub const IDI_CPSUI_DUPLEX_HORZ_L: u32 = 64085u32;
pub const IDI_CPSUI_DUPLEX_NONE: u32 = 64031u32;
pub const IDI_CPSUI_DUPLEX_NONE_L: u32 = 64084u32;
pub const IDI_CPSUI_DUPLEX_VERT: u32 = 64033u32;
pub const IDI_CPSUI_DUPLEX_VERT_L: u32 = 64086u32;
pub const IDI_CPSUI_EMPTY: u32 = 64000u32;
pub const IDI_CPSUI_ENVELOPE: u32 = 64010u32;
pub const IDI_CPSUI_ENVELOPE_FEED: u32 = 64097u32;
pub const IDI_CPSUI_ERROR: u32 = 64050u32;
pub const IDI_CPSUI_FALSE: u32 = 64005u32;
pub const IDI_CPSUI_FAX: u32 = 64095u32;
pub const IDI_CPSUI_FONTCART: u32 = 64013u32;
pub const IDI_CPSUI_FONTCARTHDR: u32 = 64012u32;
pub const IDI_CPSUI_FONTCART_SLOT: u32 = 64098u32;
pub const IDI_CPSUI_FONTSUB: u32 = 64081u32;
pub const IDI_CPSUI_FORMTRAYASSIGN: u32 = 64076u32;
pub const IDI_CPSUI_GENERIC_ITEM: u32 = 64073u32;
pub const IDI_CPSUI_GENERIC_OPTION: u32 = 64072u32;
pub const IDI_CPSUI_GRAPHIC: u32 = 64057u32;
pub const IDI_CPSUI_HALFTONE_SETUP: u32 = 64048u32;
pub const IDI_CPSUI_HTCLRADJ: u32 = 64047u32;
pub const IDI_CPSUI_HT_DEVICE: u32 = 64017u32;
pub const IDI_CPSUI_HT_HOST: u32 = 64016u32;
pub const IDI_CPSUI_ICM_INTENT: u32 = 64053u32;
pub const IDI_CPSUI_ICM_METHOD: u32 = 64052u32;
pub const IDI_CPSUI_ICM_OPTION: u32 = 64051u32;
pub const IDI_CPSUI_ICONID_FIRST: u32 = 64000u32;
pub const IDI_CPSUI_ICONID_LAST: u32 = 64111u32;
pub const IDI_CPSUI_INSTALLABLE_OPTION: u32 = 64078u32;
pub const IDI_CPSUI_LANDSCAPE: u32 = 64023u32;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWL: u32 = 64100u32;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWLR: u32 = 64104u32;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWS: u32 = 64101u32;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL: u32 = 64102u32;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL_NB: u32 = 64106u32;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP: u32 = 64103u32;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP_NB: u32 = 64107u32;
pub const IDI_CPSUI_LAYOUT_BMP_PORTRAIT: u32 = 64099u32;
pub const IDI_CPSUI_LAYOUT_BMP_ROT_PORT: u32 = 64105u32;
pub const IDI_CPSUI_LF_PEN_PLOTTER: u32 = 64087u32;
pub const IDI_CPSUI_LF_RASTER_PLOTTER: u32 = 64089u32;
pub const IDI_CPSUI_MANUAL_FEED: u32 = 64094u32;
pub const IDI_CPSUI_MEM: u32 = 64011u32;
pub const IDI_CPSUI_MONO: u32 = 64039u32;
pub const IDI_CPSUI_NO: u32 = 64003u32;
pub const IDI_CPSUI_NOTINSTALLED: u32 = 64069u32;
pub const IDI_CPSUI_NUP_BORDER: u32 = 64111u32;
pub const IDI_CPSUI_OFF: u32 = 64007u32;
pub const IDI_CPSUI_ON: u32 = 64008u32;
pub const IDI_CPSUI_OPTION: u32 = 64066u32;
pub const IDI_CPSUI_OPTION2: u32 = 64067u32;
pub const IDI_CPSUI_OUTBIN: u32 = 64055u32;
pub const IDI_CPSUI_OUTPUT: u32 = 64056u32;
pub const IDI_CPSUI_PAGE_PROTECT: u32 = 64096u32;
pub const IDI_CPSUI_PAPER_OUTPUT: u32 = 64009u32;
pub const IDI_CPSUI_PAPER_TRAY: u32 = 64026u32;
pub const IDI_CPSUI_PAPER_TRAY2: u32 = 64027u32;
pub const IDI_CPSUI_PAPER_TRAY3: u32 = 64028u32;
pub const IDI_CPSUI_PEN_CARROUSEL: u32 = 64092u32;
pub const IDI_CPSUI_PLOTTER_PEN: u32 = 64093u32;
pub const IDI_CPSUI_PORTRAIT: u32 = 64022u32;
pub const IDI_CPSUI_POSTSCRIPT: u32 = 64082u32;
pub const IDI_CPSUI_PRINTER: u32 = 64062u32;
pub const IDI_CPSUI_PRINTER2: u32 = 64063u32;
pub const IDI_CPSUI_PRINTER3: u32 = 64064u32;
pub const IDI_CPSUI_PRINTER4: u32 = 64065u32;
pub const IDI_CPSUI_PRINTER_FEATURE: u32 = 64079u32;
pub const IDI_CPSUI_PRINTER_FOLDER: u32 = 64077u32;
pub const IDI_CPSUI_QUESTION: u32 = 64075u32;
pub const IDI_CPSUI_RES_DRAFT: u32 = 64034u32;
pub const IDI_CPSUI_RES_HIGH: u32 = 64037u32;
pub const IDI_CPSUI_RES_LOW: u32 = 64035u32;
pub const IDI_CPSUI_RES_MEDIUM: u32 = 64036u32;
pub const IDI_CPSUI_RES_PRESENTATION: u32 = 64038u32;
pub const IDI_CPSUI_ROLL_PAPER: u32 = 64091u32;
pub const IDI_CPSUI_ROT_LAND: u32 = 64024u32;
pub const IDI_CPSUI_ROT_PORT: u32 = 64110u32;
pub const IDI_CPSUI_RUN_DIALOG: u32 = 64074u32;
pub const IDI_CPSUI_SCALING: u32 = 64045u32;
pub const IDI_CPSUI_SEL_NONE: u32 = 64001u32;
pub const IDI_CPSUI_SF_PEN_PLOTTER: u32 = 64088u32;
pub const IDI_CPSUI_SF_RASTER_PLOTTER: u32 = 64090u32;
pub const IDI_CPSUI_STAPLER_OFF: u32 = 64015u32;
pub const IDI_CPSUI_STAPLER_ON: u32 = 64014u32;
pub const IDI_CPSUI_STD_FORM: u32 = 64054u32;
pub const IDI_CPSUI_STOP: u32 = 64068u32;
pub const IDI_CPSUI_STOP_WARNING_OVERLAY: u32 = 64071u32;
pub const IDI_CPSUI_TELEPHONE: u32 = 64083u32;
pub const IDI_CPSUI_TRANSPARENT: u32 = 64029u32;
pub const IDI_CPSUI_TRUE: u32 = 64006u32;
pub const IDI_CPSUI_TT_DOWNLOADSOFT: u32 = 64019u32;
pub const IDI_CPSUI_TT_DOWNLOADVECT: u32 = 64020u32;
pub const IDI_CPSUI_TT_PRINTASGRAPHIC: u32 = 64018u32;
pub const IDI_CPSUI_TT_SUBDEV: u32 = 64021u32;
pub const IDI_CPSUI_WARNING: u32 = 64002u32;
pub const IDI_CPSUI_WARNING_OVERLAY: u32 = 64070u32;
pub const IDI_CPSUI_WATERMARK: u32 = 64049u32;
pub const IDI_CPSUI_YES: u32 = 64004u32;
pub const IDS_CPSUI_ABOUT: u32 = 64848u32;
pub const IDS_CPSUI_ADVANCED: u32 = 64722u32;
pub const IDS_CPSUI_ADVANCEDOCUMENT: u32 = 64716u32;
pub const IDS_CPSUI_ALL: u32 = 64841u32;
pub const IDS_CPSUI_AUTOSELECT: u32 = 64718u32;
pub const IDS_CPSUI_BACKTOFRONT: u32 = 64857u32;
pub const IDS_CPSUI_BOND: u32 = 64786u32;
pub const IDS_CPSUI_BOOKLET: u32 = 64873u32;
pub const IDS_CPSUI_BOOKLET_EDGE: u32 = 64888u32;
pub const IDS_CPSUI_BOOKLET_EDGE_LEFT: u32 = 64889u32;
pub const IDS_CPSUI_BOOKLET_EDGE_RIGHT: u32 = 64890u32;
pub const IDS_CPSUI_CASSETTE_TRAY: u32 = 64810u32;
pub const IDS_CPSUI_CHANGE: u32 = 64702u32;
pub const IDS_CPSUI_CHANGED: u32 = 64846u32;
pub const IDS_CPSUI_CHANGES: u32 = 64845u32;
pub const IDS_CPSUI_COARSE: u32 = 64787u32;
pub const IDS_CPSUI_COLLATE: u32 = 64756u32;
pub const IDS_CPSUI_COLLATED: u32 = 64757u32;
pub const IDS_CPSUI_COLON_SEP: u32 = 64707u32;
pub const IDS_CPSUI_COLOR: u32 = 64764u32;
pub const IDS_CPSUI_COLOR_APPERANCE: u32 = 64744u32;
pub const IDS_CPSUI_COPIES: u32 = 64831u32;
pub const IDS_CPSUI_COPY: u32 = 64830u32;
pub const IDS_CPSUI_DEFAULT: u32 = 64732u32;
pub const IDS_CPSUI_DEFAULTDOCUMENT: u32 = 64714u32;
pub const IDS_CPSUI_DEFAULT_TRAY: u32 = 64811u32;
pub const IDS_CPSUI_DEVICE: u32 = 64842u32;
pub const IDS_CPSUI_DEVICEOPTIONS: u32 = 64725u32;
pub const IDS_CPSUI_DEVICE_SETTINGS: u32 = 64852u32;
pub const IDS_CPSUI_DITHERING: u32 = 64752u32;
pub const IDS_CPSUI_DOCUMENT: u32 = 64715u32;
pub const IDS_CPSUI_DOWN_THEN_LEFT: u32 = 64882u32;
pub const IDS_CPSUI_DOWN_THEN_RIGHT: u32 = 64880u32;
pub const IDS_CPSUI_DRAFT: u32 = 64759u32;
pub const IDS_CPSUI_DUPLEX: u32 = 64745u32;
pub const IDS_CPSUI_ENVELOPE_TRAY: u32 = 64804u32;
pub const IDS_CPSUI_ENVMANUAL_TRAY: u32 = 64805u32;
pub const IDS_CPSUI_ERRDIFFUSE: u32 = 64790u32;
pub const IDS_CPSUI_ERROR: u32 = 64733u32;
pub const IDS_CPSUI_EXIST: u32 = 64736u32;
pub const IDS_CPSUI_FALSE: u32 = 64726u32;
pub const IDS_CPSUI_FAST: u32 = 64838u32;
pub const IDS_CPSUI_FAX: u32 = 64835u32;
pub const IDS_CPSUI_FINE: u32 = 64788u32;
pub const IDS_CPSUI_FORMNAME: u32 = 64747u32;
pub const IDS_CPSUI_FORMSOURCE: u32 = 64812u32;
pub const IDS_CPSUI_FORMTRAYASSIGN: u32 = 64798u32;
pub const IDS_CPSUI_FRONTTOBACK: u32 = 64856u32;
pub const IDS_CPSUI_GLOSSY: u32 = 64783u32;
pub const IDS_CPSUI_GRAPHIC: u32 = 64720u32;
pub const IDS_CPSUI_GRAYSCALE: u32 = 64765u32;
pub const IDS_CPSUI_HALFTONE: u32 = 64791u32;
pub const IDS_CPSUI_HALFTONE_SETUP: u32 = 64817u32;
pub const IDS_CPSUI_HIGH: u32 = 64762u32;
pub const IDS_CPSUI_HORIZONTAL: u32 = 64768u32;
pub const IDS_CPSUI_HTCLRADJ: u32 = 64792u32;
pub const IDS_CPSUI_ICM: u32 = 64748u32;
pub const IDS_CPSUI_ICMINTENT: u32 = 64750u32;
pub const IDS_CPSUI_ICMMETHOD: u32 = 64749u32;
pub const IDS_CPSUI_ICM_BLACKWHITE: u32 = 64776u32;
pub const IDS_CPSUI_ICM_COLORMETRIC: u32 = 64781u32;
pub const IDS_CPSUI_ICM_CONTRAST: u32 = 64780u32;
pub const IDS_CPSUI_ICM_NO: u32 = 64777u32;
pub const IDS_CPSUI_ICM_SATURATION: u32 = 64779u32;
pub const IDS_CPSUI_ICM_YES: u32 = 64778u32;
pub const IDS_CPSUI_INSTFONTCART: u32 = 64818u32;
pub const IDS_CPSUI_LANDSCAPE: u32 = 64754u32;
pub const IDS_CPSUI_LARGECAP_TRAY: u32 = 64809u32;
pub const IDS_CPSUI_LARGEFMT_TRAY: u32 = 64808u32;
pub const IDS_CPSUI_LBCB_NOSEL: u32 = 64712u32;
pub const IDS_CPSUI_LEFT_ANGLE: u32 = 64708u32;
pub const IDS_CPSUI_LEFT_SLOT: u32 = 64823u32;
pub const IDS_CPSUI_LEFT_THEN_DOWN: u32 = 64881u32;
pub const IDS_CPSUI_LINEART: u32 = 64789u32;
pub const IDS_CPSUI_LONG_SIDE: u32 = 64770u32;
pub const IDS_CPSUI_LOW: u32 = 64760u32;
pub const IDS_CPSUI_LOWER_TRAY: u32 = 64801u32;
pub const IDS_CPSUI_MAILBOX: u32 = 64829u32;
pub const IDS_CPSUI_MAKE: u32 = 64833u32;
pub const IDS_CPSUI_MANUALFEED: u32 = 64813u32;
pub const IDS_CPSUI_MANUAL_DUPLEX: u32 = 64883u32;
pub const IDS_CPSUI_MANUAL_DUPLEX_OFF: u32 = 64885u32;
pub const IDS_CPSUI_MANUAL_DUPLEX_ON: u32 = 64884u32;
pub const IDS_CPSUI_MANUAL_TRAY: u32 = 64803u32;
pub const IDS_CPSUI_MEDIA: u32 = 64751u32;
pub const IDS_CPSUI_MEDIUM: u32 = 64761u32;
pub const IDS_CPSUI_MIDDLE_TRAY: u32 = 64802u32;
pub const IDS_CPSUI_MONOCHROME: u32 = 64766u32;
pub const IDS_CPSUI_MORE: u32 = 64701u32;
pub const IDS_CPSUI_NO: u32 = 64728u32;
pub const IDS_CPSUI_NONE: u32 = 64734u32;
pub const IDS_CPSUI_NOT: u32 = 64735u32;
pub const IDS_CPSUI_NOTINSTALLED: u32 = 64737u32;
pub const IDS_CPSUI_NO_NAME: u32 = 64850u32;
pub const IDS_CPSUI_NUM_OF_COPIES: u32 = 64740u32;
pub const IDS_CPSUI_NUP: u32 = 64864u32;
pub const IDS_CPSUI_NUP_BORDER: u32 = 64891u32;
pub const IDS_CPSUI_NUP_BORDERED: u32 = 64892u32;
pub const IDS_CPSUI_NUP_DIRECTION: u32 = 64878u32;
pub const IDS_CPSUI_NUP_FOURUP: u32 = 64867u32;
pub const IDS_CPSUI_NUP_NINEUP: u32 = 64869u32;
pub const IDS_CPSUI_NUP_NORMAL: u32 = 64865u32;
pub const IDS_CPSUI_NUP_SIXTEENUP: u32 = 64870u32;
pub const IDS_CPSUI_NUP_SIXUP: u32 = 64868u32;
pub const IDS_CPSUI_NUP_TWOUP: u32 = 64866u32;
pub const IDS_CPSUI_OF: u32 = 64704u32;
pub const IDS_CPSUI_OFF: u32 = 64730u32;
pub const IDS_CPSUI_ON: u32 = 64731u32;
pub const IDS_CPSUI_ONLYONE: u32 = 64800u32;
pub const IDS_CPSUI_OPTION: u32 = 64703u32;
pub const IDS_CPSUI_OPTIONS: u32 = 64721u32;
pub const IDS_CPSUI_ORIENTATION: u32 = 64738u32;
pub const IDS_CPSUI_OUTBINASSIGN: u32 = 64796u32;
pub const IDS_CPSUI_OUTPUTBIN: u32 = 64863u32;
pub const IDS_CPSUI_PAGEORDER: u32 = 64855u32;
pub const IDS_CPSUI_PAGEPROTECT: u32 = 64816u32;
pub const IDS_CPSUI_PAPER_OUTPUT: u32 = 64719u32;
pub const IDS_CPSUI_PERCENT: u32 = 64711u32;
pub const IDS_CPSUI_PLOT: u32 = 64836u32;
pub const IDS_CPSUI_PORTRAIT: u32 = 64753u32;
pub const IDS_CPSUI_POSTER: u32 = 64874u32;
pub const IDS_CPSUI_POSTER_2x2: u32 = 64875u32;
pub const IDS_CPSUI_POSTER_3x3: u32 = 64876u32;
pub const IDS_CPSUI_POSTER_4x4: u32 = 64877u32;
pub const IDS_CPSUI_PRESENTATION: u32 = 64763u32;
pub const IDS_CPSUI_PRINT: u32 = 64834u32;
pub const IDS_CPSUI_PRINTER: u32 = 64717u32;
pub const IDS_CPSUI_PRINTERMEM_KB: u32 = 64814u32;
pub const IDS_CPSUI_PRINTERMEM_MB: u32 = 64815u32;
pub const IDS_CPSUI_PRINTFLDSETTING: u32 = 64758u32;
pub const IDS_CPSUI_PRINTQUALITY: u32 = 64742u32;
pub const IDS_CPSUI_PROPERTIES: u32 = 64713u32;
pub const IDS_CPSUI_QUALITY_BEST: u32 = 64861u32;
pub const IDS_CPSUI_QUALITY_BETTER: u32 = 64860u32;
pub const IDS_CPSUI_QUALITY_CUSTOM: u32 = 64862u32;
pub const IDS_CPSUI_QUALITY_DRAFT: u32 = 64859u32;
pub const IDS_CPSUI_QUALITY_SETTINGS: u32 = 64858u32;
pub const IDS_CPSUI_RANGE_FROM: u32 = 64705u32;
pub const IDS_CPSUI_REGULAR: u32 = 64785u32;
pub const IDS_CPSUI_RESET: u32 = 64840u32;
pub const IDS_CPSUI_RESOLUTION: u32 = 64743u32;
pub const IDS_CPSUI_REVERT: u32 = 64844u32;
pub const IDS_CPSUI_RIGHT_ANGLE: u32 = 64709u32;
pub const IDS_CPSUI_RIGHT_SLOT: u32 = 64824u32;
pub const IDS_CPSUI_RIGHT_THEN_DOWN: u32 = 64879u32;
pub const IDS_CPSUI_ROTATED: u32 = 64839u32;
pub const IDS_CPSUI_ROT_LAND: u32 = 64755u32;
pub const IDS_CPSUI_ROT_PORT: u32 = 64886u32;
pub const IDS_CPSUI_SCALING: u32 = 64739u32;
pub const IDS_CPSUI_SETTING: u32 = 64851u32;
pub const IDS_CPSUI_SETTINGS: u32 = 64843u32;
pub const IDS_CPSUI_SETUP: u32 = 64700u32;
pub const IDS_CPSUI_SHORT_SIDE: u32 = 64771u32;
pub const IDS_CPSUI_SIDE1: u32 = 64871u32;
pub const IDS_CPSUI_SIDE2: u32 = 64872u32;
pub const IDS_CPSUI_SIMPLEX: u32 = 64767u32;
pub const IDS_CPSUI_SLASH_SEP: u32 = 64710u32;
pub const IDS_CPSUI_SLOT1: u32 = 64819u32;
pub const IDS_CPSUI_SLOT2: u32 = 64820u32;
pub const IDS_CPSUI_SLOT3: u32 = 64821u32;
pub const IDS_CPSUI_SLOT4: u32 = 64822u32;
pub const IDS_CPSUI_SLOW: u32 = 64837u32;
pub const IDS_CPSUI_SMALLFMT_TRAY: u32 = 64807u32;
pub const IDS_CPSUI_SOURCE: u32 = 64741u32;
pub const IDS_CPSUI_STACKER: u32 = 64828u32;
pub const IDS_CPSUI_STANDARD: u32 = 64782u32;
pub const IDS_CPSUI_STAPLE: u32 = 64887u32;
pub const IDS_CPSUI_STAPLER: u32 = 64825u32;
pub const IDS_CPSUI_STAPLER_OFF: u32 = 64827u32;
pub const IDS_CPSUI_STAPLER_ON: u32 = 64826u32;
pub const IDS_CPSUI_STDDOCPROPTAB: u32 = 64723u32;
pub const IDS_CPSUI_STDDOCPROPTAB1: u32 = 64853u32;
pub const IDS_CPSUI_STDDOCPROPTAB2: u32 = 64854u32;
pub const IDS_CPSUI_STDDOCPROPTVTAB: u32 = 64724u32;
pub const IDS_CPSUI_STRID_FIRST: u32 = 64700u32;
pub const IDS_CPSUI_STRID_LAST: u32 = 64892u32;
pub const IDS_CPSUI_TO: u32 = 64706u32;
pub const IDS_CPSUI_TOTAL: u32 = 64832u32;
pub const IDS_CPSUI_TRACTOR_TRAY: u32 = 64806u32;
pub const IDS_CPSUI_TRANSPARENCY: u32 = 64784u32;
pub const IDS_CPSUI_TRUE: u32 = 64727u32;
pub const IDS_CPSUI_TTOPTION: u32 = 64746u32;
pub const IDS_CPSUI_TT_DOWNLOADSOFT: u32 = 64773u32;
pub const IDS_CPSUI_TT_DOWNLOADVECT: u32 = 64774u32;
pub const IDS_CPSUI_TT_PRINTASGRAPHIC: u32 = 64772u32;
pub const IDS_CPSUI_TT_SUBDEV: u32 = 64775u32;
pub const IDS_CPSUI_UPPER_TRAY: u32 = 64799u32;
pub const IDS_CPSUI_USE_DEVICE_HT: u32 = 64794u32;
pub const IDS_CPSUI_USE_HOST_HT: u32 = 64793u32;
pub const IDS_CPSUI_USE_PRINTER_HT: u32 = 64795u32;
pub const IDS_CPSUI_VERSION: u32 = 64849u32;
pub const IDS_CPSUI_VERTICAL: u32 = 64769u32;
pub const IDS_CPSUI_WARNING: u32 = 64847u32;
pub const IDS_CPSUI_WATERMARK: u32 = 64797u32;
pub const IDS_CPSUI_YES: u32 = 64729u32;
windows_core::imp::define_interface!(IFixedDocument, IFixedDocument_Vtbl, 0xf222ca9f_9968_4db9_81bd_abaebf15f93f);
windows_core::imp::interface_hierarchy!(IFixedDocument, windows_core::IUnknown);
impl IFixedDocument {
    pub unsafe fn GetUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPrintTicket(&self) -> windows_core::Result<IPartPrintTicket> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrintTicket)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrintTicket<P0>(&self, pprintticket: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPartPrintTicket>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrintTicket)(windows_core::Interface::as_raw(self), pprintticket.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFixedDocument_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFixedDocument_Impl: windows_core::IUnknownImpl {
    fn GetUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPrintTicket(&self) -> windows_core::Result<IPartPrintTicket>;
    fn SetPrintTicket(&self, pprintticket: windows_core::Ref<IPartPrintTicket>) -> windows_core::Result<()>;
}
impl IFixedDocument_Vtbl {
    pub const fn new<Identity: IFixedDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUri<Identity: IFixedDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFixedDocument_Impl::GetUri(this) {
                    Ok(ok__) => {
                        uri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrintTicket<Identity: IFixedDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprintticket: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFixedDocument_Impl::GetPrintTicket(this) {
                    Ok(ok__) => {
                        ppprintticket.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrintTicket<Identity: IFixedDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFixedDocument_Impl::SetPrintTicket(this, core::mem::transmute_copy(&pprintticket)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUri: GetUri::<Identity, OFFSET>,
            GetPrintTicket: GetPrintTicket::<Identity, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFixedDocument as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFixedDocument {}
windows_core::imp::define_interface!(IFixedDocumentSequence, IFixedDocumentSequence_Vtbl, 0x8028d181_2c32_4249_8493_1bfb22045574);
windows_core::imp::interface_hierarchy!(IFixedDocumentSequence, windows_core::IUnknown);
impl IFixedDocumentSequence {
    pub unsafe fn GetUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPrintTicket(&self) -> windows_core::Result<IPartPrintTicket> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrintTicket)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrintTicket<P0>(&self, pprintticket: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPartPrintTicket>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrintTicket)(windows_core::Interface::as_raw(self), pprintticket.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFixedDocumentSequence_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFixedDocumentSequence_Impl: windows_core::IUnknownImpl {
    fn GetUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPrintTicket(&self) -> windows_core::Result<IPartPrintTicket>;
    fn SetPrintTicket(&self, pprintticket: windows_core::Ref<IPartPrintTicket>) -> windows_core::Result<()>;
}
impl IFixedDocumentSequence_Vtbl {
    pub const fn new<Identity: IFixedDocumentSequence_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUri<Identity: IFixedDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFixedDocumentSequence_Impl::GetUri(this) {
                    Ok(ok__) => {
                        uri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrintTicket<Identity: IFixedDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprintticket: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFixedDocumentSequence_Impl::GetPrintTicket(this) {
                    Ok(ok__) => {
                        ppprintticket.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrintTicket<Identity: IFixedDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFixedDocumentSequence_Impl::SetPrintTicket(this, core::mem::transmute_copy(&pprintticket)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUri: GetUri::<Identity, OFFSET>,
            GetPrintTicket: GetPrintTicket::<Identity, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFixedDocumentSequence as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFixedDocumentSequence {}
windows_core::imp::define_interface!(IFixedPage, IFixedPage_Vtbl, 0x3d9f6448_7e95_4cb5_94fb_0180c2883a57);
impl core::ops::Deref for IFixedPage {
    type Target = IPartBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFixedPage, windows_core::IUnknown, IPartBase);
impl IFixedPage {
    pub unsafe fn GetPrintTicket(&self) -> windows_core::Result<IPartPrintTicket> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrintTicket)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPagePart<P0>(&self, uri: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPagePart)(windows_core::Interface::as_raw(self), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWriteStream(&self) -> windows_core::Result<IPrintWriteStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriteStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrintTicket<P0>(&self, ppprintticket: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPartPrintTicket>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrintTicket)(windows_core::Interface::as_raw(self), ppprintticket.param().abi()).ok() }
    }
    pub unsafe fn SetPagePart<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPagePart)(windows_core::Interface::as_raw(self), punk.param().abi()).ok() }
    }
    pub unsafe fn DeleteResource<P0>(&self, uri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteResource)(windows_core::Interface::as_raw(self), uri.param().abi()).ok() }
    }
    pub unsafe fn GetXpsPartIterator(&self) -> windows_core::Result<IXpsPartIterator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXpsPartIterator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFixedPage_Vtbl {
    pub base__: IPartBase_Vtbl,
    pub GetPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPagePart: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWriteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPagePart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteResource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetXpsPartIterator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFixedPage_Impl: IPartBase_Impl {
    fn GetPrintTicket(&self) -> windows_core::Result<IPartPrintTicket>;
    fn GetPagePart(&self, uri: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn GetWriteStream(&self) -> windows_core::Result<IPrintWriteStream>;
    fn SetPrintTicket(&self, ppprintticket: windows_core::Ref<IPartPrintTicket>) -> windows_core::Result<()>;
    fn SetPagePart(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DeleteResource(&self, uri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetXpsPartIterator(&self) -> windows_core::Result<IXpsPartIterator>;
}
impl IFixedPage_Vtbl {
    pub const fn new<Identity: IFixedPage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPrintTicket<Identity: IFixedPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprintticket: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFixedPage_Impl::GetPrintTicket(this) {
                    Ok(ok__) => {
                        ppprintticket.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPagePart<Identity: IFixedPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFixedPage_Impl::GetPagePart(this, core::mem::transmute(&uri)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: IFixedPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwritestream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFixedPage_Impl::GetWriteStream(this) {
                    Ok(ok__) => {
                        ppwritestream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrintTicket<Identity: IFixedPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFixedPage_Impl::SetPrintTicket(this, core::mem::transmute_copy(&ppprintticket)).into()
            }
        }
        unsafe extern "system" fn SetPagePart<Identity: IFixedPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFixedPage_Impl::SetPagePart(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn DeleteResource<Identity: IFixedPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFixedPage_Impl::DeleteResource(this, core::mem::transmute(&uri)).into()
            }
        }
        unsafe extern "system" fn GetXpsPartIterator<Identity: IFixedPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxpspartit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFixedPage_Impl::GetXpsPartIterator(this) {
                    Ok(ok__) => {
                        pxpspartit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, OFFSET>(),
            GetPrintTicket: GetPrintTicket::<Identity, OFFSET>,
            GetPagePart: GetPagePart::<Identity, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, OFFSET>,
            SetPagePart: SetPagePart::<Identity, OFFSET>,
            DeleteResource: DeleteResource::<Identity, OFFSET>,
            GetXpsPartIterator: GetXpsPartIterator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFixedPage as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFixedPage {}
#[cfg(feature = "Win32_System_Ole")]
windows_core::imp::define_interface!(IImgCreateErrorInfo, IImgCreateErrorInfo_Vtbl, 0x1c55a64c_07cd_4fb5_90f7_b753d91f0c9e);
#[cfg(feature = "Win32_System_Ole")]
impl core::ops::Deref for IImgCreateErrorInfo {
    type Target = super::super::System::Ole::ICreateErrorInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Ole")]
windows_core::imp::interface_hierarchy!(IImgCreateErrorInfo, windows_core::IUnknown, super::super::System::Ole::ICreateErrorInfo);
#[cfg(feature = "Win32_System_Ole")]
impl IImgCreateErrorInfo {
    pub unsafe fn AttachToErrorInfo(&self, perrorinfo: *mut ImgErrorInfo) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AttachToErrorInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(perrorinfo)).ok() }
    }
}
#[cfg(feature = "Win32_System_Ole")]
#[repr(C)]
#[doc(hidden)]
pub struct IImgCreateErrorInfo_Vtbl {
    pub base__: super::super::System::Ole::ICreateErrorInfo_Vtbl,
    pub AttachToErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImgErrorInfo) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IImgCreateErrorInfo_Impl: super::super::System::Ole::ICreateErrorInfo_Impl {
    fn AttachToErrorInfo(&self, perrorinfo: *mut ImgErrorInfo) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl IImgCreateErrorInfo_Vtbl {
    pub const fn new<Identity: IImgCreateErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachToErrorInfo<Identity: IImgCreateErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorinfo: *mut ImgErrorInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImgCreateErrorInfo_Impl::AttachToErrorInfo(this, core::mem::transmute_copy(&perrorinfo)).into()
            }
        }
        Self { base__: super::super::System::Ole::ICreateErrorInfo_Vtbl::new::<Identity, OFFSET>(), AttachToErrorInfo: AttachToErrorInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImgCreateErrorInfo as windows_core::Interface>::IID || iid == &<super::super::System::Ole::ICreateErrorInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for IImgCreateErrorInfo {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IImgErrorInfo, IImgErrorInfo_Vtbl, 0x2bce4ece_d30e_445a_9423_6829be945ad8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IImgErrorInfo {
    type Target = super::super::System::Com::IErrorInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IImgErrorInfo, windows_core::IUnknown, super::super::System::Com::IErrorInfo);
#[cfg(feature = "Win32_System_Com")]
impl IImgErrorInfo {
    pub unsafe fn GetDeveloperDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeveloperDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetUserErrorId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserErrorId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUserParameterCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserParameterCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUserParameter(&self, cparam: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserParameter)(windows_core::Interface::as_raw(self), cparam, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetUserFallback(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserFallback)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetExceptionId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExceptionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DetachErrorInfo(&self, perrorinfo: *mut ImgErrorInfo) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DetachErrorInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(perrorinfo)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IImgErrorInfo_Vtbl {
    pub base__: super::super::System::Com::IErrorInfo_Vtbl,
    pub GetDeveloperDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserErrorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetUserParameterCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetUserParameter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetExceptionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DetachErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImgErrorInfo) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IImgErrorInfo_Impl: super::super::System::Com::IErrorInfo_Impl {
    fn GetDeveloperDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUserErrorId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetUserParameterCount(&self) -> windows_core::Result<u32>;
    fn GetUserParameter(&self, cparam: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetUserFallback(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetExceptionId(&self) -> windows_core::Result<u32>;
    fn DetachErrorInfo(&self, perrorinfo: *mut ImgErrorInfo) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IImgErrorInfo_Vtbl {
    pub const fn new<Identity: IImgErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeveloperDescription<Identity: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImgErrorInfo_Impl::GetDeveloperDescription(this) {
                    Ok(ok__) => {
                        pbstrdevdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserErrorId<Identity: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImgErrorInfo_Impl::GetUserErrorId(this) {
                    Ok(ok__) => {
                        perrorid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserParameterCount<Identity: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcuserparams: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImgErrorInfo_Impl::GetUserParameterCount(this) {
                    Ok(ok__) => {
                        pcuserparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserParameter<Identity: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cparam: u32, pbstrparam: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImgErrorInfo_Impl::GetUserParameter(this, core::mem::transmute_copy(&cparam)) {
                    Ok(ok__) => {
                        pbstrparam.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserFallback<Identity: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImgErrorInfo_Impl::GetUserFallback(this) {
                    Ok(ok__) => {
                        pbstrfallback.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExceptionId<Identity: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexceptionid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImgErrorInfo_Impl::GetExceptionId(this) {
                    Ok(ok__) => {
                        pexceptionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DetachErrorInfo<Identity: IImgErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorinfo: *mut ImgErrorInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImgErrorInfo_Impl::DetachErrorInfo(this, core::mem::transmute_copy(&perrorinfo)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IErrorInfo_Vtbl::new::<Identity, OFFSET>(),
            GetDeveloperDescription: GetDeveloperDescription::<Identity, OFFSET>,
            GetUserErrorId: GetUserErrorId::<Identity, OFFSET>,
            GetUserParameterCount: GetUserParameterCount::<Identity, OFFSET>,
            GetUserParameter: GetUserParameter::<Identity, OFFSET>,
            GetUserFallback: GetUserFallback::<Identity, OFFSET>,
            GetExceptionId: GetExceptionId::<Identity, OFFSET>,
            DetachErrorInfo: DetachErrorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImgErrorInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IErrorInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IImgErrorInfo {}
windows_core::imp::define_interface!(IInterFilterCommunicator, IInterFilterCommunicator_Vtbl, 0x4daf1e69_81fd_462d_940f_8cd3ddf56fca);
windows_core::imp::interface_hierarchy!(IInterFilterCommunicator, windows_core::IUnknown);
impl IInterFilterCommunicator {
    pub unsafe fn RequestReader(&self, ppireader: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestReader)(windows_core::Interface::as_raw(self), ppireader as _).ok() }
    }
    pub unsafe fn RequestWriter(&self, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestWriter)(windows_core::Interface::as_raw(self), ppiwriter as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInterFilterCommunicator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInterFilterCommunicator_Impl: windows_core::IUnknownImpl {
    fn RequestReader(&self, ppireader: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestWriter(&self, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IInterFilterCommunicator_Vtbl {
    pub const fn new<Identity: IInterFilterCommunicator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestReader<Identity: IInterFilterCommunicator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppireader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInterFilterCommunicator_Impl::RequestReader(this, core::mem::transmute_copy(&ppireader)).into()
            }
        }
        unsafe extern "system" fn RequestWriter<Identity: IInterFilterCommunicator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInterFilterCommunicator_Impl::RequestWriter(this, core::mem::transmute_copy(&ppiwriter)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestReader: RequestReader::<Identity, OFFSET>,
            RequestWriter: RequestWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInterFilterCommunicator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInterFilterCommunicator {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INSERTPSUIPAGE_INFO {
    pub cbSize: u16,
    pub Type: u8,
    pub Mode: u8,
    pub dwData1: usize,
    pub dwData2: usize,
    pub dwData3: usize,
}
pub const INSPSUIPAGE_MODE_AFTER: u32 = 1u32;
pub const INSPSUIPAGE_MODE_BEFORE: u32 = 0u32;
pub const INSPSUIPAGE_MODE_FIRST_CHILD: u32 = 2u32;
pub const INSPSUIPAGE_MODE_INDEX: u32 = 4u32;
pub const INSPSUIPAGE_MODE_LAST_CHILD: u32 = 3u32;
pub const INTERNAL_NOTIFICATION_QUEUE_IS_FULL: PrintAsyncNotifyError = PrintAsyncNotifyError(19i32);
pub const INVALID_NOTIFICATION_TYPE: PrintAsyncNotifyError = PrintAsyncNotifyError(20i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INVOC {
    pub dwCount: u32,
    pub loOffset: u32,
}
pub const IOCTL_USBPRINT_ADD_CHILD_DEVICE: u32 = 2228316u32;
pub const IOCTL_USBPRINT_ADD_MSIPP_COMPAT_ID: u32 = 2228308u32;
pub const IOCTL_USBPRINT_CYCLE_PORT: u32 = 2228320u32;
pub const IOCTL_USBPRINT_GET_1284_ID: u32 = 2228276u32;
pub const IOCTL_USBPRINT_GET_INTERFACE_TYPE: u32 = 2228300u32;
pub const IOCTL_USBPRINT_GET_LPT_STATUS: u32 = 2228272u32;
pub const IOCTL_USBPRINT_GET_PROTOCOL: u32 = 2228292u32;
pub const IOCTL_USBPRINT_SET_DEVICE_ID: u32 = 2228312u32;
pub const IOCTL_USBPRINT_SET_PORT_NUMBER: u32 = 2228304u32;
pub const IOCTL_USBPRINT_SET_PROTOCOL: u32 = 2228296u32;
pub const IOCTL_USBPRINT_SOFT_RESET: u32 = 2228288u32;
pub const IOCTL_USBPRINT_VENDOR_GET_COMMAND: u32 = 2228284u32;
pub const IOCTL_USBPRINT_VENDOR_SET_COMMAND: u32 = 2228280u32;
pub const IPDFP_COPY_ALL_FILES: u32 = 1u32;
windows_core::imp::define_interface!(IPartBase, IPartBase_Vtbl, 0x36d51e28_369e_43ba_a666_9540c62c3f58);
windows_core::imp::interface_hierarchy!(IPartBase, windows_core::IUnknown);
impl IPartBase {
    pub unsafe fn GetUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetStream(&self) -> windows_core::Result<IPrintReadStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPartCompression(&self) -> windows_core::Result<EXpsCompressionOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartCompression)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPartCompression)(windows_core::Interface::as_raw(self), compression).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPartCompression: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EXpsCompressionOptions) -> windows_core::HRESULT,
    pub SetPartCompression: unsafe extern "system" fn(*mut core::ffi::c_void, EXpsCompressionOptions) -> windows_core::HRESULT,
}
pub trait IPartBase_Impl: windows_core::IUnknownImpl {
    fn GetUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetStream(&self) -> windows_core::Result<IPrintReadStream>;
    fn GetPartCompression(&self) -> windows_core::Result<EXpsCompressionOptions>;
    fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> windows_core::Result<()>;
}
impl IPartBase_Vtbl {
    pub const fn new<Identity: IPartBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUri<Identity: IPartBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPartBase_Impl::GetUri(this) {
                    Ok(ok__) => {
                        uri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IPartBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPartBase_Impl::GetStream(this) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartCompression<Identity: IPartBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompression: *mut EXpsCompressionOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPartBase_Impl::GetPartCompression(this) {
                    Ok(ok__) => {
                        pcompression.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPartCompression<Identity: IPartBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compression: EXpsCompressionOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPartBase_Impl::SetPartCompression(this, core::mem::transmute_copy(&compression)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUri: GetUri::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            GetPartCompression: GetPartCompression::<Identity, OFFSET>,
            SetPartCompression: SetPartCompression::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartBase {}
windows_core::imp::define_interface!(IPartColorProfile, IPartColorProfile_Vtbl, 0x63cca95b_7d18_4762_b15e_98658693d24a);
impl core::ops::Deref for IPartColorProfile {
    type Target = IPartBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPartColorProfile, windows_core::IUnknown, IPartBase);
#[repr(C)]
#[doc(hidden)]
pub struct IPartColorProfile_Vtbl {
    pub base__: IPartBase_Vtbl,
}
pub trait IPartColorProfile_Impl: IPartBase_Impl {}
impl IPartColorProfile_Vtbl {
    pub const fn new<Identity: IPartColorProfile_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPartBase_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartColorProfile as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartColorProfile {}
windows_core::imp::define_interface!(IPartDiscardControl, IPartDiscardControl_Vtbl, 0xcc350c00_095b_42a5_bf0f_c8780edadb3c);
windows_core::imp::interface_hierarchy!(IPartDiscardControl, windows_core::IUnknown);
impl IPartDiscardControl {
    pub unsafe fn GetDiscardProperties(&self, urisentinelpage: *mut windows_core::BSTR, uriparttodiscard: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDiscardProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(urisentinelpage), core::mem::transmute(uriparttodiscard)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartDiscardControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDiscardProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPartDiscardControl_Impl: windows_core::IUnknownImpl {
    fn GetDiscardProperties(&self, urisentinelpage: *mut windows_core::BSTR, uriparttodiscard: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IPartDiscardControl_Vtbl {
    pub const fn new<Identity: IPartDiscardControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDiscardProperties<Identity: IPartDiscardControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urisentinelpage: *mut *mut core::ffi::c_void, uriparttodiscard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPartDiscardControl_Impl::GetDiscardProperties(this, core::mem::transmute_copy(&urisentinelpage), core::mem::transmute_copy(&uriparttodiscard)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDiscardProperties: GetDiscardProperties::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartDiscardControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartDiscardControl {}
windows_core::imp::define_interface!(IPartFont, IPartFont_Vtbl, 0xe07fe0ab_1124_43d0_a865_e8ffb6a3ea82);
impl core::ops::Deref for IPartFont {
    type Target = IPartBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPartFont, windows_core::IUnknown, IPartBase);
impl IPartFont {
    pub unsafe fn GetFontProperties(&self, pcontenttype: *mut windows_core::BSTR, pfontoptions: *mut EXpsFontOptions) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFontProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(pcontenttype), pfontoptions as _).ok() }
    }
    pub unsafe fn SetFontContent<P0>(&self, pcontenttype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFontContent)(windows_core::Interface::as_raw(self), pcontenttype.param().abi()).ok() }
    }
    pub unsafe fn SetFontOptions(&self, options: EXpsFontOptions) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFontOptions)(windows_core::Interface::as_raw(self), options).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartFont_Vtbl {
    pub base__: IPartBase_Vtbl,
    pub GetFontProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut EXpsFontOptions) -> windows_core::HRESULT,
    pub SetFontContent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFontOptions: unsafe extern "system" fn(*mut core::ffi::c_void, EXpsFontOptions) -> windows_core::HRESULT,
}
pub trait IPartFont_Impl: IPartBase_Impl {
    fn GetFontProperties(&self, pcontenttype: *mut windows_core::BSTR, pfontoptions: *mut EXpsFontOptions) -> windows_core::Result<()>;
    fn SetFontContent(&self, pcontenttype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFontOptions(&self, options: EXpsFontOptions) -> windows_core::Result<()>;
}
impl IPartFont_Vtbl {
    pub const fn new<Identity: IPartFont_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontProperties<Identity: IPartFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: *mut *mut core::ffi::c_void, pfontoptions: *mut EXpsFontOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPartFont_Impl::GetFontProperties(this, core::mem::transmute_copy(&pcontenttype), core::mem::transmute_copy(&pfontoptions)).into()
            }
        }
        unsafe extern "system" fn SetFontContent<Identity: IPartFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPartFont_Impl::SetFontContent(this, core::mem::transmute(&pcontenttype)).into()
            }
        }
        unsafe extern "system" fn SetFontOptions<Identity: IPartFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: EXpsFontOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPartFont_Impl::SetFontOptions(this, core::mem::transmute_copy(&options)).into()
            }
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, OFFSET>(),
            GetFontProperties: GetFontProperties::<Identity, OFFSET>,
            SetFontContent: SetFontContent::<Identity, OFFSET>,
            SetFontOptions: SetFontOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartFont as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartFont {}
windows_core::imp::define_interface!(IPartFont2, IPartFont2_Vtbl, 0x511e025f_d6cb_43be_bf65_63fe88515a39);
impl core::ops::Deref for IPartFont2 {
    type Target = IPartFont;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPartFont2, windows_core::IUnknown, IPartBase, IPartFont);
impl IPartFont2 {
    pub unsafe fn GetFontRestriction(&self) -> windows_core::Result<EXpsFontRestriction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontRestriction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartFont2_Vtbl {
    pub base__: IPartFont_Vtbl,
    pub GetFontRestriction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EXpsFontRestriction) -> windows_core::HRESULT,
}
pub trait IPartFont2_Impl: IPartFont_Impl {
    fn GetFontRestriction(&self) -> windows_core::Result<EXpsFontRestriction>;
}
impl IPartFont2_Vtbl {
    pub const fn new<Identity: IPartFont2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontRestriction<Identity: IPartFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestriction: *mut EXpsFontRestriction) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPartFont2_Impl::GetFontRestriction(this) {
                    Ok(ok__) => {
                        prestriction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IPartFont_Vtbl::new::<Identity, OFFSET>(), GetFontRestriction: GetFontRestriction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartFont2 as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID || iid == &<IPartFont as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartFont2 {}
windows_core::imp::define_interface!(IPartImage, IPartImage_Vtbl, 0x725f2e3c_401a_4705_9de0_fe6f1353b87f);
impl core::ops::Deref for IPartImage {
    type Target = IPartBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPartImage, windows_core::IUnknown, IPartBase);
impl IPartImage {
    pub unsafe fn GetImageProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetImageContent<P0>(&self, pcontenttype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetImageContent)(windows_core::Interface::as_raw(self), pcontenttype.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartImage_Vtbl {
    pub base__: IPartBase_Vtbl,
    pub GetImageProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetImageContent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IPartImage_Impl: IPartBase_Impl {
    fn GetImageProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetImageContent(&self, pcontenttype: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IPartImage_Vtbl {
    pub const fn new<Identity: IPartImage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetImageProperties<Identity: IPartImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPartImage_Impl::GetImageProperties(this) {
                    Ok(ok__) => {
                        pcontenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetImageContent<Identity: IPartImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPartImage_Impl::SetImageContent(this, core::mem::transmute(&pcontenttype)).into()
            }
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, OFFSET>(),
            GetImageProperties: GetImageProperties::<Identity, OFFSET>,
            SetImageContent: SetImageContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartImage as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartImage {}
windows_core::imp::define_interface!(IPartPrintTicket, IPartPrintTicket_Vtbl, 0x4a0f50f6_f9a2_41f0_99e7_5ae955be8e9e);
impl core::ops::Deref for IPartPrintTicket {
    type Target = IPartBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPartPrintTicket, windows_core::IUnknown, IPartBase);
#[repr(C)]
#[doc(hidden)]
pub struct IPartPrintTicket_Vtbl {
    pub base__: IPartBase_Vtbl,
}
pub trait IPartPrintTicket_Impl: IPartBase_Impl {}
impl IPartPrintTicket_Vtbl {
    pub const fn new<Identity: IPartPrintTicket_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPartBase_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartPrintTicket as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartPrintTicket {}
windows_core::imp::define_interface!(IPartResourceDictionary, IPartResourceDictionary_Vtbl, 0x16cfce6d_e744_4fb3_b474_f1d54f024a01);
impl core::ops::Deref for IPartResourceDictionary {
    type Target = IPartBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPartResourceDictionary, windows_core::IUnknown, IPartBase);
#[repr(C)]
#[doc(hidden)]
pub struct IPartResourceDictionary_Vtbl {
    pub base__: IPartBase_Vtbl,
}
pub trait IPartResourceDictionary_Impl: IPartBase_Impl {}
impl IPartResourceDictionary_Vtbl {
    pub const fn new<Identity: IPartResourceDictionary_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPartBase_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartResourceDictionary as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartResourceDictionary {}
windows_core::imp::define_interface!(IPartThumbnail, IPartThumbnail_Vtbl, 0x027ed1c9_ba39_4cc5_aa55_7ec3a0de171a);
impl core::ops::Deref for IPartThumbnail {
    type Target = IPartBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPartThumbnail, windows_core::IUnknown, IPartBase);
impl IPartThumbnail {
    pub unsafe fn GetThumbnailProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThumbnailProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetThumbnailContent<P0>(&self, pcontenttype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetThumbnailContent)(windows_core::Interface::as_raw(self), pcontenttype.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartThumbnail_Vtbl {
    pub base__: IPartBase_Vtbl,
    pub GetThumbnailProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThumbnailContent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IPartThumbnail_Impl: IPartBase_Impl {
    fn GetThumbnailProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetThumbnailContent(&self, pcontenttype: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IPartThumbnail_Vtbl {
    pub const fn new<Identity: IPartThumbnail_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetThumbnailProperties<Identity: IPartThumbnail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPartThumbnail_Impl::GetThumbnailProperties(this) {
                    Ok(ok__) => {
                        pcontenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetThumbnailContent<Identity: IPartThumbnail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPartThumbnail_Impl::SetThumbnailContent(this, core::mem::transmute(&pcontenttype)).into()
            }
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, OFFSET>(),
            GetThumbnailProperties: GetThumbnailProperties::<Identity, OFFSET>,
            SetThumbnailContent: SetThumbnailContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartThumbnail as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartThumbnail {}
windows_core::imp::define_interface!(IPrintAsyncCookie, IPrintAsyncCookie_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IPrintAsyncCookie, windows_core::IUnknown);
impl IPrintAsyncCookie {
    pub unsafe fn FinishAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FinishAsyncCall)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn CancelAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelAsyncCall)(windows_core::Interface::as_raw(self), param0).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncCookie_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FinishAsyncCall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub CancelAsyncCall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IPrintAsyncCookie_Impl: windows_core::IUnknownImpl {
    fn FinishAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()>;
    fn CancelAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IPrintAsyncCookie_Vtbl {
    pub const fn new<Identity: IPrintAsyncCookie_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FinishAsyncCall<Identity: IPrintAsyncCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncCookie_Impl::FinishAsyncCall(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: IPrintAsyncCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncCookie_Impl::CancelAsyncCall(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FinishAsyncCall: FinishAsyncCall::<Identity, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncCookie as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintAsyncCookie {}
windows_core::imp::define_interface!(IPrintAsyncNewChannelCookie, IPrintAsyncNewChannelCookie_Vtbl, 0);
impl core::ops::Deref for IPrintAsyncNewChannelCookie {
    type Target = IPrintAsyncCookie;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintAsyncNewChannelCookie, windows_core::IUnknown, IPrintAsyncCookie);
impl IPrintAsyncNewChannelCookie {
    pub unsafe fn FinishAsyncCallWithData(&self, param0: *const Option<IPrintAsyncNotifyChannel>, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FinishAsyncCallWithData)(windows_core::Interface::as_raw(self), core::mem::transmute(param0), param1).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNewChannelCookie_Vtbl {
    pub base__: IPrintAsyncCookie_Vtbl,
    pub FinishAsyncCallWithData: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IPrintAsyncNewChannelCookie_Impl: IPrintAsyncCookie_Impl {
    fn FinishAsyncCallWithData(&self, param0: *const Option<IPrintAsyncNotifyChannel>, param1: u32) -> windows_core::Result<()>;
}
impl IPrintAsyncNewChannelCookie_Vtbl {
    pub const fn new<Identity: IPrintAsyncNewChannelCookie_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: IPrintAsyncNewChannelCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNewChannelCookie_Impl::FinishAsyncCallWithData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self { base__: IPrintAsyncCookie_Vtbl::new::<Identity, OFFSET>(), FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNewChannelCookie as windows_core::Interface>::IID || iid == &<IPrintAsyncCookie as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintAsyncNewChannelCookie {}
windows_core::imp::define_interface!(IPrintAsyncNotify, IPrintAsyncNotify_Vtbl, 0x532818f7_921b_4fb2_bff8_2f4fd52ebebf);
windows_core::imp::interface_hierarchy!(IPrintAsyncNotify, windows_core::IUnknown);
impl IPrintAsyncNotify {
    pub unsafe fn CreatePrintAsyncNotifyChannel<P4>(&self, param0: u32, param1: *const windows_core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: P4) -> windows_core::Result<IPrintAsyncNotifyChannel>
    where
        P4: windows_core::Param<IPrintAsyncNotifyCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePrintAsyncNotifyChannel)(windows_core::Interface::as_raw(self), param0, param1, param2, param3, param4.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePrintAsyncNotifyRegistration<P3>(&self, param0: *const windows_core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: P3) -> windows_core::Result<IPrintAsyncNotifyRegistration>
    where
        P3: windows_core::Param<IPrintAsyncNotifyCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePrintAsyncNotifyRegistration)(windows_core::Interface::as_raw(self), param0, param1, param2, param3.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreatePrintAsyncNotifyChannel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, PrintAsyncNotifyUserFilter, PrintAsyncNotifyConversationStyle, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePrintAsyncNotifyRegistration: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, PrintAsyncNotifyUserFilter, PrintAsyncNotifyConversationStyle, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintAsyncNotify_Impl: windows_core::IUnknownImpl {
    fn CreatePrintAsyncNotifyChannel(&self, param0: u32, param1: *const windows_core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: windows_core::Ref<IPrintAsyncNotifyCallback>) -> windows_core::Result<IPrintAsyncNotifyChannel>;
    fn CreatePrintAsyncNotifyRegistration(&self, param0: *const windows_core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: windows_core::Ref<IPrintAsyncNotifyCallback>) -> windows_core::Result<IPrintAsyncNotifyRegistration>;
}
impl IPrintAsyncNotify_Vtbl {
    pub const fn new<Identity: IPrintAsyncNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePrintAsyncNotifyChannel<Identity: IPrintAsyncNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *const windows_core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: *mut core::ffi::c_void, param5: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintAsyncNotify_Impl::CreatePrintAsyncNotifyChannel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)) {
                    Ok(ok__) => {
                        param5.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePrintAsyncNotifyRegistration<Identity: IPrintAsyncNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: *mut core::ffi::c_void, param4: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintAsyncNotify_Impl::CreatePrintAsyncNotifyRegistration(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)) {
                    Ok(ok__) => {
                        param4.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePrintAsyncNotifyChannel: CreatePrintAsyncNotifyChannel::<Identity, OFFSET>,
            CreatePrintAsyncNotifyRegistration: CreatePrintAsyncNotifyRegistration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintAsyncNotify {}
windows_core::imp::define_interface!(IPrintAsyncNotifyCallback, IPrintAsyncNotifyCallback_Vtbl, 0x7def34c1_9d92_4c99_b3b3_db94a9d4191b);
windows_core::imp::interface_hierarchy!(IPrintAsyncNotifyCallback, windows_core::IUnknown);
impl IPrintAsyncNotifyCallback {
    pub unsafe fn OnEventNotify<P0, P1>(&self, pchannel: P0, pdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintAsyncNotifyChannel>,
        P1: windows_core::Param<IPrintAsyncNotifyDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEventNotify)(windows_core::Interface::as_raw(self), pchannel.param().abi(), pdata.param().abi()).ok() }
    }
    pub unsafe fn ChannelClosed<P0, P1>(&self, pchannel: P0, pdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintAsyncNotifyChannel>,
        P1: windows_core::Param<IPrintAsyncNotifyDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).ChannelClosed)(windows_core::Interface::as_raw(self), pchannel.param().abi(), pdata.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotifyCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnEventNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChannelClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintAsyncNotifyCallback_Impl: windows_core::IUnknownImpl {
    fn OnEventNotify(&self, pchannel: windows_core::Ref<IPrintAsyncNotifyChannel>, pdata: windows_core::Ref<IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
    fn ChannelClosed(&self, pchannel: windows_core::Ref<IPrintAsyncNotifyChannel>, pdata: windows_core::Ref<IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
}
impl IPrintAsyncNotifyCallback_Vtbl {
    pub const fn new<Identity: IPrintAsyncNotifyCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnEventNotify<Identity: IPrintAsyncNotifyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNotifyCallback_Impl::OnEventNotify(this, core::mem::transmute_copy(&pchannel), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn ChannelClosed<Identity: IPrintAsyncNotifyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNotifyCallback_Impl::ChannelClosed(this, core::mem::transmute_copy(&pchannel), core::mem::transmute_copy(&pdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnEventNotify: OnEventNotify::<Identity, OFFSET>,
            ChannelClosed: ChannelClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintAsyncNotifyCallback {}
windows_core::imp::define_interface!(IPrintAsyncNotifyChannel, IPrintAsyncNotifyChannel_Vtbl, 0x4a5031b1_1f3f_4db0_a462_4530ed8b0451);
windows_core::imp::interface_hierarchy!(IPrintAsyncNotifyChannel, windows_core::IUnknown);
impl IPrintAsyncNotifyChannel {
    pub unsafe fn SendNotification<P0>(&self, pdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintAsyncNotifyDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendNotification)(windows_core::Interface::as_raw(self), pdata.param().abi()).ok() }
    }
    pub unsafe fn CloseChannel<P0>(&self, pdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintAsyncNotifyDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).CloseChannel)(windows_core::Interface::as_raw(self), pdata.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotifyChannel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SendNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintAsyncNotifyChannel_Impl: windows_core::IUnknownImpl {
    fn SendNotification(&self, pdata: windows_core::Ref<IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
    fn CloseChannel(&self, pdata: windows_core::Ref<IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
}
impl IPrintAsyncNotifyChannel_Vtbl {
    pub const fn new<Identity: IPrintAsyncNotifyChannel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendNotification<Identity: IPrintAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNotifyChannel_Impl::SendNotification(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn CloseChannel<Identity: IPrintAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNotifyChannel_Impl::CloseChannel(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendNotification: SendNotification::<Identity, OFFSET>,
            CloseChannel: CloseChannel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyChannel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintAsyncNotifyChannel {}
windows_core::imp::define_interface!(IPrintAsyncNotifyDataObject, IPrintAsyncNotifyDataObject_Vtbl, 0x77cf513e_5d49_4789_9f30_d0822b335c0d);
windows_core::imp::interface_hierarchy!(IPrintAsyncNotifyDataObject, windows_core::IUnknown);
impl IPrintAsyncNotifyDataObject {
    pub unsafe fn AcquireData(&self, ppnotificationdata: Option<*mut *mut u8>, psize: Option<*mut u32>, ppschema: Option<*mut *mut windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AcquireData)(windows_core::Interface::as_raw(self), ppnotificationdata.unwrap_or(core::mem::zeroed()) as _, psize.unwrap_or(core::mem::zeroed()) as _, ppschema.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ReleaseData(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseData)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotifyDataObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AcquireData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ReleaseData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintAsyncNotifyDataObject_Impl: windows_core::IUnknownImpl {
    fn AcquireData(&self, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn ReleaseData(&self) -> windows_core::Result<()>;
}
impl IPrintAsyncNotifyDataObject_Vtbl {
    pub const fn new<Identity: IPrintAsyncNotifyDataObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AcquireData<Identity: IPrintAsyncNotifyDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNotifyDataObject_Impl::AcquireData(this, core::mem::transmute_copy(&ppnotificationdata), core::mem::transmute_copy(&psize), core::mem::transmute_copy(&ppschema)).into()
            }
        }
        unsafe extern "system" fn ReleaseData<Identity: IPrintAsyncNotifyDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNotifyDataObject_Impl::ReleaseData(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireData: AcquireData::<Identity, OFFSET>,
            ReleaseData: ReleaseData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyDataObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintAsyncNotifyDataObject {}
windows_core::imp::define_interface!(IPrintAsyncNotifyRegistration, IPrintAsyncNotifyRegistration_Vtbl, 0x0f6f27b6_6f86_4591_9203_64c3bfadedfe);
windows_core::imp::interface_hierarchy!(IPrintAsyncNotifyRegistration, windows_core::IUnknown);
impl IPrintAsyncNotifyRegistration {
    pub unsafe fn RegisterForNotifications(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterForNotifications)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UnregisterForNotifications(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnregisterForNotifications)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotifyRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterForNotifications: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterForNotifications: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintAsyncNotifyRegistration_Impl: windows_core::IUnknownImpl {
    fn RegisterForNotifications(&self) -> windows_core::Result<()>;
    fn UnregisterForNotifications(&self) -> windows_core::Result<()>;
}
impl IPrintAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: IPrintAsyncNotifyRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterForNotifications<Identity: IPrintAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNotifyRegistration_Impl::RegisterForNotifications(this).into()
            }
        }
        unsafe extern "system" fn UnregisterForNotifications<Identity: IPrintAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNotifyRegistration_Impl::UnregisterForNotifications(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterForNotifications: RegisterForNotifications::<Identity, OFFSET>,
            UnregisterForNotifications: UnregisterForNotifications::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyRegistration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintAsyncNotifyRegistration {}
windows_core::imp::define_interface!(IPrintAsyncNotifyServerReferral, IPrintAsyncNotifyServerReferral_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IPrintAsyncNotifyServerReferral, windows_core::IUnknown);
impl IPrintAsyncNotifyServerReferral {
    pub unsafe fn GetServerReferral(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServerReferral)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AsyncGetServerReferral<P0>(&self, param0: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncGetSrvReferralCookie>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncGetServerReferral)(windows_core::Interface::as_raw(self), param0.param().abi()).ok() }
    }
    pub unsafe fn SetServerReferral<P0>(&self, prmtserverreferral: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServerReferral)(windows_core::Interface::as_raw(self), prmtserverreferral.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotifyServerReferral_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetServerReferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub AsyncGetServerReferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServerReferral: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IPrintAsyncNotifyServerReferral_Impl: windows_core::IUnknownImpl {
    fn GetServerReferral(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn AsyncGetServerReferral(&self, param0: windows_core::Ref<IAsyncGetSrvReferralCookie>) -> windows_core::Result<()>;
    fn SetServerReferral(&self, prmtserverreferral: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IPrintAsyncNotifyServerReferral_Vtbl {
    pub const fn new<Identity: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetServerReferral<Identity: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintAsyncNotifyServerReferral_Impl::GetServerReferral(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AsyncGetServerReferral<Identity: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNotifyServerReferral_Impl::AsyncGetServerReferral(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetServerReferral<Identity: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prmtserverreferral: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintAsyncNotifyServerReferral_Impl::SetServerReferral(this, core::mem::transmute(&prmtserverreferral)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetServerReferral: GetServerReferral::<Identity, OFFSET>,
            AsyncGetServerReferral: AsyncGetServerReferral::<Identity, OFFSET>,
            SetServerReferral: SetServerReferral::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyServerReferral as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintAsyncNotifyServerReferral {}
windows_core::imp::define_interface!(IPrintBidiAsyncNotifyRegistration, IPrintBidiAsyncNotifyRegistration_Vtbl, 0);
impl core::ops::Deref for IPrintBidiAsyncNotifyRegistration {
    type Target = IPrintAsyncNotifyRegistration;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintBidiAsyncNotifyRegistration, windows_core::IUnknown, IPrintAsyncNotifyRegistration);
impl IPrintBidiAsyncNotifyRegistration {
    pub unsafe fn AsyncGetNewChannel<P0>(&self, param0: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintAsyncNewChannelCookie>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncGetNewChannel)(windows_core::Interface::as_raw(self), param0.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintBidiAsyncNotifyRegistration_Vtbl {
    pub base__: IPrintAsyncNotifyRegistration_Vtbl,
    pub AsyncGetNewChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintBidiAsyncNotifyRegistration_Impl: IPrintAsyncNotifyRegistration_Impl {
    fn AsyncGetNewChannel(&self, param0: windows_core::Ref<IPrintAsyncNewChannelCookie>) -> windows_core::Result<()>;
}
impl IPrintBidiAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: IPrintBidiAsyncNotifyRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsyncGetNewChannel<Identity: IPrintBidiAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintBidiAsyncNotifyRegistration_Impl::AsyncGetNewChannel(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        Self { base__: IPrintAsyncNotifyRegistration_Vtbl::new::<Identity, OFFSET>(), AsyncGetNewChannel: AsyncGetNewChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintBidiAsyncNotifyRegistration as windows_core::Interface>::IID || iid == &<IPrintAsyncNotifyRegistration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintBidiAsyncNotifyRegistration {}
windows_core::imp::define_interface!(IPrintClassObjectFactory, IPrintClassObjectFactory_Vtbl, 0x9af593dd_9b02_48a8_9bad_69ace423f88b);
windows_core::imp::interface_hierarchy!(IPrintClassObjectFactory, windows_core::IUnknown);
impl IPrintClassObjectFactory {
    pub unsafe fn GetPrintClassObject<P0>(&self, pszprintername: P0, riid: *const windows_core::GUID, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPrintClassObject)(windows_core::Interface::as_raw(self), pszprintername.param().abi(), riid, ppnewobject as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintClassObjectFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPrintClassObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintClassObjectFactory_Impl: windows_core::IUnknownImpl {
    fn GetPrintClassObject(&self, pszprintername: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IPrintClassObjectFactory_Vtbl {
    pub const fn new<Identity: IPrintClassObjectFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPrintClassObject<Identity: IPrintClassObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprintername: windows_core::PCWSTR, riid: *const windows_core::GUID, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintClassObjectFactory_Impl::GetPrintClassObject(this, core::mem::transmute(&pszprintername), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppnewobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPrintClassObject: GetPrintClassObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintClassObjectFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintClassObjectFactory {}
windows_core::imp::define_interface!(IPrintCoreHelper, IPrintCoreHelper_Vtbl, 0xa89ec53e_3905_49c6_9c1a_c0a88117fdb6);
windows_core::imp::interface_hierarchy!(IPrintCoreHelper, windows_core::IUnknown);
impl IPrintCoreHelper {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetOption<P2>(&self, pdevmode: Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszfeaturerequested: P2) -> windows_core::Result<windows_core::PCSTR>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOption)(windows_core::Interface::as_raw(self), pdevmode.unwrap_or(core::mem::zeroed()) as _, cbsize, pszfeaturerequested.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetOptions(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: bool, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), pdevmode as _, cbsize, bresolveconflicts.into(), pfopairs, cpairs, pcpairswritten as _, pdwresult as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn EnumConstrainedOptions<P2>(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: P2, pconstrainedoptionlist: *const *const *const windows_core::PCSTR, pdwnumoptions: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumConstrainedOptions)(windows_core::Interface::as_raw(self), pdevmode, cbsize, pszfeaturekeyword.param().abi(), pconstrainedoptionlist, pdwnumoptions as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn WhyConstrained<P2, P3>(&self, pdevmode: Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszfeaturekeyword: P2, pszoptionkeyword: P3, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WhyConstrained)(windows_core::Interface::as_raw(self), pdevmode.unwrap_or(core::mem::zeroed()) as _, cbsize, pszfeaturekeyword.param().abi(), pszoptionkeyword.param().abi(), ppfoconstraints as _, pdwnumoptions as _).ok() }
    }
    pub unsafe fn EnumFeatures(&self, pfeaturelist: *mut *mut *mut windows_core::PCSTR, pdwnumfeatures: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumFeatures)(windows_core::Interface::as_raw(self), pfeaturelist as _, pdwnumfeatures as _).ok() }
    }
    pub unsafe fn EnumOptions<P0>(&self, pszfeaturekeyword: P0, poptionlist: *mut *mut *mut windows_core::PCSTR, pdwnumoptions: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumOptions)(windows_core::Interface::as_raw(self), pszfeaturekeyword.param().abi(), poptionlist as _, pdwnumoptions as _).ok() }
    }
    pub unsafe fn GetFontSubstitution<P0>(&self, psztruetypefontname: P0, ppszdevfontname: *const windows_core::PCWSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFontSubstitution)(windows_core::Interface::as_raw(self), psztruetypefontname.param().abi(), ppszdevfontname).ok() }
    }
    pub unsafe fn SetFontSubstitution<P0, P1>(&self, psztruetypefontname: P0, pszdevfontname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFontSubstitution)(windows_core::Interface::as_raw(self), psztruetypefontname.param().abi(), pszdevfontname.param().abi()).ok() }
    }
    pub unsafe fn CreateInstanceOfMSXMLObject<P1>(&self, rclsid: *const windows_core::GUID, punkouter: P1, dwclscontext: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateInstanceOfMSXMLObject)(windows_core::Interface::as_raw(self), rclsid, punkouter.param().abi(), dwclscontext, riid, ppv as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCoreHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetOption: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Gdi::DEVMODEA, u32, windows_core::PCSTR, *mut windows_core::PCSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetOption: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Gdi::DEVMODEA, u32, windows_core::BOOL, *const PRINT_FEATURE_OPTION, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetOptions: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub EnumConstrainedOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Gdi::DEVMODEA, u32, windows_core::PCSTR, *const *const *const windows_core::PCSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    EnumConstrainedOptions: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub WhyConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Gdi::DEVMODEA, u32, windows_core::PCSTR, windows_core::PCSTR, *mut *mut PRINT_FEATURE_OPTION, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    WhyConstrained: usize,
    pub EnumFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut *mut windows_core::PCSTR, *mut u32) -> windows_core::HRESULT,
    pub EnumOptions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut *mut *mut windows_core::PCSTR, *mut u32) -> windows_core::HRESULT,
    pub GetFontSubstitution: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFontSubstitution: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CreateInstanceOfMSXMLObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IPrintCoreHelper_Impl: windows_core::IUnknownImpl {
    fn GetOption(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: &windows_core::PCSTR) -> windows_core::Result<windows_core::PCSTR>;
    fn SetOptions(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: windows_core::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> windows_core::Result<()>;
    fn EnumConstrainedOptions(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: &windows_core::PCSTR, pconstrainedoptionlist: *const *const *const windows_core::PCSTR, pdwnumoptions: *mut u32) -> windows_core::Result<()>;
    fn WhyConstrained(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: &windows_core::PCSTR, pszoptionkeyword: &windows_core::PCSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> windows_core::Result<()>;
    fn EnumFeatures(&self, pfeaturelist: *mut *mut *mut windows_core::PCSTR, pdwnumfeatures: *mut u32) -> windows_core::Result<()>;
    fn EnumOptions(&self, pszfeaturekeyword: &windows_core::PCSTR, poptionlist: *mut *mut *mut windows_core::PCSTR, pdwnumoptions: *mut u32) -> windows_core::Result<()>;
    fn GetFontSubstitution(&self, psztruetypefontname: &windows_core::PCWSTR, ppszdevfontname: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFontSubstitution(&self, psztruetypefontname: &windows_core::PCWSTR, pszdevfontname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CreateInstanceOfMSXMLObject(&self, rclsid: *const windows_core::GUID, punkouter: windows_core::Ref<windows_core::IUnknown>, dwclscontext: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IPrintCoreHelper_Vtbl {
    pub const fn new<Identity: IPrintCoreHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOption<Identity: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: windows_core::PCSTR, ppszoption: *mut windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintCoreHelper_Impl::GetOption(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute(&pszfeaturerequested)) {
                    Ok(ok__) => {
                        ppszoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOptions<Identity: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: windows_core::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelper_Impl::SetOptions(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&bresolveconflicts), core::mem::transmute_copy(&pfopairs), core::mem::transmute_copy(&cpairs), core::mem::transmute_copy(&pcpairswritten), core::mem::transmute_copy(&pdwresult)).into()
            }
        }
        unsafe extern "system" fn EnumConstrainedOptions<Identity: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: windows_core::PCSTR, pconstrainedoptionlist: *const *const *const windows_core::PCSTR, pdwnumoptions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelper_Impl::EnumConstrainedOptions(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute_copy(&pconstrainedoptionlist), core::mem::transmute_copy(&pdwnumoptions)).into()
            }
        }
        unsafe extern "system" fn WhyConstrained<Identity: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: windows_core::PCSTR, pszoptionkeyword: windows_core::PCSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelper_Impl::WhyConstrained(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszoptionkeyword), core::mem::transmute_copy(&ppfoconstraints), core::mem::transmute_copy(&pdwnumoptions)).into()
            }
        }
        unsafe extern "system" fn EnumFeatures<Identity: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfeaturelist: *mut *mut *mut windows_core::PCSTR, pdwnumfeatures: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelper_Impl::EnumFeatures(this, core::mem::transmute_copy(&pfeaturelist), core::mem::transmute_copy(&pdwnumfeatures)).into()
            }
        }
        unsafe extern "system" fn EnumOptions<Identity: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfeaturekeyword: windows_core::PCSTR, poptionlist: *mut *mut *mut windows_core::PCSTR, pdwnumoptions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelper_Impl::EnumOptions(this, core::mem::transmute(&pszfeaturekeyword), core::mem::transmute_copy(&poptionlist), core::mem::transmute_copy(&pdwnumoptions)).into()
            }
        }
        unsafe extern "system" fn GetFontSubstitution<Identity: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztruetypefontname: windows_core::PCWSTR, ppszdevfontname: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelper_Impl::GetFontSubstitution(this, core::mem::transmute(&psztruetypefontname), core::mem::transmute_copy(&ppszdevfontname)).into()
            }
        }
        unsafe extern "system" fn SetFontSubstitution<Identity: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztruetypefontname: windows_core::PCWSTR, pszdevfontname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelper_Impl::SetFontSubstitution(this, core::mem::transmute(&psztruetypefontname), core::mem::transmute(&pszdevfontname)).into()
            }
        }
        unsafe extern "system" fn CreateInstanceOfMSXMLObject<Identity: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, punkouter: *mut core::ffi::c_void, dwclscontext: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelper_Impl::CreateInstanceOfMSXMLObject(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOption: GetOption::<Identity, OFFSET>,
            SetOptions: SetOptions::<Identity, OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Identity, OFFSET>,
            WhyConstrained: WhyConstrained::<Identity, OFFSET>,
            EnumFeatures: EnumFeatures::<Identity, OFFSET>,
            EnumOptions: EnumOptions::<Identity, OFFSET>,
            GetFontSubstitution: GetFontSubstitution::<Identity, OFFSET>,
            SetFontSubstitution: SetFontSubstitution::<Identity, OFFSET>,
            CreateInstanceOfMSXMLObject: CreateInstanceOfMSXMLObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCoreHelper as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IPrintCoreHelper {}
windows_core::imp::define_interface!(IPrintCoreHelperPS, IPrintCoreHelperPS_Vtbl, 0xc2c14f6f_95d3_4d63_96cf_6bd9e6c907c2);
impl core::ops::Deref for IPrintCoreHelperPS {
    type Target = IPrintCoreHelper;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintCoreHelperPS, windows_core::IUnknown, IPrintCoreHelper);
impl IPrintCoreHelperPS {
    pub unsafe fn GetGlobalAttribute<P0>(&self, pszattribute: P0, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetGlobalAttribute)(windows_core::Interface::as_raw(self), pszattribute.param().abi(), pdwdatatype as _, ppbdata as _, pcbsize as _).ok() }
    }
    pub unsafe fn GetFeatureAttribute<P0, P1>(&self, pszfeaturekeyword: P0, pszattribute: P1, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFeatureAttribute)(windows_core::Interface::as_raw(self), pszfeaturekeyword.param().abi(), pszattribute.param().abi(), pdwdatatype as _, ppbdata as _, pcbsize as _).ok() }
    }
    pub unsafe fn GetOptionAttribute<P0, P1, P2>(&self, pszfeaturekeyword: P0, pszoptionkeyword: P1, pszattribute: P2, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetOptionAttribute)(windows_core::Interface::as_raw(self), pszfeaturekeyword.param().abi(), pszoptionkeyword.param().abi(), pszattribute.param().abi(), pdwdatatype as _, ppbdata as _, pcbsize as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCoreHelperPS_Vtbl {
    pub base__: IPrintCoreHelper_Vtbl,
    pub GetGlobalAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetFeatureAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, *mut u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetOptionAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, windows_core::PCSTR, *mut u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IPrintCoreHelperPS_Impl: IPrintCoreHelper_Impl {
    fn GetGlobalAttribute(&self, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::Result<()>;
    fn GetFeatureAttribute(&self, pszfeaturekeyword: &windows_core::PCSTR, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::Result<()>;
    fn GetOptionAttribute(&self, pszfeaturekeyword: &windows_core::PCSTR, pszoptionkeyword: &windows_core::PCSTR, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IPrintCoreHelperPS_Vtbl {
    pub const fn new<Identity: IPrintCoreHelperPS_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGlobalAttribute<Identity: IPrintCoreHelperPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelperPS_Impl::GetGlobalAttribute(this, core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbsize)).into()
            }
        }
        unsafe extern "system" fn GetFeatureAttribute<Identity: IPrintCoreHelperPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfeaturekeyword: windows_core::PCSTR, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelperPS_Impl::GetFeatureAttribute(this, core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbsize)).into()
            }
        }
        unsafe extern "system" fn GetOptionAttribute<Identity: IPrintCoreHelperPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfeaturekeyword: windows_core::PCSTR, pszoptionkeyword: windows_core::PCSTR, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelperPS_Impl::GetOptionAttribute(this, core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszoptionkeyword), core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbsize)).into()
            }
        }
        Self {
            base__: IPrintCoreHelper_Vtbl::new::<Identity, OFFSET>(),
            GetGlobalAttribute: GetGlobalAttribute::<Identity, OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Identity, OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCoreHelperPS as windows_core::Interface>::IID || iid == &<IPrintCoreHelper as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IPrintCoreHelperPS {}
windows_core::imp::define_interface!(IPrintCoreHelperUni, IPrintCoreHelperUni_Vtbl, 0x7e8e51d6_e5ee_4426_817b_958b9444eb79);
impl core::ops::Deref for IPrintCoreHelperUni {
    type Target = IPrintCoreHelper;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintCoreHelperUni, windows_core::IUnknown, IPrintCoreHelper);
impl IPrintCoreHelperUni {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn CreateGDLSnapshot(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut Option<super::super::System::Com::IStream>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateGDLSnapshot)(windows_core::Interface::as_raw(self), pdevmode as _, cbsize, dwflags, core::mem::transmute(ppsnapshotstream)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDefaultGDLSnapshot(&self, dwflags: u32) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDefaultGDLSnapshot)(windows_core::Interface::as_raw(self), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCoreHelperUni_Vtbl {
    pub base__: IPrintCoreHelper_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub CreateGDLSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Gdi::DEVMODEA, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    CreateGDLSnapshot: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDefaultGDLSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDefaultGDLSnapshot: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni_Impl: IPrintCoreHelper_Impl {
    fn CreateGDLSnapshot(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: windows_core::OutRef<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CreateDefaultGDLSnapshot(&self, dwflags: u32) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUni_Vtbl {
    pub const fn new<Identity: IPrintCoreHelperUni_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateGDLSnapshot<Identity: IPrintCoreHelperUni_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelperUni_Impl::CreateGDLSnapshot(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppsnapshotstream)).into()
            }
        }
        unsafe extern "system" fn CreateDefaultGDLSnapshot<Identity: IPrintCoreHelperUni_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppsnapshotstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintCoreHelperUni_Impl::CreateDefaultGDLSnapshot(this, core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppsnapshotstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPrintCoreHelper_Vtbl::new::<Identity, OFFSET>(),
            CreateGDLSnapshot: CreateGDLSnapshot::<Identity, OFFSET>,
            CreateDefaultGDLSnapshot: CreateDefaultGDLSnapshot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCoreHelperUni as windows_core::Interface>::IID || iid == &<IPrintCoreHelper as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintCoreHelperUni {}
windows_core::imp::define_interface!(IPrintCoreHelperUni2, IPrintCoreHelperUni2_Vtbl, 0x6c8afdfc_ead0_4d2d_8071_9bf0175a6c3a);
impl core::ops::Deref for IPrintCoreHelperUni2 {
    type Target = IPrintCoreHelperUni;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintCoreHelperUni2, windows_core::IUnknown, IPrintCoreHelper, IPrintCoreHelperUni);
impl IPrintCoreHelperUni2 {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetNamedCommand<P2>(&self, pdevmode: Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszcommandname: P2, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetNamedCommand)(windows_core::Interface::as_raw(self), pdevmode.unwrap_or(core::mem::zeroed()) as _, cbsize, pszcommandname.param().abi(), ppcommandbytes as _, pcbcommandsize as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCoreHelperUni2_Vtbl {
    pub base__: IPrintCoreHelperUni_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetNamedCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Gdi::DEVMODEA, u32, windows_core::PCWSTR, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetNamedCommand: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni2_Impl: IPrintCoreHelperUni_Impl {
    fn GetNamedCommand(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: &windows_core::PCWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUni2_Vtbl {
    pub const fn new<Identity: IPrintCoreHelperUni2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNamedCommand<Identity: IPrintCoreHelperUni2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: windows_core::PCWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreHelperUni2_Impl::GetNamedCommand(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute(&pszcommandname), core::mem::transmute_copy(&ppcommandbytes), core::mem::transmute_copy(&pcbcommandsize)).into()
            }
        }
        Self { base__: IPrintCoreHelperUni_Vtbl::new::<Identity, OFFSET>(), GetNamedCommand: GetNamedCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCoreHelperUni2 as windows_core::Interface>::IID || iid == &<IPrintCoreHelper as windows_core::Interface>::IID || iid == &<IPrintCoreHelperUni as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintCoreHelperUni2 {}
windows_core::imp::define_interface!(IPrintCoreUI2, IPrintCoreUI2_Vtbl, 0x085ccfca_3adf_4c9e_b491_d851a6edc997);
impl core::ops::Deref for IPrintCoreUI2 {
    type Target = IPrintOemDriverUI;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintCoreUI2, windows_core::IUnknown, IPrintOemDriverUI);
impl IPrintCoreUI2 {
    pub unsafe fn GetOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: Option<u32>, pmszfeaturesrequested: Option<&[u8]>, pmszfeatureoptionbuf: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetOptions)(
                windows_core::Interface::as_raw(self),
                poemuiobj,
                dwflags.unwrap_or(core::mem::zeroed()) as _,
                core::mem::transmute(pmszfeaturesrequested.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
                pmszfeaturesrequested.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(pmszfeatureoptionbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
                pmszfeatureoptionbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
                pcbneeded as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: &[u8]) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), poemuiobj, dwflags, core::mem::transmute(pmszfeatureoptionbuf.as_ptr()), pmszfeatureoptionbuf.len().try_into().unwrap(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumConstrainedOptions<P2>(&self, poemuiobj: *const OEMUIOBJ, dwflags: Option<u32>, pszfeaturekeyword: P2, pmszconstrainedoptionlist: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumConstrainedOptions)(windows_core::Interface::as_raw(self), poemuiobj, dwflags.unwrap_or(core::mem::zeroed()) as _, pszfeaturekeyword.param().abi(), core::mem::transmute(pmszconstrainedoptionlist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pmszconstrainedoptionlist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _).ok() }
    }
    pub unsafe fn WhyConstrained<P2, P3>(&self, poemuiobj: *const OEMUIOBJ, dwflags: Option<u32>, pszfeaturekeyword: P2, pszoptionkeyword: P3, pmszreasonlist: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WhyConstrained)(windows_core::Interface::as_raw(self), poemuiobj, dwflags.unwrap_or(core::mem::zeroed()) as _, pszfeaturekeyword.param().abi(), pszoptionkeyword.param().abi(), core::mem::transmute(pmszreasonlist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pmszreasonlist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _).ok() }
    }
    pub unsafe fn GetGlobalAttribute<P2>(&self, poemuiobj: *const OEMUIOBJ, dwflags: Option<u32>, pszattribute: P2, pdwdatatype: *mut u32, pbdata: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetGlobalAttribute)(windows_core::Interface::as_raw(self), poemuiobj, dwflags.unwrap_or(core::mem::zeroed()) as _, pszattribute.param().abi(), pdwdatatype as _, core::mem::transmute(pbdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _).ok() }
    }
    pub unsafe fn GetFeatureAttribute<P2, P3>(&self, poemuiobj: *const OEMUIOBJ, dwflags: Option<u32>, pszfeaturekeyword: P2, pszattribute: P3, pdwdatatype: *mut u32, pbdata: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFeatureAttribute)(windows_core::Interface::as_raw(self), poemuiobj, dwflags.unwrap_or(core::mem::zeroed()) as _, pszfeaturekeyword.param().abi(), pszattribute.param().abi(), pdwdatatype as _, core::mem::transmute(pbdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _).ok() }
    }
    pub unsafe fn GetOptionAttribute<P2, P3, P4>(&self, poemuiobj: *const OEMUIOBJ, dwflags: Option<u32>, pszfeaturekeyword: P2, pszoptionkeyword: P3, pszattribute: P4, pdwdatatype: *mut u32, pbdata: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
        P4: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetOptionAttribute)(windows_core::Interface::as_raw(self), poemuiobj, dwflags.unwrap_or(core::mem::zeroed()) as _, pszfeaturekeyword.param().abi(), pszoptionkeyword.param().abi(), pszattribute.param().abi(), pdwdatatype as _, core::mem::transmute(pbdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _).ok() }
    }
    pub unsafe fn EnumFeatures(&self, poemuiobj: *const OEMUIOBJ, dwflags: Option<u32>, pmszfeaturelist: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumFeatures)(windows_core::Interface::as_raw(self), poemuiobj, dwflags.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pmszfeaturelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pmszfeaturelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _).ok() }
    }
    pub unsafe fn EnumOptions<P2>(&self, poemuiobj: *const OEMUIOBJ, dwflags: Option<u32>, pszfeaturekeyword: P2, pmszoptionlist: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumOptions)(windows_core::Interface::as_raw(self), poemuiobj, dwflags.unwrap_or(core::mem::zeroed()) as _, pszfeaturekeyword.param().abi(), core::mem::transmute(pmszoptionlist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pmszoptionlist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _).ok() }
    }
    pub unsafe fn QuerySimulationSupport(&self, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: Option<&mut [u8]>, pcbneeded: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QuerySimulationSupport)(windows_core::Interface::as_raw(self), hprinter, dwlevel, core::mem::transmute(pcaps.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcaps.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbneeded as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCoreUI2_Vtbl {
    pub base__: IPrintOemDriverUI_Vtbl,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const OEMUIOBJ, u32, *const i8, u32, windows_core::PSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const OEMUIOBJ, u32, *const i8, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumConstrainedOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const OEMUIOBJ, u32, windows_core::PCSTR, windows_core::PSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub WhyConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *const OEMUIOBJ, u32, windows_core::PCSTR, windows_core::PCSTR, windows_core::PSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetGlobalAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const OEMUIOBJ, u32, windows_core::PCSTR, *mut u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFeatureAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const OEMUIOBJ, u32, windows_core::PCSTR, windows_core::PCSTR, *mut u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetOptionAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const OEMUIOBJ, u32, windows_core::PCSTR, windows_core::PCSTR, windows_core::PCSTR, *mut u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *const OEMUIOBJ, u32, windows_core::PSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const OEMUIOBJ, u32, windows_core::PCSTR, windows_core::PSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub QuerySimulationSupport: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IPrintCoreUI2_Impl: IPrintOemDriverUI_Impl {
    fn GetOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn SetOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32) -> windows_core::Result<u32>;
    fn EnumConstrainedOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &windows_core::PCSTR, pmszconstrainedoptionlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn WhyConstrained(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &windows_core::PCSTR, pszoptionkeyword: &windows_core::PCSTR, pmszreasonlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn GetGlobalAttribute(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn GetFeatureAttribute(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &windows_core::PCSTR, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn GetOptionAttribute(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &windows_core::PCSTR, pszoptionkeyword: &windows_core::PCSTR, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn EnumFeatures(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn EnumOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &windows_core::PCSTR, pmszoptionlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn QuerySimulationSupport(&self, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
}
impl IPrintCoreUI2_Vtbl {
    pub const fn new<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptions<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreUI2_Impl::GetOptions(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pmszfeaturesrequested), core::mem::transmute_copy(&cbin), core::mem::transmute_copy(&pmszfeatureoptionbuf), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
            }
        }
        unsafe extern "system" fn SetOptions<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintCoreUI2_Impl::SetOptions(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pmszfeatureoptionbuf), core::mem::transmute_copy(&cbin)) {
                    Ok(ok__) => {
                        pdwresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumConstrainedOptions<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: windows_core::PCSTR, pmszconstrainedoptionlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreUI2_Impl::EnumConstrainedOptions(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute_copy(&pmszconstrainedoptionlist), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
            }
        }
        unsafe extern "system" fn WhyConstrained<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: windows_core::PCSTR, pszoptionkeyword: windows_core::PCSTR, pmszreasonlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreUI2_Impl::WhyConstrained(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszoptionkeyword), core::mem::transmute_copy(&pmszreasonlist), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
            }
        }
        unsafe extern "system" fn GetGlobalAttribute<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreUI2_Impl::GetGlobalAttribute(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
            }
        }
        unsafe extern "system" fn GetFeatureAttribute<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: windows_core::PCSTR, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreUI2_Impl::GetFeatureAttribute(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
            }
        }
        unsafe extern "system" fn GetOptionAttribute<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: windows_core::PCSTR, pszoptionkeyword: windows_core::PCSTR, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreUI2_Impl::GetOptionAttribute(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszoptionkeyword), core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
            }
        }
        unsafe extern "system" fn EnumFeatures<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreUI2_Impl::EnumFeatures(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pmszfeaturelist), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
            }
        }
        unsafe extern "system" fn EnumOptions<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: windows_core::PCSTR, pmszoptionlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreUI2_Impl::EnumOptions(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute_copy(&pmszoptionlist), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
            }
        }
        unsafe extern "system" fn QuerySimulationSupport<Identity: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintCoreUI2_Impl::QuerySimulationSupport(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&dwlevel), core::mem::transmute_copy(&pcaps), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
            }
        }
        Self {
            base__: IPrintOemDriverUI_Vtbl::new::<Identity, OFFSET>(),
            GetOptions: GetOptions::<Identity, OFFSET>,
            SetOptions: SetOptions::<Identity, OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Identity, OFFSET>,
            WhyConstrained: WhyConstrained::<Identity, OFFSET>,
            GetGlobalAttribute: GetGlobalAttribute::<Identity, OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Identity, OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Identity, OFFSET>,
            EnumFeatures: EnumFeatures::<Identity, OFFSET>,
            EnumOptions: EnumOptions::<Identity, OFFSET>,
            QuerySimulationSupport: QuerySimulationSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCoreUI2 as windows_core::Interface>::IID || iid == &<IPrintOemDriverUI as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintCoreUI2 {}
windows_core::imp::define_interface!(IPrintJob, IPrintJob_Vtbl, 0xb771dab8_1282_41b7_858c_f206e4d20577);
windows_core::imp::interface_hierarchy!(IPrintJob, windows_core::IUnknown);
impl IPrintJob {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrintedPages(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrintedPages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalPages(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalPages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<PrintJobStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SubmissionTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubmissionTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RequestCancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestCancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintJob_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PrintedPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TotalPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintJobStatus) -> windows_core::HRESULT,
    pub SubmissionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub RequestCancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintJob_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<u32>;
    fn PrintedPages(&self) -> windows_core::Result<u32>;
    fn TotalPages(&self) -> windows_core::Result<u32>;
    fn Status(&self) -> windows_core::Result<PrintJobStatus>;
    fn SubmissionTime(&self) -> windows_core::Result<f64>;
    fn RequestCancel(&self) -> windows_core::Result<()>;
}
impl IPrintJob_Vtbl {
    pub const fn new<Identity: IPrintJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintJob_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: IPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintJob_Impl::Id(this) {
                    Ok(ok__) => {
                        pulid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrintedPages<Identity: IPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintJob_Impl::PrintedPages(this) {
                    Ok(ok__) => {
                        pulpages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalPages<Identity: IPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpages: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintJob_Impl::TotalPages(this) {
                    Ok(ok__) => {
                        pulpages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut PrintJobStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintJob_Impl::Status(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubmissionTime<Identity: IPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubmissiontime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintJob_Impl::SubmissionTime(this) {
                    Ok(ok__) => {
                        psubmissiontime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestCancel<Identity: IPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintJob_Impl::RequestCancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            PrintedPages: PrintedPages::<Identity, OFFSET>,
            TotalPages: TotalPages::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, OFFSET>,
            RequestCancel: RequestCancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintJob as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintJob {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintJobCollection, IPrintJobCollection_Vtbl, 0x72b82a24_a598_4e87_895f_cdb23a49e9dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintJobCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintJobCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrintJobCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, ulindex: u32) -> windows_core::Result<IPrintJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), ulindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintJobCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintJobCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, ulindex: u32) -> windows_core::Result<IPrintJob>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintJobCollection_Vtbl {
    pub const fn new<Identity: IPrintJobCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IPrintJobCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintJobCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pulcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPrintJobCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintJobCollection_Impl::GetAt(this, core::mem::transmute_copy(&ulindex)) {
                    Ok(ok__) => {
                        ppjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IPrintJobCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintJobCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintJobCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintJobCollection {}
windows_core::imp::define_interface!(IPrintOemCommon, IPrintOemCommon_Vtbl, 0x7f42285e_91d5_11d1_8820_00c04fb961ec);
windows_core::imp::interface_hierarchy!(IPrintOemCommon, windows_core::IUnknown);
impl IPrintOemCommon {
    pub unsafe fn GetInfo(&self, dwmode: u32, pbuffer: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), dwmode, pbuffer as _, cbsize, pcbneeded as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn DevMode(&self, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DevMode)(windows_core::Interface::as_raw(self), dwmode, poemdmparam as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOemCommon_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub DevMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut OEMDMPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    DevMode: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IPrintOemCommon_Impl: windows_core::IUnknownImpl {
    fn GetInfo(&self, dwmode: u32, pbuffer: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn DevMode(&self, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IPrintOemCommon_Vtbl {
    pub const fn new<Identity: IPrintOemCommon_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInfo<Identity: IPrintOemCommon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32, pbuffer: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemCommon_Impl::GetInfo(this, core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
            }
        }
        unsafe extern "system" fn DevMode<Identity: IPrintOemCommon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemCommon_Impl::DevMode(this, core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&poemdmparam)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetInfo: GetInfo::<Identity, OFFSET>, DevMode: DevMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOemCommon as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IPrintOemCommon {}
windows_core::imp::define_interface!(IPrintOemDriverUI, IPrintOemDriverUI_Vtbl, 0x92b05d50_78bc_11d1_9480_00a0c90640b8);
windows_core::imp::interface_hierarchy!(IPrintOemDriverUI, windows_core::IUnknown);
impl IPrintOemDriverUI {
    pub unsafe fn DrvGetDriverSetting<P1>(&self, pci: *mut core::ffi::c_void, feature: P1, poutput: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrvGetDriverSetting)(windows_core::Interface::as_raw(self), pci as _, feature.param().abi(), poutput as _, cbsize, pcbneeded as _, pdwoptionsreturned as _).ok() }
    }
    pub unsafe fn DrvUpgradeRegistrySetting<P1, P2>(&self, hprinter: super::super::Foundation::HANDLE, pfeature: P1, poption: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrvUpgradeRegistrySetting)(windows_core::Interface::as_raw(self), hprinter, pfeature.param().abi(), poption.param().abi()).ok() }
    }
    pub unsafe fn DrvUpdateUISetting(&self, pci: *mut core::ffi::c_void, poptitem: *mut core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DrvUpdateUISetting)(windows_core::Interface::as_raw(self), pci as _, poptitem as _, dwpreviousselection, dwmode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOemDriverUI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DrvGetDriverSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCSTR, *mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub DrvUpgradeRegistrySetting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, windows_core::PCSTR, windows_core::PCSTR) -> windows_core::HRESULT,
    pub DrvUpdateUISetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait IPrintOemDriverUI_Impl: windows_core::IUnknownImpl {
    fn DrvGetDriverSetting(&self, pci: *mut core::ffi::c_void, feature: &windows_core::PCSTR, poutput: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> windows_core::Result<()>;
    fn DrvUpgradeRegistrySetting(&self, hprinter: super::super::Foundation::HANDLE, pfeature: &windows_core::PCSTR, poption: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn DrvUpdateUISetting(&self, pci: *mut core::ffi::c_void, poptitem: *mut core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> windows_core::Result<()>;
}
impl IPrintOemDriverUI_Vtbl {
    pub const fn new<Identity: IPrintOemDriverUI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DrvGetDriverSetting<Identity: IPrintOemDriverUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pci: *mut core::ffi::c_void, feature: windows_core::PCSTR, poutput: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemDriverUI_Impl::DrvGetDriverSetting(this, core::mem::transmute_copy(&pci), core::mem::transmute(&feature), core::mem::transmute_copy(&poutput), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded), core::mem::transmute_copy(&pdwoptionsreturned)).into()
            }
        }
        unsafe extern "system" fn DrvUpgradeRegistrySetting<Identity: IPrintOemDriverUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pfeature: windows_core::PCSTR, poption: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemDriverUI_Impl::DrvUpgradeRegistrySetting(this, core::mem::transmute_copy(&hprinter), core::mem::transmute(&pfeature), core::mem::transmute(&poption)).into()
            }
        }
        unsafe extern "system" fn DrvUpdateUISetting<Identity: IPrintOemDriverUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pci: *mut core::ffi::c_void, poptitem: *mut core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemDriverUI_Impl::DrvUpdateUISetting(this, core::mem::transmute_copy(&pci), core::mem::transmute_copy(&poptitem), core::mem::transmute_copy(&dwpreviousselection), core::mem::transmute_copy(&dwmode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DrvGetDriverSetting: DrvGetDriverSetting::<Identity, OFFSET>,
            DrvUpgradeRegistrySetting: DrvUpgradeRegistrySetting::<Identity, OFFSET>,
            DrvUpdateUISetting: DrvUpdateUISetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOemDriverUI as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintOemDriverUI {}
windows_core::imp::define_interface!(IPrintOemUI, IPrintOemUI_Vtbl, 0xc6a7a9d0_774c_11d1_947f_00a0c90640b8);
impl core::ops::Deref for IPrintOemUI {
    type Target = IPrintOemCommon;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintOemUI, windows_core::IUnknown, IPrintOemCommon);
impl IPrintOemUI {
    pub unsafe fn PublishDriverInterface<P0>(&self, piunknown: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).PublishDriverInterface)(windows_core::Interface::as_raw(self), piunknown.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn CommonUIProp(&self, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CommonUIProp)(windows_core::Interface::as_raw(self), dwmode, poemcuipparam).ok() }
    }
    pub unsafe fn DocumentPropertySheets(&self, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DocumentPropertySheets)(windows_core::Interface::as_raw(self), ppsuiinfo as _, lparam).ok() }
    }
    pub unsafe fn DevicePropertySheets(&self, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DevicePropertySheets)(windows_core::Interface::as_raw(self), ppsuiinfo, lparam).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn DevQueryPrintEx(&self, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DevQueryPrintEx)(windows_core::Interface::as_raw(self), poemuiobj, pdqpinfo, ppublicdm, poemdm).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn DeviceCapabilitiesA<P2>(&self, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: P2, wcapability: u16, poutput: *mut core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeviceCapabilitiesA)(windows_core::Interface::as_raw(self), poemuiobj as _, hprinter, pdevicename.param().abi(), wcapability, poutput as _, ppublicdm, poemdm, dwold, dwresult as _).ok() }
    }
    pub unsafe fn UpgradePrinter(&self, dwlevel: u32, pdriverupgradeinfo: *const u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpgradePrinter)(windows_core::Interface::as_raw(self), dwlevel, pdriverupgradeinfo).ok() }
    }
    pub unsafe fn PrinterEvent<P0>(&self, pprintername: P0, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PrinterEvent)(windows_core::Interface::as_raw(self), pprintername.param().abi(), idriverevent, dwflags, lparam).ok() }
    }
    pub unsafe fn DriverEvent(&self, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DriverEvent)(windows_core::Interface::as_raw(self), dwdriverevent, dwlevel, pdriverinfo, lparam).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn QueryColorProfile(&self, hprinter: PRINTER_HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryColorProfile)(windows_core::Interface::as_raw(self), core::mem::transmute(hprinter), poemuiobj, ppublicdm, poemdm, ulquerymode, pvprofiledata as _, pcbprofiledata as _, pflprofiledata as _).ok() }
    }
    pub unsafe fn FontInstallerDlgProc(&self, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FontInstallerDlgProc)(windows_core::Interface::as_raw(self), hwnd, usmsg, wparam, lparam).ok() }
    }
    pub unsafe fn UpdateExternalFonts<P2>(&self, hprinter: PRINTER_HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateExternalFonts)(windows_core::Interface::as_raw(self), core::mem::transmute(hprinter), hheap, pwstrcartridges.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOemUI_Vtbl {
    pub base__: IPrintOemCommon_Vtbl,
    pub PublishDriverInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub CommonUIProp: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const OEMCUIPPARAM) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    CommonUIProp: usize,
    pub DocumentPropertySheets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPSHEETUI_INFO, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub DevicePropertySheets: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROPSHEETUI_INFO, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub DevQueryPrintEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const OEMUIOBJ, *const DEVQUERYPRINT_INFO, *const super::Gdi::DEVMODEA, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    DevQueryPrintEx: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub DeviceCapabilitiesA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OEMUIOBJ, super::super::Foundation::HANDLE, windows_core::PCWSTR, u16, *mut core::ffi::c_void, *const super::Gdi::DEVMODEA, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    DeviceCapabilitiesA: usize,
    pub UpgradePrinter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub PrinterEvent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, u32, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub DriverEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub QueryColorProfile: unsafe extern "system" fn(*mut core::ffi::c_void, PRINTER_HANDLE, *const OEMUIOBJ, *const super::Gdi::DEVMODEA, *const core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    QueryColorProfile: usize,
    pub FontInstallerDlgProc: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub UpdateExternalFonts: unsafe extern "system" fn(*mut core::ffi::c_void, PRINTER_HANDLE, super::super::Foundation::HANDLE, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI_Impl: IPrintOemCommon_Impl {
    fn PublishDriverInterface(&self, piunknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CommonUIProp(&self, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> windows_core::Result<()>;
    fn DocumentPropertySheets(&self, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn DevicePropertySheets(&self, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn DevQueryPrintEx(&self, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn DeviceCapabilitiesA(&self, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: &windows_core::PCWSTR, wcapability: u16, poutput: *mut core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> windows_core::Result<()>;
    fn UpgradePrinter(&self, dwlevel: u32, pdriverupgradeinfo: *const u8) -> windows_core::Result<()>;
    fn PrinterEvent(&self, pprintername: &windows_core::PCWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn DriverEvent(&self, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn QueryColorProfile(&self, hprinter: &PRINTER_HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> windows_core::Result<()>;
    fn FontInstallerDlgProc(&self, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn UpdateExternalFonts(&self, hprinter: &PRINTER_HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUI_Vtbl {
    pub const fn new<Identity: IPrintOemUI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PublishDriverInterface<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piunknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::PublishDriverInterface(this, core::mem::transmute_copy(&piunknown)).into()
            }
        }
        unsafe extern "system" fn CommonUIProp<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::CommonUIProp(this, core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&poemcuipparam)).into()
            }
        }
        unsafe extern "system" fn DocumentPropertySheets<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::DocumentPropertySheets(this, core::mem::transmute_copy(&ppsuiinfo), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn DevicePropertySheets<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::DevicePropertySheets(this, core::mem::transmute_copy(&ppsuiinfo), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn DevQueryPrintEx<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::DevQueryPrintEx(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&pdqpinfo), core::mem::transmute_copy(&ppublicdm), core::mem::transmute_copy(&poemdm)).into()
            }
        }
        unsafe extern "system" fn DeviceCapabilitiesA<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: windows_core::PCWSTR, wcapability: u16, poutput: *mut core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::DeviceCapabilitiesA(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&hprinter), core::mem::transmute(&pdevicename), core::mem::transmute_copy(&wcapability), core::mem::transmute_copy(&poutput), core::mem::transmute_copy(&ppublicdm), core::mem::transmute_copy(&poemdm), core::mem::transmute_copy(&dwold), core::mem::transmute_copy(&dwresult)).into()
            }
        }
        unsafe extern "system" fn UpgradePrinter<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlevel: u32, pdriverupgradeinfo: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::UpgradePrinter(this, core::mem::transmute_copy(&dwlevel), core::mem::transmute_copy(&pdriverupgradeinfo)).into()
            }
        }
        unsafe extern "system" fn PrinterEvent<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintername: windows_core::PCWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::PrinterEvent(this, core::mem::transmute(&pprintername), core::mem::transmute_copy(&idriverevent), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn DriverEvent<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::DriverEvent(this, core::mem::transmute_copy(&dwdriverevent), core::mem::transmute_copy(&dwlevel), core::mem::transmute_copy(&pdriverinfo), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn QueryColorProfile<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: PRINTER_HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::QueryColorProfile(this, core::mem::transmute(&hprinter), core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&ppublicdm), core::mem::transmute_copy(&poemdm), core::mem::transmute_copy(&ulquerymode), core::mem::transmute_copy(&pvprofiledata), core::mem::transmute_copy(&pcbprofiledata), core::mem::transmute_copy(&pflprofiledata)).into()
            }
        }
        unsafe extern "system" fn FontInstallerDlgProc<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::FontInstallerDlgProc(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&usmsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn UpdateExternalFonts<Identity: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: PRINTER_HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI_Impl::UpdateExternalFonts(this, core::mem::transmute(&hprinter), core::mem::transmute_copy(&hheap), core::mem::transmute(&pwstrcartridges)).into()
            }
        }
        Self {
            base__: IPrintOemCommon_Vtbl::new::<Identity, OFFSET>(),
            PublishDriverInterface: PublishDriverInterface::<Identity, OFFSET>,
            CommonUIProp: CommonUIProp::<Identity, OFFSET>,
            DocumentPropertySheets: DocumentPropertySheets::<Identity, OFFSET>,
            DevicePropertySheets: DevicePropertySheets::<Identity, OFFSET>,
            DevQueryPrintEx: DevQueryPrintEx::<Identity, OFFSET>,
            DeviceCapabilitiesA: DeviceCapabilitiesA::<Identity, OFFSET>,
            UpgradePrinter: UpgradePrinter::<Identity, OFFSET>,
            PrinterEvent: PrinterEvent::<Identity, OFFSET>,
            DriverEvent: DriverEvent::<Identity, OFFSET>,
            QueryColorProfile: QueryColorProfile::<Identity, OFFSET>,
            FontInstallerDlgProc: FontInstallerDlgProc::<Identity, OFFSET>,
            UpdateExternalFonts: UpdateExternalFonts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOemUI as windows_core::Interface>::IID || iid == &<IPrintOemCommon as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IPrintOemUI {}
windows_core::imp::define_interface!(IPrintOemUI2, IPrintOemUI2_Vtbl, 0x292515f9_b54b_489b_9275_bab56821395e);
impl core::ops::Deref for IPrintOemUI2 {
    type Target = IPrintOemUI;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintOemUI2, windows_core::IUnknown, IPrintOemCommon, IPrintOemUI);
impl IPrintOemUI2 {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn QueryJobAttributes(&self, hprinter: PRINTER_HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryJobAttributes)(windows_core::Interface::as_raw(self), core::mem::transmute(hprinter), pdevmode, dwlevel, lpattributeinfo).ok() }
    }
    pub unsafe fn HideStandardUI(&self, dwmode: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HideStandardUI)(windows_core::Interface::as_raw(self), dwmode).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn DocumentEvent(&self, hprinter: PRINTER_HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut core::ffi::c_void, cbout: u32, pvout: *mut core::ffi::c_void, piresult: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DocumentEvent)(windows_core::Interface::as_raw(self), core::mem::transmute(hprinter), hdc, iesc, cbin, pvin as _, cbout, pvout as _, piresult as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOemUI2_Vtbl {
    pub base__: IPrintOemUI_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub QueryJobAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, PRINTER_HANDLE, *const super::Gdi::DEVMODEA, u32, *const u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    QueryJobAttributes: usize,
    pub HideStandardUI: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub DocumentEvent: unsafe extern "system" fn(*mut core::ffi::c_void, PRINTER_HANDLE, super::Gdi::HDC, i32, u32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    DocumentEvent: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI2_Impl: IPrintOemUI_Impl {
    fn QueryJobAttributes(&self, hprinter: &PRINTER_HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> windows_core::Result<()>;
    fn HideStandardUI(&self, dwmode: u32) -> windows_core::Result<()>;
    fn DocumentEvent(&self, hprinter: &PRINTER_HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut core::ffi::c_void, cbout: u32, pvout: *mut core::ffi::c_void, piresult: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUI2_Vtbl {
    pub const fn new<Identity: IPrintOemUI2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryJobAttributes<Identity: IPrintOemUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: PRINTER_HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI2_Impl::QueryJobAttributes(this, core::mem::transmute(&hprinter), core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&dwlevel), core::mem::transmute_copy(&lpattributeinfo)).into()
            }
        }
        unsafe extern "system" fn HideStandardUI<Identity: IPrintOemUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI2_Impl::HideStandardUI(this, core::mem::transmute_copy(&dwmode)).into()
            }
        }
        unsafe extern "system" fn DocumentEvent<Identity: IPrintOemUI2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: PRINTER_HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut core::ffi::c_void, cbout: u32, pvout: *mut core::ffi::c_void, piresult: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUI2_Impl::DocumentEvent(this, core::mem::transmute(&hprinter), core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&iesc), core::mem::transmute_copy(&cbin), core::mem::transmute_copy(&pvin), core::mem::transmute_copy(&cbout), core::mem::transmute_copy(&pvout), core::mem::transmute_copy(&piresult)).into()
            }
        }
        Self {
            base__: IPrintOemUI_Vtbl::new::<Identity, OFFSET>(),
            QueryJobAttributes: QueryJobAttributes::<Identity, OFFSET>,
            HideStandardUI: HideStandardUI::<Identity, OFFSET>,
            DocumentEvent: DocumentEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOemUI2 as windows_core::Interface>::IID || iid == &<IPrintOemCommon as windows_core::Interface>::IID || iid == &<IPrintOemUI as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IPrintOemUI2 {}
windows_core::imp::define_interface!(IPrintOemUIMXDC, IPrintOemUIMXDC_Vtbl, 0x7349d725_e2c1_4dca_afb5_c13e91bc9306);
windows_core::imp::interface_hierarchy!(IPrintOemUIMXDC, windows_core::IUnknown);
impl IPrintOemUIMXDC {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AdjustImageableArea(&self, hprinter: PRINTER_HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AdjustImageableArea)(windows_core::Interface::as_raw(self), core::mem::transmute(hprinter), cbdevmode, pdevmode, cboemdm, poemdm, prclimageablearea as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AdjustImageCompression(&self, hprinter: PRINTER_HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, pcompressionmode: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AdjustImageCompression)(windows_core::Interface::as_raw(self), core::mem::transmute(hprinter), cbdevmode, pdevmode, cboemdm, poemdm, pcompressionmode as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AdjustDPI(&self, hprinter: PRINTER_HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, pdpi: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AdjustDPI)(windows_core::Interface::as_raw(self), core::mem::transmute(hprinter), cbdevmode, pdevmode, cboemdm, poemdm, pdpi as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOemUIMXDC_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AdjustImageableArea: unsafe extern "system" fn(*mut core::ffi::c_void, PRINTER_HANDLE, u32, *const super::Gdi::DEVMODEA, u32, *const core::ffi::c_void, *mut super::super::Foundation::RECTL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AdjustImageableArea: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AdjustImageCompression: unsafe extern "system" fn(*mut core::ffi::c_void, PRINTER_HANDLE, u32, *const super::Gdi::DEVMODEA, u32, *const core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AdjustImageCompression: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AdjustDPI: unsafe extern "system" fn(*mut core::ffi::c_void, PRINTER_HANDLE, u32, *const super::Gdi::DEVMODEA, u32, *const core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AdjustDPI: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IPrintOemUIMXDC_Impl: windows_core::IUnknownImpl {
    fn AdjustImageableArea(&self, hprinter: &PRINTER_HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> windows_core::Result<()>;
    fn AdjustImageCompression(&self, hprinter: &PRINTER_HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, pcompressionmode: *mut i32) -> windows_core::Result<()>;
    fn AdjustDPI(&self, hprinter: &PRINTER_HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, pdpi: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IPrintOemUIMXDC_Vtbl {
    pub const fn new<Identity: IPrintOemUIMXDC_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdjustImageableArea<Identity: IPrintOemUIMXDC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: PRINTER_HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUIMXDC_Impl::AdjustImageableArea(this, core::mem::transmute(&hprinter), core::mem::transmute_copy(&cbdevmode), core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cboemdm), core::mem::transmute_copy(&poemdm), core::mem::transmute_copy(&prclimageablearea)).into()
            }
        }
        unsafe extern "system" fn AdjustImageCompression<Identity: IPrintOemUIMXDC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: PRINTER_HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, pcompressionmode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUIMXDC_Impl::AdjustImageCompression(this, core::mem::transmute(&hprinter), core::mem::transmute_copy(&cbdevmode), core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cboemdm), core::mem::transmute_copy(&poemdm), core::mem::transmute_copy(&pcompressionmode)).into()
            }
        }
        unsafe extern "system" fn AdjustDPI<Identity: IPrintOemUIMXDC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: PRINTER_HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, pdpi: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintOemUIMXDC_Impl::AdjustDPI(this, core::mem::transmute(&hprinter), core::mem::transmute_copy(&cbdevmode), core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cboemdm), core::mem::transmute_copy(&poemdm), core::mem::transmute_copy(&pdpi)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdjustImageableArea: AdjustImageableArea::<Identity, OFFSET>,
            AdjustImageCompression: AdjustImageCompression::<Identity, OFFSET>,
            AdjustDPI: AdjustDPI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOemUIMXDC as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IPrintOemUIMXDC {}
windows_core::imp::define_interface!(IPrintPipelineFilter, IPrintPipelineFilter_Vtbl, 0xcdb62fc0_8bed_434e_86fb_a2cae55f19ea);
windows_core::imp::interface_hierarchy!(IPrintPipelineFilter, windows_core::IUnknown);
impl IPrintPipelineFilter {
    pub unsafe fn InitializeFilter<P0, P1, P2>(&self, pinegotiation: P0, pipropertybag: P1, pipipelinecontrol: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IInterFilterCommunicator>,
        P1: windows_core::Param<IPrintPipelinePropertyBag>,
        P2: windows_core::Param<IPrintPipelineManagerControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFilter)(windows_core::Interface::as_raw(self), pinegotiation.param().abi(), pipropertybag.param().abi(), pipipelinecontrol.param().abi()).ok() }
    }
    pub unsafe fn ShutdownOperation(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShutdownOperation)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn StartOperation(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartOperation)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPipelineFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutdownOperation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartOperation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintPipelineFilter_Impl: windows_core::IUnknownImpl {
    fn InitializeFilter(&self, pinegotiation: windows_core::Ref<IInterFilterCommunicator>, pipropertybag: windows_core::Ref<IPrintPipelinePropertyBag>, pipipelinecontrol: windows_core::Ref<IPrintPipelineManagerControl>) -> windows_core::Result<()>;
    fn ShutdownOperation(&self) -> windows_core::Result<()>;
    fn StartOperation(&self) -> windows_core::Result<()>;
}
impl IPrintPipelineFilter_Vtbl {
    pub const fn new<Identity: IPrintPipelineFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeFilter<Identity: IPrintPipelineFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinegotiation: *mut core::ffi::c_void, pipropertybag: *mut core::ffi::c_void, pipipelinecontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPipelineFilter_Impl::InitializeFilter(this, core::mem::transmute_copy(&pinegotiation), core::mem::transmute_copy(&pipropertybag), core::mem::transmute_copy(&pipipelinecontrol)).into()
            }
        }
        unsafe extern "system" fn ShutdownOperation<Identity: IPrintPipelineFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPipelineFilter_Impl::ShutdownOperation(this).into()
            }
        }
        unsafe extern "system" fn StartOperation<Identity: IPrintPipelineFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPipelineFilter_Impl::StartOperation(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFilter: InitializeFilter::<Identity, OFFSET>,
            ShutdownOperation: ShutdownOperation::<Identity, OFFSET>,
            StartOperation: StartOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPipelineFilter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintPipelineFilter {}
windows_core::imp::define_interface!(IPrintPipelineManagerControl, IPrintPipelineManagerControl_Vtbl, 0xaa3e4910_5889_4681_91ef_823ad4ed4e44);
windows_core::imp::interface_hierarchy!(IPrintPipelineManagerControl, windows_core::IUnknown);
impl IPrintPipelineManagerControl {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RequestShutdown<P1>(&self, hrreason: windows_core::HRESULT, preason: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IImgErrorInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).RequestShutdown)(windows_core::Interface::as_raw(self), hrreason, preason.param().abi()).ok() }
    }
    pub unsafe fn FilterFinished(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FilterFinished)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPipelineManagerControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RequestShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RequestShutdown: usize,
    pub FilterFinished: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintPipelineManagerControl_Impl: windows_core::IUnknownImpl {
    fn RequestShutdown(&self, hrreason: windows_core::HRESULT, preason: windows_core::Ref<IImgErrorInfo>) -> windows_core::Result<()>;
    fn FilterFinished(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintPipelineManagerControl_Vtbl {
    pub const fn new<Identity: IPrintPipelineManagerControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestShutdown<Identity: IPrintPipelineManagerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrreason: windows_core::HRESULT, preason: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPipelineManagerControl_Impl::RequestShutdown(this, core::mem::transmute_copy(&hrreason), core::mem::transmute_copy(&preason)).into()
            }
        }
        unsafe extern "system" fn FilterFinished<Identity: IPrintPipelineManagerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPipelineManagerControl_Impl::FilterFinished(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestShutdown: RequestShutdown::<Identity, OFFSET>,
            FilterFinished: FilterFinished::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPipelineManagerControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintPipelineManagerControl {}
windows_core::imp::define_interface!(IPrintPipelineProgressReport, IPrintPipelineProgressReport_Vtbl, 0xedc12c7c_ed40_4ea5_96a6_5e4397497a61);
windows_core::imp::interface_hierarchy!(IPrintPipelineProgressReport, windows_core::IUnknown);
impl IPrintPipelineProgressReport {
    pub unsafe fn ReportProgress(&self, update: EXpsJobConsumption) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportProgress)(windows_core::Interface::as_raw(self), update).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPipelineProgressReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportProgress: unsafe extern "system" fn(*mut core::ffi::c_void, EXpsJobConsumption) -> windows_core::HRESULT,
}
pub trait IPrintPipelineProgressReport_Impl: windows_core::IUnknownImpl {
    fn ReportProgress(&self, update: EXpsJobConsumption) -> windows_core::Result<()>;
}
impl IPrintPipelineProgressReport_Vtbl {
    pub const fn new<Identity: IPrintPipelineProgressReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportProgress<Identity: IPrintPipelineProgressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, update: EXpsJobConsumption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPipelineProgressReport_Impl::ReportProgress(this, core::mem::transmute_copy(&update)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportProgress: ReportProgress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPipelineProgressReport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintPipelineProgressReport {}
windows_core::imp::define_interface!(IPrintPipelinePropertyBag, IPrintPipelinePropertyBag_Vtbl, 0x8b8c99dc_7892_4a95_8a04_57422e9fbb47);
windows_core::imp::interface_hierarchy!(IPrintPipelinePropertyBag, windows_core::IUnknown);
impl IPrintPipelinePropertyBag {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddProperty<P0>(&self, pszname: P0, pvar: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddProperty)(windows_core::Interface::as_raw(self), pszname.param().abi(), core::mem::transmute(pvar)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty<P0>(&self, pszname: P0) -> windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pszname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DeleteProperty<P0>(&self, pszname: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteProperty)(windows_core::Interface::as_raw(self), pszname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPipelinePropertyBag_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    pub DeleteProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::BOOL,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintPipelinePropertyBag_Impl: windows_core::IUnknownImpl {
    fn AddProperty(&self, pszname: &windows_core::PCWSTR, pvar: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetProperty(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DeleteProperty(&self, pszname: &windows_core::PCWSTR) -> windows_core::BOOL;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintPipelinePropertyBag_Vtbl {
    pub const fn new<Identity: IPrintPipelinePropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddProperty<Identity: IPrintPipelinePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pvar: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPipelinePropertyBag_Impl::AddProperty(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&pvar)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IPrintPipelinePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pvar: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintPipelinePropertyBag_Impl::GetProperty(this, core::mem::transmute(&pszname)) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteProperty<Identity: IPrintPipelinePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPipelinePropertyBag_Impl::DeleteProperty(this, core::mem::transmute(&pszname))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddProperty: AddProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPipelinePropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintPipelinePropertyBag {}
windows_core::imp::define_interface!(IPrintPreviewDxgiPackageTarget, IPrintPreviewDxgiPackageTarget_Vtbl, 0x1a6dd0ad_1e2a_4e99_a5ba_91f17818290e);
windows_core::imp::interface_hierarchy!(IPrintPreviewDxgiPackageTarget, windows_core::IUnknown);
impl IPrintPreviewDxgiPackageTarget {
    pub unsafe fn SetJobPageCount(&self, counttype: PageCountType, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJobPageCount)(windows_core::Interface::as_raw(self), counttype, count).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn DrawPage<P1>(&self, jobpagenumber: u32, pageimage: P1, dpix: f32, dpiy: f32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Dxgi::IDXGISurface>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawPage)(windows_core::Interface::as_raw(self), jobpagenumber, pageimage.param().abi(), dpix, dpiy).ok() }
    }
    pub unsafe fn InvalidatePreview(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InvalidatePreview)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPreviewDxgiPackageTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetJobPageCount: unsafe extern "system" fn(*mut core::ffi::c_void, PageCountType, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub DrawPage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    DrawPage: usize,
    pub InvalidatePreview: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait IPrintPreviewDxgiPackageTarget_Impl: windows_core::IUnknownImpl {
    fn SetJobPageCount(&self, counttype: PageCountType, count: u32) -> windows_core::Result<()>;
    fn DrawPage(&self, jobpagenumber: u32, pageimage: windows_core::Ref<super::Dxgi::IDXGISurface>, dpix: f32, dpiy: f32) -> windows_core::Result<()>;
    fn InvalidatePreview(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl IPrintPreviewDxgiPackageTarget_Vtbl {
    pub const fn new<Identity: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetJobPageCount<Identity: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, counttype: PageCountType, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPreviewDxgiPackageTarget_Impl::SetJobPageCount(this, core::mem::transmute_copy(&counttype), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn DrawPage<Identity: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobpagenumber: u32, pageimage: *mut core::ffi::c_void, dpix: f32, dpiy: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPreviewDxgiPackageTarget_Impl::DrawPage(this, core::mem::transmute_copy(&jobpagenumber), core::mem::transmute_copy(&pageimage), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy)).into()
            }
        }
        unsafe extern "system" fn InvalidatePreview<Identity: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPreviewDxgiPackageTarget_Impl::InvalidatePreview(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetJobPageCount: SetJobPageCount::<Identity, OFFSET>,
            DrawPage: DrawPage::<Identity, OFFSET>,
            InvalidatePreview: InvalidatePreview::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPreviewDxgiPackageTarget as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl windows_core::RuntimeName for IPrintPreviewDxgiPackageTarget {}
windows_core::imp::define_interface!(IPrintReadStream, IPrintReadStream_Vtbl, 0x4d47a67c_66cc_4430_850e_daf466fe5bc4);
windows_core::imp::interface_hierarchy!(IPrintReadStream, windows_core::IUnknown);
impl IPrintReadStream {
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: u32, plibnewposition: Option<*mut u64>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), dlibmove, dworigin, plibnewposition.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ReadBytes(&self, pvbuffer: *mut core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadBytes)(windows_core::Interface::as_raw(self), pvbuffer as _, cbrequested, pcbread as _, pbendoffile as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintReadStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32, *mut u64) -> windows_core::HRESULT,
    pub ReadBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IPrintReadStream_Impl: windows_core::IUnknownImpl {
    fn Seek(&self, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> windows_core::Result<()>;
    fn ReadBytes(&self, pvbuffer: *mut core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl IPrintReadStream_Vtbl {
    pub const fn new<Identity: IPrintReadStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Seek<Identity: IPrintReadStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintReadStream_Impl::Seek(this, core::mem::transmute_copy(&dlibmove), core::mem::transmute_copy(&dworigin), core::mem::transmute_copy(&plibnewposition)).into()
            }
        }
        unsafe extern "system" fn ReadBytes<Identity: IPrintReadStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbuffer: *mut core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintReadStream_Impl::ReadBytes(this, core::mem::transmute_copy(&pvbuffer), core::mem::transmute_copy(&cbrequested), core::mem::transmute_copy(&pcbread), core::mem::transmute_copy(&pbendoffile)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Seek: Seek::<Identity, OFFSET>, ReadBytes: ReadBytes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintReadStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintReadStream {}
windows_core::imp::define_interface!(IPrintReadStreamFactory, IPrintReadStreamFactory_Vtbl, 0xacb971e3_df8d_4fc2_bee6_0609d15f3cf9);
windows_core::imp::interface_hierarchy!(IPrintReadStreamFactory, windows_core::IUnknown);
impl IPrintReadStreamFactory {
    pub unsafe fn GetStream(&self) -> windows_core::Result<IPrintReadStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintReadStreamFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintReadStreamFactory_Impl: windows_core::IUnknownImpl {
    fn GetStream(&self) -> windows_core::Result<IPrintReadStream>;
}
impl IPrintReadStreamFactory_Vtbl {
    pub const fn new<Identity: IPrintReadStreamFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStream<Identity: IPrintReadStreamFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintReadStreamFactory_Impl::GetStream(this) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetStream: GetStream::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintReadStreamFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintReadStreamFactory {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaAsyncOperation, IPrintSchemaAsyncOperation_Vtbl, 0x143c8dcb_d37f_47f7_88e8_6b1d21f2c5f7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaAsyncOperation {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaAsyncOperation, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaAsyncOperation {
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaAsyncOperation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaAsyncOperation_Impl: super::super::System::Com::IDispatch_Impl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaAsyncOperation_Vtbl {
    pub const fn new<Identity: IPrintSchemaAsyncOperation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IPrintSchemaAsyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintSchemaAsyncOperation_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPrintSchemaAsyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintSchemaAsyncOperation_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaAsyncOperation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaAsyncOperation {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaAsyncOperationEvent, IPrintSchemaAsyncOperationEvent_Vtbl, 0x23adbb16_0133_4906_b29a_1dce1d026379);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaAsyncOperationEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaAsyncOperationEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaAsyncOperationEvent {
    pub unsafe fn Completed<P0>(&self, pticket: P0, hroperation: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintSchemaTicket>,
    {
        unsafe { (windows_core::Interface::vtable(self).Completed)(windows_core::Interface::as_raw(self), pticket.param().abi(), hroperation).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaAsyncOperationEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaAsyncOperationEvent_Impl: super::super::System::Com::IDispatch_Impl {
    fn Completed(&self, pticket: windows_core::Ref<IPrintSchemaTicket>, hroperation: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaAsyncOperationEvent_Vtbl {
    pub const fn new<Identity: IPrintSchemaAsyncOperationEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Completed<Identity: IPrintSchemaAsyncOperationEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pticket: *mut core::ffi::c_void, hroperation: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintSchemaAsyncOperationEvent_Impl::Completed(this, core::mem::transmute_copy(&pticket), core::mem::transmute_copy(&hroperation)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Completed: Completed::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaAsyncOperationEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaAsyncOperationEvent {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaCapabilities, IPrintSchemaCapabilities_Vtbl, 0x5a577640_501d_4927_bcd0_5ef57a7ed175);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaCapabilities {
    type Target = IPrintSchemaElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaCapabilities, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaCapabilities {
    pub unsafe fn GetFeatureByKeyName(&self, bstrkeyname: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFeatureByKeyName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrkeyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFeature(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFeature)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrnamespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PageImageableSize(&self) -> windows_core::Result<IPrintSchemaPageImageableSize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PageImageableSize)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn JobCopiesAllDocumentsMinValue(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JobCopiesAllDocumentsMinValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn JobCopiesAllDocumentsMaxValue(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JobCopiesAllDocumentsMaxValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSelectedOptionInPrintTicket<P0>(&self, pfeature: P0) -> windows_core::Result<IPrintSchemaOption>
    where
        P0: windows_core::Param<IPrintSchemaFeature>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelectedOptionInPrintTicket)(windows_core::Interface::as_raw(self), pfeature.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOptions<P0>(&self, pfeature: P0) -> windows_core::Result<IPrintSchemaOptionCollection>
    where
        P0: windows_core::Param<IPrintSchemaFeature>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), pfeature.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaCapabilities_Vtbl {
    pub base__: IPrintSchemaElement_Vtbl,
    pub GetFeatureByKeyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageImageableSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobCopiesAllDocumentsMinValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub JobCopiesAllDocumentsMaxValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSelectedOptionInPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaCapabilities_Impl: IPrintSchemaElement_Impl {
    fn GetFeatureByKeyName(&self, bstrkeyname: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature>;
    fn GetFeature(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature>;
    fn PageImageableSize(&self) -> windows_core::Result<IPrintSchemaPageImageableSize>;
    fn JobCopiesAllDocumentsMinValue(&self) -> windows_core::Result<u32>;
    fn JobCopiesAllDocumentsMaxValue(&self) -> windows_core::Result<u32>;
    fn GetSelectedOptionInPrintTicket(&self, pfeature: windows_core::Ref<IPrintSchemaFeature>) -> windows_core::Result<IPrintSchemaOption>;
    fn GetOptions(&self, pfeature: windows_core::Ref<IPrintSchemaFeature>) -> windows_core::Result<IPrintSchemaOptionCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaCapabilities_Vtbl {
    pub const fn new<Identity: IPrintSchemaCapabilities_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFeatureByKeyName<Identity: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrkeyname: *mut core::ffi::c_void, ppfeature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaCapabilities_Impl::GetFeatureByKeyName(this, core::mem::transmute(&bstrkeyname)) {
                    Ok(ok__) => {
                        ppfeature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFeature<Identity: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrnamespaceuri: *mut core::ffi::c_void, ppfeature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaCapabilities_Impl::GetFeature(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                    Ok(ok__) => {
                        ppfeature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PageImageableSize<Identity: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppageimageablesize: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaCapabilities_Impl::PageImageableSize(this) {
                    Ok(ok__) => {
                        pppageimageablesize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMinValue<Identity: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puljobcopiesalldocumentsminvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaCapabilities_Impl::JobCopiesAllDocumentsMinValue(this) {
                    Ok(ok__) => {
                        puljobcopiesalldocumentsminvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMaxValue<Identity: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puljobcopiesalldocumentsmaxvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaCapabilities_Impl::JobCopiesAllDocumentsMaxValue(this) {
                    Ok(ok__) => {
                        puljobcopiesalldocumentsmaxvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelectedOptionInPrintTicket<Identity: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfeature: *mut core::ffi::c_void, ppoption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaCapabilities_Impl::GetSelectedOptionInPrintTicket(this, core::mem::transmute_copy(&pfeature)) {
                    Ok(ok__) => {
                        ppoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOptions<Identity: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfeature: *mut core::ffi::c_void, ppoptioncollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaCapabilities_Impl::GetOptions(this, core::mem::transmute_copy(&pfeature)) {
                    Ok(ok__) => {
                        ppoptioncollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPrintSchemaElement_Vtbl::new::<Identity, OFFSET>(),
            GetFeatureByKeyName: GetFeatureByKeyName::<Identity, OFFSET>,
            GetFeature: GetFeature::<Identity, OFFSET>,
            PageImageableSize: PageImageableSize::<Identity, OFFSET>,
            JobCopiesAllDocumentsMinValue: JobCopiesAllDocumentsMinValue::<Identity, OFFSET>,
            JobCopiesAllDocumentsMaxValue: JobCopiesAllDocumentsMaxValue::<Identity, OFFSET>,
            GetSelectedOptionInPrintTicket: GetSelectedOptionInPrintTicket::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaCapabilities as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaCapabilities {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaCapabilities2, IPrintSchemaCapabilities2_Vtbl, 0xb58845f4_9970_4d87_a636_169fb82ed642);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaCapabilities2 {
    type Target = IPrintSchemaCapabilities;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaCapabilities2, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaCapabilities);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaCapabilities2 {
    pub unsafe fn GetParameterDefinition(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaParameterDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParameterDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrnamespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaCapabilities2_Vtbl {
    pub base__: IPrintSchemaCapabilities_Vtbl,
    pub GetParameterDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaCapabilities2_Impl: IPrintSchemaCapabilities_Impl {
    fn GetParameterDefinition(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaParameterDefinition>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaCapabilities2_Vtbl {
    pub const fn new<Identity: IPrintSchemaCapabilities2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParameterDefinition<Identity: IPrintSchemaCapabilities2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrnamespaceuri: *mut core::ffi::c_void, ppparameterdefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaCapabilities2_Impl::GetParameterDefinition(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                    Ok(ok__) => {
                        ppparameterdefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IPrintSchemaCapabilities_Vtbl::new::<Identity, OFFSET>(), GetParameterDefinition: GetParameterDefinition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaCapabilities2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaCapabilities2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaDisplayableElement, IPrintSchemaDisplayableElement_Vtbl, 0xaf45af49_d6aa_407d_bf87_3912236e9d94);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaDisplayableElement {
    type Target = IPrintSchemaElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaDisplayableElement, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaDisplayableElement {
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaDisplayableElement_Vtbl {
    pub base__: IPrintSchemaElement_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaDisplayableElement_Impl: IPrintSchemaElement_Impl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaDisplayableElement_Vtbl {
    pub const fn new<Identity: IPrintSchemaDisplayableElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisplayName<Identity: IPrintSchemaDisplayableElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaDisplayableElement_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        pbstrdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IPrintSchemaElement_Vtbl::new::<Identity, OFFSET>(), DisplayName: DisplayName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaDisplayableElement {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaElement, IPrintSchemaElement_Vtbl, 0x724c1646_e64b_4bbf_8eb4_d45e4fd580da);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaElement {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaElement, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaElement {
    pub unsafe fn XmlNode(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XmlNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn NamespaceUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NamespaceUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaElement_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub XmlNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaElement_Impl: super::super::System::Com::IDispatch_Impl {
    fn XmlNode(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn NamespaceUri(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaElement_Vtbl {
    pub const fn new<Identity: IPrintSchemaElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn XmlNode<Identity: IPrintSchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppxmlnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaElement_Impl::XmlNode(this) {
                    Ok(ok__) => {
                        ppxmlnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IPrintSchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaElement_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NamespaceUri<Identity: IPrintSchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnamespaceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaElement_Impl::NamespaceUri(this) {
                    Ok(ok__) => {
                        pbstrnamespaceuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            XmlNode: XmlNode::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            NamespaceUri: NamespaceUri::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaElement {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaFeature, IPrintSchemaFeature_Vtbl, 0xef189461_5d62_4626_8e57_ff83583c4826);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaFeature {
    type Target = IPrintSchemaDisplayableElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaFeature, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaDisplayableElement);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaFeature {
    pub unsafe fn SelectedOption(&self) -> windows_core::Result<IPrintSchemaOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedOption)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSelectedOption<P0>(&self, poption: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintSchemaOption>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedOption)(windows_core::Interface::as_raw(self), poption.param().abi()).ok() }
    }
    pub unsafe fn SelectionType(&self) -> windows_core::Result<PrintSchemaSelectionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOption(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOption)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrnamespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DisplayUI(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayUI)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaFeature_Vtbl {
    pub base__: IPrintSchemaDisplayableElement_Vtbl,
    pub SelectedOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSelectedOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintSchemaSelectionType) -> windows_core::HRESULT,
    pub GetOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaFeature_Impl: IPrintSchemaDisplayableElement_Impl {
    fn SelectedOption(&self) -> windows_core::Result<IPrintSchemaOption>;
    fn SetSelectedOption(&self, poption: windows_core::Ref<IPrintSchemaOption>) -> windows_core::Result<()>;
    fn SelectionType(&self) -> windows_core::Result<PrintSchemaSelectionType>;
    fn GetOption(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaOption>;
    fn DisplayUI(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaFeature_Vtbl {
    pub const fn new<Identity: IPrintSchemaFeature_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SelectedOption<Identity: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaFeature_Impl::SelectedOption(this) {
                    Ok(ok__) => {
                        ppoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelectedOption<Identity: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poption: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintSchemaFeature_Impl::SetSelectedOption(this, core::mem::transmute_copy(&poption)).into()
            }
        }
        unsafe extern "system" fn SelectionType<Identity: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pselectiontype: *mut PrintSchemaSelectionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaFeature_Impl::SelectionType(this) {
                    Ok(ok__) => {
                        pselectiontype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOption<Identity: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrnamespaceuri: *mut core::ffi::c_void, ppoption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaFeature_Impl::GetOption(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                    Ok(ok__) => {
                        ppoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbshow: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaFeature_Impl::DisplayUI(this) {
                    Ok(ok__) => {
                        pbshow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, OFFSET>(),
            SelectedOption: SelectedOption::<Identity, OFFSET>,
            SetSelectedOption: SetSelectedOption::<Identity, OFFSET>,
            SelectionType: SelectionType::<Identity, OFFSET>,
            GetOption: GetOption::<Identity, OFFSET>,
            DisplayUI: DisplayUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaFeature as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaFeature {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaNUpOption, IPrintSchemaNUpOption_Vtbl, 0x1f6342f2_d848_42e3_8995_c10a9ef9a3ba);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaNUpOption {
    type Target = IPrintSchemaOption;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaNUpOption, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaDisplayableElement, IPrintSchemaOption);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaNUpOption {
    pub unsafe fn PagesPerSheet(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PagesPerSheet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaNUpOption_Vtbl {
    pub base__: IPrintSchemaOption_Vtbl,
    pub PagesPerSheet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaNUpOption_Impl: IPrintSchemaOption_Impl {
    fn PagesPerSheet(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaNUpOption_Vtbl {
    pub const fn new<Identity: IPrintSchemaNUpOption_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PagesPerSheet<Identity: IPrintSchemaNUpOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpagespersheet: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaNUpOption_Impl::PagesPerSheet(this) {
                    Ok(ok__) => {
                        pulpagespersheet.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IPrintSchemaOption_Vtbl::new::<Identity, OFFSET>(), PagesPerSheet: PagesPerSheet::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaNUpOption as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID || iid == &<IPrintSchemaOption as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaNUpOption {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaOption, IPrintSchemaOption_Vtbl, 0x66bb2f51_5844_4997_8d70_4b7cc221cf92);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaOption {
    type Target = IPrintSchemaDisplayableElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaOption, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaDisplayableElement);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaOption {
    pub unsafe fn Selected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Selected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Constrained(&self) -> windows_core::Result<PrintSchemaConstrainedSetting> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Constrained)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPropertyValue(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrnamespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaOption_Vtbl {
    pub base__: IPrintSchemaDisplayableElement_Vtbl,
    pub Selected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Constrained: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintSchemaConstrainedSetting) -> windows_core::HRESULT,
    pub GetPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaOption_Impl: IPrintSchemaDisplayableElement_Impl {
    fn Selected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Constrained(&self) -> windows_core::Result<PrintSchemaConstrainedSetting>;
    fn GetPropertyValue(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaOption_Vtbl {
    pub const fn new<Identity: IPrintSchemaOption_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Selected<Identity: IPrintSchemaOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisselected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaOption_Impl::Selected(this) {
                    Ok(ok__) => {
                        pbisselected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Constrained<Identity: IPrintSchemaOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psetting: *mut PrintSchemaConstrainedSetting) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaOption_Impl::Constrained(this) {
                    Ok(ok__) => {
                        psetting.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyValue<Identity: IPrintSchemaOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrnamespaceuri: *mut core::ffi::c_void, ppxmlvaluenode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaOption_Impl::GetPropertyValue(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                    Ok(ok__) => {
                        ppxmlvaluenode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, OFFSET>(),
            Selected: Selected::<Identity, OFFSET>,
            Constrained: Constrained::<Identity, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaOption as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaOption {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaOptionCollection, IPrintSchemaOptionCollection_Vtbl, 0xbaecb0bd_a946_4771_bc30_e8b24f8d45c1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaOptionCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaOptionCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaOptionCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, ulindex: u32) -> windows_core::Result<IPrintSchemaOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), ulindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaOptionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaOptionCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, ulindex: u32) -> windows_core::Result<IPrintSchemaOption>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaOptionCollection_Vtbl {
    pub const fn new<Identity: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaOptionCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pulcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppoption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaOptionCollection_Impl::GetAt(this, core::mem::transmute_copy(&ulindex)) {
                    Ok(ok__) => {
                        ppoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaOptionCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaOptionCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaOptionCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaPageImageableSize, IPrintSchemaPageImageableSize_Vtbl, 0x7c85bf5e_dc7c_4f61_839b_4107e1c9b68e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaPageImageableSize {
    type Target = IPrintSchemaElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaPageImageableSize, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaPageImageableSize {
    pub unsafe fn ImageableSizeWidthInMicrons(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImageableSizeWidthInMicrons)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ImageableSizeHeightInMicrons(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImageableSizeHeightInMicrons)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OriginWidthInMicrons(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OriginWidthInMicrons)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OriginHeightInMicrons(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OriginHeightInMicrons)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExtentWidthInMicrons(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtentWidthInMicrons)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExtentHeightInMicrons(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtentHeightInMicrons)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaPageImageableSize_Vtbl {
    pub base__: IPrintSchemaElement_Vtbl,
    pub ImageableSizeWidthInMicrons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ImageableSizeHeightInMicrons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OriginWidthInMicrons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OriginHeightInMicrons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExtentWidthInMicrons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExtentHeightInMicrons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaPageImageableSize_Impl: IPrintSchemaElement_Impl {
    fn ImageableSizeWidthInMicrons(&self) -> windows_core::Result<u32>;
    fn ImageableSizeHeightInMicrons(&self) -> windows_core::Result<u32>;
    fn OriginWidthInMicrons(&self) -> windows_core::Result<u32>;
    fn OriginHeightInMicrons(&self) -> windows_core::Result<u32>;
    fn ExtentWidthInMicrons(&self) -> windows_core::Result<u32>;
    fn ExtentHeightInMicrons(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaPageImageableSize_Vtbl {
    pub const fn new<Identity: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ImageableSizeWidthInMicrons<Identity: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulimageablesizewidth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaPageImageableSize_Impl::ImageableSizeWidthInMicrons(this) {
                    Ok(ok__) => {
                        pulimageablesizewidth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImageableSizeHeightInMicrons<Identity: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulimageablesizeheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaPageImageableSize_Impl::ImageableSizeHeightInMicrons(this) {
                    Ok(ok__) => {
                        pulimageablesizeheight.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OriginWidthInMicrons<Identity: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puloriginwidth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaPageImageableSize_Impl::OriginWidthInMicrons(this) {
                    Ok(ok__) => {
                        puloriginwidth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OriginHeightInMicrons<Identity: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puloriginheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaPageImageableSize_Impl::OriginHeightInMicrons(this) {
                    Ok(ok__) => {
                        puloriginheight.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtentWidthInMicrons<Identity: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulextentwidth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaPageImageableSize_Impl::ExtentWidthInMicrons(this) {
                    Ok(ok__) => {
                        pulextentwidth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtentHeightInMicrons<Identity: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulextentheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaPageImageableSize_Impl::ExtentHeightInMicrons(this) {
                    Ok(ok__) => {
                        pulextentheight.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPrintSchemaElement_Vtbl::new::<Identity, OFFSET>(),
            ImageableSizeWidthInMicrons: ImageableSizeWidthInMicrons::<Identity, OFFSET>,
            ImageableSizeHeightInMicrons: ImageableSizeHeightInMicrons::<Identity, OFFSET>,
            OriginWidthInMicrons: OriginWidthInMicrons::<Identity, OFFSET>,
            OriginHeightInMicrons: OriginHeightInMicrons::<Identity, OFFSET>,
            ExtentWidthInMicrons: ExtentWidthInMicrons::<Identity, OFFSET>,
            ExtentHeightInMicrons: ExtentHeightInMicrons::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaPageImageableSize as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaPageImageableSize {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaPageMediaSizeOption, IPrintSchemaPageMediaSizeOption_Vtbl, 0x68746729_f493_4830_a10f_69028774605d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaPageMediaSizeOption {
    type Target = IPrintSchemaOption;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaPageMediaSizeOption, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaDisplayableElement, IPrintSchemaOption);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaPageMediaSizeOption {
    pub unsafe fn WidthInMicrons(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidthInMicrons)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HeightInMicrons(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HeightInMicrons)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaPageMediaSizeOption_Vtbl {
    pub base__: IPrintSchemaOption_Vtbl,
    pub WidthInMicrons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub HeightInMicrons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaPageMediaSizeOption_Impl: IPrintSchemaOption_Impl {
    fn WidthInMicrons(&self) -> windows_core::Result<u32>;
    fn HeightInMicrons(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaPageMediaSizeOption_Vtbl {
    pub const fn new<Identity: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WidthInMicrons<Identity: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulwidth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaPageMediaSizeOption_Impl::WidthInMicrons(this) {
                    Ok(ok__) => {
                        pulwidth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HeightInMicrons<Identity: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaPageMediaSizeOption_Impl::HeightInMicrons(this) {
                    Ok(ok__) => {
                        pulheight.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPrintSchemaOption_Vtbl::new::<Identity, OFFSET>(),
            WidthInMicrons: WidthInMicrons::<Identity, OFFSET>,
            HeightInMicrons: HeightInMicrons::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaPageMediaSizeOption as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID || iid == &<IPrintSchemaOption as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaPageMediaSizeOption {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaParameterDefinition, IPrintSchemaParameterDefinition_Vtbl, 0xb5ade81e_0e61_4fe1_81c6_c333e4ffe0f1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaParameterDefinition {
    type Target = IPrintSchemaDisplayableElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaParameterDefinition, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaDisplayableElement);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaParameterDefinition {
    pub unsafe fn UserInputRequired(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserInputRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnitType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnitType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DataType(&self) -> windows_core::Result<PrintSchemaParameterDataType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RangeMin(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeMin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RangeMax(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaParameterDefinition_Vtbl {
    pub base__: IPrintSchemaDisplayableElement_Vtbl,
    pub UserInputRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub UnitType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrintSchemaParameterDataType) -> windows_core::HRESULT,
    pub RangeMin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RangeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaParameterDefinition_Impl: IPrintSchemaDisplayableElement_Impl {
    fn UserInputRequired(&self) -> windows_core::Result<windows_core::BOOL>;
    fn UnitType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DataType(&self) -> windows_core::Result<PrintSchemaParameterDataType>;
    fn RangeMin(&self) -> windows_core::Result<i32>;
    fn RangeMax(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaParameterDefinition_Vtbl {
    pub const fn new<Identity: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UserInputRequired<Identity: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisrequired: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaParameterDefinition_Impl::UserInputRequired(this) {
                    Ok(ok__) => {
                        pbisrequired.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnitType<Identity: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrunittype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaParameterDefinition_Impl::UnitType(this) {
                    Ok(ok__) => {
                        pbstrunittype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataType<Identity: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatatype: *mut PrintSchemaParameterDataType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaParameterDefinition_Impl::DataType(this) {
                    Ok(ok__) => {
                        pdatatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RangeMin<Identity: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangemin: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaParameterDefinition_Impl::RangeMin(this) {
                    Ok(ok__) => {
                        prangemin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RangeMax<Identity: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangemax: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaParameterDefinition_Impl::RangeMax(this) {
                    Ok(ok__) => {
                        prangemax.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, OFFSET>(),
            UserInputRequired: UserInputRequired::<Identity, OFFSET>,
            UnitType: UnitType::<Identity, OFFSET>,
            DataType: DataType::<Identity, OFFSET>,
            RangeMin: RangeMin::<Identity, OFFSET>,
            RangeMax: RangeMax::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaParameterDefinition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaParameterDefinition {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaParameterInitializer, IPrintSchemaParameterInitializer_Vtbl, 0x52027082_0b74_4648_9564_828cc6cb656c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaParameterInitializer {
    type Target = IPrintSchemaElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaParameterInitializer, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaParameterInitializer {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, pvar: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaParameterInitializer_Vtbl {
    pub base__: IPrintSchemaElement_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaParameterInitializer_Impl: IPrintSchemaElement_Impl {
    fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValue(&self, pvar: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaParameterInitializer_Vtbl {
    pub const fn new<Identity: IPrintSchemaParameterInitializer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: IPrintSchemaParameterInitializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaParameterInitializer_Impl::Value(this) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IPrintSchemaParameterInitializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintSchemaParameterInitializer_Impl::SetValue(this, core::mem::transmute_copy(&pvar)).into()
            }
        }
        Self { base__: IPrintSchemaElement_Vtbl::new::<Identity, OFFSET>(), Value: Value::<Identity, OFFSET>, SetValue: SetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaParameterInitializer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaParameterInitializer {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaTicket, IPrintSchemaTicket_Vtbl, 0xe480b861_4708_4e6d_a5b4_a2b4eeb9baa4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaTicket {
    type Target = IPrintSchemaElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaTicket, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaTicket {
    pub unsafe fn GetFeatureByKeyName(&self, bstrkeyname: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFeatureByKeyName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrkeyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFeature(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFeature)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrnamespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ValidateAsync(&self) -> windows_core::Result<IPrintSchemaAsyncOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidateAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommitAsync<P0>(&self, pprintticketcommit: P0) -> windows_core::Result<IPrintSchemaAsyncOperation>
    where
        P0: windows_core::Param<IPrintSchemaTicket>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitAsync)(windows_core::Interface::as_raw(self), pprintticketcommit.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NotifyXmlChanged(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyXmlChanged)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<IPrintSchemaCapabilities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn JobCopiesAllDocuments(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JobCopiesAllDocuments)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJobCopiesAllDocuments(&self, uljobcopiesalldocuments: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJobCopiesAllDocuments)(windows_core::Interface::as_raw(self), uljobcopiesalldocuments).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaTicket_Vtbl {
    pub base__: IPrintSchemaElement_Vtbl,
    pub GetFeatureByKeyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValidateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommitAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyXmlChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobCopiesAllDocuments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetJobCopiesAllDocuments: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaTicket_Impl: IPrintSchemaElement_Impl {
    fn GetFeatureByKeyName(&self, bstrkeyname: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature>;
    fn GetFeature(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature>;
    fn ValidateAsync(&self) -> windows_core::Result<IPrintSchemaAsyncOperation>;
    fn CommitAsync(&self, pprintticketcommit: windows_core::Ref<IPrintSchemaTicket>) -> windows_core::Result<IPrintSchemaAsyncOperation>;
    fn NotifyXmlChanged(&self) -> windows_core::Result<()>;
    fn GetCapabilities(&self) -> windows_core::Result<IPrintSchemaCapabilities>;
    fn JobCopiesAllDocuments(&self) -> windows_core::Result<u32>;
    fn SetJobCopiesAllDocuments(&self, uljobcopiesalldocuments: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaTicket_Vtbl {
    pub const fn new<Identity: IPrintSchemaTicket_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFeatureByKeyName<Identity: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrkeyname: *mut core::ffi::c_void, ppfeature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaTicket_Impl::GetFeatureByKeyName(this, core::mem::transmute(&bstrkeyname)) {
                    Ok(ok__) => {
                        ppfeature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFeature<Identity: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrnamespaceuri: *mut core::ffi::c_void, ppfeature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaTicket_Impl::GetFeature(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                    Ok(ok__) => {
                        ppfeature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValidateAsync<Identity: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaTicket_Impl::ValidateAsync(this) {
                    Ok(ok__) => {
                        ppasyncoperation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommitAsync<Identity: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticketcommit: *mut core::ffi::c_void, ppasyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaTicket_Impl::CommitAsync(this, core::mem::transmute_copy(&pprintticketcommit)) {
                    Ok(ok__) => {
                        ppasyncoperation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NotifyXmlChanged<Identity: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintSchemaTicket_Impl::NotifyXmlChanged(this).into()
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaTicket_Impl::GetCapabilities(this) {
                    Ok(ok__) => {
                        ppcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn JobCopiesAllDocuments<Identity: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puljobcopiesalldocuments: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaTicket_Impl::JobCopiesAllDocuments(this) {
                    Ok(ok__) => {
                        puljobcopiesalldocuments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJobCopiesAllDocuments<Identity: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uljobcopiesalldocuments: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintSchemaTicket_Impl::SetJobCopiesAllDocuments(this, core::mem::transmute_copy(&uljobcopiesalldocuments)).into()
            }
        }
        Self {
            base__: IPrintSchemaElement_Vtbl::new::<Identity, OFFSET>(),
            GetFeatureByKeyName: GetFeatureByKeyName::<Identity, OFFSET>,
            GetFeature: GetFeature::<Identity, OFFSET>,
            ValidateAsync: ValidateAsync::<Identity, OFFSET>,
            CommitAsync: CommitAsync::<Identity, OFFSET>,
            NotifyXmlChanged: NotifyXmlChanged::<Identity, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            JobCopiesAllDocuments: JobCopiesAllDocuments::<Identity, OFFSET>,
            SetJobCopiesAllDocuments: SetJobCopiesAllDocuments::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaTicket as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaTicket {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrintSchemaTicket2, IPrintSchemaTicket2_Vtbl, 0x2ec1f844_766a_47a1_91f4_2eeb6190f80c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrintSchemaTicket2 {
    type Target = IPrintSchemaTicket;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrintSchemaTicket2, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaTicket);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaTicket2 {
    pub unsafe fn GetParameterInitializer(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaParameterInitializer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParameterInitializer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrnamespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaTicket2_Vtbl {
    pub base__: IPrintSchemaTicket_Vtbl,
    pub GetParameterInitializer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaTicket2_Impl: IPrintSchemaTicket_Impl {
    fn GetParameterInitializer(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaParameterInitializer>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrintSchemaTicket2_Vtbl {
    pub const fn new<Identity: IPrintSchemaTicket2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParameterInitializer<Identity: IPrintSchemaTicket2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrnamespaceuri: *mut core::ffi::c_void, ppparameterinitializer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintSchemaTicket2_Impl::GetParameterInitializer(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                    Ok(ok__) => {
                        ppparameterinitializer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IPrintSchemaTicket_Vtbl::new::<Identity, OFFSET>(), GetParameterInitializer: GetParameterInitializer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaTicket2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaTicket as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrintSchemaTicket2 {}
windows_core::imp::define_interface!(IPrintTicketProvider, IPrintTicketProvider_Vtbl, 0xbb5116db_0a23_4c3a_a6b6_89e5558dfb5d);
windows_core::imp::interface_hierarchy!(IPrintTicketProvider, windows_core::IUnknown);
impl IPrintTicketProvider {
    pub unsafe fn GetSupportedVersions(&self, hprinter: PRINTER_HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedVersions)(windows_core::Interface::as_raw(self), core::mem::transmute(hprinter), ppversions as _, cversions as _).ok() }
    }
    pub unsafe fn BindPrinter(&self, hprinter: PRINTER_HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BindPrinter)(windows_core::Interface::as_raw(self), core::mem::transmute(hprinter), version, poptions as _, pdevmodeflags as _, cnamespaces as _, ppnamespaces as _).ok() }
    }
    pub unsafe fn QueryDeviceNamespace(&self, pdefaultnamespace: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryDeviceNamespace)(windows_core::Interface::as_raw(self), core::mem::transmute(pdefaultnamespace)).ok() }
    }
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn ConvertPrintTicketToDevMode<P0>(&self, pprintticket: P0, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Data::Xml::MsXml::IXMLDOMDocument2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertPrintTicketToDevMode)(windows_core::Interface::as_raw(self), pprintticket.param().abi(), cbdevmodein, pdevmodein as _, pcbdevmodeout as _, ppdevmodeout as _).ok() }
    }
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn ConvertDevModeToPrintTicket<P2>(&self, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::super::Data::Xml::MsXml::IXMLDOMDocument2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertDevModeToPrintTicket)(windows_core::Interface::as_raw(self), cbdevmode, pdevmode as _, pprintticket.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetPrintCapabilities<P0>(&self, pprintticket: P0) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>
    where
        P0: windows_core::Param<super::super::Data::Xml::MsXml::IXMLDOMDocument2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrintCapabilities)(windows_core::Interface::as_raw(self), pprintticket.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn ValidatePrintTicket<P0>(&self, pbaseticket: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Data::Xml::MsXml::IXMLDOMDocument2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ValidatePrintTicket)(windows_core::Interface::as_raw(self), pbaseticket.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSupportedVersions: unsafe extern "system" fn(*mut core::ffi::c_void, PRINTER_HANDLE, *mut *mut i32, *mut i32) -> windows_core::HRESULT,
    pub BindPrinter: unsafe extern "system" fn(*mut core::ffi::c_void, PRINTER_HANDLE, i32, *mut SHIMOPTS, *mut u32, *mut i32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryDeviceNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub ConvertPrintTicketToDevMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::Gdi::DEVMODEA, *mut u32, *mut *mut super::Gdi::DEVMODEA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    ConvertPrintTicketToDevMode: usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub ConvertDevModeToPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::Gdi::DEVMODEA, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    ConvertDevModeToPrintTicket: usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetPrintCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetPrintCapabilities: usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub ValidatePrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    ValidatePrintTicket: usize,
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider_Impl: windows_core::IUnknownImpl {
    fn GetSupportedVersions(&self, hprinter: &PRINTER_HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> windows_core::Result<()>;
    fn BindPrinter(&self, hprinter: &PRINTER_HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryDeviceNamespace(&self, pdefaultnamespace: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ConvertPrintTicketToDevMode(&self, pprintticket: windows_core::Ref<super::super::Data::Xml::MsXml::IXMLDOMDocument2>, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> windows_core::Result<()>;
    fn ConvertDevModeToPrintTicket(&self, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: windows_core::Ref<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> windows_core::Result<()>;
    fn GetPrintCapabilities(&self, pprintticket: windows_core::Ref<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn ValidatePrintTicket(&self, pbaseticket: windows_core::Ref<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintTicketProvider_Vtbl {
    pub const fn new<Identity: IPrintTicketProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedVersions<Identity: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: PRINTER_HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintTicketProvider_Impl::GetSupportedVersions(this, core::mem::transmute(&hprinter), core::mem::transmute_copy(&ppversions), core::mem::transmute_copy(&cversions)).into()
            }
        }
        unsafe extern "system" fn BindPrinter<Identity: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: PRINTER_HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintTicketProvider_Impl::BindPrinter(this, core::mem::transmute(&hprinter), core::mem::transmute_copy(&version), core::mem::transmute_copy(&poptions), core::mem::transmute_copy(&pdevmodeflags), core::mem::transmute_copy(&cnamespaces), core::mem::transmute_copy(&ppnamespaces)).into()
            }
        }
        unsafe extern "system" fn QueryDeviceNamespace<Identity: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdefaultnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintTicketProvider_Impl::QueryDeviceNamespace(this, core::mem::transmute_copy(&pdefaultnamespace)).into()
            }
        }
        unsafe extern "system" fn ConvertPrintTicketToDevMode<Identity: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticket: *mut core::ffi::c_void, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintTicketProvider_Impl::ConvertPrintTicketToDevMode(this, core::mem::transmute_copy(&pprintticket), core::mem::transmute_copy(&cbdevmodein), core::mem::transmute_copy(&pdevmodein), core::mem::transmute_copy(&pcbdevmodeout), core::mem::transmute_copy(&ppdevmodeout)).into()
            }
        }
        unsafe extern "system" fn ConvertDevModeToPrintTicket<Identity: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintTicketProvider_Impl::ConvertDevModeToPrintTicket(this, core::mem::transmute_copy(&cbdevmode), core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&pprintticket)).into()
            }
        }
        unsafe extern "system" fn GetPrintCapabilities<Identity: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticket: *mut core::ffi::c_void, ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintTicketProvider_Impl::GetPrintCapabilities(this, core::mem::transmute_copy(&pprintticket)) {
                    Ok(ok__) => {
                        ppcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValidatePrintTicket<Identity: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbaseticket: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintTicketProvider_Impl::ValidatePrintTicket(this, core::mem::transmute_copy(&pbaseticket)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedVersions: GetSupportedVersions::<Identity, OFFSET>,
            BindPrinter: BindPrinter::<Identity, OFFSET>,
            QueryDeviceNamespace: QueryDeviceNamespace::<Identity, OFFSET>,
            ConvertPrintTicketToDevMode: ConvertPrintTicketToDevMode::<Identity, OFFSET>,
            ConvertDevModeToPrintTicket: ConvertDevModeToPrintTicket::<Identity, OFFSET>,
            GetPrintCapabilities: GetPrintCapabilities::<Identity, OFFSET>,
            ValidatePrintTicket: ValidatePrintTicket::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTicketProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintTicketProvider {}
windows_core::imp::define_interface!(IPrintTicketProvider2, IPrintTicketProvider2_Vtbl, 0xb8a70ab2_3dfc_4fec_a074_511b13c651cb);
impl core::ops::Deref for IPrintTicketProvider2 {
    type Target = IPrintTicketProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintTicketProvider2, windows_core::IUnknown, IPrintTicketProvider);
impl IPrintTicketProvider2 {
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetPrintDeviceCapabilities<P0>(&self, pprintticket: P0) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>
    where
        P0: windows_core::Param<super::super::Data::Xml::MsXml::IXMLDOMDocument2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrintDeviceCapabilities)(windows_core::Interface::as_raw(self), pprintticket.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetPrintDeviceResources<P0, P1>(&self, pszlocalename: P0, pprintticket: P1) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Data::Xml::MsXml::IXMLDOMDocument2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrintDeviceResources)(windows_core::Interface::as_raw(self), pszlocalename.param().abi(), pprintticket.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketProvider2_Vtbl {
    pub base__: IPrintTicketProvider_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetPrintDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetPrintDeviceCapabilities: usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetPrintDeviceResources: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetPrintDeviceResources: usize,
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider2_Impl: IPrintTicketProvider_Impl {
    fn GetPrintDeviceCapabilities(&self, pprintticket: windows_core::Ref<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn GetPrintDeviceResources(&self, pszlocalename: &windows_core::PCWSTR, pprintticket: windows_core::Ref<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintTicketProvider2_Vtbl {
    pub const fn new<Identity: IPrintTicketProvider2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPrintDeviceCapabilities<Identity: IPrintTicketProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticket: *mut core::ffi::c_void, ppdevicecapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintTicketProvider2_Impl::GetPrintDeviceCapabilities(this, core::mem::transmute_copy(&pprintticket)) {
                    Ok(ok__) => {
                        ppdevicecapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrintDeviceResources<Identity: IPrintTicketProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlocalename: windows_core::PCWSTR, pprintticket: *mut core::ffi::c_void, ppdeviceresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintTicketProvider2_Impl::GetPrintDeviceResources(this, core::mem::transmute(&pszlocalename), core::mem::transmute_copy(&pprintticket)) {
                    Ok(ok__) => {
                        ppdeviceresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPrintTicketProvider_Vtbl::new::<Identity, OFFSET>(),
            GetPrintDeviceCapabilities: GetPrintDeviceCapabilities::<Identity, OFFSET>,
            GetPrintDeviceResources: GetPrintDeviceResources::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTicketProvider2 as windows_core::Interface>::IID || iid == &<IPrintTicketProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintTicketProvider2 {}
windows_core::imp::define_interface!(IPrintUnidiAsyncNotifyRegistration, IPrintUnidiAsyncNotifyRegistration_Vtbl, 0);
impl core::ops::Deref for IPrintUnidiAsyncNotifyRegistration {
    type Target = IPrintAsyncNotifyRegistration;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintUnidiAsyncNotifyRegistration, windows_core::IUnknown, IPrintAsyncNotifyRegistration);
impl IPrintUnidiAsyncNotifyRegistration {
    pub unsafe fn AsyncGetNotification<P0>(&self, param0: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncGetSendNotificationCookie>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncGetNotification)(windows_core::Interface::as_raw(self), param0.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintUnidiAsyncNotifyRegistration_Vtbl {
    pub base__: IPrintAsyncNotifyRegistration_Vtbl,
    pub AsyncGetNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintUnidiAsyncNotifyRegistration_Impl: IPrintAsyncNotifyRegistration_Impl {
    fn AsyncGetNotification(&self, param0: windows_core::Ref<IAsyncGetSendNotificationCookie>) -> windows_core::Result<()>;
}
impl IPrintUnidiAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: IPrintUnidiAsyncNotifyRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsyncGetNotification<Identity: IPrintUnidiAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintUnidiAsyncNotifyRegistration_Impl::AsyncGetNotification(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        Self { base__: IPrintAsyncNotifyRegistration_Vtbl::new::<Identity, OFFSET>(), AsyncGetNotification: AsyncGetNotification::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintUnidiAsyncNotifyRegistration as windows_core::Interface>::IID || iid == &<IPrintAsyncNotifyRegistration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintUnidiAsyncNotifyRegistration {}
windows_core::imp::define_interface!(IPrintWriteStream, IPrintWriteStream_Vtbl, 0x65bb7f1b_371e_4571_8ac7_912f510c1a38);
windows_core::imp::interface_hierarchy!(IPrintWriteStream, windows_core::IUnknown);
impl IPrintWriteStream {
    pub unsafe fn WriteBytes(&self, pvbuffer: *const core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteBytes)(windows_core::Interface::as_raw(self), pvbuffer, cbbuffer, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWriteStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WriteBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IPrintWriteStream_Impl: windows_core::IUnknownImpl {
    fn WriteBytes(&self, pvbuffer: *const core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<u32>;
    fn Close(&self);
}
impl IPrintWriteStream_Vtbl {
    pub const fn new<Identity: IPrintWriteStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteBytes<Identity: IPrintWriteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbuffer: *const core::ffi::c_void, cbbuffer: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintWriteStream_Impl::WriteBytes(this, core::mem::transmute_copy(&pvbuffer), core::mem::transmute_copy(&cbbuffer)) {
                    Ok(ok__) => {
                        pcbwritten.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IPrintWriteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintWriteStream_Impl::Close(this)
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), WriteBytes: WriteBytes::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWriteStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintWriteStream {}
windows_core::imp::define_interface!(IPrintWriteStreamFlush, IPrintWriteStreamFlush_Vtbl, 0x07d11ff8_1753_4873_b749_6cdaf068e4c3);
windows_core::imp::interface_hierarchy!(IPrintWriteStreamFlush, windows_core::IUnknown);
impl IPrintWriteStreamFlush {
    pub unsafe fn FlushData(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FlushData)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWriteStreamFlush_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FlushData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintWriteStreamFlush_Impl: windows_core::IUnknownImpl {
    fn FlushData(&self) -> windows_core::Result<()>;
}
impl IPrintWriteStreamFlush_Vtbl {
    pub const fn new<Identity: IPrintWriteStreamFlush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FlushData<Identity: IPrintWriteStreamFlush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintWriteStreamFlush_Impl::FlushData(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FlushData: FlushData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWriteStreamFlush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintWriteStreamFlush {}
windows_core::imp::define_interface!(IPrinterBidiSetRequestCallback, IPrinterBidiSetRequestCallback_Vtbl, 0xc52d32dd_f2b4_4052_8502_ec4305ecb71f);
windows_core::imp::interface_hierarchy!(IPrinterBidiSetRequestCallback, windows_core::IUnknown);
impl IPrinterBidiSetRequestCallback {
    pub unsafe fn Completed(&self, bstrresponse: &windows_core::BSTR, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Completed)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrresponse), hrstatus).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterBidiSetRequestCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IPrinterBidiSetRequestCallback_Impl: windows_core::IUnknownImpl {
    fn Completed(&self, bstrresponse: &windows_core::BSTR, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IPrinterBidiSetRequestCallback_Vtbl {
    pub const fn new<Identity: IPrinterBidiSetRequestCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Completed<Identity: IPrinterBidiSetRequestCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresponse: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterBidiSetRequestCallback_Impl::Completed(this, core::mem::transmute(&bstrresponse), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Completed: Completed::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterBidiSetRequestCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrinterBidiSetRequestCallback {}
windows_core::imp::define_interface!(IPrinterExtensionAsyncOperation, IPrinterExtensionAsyncOperation_Vtbl, 0x108d6a23_6a4b_4552_9448_68b427186acd);
windows_core::imp::interface_hierarchy!(IPrinterExtensionAsyncOperation, windows_core::IUnknown);
impl IPrinterExtensionAsyncOperation {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionAsyncOperation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrinterExtensionAsyncOperation_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl IPrinterExtensionAsyncOperation_Vtbl {
    pub const fn new<Identity: IPrinterExtensionAsyncOperation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: IPrinterExtensionAsyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterExtensionAsyncOperation_Impl::Cancel(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionAsyncOperation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrinterExtensionAsyncOperation {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterExtensionContext, IPrinterExtensionContext_Vtbl, 0x39843bf2_c4d2_41fd_b4b2_aedbee5e1900);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterExtensionContext {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterExtensionContext, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionContext {
    pub unsafe fn PrinterQueue(&self) -> windows_core::Result<IPrinterQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrinterQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrintSchemaTicket(&self) -> windows_core::Result<IPrintSchemaTicket> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrintSchemaTicket)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DriverProperties(&self) -> windows_core::Result<IPrinterPropertyBag> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn UserProperties(&self) -> windows_core::Result<IPrinterPropertyBag> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub PrinterQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrintSchemaTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionContext_Impl: super::super::System::Com::IDispatch_Impl {
    fn PrinterQueue(&self) -> windows_core::Result<IPrinterQueue>;
    fn PrintSchemaTicket(&self) -> windows_core::Result<IPrintSchemaTicket>;
    fn DriverProperties(&self) -> windows_core::Result<IPrinterPropertyBag>;
    fn UserProperties(&self) -> windows_core::Result<IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterExtensionContext_Vtbl {
    pub const fn new<Identity: IPrinterExtensionContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrinterQueue<Identity: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionContext_Impl::PrinterQueue(this) {
                    Ok(ok__) => {
                        ppqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrintSchemaTicket<Identity: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppticket: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionContext_Impl::PrintSchemaTicket(this) {
                    Ok(ok__) => {
                        ppticket.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverProperties<Identity: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionContext_Impl::DriverProperties(this) {
                    Ok(ok__) => {
                        pppropertybag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserProperties<Identity: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionContext_Impl::UserProperties(this) {
                    Ok(ok__) => {
                        pppropertybag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PrinterQueue: PrinterQueue::<Identity, OFFSET>,
            PrintSchemaTicket: PrintSchemaTicket::<Identity, OFFSET>,
            DriverProperties: DriverProperties::<Identity, OFFSET>,
            UserProperties: UserProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionContext as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterExtensionContext {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterExtensionContextCollection, IPrinterExtensionContextCollection_Vtbl, 0xfb476970_9bab_4861_811e_3e98b0c5addf);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterExtensionContextCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterExtensionContextCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionContextCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, ulindex: u32) -> windows_core::Result<IPrinterExtensionContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), ulindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionContextCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionContextCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, ulindex: u32) -> windows_core::Result<IPrinterExtensionContext>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterExtensionContextCollection_Vtbl {
    pub const fn new<Identity: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionContextCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pulcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionContextCollection_Impl::GetAt(this, core::mem::transmute_copy(&ulindex)) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionContextCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionContextCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterExtensionContextCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterExtensionEvent, IPrinterExtensionEvent_Vtbl, 0xc093cb63_5ef5_4585_af8e_4d5637487b57);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterExtensionEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterExtensionEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionEvent {
    pub unsafe fn OnDriverEvent<P0>(&self, peventargs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrinterExtensionEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDriverEvent)(windows_core::Interface::as_raw(self), peventargs.param().abi()).ok() }
    }
    pub unsafe fn OnPrinterQueuesEnumerated<P0>(&self, pcontextcollection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrinterExtensionContextCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPrinterQueuesEnumerated)(windows_core::Interface::as_raw(self), pcontextcollection.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub OnDriverEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPrinterQueuesEnumerated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionEvent_Impl: super::super::System::Com::IDispatch_Impl {
    fn OnDriverEvent(&self, peventargs: windows_core::Ref<IPrinterExtensionEventArgs>) -> windows_core::Result<()>;
    fn OnPrinterQueuesEnumerated(&self, pcontextcollection: windows_core::Ref<IPrinterExtensionContextCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterExtensionEvent_Vtbl {
    pub const fn new<Identity: IPrinterExtensionEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDriverEvent<Identity: IPrinterExtensionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventargs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterExtensionEvent_Impl::OnDriverEvent(this, core::mem::transmute_copy(&peventargs)).into()
            }
        }
        unsafe extern "system" fn OnPrinterQueuesEnumerated<Identity: IPrinterExtensionEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontextcollection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterExtensionEvent_Impl::OnPrinterQueuesEnumerated(this, core::mem::transmute_copy(&pcontextcollection)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OnDriverEvent: OnDriverEvent::<Identity, OFFSET>,
            OnPrinterQueuesEnumerated: OnPrinterQueuesEnumerated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterExtensionEvent {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterExtensionEventArgs, IPrinterExtensionEventArgs_Vtbl, 0x39843bf4_c4d2_41fd_b4b2_aedbee5e1900);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterExtensionEventArgs {
    type Target = IPrinterExtensionContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterExtensionEventArgs, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrinterExtensionContext);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionEventArgs {
    pub unsafe fn BidiNotification(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BidiNotification)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ReasonId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReasonId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Request(&self) -> windows_core::Result<IPrinterExtensionRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Request)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SourceApplication(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceApplication)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DetailedReasonId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetailedReasonId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WindowModal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowModal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WindowParent(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowParent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionEventArgs_Vtbl {
    pub base__: IPrinterExtensionContext_Vtbl,
    pub BidiNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReasonId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SourceApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetailedReasonId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub WindowModal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub WindowParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionEventArgs_Impl: IPrinterExtensionContext_Impl {
    fn BidiNotification(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ReasonId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Request(&self) -> windows_core::Result<IPrinterExtensionRequest>;
    fn SourceApplication(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DetailedReasonId(&self) -> windows_core::Result<windows_core::GUID>;
    fn WindowModal(&self) -> windows_core::Result<windows_core::BOOL>;
    fn WindowParent(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterExtensionEventArgs_Vtbl {
    pub const fn new<Identity: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BidiNotification<Identity: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbidinotification: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionEventArgs_Impl::BidiNotification(this) {
                    Ok(ok__) => {
                        pbstrbidinotification.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReasonId<Identity: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preasonid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionEventArgs_Impl::ReasonId(this) {
                    Ok(ok__) => {
                        preasonid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Request<Identity: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionEventArgs_Impl::Request(this) {
                    Ok(ok__) => {
                        pprequest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SourceApplication<Identity: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrapplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionEventArgs_Impl::SourceApplication(this) {
                    Ok(ok__) => {
                        pbstrapplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DetailedReasonId<Identity: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdetailedreasonid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionEventArgs_Impl::DetailedReasonId(this) {
                    Ok(ok__) => {
                        pdetailedreasonid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WindowModal<Identity: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmodal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionEventArgs_Impl::WindowModal(this) {
                    Ok(ok__) => {
                        pbmodal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WindowParent<Identity: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwndparent: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterExtensionEventArgs_Impl::WindowParent(this) {
                    Ok(ok__) => {
                        phwndparent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPrinterExtensionContext_Vtbl::new::<Identity, OFFSET>(),
            BidiNotification: BidiNotification::<Identity, OFFSET>,
            ReasonId: ReasonId::<Identity, OFFSET>,
            Request: Request::<Identity, OFFSET>,
            SourceApplication: SourceApplication::<Identity, OFFSET>,
            DetailedReasonId: DetailedReasonId::<Identity, OFFSET>,
            WindowModal: WindowModal::<Identity, OFFSET>,
            WindowParent: WindowParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionEventArgs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrinterExtensionContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterExtensionEventArgs {}
windows_core::imp::define_interface!(IPrinterExtensionManager, IPrinterExtensionManager_Vtbl, 0x93c6eb8c_b001_4355_9629_8e8a1b3f8e77);
windows_core::imp::interface_hierarchy!(IPrinterExtensionManager, windows_core::IUnknown);
impl IPrinterExtensionManager {
    pub unsafe fn EnableEvents(&self, printerdriverid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableEvents)(windows_core::Interface::as_raw(self), core::mem::transmute(printerdriverid)).ok() }
    }
    pub unsafe fn DisableEvents(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableEvents)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableEvents: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub DisableEvents: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrinterExtensionManager_Impl: windows_core::IUnknownImpl {
    fn EnableEvents(&self, printerdriverid: &windows_core::GUID) -> windows_core::Result<()>;
    fn DisableEvents(&self) -> windows_core::Result<()>;
}
impl IPrinterExtensionManager_Vtbl {
    pub const fn new<Identity: IPrinterExtensionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableEvents<Identity: IPrinterExtensionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printerdriverid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterExtensionManager_Impl::EnableEvents(this, core::mem::transmute(&printerdriverid)).into()
            }
        }
        unsafe extern "system" fn DisableEvents<Identity: IPrinterExtensionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterExtensionManager_Impl::DisableEvents(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableEvents: EnableEvents::<Identity, OFFSET>,
            DisableEvents: DisableEvents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrinterExtensionManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterExtensionRequest, IPrinterExtensionRequest_Vtbl, 0x39843bf3_c4d2_41fd_b4b2_aedbee5e1900);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterExtensionRequest {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterExtensionRequest, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionRequest {
    pub unsafe fn Cancel(&self, hrstatus: windows_core::HRESULT, bstrlogmessage: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), hrstatus, core::mem::transmute_copy(bstrlogmessage)).ok() }
    }
    pub unsafe fn Complete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionRequest_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionRequest_Impl: super::super::System::Com::IDispatch_Impl {
    fn Cancel(&self, hrstatus: windows_core::HRESULT, bstrlogmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Complete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterExtensionRequest_Vtbl {
    pub const fn new<Identity: IPrinterExtensionRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: IPrinterExtensionRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, bstrlogmessage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterExtensionRequest_Impl::Cancel(this, core::mem::transmute_copy(&hrstatus), core::mem::transmute(&bstrlogmessage)).into()
            }
        }
        unsafe extern "system" fn Complete<Identity: IPrinterExtensionRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterExtensionRequest_Impl::Complete(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Complete: Complete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionRequest as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterExtensionRequest {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterPropertyBag, IPrinterPropertyBag_Vtbl, 0xfea77364_df95_4a23_a905_019b79a8e481);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterPropertyBag {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterPropertyBag, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterPropertyBag {
    pub unsafe fn GetBool(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBool)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBool(&self, bstrname: &windows_core::BSTR, bvalue: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBool)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), bvalue.into()).ok() }
    }
    pub unsafe fn GetInt32(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInt32)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInt32(&self, bstrname: &windows_core::BSTR, nvalue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInt32)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), nvalue).ok() }
    }
    pub unsafe fn GetString(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetString(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrvalue)).ok() }
    }
    pub unsafe fn GetBytes(&self, bstrname: &windows_core::BSTR, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBytes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), pcbvalue as _, ppvalue as _).ok() }
    }
    pub unsafe fn SetBytes(&self, bstrname: &windows_core::BSTR, pvalue: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBytes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), pvalue.len().try_into().unwrap(), core::mem::transmute(pvalue.as_ptr())).ok() }
    }
    pub unsafe fn GetReadStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReadStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWriteStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriteStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterPropertyBag_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetBool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub GetReadStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWriteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterPropertyBag_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetBool(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL>;
    fn SetBool(&self, bstrname: &windows_core::BSTR, bvalue: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetInt32(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn SetInt32(&self, bstrname: &windows_core::BSTR, nvalue: i32) -> windows_core::Result<()>;
    fn GetString(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetString(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBytes(&self, bstrname: &windows_core::BSTR, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> windows_core::Result<()>;
    fn SetBytes(&self, bstrname: &windows_core::BSTR, cbvalue: u32, pvalue: *const u8) -> windows_core::Result<()>;
    fn GetReadStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream>;
    fn GetWriteStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterPropertyBag_Vtbl {
    pub const fn new<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBool<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pbvalue: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterPropertyBag_Impl::GetBool(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pbvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBool<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bvalue: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterPropertyBag_Impl::SetBool(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&bvalue)).into()
            }
        }
        unsafe extern "system" fn GetInt32<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pnvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterPropertyBag_Impl::GetInt32(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pnvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInt32<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, nvalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterPropertyBag_Impl::SetInt32(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&nvalue)).into()
            }
        }
        unsafe extern "system" fn GetString<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterPropertyBag_Impl::GetString(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetString<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterPropertyBag_Impl::SetString(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue)).into()
            }
        }
        unsafe extern "system" fn GetBytes<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterPropertyBag_Impl::GetBytes(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&pcbvalue), core::mem::transmute_copy(&ppvalue)).into()
            }
        }
        unsafe extern "system" fn SetBytes<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, cbvalue: u32, pvalue: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterPropertyBag_Impl::SetBytes(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&cbvalue), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetReadStream<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterPropertyBag_Impl::GetReadStream(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterPropertyBag_Impl::GetWriteStream(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetBool: GetBool::<Identity, OFFSET>,
            SetBool: SetBool::<Identity, OFFSET>,
            GetInt32: GetInt32::<Identity, OFFSET>,
            SetInt32: SetInt32::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            SetString: SetString::<Identity, OFFSET>,
            GetBytes: GetBytes::<Identity, OFFSET>,
            SetBytes: SetBytes::<Identity, OFFSET>,
            GetReadStream: GetReadStream::<Identity, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterPropertyBag as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterPropertyBag {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterQueue, IPrinterQueue_Vtbl, 0x3580a828_07fe_4b94_ac1a_757d9d2d3056);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterQueue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterQueue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueue {
    pub unsafe fn Handle(&self) -> windows_core::Result<PRINTER_HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SendBidiQuery(&self, bstrbidiquery: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendBidiQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbidiquery)).ok() }
    }
    pub unsafe fn GetProperties(&self) -> windows_core::Result<IPrinterPropertyBag> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PRINTER_HANDLE) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendBidiQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueue_Impl: super::super::System::Com::IDispatch_Impl {
    fn Handle(&self) -> windows_core::Result<PRINTER_HANDLE>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SendBidiQuery(&self, bstrbidiquery: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetProperties(&self) -> windows_core::Result<IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterQueue_Vtbl {
    pub const fn new<Identity: IPrinterQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handle<Identity: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phprinter: *mut PRINTER_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterQueue_Impl::Handle(this) {
                    Ok(ok__) => {
                        phprinter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterQueue_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendBidiQuery<Identity: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbidiquery: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterQueue_Impl::SendBidiQuery(this, core::mem::transmute(&bstrbidiquery)).into()
            }
        }
        unsafe extern "system" fn GetProperties<Identity: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterQueue_Impl::GetProperties(this) {
                    Ok(ok__) => {
                        pppropertybag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Handle: Handle::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SendBidiQuery: SendBidiQuery::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterQueue {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterQueue2, IPrinterQueue2_Vtbl, 0x8cd444e8_c9bb_49b3_8e38_e03209416131);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterQueue2 {
    type Target = IPrinterQueue;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterQueue2, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrinterQueue);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueue2 {
    pub unsafe fn SendBidiSetRequestAsync<P1>(&self, bstrbidirequest: &windows_core::BSTR, pcallback: P1) -> windows_core::Result<IPrinterExtensionAsyncOperation>
    where
        P1: windows_core::Param<IPrinterBidiSetRequestCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendBidiSetRequestAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbidirequest), pcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPrinterQueueView(&self, ulviewoffset: u32, ulviewsize: u32) -> windows_core::Result<IPrinterQueueView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrinterQueueView)(windows_core::Interface::as_raw(self), ulviewoffset, ulviewsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterQueue2_Vtbl {
    pub base__: IPrinterQueue_Vtbl,
    pub SendBidiSetRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrinterQueueView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueue2_Impl: IPrinterQueue_Impl {
    fn SendBidiSetRequestAsync(&self, bstrbidirequest: &windows_core::BSTR, pcallback: windows_core::Ref<IPrinterBidiSetRequestCallback>) -> windows_core::Result<IPrinterExtensionAsyncOperation>;
    fn GetPrinterQueueView(&self, ulviewoffset: u32, ulviewsize: u32) -> windows_core::Result<IPrinterQueueView>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterQueue2_Vtbl {
    pub const fn new<Identity: IPrinterQueue2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendBidiSetRequestAsync<Identity: IPrinterQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbidirequest: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, ppasyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterQueue2_Impl::SendBidiSetRequestAsync(this, core::mem::transmute(&bstrbidirequest), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        ppasyncoperation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrinterQueueView<Identity: IPrinterQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ppjobview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterQueue2_Impl::GetPrinterQueueView(this, core::mem::transmute_copy(&ulviewoffset), core::mem::transmute_copy(&ulviewsize)) {
                    Ok(ok__) => {
                        ppjobview.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPrinterQueue_Vtbl::new::<Identity, OFFSET>(),
            SendBidiSetRequestAsync: SendBidiSetRequestAsync::<Identity, OFFSET>,
            GetPrinterQueueView: GetPrinterQueueView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterQueue2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrinterQueue as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterQueue2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterQueueEvent, IPrinterQueueEvent_Vtbl, 0x214685f6_7b78_4681_87e0_495f739273d1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterQueueEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterQueueEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueEvent {
    pub unsafe fn OnBidiResponseReceived(&self, bstrresponse: &windows_core::BSTR, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnBidiResponseReceived)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrresponse), hrstatus).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterQueueEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub OnBidiResponseReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueueEvent_Impl: super::super::System::Com::IDispatch_Impl {
    fn OnBidiResponseReceived(&self, bstrresponse: &windows_core::BSTR, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterQueueEvent_Vtbl {
    pub const fn new<Identity: IPrinterQueueEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnBidiResponseReceived<Identity: IPrinterQueueEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresponse: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterQueueEvent_Impl::OnBidiResponseReceived(this, core::mem::transmute(&bstrresponse), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), OnBidiResponseReceived: OnBidiResponseReceived::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterQueueEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterQueueEvent {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterQueueView, IPrinterQueueView_Vtbl, 0x476e2969_3b2b_4b3f_8277_cff6056042aa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterQueueView {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterQueueView, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueView {
    pub unsafe fn SetViewRange(&self, ulviewoffset: u32, ulviewsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetViewRange)(windows_core::Interface::as_raw(self), ulviewoffset, ulviewsize).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterQueueView_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetViewRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueueView_Impl: super::super::System::Com::IDispatch_Impl {
    fn SetViewRange(&self, ulviewoffset: u32, ulviewsize: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterQueueView_Vtbl {
    pub const fn new<Identity: IPrinterQueueView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetViewRange<Identity: IPrinterQueueView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterQueueView_Impl::SetViewRange(this, core::mem::transmute_copy(&ulviewoffset), core::mem::transmute_copy(&ulviewsize)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), SetViewRange: SetViewRange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterQueueView as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterQueueView {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterQueueViewEvent, IPrinterQueueViewEvent_Vtbl, 0xc5b6042b_fd21_404a_a0ef_e2fbb52b9080);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterQueueViewEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterQueueViewEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueViewEvent {
    pub unsafe fn OnChanged<P0>(&self, pcollection: P0, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintJobCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnChanged)(windows_core::Interface::as_raw(self), pcollection.param().abi(), ulviewoffset, ulviewsize, ulcountjobsinprintqueue).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterQueueViewEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub OnChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueueViewEvent_Impl: super::super::System::Com::IDispatch_Impl {
    fn OnChanged(&self, pcollection: windows_core::Ref<IPrintJobCollection>, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterQueueViewEvent_Vtbl {
    pub const fn new<Identity: IPrinterQueueViewEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnChanged<Identity: IPrinterQueueViewEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcollection: *mut core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterQueueViewEvent_Impl::OnChanged(this, core::mem::transmute_copy(&pcollection), core::mem::transmute_copy(&ulviewoffset), core::mem::transmute_copy(&ulviewsize), core::mem::transmute_copy(&ulcountjobsinprintqueue)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), OnChanged: OnChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterQueueViewEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterQueueViewEvent {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterScriptContext, IPrinterScriptContext_Vtbl, 0x066acbca_8881_49c9_bb98_fae16b4889e1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterScriptContext {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterScriptContext, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptContext {
    pub unsafe fn DriverProperties(&self) -> windows_core::Result<IPrinterScriptablePropertyBag> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QueueProperties(&self) -> windows_core::Result<IPrinterScriptablePropertyBag> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn UserProperties(&self) -> windows_core::Result<IPrinterScriptablePropertyBag> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterScriptContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DriverProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueueProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptContext_Impl: super::super::System::Com::IDispatch_Impl {
    fn DriverProperties(&self) -> windows_core::Result<IPrinterScriptablePropertyBag>;
    fn QueueProperties(&self) -> windows_core::Result<IPrinterScriptablePropertyBag>;
    fn UserProperties(&self) -> windows_core::Result<IPrinterScriptablePropertyBag>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterScriptContext_Vtbl {
    pub const fn new<Identity: IPrinterScriptContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DriverProperties<Identity: IPrinterScriptContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptContext_Impl::DriverProperties(this) {
                    Ok(ok__) => {
                        pppropertybag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueProperties<Identity: IPrinterScriptContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptContext_Impl::QueueProperties(this) {
                    Ok(ok__) => {
                        pppropertybag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserProperties<Identity: IPrinterScriptContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptContext_Impl::UserProperties(this) {
                    Ok(ok__) => {
                        pppropertybag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DriverProperties: DriverProperties::<Identity, OFFSET>,
            QueueProperties: QueueProperties::<Identity, OFFSET>,
            UserProperties: UserProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterScriptContext as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterScriptContext {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterScriptablePropertyBag, IPrinterScriptablePropertyBag_Vtbl, 0x91c7765f_ed57_49ad_8b01_dc24816a5294);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterScriptablePropertyBag {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterScriptablePropertyBag, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptablePropertyBag {
    pub unsafe fn GetBool(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBool)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBool(&self, bstrname: &windows_core::BSTR, bvalue: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBool)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), bvalue.into()).ok() }
    }
    pub unsafe fn GetInt32(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInt32)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInt32(&self, bstrname: &windows_core::BSTR, nvalue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInt32)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), nvalue).ok() }
    }
    pub unsafe fn GetString(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetString(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrvalue)).ok() }
    }
    pub unsafe fn GetBytes(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBytes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBytes<P1>(&self, bstrname: &windows_core::BSTR, parray: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBytes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), parray.param().abi()).ok() }
    }
    pub unsafe fn GetReadStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IPrinterScriptableStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReadStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWriteStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IPrinterScriptableStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriteStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterScriptablePropertyBag_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetBool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReadStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWriteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptablePropertyBag_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetBool(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL>;
    fn SetBool(&self, bstrname: &windows_core::BSTR, bvalue: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetInt32(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn SetInt32(&self, bstrname: &windows_core::BSTR, nvalue: i32) -> windows_core::Result<()>;
    fn GetString(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetString(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBytes(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetBytes(&self, bstrname: &windows_core::BSTR, parray: windows_core::Ref<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn GetReadStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IPrinterScriptableStream>;
    fn GetWriteStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IPrinterScriptableStream>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterScriptablePropertyBag_Vtbl {
    pub const fn new<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBool<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pbvalue: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptablePropertyBag_Impl::GetBool(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pbvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBool<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bvalue: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterScriptablePropertyBag_Impl::SetBool(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&bvalue)).into()
            }
        }
        unsafe extern "system" fn GetInt32<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pnvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptablePropertyBag_Impl::GetInt32(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pnvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInt32<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, nvalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterScriptablePropertyBag_Impl::SetInt32(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&nvalue)).into()
            }
        }
        unsafe extern "system" fn GetString<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptablePropertyBag_Impl::GetString(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetString<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterScriptablePropertyBag_Impl::SetString(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue)).into()
            }
        }
        unsafe extern "system" fn GetBytes<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pparray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptablePropertyBag_Impl::GetBytes(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pparray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBytes<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, parray: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterScriptablePropertyBag_Impl::SetBytes(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&parray)).into()
            }
        }
        unsafe extern "system" fn GetReadStream<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptablePropertyBag_Impl::GetReadStream(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptablePropertyBag_Impl::GetWriteStream(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetBool: GetBool::<Identity, OFFSET>,
            SetBool: SetBool::<Identity, OFFSET>,
            GetInt32: GetInt32::<Identity, OFFSET>,
            SetInt32: SetInt32::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            SetString: SetString::<Identity, OFFSET>,
            GetBytes: GetBytes::<Identity, OFFSET>,
            SetBytes: SetBytes::<Identity, OFFSET>,
            GetReadStream: GetReadStream::<Identity, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterScriptablePropertyBag as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterScriptablePropertyBag {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterScriptablePropertyBag2, IPrinterScriptablePropertyBag2_Vtbl, 0x2a1c53c4_8638_4b3e_b518_2773c94556a3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterScriptablePropertyBag2 {
    type Target = IPrinterScriptablePropertyBag;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterScriptablePropertyBag2, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrinterScriptablePropertyBag);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptablePropertyBag2 {
    pub unsafe fn GetReadStreamAsXML(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReadStreamAsXML)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterScriptablePropertyBag2_Vtbl {
    pub base__: IPrinterScriptablePropertyBag_Vtbl,
    pub GetReadStreamAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptablePropertyBag2_Impl: IPrinterScriptablePropertyBag_Impl {
    fn GetReadStreamAsXML(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterScriptablePropertyBag2_Vtbl {
    pub const fn new<Identity: IPrinterScriptablePropertyBag2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetReadStreamAsXML<Identity: IPrinterScriptablePropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ppxmlnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptablePropertyBag2_Impl::GetReadStreamAsXML(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        ppxmlnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IPrinterScriptablePropertyBag_Vtbl::new::<Identity, OFFSET>(), GetReadStreamAsXML: GetReadStreamAsXML::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterScriptablePropertyBag2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrinterScriptablePropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterScriptablePropertyBag2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterScriptableSequentialStream, IPrinterScriptableSequentialStream_Vtbl, 0x2072838a_316f_467a_a949_27f68c44a854);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterScriptableSequentialStream {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterScriptableSequentialStream, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptableSequentialStream {
    pub unsafe fn Read(&self, cbread: i32) -> windows_core::Result<super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), cbread, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Write<P0>(&self, parray: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), parray.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterScriptableSequentialStream_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptableSequentialStream_Impl: super::super::System::Com::IDispatch_Impl {
    fn Read(&self, cbread: i32) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn Write(&self, parray: windows_core::Ref<super::super::System::Com::IDispatch>) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterScriptableSequentialStream_Vtbl {
    pub const fn new<Identity: IPrinterScriptableSequentialStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IPrinterScriptableSequentialStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbread: i32, pparray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptableSequentialStream_Impl::Read(this, core::mem::transmute_copy(&cbread)) {
                    Ok(ok__) => {
                        pparray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Write<Identity: IPrinterScriptableSequentialStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *mut core::ffi::c_void, pcbwritten: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptableSequentialStream_Impl::Write(this, core::mem::transmute_copy(&parray)) {
                    Ok(ok__) => {
                        pcbwritten.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Write: Write::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterScriptableSequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterScriptableSequentialStream {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrinterScriptableStream, IPrinterScriptableStream_Vtbl, 0x7edf9a92_4750_41a5_a17f_879a6f4f7dcb);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrinterScriptableStream {
    type Target = IPrinterScriptableSequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrinterScriptableStream, windows_core::IUnknown, super::super::System::Com::IDispatch, IPrinterScriptableSequentialStream);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptableStream {
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Seek(&self, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), loffset, streamseek, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSize(&self, lsize: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), lsize).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterScriptableStream_Vtbl {
    pub base__: IPrinterScriptableSequentialStream_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::System::Com::STREAM_SEEK, *mut i32) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptableStream_Impl: IPrinterScriptableSequentialStream_Impl {
    fn Commit(&self) -> windows_core::Result<()>;
    fn Seek(&self, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK) -> windows_core::Result<i32>;
    fn SetSize(&self, lsize: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrinterScriptableStream_Vtbl {
    pub const fn new<Identity: IPrinterScriptableStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: IPrinterScriptableStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterScriptableStream_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn Seek<Identity: IPrinterScriptableStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK, plposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrinterScriptableStream_Impl::Seek(this, core::mem::transmute_copy(&loffset), core::mem::transmute_copy(&streamseek)) {
                    Ok(ok__) => {
                        plposition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSize<Identity: IPrinterScriptableStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinterScriptableStream_Impl::SetSize(this, core::mem::transmute_copy(&lsize)).into()
            }
        }
        Self {
            base__: IPrinterScriptableSequentialStream_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterScriptableStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrinterScriptableSequentialStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrinterScriptableStream {}
windows_core::imp::define_interface!(IXpsDocument, IXpsDocument_Vtbl, 0xe8d907db_62a9_4a95_abe7_e01763dd30f8);
windows_core::imp::interface_hierarchy!(IXpsDocument, windows_core::IUnknown);
impl IXpsDocument {
    pub unsafe fn GetThumbnail(&self) -> windows_core::Result<IPartThumbnail> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThumbnail)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetThumbnail<P0>(&self, pthumbnail: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPartThumbnail>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetThumbnail)(windows_core::Interface::as_raw(self), pthumbnail.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocument_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsDocument_Impl: windows_core::IUnknownImpl {
    fn GetThumbnail(&self) -> windows_core::Result<IPartThumbnail>;
    fn SetThumbnail(&self, pthumbnail: windows_core::Ref<IPartThumbnail>) -> windows_core::Result<()>;
}
impl IXpsDocument_Vtbl {
    pub const fn new<Identity: IXpsDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetThumbnail<Identity: IXpsDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppthumbnail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsDocument_Impl::GetThumbnail(this) {
                    Ok(ok__) => {
                        ppthumbnail.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetThumbnail<Identity: IXpsDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthumbnail: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsDocument_Impl::SetThumbnail(this, core::mem::transmute_copy(&pthumbnail)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetThumbnail: GetThumbnail::<Identity, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsDocument as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsDocument {}
windows_core::imp::define_interface!(IXpsDocumentConsumer, IXpsDocumentConsumer_Vtbl, 0x4368d8a2_4181_4a9f_b295_3d9a38bb9ba0);
windows_core::imp::interface_hierarchy!(IXpsDocumentConsumer, windows_core::IUnknown);
impl IXpsDocumentConsumer {
    pub unsafe fn SendXpsUnknown<P0>(&self, punknown: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendXpsUnknown)(windows_core::Interface::as_raw(self), punknown.param().abi()).ok() }
    }
    pub unsafe fn SendXpsDocument<P0>(&self, pixpsdocument: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsDocument>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendXpsDocument)(windows_core::Interface::as_raw(self), pixpsdocument.param().abi()).ok() }
    }
    pub unsafe fn SendFixedDocumentSequence<P0>(&self, pifixeddocumentsequence: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFixedDocumentSequence>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendFixedDocumentSequence)(windows_core::Interface::as_raw(self), pifixeddocumentsequence.param().abi()).ok() }
    }
    pub unsafe fn SendFixedDocument<P0>(&self, pifixeddocument: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFixedDocument>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendFixedDocument)(windows_core::Interface::as_raw(self), pifixeddocument.param().abi()).ok() }
    }
    pub unsafe fn SendFixedPage<P0>(&self, pifixedpage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFixedPage>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendFixedPage)(windows_core::Interface::as_raw(self), pifixedpage.param().abi()).ok() }
    }
    pub unsafe fn CloseSender(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseSender)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetNewEmptyPart<P0>(&self, uri: P0, riid: *const windows_core::GUID, ppnewobject: *mut *mut core::ffi::c_void, ppwritestream: *mut Option<IPrintWriteStream>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetNewEmptyPart)(windows_core::Interface::as_raw(self), uri.param().abi(), riid, ppnewobject as _, core::mem::transmute(ppwritestream)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentConsumer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SendXpsUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendXpsDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendFixedDocumentSequence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendFixedDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendFixedPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseSender: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNewEmptyPart: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsDocumentConsumer_Impl: windows_core::IUnknownImpl {
    fn SendXpsUnknown(&self, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SendXpsDocument(&self, pixpsdocument: windows_core::Ref<IXpsDocument>) -> windows_core::Result<()>;
    fn SendFixedDocumentSequence(&self, pifixeddocumentsequence: windows_core::Ref<IFixedDocumentSequence>) -> windows_core::Result<()>;
    fn SendFixedDocument(&self, pifixeddocument: windows_core::Ref<IFixedDocument>) -> windows_core::Result<()>;
    fn SendFixedPage(&self, pifixedpage: windows_core::Ref<IFixedPage>) -> windows_core::Result<()>;
    fn CloseSender(&self) -> windows_core::Result<()>;
    fn GetNewEmptyPart(&self, uri: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppnewobject: *mut *mut core::ffi::c_void, ppwritestream: windows_core::OutRef<IPrintWriteStream>) -> windows_core::Result<()>;
}
impl IXpsDocumentConsumer_Vtbl {
    pub const fn new<Identity: IXpsDocumentConsumer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendXpsUnknown<Identity: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsDocumentConsumer_Impl::SendXpsUnknown(this, core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn SendXpsDocument<Identity: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixpsdocument: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsDocumentConsumer_Impl::SendXpsDocument(this, core::mem::transmute_copy(&pixpsdocument)).into()
            }
        }
        unsafe extern "system" fn SendFixedDocumentSequence<Identity: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifixeddocumentsequence: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsDocumentConsumer_Impl::SendFixedDocumentSequence(this, core::mem::transmute_copy(&pifixeddocumentsequence)).into()
            }
        }
        unsafe extern "system" fn SendFixedDocument<Identity: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifixeddocument: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsDocumentConsumer_Impl::SendFixedDocument(this, core::mem::transmute_copy(&pifixeddocument)).into()
            }
        }
        unsafe extern "system" fn SendFixedPage<Identity: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifixedpage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsDocumentConsumer_Impl::SendFixedPage(this, core::mem::transmute_copy(&pifixedpage)).into()
            }
        }
        unsafe extern "system" fn CloseSender<Identity: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsDocumentConsumer_Impl::CloseSender(this).into()
            }
        }
        unsafe extern "system" fn GetNewEmptyPart<Identity: IXpsDocumentConsumer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, riid: *const windows_core::GUID, ppnewobject: *mut *mut core::ffi::c_void, ppwritestream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsDocumentConsumer_Impl::GetNewEmptyPart(this, core::mem::transmute(&uri), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppnewobject), core::mem::transmute_copy(&ppwritestream)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendXpsUnknown: SendXpsUnknown::<Identity, OFFSET>,
            SendXpsDocument: SendXpsDocument::<Identity, OFFSET>,
            SendFixedDocumentSequence: SendFixedDocumentSequence::<Identity, OFFSET>,
            SendFixedDocument: SendFixedDocument::<Identity, OFFSET>,
            SendFixedPage: SendFixedPage::<Identity, OFFSET>,
            CloseSender: CloseSender::<Identity, OFFSET>,
            GetNewEmptyPart: GetNewEmptyPart::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsDocumentConsumer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsDocumentConsumer {}
windows_core::imp::define_interface!(IXpsDocumentProvider, IXpsDocumentProvider_Vtbl, 0xb8cf8530_5562_47c4_ab67_b1f69ecf961e);
windows_core::imp::interface_hierarchy!(IXpsDocumentProvider, windows_core::IUnknown);
impl IXpsDocumentProvider {
    pub unsafe fn GetXpsPart(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXpsPart)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetXpsPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsDocumentProvider_Impl: windows_core::IUnknownImpl {
    fn GetXpsPart(&self) -> windows_core::Result<windows_core::IUnknown>;
}
impl IXpsDocumentProvider_Vtbl {
    pub const fn new<Identity: IXpsDocumentProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetXpsPart<Identity: IXpsDocumentProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixpspart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsDocumentProvider_Impl::GetXpsPart(this) {
                    Ok(ok__) => {
                        ppixpspart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetXpsPart: GetXpsPart::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsDocumentProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsDocumentProvider {}
windows_core::imp::define_interface!(IXpsPartIterator, IXpsPartIterator_Vtbl, 0x0021d3cd_af6f_42ab_9999_14bc82a62d2e);
windows_core::imp::interface_hierarchy!(IXpsPartIterator, windows_core::IUnknown);
impl IXpsPartIterator {
    pub unsafe fn Reset(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Current(&self, puri: *mut windows_core::BSTR, ppxpspart: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Current)(windows_core::Interface::as_raw(self), core::mem::transmute(puri), core::mem::transmute(ppxpspart)).ok() }
    }
    pub unsafe fn IsDone(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsDone)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Next(&self) {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPartIterator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsDone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IXpsPartIterator_Impl: windows_core::IUnknownImpl {
    fn Reset(&self);
    fn Current(&self, puri: *mut windows_core::BSTR, ppxpspart: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn IsDone(&self) -> windows_core::BOOL;
    fn Next(&self);
}
impl IXpsPartIterator_Vtbl {
    pub const fn new<Identity: IXpsPartIterator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IXpsPartIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsPartIterator_Impl::Reset(this)
            }
        }
        unsafe extern "system" fn Current<Identity: IXpsPartIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut *mut core::ffi::c_void, ppxpspart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsPartIterator_Impl::Current(this, core::mem::transmute_copy(&puri), core::mem::transmute_copy(&ppxpspart)).into()
            }
        }
        unsafe extern "system" fn IsDone<Identity: IXpsPartIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsPartIterator_Impl::IsDone(this)
            }
        }
        unsafe extern "system" fn Next<Identity: IXpsPartIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsPartIterator_Impl::Next(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Current: Current::<Identity, OFFSET>,
            IsDone: IsDone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsPartIterator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsPartIterator {}
windows_core::imp::define_interface!(IXpsRasterizationFactory, IXpsRasterizationFactory_Vtbl, 0xe094808a_24c6_482b_a3a7_c21ac9b55f17);
windows_core::imp::interface_hierarchy!(IXpsRasterizationFactory, windows_core::IUnknown);
impl IXpsRasterizationFactory {
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn CreateRasterizer<P0>(&self, xpspage: P0, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE) -> windows_core::Result<IXpsRasterizer>
    where
        P0: windows_core::Param<super::super::Storage::Xps::IXpsOMPage>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRasterizer)(windows_core::Interface::as_raw(self), xpspage.param().abi(), dpi, nontextrenderingmode, textrenderingmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsRasterizationFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub CreateRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, XPSRAS_RENDERING_MODE, XPSRAS_RENDERING_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    CreateRasterizer: usize,
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory_Impl: windows_core::IUnknownImpl {
    fn CreateRasterizer(&self, xpspage: windows_core::Ref<super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE) -> windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory_Vtbl {
    pub const fn new<Identity: IXpsRasterizationFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateRasterizer<Identity: IXpsRasterizationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpspage: *mut core::ffi::c_void, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, ppixpsrasterizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsRasterizationFactory_Impl::CreateRasterizer(this, core::mem::transmute_copy(&xpspage), core::mem::transmute_copy(&dpi), core::mem::transmute_copy(&nontextrenderingmode), core::mem::transmute_copy(&textrenderingmode)) {
                    Ok(ok__) => {
                        ppixpsrasterizer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
impl windows_core::RuntimeName for IXpsRasterizationFactory {}
windows_core::imp::define_interface!(IXpsRasterizationFactory1, IXpsRasterizationFactory1_Vtbl, 0x2d6e5f77_6414_4a1e_a8e0_d4194ce6a26f);
windows_core::imp::interface_hierarchy!(IXpsRasterizationFactory1, windows_core::IUnknown);
impl IXpsRasterizationFactory1 {
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn CreateRasterizer<P0>(&self, xpspage: P0, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT) -> windows_core::Result<IXpsRasterizer>
    where
        P0: windows_core::Param<super::super::Storage::Xps::IXpsOMPage>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRasterizer)(windows_core::Interface::as_raw(self), xpspage.param().abi(), dpi, nontextrenderingmode, textrenderingmode, pixelformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsRasterizationFactory1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub CreateRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, XPSRAS_RENDERING_MODE, XPSRAS_RENDERING_MODE, XPSRAS_PIXEL_FORMAT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    CreateRasterizer: usize,
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory1_Impl: windows_core::IUnknownImpl {
    fn CreateRasterizer(&self, xpspage: windows_core::Ref<super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT) -> windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory1_Vtbl {
    pub const fn new<Identity: IXpsRasterizationFactory1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateRasterizer<Identity: IXpsRasterizationFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpspage: *mut core::ffi::c_void, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, ppixpsrasterizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsRasterizationFactory1_Impl::CreateRasterizer(this, core::mem::transmute_copy(&xpspage), core::mem::transmute_copy(&dpi), core::mem::transmute_copy(&nontextrenderingmode), core::mem::transmute_copy(&textrenderingmode), core::mem::transmute_copy(&pixelformat)) {
                    Ok(ok__) => {
                        ppixpsrasterizer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
impl windows_core::RuntimeName for IXpsRasterizationFactory1 {}
windows_core::imp::define_interface!(IXpsRasterizationFactory2, IXpsRasterizationFactory2_Vtbl, 0x9c16ce3e_10f5_41fd_9ddc_6826669c2ff6);
windows_core::imp::interface_hierarchy!(IXpsRasterizationFactory2, windows_core::IUnknown);
impl IXpsRasterizationFactory2 {
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn CreateRasterizer<P0>(&self, xpspage: P0, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR) -> windows_core::Result<IXpsRasterizer>
    where
        P0: windows_core::Param<super::super::Storage::Xps::IXpsOMPage>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRasterizer)(windows_core::Interface::as_raw(self), xpspage.param().abi(), dpix, dpiy, nontextrenderingmode, textrenderingmode, pixelformat, backgroundcolor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsRasterizationFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub CreateRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, f32, XPSRAS_RENDERING_MODE, XPSRAS_RENDERING_MODE, XPSRAS_PIXEL_FORMAT, XPSRAS_BACKGROUND_COLOR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    CreateRasterizer: usize,
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory2_Impl: windows_core::IUnknownImpl {
    fn CreateRasterizer(&self, xpspage: windows_core::Ref<super::super::Storage::Xps::IXpsOMPage>, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR) -> windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory2_Vtbl {
    pub const fn new<Identity: IXpsRasterizationFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateRasterizer<Identity: IXpsRasterizationFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpspage: *mut core::ffi::c_void, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR, ppixpsrasterizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsRasterizationFactory2_Impl::CreateRasterizer(this, core::mem::transmute_copy(&xpspage), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy), core::mem::transmute_copy(&nontextrenderingmode), core::mem::transmute_copy(&textrenderingmode), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&backgroundcolor)) {
                    Ok(ok__) => {
                        ppixpsrasterizer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
impl windows_core::RuntimeName for IXpsRasterizationFactory2 {}
windows_core::imp::define_interface!(IXpsRasterizer, IXpsRasterizer_Vtbl, 0x7567cfc8_c156_47a8_9dac_11a2ae5bdd6b);
windows_core::imp::interface_hierarchy!(IXpsRasterizer, windows_core::IUnknown);
impl IXpsRasterizer {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn RasterizeRect<P4>(&self, x: i32, y: i32, width: i32, height: i32, notificationcallback: P4) -> windows_core::Result<super::Imaging::IWICBitmap>
    where
        P4: windows_core::Param<IXpsRasterizerNotificationCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RasterizeRect)(windows_core::Interface::as_raw(self), x, y, width, height, notificationcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMinimalLineWidth(&self, width: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinimalLineWidth)(windows_core::Interface::as_raw(self), width).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsRasterizer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub RasterizeRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    RasterizeRect: usize,
    pub SetMinimalLineWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IXpsRasterizer_Impl: windows_core::IUnknownImpl {
    fn RasterizeRect(&self, x: i32, y: i32, width: i32, height: i32, notificationcallback: windows_core::Ref<IXpsRasterizerNotificationCallback>) -> windows_core::Result<super::Imaging::IWICBitmap>;
    fn SetMinimalLineWidth(&self, width: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IXpsRasterizer_Vtbl {
    pub const fn new<Identity: IXpsRasterizer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RasterizeRect<Identity: IXpsRasterizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, width: i32, height: i32, notificationcallback: *mut core::ffi::c_void, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsRasterizer_Impl::RasterizeRect(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&notificationcallback)) {
                    Ok(ok__) => {
                        bitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinimalLineWidth<Identity: IXpsRasterizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsRasterizer_Impl::SetMinimalLineWidth(this, core::mem::transmute_copy(&width)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RasterizeRect: RasterizeRect::<Identity, OFFSET>,
            SetMinimalLineWidth: SetMinimalLineWidth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsRasterizer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl windows_core::RuntimeName for IXpsRasterizer {}
windows_core::imp::define_interface!(IXpsRasterizerNotificationCallback, IXpsRasterizerNotificationCallback_Vtbl, 0x9ab8fd0d_cb94_49c2_9cb0_97ec1d5469d2);
windows_core::imp::interface_hierarchy!(IXpsRasterizerNotificationCallback, windows_core::IUnknown);
impl IXpsRasterizerNotificationCallback {
    pub unsafe fn Continue(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Continue)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsRasterizerNotificationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Continue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsRasterizerNotificationCallback_Impl: windows_core::IUnknownImpl {
    fn Continue(&self) -> windows_core::Result<()>;
}
impl IXpsRasterizerNotificationCallback_Vtbl {
    pub const fn new<Identity: IXpsRasterizerNotificationCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Continue<Identity: IXpsRasterizerNotificationCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsRasterizerNotificationCallback_Impl::Continue(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Continue: Continue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsRasterizerNotificationCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsRasterizerNotificationCallback {}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImgErrorInfo {
    pub description: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub guid: windows_core::GUID,
    pub helpContext: u32,
    pub helpFile: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub source: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub devDescription: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub errorID: windows_core::GUID,
    pub cUserParameters: u32,
    pub aUserParameters: *mut windows_core::BSTR,
    pub userFallback: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub exceptionID: u32,
}
impl Default for ImgErrorInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IntermediatePageCount: PageCountType = PageCountType(1i32);
pub const JOB_ACCESS_ADMINISTER: u32 = 16u32;
pub const JOB_ACCESS_READ: u32 = 32u32;
pub const JOB_CONTROL_CANCEL: u32 = 3u32;
pub const JOB_CONTROL_DELETE: u32 = 5u32;
pub const JOB_CONTROL_LAST_PAGE_EJECTED: u32 = 7u32;
pub const JOB_CONTROL_PAUSE: u32 = 1u32;
pub const JOB_CONTROL_RELEASE: u32 = 9u32;
pub const JOB_CONTROL_RESTART: u32 = 4u32;
pub const JOB_CONTROL_RESUME: u32 = 2u32;
pub const JOB_CONTROL_RETAIN: u32 = 8u32;
pub const JOB_CONTROL_SEND_TOAST: u32 = 10u32;
pub const JOB_CONTROL_SENT_TO_PRINTER: u32 = 6u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOB_INFO_1A {
    pub JobId: u32,
    pub pPrinterName: windows_core::PSTR,
    pub pMachineName: windows_core::PSTR,
    pub pUserName: windows_core::PSTR,
    pub pDocument: windows_core::PSTR,
    pub pDatatype: windows_core::PSTR,
    pub pStatus: windows_core::PSTR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub TotalPages: u32,
    pub PagesPrinted: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOB_INFO_1W {
    pub JobId: u32,
    pub pPrinterName: windows_core::PWSTR,
    pub pMachineName: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub pDocument: windows_core::PWSTR,
    pub pDatatype: windows_core::PWSTR,
    pub pStatus: windows_core::PWSTR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub TotalPages: u32,
    pub PagesPrinted: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JOB_INFO_2A {
    pub JobId: u32,
    pub pPrinterName: windows_core::PSTR,
    pub pMachineName: windows_core::PSTR,
    pub pUserName: windows_core::PSTR,
    pub pDocument: windows_core::PSTR,
    pub pNotifyName: windows_core::PSTR,
    pub pDatatype: windows_core::PSTR,
    pub pPrintProcessor: windows_core::PSTR,
    pub pParameters: windows_core::PSTR,
    pub pDriverName: windows_core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pStatus: windows_core::PSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for JOB_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JOB_INFO_2W {
    pub JobId: u32,
    pub pPrinterName: windows_core::PWSTR,
    pub pMachineName: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub pDocument: windows_core::PWSTR,
    pub pNotifyName: windows_core::PWSTR,
    pub pDatatype: windows_core::PWSTR,
    pub pPrintProcessor: windows_core::PWSTR,
    pub pParameters: windows_core::PWSTR,
    pub pDriverName: windows_core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pStatus: windows_core::PWSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for JOB_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOB_INFO_3 {
    pub JobId: u32,
    pub NextJobId: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JOB_INFO_4A {
    pub JobId: u32,
    pub pPrinterName: windows_core::PSTR,
    pub pMachineName: windows_core::PSTR,
    pub pUserName: windows_core::PSTR,
    pub pDocument: windows_core::PSTR,
    pub pNotifyName: windows_core::PSTR,
    pub pDatatype: windows_core::PSTR,
    pub pPrintProcessor: windows_core::PSTR,
    pub pParameters: windows_core::PSTR,
    pub pDriverName: windows_core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pStatus: windows_core::PSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
    pub SizeHigh: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for JOB_INFO_4A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JOB_INFO_4W {
    pub JobId: u32,
    pub pPrinterName: windows_core::PWSTR,
    pub pMachineName: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub pDocument: windows_core::PWSTR,
    pub pNotifyName: windows_core::PWSTR,
    pub pDatatype: windows_core::PWSTR,
    pub pPrintProcessor: windows_core::PWSTR,
    pub pParameters: windows_core::PWSTR,
    pub pDriverName: windows_core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pStatus: windows_core::PWSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
    pub SizeHigh: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for JOB_INFO_4W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const JOB_NOTIFY_FIELD_BYTES_PRINTED: u32 = 23u32;
pub const JOB_NOTIFY_FIELD_DATATYPE: u32 = 5u32;
pub const JOB_NOTIFY_FIELD_DEVMODE: u32 = 9u32;
pub const JOB_NOTIFY_FIELD_DOCUMENT: u32 = 13u32;
pub const JOB_NOTIFY_FIELD_DRIVER_NAME: u32 = 8u32;
pub const JOB_NOTIFY_FIELD_MACHINE_NAME: u32 = 1u32;
pub const JOB_NOTIFY_FIELD_NOTIFY_NAME: u32 = 4u32;
pub const JOB_NOTIFY_FIELD_PAGES_PRINTED: u32 = 21u32;
pub const JOB_NOTIFY_FIELD_PARAMETERS: u32 = 7u32;
pub const JOB_NOTIFY_FIELD_PORT_NAME: u32 = 2u32;
pub const JOB_NOTIFY_FIELD_POSITION: u32 = 15u32;
pub const JOB_NOTIFY_FIELD_PRINTER_NAME: u32 = 0u32;
pub const JOB_NOTIFY_FIELD_PRINT_PROCESSOR: u32 = 6u32;
pub const JOB_NOTIFY_FIELD_PRIORITY: u32 = 14u32;
pub const JOB_NOTIFY_FIELD_REMOTE_JOB_ID: u32 = 24u32;
pub const JOB_NOTIFY_FIELD_SECURITY_DESCRIPTOR: u32 = 12u32;
pub const JOB_NOTIFY_FIELD_START_TIME: u32 = 17u32;
pub const JOB_NOTIFY_FIELD_STATUS: u32 = 10u32;
pub const JOB_NOTIFY_FIELD_STATUS_STRING: u32 = 11u32;
pub const JOB_NOTIFY_FIELD_SUBMITTED: u32 = 16u32;
pub const JOB_NOTIFY_FIELD_TIME: u32 = 19u32;
pub const JOB_NOTIFY_FIELD_TOTAL_BYTES: u32 = 22u32;
pub const JOB_NOTIFY_FIELD_TOTAL_PAGES: u32 = 20u32;
pub const JOB_NOTIFY_FIELD_UNTIL_TIME: u32 = 18u32;
pub const JOB_NOTIFY_FIELD_USER_NAME: u32 = 3u32;
pub const JOB_NOTIFY_TYPE: u32 = 1u32;
pub const JOB_POSITION_UNSPECIFIED: u32 = 0u32;
pub const JOB_STATUS_BLOCKED_DEVQ: u32 = 512u32;
pub const JOB_STATUS_COMPLETE: u32 = 4096u32;
pub const JOB_STATUS_DELETED: u32 = 256u32;
pub const JOB_STATUS_DELETING: u32 = 4u32;
pub const JOB_STATUS_ERROR: u32 = 2u32;
pub const JOB_STATUS_OFFLINE: u32 = 32u32;
pub const JOB_STATUS_PAPEROUT: u32 = 64u32;
pub const JOB_STATUS_PAUSED: u32 = 1u32;
pub const JOB_STATUS_PRINTED: u32 = 128u32;
pub const JOB_STATUS_PRINTING: u32 = 16u32;
pub const JOB_STATUS_RENDERING_LOCALLY: u32 = 16384u32;
pub const JOB_STATUS_RESTART: u32 = 2048u32;
pub const JOB_STATUS_RETAINED: u32 = 8192u32;
pub const JOB_STATUS_SPOOLING: u32 = 8u32;
pub const JOB_STATUS_USER_INTERVENTION: u32 = 1024u32;
#[repr(C)]
#[cfg(feature = "Win32_Devices_Display")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERNDATA {
    pub dwSize: u32,
    pub dwKernPairNum: u32,
    pub KernPair: [super::super::Devices::Display::FD_KERNINGPAIR; 1],
}
#[cfg(feature = "Win32_Devices_Display")]
impl Default for KERNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LOCAL_ONLY_REGISTRATION: PrintAsyncNotifyError = PrintAsyncNotifyError(23i32);
pub const LPR: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MAPTABLE {
    pub dwSize: u32,
    pub dwGlyphNum: u32,
    pub Trans: [TRANSDATA; 1],
}
impl Default for MAPTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAX_ADDRESS_STR_LEN: u32 = 13u32;
pub const MAX_CHANNEL_COUNT_EXCEEDED: PrintAsyncNotifyError = PrintAsyncNotifyError(22i32);
pub const MAX_CPSFUNC_INDEX: u32 = 26u32;
pub const MAX_DEVICEDESCRIPTION_STR_LEN: u32 = 257u32;
pub const MAX_DLGPAGE_COUNT: u32 = 64u32;
pub const MAX_FORM_KEYWORD_LENGTH: u32 = 64u32;
pub const MAX_IPADDR_STR_LEN: u32 = 16u32;
pub const MAX_NETWORKNAME2_LEN: u32 = 128u32;
pub const MAX_NETWORKNAME_LEN: u32 = 49u32;
pub const MAX_NOTIFICATION_SIZE_EXCEEDED: PrintAsyncNotifyError = PrintAsyncNotifyError(18i32);
pub const MAX_PORTNAME_LEN: u32 = 64u32;
pub const MAX_PRIORITY: u32 = 99u32;
pub const MAX_PROPSHEETUI_REASON_INDEX: u32 = 5u32;
pub const MAX_PSUIPAGEINSERT_INDEX: u32 = 5u32;
pub const MAX_QUEUENAME_LEN: u32 = 33u32;
pub const MAX_REGISTRATION_COUNT_EXCEEDED: PrintAsyncNotifyError = PrintAsyncNotifyError(21i32);
pub const MAX_RES_STR_CHARS: u32 = 160u32;
pub const MAX_SNMP_COMMUNITY_STR_LEN: u32 = 33u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MESSAGEBOX_PARAMS {
    pub cbSize: u32,
    pub pTitle: windows_core::PWSTR,
    pub pMessage: windows_core::PWSTR,
    pub Style: u32,
    pub dwTimeout: u32,
    pub bWait: windows_core::BOOL,
}
pub const MIN_PRIORITY: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Communication", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct MONITOR {
    pub pfnEnumPorts: PFN_PRINTING_ENUMPORTS,
    pub pfnOpenPort: PFN_PRINTING_OPENPORT,
    pub pfnOpenPortEx: PFN_PRINTING_OPENPORTEX,
    pub pfnStartDocPort: PFN_PRINTING_STARTDOCPORT,
    pub pfnWritePort: PFN_PRINTING_WRITEPORT,
    pub pfnReadPort: PFN_PRINTING_READPORT,
    pub pfnEndDocPort: PFN_PRINTING_ENDDOCPORT,
    pub pfnClosePort: PFN_PRINTING_CLOSEPORT,
    pub pfnAddPort: PFN_PRINTING_ADDPORT,
    pub pfnAddPortEx: PFN_PRINTING_ADDPORTEX,
    pub pfnConfigurePort: PFN_PRINTING_CONFIGUREPORT,
    pub pfnDeletePort: PFN_PRINTING_DELETEPORT,
    pub pfnGetPrinterDataFromPort: PFN_PRINTING_GETPRINTERDATAFROMPORT,
    pub pfnSetPortTimeOuts: PFN_PRINTING_SETPORTTIMEOUTS,
    pub pfnXcvOpenPort: PFN_PRINTING_XCVOPENPORT,
    pub pfnXcvDataPort: PFN_PRINTING_XCVDATAPORT,
    pub pfnXcvClosePort: PFN_PRINTING_XCVCLOSEPORT,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Communication", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct MONITOR2 {
    pub cbSize: u32,
    pub pfnEnumPorts: PFN_PRINTING_ENUMPORTS2,
    pub pfnOpenPort: PFN_PRINTING_OPENPORT2,
    pub pfnOpenPortEx: PFN_PRINTING_OPENPORTEX2,
    pub pfnStartDocPort: PFN_PRINTING_STARTDOCPORT2,
    pub pfnWritePort: PFN_PRINTING_WRITEPORT2,
    pub pfnReadPort: PFN_PRINTING_READPORT2,
    pub pfnEndDocPort: PFN_PRINTING_ENDDOCPORT2,
    pub pfnClosePort: PFN_PRINTING_CLOSEPORT2,
    pub pfnAddPort: PFN_PRINTING_ADDPORT2,
    pub pfnAddPortEx: PFN_PRINTING_ADDPORTEX2,
    pub pfnConfigurePort: PFN_PRINTING_CONFIGUREPORT2,
    pub pfnDeletePort: PFN_PRINTING_DELETEPORT2,
    pub pfnGetPrinterDataFromPort: PFN_PRINTING_GETPRINTERDATAFROMPORT2,
    pub pfnSetPortTimeOuts: PFN_PRINTING_SETPORTTIMEOUTS2,
    pub pfnXcvOpenPort: PFN_PRINTING_XCVOPENPORT2,
    pub pfnXcvDataPort: PFN_PRINTING_XCVDATAPORT2,
    pub pfnXcvClosePort: PFN_PRINTING_XCVCLOSEPORT2,
    pub pfnShutdown: PFN_PRINTING_SHUTDOWN2,
    pub pfnSendRecvBidiDataFromPort: PFN_PRINTING_SENDRECVBIDIDATAFROMPORT2,
    pub pfnNotifyUsedPorts: PFN_PRINTING_NOTIFYUSEDPORTS2,
    pub pfnNotifyUnusedPorts: PFN_PRINTING_NOTIFYUNUSEDPORTS2,
    pub pfnPowerEvent: PFN_PRINTING_POWEREVENT2,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Communication", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct MONITOREX {
    pub dwMonitorSize: u32,
    pub Monitor: MONITOR,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONITORINIT {
    pub cbSize: u32,
    pub hSpooler: super::super::Foundation::HANDLE,
    pub hckRegistryRoot: super::super::System::Registry::HKEY,
    pub pMonitorReg: *mut MONITORREG,
    pub bLocal: windows_core::BOOL,
    pub pszServerName: windows_core::PCWSTR,
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for MONITORINIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITORREG {
    pub cbSize: u32,
    pub fpCreateKey: isize,
    pub fpOpenKey: isize,
    pub fpCloseKey: isize,
    pub fpDeleteKey: isize,
    pub fpEnumKey: isize,
    pub fpQueryInfoKey: isize,
    pub fpSetValue: isize,
    pub fpDeleteValue: isize,
    pub fpEnumValue: isize,
    pub fpQueryValue: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITORUI {
    pub dwMonitorUISize: u32,
    pub pfnAddPortUI: isize,
    pub pfnConfigurePortUI: isize,
    pub pfnDeletePortUI: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITOR_INFO_1A {
    pub pName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITOR_INFO_1W {
    pub pName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITOR_INFO_2A {
    pub pName: windows_core::PSTR,
    pub pEnvironment: windows_core::PSTR,
    pub pDLLName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITOR_INFO_2W {
    pub pName: windows_core::PWSTR,
    pub pEnvironment: windows_core::PWSTR,
    pub pDLLName: windows_core::PWSTR,
}
pub const MS_PRINT_JOB_OUTPUT_FILE: windows_core::PCWSTR = windows_core::w!("MsPrintJobOutputFile");
pub const MTYPE_ADD: u32 = 64u32;
pub const MTYPE_COMPOSE: u32 = 1u32;
pub const MTYPE_DIRECT: u32 = 2u32;
pub const MTYPE_DISABLE: u32 = 128u32;
pub const MTYPE_DOUBLE: u32 = 16u32;
pub const MTYPE_DOUBLEBYTECHAR_MASK: u32 = 24u32;
pub const MTYPE_FORMAT_MASK: u32 = 7u32;
pub const MTYPE_PAIRED: u32 = 4u32;
pub const MTYPE_PREDEFIN_MASK: u32 = 224u32;
pub const MTYPE_REPLACE: u32 = 32u32;
pub const MTYPE_SINGLE: u32 = 8u32;
pub const MV_GRAPHICS: u32 = 4u32;
pub const MV_PHYSICAL: u32 = 8u32;
pub const MV_RELATIVE: u32 = 2u32;
pub const MV_SENDXMOVECMD: u32 = 16u32;
pub const MV_SENDYMOVECMD: u32 = 32u32;
pub const MV_UPDATE: u32 = 1u32;
pub const MXDCOP_GET_FILENAME: u32 = 14u32;
pub const MXDCOP_PRINTTICKET_FIXED_DOC: u32 = 24u32;
pub const MXDCOP_PRINTTICKET_FIXED_DOC_SEQ: u32 = 22u32;
pub const MXDCOP_PRINTTICKET_FIXED_PAGE: u32 = 26u32;
pub const MXDCOP_SET_S0PAGE: u32 = 28u32;
pub const MXDCOP_SET_S0PAGE_RESOURCE: u32 = 30u32;
pub const MXDCOP_SET_XPSPASSTHRU_MODE: u32 = 32u32;
pub const MXDC_ESCAPE: u32 = 4122u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MXDC_ESCAPE_HEADER_T {
    pub cbInput: u32,
    pub cbOutput: u32,
    pub opCode: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MXDC_GET_FILENAME_DATA_T {
    pub cbOutput: u32,
    pub wszData: [u16; 1],
}
impl Default for MXDC_GET_FILENAME_DATA_T {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MXDC_IMAGETYPE_JPEGHIGH_COMPRESSION: MXDC_IMAGE_TYPE_ENUMS = MXDC_IMAGE_TYPE_ENUMS(1i32);
pub const MXDC_IMAGETYPE_JPEGLOW_COMPRESSION: MXDC_IMAGE_TYPE_ENUMS = MXDC_IMAGE_TYPE_ENUMS(3i32);
pub const MXDC_IMAGETYPE_JPEGMEDIUM_COMPRESSION: MXDC_IMAGE_TYPE_ENUMS = MXDC_IMAGE_TYPE_ENUMS(2i32);
pub const MXDC_IMAGETYPE_PNG: MXDC_IMAGE_TYPE_ENUMS = MXDC_IMAGE_TYPE_ENUMS(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MXDC_IMAGE_TYPE_ENUMS(pub i32);
pub const MXDC_LANDSCAPE_ROTATE_COUNTERCLOCKWISE_270_DEGREES: MXDC_LANDSCAPE_ROTATION_ENUMS = MXDC_LANDSCAPE_ROTATION_ENUMS(-90i32);
pub const MXDC_LANDSCAPE_ROTATE_COUNTERCLOCKWISE_90_DEGREES: MXDC_LANDSCAPE_ROTATION_ENUMS = MXDC_LANDSCAPE_ROTATION_ENUMS(90i32);
pub const MXDC_LANDSCAPE_ROTATE_NONE: MXDC_LANDSCAPE_ROTATION_ENUMS = MXDC_LANDSCAPE_ROTATION_ENUMS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MXDC_LANDSCAPE_ROTATION_ENUMS(pub i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MXDC_PRINTTICKET_DATA_T {
    pub dwDataSize: u32,
    pub bData: [u8; 1],
}
impl Default for MXDC_PRINTTICKET_DATA_T {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MXDC_PRINTTICKET_ESCAPE_T {
    pub mxdcEscape: MXDC_ESCAPE_HEADER_T,
    pub printTicketData: MXDC_PRINTTICKET_DATA_T,
}
pub const MXDC_RESOURCE_DICTIONARY: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(5i32);
pub const MXDC_RESOURCE_ICC_PROFILE: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(6i32);
pub const MXDC_RESOURCE_JPEG: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(1i32);
pub const MXDC_RESOURCE_JPEG_THUMBNAIL: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(7i32);
pub const MXDC_RESOURCE_MAX: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(9i32);
pub const MXDC_RESOURCE_PNG: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(2i32);
pub const MXDC_RESOURCE_PNG_THUMBNAIL: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(8i32);
pub const MXDC_RESOURCE_TIFF: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(3i32);
pub const MXDC_RESOURCE_TTF: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(0i32);
pub const MXDC_RESOURCE_WDP: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(4i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MXDC_S0PAGE_DATA_T {
    pub dwSize: u32,
    pub bData: [u8; 1],
}
impl Default for MXDC_S0PAGE_DATA_T {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MXDC_S0PAGE_PASSTHROUGH_ESCAPE_T {
    pub mxdcEscape: MXDC_ESCAPE_HEADER_T,
    pub xpsS0PageData: MXDC_S0PAGE_DATA_T,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MXDC_S0PAGE_RESOURCE_ESCAPE_T {
    pub mxdcEscape: MXDC_ESCAPE_HEADER_T,
    pub xpsS0PageResourcePassthrough: MXDC_XPS_S0PAGE_RESOURCE_T,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MXDC_S0_PAGE_ENUMS(pub i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MXDC_XPS_S0PAGE_RESOURCE_T {
    pub dwSize: u32,
    pub dwResourceType: u32,
    pub szUri: [u8; 260],
    pub dwDataSize: u32,
    pub bData: [u8; 1],
}
impl Default for MXDC_XPS_S0PAGE_RESOURCE_T {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NORMAL_PRINT: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NOTIFICATION_CALLBACK_COMMANDS(pub i32);
pub const NOTIFICATION_COMMAND_CONTEXT_ACQUIRE: NOTIFICATION_CALLBACK_COMMANDS = NOTIFICATION_CALLBACK_COMMANDS(1i32);
pub const NOTIFICATION_COMMAND_CONTEXT_RELEASE: NOTIFICATION_CALLBACK_COMMANDS = NOTIFICATION_CALLBACK_COMMANDS(2i32);
pub const NOTIFICATION_COMMAND_NOTIFY: NOTIFICATION_CALLBACK_COMMANDS = NOTIFICATION_CALLBACK_COMMANDS(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NOTIFICATION_CONFIG_1 {
    pub cbSize: u32,
    pub fdwFlags: u32,
    pub pfnNotifyCallback: ROUTER_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
}
impl Default for NOTIFICATION_CONFIG_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NOTIFICATION_CONFIG_ASYNC_CHANNEL: NOTIFICATION_CONFIG_FLAGS = NOTIFICATION_CONFIG_FLAGS(8i32);
pub const NOTIFICATION_CONFIG_CREATE_EVENT: NOTIFICATION_CONFIG_FLAGS = NOTIFICATION_CONFIG_FLAGS(1i32);
pub const NOTIFICATION_CONFIG_EVENT_TRIGGER: NOTIFICATION_CONFIG_FLAGS = NOTIFICATION_CONFIG_FLAGS(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NOTIFICATION_CONFIG_FLAGS(pub i32);
pub const NOTIFICATION_CONFIG_REGISTER_CALLBACK: NOTIFICATION_CONFIG_FLAGS = NOTIFICATION_CONFIG_FLAGS(2i32);
pub const NOTIFICATION_RELEASE: windows_core::GUID = windows_core::GUID::from_u128(0xba9a5027_a70e_4ae7_9b7d_eb3e06ad4157);
pub const NOT_REGISTERED: PrintAsyncNotifyError = PrintAsyncNotifyError(13i32);
pub const NO_BORDER_PRINT: u32 = 1u32;
pub const NO_COLOR_OPTIMIZATION: u32 = 0u32;
pub const NO_LISTENERS: PrintAsyncNotifyError = PrintAsyncNotifyError(7i32);
pub const NO_PRIORITY: u32 = 0u32;
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type OEMCUIPCALLBACK = Option<unsafe extern "system" fn(param0: *mut CPSUICBPARAM, param1: *mut OEMCUIPPARAM) -> i32>;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug)]
pub struct OEMCUIPPARAM {
    pub cbSize: u32,
    pub poemuiobj: *mut OEMUIOBJ,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pPrinterName: windows_core::PWSTR,
    pub hModule: super::super::Foundation::HANDLE,
    pub hOEMHeap: super::super::Foundation::HANDLE,
    pub pPublicDM: *mut super::Gdi::DEVMODEA,
    pub pOEMDM: *mut core::ffi::c_void,
    pub dwFlags: u32,
    pub pDrvOptItems: *mut OPTITEM,
    pub cDrvOptItems: u32,
    pub pOEMOptItems: *mut OPTITEM,
    pub cOEMOptItems: u32,
    pub pOEMUserData: *mut core::ffi::c_void,
    pub OEMCUIPCallback: OEMCUIPCALLBACK,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OEMCUIPPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OEMCUIP_DOCPROP: u32 = 1u32;
pub const OEMCUIP_PRNPROP: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OEMDMPARAM {
    pub cbSize: u32,
    pub pdriverobj: *mut core::ffi::c_void,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub hModule: super::super::Foundation::HANDLE,
    pub pPublicDMIn: *mut super::Gdi::DEVMODEA,
    pub pPublicDMOut: *mut super::Gdi::DEVMODEA,
    pub pOEMDMIn: *mut core::ffi::c_void,
    pub pOEMDMOut: *mut core::ffi::c_void,
    pub cbBufSize: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for OEMDMPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OEMDM_CONVERT: u32 = 3u32;
pub const OEMDM_DEFAULT: u32 = 2u32;
pub const OEMDM_MERGE: u32 = 4u32;
pub const OEMDM_SIZE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OEMFONTINSTPARAM {
    pub cbSize: u32,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub hModule: super::super::Foundation::HANDLE,
    pub hHeap: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
    pub pFontInstallerName: windows_core::PWSTR,
}
pub const OEMGDS_FREEMEM: u32 = 32769u32;
pub const OEMGDS_JOBTIMEOUT: u32 = 32770u32;
pub const OEMGDS_MAX: u32 = 65536u32;
pub const OEMGDS_MAXBITMAP: u32 = 32774u32;
pub const OEMGDS_MINOUTLINE: u32 = 32773u32;
pub const OEMGDS_MIN_DOCSTICKY: u32 = 1u32;
pub const OEMGDS_MIN_PRINTERSTICKY: u32 = 32768u32;
pub const OEMGDS_PRINTFLAGS: u32 = 32768u32;
pub const OEMGDS_PROTOCOL: u32 = 32772u32;
pub const OEMGDS_PSDM_CUSTOMSIZE: u32 = 6u32;
pub const OEMGDS_PSDM_DIALECT: u32 = 2u32;
pub const OEMGDS_PSDM_FLAGS: u32 = 1u32;
pub const OEMGDS_PSDM_NUP: u32 = 4u32;
pub const OEMGDS_PSDM_PSLEVEL: u32 = 5u32;
pub const OEMGDS_PSDM_TTDLFMT: u32 = 3u32;
pub const OEMGDS_UNIDM_FLAGS: u32 = 16385u32;
pub const OEMGDS_UNIDM_GPDVER: u32 = 16384u32;
pub const OEMGDS_WAITTIMEOUT: u32 = 32771u32;
pub const OEMGI_GETINTERFACEVERSION: u32 = 2u32;
pub const OEMGI_GETPUBLISHERINFO: u32 = 4u32;
pub const OEMGI_GETREQUESTEDHELPERINTERFACES: u32 = 5u32;
pub const OEMGI_GETSIGNATURE: u32 = 1u32;
pub const OEMGI_GETVERSION: u32 = 3u32;
pub const OEMPUBLISH_DEFAULT: u32 = 0u32;
pub const OEMPUBLISH_IPRINTCOREHELPER: u32 = 1u32;
pub const OEMTTY_INFO_CODEPAGE: u32 = 2u32;
pub const OEMTTY_INFO_MARGINS: u32 = 1u32;
pub const OEMTTY_INFO_NUM_UFMS: u32 = 3u32;
pub const OEMTTY_INFO_UFM_IDS: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OEMUIOBJ {
    pub cbSize: u32,
    pub pOemUIProcs: *mut OEMUIPROCS,
}
impl Default for OEMUIOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OEMUIPROCS {
    pub DrvGetDriverSetting: PFN_DrvGetDriverSetting,
    pub DrvUpdateUISetting: PFN_DrvUpdateUISetting,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OEMUIPSPARAM {
    pub cbSize: u32,
    pub poemuiobj: *mut OEMUIOBJ,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pPrinterName: windows_core::PWSTR,
    pub hModule: super::super::Foundation::HANDLE,
    pub hOEMHeap: super::super::Foundation::HANDLE,
    pub pPublicDM: *mut super::Gdi::DEVMODEA,
    pub pOEMDM: *mut core::ffi::c_void,
    pub pOEMUserData: *mut core::ffi::c_void,
    pub dwFlags: u32,
    pub pOemEntry: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for OEMUIPSPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OEM_DMEXTRAHEADER {
    pub dwSize: u32,
    pub dwSignature: u32,
    pub dwVersion: u32,
}
pub const OEM_MODE_PUBLISHER: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OIEXT {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::super::Foundation::HINSTANCE,
    pub pHelpFile: *mut i8,
    pub dwReserved: [usize; 4],
}
impl Default for OIEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OIEXTF_ANSI_STRING: u32 = 1u32;
pub const OPTCF_HIDE: u32 = 1u32;
pub const OPTCF_MASK: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPTCOMBO {
    pub cbSize: u16,
    pub Flags: u8,
    pub cListItem: u16,
    pub pListItem: *mut OPTPARAM,
    pub Sel: i32,
    pub dwReserved: [u32; 3],
}
impl Default for OPTCOMBO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPTIF_CALLBACK: i32 = 4i32;
pub const OPTIF_CHANGED: i32 = 8i32;
pub const OPTIF_CHANGEONCE: i32 = 16i32;
pub const OPTIF_COLLAPSE: i32 = 1i32;
pub const OPTIF_DISABLED: i32 = 32i32;
pub const OPTIF_ECB_CHECKED: i32 = 64i32;
pub const OPTIF_EXT_DISABLED: i32 = 256i32;
pub const OPTIF_EXT_HIDE: i32 = 128i32;
pub const OPTIF_EXT_IS_EXTPUSH: i32 = 1024i32;
pub const OPTIF_HAS_POIEXT: i32 = 65536i32;
pub const OPTIF_HIDE: i32 = 2i32;
pub const OPTIF_INITIAL_TVITEM: i32 = 32768i32;
pub const OPTIF_MASK: i32 = 131071i32;
pub const OPTIF_NO_GROUPBOX_NAME: i32 = 2048i32;
pub const OPTIF_OVERLAY_NO_ICON: i32 = 16384i32;
pub const OPTIF_OVERLAY_STOP_ICON: i32 = 8192i32;
pub const OPTIF_OVERLAY_WARNING_ICON: i32 = 4096i32;
pub const OPTIF_SEL_AS_HICON: i32 = 512i32;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct OPTITEM {
    pub cbSize: u16,
    pub Level: u8,
    pub DlgPageIdx: u8,
    pub Flags: u32,
    pub UserData: usize,
    pub pName: *mut i8,
    pub Anonymous1: OPTITEM_0,
    pub Anonymous2: OPTITEM_1,
    pub pOptType: *mut OPTTYPE,
    pub HelpIndex: u32,
    pub DMPubID: u8,
    pub UserItemID: u8,
    pub wReserved: u16,
    pub pOIExt: *mut OIEXT,
    pub dwReserved: [usize; 3],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OPTITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union OPTITEM_0 {
    pub Sel: i32,
    pub pSel: *mut i8,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OPTITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union OPTITEM_1 {
    pub pExtChkBox: *mut EXTCHKBOX,
    pub pExtPush: *mut EXTPUSH,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OPTITEM_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPTPARAM {
    pub cbSize: u16,
    pub Flags: u8,
    pub Style: u8,
    pub pData: *mut i8,
    pub IconID: usize,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwReserved: [usize; 2],
}
impl Default for OPTPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPTPF_DISABLED: u32 = 2u32;
pub const OPTPF_HIDE: u32 = 1u32;
pub const OPTPF_ICONID_AS_HICON: u32 = 4u32;
pub const OPTPF_MASK: u32 = 127u32;
pub const OPTPF_OVERLAY_NO_ICON: u32 = 32u32;
pub const OPTPF_OVERLAY_STOP_ICON: u32 = 16u32;
pub const OPTPF_OVERLAY_WARNING_ICON: u32 = 8u32;
pub const OPTPF_USE_HDLGTEMPLATE: u32 = 64u32;
pub const OPTTF_MASK: u32 = 3u32;
pub const OPTTF_NOSPACE_BEFORE_POSTFIX: u32 = 2u32;
pub const OPTTF_TYPE_DISABLED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPTTYPE {
    pub cbSize: u16,
    pub Type: u8,
    pub Flags: u8,
    pub Count: u16,
    pub BegCtrlID: u16,
    pub pOptParam: *mut OPTPARAM,
    pub Style: u16,
    pub wReserved: [u16; 3],
    pub dwReserved: [usize; 3],
}
impl Default for OPTTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OTS_LBCB_INCL_ITEM_NONE: u32 = 8u32;
pub const OTS_LBCB_NO_ICON16_IN_ITEM: u32 = 16u32;
pub const OTS_LBCB_PROPPAGE_CBUSELB: u32 = 4u32;
pub const OTS_LBCB_PROPPAGE_LBUSECB: u32 = 2u32;
pub const OTS_LBCB_SORT: u32 = 1u32;
pub const OTS_MASK: u32 = 255u32;
pub const OTS_PUSH_ENABLE_ALWAYS: u32 = 128u32;
pub const OTS_PUSH_INCL_SETUP_TITLE: u32 = 32u32;
pub const OTS_PUSH_NO_DOT_DOT_DOT: u32 = 64u32;
pub const PDEV_ADJUST_PAPER_MARGIN_TYPE: u32 = 1u32;
pub const PDEV_HOSTFONT_ENABLED_TYPE: u32 = 2u32;
pub const PDEV_USE_TRUE_COLOR_TYPE: u32 = 3u32;
pub type PFNCOMPROPSHEET = Option<unsafe extern "system" fn(hcompropsheet: super::super::Foundation::HANDLE, function: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> isize>;
pub type PFNPROPSHEETUI = Option<unsafe extern "system" fn(ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> i32>;
pub type PFN_DrvGetDriverSetting = Option<unsafe extern "system" fn(pdriverobj: *mut core::ffi::c_void, feature: windows_core::PCSTR, poutput: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> windows_core::BOOL>;
pub type PFN_DrvUpdateUISetting = Option<unsafe extern "system" fn(pdriverobj: *mut core::ffi::c_void, poptitem: *mut core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> windows_core::BOOL>;
pub type PFN_DrvUpgradeRegistrySetting = Option<unsafe extern "system" fn(hprinter: super::super::Foundation::HANDLE, pfeature: windows_core::PCSTR, poption: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PFN_PRINTING_ADDPORT = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::HWND, param2: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFN_PRINTING_ADDPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: super::super::Foundation::HWND, param3: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFN_PRINTING_ADDPORTEX = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: *const u8, param3: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFN_PRINTING_ADDPORTEX2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: u32, param3: *const u8, param4: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFN_PRINTING_CLOSEPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFN_PRINTING_CLOSEPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFN_PRINTING_CONFIGUREPORT = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::HWND, param2: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFN_PRINTING_CONFIGUREPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: super::super::Foundation::HWND, param3: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFN_PRINTING_DELETEPORT = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::HWND, param2: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFN_PRINTING_DELETEPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: super::super::Foundation::HWND, param3: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFN_PRINTING_ENDDOCPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFN_PRINTING_ENDDOCPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFN_PRINTING_ENUMPORTS = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: *mut u8, param3: u32, param4: *mut u32, param5: *mut u32) -> windows_core::BOOL>;
pub type PFN_PRINTING_ENUMPORTS2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: u32, param3: *mut u8, param4: u32, param5: *mut u32, param6: *mut u32) -> windows_core::BOOL>;
pub type PFN_PRINTING_GETPRINTERDATAFROMPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: windows_core::PCWSTR, param3: windows_core::PCWSTR, param4: u32, param5: windows_core::PCWSTR, param6: u32, param7: *mut u32) -> windows_core::BOOL>;
pub type PFN_PRINTING_GETPRINTERDATAFROMPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: windows_core::PCWSTR, param3: windows_core::PCWSTR, param4: u32, param5: windows_core::PCWSTR, param6: u32, param7: *mut u32) -> windows_core::BOOL>;
pub type PFN_PRINTING_NOTIFYUNUSEDPORTS2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: windows_core::PCWSTR) -> u32>;
pub type PFN_PRINTING_NOTIFYUSEDPORTS2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: windows_core::PCWSTR) -> u32>;
pub type PFN_PRINTING_OPENPORT = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: *mut super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFN_PRINTING_OPENPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: *mut super::super::Foundation::HANDLE) -> windows_core::BOOL>;
#[cfg(all(feature = "Win32_Devices_Communication", feature = "Win32_System_Power"))]
pub type PFN_PRINTING_OPENPORTEX = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: *mut super::super::Foundation::HANDLE, param4: *const MONITOR2) -> windows_core::BOOL>;
#[cfg(all(feature = "Win32_Devices_Communication", feature = "Win32_System_Power"))]
pub type PFN_PRINTING_OPENPORTEX2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: windows_core::PCWSTR, param3: windows_core::PCWSTR, param4: *mut super::super::Foundation::HANDLE, param5: *const MONITOR2) -> windows_core::BOOL>;
#[cfg(feature = "Win32_System_Power")]
pub type PFN_PRINTING_POWEREVENT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *const super::super::System::Power::POWERBROADCAST_SETTING) -> u32>;
pub type PFN_PRINTING_READPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut u8, param2: u32, param3: *mut u32) -> windows_core::BOOL>;
pub type PFN_PRINTING_READPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut u8, param2: u32, param3: *mut u32) -> windows_core::BOOL>;
pub type PFN_PRINTING_SENDRECVBIDIDATAFROMPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: windows_core::PCWSTR, param3: *const BIDI_REQUEST_CONTAINER, param4: *mut *mut BIDI_RESPONSE_CONTAINER) -> u32>;
#[cfg(feature = "Win32_Devices_Communication")]
pub type PFN_PRINTING_SETPORTTIMEOUTS = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *const super::super::Devices::Communication::COMMTIMEOUTS, param2: u32) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Devices_Communication")]
pub type PFN_PRINTING_SETPORTTIMEOUTS2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *const super::super::Devices::Communication::COMMTIMEOUTS, param2: u32) -> windows_core::BOOL>;
pub type PFN_PRINTING_SHUTDOWN2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE)>;
pub type PFN_PRINTING_STARTDOCPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: u32, param3: u32, param4: *const u8) -> windows_core::BOOL>;
pub type PFN_PRINTING_STARTDOCPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: u32, param3: u32, param4: *const u8) -> windows_core::BOOL>;
pub type PFN_PRINTING_WRITEPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *const u8, param2: u32, param3: *mut u32) -> windows_core::BOOL>;
pub type PFN_PRINTING_WRITEPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *const u8, param2: u32, param3: *mut u32) -> windows_core::BOOL>;
pub type PFN_PRINTING_XCVCLOSEPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFN_PRINTING_XCVCLOSEPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFN_PRINTING_XCVDATAPORT = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: *const u8, param3: u32, param4: *mut u8, param5: u32, param6: *mut u32) -> u32>;
pub type PFN_PRINTING_XCVDATAPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: *const u8, param3: u32, param4: *mut u8, param5: u32, param6: *mut u32) -> u32>;
pub type PFN_PRINTING_XCVOPENPORT = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: u32, param2: *mut super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFN_PRINTING_XCVOPENPORT2 = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: windows_core::PCWSTR, param2: u32, param3: *mut super::super::Foundation::HANDLE) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PORT_DATA_1 {
    pub sztPortName: [u16; 64],
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub cbSize: u32,
    pub dwReserved: u32,
    pub sztHostAddress: [u16; 49],
    pub sztSNMPCommunity: [u16; 33],
    pub dwDoubleSpool: u32,
    pub sztQueue: [u16; 33],
    pub sztIPAddress: [u16; 16],
    pub Reserved: [u8; 540],
    pub dwPortNumber: u32,
    pub dwSNMPEnabled: u32,
    pub dwSNMPDevIndex: u32,
}
impl Default for PORT_DATA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PORT_DATA_2 {
    pub sztPortName: [u16; 64],
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub cbSize: u32,
    pub dwReserved: u32,
    pub sztHostAddress: [u16; 128],
    pub sztSNMPCommunity: [u16; 33],
    pub dwDoubleSpool: u32,
    pub sztQueue: [u16; 33],
    pub Reserved: [u8; 514],
    pub dwPortNumber: u32,
    pub dwSNMPEnabled: u32,
    pub dwSNMPDevIndex: u32,
    pub dwPortMonitorMibIndex: u32,
}
impl Default for PORT_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PORT_DATA_LIST_1 {
    pub dwVersion: u32,
    pub cPortData: u32,
    pub pPortData: [PORT_DATA_2; 1],
}
impl Default for PORT_DATA_LIST_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PORT_INFO_1A {
    pub pName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PORT_INFO_1W {
    pub pName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PORT_INFO_2A {
    pub pPortName: windows_core::PSTR,
    pub pMonitorName: windows_core::PSTR,
    pub pDescription: windows_core::PSTR,
    pub fPortType: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PORT_INFO_2W {
    pub pPortName: windows_core::PWSTR,
    pub pMonitorName: windows_core::PWSTR,
    pub pDescription: windows_core::PWSTR,
    pub fPortType: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PORT_INFO_3A {
    pub dwStatus: u32,
    pub pszStatus: windows_core::PSTR,
    pub dwSeverity: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PORT_INFO_3W {
    pub dwStatus: u32,
    pub pszStatus: windows_core::PWSTR,
    pub dwSeverity: u32,
}
pub const PORT_STATUS_DOOR_OPEN: u32 = 7u32;
pub const PORT_STATUS_NO_TONER: u32 = 6u32;
pub const PORT_STATUS_OFFLINE: u32 = 1u32;
pub const PORT_STATUS_OUTPUT_BIN_FULL: u32 = 4u32;
pub const PORT_STATUS_OUT_OF_MEMORY: u32 = 9u32;
pub const PORT_STATUS_PAPER_JAM: u32 = 2u32;
pub const PORT_STATUS_PAPER_OUT: u32 = 3u32;
pub const PORT_STATUS_PAPER_PROBLEM: u32 = 5u32;
pub const PORT_STATUS_POWER_SAVE: u32 = 12u32;
pub const PORT_STATUS_TONER_LOW: u32 = 10u32;
pub const PORT_STATUS_TYPE_ERROR: u32 = 1u32;
pub const PORT_STATUS_TYPE_INFO: u32 = 3u32;
pub const PORT_STATUS_TYPE_WARNING: u32 = 2u32;
pub const PORT_STATUS_USER_INTERVENTION: u32 = 8u32;
pub const PORT_STATUS_WARMING_UP: u32 = 11u32;
pub const PORT_TYPE_NET_ATTACHED: u32 = 8u32;
pub const PORT_TYPE_READ: u32 = 2u32;
pub const PORT_TYPE_REDIRECTED: u32 = 4u32;
pub const PORT_TYPE_WRITE: u32 = 1u32;
pub const PPCAPS_BOOKLET_EDGE: u32 = 1u32;
pub const PPCAPS_BORDER_PRINT: u32 = 1u32;
pub const PPCAPS_REVERSE_PAGES_FOR_REVERSE_DUPLEX: u32 = 1u32;
pub const PPCAPS_RIGHT_THEN_DOWN: u32 = 1u32;
pub const PPCAPS_SQUARE_SCALING: u32 = 1u32;
pub const PRINTER_ACCESS_ADMINISTER: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(4u32);
pub const PRINTER_ACCESS_MANAGE_LIMITED: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(64u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PRINTER_ACCESS_RIGHTS(pub u32);
impl PRINTER_ACCESS_RIGHTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PRINTER_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PRINTER_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PRINTER_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PRINTER_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PRINTER_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PRINTER_ACCESS_USE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(8u32);
pub const PRINTER_ALL_ACCESS: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(983052u32);
pub const PRINTER_ATTRIBUTE_DEFAULT: u32 = 4u32;
pub const PRINTER_ATTRIBUTE_DIRECT: u32 = 2u32;
pub const PRINTER_ATTRIBUTE_DO_COMPLETE_FIRST: u32 = 512u32;
pub const PRINTER_ATTRIBUTE_ENABLE_BIDI: u32 = 2048u32;
pub const PRINTER_ATTRIBUTE_ENABLE_DEVQ: u32 = 128u32;
pub const PRINTER_ATTRIBUTE_ENTERPRISE_CLOUD: u32 = 8388608u32;
pub const PRINTER_ATTRIBUTE_FAX: u32 = 16384u32;
pub const PRINTER_ATTRIBUTE_FRIENDLY_NAME: u32 = 1048576u32;
pub const PRINTER_ATTRIBUTE_HIDDEN: u32 = 32u32;
pub const PRINTER_ATTRIBUTE_KEEPPRINTEDJOBS: u32 = 256u32;
pub const PRINTER_ATTRIBUTE_LOCAL: u32 = 64u32;
pub const PRINTER_ATTRIBUTE_MACHINE: u32 = 524288u32;
pub const PRINTER_ATTRIBUTE_NETWORK: u32 = 16u32;
pub const PRINTER_ATTRIBUTE_PER_USER: u32 = 4194304u32;
pub const PRINTER_ATTRIBUTE_PUBLISHED: u32 = 8192u32;
pub const PRINTER_ATTRIBUTE_PUSHED_MACHINE: u32 = 262144u32;
pub const PRINTER_ATTRIBUTE_PUSHED_USER: u32 = 131072u32;
pub const PRINTER_ATTRIBUTE_QUEUED: u32 = 1u32;
pub const PRINTER_ATTRIBUTE_RAW_ONLY: u32 = 4096u32;
pub const PRINTER_ATTRIBUTE_SHARED: u32 = 8u32;
pub const PRINTER_ATTRIBUTE_TS: u32 = 32768u32;
pub const PRINTER_ATTRIBUTE_TS_GENERIC_DRIVER: u32 = 2097152u32;
pub const PRINTER_ATTRIBUTE_WORK_OFFLINE: u32 = 1024u32;
pub const PRINTER_CHANGE_ADD_FORM: u32 = 65536u32;
pub const PRINTER_CHANGE_ADD_JOB: u32 = 256u32;
pub const PRINTER_CHANGE_ADD_PORT: u32 = 1048576u32;
pub const PRINTER_CHANGE_ADD_PRINTER: u32 = 1u32;
pub const PRINTER_CHANGE_ADD_PRINTER_DRIVER: u32 = 268435456u32;
pub const PRINTER_CHANGE_ADD_PRINT_PROCESSOR: u32 = 16777216u32;
pub const PRINTER_CHANGE_ALL: u32 = 2138570751u32;
pub const PRINTER_CHANGE_CONFIGURE_PORT: u32 = 2097152u32;
pub const PRINTER_CHANGE_DELETE_FORM: u32 = 262144u32;
pub const PRINTER_CHANGE_DELETE_JOB: u32 = 1024u32;
pub const PRINTER_CHANGE_DELETE_PORT: u32 = 4194304u32;
pub const PRINTER_CHANGE_DELETE_PRINTER: u32 = 4u32;
pub const PRINTER_CHANGE_DELETE_PRINTER_DRIVER: u32 = 1073741824u32;
pub const PRINTER_CHANGE_DELETE_PRINT_PROCESSOR: u32 = 67108864u32;
pub const PRINTER_CHANGE_FAILED_CONNECTION_PRINTER: u32 = 8u32;
pub const PRINTER_CHANGE_FORM: u32 = 458752u32;
pub const PRINTER_CHANGE_JOB: u32 = 65280u32;
pub const PRINTER_CHANGE_PORT: u32 = 7340032u32;
pub const PRINTER_CHANGE_PRINTER: u32 = 255u32;
pub const PRINTER_CHANGE_PRINTER_DRIVER: u32 = 1879048192u32;
pub const PRINTER_CHANGE_PRINT_PROCESSOR: u32 = 117440512u32;
pub const PRINTER_CHANGE_SERVER: u32 = 134217728u32;
pub const PRINTER_CHANGE_SET_FORM: u32 = 131072u32;
pub const PRINTER_CHANGE_SET_JOB: u32 = 512u32;
pub const PRINTER_CHANGE_SET_PRINTER: u32 = 2u32;
pub const PRINTER_CHANGE_SET_PRINTER_DRIVER: u32 = 536870912u32;
pub const PRINTER_CHANGE_TIMEOUT: u32 = 2147483648u32;
pub const PRINTER_CHANGE_WRITE_JOB: u32 = 2048u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_CONNECTION_INFO_1A {
    pub dwFlags: u32,
    pub pszDriverName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_CONNECTION_INFO_1W {
    pub dwFlags: u32,
    pub pszDriverName: windows_core::PWSTR,
}
pub const PRINTER_CONNECTION_MISMATCH: u32 = 32u32;
pub const PRINTER_CONNECTION_NO_UI: u32 = 64u32;
pub const PRINTER_CONTROL_PAUSE: u32 = 1u32;
pub const PRINTER_CONTROL_PURGE: u32 = 3u32;
pub const PRINTER_CONTROL_RESUME: u32 = 2u32;
pub const PRINTER_CONTROL_SET_STATUS: u32 = 4u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_DEFAULTSA {
    pub pDatatype: windows_core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub DesiredAccess: PRINTER_ACCESS_RIGHTS,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_DEFAULTSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_DEFAULTSW {
    pub pDatatype: windows_core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub DesiredAccess: PRINTER_ACCESS_RIGHTS,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_DEFAULTSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_DELETE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(65536u32);
pub const PRINTER_DRIVER_CATEGORY_3D: u32 = 4096u32;
pub const PRINTER_DRIVER_CATEGORY_CLOUD: u32 = 8192u32;
pub const PRINTER_DRIVER_CATEGORY_FAX: u32 = 64u32;
pub const PRINTER_DRIVER_CATEGORY_FILE: u32 = 128u32;
pub const PRINTER_DRIVER_CATEGORY_SERVICE: u32 = 512u32;
pub const PRINTER_DRIVER_CATEGORY_VIRTUAL: u32 = 256u32;
pub const PRINTER_DRIVER_CLASS: u32 = 8u32;
pub const PRINTER_DRIVER_DERIVED: u32 = 16u32;
pub const PRINTER_DRIVER_NOT_SHAREABLE: u32 = 32u32;
pub const PRINTER_DRIVER_PACKAGE_AWARE: u32 = 1u32;
pub const PRINTER_DRIVER_SANDBOX_DISABLED: u32 = 2048u32;
pub const PRINTER_DRIVER_SANDBOX_ENABLED: u32 = 4u32;
pub const PRINTER_DRIVER_SOFT_RESET_REQUIRED: u32 = 1024u32;
pub const PRINTER_DRIVER_XPS: u32 = 2u32;
pub const PRINTER_ENUM_CATEGORY_3D: u32 = 67108864u32;
pub const PRINTER_ENUM_CATEGORY_ALL: u32 = 33554432u32;
pub const PRINTER_ENUM_CONNECTIONS: u32 = 4u32;
pub const PRINTER_ENUM_CONTAINER: u32 = 32768u32;
pub const PRINTER_ENUM_DEFAULT: u32 = 1u32;
pub const PRINTER_ENUM_EXPAND: u32 = 16384u32;
pub const PRINTER_ENUM_FAVORITE: u32 = 4u32;
pub const PRINTER_ENUM_HIDE: u32 = 16777216u32;
pub const PRINTER_ENUM_ICON1: u32 = 65536u32;
pub const PRINTER_ENUM_ICON2: u32 = 131072u32;
pub const PRINTER_ENUM_ICON3: u32 = 262144u32;
pub const PRINTER_ENUM_ICON4: u32 = 524288u32;
pub const PRINTER_ENUM_ICON5: u32 = 1048576u32;
pub const PRINTER_ENUM_ICON6: u32 = 2097152u32;
pub const PRINTER_ENUM_ICON7: u32 = 4194304u32;
pub const PRINTER_ENUM_ICON8: u32 = 8388608u32;
pub const PRINTER_ENUM_ICONMASK: u32 = 16711680u32;
pub const PRINTER_ENUM_LOCAL: u32 = 2u32;
pub const PRINTER_ENUM_NAME: u32 = 8u32;
pub const PRINTER_ENUM_NETWORK: u32 = 64u32;
pub const PRINTER_ENUM_REMOTE: u32 = 16u32;
pub const PRINTER_ENUM_SHARED: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_ENUM_VALUESA {
    pub pValueName: windows_core::PSTR,
    pub cbValueName: u32,
    pub dwType: u32,
    pub pData: *mut u8,
    pub cbData: u32,
}
impl Default for PRINTER_ENUM_VALUESA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_ENUM_VALUESW {
    pub pValueName: windows_core::PWSTR,
    pub cbValueName: u32,
    pub dwType: u32,
    pub pData: *mut u8,
    pub cbData: u32,
}
impl Default for PRINTER_ENUM_VALUESW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_ERROR_INFORMATION: u32 = 2147483648u32;
pub const PRINTER_ERROR_JAM: u32 = 2u32;
pub const PRINTER_ERROR_OUTOFPAPER: u32 = 1u32;
pub const PRINTER_ERROR_OUTOFTONER: u32 = 4u32;
pub const PRINTER_ERROR_SEVERE: u32 = 536870912u32;
pub const PRINTER_ERROR_WARNING: u32 = 1073741824u32;
pub const PRINTER_EVENT_ADD_CONNECTION: u32 = 1u32;
pub const PRINTER_EVENT_ADD_CONNECTION_NO_UI: u32 = 9u32;
pub const PRINTER_EVENT_ATTRIBUTES_CHANGED: u32 = 7u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_EVENT_ATTRIBUTES_INFO {
    pub cbSize: u32,
    pub dwOldAttributes: u32,
    pub dwNewAttributes: u32,
}
pub const PRINTER_EVENT_CACHE_DELETE: u32 = 6u32;
pub const PRINTER_EVENT_CACHE_REFRESH: u32 = 5u32;
pub const PRINTER_EVENT_CONFIGURATION_CHANGE: u32 = 0u32;
pub const PRINTER_EVENT_CONFIGURATION_UPDATE: u32 = 8u32;
pub const PRINTER_EVENT_DELETE: u32 = 4u32;
pub const PRINTER_EVENT_DELETE_CONNECTION: u32 = 2u32;
pub const PRINTER_EVENT_DELETE_CONNECTION_NO_UI: u32 = 10u32;
pub const PRINTER_EVENT_FLAG_NO_UI: u32 = 1u32;
pub const PRINTER_EVENT_INITIALIZE: u32 = 3u32;
pub const PRINTER_EXECUTE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131080u32);
pub const PRINTER_EXTENSION_DETAILEDREASON_PRINTER_STATUS: windows_core::GUID = windows_core::GUID::from_u128(0x5d5a1704_dfd1_4181_8eee_815c86edad31);
pub const PRINTER_EXTENSION_REASON_DRIVER_EVENT: windows_core::GUID = windows_core::GUID::from_u128(0x23bb1328_63de_4293_915b_a6a23d929acb);
pub const PRINTER_EXTENSION_REASON_PRINT_PREFERENCES: windows_core::GUID = windows_core::GUID::from_u128(0xec8f261f_267c_469f_b5d6_3933023c29cc);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_HANDLE {
    pub Value: *mut core::ffi::c_void,
}
impl Default for PRINTER_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_1A {
    pub Flags: u32,
    pub pDescription: windows_core::PSTR,
    pub pName: windows_core::PSTR,
    pub pComment: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_1W {
    pub Flags: u32,
    pub pDescription: windows_core::PWSTR,
    pub pName: windows_core::PWSTR,
    pub pComment: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_2A {
    pub pServerName: windows_core::PSTR,
    pub pPrinterName: windows_core::PSTR,
    pub pShareName: windows_core::PSTR,
    pub pPortName: windows_core::PSTR,
    pub pDriverName: windows_core::PSTR,
    pub pComment: windows_core::PSTR,
    pub pLocation: windows_core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pSepFile: windows_core::PSTR,
    pub pPrintProcessor: windows_core::PSTR,
    pub pDatatype: windows_core::PSTR,
    pub pParameters: windows_core::PSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Attributes: u32,
    pub Priority: u32,
    pub DefaultPriority: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub Status: u32,
    pub cJobs: u32,
    pub AveragePPM: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for PRINTER_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_2W {
    pub pServerName: windows_core::PWSTR,
    pub pPrinterName: windows_core::PWSTR,
    pub pShareName: windows_core::PWSTR,
    pub pPortName: windows_core::PWSTR,
    pub pDriverName: windows_core::PWSTR,
    pub pComment: windows_core::PWSTR,
    pub pLocation: windows_core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pSepFile: windows_core::PWSTR,
    pub pPrintProcessor: windows_core::PWSTR,
    pub pDatatype: windows_core::PWSTR,
    pub pParameters: windows_core::PWSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Attributes: u32,
    pub Priority: u32,
    pub DefaultPriority: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub Status: u32,
    pub cJobs: u32,
    pub AveragePPM: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl Default for PRINTER_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_3 {
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_4A {
    pub pPrinterName: windows_core::PSTR,
    pub pServerName: windows_core::PSTR,
    pub Attributes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_4W {
    pub pPrinterName: windows_core::PWSTR,
    pub pServerName: windows_core::PWSTR,
    pub Attributes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_5A {
    pub pPrinterName: windows_core::PSTR,
    pub pPortName: windows_core::PSTR,
    pub Attributes: u32,
    pub DeviceNotSelectedTimeout: u32,
    pub TransmissionRetryTimeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_5W {
    pub pPrinterName: windows_core::PWSTR,
    pub pPortName: windows_core::PWSTR,
    pub Attributes: u32,
    pub DeviceNotSelectedTimeout: u32,
    pub TransmissionRetryTimeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_6 {
    pub dwStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_7A {
    pub pszObjectGUID: windows_core::PSTR,
    pub dwAction: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_INFO_7W {
    pub pszObjectGUID: windows_core::PWSTR,
    pub dwAction: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_8A {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_INFO_8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_8W {
    pub pDevMode: *mut super::Gdi::DEVMODEW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_INFO_8W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_9A {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_INFO_9A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_INFO_9W {
    pub pDevMode: *mut super::Gdi::DEVMODEW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTER_INFO_9W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_CATEGORY_3D: u32 = 8192u32;
pub const PRINTER_NOTIFY_CATEGORY_ALL: u32 = 4096u32;
pub const PRINTER_NOTIFY_FIELD_ATTRIBUTES: u32 = 13u32;
pub const PRINTER_NOTIFY_FIELD_AVERAGE_PPM: u32 = 21u32;
pub const PRINTER_NOTIFY_FIELD_BRANCH_OFFICE_PRINTING: u32 = 28u32;
pub const PRINTER_NOTIFY_FIELD_BYTES_PRINTED: u32 = 25u32;
pub const PRINTER_NOTIFY_FIELD_CJOBS: u32 = 20u32;
pub const PRINTER_NOTIFY_FIELD_COMMENT: u32 = 5u32;
pub const PRINTER_NOTIFY_FIELD_DATATYPE: u32 = 11u32;
pub const PRINTER_NOTIFY_FIELD_DEFAULT_PRIORITY: u32 = 15u32;
pub const PRINTER_NOTIFY_FIELD_DEVMODE: u32 = 7u32;
pub const PRINTER_NOTIFY_FIELD_DRIVER_NAME: u32 = 4u32;
pub const PRINTER_NOTIFY_FIELD_FRIENDLY_NAME: u32 = 27u32;
pub const PRINTER_NOTIFY_FIELD_LOCATION: u32 = 6u32;
pub const PRINTER_NOTIFY_FIELD_OBJECT_GUID: u32 = 26u32;
pub const PRINTER_NOTIFY_FIELD_PAGES_PRINTED: u32 = 23u32;
pub const PRINTER_NOTIFY_FIELD_PARAMETERS: u32 = 10u32;
pub const PRINTER_NOTIFY_FIELD_PORT_NAME: u32 = 3u32;
pub const PRINTER_NOTIFY_FIELD_PRINTER_NAME: u32 = 1u32;
pub const PRINTER_NOTIFY_FIELD_PRINT_PROCESSOR: u32 = 9u32;
pub const PRINTER_NOTIFY_FIELD_PRIORITY: u32 = 14u32;
pub const PRINTER_NOTIFY_FIELD_SECURITY_DESCRIPTOR: u32 = 12u32;
pub const PRINTER_NOTIFY_FIELD_SEPFILE: u32 = 8u32;
pub const PRINTER_NOTIFY_FIELD_SERVER_NAME: u32 = 0u32;
pub const PRINTER_NOTIFY_FIELD_SHARE_NAME: u32 = 2u32;
pub const PRINTER_NOTIFY_FIELD_START_TIME: u32 = 16u32;
pub const PRINTER_NOTIFY_FIELD_STATUS: u32 = 18u32;
pub const PRINTER_NOTIFY_FIELD_STATUS_STRING: u32 = 19u32;
pub const PRINTER_NOTIFY_FIELD_TOTAL_BYTES: u32 = 24u32;
pub const PRINTER_NOTIFY_FIELD_TOTAL_PAGES: u32 = 22u32;
pub const PRINTER_NOTIFY_FIELD_UNTIL_TIME: u32 = 17u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_NOTIFY_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [PRINTER_NOTIFY_INFO_DATA; 1],
}
impl Default for PRINTER_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_NOTIFY_INFO_DATA {
    pub Type: u16,
    pub Field: u16,
    pub Reserved: u32,
    pub Id: u32,
    pub NotifyData: PRINTER_NOTIFY_INFO_DATA_0,
}
impl Default for PRINTER_NOTIFY_INFO_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PRINTER_NOTIFY_INFO_DATA_0 {
    pub adwData: [u32; 2],
    pub Data: PRINTER_NOTIFY_INFO_DATA_0_0,
}
impl Default for PRINTER_NOTIFY_INFO_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_NOTIFY_INFO_DATA_0_0 {
    pub cbBuf: u32,
    pub pBuf: *mut core::ffi::c_void,
}
impl Default for PRINTER_NOTIFY_INFO_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_INFO_DATA_COMPACT: u32 = 1u32;
pub const PRINTER_NOTIFY_INFO_DISCARDED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_NOTIFY_INIT {
    pub Size: u32,
    pub Reserved: u32,
    pub PollTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_NOTIFY_OPTIONS {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub pTypes: *mut PRINTER_NOTIFY_OPTIONS_TYPE,
}
impl Default for PRINTER_NOTIFY_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_OPTIONS_REFRESH: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTER_NOTIFY_OPTIONS_TYPE {
    pub Type: u16,
    pub Reserved0: u16,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Count: u32,
    pub pFields: *mut u16,
}
impl Default for PRINTER_NOTIFY_OPTIONS_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_NOTIFY_STATUS_ENDPOINT: u32 = 1u32;
pub const PRINTER_NOTIFY_STATUS_INFO: u32 = 4u32;
pub const PRINTER_NOTIFY_STATUS_POLL: u32 = 2u32;
pub const PRINTER_NOTIFY_TYPE: u32 = 0u32;
pub const PRINTER_OEMINTF_VERSION: u32 = 65536u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_OPTIONSA {
    pub cbSize: u32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTER_OPTIONSW {
    pub cbSize: u32,
    pub dwFlags: u32,
}
pub const PRINTER_OPTION_CACHE: PRINTER_OPTION_FLAGS = PRINTER_OPTION_FLAGS(2i32);
pub const PRINTER_OPTION_CLIENT_CHANGE: PRINTER_OPTION_FLAGS = PRINTER_OPTION_FLAGS(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PRINTER_OPTION_FLAGS(pub i32);
pub const PRINTER_OPTION_NO_CACHE: PRINTER_OPTION_FLAGS = PRINTER_OPTION_FLAGS(1i32);
pub const PRINTER_OPTION_NO_CLIENT_DATA: PRINTER_OPTION_FLAGS = PRINTER_OPTION_FLAGS(8i32);
pub const PRINTER_READ: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131080u32);
pub const PRINTER_READ_CONTROL: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131072u32);
pub const PRINTER_STANDARD_RIGHTS_EXECUTE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131072u32);
pub const PRINTER_STANDARD_RIGHTS_READ: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131072u32);
pub const PRINTER_STANDARD_RIGHTS_REQUIRED: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(983040u32);
pub const PRINTER_STANDARD_RIGHTS_WRITE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131072u32);
pub const PRINTER_STATUS_BUSY: u32 = 512u32;
pub const PRINTER_STATUS_DOOR_OPEN: u32 = 4194304u32;
pub const PRINTER_STATUS_DRIVER_UPDATE_NEEDED: u32 = 67108864u32;
pub const PRINTER_STATUS_ERROR: u32 = 2u32;
pub const PRINTER_STATUS_INITIALIZING: u32 = 32768u32;
pub const PRINTER_STATUS_IO_ACTIVE: u32 = 256u32;
pub const PRINTER_STATUS_MANUAL_FEED: u32 = 32u32;
pub const PRINTER_STATUS_NOT_AVAILABLE: u32 = 4096u32;
pub const PRINTER_STATUS_NO_TONER: u32 = 262144u32;
pub const PRINTER_STATUS_OFFLINE: u32 = 128u32;
pub const PRINTER_STATUS_OUTPUT_BIN_FULL: u32 = 2048u32;
pub const PRINTER_STATUS_OUT_OF_MEMORY: u32 = 2097152u32;
pub const PRINTER_STATUS_PAGE_PUNT: u32 = 524288u32;
pub const PRINTER_STATUS_PAPER_JAM: u32 = 8u32;
pub const PRINTER_STATUS_PAPER_OUT: u32 = 16u32;
pub const PRINTER_STATUS_PAPER_PROBLEM: u32 = 64u32;
pub const PRINTER_STATUS_PAUSED: u32 = 1u32;
pub const PRINTER_STATUS_PENDING_DELETION: u32 = 4u32;
pub const PRINTER_STATUS_POWER_SAVE: u32 = 16777216u32;
pub const PRINTER_STATUS_PRINTING: u32 = 1024u32;
pub const PRINTER_STATUS_PROCESSING: u32 = 16384u32;
pub const PRINTER_STATUS_SERVER_OFFLINE: u32 = 33554432u32;
pub const PRINTER_STATUS_SERVER_UNKNOWN: u32 = 8388608u32;
pub const PRINTER_STATUS_TONER_LOW: u32 = 131072u32;
pub const PRINTER_STATUS_USER_INTERVENTION: u32 = 1048576u32;
pub const PRINTER_STATUS_WAITING: u32 = 8192u32;
pub const PRINTER_STATUS_WARMING_UP: u32 = 65536u32;
pub const PRINTER_SYNCHRONIZE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(1048576u32);
pub const PRINTER_WRITE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131080u32);
pub const PRINTER_WRITE_DAC: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(262144u32);
pub const PRINTER_WRITE_OWNER: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(524288u32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTIFI32 {
    pub cjThis: u32,
    pub cjIfiExtra: u32,
    pub dpwszFamilyName: i32,
    pub dpwszStyleName: i32,
    pub dpwszFaceName: i32,
    pub dpwszUniqueName: i32,
    pub dpFontSim: i32,
    pub lEmbedId: i32,
    pub lItalicAngle: i32,
    pub lCharBias: i32,
    pub dpCharSets: i32,
    pub jWinCharSet: u8,
    pub jWinPitchAndFamily: u8,
    pub usWinWeight: u16,
    pub flInfo: u32,
    pub fsSelection: u16,
    pub fsType: u16,
    pub fwdUnitsPerEm: i16,
    pub fwdLowestPPEm: i16,
    pub fwdWinAscender: i16,
    pub fwdWinDescender: i16,
    pub fwdMacAscender: i16,
    pub fwdMacDescender: i16,
    pub fwdMacLineGap: i16,
    pub fwdTypoAscender: i16,
    pub fwdTypoDescender: i16,
    pub fwdTypoLineGap: i16,
    pub fwdAveCharWidth: i16,
    pub fwdMaxCharInc: i16,
    pub fwdCapHeight: i16,
    pub fwdXHeight: i16,
    pub fwdSubscriptXSize: i16,
    pub fwdSubscriptYSize: i16,
    pub fwdSubscriptXOffset: i16,
    pub fwdSubscriptYOffset: i16,
    pub fwdSuperscriptXSize: i16,
    pub fwdSuperscriptYSize: i16,
    pub fwdSuperscriptXOffset: i16,
    pub fwdSuperscriptYOffset: i16,
    pub fwdUnderscoreSize: i16,
    pub fwdUnderscorePosition: i16,
    pub fwdStrikeoutSize: i16,
    pub fwdStrikeoutPosition: i16,
    pub chFirstChar: u8,
    pub chLastChar: u8,
    pub chDefaultChar: u8,
    pub chBreakChar: u8,
    pub wcFirstChar: u16,
    pub wcLastChar: u16,
    pub wcDefaultChar: u16,
    pub wcBreakChar: u16,
    pub ptlBaseline: super::super::Foundation::POINTL,
    pub ptlAspect: super::super::Foundation::POINTL,
    pub ptlCaret: super::super::Foundation::POINTL,
    pub rclFontBox: super::super::Foundation::RECTL,
    pub achVendId: [u8; 4],
    pub cKerningPairs: u32,
    pub ulPanoseCulture: u32,
    pub panose: super::Gdi::PANOSE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTIFI32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRINTPROCESSOROPENDATA {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pDatatype: windows_core::PWSTR,
    pub pParameters: windows_core::PWSTR,
    pub pDocumentName: windows_core::PWSTR,
    pub JobId: u32,
    pub pOutputFile: windows_core::PWSTR,
    pub pPrinterName: windows_core::PWSTR,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTPROCESSOROPENDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTPROCESSOR_CAPS_1 {
    pub dwLevel: u32,
    pub dwNupOptions: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTPROCESSOR_CAPS_2 {
    pub dwLevel: u32,
    pub dwNupOptions: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
    pub dwDuplexHandlingCaps: u32,
    pub dwNupDirectionCaps: u32,
    pub dwNupBorderCaps: u32,
    pub dwBookletHandlingCaps: u32,
    pub dwScalingCaps: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTPROCESSOR_INFO_1A {
    pub pName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTPROCESSOR_INFO_1W {
    pub pName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINTPROVIDOR {
    pub fpOpenPrinter: isize,
    pub fpSetJob: isize,
    pub fpGetJob: isize,
    pub fpEnumJobs: isize,
    pub fpAddPrinter: isize,
    pub fpDeletePrinter: isize,
    pub fpSetPrinter: isize,
    pub fpGetPrinter: isize,
    pub fpEnumPrinters: isize,
    pub fpAddPrinterDriver: isize,
    pub fpEnumPrinterDrivers: isize,
    pub fpGetPrinterDriver: isize,
    pub fpGetPrinterDriverDirectory: isize,
    pub fpDeletePrinterDriver: isize,
    pub fpAddPrintProcessor: isize,
    pub fpEnumPrintProcessors: isize,
    pub fpGetPrintProcessorDirectory: isize,
    pub fpDeletePrintProcessor: isize,
    pub fpEnumPrintProcessorDatatypes: isize,
    pub fpStartDocPrinter: isize,
    pub fpStartPagePrinter: isize,
    pub fpWritePrinter: isize,
    pub fpEndPagePrinter: isize,
    pub fpAbortPrinter: isize,
    pub fpReadPrinter: isize,
    pub fpEndDocPrinter: isize,
    pub fpAddJob: isize,
    pub fpScheduleJob: isize,
    pub fpGetPrinterData: isize,
    pub fpSetPrinterData: isize,
    pub fpWaitForPrinterChange: isize,
    pub fpClosePrinter: isize,
    pub fpAddForm: isize,
    pub fpDeleteForm: isize,
    pub fpGetForm: isize,
    pub fpSetForm: isize,
    pub fpEnumForms: isize,
    pub fpEnumMonitors: isize,
    pub fpEnumPorts: isize,
    pub fpAddPort: isize,
    pub fpConfigurePort: isize,
    pub fpDeletePort: isize,
    pub fpCreatePrinterIC: isize,
    pub fpPlayGdiScriptOnPrinterIC: isize,
    pub fpDeletePrinterIC: isize,
    pub fpAddPrinterConnection: isize,
    pub fpDeletePrinterConnection: isize,
    pub fpPrinterMessageBox: isize,
    pub fpAddMonitor: isize,
    pub fpDeleteMonitor: isize,
    pub fpResetPrinter: isize,
    pub fpGetPrinterDriverEx: isize,
    pub fpFindFirstPrinterChangeNotification: isize,
    pub fpFindClosePrinterChangeNotification: isize,
    pub fpAddPortEx: isize,
    pub fpShutDown: isize,
    pub fpRefreshPrinterChangeNotification: isize,
    pub fpOpenPrinterEx: isize,
    pub fpAddPrinterEx: isize,
    pub fpSetPort: isize,
    pub fpEnumPrinterData: isize,
    pub fpDeletePrinterData: isize,
    pub fpClusterSplOpen: isize,
    pub fpClusterSplClose: isize,
    pub fpClusterSplIsAlive: isize,
    pub fpSetPrinterDataEx: isize,
    pub fpGetPrinterDataEx: isize,
    pub fpEnumPrinterDataEx: isize,
    pub fpEnumPrinterKey: isize,
    pub fpDeletePrinterDataEx: isize,
    pub fpDeletePrinterKey: isize,
    pub fpSeekPrinter: isize,
    pub fpDeletePrinterDriverEx: isize,
    pub fpAddPerMachineConnection: isize,
    pub fpDeletePerMachineConnection: isize,
    pub fpEnumPerMachineConnections: isize,
    pub fpXcvData: isize,
    pub fpAddPrinterDriverEx: isize,
    pub fpSplReadPrinter: isize,
    pub fpDriverUnloadComplete: isize,
    pub fpGetSpoolFileInfo: isize,
    pub fpCommitSpoolData: isize,
    pub fpCloseSpoolFileHandle: isize,
    pub fpFlushPrinter: isize,
    pub fpSendRecvBidiData: isize,
    pub fpAddPrinterConnection2: isize,
    pub fpGetPrintClassObject: isize,
    pub fpReportJobProcessingProgress: isize,
    pub fpEnumAndLogProvidorObjects: isize,
    pub fpInternalGetPrinterDriver: isize,
    pub fpFindCompatibleDriver: isize,
    pub fpInstallPrinterDriverPackageFromConnection: isize,
    pub fpGetJobNamedPropertyValue: isize,
    pub fpSetJobNamedProperty: isize,
    pub fpDeleteJobNamedProperty: isize,
    pub fpEnumJobNamedProperties: isize,
    pub fpPowerEvent: isize,
    pub fpGetUserPropertyBag: isize,
    pub fpCanShutdown: isize,
    pub fpLogJobInfoForBranchOffice: isize,
    pub fpRegeneratePrintDeviceCapabilities: isize,
    pub fpPrintSupportOperation: isize,
    pub fpIppCreateJobOnPrinter: isize,
    pub fpIppGetJobAttributes: isize,
    pub fpIppSetJobAttributes: isize,
    pub fpIppGetPrinterAttributes: isize,
    pub fpIppSetPrinterAttributes: isize,
    pub fpIppCreateJobOnPrinterWithAttributes: isize,
}
pub const PRINT_APP_BIDI_NOTIFY_CHANNEL: windows_core::GUID = windows_core::GUID::from_u128(0x2abad223_b994_4aca_82fc_4571b1b585ac);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PRINT_EXECUTION_CONTEXT(pub i32);
pub const PRINT_EXECUTION_CONTEXT_APPLICATION: PRINT_EXECUTION_CONTEXT = PRINT_EXECUTION_CONTEXT(0i32);
pub const PRINT_EXECUTION_CONTEXT_FILTER_PIPELINE: PRINT_EXECUTION_CONTEXT = PRINT_EXECUTION_CONTEXT(3i32);
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_ISOLATION_HOST: PRINT_EXECUTION_CONTEXT = PRINT_EXECUTION_CONTEXT(2i32);
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_SERVICE: PRINT_EXECUTION_CONTEXT = PRINT_EXECUTION_CONTEXT(1i32);
pub const PRINT_EXECUTION_CONTEXT_WOW64: PRINT_EXECUTION_CONTEXT = PRINT_EXECUTION_CONTEXT(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINT_EXECUTION_DATA {
    pub context: PRINT_EXECUTION_CONTEXT,
    pub clientAppPID: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRINT_FEATURE_OPTION {
    pub pszFeature: windows_core::PCSTR,
    pub pszOption: windows_core::PCSTR,
}
pub const PRINT_PORT_MONITOR_NOTIFY_CHANNEL: windows_core::GUID = windows_core::GUID::from_u128(0x25df3b0e_74a9_47f5_80ce_79b4b1eb5c58);
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROPSHEETUI_GETICON_INFO {
    pub cbSize: u16,
    pub Flags: u16,
    pub cxIcon: u16,
    pub cyIcon: u16,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PROPSHEETUI_INFO {
    pub cbSize: u16,
    pub Version: u16,
    pub Flags: u16,
    pub Reason: u16,
    pub hComPropSheet: super::super::Foundation::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
    pub lParamInit: super::super::Foundation::LPARAM,
    pub UserData: usize,
    pub Result: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct PROPSHEETUI_INFO_HEADER {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub hWndParent: super::super::Foundation::HWND,
    pub hInst: super::super::Foundation::HINSTANCE,
    pub Anonymous: PROPSHEETUI_INFO_HEADER_0,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for PROPSHEETUI_INFO_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union PROPSHEETUI_INFO_HEADER_0 {
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub IconID: usize,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for PROPSHEETUI_INFO_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROPSHEETUI_INFO_VERSION: u32 = 256u32;
pub const PROPSHEETUI_REASON_BEFORE_INIT: u32 = 5u32;
pub const PROPSHEETUI_REASON_DESTROY: u32 = 2u32;
pub const PROPSHEETUI_REASON_GET_ICON: u32 = 4u32;
pub const PROPSHEETUI_REASON_GET_INFO_HEADER: u32 = 1u32;
pub const PROPSHEETUI_REASON_INIT: u32 = 0u32;
pub const PROPSHEETUI_REASON_SET_RESULT: u32 = 3u32;
pub const PROTOCOL_LPR_TYPE: u32 = 2u32;
pub const PROTOCOL_RAWTCP_TYPE: u32 = 1u32;
pub const PROTOCOL_UNKNOWN_TYPE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROVIDOR_INFO_1A {
    pub pName: windows_core::PSTR,
    pub pEnvironment: windows_core::PSTR,
    pub pDLLName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROVIDOR_INFO_1W {
    pub pName: windows_core::PWSTR,
    pub pEnvironment: windows_core::PWSTR,
    pub pDLLName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROVIDOR_INFO_2A {
    pub pOrder: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROVIDOR_INFO_2W {
    pub pOrder: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSCRIPT5_PRIVATE_DEVMODE {
    pub wReserved: [u16; 57],
    pub wSize: u16,
}
impl Default for PSCRIPT5_PRIVATE_DEVMODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PSPINFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hComPropSheet: super::super::Foundation::HANDLE,
    pub hCPSUIPage: super::super::Foundation::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
}
pub const PSUIHDRF_DEFTITLE: u32 = 16u32;
pub const PSUIHDRF_EXACT_PTITLE: u32 = 32u32;
pub const PSUIHDRF_NOAPPLYNOW: u32 = 2u32;
pub const PSUIHDRF_OBSOLETE: u32 = 1u32;
pub const PSUIHDRF_PROPTITLE: u32 = 4u32;
pub const PSUIHDRF_USEHICON: u32 = 8u32;
pub const PSUIINFO_UNICODE: u32 = 1u32;
pub const PSUIPAGEINSERT_DLL: u32 = 5u32;
pub const PSUIPAGEINSERT_GROUP_PARENT: u32 = 0u32;
pub const PSUIPAGEINSERT_HPROPSHEETPAGE: u32 = 4u32;
pub const PSUIPAGEINSERT_PCOMPROPSHEETUI: u32 = 1u32;
pub const PSUIPAGEINSERT_PFNPROPSHEETUI: u32 = 2u32;
pub const PSUIPAGEINSERT_PROPSHEETPAGE: u32 = 3u32;
pub const PTSHIM_DEFAULT: SHIMOPTS = SHIMOPTS(0i32);
pub const PTSHIM_NOSNAPSHOT: SHIMOPTS = SHIMOPTS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PUBLISHERINFO {
    pub dwMode: u32,
    pub wMinoutlinePPEM: u16,
    pub wMaxbitmapPPEM: u16,
}
pub const PUSHBUTTON_TYPE_CALLBACK: u32 = 1u32;
pub const PUSHBUTTON_TYPE_DLGPROC: u32 = 0u32;
pub const PUSHBUTTON_TYPE_HTCLRADJ: u32 = 2u32;
pub const PUSHBUTTON_TYPE_HTSETUP: u32 = 3u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PageCountType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrintAsyncNotifyConversationStyle(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrintAsyncNotifyError(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrintAsyncNotifyUserFilter(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrintJobStatus(pub i32);
pub const PrintJobStatus_BlockedDeviceQueue: PrintJobStatus = PrintJobStatus(512i32);
pub const PrintJobStatus_Complete: PrintJobStatus = PrintJobStatus(4096i32);
pub const PrintJobStatus_Deleted: PrintJobStatus = PrintJobStatus(256i32);
pub const PrintJobStatus_Deleting: PrintJobStatus = PrintJobStatus(4i32);
pub const PrintJobStatus_Error: PrintJobStatus = PrintJobStatus(2i32);
pub const PrintJobStatus_Offline: PrintJobStatus = PrintJobStatus(32i32);
pub const PrintJobStatus_PaperOut: PrintJobStatus = PrintJobStatus(64i32);
pub const PrintJobStatus_Paused: PrintJobStatus = PrintJobStatus(1i32);
pub const PrintJobStatus_Printed: PrintJobStatus = PrintJobStatus(128i32);
pub const PrintJobStatus_Printing: PrintJobStatus = PrintJobStatus(16i32);
pub const PrintJobStatus_Restarted: PrintJobStatus = PrintJobStatus(2048i32);
pub const PrintJobStatus_Retained: PrintJobStatus = PrintJobStatus(8192i32);
pub const PrintJobStatus_Spooling: PrintJobStatus = PrintJobStatus(8i32);
pub const PrintJobStatus_UserIntervention: PrintJobStatus = PrintJobStatus(1024i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PrintNamedProperty {
    pub propertyName: windows_core::PWSTR,
    pub propertyValue: PrintPropertyValue,
}
impl Default for PrintNamedProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrintPropertiesCollection {
    pub numberOfProperties: u32,
    pub propertiesCollection: *mut PrintNamedProperty,
}
impl Default for PrintPropertiesCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PrintPropertyValue {
    pub ePropertyType: EPrintPropertyType,
    pub value: PrintPropertyValue_0,
}
impl Default for PrintPropertyValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PrintPropertyValue_0 {
    pub propertyByte: u8,
    pub propertyString: windows_core::PWSTR,
    pub propertyInt32: i32,
    pub propertyInt64: i64,
    pub propertyBlob: PrintPropertyValue_0_0,
}
impl Default for PrintPropertyValue_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrintPropertyValue_0_0 {
    pub cbBuf: u32,
    pub pBuf: *mut core::ffi::c_void,
}
impl Default for PrintPropertyValue_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PrintSchemaAsyncOperation: windows_core::GUID = windows_core::GUID::from_u128(0x43b2f83d_10f2_48ab_831b_55fdbdbd34a4);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrintSchemaConstrainedSetting(pub i32);
pub const PrintSchemaConstrainedSetting_Admin: PrintSchemaConstrainedSetting = PrintSchemaConstrainedSetting(2i32);
pub const PrintSchemaConstrainedSetting_Device: PrintSchemaConstrainedSetting = PrintSchemaConstrainedSetting(3i32);
pub const PrintSchemaConstrainedSetting_None: PrintSchemaConstrainedSetting = PrintSchemaConstrainedSetting(0i32);
pub const PrintSchemaConstrainedSetting_PrintTicket: PrintSchemaConstrainedSetting = PrintSchemaConstrainedSetting(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrintSchemaParameterDataType(pub i32);
pub const PrintSchemaParameterDataType_Integer: PrintSchemaParameterDataType = PrintSchemaParameterDataType(0i32);
pub const PrintSchemaParameterDataType_NumericString: PrintSchemaParameterDataType = PrintSchemaParameterDataType(1i32);
pub const PrintSchemaParameterDataType_String: PrintSchemaParameterDataType = PrintSchemaParameterDataType(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrintSchemaSelectionType(pub i32);
pub const PrintSchemaSelectionType_PickMany: PrintSchemaSelectionType = PrintSchemaSelectionType(1i32);
pub const PrintSchemaSelectionType_PickOne: PrintSchemaSelectionType = PrintSchemaSelectionType(0i32);
pub const PrinterExtensionManager: windows_core::GUID = windows_core::GUID::from_u128(0x331b60da_9e90_4dd0_9c84_eac4e659b61f);
pub const PrinterQueue: windows_core::GUID = windows_core::GUID::from_u128(0xeb54c230_798c_4c9e_b461_29fad04039b1);
pub const PrinterQueueView: windows_core::GUID = windows_core::GUID::from_u128(0xeb54c231_798c_4c9e_b461_29fad04039b1);
pub const QCP_DEVICEPROFILE: u32 = 0u32;
pub const QCP_PROFILEDISK: u32 = 2u32;
pub const QCP_PROFILEMEMORY: u32 = 1u32;
pub const QCP_SOURCEPROFILE: u32 = 1u32;
pub const RAWTCP: u32 = 1u32;
pub const REMOTE_ONLY_REGISTRATION: PrintAsyncNotifyError = PrintAsyncNotifyError(24i32);
pub const REVERSE_PAGES_FOR_REVERSE_DUPLEX: u32 = 1u32;
pub const REVERSE_PRINT: u32 = 1u32;
pub const RIGHT_THEN_DOWN: u32 = 1u32;
pub type ROUTER_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(dwcommand: u32, pcontext: *const core::ffi::c_void, dwcolor: u32, pnofityinfo: *const PRINTER_NOTIFY_INFO, fdwflags: u32, pdwresult: *mut u32) -> windows_core::BOOL>;
pub const ROUTER_STOP_ROUTING: u32 = 2u32;
pub const ROUTER_SUCCESS: u32 = 1u32;
pub const ROUTER_UNKNOWN: u32 = 0u32;
pub const SERVER_ACCESS_ADMINISTER: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(1u32);
pub const SERVER_ACCESS_ENUMERATE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(2u32);
pub const SERVER_ALL_ACCESS: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(983043u32);
pub const SERVER_EXECUTE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131074u32);
pub const SERVER_NOTIFY_FIELD_PRINT_DRIVER_ISOLATION_GROUP: u32 = 0u32;
pub const SERVER_NOTIFY_TYPE: u32 = 2u32;
pub const SERVER_READ: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131074u32);
pub const SERVER_WRITE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131075u32);
pub const SETOPTIONS_FLAG_KEEP_CONFLICT: u32 = 2u32;
pub const SETOPTIONS_FLAG_RESOLVE_CONFLICT: u32 = 1u32;
pub const SETOPTIONS_RESULT_CONFLICT_REMAINED: u32 = 2u32;
pub const SETOPTIONS_RESULT_CONFLICT_RESOLVED: u32 = 1u32;
pub const SETOPTIONS_RESULT_NO_CONFLICT: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SETRESULT_INFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hSetResult: super::super::Foundation::HANDLE,
    pub Result: super::super::Foundation::LRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHIMOPTS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SHOWUIPARAMS {
    pub UIType: UI_TYPE,
    pub MessageBoxParams: MESSAGEBOX_PARAMS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SIMULATE_CAPS_1 {
    pub dwLevel: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
    pub dwCollate: u32,
    pub dwNupOptions: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPLCLIENT_INFO_1 {
    pub dwSize: u32,
    pub pMachineName: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub dwBuildNum: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub wProcessorArchitecture: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPLCLIENT_INFO_2_W2K {
    pub hSplPrinter: usize,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPLCLIENT_INFO_2_WINXP {
    pub hSplPrinter: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPLCLIENT_INFO_2_WINXP {
    pub hSplPrinter: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPLCLIENT_INFO_3_VISTA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwSize: u32,
    pub pMachineName: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub dwBuildNum: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub wProcessorArchitecture: u16,
    pub hSplPrinter: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SPLCLIENT_INFO_INTERNAL {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwSize: u32,
    pub pMachineName: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub dwBuildNum: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub wProcessorArchitecture: u16,
    pub hSplPrinter: u64,
    pub dwProcessId: u32,
    pub dwSessionId: u32,
}
pub const SPLCLIENT_INFO_INTERNAL_LEVEL: u32 = 100u32;
pub const SPLDS_ASSET_NUMBER: windows_core::PCWSTR = windows_core::w!("assetNumber");
pub const SPLDS_BYTES_PER_MINUTE: windows_core::PCWSTR = windows_core::w!("bytesPerMinute");
pub const SPLDS_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("description");
pub const SPLDS_DRIVER_KEY: windows_core::PCWSTR = windows_core::w!("DsDriver");
pub const SPLDS_DRIVER_NAME: windows_core::PCWSTR = windows_core::w!("driverName");
pub const SPLDS_DRIVER_VERSION: windows_core::PCWSTR = windows_core::w!("driverVersion");
pub const SPLDS_FLAGS: windows_core::PCWSTR = windows_core::w!("flags");
pub const SPLDS_LOCATION: windows_core::PCWSTR = windows_core::w!("location");
pub const SPLDS_PORT_NAME: windows_core::PCWSTR = windows_core::w!("portName");
pub const SPLDS_PRINTER_CLASS: windows_core::PCWSTR = windows_core::w!("printQueue");
pub const SPLDS_PRINTER_LOCATIONS: windows_core::PCWSTR = windows_core::w!("printerLocations");
pub const SPLDS_PRINTER_MODEL: windows_core::PCWSTR = windows_core::w!("printerModel");
pub const SPLDS_PRINTER_NAME: windows_core::PCWSTR = windows_core::w!("printerName");
pub const SPLDS_PRINTER_NAME_ALIASES: windows_core::PCWSTR = windows_core::w!("printerNameAliases");
pub const SPLDS_PRINT_ATTRIBUTES: windows_core::PCWSTR = windows_core::w!("printAttributes");
pub const SPLDS_PRINT_BIN_NAMES: windows_core::PCWSTR = windows_core::w!("printBinNames");
pub const SPLDS_PRINT_COLLATE: windows_core::PCWSTR = windows_core::w!("printCollate");
pub const SPLDS_PRINT_COLOR: windows_core::PCWSTR = windows_core::w!("printColor");
pub const SPLDS_PRINT_DUPLEX_SUPPORTED: windows_core::PCWSTR = windows_core::w!("printDuplexSupported");
pub const SPLDS_PRINT_END_TIME: windows_core::PCWSTR = windows_core::w!("printEndTime");
pub const SPLDS_PRINT_KEEP_PRINTED_JOBS: windows_core::PCWSTR = windows_core::w!("printKeepPrintedJobs");
pub const SPLDS_PRINT_LANGUAGE: windows_core::PCWSTR = windows_core::w!("printLanguage");
pub const SPLDS_PRINT_MAC_ADDRESS: windows_core::PCWSTR = windows_core::w!("printMACAddress");
pub const SPLDS_PRINT_MAX_RESOLUTION_SUPPORTED: windows_core::PCWSTR = windows_core::w!("printMaxResolutionSupported");
pub const SPLDS_PRINT_MAX_X_EXTENT: windows_core::PCWSTR = windows_core::w!("printMaxXExtent");
pub const SPLDS_PRINT_MAX_Y_EXTENT: windows_core::PCWSTR = windows_core::w!("printMaxYExtent");
pub const SPLDS_PRINT_MEDIA_READY: windows_core::PCWSTR = windows_core::w!("printMediaReady");
pub const SPLDS_PRINT_MEDIA_SUPPORTED: windows_core::PCWSTR = windows_core::w!("printMediaSupported");
pub const SPLDS_PRINT_MEMORY: windows_core::PCWSTR = windows_core::w!("printMemory");
pub const SPLDS_PRINT_MIN_X_EXTENT: windows_core::PCWSTR = windows_core::w!("printMinXExtent");
pub const SPLDS_PRINT_MIN_Y_EXTENT: windows_core::PCWSTR = windows_core::w!("printMinYExtent");
pub const SPLDS_PRINT_NETWORK_ADDRESS: windows_core::PCWSTR = windows_core::w!("printNetworkAddress");
pub const SPLDS_PRINT_NOTIFY: windows_core::PCWSTR = windows_core::w!("printNotify");
pub const SPLDS_PRINT_NUMBER_UP: windows_core::PCWSTR = windows_core::w!("printNumberUp");
pub const SPLDS_PRINT_ORIENTATIONS_SUPPORTED: windows_core::PCWSTR = windows_core::w!("printOrientationsSupported");
pub const SPLDS_PRINT_OWNER: windows_core::PCWSTR = windows_core::w!("printOwner");
pub const SPLDS_PRINT_PAGES_PER_MINUTE: windows_core::PCWSTR = windows_core::w!("printPagesPerMinute");
pub const SPLDS_PRINT_RATE: windows_core::PCWSTR = windows_core::w!("printRate");
pub const SPLDS_PRINT_RATE_UNIT: windows_core::PCWSTR = windows_core::w!("printRateUnit");
pub const SPLDS_PRINT_SEPARATOR_FILE: windows_core::PCWSTR = windows_core::w!("printSeparatorFile");
pub const SPLDS_PRINT_SHARE_NAME: windows_core::PCWSTR = windows_core::w!("printShareName");
pub const SPLDS_PRINT_SPOOLING: windows_core::PCWSTR = windows_core::w!("printSpooling");
pub const SPLDS_PRINT_STAPLING_SUPPORTED: windows_core::PCWSTR = windows_core::w!("printStaplingSupported");
pub const SPLDS_PRINT_START_TIME: windows_core::PCWSTR = windows_core::w!("printStartTime");
pub const SPLDS_PRINT_STATUS: windows_core::PCWSTR = windows_core::w!("printStatus");
pub const SPLDS_PRIORITY: windows_core::PCWSTR = windows_core::w!("priority");
pub const SPLDS_SERVER_NAME: windows_core::PCWSTR = windows_core::w!("serverName");
pub const SPLDS_SHORT_SERVER_NAME: windows_core::PCWSTR = windows_core::w!("shortServerName");
pub const SPLDS_SPOOLER_KEY: windows_core::PCWSTR = windows_core::w!("DsSpooler");
pub const SPLDS_UNC_NAME: windows_core::PCWSTR = windows_core::w!("uNCName");
pub const SPLDS_URL: windows_core::PCWSTR = windows_core::w!("url");
pub const SPLDS_USER_KEY: windows_core::PCWSTR = windows_core::w!("DsUser");
pub const SPLDS_VERSION_NUMBER: windows_core::PCWSTR = windows_core::w!("versionNumber");
pub const SPLPRINTER_USER_MODE_PRINTER_DRIVER: windows_core::PCWSTR = windows_core::w!("SPLUserModePrinterDriver");
pub const SPLREG_ALLOW_USER_MANAGEFORMS: windows_core::PCWSTR = windows_core::w!("AllowUserManageForms");
pub const SPLREG_ARCHITECTURE: windows_core::PCWSTR = windows_core::w!("Architecture");
pub const SPLREG_BEEP_ENABLED: windows_core::PCWSTR = windows_core::w!("BeepEnabled");
pub const SPLREG_DEFAULT_SPOOL_DIRECTORY: windows_core::PCWSTR = windows_core::w!("DefaultSpoolDirectory");
pub const SPLREG_DNS_MACHINE_NAME: windows_core::PCWSTR = windows_core::w!("DNSMachineName");
pub const SPLREG_DS_PRESENT: windows_core::PCWSTR = windows_core::w!("DsPresent");
pub const SPLREG_DS_PRESENT_FOR_USER: windows_core::PCWSTR = windows_core::w!("DsPresentForUser");
pub const SPLREG_EVENT_LOG: windows_core::PCWSTR = windows_core::w!("EventLog");
pub const SPLREG_MAJOR_VERSION: windows_core::PCWSTR = windows_core::w!("MajorVersion");
pub const SPLREG_MINOR_VERSION: windows_core::PCWSTR = windows_core::w!("MinorVersion");
pub const SPLREG_NET_POPUP: windows_core::PCWSTR = windows_core::w!("NetPopup");
pub const SPLREG_NET_POPUP_TO_COMPUTER: windows_core::PCWSTR = windows_core::w!("NetPopupToComputer");
pub const SPLREG_OS_VERSION: windows_core::PCWSTR = windows_core::w!("OSVersion");
pub const SPLREG_OS_VERSIONEX: windows_core::PCWSTR = windows_core::w!("OSVersionEx");
pub const SPLREG_PORT_THREAD_PRIORITY: windows_core::PCWSTR = windows_core::w!("PortThreadPriority");
pub const SPLREG_PORT_THREAD_PRIORITY_DEFAULT: windows_core::PCWSTR = windows_core::w!("PortThreadPriorityDefault");
pub const SPLREG_PRINT_DRIVER_ISOLATION_EXECUTION_POLICY: windows_core::PCWSTR = windows_core::w!("PrintDriverIsolationExecutionPolicy");
pub const SPLREG_PRINT_DRIVER_ISOLATION_GROUPS: windows_core::PCWSTR = windows_core::w!("PrintDriverIsolationGroups");
pub const SPLREG_PRINT_DRIVER_ISOLATION_IDLE_TIMEOUT: windows_core::PCWSTR = windows_core::w!("PrintDriverIsolationIdleTimeout");
pub const SPLREG_PRINT_DRIVER_ISOLATION_MAX_OBJECTS_BEFORE_RECYCLE: windows_core::PCWSTR = windows_core::w!("PrintDriverIsolationMaxobjsBeforeRecycle");
pub const SPLREG_PRINT_DRIVER_ISOLATION_OVERRIDE_POLICY: windows_core::PCWSTR = windows_core::w!("PrintDriverIsolationOverrideCompat");
pub const SPLREG_PRINT_DRIVER_ISOLATION_TIME_BEFORE_RECYCLE: windows_core::PCWSTR = windows_core::w!("PrintDriverIsolationTimeBeforeRecycle");
pub const SPLREG_PRINT_QUEUE_V4_DRIVER_DIRECTORY: windows_core::PCWSTR = windows_core::w!("PrintQueueV4DriverDirectory");
pub const SPLREG_REMOTE_FAX: windows_core::PCWSTR = windows_core::w!("RemoteFax");
pub const SPLREG_RESTART_JOB_ON_POOL_ENABLED: windows_core::PCWSTR = windows_core::w!("RestartJobOnPoolEnabled");
pub const SPLREG_RESTART_JOB_ON_POOL_ERROR: windows_core::PCWSTR = windows_core::w!("RestartJobOnPoolError");
pub const SPLREG_RETRY_POPUP: windows_core::PCWSTR = windows_core::w!("RetryPopup");
pub const SPLREG_SCHEDULER_THREAD_PRIORITY: windows_core::PCWSTR = windows_core::w!("SchedulerThreadPriority");
pub const SPLREG_SCHEDULER_THREAD_PRIORITY_DEFAULT: windows_core::PCWSTR = windows_core::w!("SchedulerThreadPriorityDefault");
pub const SPLREG_WEBSHAREMGMT: windows_core::PCWSTR = windows_core::w!("WebShareMgmt");
pub const SPOOL_FILE_PERSISTENT: u32 = 1u32;
pub const SPOOL_FILE_TEMPORARY: u32 = 2u32;
pub const SR_OWNER: u32 = 0u32;
pub const SR_OWNER_PARENT: u32 = 1u32;
pub const SSP_STDPAGE1: u32 = 10001u32;
pub const SSP_STDPAGE2: u32 = 10002u32;
pub const SSP_TVPAGE: u32 = 10000u32;
pub const STRING_LANGPAIR: u32 = 4u32;
pub const STRING_MUIDLL: u32 = 2u32;
pub const STRING_NONE: u32 = 1u32;
pub const S_CONFLICT_RESOLVED: u32 = 262146u32;
pub const S_DEVCAP_OUTPUT_FULL_REPLACEMENT: windows_core::HRESULT = windows_core::HRESULT(0x4DC01_u32 as _);
pub const S_NO_CONFLICT: u32 = 262145u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TRANSDATA {
    pub ubCodePageID: u8,
    pub ubType: u8,
    pub uCode: TRANSDATA_0,
}
impl Default for TRANSDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRANSDATA_0 {
    pub sCode: i16,
    pub ubCode: u8,
    pub ubPairs: [u8; 2],
}
impl Default for TRANSDATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TTDOWNLOAD_BITMAP: u32 = 2u32;
pub const TTDOWNLOAD_DONTCARE: u32 = 0u32;
pub const TTDOWNLOAD_GRAPHICS: u32 = 1u32;
pub const TTDOWNLOAD_TTOUTLINE: u32 = 3u32;
pub const TVOT_2STATES: u32 = 0u32;
pub const TVOT_3STATES: u32 = 1u32;
pub const TVOT_CHKBOX: u32 = 9u32;
pub const TVOT_COMBOBOX: u32 = 6u32;
pub const TVOT_EDITBOX: u32 = 7u32;
pub const TVOT_LISTBOX: u32 = 5u32;
pub const TVOT_NSTATES_EX: u32 = 10u32;
pub const TVOT_PUSHBUTTON: u32 = 8u32;
pub const TVOT_SCROLLBAR: u32 = 4u32;
pub const TVOT_TRACKBAR: u32 = 3u32;
pub const TVOT_UDARROW: u32 = 2u32;
pub const TYPE_GLYPHHANDLE: u32 = 3u32;
pub const TYPE_GLYPHID: u32 = 4u32;
pub const TYPE_TRANSDATA: u32 = 2u32;
pub const TYPE_UNICODE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UFF_FILEHEADER {
    pub dwSignature: u32,
    pub dwVersion: u32,
    pub dwSize: u32,
    pub nFonts: u32,
    pub nGlyphSets: u32,
    pub nVarData: u32,
    pub offFontDir: u32,
    pub dwFlags: u32,
    pub dwReserved: [u32; 4],
}
impl Default for UFF_FILEHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UFF_FONTDIRECTORY {
    pub dwSignature: u32,
    pub wSize: u16,
    pub wFontID: u16,
    pub sGlyphID: i16,
    pub wFlags: u16,
    pub dwInstallerSig: u32,
    pub offFontName: u32,
    pub offCartridgeName: u32,
    pub offFontData: u32,
    pub offGlyphData: u32,
    pub offVarData: u32,
}
pub const UFF_VERSION_NUMBER: u32 = 65537u32;
pub const UFM_CART: u32 = 2u32;
pub const UFM_SCALABLE: u32 = 4u32;
pub const UFM_SOFT: u32 = 1u32;
pub const UFOFLAG_TTDOWNLOAD_BITMAP: u32 = 2u32;
pub const UFOFLAG_TTDOWNLOAD_TTOUTLINE: u32 = 4u32;
pub const UFOFLAG_TTFONT: u32 = 1u32;
pub const UFOFLAG_TTOUTLINE_BOLD_SIM: u32 = 8u32;
pub const UFOFLAG_TTOUTLINE_ITALIC_SIM: u32 = 16u32;
pub const UFOFLAG_TTOUTLINE_VERTICAL: u32 = 32u32;
pub const UFOFLAG_TTSUBSTITUTED: u32 = 64u32;
pub const UFO_GETINFO_FONTOBJ: u32 = 1u32;
pub const UFO_GETINFO_GLYPHBITMAP: u32 = 3u32;
pub const UFO_GETINFO_GLYPHSTRING: u32 = 2u32;
pub const UFO_GETINFO_GLYPHWIDTH: u32 = 4u32;
pub const UFO_GETINFO_MEMORY: u32 = 5u32;
pub const UFO_GETINFO_STDVARIABLE: u32 = 6u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UI_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNIDRVINFO {
    pub dwSize: u32,
    pub flGenFlags: u32,
    pub wType: u16,
    pub fCaps: u16,
    pub wXRes: u16,
    pub wYRes: u16,
    pub sYAdjust: i16,
    pub sYMoved: i16,
    pub wPrivateData: u16,
    pub sShift: i16,
    pub SelectFont: INVOC,
    pub UnSelectFont: INVOC,
    pub wReserved: [u16; 4],
}
impl Default for UNIDRVINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNIDRV_PRIVATE_DEVMODE {
    pub wReserved: [u16; 4],
    pub wSize: u16,
}
impl Default for UNIDRV_PRIVATE_DEVMODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNIFM_HDR {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub ulDefaultCodepage: u32,
    pub lGlyphSetDataRCID: i32,
    pub loUnidrvInfo: u32,
    pub loIFIMetrics: u32,
    pub loExtTextMetric: u32,
    pub loWidthTable: u32,
    pub loKernPair: u32,
    pub dwReserved: [u32; 2],
}
impl Default for UNIFM_HDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNIFM_VERSION_1_0: u32 = 65536u32;
pub const UNIRECTIONAL_NOTIFICATION_LOST: PrintAsyncNotifyError = PrintAsyncNotifyError(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNI_CODEPAGEINFO {
    pub dwCodePage: u32,
    pub SelectSymbolSet: INVOC,
    pub UnSelectSymbolSet: INVOC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNI_GLYPHSETDATA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub lPredefinedID: i32,
    pub dwGlyphCount: u32,
    pub dwRunCount: u32,
    pub loRunOffset: u32,
    pub dwCodePageCount: u32,
    pub loCodePageOffset: u32,
    pub loMapTableOffset: u32,
    pub dwReserved: [u32; 2],
}
impl Default for UNI_GLYPHSETDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNI_GLYPHSETDATA_VERSION_1_0: u32 = 65536u32;
pub const UNKNOWN_PROTOCOL: u32 = 0u32;
pub const UPDP_CHECK_DRIVERSTORE: u32 = 4u32;
pub const UPDP_SILENT_UPLOAD: u32 = 1u32;
pub const UPDP_UPLOAD_ALWAYS: u32 = 2u32;
pub const USBPRINT_IOCTL_INDEX: u32 = 0u32;
pub const USB_PRINTER_INTERFACE_CLASSIC: u32 = 1u32;
pub const USB_PRINTER_INTERFACE_DUAL: u32 = 3u32;
pub const USB_PRINTER_INTERFACE_IPP: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USERDATA {
    pub dwSize: u32,
    pub dwItemID: usize,
    pub pKeyWordName: windows_core::PSTR,
    pub dwReserved: [u32; 8],
}
impl Default for USERDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIDTHRUN {
    pub wStartGlyph: u16,
    pub wGlyphCount: u16,
    pub loCharWidthOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIDTHTABLE {
    pub dwSize: u32,
    pub dwRunNum: u32,
    pub WidthRun: [WIDTHRUN; 1],
}
impl Default for WIDTHTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WM_FI_FILENAME: u32 = 900u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPSRAS_BACKGROUND_COLOR(pub i32);
pub const XPSRAS_BACKGROUND_COLOR_OPAQUE: XPSRAS_BACKGROUND_COLOR = XPSRAS_BACKGROUND_COLOR(1i32);
pub const XPSRAS_BACKGROUND_COLOR_TRANSPARENT: XPSRAS_BACKGROUND_COLOR = XPSRAS_BACKGROUND_COLOR(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPSRAS_PIXEL_FORMAT(pub i32);
pub const XPSRAS_PIXEL_FORMAT_128BPP_PRGBA_FLOAT_SCRGB: XPSRAS_PIXEL_FORMAT = XPSRAS_PIXEL_FORMAT(3i32);
pub const XPSRAS_PIXEL_FORMAT_32BPP_PBGRA_UINT_SRGB: XPSRAS_PIXEL_FORMAT = XPSRAS_PIXEL_FORMAT(1i32);
pub const XPSRAS_PIXEL_FORMAT_64BPP_PRGBA_HALF_SCRGB: XPSRAS_PIXEL_FORMAT = XPSRAS_PIXEL_FORMAT(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPSRAS_RENDERING_MODE(pub i32);
pub const XPSRAS_RENDERING_MODE_ALIASED: XPSRAS_RENDERING_MODE = XPSRAS_RENDERING_MODE(1i32);
pub const XPSRAS_RENDERING_MODE_ANTIALIASED: XPSRAS_RENDERING_MODE = XPSRAS_RENDERING_MODE(0i32);
pub const XPS_FP_DRIVER_PROPERTY_BAG: windows_core::PCWSTR = windows_core::w!("DriverPropertyBag");
pub const XPS_FP_JOB_ID: windows_core::PCWSTR = windows_core::w!("PrintJobId");
pub const XPS_FP_JOB_LEVEL_PRINTTICKET: windows_core::PCWSTR = windows_core::w!("JobPrintTicket");
pub const XPS_FP_MERGED_DATAFILE_PATH: windows_core::PCWSTR = windows_core::w!("MergedDataFilePath");
pub const XPS_FP_MS_CONTENT_TYPE: windows_core::PCWSTR = windows_core::w!("DriverMultiContentType");
pub const XPS_FP_MS_CONTENT_TYPE_OPENXPS: windows_core::PCWSTR = windows_core::w!("OpenXPS");
pub const XPS_FP_MS_CONTENT_TYPE_XPS: windows_core::PCWSTR = windows_core::w!("XPS");
pub const XPS_FP_OUTPUT_FILE: windows_core::PCWSTR = windows_core::w!("PrintOutputFileName");
pub const XPS_FP_PRINTDEVICECAPABILITIES: windows_core::PCWSTR = windows_core::w!("PrintDeviceCapabilities");
pub const XPS_FP_PRINTER_HANDLE: windows_core::PCWSTR = windows_core::w!("PrinterHandle");
pub const XPS_FP_PRINTER_NAME: windows_core::PCWSTR = windows_core::w!("PrinterName");
pub const XPS_FP_PRINT_CLASS_FACTORY: windows_core::PCWSTR = windows_core::w!("PrintClassFactory");
pub const XPS_FP_PROGRESS_REPORT: windows_core::PCWSTR = windows_core::w!("ProgressReport");
pub const XPS_FP_QUEUE_PROPERTY_BAG: windows_core::PCWSTR = windows_core::w!("QueuePropertyBag");
pub const XPS_FP_RESOURCE_DLL_PATHS: windows_core::PCWSTR = windows_core::w!("ResourceDLLPaths");
pub const XPS_FP_USER_PRINT_TICKET: windows_core::PCWSTR = windows_core::w!("PerUserPrintTicket");
pub const XPS_FP_USER_TOKEN: windows_core::PCWSTR = windows_core::w!("UserSecurityToken");
pub const XpsJob_DocumentSequenceAdded: EXpsJobConsumption = EXpsJobConsumption(0i32);
pub const XpsJob_FixedDocumentAdded: EXpsJobConsumption = EXpsJobConsumption(1i32);
pub const XpsJob_FixedPageAdded: EXpsJobConsumption = EXpsJobConsumption(2i32);
pub const Xps_Restricted_Font_Editable: EXpsFontRestriction = EXpsFontRestriction(8i32);
pub const Xps_Restricted_Font_Installable: EXpsFontRestriction = EXpsFontRestriction(0i32);
pub const Xps_Restricted_Font_NoEmbedding: EXpsFontRestriction = EXpsFontRestriction(2i32);
pub const Xps_Restricted_Font_PreviewPrint: EXpsFontRestriction = EXpsFontRestriction(4i32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type _CPSUICALLBACK = Option<unsafe extern "system" fn(pcpsuicbparam: *mut CPSUICBPARAM) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _SPLCLIENT_INFO_2_V3 {
    pub hSplPrinter: u64,
}
pub const kADT_ASCII: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(5i32);
pub const kADT_BINARY: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(7i32);
pub const kADT_BOOL: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(1i32);
pub const kADT_CUSTOMSIZEPARAMS: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(10i32);
pub const kADT_DWORD: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(4i32);
pub const kADT_INT: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(2i32);
pub const kADT_LONG: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(3i32);
pub const kADT_RECT: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(9i32);
pub const kADT_SIZE: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(8i32);
pub const kADT_UNICODE: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(6i32);
pub const kADT_UNKNOWN: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(0i32);
pub const kAddingDocumentSequence: EPrintXPSJobProgress = EPrintXPSJobProgress(0i32);
pub const kAddingFixedDocument: EPrintXPSJobProgress = EPrintXPSJobProgress(2i32);
pub const kAddingFixedPage: EPrintXPSJobProgress = EPrintXPSJobProgress(4i32);
pub const kAllUsers: PrintAsyncNotifyUserFilter = PrintAsyncNotifyUserFilter(1i32);
pub const kBiDirectional: PrintAsyncNotifyConversationStyle = PrintAsyncNotifyConversationStyle(0i32);
pub const kDocumentSequenceAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(1i32);
pub const kFixedDocumentAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(3i32);
pub const kFixedPageAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(5i32);
pub const kFontAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(7i32);
pub const kImageAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(8i32);
pub const kInvalidJobState: EBranchOfficeJobEventType = EBranchOfficeJobEventType(0i32);
pub const kJobConsumption: EPrintXPSJobOperation = EPrintXPSJobOperation(2i32);
pub const kJobProduction: EPrintXPSJobOperation = EPrintXPSJobOperation(1i32);
pub const kLogJobError: EBranchOfficeJobEventType = EBranchOfficeJobEventType(3i32);
pub const kLogJobPipelineError: EBranchOfficeJobEventType = EBranchOfficeJobEventType(4i32);
pub const kLogJobPrinted: EBranchOfficeJobEventType = EBranchOfficeJobEventType(1i32);
pub const kLogJobRendered: EBranchOfficeJobEventType = EBranchOfficeJobEventType(2i32);
pub const kLogOfflineFileFull: EBranchOfficeJobEventType = EBranchOfficeJobEventType(5i32);
pub const kMessageBox: UI_TYPE = UI_TYPE(0i32);
pub const kPerUser: PrintAsyncNotifyUserFilter = PrintAsyncNotifyUserFilter(0i32);
pub const kPropertyTypeBuffer: EPrintPropertyType = EPrintPropertyType(10i32);
pub const kPropertyTypeByte: EPrintPropertyType = EPrintPropertyType(4i32);
pub const kPropertyTypeDevMode: EPrintPropertyType = EPrintPropertyType(6i32);
pub const kPropertyTypeInt32: EPrintPropertyType = EPrintPropertyType(2i32);
pub const kPropertyTypeInt64: EPrintPropertyType = EPrintPropertyType(3i32);
pub const kPropertyTypeNotificationOptions: EPrintPropertyType = EPrintPropertyType(9i32);
pub const kPropertyTypeNotificationReply: EPrintPropertyType = EPrintPropertyType(8i32);
pub const kPropertyTypeSD: EPrintPropertyType = EPrintPropertyType(7i32);
pub const kPropertyTypeString: EPrintPropertyType = EPrintPropertyType(1i32);
pub const kPropertyTypeTime: EPrintPropertyType = EPrintPropertyType(5i32);
pub const kResourceAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(6i32);
pub const kUniDirectional: PrintAsyncNotifyConversationStyle = PrintAsyncNotifyConversationStyle(1i32);
pub const kXpsDocumentCommitted: EPrintXPSJobProgress = EPrintXPSJobProgress(9i32);
