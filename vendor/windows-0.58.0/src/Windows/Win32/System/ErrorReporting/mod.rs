#[inline]
pub unsafe fn AddERExcludedApplicationA<P0>(szapplication: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("faultrep.dll" "system" fn AddERExcludedApplicationA(szapplication : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    AddERExcludedApplicationA(szapplication.param().abi()).ok()
}
#[inline]
pub unsafe fn AddERExcludedApplicationW<P0>(wszapplication: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("faultrep.dll" "system" fn AddERExcludedApplicationW(wszapplication : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    AddERExcludedApplicationW(wszapplication.param().abi()).ok()
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn ReportFault(pep: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, dwopt: u32) -> EFaultRepRetVal {
    windows_targets::link!("faultrep.dll" "system" fn ReportFault(pep : *const super::Diagnostics::Debug:: EXCEPTION_POINTERS, dwopt : u32) -> EFaultRepRetVal);
    ReportFault(pep, dwopt)
}
#[inline]
pub unsafe fn WerAddExcludedApplication<P0, P1>(pwzexename: P0, ballusers: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("wer.dll" "system" fn WerAddExcludedApplication(pwzexename : windows_core::PCWSTR, ballusers : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    WerAddExcludedApplication(pwzexename.param().abi(), ballusers.param().abi()).ok()
}
#[inline]
pub unsafe fn WerFreeString<P0>(pwszstr: P0)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wer.dll" "system" fn WerFreeString(pwszstr : windows_core::PCWSTR));
    WerFreeString(pwszstr.param().abi())
}
#[inline]
pub unsafe fn WerGetFlags<P0>(hprocess: P0) -> windows_core::Result<WER_FAULT_REPORTING>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WerGetFlags(hprocess : super::super::Foundation:: HANDLE, pdwflags : *mut WER_FAULT_REPORTING) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WerGetFlags(hprocess.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WerRegisterAdditionalProcess(processid: u32, captureextrainfoforthreadid: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn WerRegisterAdditionalProcess(processid : u32, captureextrainfoforthreadid : u32) -> windows_core::HRESULT);
    WerRegisterAdditionalProcess(processid, captureextrainfoforthreadid).ok()
}
#[inline]
pub unsafe fn WerRegisterAppLocalDump<P0>(localappdatarelativepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WerRegisterAppLocalDump(localappdatarelativepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    WerRegisterAppLocalDump(localappdatarelativepath.param().abi()).ok()
}
#[inline]
pub unsafe fn WerRegisterCustomMetadata<P0, P1>(key: P0, value: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WerRegisterCustomMetadata(key : windows_core::PCWSTR, value : windows_core::PCWSTR) -> windows_core::HRESULT);
    WerRegisterCustomMetadata(key.param().abi(), value.param().abi()).ok()
}
#[inline]
pub unsafe fn WerRegisterExcludedMemoryBlock(address: *const core::ffi::c_void, size: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn WerRegisterExcludedMemoryBlock(address : *const core::ffi::c_void, size : u32) -> windows_core::HRESULT);
    WerRegisterExcludedMemoryBlock(address, size).ok()
}
#[inline]
pub unsafe fn WerRegisterFile<P0>(pwzfile: P0, regfiletype: WER_REGISTER_FILE_TYPE, dwflags: WER_FILE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WerRegisterFile(pwzfile : windows_core::PCWSTR, regfiletype : WER_REGISTER_FILE_TYPE, dwflags : WER_FILE) -> windows_core::HRESULT);
    WerRegisterFile(pwzfile.param().abi(), regfiletype, dwflags).ok()
}
#[inline]
pub unsafe fn WerRegisterMemoryBlock(pvaddress: *const core::ffi::c_void, dwsize: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn WerRegisterMemoryBlock(pvaddress : *const core::ffi::c_void, dwsize : u32) -> windows_core::HRESULT);
    WerRegisterMemoryBlock(pvaddress, dwsize).ok()
}
#[inline]
pub unsafe fn WerRegisterRuntimeExceptionModule<P0>(pwszoutofprocesscallbackdll: P0, pcontext: *const core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll : windows_core::PCWSTR, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll.param().abi(), pcontext).ok()
}
#[inline]
pub unsafe fn WerRemoveExcludedApplication<P0, P1>(pwzexename: P0, ballusers: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("wer.dll" "system" fn WerRemoveExcludedApplication(pwzexename : windows_core::PCWSTR, ballusers : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    WerRemoveExcludedApplication(pwzexename.param().abi(), ballusers.param().abi()).ok()
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn WerReportAddDump<P0, P1, P2>(hreporthandle: P0, hprocess: P1, hthread: P2, dumptype: WER_DUMP_TYPE, pexceptionparam: Option<*const WER_EXCEPTION_INFORMATION>, pdumpcustomoptions: Option<*const WER_DUMP_CUSTOM_OPTIONS>, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORT>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wer.dll" "system" fn WerReportAddDump(hreporthandle : HREPORT, hprocess : super::super::Foundation:: HANDLE, hthread : super::super::Foundation:: HANDLE, dumptype : WER_DUMP_TYPE, pexceptionparam : *const WER_EXCEPTION_INFORMATION, pdumpcustomoptions : *const WER_DUMP_CUSTOM_OPTIONS, dwflags : u32) -> windows_core::HRESULT);
    WerReportAddDump(hreporthandle.param().abi(), hprocess.param().abi(), hthread.param().abi(), dumptype, core::mem::transmute(pexceptionparam.unwrap_or(std::ptr::null())), core::mem::transmute(pdumpcustomoptions.unwrap_or(std::ptr::null())), dwflags).ok()
}
#[inline]
pub unsafe fn WerReportAddFile<P0, P1>(hreporthandle: P0, pwzpath: P1, repfiletype: WER_FILE_TYPE, dwfileflags: WER_FILE) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORT>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wer.dll" "system" fn WerReportAddFile(hreporthandle : HREPORT, pwzpath : windows_core::PCWSTR, repfiletype : WER_FILE_TYPE, dwfileflags : WER_FILE) -> windows_core::HRESULT);
    WerReportAddFile(hreporthandle.param().abi(), pwzpath.param().abi(), repfiletype, dwfileflags).ok()
}
#[inline]
pub unsafe fn WerReportCloseHandle<P0>(hreporthandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORT>,
{
    windows_targets::link!("wer.dll" "system" fn WerReportCloseHandle(hreporthandle : HREPORT) -> windows_core::HRESULT);
    WerReportCloseHandle(hreporthandle.param().abi()).ok()
}
#[inline]
pub unsafe fn WerReportCreate<P0>(pwzeventtype: P0, reptype: WER_REPORT_TYPE, preportinformation: Option<*const WER_REPORT_INFORMATION>) -> windows_core::Result<HREPORT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wer.dll" "system" fn WerReportCreate(pwzeventtype : windows_core::PCWSTR, reptype : WER_REPORT_TYPE, preportinformation : *const WER_REPORT_INFORMATION, phreporthandle : *mut HREPORT) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WerReportCreate(pwzeventtype.param().abi(), reptype, core::mem::transmute(preportinformation.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WerReportHang<P0, P1>(hwndhungapp: P0, pwzhungapplicationname: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("faultrep.dll" "system" fn WerReportHang(hwndhungapp : super::super::Foundation:: HWND, pwzhungapplicationname : windows_core::PCWSTR) -> windows_core::HRESULT);
    WerReportHang(hwndhungapp.param().abi(), pwzhungapplicationname.param().abi()).ok()
}
#[inline]
pub unsafe fn WerReportSetParameter<P0, P1, P2>(hreporthandle: P0, dwparamid: u32, pwzname: P1, pwzvalue: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORT>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wer.dll" "system" fn WerReportSetParameter(hreporthandle : HREPORT, dwparamid : u32, pwzname : windows_core::PCWSTR, pwzvalue : windows_core::PCWSTR) -> windows_core::HRESULT);
    WerReportSetParameter(hreporthandle.param().abi(), dwparamid, pwzname.param().abi(), pwzvalue.param().abi()).ok()
}
#[inline]
pub unsafe fn WerReportSetUIOption<P0, P1>(hreporthandle: P0, repuitypeid: WER_REPORT_UI, pwzvalue: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORT>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wer.dll" "system" fn WerReportSetUIOption(hreporthandle : HREPORT, repuitypeid : WER_REPORT_UI, pwzvalue : windows_core::PCWSTR) -> windows_core::HRESULT);
    WerReportSetUIOption(hreporthandle.param().abi(), repuitypeid, pwzvalue.param().abi()).ok()
}
#[inline]
pub unsafe fn WerReportSubmit<P0>(hreporthandle: P0, consent: WER_CONSENT, dwflags: WER_SUBMIT_FLAGS, psubmitresult: Option<*mut WER_SUBMIT_RESULT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORT>,
{
    windows_targets::link!("wer.dll" "system" fn WerReportSubmit(hreporthandle : HREPORT, consent : WER_CONSENT, dwflags : WER_SUBMIT_FLAGS, psubmitresult : *mut WER_SUBMIT_RESULT) -> windows_core::HRESULT);
    WerReportSubmit(hreporthandle.param().abi(), consent, dwflags, core::mem::transmute(psubmitresult.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn WerSetFlags(dwflags: WER_FAULT_REPORTING) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn WerSetFlags(dwflags : WER_FAULT_REPORTING) -> windows_core::HRESULT);
    WerSetFlags(dwflags).ok()
}
#[inline]
pub unsafe fn WerStoreClose<P0>(hreportstore: P0)
where
    P0: windows_core::Param<HREPORTSTORE>,
{
    windows_targets::link!("wer.dll" "system" fn WerStoreClose(hreportstore : HREPORTSTORE));
    WerStoreClose(hreportstore.param().abi())
}
#[inline]
pub unsafe fn WerStoreGetFirstReportKey<P0>(hreportstore: P0, ppszreportkey: Option<*mut windows_core::PCWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORTSTORE>,
{
    windows_targets::link!("wer.dll" "system" fn WerStoreGetFirstReportKey(hreportstore : HREPORTSTORE, ppszreportkey : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    WerStoreGetFirstReportKey(hreportstore.param().abi(), core::mem::transmute(ppszreportkey.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn WerStoreGetNextReportKey<P0>(hreportstore: P0, ppszreportkey: Option<*mut windows_core::PCWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORTSTORE>,
{
    windows_targets::link!("wer.dll" "system" fn WerStoreGetNextReportKey(hreportstore : HREPORTSTORE, ppszreportkey : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    WerStoreGetNextReportKey(hreportstore.param().abi(), core::mem::transmute(ppszreportkey.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn WerStoreGetReportCount<P0>(hreportstore: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<HREPORTSTORE>,
{
    windows_targets::link!("wer.dll" "system" fn WerStoreGetReportCount(hreportstore : HREPORTSTORE, pdwreportcount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WerStoreGetReportCount(hreportstore.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WerStoreGetSizeOnDisk<P0>(hreportstore: P0) -> windows_core::Result<u64>
where
    P0: windows_core::Param<HREPORTSTORE>,
{
    windows_targets::link!("wer.dll" "system" fn WerStoreGetSizeOnDisk(hreportstore : HREPORTSTORE, pqwsizeinbytes : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WerStoreGetSizeOnDisk(hreportstore.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WerStoreOpen(repstoretype: REPORT_STORE_TYPES) -> windows_core::Result<HREPORTSTORE> {
    windows_targets::link!("wer.dll" "system" fn WerStoreOpen(repstoretype : REPORT_STORE_TYPES, phreportstore : *mut HREPORTSTORE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WerStoreOpen(repstoretype, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WerStorePurge() -> windows_core::Result<()> {
    windows_targets::link!("wer.dll" "system" fn WerStorePurge() -> windows_core::HRESULT);
    WerStorePurge().ok()
}
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV1<P0, P1>(hreportstore: P0, pszreportkey: P1, preportmetadata: *mut WER_REPORT_METADATA_V1) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORTSTORE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wer.dll" "system" fn WerStoreQueryReportMetadataV1(hreportstore : HREPORTSTORE, pszreportkey : windows_core::PCWSTR, preportmetadata : *mut WER_REPORT_METADATA_V1) -> windows_core::HRESULT);
    WerStoreQueryReportMetadataV1(hreportstore.param().abi(), pszreportkey.param().abi(), preportmetadata).ok()
}
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV2<P0, P1>(hreportstore: P0, pszreportkey: P1, preportmetadata: *mut WER_REPORT_METADATA_V2) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORTSTORE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wer.dll" "system" fn WerStoreQueryReportMetadataV2(hreportstore : HREPORTSTORE, pszreportkey : windows_core::PCWSTR, preportmetadata : *mut WER_REPORT_METADATA_V2) -> windows_core::HRESULT);
    WerStoreQueryReportMetadataV2(hreportstore.param().abi(), pszreportkey.param().abi(), preportmetadata).ok()
}
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV3<P0, P1>(hreportstore: P0, pszreportkey: P1, preportmetadata: *mut WER_REPORT_METADATA_V3) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORTSTORE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wer.dll" "system" fn WerStoreQueryReportMetadataV3(hreportstore : HREPORTSTORE, pszreportkey : windows_core::PCWSTR, preportmetadata : *mut WER_REPORT_METADATA_V3) -> windows_core::HRESULT);
    WerStoreQueryReportMetadataV3(hreportstore.param().abi(), pszreportkey.param().abi(), preportmetadata).ok()
}
#[inline]
pub unsafe fn WerStoreUploadReport<P0, P1>(hreportstore: P0, pszreportkey: P1, dwflags: u32, psubmitresult: Option<*mut WER_SUBMIT_RESULT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<HREPORTSTORE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wer.dll" "system" fn WerStoreUploadReport(hreportstore : HREPORTSTORE, pszreportkey : windows_core::PCWSTR, dwflags : u32, psubmitresult : *mut WER_SUBMIT_RESULT) -> windows_core::HRESULT);
    WerStoreUploadReport(hreportstore.param().abi(), pszreportkey.param().abi(), dwflags, core::mem::transmute(psubmitresult.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn WerUnregisterAdditionalProcess(processid: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn WerUnregisterAdditionalProcess(processid : u32) -> windows_core::HRESULT);
    WerUnregisterAdditionalProcess(processid).ok()
}
#[inline]
pub unsafe fn WerUnregisterAppLocalDump() -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn WerUnregisterAppLocalDump() -> windows_core::HRESULT);
    WerUnregisterAppLocalDump().ok()
}
#[inline]
pub unsafe fn WerUnregisterCustomMetadata<P0>(key: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WerUnregisterCustomMetadata(key : windows_core::PCWSTR) -> windows_core::HRESULT);
    WerUnregisterCustomMetadata(key.param().abi()).ok()
}
#[inline]
pub unsafe fn WerUnregisterExcludedMemoryBlock(address: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn WerUnregisterExcludedMemoryBlock(address : *const core::ffi::c_void) -> windows_core::HRESULT);
    WerUnregisterExcludedMemoryBlock(address).ok()
}
#[inline]
pub unsafe fn WerUnregisterFile<P0>(pwzfilepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WerUnregisterFile(pwzfilepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    WerUnregisterFile(pwzfilepath.param().abi()).ok()
}
#[inline]
pub unsafe fn WerUnregisterMemoryBlock(pvaddress: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn WerUnregisterMemoryBlock(pvaddress : *const core::ffi::c_void) -> windows_core::HRESULT);
    WerUnregisterMemoryBlock(pvaddress).ok()
}
#[inline]
pub unsafe fn WerUnregisterRuntimeExceptionModule<P0>(pwszoutofprocesscallbackdll: P0, pcontext: *const core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll : windows_core::PCWSTR, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll.param().abi(), pcontext).ok()
}
pub const APPCRASH_EVENT: windows_core::PCWSTR = windows_core::w!("APPCRASH");
pub const E_STORE_INVALID: REPORT_STORE_TYPES = REPORT_STORE_TYPES(4i32);
pub const E_STORE_MACHINE_ARCHIVE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(2i32);
pub const E_STORE_MACHINE_QUEUE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(3i32);
pub const E_STORE_USER_ARCHIVE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(0i32);
pub const E_STORE_USER_QUEUE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(1i32);
pub const PACKAGED_APPCRASH_EVENT: windows_core::PCWSTR = windows_core::w!("MoAppCrash");
pub const WER_DUMP_AUXILIARY: u32 = 2u32;
pub const WER_DUMP_MASK_START: u32 = 1u32;
pub const WER_DUMP_NOHEAP_ONQUEUE: u32 = 1u32;
pub const WER_FAULT_REPORTING_ALWAYS_SHOW_UI: WER_FAULT_REPORTING = WER_FAULT_REPORTING(16u32);
pub const WER_FAULT_REPORTING_CRITICAL: u32 = 512u32;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH: u32 = 128u32;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG: u32 = 256u32;
pub const WER_FAULT_REPORTING_DURABLE: u32 = 1024u32;
pub const WER_FAULT_REPORTING_FLAG_DISABLE_THREAD_SUSPENSION: WER_FAULT_REPORTING = WER_FAULT_REPORTING(4u32);
pub const WER_FAULT_REPORTING_FLAG_NOHEAP: WER_FAULT_REPORTING = WER_FAULT_REPORTING(1u32);
pub const WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE: u32 = 64u32;
pub const WER_FAULT_REPORTING_FLAG_QUEUE: WER_FAULT_REPORTING = WER_FAULT_REPORTING(2u32);
pub const WER_FAULT_REPORTING_FLAG_QUEUE_UPLOAD: WER_FAULT_REPORTING = WER_FAULT_REPORTING(8u32);
pub const WER_FAULT_REPORTING_NO_UI: u32 = 32u32;
pub const WER_FILE_ANONYMOUS_DATA: WER_FILE = WER_FILE(2u32);
pub const WER_FILE_COMPRESSED: u32 = 4u32;
pub const WER_FILE_DELETE_WHEN_DONE: WER_FILE = WER_FILE(1u32);
pub const WER_MAX_APPLICATION_NAME_LENGTH: u32 = 128u32;
pub const WER_MAX_BUCKET_ID_STRING_LENGTH: u32 = 260u32;
pub const WER_MAX_DESCRIPTION_LENGTH: u32 = 512u32;
pub const WER_MAX_EVENT_NAME_LENGTH: u32 = 64u32;
pub const WER_MAX_FRIENDLY_EVENT_NAME_LENGTH: u32 = 128u32;
pub const WER_MAX_LOCAL_DUMP_SUBPATH_LENGTH: u32 = 64u32;
pub const WER_MAX_PARAM_COUNT: u32 = 10u32;
pub const WER_MAX_PARAM_LENGTH: u32 = 260u32;
pub const WER_MAX_PREFERRED_MODULES: u32 = 128u32;
pub const WER_MAX_PREFERRED_MODULES_BUFFER: u32 = 256u32;
pub const WER_MAX_REGISTERED_DUMPCOLLECTION: u32 = 4u32;
pub const WER_MAX_REGISTERED_ENTRIES: u32 = 512u32;
pub const WER_MAX_REGISTERED_METADATA: u32 = 8u32;
pub const WER_MAX_REGISTERED_RUNTIME_EXCEPTION_MODULES: u32 = 16u32;
pub const WER_MAX_SIGNATURE_NAME_LENGTH: u32 = 128u32;
pub const WER_MAX_TOTAL_PARAM_LENGTH: u32 = 1720u32;
pub const WER_METADATA_KEY_MAX_LENGTH: u32 = 64u32;
pub const WER_METADATA_VALUE_MAX_LENGTH: u32 = 128u32;
pub const WER_P0: u32 = 0u32;
pub const WER_P1: u32 = 1u32;
pub const WER_P2: u32 = 2u32;
pub const WER_P3: u32 = 3u32;
pub const WER_P4: u32 = 4u32;
pub const WER_P5: u32 = 5u32;
pub const WER_P6: u32 = 6u32;
pub const WER_P7: u32 = 7u32;
pub const WER_P8: u32 = 8u32;
pub const WER_P9: u32 = 9u32;
pub const WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH: windows_core::PCSTR = windows_core::s!("OutOfProcessExceptionEventDebuggerLaunchCallback");
pub const WER_RUNTIME_EXCEPTION_EVENT_FUNCTION: windows_core::PCSTR = windows_core::s!("OutOfProcessExceptionEventCallback");
pub const WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE_FUNCTION: windows_core::PCSTR = windows_core::s!("OutOfProcessExceptionEventSignatureCallback");
pub const WER_SUBMIT_ADD_REGISTERED_DATA: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(16u32);
pub const WER_SUBMIT_ARCHIVE_PARAMETERS_ONLY: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(4096u32);
pub const WER_SUBMIT_BYPASS_DATA_THROTTLING: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(2048u32);
pub const WER_SUBMIT_BYPASS_NETWORK_COST_THROTTLING: u32 = 32768u32;
pub const WER_SUBMIT_BYPASS_POWER_THROTTLING: u32 = 16384u32;
pub const WER_SUBMIT_HONOR_RECOVERY: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(1u32);
pub const WER_SUBMIT_HONOR_RESTART: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(2u32);
pub const WER_SUBMIT_NO_ARCHIVE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(256u32);
pub const WER_SUBMIT_NO_CLOSE_UI: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(64u32);
pub const WER_SUBMIT_NO_QUEUE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(128u32);
pub const WER_SUBMIT_OUTOFPROCESS: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(32u32);
pub const WER_SUBMIT_OUTOFPROCESS_ASYNC: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(1024u32);
pub const WER_SUBMIT_QUEUE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(4u32);
pub const WER_SUBMIT_REPORT_MACHINE_ID: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(8192u32);
pub const WER_SUBMIT_SHOW_DEBUG: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(8u32);
pub const WER_SUBMIT_START_MINIMIZED: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(512u32);
pub const WerConsentAlwaysPrompt: WER_CONSENT = WER_CONSENT(4i32);
pub const WerConsentApproved: WER_CONSENT = WER_CONSENT(2i32);
pub const WerConsentDenied: WER_CONSENT = WER_CONSENT(3i32);
pub const WerConsentMax: WER_CONSENT = WER_CONSENT(5i32);
pub const WerConsentNotAsked: WER_CONSENT = WER_CONSENT(1i32);
pub const WerCustomAction: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(9i32);
pub const WerDisabled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(5i32);
pub const WerDisabledQueue: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(7i32);
pub const WerDumpTypeHeapDump: WER_DUMP_TYPE = WER_DUMP_TYPE(3i32);
pub const WerDumpTypeMax: WER_DUMP_TYPE = WER_DUMP_TYPE(5i32);
pub const WerDumpTypeMicroDump: WER_DUMP_TYPE = WER_DUMP_TYPE(1i32);
pub const WerDumpTypeMiniDump: WER_DUMP_TYPE = WER_DUMP_TYPE(2i32);
pub const WerDumpTypeNone: WER_DUMP_TYPE = WER_DUMP_TYPE(0i32);
pub const WerDumpTypeTriageDump: WER_DUMP_TYPE = WER_DUMP_TYPE(4i32);
pub const WerFileTypeAuxiliaryDump: WER_FILE_TYPE = WER_FILE_TYPE(8i32);
pub const WerFileTypeCustomDump: WER_FILE_TYPE = WER_FILE_TYPE(7i32);
pub const WerFileTypeEtlTrace: WER_FILE_TYPE = WER_FILE_TYPE(9i32);
pub const WerFileTypeHeapdump: WER_FILE_TYPE = WER_FILE_TYPE(3i32);
pub const WerFileTypeMax: WER_FILE_TYPE = WER_FILE_TYPE(10i32);
pub const WerFileTypeMicrodump: WER_FILE_TYPE = WER_FILE_TYPE(1i32);
pub const WerFileTypeMinidump: WER_FILE_TYPE = WER_FILE_TYPE(2i32);
pub const WerFileTypeOther: WER_FILE_TYPE = WER_FILE_TYPE(5i32);
pub const WerFileTypeTriagedump: WER_FILE_TYPE = WER_FILE_TYPE(6i32);
pub const WerFileTypeUserDocument: WER_FILE_TYPE = WER_FILE_TYPE(4i32);
pub const WerRegFileTypeMax: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(3i32);
pub const WerRegFileTypeOther: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(2i32);
pub const WerRegFileTypeUserDocument: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(1i32);
pub const WerReportApplicationCrash: WER_REPORT_TYPE = WER_REPORT_TYPE(2i32);
pub const WerReportApplicationHang: WER_REPORT_TYPE = WER_REPORT_TYPE(3i32);
pub const WerReportAsync: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(8i32);
pub const WerReportCancelled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(6i32);
pub const WerReportCritical: WER_REPORT_TYPE = WER_REPORT_TYPE(1i32);
pub const WerReportDebug: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(3i32);
pub const WerReportFailed: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(4i32);
pub const WerReportInvalid: WER_REPORT_TYPE = WER_REPORT_TYPE(5i32);
pub const WerReportKernel: WER_REPORT_TYPE = WER_REPORT_TYPE(4i32);
pub const WerReportNonCritical: WER_REPORT_TYPE = WER_REPORT_TYPE(0i32);
pub const WerReportQueued: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(1i32);
pub const WerReportUploaded: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(2i32);
pub const WerReportUploadedCab: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(11i32);
pub const WerStorageLocationNotFound: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(12i32);
pub const WerSubmitResultMax: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(13i32);
pub const WerThrottled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(10i32);
pub const WerUIAdditionalDataDlgHeader: WER_REPORT_UI = WER_REPORT_UI(1i32);
pub const WerUICloseDlgBody: WER_REPORT_UI = WER_REPORT_UI(9i32);
pub const WerUICloseDlgButtonText: WER_REPORT_UI = WER_REPORT_UI(10i32);
pub const WerUICloseDlgHeader: WER_REPORT_UI = WER_REPORT_UI(8i32);
pub const WerUICloseText: WER_REPORT_UI = WER_REPORT_UI(7i32);
pub const WerUIConsentDlgBody: WER_REPORT_UI = WER_REPORT_UI(4i32);
pub const WerUIConsentDlgHeader: WER_REPORT_UI = WER_REPORT_UI(3i32);
pub const WerUIIconFilePath: WER_REPORT_UI = WER_REPORT_UI(2i32);
pub const WerUIMax: WER_REPORT_UI = WER_REPORT_UI(11i32);
pub const WerUIOfflineSolutionCheckText: WER_REPORT_UI = WER_REPORT_UI(6i32);
pub const WerUIOnlineSolutionCheckText: WER_REPORT_UI = WER_REPORT_UI(5i32);
pub const frrvErr: EFaultRepRetVal = EFaultRepRetVal(3i32);
pub const frrvErrAnotherInstance: EFaultRepRetVal = EFaultRepRetVal(8i32);
pub const frrvErrDoubleFault: EFaultRepRetVal = EFaultRepRetVal(10i32);
pub const frrvErrNoDW: EFaultRepRetVal = EFaultRepRetVal(4i32);
pub const frrvErrNoMemory: EFaultRepRetVal = EFaultRepRetVal(9i32);
pub const frrvErrTimeout: EFaultRepRetVal = EFaultRepRetVal(5i32);
pub const frrvLaunchDebugger: EFaultRepRetVal = EFaultRepRetVal(6i32);
pub const frrvOk: EFaultRepRetVal = EFaultRepRetVal(0i32);
pub const frrvOkHeadless: EFaultRepRetVal = EFaultRepRetVal(7i32);
pub const frrvOkManifest: EFaultRepRetVal = EFaultRepRetVal(1i32);
pub const frrvOkQueued: EFaultRepRetVal = EFaultRepRetVal(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EFaultRepRetVal(pub i32);
impl windows_core::TypeKind for EFaultRepRetVal {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EFaultRepRetVal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EFaultRepRetVal").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REPORT_STORE_TYPES(pub i32);
impl windows_core::TypeKind for REPORT_STORE_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REPORT_STORE_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REPORT_STORE_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WER_CONSENT(pub i32);
impl windows_core::TypeKind for WER_CONSENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WER_CONSENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WER_CONSENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WER_DUMP_TYPE(pub i32);
impl windows_core::TypeKind for WER_DUMP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WER_DUMP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WER_DUMP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WER_FAULT_REPORTING(pub u32);
impl windows_core::TypeKind for WER_FAULT_REPORTING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WER_FAULT_REPORTING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WER_FAULT_REPORTING").field(&self.0).finish()
    }
}
impl WER_FAULT_REPORTING {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WER_FAULT_REPORTING {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WER_FAULT_REPORTING {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WER_FAULT_REPORTING {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WER_FAULT_REPORTING {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WER_FAULT_REPORTING {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WER_FILE(pub u32);
impl windows_core::TypeKind for WER_FILE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WER_FILE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WER_FILE").field(&self.0).finish()
    }
}
impl WER_FILE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WER_FILE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WER_FILE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WER_FILE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WER_FILE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WER_FILE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WER_FILE_TYPE(pub i32);
impl windows_core::TypeKind for WER_FILE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WER_FILE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WER_FILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WER_REGISTER_FILE_TYPE(pub i32);
impl windows_core::TypeKind for WER_REGISTER_FILE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WER_REGISTER_FILE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WER_REGISTER_FILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WER_REPORT_TYPE(pub i32);
impl windows_core::TypeKind for WER_REPORT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WER_REPORT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WER_REPORT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WER_REPORT_UI(pub i32);
impl windows_core::TypeKind for WER_REPORT_UI {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WER_REPORT_UI {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WER_REPORT_UI").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WER_SUBMIT_FLAGS(pub u32);
impl windows_core::TypeKind for WER_SUBMIT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WER_SUBMIT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WER_SUBMIT_FLAGS").field(&self.0).finish()
    }
}
impl WER_SUBMIT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WER_SUBMIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WER_SUBMIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WER_SUBMIT_RESULT(pub i32);
impl windows_core::TypeKind for WER_SUBMIT_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WER_SUBMIT_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WER_SUBMIT_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREPORT(pub *mut core::ffi::c_void);
impl HREPORT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HREPORT {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = WerReportCloseHandle(*self);
        }
    }
}
impl Default for HREPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HREPORT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREPORTSTORE(pub *mut core::ffi::c_void);
impl HREPORTSTORE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HREPORTSTORE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            WerStoreClose(*self);
        }
    }
}
impl Default for HREPORTSTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HREPORTSTORE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_DUMP_CUSTOM_OPTIONS {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: super::super::Foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
}
impl windows_core::TypeKind for WER_DUMP_CUSTOM_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_DUMP_CUSTOM_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_DUMP_CUSTOM_OPTIONS_V2 {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: super::super::Foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
    pub dwPreferredModuleResetFlags: u32,
    pub dwOtherModuleResetFlags: u32,
}
impl windows_core::TypeKind for WER_DUMP_CUSTOM_OPTIONS_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_DUMP_CUSTOM_OPTIONS_V3 {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: super::super::Foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
    pub dwPreferredModuleResetFlags: u32,
    pub dwOtherModuleResetFlags: u32,
    pub pvDumpKey: *mut core::ffi::c_void,
    pub hSnapshot: super::super::Foundation::HANDLE,
    pub dwThreadID: u32,
}
impl windows_core::TypeKind for WER_DUMP_CUSTOM_OPTIONS_V3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_EXCEPTION_INFORMATION {
    pub pExceptionPointers: *mut super::Diagnostics::Debug::EXCEPTION_POINTERS,
    pub bClientPointers: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for WER_EXCEPTION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl Default for WER_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_REPORT_INFORMATION {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
}
impl windows_core::TypeKind for WER_REPORT_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_REPORT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_REPORT_INFORMATION_V3 {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
}
impl windows_core::TypeKind for WER_REPORT_INFORMATION_V3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_REPORT_INFORMATION_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_REPORT_INFORMATION_V4 {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
    pub rgbApplicationIdentity: [u8; 16],
    pub hSnapshot: super::super::Foundation::HANDLE,
    pub hDeleteFilesImpersonationToken: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for WER_REPORT_INFORMATION_V4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_REPORT_INFORMATION_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_REPORT_INFORMATION_V5 {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
    pub rgbApplicationIdentity: [u8; 16],
    pub hSnapshot: super::super::Foundation::HANDLE,
    pub hDeleteFilesImpersonationToken: super::super::Foundation::HANDLE,
    pub submitResultMax: WER_SUBMIT_RESULT,
}
impl windows_core::TypeKind for WER_REPORT_INFORMATION_V5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_REPORT_INFORMATION_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_REPORT_METADATA_V1 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: windows_core::GUID,
    pub ReportId: windows_core::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
}
impl windows_core::TypeKind for WER_REPORT_METADATA_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_REPORT_METADATA_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_REPORT_METADATA_V2 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: windows_core::GUID,
    pub ReportId: windows_core::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: windows_core::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: windows_core::PWSTR,
}
impl windows_core::TypeKind for WER_REPORT_METADATA_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_REPORT_METADATA_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_REPORT_METADATA_V3 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: windows_core::GUID,
    pub ReportId: windows_core::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: windows_core::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: windows_core::PWSTR,
    pub FriendlyEventName: [u16; 128],
    pub ApplicationName: [u16; 128],
    pub ApplicationPath: [u16; 260],
    pub Description: [u16; 512],
    pub BucketIdString: [u16; 260],
    pub LegacyBucketId: u64,
}
impl windows_core::TypeKind for WER_REPORT_METADATA_V3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_REPORT_METADATA_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_REPORT_PARAMETER {
    pub Name: [u16; 129],
    pub Value: [u16; 260],
}
impl windows_core::TypeKind for WER_REPORT_PARAMETER {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WER_REPORT_SIGNATURE {
    pub EventName: [u16; 65],
    pub Parameters: [WER_REPORT_PARAMETER; 10],
}
impl windows_core::TypeKind for WER_REPORT_SIGNATURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for WER_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct WER_RUNTIME_EXCEPTION_INFORMATION {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub exceptionRecord: super::Diagnostics::Debug::EXCEPTION_RECORD,
    pub context: super::Diagnostics::Debug::CONTEXT,
    pub pwszReportId: windows_core::PCWSTR,
    pub bIsFatal: super::super::Foundation::BOOL,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for WER_RUNTIME_EXCEPTION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl Default for WER_RUNTIME_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH = Option<unsafe extern "system" fn(pcontext: *const core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbiscustomdebugger: *mut super::super::Foundation::BOOL, pwszdebuggerlaunch: windows_core::PWSTR, pchdebuggerlaunch: *mut u32, pbisdebuggerautolaunch: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT = Option<unsafe extern "system" fn(pcontext: *const core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbownershipclaimed: *mut super::super::Foundation::BOOL, pwszeventname: windows_core::PWSTR, pchsize: *mut u32, pdwsignaturecount: *mut u32) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE = Option<unsafe extern "system" fn(pcontext: *const core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, dwindex: u32, pwszname: windows_core::PWSTR, pchname: *mut u32, pwszvalue: windows_core::PWSTR, pchvalue: *mut u32) -> windows_core::HRESULT>;
pub type pfn_ADDEREXCLUDEDAPPLICATIONA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR) -> EFaultRepRetVal>;
pub type pfn_ADDEREXCLUDEDAPPLICATIONW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> EFaultRepRetVal>;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type pfn_REPORTFAULT = Option<unsafe extern "system" fn(param0: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, param1: u32) -> EFaultRepRetVal>;
