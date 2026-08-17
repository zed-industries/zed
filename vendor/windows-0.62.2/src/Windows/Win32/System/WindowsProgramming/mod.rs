#[inline]
pub unsafe fn AddDelBackupEntryA<P0, P1, P2>(lpcszfilelist: P0, lpcszbackupdir: P1, lpcszbasename: P2, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn AddDelBackupEntryA(lpcszfilelist : windows_core::PCSTR, lpcszbackupdir : windows_core::PCSTR, lpcszbasename : windows_core::PCSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { AddDelBackupEntryA(lpcszfilelist.param().abi(), lpcszbackupdir.param().abi(), lpcszbasename.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn AddDelBackupEntryW<P0, P1, P2>(lpcszfilelist: P0, lpcszbackupdir: P1, lpcszbasename: P2, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn AddDelBackupEntryW(lpcszfilelist : windows_core::PCWSTR, lpcszbackupdir : windows_core::PCWSTR, lpcszbasename : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { AddDelBackupEntryW(lpcszfilelist.param().abi(), lpcszbackupdir.param().abi(), lpcszbasename.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn AdvInstallFileA<P1, P2, P3, P4>(hwnd: super::super::Foundation::HWND, lpszsourcedir: P1, lpszsourcefile: P2, lpszdestdir: P3, lpszdestfile: P4, dwflags: u32, dwreserved: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn AdvInstallFileA(hwnd : super::super::Foundation:: HWND, lpszsourcedir : windows_core::PCSTR, lpszsourcefile : windows_core::PCSTR, lpszdestdir : windows_core::PCSTR, lpszdestfile : windows_core::PCSTR, dwflags : u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { AdvInstallFileA(hwnd, lpszsourcedir.param().abi(), lpszsourcefile.param().abi(), lpszdestdir.param().abi(), lpszdestfile.param().abi(), dwflags, dwreserved).ok() }
}
#[inline]
pub unsafe fn AdvInstallFileW<P1, P2, P3, P4>(hwnd: super::super::Foundation::HWND, lpszsourcedir: P1, lpszsourcefile: P2, lpszdestdir: P3, lpszdestfile: P4, dwflags: u32, dwreserved: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn AdvInstallFileW(hwnd : super::super::Foundation:: HWND, lpszsourcedir : windows_core::PCWSTR, lpszsourcefile : windows_core::PCWSTR, lpszdestdir : windows_core::PCWSTR, lpszdestfile : windows_core::PCWSTR, dwflags : u32, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { AdvInstallFileW(hwnd, lpszsourcedir.param().abi(), lpszsourcefile.param().abi(), lpszdestdir.param().abi(), lpszdestfile.param().abi(), dwflags, dwreserved).ok() }
}
#[inline]
pub unsafe fn ApphelpCheckShellObject(objectclsid: *const windows_core::GUID, bshimifnecessary: bool, pullflags: *mut u64) -> windows_core::BOOL {
    windows_core::link!("apphelp.dll" "system" fn ApphelpCheckShellObject(objectclsid : *const windows_core::GUID, bshimifnecessary : windows_core::BOOL, pullflags : *mut u64) -> windows_core::BOOL);
    unsafe { ApphelpCheckShellObject(objectclsid, bshimifnecessary.into(), pullflags as _) }
}
#[inline]
pub unsafe fn CancelDeviceWakeupRequest(hdevice: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelDeviceWakeupRequest(hdevice : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { CancelDeviceWakeupRequest(hdevice) }
}
#[inline]
pub unsafe fn CloseINFEngine(hinf: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("advpack.dll" "system" fn CloseINFEngine(hinf : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CloseINFEngine(hinf as _).ok() }
}
#[inline]
pub unsafe fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue: u64, lpperformancecountervalue: *mut u64, lpconversionerror: Option<*mut u64>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue : u64, lpperformancecountervalue : *mut u64, lpconversionerror : *mut u64) -> windows_core::HRESULT);
    unsafe { ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue, lpperformancecountervalue as _, lpconversionerror.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue: u64, lpauxiliarycountervalue: *mut u64, lpconversionerror: Option<*mut u64>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue : u64, lpauxiliarycountervalue : *mut u64, lpconversionerror : *mut u64) -> windows_core::HRESULT);
    unsafe { ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue, lpauxiliarycountervalue as _, lpconversionerror.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DCIBeginAccess(pdci: *mut DCISURFACEINFO, x: i32, y: i32, dx: i32, dy: i32) -> i32 {
    windows_core::link!("dciman32.dll" "system" fn DCIBeginAccess(pdci : *mut DCISURFACEINFO, x : i32, y : i32, dx : i32, dy : i32) -> i32);
    unsafe { DCIBeginAccess(pdci as _, x, y, dx, dy) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICloseProvider(hdc: super::super::Graphics::Gdi::HDC) {
    windows_core::link!("dciman32.dll" "system" fn DCICloseProvider(hdc : super::super::Graphics::Gdi:: HDC));
    unsafe { DCICloseProvider(hdc) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreateOffscreen(hdc: super::super::Graphics::Gdi::HDC, dwcompression: u32, dwredmask: u32, dwgreenmask: u32, dwbluemask: u32, dwwidth: u32, dwheight: u32, dwdcicaps: u32, dwbitcount: u32, lplpsurface: *mut *mut DCIOFFSCREEN) -> i32 {
    windows_core::link!("dciman32.dll" "system" fn DCICreateOffscreen(hdc : super::super::Graphics::Gdi:: HDC, dwcompression : u32, dwredmask : u32, dwgreenmask : u32, dwbluemask : u32, dwwidth : u32, dwheight : u32, dwdcicaps : u32, dwbitcount : u32, lplpsurface : *mut *mut DCIOFFSCREEN) -> i32);
    unsafe { DCICreateOffscreen(hdc, dwcompression, dwredmask, dwgreenmask, dwbluemask, dwwidth, dwheight, dwdcicaps, dwbitcount, lplpsurface as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreateOverlay(hdc: super::super::Graphics::Gdi::HDC, lpoffscreensurf: *mut core::ffi::c_void, lplpsurface: *mut *mut DCIOVERLAY) -> i32 {
    windows_core::link!("dciman32.dll" "system" fn DCICreateOverlay(hdc : super::super::Graphics::Gdi:: HDC, lpoffscreensurf : *mut core::ffi::c_void, lplpsurface : *mut *mut DCIOVERLAY) -> i32);
    unsafe { DCICreateOverlay(hdc, lpoffscreensurf as _, lplpsurface as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreatePrimary(hdc: super::super::Graphics::Gdi::HDC, lplpsurface: *mut *mut DCISURFACEINFO) -> i32 {
    windows_core::link!("dciman32.dll" "system" fn DCICreatePrimary(hdc : super::super::Graphics::Gdi:: HDC, lplpsurface : *mut *mut DCISURFACEINFO) -> i32);
    unsafe { DCICreatePrimary(hdc, lplpsurface as _) }
}
#[inline]
pub unsafe fn DCIDestroy(pdci: *mut DCISURFACEINFO) {
    windows_core::link!("dciman32.dll" "system" fn DCIDestroy(pdci : *mut DCISURFACEINFO));
    unsafe { DCIDestroy(pdci as _) }
}
#[inline]
pub unsafe fn DCIDraw(pdci: *mut DCIOFFSCREEN) -> i32 {
    windows_core::link!("dciman32.dll" "system" fn DCIDraw(pdci : *mut DCIOFFSCREEN) -> i32);
    unsafe { DCIDraw(pdci as _) }
}
#[inline]
pub unsafe fn DCIEndAccess(pdci: *mut DCISURFACEINFO) {
    windows_core::link!("dciman32.dll" "system" fn DCIEndAccess(pdci : *mut DCISURFACEINFO));
    unsafe { DCIEndAccess(pdci as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCIEnum(hdc: super::super::Graphics::Gdi::HDC, lprdst: *mut super::super::Foundation::RECT, lprsrc: *mut super::super::Foundation::RECT, lpfncallback: *mut core::ffi::c_void, lpcontext: *mut core::ffi::c_void) -> i32 {
    windows_core::link!("dciman32.dll" "system" fn DCIEnum(hdc : super::super::Graphics::Gdi:: HDC, lprdst : *mut super::super::Foundation:: RECT, lprsrc : *mut super::super::Foundation:: RECT, lpfncallback : *mut core::ffi::c_void, lpcontext : *mut core::ffi::c_void) -> i32);
    unsafe { DCIEnum(hdc, lprdst as _, lprsrc as _, lpfncallback as _, lpcontext as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCIOpenProvider() -> super::super::Graphics::Gdi::HDC {
    windows_core::link!("dciman32.dll" "system" fn DCIOpenProvider() -> super::super::Graphics::Gdi:: HDC);
    unsafe { DCIOpenProvider() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCISetClipList(pdci: *mut DCIOFFSCREEN, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32 {
    windows_core::link!("dciman32.dll" "system" fn DCISetClipList(pdci : *mut DCIOFFSCREEN, prd : *mut super::super::Graphics::Gdi:: RGNDATA) -> i32);
    unsafe { DCISetClipList(pdci as _, prd as _) }
}
#[inline]
pub unsafe fn DCISetDestination(pdci: *mut DCIOFFSCREEN, dst: *mut super::super::Foundation::RECT, src: *mut super::super::Foundation::RECT) -> i32 {
    windows_core::link!("dciman32.dll" "system" fn DCISetDestination(pdci : *mut DCIOFFSCREEN, dst : *mut super::super::Foundation:: RECT, src : *mut super::super::Foundation:: RECT) -> i32);
    unsafe { DCISetDestination(pdci as _, dst as _, src as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCISetSrcDestClip(pdci: *mut DCIOFFSCREEN, srcrc: *mut super::super::Foundation::RECT, destrc: *mut super::super::Foundation::RECT, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32 {
    windows_core::link!("dciman32.dll" "system" fn DCISetSrcDestClip(pdci : *mut DCIOFFSCREEN, srcrc : *mut super::super::Foundation:: RECT, destrc : *mut super::super::Foundation:: RECT, prd : *mut super::super::Graphics::Gdi:: RGNDATA) -> i32);
    unsafe { DCISetSrcDestClip(pdci as _, srcrc as _, destrc as _, prd as _) }
}
#[inline]
pub unsafe fn DelNodeA<P0>(pszfileordirname: P0, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn DelNodeA(pszfileordirname : windows_core::PCSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { DelNodeA(pszfileordirname.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn DelNodeRunDLL32W(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: windows_core::PWSTR, nshow: i32) -> windows_core::Result<()> {
    windows_core::link!("advpack.dll" "system" fn DelNodeRunDLL32W(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_core::PWSTR, nshow : i32) -> windows_core::HRESULT);
    unsafe { DelNodeRunDLL32W(hwnd, hinstance, core::mem::transmute(pszparms), nshow).ok() }
}
#[inline]
pub unsafe fn DelNodeW<P0>(pszfileordirname: P0, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn DelNodeW(pszfileordirname : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { DelNodeW(pszfileordirname.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn DnsHostnameToComputerNameA<P0>(hostname: P0, computername: Option<windows_core::PSTR>, nsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameA(hostname : windows_core::PCSTR, computername : windows_core::PSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { DnsHostnameToComputerNameA(hostname.param().abi(), computername.unwrap_or(core::mem::zeroed()) as _, nsize as _).ok() }
}
#[inline]
pub unsafe fn DnsHostnameToComputerNameW<P0>(hostname: P0, computername: Option<windows_core::PWSTR>, nsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameW(hostname : windows_core::PCWSTR, computername : windows_core::PWSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { DnsHostnameToComputerNameW(hostname.param().abi(), computername.unwrap_or(core::mem::zeroed()) as _, nsize as _).ok() }
}
#[inline]
pub unsafe fn DosDateTimeToFileTime(wfatdate: u16, wfattime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn DosDateTimeToFileTime(wfatdate : u16, wfattime : u16, lpfiletime : *mut super::super::Foundation:: FILETIME) -> windows_core::BOOL);
    unsafe { DosDateTimeToFileTime(wfatdate, wfattime, lpfiletime as _).ok() }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn EnableProcessOptionalXStateFeatures(features: u64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnableProcessOptionalXStateFeatures(features : u64) -> windows_core::BOOL);
    unsafe { EnableProcessOptionalXStateFeatures(features) }
}
#[inline]
pub unsafe fn ExecuteCabA(hwnd: super::super::Foundation::HWND, pcab: *mut CABINFOA, preserved: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("advpack.dll" "system" fn ExecuteCabA(hwnd : super::super::Foundation:: HWND, pcab : *mut CABINFOA, preserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ExecuteCabA(hwnd, pcab as _, preserved as _).ok() }
}
#[inline]
pub unsafe fn ExecuteCabW(hwnd: super::super::Foundation::HWND, pcab: *mut CABINFOW, preserved: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("advpack.dll" "system" fn ExecuteCabW(hwnd : super::super::Foundation:: HWND, pcab : *mut CABINFOW, preserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ExecuteCabW(hwnd, pcab as _, preserved as _).ok() }
}
#[inline]
pub unsafe fn ExtractFilesA<P0, P1, P3>(pszcabname: P0, pszexpanddir: P1, dwflags: u32, pszfilelist: P3, lpreserved: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn ExtractFilesA(pszcabname : windows_core::PCSTR, pszexpanddir : windows_core::PCSTR, dwflags : u32, pszfilelist : windows_core::PCSTR, lpreserved : *mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { ExtractFilesA(pszcabname.param().abi(), pszexpanddir.param().abi(), dwflags, pszfilelist.param().abi(), lpreserved as _, dwreserved).ok() }
}
#[inline]
pub unsafe fn ExtractFilesW<P0, P1, P3>(pszcabname: P0, pszexpanddir: P1, dwflags: u32, pszfilelist: P3, lpreserved: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn ExtractFilesW(pszcabname : windows_core::PCWSTR, pszexpanddir : windows_core::PCWSTR, dwflags : u32, pszfilelist : windows_core::PCWSTR, lpreserved : *mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { ExtractFilesW(pszcabname.param().abi(), pszexpanddir.param().abi(), dwflags, pszfilelist.param().abi(), lpreserved as _, dwreserved).ok() }
}
#[inline]
pub unsafe fn FileSaveMarkNotExistA<P0, P1, P2>(lpfilelist: P0, lpdir: P1, lpbasename: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn FileSaveMarkNotExistA(lpfilelist : windows_core::PCSTR, lpdir : windows_core::PCSTR, lpbasename : windows_core::PCSTR) -> windows_core::HRESULT);
    unsafe { FileSaveMarkNotExistA(lpfilelist.param().abi(), lpdir.param().abi(), lpbasename.param().abi()).ok() }
}
#[inline]
pub unsafe fn FileSaveMarkNotExistW<P0, P1, P2>(lpfilelist: P0, lpdir: P1, lpbasename: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn FileSaveMarkNotExistW(lpfilelist : windows_core::PCWSTR, lpdir : windows_core::PCWSTR, lpbasename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { FileSaveMarkNotExistW(lpfilelist.param().abi(), lpdir.param().abi(), lpbasename.param().abi()).ok() }
}
#[inline]
pub unsafe fn FileSaveRestoreOnINFA<P1, P2, P3, P4, P5>(hwnd: super::super::Foundation::HWND, psztitle: P1, pszinf: P2, pszsection: P3, pszbackupdir: P4, pszbasebackupfile: P5, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn FileSaveRestoreOnINFA(hwnd : super::super::Foundation:: HWND, psztitle : windows_core::PCSTR, pszinf : windows_core::PCSTR, pszsection : windows_core::PCSTR, pszbackupdir : windows_core::PCSTR, pszbasebackupfile : windows_core::PCSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { FileSaveRestoreOnINFA(hwnd, psztitle.param().abi(), pszinf.param().abi(), pszsection.param().abi(), pszbackupdir.param().abi(), pszbasebackupfile.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn FileSaveRestoreOnINFW<P1, P2, P3, P4, P5>(hwnd: super::super::Foundation::HWND, psztitle: P1, pszinf: P2, pszsection: P3, pszbackupdir: P4, pszbasebackupfile: P5, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn FileSaveRestoreOnINFW(hwnd : super::super::Foundation:: HWND, psztitle : windows_core::PCWSTR, pszinf : windows_core::PCWSTR, pszsection : windows_core::PCWSTR, pszbackupdir : windows_core::PCWSTR, pszbasebackupfile : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { FileSaveRestoreOnINFW(hwnd, psztitle.param().abi(), pszinf.param().abi(), pszsection.param().abi(), pszbackupdir.param().abi(), pszbasebackupfile.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn FileSaveRestoreW<P1, P2, P3>(hdlg: super::super::Foundation::HWND, lpfilelist: P1, lpdir: P2, lpbasename: P3, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn FileSaveRestoreW(hdlg : super::super::Foundation:: HWND, lpfilelist : windows_core::PCWSTR, lpdir : windows_core::PCWSTR, lpbasename : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { FileSaveRestoreW(hdlg, lpfilelist.param().abi(), lpdir.param().abi(), lpbasename.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn FileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpfatdate: *mut u16, lpfattime: *mut u16) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn FileTimeToDosDateTime(lpfiletime : *const super::super::Foundation:: FILETIME, lpfatdate : *mut u16, lpfattime : *mut u16) -> windows_core::BOOL);
    unsafe { FileTimeToDosDateTime(lpfiletime, lpfatdate as _, lpfattime as _).ok() }
}
#[inline]
pub unsafe fn GdiEntry13() -> u32 {
    windows_core::link!("api-ms-win-dx-d3dkmt-l1-1-0.dll" "system" fn GdiEntry13() -> u32);
    unsafe { GdiEntry13() }
}
#[inline]
pub unsafe fn GetComputerNameA(lpbuffer: Option<windows_core::PSTR>, nsize: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetComputerNameA(lpbuffer : windows_core::PSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetComputerNameA(lpbuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _).ok() }
}
#[inline]
pub unsafe fn GetComputerNameW(lpbuffer: Option<windows_core::PWSTR>, nsize: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetComputerNameW(lpbuffer : windows_core::PWSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetComputerNameW(lpbuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _).ok() }
}
#[inline]
pub unsafe fn GetCurrentHwProfileA(lphwprofileinfo: *mut HW_PROFILE_INFOA) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn GetCurrentHwProfileA(lphwprofileinfo : *mut HW_PROFILE_INFOA) -> windows_core::BOOL);
    unsafe { GetCurrentHwProfileA(lphwprofileinfo as _).ok() }
}
#[inline]
pub unsafe fn GetCurrentHwProfileW(lphwprofileinfo: *mut HW_PROFILE_INFOW) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn GetCurrentHwProfileW(lphwprofileinfo : *mut HW_PROFILE_INFOW) -> windows_core::BOOL);
    unsafe { GetCurrentHwProfileW(lphwprofileinfo as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetDCRegionData(hdc: super::super::Graphics::Gdi::HDC, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32 {
    windows_core::link!("dciman32.dll" "system" fn GetDCRegionData(hdc : super::super::Graphics::Gdi:: HDC, size : u32, prd : *mut super::super::Graphics::Gdi:: RGNDATA) -> u32);
    unsafe { GetDCRegionData(hdc, size, prd as _) }
}
#[inline]
pub unsafe fn GetFeatureEnabledState(featureid: u32, changetime: FEATURE_CHANGE_TIME) -> FEATURE_ENABLED_STATE {
    windows_core::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn GetFeatureEnabledState(featureid : u32, changetime : FEATURE_CHANGE_TIME) -> FEATURE_ENABLED_STATE);
    unsafe { GetFeatureEnabledState(featureid, changetime) }
}
#[inline]
pub unsafe fn GetFeatureVariant(featureid: u32, changetime: FEATURE_CHANGE_TIME, payloadid: *mut u32, hasnotification: *mut windows_core::BOOL) -> u32 {
    windows_core::link!("api-ms-win-core-featurestaging-l1-1-1.dll" "system" fn GetFeatureVariant(featureid : u32, changetime : FEATURE_CHANGE_TIME, payloadid : *mut u32, hasnotification : *mut windows_core::BOOL) -> u32);
    unsafe { GetFeatureVariant(featureid, changetime, payloadid as _, hasnotification as _) }
}
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableA<P0, P1>(lpname: P0, lpguid: P1, pbuffer: Option<*mut core::ffi::c_void>, nsize: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableA(lpname : windows_core::PCSTR, lpguid : windows_core::PCSTR, pbuffer : *mut core::ffi::c_void, nsize : u32) -> u32);
    unsafe { GetFirmwareEnvironmentVariableA(lpname.param().abi(), lpguid.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableExA<P0, P1>(lpname: P0, lpguid: P1, pbuffer: Option<*mut core::ffi::c_void>, nsize: u32, pdwattribubutes: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableExA(lpname : windows_core::PCSTR, lpguid : windows_core::PCSTR, pbuffer : *mut core::ffi::c_void, nsize : u32, pdwattribubutes : *mut u32) -> u32);
    unsafe { GetFirmwareEnvironmentVariableExA(lpname.param().abi(), lpguid.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, nsize, pdwattribubutes.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableExW<P0, P1>(lpname: P0, lpguid: P1, pbuffer: Option<*mut core::ffi::c_void>, nsize: u32, pdwattribubutes: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableExW(lpname : windows_core::PCWSTR, lpguid : windows_core::PCWSTR, pbuffer : *mut core::ffi::c_void, nsize : u32, pdwattribubutes : *mut u32) -> u32);
    unsafe { GetFirmwareEnvironmentVariableExW(lpname.param().abi(), lpguid.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, nsize, pdwattribubutes.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableW<P0, P1>(lpname: P0, lpguid: P1, pbuffer: Option<*mut core::ffi::c_void>, nsize: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableW(lpname : windows_core::PCWSTR, lpguid : windows_core::PCWSTR, pbuffer : *mut core::ffi::c_void, nsize : u32) -> u32);
    unsafe { GetFirmwareEnvironmentVariableW(lpname.param().abi(), lpguid.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[inline]
pub unsafe fn GetPrivateProfileIntA<P0, P1, P3>(lpappname: P0, lpkeyname: P1, ndefault: i32, lpfilename: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileIntA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, ndefault : i32, lpfilename : windows_core::PCSTR) -> u32);
    unsafe { GetPrivateProfileIntA(lpappname.param().abi(), lpkeyname.param().abi(), ndefault, lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn GetPrivateProfileIntW<P0, P1, P3>(lpappname: P0, lpkeyname: P1, ndefault: i32, lpfilename: P3) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileIntW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, ndefault : i32, lpfilename : windows_core::PCWSTR) -> i32);
    unsafe { GetPrivateProfileIntW(lpappname.param().abi(), lpkeyname.param().abi(), ndefault, lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn GetPrivateProfileSectionA<P0, P3>(lpappname: P0, lpreturnedstring: Option<&mut [u8]>, lpfilename: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileSectionA(lpappname : windows_core::PCSTR, lpreturnedstring : windows_core::PSTR, nsize : u32, lpfilename : windows_core::PCSTR) -> u32);
    unsafe { GetPrivateProfileSectionA(lpappname.param().abi(), core::mem::transmute(lpreturnedstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn GetPrivateProfileSectionNamesA<P2>(lpszreturnbuffer: Option<&mut [u8]>, lpfilename: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileSectionNamesA(lpszreturnbuffer : windows_core::PSTR, nsize : u32, lpfilename : windows_core::PCSTR) -> u32);
    unsafe { GetPrivateProfileSectionNamesA(core::mem::transmute(lpszreturnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszreturnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn GetPrivateProfileSectionNamesW<P2>(lpszreturnbuffer: Option<&mut [u16]>, lpfilename: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileSectionNamesW(lpszreturnbuffer : windows_core::PWSTR, nsize : u32, lpfilename : windows_core::PCWSTR) -> u32);
    unsafe { GetPrivateProfileSectionNamesW(core::mem::transmute(lpszreturnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszreturnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn GetPrivateProfileSectionW<P0, P3>(lpappname: P0, lpreturnedstring: Option<&mut [u16]>, lpfilename: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileSectionW(lpappname : windows_core::PCWSTR, lpreturnedstring : windows_core::PWSTR, nsize : u32, lpfilename : windows_core::PCWSTR) -> u32);
    unsafe { GetPrivateProfileSectionW(lpappname.param().abi(), core::mem::transmute(lpreturnedstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn GetPrivateProfileStringA<P0, P1, P2, P5>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: Option<&mut [u8]>, lpfilename: P5) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileStringA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, lpdefault : windows_core::PCSTR, lpreturnedstring : windows_core::PSTR, nsize : u32, lpfilename : windows_core::PCSTR) -> u32);
    unsafe { GetPrivateProfileStringA(lpappname.param().abi(), lpkeyname.param().abi(), lpdefault.param().abi(), core::mem::transmute(lpreturnedstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn GetPrivateProfileStringW<P0, P1, P2, P5>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: Option<&mut [u16]>, lpfilename: P5) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileStringW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, lpdefault : windows_core::PCWSTR, lpreturnedstring : windows_core::PWSTR, nsize : u32, lpfilename : windows_core::PCWSTR) -> u32);
    unsafe { GetPrivateProfileStringW(lpappname.param().abi(), lpkeyname.param().abi(), lpdefault.param().abi(), core::mem::transmute(lpreturnedstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn GetPrivateProfileStructA<P0, P1, P4>(lpszsection: P0, lpszkey: P1, lpstruct: Option<*mut core::ffi::c_void>, usizestruct: u32, szfile: P4) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileStructA(lpszsection : windows_core::PCSTR, lpszkey : windows_core::PCSTR, lpstruct : *mut core::ffi::c_void, usizestruct : u32, szfile : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { GetPrivateProfileStructA(lpszsection.param().abi(), lpszkey.param().abi(), lpstruct.unwrap_or(core::mem::zeroed()) as _, usizestruct, szfile.param().abi()) }
}
#[inline]
pub unsafe fn GetPrivateProfileStructW<P0, P1, P4>(lpszsection: P0, lpszkey: P1, lpstruct: Option<*mut core::ffi::c_void>, usizestruct: u32, szfile: P4) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileStructW(lpszsection : windows_core::PCWSTR, lpszkey : windows_core::PCWSTR, lpstruct : *mut core::ffi::c_void, usizestruct : u32, szfile : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { GetPrivateProfileStructW(lpszsection.param().abi(), lpszkey.param().abi(), lpstruct.unwrap_or(core::mem::zeroed()) as _, usizestruct, szfile.param().abi()) }
}
#[inline]
pub unsafe fn GetProfileIntA<P0, P1>(lpappname: P0, lpkeyname: P1, ndefault: i32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileIntA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, ndefault : i32) -> u32);
    unsafe { GetProfileIntA(lpappname.param().abi(), lpkeyname.param().abi(), ndefault) }
}
#[inline]
pub unsafe fn GetProfileIntW<P0, P1>(lpappname: P0, lpkeyname: P1, ndefault: i32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileIntW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, ndefault : i32) -> u32);
    unsafe { GetProfileIntW(lpappname.param().abi(), lpkeyname.param().abi(), ndefault) }
}
#[inline]
pub unsafe fn GetProfileSectionA<P0>(lpappname: P0, lpreturnedstring: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileSectionA(lpappname : windows_core::PCSTR, lpreturnedstring : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetProfileSectionA(lpappname.param().abi(), core::mem::transmute(lpreturnedstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetProfileSectionW<P0>(lpappname: P0, lpreturnedstring: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileSectionW(lpappname : windows_core::PCWSTR, lpreturnedstring : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetProfileSectionW(lpappname.param().abi(), core::mem::transmute(lpreturnedstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetProfileStringA<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileStringA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, lpdefault : windows_core::PCSTR, lpreturnedstring : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetProfileStringA(lpappname.param().abi(), lpkeyname.param().abi(), lpdefault.param().abi(), core::mem::transmute(lpreturnedstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetProfileStringW<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileStringW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, lpdefault : windows_core::PCWSTR, lpreturnedstring : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetProfileStringW(lpappname.param().abi(), lpkeyname.param().abi(), lpdefault.param().abi(), core::mem::transmute(lpreturnedstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemRegistryQuota(pdwquotaallowed: Option<*mut u32>, pdwquotaused: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetSystemRegistryQuota(pdwquotaallowed : *mut u32, pdwquotaused : *mut u32) -> windows_core::BOOL);
    unsafe { GetSystemRegistryQuota(pdwquotaallowed.unwrap_or(core::mem::zeroed()) as _, pdwquotaused.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetThreadEnabledXStateFeatures() -> u64 {
    windows_core::link!("kernel32.dll" "system" fn GetThreadEnabledXStateFeatures() -> u64);
    unsafe { GetThreadEnabledXStateFeatures() }
}
#[inline]
pub unsafe fn GetUserNameA(lpbuffer: Option<windows_core::PSTR>, pcbbuffer: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn GetUserNameA(lpbuffer : windows_core::PSTR, pcbbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetUserNameA(lpbuffer.unwrap_or(core::mem::zeroed()) as _, pcbbuffer as _).ok() }
}
#[inline]
pub unsafe fn GetUserNameW(lpbuffer: Option<windows_core::PWSTR>, pcbbuffer: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn GetUserNameW(lpbuffer : windows_core::PWSTR, pcbbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetUserNameW(lpbuffer.unwrap_or(core::mem::zeroed()) as _, pcbbuffer as _).ok() }
}
#[inline]
pub unsafe fn GetVersionFromFileA<P0>(lpszfilename: P0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: bool) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn GetVersionFromFileA(lpszfilename : windows_core::PCSTR, pdwmsver : *mut u32, pdwlsver : *mut u32, bversion : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { GetVersionFromFileA(lpszfilename.param().abi(), pdwmsver as _, pdwlsver as _, bversion.into()).ok() }
}
#[inline]
pub unsafe fn GetVersionFromFileExA<P0>(lpszfilename: P0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: bool) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn GetVersionFromFileExA(lpszfilename : windows_core::PCSTR, pdwmsver : *mut u32, pdwlsver : *mut u32, bversion : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { GetVersionFromFileExA(lpszfilename.param().abi(), pdwmsver as _, pdwlsver as _, bversion.into()).ok() }
}
#[inline]
pub unsafe fn GetVersionFromFileExW<P0>(lpszfilename: P0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: bool) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn GetVersionFromFileExW(lpszfilename : windows_core::PCWSTR, pdwmsver : *mut u32, pdwlsver : *mut u32, bversion : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { GetVersionFromFileExW(lpszfilename.param().abi(), pdwmsver as _, pdwlsver as _, bversion.into()).ok() }
}
#[inline]
pub unsafe fn GetVersionFromFileW<P0>(lpszfilename: P0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: bool) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn GetVersionFromFileW(lpszfilename : windows_core::PCWSTR, pdwmsver : *mut u32, pdwlsver : *mut u32, bversion : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { GetVersionFromFileW(lpszfilename.param().abi(), pdwmsver as _, pdwlsver as _, bversion.into()).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetWindowRegionData(hwnd: super::super::Foundation::HWND, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32 {
    windows_core::link!("dciman32.dll" "system" fn GetWindowRegionData(hwnd : super::super::Foundation:: HWND, size : u32, prd : *mut super::super::Graphics::Gdi:: RGNDATA) -> u32);
    unsafe { GetWindowRegionData(hwnd, size, prd as _) }
}
#[inline]
pub unsafe fn GlobalCompact(dwminfree: u32) -> usize {
    windows_core::link!("kernel32.dll" "system" fn GlobalCompact(dwminfree : u32) -> usize);
    unsafe { GlobalCompact(dwminfree) }
}
#[inline]
pub unsafe fn GlobalFix(hmem: super::super::Foundation::HGLOBAL) {
    windows_core::link!("kernel32.dll" "system" fn GlobalFix(hmem : super::super::Foundation:: HGLOBAL));
    unsafe { GlobalFix(hmem) }
}
#[inline]
pub unsafe fn GlobalUnWire(hmem: super::super::Foundation::HGLOBAL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GlobalUnWire(hmem : super::super::Foundation:: HGLOBAL) -> windows_core::BOOL);
    unsafe { GlobalUnWire(hmem) }
}
#[inline]
pub unsafe fn GlobalUnfix(hmem: super::super::Foundation::HGLOBAL) {
    windows_core::link!("kernel32.dll" "system" fn GlobalUnfix(hmem : super::super::Foundation:: HGLOBAL));
    unsafe { GlobalUnfix(hmem) }
}
#[inline]
pub unsafe fn GlobalWire(hmem: super::super::Foundation::HGLOBAL) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn GlobalWire(hmem : super::super::Foundation:: HGLOBAL) -> *mut core::ffi::c_void);
    unsafe { GlobalWire(hmem) }
}
#[inline]
pub unsafe fn IMPGetIMEA(param0: super::super::Foundation::HWND, param1: *mut IMEPROA) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IMPGetIMEA(param0 : super::super::Foundation:: HWND, param1 : *mut IMEPROA) -> windows_core::BOOL);
    unsafe { IMPGetIMEA(param0, param1 as _) }
}
#[inline]
pub unsafe fn IMPGetIMEW(param0: super::super::Foundation::HWND, param1: *mut IMEPROW) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IMPGetIMEW(param0 : super::super::Foundation:: HWND, param1 : *mut IMEPROW) -> windows_core::BOOL);
    unsafe { IMPGetIMEW(param0, param1 as _) }
}
#[inline]
pub unsafe fn IMPQueryIMEA(param0: *mut IMEPROA) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IMPQueryIMEA(param0 : *mut IMEPROA) -> windows_core::BOOL);
    unsafe { IMPQueryIMEA(param0 as _) }
}
#[inline]
pub unsafe fn IMPQueryIMEW(param0: *mut IMEPROW) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IMPQueryIMEW(param0 : *mut IMEPROW) -> windows_core::BOOL);
    unsafe { IMPQueryIMEW(param0 as _) }
}
#[inline]
pub unsafe fn IMPSetIMEA(param0: super::super::Foundation::HWND, param1: *mut IMEPROA) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IMPSetIMEA(param0 : super::super::Foundation:: HWND, param1 : *mut IMEPROA) -> windows_core::BOOL);
    unsafe { IMPSetIMEA(param0, param1 as _) }
}
#[inline]
pub unsafe fn IMPSetIMEW(param0: super::super::Foundation::HWND, param1: *mut IMEPROW) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IMPSetIMEW(param0 : super::super::Foundation:: HWND, param1 : *mut IMEPROW) -> windows_core::BOOL);
    unsafe { IMPSetIMEW(param0, param1 as _) }
}
#[inline]
pub unsafe fn IsApiSetImplemented<P0>(contract: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("api-ms-win-core-apiquery-l2-1-0.dll" "system" fn IsApiSetImplemented(contract : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { IsApiSetImplemented(contract.param().abi()) }
}
#[inline]
pub unsafe fn IsBadHugeReadPtr(lp: Option<*const core::ffi::c_void>, ucb: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsBadHugeReadPtr(lp : *const core::ffi::c_void, ucb : usize) -> windows_core::BOOL);
    unsafe { IsBadHugeReadPtr(lp.unwrap_or(core::mem::zeroed()) as _, ucb) }
}
#[inline]
pub unsafe fn IsBadHugeWritePtr(lp: Option<*const core::ffi::c_void>, ucb: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsBadHugeWritePtr(lp : *const core::ffi::c_void, ucb : usize) -> windows_core::BOOL);
    unsafe { IsBadHugeWritePtr(lp.unwrap_or(core::mem::zeroed()) as _, ucb) }
}
#[inline]
pub unsafe fn IsNTAdmin(dwreserved: u32, lpdwreserved: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advpack.dll" "system" fn IsNTAdmin(dwreserved : u32, lpdwreserved : *mut u32) -> windows_core::BOOL);
    unsafe { IsNTAdmin(dwreserved, lpdwreserved as _) }
}
#[inline]
pub unsafe fn IsNativeVhdBoot(nativevhdboot: *mut windows_core::BOOL) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn IsNativeVhdBoot(nativevhdboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { IsNativeVhdBoot(nativevhdboot as _).ok() }
}
#[inline]
pub unsafe fn IsTokenUntrusted(tokenhandle: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsTokenUntrusted(tokenhandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { IsTokenUntrusted(tokenhandle) }
}
#[inline]
pub unsafe fn LaunchINFSectionExW<P2>(hwnd: Option<super::super::Foundation::HWND>, hinstance: Option<super::super::Foundation::HINSTANCE>, pszparms: P2, nshow: i32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn LaunchINFSectionExW(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_core::PCWSTR, nshow : i32) -> windows_core::HRESULT);
    unsafe { LaunchINFSectionExW(hwnd.unwrap_or(core::mem::zeroed()) as _, hinstance.unwrap_or(core::mem::zeroed()) as _, pszparms.param().abi(), nshow).ok() }
}
#[inline]
pub unsafe fn LaunchINFSectionW(hwndowner: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparams: windows_core::PWSTR, nshow: i32) -> i32 {
    windows_core::link!("advpack.dll" "system" fn LaunchINFSectionW(hwndowner : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparams : windows_core::PWSTR, nshow : i32) -> i32);
    unsafe { LaunchINFSectionW(hwndowner, hinstance, core::mem::transmute(pszparams), nshow) }
}
#[inline]
pub unsafe fn LocalCompact(uminfree: u32) -> usize {
    windows_core::link!("kernel32.dll" "system" fn LocalCompact(uminfree : u32) -> usize);
    unsafe { LocalCompact(uminfree) }
}
#[inline]
pub unsafe fn LocalShrink(hmem: super::super::Foundation::HLOCAL, cbnewsize: u32) -> usize {
    windows_core::link!("kernel32.dll" "system" fn LocalShrink(hmem : super::super::Foundation:: HLOCAL, cbnewsize : u32) -> usize);
    unsafe { LocalShrink(hmem, cbnewsize) }
}
#[inline]
pub unsafe fn MulDiv(nnumber: i32, nnumerator: i32, ndenominator: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn MulDiv(nnumber : i32, nnumerator : i32, ndenominator : i32) -> i32);
    unsafe { MulDiv(nnumber, nnumerator, ndenominator) }
}
#[inline]
pub unsafe fn NeedReboot(dwrebootcheck: u32) -> windows_core::BOOL {
    windows_core::link!("advpack.dll" "system" fn NeedReboot(dwrebootcheck : u32) -> windows_core::BOOL);
    unsafe { NeedReboot(dwrebootcheck) }
}
#[inline]
pub unsafe fn NeedRebootInit() -> u32 {
    windows_core::link!("advpack.dll" "system" fn NeedRebootInit() -> u32);
    unsafe { NeedRebootInit() }
}
#[inline]
pub unsafe fn OpenINFEngineA<P0, P1>(pszinffilename: P0, pszinstallsection: P1, dwflags: u32, phinf: *mut *mut core::ffi::c_void, pvreserved: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn OpenINFEngineA(pszinffilename : windows_core::PCSTR, pszinstallsection : windows_core::PCSTR, dwflags : u32, phinf : *mut *mut core::ffi::c_void, pvreserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OpenINFEngineA(pszinffilename.param().abi(), pszinstallsection.param().abi(), dwflags, phinf as _, pvreserved as _).ok() }
}
#[inline]
pub unsafe fn OpenINFEngineW<P0, P1>(pszinffilename: P0, pszinstallsection: P1, dwflags: u32, phinf: *mut *mut core::ffi::c_void, pvreserved: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn OpenINFEngineW(pszinffilename : windows_core::PCWSTR, pszinstallsection : windows_core::PCWSTR, dwflags : u32, phinf : *mut *mut core::ffi::c_void, pvreserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OpenINFEngineW(pszinffilename.param().abi(), pszinstallsection.param().abi(), dwflags, phinf as _, pvreserved as _).ok() }
}
#[inline]
pub unsafe fn OpenMutexA<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> super::super::Foundation::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenMutexA(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    unsafe { OpenMutexA(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[inline]
pub unsafe fn OpenSemaphoreA<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> super::super::Foundation::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenSemaphoreA(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    unsafe { OpenSemaphoreA(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[inline]
pub unsafe fn QueryAuxiliaryCounterFrequency() -> windows_core::Result<u64> {
    windows_core::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        QueryAuxiliaryCounterFrequency(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn QueryIdleProcessorCycleTime(bufferlength: *mut u32, processoridlecycletime: Option<*mut u64>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTime(bufferlength : *mut u32, processoridlecycletime : *mut u64) -> windows_core::BOOL);
    unsafe { QueryIdleProcessorCycleTime(bufferlength as _, processoridlecycletime.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn QueryIdleProcessorCycleTimeEx(group: u16, bufferlength: *mut u32, processoridlecycletime: Option<*mut u64>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTimeEx(group : u16, bufferlength : *mut u32, processoridlecycletime : *mut u64) -> windows_core::BOOL);
    unsafe { QueryIdleProcessorCycleTimeEx(group, bufferlength as _, processoridlecycletime.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn QueryInterruptTime() -> u64 {
    windows_core::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTime(lpinterrupttime : *mut u64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        QueryInterruptTime(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn QueryInterruptTimePrecise() -> u64 {
    windows_core::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTimePrecise(lpinterrupttimeprecise : *mut u64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        QueryInterruptTimePrecise(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn QueryProcessCycleTime(processhandle: super::super::Foundation::HANDLE, cycletime: *mut u64) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn QueryProcessCycleTime(processhandle : super::super::Foundation:: HANDLE, cycletime : *mut u64) -> windows_core::BOOL);
    unsafe { QueryProcessCycleTime(processhandle, cycletime as _).ok() }
}
#[inline]
pub unsafe fn QueryThreadCycleTime(threadhandle: super::super::Foundation::HANDLE, cycletime: *mut u64) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn QueryThreadCycleTime(threadhandle : super::super::Foundation:: HANDLE, cycletime : *mut u64) -> windows_core::BOOL);
    unsafe { QueryThreadCycleTime(threadhandle, cycletime as _).ok() }
}
#[inline]
pub unsafe fn QueryUnbiasedInterruptTime(unbiasedtime: *mut u64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryUnbiasedInterruptTime(unbiasedtime : *mut u64) -> windows_core::BOOL);
    unsafe { QueryUnbiasedInterruptTime(unbiasedtime as _) }
}
#[inline]
pub unsafe fn QueryUnbiasedInterruptTimePrecise() -> u64 {
    windows_core::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise : *mut u64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        QueryUnbiasedInterruptTimePrecise(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32 {
    windows_core::link!("api-ms-win-core-backgroundtask-l1-1-0.dll" "system" fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig : *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32);
    unsafe { RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig) }
}
#[inline]
pub unsafe fn RebootCheckOnInstallA<P1, P2>(hwnd: super::super::Foundation::HWND, pszinf: P1, pszsec: P2, dwreserved: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RebootCheckOnInstallA(hwnd : super::super::Foundation:: HWND, pszinf : windows_core::PCSTR, pszsec : windows_core::PCSTR, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { RebootCheckOnInstallA(hwnd, pszinf.param().abi(), pszsec.param().abi(), dwreserved).ok() }
}
#[inline]
pub unsafe fn RebootCheckOnInstallW<P1, P2>(hwnd: super::super::Foundation::HWND, pszinf: P1, pszsec: P2, dwreserved: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RebootCheckOnInstallW(hwnd : super::super::Foundation:: HWND, pszinf : windows_core::PCWSTR, pszsec : windows_core::PCWSTR, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { RebootCheckOnInstallW(hwnd, pszinf.param().abi(), pszsec.param().abi(), dwreserved).ok() }
}
#[inline]
pub unsafe fn RecordFeatureError(featureid: u32, error: *const FEATURE_ERROR) {
    windows_core::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn RecordFeatureError(featureid : u32, error : *const FEATURE_ERROR));
    unsafe { RecordFeatureError(featureid, error) }
}
#[inline]
pub unsafe fn RecordFeatureUsage<P3>(featureid: u32, kind: u32, addend: u32, originname: P3)
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn RecordFeatureUsage(featureid : u32, kind : u32, addend : u32, originname : windows_core::PCSTR));
    unsafe { RecordFeatureUsage(featureid, kind, addend, originname.param().abi()) }
}
#[inline]
pub unsafe fn RegInstallA<P1>(hmod: super::super::Foundation::HMODULE, pszsection: P1, psttable: *const STRTABLEA) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RegInstallA(hmod : super::super::Foundation:: HMODULE, pszsection : windows_core::PCSTR, psttable : *const STRTABLEA) -> windows_core::HRESULT);
    unsafe { RegInstallA(hmod, pszsection.param().abi(), psttable).ok() }
}
#[inline]
pub unsafe fn RegInstallW<P1>(hmod: super::super::Foundation::HMODULE, pszsection: P1, psttable: *const STRTABLEW) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RegInstallW(hmod : super::super::Foundation:: HMODULE, pszsection : windows_core::PCWSTR, psttable : *const STRTABLEW) -> windows_core::HRESULT);
    unsafe { RegInstallW(hmod, pszsection.param().abi(), psttable).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn RegRestoreAllA<P1>(hwnd: Option<super::super::Foundation::HWND>, psztitlestring: P1, hkbckupkey: super::Registry::HKEY) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RegRestoreAllA(hwnd : super::super::Foundation:: HWND, psztitlestring : windows_core::PCSTR, hkbckupkey : super::Registry:: HKEY) -> windows_core::HRESULT);
    unsafe { RegRestoreAllA(hwnd.unwrap_or(core::mem::zeroed()) as _, psztitlestring.param().abi(), hkbckupkey).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn RegRestoreAllW<P1>(hwnd: Option<super::super::Foundation::HWND>, psztitlestring: P1, hkbckupkey: super::Registry::HKEY) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RegRestoreAllW(hwnd : super::super::Foundation:: HWND, psztitlestring : windows_core::PCWSTR, hkbckupkey : super::Registry:: HKEY) -> windows_core::HRESULT);
    unsafe { RegRestoreAllW(hwnd.unwrap_or(core::mem::zeroed()) as _, psztitlestring.param().abi(), hkbckupkey).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn RegSaveRestoreA<P1, P3, P4, P5>(hwnd: super::super::Foundation::HWND, psztitlestring: P1, hkbckupkey: super::Registry::HKEY, pcszrootkey: P3, pcszsubkey: P4, pcszvaluename: P5, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RegSaveRestoreA(hwnd : super::super::Foundation:: HWND, psztitlestring : windows_core::PCSTR, hkbckupkey : super::Registry:: HKEY, pcszrootkey : windows_core::PCSTR, pcszsubkey : windows_core::PCSTR, pcszvaluename : windows_core::PCSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { RegSaveRestoreA(hwnd, psztitlestring.param().abi(), hkbckupkey, pcszrootkey.param().abi(), pcszsubkey.param().abi(), pcszvaluename.param().abi(), dwflags).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn RegSaveRestoreOnINFA<P1, P2, P3>(hwnd: super::super::Foundation::HWND, psztitle: P1, pszinf: P2, pszsection: P3, hhklmbackkey: super::Registry::HKEY, hhkcubackkey: super::Registry::HKEY, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RegSaveRestoreOnINFA(hwnd : super::super::Foundation:: HWND, psztitle : windows_core::PCSTR, pszinf : windows_core::PCSTR, pszsection : windows_core::PCSTR, hhklmbackkey : super::Registry:: HKEY, hhkcubackkey : super::Registry:: HKEY, dwflags : u32) -> windows_core::HRESULT);
    unsafe { RegSaveRestoreOnINFA(hwnd, psztitle.param().abi(), pszinf.param().abi(), pszsection.param().abi(), hhklmbackkey, hhkcubackkey, dwflags).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn RegSaveRestoreOnINFW<P1, P2, P3>(hwnd: super::super::Foundation::HWND, psztitle: P1, pszinf: P2, pszsection: P3, hhklmbackkey: super::Registry::HKEY, hhkcubackkey: super::Registry::HKEY, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RegSaveRestoreOnINFW(hwnd : super::super::Foundation:: HWND, psztitle : windows_core::PCWSTR, pszinf : windows_core::PCWSTR, pszsection : windows_core::PCWSTR, hhklmbackkey : super::Registry:: HKEY, hhkcubackkey : super::Registry:: HKEY, dwflags : u32) -> windows_core::HRESULT);
    unsafe { RegSaveRestoreOnINFW(hwnd, psztitle.param().abi(), pszinf.param().abi(), pszsection.param().abi(), hhklmbackkey, hhkcubackkey, dwflags).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn RegSaveRestoreW<P1, P3, P4, P5>(hwnd: super::super::Foundation::HWND, psztitlestring: P1, hkbckupkey: super::Registry::HKEY, pcszrootkey: P3, pcszsubkey: P4, pcszvaluename: P5, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RegSaveRestoreW(hwnd : super::super::Foundation:: HWND, psztitlestring : windows_core::PCWSTR, hkbckupkey : super::Registry:: HKEY, pcszrootkey : windows_core::PCWSTR, pcszsubkey : windows_core::PCWSTR, pcszvaluename : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { RegSaveRestoreW(hwnd, psztitlestring.param().abi(), hkbckupkey, pcszrootkey.param().abi(), pcszsubkey.param().abi(), pcszvaluename.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn ReplacePartitionUnit<P0, P1>(targetpartition: P0, sparepartition: P1, flags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ReplacePartitionUnit(targetpartition : windows_core::PCWSTR, sparepartition : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { ReplacePartitionUnit(targetpartition.param().abi(), sparepartition.param().abi(), flags) }
}
#[inline]
pub unsafe fn RequestDeviceWakeup(hdevice: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RequestDeviceWakeup(hdevice : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { RequestDeviceWakeup(hdevice) }
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlAnsiStringToUnicodeString(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: *mut super::Kernel::STRING, allocatedestinationstring: bool) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAnsiStringToUnicodeString(destinationstring : *mut super::super::Foundation:: UNICODE_STRING, sourcestring : *mut super::Kernel:: STRING, allocatedestinationstring : bool) -> super::super::Foundation:: NTSTATUS);
    unsafe { RtlAnsiStringToUnicodeString(destinationstring as _, sourcestring as _, allocatedestinationstring) }
}
#[inline]
pub unsafe fn RtlCharToInteger(string: *mut i8, base: u32, value: *mut u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCharToInteger(string : *mut i8, base : u32, value : *mut u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { RtlCharToInteger(string as _, base, value as _) }
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlFreeAnsiString(ansistring: *mut super::Kernel::STRING) {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeAnsiString(ansistring : *mut super::Kernel:: STRING));
    unsafe { RtlFreeAnsiString(ansistring as _) }
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlFreeOemString(oemstring: *mut super::Kernel::STRING) {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeOemString(oemstring : *mut super::Kernel:: STRING));
    unsafe { RtlFreeOemString(oemstring as _) }
}
#[inline]
pub unsafe fn RtlFreeUnicodeString(unicodestring: *mut super::super::Foundation::UNICODE_STRING) {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeUnicodeString(unicodestring : *mut super::super::Foundation:: UNICODE_STRING));
    unsafe { RtlFreeUnicodeString(unicodestring as _) }
}
#[inline]
pub unsafe fn RtlGetReturnAddressHijackTarget() -> usize {
    windows_core::link!("ntdll.dll" "system" fn RtlGetReturnAddressHijackTarget() -> usize);
    unsafe { RtlGetReturnAddressHijackTarget() }
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlInitAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) {
    windows_core::link!("ntdll.dll" "system" fn RtlInitAnsiString(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut i8));
    unsafe { RtlInitAnsiString(destinationstring as _, sourcestring as _) }
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlInitAnsiStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlInitAnsiStringEx(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut i8) -> super::super::Foundation:: NTSTATUS);
    unsafe { RtlInitAnsiStringEx(destinationstring as _, sourcestring as _) }
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlInitString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) {
    windows_core::link!("ntdll.dll" "system" fn RtlInitString(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut i8));
    unsafe { RtlInitString(destinationstring as _, sourcestring as _) }
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlInitStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlInitStringEx(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut i8) -> super::super::Foundation:: NTSTATUS);
    unsafe { RtlInitStringEx(destinationstring as _, sourcestring as _) }
}
#[inline]
pub unsafe fn RtlInitUnicodeString<P1>(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: P1)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlInitUnicodeString(destinationstring : *mut super::super::Foundation:: UNICODE_STRING, sourcestring : windows_core::PCWSTR));
    unsafe { RtlInitUnicodeString(destinationstring as _, sourcestring.param().abi()) }
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlIsNameLegalDOS8Dot3(name: *mut super::super::Foundation::UNICODE_STRING, oemname: *mut super::Kernel::STRING, namecontainsspaces: *mut bool) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlIsNameLegalDOS8Dot3(name : *mut super::super::Foundation:: UNICODE_STRING, oemname : *mut super::Kernel:: STRING, namecontainsspaces : *mut bool) -> bool);
    unsafe { RtlIsNameLegalDOS8Dot3(name as _, oemname as _, namecontainsspaces as _) }
}
#[inline]
pub unsafe fn RtlLocalTimeToSystemTime(localtime: *mut i64, systemtime: *mut i64) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlLocalTimeToSystemTime(localtime : *mut i64, systemtime : *mut i64) -> super::super::Foundation:: NTSTATUS);
    unsafe { RtlLocalTimeToSystemTime(localtime as _, systemtime as _) }
}
#[inline]
pub unsafe fn RtlRaiseCustomSystemEventTrigger(triggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlRaiseCustomSystemEventTrigger(triggerconfig : *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32);
    unsafe { RtlRaiseCustomSystemEventTrigger(triggerconfig) }
}
#[inline]
pub unsafe fn RtlTimeToSecondsSince1970(time: *mut i64, elapsedseconds: *mut u32) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlTimeToSecondsSince1970(time : *mut i64, elapsedseconds : *mut u32) -> bool);
    unsafe { RtlTimeToSecondsSince1970(time as _, elapsedseconds as _) }
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlUnicodeStringToAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: bool) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeStringToAnsiString(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut super::super::Foundation:: UNICODE_STRING, allocatedestinationstring : bool) -> super::super::Foundation:: NTSTATUS);
    unsafe { RtlUnicodeStringToAnsiString(destinationstring as _, sourcestring as _, allocatedestinationstring) }
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlUnicodeStringToOemString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: bool) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeStringToOemString(destinationstring : *mut super::Kernel:: STRING, sourcestring : *mut super::super::Foundation:: UNICODE_STRING, allocatedestinationstring : bool) -> super::super::Foundation:: NTSTATUS);
    unsafe { RtlUnicodeStringToOemString(destinationstring as _, sourcestring as _, allocatedestinationstring) }
}
#[inline]
pub unsafe fn RtlUnicodeToMultiByteSize<P1>(bytesinmultibytestring: *mut u32, unicodestring: P1, bytesinunicodestring: u32) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToMultiByteSize(bytesinmultibytestring : *mut u32, unicodestring : windows_core::PCWSTR, bytesinunicodestring : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { RtlUnicodeToMultiByteSize(bytesinmultibytestring as _, unicodestring.param().abi(), bytesinunicodestring) }
}
#[inline]
pub unsafe fn RtlUniform(seed: *mut u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlUniform(seed : *mut u32) -> u32);
    unsafe { RtlUniform(seed as _) }
}
#[inline]
pub unsafe fn RunSetupCommandA<P1, P2, P3, P4>(hwnd: super::super::Foundation::HWND, szcmdname: P1, szinfsection: P2, szdir: P3, lpsztitle: P4, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RunSetupCommandA(hwnd : super::super::Foundation:: HWND, szcmdname : windows_core::PCSTR, szinfsection : windows_core::PCSTR, szdir : windows_core::PCSTR, lpsztitle : windows_core::PCSTR, phexe : *mut super::super::Foundation:: HANDLE, dwflags : u32, pvreserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RunSetupCommandA(hwnd, szcmdname.param().abi(), szinfsection.param().abi(), szdir.param().abi(), lpsztitle.param().abi(), phexe as _, dwflags, pvreserved as _).ok() }
}
#[inline]
pub unsafe fn RunSetupCommandW<P1, P2, P3, P4>(hwnd: super::super::Foundation::HWND, szcmdname: P1, szinfsection: P2, szdir: P3, lpsztitle: P4, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn RunSetupCommandW(hwnd : super::super::Foundation:: HWND, szcmdname : windows_core::PCWSTR, szinfsection : windows_core::PCWSTR, szdir : windows_core::PCWSTR, lpsztitle : windows_core::PCWSTR, phexe : *mut super::super::Foundation:: HANDLE, dwflags : u32, pvreserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RunSetupCommandW(hwnd, szcmdname.param().abi(), szinfsection.param().abi(), szdir.param().abi(), lpsztitle.param().abi(), phexe as _, dwflags, pvreserved as _).ok() }
}
#[inline]
pub unsafe fn SendIMEMessageExA(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT {
    windows_core::link!("user32.dll" "system" fn SendIMEMessageExA(param0 : super::super::Foundation:: HWND, param1 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    unsafe { SendIMEMessageExA(param0, param1) }
}
#[inline]
pub unsafe fn SendIMEMessageExW(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT {
    windows_core::link!("user32.dll" "system" fn SendIMEMessageExW(param0 : super::super::Foundation:: HWND, param1 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    unsafe { SendIMEMessageExW(param0, param1) }
}
#[inline]
pub unsafe fn SetEnvironmentStringsA<P0>(newenvironment: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetEnvironmentStringsA(newenvironment : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetEnvironmentStringsA(newenvironment.param().abi()) }
}
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableA<P0, P1>(lpname: P0, lpguid: P1, pvalue: Option<*const core::ffi::c_void>, nsize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableA(lpname : windows_core::PCSTR, lpguid : windows_core::PCSTR, pvalue : *const core::ffi::c_void, nsize : u32) -> windows_core::BOOL);
    unsafe { SetFirmwareEnvironmentVariableA(lpname.param().abi(), lpguid.param().abi(), pvalue.unwrap_or(core::mem::zeroed()) as _, nsize).ok() }
}
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableExA<P0, P1>(lpname: P0, lpguid: P1, pvalue: Option<*const core::ffi::c_void>, nsize: u32, dwattributes: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableExA(lpname : windows_core::PCSTR, lpguid : windows_core::PCSTR, pvalue : *const core::ffi::c_void, nsize : u32, dwattributes : u32) -> windows_core::BOOL);
    unsafe { SetFirmwareEnvironmentVariableExA(lpname.param().abi(), lpguid.param().abi(), pvalue.unwrap_or(core::mem::zeroed()) as _, nsize, dwattributes).ok() }
}
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableExW<P0, P1>(lpname: P0, lpguid: P1, pvalue: Option<*const core::ffi::c_void>, nsize: u32, dwattributes: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableExW(lpname : windows_core::PCWSTR, lpguid : windows_core::PCWSTR, pvalue : *const core::ffi::c_void, nsize : u32, dwattributes : u32) -> windows_core::BOOL);
    unsafe { SetFirmwareEnvironmentVariableExW(lpname.param().abi(), lpguid.param().abi(), pvalue.unwrap_or(core::mem::zeroed()) as _, nsize, dwattributes).ok() }
}
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableW<P0, P1>(lpname: P0, lpguid: P1, pvalue: Option<*const core::ffi::c_void>, nsize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableW(lpname : windows_core::PCWSTR, lpguid : windows_core::PCWSTR, pvalue : *const core::ffi::c_void, nsize : u32) -> windows_core::BOOL);
    unsafe { SetFirmwareEnvironmentVariableW(lpname.param().abi(), lpguid.param().abi(), pvalue.unwrap_or(core::mem::zeroed()) as _, nsize).ok() }
}
#[inline]
pub unsafe fn SetHandleCount(unumber: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SetHandleCount(unumber : u32) -> u32);
    unsafe { SetHandleCount(unumber) }
}
#[inline]
pub unsafe fn SetMessageWaitingIndicator(hmsgindicator: super::super::Foundation::HANDLE, ulmsgcount: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetMessageWaitingIndicator(hmsgindicator : super::super::Foundation:: HANDLE, ulmsgcount : u32) -> windows_core::BOOL);
    unsafe { SetMessageWaitingIndicator(hmsgindicator, ulmsgcount) }
}
#[inline]
pub unsafe fn SetPerUserSecValuesA(pperuser: *mut PERUSERSECTIONA) -> windows_core::Result<()> {
    windows_core::link!("advpack.dll" "system" fn SetPerUserSecValuesA(pperuser : *mut PERUSERSECTIONA) -> windows_core::HRESULT);
    unsafe { SetPerUserSecValuesA(pperuser as _).ok() }
}
#[inline]
pub unsafe fn SetPerUserSecValuesW(pperuser: *mut PERUSERSECTIONW) -> windows_core::Result<()> {
    windows_core::link!("advpack.dll" "system" fn SetPerUserSecValuesW(pperuser : *mut PERUSERSECTIONW) -> windows_core::HRESULT);
    unsafe { SetPerUserSecValuesW(pperuser as _).ok() }
}
#[inline]
pub unsafe fn SubscribeFeatureStateChangeNotification(subscription: *mut FEATURE_STATE_CHANGE_SUBSCRIPTION, callback: PFEATURE_STATE_CHANGE_CALLBACK, context: Option<*const core::ffi::c_void>) {
    windows_core::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn SubscribeFeatureStateChangeNotification(subscription : *mut FEATURE_STATE_CHANGE_SUBSCRIPTION, callback : PFEATURE_STATE_CHANGE_CALLBACK, context : *const core::ffi::c_void));
    unsafe { SubscribeFeatureStateChangeNotification(subscription as _, callback, context.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TranslateInfStringA<P0, P1, P2, P3>(pszinffilename: P0, pszinstallsection: P1, psztranslatesection: P2, psztranslatekey: P3, pszbuffer: Option<&mut [u8]>, pdwrequiredsize: *mut u32, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn TranslateInfStringA(pszinffilename : windows_core::PCSTR, pszinstallsection : windows_core::PCSTR, psztranslatesection : windows_core::PCSTR, psztranslatekey : windows_core::PCSTR, pszbuffer : windows_core::PSTR, cchbuffer : u32, pdwrequiredsize : *mut u32, pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { TranslateInfStringA(pszinffilename.param().abi(), pszinstallsection.param().abi(), psztranslatesection.param().abi(), psztranslatekey.param().abi(), core::mem::transmute(pszbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pdwrequiredsize as _, pvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn TranslateInfStringExA<P1, P2, P3>(hinf: *mut core::ffi::c_void, pszinffilename: P1, psztranslatesection: P2, psztranslatekey: P3, pszbuffer: &mut [u8], pdwrequiredsize: *mut u32, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn TranslateInfStringExA(hinf : *mut core::ffi::c_void, pszinffilename : windows_core::PCSTR, psztranslatesection : windows_core::PCSTR, psztranslatekey : windows_core::PCSTR, pszbuffer : windows_core::PSTR, dwbuffersize : u32, pdwrequiredsize : *mut u32, pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { TranslateInfStringExA(hinf as _, pszinffilename.param().abi(), psztranslatesection.param().abi(), psztranslatekey.param().abi(), core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap(), pdwrequiredsize as _, pvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn TranslateInfStringExW<P1, P2, P3>(hinf: *mut core::ffi::c_void, pszinffilename: P1, psztranslatesection: P2, psztranslatekey: P3, pszbuffer: &mut [u16], pdwrequiredsize: *mut u32, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn TranslateInfStringExW(hinf : *mut core::ffi::c_void, pszinffilename : windows_core::PCWSTR, psztranslatesection : windows_core::PCWSTR, psztranslatekey : windows_core::PCWSTR, pszbuffer : windows_core::PWSTR, dwbuffersize : u32, pdwrequiredsize : *mut u32, pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { TranslateInfStringExW(hinf as _, pszinffilename.param().abi(), psztranslatesection.param().abi(), psztranslatekey.param().abi(), core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap(), pdwrequiredsize as _, pvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn TranslateInfStringW<P0, P1, P2, P3>(pszinffilename: P0, pszinstallsection: P1, psztranslatesection: P2, psztranslatekey: P3, pszbuffer: Option<&mut [u16]>, pdwrequiredsize: *mut u32, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn TranslateInfStringW(pszinffilename : windows_core::PCWSTR, pszinstallsection : windows_core::PCWSTR, psztranslatesection : windows_core::PCWSTR, psztranslatekey : windows_core::PCWSTR, pszbuffer : windows_core::PWSTR, cchbuffer : u32, pdwrequiredsize : *mut u32, pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { TranslateInfStringW(pszinffilename.param().abi(), pszinstallsection.param().abi(), psztranslatesection.param().abi(), psztranslatekey.param().abi(), core::mem::transmute(pszbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pdwrequiredsize as _, pvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn UnsubscribeFeatureStateChangeNotification(subscription: FEATURE_STATE_CHANGE_SUBSCRIPTION) {
    windows_core::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn UnsubscribeFeatureStateChangeNotification(subscription : FEATURE_STATE_CHANGE_SUBSCRIPTION));
    unsafe { UnsubscribeFeatureStateChangeNotification(subscription) }
}
#[inline]
pub unsafe fn UserInstStubWrapperA<P2>(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: P2, nshow: i32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn UserInstStubWrapperA(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_core::PCSTR, nshow : i32) -> windows_core::HRESULT);
    unsafe { UserInstStubWrapperA(hwnd, hinstance, pszparms.param().abi(), nshow).ok() }
}
#[inline]
pub unsafe fn UserInstStubWrapperW<P2>(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: P2, nshow: i32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn UserInstStubWrapperW(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_core::PCWSTR, nshow : i32) -> windows_core::HRESULT);
    unsafe { UserInstStubWrapperW(hwnd, hinstance, pszparms.param().abi(), nshow).ok() }
}
#[inline]
pub unsafe fn UserUnInstStubWrapperA<P2>(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: P2, nshow: i32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advpack.dll" "system" fn UserUnInstStubWrapperA(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_core::PCSTR, nshow : i32) -> windows_core::HRESULT);
    unsafe { UserUnInstStubWrapperA(hwnd, hinstance, pszparms.param().abi(), nshow).ok() }
}
#[inline]
pub unsafe fn UserUnInstStubWrapperW<P2>(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: P2, nshow: i32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advpack.dll" "system" fn UserUnInstStubWrapperW(hwnd : super::super::Foundation:: HWND, hinstance : super::super::Foundation:: HINSTANCE, pszparms : windows_core::PCWSTR, nshow : i32) -> windows_core::HRESULT);
    unsafe { UserUnInstStubWrapperW(hwnd, hinstance, pszparms.param().abi(), nshow).ok() }
}
#[inline]
pub unsafe fn WINNLSEnableIME(param0: super::super::Foundation::HWND, param1: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn WINNLSEnableIME(param0 : super::super::Foundation:: HWND, param1 : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WINNLSEnableIME(param0, param1.into()) }
}
#[inline]
pub unsafe fn WINNLSGetEnableStatus(param0: super::super::Foundation::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn WINNLSGetEnableStatus(param0 : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { WINNLSGetEnableStatus(param0) }
}
#[inline]
pub unsafe fn WINNLSGetIMEHotkey(param0: super::super::Foundation::HWND) -> u32 {
    windows_core::link!("user32.dll" "system" fn WINNLSGetIMEHotkey(param0 : super::super::Foundation:: HWND) -> u32);
    unsafe { WINNLSGetIMEHotkey(param0) }
}
#[inline]
pub unsafe fn WinWatchClose(hww: HWINWATCH) {
    windows_core::link!("dciman32.dll" "system" fn WinWatchClose(hww : HWINWATCH));
    unsafe { WinWatchClose(hww) }
}
#[inline]
pub unsafe fn WinWatchDidStatusChange(hww: HWINWATCH) -> windows_core::BOOL {
    windows_core::link!("dciman32.dll" "system" fn WinWatchDidStatusChange(hww : HWINWATCH) -> windows_core::BOOL);
    unsafe { WinWatchDidStatusChange(hww) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn WinWatchGetClipList(hww: HWINWATCH, prc: *mut super::super::Foundation::RECT, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32 {
    windows_core::link!("dciman32.dll" "system" fn WinWatchGetClipList(hww : HWINWATCH, prc : *mut super::super::Foundation:: RECT, size : u32, prd : *mut super::super::Graphics::Gdi:: RGNDATA) -> u32);
    unsafe { WinWatchGetClipList(hww, prc as _, size, prd as _) }
}
#[inline]
pub unsafe fn WinWatchNotify(hww: HWINWATCH, notifycallback: WINWATCHNOTIFYPROC, notifyparam: super::super::Foundation::LPARAM) -> windows_core::BOOL {
    windows_core::link!("dciman32.dll" "system" fn WinWatchNotify(hww : HWINWATCH, notifycallback : WINWATCHNOTIFYPROC, notifyparam : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { WinWatchNotify(hww, notifycallback, notifyparam) }
}
#[inline]
pub unsafe fn WinWatchOpen(hwnd: super::super::Foundation::HWND) -> HWINWATCH {
    windows_core::link!("dciman32.dll" "system" fn WinWatchOpen(hwnd : super::super::Foundation:: HWND) -> HWINWATCH);
    unsafe { WinWatchOpen(hwnd) }
}
#[inline]
pub unsafe fn WldpCanExecuteBuffer<P4>(host: *const windows_core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, buffer: &[u8], auditinfo: P4) -> windows_core::Result<WLDP_EXECUTION_POLICY>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wldp.dll" "system" fn WldpCanExecuteBuffer(host : *const windows_core::GUID, options : WLDP_EXECUTION_EVALUATION_OPTIONS, buffer : *const u8, buffersize : u32, auditinfo : windows_core::PCWSTR, result : *mut WLDP_EXECUTION_POLICY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WldpCanExecuteBuffer(host, options, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), auditinfo.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WldpCanExecuteFile<P3>(host: *const windows_core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, filehandle: super::super::Foundation::HANDLE, auditinfo: P3) -> windows_core::Result<WLDP_EXECUTION_POLICY>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wldp.dll" "system" fn WldpCanExecuteFile(host : *const windows_core::GUID, options : WLDP_EXECUTION_EVALUATION_OPTIONS, filehandle : super::super::Foundation:: HANDLE, auditinfo : windows_core::PCWSTR, result : *mut WLDP_EXECUTION_POLICY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WldpCanExecuteFile(host, options, filehandle, auditinfo.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WldpCanExecuteStream<P2, P3>(host: *const windows_core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, stream: P2, auditinfo: P3) -> windows_core::Result<WLDP_EXECUTION_POLICY>
where
    P2: windows_core::Param<super::Com::IStream>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wldp.dll" "system" fn WldpCanExecuteStream(host : *const windows_core::GUID, options : WLDP_EXECUTION_EVALUATION_OPTIONS, stream : * mut core::ffi::c_void, auditinfo : windows_core::PCWSTR, result : *mut WLDP_EXECUTION_POLICY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WldpCanExecuteStream(host, options, stream.param().abi(), auditinfo.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WldpGetLockdownPolicy(hostinformation: Option<*const WLDP_HOST_INFORMATION>, lockdownstate: *mut u32, lockdownflags: u32) -> windows_core::Result<()> {
    windows_core::link!("wldp.dll" "system" fn WldpGetLockdownPolicy(hostinformation : *const WLDP_HOST_INFORMATION, lockdownstate : *mut u32, lockdownflags : u32) -> windows_core::HRESULT);
    unsafe { WldpGetLockdownPolicy(hostinformation.unwrap_or(core::mem::zeroed()) as _, lockdownstate as _, lockdownflags).ok() }
}
#[inline]
pub unsafe fn WldpIsClassInApprovedList(classid: *const windows_core::GUID, hostinformation: *const WLDP_HOST_INFORMATION, isapproved: *mut windows_core::BOOL, optionalflags: u32) -> windows_core::Result<()> {
    windows_core::link!("wldp.dll" "system" fn WldpIsClassInApprovedList(classid : *const windows_core::GUID, hostinformation : *const WLDP_HOST_INFORMATION, isapproved : *mut windows_core::BOOL, optionalflags : u32) -> windows_core::HRESULT);
    unsafe { WldpIsClassInApprovedList(classid, hostinformation, isapproved as _, optionalflags).ok() }
}
#[inline]
pub unsafe fn WldpIsDynamicCodePolicyEnabled() -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("wldp.dll" "system" fn WldpIsDynamicCodePolicyEnabled(isenabled : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WldpIsDynamicCodePolicyEnabled(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WldpQueryDeviceSecurityInformation(information: Option<&mut [WLDP_DEVICE_SECURITY_INFORMATION]>, returnlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wldp.dll" "system" fn WldpQueryDeviceSecurityInformation(information : *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength : u32, returnlength : *mut u32) -> windows_core::HRESULT);
    unsafe { WldpQueryDeviceSecurityInformation(core::mem::transmute(information.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), information.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), returnlength as _).ok() }
}
#[inline]
pub unsafe fn WldpQueryDynamicCodeTrust(filehandle: Option<super::super::Foundation::HANDLE>, baseimage: Option<*const core::ffi::c_void>, imagesize: u32) -> windows_core::Result<()> {
    windows_core::link!("wldp.dll" "system" fn WldpQueryDynamicCodeTrust(filehandle : super::super::Foundation:: HANDLE, baseimage : *const core::ffi::c_void, imagesize : u32) -> windows_core::HRESULT);
    unsafe { WldpQueryDynamicCodeTrust(filehandle.unwrap_or(core::mem::zeroed()) as _, baseimage.unwrap_or(core::mem::zeroed()) as _, imagesize).ok() }
}
#[inline]
pub unsafe fn WldpSetDynamicCodeTrust(filehandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wldp.dll" "system" fn WldpSetDynamicCodeTrust(filehandle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { WldpSetDynamicCodeTrust(filehandle).ok() }
}
#[inline]
pub unsafe fn WritePrivateProfileSectionA<P0, P1, P2>(lpappname: P0, lpstring: P1, lpfilename: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileSectionA(lpappname : windows_core::PCSTR, lpstring : windows_core::PCSTR, lpfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileSectionA(lpappname.param().abi(), lpstring.param().abi(), lpfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn WritePrivateProfileSectionW<P0, P1, P2>(lpappname: P0, lpstring: P1, lpfilename: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileSectionW(lpappname : windows_core::PCWSTR, lpstring : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileSectionW(lpappname.param().abi(), lpstring.param().abi(), lpfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn WritePrivateProfileStringA<P0, P1, P2, P3>(lpappname: P0, lpkeyname: P1, lpstring: P2, lpfilename: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileStringA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, lpstring : windows_core::PCSTR, lpfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileStringA(lpappname.param().abi(), lpkeyname.param().abi(), lpstring.param().abi(), lpfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn WritePrivateProfileStringW<P0, P1, P2, P3>(lpappname: P0, lpkeyname: P1, lpstring: P2, lpfilename: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileStringW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, lpstring : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileStringW(lpappname.param().abi(), lpkeyname.param().abi(), lpstring.param().abi(), lpfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn WritePrivateProfileStructA<P0, P1, P4>(lpszsection: P0, lpszkey: P1, lpstruct: Option<*const core::ffi::c_void>, usizestruct: u32, szfile: P4) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileStructA(lpszsection : windows_core::PCSTR, lpszkey : windows_core::PCSTR, lpstruct : *const core::ffi::c_void, usizestruct : u32, szfile : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileStructA(lpszsection.param().abi(), lpszkey.param().abi(), lpstruct.unwrap_or(core::mem::zeroed()) as _, usizestruct, szfile.param().abi()).ok() }
}
#[inline]
pub unsafe fn WritePrivateProfileStructW<P0, P1, P4>(lpszsection: P0, lpszkey: P1, lpstruct: Option<*const core::ffi::c_void>, usizestruct: u32, szfile: P4) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileStructW(lpszsection : windows_core::PCWSTR, lpszkey : windows_core::PCWSTR, lpstruct : *const core::ffi::c_void, usizestruct : u32, szfile : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileStructW(lpszsection.param().abi(), lpszkey.param().abi(), lpstruct.unwrap_or(core::mem::zeroed()) as _, usizestruct, szfile.param().abi()).ok() }
}
#[inline]
pub unsafe fn WriteProfileSectionA<P0, P1>(lpappname: P0, lpstring: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WriteProfileSectionA(lpappname : windows_core::PCSTR, lpstring : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { WriteProfileSectionA(lpappname.param().abi(), lpstring.param().abi()).ok() }
}
#[inline]
pub unsafe fn WriteProfileSectionW<P0, P1>(lpappname: P0, lpstring: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WriteProfileSectionW(lpappname : windows_core::PCWSTR, lpstring : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WriteProfileSectionW(lpappname.param().abi(), lpstring.param().abi()).ok() }
}
#[inline]
pub unsafe fn WriteProfileStringA<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpstring: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WriteProfileStringA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, lpstring : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { WriteProfileStringA(lpappname.param().abi(), lpkeyname.param().abi(), lpstring.param().abi()).ok() }
}
#[inline]
pub unsafe fn WriteProfileStringW<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpstring: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WriteProfileStringW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, lpstring : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WriteProfileStringW(lpappname.param().abi(), lpkeyname.param().abi(), lpstring.param().abi()).ok() }
}
#[inline]
pub unsafe fn _hread(hfile: i32, lpbuffer: *mut core::ffi::c_void, lbytes: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn _hread(hfile : i32, lpbuffer : *mut core::ffi::c_void, lbytes : i32) -> i32);
    unsafe { _hread(hfile, lpbuffer as _, lbytes) }
}
#[inline]
pub unsafe fn _hwrite(hfile: i32, lpbuffer: &[u8]) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn _hwrite(hfile : i32, lpbuffer : windows_core::PCSTR, lbytes : i32) -> i32);
    unsafe { _hwrite(hfile, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn _lclose(hfile: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn _lclose(hfile : i32) -> i32);
    unsafe { _lclose(hfile) }
}
#[inline]
pub unsafe fn _lcreat<P0>(lppathname: P0, iattribute: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn _lcreat(lppathname : windows_core::PCSTR, iattribute : i32) -> i32);
    unsafe { _lcreat(lppathname.param().abi(), iattribute) }
}
#[inline]
pub unsafe fn _llseek(hfile: i32, loffset: i32, iorigin: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn _llseek(hfile : i32, loffset : i32, iorigin : i32) -> i32);
    unsafe { _llseek(hfile, loffset, iorigin) }
}
#[inline]
pub unsafe fn _lopen<P0>(lppathname: P0, ireadwrite: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn _lopen(lppathname : windows_core::PCSTR, ireadwrite : i32) -> i32);
    unsafe { _lopen(lppathname.param().abi(), ireadwrite) }
}
#[inline]
pub unsafe fn _lread(hfile: i32, lpbuffer: *mut core::ffi::c_void, ubytes: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn _lread(hfile : i32, lpbuffer : *mut core::ffi::c_void, ubytes : u32) -> u32);
    unsafe { _lread(hfile, lpbuffer as _, ubytes) }
}
#[inline]
pub unsafe fn _lwrite(hfile: i32, lpbuffer: &[u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn _lwrite(hfile : i32, lpbuffer : windows_core::PCSTR, ubytes : u32) -> u32);
    unsafe { _lwrite(hfile, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap()) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrcmpW(string1: *const u16, string2: *const u16) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn uaw_lstrcmpW(string1 : *const u16, string2 : *const u16) -> i32);
    unsafe { uaw_lstrcmpW(string1, string2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrcmpiW(string1: *const u16, string2: *const u16) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn uaw_lstrcmpiW(string1 : *const u16, string2 : *const u16) -> i32);
    unsafe { uaw_lstrcmpiW(string1, string2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrlenW(string: *const u16) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn uaw_lstrlenW(string : *const u16) -> i32);
    unsafe { uaw_lstrlenW(string) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcschr(string: *const u16, character: u16) -> *mut u16 {
    windows_core::link!("kernel32.dll" "system" fn uaw_wcschr(string : *const u16, character : u16) -> *mut u16);
    unsafe { uaw_wcschr(string, character) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcscpy(destination: *mut u16, source: *const u16) -> *mut u16 {
    windows_core::link!("kernel32.dll" "system" fn uaw_wcscpy(destination : *mut u16, source : *const u16) -> *mut u16);
    unsafe { uaw_wcscpy(destination as _, source) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcsicmp(string1: *const u16, string2: *const u16) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn uaw_wcsicmp(string1 : *const u16, string2 : *const u16) -> i32);
    unsafe { uaw_wcsicmp(string1, string2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcslen(string: *const u16) -> usize {
    windows_core::link!("kernel32.dll" "system" fn uaw_wcslen(string : *const u16) -> usize);
    unsafe { uaw_wcslen(string) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcsrchr(string: *const u16, character: u16) -> *mut u16 {
    windows_core::link!("kernel32.dll" "system" fn uaw_wcsrchr(string : *const u16, character : u16) -> *mut u16);
    unsafe { uaw_wcsrchr(string, character) }
}
pub const AADBE_ADD_ENTRY: u32 = 1u32;
pub const AADBE_DEL_ENTRY: u32 = 2u32;
pub const ACTCTX_FLAG_APPLICATION_NAME_VALID: u32 = 32u32;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 4u32;
pub const ACTCTX_FLAG_HMODULE_VALID: u32 = 128u32;
pub const ACTCTX_FLAG_LANGID_VALID: u32 = 2u32;
pub const ACTCTX_FLAG_PROCESSOR_ARCHITECTURE_VALID: u32 = 1u32;
pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 8u32;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 16u32;
pub const ACTCTX_FLAG_SOURCE_IS_ASSEMBLYREF: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTCTX_SECTION_KEYED_DATA_2600 {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
}
impl Default for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    pub lpInformation: *mut core::ffi::c_void,
    pub lpSectionBase: *mut core::ffi::c_void,
    pub ulSectionLength: u32,
    pub lpSectionGlobalDataBase: *mut core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
}
impl Default for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTIVATION_CONTEXT_BASIC_INFORMATION {
    pub hActCtx: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
}
pub const ACTIVATION_CONTEXT_BASIC_INFORMATION_DEFINED: u32 = 1u32;
pub const AC_LINE_BACKUP_POWER: u32 = 2u32;
pub const AC_LINE_OFFLINE: u32 = 0u32;
pub const AC_LINE_ONLINE: u32 = 1u32;
pub const AC_LINE_UNKNOWN: u32 = 255u32;
pub const ADN_DEL_IF_EMPTY: u32 = 1u32;
pub const ADN_DEL_UNC_PATHS: u32 = 8u32;
pub const ADN_DONT_DEL_DIR: u32 = 4u32;
pub const ADN_DONT_DEL_SUBDIRS: u32 = 2u32;
pub const AFSR_BACKNEW: u32 = 2u32;
pub const AFSR_EXTRAINCREFCNT: u32 = 2048u32;
pub const AFSR_NODELETENEW: u32 = 4u32;
pub const AFSR_NOMESSAGES: u32 = 8u32;
pub const AFSR_NOPROGRESS: u32 = 16u32;
pub const AFSR_RESTORE: u32 = 1u32;
pub const AFSR_UPDREFCNT: u32 = 512u32;
pub const AFSR_USEREFCNT: u32 = 1024u32;
pub const AIF_FORCE_FILE_IN_USE: u32 = 8u32;
pub const AIF_NOLANGUAGECHECK: u32 = 268435456u32;
pub const AIF_NOOVERWRITE: u32 = 16u32;
pub const AIF_NOSKIP: u32 = 2u32;
pub const AIF_NOVERSIONCHECK: u32 = 4u32;
pub const AIF_NO_VERSION_DIALOG: u32 = 32u32;
pub const AIF_QUIET: u32 = 536870912u32;
pub const AIF_REPLACEONLY: u32 = 1024u32;
pub const AIF_WARNIFSKIP: u32 = 1u32;
pub const ALINF_BKINSTALL: u32 = 32u32;
pub const ALINF_CHECKBKDATA: u32 = 128u32;
pub const ALINF_DELAYREGISTEROCX: u32 = 512u32;
pub const ALINF_NGCONV: u32 = 8u32;
pub const ALINF_QUIET: u32 = 4u32;
pub const ALINF_ROLLBACK: u32 = 64u32;
pub const ALINF_ROLLBKDOALL: u32 = 256u32;
pub const ALINF_UPDHLPDLLS: u32 = 16u32;
pub type APPLICATION_RECOVERY_CALLBACK = Option<unsafe extern "system" fn(pvparameter: *mut core::ffi::c_void) -> u32>;
pub const ARSR_NOMESSAGES: u32 = 8u32;
pub const ARSR_REGSECTION: u32 = 128u32;
pub const ARSR_REMOVREGBKDATA: u32 = 4096u32;
pub const ARSR_RESTORE: u32 = 1u32;
pub const ATOM_FLAG_GLOBAL: u32 = 2u32;
pub const AT_ARP: u32 = 640u32;
pub const AT_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(640u32);
pub const AT_NULL: u32 = 642u32;
pub const BACKUP_GHOSTED_FILE_EXTENTS: u32 = 11u32;
pub const BACKUP_INVALID: u32 = 0u32;
pub const BASE_SEARCH_PATH_DISABLE_SAFE_SEARCHMODE: u32 = 65536u32;
pub const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: u32 = 1u32;
pub const BASE_SEARCH_PATH_PERMANENT: u32 = 32768u32;
pub const BATTERY_FLAG_CHARGING: u32 = 8u32;
pub const BATTERY_FLAG_CRITICAL: u32 = 4u32;
pub const BATTERY_FLAG_HIGH: u32 = 1u32;
pub const BATTERY_FLAG_LOW: u32 = 2u32;
pub const BATTERY_FLAG_NO_BATTERY: u32 = 128u32;
pub const BATTERY_FLAG_UNKNOWN: u32 = 255u32;
pub const BATTERY_LIFE_UNKNOWN: u32 = 4294967295u32;
pub const BATTERY_PERCENTAGE_UNKNOWN: u32 = 255u32;
pub const BAUD_075: u32 = 1u32;
pub const BAUD_110: u32 = 2u32;
pub const BAUD_115200: u32 = 131072u32;
pub const BAUD_1200: u32 = 64u32;
pub const BAUD_128K: u32 = 65536u32;
pub const BAUD_134_5: u32 = 4u32;
pub const BAUD_14400: u32 = 4096u32;
pub const BAUD_150: u32 = 8u32;
pub const BAUD_1800: u32 = 128u32;
pub const BAUD_19200: u32 = 8192u32;
pub const BAUD_2400: u32 = 256u32;
pub const BAUD_300: u32 = 16u32;
pub const BAUD_38400: u32 = 16384u32;
pub const BAUD_4800: u32 = 512u32;
pub const BAUD_56K: u32 = 32768u32;
pub const BAUD_57600: u32 = 262144u32;
pub const BAUD_600: u32 = 32u32;
pub const BAUD_7200: u32 = 1024u32;
pub const BAUD_9600: u32 = 2048u32;
pub const BAUD_USER: u32 = 268435456u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABINFOA {
    pub pszCab: windows_core::PSTR,
    pub pszInf: windows_core::PSTR,
    pub pszSection: windows_core::PSTR,
    pub szSrcPath: [i8; 260],
    pub dwFlags: u32,
}
impl Default for CABINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CABINFOW {
    pub pszCab: windows_core::PWSTR,
    pub pszInf: windows_core::PWSTR,
    pub pszSection: windows_core::PWSTR,
    pub szSrcPath: [u16; 260],
    pub dwFlags: u32,
}
impl Default for CABINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CATID_DeleteBrowsingHistory: windows_core::GUID = windows_core::GUID::from_u128(0x31caf6e4_d6aa_4090_a050_a5ac8972e9ef);
pub const CBR_110: u32 = 110u32;
pub const CBR_115200: u32 = 115200u32;
pub const CBR_1200: u32 = 1200u32;
pub const CBR_128000: u32 = 128000u32;
pub const CBR_14400: u32 = 14400u32;
pub const CBR_19200: u32 = 19200u32;
pub const CBR_2400: u32 = 2400u32;
pub const CBR_256000: u32 = 256000u32;
pub const CBR_300: u32 = 300u32;
pub const CBR_38400: u32 = 38400u32;
pub const CBR_4800: u32 = 4800u32;
pub const CBR_56000: u32 = 56000u32;
pub const CBR_57600: u32 = 57600u32;
pub const CBR_600: u32 = 600u32;
pub const CBR_9600: u32 = 9600u32;
pub const CE_DNS: u32 = 2048u32;
pub const CE_IOE: u32 = 1024u32;
pub const CE_MODE: u32 = 32768u32;
pub const CE_OOP: u32 = 4096u32;
pub const CE_PTO: u32 = 512u32;
pub const CE_TXFULL: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLIENT_ID {
    pub UniqueProcess: super::super::Foundation::HANDLE,
    pub UniqueThread: super::super::Foundation::HANDLE,
}
pub const CL_NL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(769u32);
pub const CL_NL_IP: u32 = 771u32;
pub const CL_NL_IPX: u32 = 769u32;
pub const CL_TL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(1025u32);
pub const CL_TL_NBF: u32 = 1025u32;
pub const CL_TL_UDP: u32 = 1027u32;
pub const CODEINTEGRITY_OPTION_DEBUGMODE_ENABLED: u32 = 128u32;
pub const CODEINTEGRITY_OPTION_ENABLED: u32 = 1u32;
pub const CODEINTEGRITY_OPTION_FLIGHTING_ENABLED: u32 = 512u32;
pub const CODEINTEGRITY_OPTION_FLIGHT_BUILD: u32 = 256u32;
pub const CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED: u32 = 8192u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_AUDITMODE_ENABLED: u32 = 2048u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED: u32 = 1024u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED: u32 = 4096u32;
pub const CODEINTEGRITY_OPTION_PREPRODUCTION_BUILD: u32 = 64u32;
pub const CODEINTEGRITY_OPTION_TESTSIGN: u32 = 2u32;
pub const CODEINTEGRITY_OPTION_TEST_BUILD: u32 = 32u32;
pub const CODEINTEGRITY_OPTION_UMCI_AUDITMODE_ENABLED: u32 = 8u32;
pub const CODEINTEGRITY_OPTION_UMCI_ENABLED: u32 = 4u32;
pub const CODEINTEGRITY_OPTION_UMCI_EXCLUSIONPATHS_ENABLED: u32 = 16u32;
pub const COMMPROP_INITIALIZED: u32 = 3879531822u32;
pub const CONTEXT_SIZE: u32 = 16u32;
pub const CO_NL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(768u32);
pub const CO_TL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(1024u32);
pub const CO_TL_NBF: u32 = 1024u32;
pub const CO_TL_SPP: u32 = 1030u32;
pub const CO_TL_SPX: u32 = 1026u32;
pub const CO_TL_TCP: u32 = 1028u32;
pub const CP_DIRECT: u32 = 2u32;
pub const CP_HWND: u32 = 0u32;
pub const CP_LEVEL: u32 = 3u32;
pub const CP_OPEN: u32 = 1u32;
pub const CREATE_FOR_DIR: u32 = 2u32;
pub const CREATE_FOR_IMPORT: u32 = 1u32;
pub const CRITICAL_SECTION_NO_DEBUG_INFO: u32 = 16777216u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    pub Size: u32,
    pub TriggerId: windows_core::PCWSTR,
}
pub const CameraUIControl: windows_core::GUID = windows_core::GUID::from_u128(0x16d5a2be_b1c5_47b3_8eae_ccbcf452c7e8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CameraUIControlCaptureMode(pub i32);
impl CameraUIControlCaptureMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CameraUIControlLinearSelectionMode(pub i32);
impl CameraUIControlLinearSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CameraUIControlMode(pub i32);
impl CameraUIControlMode {
    pub const Browse: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CameraUIControlPhotoFormat(pub i32);
impl CameraUIControlPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CameraUIControlVideoFormat(pub i32);
impl CameraUIControlVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CameraUIControlViewType(pub i32);
impl CameraUIControlViewType {
    pub const SingleItem: Self = Self(0i32);
    pub const ItemList: Self = Self(1i32);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DATETIME {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub min: u16,
    pub sec: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DCICMD {
    pub dwCommand: u32,
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub dwVersion: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DCICREATEINPUT {
    pub cmd: DCICMD,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwDCICaps: u32,
    pub dwBitCount: u32,
    pub lpSurface: *mut core::ffi::c_void,
}
impl Default for DCICREATEINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DCICREATEOFFSCREENSURFACE: u32 = 2u32;
pub const DCICREATEOVERLAYSURFACE: u32 = 3u32;
pub const DCICREATEPRIMARYSURFACE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DCIENUMINPUT {
    pub cmd: DCICMD,
    pub rSrc: super::super::Foundation::RECT,
    pub rDst: super::super::Foundation::RECT,
    pub EnumCallback: isize,
    pub lpContext: *mut core::ffi::c_void,
}
impl Default for DCIENUMINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DCIENUMSURFACE: u32 = 4u32;
pub const DCIESCAPE: u32 = 5u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DCIOFFSCREEN {
    pub dciInfo: DCISURFACEINFO,
    pub Draw: isize,
    pub SetClipList: isize,
    pub SetDestination: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DCIOVERLAY {
    pub dciInfo: DCISURFACEINFO,
    pub dwChromakeyValue: u32,
    pub dwChromakeyMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DCISURFACEINFO {
    pub dwSize: u32,
    pub dwDCICaps: u32,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lStride: i32,
    pub dwBitCount: u32,
    pub dwOffSurface: usize,
    pub wSelSurface: u16,
    pub wReserved: u16,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub BeginAccess: isize,
    pub EndAccess: isize,
    pub DestroySurface: isize,
}
impl Default for DCISURFACEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DCI_1632_ACCESS: u32 = 64u32;
pub const DCI_ASYNC: u32 = 1024u32;
pub const DCI_CANOVERLAY: u32 = 65536u32;
pub const DCI_CAN_STRETCHX: u32 = 4096u32;
pub const DCI_CAN_STRETCHXN: u32 = 16384u32;
pub const DCI_CAN_STRETCHY: u32 = 8192u32;
pub const DCI_CAN_STRETCHYN: u32 = 32768u32;
pub const DCI_CHROMAKEY: u32 = 32u32;
pub const DCI_DWORDALIGN: u32 = 256u32;
pub const DCI_DWORDSIZE: u32 = 128u32;
pub const DCI_ERR_CURRENTLYNOTAVAIL: i32 = -5i32;
pub const DCI_ERR_HEIGHTALIGN: i32 = -21i32;
pub const DCI_ERR_INVALIDCLIPLIST: i32 = -15i32;
pub const DCI_ERR_INVALIDPOSITION: i32 = -13i32;
pub const DCI_ERR_INVALIDRECT: i32 = -6i32;
pub const DCI_ERR_INVALIDSTRETCH: i32 = -14i32;
pub const DCI_ERR_OUTOFMEMORY: i32 = -12i32;
pub const DCI_ERR_SURFACEISOBSCURED: i32 = -16i32;
pub const DCI_ERR_TOOBIGHEIGHT: i32 = -9i32;
pub const DCI_ERR_TOOBIGSIZE: i32 = -11i32;
pub const DCI_ERR_TOOBIGWIDTH: i32 = -10i32;
pub const DCI_ERR_UNSUPPORTEDFORMAT: i32 = -7i32;
pub const DCI_ERR_UNSUPPORTEDMASK: i32 = -8i32;
pub const DCI_ERR_WIDTHALIGN: i32 = -20i32;
pub const DCI_ERR_XALIGN: i32 = -17i32;
pub const DCI_ERR_XYALIGN: i32 = -19i32;
pub const DCI_ERR_YALIGN: i32 = -18i32;
pub const DCI_FAIL_GENERIC: i32 = -1i32;
pub const DCI_FAIL_INVALIDSURFACE: i32 = -3i32;
pub const DCI_FAIL_UNSUPPORTED: i32 = -4i32;
pub const DCI_FAIL_UNSUPPORTEDVERSION: i32 = -2i32;
pub const DCI_OFFSCREEN: u32 = 1u32;
pub const DCI_OK: u32 = 0u32;
pub const DCI_OVERLAY: u32 = 2u32;
pub const DCI_PRIMARY: u32 = 0u32;
pub const DCI_STATUS_CHROMAKEYCHANGED: u32 = 16u32;
pub const DCI_STATUS_FORMATCHANGED: u32 = 4u32;
pub const DCI_STATUS_POINTERCHANGED: u32 = 1u32;
pub const DCI_STATUS_STRIDECHANGED: u32 = 2u32;
pub const DCI_STATUS_SURFACEINFOCHANGED: u32 = 8u32;
pub const DCI_STATUS_WASSTILLDRAWING: u32 = 32u32;
pub const DCI_SURFACE_TYPE: u32 = 15u32;
pub const DCI_VERSION: u32 = 256u32;
pub const DCI_VISIBLE: u32 = 16u32;
pub const DCI_WRITEONLY: u32 = 512u32;
pub const DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECISION_LOCATION(pub i32);
pub const DECISION_LOCATION_AUDIT: DECISION_LOCATION = DECISION_LOCATION(2i32);
pub const DECISION_LOCATION_ENFORCE_STATE_LIST: DECISION_LOCATION = DECISION_LOCATION(7i32);
pub const DECISION_LOCATION_ENTERPRISE_DEFINED_CLASS_ID: DECISION_LOCATION = DECISION_LOCATION(4i32);
pub const DECISION_LOCATION_FAILED_CONVERT_GUID: DECISION_LOCATION = DECISION_LOCATION(3i32);
pub const DECISION_LOCATION_GLOBAL_BUILT_IN_LIST: DECISION_LOCATION = DECISION_LOCATION(5i32);
pub const DECISION_LOCATION_NOT_FOUND: DECISION_LOCATION = DECISION_LOCATION(8i32);
pub const DECISION_LOCATION_PARAMETER_VALIDATION: DECISION_LOCATION = DECISION_LOCATION(1i32);
pub const DECISION_LOCATION_PROVIDER_BUILT_IN_LIST: DECISION_LOCATION = DECISION_LOCATION(6i32);
pub const DECISION_LOCATION_REFRESH_GLOBAL_DATA: DECISION_LOCATION = DECISION_LOCATION(0i32);
pub const DECISION_LOCATION_UNKNOWN: DECISION_LOCATION = DECISION_LOCATION(9i32);
pub const DELAYLOAD_GPA_FAILURE: u32 = 4u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA32,
    pub TargetDllName: windows_core::PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut core::ffi::c_void,
    pub Unused: *mut core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(target_arch = "x86")]
impl Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA64,
    pub TargetDllName: windows_core::PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut core::ffi::c_void,
    pub Unused: *mut core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DELAYLOAD_PROC_DESCRIPTOR {
    pub ImportDescribedByName: u32,
    pub Description: DELAYLOAD_PROC_DESCRIPTOR_0,
}
impl Default for DELAYLOAD_PROC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DELAYLOAD_PROC_DESCRIPTOR_0 {
    pub Name: windows_core::PCSTR,
    pub Ordinal: u32,
}
impl Default for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DELETE_BROWSING_HISTORY_COOKIES: u32 = 2u32;
pub const DELETE_BROWSING_HISTORY_DOWNLOADHISTORY: u32 = 64u32;
pub const DELETE_BROWSING_HISTORY_FORMDATA: u32 = 8u32;
pub const DELETE_BROWSING_HISTORY_HISTORY: u32 = 1u32;
pub const DELETE_BROWSING_HISTORY_PASSWORDS: u32 = 16u32;
pub const DELETE_BROWSING_HISTORY_PRESERVEFAVORITES: u32 = 32u32;
pub const DELETE_BROWSING_HISTORY_TIF: u32 = 4u32;
pub const DOCKINFO_DOCKED: u32 = 2u32;
pub const DOCKINFO_UNDOCKED: u32 = 1u32;
pub const DOCKINFO_USER_SUPPLIED: u32 = 4u32;
pub const DRIVE_CDROM: u32 = 5u32;
pub const DRIVE_FIXED: u32 = 3u32;
pub const DRIVE_NO_ROOT_DIR: u32 = 1u32;
pub const DRIVE_RAMDISK: u32 = 6u32;
pub const DRIVE_REMOTE: u32 = 4u32;
pub const DRIVE_REMOVABLE: u32 = 2u32;
pub const DRIVE_UNKNOWN: u32 = 0u32;
pub const DTR_CONTROL_DISABLE: u32 = 0u32;
pub const DTR_CONTROL_ENABLE: u32 = 1u32;
pub const DTR_CONTROL_HANDSHAKE: u32 = 2u32;
pub const DefaultBrowserSyncSettings: windows_core::GUID = windows_core::GUID::from_u128(0x3ac83423_3112_4aa6_9b5b_1feb23d0c5f9);
pub const EFSRPC_SECURE_ONLY: u32 = 8u32;
pub const EFS_DROP_ALTERNATE_STREAMS: u32 = 16u32;
pub const EFS_USE_RECOVERY_KEYS: u32 = 1u32;
pub const ENTITY_LIST_ID: u32 = 0u32;
pub const ENTITY_TYPE_ID: u32 = 1u32;
pub type ENUM_CALLBACK = Option<unsafe extern "system" fn(lpsurfaceinfo: *mut DCISURFACEINFO, lpcontext: *mut core::ffi::c_void)>;
pub const ER_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(896u32);
pub const ER_ICMP: u32 = 896u32;
pub const EVENTLOG_FULL_INFO: u32 = 0u32;
pub const EditionUpgradeBroker: windows_core::GUID = windows_core::GUID::from_u128(0xc4270827_4f39_45df_9288_12ff6b85a921);
pub const EditionUpgradeHelper: windows_core::GUID = windows_core::GUID::from_u128(0x01776df3_b9af_4e50_9b1c_56e93116d704);
pub const EndpointIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(0i32);
pub const FAIL_FAST_GENERATE_EXCEPTION_ADDRESS: u32 = 1u32;
pub const FAIL_FAST_NO_HARD_ERROR_DLG: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_CHANGE_TIME(pub i32);
pub const FEATURE_CHANGE_TIME_MODULE_RELOAD: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(1i32);
pub const FEATURE_CHANGE_TIME_READ: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(0i32);
pub const FEATURE_CHANGE_TIME_REBOOT: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(3i32);
pub const FEATURE_CHANGE_TIME_SESSION: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_ENABLED_STATE(pub i32);
pub const FEATURE_ENABLED_STATE_DEFAULT: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(0i32);
pub const FEATURE_ENABLED_STATE_DISABLED: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(1i32);
pub const FEATURE_ENABLED_STATE_ENABLED: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_ERROR {
    pub hr: windows_core::HRESULT,
    pub lineNumber: u16,
    pub file: windows_core::PCSTR,
    pub process: windows_core::PCSTR,
    pub module: windows_core::PCSTR,
    pub callerReturnAddressOffset: u32,
    pub callerModule: windows_core::PCSTR,
    pub message: windows_core::PCSTR,
    pub originLineNumber: u16,
    pub originFile: windows_core::PCSTR,
    pub originModule: windows_core::PCSTR,
    pub originCallerReturnAddressOffset: u32,
    pub originCallerModule: windows_core::PCSTR,
    pub originName: windows_core::PCSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FEATURE_STATE_CHANGE_SUBSCRIPTION(pub *mut core::ffi::c_void);
impl FEATURE_STATE_CHANGE_SUBSCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("api-ms-win-core-featurestaging-l1-1-0.dll" "system" fn UnsubscribeFeatureStateChangeNotification(subscription : *mut core::ffi::c_void));
            unsafe {
                UnsubscribeFeatureStateChangeNotification(self.0);
            }
        }
    }
}
impl Default for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIBER_FLAG_FLOAT_SWITCH: u32 = 1u32;
pub const FILE_CREATED: u32 = 2u32;
pub const FILE_DIR_DISALLOWED: u32 = 9u32;
pub const FILE_DOES_NOT_EXIST: u32 = 5u32;
pub const FILE_ENCRYPTABLE: u32 = 0u32;
pub const FILE_EXISTS: u32 = 4u32;
pub const FILE_FLAG_IGNORE_IMPERSONATED_DEVICEMAP: u32 = 131072u32;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: u32 = 262144u32;
pub const FILE_IS_ENCRYPTED: u32 = 1u32;
pub const FILE_MAXIMUM_DISPOSITION: u32 = 5u32;
pub const FILE_NO_COMPRESSION: u32 = 32768u32;
pub const FILE_OPENED: u32 = 1u32;
pub const FILE_OPEN_NO_RECALL: u32 = 4194304u32;
pub const FILE_OPEN_REMOTE_INSTANCE: u32 = 1024u32;
pub const FILE_OVERWRITTEN: u32 = 3u32;
pub const FILE_READ_ONLY: u32 = 8u32;
pub const FILE_RENAME_FLAG_POSIX_SEMANTICS: u32 = 2u32;
pub const FILE_RENAME_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
pub const FILE_RENAME_FLAG_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4u32;
pub const FILE_ROOT_DIR: u32 = 3u32;
pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u32 = 1u32;
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: u32 = 2u32;
pub const FILE_SUPERSEDED: u32 = 0u32;
pub const FILE_SYSTEM_ATTR: u32 = 2u32;
pub const FILE_SYSTEM_DIR: u32 = 4u32;
pub const FILE_SYSTEM_NOT_SUPPORT: u32 = 6u32;
pub const FILE_UNKNOWN: u32 = 5u32;
pub const FILE_USER_DISALLOWED: u32 = 7u32;
pub const FILE_VALID_MAILSLOT_OPTION_FLAGS: u32 = 50u32;
pub const FILE_VALID_OPTION_FLAGS: u32 = 16777215u32;
pub const FILE_VALID_PIPE_OPTION_FLAGS: u32 = 50u32;
pub const FILE_VALID_SET_FLAGS: u32 = 54u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_ASSEMBLY_METADATA: u32 = 4u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_FLAGS: u32 = 2u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_HACTCTX: u32 = 1u32;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: u32 = 255u32;
pub const FS_CASE_IS_PRESERVED: u32 = 2u32;
pub const FS_CASE_SENSITIVE: u32 = 1u32;
pub const FS_FILE_COMPRESSION: u32 = 16u32;
pub const FS_FILE_ENCRYPTION: u32 = 131072u32;
pub const FS_PERSISTENT_ACLS: u32 = 8u32;
pub const FS_UNICODE_STORED_ON_DISK: u32 = 4u32;
pub const FS_VOL_IS_COMPRESSED: u32 = 32768u32;
pub const GENERIC_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(0u32);
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_A: windows_core::PCSTR = windows_core::s!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_T: windows_core::PCWSTR = windows_core::w!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_W: windows_core::PCWSTR = windows_core::w!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_A: windows_core::PCWSTR = windows_core::w!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_T: windows_core::PCWSTR = windows_core::w!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_W: windows_core::PCWSTR = windows_core::w!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_A: windows_core::PCSTR = windows_core::s!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_T: windows_core::PCWSTR = windows_core::w!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_W: windows_core::PCWSTR = windows_core::w!("GetSystemWow64DirectoryW");
pub const GMEM_DDESHARE: u32 = 8192u32;
pub const GMEM_DISCARDABLE: u32 = 256u32;
pub const GMEM_DISCARDED: u32 = 16384u32;
pub const GMEM_INVALID_HANDLE: u32 = 32768u32;
pub const GMEM_LOCKCOUNT: u32 = 255u32;
pub const GMEM_LOWER: u32 = 4096u32;
pub const GMEM_MODIFY: u32 = 128u32;
pub const GMEM_NOCOMPACT: u32 = 16u32;
pub const GMEM_NODISCARD: u32 = 32u32;
pub const GMEM_NOTIFY: u32 = 16384u32;
pub const GMEM_NOT_BANKED: u32 = 4096u32;
pub const GMEM_SHARE: u32 = 8192u32;
pub const GMEM_VALID_FLAGS: u32 = 32626u32;
pub const GetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(2i32);
pub const HANJA_WINDOW: u32 = 2u32;
pub const HINSTANCE_ERROR: u32 = 32u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWINWATCH(pub *mut core::ffi::c_void);
impl HWINWATCH {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HWINWATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HW_PROFILE_GUIDLEN: u32 = 39u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HW_PROFILE_INFOA {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [i8; 39],
    pub szHwProfileName: [i8; 80],
}
impl Default for HW_PROFILE_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HW_PROFILE_INFOW {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [u16; 39],
    pub szHwProfileName: [u16; 80],
}
impl Default for HW_PROFILE_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(ICameraUIControl, ICameraUIControl_Vtbl, 0xb8733adf_3d68_4b8f_bb08_e28a0bed0376);
windows_core::imp::interface_hierarchy!(ICameraUIControl, windows_core::IUnknown);
impl ICameraUIControl {
    pub unsafe fn Show<P0, P7>(&self, pwindow: P0, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: bool, peventcallback: P7) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P7: windows_core::Param<ICameraUIControlEventCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), pwindow.param().abi(), mode, selectionmode, capturemode, photoformat, videoformat, bhasclosebutton.into(), peventcallback.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Suspend(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetCurrentViewType(&self) -> windows_core::Result<CameraUIControlViewType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentViewType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetActiveItem(&self, pbstractiveitempath: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetActiveItem)(windows_core::Interface::as_raw(self), pbstractiveitempath.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelectedItems(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelectedItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveCapturedItem<P0>(&self, pszpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveCapturedItem)(windows_core::Interface::as_raw(self), pszpath.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraUIControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, CameraUIControlMode, CameraUIControlLinearSelectionMode, CameraUIControlCaptureMode, CameraUIControlPhotoFormat, CameraUIControlVideoFormat, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentViewType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CameraUIControlViewType) -> windows_core::HRESULT,
    pub GetActiveItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelectedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelectedItems: usize,
    pub RemoveCapturedItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICameraUIControl_Impl: windows_core::IUnknownImpl {
    fn Show(&self, pwindow: windows_core::Ref<windows_core::IUnknown>, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: windows_core::BOOL, peventcallback: windows_core::Ref<ICameraUIControlEventCallback>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Suspend(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn GetCurrentViewType(&self) -> windows_core::Result<CameraUIControlViewType>;
    fn GetActiveItem(&self, pbstractiveitempath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSelectedItems(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn RemoveCapturedItem(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ICameraUIControl_Vtbl {
    pub const fn new<Identity: ICameraUIControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindow: *mut core::ffi::c_void, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: windows_core::BOOL, peventcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICameraUIControl_Impl::Show(this, core::mem::transmute_copy(&pwindow), core::mem::transmute_copy(&mode), core::mem::transmute_copy(&selectionmode), core::mem::transmute_copy(&capturemode), core::mem::transmute_copy(&photoformat), core::mem::transmute_copy(&videoformat), core::mem::transmute_copy(&bhasclosebutton), core::mem::transmute_copy(&peventcallback)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICameraUIControl_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Suspend<Identity: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdeferralrequired: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICameraUIControl_Impl::Suspend(this) {
                    Ok(ok__) => {
                        pbdeferralrequired.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Resume<Identity: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICameraUIControl_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn GetCurrentViewType<Identity: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewtype: *mut CameraUIControlViewType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICameraUIControl_Impl::GetCurrentViewType(this) {
                    Ok(ok__) => {
                        pviewtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActiveItem<Identity: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstractiveitempath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICameraUIControl_Impl::GetActiveItem(this, core::mem::transmute_copy(&pbstractiveitempath)).into()
            }
        }
        unsafe extern "system" fn GetSelectedItems<Identity: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppselecteditempaths: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICameraUIControl_Impl::GetSelectedItems(this) {
                    Ok(ok__) => {
                        ppselecteditempaths.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveCapturedItem<Identity: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICameraUIControl_Impl::RemoveCapturedItem(this, core::mem::transmute(&pszpath)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Show: Show::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            GetCurrentViewType: GetCurrentViewType::<Identity, OFFSET>,
            GetActiveItem: GetActiveItem::<Identity, OFFSET>,
            GetSelectedItems: GetSelectedItems::<Identity, OFFSET>,
            RemoveCapturedItem: RemoveCapturedItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICameraUIControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICameraUIControl {}
windows_core::imp::define_interface!(ICameraUIControlEventCallback, ICameraUIControlEventCallback_Vtbl, 0x1bfa0c2c_fbcd_4776_bda4_88bf974e74f4);
windows_core::imp::interface_hierarchy!(ICameraUIControlEventCallback, windows_core::IUnknown);
impl ICameraUIControlEventCallback {
    pub unsafe fn OnStartupComplete(&self) {
        unsafe { (windows_core::Interface::vtable(self).OnStartupComplete)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnSuspendComplete(&self) {
        unsafe { (windows_core::Interface::vtable(self).OnSuspendComplete)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnItemCaptured<P0>(&self, pszpath: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnItemCaptured)(windows_core::Interface::as_raw(self), pszpath.param().abi()) }
    }
    pub unsafe fn OnItemDeleted<P0>(&self, pszpath: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnItemDeleted)(windows_core::Interface::as_raw(self), pszpath.param().abi()) }
    }
    pub unsafe fn OnClosed(&self) {
        unsafe { (windows_core::Interface::vtable(self).OnClosed)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraUIControlEventCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStartupComplete: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnSuspendComplete: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnItemCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR),
    pub OnItemDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR),
    pub OnClosed: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait ICameraUIControlEventCallback_Impl: windows_core::IUnknownImpl {
    fn OnStartupComplete(&self);
    fn OnSuspendComplete(&self);
    fn OnItemCaptured(&self, pszpath: &windows_core::PCWSTR);
    fn OnItemDeleted(&self, pszpath: &windows_core::PCWSTR);
    fn OnClosed(&self);
}
impl ICameraUIControlEventCallback_Vtbl {
    pub const fn new<Identity: ICameraUIControlEventCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStartupComplete<Identity: ICameraUIControlEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICameraUIControlEventCallback_Impl::OnStartupComplete(this)
            }
        }
        unsafe extern "system" fn OnSuspendComplete<Identity: ICameraUIControlEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICameraUIControlEventCallback_Impl::OnSuspendComplete(this)
            }
        }
        unsafe extern "system" fn OnItemCaptured<Identity: ICameraUIControlEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICameraUIControlEventCallback_Impl::OnItemCaptured(this, core::mem::transmute(&pszpath))
            }
        }
        unsafe extern "system" fn OnItemDeleted<Identity: ICameraUIControlEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICameraUIControlEventCallback_Impl::OnItemDeleted(this, core::mem::transmute(&pszpath))
            }
        }
        unsafe extern "system" fn OnClosed<Identity: ICameraUIControlEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICameraUIControlEventCallback_Impl::OnClosed(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartupComplete: OnStartupComplete::<Identity, OFFSET>,
            OnSuspendComplete: OnSuspendComplete::<Identity, OFFSET>,
            OnItemCaptured: OnItemCaptured::<Identity, OFFSET>,
            OnItemDeleted: OnItemDeleted::<Identity, OFFSET>,
            OnClosed: OnClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICameraUIControlEventCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICameraUIControlEventCallback {}
windows_core::imp::define_interface!(IClipServiceNotificationHelper, IClipServiceNotificationHelper_Vtbl, 0xc39948f0_6142_44fd_98ca_e1681a8d68b5);
windows_core::imp::interface_hierarchy!(IClipServiceNotificationHelper, windows_core::IUnknown);
impl IClipServiceNotificationHelper {
    pub unsafe fn ShowToast(&self, titletext: &windows_core::BSTR, bodytext: &windows_core::BSTR, packagename: &windows_core::BSTR, appid: &windows_core::BSTR, launchcommand: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShowToast)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(titletext), core::mem::transmute_copy(bodytext), core::mem::transmute_copy(packagename), core::mem::transmute_copy(appid), core::mem::transmute_copy(launchcommand)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipServiceNotificationHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ShowToast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IClipServiceNotificationHelper_Impl: windows_core::IUnknownImpl {
    fn ShowToast(&self, titletext: &windows_core::BSTR, bodytext: &windows_core::BSTR, packagename: &windows_core::BSTR, appid: &windows_core::BSTR, launchcommand: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IClipServiceNotificationHelper_Vtbl {
    pub const fn new<Identity: IClipServiceNotificationHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShowToast<Identity: IClipServiceNotificationHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, titletext: *mut core::ffi::c_void, bodytext: *mut core::ffi::c_void, packagename: *mut core::ffi::c_void, appid: *mut core::ffi::c_void, launchcommand: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClipServiceNotificationHelper_Impl::ShowToast(this, core::mem::transmute(&titletext), core::mem::transmute(&bodytext), core::mem::transmute(&packagename), core::mem::transmute(&appid), core::mem::transmute(&launchcommand)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ShowToast: ShowToast::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClipServiceNotificationHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IClipServiceNotificationHelper {}
windows_core::imp::define_interface!(IContainerActivationHelper, IContainerActivationHelper_Vtbl, 0xb524f93f_80d5_4ec7_ae9e_d66e93ade1fa);
windows_core::imp::interface_hierarchy!(IContainerActivationHelper, windows_core::IUnknown);
impl IContainerActivationHelper {
    pub unsafe fn CanActivateClientVM(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanActivateClientVM)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContainerActivationHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CanActivateClientVM: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
pub trait IContainerActivationHelper_Impl: windows_core::IUnknownImpl {
    fn CanActivateClientVM(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
impl IContainerActivationHelper_Vtbl {
    pub const fn new<Identity: IContainerActivationHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanActivateClientVM<Identity: IContainerActivationHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isallowed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContainerActivationHelper_Impl::CanActivateClientVM(this) {
                    Ok(ok__) => {
                        isallowed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CanActivateClientVM: CanActivateClientVM::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContainerActivationHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContainerActivationHelper {}
windows_core::imp::define_interface!(IDefaultBrowserSyncSettings, IDefaultBrowserSyncSettings_Vtbl, 0x7a27faad_5ae6_4255_9030_c530936292e3);
windows_core::imp::interface_hierarchy!(IDefaultBrowserSyncSettings, windows_core::IUnknown);
impl IDefaultBrowserSyncSettings {
    pub unsafe fn IsEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultBrowserSyncSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
pub trait IDefaultBrowserSyncSettings_Impl: windows_core::IUnknownImpl {
    fn IsEnabled(&self) -> windows_core::BOOL;
}
impl IDefaultBrowserSyncSettings_Vtbl {
    pub const fn new<Identity: IDefaultBrowserSyncSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsEnabled<Identity: IDefaultBrowserSyncSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDefaultBrowserSyncSettings_Impl::IsEnabled(this)
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsEnabled: IsEnabled::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDefaultBrowserSyncSettings as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDefaultBrowserSyncSettings {}
windows_core::imp::define_interface!(IDeleteBrowsingHistory, IDeleteBrowsingHistory_Vtbl, 0xcf38ed4b_2be7_4461_8b5e_9a466dc82ae3);
windows_core::imp::interface_hierarchy!(IDeleteBrowsingHistory, windows_core::IUnknown);
impl IDeleteBrowsingHistory {
    pub unsafe fn DeleteBrowsingHistory(&self, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteBrowsingHistory)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteBrowsingHistory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeleteBrowsingHistory: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDeleteBrowsingHistory_Impl: windows_core::IUnknownImpl {
    fn DeleteBrowsingHistory(&self, dwflags: u32) -> windows_core::Result<()>;
}
impl IDeleteBrowsingHistory_Vtbl {
    pub const fn new<Identity: IDeleteBrowsingHistory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeleteBrowsingHistory<Identity: IDeleteBrowsingHistory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeleteBrowsingHistory_Impl::DeleteBrowsingHistory(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DeleteBrowsingHistory: DeleteBrowsingHistory::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeleteBrowsingHistory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDeleteBrowsingHistory {}
pub const IE4_BACKNEW: u32 = 2u32;
pub const IE4_EXTRAINCREFCNT: u32 = 2048u32;
pub const IE4_FRDOALL: u32 = 256u32;
pub const IE4_NODELETENEW: u32 = 4u32;
pub const IE4_NOENUMKEY: u32 = 32u32;
pub const IE4_NOMESSAGES: u32 = 8u32;
pub const IE4_NOPROGRESS: u32 = 16u32;
pub const IE4_NO_CRC_MAPPING: u32 = 64u32;
pub const IE4_REGSECTION: u32 = 128u32;
pub const IE4_REMOVREGBKDATA: u32 = 4096u32;
pub const IE4_RESTORE: u32 = 1u32;
pub const IE4_UPDREFCNT: u32 = 512u32;
pub const IE4_USEREFCNT: u32 = 1024u32;
pub const IE_BADID: i32 = -1i32;
pub const IE_BAUDRATE: i32 = -12i32;
pub const IE_BYTESIZE: i32 = -11i32;
pub const IE_DEFAULT: i32 = -5i32;
pub const IE_HARDWARE: i32 = -10i32;
pub const IE_MEMORY: i32 = -4i32;
pub const IE_NOPEN: i32 = -3i32;
pub const IE_OPEN: i32 = -2i32;
windows_core::imp::define_interface!(IEditionUpgradeBroker, IEditionUpgradeBroker_Vtbl, 0xff19cbcf_9455_4937_b872_6b7929a460af);
windows_core::imp::interface_hierarchy!(IEditionUpgradeBroker, windows_core::IUnknown);
impl IEditionUpgradeBroker {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn InitializeParentWindow(&self, parenthandle: super::Ole::OLE_HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeParentWindow)(windows_core::Interface::as_raw(self), parenthandle).ok() }
    }
    pub unsafe fn UpdateOperatingSystem(&self, parameter: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateOperatingSystem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(parameter)).ok() }
    }
    pub unsafe fn ShowProductKeyUI(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShowProductKeyUI)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CanUpgrade(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CanUpgrade)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEditionUpgradeBroker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub InitializeParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::Ole::OLE_HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    InitializeParentWindow: usize,
    pub UpdateOperatingSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowProductKeyUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanUpgrade: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IEditionUpgradeBroker_Impl: windows_core::IUnknownImpl {
    fn InitializeParentWindow(&self, parenthandle: super::Ole::OLE_HANDLE) -> windows_core::Result<()>;
    fn UpdateOperatingSystem(&self, parameter: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ShowProductKeyUI(&self) -> windows_core::Result<()>;
    fn CanUpgrade(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl IEditionUpgradeBroker_Vtbl {
    pub const fn new<Identity: IEditionUpgradeBroker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeParentWindow<Identity: IEditionUpgradeBroker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parenthandle: super::Ole::OLE_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEditionUpgradeBroker_Impl::InitializeParentWindow(this, core::mem::transmute_copy(&parenthandle)).into()
            }
        }
        unsafe extern "system" fn UpdateOperatingSystem<Identity: IEditionUpgradeBroker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEditionUpgradeBroker_Impl::UpdateOperatingSystem(this, core::mem::transmute(&parameter)).into()
            }
        }
        unsafe extern "system" fn ShowProductKeyUI<Identity: IEditionUpgradeBroker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEditionUpgradeBroker_Impl::ShowProductKeyUI(this).into()
            }
        }
        unsafe extern "system" fn CanUpgrade<Identity: IEditionUpgradeBroker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEditionUpgradeBroker_Impl::CanUpgrade(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeParentWindow: InitializeParentWindow::<Identity, OFFSET>,
            UpdateOperatingSystem: UpdateOperatingSystem::<Identity, OFFSET>,
            ShowProductKeyUI: ShowProductKeyUI::<Identity, OFFSET>,
            CanUpgrade: CanUpgrade::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEditionUpgradeBroker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for IEditionUpgradeBroker {}
windows_core::imp::define_interface!(IEditionUpgradeHelper, IEditionUpgradeHelper_Vtbl, 0xd3e9e342_5deb_43b6_849e_6913b85d503a);
windows_core::imp::interface_hierarchy!(IEditionUpgradeHelper, windows_core::IUnknown);
impl IEditionUpgradeHelper {
    pub unsafe fn CanUpgrade(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanUpgrade)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpdateOperatingSystem<P0>(&self, contentid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOperatingSystem)(windows_core::Interface::as_raw(self), contentid.param().abi()).ok() }
    }
    pub unsafe fn ShowProductKeyUI(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShowProductKeyUI)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetOsProductContentId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOsProductContentId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGenuineLocalStatus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGenuineLocalStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEditionUpgradeHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CanUpgrade: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub UpdateOperatingSystem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ShowProductKeyUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOsProductContentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetGenuineLocalStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IEditionUpgradeHelper_Impl: windows_core::IUnknownImpl {
    fn CanUpgrade(&self) -> windows_core::Result<windows_core::BOOL>;
    fn UpdateOperatingSystem(&self, contentid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ShowProductKeyUI(&self) -> windows_core::Result<()>;
    fn GetOsProductContentId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetGenuineLocalStatus(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IEditionUpgradeHelper_Vtbl {
    pub const fn new<Identity: IEditionUpgradeHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanUpgrade<Identity: IEditionUpgradeHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isallowed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEditionUpgradeHelper_Impl::CanUpgrade(this) {
                    Ok(ok__) => {
                        isallowed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateOperatingSystem<Identity: IEditionUpgradeHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEditionUpgradeHelper_Impl::UpdateOperatingSystem(this, core::mem::transmute(&contentid)).into()
            }
        }
        unsafe extern "system" fn ShowProductKeyUI<Identity: IEditionUpgradeHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEditionUpgradeHelper_Impl::ShowProductKeyUI(this).into()
            }
        }
        unsafe extern "system" fn GetOsProductContentId<Identity: IEditionUpgradeHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEditionUpgradeHelper_Impl::GetOsProductContentId(this) {
                    Ok(ok__) => {
                        contentid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGenuineLocalStatus<Identity: IEditionUpgradeHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isgenuine: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEditionUpgradeHelper_Impl::GetGenuineLocalStatus(this) {
                    Ok(ok__) => {
                        isgenuine.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CanUpgrade: CanUpgrade::<Identity, OFFSET>,
            UpdateOperatingSystem: UpdateOperatingSystem::<Identity, OFFSET>,
            ShowProductKeyUI: ShowProductKeyUI::<Identity, OFFSET>,
            GetOsProductContentId: GetOsProductContentId::<Identity, OFFSET>,
            GetGenuineLocalStatus: GetGenuineLocalStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEditionUpgradeHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEditionUpgradeHelper {}
windows_core::imp::define_interface!(IFClipNotificationHelper, IFClipNotificationHelper_Vtbl, 0x3d5e3d21_bd41_4c2a_a669_b17ce87fb50b);
windows_core::imp::interface_hierarchy!(IFClipNotificationHelper, windows_core::IUnknown);
impl IFClipNotificationHelper {
    pub unsafe fn ShowSystemDialog(&self, titletext: &windows_core::BSTR, bodytext: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShowSystemDialog)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(titletext), core::mem::transmute_copy(bodytext)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFClipNotificationHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ShowSystemDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFClipNotificationHelper_Impl: windows_core::IUnknownImpl {
    fn ShowSystemDialog(&self, titletext: &windows_core::BSTR, bodytext: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IFClipNotificationHelper_Vtbl {
    pub const fn new<Identity: IFClipNotificationHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShowSystemDialog<Identity: IFClipNotificationHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, titletext: *mut core::ffi::c_void, bodytext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFClipNotificationHelper_Impl::ShowSystemDialog(this, core::mem::transmute(&titletext), core::mem::transmute(&bodytext)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ShowSystemDialog: ShowSystemDialog::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFClipNotificationHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFClipNotificationHelper {}
pub const IF_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(512u32);
pub const IF_GENERIC: u32 = 512u32;
pub const IF_MIB: u32 = 514u32;
pub const IGNORE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR {
    pub Attributes: IMAGE_DELAYLOAD_DESCRIPTOR_0,
    pub DllNameRVA: u32,
    pub ModuleHandleRVA: u32,
    pub ImportAddressTableRVA: u32,
    pub ImportNameTableRVA: u32,
    pub BoundImportAddressTableRVA: u32,
    pub UnloadInformationTableRVA: u32,
    pub TimeDateStamp: u32,
}
impl Default for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    pub AllAttributes: u32,
    pub Anonymous: IMAGE_DELAYLOAD_DESCRIPTOR_0_0,
}
impl Default for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_THUNK_DATA32 {
    pub u1: IMAGE_THUNK_DATA32_0,
}
impl Default for IMAGE_THUNK_DATA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_THUNK_DATA32_0 {
    pub ForwarderString: u32,
    pub Function: u32,
    pub Ordinal: u32,
    pub AddressOfData: u32,
}
impl Default for IMAGE_THUNK_DATA32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_THUNK_DATA64 {
    pub u1: IMAGE_THUNK_DATA64_0,
}
impl Default for IMAGE_THUNK_DATA64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_THUNK_DATA64_0 {
    pub ForwarderString: u64,
    pub Function: u64,
    pub Ordinal: u64,
    pub AddressOfData: u64,
}
impl Default for IMAGE_THUNK_DATA64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMEA_INIT: u32 = 1u32;
pub const IMEA_NEXT: u32 = 2u32;
pub const IMEA_PREV: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMEPROA {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u8; 50],
    pub szName: [u8; 80],
    pub szOptions: [u8; 30],
}
impl Default for IMEPROA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMEPROW {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u16; 50],
    pub szName: [u16; 80],
    pub szOptions: [u16; 30],
}
impl Default for IMEPROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMESTRUCT {
    pub fnc: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub wCount: u32,
    pub dchSource: u32,
    pub dchDest: u32,
    pub lParam1: super::super::Foundation::LPARAM,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lParam3: super::super::Foundation::LPARAM,
}
pub const IME_BANJAtoJUNJA: u32 = 19u32;
pub const IME_ENABLE_CONVERT: u32 = 2u32;
pub const IME_ENTERWORDREGISTERMODE: u32 = 24u32;
pub const IME_GETCONVERSIONMODE: u32 = 17u32;
pub const IME_GETIMECAPS: u32 = 3u32;
pub const IME_GETOPEN: u32 = 5u32;
pub const IME_GETVERSION: u32 = 7u32;
pub const IME_JOHABtoKS: u32 = 21u32;
pub const IME_JUNJAtoBANJA: u32 = 20u32;
pub const IME_KStoJOHAB: u32 = 22u32;
pub const IME_MAXPROCESS: u32 = 32u32;
pub const IME_MODE_ALPHANUMERIC: u32 = 1u32;
pub const IME_MODE_CODEINPUT: u32 = 128u32;
pub const IME_MODE_DBCSCHAR: u32 = 16u32;
pub const IME_MODE_HANJACONVERT: u32 = 4u32;
pub const IME_MODE_HIRAGANA: u32 = 4u32;
pub const IME_MODE_KATAKANA: u32 = 2u32;
pub const IME_MODE_NOCODEINPUT: u32 = 256u32;
pub const IME_MODE_NOROMAN: u32 = 64u32;
pub const IME_MODE_ROMAN: u32 = 32u32;
pub const IME_MODE_SBCSCHAR: u32 = 2u32;
pub const IME_MOVEIMEWINDOW: u32 = 8u32;
pub const IME_REQUEST_CONVERT: u32 = 1u32;
pub const IME_RS_DISKERROR: u32 = 14u32;
pub const IME_RS_ERROR: u32 = 1u32;
pub const IME_RS_ILLEGAL: u32 = 6u32;
pub const IME_RS_INVALID: u32 = 17u32;
pub const IME_RS_NEST: u32 = 18u32;
pub const IME_RS_NOIME: u32 = 2u32;
pub const IME_RS_NOROOM: u32 = 10u32;
pub const IME_RS_NOTFOUND: u32 = 7u32;
pub const IME_RS_SYSTEMMODAL: u32 = 19u32;
pub const IME_RS_TOOLONG: u32 = 5u32;
pub const IME_SENDVKEY: u32 = 19u32;
pub const IME_SETCONVERSIONFONTEX: u32 = 25u32;
pub const IME_SETCONVERSIONMODE: u32 = 16u32;
pub const IME_SETCONVERSIONWINDOW: u32 = 8u32;
pub const IME_SETOPEN: u32 = 4u32;
pub const IME_SET_MODE: u32 = 18u32;
pub const INFO_CLASS_GENERIC: u32 = 256u32;
pub const INFO_CLASS_IMPLEMENTATION: u32 = 768u32;
pub const INFO_CLASS_PROTOCOL: u32 = 512u32;
pub const INFO_TYPE_ADDRESS_OBJECT: u32 = 512u32;
pub const INFO_TYPE_CONNECTION: u32 = 768u32;
pub const INFO_TYPE_PROVIDER: u32 = 256u32;
pub const INTERIM_WINDOW: u32 = 0u32;
pub const INVALID_ENTITY_INSTANCE: i32 = -1i32;
pub const IOCTL_TDI_TL_IO_CONTROL_ENDPOINT: u32 = 2162744u32;
pub const IR_CHANGECONVERT: u32 = 289u32;
pub const IR_CLOSECONVERT: u32 = 290u32;
pub const IR_DBCSCHAR: u32 = 352u32;
pub const IR_FULLCONVERT: u32 = 291u32;
pub const IR_IMESELECT: u32 = 304u32;
pub const IR_MODEINFO: u32 = 400u32;
pub const IR_OPENCONVERT: u32 = 288u32;
pub const IR_STRING: u32 = 320u32;
pub const IR_STRINGEND: u32 = 257u32;
pub const IR_STRINGEX: u32 = 384u32;
pub const IR_STRINGSTART: u32 = 256u32;
pub const IR_UNDETERMINE: u32 = 368u32;
windows_core::imp::define_interface!(IWindowsLockModeHelper, IWindowsLockModeHelper_Vtbl, 0xf342d19e_cc22_4648_bb5d_03ccf75b47c5);
windows_core::imp::interface_hierarchy!(IWindowsLockModeHelper, windows_core::IUnknown);
impl IWindowsLockModeHelper {
    pub unsafe fn GetSMode(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsLockModeHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWindowsLockModeHelper_Impl: windows_core::IUnknownImpl {
    fn GetSMode(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IWindowsLockModeHelper_Vtbl {
    pub const fn new<Identity: IWindowsLockModeHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSMode<Identity: IWindowsLockModeHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, issmode: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsLockModeHelper_Impl::GetSMode(this) {
                    Ok(ok__) => {
                        issmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSMode: GetSMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsLockModeHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWindowsLockModeHelper {}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JAVA_TRUST {
    pub cbSize: u32,
    pub flag: u32,
    pub fAllActiveXPermissions: windows_core::BOOL,
    pub fAllPermissions: windows_core::BOOL,
    pub dwEncodingType: u32,
    pub pbJavaPermissions: *mut u8,
    pub cbJavaPermissions: u32,
    pub pbSigner: *mut u8,
    pub cbSigner: u32,
    pub pwszZone: windows_core::PCWSTR,
    pub guidZone: windows_core::GUID,
    pub hVerify: windows_core::HRESULT,
}
impl Default for JAVA_TRUST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JIT_DEBUG_INFO {
    pub dwSize: u32,
    pub dwProcessorArchitecture: u32,
    pub dwThreadID: u32,
    pub dwReserved0: u32,
    pub lpExceptionAddress: u64,
    pub lpExceptionRecord: u64,
    pub lpContextRecord: u64,
}
pub const KEY_ALL_KEYS: WLDP_KEY = WLDP_KEY(2i32);
pub const KEY_OVERRIDE: WLDP_KEY = WLDP_KEY(1i32);
pub const KEY_UNKNOWN: WLDP_KEY = WLDP_KEY(0i32);
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct LDR_DATA_TABLE_ENTRY {
    pub Reserved1: [*mut core::ffi::c_void; 2],
    pub InMemoryOrderLinks: super::Kernel::LIST_ENTRY,
    pub Reserved2: [*mut core::ffi::c_void; 2],
    pub DllBase: *mut core::ffi::c_void,
    pub Reserved3: [*mut core::ffi::c_void; 2],
    pub FullDllName: super::super::Foundation::UNICODE_STRING,
    pub Reserved4: [u8; 8],
    pub Reserved5: [*mut core::ffi::c_void; 3],
    pub Anonymous: LDR_DATA_TABLE_ENTRY_0,
    pub TimeDateStamp: u32,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for LDR_DATA_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union LDR_DATA_TABLE_ENTRY_0 {
    pub CheckSum: u32,
    pub Reserved6: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for LDR_DATA_TABLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LIS_NOGRPCONV: u32 = 2u32;
pub const LIS_QUIET: u32 = 1u32;
pub const LOGON32_PROVIDER_VIRTUAL: u32 = 4u32;
pub const LOGON32_PROVIDER_WINNT35: u32 = 1u32;
pub const LOGON_ZERO_PASSWORD_BUFFER: u32 = 2147483648u32;
pub const LPTx: u32 = 128u32;
pub const MAXINTATOM: u32 = 49152u32;
pub const MAX_COMPUTERNAME_LENGTH: u32 = 15u32;
pub const MAX_TDI_ENTITIES: u32 = 4096u32;
pub const MCW_DEFAULT: u32 = 0u32;
pub const MCW_HIDDEN: u32 = 16u32;
pub const MCW_RECT: u32 = 1u32;
pub const MCW_SCREEN: u32 = 4u32;
pub const MCW_VERTICAL: u32 = 8u32;
pub const MCW_WINDOW: u32 = 2u32;
pub const MICROSOFT_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
pub const MICROSOFT_WINDOWS_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
pub const MODE_WINDOW: u32 = 1u32;
pub const OFS_MAXPATHNAME: u32 = 128u32;
pub const OPERATION_API_VERSION: u32 = 1u32;
pub const OVERWRITE_HIDDEN: u32 = 4u32;
pub const PCF_16BITMODE: u32 = 512u32;
pub const PCF_DTRDSR: u32 = 1u32;
pub const PCF_INTTIMEOUTS: u32 = 128u32;
pub const PCF_PARITY_CHECK: u32 = 8u32;
pub const PCF_RLSD: u32 = 4u32;
pub const PCF_RTSCTS: u32 = 2u32;
pub const PCF_SETXCHAR: u32 = 32u32;
pub const PCF_SPECIALCHARS: u32 = 256u32;
pub const PCF_TOTALTIMEOUTS: u32 = 64u32;
pub const PCF_XONXOFF: u32 = 16u32;
pub type PDELAYLOAD_FAILURE_DLL_CALLBACK = Option<unsafe extern "system" fn(notificationreason: u32, delayloadinfo: *const DELAYLOAD_INFO) -> *mut core::ffi::c_void>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PERUSERSECTIONA {
    pub szGUID: [i8; 59],
    pub szDispName: [i8; 128],
    pub szLocale: [i8; 10],
    pub szStub: [i8; 1040],
    pub szVersion: [i8; 32],
    pub szCompID: [i8; 128],
    pub dwIsInstalled: u32,
    pub bRollback: windows_core::BOOL,
}
impl Default for PERUSERSECTIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PERUSERSECTIONW {
    pub szGUID: [u16; 59],
    pub szDispName: [u16; 128],
    pub szLocale: [u16; 10],
    pub szStub: [u16; 1040],
    pub szVersion: [u16; 32],
    pub szCompID: [u16; 128],
    pub dwIsInstalled: u32,
    pub bRollback: windows_core::BOOL,
}
impl Default for PERUSERSECTIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFEATURE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PFIBER_CALLOUT_ROUTINE = Option<unsafe extern "system" fn(lpparameter: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type PQUERYACTCTXW_FUNC = Option<unsafe extern "system" fn(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: *const core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> windows_core::BOOL>;
pub const PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT: u32 = 1u32;
pub const PROCESS_CREATION_CHILD_PROCESS_OVERRIDE: u32 = 2u32;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED: u32 = 1u32;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED_UNLESS_SECURE: u32 = 4u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_DISABLE_PROCESS_TREE: u32 = 2u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_ENABLE_PROCESS_TREE: u32 = 1u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_OVERRIDE: u32 = 4u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE: u32 = 2u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE: u32 = 1u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE: u32 = 4u32;
pub const PROC_THREAD_ATTRIBUTE_ADDITIVE: u32 = 262144u32;
pub const PROC_THREAD_ATTRIBUTE_INPUT: u32 = 131072u32;
pub const PROC_THREAD_ATTRIBUTE_NUMBER: u32 = 65535u32;
pub const PROC_THREAD_ATTRIBUTE_THREAD: u32 = 65536u32;
pub const PROTECTION_LEVEL_SAME: u32 = 4294967295u32;
pub const PST_FAX: u32 = 33u32;
pub const PST_LAT: u32 = 257u32;
pub const PST_MODEM: u32 = 6u32;
pub const PST_NETWORK_BRIDGE: u32 = 256u32;
pub const PST_PARALLELPORT: u32 = 2u32;
pub const PST_RS232: u32 = 1u32;
pub const PST_RS422: u32 = 3u32;
pub const PST_RS423: u32 = 4u32;
pub const PST_RS449: u32 = 5u32;
pub const PST_SCANNER: u32 = 34u32;
pub const PST_TCPIP_TELNET: u32 = 258u32;
pub const PST_UNSPECIFIED: u32 = 0u32;
pub const PST_X25: u32 = 259u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub Reserved: [u32; 10],
}
impl Default for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: super::super::Foundation::UNICODE_STRING,
    pub Reserved: [u32; 22],
}
impl Default for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PWINSTATIONQUERYINFORMATIONW = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: WINSTATIONINFOCLASS, param3: *mut core::ffi::c_void, param4: u32, param5: *mut u32) -> bool>;
pub type PWLDP_CANEXECUTEBUFFER_API = Option<unsafe extern "system" fn(host: *const windows_core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, buffer: *const u8, buffersize: u32, auditinfo: windows_core::PCWSTR, result: *mut WLDP_EXECUTION_POLICY) -> windows_core::HRESULT>;
pub type PWLDP_CANEXECUTEFILE_API = Option<unsafe extern "system" fn(host: *const windows_core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, filehandle: super::super::Foundation::HANDLE, auditinfo: windows_core::PCWSTR, result: *mut WLDP_EXECUTION_POLICY) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_System_Com")]
pub type PWLDP_CANEXECUTESTREAM_API = Option<unsafe extern "system" fn(host: *const windows_core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, stream: windows_core::Ref<super::Com::IStream>, auditinfo: windows_core::PCWSTR, result: *mut WLDP_EXECUTION_POLICY) -> windows_core::HRESULT>;
pub type PWLDP_ISAPPAPPROVEDBYPOLICY_API = Option<unsafe extern "system" fn(packagefamilyname: windows_core::PCWSTR, packageversion: u64) -> windows_core::HRESULT>;
pub type PWLDP_ISDYNAMICCODEPOLICYENABLED_API = Option<unsafe extern "system" fn(pbenabled: *mut windows_core::BOOL) -> windows_core::HRESULT>;
pub type PWLDP_ISPRODUCTIONCONFIGURATION_API = Option<unsafe extern "system" fn(isproductionconfiguration: *mut windows_core::BOOL) -> windows_core::HRESULT>;
pub type PWLDP_ISWCOSPRODUCTIONCONFIGURATION_API = Option<unsafe extern "system" fn(isproductionconfiguration: *mut windows_core::BOOL) -> windows_core::HRESULT>;
pub type PWLDP_QUERYDEVICESECURITYINFORMATION_API = Option<unsafe extern "system" fn(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> windows_core::HRESULT>;
pub type PWLDP_QUERYDYNAMICODETRUST_API = Option<unsafe extern "system" fn(filehandle: super::super::Foundation::HANDLE, baseimage: *const core::ffi::c_void, imagesize: u32) -> windows_core::HRESULT>;
pub type PWLDP_QUERYPOLICYSETTINGENABLED2_API = Option<unsafe extern "system" fn(setting: windows_core::PCWSTR, enabled: *mut windows_core::BOOL) -> windows_core::HRESULT>;
pub type PWLDP_QUERYPOLICYSETTINGENABLED_API = Option<unsafe extern "system" fn(setting: WLDP_POLICY_SETTING, enabled: *mut windows_core::BOOL) -> windows_core::HRESULT>;
pub type PWLDP_QUERYWINDOWSLOCKDOWNMODE_API = Option<unsafe extern "system" fn(lockdownmode: *mut WLDP_WINDOWS_LOCKDOWN_MODE) -> windows_core::HRESULT>;
pub type PWLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_API = Option<unsafe extern "system" fn(lockdownrestriction: *mut WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> windows_core::HRESULT>;
pub type PWLDP_RESETPRODUCTIONCONFIGURATION_API = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type PWLDP_RESETWCOSPRODUCTIONCONFIGURATION_API = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type PWLDP_SETDYNAMICCODETRUST_API = Option<unsafe extern "system" fn(hfilehandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT>;
pub type PWLDP_SETWINDOWSLOCKDOWNRESTRICTION_API = Option<unsafe extern "system" fn(lockdownrestriction: WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> windows_core::HRESULT>;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS: u32 = 16u32;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_HMODULE: u32 = 8u32;
pub const QUERY_ACTCTX_FLAG_NO_ADDREF: u32 = 2147483648u32;
pub const QUERY_ACTCTX_FLAG_USE_ACTIVE_ACTCTX: u32 = 4u32;
pub const RECOVERY_DEFAULT_PING_INTERVAL: u32 = 5000u32;
pub type REGINSTALLA = Option<unsafe extern "system" fn(hm: super::super::Foundation::HMODULE, pszsection: windows_core::PCSTR, psttable: *const STRTABLEA) -> windows_core::HRESULT>;
pub const REG_RESTORE_LOG_KEY: windows_core::PCWSTR = windows_core::w!("RegRestoreLogFile");
pub const REG_SAVE_LOG_KEY: windows_core::PCWSTR = windows_core::w!("RegSaveLogFile");
pub const REMOTE_PROTOCOL_INFO_FLAG_LOOPBACK: u32 = 1u32;
pub const REMOTE_PROTOCOL_INFO_FLAG_OFFLINE: u32 = 2u32;
pub const REMOTE_PROTOCOL_INFO_FLAG_PERSISTENT_HANDLE: u32 = 4u32;
pub const RESETDEV: u32 = 7u32;
pub const RESTART_MAX_CMD_LINE: u32 = 1024u32;
pub const RPI_FLAG_SMB2_SHARECAP_CLUSTER: u32 = 64u32;
pub const RPI_FLAG_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16u32;
pub const RPI_FLAG_SMB2_SHARECAP_DFS: u32 = 8u32;
pub const RPI_FLAG_SMB2_SHARECAP_SCALEOUT: u32 = 32u32;
pub const RPI_FLAG_SMB2_SHARECAP_TIMEWARP: u32 = 2u32;
pub const RPI_SMB2_FLAG_SERVERCAP_DFS: u32 = 1u32;
pub const RPI_SMB2_FLAG_SERVERCAP_DIRECTORY_LEASING: u32 = 32u32;
pub const RPI_SMB2_FLAG_SERVERCAP_LARGEMTU: u32 = 4u32;
pub const RPI_SMB2_FLAG_SERVERCAP_LEASING: u32 = 2u32;
pub const RPI_SMB2_FLAG_SERVERCAP_MULTICHANNEL: u32 = 8u32;
pub const RPI_SMB2_FLAG_SERVERCAP_PERSISTENT_HANDLES: u32 = 16u32;
pub const RPI_SMB2_SHAREFLAG_COMPRESS_DATA: u32 = 2u32;
pub const RPI_SMB2_SHAREFLAG_ENCRYPT_DATA: u32 = 1u32;
pub const RSC_FLAG_DELAYREGISTEROCX: u32 = 512u32;
pub const RSC_FLAG_INF: u32 = 1u32;
pub const RSC_FLAG_NGCONV: u32 = 8u32;
pub const RSC_FLAG_QUIET: u32 = 4u32;
pub const RSC_FLAG_SETUPAPI: u32 = 1024u32;
pub const RSC_FLAG_SKIPDISKSPACECHECK: u32 = 2u32;
pub const RSC_FLAG_UPDHLPDLLS: u32 = 16u32;
pub const RTS_CONTROL_DISABLE: u32 = 0u32;
pub const RTS_CONTROL_ENABLE: u32 = 1u32;
pub const RTS_CONTROL_HANDSHAKE: u32 = 2u32;
pub const RTS_CONTROL_TOGGLE: u32 = 3u32;
pub const RUNCMDS_DELAYPOSTCMD: u32 = 4u32;
pub const RUNCMDS_NOWAIT: u32 = 2u32;
pub const RUNCMDS_QUIET: u32 = 1u32;
pub const SCS_32BIT_BINARY: u32 = 0u32;
pub const SCS_64BIT_BINARY: u32 = 6u32;
pub const SCS_DOS_BINARY: u32 = 1u32;
pub const SCS_OS216_BINARY: u32 = 5u32;
pub const SCS_PIF_BINARY: u32 = 3u32;
pub const SCS_POSIX_BINARY: u32 = 4u32;
pub const SCS_THIS_PLATFORM_BINARY: u32 = 6u32;
pub const SCS_WOW_BINARY: u32 = 2u32;
pub const SHUTDOWN_NORETRY: u32 = 1u32;
pub const SP_BAUD: u32 = 2u32;
pub const SP_DATABITS: u32 = 4u32;
pub const SP_HANDSHAKING: u32 = 16u32;
pub const SP_PARITY: u32 = 1u32;
pub const SP_PARITY_CHECK: u32 = 32u32;
pub const SP_RLSD: u32 = 64u32;
pub const SP_SERIALCOMM: u32 = 1u32;
pub const SP_STOPBITS: u32 = 8u32;
pub const STARTF_HOLOGRAPHIC: u32 = 262144u32;
pub const STORAGE_INFO_FLAGS_ALIGNED_DEVICE: u32 = 1u32;
pub const STORAGE_INFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 2u32;
pub const STORAGE_INFO_OFFSET_UNKNOWN: u32 = 4294967295u32;
pub const STREAM_CONTAINS_GHOSTED_FILE_EXTENTS: u32 = 16u32;
pub const STREAM_CONTAINS_PROPERTIES: u32 = 4u32;
pub const STREAM_CONTAINS_SECURITY: u32 = 2u32;
pub const STREAM_MODIFIED_WHEN_READ: u32 = 1u32;
pub const STREAM_NORMAL_ATTRIBUTE: u32 = 0u32;
pub const STREAM_SPARSE_ATTRIBUTE: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STRENTRYA {
    pub pszName: windows_core::PSTR,
    pub pszValue: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STRENTRYW {
    pub pszName: windows_core::PWSTR,
    pub pszValue: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STRINGEXSTRUCT {
    pub dwSize: u32,
    pub uDeterminePos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiPos: u32,
    pub uYomiDelimPos: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STRTABLEA {
    pub cEntries: u32,
    pub pse: *mut STRENTRYA,
}
impl Default for STRTABLEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STRTABLEW {
    pub cEntries: u32,
    pub pse: *mut STRENTRYW,
}
impl Default for STRTABLEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_BASIC_INFORMATION {
    pub Reserved1: [u8; 24],
    pub Reserved2: [*mut core::ffi::c_void; 4],
    pub NumberOfProcessors: i8,
}
impl Default for SYSTEM_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_CODEINTEGRITY_INFORMATION {
    pub Length: u32,
    pub CodeIntegrityOptions: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_EXCEPTION_INFORMATION {
    pub Reserved1: [u8; 16],
}
impl Default for SYSTEM_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_INTERRUPT_INFORMATION {
    pub Reserved1: [u8; 24],
}
impl Default for SYSTEM_INTERRUPT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_LOOKASIDE_INFORMATION {
    pub Reserved1: [u8; 32],
}
impl Default for SYSTEM_LOOKASIDE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_PERFORMANCE_INFORMATION {
    pub Reserved1: [u8; 312],
}
impl Default for SYSTEM_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut core::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl Default for SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
    pub Reserved1: [i64; 2],
    pub Reserved2: u32,
}
impl Default for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub Reserved1: [u8; 48],
    pub ImageName: super::super::Foundation::UNICODE_STRING,
    pub BasePriority: i32,
    pub UniqueProcessId: super::super::Foundation::HANDLE,
    pub Reserved2: *mut core::ffi::c_void,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub Reserved3: *mut core::ffi::c_void,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub Reserved4: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub Reserved5: *mut core::ffi::c_void,
    pub QuotaPagedPoolUsage: usize,
    pub Reserved6: *mut core::ffi::c_void,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivatePageCount: usize,
    pub Reserved7: [i64; 6],
}
impl Default for SYSTEM_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    pub RegistryQuotaAllowed: u32,
    pub RegistryQuotaUsed: u32,
    pub Reserved1: *mut core::ffi::c_void,
}
impl Default for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SYSTEM_STATUS_FLAG_POWER_SAVING_ON: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_THREAD_INFORMATION {
    pub Reserved1: [i64; 3],
    pub Reserved2: u32,
    pub StartAddress: *mut core::ffi::c_void,
    pub ClientId: CLIENT_ID,
    pub Priority: i32,
    pub BasePriority: i32,
    pub Reserved3: u32,
    pub ThreadState: u32,
    pub WaitReason: u32,
}
impl Default for SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_TIMEOFDAY_INFORMATION {
    pub Reserved1: [u8; 48],
}
impl Default for SYSTEM_TIMEOFDAY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const S_ALLTHRESHOLD: u32 = 2u32;
pub const S_LEGATO: u32 = 1u32;
pub const S_NORMAL: u32 = 0u32;
pub const S_PERIOD1024: u32 = 1u32;
pub const S_PERIOD2048: u32 = 2u32;
pub const S_PERIOD512: u32 = 0u32;
pub const S_PERIODVOICE: u32 = 3u32;
pub const S_QUEUEEMPTY: u32 = 0u32;
pub const S_SERBDNT: i32 = -5i32;
pub const S_SERDCC: i32 = -7i32;
pub const S_SERDDR: i32 = -14i32;
pub const S_SERDFQ: i32 = -13i32;
pub const S_SERDLN: i32 = -6i32;
pub const S_SERDMD: i32 = -10i32;
pub const S_SERDPT: i32 = -12i32;
pub const S_SERDSH: i32 = -11i32;
pub const S_SERDSR: i32 = -15i32;
pub const S_SERDST: i32 = -16i32;
pub const S_SERDTP: i32 = -8i32;
pub const S_SERDVL: i32 = -9i32;
pub const S_SERDVNA: i32 = -1i32;
pub const S_SERMACT: i32 = -3i32;
pub const S_SEROFM: i32 = -2i32;
pub const S_SERQFUL: i32 = -4i32;
pub const S_STACCATO: u32 = 2u32;
pub const S_THRESHOLD: u32 = 1u32;
pub const S_WHITE1024: u32 = 5u32;
pub const S_WHITE2048: u32 = 6u32;
pub const S_WHITE512: u32 = 4u32;
pub const S_WHITEVOICE: u32 = 7u32;
pub const SetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(1i32);
pub const SocketIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(3i32);
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    pub ID: TDIObjectID,
    pub Context: [u32; 4],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    pub ID: TDIObjectID,
    pub Context: [u8; 16],
}
impl Default for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    pub ID: TDIObjectID,
    pub Context: [usize; 4],
}
impl Default for TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TCP_REQUEST_SET_INFORMATION_EX {
    pub ID: TDIObjectID,
    pub BufferSize: u32,
    pub Buffer: [u8; 1],
}
impl Default for TCP_REQUEST_SET_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TC_GP_TRAP: u32 = 2u32;
pub const TC_HARDERR: u32 = 1u32;
pub const TC_NORMAL: u32 = 0u32;
pub const TC_SIGNAL: u32 = 3u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TDIENTITY_ENTITY_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TDIEntityID {
    pub tei_entity: TDIENTITY_ENTITY_TYPE,
    pub tei_instance: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TDIObjectID {
    pub toi_entity: TDIEntityID,
    pub toi_class: u32,
    pub toi_type: u32,
    pub toi_id: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TDI_TL_IO_CONTROL_ENDPOINT {
    pub Type: TDI_TL_IO_CONTROL_TYPE,
    pub Level: u32,
    pub Anonymous: TDI_TL_IO_CONTROL_ENDPOINT_0,
    pub InputBuffer: *mut core::ffi::c_void,
    pub InputBufferLength: u32,
    pub OutputBuffer: *mut core::ffi::c_void,
    pub OutputBufferLength: u32,
}
impl Default for TDI_TL_IO_CONTROL_ENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TDI_TL_IO_CONTROL_ENDPOINT_0 {
    pub IoControlCode: u32,
    pub OptionName: u32,
}
impl Default for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TDI_TL_IO_CONTROL_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct THREAD_NAME_INFORMATION {
    pub ThreadName: super::super::Foundation::UNICODE_STRING,
}
pub const THREAD_PRIORITY_ERROR_RETURN: u32 = 2147483647u32;
pub const UMS_VERSION: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNDETERMINESTRUCT {
    pub dwSize: u32,
    pub uDefIMESize: u32,
    pub uDefIMEPos: u32,
    pub uUndetTextLen: u32,
    pub uUndetTextPos: u32,
    pub uUndetAttrPos: u32,
    pub uCursorPos: u32,
    pub uDeltaStart: u32,
    pub uDetermineTextLen: u32,
    pub uDetermineTextPos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiTextLen: u32,
    pub uYomiTextPos: u32,
    pub uYomiDelimPos: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VALUENAME(pub i32);
pub const VALUENAME_BUILT_IN_LIST: VALUENAME = VALUENAME(2i32);
pub const VALUENAME_ENTERPRISE_DEFINED_CLASS_ID: VALUENAME = VALUENAME(1i32);
pub const VALUENAME_UNKNOWN: VALUENAME = VALUENAME(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINSTATIONINFOCLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINSTATIONINFORMATIONW {
    pub Reserved2: [u8; 70],
    pub LogonId: u32,
    pub Reserved3: [u8; 1140],
}
impl Default for WINSTATIONINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WINWATCHNOTIFYPROC = Option<unsafe extern "system" fn(hww: HWINWATCH, hwnd: super::super::Foundation::HWND, code: u32, lparam: super::super::Foundation::LPARAM)>;
pub const WINWATCHNOTIFY_CHANGED: u32 = 4u32;
pub const WINWATCHNOTIFY_CHANGING: u32 = 3u32;
pub const WINWATCHNOTIFY_DESTROY: u32 = 2u32;
pub const WINWATCHNOTIFY_START: u32 = 0u32;
pub const WINWATCHNOTIFY_STOP: u32 = 1u32;
pub const WLDP_CANEXECUTEBUFFER_FN: windows_core::PCSTR = windows_core::s!("WldpCanExecuteBuffer");
pub const WLDP_CANEXECUTEFILE_FN: windows_core::PCSTR = windows_core::s!("WldpCanExecuteFile");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WLDP_DEVICE_SECURITY_INFORMATION {
    pub UnlockIdSize: u32,
    pub UnlockId: *mut u8,
    pub ManufacturerIDLength: u32,
    pub ManufacturerID: windows_core::PWSTR,
}
impl Default for WLDP_DEVICE_SECURITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WLDP_DLL: windows_core::PCWSTR = windows_core::w!("WLDP.DLL");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WLDP_EXECUTION_EVALUATION_OPTIONS(pub i32);
impl WLDP_EXECUTION_EVALUATION_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WLDP_EXECUTION_EVALUATION_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WLDP_EXECUTION_EVALUATION_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WLDP_EXECUTION_EVALUATION_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WLDP_EXECUTION_EVALUATION_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WLDP_EXECUTION_EVALUATION_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WLDP_EXECUTION_EVALUATION_OPTION_EXECUTE_IN_INTERACTIVE_SESSION: WLDP_EXECUTION_EVALUATION_OPTIONS = WLDP_EXECUTION_EVALUATION_OPTIONS(1i32);
pub const WLDP_EXECUTION_EVALUATION_OPTION_NONE: WLDP_EXECUTION_EVALUATION_OPTIONS = WLDP_EXECUTION_EVALUATION_OPTIONS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WLDP_EXECUTION_POLICY(pub i32);
pub const WLDP_EXECUTION_POLICY_ALLOWED: WLDP_EXECUTION_POLICY = WLDP_EXECUTION_POLICY(1i32);
pub const WLDP_EXECUTION_POLICY_BLOCKED: WLDP_EXECUTION_POLICY = WLDP_EXECUTION_POLICY(0i32);
pub const WLDP_EXECUTION_POLICY_REQUIRE_SANDBOX: WLDP_EXECUTION_POLICY = WLDP_EXECUTION_POLICY(2i32);
pub const WLDP_FLAGS_SKIPSIGNATUREVALIDATION: u32 = 256u32;
pub const WLDP_GETLOCKDOWNPOLICY_FN: windows_core::PCSTR = windows_core::s!("WldpGetLockdownPolicy");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WLDP_HOST(pub i32);
pub const WLDP_HOST_CMD: windows_core::GUID = windows_core::GUID::from_u128(0x5baea1d6_6f1c_488e_8490_347fa5c5067f);
pub const WLDP_HOST_HTML: windows_core::GUID = windows_core::GUID::from_u128(0xb35a71b6_fe56_48d6_9543_2dff0ecded66);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WLDP_HOST_ID(pub i32);
pub const WLDP_HOST_ID_ALL: WLDP_HOST_ID = WLDP_HOST_ID(7i32);
pub const WLDP_HOST_ID_GLOBAL: WLDP_HOST_ID = WLDP_HOST_ID(1i32);
pub const WLDP_HOST_ID_IE: WLDP_HOST_ID = WLDP_HOST_ID(5i32);
pub const WLDP_HOST_ID_MAX: WLDP_HOST_ID = WLDP_HOST_ID(8i32);
pub const WLDP_HOST_ID_MSI: WLDP_HOST_ID = WLDP_HOST_ID(6i32);
pub const WLDP_HOST_ID_POWERSHELL: WLDP_HOST_ID = WLDP_HOST_ID(4i32);
pub const WLDP_HOST_ID_UNKNOWN: WLDP_HOST_ID = WLDP_HOST_ID(0i32);
pub const WLDP_HOST_ID_VBA: WLDP_HOST_ID = WLDP_HOST_ID(2i32);
pub const WLDP_HOST_ID_WSH: WLDP_HOST_ID = WLDP_HOST_ID(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WLDP_HOST_INFORMATION {
    pub dwRevision: u32,
    pub dwHostId: WLDP_HOST_ID,
    pub szSource: windows_core::PCWSTR,
    pub hSource: super::super::Foundation::HANDLE,
}
pub const WLDP_HOST_INFORMATION_REVISION: u32 = 1u32;
pub const WLDP_HOST_JAVASCRIPT: windows_core::GUID = windows_core::GUID::from_u128(0x5629f0d5_1cca_4fed_a1a3_36a8c18d74c0);
pub const WLDP_HOST_MAX: WLDP_HOST = WLDP_HOST(2i32);
pub const WLDP_HOST_MSI: windows_core::GUID = windows_core::GUID::from_u128(0x624eb611_6e7e_4eec_9bfe_f0ecdbfcf390);
pub const WLDP_HOST_OTHER: windows_core::GUID = windows_core::GUID::from_u128(0x626cbec3_e1fa_4227_9800_ed210274cf7c);
pub const WLDP_HOST_POWERSHELL: windows_core::GUID = windows_core::GUID::from_u128(0x8e9aaa7c_198b_4879_ae41_a50d47ad6458);
pub const WLDP_HOST_PYTHON: windows_core::GUID = windows_core::GUID::from_u128(0xbfd557ef_2448_42ec_810b_0d9f09352d4a);
pub const WLDP_HOST_RUNDLL32: WLDP_HOST = WLDP_HOST(0i32);
pub const WLDP_HOST_SVCHOST: WLDP_HOST = WLDP_HOST(1i32);
pub const WLDP_HOST_WINDOWS_SCRIPT_HOST: windows_core::GUID = windows_core::GUID::from_u128(0xd30b84c5_29ce_4ff3_86ec_a30007a82e49);
pub const WLDP_HOST_XML: windows_core::GUID = windows_core::GUID::from_u128(0x5594be58_c6bf_4295_82f4_d494d20e3a36);
pub const WLDP_ISAPPAPPROVEDBYPOLICY_FN: windows_core::PCSTR = windows_core::s!("WldpIsAppApprovedByPolicy");
pub const WLDP_ISCLASSINAPPROVEDLIST_FN: windows_core::PCSTR = windows_core::s!("WldpIsClassInApprovedList");
pub const WLDP_ISDYNAMICCODEPOLICYENABLED_FN: windows_core::PCSTR = windows_core::s!("WldpIsDynamicCodePolicyEnabled");
pub const WLDP_ISPRODUCTIONCONFIGURATION_FN: windows_core::PCSTR = windows_core::s!("WldpIsProductionConfiguration");
pub const WLDP_ISWCOSPRODUCTIONCONFIGURATION_FN: windows_core::PCSTR = windows_core::s!("WldpIsWcosProductionConfiguration");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WLDP_KEY(pub i32);
pub const WLDP_LOCKDOWN_AUDIT_FLAG: u32 = 8u32;
pub const WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG: u32 = 2u32;
pub const WLDP_LOCKDOWN_CONFIG_CI_FLAG: u32 = 1u32;
pub const WLDP_LOCKDOWN_DEFINED_FLAG: u32 = 2147483648u32;
pub const WLDP_LOCKDOWN_EXCLUSION_FLAG: u32 = 16u32;
pub const WLDP_LOCKDOWN_OFF: u32 = 2147483648u32;
pub const WLDP_LOCKDOWN_UMCIENFORCE_FLAG: u32 = 4u32;
pub const WLDP_LOCKDOWN_UNDEFINED: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WLDP_POLICY_SETTING(pub i32);
pub const WLDP_POLICY_SETTING_AV_PERF_MODE: WLDP_POLICY_SETTING = WLDP_POLICY_SETTING(1000i32);
pub const WLDP_QUERYDANAMICCODETRUST_FN: windows_core::PCSTR = windows_core::s!("WldpQueryDynamicCodeTrust");
pub const WLDP_QUERYDEVICESECURITYINFORMATION_FN: windows_core::PCSTR = windows_core::s!("WldpQueryDeviceSecurityInformation");
pub const WLDP_QUERYDYNAMICCODETRUST_FN: windows_core::PCSTR = windows_core::s!("WldpQueryDynamicCodeTrust");
pub const WLDP_QUERYPOLICYSETTINGENABLED2_FN: windows_core::PCSTR = windows_core::s!("WldpQueryPolicySettingEnabled2");
pub const WLDP_QUERYPOLICYSETTINGENABLED_FN: windows_core::PCSTR = windows_core::s!("WldpQueryPolicySettingEnabled");
pub const WLDP_QUERYWINDOWSLOCKDOWNMODE_FN: windows_core::PCSTR = windows_core::s!("WldpQueryWindowsLockdownMode");
pub const WLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_FN: windows_core::PCSTR = windows_core::s!("WldpQueryWindowsLockdownRestriction");
pub const WLDP_RESETPRODUCTIONCONFIGURATION_FN: windows_core::PCSTR = windows_core::s!("WldpResetProductionConfiguration");
pub const WLDP_RESETWCOSPRODUCTIONCONFIGURATION_FN: windows_core::PCSTR = windows_core::s!("WldpResetWcosProductionConfiguration");
pub const WLDP_SETDYNAMICCODETRUST_FN: windows_core::PCSTR = windows_core::s!("WldpSetDynamicCodeTrust");
pub const WLDP_SETWINDOWSLOCKDOWNRESTRICTION_FN: windows_core::PCSTR = windows_core::s!("WldpSetWindowsLockdownRestriction");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WLDP_WINDOWS_LOCKDOWN_MODE(pub i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(2i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_MAX: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(3i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(1i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WLDP_WINDOWS_LOCKDOWN_RESTRICTION(pub i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_MAX: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(3i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NONE: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(0i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(1i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK_PERMANENT: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(2i32);
pub const WM_CONVERTREQUEST: u32 = 266u32;
pub const WM_CONVERTRESULT: u32 = 267u32;
pub const WM_IMEKEYDOWN: u32 = 656u32;
pub const WM_IMEKEYUP: u32 = 657u32;
pub const WM_IME_REPORT: u32 = 640u32;
pub const WM_INTERIM: u32 = 268u32;
pub const WM_WNT_CONVERTREQUESTEX: u32 = 265u32;
pub const WinStationInformation: WINSTATIONINFOCLASS = WINSTATIONINFOCLASS(8i32);
